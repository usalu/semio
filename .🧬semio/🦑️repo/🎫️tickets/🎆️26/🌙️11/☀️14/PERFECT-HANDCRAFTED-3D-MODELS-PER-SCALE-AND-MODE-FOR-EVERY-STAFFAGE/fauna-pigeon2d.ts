/** 🕊️ Fauna: 2D root repair of the pigeon (Columba livia; Cramp/Simmons BWP: tail 110-125 mm and about 25-30 mm deep at the base): the drawn elevation tail is a paddle 60-80 mm deep; at 1:50, 1:100 and 1:200 it becomes a wedge that is 34 mm deep at the base and 16 mm (30 mm at 1:200) at the tip, keeping the drawn upper edge.
 * Usage: bun fauna-pigeon2d.ts > 🗑️generated/fauna/pigeon2d-patch.json */
import { drawn, openCtx } from "./kit-lib.ts";
const A = "animals/pigeon";
const out: any[] = [];
const r5 = (x: number) => Math.round(x * 1e5) / 1e5;
type P = [number, number];
for (const sc of [50, 100, 200]) {
  const c = openCtx(A, sc);
  for (const id of ["tail", "far-tail"]) {
    const sh = drawn(c, "elevation").find((s) => s.id === id);
    if (sh === undefined) continue;
    const f: any = c.state.frames[sh.bone]!;
    const ring: P[] = sh.rings[0]!.map((p) => [p.x, p.y]);
    const xs0 = Math.min(...ring.map((p) => p[0]));
    const xs1 = Math.max(...ring.map((p) => p[0]));
    const mid = ring.map((p) => p[1]).reduce((a, b) => a + b) / ring.length;
    const topAt = (x: number): number => { let best = -Infinity; for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) { const a = ring[k]!; const b = ring[i]!; if (a[0] <= x !== b[0] <= x) { const y = a[1] + ((x - a[0]) / (b[0] - a[0])) * (b[1] - a[1]); if (y > best) best = y; } } return best; };
    const n = 9;
    const xsq = Array.from({ length: n }, (_, i) => xs1 - ((xs1 - xs0) * i) / (n - 1));
    const tip = sc === 200 ? 0.03 : 0.016;
    const h = (t: number) => 0.034 + (tip - 0.034) * t;
    const top: P[] = xsq.filter((x) => Number.isFinite(topAt(x))).map((x) => [x, topAt(x)]);
    const bot: P[] = top.map(([x, y], i) => [x, y - h(i / (top.length - 1))] as P).reverse();
    void mid;
    const pts = [...top, ...bot].map(([x, y]) => [r5((x - f.o[0]) * f.u[0] + (y - f.o[1]) * f.u[1]), r5((x - f.o[0]) * f.v[0] + (y - f.o[1]) * f.v[1])]);
    out.push({ scale: String(sc), view: "elevation", id, set: { points: pts } });
    out.push({ scale: String(sc), view: "front", id, set: { points: pts } });
  }
}
console.log(JSON.stringify(out));
