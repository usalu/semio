/** 📋️ Selects the orange row, copies, pastes and cuts with times, and prints every guest refusal with the input that preceded it. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
const t0 = performance.now(), at = () => Math.round((performance.now() - t0) / 100) / 10, mod = process.platform === "darwin" ? "Meta" : "Control";
page.on("console", message => { const text = message.text(); if (/original-turn|refus|fault|panic|trap|unreachable|stall/i.test(text) && !text.includes("receiving quote")) console.log(at(), message.type(), text.slice(0, 420)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
console.log(at(), "ready");
await page.waitForTimeout(8000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(1500);
await page.locator('[role="treeitem"][id="panel:framework.panel.artifact/emblem-orange"]').click();
await page.waitForTimeout(4000);
const rows = () => page.locator('[role="treeitem"][id^="panel:framework.panel.artifact/"]').count();
const canvas = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first();
console.log(at(), "rows", await rows());
for (const key of ["c", "v", "x", "z", "z"]) {
  console.log(at(), "focus"); await canvas.focus(); await page.waitForTimeout(500);
  console.log(at(), "press", key); await page.keyboard.press(`${mod}+${key}`);
  await page.waitForTimeout(8000);
  console.log(at(), "after", key, "rows", await rows());
}
await page.screenshot({ path: resolve(process.argv[2]) });
await browser.close();
