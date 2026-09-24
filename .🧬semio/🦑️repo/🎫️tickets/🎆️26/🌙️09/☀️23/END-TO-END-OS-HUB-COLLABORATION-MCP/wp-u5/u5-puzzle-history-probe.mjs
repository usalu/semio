#!/usr/bin/env bun
/** 🧩️ U5 — puzzle's History rows en ↔ de: spawn puzzle from Home, drive one real mutation through the sweep's verb map,
 * read every History row, switch the shell locale in Settings (screenshotting what Settings shows while puzzle owns the
 * canvas) and read the SAME rows again. Usage: bun u5-puzzle-history-probe.mjs <baseUrl> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { click, dismissIntroduction, mutateUndoRedo, spawnProgram } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl = "http://127.0.0.1:6580/", tag = "puzzle"] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const rows = async () => {
  if ((await page.locator('[id^="framework.history.entry."]').count()) === 0) {
    await click(page, '[id="framework.panel.history"]');
    await page.waitForTimeout(1_500);
  }
  return page.evaluate(() => [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).map((element) => ({ id: element.id, label: (element.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 120) })));
};
const settingsDump = () => page.evaluate(() => ({
  lang: document.documentElement.lang,
  tabs: [...document.querySelectorAll('[role="tab"], [role="button"], button')].map((element) => (element.textContent ?? "").trim()).filter((text) => /language|sprache|settings|einstellungen/iu.test(text)).slice(0, 12),
  selects: [...document.querySelectorAll('select, [role="combobox"]')].map((element) => `${element.id}|${(element.textContent ?? "").trim().slice(0, 60)}`).slice(0, 12),
}));
async function setLocale(wanted, step) {
  await click(page, '[id="os.openSettings"], [data-slot="navbar"] [id*="settings" i], [id="framework.settings"]');
  await page.waitForTimeout(2_000);
  const before = await settingsDump();
  await page.screenshot({ path: out(`u5-${tag}-${step}-settings.png`) });
  const language = page.locator('[role="tab"], [role="button"], button').filter({ hasText: /^(language|sprache)$/iu }).first();
  if ((await language.count()) > 0) await language.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_000);
  const control = page.locator('select, [role="combobox"]').filter({ hasText: /english|deutsch|german|englisch/iu }).first();
  const found = (await control.count()) > 0;
  if (found) {
    if ((await control.evaluate((element) => element.tagName.toLowerCase())) === "select") await control.selectOption(wanted).catch(() => undefined);
    else {
      await control.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(600);
      await page.locator('[role="option"]').filter({ hasText: wanted === "de" ? /deutsch|german/iu : /english|englisch/iu }).first().click({ force: true }).catch(() => undefined);
    }
    await page.waitForFunction((locale) => document.documentElement.lang === locale, wanted, { timeout: 15_000 }).catch(() => undefined);
  }
  await page.keyboard.press("Escape").catch(() => undefined);
  await click(page, '[id="framework.settings"]');
  await page.waitForTimeout(1_500);
  return { wanted, found, before, lang: await page.evaluate(() => document.documentElement.lang) };
}
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(3_000);
const spawned = await spawnProgram(page, "puzzle");
const refusals = [];
const mutation = await mutateUndoRedo(page, refusals, "puzzle");
const en = await rows();
await page.screenshot({ path: out(`u5-${tag}-en.png`) });
const toDe = await setLocale("de", "to-de");
const de = await rows();
await page.screenshot({ path: out(`u5-${tag}-de.png`) });
const toEn = await setLocale("en", "to-en");
const byId = new Map(de.map((row) => [row.id, row.label]));
const pairs = en.map((row) => ({ id: row.id, en: row.label, de: byId.get(row.id) ?? null }));
writeFileSync(out(`u5-${tag}.json`), JSON.stringify({ baseUrl, spawned, mutation, refusals, toDe, toEn, pairs }, null, 1));
console.log(JSON.stringify({ spawned: spawned.windowIds, mutation: mutation.mutation, detail: mutation.mutationDetail, toDe, toEn, pairs }, null, 1).slice(0, 4000));
await browser.close();
