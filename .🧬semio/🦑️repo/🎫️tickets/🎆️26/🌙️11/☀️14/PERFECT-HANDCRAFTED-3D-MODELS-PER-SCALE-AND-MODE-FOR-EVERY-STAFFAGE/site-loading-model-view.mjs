/** 🧊️ Confirms that the 3D view of an asset page renders at every preset scale and in every view of the archive.
 * Usage: `node site-loading-model-view.mjs <origin> <category/slug> <screenshot dir> [--hold-findings]` — prints one JSON line per scale and view and writes one screenshot per scale.
 * A view counts as rendered when the model stage holds a canvas, reports neither a loading nor an error state, and a screenshot of the canvas, without the 12 pixels along its edges (the frame), shows more than 0.1 % of pixels that differ clearly from its most frequent colour (the ground); the page is polled for up to two minutes per view.
 * `--hold-findings` leaves the findings stream of the development server unanswered, so an asset with findings stays offered and the viewer itself can be judged. */
import { createRequire } from "node:module";
import { writeFileSync } from "node:fs";

const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { chromium } = require("playwright");
const [origin, route, directory, hold] = process.argv.slice(2);
const slug = route.split("/")[1];
const browser = await chromium.launch({ channel: "chrome", headless: true });
const context = await browser.newContext({ locale: "en-US", viewport: { width: 1440, height: 900 } });
const page = await context.newPage();
if (hold === "--hold-findings") await page.route("**/@staffage-findings/**", () => undefined);
const errors = [];
page.on("pageerror", (error) => errors.push(String(error).slice(0, 200)));
let rendered = 0, total = 0;
for (const scale of [50, 100, 200, 500]) {
  for (const view of ["dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments"]) {
    total += 1;
    const start = Date.now();
    await page.goto(`${origin}/#/${route}?vp=model&scale=${scale}&view=${view}`);
    if (total === 1) await page.reload();
    const selector = `[data-page="asset:${slug}"] [data-model-figure][data-asset="${slug}"] canvas`;
    let outcome = { scale, view, rendered: false };
    try {
      const canvas = await page.waitForSelector(selector, { timeout: 120000, state: "visible" });
      let inked = 0, state = "loading", shot = null;
      for (let waited = 0; waited < 120000 && !(state === "shown" && inked > 0.001); waited += 500) {
        await page.waitForTimeout(500);
        state = await page.evaluate(() => document.querySelector('[data-page^="asset:"] [data-model-state]')?.getAttribute("data-model-state") ?? "shown");
        if (state === "error") break;
        shot = await canvas.screenshot();
        inked = await page.evaluate(async (base64) => {
          const image = new Image();
          image.src = `data:image/png;base64,${base64}`;
          await image.decode();
          const probe = document.createElement("canvas");
          const edge = 12;
          probe.width = image.width - 2 * edge;
          probe.height = image.height - 2 * edge;
          const pen = probe.getContext("2d");
          pen.drawImage(image, -edge, -edge);
          const { data } = pen.getImageData(0, 0, probe.width, probe.height);
          const counts = new Map();
          for (let i = 0; i < data.length; i += 4) {
            const key = (data[i] << 16) | (data[i + 1] << 8) | data[i + 2];
            counts.set(key, (counts.get(key) ?? 0) + 1);
          }
          const ground = [...counts].sort((p, q) => q[1] - p[1])[0][0];
          let different = 0;
          for (let i = 0; i < data.length; i += 4) if (Math.abs(data[i] - (ground >> 16)) + Math.abs(data[i + 1] - ((ground >> 8) & 255)) + Math.abs(data[i + 2] - (ground & 255)) > 40) different += 1;
          return different / (data.length / 4);
        }, shot.toString("base64"));
      }
      outcome = { scale, view, rendered: state === "shown" && inked > 0.001, state, inkedFraction: Number(inked.toFixed(4)), ms: Date.now() - start };
      if (shot !== null) writeFileSync(`${directory}/model-${scale}-${view}.png`, shot);
    } catch (error) {
      outcome = { scale, view, rendered: false, error: String(error).slice(0, 160) };
    }
    if (outcome.rendered) rendered += 1;
    console.log(JSON.stringify(outcome));
  }
}
console.log(JSON.stringify({ route, rendered, total, pageErrors: errors.slice(0, 5) }));
await browser.close();
