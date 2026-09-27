#!/usr/bin/env bun
/** 🎚️ S16 item 3: a SECOND device (fresh browser context) signs in as the same hub user and is watched for 45 s: every request
 * of the preference lane (`/directory/preference-page/v1`), the worker/shell console lines about it, and the applied appearance
 * and language every 3 s — does a preference recorded on device A reach device B, and when?
 * usage: bun s16-preference-device-b.mjs <serveUrl> [tag] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dismissIntroduction } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl = "http://127.0.0.1:6541/", tag = "b1"] = process.argv.slice(2);
const out = fileURLToPath(new URL(`./generated/s16-preference-device-${tag}.json`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
await context.addInitScript(() => {
  const label = (data) => {
    const bytes = data?.wire instanceof Uint8Array ? data.wire : data instanceof Uint8Array ? data : null;
    if (bytes === null) return null;
    const text = new TextDecoder("latin1").decode(bytes);
    const kind = /(preference-lane-[a-z]+|directory-bootstrap-[a-z]+|directory-event-page|identity[a-z-]*)/u.exec(text)?.[1];
    return kind ?? null;
  };
  const post = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (message, ...rest) {
    const kind = label(message);
    if (kind?.startsWith("preference")) console.log(`[s16-wire] out ${kind} ${new TextDecoder("latin1").decode(message.wire).replace(/[^\x20-\x7e]+/gu, " ").slice(0, 200)}`);
    return post.call(this, message, ...rest);
  };
  const Native = Worker;
  globalThis.Worker = function (...args) {
    const worker = new Native(...args);
    worker.addEventListener("message", (event) => {
      const kind = label(event.data);
      if (kind?.startsWith("preference")) console.log(`[s16-wire] in ${kind} ${new TextDecoder("latin1").decode(event.data.wire ?? event.data).replace(/[^\x20-\x7e]+/gu, " ").slice(0, 300)}`);
    });
    return worker;
  };
  globalThis.Worker.prototype = Native.prototype;
});
const page = await context.newPage();
const t0 = Date.now();
const report = { requests: [], console: [], samples: [] };
page.on("response", (response) => {
  const url = decodeURIComponent(response.url());
  if (/preference|\/directory\/(event-page|commands|socket|events)/u.test(url)) report.requests.push(`${Date.now() - t0} ${response.status()} ${response.request().method()} ${url.replace(/^https?:\/\/[^/]+/u, "").slice(0, 140)}`);
});
page.on("console", (message) => {
  const text = message.text();
  if (/prefer|ui-preferences|lane|s16-wire/iu.test(text)) report.console.push(`${Date.now() - t0} ${message.type()} ${text.slice(0, 300)}`);
});
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true });
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 60_000 });
await form.locator('input[type="email"]').fill(process.env.S6_SIGN_IN_EMAIL ?? "user1@semio.dev");
await form.locator('input[type="password"]').fill(process.env.S6_SIGN_IN_PASSWORD ?? "gm1-local-dev-pass-1");
await form.locator('[id="os.hub.signIn.submit"]').click({ force: true });
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
report.signedInMs = Date.now() - t0;
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
for (let sample = 0; sample < 15; sample += 1) {
  report.samples.push({ atMs: Date.now() - t0, ...(await page.evaluate(() => ({ lang: document.documentElement.lang, appearance: document.querySelector("[data-appearance], [data-ui-appearance]")?.getAttribute("data-appearance") ?? document.querySelector("[data-ui-appearance]")?.getAttribute("data-ui-appearance") ?? null, stored: (localStorage.getItem("semio.os.config") ?? "").slice(0, 400) }))) });
  await page.waitForTimeout(3_000);
}
writeFileSync(out, JSON.stringify(report, null, 1));
console.log(JSON.stringify({ signedInMs: report.signedInMs, requests: report.requests, console: report.console.slice(0, 12), first: report.samples[0], last: report.samples.at(-1) }, null, 1).slice(0, 4000));
await browser.close();
