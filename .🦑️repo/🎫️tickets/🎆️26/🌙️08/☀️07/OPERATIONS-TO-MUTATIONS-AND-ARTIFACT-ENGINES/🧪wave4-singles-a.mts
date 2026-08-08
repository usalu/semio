#!/usr/bin/env bun
/**
 * Wave 4 bulk rename helper for plugin crates (run from repo root).
 * Usage: bun .🦑️repo/.../🧪wave4-singles-a.mts <plugin-dir-relative-to-s-repo>
 */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const pluginRoot = process.argv[2];
if (!pluginRoot) {
  console.error("usage: wave4-singles-a.mts <path-under-✏️s/🔌️plugins/...>");
  process.exit(1);
}

const repoRoot = join(import.meta.dir, "../../../../../..");
const abs = join(repoRoot, pluginRoot);
const walk = (dir: string, out: string[] = []) => {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, ent.name);
    if (ent.isDirectory()) walk(p, out);
    else if (ent.name.endsWith(".rs") || ent.name.endsWith(".ts") || ent.name.endsWith(".semio")) out.push(p);
  }
  return out;
};

const replacements: [RegExp, string][] = [
  [/(\w+)Operation/g, "$1Mutation"],
  [/document_operations/g, "document_mutations"],
  [/config_operations/g, "config_mutations"],
  [/draft_operations/g, "draft_mutations"],
  [/Emit::operations/g, "Emit::mutations"],
  [/operations: vec/g, "mutations: vec"],
  [/type Operation =/g, "type Mutation ="],
  [/type ConfigOperation =/g, "type ConfigMutation ="],
  [/type DraftOperation =/g, "type DraftMutation ="],
  [/protocol::OperationDiff/g, "protocol::MutationDiff"],
  [/use protocol::Operation;/g, "use protocol::Mutation;"],
  [/impl Operation</g, "impl Mutation<"],
  [/fn backwards\(/g, "fn inverse("],
  [/NoDraftOperation/g, "NoDraftMutation"],
  [/result\.operations/g, "result.document_mutations"],
  [/start operation/g, "start mutation"],
  [/schema ([\w.]+)\.operation/g, "schema $1.mutation"],
  [/serde\(tag = "operation"/g, 'serde(tag = "mutation"'],
];

for (const file of walk(abs)) {
  let text = readFileSync(file, "utf8");
  let next = text;
  for (const [re, rep] of replacements) next = next.replace(re, rep);
  if (next !== text) writeFileSync(file, next);
}
console.log("renamed under", abs);
