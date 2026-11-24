/** ⏱️ Measures how long a panel-added layer takes to reach the layer tree and the canvas. */
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
await page.route(url => decodeURIComponent(url.href).includes("handle-turn-drive"), async route => {
  const response = await route.fetch();
  const body = (await response.text()).replace(/last = await shardClient\.stepJob\(actorId, job, jobStepBudget\);/, '{ const t = performance.now(); last = await shardClient.stepJob(actorId, job, jobStepBudget); (globalThis.__steps ??= []).push([performance.now() - t, last.progress ? Number(new DataView(last.progress.buffer, last.progress.byteOffset, 8).getBigUint64(0, true)) : null, t]); }');
  console.log("budget", body.match(/jobStepBudget = [^;]*/)?.[0]);
  await route.fulfill({ response, body });
});
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(8000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(3000);
const hash = async () => createHash("sha256").update(await page.screenshot({ clip: { x: 310, y: 85, width: 980, height: 820 } })).digest("hex").slice(0, 10);
const rows = () => page.locator('[role="treeitem"][id*="drawing-play-layers.shape."]').count();
for (const name of ["Add Ellipse"]) {
  const before = await hash(), count = await rows(), start = performance.now();
  await page.getByText(name, { exact: true }).first().click();
  let row = null, canvas = null;
  while (performance.now() - start < 45000 && (row === null || canvas === null)) {
    if (row === null && await rows() > count) row = Math.round(performance.now() - start);
    if (canvas === null && await hash() !== before) canvas = Math.round(performance.now() - start);
    await page.waitForTimeout(100);
  }
  console.log(name, "row ms", row, "canvas ms", canvas);
  await page.waitForTimeout(6000);
}
const steps = await page.evaluate(() => globalThis.__steps);
const big = steps.slice(-1100);
const sorted = big.map(row => row[0]).sort((a, b) => a - b);
console.log("steps", steps.length, "median ms", sorted[sorted.length >> 1].toFixed(2), "p90", sorted[Math.floor(sorted.length * 0.9)].toFixed(2), "sum s", (sorted.reduce((a, b) => a + b, 0) / 1000).toFixed(1), "span s", ((big.at(-1)[2] - big[0][2]) / 1000).toFixed(1), "gap median", (() => { const gaps = big.slice(1).map((row, i) => row[2] - big[i][2] - big[i][0]).sort((a, b) => a - b); return gaps[gaps.length >> 1].toFixed(2) + " sum " + (gaps.reduce((a, b) => a + b, 0) / 1000).toFixed(1); })());
console.log("null progress", big.filter(row => row[1] === null).length, "work first/last", big.find(row => row[1] !== null)?.[1], big.findLast(row => row[1] !== null)?.[1]);
await browser.close();
