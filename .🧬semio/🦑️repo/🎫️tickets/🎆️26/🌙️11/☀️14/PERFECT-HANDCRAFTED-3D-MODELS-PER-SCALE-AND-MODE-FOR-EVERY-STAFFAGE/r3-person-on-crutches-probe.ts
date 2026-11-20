/** 🔎 Probe of single solids of the 1:50 body of `humans/person-on-crutches`: the angular coverage of the cuff about the forearm axis and the local bounding box of named solids.
 * Usage: `bun r3-person-on-crutches-probe.ts [id ...]` */
import { localGeometry } from "./volumes-lib.ts";
import { body50 } from "./r3-person-on-crutches-s50.ts";
const body = body50();
for (const id of process.argv.slice(2).length ? process.argv.slice(2) : ["crutch-cuff-l", "crutch-cuff-r"]) {
  const s = body.find((b) => b.id === id)!;
  const g = localGeometry(s, { chordM: 0.002, minSegments: 16, maxSegments: 48 });
  const bins = new Array(12).fill(0);
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < g.positions.length; i += 3) {
    const a = (Math.atan2(g.positions[i + 2]!, g.positions[i + 1]!) * 180) / Math.PI;
    bins[Math.floor(((a + 360) % 360) / 30)]++;
    for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, g.positions[i + k]!); hi[k] = Math.max(hi[k]!, g.positions[i + k]!); }
  }
  console.log(id, "vertices per 30 degrees from +v towards +w:", bins.join(" "), "box", lo.map((x) => x.toFixed(3)).join(","), hi.map((x) => x.toFixed(3)).join(","));
}
