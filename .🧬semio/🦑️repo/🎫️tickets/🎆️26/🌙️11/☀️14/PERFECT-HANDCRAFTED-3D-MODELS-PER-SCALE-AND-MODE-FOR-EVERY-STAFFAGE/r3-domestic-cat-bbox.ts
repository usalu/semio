/** 📏️ Prints the bounding box of the body of the working domestic cat in the default pose at every authored scale, against `dimensions`. Usage: bun r3-domestic-cat-bbox.ts */
import { meshesAt } from "./volumes-lib.ts";
import { working } from "./r3-domestic-cat-work.ts";
const m = working();
for (const scale of [50, 100, 200]) {
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const mesh of meshesAt(m, scale, m.volumes[String(scale)].body)) for (let i = 0; i < mesh.world.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, mesh.world[i]!); hi[i % 3] = Math.max(hi[i % 3]!, mesh.world[i]!); }
  console.log(`1:${scale} length ${(hi[0]! - lo[0]!).toFixed(4)} (${lo[0]!.toFixed(4)}..${hi[0]!.toFixed(4)}) of ${m.dimensions.widthM}, height ${hi[1]!.toFixed(4)} (from ${lo[1]!.toFixed(4)}) of ${m.dimensions.heightM}, width ${(hi[2]! - lo[2]!).toFixed(4)} of ${m.dimensions.depthM}`);
}
