import { readFileSync, writeFileSync } from "node:fs";
import { entryOf, loadSource, merged, poseOf, sourcePath } from "./volumes-lib.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const [spec, scaleArg, idsArg = "all", optArg = ""] = process.argv.slice(2);
const scale = Number(scaleArg);
const { entry, asset: raw } = entryOf(spec!);
const source = loadSource(entry)!;
const asset: any = merged(raw, source);
const state: any = evaluate(asset, { kind: "pose", pose: poseOf(asset) }, { kind: "scale", scale: { denominator: scale } });
const body: any[] = source.volumes[String(scale)].body;
const want = idsArg === "all" ? null : new Set(idsArg.split(","));
const minr = Number((optArg.match(/minr=([\d.]+)/) ?? [])[1] ?? 0.004);
const maxShift = Number((optArg.match(/max=([\d.]+)/) ?? [])[1] ?? 0.12);
const growCap = Number((optArg.match(/grow=([\d.]+)/) ?? [])[1] ?? 2.5);
const add = (a: number[], b: number[], k = 1) => [a[0]! + b[0]! * k, a[1]! + b[1]! * k, a[2]! + b[2]! * k];

function intervals(shapes: any[], axis: "x" | "y", other: number): [number, number][] {
  const out: [number, number][] = [];
  const o = axis === "x" ? "y" : "x";
  for (const s of shapes) {
    const xs: number[] = [];
    for (const ring of s.rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
      const a = ring[k], b = ring[i];
      if ((a[o] <= other) !== (b[o] <= other)) xs.push(a[axis] + ((other - a[o]) / (b[o] - a[o])) * (b[axis] - a[axis]));
    }
    xs.sort((p, q) => p - q);
    for (let n = 0; n + 1 < xs.length; n += 2) out.push([xs[n]!, xs[n + 1]!]);
  }
  out.sort((p, q) => p[0] - q[0]);
  const merged: [number, number][] = [];
  for (const iv of out) {
    const last = merged[merged.length - 1];
    if (last && iv[0] <= last[1] + 1e-6) last[1] = Math.max(last[1], iv[1]);
    else merged.push([iv[0], iv[1]]);
  }
  return merged;
}

let changed = 0;
const report: string[] = [];
for (const s of body) {
  if (s.kind !== "loft" || (want && !want.has(s.id))) continue;
  const bf = state.bones[s.bone]?.frame;
  if (!bf) continue;
  const U = [bf.u.x, bf.u.y, bf.u.z], V = [bf.v.x, bf.v.y, bf.v.z], W = [bf.w.x, bf.w.y, bf.w.z], O = [bf.o.x, bf.o.y, bf.o.z];
  const shapes = (view: string) => (state.shapes[view] as any[]).filter((sh) => sh.shape.bone === s.bone && ["body", "detail"].includes(sh.shape.role) && sh.closed && sh.shape.filled !== false);
  const front = shapes("front"), plan = shapes("plan");
  for (const sec of s.sections) {
    const P = add(add(add(O, U, sec.at[0]), V, sec.at[1]), W, sec.at[2]);
    const candidates: { view: string; score: number; shapes: any[] }[] = [];
    if (front.length) candidates.push({ view: "front", score: Math.abs(U[1]!), shapes: front });
    if (plan.length) candidates.push({ view: "plan", score: Math.abs(U[0]!), shapes: plan });
    candidates.sort((a, b) => b.score - a.score);
    for (const c of candidates) {
      const p0 = project(c.view as any, P[0]!, P[1]!, P[2]!);
      const pz = project(c.view as any, P[0]!, P[1]!, P[2]! + 1);
      const axis: "x" | "y" = Math.abs(pz.x - p0.x) > 0.5 ? "x" : "y";
      const sign = axis === "x" ? Math.sign(pz.x - p0.x) : Math.sign(pz.y - p0.y);
      const other = axis === "x" ? p0.y : p0.x;
      const cur = axis === "x" ? p0.x : p0.y;
      const ivs = intervals(c.shapes, axis, other);
      if (!ivs.length) continue;
      let best = ivs[0]!, bd = Infinity;
      for (const iv of ivs) {
        const dd = cur >= iv[0] && cur <= iv[1] ? 0 : Math.min(Math.abs(cur - iv[0]), Math.abs(cur - iv[1]));
        if (dd < bd) { bd = dd; best = iv; }
      }
      if (bd > 0.05) continue;
      const lo = sign > 0 ? best[0] : -best[1], hi = sign > 0 ? best[1] : -best[0];
      const zc = (lo + hi) / 2, half = Math.max(minr, (hi - lo) / 2);
      const wNew = sec.at[2] + (zc - P[2]!) / (W[2]! || 1);
      if (Math.abs(wNew - sec.at[2]) > maxShift) continue;
      sec.at[2] = +wNew.toFixed(5);
      const rxNew = Math.min(half, sec.rx * growCap + 0.02);
      sec.rx = +Math.max(minr, rxNew).toFixed(5);
      changed++;
      break;
    }
  }
}
const path = sourcePath(entry);
const doc = JSON.parse(readFileSync(path, "utf8"));
doc.volumes[String(scale)].body = body;
writeFileSync(path, JSON.stringify(doc, null, 1));
console.log("snapped sections:", changed);
