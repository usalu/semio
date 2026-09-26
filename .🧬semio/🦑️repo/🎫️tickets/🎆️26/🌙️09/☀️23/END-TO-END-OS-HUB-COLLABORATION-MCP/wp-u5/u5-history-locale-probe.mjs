#!/usr/bin/env bun
/** 🕰️ U5 — mutation labels in the running `s` History panel, en AND de, per plugin.
 *
 * Per plugin: spawn it from Home, drive one real mutation (+ undo/redo) through the Actions rail with the sweep's own
 * verb map, read every History row label in English, switch the SHELL locale to German through Settings, read the
 * SAME rows again without dispatching anything, and switch back. A row whose German text equals its English text is
 * reported as `untranslated`; a row that changed proves the panel re-resolved the `LocalizedLabel` carrier at render.
 *
 * Usage: bun u5-history-locale-probe.mjs <baseUrl> <tag> <pluginId…> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { click, closeWindows, dismissIntroduction, mutateUndoRedo, spawnProgram } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl = "http://127.0.0.1:6580/", tag = "adhoc", ...plugins] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));

async function setLocale(page, wanted) {
  const opened = await click(page, '[id="os.openSettings"], [data-slot="navbar"] [id*="settings" i], [id="framework.settings"]');
  if (opened === "absent") return `${wanted}:no-settings-control`;
  await page.waitForTimeout(2_500);
  const language = page.locator('[role="tab"], [role="button"], button').filter({ hasText: /^(language|sprache)$/iu }).first();
  if ((await language.count()) > 0) {
    await language.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_500);
  }
  let control = page.locator('select, [role="combobox"]').filter({ hasText: /english|deutsch|german|englisch/iu }).first();
  if ((await control.count()) === 0) {
    await page.locator('[id="framework.settings.general"]').first().click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_500);
    control = page.locator('select, [role="combobox"]').filter({ hasText: /english|deutsch|german|englisch/iu }).first();
  }
  if ((await control.count()) === 0) return `${wanted}:no-language-control`;
  if ((await control.evaluate((element) => element.tagName.toLowerCase())) === "select") await control.selectOption(wanted).catch(() => undefined);
  else {
    await control.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(800);
    await page.locator('[role="option"]').filter({ hasText: wanted === "de" ? /deutsch|german/iu : /english|englisch/iu }).first().click({ force: true }).catch(() => undefined);
  }
  await page.waitForFunction((locale) => document.documentElement.lang === locale, wanted, { timeout: 15_000 }).catch(() => undefined);
  await page.keyboard.press("Escape").catch(() => undefined);
  await click(page, '[id="framework.settings"]');
  await page.waitForTimeout(1_500);
  return `${wanted}=${await page.evaluate(() => document.documentElement.lang || "unreported")}`;
}

async function historyRows(page) {
  if ((await page.locator('[id^="framework.history.entry."]').count()) === 0) {
    await click(page, '[id="framework.panel.history"]');
    await page.waitForTimeout(1_500);
  }
  return page.evaluate(() =>
    [...document.querySelectorAll('[id^="framework.history.entry."]')]
      .filter((element) => !element.id.endsWith(".revert"))
      .map((element) => ({ id: element.id, label: (element.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 160) })),
  );
}

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
page.on("console", (m) => lines.push(`${m.type()}: ${m.text()}`.slice(0, 300)));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(3_000);
const rows = [];
for (const pluginId of plugins) {
  const started = Date.now();
  const refusals = [];
  const spawned = await spawnProgram(page, pluginId);
  if (spawned.windowIds.length === 0) {
    rows.push({ pluginId, spawned, verdict: "not-spawned" });
    continue;
  }
  const mutation = await mutateUndoRedo(page, refusals, pluginId);
  const en = await historyRows(page);
  const seatedDe = await setLocale(page, "de");
  const de = await historyRows(page);
  const seatedEn = await setLocale(page, "en");
  const byId = new Map(de.map((row) => [row.id, row.label]));
  const pairs = en.map((row) => ({ id: row.id, en: row.label, de: byId.get(row.id) ?? null }));
  const plugin = pairs.filter((pair) => !/^(Set|Change|Switch|Open|Toggle) (panel|tab|locale|language|appearance|theme)/iu.test(pair.en));
  const untranslated = plugin.filter((pair) => pair.de !== null && pair.de === pair.en && /[a-z]{3}/iu.test(pair.en));
  rows.push({ pluginId, windows: spawned.windowIds, mutation: mutation.mutation, mutationDetail: mutation.mutationDetail, seated: [seatedDe, seatedEn], pairs, untranslated: untranslated.length, verdict: pairs.length === 0 ? "no-history-rows" : untranslated.length === 0 ? "en+de" : "partly-untranslated", ms: Date.now() - started });
  console.log(JSON.stringify({ pluginId, verdict: rows.at(-1).verdict, mutation: mutation.mutation, pairs: pairs.slice(-4) }).slice(0, 900));
  await closeWindows(page, spawned.windowIds);
}
await page.screenshot({ path: out(`u5-history-${tag}.png`) });
writeFileSync(out(`u5-history-${tag}.json`), JSON.stringify({ baseUrl, rows, faults: lines.filter((l) => /pageerror|Uncaught|trap|unreachable/iu.test(l)).slice(-40) }, null, 1));
await browser.close();
