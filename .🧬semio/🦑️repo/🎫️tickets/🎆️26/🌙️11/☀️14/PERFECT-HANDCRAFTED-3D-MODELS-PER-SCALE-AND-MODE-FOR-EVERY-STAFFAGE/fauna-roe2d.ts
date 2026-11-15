/** 🦌️ Fauna: 2D root repair of the roe deer trunk (Goldfinger; Ellenberger/Dittrich/Baum; roe deer: shoulder height 0.65-0.75 m, deep chest, tucked flank, croup higher than the withers' line behind the saddle): the drawn trunk is a uniform tube (constant depth, constant width). The elevation outline of hindbody and chest gets a flank tuck and a deep brisket, the plan outline a waist behind the ribs and wider hips and barrel.
 * Usage: bun fauna-roe2d.ts > 🗑️generated/fauna/roe2d-patch.json */
import { openCtx } from "./kit-lib.ts";
import { polyMap, type P } from "./fauna-affine.ts";
const A = "animals/roe-deer";
const out: any[] = [];
const r5 = (x: number) => Math.round(x * 1e5) / 1e5;
const pl = (knots: P[]) => (x: number): number => { if (x <= knots[0]![0]) return knots[0]![1]; for (let i = 1; i < knots.length; i++) if (x <= knots[i]![0]) { const a = knots[i - 1]!; const b = knots[i]!; return a[1] + ((b[1] - a[1]) * (x - a[0])) / (b[0] - a[0]); } return knots[knots.length - 1]![1]; };
const top = pl([[-0.145, 0.66], [-0.12, 0.745], [-0.05, 0.788], [0.1, 0.786], [0.25, 0.776], [0.4, 0.782], [0.5, 0.796], [0.62, 0.806], [0.72, 0.8], [0.8, 0.78], [0.906, 0.74]]);
const bot = pl([[-0.145, 0.60], [-0.10, 0.56], [-0.03, 0.525], [0.1, 0.545], [0.25, 0.585], [0.38, 0.59], [0.5, 0.535], [0.6, 0.472], [0.72, 0.44], [0.82, 0.465], [0.906, 0.56]]);
const wid = pl([[-0.145, 0.035], [-0.10, 0.066], [-0.04, 0.086], [0.05, 0.09], [0.2, 0.076], [0.33, 0.08], [0.45, 0.094], [0.58, 0.098], [0.7, 0.088], [0.8, 0.066], [0.906, 0.05]]);
const xsOf = (a: number, b: number, n: number): number[] => Array.from({ length: n }, (_, i) => a + ((b - a) * i) / (n - 1));
for (const sc of [50, 100, 200]) {
  const c = openCtx(A, sc);
  const geo: any = (c.m as any).representations[String(sc)].geometry;
  for (const [id, bone, x0, x1] of [["hindbody", "lumbar", -0.145, 0.5], ["chest", "thorax", 0.45, 0.906]] as const) {
    const n = sc === 50 ? 12 : 9;
    const xs = xsOf(x0, x1, n);
    const el: P[] = [...xs.map((x) => [x, top(x)] as P), ...[...xs].reverse().map((x) => [x, bot(x)] as P)];
    const f: any = c.state.frames[bone]!;
    out.push({ scale: String(sc), view: "elevation", id, set: { points: el.map(([x, y]) => [r5((x - f.o[0]) * f.u[0] + (y - f.o[1]) * f.u[1]), r5((x - f.o[0]) * f.v[0] + (y - f.o[1]) * f.v[1])]) } });
    const plRaw: P[] = geo.plan.find((s: any) => s.id === id).points;
    const m = polyMap(c, "plan", id, plRaw);
    if (m.err > 1e-4) throw new Error(`${sc} plan ${id}: affine error ${m.err}`);
    const wp: P[] = [...xs.map((x) => [x, -wid(x)] as P), ...[...xs].reverse().map((x) => [x, wid(x)] as P)];
    out.push({ scale: String(sc), view: "plan", id, set: { points: m.toLocal(wp) } });
  }
}
console.log(JSON.stringify(out));
