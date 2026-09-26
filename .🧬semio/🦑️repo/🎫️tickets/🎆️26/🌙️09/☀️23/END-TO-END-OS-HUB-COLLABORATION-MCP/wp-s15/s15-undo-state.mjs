#!/usr/bin/env bun
/** ↩️ S15: what the shell offers for undo right after ONE verb on a spawned program, with no neutral dispatch in between:
 * every History row (text, whether its `.revert` control exists), whether `action.undo` is enabled, `Check in (n)`; then
 * ONE undo press and the same readings, then quiet, then one neutral dispatch. Answers T12's question whether a
 * non-undoable row is the browser's actor identity or the guest's inverse.
 * usage: bun s15-undo-state.mjs <baseUrl> <tag> <pluginId> <appId> <paletteQuery> <preVerb|-> <verb> <argsJson> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl = "http://127.0.0.1:6540/", tag = "jack", pluginId = "trinity", appId = "s.trinity.jack@1/*#editor", query = "jack", preVerb = "selectAll", verb = "patchNodes", argsJson = '{"field":"name","value":"S15 Node"}'] = process.argv.slice(2);
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const out = fileURLToPath(new URL(`./generated/s15-undo-state-${tag}.json`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleLines = [];
if (process.env.S15_DIAG === "1") await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (message) => {
  const text = message.text();
  if (process.env.S15_CONSOLE_ALL === "1" ? !/\[vite\]|freshness/u.test(text) : /actor|undo|history|refused|revert/iu.test(text)) consoleLines.push(`${new Date().toISOString().slice(11, 23)} ${message.type()} ${text}`.slice(0, 400));
});
const read = (label) =>
  page.evaluate((label) => {
    const text = (element) => (element?.innerText ?? element?.textContent ?? "").replace(/\s+/gu, " ").trim();
    const entries = [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert"));
    const undo = document.querySelector('[data-slot="window-action-pane"] [id="action.undo"]');
    const redo = document.querySelector('[data-slot="window-action-pane"] [id="action.redo"]');
    return {
      label,
      at: new Date().toISOString(),
      checkin: text(document.querySelector("#s-checkin")),
      undo: undo === null ? "absent" : { disabled: undo.hasAttribute("disabled") || undo.getAttribute("aria-disabled") === "true", text: text(undo).slice(0, 40) },
      redo: redo === null ? "absent" : { disabled: redo.hasAttribute("disabled") || redo.getAttribute("aria-disabled") === "true" },
      history: entries.map((element) => ({ id: element.id, text: text(element).slice(0, 120), revert: document.getElementById(`${element.id}.revert`) !== null, attrs: Object.fromEntries([...element.attributes].filter((attribute) => attribute.name.startsWith("data-") || attribute.name.startsWith("aria-")).map((attribute) => [attribute.name, attribute.value])) })),
    };
  }, label);
const result = { baseUrl, tag, pluginId, appId, preVerb, verb, readings: [], steps: [], rails: [] };
/** 🎯️ Every rail row of `actionId`: its window, whether a pointer at its centre hits it, and which window is active. */
const railRows = (label, actionId) =>
  page.evaluate(({ label, actionId }) => ({
    label,
    activeWindow: document.querySelector('[data-window-active="true"], [data-active="true"][data-window-id]')?.getAttribute("data-window-id") ?? null,
    rows: [...document.querySelectorAll(`[data-slot="window-action-pane"] [id="${actionId}"]`)].map((element) => {
      const rect = element.getBoundingClientRect();
      const top = rect.width > 0 ? document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2) : null;
      return { window: element.closest("[data-window-id]")?.getAttribute("data-window-id") ?? null, visible: rect.width > 0 && rect.height > 0, hit: top !== null && (top === element || element.contains(top)) };
    }),
  }), { label, actionId });
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await sweep.awaitBeacon(page, Date.now() + 300_000);
  await sweep.dismissIntroduction(page);
  await page.waitForTimeout(3_000);
  await page.evaluate(() => document.body.focus());
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(query);
  await page.waitForTimeout(1_200);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}.${appId}"], [data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first();
  await item.click({ timeout: 10_000 });
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline && (await sweep.windowIds(page)).filter((id) => id !== "s-home-main").length === 0) await page.waitForTimeout(500);
  await page.waitForTimeout(4_000);
  result.windows = await sweep.windowIds(page);
  result.steps.push(`rail unfolded ${await sweep.unfoldActionsRail(page)}`);
  const historyOpen = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((element) => element instanceof HTMLElement && element.offsetParent !== null && /framework\.panel\.history/u.test(element.id)));
  result.steps.push(`history ${historyOpen ? "already-open" : await sweep.click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]')}`);
  await page.waitForTimeout(1_500);
  result.readings.push(await read("opened"));
  if (preVerb !== "-") {
    result.steps.push(`pre ${preVerb}: ${await sweep.clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${preVerb}"]`)}`);
    await page.waitForTimeout(1_500);
  }
  result.steps.push(`verb ${verb}: ${await sweep.clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${verb}"]`)}`);
  await page.waitForTimeout(1_000);
  for (const [key, value] of Object.entries(JSON.parse(argsJson))) result.steps.push(`arg ${await sweep.fillStagedArgument(page, key, value)}`);
  result.steps.push(`submit ${await sweep.submitStagedVerb(page, verb)}`);
  await page.waitForTimeout(2_500);
  result.readings.push(await read("after verb, no neutral dispatch"));
  await page.screenshot({ path: out.replace(/\.json$/u, "-after-verb.png") });
  await page.waitForTimeout(6_000);
  result.readings.push(await read("after verb + 6 s quiet"));
  result.selectionAfterVerb = await page.evaluate(() => [...document.querySelectorAll('[aria-selected="true"], [data-selected="true"]')].length);
  if (process.env.S15_NEUTRAL_FIRST === "1") {
    result.rails.push(await railRows("clearSelection before neutral", "action.clearSelection"));
    const profiler = process.env.S15_PROFILE === "1" ? await page.context().newCDPSession(page) : null;
    if (profiler) { await profiler.send("Profiler.enable"); await profiler.send("Profiler.setSamplingInterval", { interval: 500 }); await profiler.send("Profiler.start"); }
    result.steps.push(`neutral-before-undo ${await sweep.neutralDispatch(page)}`);
    if (profiler) {
      await page.waitForTimeout(8_000);
      const { profile } = await profiler.send("Profiler.stop");
      const byId = new Map(profile.nodes.map((node) => [node.id, node]));
      const self = new Map();
      const deltas = profile.timeDeltas ?? [];
      profile.samples.forEach((id, index) => { const node = byId.get(id); const frame = node.callFrame; const key = `${frame.functionName || "(anon)"} ${frame.url.split("/").pop()?.slice(0, 50)}:${frame.lineNumber}`; self.set(key, (self.get(key) ?? 0) + (deltas[index] ?? 0)); });
      const total = [...self.values()].reduce((sum, value) => sum + value, 0);
      const parent = new Map();
      for (const node of profile.nodes) for (const child of node.children ?? []) parent.set(child, node.id);
      const inclusive = new Map();
      profile.samples.forEach((id, index) => {
        const seen = new Set();
        for (let cursor = id; cursor !== undefined; cursor = parent.get(cursor)) {
          const frame = byId.get(cursor).callFrame;
          if (!/\.(tsx?|js|mjs)(\?|$)/u.test(frame.url)) continue;
          const key = `${frame.functionName || "(anon)"} ${decodeURIComponent(frame.url.split("/").slice(-3).join("/")).slice(0, 90)}:${frame.lineNumber + 1}`;
          if (seen.has(key)) continue;
          seen.add(key);
          inclusive.set(key, (inclusive.get(key) ?? 0) + (deltas[index] ?? 0));
        }
      });
      result.profile = { totalMs: Math.round(total / 1000), top: [...self].sort((a, b) => b[1] - a[1]).slice(0, 12).map(([key, value]) => `${Math.round(value / 1000)}ms ${decodeURIComponent(key)}`), inclusiveJs: [...inclusive].sort((a, b) => b[1] - a[1]).slice(0, 40).map(([key, value]) => `${Math.round(value / 1000)}ms ${key}`) };
    }
    result.readings.push(await read("after one neutral dispatch BEFORE undo"));
    await page.screenshot({ path: out.replace(/\.json$/u, "-after-neutral.png") });
    if (process.env.S15_PROBE_QUEUE === "1") {
      consoleLines.push(`${new Date().toISOString().slice(11, 23)} ---- second ${verb} (queue alive?)`);
      result.steps.push(`second ${verb}: ${await sweep.clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${verb}"]`)}`);
      await page.waitForTimeout(1_000);
      for (const [key, value] of Object.entries(JSON.parse(argsJson))) result.steps.push(`arg ${await sweep.fillStagedArgument(page, key, key === "value" ? `${value} 2` : value)}`);
      result.steps.push(`submit ${await sweep.submitStagedVerb(page, verb)}`);
      await page.waitForTimeout(3_000);
      result.readings.push(await read(`after a second ${verb}`));
    }
    result.selectionAfterNeutral = await page.evaluate(() => [...document.querySelectorAll('[aria-selected="true"], [data-selected="true"]')].length);
  }
  consoleLines.push(`${new Date().toISOString().slice(11, 23)} ---- rail undo`);
  result.rails.push(await railRows("undo before press", "action.undo"));
  result.rails.push(await railRows("verb row before undo", `action.${verb}`));
  result.activeBeforeUndo = await page.evaluate(() => ({ active: document.activeElement?.id ?? document.activeElement?.tagName, undoRows: [...document.querySelectorAll('[id="action.undo"]')].map((element) => element.closest("[data-window-id]")?.getAttribute("data-window-id") ?? element.closest("[id]")?.id ?? "?") }));
  result.steps.push(`undo ${await sweep.clickUncovered(page, '[data-slot="window-action-pane"] [id="action.undo"]')}`);
  await page.waitForTimeout(2_500);
  result.readings.push(await read("after ONE undo press, no neutral dispatch"));
  result.selectionAfterFirstUndo = await page.evaluate(() => [...document.querySelectorAll('[aria-selected="true"], [data-selected="true"]')].length);
  await page.waitForTimeout(6_000);
  result.readings.push(await read("after undo + 6 s quiet"));
  for (let press = 2; press <= Number(process.env.S15_UNDO_PRESSES ?? "1"); press += 1) {
    result.steps.push(`undo#${press} ${await sweep.clickUncovered(page, '[data-slot="window-action-pane"] [id="action.undo"]')}`);
    await page.waitForTimeout(2_500);
    result.readings.push(await read(`after undo press #${press}`));
  }
  if (process.env.S15_NEUTRAL_FIRST === "1") {
    consoleLines.push(`${new Date().toISOString().slice(11, 23)} ---- chord undo`);
    await page.keyboard.press(process.platform === "darwin" ? "Meta+KeyZ" : "Control+KeyZ");
    await page.waitForTimeout(2_500);
    result.readings.push(await read("after the shell undo CHORD"));
    result.selection = await page.evaluate(() => [...document.querySelectorAll('[aria-selected="true"], [data-selected="true"]')].length);
  }
  result.steps.push(`neutral ${await sweep.neutralDispatch(page)}`);
  result.readings.push(await read("after one neutral dispatch"));
  if (process.env.S15_TAIL_WAIT_MS) {
    await page.waitForTimeout(Number(process.env.S15_TAIL_WAIT_MS));
    result.readings.push(await read(`after ${process.env.S15_TAIL_WAIT_MS} ms more`));
  }
} catch (error) {
  result.fatal = String(error).slice(0, 400);
} finally {
  result.console = consoleLines.slice(-600);
  writeFileSync(out, JSON.stringify(result, null, 1));
  await browser.close();
}
for (const reading of result.readings) console.log(`[${reading.label}] checkin=${reading.checkin} undo=${JSON.stringify(reading.undo)} history=${JSON.stringify(reading.history.map((row) => `${row.text}${row.revert ? " [revert]" : ""}`))}`);
console.log(result.steps.join(" | "), result.fatal ?? "", "→", out);
