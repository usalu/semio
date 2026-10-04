/** 🛰️ Ticket tool of work package C3b: drives a private release build of the architecture quiz site in headless Chromium
 * under its own Content-Security-Policy — built with a proctor origin nobody answers on, served by
 * `serve_site_build.ts` — and records which scripts the document fetches before and after the pets come, whether the
 * half of the glue that comes with the pets (`🐾️pets/🎪️stage`) arrives as a chunk of its own beside the render target and
 * the menagerie, whether the layer draws pets, whether "Play with the pets" shows in the settings beside the first visit
 * and hands a deed to a pet on stage, and every console message, page error and policy violation. Prints JSON.
 *
 * Usage (from the repository root, build served on 6196):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c3b_release_check.mjs" [--base http://127.0.0.1:6196/] [--out <dir>]
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const address = option("base", "http://127.0.0.1:6196/");
const out = resolve(option("out", ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/🗑️generated/c3b/release"));
mkdirSync(out, { recursive: true });

const report = { scripts: [], console: [], errors: [], violations: [] };
const failed = [];
const expect = (name, holds, detail) => {
  if (!holds) failed.push(`${name}: ${JSON.stringify(detail)}`);
};

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1280, height: 860 }, locale: "en-US" });
const page = await context.newPage();
page.on("request", (request) => {
  const url = new URL(request.url());
  if (url.pathname.endsWith(".js")) report.scripts.push(decodeURIComponent(url.pathname));
});
page.on("console", (message) => report.console.push(`${message.type()}: ${message.text()}`));
page.on("pageerror", (error) => report.errors.push(String(error)));
await page.exposeFunction("c3bViolation", (detail) => report.violations.push(detail));
await page.addInitScript(() => document.addEventListener("securitypolicyviolation", (event) => window.c3bViolation(`${event.violatedDirective} ${event.blockedURI}`)));

await page.goto(address, { waitUntil: "load" });
await page.waitForSelector(".pet-layer svg.pet", { state: "attached", timeout: 45000 });
report.entry = [...(await (await page.request.get(address)).text()).matchAll(/<(?:script|link)\b[^>]*\s(?:src|href)="(\/assets\/[^"]+\.js)"/gu)].map((tag) => decodeURIComponent(tag[1]));
report.onStage = await page.evaluate(() => [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => pet.getAttribute("data-pet")).sort());
const group = page.getByRole("group", { name: "Play with the pets" });
await group.waitFor({ state: "visible", timeout: 30000 });
report.players = await group.locator("[data-pets-player]").evaluateAll((rows) => rows.map((row) => `${row.querySelector("span")?.textContent}: ${[...row.querySelectorAll("button")].map((button) => button.textContent).join(" | ")}`));
const first = group.locator("[data-pets-player]").first();
const species = await first.getAttribute("data-pets-player");
await first.getByRole("button", { name: "Hello" }).click();
await page.waitForTimeout(600);
report.status = await group.getByRole("status").textContent();
report.activity = await page.evaluate((species) => document.querySelector(`.pet-layer svg.pet[data-pet="${species}"]`)?.getAttribute("data-pet-activity") ?? null, species);
await page.screenshot({ path: resolve(out, "release-first-visit.png") });
report.scripts = [...new Set(report.scripts)];

const lazy = report.scripts.filter((script) => !report.entry.includes(script));
expect("one entry script", report.entry.filter((script) => script.endsWith(".js")).length === 1, report.entry);
expect("three lazy pets chunks besides the shared one", lazy.length >= 3, lazy);
expect("pets on stage", report.onStage.length > 0, report.onStage);
expect("the group names the pets on stage", report.players.length === report.onStage.length, report.players);
expect("the status says what was asked", /says hello\.$/u.test(report.status ?? ""), report.status);
expect("no policy violation", report.violations.length === 0, report.violations);
expect("no page error", report.errors.length === 0, report.errors);
expect("no console message but the dead proctor's", report.console.every((line) => /127\.0\.0\.1:6195|ERR_CONNECTION_REFUSED|Failed to load resource/u.test(line)), report.console);
report.failed = failed;
writeFileSync(resolve(out, "report.json"), JSON.stringify(report, null, 2));
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
await browser.close();
process.exitCode = failed.length === 0 ? 0 : 1;
