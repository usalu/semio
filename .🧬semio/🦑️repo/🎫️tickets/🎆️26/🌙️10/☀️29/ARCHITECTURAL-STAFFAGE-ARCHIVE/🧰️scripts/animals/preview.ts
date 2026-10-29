// [scratch] animals preview: renders poses and clip instants of one asset to an SVG sheet (rsvg-convert makes the PNG)
import { readFileSync, writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { depict, defaultDepictionOptions } = await import(`${S}/🔨️modules/🎨️depiction/🟦️.ts`);
const { solveThermal } = await import(`${S}/🔨️modules/🌡️thermal/🟦️.ts`);
const { solveStructure } = await import(`${S}/🔨️modules/🏗️structure/🟦️.ts`);
const { values } = parseArgs({ options: { file: { type: "string" }, asset: { type: "string" }, out: { type: "string" }, view: { type: "string", default: "light-solid" }, scale: { type: "string", default: "50" }, clip: { type: "string" }, n: { type: "string", default: "8" }, cols: { type: "string", default: "3" }, pose: { type: "string" } } });
const doc = JSON.parse(readFileSync(values.file!, "utf8"));
const asset = (doc.assets ?? [doc]).find((a: any) => a.id === values.asset);
const cmd = (c: any) => c.op === "Z" ? "Z" : c.op === "C" ? `C${c.x1} ${c.y1} ${c.x2} ${c.y2} ${c.x} ${c.y}` : `${c.op}${c.x} ${c.y}`;
function svgOf(scene: any): string {
  let s = "";
  for (const layer of scene.layers) for (const it of layer.items) {
    if (it.kind === "text") { s += `<text x="${it.x}" y="${it.y}" font-size="${it.sizeMm}" fill="${it.color}" text-anchor="${it.anchor}">${it.text}</text>`; continue; }
    const f = it.fill ? `fill="${it.fill.color}" fill-opacity="${it.fill.opacity ?? 1}"` : `fill="none"`;
    const st = it.stroke ? `stroke="${it.stroke.color}" stroke-width="${it.stroke.widthMm}" stroke-linejoin="round"` : "";
    s += `<path d="${it.commands.map(cmd).join(" ")}" ${f} ${st}/>`;
  }
  return s;
}
const queries: { label: string; q: any }[] = [];
if (values.clip) { const clip = asset.clips.find((c: any) => c.id === values.clip); const n = Number(values.n); for (let i = 0; i < n; i++) queries.push({ label: `${clip.id} ${(clip.durationS * i / n).toFixed(2)}s`, q: { kind: "clip", clip: clip.id, timeS: clip.durationS * i / n } }); }
else for (const p of asset.poses) if (!values.pose || values.pose === p.id) queries.push({ label: p.id, q: { kind: "pose", pose: p.id } });
const denominators = values.scale!.split(",").map(Number);
const cols = Number(values.cols);
let x = 0, y = 0, rowH = 0, col = 0, out = "", W = 0;
const gap = 4;
for (const d of denominators) for (const { label, q } of queries) {
  const state = evaluate(asset, q);
  const opts = defaultDepictionOptions(values.view as any, "en", d);
  opts.annotations = { dimensions: false, scaleBar: false, legend: false, labels: false, ground: true };
  const scene = depict(state, opts, { thermal: solveThermal(state), structure: solveStructure(state) });
  if (col >= cols) { col = 0; x = 0; y += rowH + gap; rowH = 0; }
  out += `<g transform="translate(${x} ${y})"><rect width="${scene.widthMm}" height="${scene.heightMm}" fill="${scene.background ?? "#fff"}" stroke="#bbb" stroke-width="0.2"/>${svgOf(scene)}<text x="1" y="${scene.heightMm - 1}" font-size="2.2" fill="#c00">${label} 1:${d}</text></g>`;
  x += scene.widthMm + gap; rowH = Math.max(rowH, scene.heightMm); W = Math.max(W, x); col++;
}
writeFileSync(values.out!, `<svg xmlns="http://www.w3.org/2000/svg" width="${W}mm" height="${y + rowH}mm" viewBox="0 0 ${W} ${y + rowH}"><rect width="100%" height="100%" fill="#eee"/>${out}</svg>`);
