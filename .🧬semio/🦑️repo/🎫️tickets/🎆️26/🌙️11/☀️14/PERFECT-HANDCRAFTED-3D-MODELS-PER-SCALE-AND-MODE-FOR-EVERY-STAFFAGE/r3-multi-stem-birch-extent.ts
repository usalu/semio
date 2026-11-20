/** 📏️ Prints, per scale, the parts of the round-3 multi-stem birch that reach farthest in -x, +x, -z, +z and +y (three each), to tune the crown to `dimensions`.
 * @see ./r3-vegetation-multi-stem-birch.md */
import { bounds } from "./r3-multi-stem-birch-kit.ts";
import { models } from "./r3-multi-stem-birch-model.ts";

for (const m of models) {
  const rows = [...m.wood, ...m.leaves].map((p) => ({ id: p.id, b: bounds([p]) }));
  const top = (key: (r: (typeof rows)[number]) => number): string => [...rows].sort((a, b) => key(b) - key(a)).slice(0, 3).map((r) => `${r.id} ${key(r).toFixed(2)}`).join(", ");
  console.log(`[DEBUG] 1:${m.scale} -x: ${top((r) => -r.b.min[0])} | +x: ${top((r) => r.b.max[0])} | -z: ${top((r) => -r.b.min[2])} | +z: ${top((r) => r.b.max[2])} | +y: ${top((r) => r.b.max[1])}`);
}
