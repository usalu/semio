/** 📐️ Round 3 mallard: largest lateral half width of every body solid of one scale in the default pose, to compare the folded wings with the flank. Usage: bun r3-mallard-duck-width.ts <scale>
 * @see ./r3-mallard-duck-2d.ts */
import { localGeometry } from "./volumes-lib.ts";
import { Bench, type Json, type V3 } from "./r3-mallard-duck-kit.ts";

const scale = Number(process.argv[2] ?? 50);
const S = new Bench(scale);
for (const solid of S.c.vol.body as Json[]) {
  const fr = S.frame(solid.bone);
  const g = localGeometry(solid, { chordM: 0.0005, minSegments: 24, maxSegments: 96 });
  let [lo, hi] = [Infinity, -Infinity];
  for (let i = 0; i < g.positions.length; i += 3) {
    const z = (fr.o as V3)[2] + g.positions[i]! * (fr.u as V3)[2] + g.positions[i + 1]! * (fr.v as V3)[2] + g.positions[i + 2]! * (fr.w as V3)[2];
    [lo, hi] = [Math.min(lo, z), Math.max(hi, z)];
  }
  console.log(`${solid.id.padEnd(22)} z ${lo.toFixed(4)} .. ${hi.toFixed(4)}`);
}
