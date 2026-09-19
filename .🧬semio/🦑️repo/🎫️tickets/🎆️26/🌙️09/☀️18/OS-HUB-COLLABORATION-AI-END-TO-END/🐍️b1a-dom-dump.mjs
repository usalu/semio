/** 🔎️ One-off DOM id dump for a live B1a playground — used to find the real History-tab control id
 * (the shell's `framework.panel.history` turned out to be a panel *toggle*, not a tab selector).
 * Usage: SEMIO_B1A_PORT=6090 SEMIO_B1A_PLUGIN=architect bun 🐍️b1a-dom-dump.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const plugin = process.env.SEMIO_B1A_PLUGIN ?? "architect";
const port = process.env.SEMIO_B1A_PORT ?? "6090";
const outDir = join(dirname(fileURLToPath(import.meta.url)), "🗑️generated");
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(`http://127.0.0.1:${port}/?plugin=${plugin}`, { waitUntil: "domcontentloaded", timeout: 60000 });
for (let i = 0; i < 60 && !(await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))); i++) await page.waitForTimeout(1000);
await page.waitForTimeout(5000);

const dump = () => page.evaluate(() => ({
  ids: [...document.querySelectorAll("[id]")].map((element) => `${element.id} <${element.tagName.toLowerCase()} role=${element.getAttribute("role") ?? ""} aria-selected=${element.getAttribute("aria-selected") ?? ""} disabled=${element.hasAttribute("disabled")}> ${(element.innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 40)}`),
  history: document.querySelector("[data-history-json]")?.getAttribute("data-history-json") ?? null,
}));

const before = await dump();
writeFileSync(join(outDir, `b1a-${plugin}-dom-before.txt`), before.ids.join("\n"));
process.stdout.write(`before ids=${before.ids.length}\n`);
for (const id of ["framework.panel.history"]) {
  await page.locator(`[id="${id}"]`).first().click({ force: true, timeout: 5000 }).catch(() => {});
  await page.waitForTimeout(1500);
}
const after = await dump();
writeFileSync(join(outDir, `b1a-${plugin}-dom-after.txt`), after.ids.join("\n"));
process.stdout.write(`after ids=${after.ids.length} history=${after.history}\n`);
await browser.close();
