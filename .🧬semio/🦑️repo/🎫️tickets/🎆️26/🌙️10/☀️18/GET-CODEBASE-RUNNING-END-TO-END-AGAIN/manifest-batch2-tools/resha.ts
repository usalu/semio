/** 🔁 Re-pins the sha256 of fileOps whose live source changed only by the json-attribute codemod since staging (guarded: the live text must equal the staged text plus `with { type: "json" }`). */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const root = process.cwd();
const file = process.argv[2]!;
const write = process.argv.includes("--write");
const delta = JSON.parse(readFileSync(file, "utf8"));
const sha = (b: Buffer): string => createHash("sha256").update(b).digest("hex");
let n = 0;
for (const o of delta.fileOps) {
  const buf = readFileSync(resolve(root, o.path));
  if (sha(buf) === o.sha256) continue;
  const stripped = Buffer.from(buf.toString("utf8").replace(/ with \{ type: "json" \}/gu, ""));
  if (sha(stripped) !== o.sha256) { console.error(`CHANGED beyond json attribute: ${o.path}`); process.exit(3); }
  o.sha256 = sha(buf); n++;
}
console.log(`re-pinned ${n}`);
if (write) writeFileSync(file, JSON.stringify(delta, null, 2) + "\n");
