/** 🖍️ Verifies the FEM 2D status layer contract with the real Canvas2d painter and Chromium. */
import assert from "node:assert/strict";
import { resolve } from "node:path";
import { chromium } from "playwright";

const painter = resolve(import.meta.dirname, "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🎨️paint/🟦️.ts");
const bundle = await Bun.build({ entrypoints: [painter], target: "browser" });
assert(bundle.success, bundle.logs.join("\n"));
const moduleSource = await bundle.outputs[0].text();
const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  await page.setContent('<canvas width="300" height="70"></canvas>');
  const paintedPixels = await page.evaluate(async source => {
    const url = URL.createObjectURL(new Blob([source], { type: "text/javascript" }));
    try {
      const { drawSceneNode } = await import(url);
      const canvas = document.querySelector("canvas")!;
      const context = canvas.getContext("2d")!;
      const text = { content: "Converged · 7 / 7 · Residual 8.555e-12", size: 11 };
      let rejected = false;
      try {
        drawSceneNode(context, { text }, new Map());
      } catch (error) {
        rejected = String(error).includes("Canvas text requires an explicit catalog family");
      }
      if (!rejected) throw Error("Expected Canvas2d to refuse a layer without a catalog font");
      drawSceneNode(context, {
        id: "analysis-progress",
        transform: [1, 0, 0, 1, 10, 20],
        fill: { color: [0.43, 0.48, 0.55, 1] },
        text: { ...text, fontFamily: "Share Tech Mono" },
      }, new Map());
      const { data } = context.getImageData(0, 0, canvas.width, canvas.height);
      return data.filter((value, index) => index % 4 === 3 && value > 0).length;
    } finally {
      URL.revokeObjectURL(url);
    }
  }, moduleSource);
  assert(paintedPixels > 0);
  console.log(`[DEBUG] FEM 2D status painted ${paintedPixels} pixels in Chromium`);
} finally {
  await browser.close();
}
