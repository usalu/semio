/** 🧪️ C8 — live proof that a gis2d `commitCheckpoint` (manual and the shell's auto check-in) no longer traps the guest.
 * Usage: bun c8-checkpoint-live.mjs <shellUrl> <tag> [query] */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync } from "node:fs";
const SHELL = process.argv[2] ?? "http://127.0.0.1:6380";
const TAG = process.argv[3] ?? "c8b";
const QUERY = process.argv[4] ?? "?plugin=gis2d";
const OUT = "/Users/ueli/Documents/semio/.tmp-ticket/wp-c8/generated";
const t0 = Date.now();
const ms = () => Date.now() - t0;
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${ms()} ${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`${ms()} pageerror ${String(e.stack ?? e).slice(0, 1500)}`));
const save = () => writeFileSync(`${OUT}/${TAG}-console.txt`, lines.join("\n"));
const state = () => page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    checkin: text(document.querySelector("#s-checkin")),
    ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => text(el).slice(0, 80)),
    actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))].length,
  };
});
const click = async (sel) => (await page.locator(sel).count()) ? page.locator(sel).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 100)) : "absent";
const step = async (name, fn) => { const r = await fn(); console.log(`${name}: ${JSON.stringify(r)}`); save(); return r; };
await page.goto(`${SHELL}/${QUERY}`, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let i = 0; i < 200; i++) { await page.waitForTimeout(1000); const s = await state(); if (s.error || (s.ready && i > 6)) break; }
for (const toggle of await page.locator('[id$=".engagement.toggle"]').all()) { await toggle.click({ timeout: 6000, force: true }).catch(() => {}); await page.waitForTimeout(700); }
await page.waitForTimeout(3000);
await step("boot", state);
await page.screenshot({ path: `${OUT}/${TAG}-boot.png` });
const addFeature = async () => {
  const opened = await click('[id="action.addFeature"]');
  await page.waitForTimeout(1200);
  const executed = await click('[id$=".action.addFeature.execute"]');
  await page.waitForTimeout(5000);
  return { opened, executed, ...(await state()) };
};
await step("edit-1", addFeature);
await step("checkpoint", async () => {
  const opened = await click('[id="action.commitCheckpoint"]');
  await page.waitForTimeout(1500);
  const executed = await click('[id$=".action.commitCheckpoint.execute"]');
  await page.waitForTimeout(8000);
  return { opened, executed, ...(await state()) };
});
await step("auto-checkin-wait", async () => { await page.waitForTimeout(20_000); return state(); });
await step("edit-2-after-checkpoint", addFeature);
await step("edit-3", addFeature);
await page.keyboard.press("Escape").catch(() => {});
await step("history", async () => {
  const opened = await click('button:has-text("History")');
  await page.waitForTimeout(2500);
  const rows = await page.evaluate(() => [...document.querySelectorAll('[id^="framework.history"]')].map((el) => `${el.id} | ${(el.innerText ?? "").replace(/\s+/g, " ").slice(0, 80)}`).slice(0, 40));
  return { opened, rows };
});
await page.screenshot({ path: `${OUT}/${TAG}-end.png` });
const faults = lines.filter((l) => /panicked|resolve_ready|unreachable|dispatch-failed|trap/i.test(l));
console.log(`faults: ${faults.length}`);
for (const f of faults.slice(0, 10)) console.log(`  ${f.slice(0, 300)}`);
save();
await browser.close();
