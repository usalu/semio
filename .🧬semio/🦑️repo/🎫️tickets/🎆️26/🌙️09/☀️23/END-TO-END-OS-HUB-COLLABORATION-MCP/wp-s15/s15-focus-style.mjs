#!/usr/bin/env bun
/** 🔍️ S15 — focuses a control by keyboard and dumps its className + focus styles. Usage: bun s15-focus-style.mjs <baseUrl> <elementId> */
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";
const [baseUrl, id] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(2_000);
for (let i = 0; i < 40; i++) {
  await page.keyboard.press("Tab");
  if (await page.evaluate((wanted) => document.activeElement?.id === wanted, id)) break;
}
console.log(JSON.stringify(await page.evaluate(() => {
  const el = document.activeElement;
  const cs = getComputedStyle(el);
  const rules = [];
  for (const sheet of document.styleSheets) { try { for (const rule of sheet.cssRules) if (rule.selectorText && el.matches(rule.selectorText.replace(/::?[a-z-]+\([^)]*\)|::[a-z-]+/gu, "")) && /box-shadow|outline|ring/u.test(rule.cssText) && /focus/u.test(rule.selectorText)) rules.push(rule.cssText.slice(0, 200)); } catch {} }
  return { id: el.id, className: el.className, focusVisible: el.matches(":focus-visible"), outline: `${cs.outlineStyle} ${cs.outlineWidth} ${cs.outlineColor}`, boxShadow: cs.boxShadow, ringVar: cs.getPropertyValue("--tw-ring-shadow"), rules: rules.slice(0, 8) };
}), null, 1));
await browser.close();
