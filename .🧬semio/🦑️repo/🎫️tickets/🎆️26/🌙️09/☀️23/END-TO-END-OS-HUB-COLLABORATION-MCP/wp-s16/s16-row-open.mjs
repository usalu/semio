#!/usr/bin/env bun
/** 🧭️ S16 item 1 (S12-3g): a hard load of `/spaces/<id>` in a signed-in profile, then a Space-index ROW open of a named hub
 * document (its row's own open control, else Enter on the focused row), then a 250 ms trace of route + windows for
 * `S16_TRACE_MS` — a document replaced by the space index shows up as a second trace row.
 * usage: S15_PROFILE_DIR=<signed-in profile> bun s16-row-open.mjs <origin> <spaceId> <documentName substring> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [origin, spaceId, documentName, tag = "row-open"] = process.argv.slice(2);
const out = fileURLToPath(new URL(`./generated/s16-row-open-${tag}.json`, import.meta.url));
const log = (...parts) => console.log("[s16-row]", ...parts);
const context = await chromium.launchPersistentContext(process.env.S15_PROFILE_DIR, { headless: true, args: ["--use-angle=metal"], viewport: { width: 1600, height: 1000 }, locale: process.env.S15_LOCALE ?? "en-US" });
const page = context.pages()[0] ?? (await context.newPage());
const result = { origin, spaceId, documentName, faults: [], console: [] };
page.on("pageerror", (error) => result.faults.push(String(error).slice(0, 240)));
page.on("console", (message) => {
  const text = message.text();
  if (/route|space index|document closed|refused|stale|error/iu.test(text)) result.console.push(`${message.type()} ${text}`.slice(0, 300));
});
const windows = () => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).join(","));
const started = Date.now();
await page.goto(`${origin.replace(/\/$/u, "")}/spaces/${spaceId}`, { waitUntil: "commit", timeout: 180_000 });
const EMAIL = process.env.S6_SIGN_IN_EMAIL ?? "user1@semio.dev";
const PASSWORD = process.env.S6_SIGN_IN_PASSWORD ?? "gm1-local-dev-pass-1";
const admissionDeadline = Date.now() + 240_000;
while (Date.now() < admissionDeadline) {
  const admission = await page.evaluate(() => document.querySelector("[data-semio-route-admission]")?.getAttribute("data-semio-route-admission") ?? null);
  if (admission === "await-sign-in") {
    result.signIn = { admission, atMs: Date.now() - started };
    await page.locator("[data-semio-route-admission] button").first().click({ force: true });
    const form = page.locator("[data-semio-hub-workspace]");
    await form.waitFor({ state: "visible", timeout: 60_000 });
    await form.locator('input[type="email"]').fill(EMAIL);
    await form.locator('input[type="password"]').fill(PASSWORD);
    await form.locator('form:has(input[type="password"]) button[type="submit"]').first().click({ force: true });
    await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
    result.signIn.doneAtMs = Date.now() - started;
    await page.waitForTimeout(2_000);
    await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_000);
    result.signIn.uriAfter = await page.evaluate(() => location.pathname);
    log("signed in through the route admission", JSON.stringify(result.signIn));
    break;
  }
  if ((await page.locator('[data-slot="window-body"] [role="row"]').count()) > 1) break;
  await page.waitForTimeout(1_000);
}
const deadline = Date.now() + 240_000;
let rowLocator = null;
while (Date.now() < deadline) {
  const rows = page.locator('[data-slot="window-body"] [role="row"]').filter({ hasText: documentName });
  if ((await rows.count()) > 0) {
    rowLocator = rows.first();
    break;
  }
  await page.waitForTimeout(1_000);
}
result.rowsAtMs = Date.now() - started;
if (rowLocator === null) {
  result.verdict = "no row";
  result.stuck = await page.evaluate(() => ({ uri: location.pathname, admission: [...document.querySelectorAll("[data-semio-route-admission]")].map((element) => `${element.getAttribute("data-semio-route-admission")}|${(element.textContent ?? "").trim().slice(0, 120)}`), status: [...document.querySelectorAll('[role="status"], [role="alert"]')].map((element) => (element.textContent ?? "").trim().slice(0, 120)).filter(Boolean).slice(0, 8), hubBadge: document.querySelector("[data-semio-hub-connection], [data-semio-shell-sync]")?.textContent?.trim().slice(0, 80) ?? null }));
  log("no row named", documentName, "windows", await windows(), JSON.stringify(result.stuck));
} else {
  result.rowDom = await rowLocator.evaluate((row) => ({ role: row.getAttribute("role"), aria: row.getAttribute("aria-label"), buttons: [...row.querySelectorAll("button, [role=button]")].map((button) => `${button.id}|${button.getAttribute("aria-label") ?? ""}|${(button.textContent ?? "").trim().slice(0, 40)}`) }));
  log("row", JSON.stringify(result.rowDom));
  const before = await windows();
  result.windowsBefore = before;
  const open = rowLocator.locator('button[aria-label^="Open"], button[aria-label^="Öffnen"], [role=button][aria-label^="Open"], [role=button][aria-label^="Öffnen"]');
  if ((await open.count()) > 0) {
    await open.first().click();
    result.openedBy = "row open button";
  } else {
    await rowLocator.focus();
    await page.keyboard.press("Enter");
    result.openedBy = "Enter on the row";
  }
  const openDeadline = Date.now() + 180_000;
  while (Date.now() < openDeadline && (await windows()) === before) await page.waitForTimeout(250);
  result.openMs = Date.now() - started;
  result.trace = [];
  const traceStart = Date.now();
  while (Date.now() - traceStart < Number(process.env.S16_TRACE_MS ?? 20_000)) {
    const row = await page.evaluate(() => `${location.pathname} | ${[...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).join(",")}`);
    if (result.trace.at(-1)?.row !== row) result.trace.push({ atMs: Date.now() - traceStart, row });
    await page.waitForTimeout(250);
  }
  const opened = result.trace.at(-1)?.row ?? "";
  result.verdict = result.trace.length === 1 && !opened.includes("framework.window.table") ? "PASS (one state, document kept)" : `FAIL (${result.trace.length} states)`;
  log(`opened by ${result.openedBy} in ${result.openMs} ms; trace ${JSON.stringify(result.trace)}`);
}
log(result.verdict, "faults", result.faults.length);
writeFileSync(out, JSON.stringify(result, null, 1));
await context.close();
