#!/usr/bin/env bun
/** ♿️ S15 — the UX bar of the `s` shell as a user meets it: settings persistence, en/de chrome completeness,
 * keyboard-only Home → palette → program, and the phone (375×812) / tablet (768×1024) layouts.
 *
 * Usage: bun s15-ux.mjs <baseUrl> [persist|locale|keyboard|devices ...]  → generated/s16-ux-<section>.json (+ png)
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, windowIds } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6540/";
const sections = process.argv.slice(3).length > 0 ? process.argv.slice(3) : ["persist", "locale", "keyboard", "devices"];
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const log = (...parts) => console.log("[s15-ux]", ...parts);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });

async function bootPage(context) {
  const page = await context.newPage();
  const faults = [];
  page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 240)));
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  await page.waitForTimeout(2_500);
  return { page, beacon, faults };
}

async function openSettings(page) {
  const open = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((el) => el instanceof HTMLElement && el.offsetParent !== null && /settings/iu.test(el.id)));
  if (!open) await page.locator('[id="framework.settings"]').first().click({ timeout: 10_000 });
  await page.waitForTimeout(1_500);
  await page.locator('[id="framework.settings.general"]').first().click({ timeout: 10_000 }).catch(() => undefined);
  await page.waitForTimeout(1_000);
}

async function chooseSetting(page, id, optionPattern) {
  await page.locator(`button[id="${id}"][role="combobox"]`).first().click({ timeout: 10_000 });
  await page.waitForTimeout(700);
  const options = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((el) => (el.textContent ?? "").trim()));
  const option = page.locator('[role="option"]').filter({ hasText: optionPattern }).first();
  const picked = (await option.count()) > 0 ? await option.click({ timeout: 10_000 }).then(() => "ok").catch((error) => String(error).slice(0, 80)) : "absent";
  await page.waitForTimeout(2_000);
  return { options, picked, shown: await page.locator(`button[id="${id}"][role="combobox"]`).first().innerText().catch(() => null) };
}

/** 🌓️ Appearance + layout chosen in Settings survive a reload of the same browser profile. */
async function persist() {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const { page, beacon, faults } = await bootPage(context);
  await openSettings(page);
  const appearance = await chooseSetting(page, "framework.settings.appearance", /^(Dark|Dunkel)/u);
  const darkBefore = await page.evaluate(() => [...document.querySelectorAll(".semio-scope, html")].some((el) => el.classList.contains("dark")));
  const stored = await page.evaluate(() => localStorage.getItem("semio.os.config")?.slice(0, 400) ?? null);
  await page.reload({ waitUntil: "commit" });
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  await page.waitForTimeout(3_000);
  const darkAfter = await page.evaluate(() => [...document.querySelectorAll(".semio-scope, html")].some((el) => el.classList.contains("dark")));
  await openSettings(page);
  const shownAfter = await page.locator('button[id="framework.settings.appearance"][role="combobox"]').first().innerText().catch(() => null);
  await page.screenshot({ path: out("s16-ux-persist-dark.png") });
  const restore = await chooseSetting(page, "framework.settings.appearance", /^(System)/u);
  await context.close();
  return { beacon, appearance, darkBefore, stored, darkAfter, shownAfter, restore, faults };
}

/** 🗣️ Every visible chrome string in en and de; a word left identical in both is an untranslated candidate. */
async function locale() {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const { page, beacon, faults } = await bootPage(context);
  const chrome = () =>
    page.evaluate(() => {
      const roots = [...document.querySelectorAll('[data-slot="navbar"], [data-slot="footer"], [id="s-home-main"], [data-slot="panel"]')];
      const texts = new Set();
      for (const root of roots) {
        if (!(root instanceof HTMLElement) || root.offsetParent === null) continue;
        for (const el of root.querySelectorAll("*")) {
          if (el.children.length > 0) continue;
          const text = (el.textContent ?? "").replace(/\s+/gu, " ").trim();
          if (text.length >= 3 && text.length <= 60) texts.add(text);
        }
        for (const el of root.querySelectorAll("[aria-label], [title], [placeholder]")) for (const attr of ["aria-label", "title", "placeholder"]) {
          const value = el.getAttribute(attr);
          if (value && value.length >= 3 && value.length <= 60) texts.add(`@${attr}:${value}`);
        }
      }
      return { lang: document.documentElement.lang, texts: [...texts].sort() };
    });
  await openSettings(page);
  const en = await chrome();
  const language = await chooseSetting(page, "framework.settings.language", /Deutsch/u);
  await page.waitForTimeout(3_000);
  const de = await chrome();
  await page.screenshot({ path: out("s16-ux-locale-de.png") });
  const deSet = new Set(de.texts);
  const identical = en.texts.filter((text) => deSet.has(text) && /[A-Za-z]{4,}/u.test(text));
  await context.close();
  return { beacon, language, enLang: en.lang, deLang: de.lang, enCount: en.texts.length, deCount: de.texts.length, identical, faults };
}

/** ⌨️ Home → palette → program with the keyboard alone, and the Tab order with its focus indicator. */
async function keyboard() {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const { page, beacon, faults } = await bootPage(context);
  await page.evaluate(() => document.body.focus());
  const walk = [];
  for (let step = 0; step < 30; step += 1) {
    await page.keyboard.press("Tab");
    await page.waitForTimeout(120);
    walk.push(
      await page.evaluate(() => {
        const el = document.activeElement;
        if (!(el instanceof HTMLElement) || el === document.body) return { tag: "BODY" };
        const style = getComputedStyle(el);
        const ring = (style.outlineStyle !== "none" && style.outlineWidth !== "0px") || style.boxShadow !== "none";
        return { tag: el.tagName, id: el.id || null, label: (el.getAttribute("aria-label") ?? el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40), visibleFocus: ring, focusVisible: el.matches(":focus-visible") };
      }),
    );
  }
  await page.evaluate(() => (document.activeElement instanceof HTMLElement ? document.activeElement.blur() : undefined));
  await page.evaluate(() => document.body.focus());
  const before = await windowIds(page);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  await page.waitForTimeout(1_200);
  const paletteOpen = (await page.locator("[role='dialog'] [data-slot='command-input']").count()) > 0;
  const paletteFocused = await page.evaluate(() => document.activeElement?.getAttribute("data-slot") ?? document.activeElement?.tagName ?? null);
  await page.keyboard.type("dag");
  await page.waitForTimeout(1_500);
  const selectedBefore = await page.evaluate(() => document.querySelector('[data-slot="command-item"][data-selected="true"], [data-slot="command-item"][aria-selected="true"]')?.getAttribute("data-command-item-id") ?? null);
  await page.keyboard.press("Enter");
  let opened = [];
  for (let wait = 0; wait < 60 && opened.length === 0; wait += 1) {
    await page.waitForTimeout(500);
    opened = (await windowIds(page)).filter((id) => !before.includes(id));
  }
  await page.waitForTimeout(3_000);
  const focusAfterOpen = await page.evaluate(() => {
    const active = document.activeElement;
    return { tag: active?.tagName ?? null, window: active?.closest('[data-slot="window"]')?.id ?? active?.closest("[data-window-id]")?.getAttribute("data-window-id") ?? null, id: active?.id ?? null, slot: active?.getAttribute("data-slot") ?? null, role: active?.getAttribute("role") ?? null, body: active === document.body, html: (active?.outerHTML ?? "").slice(0, 200) };
  });
  await page.keyboard.press("Tab");
  await page.waitForTimeout(300);
  const nextFocus = await page.evaluate(() => ({ tag: document.activeElement?.tagName ?? null, id: document.activeElement?.id ?? null, window: document.activeElement?.closest('[data-slot="window"]')?.id ?? null }));
  await page.screenshot({ path: out("s16-ux-keyboard-dag.png") });
  await context.close();
  const focusable = walk.filter((entry) => entry.tag !== "BODY");
  return { beacon, tabStops: focusable.length, withoutVisibleFocus: focusable.filter((entry) => !entry.visibleFocus).map((entry) => `${entry.tag}#${entry.id ?? ""} ${entry.label}`), walk, paletteOpen, paletteFocused, selectedBefore, opened, focusAfterOpen, nextFocus, faults };
}

/** 📱️ Phone and tablet viewports: no horizontal scroll, Home usable, a program opens and fits. */
async function devices() {
  const rows = [];
  for (const [name, viewport] of [["phone", { width: 375, height: 812 }], ["tablet", { width: 768, height: 1024 }]]) {
    const context = await browser.newContext({ viewport, hasTouch: true, isMobile: name === "phone", deviceScaleFactor: 2 });
    const { page, beacon, faults } = await bootPage(context);
    const layout = () =>
      page.evaluate(() => ({
        scrollWidth: document.documentElement.scrollWidth,
        innerWidth: window.innerWidth,
        windows: [...document.querySelectorAll('[data-slot="window"]')].map((el) => {
          const box = el.getBoundingClientRect();
          return { id: el.id, x: Math.round(box.left), w: Math.round(box.width), h: Math.round(box.height) };
        }),
        mobileTabs: [...document.querySelectorAll('[data-slot*="mobile"]')].map((el) => el.getAttribute("data-slot")).slice(0, 10),
        navbarOverflow: (() => {
          const nav = document.querySelector('[data-slot="navbar"]');
          return nav ? nav.scrollWidth - nav.clientWidth : null;
        })(),
      }));
    const home = await layout();
    await page.screenshot({ path: out(`s16-ux-${name}-home.png`) });
    const before = await windowIds(page);
    await page.evaluate(() => document.body.focus());
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    await page.waitForTimeout(1_200);
    let paletteVia = "chord";
    if ((await page.locator("[role='dialog'] [data-slot='command-input']").count()) === 0) {
      paletteVia = "footer";
      await page.locator('button:has-text("Command"), [id*="command" i][role="button"]').first().tap().catch(() => undefined);
      await page.waitForTimeout(1_200);
    }
    const paletteOpen = (await page.locator("[role='dialog'] [data-slot='command-input']").count()) > 0;
    let opened = [];
    if (paletteOpen) {
      await page.locator("[role='dialog'] [data-slot='command-input']").first().fill("note");
      await page.waitForTimeout(1_200);
      await page.locator('[data-slot="command-item"][data-command-item-id="spawn.note"]').first().tap().catch(() => page.keyboard.press("Enter"));
      for (let wait = 0; wait < 60 && opened.length === 0; wait += 1) {
        await page.waitForTimeout(500);
        opened = (await windowIds(page)).filter((id) => !before.includes(id));
      }
      await page.waitForTimeout(4_000);
    }
    const program = await layout();
    await page.screenshot({ path: out(`s16-ux-${name}-program.png`) });
    rows.push({ name, viewport, beacon, home, paletteVia, paletteOpen, opened, program, horizontalScroll: Math.max(home.scrollWidth, program.scrollWidth) > viewport.width, faults });
    await context.close();
  }
  return rows;
}

const runners = { persist, locale, keyboard, devices };
for (const section of sections) {
  const started = Date.now();
  const result = await runners[section]().catch((error) => ({ fatal: String(error).slice(0, 400) }));
  writeFileSync(out(`s16-ux-${section}.json`), JSON.stringify({ section, ms: Date.now() - started, result }, null, 1));
  log(section, JSON.stringify(result).slice(0, 1200));
}
await browser.close();
