import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const outDir = "/Users/ueli/Documents/semio/.repo/🎫/26/07/23/GET-PROCEDURAL-3D-WORKING-END-TO-END";

const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleMsgs: { type: string; text: string }[] = [];
page.on("console", (msg) => {
  const text = msg.text();
  if (msg.type() === "error" || text.includes("Maximum update") || text.includes("[DEBUG]")) {
    consoleMsgs.push({ type: msg.type(), text: text.slice(0, 500) });
  }
});
page.on("pageerror", (e) => consoleMsgs.push({ type: "pageerror", text: e.message }));

await page.goto("http://localhost:6018/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(10000);

const bodyText = await page.locator("body").innerText();
const hasRenderError = /Render error|Renderfehler|Maximum update depth/i.test(bodyText);
const canvasBoxes = await page.locator("canvas").evaluateAll((nodes) =>
  nodes.map((n) => {
    const el = n as HTMLCanvasElement;
    const r = el.getBoundingClientRect();
    return { w: Math.round(r.width), h: Math.round(r.height), cw: el.width, ch: el.height };
  }),
);
const sliderCount = await page.locator('[data-slot="slider"]').count();
const windowTitles = await page.locator('[data-slot="window"]').evaluateAll((nodes) =>
  nodes.map((n) => (n.textContent ?? "").slice(0, 80).replace(/\s+/g, " ")),
);

// Try dragging the first visible slider thumb a bit
let sliderInteracted = false;
const thumb = page.locator('[data-slot="slider-thumb"]').first();
if (await thumb.count()) {
  const box = await thumb.boundingBox();
  if (box) {
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width / 2 + 40, box.y + box.height / 2, { steps: 5 });
    await page.mouse.up();
    sliderInteracted = true;
    await page.waitForTimeout(1500);
  }
}

await page.screenshot({ path: `${outDir}/e2e-after-fix.png`, fullPage: false });
const afterBody = await page.locator("body").innerText();
const stillBroken = /Render error|Renderfehler|Maximum update depth/i.test(afterBody);
const depthErrors = consoleMsgs.filter((m) => m.text.includes("Maximum update"));

const report = {
  hasRenderError,
  stillBroken,
  canvasBoxes,
  sliderCount,
  sliderInteracted,
  windowTitles,
  depthErrorCount: depthErrors.length,
  consoleSample: consoleMsgs.slice(0, 20),
  bodyStart: bodyText.slice(0, 800),
};
writeFileSync(`${outDir}/e2e-report.json`, JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
