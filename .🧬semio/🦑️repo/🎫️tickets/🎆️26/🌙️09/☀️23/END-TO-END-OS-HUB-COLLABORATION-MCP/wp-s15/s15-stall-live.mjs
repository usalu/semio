#!/usr/bin/env bun
/** ⏱️ S15 — live proof of the command stall band inside `s`: a real program's real command made slower than the schema bound
 * (5 s) by throttling the page's CPU through CDP (the only thing the probe changes), then: the band names program + command in
 * the run's locale and counts seconds; Cancel rejects the held turn and releases the lane; with the CPU restored the same verb
 * runs again and moves the document.
 * usage: bun s15-stall-live.mjs <baseUrl> <pluginId> <verbId> [--locale de] [--rate 40] [--tag t] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const argv = process.argv.slice(2);
const [baseUrl = "http://127.0.0.1:6540/", pluginId = "dag", verbId = "addNode"] = argv;
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const locale = valueOf("--locale", "en");
const rate = Number(valueOf("--rate", "40"));
const tag = valueOf("--tag", `${pluginId}-${locale}`);
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const out = fileURLToPath(new URL(`./generated/s15-stall-live-${tag}.json`, import.meta.url));
const t0 = Date.now();
const result = { baseUrl, pluginId, verbId, locale, rate, timeline: [] };
const mark = (label, detail) => {
  result.timeline.push({ atMs: Date.now() - t0, label, detail });
  console.log(`[s15-stall] @${Date.now() - t0} ${label} ${detail === undefined ? "" : JSON.stringify(detail).slice(0, 300)}`);
};
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const lines = [];
page.on("console", (message) => {
  if (/command stalled|command-cancelled|refused|\[os-shell\]/iu.test(message.text())) lines.push(`@${Date.now() - t0} ${message.type()} ${message.text()}`.slice(0, 300));
});
const band = () => page.evaluate(() => [...document.querySelectorAll("[data-semio-command-stall-id]")].map((row) => ({ id: row.getAttribute("data-semio-command-stall-id"), command: row.getAttribute("data-command-id"), text: (row.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 200) })));
const press = () => sweep.clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${verbId}"]`);
const historyRows = () => page.evaluate(() => [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).map((element) => (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40)));
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await sweep.awaitBeacon(page, Date.now() + 300_000);
  await sweep.dismissIntroduction(page);
  await page.waitForTimeout(2_000);
  if (locale !== "en") mark("locale", await sweep.seatLocale(page, locale));
  await page.keyboard.press("Escape").catch(() => undefined);
  const spawned = await sweep.spawnProgram(page, pluginId);
  mark("spawn", spawned);
  await page.waitForTimeout(4_000);
  await sweep.unfoldActionsRail(page);
  await sweep.click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1_000);
  const ledgerBefore = (await historyRows()).length;
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate });
  mark("cpu throttled", rate);
  mark("press", await press());
  const deadline = Date.now() + 60_000;
  let seen = [];
  while (Date.now() < deadline) {
    seen = await band();
    if (seen.length > 0) break;
    await page.waitForTimeout(500);
  }
  result.bandFirst = seen;
  mark("band", seen);
  if (seen.length > 0 && !argv.includes("--no-cancel")) {
    result.bandLater = await band();
    mark("band later", result.bandLater);
    mark("cancel", await page.evaluate(() => {
      const button = document.querySelector("[data-semio-command-stall-cancel]");
      if (!(button instanceof HTMLElement)) return "absent";
      button.click();
      return "clicked";
    }));
    const cancelledAt = Date.now();
    while (Date.now() - cancelledAt < 30_000 && (await band()).length > 0) await page.waitForTimeout(250);
    result.bandGoneMs = (await band()).length === 0 ? Date.now() - cancelledAt : null;
    mark("band after cancel", { goneMs: result.bandGoneMs });
  }
  await cdp.send("Emulation.setCPUThrottlingRate", { rate: 1 });
  mark("cpu restored");
  await page.waitForTimeout(3_000);
  const after = await sweep.mutateUndoRedo(page, [], pluginId);
  result.afterCancel = { verb: after.mutation, detail: after.mutationDetail, edits: after.edits };
  result.laneReleased = after.mutation !== null && after.mutationDetail === null;
  result.historyTail = (await historyRows()).slice(-4);
  mark("verb/undo/redo after cancel", result.afterCancel);
  await page.screenshot({ path: fileURLToPath(new URL(`./generated/s15-stall-live-${tag}.png`, import.meta.url)) }).catch(() => undefined);
} catch (error) {
  result.error = String(error).slice(0, 300);
  mark("error", result.error);
} finally {
  result.lines = lines.slice(-20);
  writeFileSync(out, JSON.stringify(result, null, 1));
  await browser.close();
}
console.log(`=== ${tag} band=${(result.bandFirst ?? []).length > 0} released=${result.laneReleased ?? false} → ${out}`);
