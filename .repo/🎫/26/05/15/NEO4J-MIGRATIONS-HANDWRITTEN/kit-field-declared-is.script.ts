#!/usr/bin/env bun
/**
 * @emoji 🧭 Rebuilds each kit member's declared `IS` edge from `semio/client/schema/semio/schema.yaml`, then callers may re-run transitive `IS` materialization.
 */
import { existsSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { isAlias, isMap, isSeq, parseDocument, type YAMLMap, type YAMLNode } from "yaml";

//#region 🧭Types
/** @emoji 📌 One kit field's owner (YAML map key, any casing), member name, and FieldIs target node name in Neo4j. */
export type KitFieldDeclaredIsRow = Readonly<{
  owner: string;
  field: string;
  target: string;
}>;
//#endregion 🧭Types

//#region 🧭YamlWalk
function pascalFromYamlKey(key: string): string {
  return key.length === 0 ? key : key.charAt(0).toUpperCase() + key.slice(1);
}

const scalarAliasToNeo4jScalarName: Readonly<Record<string, string>> = {
  string: "String",
  number: "Number",
  boolean: "Boolean",
  timestamp: "Timestamp",
  uri: "Uri",
  color: "Color",
  icon: "Icon",
};

function targetNameFromAliasSource(aliasSource: string): string {
  return scalarAliasToNeo4jScalarName[aliasSource] ?? pascalFromYamlKey(aliasSource);
}

function isTerminalFieldMap(m: YAMLMap): boolean {
  return m.has("is", true) || m.has("kind", true);
}

function collectFlatFields(fieldsMap: YAMLMap, out: { name: string; valueNode: YAMLNode }[]): void {
  for (const pair of fieldsMap.items) {
    const key = pair.key;
    const value = pair.value;
    if (key == null || value == null) {
      continue;
    }
    const k = String(key);
    if (k === "implements" || k === "contraints") {
      continue;
    }
    if (isMap(value)) {
      if (isTerminalFieldMap(value)) {
        out.push({ name: k, valueNode: value });
      } else {
        collectFlatFields(value, out);
      }
    } else {
      out.push({ name: k, valueNode: value });
    }
  }
}

function declaredTargetNameForValueNode(node: YAMLNode | null | undefined): string | null {
  if (node == null) {
    return null;
  }
  if (isAlias(node)) {
    return targetNameFromAliasSource(node.source);
  }
  if (isSeq(node)) {
    const first = node.items[0];
    if (first != null && isAlias(first)) {
      return targetNameFromAliasSource(first.source);
    }
    return null;
  }
  if (isMap(node)) {
    const isNode = node.get("is", true);
    if (isAlias(isNode)) {
      return targetNameFromAliasSource(isNode.source);
    }
  }
  return null;
}

function collectFromOwnerMap(ownerYamlKey: string, body: unknown, rows: KitFieldDeclaredIsRow[]): void {
  if (!isMap(body)) {
    return;
  }
  const fieldsNode = body.get("fields", true);
  if (!isMap(fieldsNode)) {
    return;
  }
  const owner = ownerYamlKey;
  const flat: { name: string; valueNode: YAMLNode }[] = [];
  collectFlatFields(fieldsNode, flat);
  for (const { name, valueNode } of flat) {
    const target = declaredTargetNameForValueNode(valueNode);
    if (target == null) {
      continue;
    }
    rows.push({ owner, field: name, target });
  }
}

/**
 * @emoji 🗂️ Reads schema.yaml (document mode, high alias budget) and returns declared FieldIs targets per owner+field.
 */
export function collectKitFieldDeclaredIsRows(schemaYamlText: string): KitFieldDeclaredIsRow[] {
  const doc = parseDocument(schemaYamlText, { maxAliasCount: 1_000_000 });
  const schema = doc.get("schema", true);
  if (!isMap(schema)) {
    throw new Error("[kit-field-is] schema root missing or not a map");
  }
  const rows: KitFieldDeclaredIsRow[] = [];
  const general = schema.get("general", true);
  if (isMap(general)) {
    const ifaces = general.get("interfaces", true);
    if (isMap(ifaces)) {
      for (const pair of ifaces.items) {
        const key = pair.key != null ? String(pair.key) : "";
        const body = pair.value;
        collectFromOwnerMap(key, body, rows);
      }
    }
  }
  const domain = schema.get("domain", true);
  if (isMap(domain)) {
    const classes = domain.get("classes", true);
    if (isMap(classes)) {
      for (const pair of classes.items) {
        const key = pair.key != null ? String(pair.key) : "";
        collectFromOwnerMap(key, pair.value, rows);
      }
    }
    const domainIfaces = domain.get("interfaces", true);
    if (isMap(domainIfaces)) {
      for (const pair of domainIfaces.items) {
        const key = pair.key != null ? String(pair.key) : "";
        collectFromOwnerMap(key, pair.value, rows);
      }
    }
    const vcs = domain.get("vcs", true);
    if (isMap(vcs)) {
      const vcsIfaces = vcs.get("interfaces", true);
      if (isMap(vcsIfaces)) {
        for (const pair of vcsIfaces.items) {
          const key = pair.key != null ? String(pair.key) : "";
          collectFromOwnerMap(key, pair.value, rows);
        }
      }
    }
  }
  const dedup = new Map<string, KitFieldDeclaredIsRow>();
  for (const r of rows) {
    dedup.set(`${r.owner}\0${r.field}`, r);
  }
  return [...dedup.values()];
}
//#endregion 🧭YamlWalk

//#region 🧭Cypher
/** @emoji 📐 Kit `IS` targets that only implement `Data` in YAML — do not copy the entity interface ladder onto kit members. */
const PRIMITIVE_VALUE_CLASS_NAMES = [
  "Vector",
  "Point",
  "Coordinate",
  "Offset",
  "Plane",
] as const;

/** @emoji 🔗 Transitive `IS` on kit members: full interface closure from direct `Interface` hops; from `Class` only when not a primitive value class (avoids `xAxis`→`Vector`→`Entity` noise). */
export function materializeTransitiveIsForKitMembersCypher(): string {
  const skipList = PRIMITIVE_VALUE_CLASS_NAMES.map((n) => `'${n}'`).join(", ");
  return [
    "MATCH (n:Data|Computation|Reference)-[:IS]->(i:Interface)",
    "MATCH (i)-[:IS*1..25]->(b:Interface)",
    "WHERE n <> b",
    "MERGE (n)-[:IS]->(b);",
    "MATCH (n:Data|Computation|Reference)-[:IS]->(c:Class)",
    `WHERE NOT c.name IN [${skipList}]`,
    "MATCH (c)-[:IS*1..25]->(b:Interface)",
    "MERGE (n)-[:IS]->(b);",
  ].join("\n");
}

function escapeLiteral(s: string): string {
  return s.replace(/\\/g, "\\\\").replace(/'/g, "\\'");
}

function buildRepairCypher(rows: readonly KitFieldDeclaredIsRow[]): string {
  const lines: string[] = [
    "// SPDX-License-Identifier: AGPL-3.0-only",
    "// Generated by kit-field-declared-is.script.ts — strip kit-member IS, reattach declared FieldIs, idempotent.",
    "",
    "MATCH (n:Data|Computation|Reference)-[r:IS]->()",
    "MATCH (n)<-[:OWNS]-(owner)",
    "WHERE owner:Class OR owner:Interface",
    "DELETE r;",
    "",
  ];
  for (const r of rows) {
    const o = escapeLiteral(r.owner);
    const f = escapeLiteral(r.field);
    const t = escapeLiteral(r.target);
    lines.push(
      `MATCH (owner:Class|Interface)`,
      `WHERE toLower(owner.name) = toLower('${o}')`,
      `MATCH (owner)-[:OWNS]->(n:Data|Computation|Reference {name: '${f}'})`,
      `MATCH (target)`,
      `WHERE (target:Class OR target:Interface OR target:Scalar) AND toLower(target.name) = toLower('${t}')`,
      `MERGE (n)-[:IS]->(target);`,
      "",
    );
  }
  return lines.join("\n");
}
//#endregion 🧭Cypher

//#region 🧭CypherShell
function resolveCypherShell(repoRoot: string): string | null {
  const NEO4J_VERSION = "5.26.26";
  const runtimeName = process.platform === "win32" ? "cypher-shell.bat" : "cypher-shell";
  const cachedShell = join(repoRoot, ".repo", "cache", "neo4j", `neo4j-community-${NEO4J_VERSION}`, "bin", runtimeName);
  const candidates = [process.env.NEO4J_CYPHER_SHELL, cachedShell, runtimeName].filter((value): value is string => Boolean(value));
  for (const candidate of candidates) {
    if (candidate.includes("/") || candidate.includes("\\")) {
      if (existsSync(candidate)) {
        return candidate;
      }
      continue;
    }
    const probe = spawnSync(candidate, ["--version"], { stdio: "ignore" });
    if (probe.status === 0) {
      return candidate;
    }
  }
  return null;
}

function buildCypherEnv(): NodeJS.ProcessEnv {
  const env = { ...process.env };
  if (process.platform === "win32") {
    const javaHome = "C:\\Program Files\\Microsoft\\jdk-21.0.11.10-hotspot";
    const javaExecutable = join(javaHome, "bin", "java.exe");
    if (existsSync(javaExecutable)) {
      env.JAVA_HOME = javaHome;
      env.Path = `${join(javaHome, "bin")};${env.Path || ""}`;
    }
  }
  return env;
}

function runCypherFile(repoRoot: string, shell: string, database: string, filePath: string): { ok: boolean; stderr: string } {
  const result = spawnSync(
    shell,
    [
      "-a",
      process.env.NEO4J_URI || "bolt://localhost:7687",
      "-u",
      process.env.NEO4J_USERNAME || "neo4j",
      "-p",
      process.env.NEO4J_PASSWORD || "password",
      "-d",
      database,
      "--format",
      "plain",
      "-f",
      filePath,
    ],
    { encoding: "utf8", cwd: repoRoot, env: buildCypherEnv() },
  );
  const stderr = typeof result.stderr === "string" ? result.stderr : String(result.stderr ?? "");
  return { ok: result.status === 0, stderr };
}
//#endregion 🧭CypherShell

//#region 🚀Entry
/**
 * @emoji 🛠️ Deletes all `IS` from kit members, attaches declared targets from YAML, then materializes transitive interface `IS` edges.
 */
export function runKitFieldDeclaredIsRepair(opts: Readonly<{ repoRoot: string; database: string; cacheDir: string }>): void {
  const yamlPath = join(opts.repoRoot, "semio", "client", "schema", "semio", "schema.yaml");
  if (!existsSync(yamlPath)) {
    throw new Error(`[kit-field-is] missing ${yamlPath}`);
  }
  const text = readFileSync(yamlPath, "utf8");
  const rows = collectKitFieldDeclaredIsRows(text);
  if (rows.length === 0) {
    throw new Error("[kit-field-is] no declared kit field rows from yaml");
  }
  const repairBody = buildRepairCypher(rows);
  const materializeBody = materializeTransitiveIsForKitMembersCypher();
  const batchPath = join(opts.cacheDir, `kit-field-declared-is-repair-${process.pid}.cypher`);
  writeFileSync(batchPath, `${repairBody.trim()}\n\n${materializeBody.trim()}\n`, "utf8");
  const shell = resolveCypherShell(opts.repoRoot);
  if (!shell) {
    try {
      unlinkSync(batchPath);
    } catch {
      /* best-effort */
    }
    throw new Error("[kit-field-is] cypher-shell not found");
  }
  const { ok, stderr } = runCypherFile(opts.repoRoot, shell, opts.database, batchPath);
  try {
    unlinkSync(batchPath);
  } catch {
    /* best-effort */
  }
  if (!ok) {
    throw new Error(`[kit-field-is] repair failed:\n${stderr}`);
  }
}
//#endregion 🚀Entry
