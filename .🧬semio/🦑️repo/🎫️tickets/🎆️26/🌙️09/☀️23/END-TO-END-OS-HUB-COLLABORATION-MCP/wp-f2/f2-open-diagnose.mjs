#!/usr/bin/env bun
/** 🩺️ F2 — why a program does not open: boots `s`, opens each given program from the palette, and records every console line,
 * page error, transient notice and failed request from the press until 90 s later, plus a screenshot.
 * usage: bun f2-open-diagnose.mjs <baseUrl> <tag> <pluginId>=<appId> [...] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl, tag, ...specs] = process.argv.slice(2);
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
page.setDefaultNavigationTimeout(300_000);
const lines = [];
const stamp = () => new Date().toISOString().slice(11, 23);
page.on("console", (message) => lines.push(`${stamp()} ${message.type()}: ${message.text()}`.slice(0, 600)));
page.on("pageerror", (error) => lines.push(`${stamp()} pageerror: ${String(error?.stack ?? error)}`.slice(0, 900)));
page.on("requestfailed", (request) => lines.push(`${stamp()} requestfailed: ${request.failure()?.errorText} ${decodeURIComponent(request.url()).slice(0, 160)}`));
page.on("response", (response) => { if (response.status() >= 400) lines.push(`${stamp()} http ${response.status()} ${decodeURIComponent(response.url()).slice(0, 160)}`); });
page.on("worker", (worker) => worker.on("console", (message) => { if (/error|fault|trap|panic|fail/iu.test(message.text())) lines.push(`${stamp()} worker ${message.type()}: ${message.text()}`.slice(0, 600)); }));
await page.addInitScript(() => {
  const seen = [];
  Object.defineProperty(window, "__f2Notices", { value: seen });
  new MutationObserver(() => {
    for (const element of document.querySelectorAll("[data-semio-transient-notice], [role='alert']")) {
      const text = (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 300);
      if (text && !seen.includes(text)) seen.push(text);
    }
  }).observe(document, { subtree: true, childList: true, characterData: true });
});
const report = { baseUrl, tag, programs: [] };
await page.goto(baseUrl, { waitUntil: "commit" });
report.beacon = await sweep.awaitBeacon(page, Date.now() + 300_000);
await sweep.dismissIntroduction(page);
await page.keyboard.press("Escape").catch(() => undefined);
await page.waitForTimeout(8_000);
const probe = await page.evaluate(() => window.__semioOsCatalogProbe ?? null);
for (const spec of specs) {
  const [pluginId, appId] = spec.split("=");
  const cursor = lines.length;
  const plugin = probe?.plugins?.find((row) => row.pluginId === pluginId) ?? null;
  const before = await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")));
  await page.evaluate(() => { if (document.activeElement instanceof HTMLElement) document.activeElement.blur(); document.body.focus(); });
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? appId);
  await page.waitForTimeout(1_500);
  const rows = await page.evaluate(() => [...document.querySelectorAll('[data-slot="command-item"]')].map((element) => element.getAttribute("data-command-item-id")).slice(0, 12));
  let pressed = null;
  for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    if ((await item.count()) > 0) { await item.click({ force: true }).catch(() => undefined); pressed = id; break; }
  }
  if (!pressed) await page.keyboard.press("Escape");
  let opened = [];
  const deadline = Date.now() + 90_000;
  while (pressed && Date.now() < deadline) {
    opened = (await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")))).filter((id) => !before.includes(id));
    if (opened.length > 0) break;
    await page.waitForTimeout(500);
  }
  await page.waitForTimeout(3_000);
  const bodies = await page.evaluate((ids) => ids.map((id) => { const body = document.getElementById(id)?.querySelector('[data-slot="window-body"]'); return { id, elements: body?.querySelectorAll("*").length ?? null, busy: body?.querySelector('[aria-busy="true"]') !== null, text: (body?.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 200) }; }), opened);
  const shot = `${generated}f2-open-${tag}-${pluginId}-${(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? "x")}.png`;
  await page.screenshot({ path: shot }).catch(() => undefined);
  report.programs.push({ pluginId, appId, pluginStatus: plugin?.status ?? null, paletteRows: rows, pressed, opened, bodies, notices: await page.evaluate(() => window.__f2Notices.slice(-8)), lines: lines.slice(cursor).filter((line) => !/\[vite\]|DevTools|staged plugin module/iu.test(line)).slice(0, 60), screenshot: shot });
  console.log(JSON.stringify({ pluginId, appId, pluginStatus: plugin?.status ?? null, pressed, opened, bodies, errors: lines.slice(cursor).filter((line) => /error|fail|trap|panic|fault|refused/iu.test(line)).slice(0, 8) }).slice(0, 3000));
  for (const id of opened) await page.evaluate((windowId) => { const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId); tab?.querySelector('[data-slot="mode-dock-tab-close"]')?.click(); }, id);
  await page.waitForTimeout(2_000);
}
writeFileSync(`${generated}f2-open-${tag}.json`, JSON.stringify(report, null, 1));
await browser.close();
