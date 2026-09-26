#!/usr/bin/env bun
/** 💼️ U5 S12-2 — the Tasks window with a REAL spawned job: hooks the page's own job-ledger module instance (the
 * exact vite URL the plugin runtime imported, found in the resource timings) to timestamp every ledger row the
 * browser drives, opens the Tasks window, spawns `<pluginId>.<appId>` from the palette, optionally dispatches
 * one palette verb (`--verb=<label>`) to start work, then exercises Suspend → Resume → Cancel on the first job row
 * and reads the row's accessible names in the current language. Usage:
 * bun u5-job-ledger-probe.mjs <baseUrl> <pluginId> <appId> <tag> [--locale=de] [--verb=<paletteText>] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dismissIntroduction, openPalette, seatLocale } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl, pluginId, appId, tag, ...flags] = process.argv.slice(2);
const flag = (name) => flags.find((value) => value.startsWith(`--${name}=`))?.slice(name.length + 3);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
page.on("console", (m) => lines.push(`${new Date().toISOString().slice(11, 23)} ${m.type()}: ${m.text()}`.slice(0, 300)));
const report = { baseUrl, pluginId, appId, tag, steps: [] };
const step = (at, extra = {}) => { report.steps.push({ at, t: new Date().toISOString().slice(11, 23), ...extra }); console.log(JSON.stringify({ at, ...extra }).slice(0, 600)); };

await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(2_000);
if (flag("locale")) step("locale", { seated: await seatLocale(page, flag("locale")) });
const ledgerUrl = await page.evaluate(() => performance.getEntriesByType("resource").map((entry) => entry.name).find((name) => decodeURIComponent(name).includes("job-ledger")) ?? null);
step("ledger module", { ledgerUrl });
await page.evaluate(async (url) => {
  const ledger = await import(url);
  window.__u5Jobs = [];
  const seen = new Map();
  ledger.subscribeSpawnedJobsV1(() => {
    const now = Math.round(performance.now());
    const live = ledger.spawnedJobsSnapshotV1();
    for (const row of live) {
      const summary = `${row.steps}|${row.cancelling}|${row.suspended ?? ""}`;
      if (seen.get(row.key) === summary) continue;
      seen.set(row.key, summary);
      window.__u5Jobs.push({ t: now, key: row.key, kind: row.kind, steps: row.steps, reportsProgress: row.reportsProgress, cancelling: row.cancelling, suspended: row.suspended ?? null });
    }
    for (const key of [...seen.keys()]) if (!live.some((row) => row.key === key)) { seen.delete(key); window.__u5Jobs.push({ t: now, key, ended: true }); }
  });
}, ledgerUrl);
await page.locator('[id="os.task-manager"]').first().click({ timeout: 10_000 }).catch(() => undefined);
await page.waitForSelector("[data-semio-task-manager-window]", { timeout: 20_000 }).catch(() => undefined);
await openPalette(page);
const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? pluginId);
const item = page.locator(`[data-slot="command-item"][data-command-item-id="${appId === "-" ? `spawn.${pluginId}` : `spawn.${pluginId}.${appId}`}"]`).first();
await item.waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
step("spawn", { found: await item.count(), input: await input.count(), listed: await page.evaluate(() => [...document.querySelectorAll('[data-slot="command-item"]')].map((el) => el.getAttribute("data-command-item-id")).slice(0, 12)) });
await page.screenshot({ path: out(`u5-job-${tag}-palette.png`) });
await item.click({ force: true }).catch(() => undefined);
await page.waitForTimeout(15_000);
if (flag("verb")) {
  await openPalette(page);
  await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(flag("verb"));
  await page.waitForTimeout(1_500);
  const verb = page.locator('[role="dialog"] [data-slot="command-item"]').first();
  step("verb", { text: (await verb.textContent().catch(() => null))?.trim() ?? null });
  await verb.click({ force: true }).catch(() => undefined);
}
const running = await page.waitForSelector("[data-semio-task-manager-lane='job']", { timeout: 60_000 }).then(() => true).catch(() => false);
const rowState = () => page.evaluate(() => [...document.querySelectorAll("[data-semio-task-manager-lane='job']")].map((row) => ({ id: row.getAttribute("data-semio-task-manager-task"), state: row.getAttribute("data-semio-task-manager-state"), progress: row.querySelector("[role='progressbar']")?.getAttribute("aria-valuetext") ?? null, label: row.querySelector("[role='progressbar']")?.getAttribute("aria-label") ?? null, buttons: [...row.querySelectorAll("button")].map((button) => ({ id: button.id, name: button.getAttribute("aria-label") ?? button.textContent?.trim(), disabled: button.disabled })) })));
step("job row", { running, rows: await rowState() });
await page.screenshot({ path: out(`u5-job-${tag}-running.png`) });
if (running) {
  const id = await page.locator("[data-semio-task-manager-lane='job']").first().getAttribute("data-semio-task-manager-task");
  for (const [action, wait] of [["suspend", 3_000], ["resume", 3_000], ["cancel", 0]]) {
    const button = page.locator(`[id="os.task-manager.${action}.${id}"]`);
    const present = await button.count();
    if (present) await button.click({ timeout: 5_000 }).catch(() => undefined);
    await page.waitForTimeout(wait || 500);
    const jobs = await page.evaluate(() => window.__u5Jobs.slice(-3));
    step(action, { present, rows: await rowState(), jobs });
  }
  const gone = await page.waitForFunction((jobId) => !document.querySelector(`[data-semio-task-manager-task="${jobId}"]`), id, { timeout: 30_000 }).then(() => true).catch(() => false);
  step("after cancel", { gone });
}
report.jobs = await page.evaluate(() => window.__u5Jobs);
report.faults = lines.filter((line) => /pageerror|trap|unreachable|fault|job\./iu.test(line)).slice(-30);
writeFileSync(out(`u5-job-${tag}.json`), JSON.stringify(report, null, 1));
console.log(`jobs recorded: ${report.jobs.length}; kinds: ${[...new Set(report.jobs.map((row) => row.kind).filter(Boolean))].join(", ")}`);
await browser.close();
