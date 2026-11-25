/** 🖱️ Clicks an existing layer by its exact row and by its shape on the canvas and prints what the editor, the inspector and the rows show. */
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
await page.routeWebSocket(address => address.searchParams.has("token"), () => undefined);
const t0 = performance.now();
const at = () => Math.round((performance.now() - t0) / 100) / 10;
page.on("console", message => { if (message.type() === "error" && !message.text().startsWith("[DEBUG]")) console.log(at(), message.text().slice(0, 300)); });
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
await page.waitForTimeout(7000);
await page.getByText("Artifact", { exact: true }).first().click();
await page.waitForTimeout(1500);
await page.locator('button[id="framework.panel.inspection"]').first().click();
await page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first().click();
await page.waitForTimeout(800);
const show = label => page.evaluate(label => ({ label, status: document.querySelector('[id="drawing-composite"] [data-slot="engagement-status"]')?.textContent, selectedRows: [...document.querySelectorAll('[role="treeitem"][aria-selected="true"]')].map(node => node.id.split("/").pop()), inspector: (document.querySelector('[data-slot="panel"][id="framework.panelTab.framework.panel.inspection"]')?.textContent ?? "").slice(0, 90) }), label).then(value => console.log(at(), JSON.stringify(value)));
await show("boot");
await page.locator('[role="treeitem"][id="panel:framework.panel.artifact/emblem-orange"]').click();
for (const wait of [700, 2500]) { await page.waitForTimeout(wait); await show("row click"); }
await page.mouse.click(410, 200);
for (const wait of [700, 2500]) { await page.waitForTimeout(wait); await show("canvas click on orange"); }
await page.mouse.click(475, 125);
for (const wait of [700, 2500]) { await page.waitForTimeout(wait); await show("canvas click on red"); }
await page.screenshot({ path: resolve(process.argv[2]) });
await browser.close();
