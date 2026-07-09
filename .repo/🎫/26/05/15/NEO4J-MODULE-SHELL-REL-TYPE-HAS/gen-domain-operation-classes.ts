#!/usr/bin/env bun
/**
 * @emoji 🧩 Splices concrete `Operation` commands from `schema.golden.graphql` into `compose/client/schema/compose/schema.yaml` (camelCase keys, `implements: [*operation]`, empty `fields`); emits Neo4j `Command` merge/reparent/rename + `Data` argument kit from golden `*Input` types (imperative command names only).
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { spawnSync } from "node:child_process";

const REPO = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const goldenPath = join(REPO, "compose", "client", "schema", "graphql", "schema.golden.graphql");
const schemaPath = join(REPO, "compose", "client", "schema", "compose", "schema.yaml");
const goldenRepoRel = relative(REPO, goldenPath).replaceAll("\\", "/");

function toYamlKey(pascal: string): string {
  return pascal.length === 0 ? pascal : pascal.charAt(0).toLowerCase() + pascal.slice(1);
}

/** @emoji 🧭 Past-participle operation stem → imperative stem (must match rename-operations-imperative.ts). */
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

function escapeRe(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** @emoji 🧭 Golden `type Op implements Operation` carries `input: OpInput!` only when the operation has a typed argument bag. */
function operationUsesSpecificInputType(golden: string, pastOp: string): boolean {
  const re = new RegExp(`type ${escapeRe(pastOp)} implements Operation[\\s\\S]*?\\binput:\\s*${escapeRe(pastOp)}Input!`, "m");
  return re.test(golden);
}

type ArgField = { name: string; kind: "data" | "reference" | "computed"; typeStr: string; isList: boolean; coreType: string };

function stripGraphqlWrappers(typeLine: string): { core: string; isList: boolean } {
  let t = typeLine.trim();
  let isList = false;
  if (t.endsWith("!")) {
    t = t.slice(0, -1).trim();
  }
  if (t.endsWith("Connection")) {
    isList = true;
    t = t.replace(/Connection$/, "").trim();
  }
  if (t.startsWith("[") && t.endsWith("]")) {
    isList = true;
    let inner = t.slice(1, -1).trim();
    if (inner.endsWith("!")) {
      inner = inner.slice(0, -1).trim();
    }
    t = inner;
  }
  return { core: t, isList };
}

/** @emoji 🧭 Reads `# Arguments` section of `type ${pastOp}Input` in golden SDL. */
function extractOperationInputFields(golden: string, pastOp: string): ArgField[] {
  const inputType = `${pastOp}Input`;
  const re = new RegExp(`type ${escapeRe(inputType)} implements[^{]+\\{([\\s\\S]*?)\\n\\}`, "m");
  const m = golden.match(re);
  if (!m?.[1]) {
    return [];
  }
  const body = m[1];
  const ix = body.indexOf("# Arguments");
  if (ix === -1) {
    return [];
  }
  const tail = body.slice(ix);
  const lines = tail.split(/\r?\n/);
  const out: ArgField[] = [];
  for (const raw of lines.slice(1)) {
    const line = raw.trim();
    if (line === "" || line.startsWith("#")) {
      continue;
    }
    if (line === "}" || line.startsWith("}")) {
      break;
    }
    const nm = /^(\w+)\s*:\s*(.+)$/.exec(line);
    if (!nm) {
      continue;
    }
    const name = nm[1]!;
    let rhs = nm[2]!.trim();
    let kind: ArgField["kind"] = "data";
    const km = rhs.match(/#\s*(data|reference|computed)\b/);
    if (km?.[1]) {
      kind = km[1] as ArgField["kind"];
    }
    rhs = rhs.split("#")[0]!.trim();
    if (!rhs.endsWith("!")) {
      continue;
    }
    const { core, isList } = stripGraphqlWrappers(rhs);
    if (!core || core.includes("[[")) {
      continue;
    }
    out.push({ name, kind, typeStr: rhs, isList, coreType: core });
  }
  return out;
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

/** @emoji 🧭 Owning kit `Class` / `Interface` for each concrete `Command` (imperative names). */
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

function cypherIdent(cmd: string, field: string): string {
  return `arg_${cmd}_${field}`.replace(/[^A-Za-z0-9_]/g, "_");
}

/** @emoji 🧭 Neo4j `Data` argument kit: one node per `(Command.name, field.name)` via `soleOwnerKey` + detach/rewire so kit `Data` is never shared across commands. */
function buildCommandArgumentDataCypher(golden: string, pastOps: readonly string[]): string {
  const esc = (x: string) => x.replace(/'/g, "\\'");
  const lines: string[] = [
    "// Generated — remove legacy `Input` / orphan `input` references; imperative `Command` argument `Data` uses `soleOwnerKey` (no cross-command sharing).",
    "MATCH (inp:Input)-[:OWNS]->(ch)",
    "DETACH DELETE ch;",
    "MATCH (inp:Input)",
    "DETACH DELETE inp;",
    "MATCH (r:Reference)",
    "WHERE toLower(r.name) = 'input' AND NOT ()-[:OWNS]->(r)",
    "DETACH DELETE r;",
    "",
  ];
  const cmdsByCmdFields = new Map<string, ArgField[]>();
  for (const past of pastOps) {
    const cmd = imperativeOperationStem(past);
    if (!operationUsesSpecificInputType(golden, past)) {
      continue;
    }
    const fields = extractOperationInputFields(golden, past);
    if (fields.length === 0) {
      continue;
    }
    const prev = cmdsByCmdFields.get(cmd);
    if (prev == null) {
      cmdsByCmdFields.set(cmd, [...fields]);
    } else {
      const merged = [...prev];
      for (const f of fields) {
        if (!merged.some((x) => x.name === f.name)) {
          merged.push(f);
        }
      }
      cmdsByCmdFields.set(cmd, merged);
    }
  }
  for (const [cmd, fields] of cmdsByCmdFields) {
    const inList = fields.map((f) => `'${esc(f.name)}'`).join(", ");
    lines.push(`MATCH (cmd:Command {name: '${esc(cmd)}'})`, `OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)`, `WHERE pivot.name IN [${inList}]`, `DELETE r;`, "");
  }
  lines.push(
    "// Drop detached kit `Data` (no `OWNS` parent) that still carry `IS` — stale shared argument rows after `soleOwnerKey` split.",
    "MATCH (d:Data)",
    "WHERE NOT ()-[:OWNS]->(d) AND EXISTS { (d)-[:IS]->(:Class|Interface|Scalar|Command) }",
    "DETACH DELETE d;",
    "",
  );
  for (const [cmd, fields] of cmdsByCmdFields) {
    for (let i = 0; i < fields.length; i++) {
      const f = fields[i]!;
      const id = cypherIdent(cmd, f.name);
      const isList = f.isList ? "true" : "false";
      const core = esc(f.coreType);
      lines.push(
        `MATCH (cmd:Command {name: '${esc(cmd)}'})`,
        `MERGE (${id}:Data {name: '${esc(f.name)}', soleOwnerKey: '${esc(cmd)}', rank: '${i}', isList: ${isList}})`,
        `MERGE (cmd)-[:OWNS]->(${id})`,
        `WITH ${id}`,
        `OPTIONAL MATCH (t)`,
        `WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = '${core}'`,
        `WITH ${id}, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END`,
        `LIMIT 1`,
        `FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (${id})-[:IS]->(t));`,
        "",
      );
    }
  }
  return lines.join("\n");
}

const golden = readFileSync(goldenPath, "utf8");
const pastOps = [...new Set(collectOperationNames(golden))].sort((a, b) => a.localeCompare(b));
const uniqueImperative = [...new Set(pastOps.map(imperativeOperationStem))].sort((a, b) => a.localeCompare(b));
const opFieldBlock = `        implements: [*operation]
        fields:
          computed:
            modification: *modification
          reference:
            scope: *artifact`;
const block = uniqueImperative.map((p) => `      ${toYamlKey(p)}:\n${opFieldBlock}`).join("\n");

let s = readFileSync(schemaPath, "utf8");
const before = s;
s = s.replace(/      command: &command\n        implements: \[\*weakEntity]\n        fields:\n          computed:\n            response: \*response\n/, "");
s = s.replace(/        operation: &operation\n          implements: \[\*strongEntity]/, "        operation: &operation\n          implements: [*response, *strongEntity]");
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
console.log(`[gen-ops-yaml] wrote ${schemaPath} (+${uniqueImperative.length} imperative operation commands)`);

const headDiskPairs = renamePairsHeadToDisk(golden);
const imperativePairs = pastOps
  .map((from) => ({ from, to: imperativeOperationStem(from) }))
  .filter((p) => p.from !== p.to)
  .sort((a, b) => a.from.localeCompare(b.from));

const renameUnwindBody =
  imperativePairs.length === 0
    ? "// (no past→imperative command renames; skip name sync)"
    : ["UNWIND [", imperativePairs.map((r) => `{from: ${JSON.stringify(r.from)}, to: ${JSON.stringify(r.to)}}`).join(", "), "] AS row", "MATCH (c:Command {name: row.from})", "SET c.name = row.to;"].join("\n");

const headDiskNote =
  headDiskPairs.length === 0
    ? ""
    : [
        "",
        "// HEAD→disk golden drift (supplementary renames when both stems already imperative-deduped):",
        "UNWIND [",
        headDiskPairs.map((r) => `{from: ${JSON.stringify(r.from)}, to: ${JSON.stringify(r.to)}}`).join(", "),
        "] AS row",
        "MATCH (c:Command {name: row.from})",
        "SET c.name = row.to;",
      ].join("\n");

const relabelAndRenameLines = [
  "// Generated — relabel legacy `Class` operation kit nodes to `Command`, then sync names to imperative verbs (deduped).",
  "MATCH (c:Class)-[:IS]->(op:Interface|Class)",
  "WHERE toLower(op.name) = 'operation'",
  "REMOVE c:Class",
  "SET c:Command;",
  renameUnwindBody,
  headDiskNote,
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
    `UNWIND [${uniqueImperative.map((n) => JSON.stringify(n)).join(", ")}] AS opName`,
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
const rows = uniqueImperative.map((op) => ({ op, own: ownerForOperation(op) }));
writeFileSync(
  reparentOut,
  [
    "// Generated — each `Command` hangs under `Module(domainName)-[:OWNS]->Module(operation)-[:HAS]->command` (domain `Module` row must exist; kit `Class`/`Interface` must not OWNS the operation folder).",
    `UNWIND [${rows.map((r) => `{op: '${r.op}', own: '${r.own}'}`).join(", ")}] AS row`,
    "MATCH (c:Command {name: row.op})",
    "MATCH (folder:Module {name: row.own})",
    "OPTIONAL MATCH (c)<-[h:HAS]-(:Module {name: 'operation'})",
    "DELETE h",
    "MERGE (folder)-[:OWNS]->(m:Module {name: 'operation'})",
    "MERGE (m)-[:HAS]->(c)",
    "WITH c, m",
    "OPTIONAL MATCH (p)-[r:OWNS]->(c)",
    "WHERE id(p) <> id(m)",
    "DELETE r;",
    "",
  ].join("\n"),
  "utf8",
);
console.log(`[gen-ops-yaml] wrote ${reparentOut}`);

const inputSurfaceOut = join(import.meta.dir, "merge-command-input-surfaces.cypher.fragment");
writeFileSync(inputSurfaceOut, buildCommandArgumentDataCypher(golden, pastOps), "utf8");
console.log(`[gen-ops-yaml] wrote ${inputSurfaceOut}`);
