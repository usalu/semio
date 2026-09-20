/** 🔎️ Slice B3d — how many `action.undo` rows a multi-window app mounts, and which window owns each.
 *
 * 🏗️fem mutates but never undoes: `action.undo` clicks `ok`, is NOT covered by a panel, and the edit
 * count sits still for 60 s. fem is the first B3d app with TWO windows whose rails are both open
 * (`fem2d-model` + `fem2d-results`), and every rail mounts the app-level `undo`/`redo` rows under the
 * SAME element id — so `locator(...).first()` may be pressing the rail of a window that does not own
 * the document. This counts the duplicates and names their window before any fix is written.
 *
 * Usage: bun 🐍️b3d-undo-row-census.mjs <variant> <port>
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync, mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const OUT = join(dirname(fileURLToPath(import.meta.url)), "🗑️generated");
const [variant, port, pattern = "add"] = process.argv.slice(2);
mkdirSync(OUT, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const log = [];
page.on("console", (m) => log.push(`${m.type()} ${m.text().slice(0, 600)}`));
await page.goto(`http://127.0.0.1:${port}/?plugin=${variant}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 180; i++) {
  await page.waitForTimeout(1000);
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")) && i > 8) break;
}
for (const toggle of await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id))) {
  await page.locator(`[id="${toggle}"]`).first().click({ force: true, timeout: 8000 }).catch(() => {});
  await page.waitForTimeout(2500);
}
// 🧺️ Also dump every element id containing a caller-supplied fragment, with the panel or window it
// sits in: an app whose mutations are NOT on the rail (🖨️raster's artifact tree, 📏️layout's
// `addPage`) has to be driven through that element, and its id is not derivable from the source.
const census = await page.evaluate((pattern) => {
  const owner = (el) => el.closest('[data-slot="window"]')?.id ?? el.closest('[id$=".engagement"]')?.id ?? "(no window)";
  const counts = {};
  for (const el of document.querySelectorAll('[id^="action."]')) {
    if (el.id.startsWith("action.category.") || el.id.includes(".arg.")) continue;
    (counts[el.id] ??= []).push({ owner: owner(el), rect: el.getBoundingClientRect().toJSON() });
  }
  return {
    windows: [...document.querySelectorAll('[data-slot="window"]')].map((el) => `${el.id}:active=${el.getAttribute("data-active")}`),
    duplicated: Object.entries(counts).filter(([, rows]) => rows.length > 1).map(([id, rows]) => `${id} ×${rows.length} @ ${rows.map((r) => r.owner).join(" | ")}`),
    undo: counts["action.undo"] ?? [],
    redo: counts["action.redo"] ?? [],
    total: Object.keys(counts).length,
    matches: [...document.querySelectorAll("[id]")].filter((el) => el.id.toLowerCase().includes(pattern.toLowerCase())).slice(0, 60).map((el) => `${el.id} @ ${el.closest('[data-slot="panel"]')?.id ?? el.closest('[data-slot="window"]')?.id ?? "-"}`),
  };
}, pattern);
console.log(JSON.stringify(census, null, 2));
writeFileSync(join(OUT, `b3d-undo-row-census-${variant}.txt`), [JSON.stringify(census, null, 2), "", "# console", ...log].join("\n"));
await browser.close();
