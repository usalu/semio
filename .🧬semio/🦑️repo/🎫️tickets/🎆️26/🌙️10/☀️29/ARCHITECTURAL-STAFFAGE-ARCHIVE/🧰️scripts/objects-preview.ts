/** 🖨️ Scratch preview: renders every pose of one asset (or all) to SVG and PNG via the depiction module; output under the ticket's generated folder. */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { defaultDepictionOptions, depict } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
import { solveThermal } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";
import type { DrawingScene, DrawingItem } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";

const [file, only, denomArg, viewArg] = process.argv.slice(2);
const denom = Number(denomArg ?? 50);
const view = (viewArg ?? "light-line") as "light-line";
const out = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/objects";
mkdirSync(out, { recursive: true });

const f = (n: number): string => (Math.round(n * 1000) / 1000).toString();
function pathD(item: Extract<DrawingItem, { kind: "path" }>): string {
  return item.commands.map((c) => (c.op === "Z" ? "Z" : c.op === "C" ? `C ${f(c.x1)} ${f(c.y1)} ${f(c.x2)} ${f(c.y2)} ${f(c.x)} ${f(c.y)}` : `${c.op} ${f(c.x)} ${f(c.y)}`)).join(" ");
}
function svg(scene: DrawingScene): string {
  const body = scene.layers.map((layer) => `<g id="${layer.id}">` + layer.items.map((item) => {
    if (item.kind === "text") return `<text x="${f(item.x)}" y="${f(item.y)}" font-size="${f(item.sizeMm)}" fill="${item.color}" text-anchor="${item.anchor}" font-family="sans-serif">${item.text.replace(/&/g, "&amp;").replace(/</g, "&lt;")}</text>`;
    const fill = item.fill ? `fill="${item.fill.color}" fill-opacity="${item.fill.opacity ?? 1}"` : `fill="none"`;
    const stroke = item.stroke ? `stroke="${item.stroke.color}" stroke-width="${f(item.stroke.widthMm)}" stroke-linejoin="round" stroke-linecap="round"` : `stroke="none"`;
    return `<path d="${pathD(item)}" ${fill} ${stroke}/>`;
  }).join("") + "</g>").join("\n");
  const bg = scene.background ? `<rect width="100%" height="100%" fill="${scene.background}"/>` : "";
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${f(scene.widthMm)}mm" height="${f(scene.heightMm)}mm" viewBox="0 0 ${f(scene.widthMm)} ${f(scene.heightMm)}">${bg}${body}</svg>`;
}

const doc = JSON.parse(readFileSync(file!, "utf8"));
for (const asset of doc.assets.filter((a: { id: string }) => !only || a.id === only)) {
  for (const pose of asset.poses) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id });
    const thermal = solveThermal(state);
    const structure = solveStructure(state);
    const scene = depict(state, defaultDepictionOptions(view, "en", denom), { thermal, structure });
    const base = `${out}/${asset.id}--${pose.id}--${view}-1-${denom}`;
    writeFileSync(`${base}.svg`, svg(scene));
    execFileSync("rsvg-convert", ["-w", "900", "-b", "white", "-f", "png", "-o", `${base}.png`, `${base}.svg`]);
    console.log(base + ".png", "reactions", structure.reactions.map((r) => `${r.anchor}:${Math.round(r.fyN)}N`).join(" "));
  }
}
