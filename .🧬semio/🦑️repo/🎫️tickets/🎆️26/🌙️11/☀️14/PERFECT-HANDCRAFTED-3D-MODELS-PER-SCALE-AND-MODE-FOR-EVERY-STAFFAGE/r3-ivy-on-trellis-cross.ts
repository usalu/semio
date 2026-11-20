/** 🔭️ The product 3D silhouettes of two scales of the ivy on trellis in one window (finer blue, coarser red), slender strokes included as the cross-scale rule draws them.
 * Usage: bun r3-ivy-on-trellis-cross.ts <finer> <coarser> <view> <minX> <minY> <size> */
import sharp from "sharp";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { drawnFor, referenceOf, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { load, r3Out } from "./r3-lib.ts";

const [fine, coarse, view, x0, y0, size] = [Number(process.argv[2]), Number(process.argv[3]), process.argv[4] as "elevation", Number(process.argv[5]), Number(process.argv[6]), Number(process.argv[7])];
const { entry, asset } = load("vegetation/ivy-on-trellis", "r3-ivy-on-trellis-cross.ts <finer> <coarser> <view> <minX> <minY> <size>");
const px = (p: { x: number; y: number }) => `${(((p.x - x0) / size) * 1000).toFixed(2)},${((1 - (p.y - y0) / size) * 1000).toFixed(2)}`;
const path = (rings: readonly (readonly { x: number; y: number }[])[]) => rings.map((r) => `M${r.map(px).join("L")}Z`).join("");
const side = (scale: number, colour: string): string => {
  const state = evaluate(asset as never, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: scale } } as never) as never as { bones: never };
  const solids = worldSolids(asset as never, state.bones, scale).filter((w: { texture?: boolean }) => w.texture !== true);
  const ref = referenceOf(view, solids as never, undefined, drawnFor(asset as never, state as never, view, scale));
  return ref.cut.map((g) => `<path d="${path(g as never)}" fill="${colour}" fill-rule="nonzero"/>`).join("");
};
const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000"><rect width="1000" height="1000" fill="white"/><g opacity="0.5">${side(fine, "blue")}</g><g opacity="0.5">${side(coarse, "red")}</g></svg>`;
const out = r3Out("r3-ivy-on-trellis", entry as never, `cross.s${fine}-s${coarse}.${view}.png`);
await sharp(Buffer.from(svg)).png().toFile(out);
console.log(out);
