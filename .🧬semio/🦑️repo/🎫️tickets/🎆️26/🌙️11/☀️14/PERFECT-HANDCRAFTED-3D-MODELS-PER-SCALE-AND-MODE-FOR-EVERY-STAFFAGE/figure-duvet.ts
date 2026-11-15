/** 🛏️ Replaces the per-bone duvet lofts of the sleeper by one continuous draped sheet: one smooth top profile and one width profile cut from the scale's own duvet outlines (`duvet-chest`, `duvet-hips`, `duvet-thighs`, `duvet-shins`), sampled by every bone's loft over its own x range and abutting the next, so no ring, step or extension sliver remains; flat on the mattress, rounded rectangle sections (exponent 3), the head end and the foot end round off along the drawn outline.
 * Usage: `bun figure-duvet.ts humans/sleeping-person --scale N [--write]` (run after `figure-build.ts`; the lofts of the bones torso, pelvis, thigh-r, shank-r in the regions top and bottom are rebuilt, the thermal layer shells of them regenerated, everything else is kept).
 * @see ./figure.md */
import { drawn, flag, has, openCtx, saveLists, type Json } from "./kit-lib.ts";
import { boneInfos, layerShells, loftStations, type Stn } from "./figure-lib.ts";
const args = process.argv.slice(2);
const c = openCtx(args[0]!, Number(flag(args, "--scale")));
const infos = boneInfos(c);
const f = c.feature;
const fl = 0.502 * f;
const SEG: { bone: string; id: string; region: string; shape: string }[] = [
  { bone: "torso", id: "torso-top", region: "top", shape: "duvet-chest" },
  { bone: "pelvis", id: "pelvis-top", region: "top", shape: "duvet-hips" },
  { bone: "thigh-r", id: "thigh-r-bottom", region: "bottom", shape: "duvet-thighs" },
  { bone: "shank-r", id: "shank-r-bottom", region: "bottom", shape: "duvet-shins" },
];
const elev = drawn(c, "elevation").filter((s) => SEG.some((q) => q.shape === s.id));
const front = drawn(c, "front").filter((s) => SEG.some((q) => q.shape === s.id));
const colY = (rings: { x: number; y: number }[][], x: number): [number, number] | null => {
  let lo = Infinity;
  let hi = -Infinity;
  for (const r of rings) for (let i = 0, k = r.length - 1; i < r.length; k = i++) {
    const a = r[k]!;
    const b = r[i]!;
    if ((a.x <= x) !== (b.x <= x)) {
      const y = a.y + ((x - a.x) / (b.x - a.x)) * (b.y - a.y);
      lo = Math.min(lo, y);
      hi = Math.max(hi, y);
    }
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
};
const xr = (rings: { x: number; y: number }[][]): [number, number] => [Math.min(...rings.flat().map((q) => q.x)), Math.max(...rings.flat().map((q) => q.x))];
const X0 = Math.min(...elev.map((s) => xr(s.rings)[0]));
const X1 = Math.max(...elev.map((s) => xr(s.rings)[1]));
const hw = (id: string): number => {
  const s = front.find((q) => q.id === id)!;
  const xs = s.rings.flat().map((q) => q.x);
  return (Math.max(...xs) - Math.min(...xs)) / 2;
};
const ranges: Record<string, [number, number]> = {};
for (const q of SEG) ranges[q.shape] = xr(elev.find((s) => s.id === q.shape)!.rings);
const mids = SEG.slice(0, -1).map((q, i) => (ranges[q.shape]![1] + ranges[SEG[i + 1]!.shape]![0]) / 2);
const cuts = [X0, ...mids, X1];
const smooth = (t: number): number => t * t * (3 - 2 * t);
const width = (x: number): number => {
  const centres = SEG.map((q, i) => (cuts[i]! + cuts[i + 1]!) / 2);
  if (x <= centres[0]!) return hw(SEG[0]!.shape);
  for (let i = 0; i + 1 < centres.length; i++) if (x <= centres[i + 1]!) return hw(SEG[i]!.shape) + (hw(SEG[i + 1]!.shape) - hw(SEG[i]!.shape)) * smooth((x - centres[i]!) / (centres[i + 1]! - centres[i]!));
  return hw(SEG[SEG.length - 1]!.shape);
};
const profile = (x: number): [number, number] => {
  const xc = Math.min(X1 - 0.0015, Math.max(X0 + 0.0015, x));
  let lo = Infinity;
  let hi = -Infinity;
  for (const s of elev) {
    const r = colY(s.rings, xc);
    if (r === null) continue;
    lo = Math.min(lo, r[0]);
    hi = Math.max(hi, r[1]);
  }
  return [lo, hi];
};
const win = 0.05;
const topSmooth = (x: number): [number, number] => {
  const ends = Math.min(x - X0, X1 - x);
  if (ends < 0.04) return profile(x);
  const ps = [-1, -0.5, 0, 0.5, 1].map((k) => profile(x + k * win));
  return [ps.reduce((a, p) => a + p[0], 0) / ps.length, ps.reduce((a, p) => a + p[1], 0) / ps.length];
};
const old = new Map((c.vol.body as Json[]).map((s) => [s.id, s]));
const body: Json[] = (c.vol.body as Json[]).filter((s) => !SEG.some((q) => s.id === q.id || s.id === `${q.id}-lo` || s.id === `${q.id}-hi`));
const kept = (c.vol.thermal as Json[]).filter((t) => !SEG.some((q) => t.of === q.id || t.of === `${q.id}-lo` || t.of === `${q.id}-hi` || (t.id ?? "").startsWith(`th-${q.id}`)));
const thermal = [...kept];
const regions = Object.fromEntries((c.m.thermal.regions as Json[]).map((r) => [r.id, r]));
for (const [i, q] of SEG.entries()) {
  const info = infos[q.bone]!;
  const fr = info.frame;
  const a = cuts[i]!;
  const b = cuts[i + 1]!;
  const n = Math.max(4, Math.ceil((b - a) / 0.035));
  const xs = Array.from({ length: n + 1 }, (_, k) => a + ((b - a) * k) / n);
  const st: Stn[] = [];
  for (const x of xs) {
    const [lo, hi] = topSmooth(x);
    const yc = (lo + hi) / 2;
    const P = [x - fr.o[0], yc - fr.o[1]];
    st.push({ s: P[0]! * fr.u[0]! + P[1]! * fr.u[1]!, cv: P[0]! * fr.v[0]! + P[1]! * fr.v[1]!, cw: 0, rx: Math.max(fl, width(x)), ry: Math.max(fl, (hi - lo) / 2), e: 3 });
  }
  const prev = old.get(q.id)!;
  const sol = { ...loftStations({ id: q.id, bone: q.bone, material: prev.material, tone: prev.tone }, st), region: undefined } as Json;
  delete sol.region;
  body.push(sol);
  thermal.push(...layerShells(sol, regions[q.region]!, q.id, c.scale === 50));
}
console.log(`1:${c.scale} duvet: ${SEG.length} lofts, x ${X0.toFixed(3)}..${X1.toFixed(3)}, cuts ${cuts.map((x) => x.toFixed(3)).join(" ")}; body ${body.length}, thermal ${thermal.length}`);
if (has(args, "--write")) saveLists(c, { body, thermal });
