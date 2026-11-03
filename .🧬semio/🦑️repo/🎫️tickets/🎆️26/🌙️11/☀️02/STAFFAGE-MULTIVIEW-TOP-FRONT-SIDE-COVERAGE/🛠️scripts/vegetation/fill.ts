import { bands, load, save } from "./lib.ts";
import { coverage } from "./cover.ts";
const id = process.argv[2]!; const write = process.argv[3] === "write"; const x = load(id); const a = x.asset;
const r4 = (v: number) => Math.round(v * 10000) / 10000;
const cloud = (cx: number, cy: number, rx: number, ry: number, n: number, amp: number, phase: number): [number, number][] => {
  const pts: [number, number][] = [];
  for (let i = 0; i < 72; i++) { const t = (i / 72) * 2 * Math.PI; const r = 1 - amp + amp * Math.pow(Math.abs(Math.sin((n * t) / 2 + phase)), 0.7) + 0.03 * Math.sin(3 * t + phase); pts.push([r4(cx + Math.cos(t) * r * rx), r4(cy + Math.sin(t) * r * ry)]); }
  return pts;
};
for (const den of bands(a).filter((d: number) => d <= 200)) {
  const g = a.representations[String(den)].geometry; let added = 0;
  for (let k = 0; k < 14; k++) {
    const c = coverage(a, den) as any; if (c.cov < 0 || c.cov >= 0.86) break;
    const rings = c.crown.flatMap((s: any) => s.rings.flat()); const all = c.crown.flatMap((s: any) => s.rings.flat());
    const x0 = Math.min(...all.map((p: any) => p.x)), x1 = Math.max(...all.map((p: any) => p.x)), y0 = Math.min(...all.map((p: any) => p.y)), y1 = Math.max(...all.map((p: any) => p.y));
    let best: any = null, bd = -1;
    for (const [hx, hy] of c.holes) { let d = 1e9; for (const p of rings) d = Math.min(d, Math.hypot(p.x - hx, p.y - hy)); if (d > bd) { bd = d; best = [hx, hy]; } }
    const R = Math.max(0.35 * Math.min(x1 - x0, y1 - y0) / 2, Math.min(bd * 1.5, 0.5 * Math.min(x1 - x0, y1 - y0) / 2));
    const rx = R * 1.15, ry = R;
    const cx = Math.min(Math.max(best[0], x0 + rx), x1 - rx), cy = Math.min(Math.max(best[1], y0 + ry), y1 - ry);
    let near: any = null, nd = 1e9; for (const s of c.crown) { const pts = s.rings.flat(); const mx = pts.reduce((q: number, p: any) => q + p.x, 0) / pts.length, my = pts.reduce((q: number, p: any) => q + p.y, 0) / pts.length; const d = Math.hypot(mx - cx, my - cy); if (d < nd) { nd = d; near = s.shape; } }
    const zMin = Math.min(...g.plan.filter((s: any) => s.role === "body").map((s: any) => s.z));
    const sid = `crown-fill-${added + 1}`;
    g.plan.push({ id: sid, bone: near.bone, role: "body", z: r4(zMin - 0.3 - added * 0.01), filled: true, tone: near.tone, ...(near.region ? { region: near.region } : {}), ...(near.regions ? { regions: near.regions } : {}), kind: "poly", points: cloud(cx, cy, rx, ry, 9 + (added % 3), 0.1, 0.7 * (added + 1)), closed: true });
    added++;
  }
  console.log(id, den, "fill", added, "coverage", (coverage(a, den) as any).cov.toFixed(2));
}
if (write) save(x);
