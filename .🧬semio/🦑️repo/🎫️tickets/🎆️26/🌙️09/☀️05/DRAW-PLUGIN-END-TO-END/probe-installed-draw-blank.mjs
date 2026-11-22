/** 🖱️ Exercises the real installed Draw canvas with a blank document while Demo is being rebuilt. */
import assert from "node:assert/strict";
import { chromium } from "playwright";
const target = Bun.argv[2];
if (!target) throw Error("Expected the installed Draw URL");
const fixture = await Bun.file("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔣️.json").json();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
try {
  const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
  page.on("console", message => {
    if (message.type() === "error" || message.type() === "warning") console.log("[DEBUG] Draw browser", message.type(), message.text().slice(0, 1600));
  });
  page.on("pageerror", error => console.log("[DEBUG] Draw page error", error.message));
  await page.goto(target, { waitUntil: "domcontentloaded" });
  const canvas = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"] canvas').first();
  await canvas.waitFor({ state: "visible", timeout: 45000 });
  const status = async () => page.evaluate(() => ({ text: document.body.innerText.slice(-3500), ready: document.documentElement.dataset.semioOsReady }));
  const waitLayers = value => page.waitForFunction(count => {
    const status = document.body.innerText.match(/(\d+) (?:layers?|Ebenen?) · \d+ (?:selected|ausgewählt)/);
    return status && Number(status[1]) === count;
  }, value, { timeout: 30000 });
  await waitLayers(0).catch(async error => { console.log("[DEBUG] Draw blank state", JSON.stringify(await status())); throw error; });
  console.log("[DEBUG] Draw blank canvas ready");
  await page.locator('[id^="framework.window."][id$=".utilityBar.unfold"]').first().click({ timeout: 15000 });
  await page.locator('[id^="ui.utilities."][id$=".group.group:Drawing"]').first().click({ timeout: 15000 });
  await page.locator('[id="' + fixture.utility + '"]').first().click({ timeout: 15000 });
  const rect = await canvas.boundingBox();
  assert(rect && rect.width > 0 && rect.height > 0);
  const [x0,y0] = fixture.gesture.from, [x1,y1] = fixture.gesture.to;
  await page.mouse.move(rect.x + rect.width*x0, rect.y + rect.height*y0);
  await page.mouse.down();
  await page.mouse.move(rect.x + rect.width*x1, rect.y + rect.height*y1, { steps: fixture.gesture.steps });
  await page.mouse.up();
  await waitLayers(fixture.layerDelta).catch(async error => { console.log("[DEBUG] Draw rectangle state", JSON.stringify(await status())); throw error; });
  console.log(JSON.stringify({ rectangle: true, layers: fixture.layerDelta }));
} finally { await browser.close(); }
