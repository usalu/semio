#!/usr/bin/env bun
/**
 * 🗄️ Builds `.repo/🛂/semio.cypher` from `semio/client/schema/semio/schema.yaml` for Neo4j Bloom (minimal props, no schemaTag/schemaSource/moduleName).
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { isAlias, parseDocument, YAMLMap, YAMLSeq, type Document, type Node, type Pair } from "yaml";

//#region 🧭Constants
const REPO_ROOT = import.meta.dir;
const SCHEMA_YAML = join(REPO_ROOT, "semio/client/schema/semio/schema.yaml");
const OUTPUT_FILE = join(REPO_ROOT, ".repo/🛂/semio.cypher");
const NEO4J_VERSION = "5.26.26";
const MODULE_KEYS = ["general", "domain"] as const;
const SECTION_KEYS = new Set(["data", "computed", "reference"]);
//#endregion 🧭Constants

//#region 🧩YamlHelpers
function isYamlMap(n: Node | null | undefined): n is YAMLMap {
  return n instanceof YAMLMap;
}

function isYamlSeq(n: Node | null | undefined): n is YAMLSeq {
  return n instanceof YAMLSeq;
}

function pairKey(pair: Pair): string {
  return String(pair.key);
}

function fieldMapIsSectioned(fieldsRoot: YAMLMap): boolean {
  if (fieldsRoot.items.length === 0) {
    return false;
  }
  return fieldsRoot.items.every((p) => SECTION_KEYS.has(pairKey(p)) && isYamlMap(p.value));
}

function describeFieldValue(value: Node | null): { anchor: string | null; list: boolean } {
  if (!value) {
    return { anchor: null, list: false };
  }
  if (isAlias(value)) {
    return { anchor: value.source, list: false };
  }
  if (isYamlSeq(value)) {
    const first = value.items[0];
    if (first && isAlias(first)) {
      return { anchor: first.source, list: true };
    }
    return { anchor: null, list: true };
  }
  return { anchor: null, list: false };
}

function collectFieldsFromMap(
  moduleName: string,
  ownerKind: string,
  ownerName: string,
  ownerId: string,
  ownerLabel: string,
  fieldsRoot: YAMLMap,
  out: FieldEmit[],
): void {
  if (fieldMapIsSectioned(fieldsRoot)) {
    for (const sec of fieldsRoot.items) {
      const section = pairKey(sec);
      if (!isYamlMap(sec.value)) {
        continue;
      }
      for (const fp of sec.value.items) {
        const fname = pairKey(fp);
        const spec = describeFieldValue(fp.value);
        const path = `${section}.${fname}`;
        const id = fieldId(moduleName, ownerKind, ownerName, [section, fname]);
        out.push({
          id,
          ownerId,
          ownerLabel,
          name: fname,
          path,
          list: spec.list,
          refId: spec.anchor ? resolveRef(moduleName, spec.anchor).id : null,
          refLabel: spec.anchor ? resolveRef(moduleName, spec.anchor).label : null,
        });
      }
    }
    return;
  }

  for (const pair of fieldsRoot.items) {
    const k = pairKey(pair);
    const v = pair.value;
    if (k === "implements" || k === "contraints" || k === "constraints" || k === "is") {
      continue;
    }
    const spec = describeFieldValue(v);
    const path = k;
    const id = fieldId(moduleName, ownerKind, ownerName, [k]);
    out.push({
      id,
      ownerId,
      ownerLabel,
      name: k,
      path,
      list: spec.list,
      refId: spec.anchor ? resolveRef(moduleName, spec.anchor).id : null,
      refLabel: spec.anchor ? resolveRef(moduleName, spec.anchor).label : null,
    });
  }
}
//#endregion 🧩YamlHelpers

//#region 📝CypherEmit
function cypherLiteral(value: string | boolean): string {
  if (typeof value === "boolean") {
    return value ? "true" : "false";
  }
  return `'${value.replace(/\\/g, "\\\\").replace(/'/g, "\\'")}'`;
}

function renderPropMap(props: Record<string, string | boolean>): string {
  return `{ ${Object.entries(props)
    .map(([k, v]) => `${k}: ${cypherLiteral(v)}`)
    .join(", ")} }`;
}

/** Neo4j labels that collide with Cypher keywords use backticks. */
function labelToken(label: string): string {
  return ["interface", "class", "command"].includes(label) ? `\`${label}\`` : label;
}

function mergeNode(label: string, id: string, props: Record<string, string | boolean>): string[] {
  const t = labelToken(label);
  return [`MERGE (n:${t} { id: ${cypherLiteral(id)} })`, `SET n = ${renderPropMap({ id, ...props })};`, ""];
}

function mergeRel(fromId: string, rel: string, toId: string, fromLabel: string, toLabel: string): string[] {
  return [
    `MATCH (a:${labelToken(fromLabel)} { id: ${cypherLiteral(fromId)} }), (b:${labelToken(toLabel)} { id: ${cypherLiteral(toId)} })`,
    `MERGE (a)-[:${rel}]->(b);`,
    "",
  ];
}
//#endregion 📝CypherEmit

//#region 🏗️GraphBuild
type FieldEmit = {
  id: string;
  ownerId: string;
  ownerLabel: string;
  name: string;
  path: string;
  list: boolean;
  refId: string | null;
  refLabel: string | null;
};

function fieldId(moduleName: string, ownerKind: string, ownerName: string, pathParts: string[]): string {
  return `field:${moduleName}:${ownerKind}:${ownerName}:${pathParts.join(".")}`;
}

const anchorToRef = new Map<string, { label: string; id: string }>();

function registerAnchor(anchor: string, label: string, id: string): void {
  anchorToRef.set(anchor, { label, id });
}

function resolveRef(_moduleName: string, anchor: string): { label: string; id: string } {
  const hit = anchorToRef.get(anchor);
  if (!hit) {
    return { label: "interface", id: `interface:${anchor}` };
  }
  return hit;
}

function buildGraph(doc: Document): { lines: string[] } {
  anchorToRef.clear();
  const lines: string[] = [
    "// SPDX-License-Identifier: AGPL-3.0-only",
    "// Neo4j Cypher bundle for semio schema (generated from semio/client/schema/semio/schema.yaml).",
    "// Replay: load into database `semio` (see NEO4J_DATABASE). Bloom: pick database `semio`, auto-generate Perspective, search e.g. `kit` or `interface`.",
    "",
    "MATCH (n) WHERE n:module OR n:interface OR n:class OR n:field OR n:scalar OR n:command DETACH DELETE n;",
    "",
  ];

  const schema = doc.get("schema");
  if (!isYamlMap(schema)) {
    throw new Error("schema root must be a mapping");
  }

  // Scalars (general list — anchors on null scalars)
  const scalarSeq = doc.getIn(["schema", "general", "scalars"]);
  if (isYamlSeq(scalarSeq)) {
    for (const it of scalarSeq.items) {
      const anchor = (it as { anchor?: string }).anchor;
      if (typeof anchor === "string") {
        const id = `scalar:${anchor}`;
        registerAnchor(anchor, "scalar", id);
        lines.push(...mergeNode("scalar", id, { name: anchor }));
        lines.push(...mergeRel("module:general", "OWNS", id, "module", "scalar"));
      }
    }
  }

  // Modules
  for (const mod of MODULE_KEYS) {
    const mid = `module:${mod}`;
    lines.push(...mergeNode("module", mid, { name: mod }));
  }

  // Interfaces (general + domain)
  for (const mod of MODULE_KEYS) {
    const ifaceRoot = doc.getIn(["schema", mod, "interfaces"]);
    if (!isYamlMap(ifaceRoot)) {
      continue;
    }
    for (const pair of ifaceRoot.items) {
      const name = pairKey(pair);
      const body = pair.value;
      if (!isYamlMap(body)) {
        continue;
      }
      const label = name === "command" ? "command" : "interface";
      const id = name === "command" ? "command:command" : `interface:${name}`;
      registerAnchor(name, label, id);
      lines.push(...mergeNode(label, id, { name }));
      lines.push(...mergeRel(`module:${mod}`, "OWNS", id, "module", label));
    }
  }

  // Classes (domain only in current schema)
  const classRoot = doc.getIn(["schema", "domain", "classes"]);
  if (isYamlMap(classRoot)) {
    for (const pair of classRoot.items) {
      const name = pairKey(pair);
      const body = pair.value;
      if (!isYamlMap(body)) {
        continue;
      }
      const id = `class:${name}`;
      registerAnchor(name, "class", id);
      lines.push(...mergeNode("class", id, { name }));
      lines.push(...mergeRel("module:domain", "OWNS", id, "module", "class"));
    }
  }

  // EXTENDS from implements
  for (const mod of MODULE_KEYS) {
    const ifaceRoot = doc.getIn(["schema", mod, "interfaces"]);
    if (!isYamlMap(ifaceRoot)) {
      continue;
    }
    for (const pair of ifaceRoot.items) {
      const name = pairKey(pair);
      const body = pair.value;
      if (!isYamlMap(body)) {
        continue;
      }
      const label = name === "command" ? "command" : "interface";
      const id = name === "command" ? "command:command" : `interface:${name}`;
      const impl = body.get("implements");
      if (isYamlSeq(impl)) {
        for (const it of impl.items) {
          if (isAlias(it)) {
            const parent = resolveRef(mod, it.source);
            lines.push(...mergeRel(id, "EXTENDS", parent.id, label, parent.label));
          }
        }
      }
    }
  }

  if (isYamlMap(classRoot)) {
    for (const pair of classRoot.items) {
      const name = pairKey(pair);
      const body = pair.value;
      if (!isYamlMap(body)) {
        continue;
      }
      const id = `class:${name}`;
      const impl = body.get("implements");
      if (isYamlSeq(impl)) {
        for (const it of impl.items) {
          if (isAlias(it)) {
            const parent = resolveRef("domain", it.source);
            lines.push(...mergeRel(id, "EXTENDS", parent.id, "class", parent.label));
          }
        }
      }
    }
  }

  // Fields
  const fieldRows: FieldEmit[] = [];
  for (const mod of MODULE_KEYS) {
    const ifaceRoot = doc.getIn(["schema", mod, "interfaces"]);
    if (isYamlMap(ifaceRoot)) {
      for (const pair of ifaceRoot.items) {
        const name = pairKey(pair);
        const body = pair.value;
        if (!isYamlMap(body)) {
          continue;
        }
        const label = name === "command" ? "command" : "interface";
        const id = name === "command" ? "command:command" : `interface:${name}`;
        const fm = body.get("fields");
        if (isYamlMap(fm)) {
          collectFieldsFromMap(mod, label, name, id, label, fm, fieldRows);
        }
      }
    }
  }

  if (isYamlMap(classRoot)) {
    for (const pair of classRoot.items) {
      const name = pairKey(pair);
      const body = pair.value;
      if (!isYamlMap(body)) {
        continue;
      }
      const id = `class:${name}`;
      const fm = body.get("fields");
      if (isYamlMap(fm)) {
        collectFieldsFromMap("domain", "class", name, id, "class", fm, fieldRows);
      }
    }
  }

  const modForOwner = (ownerId: string): string => {
    const m = ownerId.match(/^field:(\w+):/);
    return m?.[1] ?? "domain";
  };

  for (const f of fieldRows) {
    const props: Record<string, string | boolean> = {
      name: f.name,
      path: f.path,
      list: f.list,
    };
    lines.push(...mergeNode("field", f.id, props));
    const mod = modForOwner(f.id);
    lines.push(...mergeRel(`module:${mod}`, "OWNS", f.id, "module", "field"));
    lines.push(...mergeRel(f.ownerId, "HAS_FIELD", f.id, f.ownerLabel, "field"));
    if (f.refId && f.refLabel) {
      lines.push(...mergeRel(f.id, "REFERENCES", f.refId, "field", f.refLabel));
    }
  }

  // Bloom-friendly indexes on lowercase labels
  lines.push(
    "// Bloom / Explore: indexes on names and ids",
    "CREATE INDEX bloom_field_path IF NOT EXISTS FOR (n:field) ON (n.path);",
    "CREATE INDEX bloom_field_name IF NOT EXISTS FOR (n:field) ON (n.name);",
    "CREATE INDEX bloom_class_name IF NOT EXISTS FOR (n:`class`) ON (n.name);",
    "CREATE INDEX bloom_interface_name IF NOT EXISTS FOR (n:`interface`) ON (n.name);",
    "CREATE INDEX bloom_scalar_name IF NOT EXISTS FOR (n:scalar) ON (n.name);",
    "CREATE INDEX bloom_command_name IF NOT EXISTS FOR (n:`command`) ON (n.name);",
    "",
  );

  return { lines };
}
//#endregion 🏗️GraphBuild

//#region 🚀Runtime
function resolveCypherShell(): string | null {
  const runtimeName = process.platform === "win32" ? "cypher-shell.bat" : "cypher-shell";
  const cachedShell = join(REPO_ROOT, ".repo", "cache", "neo4j", `neo4j-community-${NEO4J_VERSION}`, "bin", runtimeName);
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

function applyCypherFile(outputPath: string): void {
  const cypherShell = resolveCypherShell();
  if (!cypherShell) {
    console.warn(`[generate] cypher-shell not found; wrote ${OUTPUT_FILE} only.`);
    return;
  }

  const cypherInput = readFileSync(outputPath, "utf8");
  const result = spawnSync(
    cypherShell,
    [
      "-a",
      process.env.NEO4J_URI || "bolt://localhost:7687",
      "-u",
      process.env.NEO4J_USERNAME || "neo4j",
      "-p",
      process.env.NEO4J_PASSWORD || "password",
      "-d",
      process.env.NEO4J_DATABASE || "semio",
      "--format",
      "plain",
    ],
    {
      cwd: REPO_ROOT,
      input: cypherInput,
      stdio: "inherit",
      env: buildCypherEnv(),
    },
  );

  if (result.status !== 0) {
    throw new Error(`cypher-shell failed with exit code ${result.status ?? 1}`);
  }
}

function main(): void {
  const raw = readFileSync(SCHEMA_YAML, "utf8");
  const doc = parseDocument(raw);
  if (doc.errors.length) {
    throw new Error(`YAML parse errors: ${doc.errors.map((e) => e.message).join("; ")}`);
  }

  const { lines } = buildGraph(doc);
  mkdirSync(join(REPO_ROOT, ".repo", "🛂"), { recursive: true });
  writeFileSync(OUTPUT_FILE, `${lines.join("\n")}\n`, "utf8");
  console.log(`[generate] wrote ${OUTPUT_FILE} (${lines.length} lines) from ${SCHEMA_YAML}.`);

  if (process.argv.includes("apply")) {
    applyCypherFile(OUTPUT_FILE);
    console.log("[generate] applied to Neo4j via cypher-shell.");
  }
}

main();
//#endregion 🚀Runtime
