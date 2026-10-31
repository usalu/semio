import { readFileSync, statSync, writeFileSync, existsSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { gapsBetween, whiteSliver, featureWidthMm, squaredDistanceTransform } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { checkAssetDetailed } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const gen = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-H2";
const ids = ["skateboarder", "kneeling-gardener", "parent-carrying-toddler", "seated-guitarist"];
const seen = new Map<string, number>();
function rasterize(rings: any[], pitch: number, minX: number, minY: number, nx: number, ny: number): Uint8Array {
  const mask = new Uint8Array(nx * ny);
  for (let row = 0; row < ny; row++) {
    const y = minY + (row + 0.5) * pitch;
    const xs: number[] = [];
    for (const ring of rings)
      for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
        const a = ring[i], b = ring[j];
        if (a.y > y !== b.y > y) xs.push(a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x));
      }
    xs.sort((p, q) => p - q);
    for (let k = 0; k + 1 < xs.length; k += 2) {
      const from = Math.max(0, Math.ceil((xs[k] - minX) / pitch - 0.5));
      const to = Math.min(nx - 1, Math.floor((xs[k + 1] - minX) / pitch - 0.5));
      for (let col = from; col <= to; col++) mask[row * nx + col] = 1;
    }
  }
  return mask;
}
function segDist(px: number, py: number, a: any, b: any): number {
  const dx = b.x - a.x, dy = b.y - a.y, l2 = dx * dx + dy * dy;
  const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((px - a.x) * dx + (py - a.y) * dy) / l2));
  return Math.hypot(px - (a.x + t * dx), py - (a.y + t * dy));
}
function sliverCells(fills: any[][], strokes: any[], minGap: number): { x: number; y: number; pitch: number }[] {
  let minX = 1e9, minY = 1e9, maxX = -1e9, maxY = -1e9, pen = 0;
  for (const ring of fills.flat()) for (const p of ring) { minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y); }
  for (const st of strokes) { pen = Math.max(pen, st.penMm); for (const p of st.points) { minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y); } }
  const margin = 2 * minGap + pen;
  const pitch = Math.max(minGap / 6, Math.sqrt(((maxX - minX + 2 * margin) * (maxY - minY + 2 * margin)) / 2.5e6));
  const x0 = minX - margin, y0 = minY - margin;
  const nx = Math.ceil((maxX - minX + 2 * margin) / pitch) + 1, ny = Math.ceil((maxY - minY + 2 * margin) / pitch) + 1;
  const ink = new Uint8Array(nx * ny);
  for (const rings of fills) { const m = rasterize(rings, pitch, x0, y0, nx, ny); for (let i = 0; i < ink.length; i++) if (m[i] === 1) ink[i] = 1; }
  for (const st of strokes) {
    const half = st.penMm / 2, n = st.points.length, count = st.closed && n > 2 ? n : n - 1;
    for (let i = 0; i < count; i++) {
      const a = st.points[i], b = st.points[(i + 1) % n];
      const cx0 = Math.max(0, Math.floor((Math.min(a.x, b.x) - half - x0) / pitch - 0.5)), cx1 = Math.min(nx - 1, Math.ceil((Math.max(a.x, b.x) + half - x0) / pitch - 0.5));
      const cy0 = Math.max(0, Math.floor((Math.min(a.y, b.y) - half - y0) / pitch - 0.5)), cy1 = Math.min(ny - 1, Math.ceil((Math.max(a.y, b.y) + half - y0) / pitch - 0.5));
      for (let cy = cy0; cy <= cy1; cy++) for (let cx = cx0; cx <= cx1; cx++) if (segDist(x0 + (cx + 0.5) * pitch, y0 + (cy + 0.5) * pitch, a, b) <= half) ink[cy * nx + cx] = 1;
    }
  }
  const toInk = squaredDistanceTransform(ink, nx, ny);
  const radius = new Float64Array(ink.length);
  for (let i = 0; i < ink.length; i++) if (ink[i] === 0) radius[i] = Math.max(0, (Math.sqrt(toInk[i]) - 0.5) * pitch);
  const r = minGap / 2;
  const core = new Uint8Array(ink.length);
  for (let i = 0; i < ink.length; i++) if (ink[i] === 0 && radius[i] >= r) core[i] = 1;
  const reach = squaredDistanceTransform(core, nx, ny);
  const limit = ((r + pitch * 0.5) / pitch) ** 2;
  const out: { x: number; y: number; pitch: number }[] = [];
  for (let i = 0; i < ink.length; i++) if (ink[i] === 0 && reach[i] > limit) out.push({ x: x0 + ((i % nx) + 0.5) * pitch, y: y0 + (Math.floor(i / nx) + 0.5) * pitch, pitch });
  return out;
}
function dbg(asset: any, id: string, d: number) {
  const rep = asset.representations[String(d)];
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
    for (const g of gapsBetween(paper, 1)) if (g.gapMm < prof.minGapMm) marks.push(`<circle cx="${g.at.x}" cy="${g.at.y}" r="0.12" fill="none" stroke="red" stroke-width="0.04"/><text x="${g.at.x + 0.15}" y="${g.at.y}" font-size="0.18" fill="red">${g.a}+${g.b}</text>`);
    for (const p of prepared) if (p.closed && p.shape.filled && p.shape.role !== "ground") { const w = featureWidthMm(p.rings, prof.minFeatureMm); if (w < prof.minFeatureMm) { const q = p.rings[0][0]; marks.push(`<text x="${(p.rings[0].reduce((a: number, b: any) => a + b.x, 0) / p.rings[0].length).toFixed(2)}" y="${(p.rings[0].reduce((a: number, b: any) => a + b.y, 0) / p.rings[0].length).toFixed(2)}" font-size="0.2" fill="blue">F ${p.shape.id} ${w.toFixed(2)}</text>`); } }
    const sl = whiteSliver(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s: any) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), prof.minGapMm);
    if (sl) marks.push(`<circle cx="${sl.at.x}" cy="${sl.at.y}" r="0.25" fill="none" stroke="magenta" stroke-width="0.05"/><text x="${sl.at.x + 0.3}" y="${sl.at.y}" font-size="0.2" fill="magenta">sliver ${sl.widthMm.toFixed(2)}</text>`);
    for (const c of sliverCells(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s: any) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), prof.minGapMm)) marks.push(`<rect x="${c.x - c.pitch / 2}" y="${c.y - c.pitch / 2}" width="${c.pitch}" height="${c.pitch}" fill="magenta" opacity="0.8"/>`);
    const pad = 0.6;
    const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${minX - pad} ${minY - pad} ${maxX - minX + 2 * pad} ${maxY - minY + 2 * pad}" width="${(maxX - minX + 2 * pad) * 60}" height="${(maxY - minY + 2 * pad) * 60}"><rect x="${minX - pad}" y="${minY - pad}" width="100%" height="100%" fill="white"/>${body.join("")}${marks.join("")}</svg>`;
    writeFileSync(`${gen}/dbg-${id}-${d}-${pose.id}.svg`, svg);
  }
}
function run(id: string, full: boolean) {
  const file = `${root}/${id}/🔣️.json`;
  const lines: string[] = [];
  try {
    const asset = JSON.parse(readFileSync(file, "utf8")).asset;
    for (const i of validateAsset(asset, {})) lines.push(`VALID ${i.code} ${i.path} ${i.message.en}`);
    for (const d of [500, 200, 100, 50]) {
      const rep = asset.representations[String(d)];
      if (rep?.status !== "authored") continue;
      const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
      if (existsSync(`${gen}/dbg-${id}-${d}`)) dbg(asset, id, d);
      lines.push(`1:${d} ${s.suitable ? "ok" : "BAD"} stroke ${s.minStrokeMm.toFixed(3)} gap ${s.minGapMm.toFixed(3)} feature ${s.minFeatureMm.toFixed(3)}`);
      for (const v of s.violations) lines.push(`   ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}<${v.requiredMm}${(v as any).at ? " at " + (v as any).at.xMm.toFixed(2) + "," + (v as any).at.yMm.toFixed(2) : ""}`);
    }
    if (full) {
      const r = checkAssetDetailed(asset, { samples: 5, assets: new Set(ids) });
      lines.push(...r.problems.map((p) => "CHECK " + p));
      lines.push(r.problems.length ? "FULL ✗" : "FULL ✓");
    }
  } catch (e) { lines.push("ERROR " + (e as Error).message); }
  writeFileSync(`${gen}/m-${id}.txt`, lines.join("\n") + "\n");
}
console.log("watching");
setInterval(() => {
  for (const id of ids) {
    const file = `${root}/${id}/🔣️.json`;
    if (!existsSync(file)) continue;
    const m = statSync(file).mtimeMs;
    const fullReq = `${gen}/full-${id}`;
    const full = existsSync(fullReq);
    if (seen.get(id) !== m || full) {
      seen.set(id, m);
      if (full) { try { require("node:fs").unlinkSync(fullReq); } catch {} }
      run(id, full);
    }
  }
}, 1500);
