#!/usr/bin/env bun
/** 🧊️ S15: does one rail verb wedge a spawned program's dispatch lane? Opens a program, answers every worker's
 * responsiveness (a 5 s `evaluate` round trip per worker), presses ONE rail verb, re-measures the workers, then presses a
 * probe verb that must either change the ledger or refuse loudly, and records whether it did.
 * usage: bun s15-lane-wedge.mjs <baseUrl> <tag> <pluginId> <appId> <query> <suspectVerb> <probeVerb> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl = "http://127.0.0.1:6540/", tag = "jack", pluginId = "trinity", appId = "s.trinity.jack@1/*#editor", query = "jack", suspect = "clearSelection", probeVerb = "selectAll"] = process.argv.slice(2);
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const out = fileURLToPath(new URL(`./generated/s15-lane-wedge-${tag}.json`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const lines = [];
page.on("console", (message) => { if (!/\[vite\]|freshness|DevTools/u.test(message.text())) lines.push(`${new Date().toISOString().slice(11, 23)} ${message.type()} ${message.text()}`.slice(0, 300)); });
const workers = () => Promise.all(page.workers().map(async (worker) => {
  const started = Date.now();
  const answer = await Promise.race([worker.evaluate(() => typeof self).then(() => "ok", (error) => `error ${String(error).slice(0, 60)}`), new Promise((resolve) => setTimeout(() => resolve("TIMEOUT"), 5_000))]);
  return `${worker.url().split("/").at(-1)?.slice(0, 50)}:${answer}:${Date.now() - started}ms`;
}));
const ledger = () => page.evaluate(() => [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).length);
const result = { tag, suspect, probeVerb, steps: [] };
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await sweep.awaitBeacon(page, Date.now() + 300_000);
  await sweep.dismissIntroduction(page);
  await page.waitForTimeout(3_000);
  await page.evaluate(() => document.body.focus());
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(query);
  await page.waitForTimeout(1_200);
  await page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}.${appId}"], [data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first().click({ timeout: 10_000 });
  await page.waitForTimeout(12_000);
  await sweep.unfoldActionsRail(page);
  const open = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((element) => element instanceof HTMLElement && element.offsetParent !== null && /framework\.panel\.history/u.test(element.id)));
  if (!open) await sweep.click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1_500);
  result.workersBefore = await workers();
  result.ledgerBefore = await ledger();
  lines.push("---- suspect");
  result.steps.push(`${suspect}: ${await sweep.clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${suspect}"]`)}`);
  await page.waitForTimeout(3_000);
  result.workersAfterSuspect = await workers();
  result.ledgerAfterSuspect = await ledger();
  lines.push("---- probe verb");
  result.steps.push(`${probeVerb}: ${await sweep.clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${probeVerb}"]`)}`);
  await page.waitForTimeout(3_000);
  result.steps.push(`nudge: ${await sweep.clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${probeVerb}"]`)}`);
  await page.waitForTimeout(3_000);
  result.ledgerAfterProbe = await ledger();
  result.workersAfterProbe = await workers();
} catch (error) {
  result.fatal = String(error).slice(0, 300);
} finally {
  result.console = lines.slice(-400);
  writeFileSync(out, JSON.stringify(result, null, 1));
  await browser.close();
}
console.log(JSON.stringify({ ...result, console: undefined }, null, 1));
console.log(result.console.join("\n"));
