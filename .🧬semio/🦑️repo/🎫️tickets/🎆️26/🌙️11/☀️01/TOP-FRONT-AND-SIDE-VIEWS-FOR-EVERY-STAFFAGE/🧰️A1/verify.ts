/** [DEBUG] A1 verification: orthographic consistency at the default pose, non-empty finite views for every pose and clip sample, and ring dumps for the python renderer.
 * usage: bun verify.ts <asset> [dumpScales comma list] */
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../../..");
const S = join(root, "🎓️teaching/🏛️architecture/🧍️staffage");
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { checkAssetDetailed } = await import(join(S, "🔨️modules/🗂️catalog/🩺️check/🟦️.ts"));
const out = join(import.meta.dir, "../🗑️generated/A1");
const VIEWS = ["elevation", "front", "plan"] as const;

function ext(shapes: any[]) {
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const s of shapes) {
    if (s.shape.role === "ground") continue;
    for (const ring of s.rings) for (const p of ring) {
      if (!Number.isFinite(p.x) || !Number.isFinite(p.y)) return null;
      if (p.x < minX) minX = p.x; if (p.x > maxX) maxX = p.x; if (p.y < minY) minY = p.y; if (p.y > maxY) maxY = p.y;
    }
  }
  return Number.isFinite(minX) ? { minX, minY, maxX, maxY } : null;
}

const ids = process.argv[2]!.split(",");
const dumpScales = (process.argv[3] ?? "").split(",").filter(Boolean).map(Number);
const lines: string[] = [];
const log = (s: string) => { lines.push(s); console.log(s); };

for (const id of ids) {
  const asset = JSON.parse(readFileSync(join(S, "🗂️catalog/🐕️animals", id, "🔣️.json"), "utf8")).asset;
  const { widthM, depthM, heightM } = asset.dimensions;
  const tol = Math.max(0.02, 0.03 * Math.max(widthM, depthM, heightM));
  log(`== ${id} dims w${widthM} d${depthM} h${heightM} tol ${tol.toFixed(3)}`);
  const queries: any[] = [];
  for (const p of asset.poses) queries.push({ label: p.id, key: !!p.key, q: { kind: "pose", pose: p.id } });
  for (const c of asset.clips) for (let i = 0; i < 8; i++) queries.push({ label: `${c.id}@${(c.durationS * i / 8).toFixed(2)}`, key: true, q: { kind: "clip", clip: c.id, timeS: c.durationS * i / 8 + 1e-6 } });
  for (const [sc, rep] of Object.entries<any>(asset.representations)) {
    if (rep.status !== "authored") continue;
    const sel = { kind: "scale", scale: { denominator: Number(sc) } };
    const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel);
    const e = ext(st.shapes.elevation), f = ext(st.shapes.front), p = ext(st.shapes.plan);
    if (!e || !f || !p) { log(`1:${sc} EMPTY default pose e=${!!e} f=${!!f} p=${!!p}`); continue; }
    const chk = (name: string, a: number, b: number) => { const d = a - b; log(`1:${sc} ${name}: ${a.toFixed(3)} vs ${b.toFixed(3)} diff ${d.toFixed(3)} ${Math.abs(d) <= tol ? "ok" : "FAIL"}`); };
    chk("elev.x==plan.x min", e.minX, p.minX); chk("elev.x==plan.x max", e.maxX, p.maxX);
    chk("elev.y==front.y min", e.minY, f.minY); chk("elev.y==front.y max", e.maxY, f.maxY);
    chk("front.x==plan.y min", f.minX, p.minY); chk("front.x==plan.y max", f.maxX, p.maxY);
    chk("width", e.maxX - e.minX, widthM); chk("height", e.maxY - e.minY, heightM); chk("depth", f.maxX - f.minX, depthM); chk("depth(plan)", p.maxY - p.minY, depthM);
    let bad = 0;
    const dump: any[] = [];
    for (const { label, key, q } of queries) {
      let s: any;
      try { s = evaluate(asset, q, sel); } catch (err) { log(`1:${sc} ${label} ERROR ${String(err).slice(0, 120)}`); bad++; continue; }
      const ex = VIEWS.map((v) => ext(s.shapes[v]));
      if (ex.some((x) => x === null)) { log(`1:${sc} ${label} EMPTY/NONFINITE ${ex.map((x) => !!x)}`); bad++; }
      if (dumpScales.includes(Number(sc))) dump.push({ label, views: Object.fromEntries(VIEWS.map((v) => [v, s.shapes[v].map((x: any) => ({ id: x.shape.id, z: x.shape.z, filled: x.shape.filled, tone: x.shape.tone, closed: x.closed, role: x.shape.role, rings: x.rings.map((r: any[]) => r.map((pt) => [Math.round(pt.x * 1e4) / 1e4, Math.round(pt.y * 1e4) / 1e4])) }))])) });
    }
    log(`1:${sc} queries ${queries.length} bad ${bad}`);
    if (dump.length) writeFileSync(join(out, `${id}-dump-${sc}.json`), JSON.stringify(dump));
  }
}
for (const id of ids) {
  const asset = JSON.parse(readFileSync(join(S, "🗂️catalog/🐕️animals", id, "🔣️.json"), "utf8")).asset;
  const report = checkAssetDetailed(asset, { samples: 9, physics: false });
  log(`CHECK ${id}: ${report.problems.length ? "PROBLEMS" : "ok"}`);
  for (const l of report.lines) log("  " + l.slice(0, 200));
  for (const p of report.problems) log("  ! " + p.slice(0, 300));
}
writeFileSync(join(out, `${ids[0]}-verify.txt`), lines.join("\n") + "\n");
