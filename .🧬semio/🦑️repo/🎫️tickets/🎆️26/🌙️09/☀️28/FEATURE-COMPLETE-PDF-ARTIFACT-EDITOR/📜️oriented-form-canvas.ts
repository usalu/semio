/** 🪞️ Draws the PDF clip fixture through the browser host painter and a Skia Canvas implementation. */

import {createCanvas, Path2D} from "@napi-rs/canvas";
import {writeFileSync} from "node:fs";
import {drawSceneNode, type CanvasSceneNode} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🎨️paint/🟦️.ts";
import fixture from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🧪️oriented-form-clip.json";

Object.assign(globalThis, {Path2D});

const [width, height] = fixture.page;
const canvas = createCanvas(width!, height!);
const ctx = canvas.getContext("2d");
ctx.fillStyle = "white";
ctx.fillRect(0, 0, width!, height!);
const polygon = fixture.polygon as [[number, number], [number, number], [number, number], [number, number]];
const layer: CanvasSceneNode = {
  clipPolygons: [polygon],
  segments: [
    {kind: "move", to: [0, 0]},
    {kind: "line", to: [width!, 0]},
    {kind: "line", to: [width!, height!]},
    {kind: "line", to: [0, height!]},
    {kind: "close"},
  ],
  fill: {color: [1, 0, 0, 1]},
};
drawSceneNode(ctx as unknown as CanvasRenderingContext2D, layer, new Map());
for (const probe of fixture.probes) {
  const got = Array.from(ctx.getImageData(probe.at[0]!, probe.at[1]!, 1, 1).data.slice(0, 3));
  if (got.some((component, i) => component !== probe.rgb[i])) {
    throw new Error(`probe ${probe.at}: expected ${probe.rgb} but read ${got}`);
  }
}
writeFileSync(process.argv[2]!, canvas.toBuffer("image/png"));
console.log(`Browser host painter matched ${fixture.probes.length} Canvas pixel probes`);
