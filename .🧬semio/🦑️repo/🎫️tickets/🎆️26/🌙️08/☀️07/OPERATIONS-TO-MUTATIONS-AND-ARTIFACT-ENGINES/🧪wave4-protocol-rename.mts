#!/usr/bin/env bun
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = join(import.meta.dir, "../../../../../..");
const plugins = [
  "✏️s/🔌️plugins/🗒️note",
  "✏️s/🔌️plugins/🖨️raster",
  "✏️s/🔌️plugins/🕸️dag",
  "✏️s/🔌️plugins/🎬️sequence",
  "✏️s/🔌️plugins/🎞️animate",
  "✏️s/🔌️plugins/📜️imperative",
  "✏️s/🔌️plugins/📏️layout",
  "✏️s/🔌️plugins/📋️forms",
  "✏️s/🔌️plugins/🌊️flow",
];

const pairs: [string, string][] = [
  ["protocol::Operation<", "protocol::Mutation<"],
  ["impl Operation<", "impl Mutation<"],
  ["impl OperationDiff", "impl MutationDiff"],
  ["protocol::OperationDiff", "protocol::MutationDiff"],
  ["invert_collection_operation", "inverse_collection_mutation"],
  ["apply_collection_operation", "apply_collection_mutation"],
  ["collection_diff_from_operation", "collection_diff_from_mutation"],
  ["apply_layout_operation", "apply_layout_mutation"],
  ["inverse_layout_operation", "inverse_layout_mutation"],
];

function walk(dir: string, out: string[]) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else if (name.endsWith(".rs")) out.push(p);
  }
}

for (const rel of plugins) {
  const root = join(repo, rel);
  const files: string[] = [];
  walk(root, files);
  for (const f of files) {
    let s = readFileSync(f, "utf8");
    let n = s;
    for (const [a, b] of pairs) n = n.replaceAll(a, b);
    if (n !== s) writeFileSync(f, n);
  }
}
console.log("protocol rename ok");
