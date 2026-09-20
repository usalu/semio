/** 🔍️ Why `flowEvalTick` reaches the guest through the ACTION channel in the `flow` playground but
 * not in `generation2d`: reads what the host knows about the live session at the moment of the re-arm.
 *
 * `makeEffectDispatchOne` (`🛠️ShellHelpers/🟦️.tsx:886`) routes to `handleCommand` only when
 * `baseSession.app.commands` names the action AND the handle exposes `handleCommand`. The guest then
 * refuses anything else. This prints, per variant: the shell's ready/app attributes, whatever the
 * plugin bridge module publishes for the editor app's `commands`, and the first refusal line.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { fileURLToPath } from "node:url";
import { mkdirSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";

const TICKET = dirname(fileURLToPath(new URL(import.meta.url)));
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });

const variant = process.env.B3C_VARIANT ?? "flow";
const port = Number(process.env.B3C_PORT ?? 6016);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const lines = [];
page.on("console", (m) => lines.push(`${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`pageerror ${String(e).slice(0, 600)}`));

await page.goto(`http://127.0.0.1:${port}/?plugin=${variant}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 60; i++) {
  await page.waitForTimeout(1000);
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready") !== null)) break;
}
await page.waitForTimeout(8000);

const facts = await page.evaluate(async () => {
  const session = await fetch("/🎮️playground-session.js").then((r) => (r.ok ? r.text() : null)).catch(() => null);
  return {
    datasets: { ...document.documentElement.dataset },
    windowIds: [...document.querySelectorAll('[data-slot="window"]')].map((el) => el.id),
    sessionModuleHead: session === null ? null : session.slice(0, 400),
    globals: Object.keys(globalThis).filter((k) => /semio|shell|plugin/i.test(k)).slice(0, 40),
  };
});

const refusals = lines.filter((l) => /not a framework-reserved action/.test(l));
const out = { variant, port, facts, refusalCount: refusals.length, firstRefusal: refusals[0] ?? null, console: lines.slice(0, 400) };
writeFileSync(join(OUT, `b3c-${variant}-diagnose.txt`), JSON.stringify(out, null, 2));
console.log(JSON.stringify({ variant, ready: facts.datasets.semioOsReady, windowIds: facts.windowIds, globals: facts.globals, refusalCount: refusals.length }, null, 1));
await browser.close();
