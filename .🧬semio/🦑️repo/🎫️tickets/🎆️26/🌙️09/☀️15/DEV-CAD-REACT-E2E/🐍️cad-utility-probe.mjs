import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 🧰️ Select a building object, open the Building pane's Utilities popover, activate Dislocate and
// confirm the guest arms the gumball (`gumballActive` in the pane's selection lane).
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6020/?plugin=cad";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 45);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "utility");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 1200)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1200)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(bootSeconds * 1000);
const report = {};
const note = (key, value) => { report[key] = value; lines.push(`${Date.now() - t0} probe ${key} ${JSON.stringify(value).slice(0, 600)}`); };
const guest = (surface) => page.evaluate((s) => JSON.parse(document.querySelector(`[data-surface-id="${s}"]`)?.getAttribute("data-guest-selection-json") ?? "{}"), surface);
try {
  await page.getByRole("button", { name: "Artifact", exact: true }).first().click();
  await page.waitForTimeout(2000);
  const row = page.locator('[id="panel:cad-play-document/object-hexagonal-cut-concrete-forest-left-bim-1"]').first();
  const box = await row.boundingBox();
  await page.mouse.click(box.x + 90, box.y + box.height / 2);
  await page.waitForTimeout(2500);
  note("selected", await guest("window:cad-play-building"));
  // 🧰️ The Building pane's Utilities button sits in its own window chrome (bottom-left of the pane).
  const host = await page.evaluate(() => { const r = document.querySelector('[data-surface-id="window:cad-play-building"]').getBoundingClientRect(); return { x: r.x, y: r.y, w: r.width, h: r.height }; });
  const buttons = await page.evaluate(() => [...document.querySelectorAll("button")].filter((b) => b.textContent?.trim() === "Utilities").map((b) => { const r = b.getBoundingClientRect(); return { x: r.x, y: r.y }; }));
  note("utilityButtons", buttons);
  const target = buttons.find((b) => b.x >= host.x && b.x <= host.x + host.w && b.y >= host.y && b.y <= host.y + host.h);
  await page.mouse.click(target.x + 20, target.y + 8);
  await page.waitForTimeout(1500);
  const items = await page.evaluate(() => [...document.querySelectorAll('[role="menuitem"], [role="menuitemradio"], [role="menuitemcheckbox"], [role="option"], [data-radix-popper-content-wrapper] button, [data-slot="popover-content"] button')].map((e) => ({ text: e.textContent?.trim().slice(0, 40), role: e.getAttribute("role") })));
  note("popoverItems", items);
  await page.screenshot({ path: join(outDir, "popover.png"), type: "png" });
  const dislocate = page.locator('[role="menuitem"], [role="menuitemradio"], [role="option"], button').filter({ hasText: /Dislocate/i }).first();
  if (await dislocate.count()) { await dislocate.click(); await page.waitForTimeout(3000); }
  note("afterDislocate", await guest("window:cad-play-building"));
  note("hostSelection", await page.evaluate(() => JSON.parse(document.querySelector('[data-surface-id="window:cad-play-building"]')?.getAttribute("data-selection-json") ?? "{}")));
  await page.screenshot({ path: join(outDir, "gumball.png"), type: "png" });
} catch (error) {
  note("error", String(error).slice(0, 300));
  await page.screenshot({ path: join(outDir, "error.png"), type: "png" }).catch(() => {});
}
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
const faults = lines.filter((l) => /pageerror|dropped action|Unknown action|fixed-capacity|action failed/i.test(l));
console.log("DONE faults", faults.length);
for (const f of faults.slice(0, 8)) console.log("FAULT", f.slice(0, 300));
console.log(JSON.stringify(report, null, 1).slice(0, 3000));
await browser.close();
