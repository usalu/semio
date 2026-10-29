import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { VIEWS } = await import(`${S}/🔨️modules/🔗️contracts/🟦️.ts`);
const { defaultDepictionOptions, depict } = await import(`${S}/🔨️modules/🎨️depiction/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { solveStructure } = await import(`${S}/🔨️modules/🏗️structure/🟦️.ts`);
const { solveThermal } = await import(`${S}/🔨️modules/🌡️thermal/🟦️.ts`);

const { values } = parseArgs({ options: { file: { type: "string" }, asset: { type: "string" }, out: { type: "string" }, scale: { type: "string", default: "50" }, views: { type: "string", default: "light-line,dark-solid,thermal,forces,moments" }, queries: { type: "string" } } });
const doc = JSON.parse(readFileSync(resolve(values.file!), "utf8")) as any;
const asset = doc.assets ? doc.assets.find((a: any) => a.id === values.asset)! : doc;
const views = values.views!.split(",");
const queries: any[] = values.queries ? values.queries.split(",").map((q) => (q.includes("@") ? { kind: "clip", clip: q.split("@")[0], timeS: Number(q.split("@")[1]) } : { kind: "pose", pose: q })) : asset.poses.map((p: any) => ({ kind: "pose", pose: p.id }));

const esc = (s: string) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;");
function cmd(c: any): string { return c.op === "M" ? `M${c.x} ${c.y}` : c.op === "L" ? `L${c.x} ${c.y}` : c.op === "C" ? `C${c.x1} ${c.y1} ${c.x2} ${c.y2} ${c.x} ${c.y}` : "Z"; }
function svgOf(scene: any): string {
  let o = `<svg x="0" y="0" width="${scene.widthMm}" height="${scene.heightMm}" viewBox="0 0 ${scene.widthMm} ${scene.heightMm}">`;
  if (scene.background) o += `<rect width="100%" height="100%" fill="${scene.background}"/>`;
  for (const l of scene.layers) for (const it of l.items) {
    if (it.kind === "text") { o += `<text x="${it.x}" y="${it.y}" font-size="${it.sizeMm}" fill="${it.color}" text-anchor="${it.anchor}" font-family="sans-serif">${esc(it.text)}</text>`; continue; }
    const f = it.fill ? `fill="${it.fill.color}" fill-opacity="${it.fill.opacity ?? 1}"` : `fill="none"`;
    const s = it.stroke ? `stroke="${it.stroke.color}" stroke-width="${it.stroke.widthMm}" stroke-opacity="${it.stroke.opacity ?? 1}" stroke-linecap="${it.stroke.cap ?? "round"}" stroke-linejoin="${it.stroke.join ?? "round"}"${it.stroke.dashMm ? ` stroke-dasharray="${it.stroke.dashMm.join(" ")}"` : ""}` : "";
    o += `<path d="${it.commands.map(cmd).join("")}" ${f} ${s}/>`;
  }
  return o + "</svg>";
}
const scales = values.scale!.split(",").map(Number);
let y = 0, maxW = 0;
let body = "";
for (const sc of scales) for (const q of queries) {
  const state = evaluate(asset, q);
  const thermal = views.includes('thermal') ? solveThermal(state) : undefined, structure = views.some((v) => v === 'forces' || v === 'moments') ? solveStructure(state) : undefined;
  let x = 0, rowH = 0;
  for (const v of views) {
    const scene = depict(state, { ...defaultDepictionOptions(v as any, "en", sc), annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: true } }, { thermal, structure });
    body += `<g transform="translate(${x} ${y})">${svgOf(scene)}</g>`;
    x += scene.widthMm + 2; rowH = Math.max(rowH, scene.heightMm);
  }
  body += `<text x="1" y="${y + 3}" font-size="2.5" fill="#c00" font-family="sans-serif">${q.kind === "pose" ? q.pose : q.clip + "@" + q.timeS} 1:${sc}</text>`;
  y += rowH + 2; maxW = Math.max(maxW, x);
}
writeFileSync(values.out!, `<svg xmlns="http://www.w3.org/2000/svg" width="${maxW}mm" height="${y}mm" viewBox="0 0 ${maxW} ${y}"><rect width="100%" height="100%" fill="#888"/>${body}</svg>`);
console.log("wrote", values.out, maxW.toFixed(0), y.toFixed(0));
