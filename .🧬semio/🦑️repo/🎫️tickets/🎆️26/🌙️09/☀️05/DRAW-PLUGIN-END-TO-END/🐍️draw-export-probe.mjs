/** 📤️ Draw PDF export probe (ticket 26/09/05/DRAW-PLUGIN-END-TO-END, goal "export to pdf end to end"):
 * boots the draw react playground (6064) on the Demo example, opens the Canvas window's Actions pane,
 * presses the `Export PDF` row (`action.exportDocument`, arg-less → pdf) and captures the browser
 * download the host performs for `Effect::DownloadMediaExport`. The file is saved next to the report
 * and checked: PDF 1.4 header/trailer, one page, the artboard MediaBox, painting operators present.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=draw-export-1 bun 🐍️draw-export-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync, readFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6064/?plugin=draw";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "draw-export");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
const page = await context.newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
page.on("worker", (worker) => { worker.on("console", (msg) => lines.push(`${Date.now() - t0} worker:${msg.type()} ${msg.text().slice(0, 1200)}`)); });
const report = { url, steps: [] };
const note = (step, detail) => { report.steps.push({ step, t: Date.now() - t0, detail }); console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1500)}`); writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2)); writeFileSync(join(outDir, "console.txt"), lines.join("\n")); };
const state = () => page.evaluate(() => {
  const body = document.body.innerText.replace(/\s+/g, " ");
  const status = body.match(/(\d+) layers? · (\d+) selected/);
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    layers: status ? Number(status[1]) : null,
    fixture: document.querySelector("#playground\\.navbar\\.fixture")?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.layers !== null && s.layers > 0 && i > 8) break; if (s.error) break; }
note("boot", s);

const toggle = page.locator('[id="framework.window.drawingComposite.engagement.toggle"]').first();
let toggled = "absent";
if (await toggle.count()) toggled = await toggle.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
await page.waitForTimeout(1500);
const opened = await state();
const rowId = "action.exportDocument";
const label = await page.locator(`[id="${rowId}"]`).first().innerText().catch(() => null);
const downloadPromise = page.waitForEvent("download", { timeout: 60000 }).catch((e) => ({ error: String(e).slice(0, 200) }));
let clicked = "absent";
if (opened.actionRows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
const download = await downloadPromise;
let saved = null;
if (download && !download.error) {
  saved = { suggested: download.suggestedFilename(), path: join(outDir, download.suggestedFilename()) };
  await download.saveAs(saved.path);
  const bytes = readFileSync(saved.path);
  const text = bytes.toString("latin1");
  saved.bytes = bytes.length;
  saved.header = text.slice(0, 8);
  saved.trailer = text.slice(-6).trim();
  saved.pages = (text.match(/\/Type \/Page(?![s])/g) ?? []).length;
  saved.mediaBox = text.match(/\/MediaBox \[[^\]]*\]/)?.[0] ?? null;
  saved.flateStreams = (text.match(/\/FlateDecode/g) ?? []).length;
  saved.xref = text.includes("\nxref\n") && text.includes("startxref");
}
await page.waitForTimeout(1500);
const faults = lines.filter((l) => /action failed|trapped|panicked|pageerror|unreachable|export/i.test(l)).slice(-12).map((l) => l.slice(0, 300));
note("export-pdf", { toggled, label, clicked, download: download?.error ?? "ok", saved, faults, ...(await state()) });
await page.screenshot({ path: join(outDir, "export.png") }).catch(() => {});
await browser.close();
console.log("[DEBUG] EXPORT DONE");
