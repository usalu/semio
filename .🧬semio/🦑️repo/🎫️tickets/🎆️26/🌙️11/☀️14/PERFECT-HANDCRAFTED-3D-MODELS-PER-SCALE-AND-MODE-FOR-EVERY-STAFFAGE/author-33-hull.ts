/** 🏗️ Hull loft of an OBLIQUE bone from the scale's own drawings (kit-fit --hull handles axis-aligned bones only).
 * Usage: `bun author-33-hull.ts <asset> --scale N --bone b --id id --material m [--tone .5] [--n 9] [--range s0,s1] [--pick shape,shape] [--exp 2.6] [--tol f] [--round 0.7] [--source file]` prints the loft solid as JSON.
 * Sections: the line through the bone axis at station s, perpendicular to u in the sagittal plane, cut with the bone's elevation shapes gives v extent; the lateral extent comes from the plan (|u.x| >= |u.y|) or the front view. */
import { drawn, flag, openCtx, selectStations, VIEWS, type Json } from "./kit-lib.ts";
const args = process.argv.slice(2);
const c = openCtx(args[0]!, Number(flag(args, "--scale")), flag(args, "--source"));
const bone = flag(args, "--bone")!;
const pick = flag(args, "--pick")?.split(",");
const f = (c.state as any).frames[bone] as { o: number[]; u: number[]; v: number[]; w: number[] };
const feature = c.feature;
const ebone = flag(args, "--ebone");
const shapes = (view: (typeof VIEWS)[number]) => drawn(c, view, view === "elevation" && ebone ? ebone : bone).filter((s) => !pick || (ebone !== undefined && view !== "elevation") || pick.includes(s.id));
const proj: Record<string, (p: number[]) => [number, number]> = { elevation: (p) => [p[0]!, p[1]!], front: (p) => [p[2]!, p[1]!], plan: (p) => [p[0]!, p[2]!] };
const dir2: Record<string, (d: number[]) => [number, number]> = proj as never;
function cut(view: string, P: number[], d: number[]): [number, number] | null {
  const [px, py] = proj[view]!(P);
  const [dx, dy] = dir2[view]!(d);
  let lo = Infinity, hi = -Infinity;
  for (const s of shapes(view as never)) for (const ring of s.rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
    const a = ring[k]!, b = ring[i]!;
    const ex = b.x - a.x, ey = b.y - a.y;
    const den = dx * ey - dy * ex;
    if (Math.abs(den) < 1e-12) continue;
    const t = ((a.x - px) * ey - (a.y - py) * ex) / den;
    const q = ((a.x - px) * dy - (a.y - py) * dx) / den;
    if (q >= 0 && q <= 1) { lo = Math.min(lo, t); hi = Math.max(hi, t); }
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}
let s0 = Infinity, s1 = -Infinity;
for (const view of ["elevation", "front", "plan"] as const) for (const s of shapes(view)) for (const ring of s.rings) for (const q of ring) {
  const p3 = view === "elevation" ? [q.x, q.y, f.o[2]!] : view === "front" ? [f.o[0]!, q.y, q.x] : [q.x, f.o[1]!, q.y];
  void p3;
}
const elev = shapes("elevation");
for (const s of elev) for (const ring of s.rings) for (const q of ring) {
  const sv = (q.x - f.o[0]!) * f.u[0]! + (q.y - f.o[1]!) * f.u[1]!;
  s0 = Math.min(s0, sv); s1 = Math.max(s1, sv);
}
const range = flag(args, "--range")?.split(",").map(Number);
if (range) { s0 = range[0]!; s1 = range[1]!; }
const n = Number(flag(args, "--n", "9"));
const e = Number(flag(args, "--exp", "2.6"));
const lateralView = Math.abs(f.u[0]!) >= Math.abs(f.u[1]!) ? "plan" : "front";
const raw: { s: number; c1: number; c2: number; rx: number; ry: number }[] = [];
for (let i = 0; i < n; i++) {
  const s = s0 + ((s1 - s0) * (i + 0.5)) / n;
  const P = [0, 1, 2].map((k) => f.o[k]! + s * f.u[k]!);
  const ev = cut("elevation", P, f.v);
  const wDir = f.w;
  let lat = cut(lateralView, P, wDir);
  if (lat === null) lat = cut(lateralView === "plan" ? "front" : "plan", P, wDir);
  if (ev === null && lat === null) continue;
  const ry = ev ? (ev[1] - ev[0]) / 2 : (lat![1] - lat![0]) / 2;
  const rx = lat ? (lat[1] - lat[0]) / 2 : ry;
  raw.push({ s, c1: ev ? (ev[0] + ev[1]) / 2 : 0, c2: lat ? (lat[0] + lat[1]) / 2 : 0, rx: Math.max(rx, Number(flag(args, "--floor", "0.502")) * feature), ry: Math.max(ry, Number(flag(args, "--floor", "0.502")) * feature) });
}
const sel = selectStations(raw.map((r) => ({ s: r.s, c1: r.c1, c2: r.c2, rx: r.rx, ry: r.ry, e })) as never, Number(flag(args, "--tol", String(0.5 * feature))), Number(flag(args, "--floor", "0.502")) * feature) as any[];
const round = Number(flag(args, "--round", "1"));
if (sel.length > 0) { sel[0].s = s0; sel[sel.length - 1].s = s1; }
const sections = sel.map((r, i) => {
  const k = (i === 0 || i === sel.length - 1) && round < 1 ? round : 1;
  return { at: [r.s, r.c1, r.c2], rx: Math.max(r.rx * k, Number(flag(args, "--floor", "0.502")) * feature), ry: Math.max(r.ry * k, Number(flag(args, "--floor", "0.502")) * feature), exponent: e, rotation: [0, -0.70711, 0, 0.70711] };
});
const out = { id: flag(args, "--id", bone), kind: "loft", bone, material: flag(args, "--material", "skin"), role: "body", tone: Number(flag(args, "--tone", "0.5")), sections, capStart: true, capEnd: true };
console.log(JSON.stringify(out));
