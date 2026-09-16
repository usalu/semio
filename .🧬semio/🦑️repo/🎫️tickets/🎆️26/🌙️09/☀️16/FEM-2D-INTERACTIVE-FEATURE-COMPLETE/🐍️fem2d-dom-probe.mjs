/** 🔎️ Fem2d DOM probe: boots the react lane, opens each dock tab and lists every `panel:` element id plus the
 * window surfaces' data attributes — the id vocabulary the interaction probes drive.
 * Usage: SEMIO_PROBE_URL=... SEMIO_PROBE_OUT=fem2d-dom-1 bun 🐍️fem2d-dom-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6086/?plugin=fem2d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "fem2d-dom");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`pageerror ${String(e).slice(0, 600)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 60; i++) { await page.waitForTimeout(1000); const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")); if (ready && i > 10) break; }
const report = {};
const ids = () => page.evaluate(() => [...document.querySelectorAll('[id^="panel:"]')].map((e) => e.id).slice(0, 200));
report.tabs = await page.evaluate(() => [...document.querySelectorAll("button")].map((b) => b.textContent?.trim()).filter((t) => t && t.length < 24).slice(0, 60));
for (const name of ["Artifact", "Inspection", "Results"]) {
  const tab = page.getByRole("button", { name, exact: true }).first();
  if (await tab.count()) { await tab.click(); await page.waitForTimeout(2500); }
  report[name] = await ids();
  await page.screenshot({ path: join(outDir, `${name}.png`) }).catch(() => {});
}
report.surfaces = await page.evaluate(() => [...document.querySelectorAll("[data-surface-id]")].map((el) => { const attrs = {}; for (const a of el.attributes) if (a.name.startsWith("data-")) attrs[a.name] = a.value.length > 120 ? a.value.slice(0, 120) + "…" : a.value; return attrs; }));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(JSON.stringify(report).slice(0, 6000));
await browser.close();
