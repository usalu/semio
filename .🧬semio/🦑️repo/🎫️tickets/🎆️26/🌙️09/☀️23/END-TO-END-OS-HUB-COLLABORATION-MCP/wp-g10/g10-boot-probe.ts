#!/usr/bin/env bun
/** 🔎️ G10 probe: boots the shell once per locale and records the first boot beacon, the error text the
 * shell shows, and the console, so a transient boot error is named instead of guessed.
 * usage: bun g10-boot-probe.ts <shellUrl> <en|de> <capture> */
import { writeFileSync } from "node:fs";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const [URL = "http://127.0.0.1:6530/?plugin=note", LOCALE = "de", CAPTURE = "/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/generated/boot-probe.txt"] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: LOCALE === "de" ? "de-DE" : "en-US" })).newPage();
const lines: string[] = [];
const t0 = Date.now();
page.on("console", (message) => lines.push(`${Date.now() - t0}ms ${message.type()} ${message.text().slice(0, 600)}`));
page.on("pageerror", (error) => lines.push(`${Date.now() - t0}ms pageerror ${String(error).slice(0, 800)}`));
await page.goto(URL, { waitUntil: "domcontentloaded", timeout: 180_000 });
const beacons: string[] = [];
for (let tick = 0; tick < 240; tick += 1) {
  const view = await page.evaluate(() => ({ ready: document.documentElement.getAttribute("data-semio-os-ready"), error: document.documentElement.getAttribute("data-semio-os-error"), text: (document.querySelector("[role='alert']") as HTMLElement | null)?.innerText?.slice(0, 400) ?? "" }));
  const line = `ready=${view.ready} error=${view.error} alert=${view.text.replace(/\s+/gu, " ")}`;
  if (beacons.at(-1) !== line) beacons.push(`${Date.now() - t0}ms ${line}`);
  if (view.ready && tick > 10) break;
  await page.waitForTimeout(500);
}
writeFileSync(CAPTURE, [`# ${URL} ${LOCALE}`, ...beacons, "# console", ...lines.filter((line) => !/status of 404|DevTools/u.test(line)).slice(-200)].join("\n"));
console.log(beacons.join("\n"));
await browser.close();
