#!/usr/bin/env bun
/** 🔍️ S12 — why `createArtifact`'s `kindChoice` chooser offers nothing on a mounted space index.
 * Dumps the staged row's own markup plus the shell's creation-catalog state, on a space chosen by
 * the text its hub-workspace row shows. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6072/";
const tag = process.argv[3] ?? "diag";
const spaceFilter = process.argv[4] ?? "note";
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const log = (...p) => console.log("[s12-diag]", ...p);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.setDefaultNavigationTimeout(180_000);
const lines = [];
page.on("console", (m) => { const t = m.text(); if (m.type() === "error" || m.type() === "warning" || /\[os-shell\]|catalog|creation|refused|rejected|dropped|scope|directory/iu.test(t)) lines.push(`${m.type()}| ${t.slice(0, 220)}`); });
const net = [];
page.on("requestfailed", (r) => net.push(`FAILED ${r.method()} ${r.url().slice(0, 140)} ${r.failure()?.errorText ?? ""}`));
page.on("response", (r) => { const u = r.url(); if (/artifact-creations|\/_semio\/hub\//u.test(u)) net.push(`${r.status()} ${r.request().method()} ${u.slice(0, 160)}`); });
const out = { baseUrl, spaceFilter, spaces: [], chosen: null, uri: null, railRowIds: [], stagedHtml: [], afterClickHtml: [], optionRoles: [], catalog: null, lines: [] };
try {
  await page.goto(baseUrl, { waitUntil: "commit" });
  const deadline = Date.now() + 300_000;
  while (Date.now() < deadline) { if (await page.evaluate(() => document.documentElement.dataset.semioOsReady !== undefined)) break; await page.waitForTimeout(1_000); }
  for (let i = 0; i < 60; i += 1) { if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) break; await page.keyboard.press("Escape").catch(() => undefined); await page.waitForTimeout(500); }
  await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true }).catch(() => undefined);
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  await form.locator('input[type="email"]').fill(process.env.S6_SIGN_IN_EMAIL ?? "user1@semio.dev");
  await form.locator('input[type="password"]').fill(process.env.S6_SIGN_IN_PASSWORD ?? "gm1-local-dev-pass-1");
  await form.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.waitForTimeout(6_000);
  out.spaces = await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] li[data-space-id]")].map((r) => ({ id: r.getAttribute("data-space-id"), text: (r.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 90) })));
  if (spaceFilter === "FRESH") {
    const before = out.spaces.map((s) => s.id);
    await page.locator('[data-element-alias="os.hub.spaces.createName"]').first().fill(`S12 Fresh ${Date.now() % 100000}`).catch(() => undefined);
    await page.locator('[id="os.hub.spaces.createSubmit"]').first().click({ force: true }).catch(() => undefined);
    const dl = Date.now() + 90_000;
    while (Date.now() < dl) {
      out.spaces = await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] li[data-space-id]")].map((r) => ({ id: r.getAttribute("data-space-id"), text: (r.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 90) })));
      if (out.spaces.some((s) => !before.includes(s.id))) break;
      await page.waitForTimeout(2_000);
    }
    out.freshCreated = out.spaces.filter((s) => !before.includes(s.id));
  }
  const chosen = spaceFilter === "FRESH" ? (out.freshCreated?.[0] ?? out.spaces[0]) : (out.spaces.find((s) => new RegExp(spaceFilter, "iu").test(s.text)) ?? out.spaces[0]);
  out.chosen = chosen;
  log(`chosen ${JSON.stringify(chosen)}`);
  await page.locator(`[data-semio-hub-workspace] li[data-space-id="${chosen.id}"] button`).first().click({ force: true }).catch(() => undefined);
  const d2 = Date.now() + 180_000;
  while (Date.now() < d2) { const u = await page.evaluate(() => window.location.pathname); if (u.includes(chosen.id)) break; await page.waitForTimeout(1_500); }
  await page.waitForTimeout(12_000);
  out.uri = await page.evaluate(() => window.location.pathname);
  const toggles = page.locator('[id$=".engagement.toggle"]');
  for (let i = 0; i < (await toggles.count()); i += 1) await toggles.nth(i).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(3_000);
  out.railRowIds = await page.evaluate(() => [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((e) => e.id))]);
  await page.locator('[data-slot="window-action-pane"] [id="action.createArtifact"]').first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(2_500);
  out.stagedHtml = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] [id*=".arg."]')].map((e) => e.outerHTML.replace(/\s+/gu, " ").replace(/ style="[^"]*"/gu, "").replace(/ class="[^"]*"/gu, "").slice(0, 3000)));
  const kind = page.locator('[data-slot="window-action-pane"] [id$=".arg.kindChoice"]').first();
  await kind.locator('button, [role="combobox"], select').first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(2_500);
  out.afterClickHtml = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] [id$=".arg.kindChoice"]')].map((e) => e.outerHTML.replace(/\s+/gu, " ").replace(/ style="[^"]*"/gu, "").replace(/ class="[^"]*"/gu, "").slice(0, 4000)));
  out.optionRoles = await page.evaluate(() => [...document.querySelectorAll('[role="option"], [data-slot="command-item"], [role="menuitem"], option')].map((e) => `${e.getAttribute("role") ?? e.tagName.toLowerCase()}|${(e.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 50)}`).slice(0, 40));
  out.catalog = await page.evaluate(() => {
    const el = document.querySelector("[data-semio-artifact-creation-catalog]");
    return el === null ? null : { attr: el.getAttribute("data-semio-artifact-creation-catalog"), text: (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 300) };
  });
  out.net = net.slice(0, 40);
  out.bodyText = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"]')].map((b) => (b.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 500)));
  out.catalogBody = await page.evaluate(async () => {
    try {
      const path = `/spaces/${window.location.pathname.split("/")[2]}/artifact-creations`;
      const response = await fetch(`/_semio/hub${path}`, { credentials: "include" });
      return { status: response.status, body: (await response.text()).slice(0, 600) };
    } catch (error) { return { status: -1, body: String(error).slice(0, 200) }; }
  });
  await page.screenshot({ path: `${generated}s12-kind-chooser-${tag}.png` }).catch(() => undefined);
} catch (e) { out.fatal = String(e).slice(0, 300); }
out.lines = lines.slice(0, 25);
writeFileSync(`${generated}s12-kind-chooser-${tag}.txt`, JSON.stringify(out, null, 2));
log(`=== ${generated}s12-kind-chooser-${tag}.txt ===`);
await browser.close();
