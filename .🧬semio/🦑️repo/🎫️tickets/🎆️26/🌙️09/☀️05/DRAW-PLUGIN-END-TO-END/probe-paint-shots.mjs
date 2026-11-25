/** 📸️ Adds an ellipse, a polygon and a rectangle and stores canvas pictures at fixed delays after each click. */
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const output = resolve(process.argv[2]);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 120000 });
await page.waitForTimeout(8000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(3000);
const shot = name => page.screenshot({ path: join(output, name + ".png"), clip: { x: 0, y: 0, width: 1000, height: 700 } });
await shot("0-before");
for (const name of ["Ellipse", "Polygon", "Rectangle"]) {
  const start = performance.now();
  await page.getByText("Add " + name, { exact: true }).first().click();
  for (const wait of [1500, 3000, 6000, 12000]) {
    await page.waitForTimeout(Math.max(0, wait - (performance.now() - start)));
    await shot(name + "-" + wait);
  }
}
await browser.close();
