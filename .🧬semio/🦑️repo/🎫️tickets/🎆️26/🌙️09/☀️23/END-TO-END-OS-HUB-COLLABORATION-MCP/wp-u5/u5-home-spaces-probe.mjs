#!/usr/bin/env bun
/** 🏠️ U5 — Home's own space table after sign-in vs the hub workspace overlay's list (S3's 09-24 discrepancy), plus the
 * hub badge through sign-in. Usage: bun u5-home-spaces-probe.mjs <baseUrl> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6580/";
const tag = process.argv[3] ?? "home-spaces";
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const page = await context.newPage();
const requests = [];
context.on("response", (response) => {
  const url = response.url();
  if (response.status() >= 400 || /directory|spaces|auth\/sessions|_semio/iu.test(url)) requests.push(`${new Date().toISOString().slice(11, 23)} ${response.status()} ${response.request().method()} ${url}`.slice(0, 300));
});
const lines = [];
page.on("console", (m) => lines.push(`${new Date().toISOString().slice(11, 23)} ${m.type()}: ${m.text()}`.slice(0, 500)));
page.on("pageerror", (e) => lines.push(`pageerror: ${String(e)}`.slice(0, 500)));
const read = () => page.evaluate(() => ({
  hub: document.querySelector('[role="status"][data-semio-hub-connection]')?.getAttribute("data-semio-hub-connection") ?? null,
  signInOffered: document.querySelectorAll('[data-semio-hub-sign-in=""]').length,
  spaceRows: [...document.querySelectorAll('[data-row-id^="space:"]')].map((el) => el.getAttribute("data-row-id")),
  allRows: [...document.querySelectorAll('[data-window-id="s-home-main"] [data-row-id]')].map((el) => el.getAttribute("data-row-id")).slice(0, 20),
  homeHtml: (document.querySelector('[data-window-id="s-home-main"]')?.innerHTML ?? "").length,
  homeText: (document.querySelector('[data-window-id="s-home-main"]')?.innerText ?? "").replace(/\s+/gu, " ").slice(0, 300),
  overlay: document.querySelectorAll("[data-semio-hub-workspace]").length,
  overlaySpaces: [...document.querySelectorAll('[data-semio-hub-workspace] button[aria-label^="Open "]')].map((b) => b.getAttribute("aria-label")),
  bootstrap: [...document.querySelectorAll("[data-semio-directory-bootstrap], [data-directory-bootstrap]")].map((el) => el.outerHTML.slice(0, 200)),
}));
const timeline = [];
const notices = [];
await page.exposeFunction("__u5Notice", (text) => notices.push(`${new Date().toISOString().slice(11, 23)} ${text}`));
await page.addInitScript(() => {
  const seen = new Set();
  const tick = () => {
    for (const el of document.querySelectorAll("[data-directory-bootstrap], [data-semio-window-fault]")) {
      const key = `${el.getAttribute("data-directory-bootstrap") ?? ""}|${el.getAttribute("data-directory-bootstrap-code") ?? ""}|${el.getAttribute("data-semio-window-fault") ?? ""}|${(el.textContent ?? "").slice(0, 120)}`;
      if (!seen.has(key)) { seen.add(key); window.__u5Notice?.(key); }
    }
  };
  setInterval(tick, 100);
});
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined || document.documentElement.dataset.semioOsError !== undefined, undefined, { timeout: 300_000 });
await page.waitForTimeout(3_000);
timeline.push({ at: "signed-out", ...(await read()) });
await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true });
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 60_000 });
for (const skip of await page.getByRole("button", { name: /^(Skip|Überspringen)$/u }).all()) await skip.click({ force: true }).catch(() => undefined);
await form.locator('input[type="email"]').fill("ada@example.org");
await form.locator('input[type="password"]').fill("correct horse battery staple");
await form.locator('[id="os.hub.signIn.submit"]').click({ force: true });
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
await page.waitForTimeout(5_000);
timeline.push({ at: "signed-in overlay open", ...(await read()) });
await page.screenshot({ path: out(`u5-${tag}-overlay.png`) });
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
for (const wait of [2_000, 8_000, 20_000]) {
  await page.waitForTimeout(wait);
  timeline.push({ at: `overlay closed +${wait}ms`, ...(await read()) });
}
await page.screenshot({ path: out(`u5-${tag}-home.png`) });
await page.reload({ waitUntil: "commit" });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 }).catch(() => undefined);
await page.waitForTimeout(15_000);
timeline.push({ at: "after reload +15s", ...(await read()) });
await page.screenshot({ path: out(`u5-${tag}-reload.png`) });
writeFileSync(out(`u5-${tag}.json`), JSON.stringify({ baseUrl, timeline, notices, requests: requests.slice(-200), console: lines.filter((l) => /directory|space|hub|identity|error|warn/iu.test(l)).slice(-150) }, null, 1));
for (const row of timeline) console.log(JSON.stringify({ at: row.at, hub: row.hub, signIn: row.signInOffered, rows: row.spaceRows.length, allRows: row.allRows, overlay: row.overlay, overlaySpaces: row.overlaySpaces.length, home: row.homeText.slice(0, 90) }));
for (const notice of notices) console.log("notice", notice);
await browser.close();
