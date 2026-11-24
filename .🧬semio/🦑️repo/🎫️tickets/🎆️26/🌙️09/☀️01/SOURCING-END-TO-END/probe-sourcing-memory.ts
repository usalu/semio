import { mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🔭️ Loads the sourcing curation editor, waits for boot, and dumps console, page errors, failed requests and the accessibility tree to `🗑️generated/probe`.
 * Usage: `repo <this file> [en-US|de-DE]` with `serve-sourcing-react-prototype` answering on 6081 (override with `SOURCING_URL`).
 * @see ./🌐️browser-journey-2026-11.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const url = process.env.SOURCING_URL ?? "http://127.0.0.1:6081/";
const locale = process.argv[2] ?? "en-US";
const out = join(import.meta.dir, "🗑️generated", "probe-memory");
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, dismissIntroduction } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
await mkdir(out, { recursive: true });
const browser = await chromium.launch();
const page = await (await browser.newContext({ locale, viewport: { width: 1600, height: 1000 } })).newPage();
const log: string[] = [];
page.on("pageerror", error => log.push(`pageerror: ${error.message}`.slice(0, 400)));
page.on("console", message => log.push(`${message.type()}: ${message.text()}`.slice(0, 400)));
page.on("requestfailed", request => log.push(`requestfailed: ${request.url().slice(0, 200)}`));
page.on("response", response => { if (response.status() >= 400) log.push(`http ${response.status()}: ${response.url().slice(0, 200)}`); });
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
await page.goto(url);
const beacon = await awaitBeacon(page, Date.now() + 240_000).catch(error => `ERR ${error}`);
await dismissIntroduction(page).catch(() => undefined);
await page.waitForTimeout(5_000);
console.log("beacon", beacon);
await page.screenshot({ path: join(out, `probe-${locale}.png`) });
const tree = await page.locator("body").ariaSnapshot().catch(error => `ariaSnapshot failed: ${error}`);
const dom = await page.evaluate(() => ({
  title: document.title,
  windows: [...document.querySelectorAll("[data-window-id]")].map(element => element.getAttribute("data-window-id")),
  tables: [...document.querySelectorAll("table,[role=grid],[role=table]")].map(element => `${element.tagName}|${element.getAttribute("aria-label") ?? ""}|rows=${element.querySelectorAll("tr,[role=row]").length}`),
  canvases: [...document.querySelectorAll("canvas")].map(canvas => `${canvas.width}x${canvas.height}`),
  controls: [...document.querySelectorAll("button,input,select,[role=combobox],[role=checkbox],[role=switch]")].map(element => `${element.tagName.toLowerCase()}|${element.id}|${element.getAttribute("aria-label") ?? ""}|${(element.textContent ?? "").trim().slice(0, 30)}`).slice(0, 200),
}));
await writeFile(join(out, `console-${locale}.txt`), `${log.join("\n")}\n`);
await writeFile(join(out, `aria-${locale}.yml`), `${tree}\n`);
await writeFile(join(out, `dom-${locale}.json`), `${JSON.stringify({ beacon, ...dom }, null, 2)}\n`);
console.log(`log lines ${log.length}; errors ${log.filter(line => /^(pageerror|error)/u.test(line)).length}`);
await browser.close();
