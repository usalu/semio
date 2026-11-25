/** 📋️ Selects a layer by its row and by its shape, shows the selection count the editor reports, copies and pastes, and prints every notice. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 }, permissions: ["clipboard-read", "clipboard-write"] })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
const t0 = performance.now();
const at = () => Math.round((performance.now() - t0) / 100) / 10;
page.on("console", message => { const text = message.text(); if (text.includes("publish failed")) console.log(at(), text.slice(0, 260)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(6000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(1500);
const status = async () => { const open = page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first(); const value = page.locator('[id="drawing-composite"] [data-slot="engagement-status"]'); if (!(await value.isVisible())) await open.click(); const text = await value.textContent({ timeout: 5000 }).catch(() => "?"); await open.click(); return text; };
const notices = () => page.evaluate(() => [...document.querySelectorAll('[role="status"],[role="alert"],[data-sonner-toast],[data-slot*="notice"],[data-semio-notice]')].map(node => node.textContent.trim()).filter(Boolean).slice(0, 8));
const rows = () => page.locator('[role="treeitem"][id^="panel:framework.panel.artifact/drawing-play-layers"]').count();
console.log(at(), "start", await status(), "rows", await rows());
const row = page.locator('[role="treeitem"][id*="emblem-orange"]').first();
await row.click();
for (const wait of [1000, 3000]) { await page.waitForTimeout(wait); console.log(at(), "after row click", await status()); }
await page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first().focus();
await page.keyboard.press(`${process.platform === "darwin" ? "Meta" : "Control"}+c`);
for (const wait of [1500, 4000, 6000]) { await page.waitForTimeout(wait); console.log(at(), "after copy", JSON.stringify(await notices())); }
console.log(at(), "rows", await rows(), await status());
await page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first().focus();
await page.keyboard.press(`${process.platform === "darwin" ? "Meta" : "Control"}+v`);
for (const wait of [2000, 8000]) { await page.waitForTimeout(wait); console.log(at(), "after paste rows", await rows(), JSON.stringify(await notices())); }
const canvas = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"] canvas').first();
const box = await canvas.boundingBox();
await page.mouse.click(box.x + 410 - box.x + 0, box.y + 200 - box.y + 0);
await page.waitForTimeout(2500);
console.log(at(), "after canvas click", await status());
await page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first().focus();
await page.keyboard.press(`${process.platform === "darwin" ? "Meta" : "Control"}+c`);
for (const wait of [1500, 6000]) { await page.waitForTimeout(wait); console.log(at(), "after copy 2", JSON.stringify(await notices())); }
await page.screenshot({ path: resolve(process.argv[2]) });
await browser.close();
