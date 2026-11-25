/** 🛠️ Presses the rectangle utility, draws with it and prints every refusal with its time. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
import fixture from "../../../../../../../✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔣️.json";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
const t0 = performance.now(), at = () => Math.round((performance.now() - t0) / 100) / 10;
page.on("console", message => { const text = message.text(); if (/original-turn|refus|fault|panic|trap|unreachable|stall|work loan/i.test(text) && !text.includes("receiving quote")) console.log(at(), message.type(), text.slice(0, 500)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(8000);
console.log(at(), "ready");
await page.locator('[id^="framework.window."][id$=".utilityBar.unfold"]').first().click({ timeout: 15000 });
await page.locator('[id^="ui.utilities."][id$=".group.group:Drawing"]').first().click({ timeout: 15000 });
for (const pressed of fixture.utilityPressStates) {
  await page.locator(`[id="${fixture.utility}"]`).first().click();
  await page.waitForTimeout(4000);
  console.log(at(), "pressed", pressed, await page.evaluate(id => document.getElementById(id)?.getAttribute("aria-pressed"), fixture.utility));
}
const rect = await page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first().boundingBox();
const [x0, y0] = fixture.gesture.from, [x1, y1] = fixture.gesture.to;
await page.mouse.move(rect.x + rect.width * x0, rect.y + rect.height * y0);
await page.mouse.down();
await page.mouse.move(rect.x + rect.width * x1, rect.y + rect.height * y1, { steps: fixture.gesture.steps });
await page.mouse.up();
console.log(at(), "gesture");
await page.waitForTimeout(8000);
await page.screenshot({ path: resolve(process.argv[2]) });
await browser.close();
