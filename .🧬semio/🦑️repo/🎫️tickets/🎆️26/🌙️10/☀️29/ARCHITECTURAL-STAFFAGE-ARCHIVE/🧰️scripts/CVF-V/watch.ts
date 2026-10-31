import { readFileSync, statSync, writeFileSync, existsSync, unlinkSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability, gapsBetween, whiteSliver, featureWidthMm } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { checkAssetDetailed } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
const gen = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CVF-V";
const ids = ["english-oak", "lombardy-poplar", "multi-stem-birch", "scots-pine", "date-palm", "weeping-willow", "hornbeam-hedge", "miscanthus-grass"];
const seen = new Map<string, number>();
function cfg(): any { try { return JSON.parse(readFileSync(`${gen}/cfg.json`, "utf8")); } catch { return {}; } }
function dbg(asset: any, id: string, d: number) {
  const k = 1000 / d;
  const prof = DEFAULT_PRINT_PROFILE;
  for (const pose of asset.poses.filter((p: any) => !p.key)) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: d } });
    const prepared = prepare(state, "elevation", sourceOf(state), k);
    const paper: any[] = [];
    for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") paper.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm });
    const marks: string[] = [];
    let minX = 1e9, minY = 1e9, maxX = -1e9, maxY = -1e9;
    const body: string[] = [];
    for (const p of prepared) {
      if (p.shape.role === "ground") continue;
      for (const ring of p.rings) for (const q of ring) { minX = Math.min(minX, q.x); maxX = Math.max(maxX, q.x); minY = Math.min(minY, q.y); maxY = Math.max(maxY, q.y); }
      if (p.closed && p.shape.filled) body.push(`<path d="${p.rings.map((r: any) => "M" + r.map((q: any) => q.x.toFixed(3) + "," + q.y.toFixed(3)).join("L") + "Z").join("")}" fill="#ddd" stroke="none"/>`);
    }
    for (const s of paper) body.push(`<path d="M${s.rings[0].map((q: any) => q.x.toFixed(3) + "," + q.y.toFixed(3)).join("L")}${s.closed ? "Z" : ""}" fill="none" stroke="#000" stroke-width="${s.penMm}" opacity="0.7"/>`);
    const rr = 0.4 * Math.max(0.2, 1);
    for (const g of gapsBetween(paper, 1)) if (g.gapMm < prof.minGapMm) marks.push(`<circle cx="${g.at.x}" cy="${g.at.y}" r="${rr}" fill="none" stroke="red" stroke-width="0.05"/><text x="${g.at.x + rr}" y="${g.at.y}" font-size="0.5" fill="red">${g.a}+${g.b} ${g.gapMm.toFixed(2)}</text>`);
    for (const p of prepared) if (p.closed && p.shape.filled && p.shape.role !== "ground") { const w = featureWidthMm(p.rings, prof.minFeatureMm); if (w < prof.minFeatureMm) marks.push(`<text x="${(p.rings[0].reduce((a: number, b: any) => a + b.x, 0) / p.rings[0].length).toFixed(2)}" y="${(p.rings[0].reduce((a: number, b: any) => a + b.y, 0) / p.rings[0].length).toFixed(2)}" font-size="0.5" fill="blue">F ${p.shape.id} ${w.toFixed(2)}</text>`); }
    const sl = whiteSliver(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s: any) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), prof.minGapMm);
    if (sl) marks.push(`<circle cx="${sl.at.x}" cy="${sl.at.y}" r="0.6" fill="none" stroke="magenta" stroke-width="0.08"/><text x="${sl.at.x + 0.7}" y="${sl.at.y}" font-size="0.5" fill="magenta">sliver ${sl.widthMm.toFixed(2)}</text>`);
    if (!marks.length) continue;
    const pad = 1.5;
    const sc = 40;
    const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${minX - pad} ${minY - pad} ${maxX - minX + 2 * pad} ${maxY - minY + 2 * pad}" width="${(maxX - minX + 2 * pad) * sc}" height="${(maxY - minY + 2 * pad) * sc}"><rect x="${minX - pad}" y="${minY - pad}" width="100%" height="100%" fill="white"/>${body.join("")}${marks.join("")}</svg>`;
    writeFileSync(`${gen}/dbg/${id}-${d}-${pose.id}.svg`, svg);
  }
}
function run(id: string, full: boolean) {
  const file = `${gen}/work/${id}.json`;
  const lines: string[] = [];
  try {
    const asset = JSON.parse(readFileSync(file, "utf8")).asset;
    const c = cfg();
    if (!c.allPoses) asset.poses = asset.poses.filter((p: any) => !p.key).slice(0, 1);
    const only: number[] = (c.scales && c.scales[id]) || [500, 200, 100, 50];
    if (full) for (const i of validateAsset(asset, {})) lines.push(`VALID ${i.code} ${i.path} ${i.message.en}`);
    for (const d of only) {
      const rep = asset.representations[String(d)];
      if (rep?.status !== "authored") continue;
      const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
      if (!s.suitable && c.dbg !== false) dbg(asset, id, d);
      lines.push(`1:${d} ${s.suitable ? "ok" : "BAD"} stroke ${s.minStrokeMm.toFixed(3)} gap ${s.minGapMm.toFixed(3)} feature ${s.minFeatureMm.toFixed(3)}`);
      for (const v of s.violations) lines.push(`   ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}<${v.requiredMm}${(v as any).at ? " at " + (v as any).at.xMm.toFixed(2) + "," + (v as any).at.yMm.toFixed(2) : ""}`);
    }
    if (full) {
      const r = checkAssetDetailed(asset, { samples: 5, assets: new Set(ids) });
      lines.push(...r.problems.map((p: any) => "CHECK " + p));
      lines.push(r.problems.length ? "FULL ✗" : "FULL ✓");
    }
  } catch (e) { lines.push("ERROR " + (e as Error).message); }
  writeFileSync(`${gen}/m-${id}.txt`, lines.join("\n") + "\n");
}
console.log("watching");
setInterval(() => {
  for (const id of ids) {
    const file = `${gen}/work/${id}.json`;
    if (!existsSync(file)) continue;
    const m = statSync(file).mtimeMs + (JSON.stringify(cfg().scales?.[id] ?? 0).length) + (cfg().allPoses ? 1e9 : 0);
    const fullReq = `${gen}/full-${id}`;
    const full = existsSync(fullReq);
    if (seen.get(id) !== m || full) {
      seen.set(id, m);
      if (full) { try { unlinkSync(fullReq); } catch {} }
      run(id, full);
    }
  }
}, 1500);
