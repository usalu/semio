import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const outDir = dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const moduleUrls: string[] = [];
page.on("response", (r) => {
  const u = decodeURIComponent(r.url());
  if (/flow_core|playground-wasm-stub|flow-core/i.test(u)) moduleUrls.push(`${r.status()} ${u.slice(-140)}`);
});
const errors: string[] = [];
page.on("console", (m) => {
  if (m.type() === "error") errors.push(m.text().slice(0, 500));
});
page.on("pageerror", (e) => errors.push(`pageerror: ${e.message.slice(0, 500)}`));

async function probeExample(label: string) {
  await page.goto("http://127.0.0.1:6018/", { waitUntil: "networkidle", timeout: 120000 });
  await page.waitForTimeout(10000);
  if (label === "rectangle-extrude") {
    const exampleBtn = page.locator("button").filter({ hasText: /Rectangle Extrude/i }).first();
    if (await exampleBtn.count()) await exampleBtn.click();
    await page.waitForTimeout(6000);
  }
  const report = await page.evaluate(() => {
    const canvases = [...document.querySelectorAll("canvas")].map((c) => ({
      w: c.width,
      h: c.height,
      cw: (c as HTMLCanvasElement).clientWidth,
      ch: (c as HTMLCanvasElement).clientHeight,
    }));
    const flowCanvas = canvases[0];
    const sized =
      flowCanvas &&
      flowCanvas.w > 400 &&
      flowCanvas.h > 400 &&
      flowCanvas.w >= flowCanvas.cw * 0.9 &&
      flowCanvas.h >= flowCanvas.ch * 0.9;
    return {
      title: document.title,
      canvasCount: canvases.length,
      canvases,
      flowCanvasSized: sized,
      bodyHasFlow: /Flow/i.test(document.body.innerText),
    };
  });
  await page.screenshot({ path: `${outDir}/probe-${label}.png`, fullPage: false });
  return report;
}

const hexagonal = await probeExample("hexagonal-mushroom");
const rectangle = await probeExample("rectangle-extrude");

const full = {
  hexagonal,
  rectangle,
  moduleUrls: moduleUrls.slice(0, 30),
  usesFlowStub: moduleUrls.some((u) => u.includes("playground-wasm-stub")),
  usesFlowCore: moduleUrls.some((u) => u.includes("flow_core")),
  errors: errors.filter((e) => !e.includes("favicon")).slice(0, 20),
  ok:
    !moduleUrls.some((u) => u.includes("playground-wasm-stub")) &&
    moduleUrls.some((u) => u.includes("flow_core")) &&
    hexagonal.flowCanvasSized &&
    rectangle.flowCanvasSized &&
    errors.filter((e) => e.startsWith("pageerror:") || /handle_typed_command|action failed/i.test(e)).length === 0,
};
writeFileSync(`${outDir}/probe-flow-graph.json`, JSON.stringify(full, null, 2));
console.log(JSON.stringify(full, null, 2));
await browser.close();
if (!full.ok) process.exit(1);
