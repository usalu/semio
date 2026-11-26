import { strict as assert } from "node:assert";
import { mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";
import type { Browser, Page } from "playwright";

/** 🔬️ Captures the full open failure of the Transitional workbook (all console, page and worker messages). Derived from the standalone pane journey (port 6272): open, edit A1, save, reopen in English and German, German decimal input,
 * Strict and Transitional workbooks, and the localized refusal of a foreign file named `.xlsx`.
 * Usage: `repo <this file> [en|de|dialects|refusal ...]`. Writes screenshots, saved workbooks and `journey.json` under
 * `🗑️generated/standalone-journey`.
 * @see ./standalone-pane-journey.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const subsets = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets";
const fixtures = {
  transitional: join(root, subsets, "🧱️base/🧪️tests/🧫️fixtures/📕️reuse-marketplaces.xlsx"),
  strict: join(root, subsets, "🔒️strict/🧪️tests/🧫️fixtures/🔒️reuse-marketplaces-strict.xlsx"),
};
const url = process.env.XLSX_PANE_URL ?? "http://127.0.0.1:6272/";
const out = join(import.meta.dir, "🗑️generated", "standalone-journey");
const steps = process.argv.slice(2).length > 0 ? process.argv.slice(2) : ["en", "de", "dialects", "refusal"];
const OPENED = /^shell\.(?:documentTransfer\.(?:file-opened|file-open-failed|open-cancelled|transfer-busy)|openFile\.[^|]+)\|/u;

const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
const { unzipSync, zipSync, strToU8 } = require("fflate") as typeof import("fflate");
await mkdir(out, { recursive: true });

type Verdict = { step: string; pass: boolean; evidence: Record<string, unknown> };
const verdicts: Verdict[] = [];


const browser = await chromium.launch({args: ["--js-flags=--experimental-wasm-jspi", "--use-angle=swiftshader", "--in-process-gpu", "--disable-accelerated-2d-canvas"]});
const log: string[] = [];
try {
  const context = await browser.newContext({ locale: "en-US", acceptDownloads: true, viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  await page.addInitScript(() => {
    const seen: string[] = [];
    Object.defineProperty(window, "__journeyNotices", { value: seen });
    new MutationObserver(() => {
      for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
        const line = `${element.getAttribute("data-notice-code") ?? ""}|${(element.textContent ?? "").trim()}`;
        if (!seen.includes(line)) seen.push(line);
      }
    }).observe(document, { subtree: true, childList: true, characterData: true });
  });
  page.on("pageerror", error => log.push(`[DEBUG] pageerror: ${error.stack ?? error.message}`));
  page.on("console", message => log.push(`[DEBUG] console.${message.type()}: ${message.text()}`));
  page.on("worker", worker => { log.push(`[DEBUG] worker: ${worker.url()}`); worker.on("close", () => log.push(`[DEBUG] worker closed: ${worker.url()}`)); });
  context.on("weberror", error => log.push(`[DEBUG] weberror: ${error.error().stack ?? error.error().message}`));
  const notices = () => page.evaluate(() => [...((window as unknown as { __journeyNotices?: string[] }).__journeyNotices ?? [])]);
  await page.goto(url);
  const beacon = await awaitBeacon(page, Date.now() + 180_000);
  log.push(`[DEBUG] beacon: ${beacon}`);
  await dismissIntroduction(page);
  const selector = `[data-slot="window-action-pane"] [id="action.openArtifactFile"]`;
  if (await page.locator(selector).count() === 0) await unfoldActionsRail(page);
  const which = process.argv[2] === "strict" ? fixtures.strict : fixtures.transitional;
  const [chooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: 30_000 }), clickUncovered(page, selector)]);
  await chooser.setFiles(which);
  let notice: string | undefined;
  for (const deadline = Date.now() + 180_000; notice === undefined && Date.now() < deadline; await page.waitForTimeout(250)) notice = (await notices()).find(line => OPENED.test(line));
  log.push(`[DEBUG] notice: ${notice}`);
  log.push(`[DEBUG] all notices: ${JSON.stringify(await notices())}`);
  await page.waitForTimeout(Number(process.env.OPEN_WAIT_MS ?? 4000));
  log.push(`[DEBUG] after: ${JSON.stringify(await page.evaluate(() => ({ windows: [...document.querySelectorAll("[data-window-id]")].map(element => `${element.getAttribute("data-window-id")}|${element.getAttribute("aria-label") ?? ""}`).slice(0, 30), inputs: [...document.querySelectorAll("input[aria-label]")].map(element => `${element.getAttribute("aria-label")}=${(element as HTMLInputElement).value}`).slice(0, 30), title: document.title })))}`);
  log.push(`[DEBUG] text: ${JSON.stringify((await page.evaluate(() => document.body.innerText)).slice(0, 6000))}`);
  if (process.env.OPEN_SAVE === "1") {
    const saveSelector = `[data-slot="window-action-pane"] [id="action.saveArtifactFile"]`;
    if (await page.locator(saveSelector).count() === 0) await unfoldActionsRail(page);
    const [download] = await Promise.all([page.waitForEvent("download", { timeout: 90_000 }), clickUncovered(page, saveSelector)]);
    await download.saveAs(join(out, "browser-open-saved.xlsx"));
    log.push(`[DEBUG] saved: ${download.suggestedFilename()} ${(await Bun.file(join(out, "browser-open-saved.xlsx")).arrayBuffer()).byteLength} bytes`);
  }
  await page.screenshot({ path: join(out, "browser-open.png") });
  await context.close();
} finally {
  await browser.close();
  await writeFile(join(out, "browser-open.log"), `${log.join("\n")}\n`);
}
console.log(log.join("\n").slice(0, 20000));
