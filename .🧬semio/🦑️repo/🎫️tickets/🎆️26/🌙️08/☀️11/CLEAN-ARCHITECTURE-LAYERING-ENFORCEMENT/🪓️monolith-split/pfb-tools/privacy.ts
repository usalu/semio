// 🔒️ Fixpoint check of the visibility widening on the FINAL files: a private item, method or field of a child that another file of the module tree still names is a violation.
// usage: bun privacy.ts <root file> <child file>... [--descendant <file>]...
import {readFileSync} from "node:fs";
import {identifiers, widenChunk} from "./widen.ts";

const args = process.argv.slice(2);
const descendants: string[] = [];
const files: string[] = [];
for (let i = 0; i < args.length; i += 1) {
  if (args[i] === "--descendant") { descendants.push(args[i + 1]!); i += 1; } else files.push(args[i]!);
}
const [root, ...children] = files;
const texts = new Map(files.map(f => [f, readFileSync(f, "utf8")]));
const extra = descendants.map(f => identifiers(readFileSync(f, "utf8")));
let violations = 0;
for (const child of children) {
  const outside = new Set<string>(identifiers(texts.get(root!)!));
  for (const other of children) if (other !== child) for (const id of identifiers(texts.get(other)!)) outside.add(id);
  for (const ids of extra) for (const id of ids) outside.add(id);
  const report: string[] = [];
  widenChunk(texts.get(child)!, outside, report, child.split("/").slice(-2, -1)[0]!);
  for (const line of report) { violations += 1; console.log("VIOLATION", line); }
}
console.log(`files ${files.length}, descendants ${descendants.length}, violations ${violations}`);
process.exit(violations === 0 ? 0 : 1);
