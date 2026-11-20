/** 🧪️ Prints the world joints of the candidate document's rig of the sleeping person under pose offsets given as JSON (debug aid while the side poses are stated): `bun r3-sleeping-person-posetry.ts '<offsets json>'`.
 * @see ./r3-sleeping-person-roots.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { framesOf } from "./r3-sleeping-person-roots.ts";
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "r3-sleeping-person", "🔣️.json"), "utf8")).asset;
const offsets = JSON.parse(process.argv[2] ?? "{}");
const frames = framesOf(asset.rig, offsets);
const f3 = (p: readonly number[]): string => p.map((x) => x.toFixed(3).padStart(7)).join(",");
for (const b of asset.rig.bones) {
  const f = frames[b.id]!;
  console.log(b.id.padEnd(11), "o", f3(f.o), "tip", f3([0, 1, 2].map((k) => f.o[k]! + b.lengthM * f.u[k]!)), "v", f3(f.v), "w", f3(f.w));
}
