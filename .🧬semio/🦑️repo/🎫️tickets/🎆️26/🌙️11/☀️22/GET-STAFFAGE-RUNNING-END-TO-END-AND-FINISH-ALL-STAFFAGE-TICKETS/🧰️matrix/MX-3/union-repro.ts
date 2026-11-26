/** 🧪️ Beads of an ellipsoid (sections every `step` degrees, as the loft tool of MV-L1 draws them) projected along a tilted view, their convex hulls unified by `unionPolygons`, against the convex hull of all their points (the ellipsoid is convex: the union must reach the hull): union-repro.ts <step degrees>… */
import { loftRings } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { unionPolygons } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
type P = { x: number; y: number };
const area = (r: P[]): number => r.reduce((s, p, k) => s + p.x * r[(k + 1) % r.length]!.y - r[(k + 1) % r.length]!.x * p.y, 0) / 2;
function hull(points: P[]): P[] {
  const s = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  const turn = (o: P, a: P, b: P) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const build = (input: P[]) => { const out: P[] = []; for (const p of input) { while (out.length >= 2 && turn(out.at(-2)!, out.at(-1)!, p) <= 0) out.pop(); out.push(p); } out.pop(); return out; };
  return [...build(s), ...build([...s].reverse())];
}
let bad = 0, total = 0;
for (const step of process.argv.slice(2).map(Number)) {
  for (const [rx, ry, rz] of [[0.3, 0.2, 0.25], [0.55, 0.35, 0.4], [0.12, 0.12, 0.12], [0.8, 0.5, 0.6]] as const)
    for (const tilt of [0, 10, 25, 40, 55, 70, 85]) for (const turn of [0, 33, 77]) {
      const sections = [];
      for (let t = -90 + step; t < 90 - 1e-9; t += step) { const a = (t * Math.PI) / 180; sections.push({ at: [rz * Math.sin(a), 0, 0] as [number, number, number], rx: rx * Math.cos(a), ry: ry * Math.cos(a) }); }
      const rings = loftRings({ sections });
      const [ct, st, cu, su] = [Math.cos((tilt * Math.PI) / 180), Math.sin((tilt * Math.PI) / 180), Math.cos((turn * Math.PI) / 180), Math.sin((turn * Math.PI) / 180)];
      const project = (p: number[]): P => { const [x, y, z] = [p[0]! * cu - p[2]! * su, p[1]!, p[0]! * su + p[2]! * cu]; return { x, y: y * ct - z * st }; };
      const projected = rings.map((ring) => ring.map(project));
      const bodies: P[][] = [];
      for (let i = 0; i + 1 < projected.length; i++) { const body = hull([...projected[i]!, ...projected[i + 1]!]); if (body.length >= 3 && Math.abs(area(body)) > 1e-12) bodies.push(body); }
      total++;
      let got = 0, note = "";
      try { got = unionPolygons(bodies).reduce((s, r) => s + area(r), 0); } catch (e) { note = String(e).slice(0, 80); }
      const want = Math.abs(area(hull(projected.flat())));
      if (note || Math.abs(got) < want * 0.98) { bad++; if (bad <= 12) console.log(`step ${step} radii ${rx},${ry},${rz} tilt ${tilt} turn ${turn}: ${bodies.length} hulls, union area ${Math.abs(got).toFixed(5)} against ${want.toFixed(5)} ${note}`); }
    }
}
console.log(`bad ${bad} of ${total}`);
