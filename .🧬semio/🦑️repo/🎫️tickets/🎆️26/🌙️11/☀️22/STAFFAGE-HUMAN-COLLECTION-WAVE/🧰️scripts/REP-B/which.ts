/** 🔎️ Applies the repair ops of a person one by one to its catalog file in memory and names the first op that does not apply.
 *  bun which.ts <slug> */
import { applyOps, readOps } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

const slug = process.argv[2]!;
const asset = (await Bun.file(catalogFile(slug)).json()).asset;
const ops = await readOps(opsFile(slug));
for (let n = 1; n <= ops.length; n++) {
  const copy = structuredClone(asset);
  try { applyOps(copy, ops.slice(0, n)); } catch (e) { const o = ops[n - 1]!; console.log(`${slug}: op ${n - 1} of ${ops.length} fails: ${o.op} ${o.scale} ${o.view} ${o.id ?? o.shape?.id} after ${o.after} group ${(o as any).group}: ${String(e).slice(0, 100)}`); process.exit(0); }
}
console.log(`${slug}: all ${ops.length} ops apply`);
