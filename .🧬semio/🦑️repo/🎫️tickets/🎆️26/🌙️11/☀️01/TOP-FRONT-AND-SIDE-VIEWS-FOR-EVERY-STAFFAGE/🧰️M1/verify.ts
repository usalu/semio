import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery";
const ids = process.argv.slice(2);
const VIEWS = ["elevation", "front", "plan"] as const;

type Ext = { minX: number; maxX: number; minY: number; maxY: number; n: number };
function ext(shapes: any[]): Ext {
  const e: Ext = { minX: Infinity, maxX: -Infinity, minY: Infinity, maxY: -Infinity, n: 0 };
  for (const s of shapes) {
    if (s.shape.role === "ground") continue;
    for (const ring of s.rings) for (const p of ring) {
      e.n++;
      e.minX = Math.min(e.minX, p.x); e.maxX = Math.max(e.maxX, p.x);
      e.minY = Math.min(e.minY, p.y); e.maxY = Math.max(e.maxY, p.y);
    }
  }
  return e;
}
const f = (v: number) => v.toFixed(3);
let bad = 0;
for (const id of ids) {
  const asset = (JSON.parse(readFileSync(`${root}/${id}/🔣️.json`, "utf8"))).asset;
  const d = asset.dimensions;
  console.log(`== ${id} dims w=${d.widthM} d=${d.depthM} h=${d.heightM}`);
  for (const sc of ["500", "200", "100", "50"]) {
    const rep = asset.representations[sc];
    if (rep.status !== "authored") continue;
    const sel = { kind: "scale", scale: { denominator: Number(sc) } } as any;
    const tol = Math.max(0.02, 0.03 * Math.max(d.widthM, d.depthM, d.heightM));
    const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel);
    const E = ext(st.shapes.elevation), F = ext(st.shapes.front), P = ext(st.shapes.plan);
    const checks: [string, number, number][] = [
      ["E.x vs P.x min", E.minX, P.minX], ["E.x vs P.x max", E.maxX, P.maxX],
      ["E.y vs F.y min", E.minY, F.minY], ["E.y vs F.y max", E.maxY, F.maxY],
      ["F.x vs P.y min", F.minX, P.minY], ["F.x vs P.y max", F.maxX, P.maxY],
    ];
    const probs = checks.filter(([, a, b]) => !(Math.abs(a - b) <= tol)).map(([n, a, b]) => `${n}: ${f(a)} vs ${f(b)}`);
    if (Math.abs((E.maxX - E.minX) - d.widthM) > tol) probs.push(`E width ${f(E.maxX - E.minX)} != ${d.widthM}`);
    if (Math.abs((P.maxY - P.minY) - d.depthM) > tol) probs.push(`P depth ${f(P.maxY - P.minY)} != ${d.depthM}`);
    if (Math.abs((E.maxY - E.minY) - d.heightM) > tol) probs.push(`E height ${f(E.maxY - E.minY)} != ${d.heightM}`);
    console.log(`  1:${sc} tol ${f(tol)} E[${f(E.minX)},${f(E.maxX)}]x[${f(E.minY)},${f(E.maxY)}] F[${f(F.minX)},${f(F.maxX)}]x[${f(F.minY)},${f(F.maxY)}] P[${f(P.minX)},${f(P.maxX)}]x[${f(P.minY)},${f(P.maxY)}] ${probs.length ? "PROBLEMS " + probs.join("; ") : "ok"}`);
    if (probs.length) bad++;
    const queries: any[] = [];
    for (const p of asset.poses) queries.push({ kind: "pose", pose: p.id });
    for (const c of asset.clips) for (let i = 0; i <= 16; i++) queries.push({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / 16 });
    for (const q of queries) {
      const s = evaluate(asset, q, sel);
      for (const v of VIEWS) {
        const e = ext(s.shapes[v]);
        if (e.n === 0 || ![e.minX, e.maxX, e.minY, e.maxY].every(Number.isFinite)) { console.log(`   EMPTY/NONFINITE ${v} ${JSON.stringify(q)}`); bad++; }
      }
    }
    if (process.env.RANGES) {
      for (const q of queries) {
        const s = evaluate(asset, q, sel);
        const t = VIEWS.map((v) => { const e = ext(s.shapes[v]); return `${v[0]}[${f(e.minX)},${f(e.maxX)}|${f(e.minY)},${f(e.maxY)}]`; }).join(" ");
        console.log(`     ${q.kind === "pose" ? q.pose : q.clip + "@" + q.timeS.toFixed(2)} ${t}`);
      }
    }
  }
}
console.log(bad ? `PROBLEMS: ${bad}` : "ALL OK");
