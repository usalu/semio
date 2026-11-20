/** 🔠 Paints the native PDF outline clip through the production Canvas2d renderer. */
import { createCanvas, Path2D } from "@napi-rs/canvas";
import { drawSceneNode, type CanvasSceneNode } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🎨️paint/🟦️.ts";

const [fixture, segmentsPath] = process.argv.slice(2);
const data = await Bun.file(fixture).json();
const segments = await Bun.file(segmentsPath).json();
const [width, height] = data.page;
Object.assign(globalThis, {Path2D});
const canvas = createCanvas(width, height);
const ctx = canvas.getContext("2d");
ctx.fillStyle = "white";
ctx.fillRect(0, 0, width, height);
const rectangle = (x: number, y: number, width: number, height: number): CanvasSceneNode["segments"] => [
  {kind: "move", to: [x, y]}, {kind: "line", to: [x + width, y]},
  {kind: "line", to: [x + width, y + height]}, {kind: "line", to: [x, y + height]}, {kind: "close"},
];
const paint = (node: CanvasSceneNode) => drawSceneNode(ctx as unknown as CanvasRenderingContext2D, node, new Map());
paint({segments: rectangle(0, 0, width, height), fill: {color: [1, 0, 0, 1]}, clipPaths: [{fillRule: "nonzero", segments}]});
paint({segments: rectangle(140, height - 15, 10, 10), fill: {color: [0, 0, 1, 1]}});
const pixel = (x: number, y: number) => [...ctx.getImageData(x, y, 1, 1).data].slice(0, 3).join();
for (const [left, top, right, bottom] of data.clip.regions) {
  let red = false;
  for (let y = top; y < bottom; y++) {
    for (let x = left; x < right; x++) red ||= pixel(x, y) === "255,0,0";
  }
  if (!red) throw new Error(`Missing red ink in glyph region ${[left, top, right, bottom].join()}`);
}
for (const probe of data.clip.probes) {
  if (pixel(probe.at[0], probe.at[1]) !== probe.rgb.join()) throw Error(`Unexpected pixel at ${probe.at}`);
}
console.log("Production Canvas2d painter matched both MuPDF clip regions, their white gap, and paint after restore.");
