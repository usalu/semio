import { readFileSync, writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
import { VIEWS, type DrawingScene, type View } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
import { solveThermal } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const f = (n: number) => n.toFixed(3);
function body(scene: DrawingScene): string {
  const out: string[] = [];
  if (scene.background) out.push(`<rect width="${f(scene.widthMm)}" height="${f(scene.heightMm)}" fill="${scene.background}"/>`);
  for (const layer of scene.layers) for (const it of layer.items) {
    if (it.kind === "text") { out.push(`<text x="${f(it.x)}" y="${f(it.y)}" font-size="${it.sizeMm}" fill="${it.color}" text-anchor="${it.anchor}" font-family="sans-serif">${it.text.replace(/&/g, "&amp;").replace(/</g, "&lt;")}</text>`); continue; }
    const d = it.commands.map((c) => c.op === "Z" ? "Z" : c.op === "C" ? `C${f(c.x1)} ${f(c.y1)} ${f(c.x2)} ${f(c.y2)} ${f(c.x)} ${f(c.y)}` : `${c.op}${f(c.x)} ${f(c.y)}`).join("");
    const fill = it.fill ? `fill="${it.fill.color}" fill-opacity="${it.fill.opacity ?? 1}"` : `fill="none"`;
    const st = it.stroke ? `stroke="${it.stroke.color}" stroke-width="${it.stroke.widthMm}" stroke-linecap="${it.stroke.cap ?? "round"}" stroke-linejoin="round"${it.stroke.dashMm ? ` stroke-dasharray="${it.stroke.dashMm.join(" ")}"` : ""}${it.stroke.opacity !== undefined ? ` stroke-opacity="${it.stroke.opacity}"` : ""}` : "";
    out.push(`<path d="${d}" ${fill} ${st}/>`);
  }
  return out.join("\n");
}
const { values } = parseArgs({ options: { file: { type: "string" }, asset: { type: "string" }, pose: { type: "string" }, clip: { type: "string" }, t: { type: "string", default: "0" }, scales: { type: "string", default: "200" }, views: { type: "string", default: "light-line" }, out: { type: "string" }, gap: { type: "string", default: "6" } } });
const doc = JSON.parse(readFileSync(values.file!, "utf8"));
const asset = (doc.assets ?? [doc])[0];
const state = values.clip ? evaluate(asset, { kind: "clip", clip: values.clip, timeS: Number(values.t) }) : evaluate(asset, { kind: "pose", pose: values.pose ?? asset.presentation.defaultPose });
const thermal = solveThermal(state); const structure = solveStructure(state);
const scenes: DrawingScene[] = [];
const scales = values.scales!.split(",").map(Number); const views = values.views!.split(",") as View[];
for (const s of scales) for (const v of (views[0] === "all" ? [...VIEWS] : views)) scenes.push(depict(state, { ...defaultDepictionOptions(v, "en", s) }, { thermal, structure }));
const gap = Number(values.gap);
let x = 0; let y = 0; let rowH = 0; let W = 0; const parts: string[] = [];
const perRow = scales.length > 1 ? views.length : 4;
scenes.forEach((sc, i) => {
  if (i > 0 && i % perRow === 0) { y += rowH + gap; x = 0; rowH = 0; }
  parts.push(`<g transform="translate(${f(x)} ${f(y)})">${body(sc)}</g>`);
  x += sc.widthMm + gap; rowH = Math.max(rowH, sc.heightMm); W = Math.max(W, x);
});
const H = y + rowH;
writeFileSync(values.out!, `<svg xmlns="http://www.w3.org/2000/svg" width="${f(W)}mm" height="${f(H)}mm" viewBox="0 0 ${f(W)} ${f(H)}"><rect width="${f(W)}" height="${f(H)}" fill="#888"/>${parts.join("\n")}</svg>`);
console.log("[DEBUG] sheet", values.out, W.toFixed(0), H.toFixed(0), "mm; bounds", JSON.stringify(state.bounds));
