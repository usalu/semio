#!/usr/bin/env bun
/**
 * @emoji 🧭 Rebuilds declared `IS` from `semio/client/schema/semio/schema.yaml`: kit members get one `IS` per field; `Class`/`Interface` types keep **only** direct `implements` targets (no transitive `IS` to supertypes like Entity).
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

/** @emoji 📐 One `Class`/`Interface` type’s YAML map key and direct `implements` targets (Neo4j node names). */
export type TypeDeclaredIsRow = Readonly<{
  owner: string;
  targets: readonly string[];
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

/** @emoji 🎯 True when this map is a leaf field spec (has `is`); sibling keys like `cached` or `constraints` do not affect declared-`IS` repair. */
function isTerminalFieldMap(m: YAMLMap): boolean {
  return m.has("is", true);
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

function implementsTargetsFromTypeBody(body: YAMLMap): string[] {
  const imp = body.get("implements", true);
  if (imp == null) {
    return [];
  }
  if (isAlias(imp)) {
    return [targetNameFromAliasSource(imp.source)];
  }
  if (isSeq(imp)) {
    const out: string[] = [];
    for (const item of imp.items) {
      if (item != null && isAlias(item)) {
        out.push(targetNameFromAliasSource(item.source));
      }
    }
    return out;
  }
  return [];
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

/**
 * @emoji 🗂️ Direct `implements` per `Class`/`Interface` map in schema.yaml (general + domain + vcs); merged by owner key.
 */
export function collectTypeDeclaredIsRows(schemaYamlText: string): TypeDeclaredIsRow[] {
  const doc = parseDocument(schemaYamlText, { maxAliasCount: 1_000_000 });
  const schema = doc.get("schema", true);
  if (!isMap(schema)) {
    throw new Error("[type-is] schema root missing or not a map");
  }
  const byOwner = new Map<string, Set<string>>();

  function record(ownerYamlKey: string, body: unknown): void {
    if (!isMap(body)) {
      return;
    }
    if (!byOwner.has(ownerYamlKey)) {
      byOwner.set(ownerYamlKey, new Set());
    }
    const set = byOwner.get(ownerYamlKey)!;
    const targets = implementsTargetsFromTypeBody(body);
    for (const t of targets) {
      set.add(t);
    }
  }

  function walkInterfaceMap(ifaceMap: unknown): void {
    if (!isMap(ifaceMap)) {
      return;
    }
    for (const pair of ifaceMap.items) {
      const key = pair.key != null ? String(pair.key) : "";
      if (key.length === 0) {
        continue;
      }
      record(key, pair.value);
    }
  }

  function walkClassMap(classMap: unknown): void {
    if (!isMap(classMap)) {
      return;
    }
    for (const pair of classMap.items) {
      const key = pair.key != null ? String(pair.key) : "";
      if (key.length === 0) {
        continue;
      }
      record(key, pair.value);
    }
  }

  const general = schema.get("general", true);
  if (isMap(general)) {
    walkInterfaceMap(general.get("interfaces", true));
  }
  const domain = schema.get("domain", true);
  if (isMap(domain)) {
    walkClassMap(domain.get("classes", true));
    walkInterfaceMap(domain.get("interfaces", true));
    const vcs = domain.get("vcs", true);
    if (isMap(vcs)) {
      walkInterfaceMap(vcs.get("interfaces", true));
    }
  }

  const rows: TypeDeclaredIsRow[] = [];
  for (const [owner, set] of byOwner) {
    rows.push({ owner, targets: [...set].sort((a, b) => a.localeCompare(b)) });
  }
  rows.sort((a, b) => a.owner.localeCompare(b.owner));
  return rows;
}
//#endregion 🧭YamlWalk

//#region 🧭Cypher
/**
 * @emoji 🔗 Kit members (`Data` / `Derived` / `Reference`) keep **only** the single declared `IS` from YAML (concrete `Class`, `Interface`, or `Scalar`). `Class`/`Interface`/`Command` types keep **only** direct `implements` edges (see {@link buildTypeDeclaredIsRepairCypher}).
 */
export function materializeTransitiveIsForKitMembersCypher(): string {
  return "";
}

function escapeLiteral(s: string): string {
  return s.replace(/\\/g, "\\\\").replace(/'/g, "\\'");
}

function buildRepairCypher(rows: readonly KitFieldDeclaredIsRow[]): string {
  const lines: string[] = [
    "// SPDX-License-Identifier: AGPL-3.0-only",
    "// Generated by kit-field-declared-is.script.ts — strip kit-member IS, reattach declared FieldIs, idempotent.",
    "",
    "MATCH (n:Data|Derived|Reference)-[r:IS]->()",
    "MATCH (n)<-[:OWNS]-(owner)",
    "WHERE owner:Class OR owner:Interface OR owner:Command",
    "DELETE r;",
    "",
  ];
  for (const r of rows) {
    const o = escapeLiteral(r.owner);
    const f = escapeLiteral(r.field);
    const t = escapeLiteral(r.target);
    lines.push(
      `MATCH (owner:Class|Interface|Command)`,
      `WHERE toLower(owner.name) = toLower('${o}')`,
      `MATCH (owner)-[:OWNS]->(n:Data|Derived|Reference {name: '${f}'})`,
      `MATCH (target)`,
      `WHERE (target:Class OR target:Interface OR target:Command OR target:Scalar) AND toLower(target.name) = toLower('${t}')`,
      `MERGE (n)-[:IS]->(target);`,
      "",
    );
  }
  return lines.join("\n");
}

/**
 * @emoji 🧷 Drops every `IS` from a type that is not listed under YAML `implements`, then `MERGE`s each declared edge (direct supertypes only).
 */
export function buildTypeDeclaredIsRepairCypher(rows: readonly TypeDeclaredIsRow[]): string {
  const lines: string[] = [
    "// SPDX-License-Identifier: AGPL-3.0-only",
    "// Class/Interface/Command: keep only direct schema.yaml `implements` as `IS` (strip transitive Entity/StrongEntity/… fan-out).",
    "",
  ];
  for (const row of rows) {
    const o = escapeLiteral(row.owner);
    if (row.targets.length === 0) {
      lines.push(
        `MATCH (owner:Class|Interface|Command)`,
        `WHERE toLower(owner.name) = toLower('${o}')`,
        `MATCH (owner)-[r:IS]->()`,
        `DELETE r;`,
        "",
      );
    } else {
      const inList = row.targets.map((t) => `'${escapeLiteral(t.toLowerCase())}'`).join(", ");
      lines.push(
        `MATCH (owner:Class|Interface|Command)`,
        `WHERE toLower(owner.name) = toLower('${o}')`,
        `MATCH (owner)-[r:IS]->(t)`,
        `WHERE NOT toLower(t.name) IN [${inList}]`,
        `DELETE r;`,
        "",
      );
    }
    for (const t of row.targets) {
      const tl = escapeLiteral(t);
      lines.push(
        `MATCH (owner:Class|Interface|Command)`,
        `WHERE toLower(owner.name) = toLower('${o}')`,
        `MATCH (target:Class|Interface)`,
        `WHERE toLower(target.name) = toLower('${tl}')`,
        `MERGE (owner)-[:IS]->(target);`,
        "",
      );
    }
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
 * @emoji 🛠️ Realigns `IS` with schema.yaml: kit fields get one declared `IS` each; `Class`/`Interface` get only direct `implements` (no transitive supertypes).
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
  const typeRows = collectTypeDeclaredIsRows(text);
  if (typeRows.length === 0) {
    throw new Error("[type-is] no class/interface rows from yaml");
  }
  const repairBody = buildRepairCypher(rows);
  const typeRepairBody = buildTypeDeclaredIsRepairCypher(typeRows);
  const materializeBody = materializeTransitiveIsForKitMembersCypher().trim();
  const batchPath = join(opts.cacheDir, `kit-field-declared-is-repair-${process.pid}.cypher`);
  const core = [repairBody.trim(), typeRepairBody.trim(), materializeBody].filter((s) => s.length > 0).join("\n\n");
  writeFileSync(batchPath, `${core}\n`, "utf8");
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
