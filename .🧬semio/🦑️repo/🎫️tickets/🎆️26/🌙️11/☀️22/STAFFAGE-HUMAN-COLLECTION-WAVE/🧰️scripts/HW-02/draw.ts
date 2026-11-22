/** 🖍️ Draws the elevation, front and plan of one asset at one scale in a pose or clip instant side by side as a PNG: the drawn shapes in grey by tone with black outlines, the projected body volume as a red outline overlay (VOL=0 hides it).
 *  bun draw.ts <file.json> <pose | clip@time> <scale> <out.png> [pxPerM=400] */
import { join, resolve } from "node:path";
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { worldSolids, referenceOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const [file, q, scaleArg, out, pxArg] = process.argv.slice(2);
const asset = (await Bun.file(file!).json()).asset;
const scale = Number(scaleArg);
const query = q!.includes("@") ? { kind: "clip", clip: q!.split("@")[0], timeS: Number(q!.split("@")[1]) } : { kind: "pose", pose: q };
const state = evaluate(asset, query as never, { scale: { denominator: scale } } as never);
const px = Number(pxArg ?? 400);
const pens = asset.representations[String(scale)].pens;
const solids = process.env.VOL === "0" ? [] : worldSolids(asset, state.bones, scale);
const views = ["elevation", "front", "plan"] as const;
let svg = "", svgD = "", svgV = "", x0 = 20;
let H = 0;
const parts: { view: string; minX: number; maxX: number; minY: number; maxY: number; body: string; mDrawn: string; mVol: string }[] = [];
for (const view of views) {
  const shapes = state.shapes[view] as { shape: any; rings: { x: number; y: number }[][]; closed: boolean }[];
  const vol = solids.length ? referenceOf(view, solids, undefined).full as { x: number; y: number }[][][] : [];
  let [minX, maxX, minY, maxY] = [Infinity, -Infinity, Infinity, -Infinity];
  const acc = (p: { x: number; y: number }) => { minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y); };
  shapes.forEach((s) => s.rings.forEach((r) => r.forEach(acc)));
  vol.forEach((g) => g.forEach((r) => r.forEach(acc)));
  let body = "", mDrawn = "", mVol = "";
  for (const s of [...shapes].sort((p, q) => p.shape.z - q.shape.z)) {
    const pen = (s.shape.penMm ?? pens[s.shape.role] ?? 0.1) * scale / 1000;
    const d = s.rings.map((r) => "M" + r.map((p) => `${p.x.toFixed(4)},${(-p.y).toFixed(4)}`).join("L") + (s.closed ? "Z" : "")).join("");
    const g = Math.round(255 * (1 - 0.75 * (s.shape.tone ?? 0.5)));
    if (s.shape.filled && s.closed && s.shape.role !== "texture") mDrawn += `<path d="${d}" fill="#000" fill-rule="evenodd"/>`;
    body += `<path d="${d}" fill="${s.shape.filled && s.closed ? `rgb(${g},${g},${g})` : "none"}" fill-rule="evenodd" stroke="${s.shape.role === "body" ? "#000" : "#335"}" stroke-width="${pen.toFixed(5)}" stroke-linejoin="round"/>`;
  }
  const ccw = (r: { x: number; y: number }[]) => { let a = 0; for (let i = 0; i < r.length; i++) { const p = r[i]!, q = r[(i + 1) % r.length]!; a += p.x * q.y - q.x * p.y; } return a >= 0 ? r : [...r].reverse(); };
  const chunks: string[] = [];
  for (let i = 0; i < vol.length; i += 400) chunks.push(vol.slice(i, i + 400).map((g) => g.map((r) => "M" + ccw(r).map((p) => `${p.x.toFixed(4)},${(-p.y).toFixed(4)}`).join("L") + "Z").join("")).join(""));
  if (vol.length) mVol = chunks.map((c) => `<path d="${c}" fill="#000" stroke="#000" stroke-width="0.0005"/>`).join("");
  if (vol.length) body += `<g opacity="0.3"><path d="${vol.map((g) => g.map((r) => "M" + r.map((p) => `${p.x.toFixed(4)},${(-p.y).toFixed(4)}`).join("L") + "Z").join("")).join("")}" fill="#e00000" fill-rule="nonzero" stroke="#e00000" stroke-width="0.0008"/></g>`;
  parts.push({ view, minX, maxX, minY, maxY, body, mDrawn, mVol } as never);
  if (process.env.EXT) {
    let [a0, a1, b0, b1] = [Infinity, -Infinity, Infinity, -Infinity];
    shapes.forEach((s) => s.rings.forEach((r) => r.forEach((p) => { a0 = Math.min(a0, p.x); a1 = Math.max(a1, p.x); b0 = Math.min(b0, p.y); b1 = Math.max(b1, p.y); })));
    const who = (f: (p: { x: number; y: number }) => boolean) => shapes.filter((s) => s.rings.some((r) => r.some(f))).map((s) => s.shape.id).join(",");
    console.log(`[DEBUG] ${view} x ${a0.toFixed(3)} (${who((p) => p.x < a0 + 0.002)}) .. ${a1.toFixed(3)} (${who((p) => p.x > a1 - 0.002)})  y ${b0.toFixed(3)} (${who((p) => p.y < b0 + 0.002)}) .. ${b1.toFixed(3)} (${who((p) => p.y > b1 - 0.002)})`);
  }
}
for (const p of parts) {
  const w = (p.maxX - p.minX) * px, h = (p.maxY - p.minY) * px;
  const tr = `<g transform="translate(${x0 - p.minX * px},${20 + p.maxY * px}) scale(${px})">`;
  svg += `${tr}${p.body}</g>`; svgD += `${tr}${p.mDrawn}</g>`; svgV += `${tr}${p.mVol}</g>`;
  svg += `<text x="${x0}" y="14" font-size="12" font-family="sans-serif">${p.view} 1:${scale} ${q}</text>`;
  x0 += w + 40; H = Math.max(H, h);
}
const doc = `<svg xmlns="http://www.w3.org/2000/svg" width="${x0}" height="${H + 40}"><rect width="100%" height="100%" fill="#fff"/>${svg}</svg>`;
if (process.env.DIFF) {
  const mask = async (inner: string) => await sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${x0}" height="${H + 40}"><rect width="100%" height="100%" fill="#fff"/>${inner}</svg>`)).extractChannel(0).raw().toBuffer({ resolveWithObject: true });
  const [a1, b1] = [await mask(svgD), await mask(svgV)];
  const { width, height } = a1.info;
  const px2 = Buffer.alloc(width * height * 3);
  for (let i = 0; i < width * height; i++) {
    const [p, q] = [a1.data[i]! < 128, b1.data[i]! < 128];
    const c = p && q ? [210, 210, 210] : p ? [0, 80, 255] : q ? [255, 0, 0] : [255, 255, 255];
    px2.set(c, i * 3);
  }
  await sharp(px2, { raw: { width, height, channels: 3 } }).png().toFile(out!);
} else await sharp(Buffer.from(doc)).png().toFile(out!);
console.log(out);
