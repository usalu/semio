import { mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";
import type { Page } from "playwright";

/** 🔭️ Control explorer: opens the sourcing editor at desktop size and, for every control the journey does not drive (row More, Actions, Restock, Editor/Panel chrome, window tabs), records the accessible controls and visible text that each click adds, plus a screenshot.
 * Usage: `bun explore-controls.ts [en|de]` with the native sourcing host on 6081; writes `🗑️generated/explore/`.
 * @see ./sourcing-journey.ts */

const root = process.cwd(), osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const out = join(import.meta.dir, "🗑️generated", "explore"), language = (process.argv[2] ?? "en") as "en" | "de";
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, dismissIntroduction } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
await mkdir(out, { recursive: true });

const controls = (page: Page) => page.evaluate(() => [...document.querySelectorAll("button,[role=button],[role=menuitem],[role=option],[role=tab],input,select,[role=combobox],[role=switch],[role=checkbox],a")].filter(element => (element as HTMLElement).offsetParent !== null).map(element => `${element.getAttribute("role") ?? element.tagName.toLowerCase()}|${element.id}|${(element.getAttribute("aria-label") ?? (element as HTMLElement).innerText ?? "").trim().slice(0, 60)}`));
const browser = await chromium.launch();
const context = await browser.newContext({ locale: language === "en" ? "en-US" : "de-DE", viewport: { width: 1800, height: 1100 } });
const page = await context.newPage();
const faults: string[] = [];
page.on("pageerror", error => faults.push(`pageerror: ${error.message}`.slice(0, 300)));
page.on("console", message => { if (message.type() === "error") faults.push(message.text().slice(0, 300)); });
await page.goto(process.env.SOURCING_URL ?? "http://127.0.0.1:6081/");
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(3000);
const report: Record<string, unknown> = { baseline: await controls(page) };
await page.screenshot({ path: join(out, `${language}-0-baseline.png`) });
const probes: [string, string][] = JSON.parse(process.env.PROBES ?? "[]");
const defaults: [string, string][] = [["row-more", "(//button[@aria-label='More' or normalize-space()='⋯'])[1]"], ["actions", "//*[normalize-space()='Actions' or normalize-space()='Aktionen']"], ["restock", "//*[@role='button' or self::button][contains(normalize-space(),'Restock') or contains(normalize-space(),'Nachfüllen') or contains(normalize-space(),'Katalog')]"], ["editor-tab", "//*[normalize-space()='Editor']"], ["panel", "//*[normalize-space()='Panel']"], ["chrome-more", "(//button[@aria-label='More' or @aria-label='Mehr'])[last()]"]];
let index = 1;
for (const [name, xpath] of probes.length ? probes : defaults) {
  const before = new Set(await controls(page)), faultsBefore = faults.length, textBefore = await page.evaluate(() => document.body.innerText);
  try { await page.locator(`xpath=${xpath}`).first().click({ timeout: 4000 }); await page.waitForTimeout(1500); }
  catch (error) { report[name] = { error: String(error).slice(0, 300) }; continue; }
  await page.screenshot({ path: join(out, `${language}-${index++}-${name}.png`) });
  const after = await controls(page), textAfter = await page.evaluate(() => document.body.innerText);
  report[name] = { added: after.filter(line => !before.has(line)), removed: [...before].filter(line => !after.includes(line)), newText: textAfter.split("\n").filter(line => !textBefore.includes(line)).slice(0, 40), faults: faults.slice(faultsBefore) };
  await page.keyboard.press("Escape"); await page.waitForTimeout(500);
}
await writeFile(join(out, `${language}-report.json`), `${JSON.stringify(report, null, 1)}\n`);
await browser.close();
console.log(join(out, `${language}-report.json`));
