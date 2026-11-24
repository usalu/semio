/** 🗂️ Adds layers through the Artifact panel, selects, edits and deletes them, and records what the canvas shows after each step. */
import { mkdirSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { chromium } = require("playwright");
const output = resolve(process.argv[2]);
mkdirSync(output, { recursive: true });
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 } })).newPage();
const log = [], errors = [];
page.on("console", message => { if (message.type() === "error") (message.text().startsWith("[DEBUG] ") ? log : errors).push(message.text().slice(0, 400)); });
page.on("pageerror", error => errors.push(String(error)));
await page.goto("http://127.0.0.1:6064/?plugin=draw");
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, undefined, { timeout: 90000 });
const open = async (id, name) => { const panel = page.locator(`[id="framework.panelTab.${id}"],[id="${id}"]`).first(); if (!(await panel.isVisible().catch(() => false))) await page.getByRole("tab", { name, exact: true }).first().click({ timeout: 15000 }).catch(async () => page.getByText(name, { exact: true }).first().click({ timeout: 15000 })); };
const steps = [];
const shot = async name => { await page.waitForTimeout(4000); const rows = await page.locator('[role="treeitem"][id^="panel:framework.panel.artifact/drawing-play-layers.path."], [role="treeitem"][id*="drawing-play-layers."]').evaluateAll(nodes => nodes.map(node => node.id.split("/").pop())); steps.push({ name, rows: rows.filter(row => !row.includes(".add.")), errors: errors.length, debug: log.length, last: log.at(-1)?.slice(0, 160) }); await page.screenshot({ path: join(output, `${String(steps.length).padStart(2, "0")}-${name}.png`) }); };
try {
  await page.getByText("Artifact", { exact: true }).first().click({ timeout: 15000 });
  await shot("boot");
  for (const name of ["Add Ellipse", "Add Text", "Add Polygon", "Add Line", "Add Rectangle", "Add Boolean"]) {
    await page.getByText(name, { exact: true }).first().click({ timeout: 20000 });
    await shot(name.replace(" ", "-"));
  }
  await page.getByText("Inspection", { exact: true }).first().click({ timeout: 15000 });
  await shot("inspection");
  await page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first().focus();
  await page.keyboard.press("Delete");
  await shot("delete");
  await page.keyboard.press("Meta+z");
  await shot("undo");
} catch (error) { steps.push({ failure: String(error).slice(0, 400) }); await page.screenshot({ path: join(output, "failure.png") }); }
writeFileSync(join(output, "report.json"), JSON.stringify({ steps, errors, log }, null, 1));
for (const step of steps) console.log(JSON.stringify(step));
console.log("errors", errors.length, errors.slice(0, 4));
await browser.close();
