import { openCtx } from "./kit-lib.ts";
import { writeFileSync } from "node:fs";
const out: any[] = [];
const want: Record<string, [number, number, number, number]> = { "near-toes": [-0.038, -0.014, -0.0017, 0.0146], "far-toes": [0.014, 0.038, -0.0017, 0.0146], "near-foot": [-0.038, -0.014, 0, 0], "far-foot": [0.014, 0.038, 0, 0] };
for (const [scale, ids] of [[50, ["near-toes", "far-toes"]], [100, ["near-toes", "far-toes"]], [200, ["near-foot", "far-foot"]]] as const) {
  const c = openCtx("animals/pigeon", scale);
  for (const id of ids) {
    const st = (c.state.front as any[]).find((s) => s.shape.id === id);
    const pts: number[][] = st.shape.points;
    const ring = st.rings[0] as { x: number; y: number }[];
    const ext = (a: number[]) => [Math.min(...a), Math.max(...a)] as [number, number];
    const [a0, a1] = ext(pts.map((p) => p[0]!)), [b0, b1] = ext(pts.map((p) => p[1]!));
    const [x0, x1] = ext(ring.map((r) => r.x)), [y0, y1] = ext(ring.map((r) => r.y));
    const w = want[id]!;
    if (scale === 200) { out.push({ scale: String(scale), view: "front", id, set: { points: [[w[0], pts[0]![1]], [w[1], pts[0]![1]]] } }); continue; }
    const inv = (v: number, lo: number, hi: number, plo: number, phi: number) => plo + ((v - lo) / (hi - lo)) * (phi - plo);
    const A0 = inv(w[0], x0, x1, a0, a1), A1 = inv(w[1], x0, x1, a0, a1), B0 = inv(w[2], y0, y1, b0, b1), B1 = inv(w[3], y0, y1, b0, b1);
    const r = (v: number) => Math.round(v * 1e5) / 1e5;
    out.push({ scale: String(scale), view: "front", id, set: { points: [[r(A0), r(B0)], [r(A1), r(B0)], [r(A1), r(B1)], [r(A0), r(B1)]] } });
  }
}
console.log(JSON.stringify(out));
writeFileSync("🗑️generated/author-29/pigeon-frontfeet.json", JSON.stringify(out));
