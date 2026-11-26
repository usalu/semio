/** 🎲️ Random masses of overlapping ellipsoid beads (sections every `step` degrees) projected along a tilted view: the bodies that `loft3` unifies and the area of `unionPolygons` of them, dumped for the shapely oracle: union-random.ts <step degrees> <cases> <seed> <out json> */
import { writeFileSync } from "node:fs";
import { loftRings } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { unionPolygons } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
type P = { x: number; y: number };
const area = (r: readonly P[]): number => r.reduce((s, p, k) => s + p.x * r[(k + 1) % r.length]!.y - r[(k + 1) % r.length]!.x * p.y, 0) / 2;
function hull(points: P[]): P[] {
  const s = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  const turn = (o: P, a: P, b: P) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const build = (input: P[]) => { const out: P[] = []; for (const p of input) { while (out.length >= 2 && turn(out.at(-2)!, out.at(-1)!, p) <= 0) out.pop(); out.push(p); } out.pop(); return out; };
  return [...build(s), ...build([...s].reverse())];
}
const [stepArg, casesArg, seedArg, out] = process.argv.slice(2);
const step = Number(stepArg);
let seed = Number(seedArg);
const random = (): number => { seed = (seed * 1664525 + 1013904223) % 4294967296; return seed / 4294967296; };
const cases: unknown[] = [];
for (let c = 0; c < Number(casesArg); c++) {
  const tilt = random() * 90, turn = random() * 180;
  const [ct, st, cu, su] = [Math.cos((tilt * Math.PI) / 180), Math.sin((tilt * Math.PI) / 180), Math.cos((turn * Math.PI) / 180), Math.sin((turn * Math.PI) / 180)];
  const bodies: P[][] = [];
  for (let k = 0, n = 2 + Math.floor(random() * 6); k < n; k++) {
    const [rx, ry, rz] = [0.1 + random() * 0.4, 0.1 + random() * 0.4, 0.1 + random() * 0.4];
    const centre = [(random() - 0.5) * 0.8, (random() - 0.5) * 0.8, (random() - 0.5) * 0.8];
    const sections = [];
    for (let t = -90 + step; t < 90 - 1e-9; t += step) { const a = (t * Math.PI) / 180; sections.push({ at: [rz * Math.sin(a), 0, 0] as [number, number, number], rx: rx * Math.cos(a), ry: ry * Math.cos(a) }); }
    const project = (p: number[]): P => { const [x, y, z] = [p[0]! + centre[0]!, p[1]! + centre[1]!, p[2]! + centre[2]!]; const [x2, z2] = [x * cu - z * su, x * su + z * cu]; return { x: x2, y: y * ct - z2 * st }; };
    const rings = loftRings({ sections }).map((ring) => ring.map(project));
    for (let i = 0; i + 1 < rings.length; i++) { const b = hull([...rings[i]!, ...rings[i + 1]!]); if (b.length >= 3 && Math.abs(area(b)) > 1e-12) bodies.push(b); }
  }
  let engineArea = -1;
  try { engineArea = unionPolygons(bodies).reduce((s, r) => s + area(r), 0); } catch { engineArea = -2; }
  cases.push({ scale: 0, view: `case ${c} tilt ${tilt.toFixed(1)} turn ${turn.toFixed(1)}`, bone: "random", id: `${step}deg`, engineArea, bodies: bodies.map((b) => b.map((p) => [p.x, p.y])) });
}
writeFileSync(out!, JSON.stringify(cases));
console.log(cases.length, "cases");
