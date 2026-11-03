import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveRig, apply } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/H4";
const [, , idsArg, mode = "states", scalesArg] = process.argv;
for (const id of idsArg!.split(",")) {
const asset = JSON.parse(readFileSync(`${ROOT}/${id}/🔣️.json`, "utf8")).asset;
for (const rep of Object.values<any>(asset.representations)) if (rep.geometry) { rep.geometry.front ??= []; rep.geometry.plan ??= []; }
const f = (n: number) => Math.round(n * 10000) / 10000;
type P = { x: number; y: number };
const sel = (d: number) => ({ kind: "scale", scale: { denominator: d } }) as any;
const scales: number[] = (scalesArg ? scalesArg.split(",").map(Number) : Object.keys(asset.representations).filter((k) => asset.representations[k].status === "authored").map(Number));
mkdirSync(`${OUT}/${id}`, { recursive: true });

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

function queries(perClip: number): { name: string; q: any }[] {
  const out: { name: string; q: any }[] = asset.poses.map((p: any) => ({ name: `pose:${p.id}`, q: { kind: "pose", pose: p.id } }));
  for (const c of asset.clips) for (let i = 0; i < perClip; i++) out.push({ name: `clip:${c.id}@${f((c.durationS * i) / perClip)}`, q: { kind: "clip", clip: c.id, timeS: (c.durationS * i) / perClip } });
  return out;
}

if (mode === "rest") {
  const rest = solveRig(asset.rig, {}, { x: 0, y: 0, angleDeg: 0 });
  const rows: any = {};
  for (const b of asset.rig.bones) {
    const o = apply(rest[b.id]!, 0, 0), t = apply(rest[b.id]!, b.lengthM, 0);
    rows[b.id] = { o: [f(o.x), f(o.y)], t: [f(t.x), f(t.y)] };
  }
  writeFileSync(`${OUT}/${id}/rest.json`, JSON.stringify(rows));
  const dflt = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel(100));
  const dp: any = {};
  for (const b of asset.rig.bones) dp[b.id] = { o: [f(dflt.bones[b.id]!.origin.x), f(dflt.bones[b.id]!.origin.y)], t: [f(dflt.bones[b.id]!.tip.x), f(dflt.bones[b.id]!.tip.y)] };
  writeFileSync(`${OUT}/${id}/default-pose.json`, JSON.stringify(dp));
  const poses: any = {};
  for (const q of queries(4)) {
    const s = evaluate(asset, q.q, sel(100));
    const o: any = {};
    for (const b of asset.rig.bones) o[b.id] = { o: [f(s.bones[b.id]!.origin.x), f(s.bones[b.id]!.origin.y)], t: [f(s.bones[b.id]!.tip.x), f(s.bones[b.id]!.tip.y)] };
    poses[q.name] = o;
  }
  writeFileSync(`${OUT}/${id}/poses.json`, JSON.stringify(poses));
}

if (mode === "states" || mode === "all") {
  const perClip = Number(process.argv[4] ?? 3);
  for (const sc of scales) {
    const states: any[] = [];
    for (const q of queries(perClip)) {
      const s = evaluate(asset, q.q, sel(sc));
      const views: any = {};
      for (const v of ["elevation", "front", "plan"] as const) {
        views[v] = s.shapes[v].map((e: any) => ({ id: e.shape.id, role: e.shape.role, z: e.shape.z, filled: e.shape.filled, tone: e.shape.tone, closed: e.closed, rings: e.rings.map((r: P[]) => r.map((p) => [f(p.x), f(p.y)])) }));
      }
      states.push({ name: q.name, views });
    }
    writeFileSync(`${OUT}/${id}/states-${sc}.json`, JSON.stringify(states));
  }
}

if (mode === "check" || mode === "all") {
  const dims = asset.dimensions;
  for (const sc of scales) {
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
    let nbad = 0;
    const qs = queries(9);
    for (const q of qs) {
      const s = evaluate(asset, q.q, sel(sc));
      for (const v of ["elevation", "front", "plan"] as const) {
        const e = ext(s.shapes[v]);
        if (e.n === 0 || ![e.minX, e.maxX, e.minY, e.maxY].every(Number.isFinite)) { nbad++; console.log("   EMPTY/NONFINITE", v, JSON.stringify(q.q)); }
      }
    }
    if (nbad === 0) console.log(`   all ${qs.length} poses/clip samples non-empty finite`);
  }
}
}
