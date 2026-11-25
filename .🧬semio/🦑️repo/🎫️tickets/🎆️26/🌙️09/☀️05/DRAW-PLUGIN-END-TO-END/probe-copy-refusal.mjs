/** 🚫️ Copies with nothing selected and prints every notice, command-stall band and console line until the second copy has been answered too. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 }, permissions: ["clipboard-read", "clipboard-write"] })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
const t0 = performance.now(), at = () => Math.round((performance.now() - t0) / 100) / 10, mod = process.platform === "darwin" ? "Meta" : "Control";
let printed = 0;
page.on("console", message => { const text = message.text(); if (/fault|refus|clip lend|stall|app-owned/i.test(text) && !text.includes("outcome false") && printed++ < 60) console.log(at(), message.type(), text.slice(0, 260)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(7000);
const texts = async () => page.evaluate(() => [...document.querySelectorAll('[role="status"],[role="alert"],[data-slot*="toast"],[data-slot*="notice"],[data-slot*="band"]')].map(node => (node.textContent ?? "").trim()).filter(text => text.length > 0 && text.length < 240));
let before = new Set(await texts());
for (const round of [1, 2]) {
  await page.mouse.click(1500, 900);
  await page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first().focus();
  await page.keyboard.press(`${mod}+c`);
  console.log(at(), "copy", round);
  for (let index = 0; index < 14; index++) { await page.waitForTimeout(1000); for (const text of await texts()) if (!before.has(text)) { before.add(text); console.log(at(), "TEXT", text); } }
}
await browser.close();
