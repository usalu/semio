#!/usr/bin/env bun
/**
 * @emoji 🧩 Splices concrete `Operation` commands from `schema.golden.graphql` into `semio/client/schema/semio/schema.yaml` (camelCase keys, `implements: [*operation]`, empty `fields`); emits Neo4j `Command` merge/reparent/name-sync fragments.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { spawnSync } from "node:child_process";

const REPO = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const goldenPath = join(REPO, "semio", "client", "schema", "graphql", "schema.golden.graphql");
const schemaPath = join(REPO, "semio", "client", "schema", "semio", "schema.yaml");
const goldenRepoRel = relative(REPO, goldenPath).replaceAll("\\", "/");

function toYamlKey(pascal: string): string {
  return pascal.length === 0 ? pascal : pascal.charAt(0).toLowerCase() + pascal.slice(1);
}

/** @emoji 🧭 Past-participle operation stem → imperative stem (must match rename-operations-imperative.script.ts). */
function imperativeOperationStem(past: string): string {
  const rules: [RegExp, string][] = [
    [/^Created/, "Create"],
    [/^Renamed/, "Rename"],
    [/^Updated/, "Update"],
    [/^Added/, "Add"],
    [/^Removed/, "Remove"],
    [/^Deleted/, "Delete"],
    [/^Changed/, "Change"],
    [/^Moved/, "Move"],
    [/^Fixed/, "Fix"],
    [/^Dragged/, "Drag"],
    [/^Flattened/, "Flatten"],
  ];
  for (const [re, rep] of rules) {
    if (re.test(past)) {
      return past.replace(re, rep);
    }
  }
  return past;
}

function collectOperationNames(golden: string): string[] {
  const re = /^type ([A-Za-z0-9_]+) implements Operation/gm;
  const names: string[] = [];
  for (const m of golden.matchAll(re)) {
    names.push(m[1]!);
  }
  return names;
}

/** @emoji 🧭 Pairs `HEAD` golden name → working-tree name when both share the same imperative stem (migration rename on existing DB). */
function renamePairsHeadToDisk(diskGolden: string): { from: string; to: string }[] {
  const git = spawnSync("git", ["show", `HEAD:${goldenRepoRel}`], { cwd: REPO, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (git.status !== 0 || !git.stdout?.trim()) {
    return [];
  }
  const headOps = collectOperationNames(git.stdout);
  const diskOps = collectOperationNames(diskGolden);
  const byStem = new Map<string, { head?: string; disk?: string }>();
  for (const h of headOps) {
    const k = imperativeOperationStem(h);
    const cur = byStem.get(k) ?? {};
    cur.head = h;
    byStem.set(k, cur);
  }
  for (const d of diskOps) {
    const k = imperativeOperationStem(d);
    const cur = byStem.get(k) ?? {};
    cur.disk = d;
    byStem.set(k, cur);
  }
  const rows: { from: string; to: string }[] = [];
  for (const v of byStem.values()) {
    if (v.head != null && v.disk != null && v.head !== v.disk) {
      rows.push({ from: v.head, to: v.disk });
    }
  }
  rows.sort((a, b) => a.from.localeCompare(b.from));
  return rows;
}

/** @emoji 🧭 Owning kit `Class` / `Interface` for each concrete `Operation` subtype (matches golden names). */
function ownerForOperation(op: string): string {
  if (op === "ChangeDescription") {
    return "Workspace";
  }
  if (op === "FlattenDesign") {
    return "Design";
  }
  if (op === "RenameKit") {
    return "Kit";
  }
  if (op.includes("Piece")) {
    return "Piece";
  }
  if (op.includes("Qualit")) {
    return "Quality";
  }
  if (op.includes("Connector")) {
    return "Connector";
  }
  if (op.includes("Concept")) {
    return "Concept";
  }
  if (op.includes("Design")) {
    return "Design";
  }
  if (op.includes("Port")) {
    return "Port";
  }
  if (op.includes("Type")) {
    return "Type";
  }
  if (op.includes("Tag")) {
    return "Tag";
  }
  if (op.includes("Kit")) {
    return "Kit";
  }
  return "Workspace";
}

const golden = readFileSync(goldenPath, "utf8");
const sorted = [...new Set(collectOperationNames(golden))].sort((a, b) => a.localeCompare(b));
const opFieldBlock = `        implements: [*operation]
        fields:
          computed:
            modification: *modification
          reference:
            scope: *artifact`;
const block = sorted.map((p) => `      ${toYamlKey(p)}:\n${opFieldBlock}`).join("\n");

let s = readFileSync(schemaPath, "utf8");
const before = s;
s = s.replace(
  /      command: &command\n        implements: \[\*weakEntity]\n        fields:\n          computed:\n            response: \*response\n/,
  "",
);
s = s.replace(
  /        operation: &operation\n          implements: \[\*strongEntity]/,
  "        operation: &operation\n          implements: [*response, *strongEntity]",
);
const startMarker = "          startedAt: *timestamp\n";
const endMarker = "\n    vcs: &vcs";
const si = s.indexOf(startMarker);
const ei = s.indexOf(endMarker);
if (si === -1 || ei === -1 || ei < si) {
  throw new Error("[gen-ops-yaml] splice markers missing (session startedAt / vcs boundary)");
}
s = `${s.slice(0, si + startMarker.length)}\n${block}${s.slice(ei)}`;
if (s === before) {
  console.log("[gen-ops-yaml] note: no net yaml change (command strip / implements / block already applied)");
}
writeFileSync(schemaPath, s, "utf8");
console.log(`[gen-ops-yaml] wrote ${schemaPath} (+${sorted.length} operation commands)`);

const renameRows = renamePairsHeadToDisk(golden);

const relabelAndRenameLines = [
  "// Generated — relabel legacy `Class` operation kit nodes to `Command`, then rename when `HEAD` golden differed.",
  "MATCH (c:Class)-[:IS]->(op:Interface|Class)",
  "WHERE toLower(op.name) = 'operation'",
  "REMOVE c:Class",
  "SET c:Command;",
  renameRows.length === 0
    ? "// (no HEAD→disk operation renames; skip name sync)"
    : [
        "UNWIND [",
        renameRows.map((r) => `{from: ${JSON.stringify(r.from)}, to: ${JSON.stringify(r.to)}}`).join(", "),
        "] AS row",
        "MATCH (c:Command {name: row.from})",
        "SET c.name = row.to;",
      ].join("\n"),
  "",
].join("\n");

const cypherOut = join(import.meta.dir, "relabel-rename-operation-commands.cypher.fragment");
writeFileSync(cypherOut, relabelAndRenameLines, "utf8");
console.log(`[gen-ops-yaml] wrote ${cypherOut}`);

const mergeOut = join(import.meta.dir, "merge-operation-classes.cypher.fragment");
writeFileSync(
  mergeOut,
  [
    "// Generated — paste after relabel-rename fragment inside migrations.cypher region MergeOperationConcreteCommands.",
    `UNWIND [${sorted.map((n) => JSON.stringify(n)).join(", ")}] AS opName`,
    "MERGE (c:Command {name: opName})",
    "WITH c",
    "MATCH (op:Interface|Class)",
    "WHERE toLower(op.name) = 'operation'",
    "WITH c, op",
    "ORDER BY id(op) ASC",
    "LIMIT 1",
    "MERGE (c)-[:IS]->(op);",
    "",
  ].join("\n"),
  "utf8",
);
console.log(`[gen-ops-yaml] wrote ${mergeOut}`);

const reparentOut = join(import.meta.dir, "reparent-operation-ownership.cypher.fragment");
const rows = sorted.map((op) => ({ op, own: ownerForOperation(op) }));
writeFileSync(
  reparentOut,
  [
    "// Generated — each concrete operation `Command` hangs under `owner-[:OWNS]->Module(operation)-[:OWNS]->command`.",
    `UNWIND [${rows.map((r) => `{op: '${r.op}', own: '${r.own}'}`).join(", ")}] AS row`,
    "MATCH (c:Command {name: row.op})",
    "MATCH (own:Class|Interface {name: row.own})",
    "MERGE (own)-[:OWNS]->(m:Module {name: 'operation'})",
    "MERGE (m)-[:OWNS]->(c)",
    "WITH c, m",
    "OPTIONAL MATCH (p:Module)-[r:OWNS]->(c)",
    "WHERE id(p) <> id(m)",
    "DELETE r;",
    "",
  ].join("\n"),
  "utf8",
);
console.log(`[gen-ops-yaml] wrote ${reparentOut}`);

const inputSurfaceLines = [
  "// Generated — one `Input` kit node per `Command` (same `name`); `data.input` reference lives under the `Input` surface.",
  `UNWIND [${sorted.map((n) => JSON.stringify(n)).join(", ")}] AS opName`,
  "MATCH (cmd:Command {name: opName})",
  "MERGE (inp:Input {name: opName})",
  "MERGE (cmd)-[:OWNS]->(inp)",
  "WITH inp",
  "MATCH (iface:Interface)",
  "WHERE toLower(iface.name) = 'input'",
  "WITH inp, iface",
  "ORDER BY id(iface) ASC",
  "LIMIT 1",
  "MERGE (inp)-[:IS]->(iface);",
  "",
  `UNWIND [${sorted.map((n) => JSON.stringify(n)).join(", ")}] AS opName`,
  "MATCH (cmd:Command {name: opName})-[:OWNS]->(inp:Input {name: opName})",
  "OPTIONAL MATCH (cmd)-[rx:OWNS]->(f:Data|Derived|Reference)",
  "WHERE toLower(f.name) = 'input'",
  "DELETE rx",
  "WITH inp, f",
  "WHERE f IS NOT NULL",
  "MERGE (inp)-[:OWNS]->(f);",
  "",
  `UNWIND [${sorted.map((n) => JSON.stringify(n)).join(", ")}] AS opName`,
  "MATCH (cmd:Command {name: opName})-[:OWNS]->(inp:Input {name: opName})",
  "WHERE NOT (inp)-[:OWNS]->(:Reference {name: 'input'})",
  "CREATE (r:Reference {name: 'input', rank: '', isList: false})",
  "MERGE (inp)-[:OWNS]->(r);",
  "",
  "MATCH (inp:Input)-[:OWNS]->(r:Reference {name: 'input'})",
  "MATCH (iface:Interface)",
  "WHERE toLower(iface.name) = 'input'",
  "WITH r, iface",
  "ORDER BY id(iface) ASC",
  "LIMIT 1",
  "MERGE (r)-[:IS]->(iface);",
  "",
].join("\n");
const inputSurfaceOut = join(import.meta.dir, "merge-command-input-surfaces.cypher.fragment");
writeFileSync(inputSurfaceOut, inputSurfaceLines, "utf8");
console.log(`[gen-ops-yaml] wrote ${inputSurfaceOut}`);
