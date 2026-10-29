import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { parseArgs } from "node:util";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const { evaluate } = await import(`${S}/🧮️evaluation/🟦️.ts`);
const { depict, defaultDepictionOptions } = await import(`${S}/🎨️depiction/🟦️.ts`);
const { solveThermal } = await import(`${S}/🌡️thermal/🟦️.ts`);
const { solveStructure } = await import(`${S}/🏗️structure/🟦️.ts`);
const { values } = parseArgs({ options: { file: { type: "string" }, asset: { type: "string" }, q: { type: "string" }, view: { type: "string", default: "light-solid" }, scale: { type: "string", default: "50" }, out: { type: "string" }, px: { type: "string", default: "1400" } } });
const doc = JSON.parse(readFileSync(values.file!, "utf8"));
const asset = (doc.assets ?? [doc]).find((a: any) => a.id === values.asset);
function svgOf(scene: any): string {
  const d = (cmds: any[]) => cmds.map((c) => (c.op === "Z" ? "Z" : c.op === "C" ? `C${c.x1} ${c.y1} ${c.x2} ${c.y2} ${c.x} ${c.y}` : `${c.op}${c.x} ${c.y}`)).join("");
  let s = `<svg xmlns="http://www.w3.org/2000/svg" width="${scene.widthMm}mm" height="${scene.heightMm}mm" viewBox="0 0 ${scene.widthMm} ${scene.heightMm}">`;
  if (scene.background) s += `<rect width="100%" height="100%" fill="${scene.background}"/>`;
  for (const l of scene.layers) { s += `<g id="${l.id}">`; for (const i of l.items) {
    if (i.kind === "text") s += `<text x="${i.x}" y="${i.y}" font-size="${i.sizeMm}" fill="${i.color}" text-anchor="${i.anchor}" font-family="sans-serif">${String(i.text).replace(/&/g, "&amp;").replace(/</g, "&lt;")}</text>`;
    else s += `<path d="${d(i.commands)}" fill="${i.fill ? i.fill.color : "none"}" fill-opacity="${i.fill?.opacity ?? 1}" ${i.stroke ? `stroke="${i.stroke.color}" stroke-width="${i.stroke.widthMm}" stroke-opacity="${i.stroke.opacity ?? 1}" stroke-linejoin="round" stroke-linecap="round"${i.stroke.dashMm ? ` stroke-dasharray="${i.stroke.dashMm.join(" ")}"` : ""}` : ""}/>`; } s += "</g>"; }
  return s + "</svg>";
}
const queries = (values.q ?? "").split(",").filter(Boolean);
const denom = Number(values.scale);
for (const q of queries) {
  const [kind, id, t] = q.split(":");
  const query = kind === "pose" ? { kind: "pose", pose: id } : { kind: "clip", clip: id, timeS: Number(t) };
  const state = evaluate(asset, query);
  const thermal = solveThermal(state); const structure = solveStructure(state);
  const opts = defaultDepictionOptions(values.view as any, "en", denom);
  const scene = depict(state, opts, { thermal, structure });
  const base = `${values.out}-${q.replace(/:/g, "_")}-${values.view}-1to${denom}`;
  writeFileSync(`${base}.svg`, svgOf(scene));
  spawnSync("rsvg-convert", ["-w", values.px!, "-b", "white", "-o", `${base}.png`, `${base}.svg`]);
  console.log(base + ".png", scene.widthMm.toFixed(1), "x", scene.heightMm.toFixed(1), "mm");
}
