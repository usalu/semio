/** 🔬️ The two sides of the product silhouette of the bamboo clump in one window, as the product builds them (3D: `worldSolids` + `referenceOf`, slender strokes included; 2D: filled shapes as areas, other shapes as pen-wide strokes), drawn as an SVG overlay (red 2D, blue 3D, purple both). Usage: `bun r3-bamboo-clump-masks.ts <scale> <view> <minX> <minY> <size>` */
import { writeFileSync } from "node:fs";
import sharp from "sharp";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { drawnFor, referenceOf, worldSolids, strokeQuad } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { load, r3Out } from "./r3-lib.ts";

const [scale, view, x0, y0, size] = [Number(process.argv[2]), process.argv[3] as "elevation", Number(process.argv[4]), Number(process.argv[5]), Number(process.argv[6])];
const { entry, asset } = load("vegetation/bamboo-clump", "r3-bamboo-clump-masks.ts <scale> <view> <minX> <minY> <size>");
const state = evaluate(asset as never, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: scale } } as never) as never as { bones: never; shapes: Record<string, { shape: { role: string; penMm?: number; filled?: boolean }; rings: { x: number; y: number }[][]; closed: boolean }[]> };
const solids = worldSolids(asset as never, state.bones, scale).filter((w: { texture?: boolean }) => w.texture !== true);
const ref = referenceOf(view, solids as never, undefined, drawnFor(asset as never, state as never, view, scale));
const pens = asset.representations[String(scale)].pens;
const px = (p: { x: number; y: number }) => `${(((p.x - x0) / size) * 1000).toFixed(2)},${((1 - (p.y - y0) / size) * 1000).toFixed(2)}`;
const path = (rings: readonly (readonly { x: number; y: number }[])[]) => rings.map((r) => `M${r.map(px).join("L")}Z`).join("");
let two = "";
for (const s of state.shapes[view]!.filter((s) => ["body", "detail", "glazing"].includes(s.shape.role))) {
  if (s.closed && s.shape.filled !== false) two += `<path d="${path(s.rings)}" fill="red" fill-rule="nonzero"/>`;
  else for (const r of s.rings) for (let i = 0; i + 1 < r.length; i++) two += `<path d="${path([strokeQuad(r[i]!, r[i + 1]!, ((s.shape.penMm ?? pens[s.shape.role] ?? 0) * scale) / 1000) as never])}" fill="red"/>`;
}
const three = ref.cut.map((g) => `<path d="${path(g as never)}" fill="blue" fill-rule="nonzero"/>`).join("");
const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000"><rect width="1000" height="1000" fill="white"/><g opacity="0.5">${two}</g><g opacity="0.5">${three}</g></svg>`;
const out = r3Out("r3-bamboo-clump", entry as never, `masks.s${scale}.${view}.png`);
await sharp(Buffer.from(svg)).png().toFile(out);
console.log(out);
