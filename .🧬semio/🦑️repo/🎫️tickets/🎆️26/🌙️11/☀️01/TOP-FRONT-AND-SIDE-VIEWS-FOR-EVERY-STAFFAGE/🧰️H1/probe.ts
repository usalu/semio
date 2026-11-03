import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveRig, apply } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const [, , id, mode = "info", arg] = process.argv;
const asset = JSON.parse(readFileSync(`${ROOT}/${id}/🔣️.json`, "utf8")).asset;
const f = (n: number) => (Math.round(n * 1000) / 1000).toString();
type P = { x: number; y: number };

function ext(shapes: any[]) {
  const e = { minX: 1e9, maxX: -1e9, minY: 1e9, maxY: -1e9, n: 0 };
  for (const s of shapes) {
    if (s.shape.role === "ground") continue;
    for (const r of s.rings) for (const p of r as P[]) {
      e.n++;
      e.minX = Math.min(e.minX, p.x); e.maxX = Math.max(e.maxX, p.x); e.minY = Math.min(e.minY, p.y); e.maxY = Math.max(e.maxY, p.y);
    }
  }
  return e;
}
const fe = (e: any) => `x[${f(e.minX)},${f(e.maxX)}] y[${f(e.minY)},${f(e.maxY)}]`;
const sel = (d: number) => ({ kind: "scale", scale: { denominator: d } }) as any;
const scales = Object.keys(asset.representations).filter((k) => asset.representations[k].status === "authored").map(Number);

if (mode === "info") {
  console.log(JSON.stringify(asset.dimensions), "default", asset.presentation.defaultPose, asset.presentation.defaultClip);
  const rest = solveRig(asset.rig, {}, { x: 0, y: 0, angleDeg: 0 });
  for (const b of asset.rig.bones) {
    const o = apply(rest[b.id]!, 0, 0), t = apply(rest[b.id]!, b.lengthM, 0);
    console.log(`${b.id.padEnd(14)} parent=${String(b.parent).padEnd(12)} rest origin(${f(o.x)},${f(o.y)}) tip(${f(t.x)},${f(t.y)}) len=${b.lengthM}`);
  }
  console.log("poses", asset.poses.map((p: any) => p.id).join(","));
  console.log("clips", asset.clips.map((c: any) => `${c.id}(${c.durationS}s,${c.rootMotion})`).join(","));
  console.log("scales", scales.join(","));
}
if (mode === "bones") {
  const pose = arg ?? asset.presentation.defaultPose;
  const st = evaluate(asset, { kind: "pose", pose }, sel(100));
  for (const b of asset.rig.bones) { const e = st.bones[b.id]!; console.log(`${b.id.padEnd(14)} o(${f(e.origin.x)},${f(e.origin.y)}) t(${f(e.tip.x)},${f(e.tip.y)})`); }
}
if (mode === "elev") {
  const pose = process.argv[5] ?? asset.presentation.defaultPose;
  const st = evaluate(asset, { kind: "pose", pose }, sel(Number(arg)));
  for (const s of st.shapes.elevation) {
    const e = ext([s]);
    console.log(`${s.shape.id.padEnd(22)} ${s.shape.bone.padEnd(11)} ${s.shape.role.padEnd(7)} z${s.shape.z} ${fe(e)} ${(s.shape as any).region ?? (s.shape as any).regions ?? ""}`);
  }
}
if (mode === "check") {
  const dims = asset.dimensions;
  const only = arg ? [Number(arg)] : scales;
  for (const sc of only) {
    const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel(sc));
    const el = ext(st.shapes.elevation), fr = ext(st.shapes.front), pl = ext(st.shapes.plan);
    const tol = Math.max(0.02, 0.03 * Math.max(dims.widthM, dims.depthM, dims.heightM));
    const bad: string[] = [];
    const cmp = (n: string, a0: number, a1: number, b0: number, b1: number) => { if (Math.abs(a0 - b0) > tol || Math.abs(a1 - b1) > tol) bad.push(`${n} [${f(a0)},${f(a1)}] vs [${f(b0)},${f(b1)}]`); };
    cmp("elev.x=plan.x", el.minX, el.maxX, pl.minX, pl.maxX);
    cmp("elev.y=front.y", el.minY, el.maxY, fr.minY, fr.maxY);
    cmp("front.x=plan.y", fr.minX, fr.maxX, pl.minY, pl.maxY);
    if (Math.abs(el.maxX - el.minX - dims.widthM) > tol) bad.push(`width ${f(el.maxX - el.minX)} vs ${dims.widthM}`);
    if (Math.abs(fr.maxX - fr.minX - dims.depthM) > tol) bad.push(`front width ${f(fr.maxX - fr.minX)} vs depth ${dims.depthM}`);
    if (Math.abs(pl.maxY - pl.minY - dims.depthM) > tol) bad.push(`plan depth ${f(pl.maxY - pl.minY)} vs depth ${dims.depthM}`);
    if (Math.abs(el.maxY - el.minY - dims.heightM) > tol) bad.push(`height ${f(el.maxY - el.minY)} vs ${dims.heightM}`);
    console.log(`1:${sc} tol=${f(tol)} elev ${fe(el)} | front ${fe(fr)} | plan ${fe(pl)}${bad.length ? "\n   BAD " + bad.join("\n   BAD ") : " ok"}`);
    const queries: any[] = asset.poses.map((p: any) => ({ kind: "pose", pose: p.id }));
    for (const c of asset.clips) for (let i = 0; i < 9; i++) queries.push({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / 9 });
    let nbad = 0;
    for (const q of queries) {
      const s = evaluate(asset, q, sel(sc));
      for (const v of ["elevation", "front", "plan"] as const) {
        const e = ext(s.shapes[v]);
        if (e.n === 0 || ![e.minX, e.maxX, e.minY, e.maxY].every(Number.isFinite)) { nbad++; console.log("   EMPTY/NONFINITE", v, JSON.stringify(q)); }
      }
    }
    if (nbad === 0) console.log(`   all ${queries.length} poses/clip samples non-empty finite`);
  }
}
