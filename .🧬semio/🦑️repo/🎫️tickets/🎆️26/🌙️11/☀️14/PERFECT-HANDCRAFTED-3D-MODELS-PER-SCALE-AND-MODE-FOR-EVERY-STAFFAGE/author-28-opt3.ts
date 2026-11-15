/** 🎯️ Author 28: coordinate descent over solid parameters against the check's own silhouette raster (drawnSet + tileCompare). Usage: bun author-28-opt.ts <asset> <scale> [--write] [--ids a,b] [--rounds n] */
import { readFileSync, writeFileSync } from "node:fs";
import { entryOf, loadSource, merged, scaleMeshes, featureM } from "./volumes-lib.ts";
import { measureAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const args = process.argv.slice(2);
const spec = args[0]!;
const scale = Number(args[1]);
const write = args.includes("--write");
const idsArg = args.includes("--ids") ? args[args.indexOf("--ids") + 1]!.split(",") : null;
const rounds = args.includes("--rounds") ? Number(args[args.indexOf("--rounds") + 1]) : 3;
const { entry, asset: raw } = entryOf(spec);
const src: any = loadSource(entry)!;
const asset: any = merged(raw, src);
const f = featureM(scale, asset);
const long = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
const VIEWS = ["elevation", "front", "plan"] as const;
const LIM: Record<string, number> = { elevation: 0.9, front: 0.88, plan: 0.88 };
const vol = asset.volumes[String(scale)];
const floorOf = (s: any) => f * 0.515;

function iou(): number[] {
  const r: any = measureAsset(asset, scale, asset.presentation.defaultPose, undefined, f);
  return VIEWS.map((v) => r.views[v].iou);
}
const score = (r: number[]) => {
  const m = VIEWS.map((v, i) => r[i]! - LIM[v]!);
  return Math.min(...m, 0.02) * 4 + m.reduce((a, b) => a + Math.min(b, 0.03), 0);
};

const WIDE = Number(process.env.WIDE || 1);
const STEP = Number(process.env.STEP || 1);
type P = { get: () => number; set: (x: number) => void; step: number; lo: number; hi: number; name: string };
const params: P[] = [];
for (const s of vol.body as any[]) {
  if (s.texture === true || (idsArg && !idsArg.includes(s.id))) continue;
  const fl = floorOf(s);
  if (s.kind === "loft") {
    s.sections.forEach((q: any, i: number) => {
      params.push({ name: `${s.id}.${i}.rx`, get: () => q.rx, set: (x) => (q.rx = x), step: 0.006 * STEP * (scale >= 500 ? 1 : scale / 500 * 1.5), lo: Math.min(fl, q.rx), hi: 0.4 });
      params.push({ name: `${s.id}.${i}.ry`, get: () => q.ry, set: (x) => (q.ry = x), step: 0.006 * STEP * (scale >= 500 ? 1 : scale / 500 * 1.5), lo: Math.min(fl, q.ry), hi: 0.4 });
      params.push({ name: `${s.id}.${i}.w`, get: () => q.at[2], set: (x) => (q.at[2] = x), step: 0.008 * STEP * (scale >= 500 ? 1 : scale / 500 * 1.5), lo: q.at[2] - 0.04 * WIDE, hi: q.at[2] + 0.04 * WIDE });
      if (process.env.WIDE) params.push({ name: `${s.id}.${i}.u`, get: () => q.at[0], set: (x) => (q.at[0] = x), step: 0.008, lo: q.at[0] - 0.03, hi: q.at[0] + 0.03 });
      params.push({ name: `${s.id}.${i}.v`, get: () => q.at[1], set: (x) => (q.at[1] = x), step: 0.008 * STEP * (scale >= 500 ? 1 : scale / 500 * 1.5), lo: q.at[1] - 0.03 * WIDE, hi: q.at[1] + 0.03 * WIDE });
    });
  } else if (s.kind === "capsule") {
    for (const k of ["p0", "p1"]) for (const a of [0, 1, 2]) params.push({ name: `${s.id}.${k}.${a}`, get: () => s[k][a], set: (x) => (s[k][a] = x), step: 0.008 * STEP * (scale >= 500 ? 1 : scale / 500 * 1.5), lo: s[k][a] - 0.04, hi: s[k][a] + 0.04 });
    if (s.id.includes("tyre")) continue;
    params.push({ name: `${s.id}.r`, get: () => s.r0, set: (x) => { s.r0 = x; s.r1 = x; }, step: 0.004 * STEP * (scale >= 500 ? 1 : scale / 500 * 1.5), lo: Math.min(fl, s.r0), hi: 0.12 });
  }
}
let best = score(iou());
console.log("start", iou().map((x) => x.toFixed(4)).join(" "));
for (let r = 0; r < rounds; r++) {
  let moved = 0;
  for (const p of params) {
    const x0 = p.get();
    for (const d of [p.step, -p.step]) {
      const x = Math.min(p.hi, Math.max(p.lo, x0 + d));
      if (x === x0) continue;
      p.set(x);
      const sc = score(iou());
      if (sc > best + 1e-6) { best = sc; moved++; break; }
      p.set(x0);
    }
  }
  console.log("round", r, "moved", moved, iou().map((x) => x.toFixed(4)).join(" "));
  if (!moved) break;
}
const RAND = Number(process.env.RAND || 0);
let seed = Number(process.env.SEED || 12345);
const rnd = () => ((seed = (seed * 1664525 + 1013904223) >>> 0) / 4294967296);
for (let n = 0; n < RAND; n++) {
  const p = params[Math.floor(rnd() * params.length)]!;
  const x0 = p.get();
  const x = Math.min(p.hi, Math.max(p.lo, x0 + (rnd() < 0.5 ? -1 : 1) * p.step * (1 + Math.floor(rnd() * 3))));
  if (x === x0) continue;
  p.set(x);
  const sc = score(iou());
  if (sc > best + 1e-6) { best = sc; console.log("rand", n, p.name, iou().map((v) => v.toFixed(4)).join(" ")); } else p.set(x0);
}
if (write) {
  const out: any = JSON.parse(readFileSync(entryPath(), "utf8"));
  out.volumes[String(scale)].body = vol.body;
  writeFileSync(entryPath(), JSON.stringify(out, null, 2) + "\n");
  console.log("written");
}
function entryPath() { return `volumes/${spec}.json`; }
