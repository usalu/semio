/** 📋️ Selects the orange layer, waits for the inspector, then copies in one of three ways (VARIANT=row|focus|menu|delete|canvas) and prints the refusal, the inspector and the rows. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 }, permissions: ["clipboard-read", "clipboard-write"] })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
const t0 = performance.now(), mod = process.platform === "darwin" ? "Meta" : "Control", variant = process.env.VARIANT ?? "row";
const at = () => Math.round((performance.now() - t0) / 100) / 10;
let turns = 0, outcomes = 0;
page.on("console", message => { const text = message.text(); if (text.includes("clip publish failed")) console.log(at(), "REFUSED", text.match(/message: "([^"]*)"/)?.[1]); if (text.includes("clip turn phase")) { turns += 1; if (turns % 20 === 1) console.log(at(), text.slice(0, 90)); } if (text.includes("clip lend") && text.includes("outcome true")) { outcomes += 1; console.log(at(), text.slice(0, 90)); } });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(7000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.locator('button[id="framework.panel.inspection"]').first().click();
await page.waitForTimeout(1500);
const inspector = () => page.evaluate(() => (document.querySelector('[data-slot="panel"][id="framework.panelTab.framework.panel.inspection"]')?.textContent ?? "").slice(14, 60));
const rows = () => page.locator('[role="treeitem"][id^="panel:framework.panel.artifact/drawing-play-layers"]').count();
if (variant === "canvas") await page.mouse.click(410, 200);
else await page.locator('[role="treeitem"][id="panel:framework.panel.artifact/drawing-play-layers.path.emblem-orange"]').click();
await page.waitForTimeout(4000);
console.log(at(), variant, "inspector", await inspector(), "rows", await rows());
const canvas = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first();
if (variant === "row") await page.keyboard.press(`${mod}+c`);
if (variant === "focus" || variant === "canvas") { await canvas.focus(); await page.keyboard.press(`${mod}+c`); }
if (variant === "delete") { await canvas.focus(); await page.keyboard.press("Delete"); }
if (variant === "menu") { await page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first().click(); await page.locator('[id="action.copy"]').first().click({ timeout: 8000 }); }
await page.waitForTimeout(9000);
console.log(at(), "after", await inspector(), "rows", await rows(), "turns", turns, "outcomes", outcomes);
await browser.close();
