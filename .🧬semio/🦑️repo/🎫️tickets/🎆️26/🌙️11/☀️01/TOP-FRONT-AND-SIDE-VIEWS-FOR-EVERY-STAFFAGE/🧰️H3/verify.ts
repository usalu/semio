import { readFileSync, writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const load = (p: string) => import(pathToFileURL(`${S}/${p}`).href);
const { evaluate } = await load("🔨️modules/🧮️evaluation/🟦️.ts");
const { validateAsset } = await load("🔨️modules/🗂️catalog/🟦️.ts");
const { DEFAULT_PRINT_PROFILE, measureSuitability } = await load("🔨️modules/🖨️representation/🟦️.ts");

const ids = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const detail = process.argv.includes("--detail");
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/H3";
const lines: string[] = [];
const log = (s: string) => lines.push(s);

type P = { x: number; y: number };
function extent(shapes: any[]) {
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const e of shapes) {
    if (e.shape.role === "ground") continue;
    for (const ring of e.rings) for (const p of ring as P[]) {
      if (p.x < minX) minX = p.x; if (p.x > maxX) maxX = p.x; if (p.y < minY) minY = p.y; if (p.y > maxY) maxY = p.y;
    }
  }
  return { minX, minY, maxX, maxY };
}
const f = (v: number) => (Number.isFinite(v) ? v.toFixed(3) : "NaN");
const box = (b: any) => `x[${f(b.minX)},${f(b.maxX)}] y[${f(b.minY)},${f(b.maxY)}]`;

for (const id of ids) {
  const path = `${S}/🗂️catalog/👤️humans/${id}/🔣️.json`;
  const asset = JSON.parse(readFileSync(path, "utf8")).asset;
  const issues = validateAsset(asset, {});
  log(`##### ${id}`);
  for (const i of issues) log(`ISSUE ${i.code} ${i.path}: ${i.message.en}`);
  const scales = Object.entries(asset.representations).filter(([, r]: any) => r.status === "authored").map(([k]) => Number(k));
  for (const d of scales) {
    try {
      const su = measureSuitability(asset, asset.representations[`${d}`], d, DEFAULT_PRINT_PROFILE);
      log(`1:${d} suitability ${su.suitable ? "ok" : "UNSUITABLE"} stroke ${su.minStrokeMm.toFixed(3)} gap ${su.minGapMm.toFixed(3)} feat ${su.minFeatureMm.toFixed(3)}` + su.violations.slice(0, 5).map((v: any) => `\n   ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}<${v.requiredMm}`).join(""));
    } catch (e) { log(`1:${d} suitability error ${(e as Error).message}`); }
    const dim = asset.dimensions;
    const sel = { kind: "scale", scale: { denominator: d } };
    const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel);
    const e = extent(st.shapes.elevation), fr = extent(st.shapes.front), pl = extent(st.shapes.plan);
    const tol = Math.max(0.02, 0.03 * Math.max(dim.widthM, dim.depthM, dim.heightM));
    const w = (a: number, b: number) => Math.abs(a - b) <= tol;
    const bad: string[] = [];
    if (!(w(e.minX, pl.minX) && w(e.maxX, pl.maxX))) bad.push("elev.x!=plan.x");
    if (!(w(e.minY, fr.minY) && w(e.maxY, fr.maxY))) bad.push("elev.y!=front.y");
    if (!(w(fr.minX, pl.minY) && w(fr.maxX, pl.maxY))) bad.push("front.x!=plan.y");
    if (!w(e.maxX - e.minX, dim.widthM)) bad.push(`width ${f(e.maxX - e.minX)}!=${dim.widthM}`);
    if (!w(fr.maxX - fr.minX, dim.depthM)) bad.push(`depth ${f(fr.maxX - fr.minX)}!=${dim.depthM}`);
    if (!w(e.maxY - e.minY, dim.heightM)) bad.push(`height ${f(e.maxY - e.minY)}!=${dim.heightM}`);
    log(`1:${d} default ${asset.presentation.defaultPose}: elev ${box(e)} | front ${box(fr)} | plan ${box(pl)} tol ${tol.toFixed(3)} ${bad.length ? "MISMATCH " + bad.join(", ") : "ok"}`);
    const queries: any[] = [
      ...asset.poses.filter((p: any) => !p.key).map((p: any) => ({ kind: "pose", pose: p.id })),
      ...asset.clips.flatMap((c: any) => Array.from({ length: 9 }, (_, i) => ({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / 8 }))),
    ];
    const problems: string[] = [];
    for (const q of queries) {
      try {
        const s = evaluate(asset, q, sel);
        const label = q.kind === "pose" ? q.pose : `${q.clip}@${q.timeS.toFixed(2)}`;
        for (const vp of ["elevation", "front", "plan"] as const) {
          const b = extent(s.shapes[vp]);
          if (!Number.isFinite(b.minX) || !Number.isFinite(b.maxY)) problems.push(`${label} ${vp} empty/non-finite`);
          else if (detail || (vp !== "elevation" && q.kind === "pose")) log(`   ${label} ${vp} ${box(b)}`);
        }
      } catch (err) { problems.push(`${q.kind === "pose" ? q.pose : q.clip}: ${(err as Error).message}`); }
    }
    if (problems.length) log(`   PROBLEMS ${problems.slice(0, 12).join("; ")}`);
  }
}
writeFileSync(`${OUT}/verify-${ids.join("+").slice(0, 80)}.txt`, lines.join("\n"));
console.log(lines.join("\n"));
