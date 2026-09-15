/** 🔍 What the shell paints after `exportDocument` is dispatched from the context menu.
 *
 * `menus · export-formats` now REACHES the row by pointer and by keyboard, but reads zero formats:
 * the step's `[role="combobox"]` filter finds nothing. This dumps every combobox, dialog, form field
 * and panel heading standing 1 s / 4 s / 10 s after the click so the step can aim at what is actually
 * there rather than at what it was guessed to be.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-sweep/export-form bun 🐍️export-form-recon.mjs
 * @see 🐍️menu-dump-probe.mjs, 🐍️viewer-actions-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "export-form");
mkdirSync(outDir, { recursive: true });
const MAIN = "window:procedural-main";
const lines = [];
const t0 = Date.now();
const report = { url, steps: [] };

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 600)}`));
const say = (step, detail) => {
  report.steps.push({ step, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1600)}`);
};

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 200; i += 1) {
  await page.waitForTimeout(1000);
  if ((await page.locator(`[data-surface-id="${MAIN}"]`).count()) > 0) break;
}
await page.waitForTimeout(8000);

const box = await page.locator(`[data-surface-id="${MAIN}"]`).first().boundingBox();
await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.82, { button: "right" });
await page.waitForTimeout(1500);
await page.locator('[id="menu.group.transfer"]').first().hover({ timeout: 4000 }).catch((e) => lines.push(`group hover ${String(e).slice(0, 140)}`));
await page.waitForTimeout(1200);
await page.locator('[id="exportDocument"]').first().hover({ timeout: 6000 }).catch((e) => lines.push(`row hover ${String(e).slice(0, 140)}`));
const clicked = await page
  .locator('[id="exportDocument"]')
  .first()
  .click({ timeout: 6000 })
  .then(() => true)
  .catch((e) => {
    lines.push(`row click ${String(e).slice(0, 200)}`);
    return false;
  });
say("clicked", { clicked });

const dump = () =>
  page.evaluate(() => {
    const brief = (el) => ({
      tag: el.tagName,
      id: el.id || null,
      role: el.getAttribute("role"),
      slot: el.getAttribute("data-slot"),
      name: el.getAttribute("aria-label") ?? el.getAttribute("name") ?? null,
      text: (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 80),
    });
    return {
      comboboxes: [...document.querySelectorAll('[role="combobox"]')].map(brief),
      selects: [...document.querySelectorAll("select")].map(brief),
      dialogs: [...document.querySelectorAll('[role="dialog"], [data-slot="dialog-content"]')].map(brief),
      options: [...document.querySelectorAll('[role="option"]')].map((el) => ({ value: el.getAttribute("data-value"), text: (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40) })),
      staged: [...document.querySelectorAll("[data-action-arg-form], [data-slot*='arg'], [data-staged-action]")].map(brief),
      panels: [...document.querySelectorAll('[data-slot="panel"] h1, [data-slot="panel"] h2, [data-slot="panel"] h3, [data-slot="panel-title"]')].map((el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60)),
      menusOpen: document.querySelectorAll('[role="menu"]').length,
    };
  });

for (const wait of [1000, 3000, 6000]) {
  await page.waitForTimeout(wait);
  say(`after-${wait}`, await dump());
}
await page.screenshot({ path: join(outDir, "after-export.png"), fullPage: false });
writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] EXPORT FORM RECON DONE -> ${outDir}`);
await browser.close();
