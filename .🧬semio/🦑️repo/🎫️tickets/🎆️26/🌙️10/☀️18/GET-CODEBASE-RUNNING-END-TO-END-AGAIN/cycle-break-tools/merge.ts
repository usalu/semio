/** 🧵 Merges the staged parts (w1, carves, extras) into delta.json; the direct parts (d*) were already applied to the tree and are listed as applied. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { Plan } from "./plan.ts";

const dir = resolve(import.meta.dir, "delta-parts");
const staged = ["w1", "carves", "extras", "w2d", "w2e", "refs", "fix6"];
const merged = new Plan();
const applied: Record<string, { edits: number; fileOps: number; newFiles: number }> = {};
for (const f of readdirSync(dir).sort()) {
  const name = f.replace(/\.json$/u, "");
  const j = JSON.parse(readFileSync(resolve(dir, f), "utf8"));
  if (!staged.includes(name)) { if (/^d\d/u.test(name)) applied[name] = { edits: j.edits.length, fileOps: j.fileOps.length, newFiles: j.newFiles.length }; continue; }
  for (const o of j.manifestOps) merged.manifest(o);
  for (const n of j.newFiles) merged.newFile(n);
  for (const e of j.edits) merged.edit(e);
  for (const o of j.fileOps) { if (o.op === "delete") merged.fileOps.set(o.path, o); else merged.fileOps.set(o.path, o); }
  for (const fl of j.flags) merged.flags.push(fl);
}
const out = { ...JSON.parse(JSON.stringify(merged)), appliedDirect: applied };
writeFileSync(resolve(import.meta.dir, "../cycle-break-delta.json"), JSON.stringify(out, null, 1));
console.log("delta:", out.manifestOps.length, "manifest ops;", out.newFiles.length, "new files;", out.edits.length, "edits;", out.fileOps.length, "file ops;", out.flags.filter((x: any) => x.severity === "blocker").length, "blockers;", "direct parts", Object.keys(applied).join(","));
