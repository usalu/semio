/** 📕️ Layout PDF export probe: boots the layout react playground (6079), unfolds the Blueprint window's
 * Actions pane, clicks the arg-less `exportPdf` row, captures the browser download the segmented
 * export lane delivers, saves it next to the report and records every console line of the run.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=layout-export-1 bun 🐍️layout-export-pdf-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync, statSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6079/?plugin=layout";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "layout-export");
const actionId = process.env.SEMIO_PROBE_ACTION ?? "exportPdf";
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 60);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
const page = await context.newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 800)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, actionId, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|refused|export.*(fail|error)/i.test(l)).map((l) => l.slice(0, 500));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1500)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const state = () => page.evaluate(() => ({
  ready: document.documentElement.getAttribute("data-semio-os-ready"),
  error: document.documentElement.getAttribute("data-semio-os-error"),
  hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => el.getAttribute("data-surface-id")),
  engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
  actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
}));

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && i > 8) break; if (s.error) break; }
await note("boot", s, 0);

{
  const from = lines.length;
  const engagement = s.engagements.find((id) => /blueprint/i.test(id)) ?? s.engagements[0];
  const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
  const toggled = (await toggle.count()) ? await toggle.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  await page.waitForTimeout(1500);
  // 🖼️ `SEMIO_PROBE_ADD_FRAME=1`: mutate the document first (Actions pane `addFrame` → Execute), so the
  // export must carry the live document — the new rect fills (48,48,120×64) with 0.2/0.24/0.3.
  let added = "skipped";
  if (process.env.SEMIO_PROBE_ADD_FRAME === "1") {
    added = await page.locator('[id="action.addFrame"]').first().click({ timeout: 8000, force: true }).then(() => "row").catch((e) => String(e).slice(0, 120));
    await page.waitForTimeout(1500);
    const execute = page.locator('[id$=".action.addFrame.execute"]').first();
    added = (await execute.count()) ? await execute.click({ timeout: 8000 }).then(() => "executed").catch((e) => String(e).slice(0, 120)) : "no-execute-control";
    await page.waitForTimeout(4000);
  }
  const opened = await state();
  const rowId = `action.${actionId}`;
  const downloadPromise = page.waitForEvent("download", { timeout: settleSeconds * 1000 }).then(async (download) => {
    const suggested = download.suggestedFilename();
    const target = join(outDir, suggested || `${actionId}.bin`);
    await download.saveAs(target);
    return { suggested, target, bytes: statSync(target).size };
  }).catch((e) => ({ error: String(e).slice(0, 200) }));
  let clicked = "absent";
  if (opened.actionRows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const submit = page.locator(`[id$=".action.${actionId}.execute"]`).first();
  const submitted = (await submit.count()) ? await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "row-executes-directly";
  const download = await downloadPromise;
  if (download.target) {
    const { spawnSync } = await import("node:child_process");
    const validated = spawnSync("python3", [join(import.meta.dir, "🐍️validate-pdf.py"), download.target], { encoding: "utf8" });
    download.validation = validated.status === 0 ? JSON.parse(validated.stdout.trim().split("\n").pop()) : { error: validated.stderr.slice(0, 600) };
  }
  const exportLines = lines.slice(from).filter((l) => /export|download|segmented|media/i.test(l)).slice(0, 40).map((l) => l.slice(0, 300));
  await note("export", { engagement, toggled, added, rowId, clicked, submitted, download, exportLines }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] EXPORT DONE");
await browser.close();
