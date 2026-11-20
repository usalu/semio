/** 🧪️ Bounds of single solids through the shared tessellation, to confirm the kernel's frame conventions (extrude planes, loft rotations) for `machinery/tower-crane`.
 *
 * Usage: `bun r3-tower-crane-probe.ts '<solid json>' ...`
 * @see ./volumes-lib.ts */
import { localGeometry } from "./volumes-lib.ts";

for (const arg of process.argv.slice(2)) {
  const s = JSON.parse(arg);
  const g = localGeometry(s, { chordM: 0.002, minSegments: 16, maxSegments: 64 });
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < g.positions.length; i += 3) for (let k = 0; k < 3; k++) {
    min[k] = Math.min(min[k]!, g.positions[i + k]!);
    max[k] = Math.max(max[k]!, g.positions[i + k]!);
  }
  process.stdout.write(`${s.id}: min ${min.map((v) => v.toFixed(3))} max ${max.map((v) => v.toFixed(3))} tris ${g.indices.length / 3}\n`);
}
