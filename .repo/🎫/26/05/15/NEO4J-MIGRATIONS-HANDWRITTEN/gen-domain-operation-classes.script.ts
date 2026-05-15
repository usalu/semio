#!/usr/bin/env bun
/**
 * @emoji 🧩 Splices concrete `Operation` classes from `schema.golden.graphql` into `semio/client/schema/semio/schema.yaml` (camelCase keys, `implements: [*operation]`, empty `fields`).
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const goldenPath = join(REPO, "semio", "client", "schema", "graphql", "schema.golden.graphql");
const schemaPath = join(REPO, "semio", "client", "schema", "semio", "schema.yaml");

function toYamlKey(pascal: string): string {
  return pascal.length === 0 ? pascal : pascal.charAt(0).toLowerCase() + pascal.slice(1);
}

/** @emoji 🧭 Owning kit `Class` / `Interface` for each concrete `Operation` subtype (matches golden names). */
function ownerForOperation(op: string): string {
  if (op === "ChangedDescription") {
    return "Workspace";
  }
  if (op === "FlattenedDesign") {
    return "Design";
  }
  if (op === "RenamedKit") {
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
const re = /^type ([A-Za-z0-9_]+) implements Operation/gm;
const names = new Set<string>();
for (const m of golden.matchAll(re)) {
  names.add(m[1]!);
}
const sorted = [...names].sort((a, b) => a.localeCompare(b));
const block = sorted.map((p) => `      ${toYamlKey(p)}:\n        implements: [*operation]\n        fields: {}`).join("\n");

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
const anchor = "          startedAt: *timestamp\n    vcs: &vcs";
if (!s.includes("addedAttributesToConcept:")) {
  if (!s.includes(anchor)) {
    throw new Error("[gen-ops-yaml] splice anchor missing (session / vcs boundary)");
  }
  s = s.replace(anchor, `          startedAt: *timestamp\n${block}\n    vcs: &vcs`);
} else {
  console.log("[gen-ops-yaml] operation classes already present; skipping block insert");
}
if (s === before) {
  console.log("[gen-ops-yaml] note: no net yaml change (command strip / implements / block already applied)");
}
writeFileSync(schemaPath, s, "utf8");
console.log(`[gen-ops-yaml] wrote ${schemaPath} (+${sorted.length} operation classes)`);

const cypherOut = join(import.meta.dir, "merge-operation-classes.cypher.fragment");
writeFileSync(
  cypherOut,
  [
    "// Generated next to gen-domain-operation-classes.script.ts — paste into migrations.cypher region MergeOperationConcreteClasses.",
    `UNWIND [${sorted.map((n) => JSON.stringify(n)).join(", ")}] AS opName`,
    "MERGE (c:Class {name: opName})",
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
console.log(`[gen-ops-yaml] wrote ${cypherOut}`);

const reparentOut = join(import.meta.dir, "reparent-operation-ownership.cypher.fragment");
const rows = sorted.map((op) => ({ op, own: ownerForOperation(op) }));
writeFileSync(
  reparentOut,
  [
    "// Generated — each concrete operation `Class` hangs under `owner-[:OWNS]->Module(operation)-[:OWNS]->class`.",
    `UNWIND [${rows.map((r) => `{op: '${r.op}', own: '${r.own}'}`).join(", ")}] AS row`,
    "MATCH (c:Class {name: row.op})",
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
