import { strict as assert } from "node:assert";
import { mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";
import type { Browser, Locator, Page } from "playwright";

/** 🧰️ Shared kit of the sourcing browser journeys: Playwright from the repo browser path, the localized labels, one locale session with console-fault recording, per-step verdicts with screenshots, and a runner that writes `<name>.json` under `🗑️generated/journey`.
 * @see ./sourcing-journey.ts */

export const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
export const url = process.env.SOURCING_URL ?? "http://127.0.0.1:6081/";
export const out = join(import.meta.dir, "🗑️generated", "journey");
export const languages = process.argv.slice(2).length > 0 ? process.argv.slice(2) : ["en", "de"];
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
export const { awaitBeacon, dismissIntroduction } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
export const { chromium } = require("playwright") as typeof import("playwright");
await mkdir(out, { recursive: true });

export type Verdict = { step: string; pass: boolean; evidence: Record<string, unknown> };
export const verdicts: Verdict[] = [];
export const text = {
  en: { pool: "Pool", curated: "Curated", preview: "Preview", grid: "Grid", search: "Search", minimum: "Minimum availability", remove: "Remove", curate: "Curate", availability: "Availability", name: "Name", noSelection: "No selection", increase: "Increase Curated", increaseCount: "Increase Count", moduleSlabs: "Slabs", typology: "Typology", allTypologies: "All Typologies" },
  de: { pool: "Pool", curated: "Kuratiert", preview: "Vorschau", grid: "Raster", search: "Suchen", minimum: "Mindestverfügbarkeit", remove: "Entfernen", curate: "Kuratieren", availability: "Verfügbarkeit", name: "Name", noSelection: "Keine Auswahl", increase: "Erhöhen Kuratiert", increaseCount: "Erhöhen Anzahl", moduleSlabs: "Slabs", typology: "Typologie", allTypologies: "Alle Typologien" },
} as const;
export const kinds = ["Glulam GL24h 200×400", "KVH C24 100×200", "Steel IPE 200", "Steel HEA 160", "Casement Window 100×120", "Fixed Window 150×150", "Tilt & Turn Window 120×140", "Concrete Slab 240mm", "CLT Slab 160mm", "Hollow Core Slab 265mm"];
export const reuse = "Hexagonal Cut Concrete Forest";
/** 🗣️ The demo stock names the editor shows per language (English document names, German catalog names); `reuse` is the common prefix of both reuse rows. */
export const stockNames = {
  en: { kinds, reuse, reuseRows: ["Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Right"] },
  de: { kinds: ["Brettschichtholz GL24h 200×400", "KVH C24 100×200", "Stahl IPE 200", "Stahl HEA 160", "Drehflügelfenster 100×120", "Festverglasung 150×150", "Dreh-Kipp-Fenster 120×140", "Betondecke 240 mm", "BSP-Decke 160 mm", "Hohlkörperdecke 265 mm"], reuse: "Sechseckig geschnittener Betonwald", reuseRows: ["Sechseckig geschnittener Betonwald links", "Sechseckig geschnittener Betonwald rechts"] },
} as const;
const allEnglish = [...kinds, "Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Right"];
const allGerman = [...stockNames.de.kinds, ...stockNames.de.reuseRows];
export const BAD = /program load failed sourcing|plugin\.descriptor-invalid|ui\.fixed-capacity|program module unavailable sourcing|Plugin Recovery/u;
export const DEBUG = /\[DEBUG\]/u;

/** 🪟️ One browser session in one locale with console-fault recording and per-step verdicts. */
export async function session(browser: Browser, language: "en" | "de") {
  const context = await browser.newContext({ locale: language === "en" ? "en-US" : "de-DE", viewport: { width: 1800, height: 1100 } });
  const page = await context.newPage();
  const faults: string[] = [];
  page.on("pageerror", error => faults.push(`pageerror: ${error.message}`.slice(0, 400)));
  page.on("console", message => { if (!DEBUG.test(message.text()) && (message.type() === "error" || BAD.test(message.text()))) faults.push(`${message.type()}: ${message.text()}`.slice(0, 400)); });
  const t = text[language];
  const names_ = stockNames[language];
  const n = (english: string) => language === "en" ? english : allGerman[allEnglish.indexOf(english)] ?? english;
  const shot = (name: string) => page.screenshot({ path: join(out, `${process.env.JOURNEY ?? "journey"}-${language}-${name}.png`) });
  const panel = (name: string) => page.getByRole("tabpanel", { name, exact: true });
  const pool = () => panel(t.pool);
  const curated = () => panel(t.curated);
  const preview = () => panel(t.preview);
  const rowIn = (scope: Locator, name: string) => scope.getByRole("row").filter({ hasText: name }).first();
  const countIn = (scope: Locator, name: string) => rowIn(scope, name).getByRole("spinbutton").first();
  const names = async (scope: Locator) => scope.locator("tbody tr").evaluateAll(rows => rows.map(row => row.querySelector("td")?.textContent?.replace(/^\s*(Drag to window|Verschieben)\s*/u, "").trim() ?? ""));
  const body = () => page.evaluate(() => document.body.innerText);
  const settle = (ms = 700) => page.waitForTimeout(ms);
  const eventually = async (check: () => Promise<void>, timeout = 10_000) => {
    const deadline = Date.now() + timeout;
    for (;;) { try { return await check(); } catch (error) { if (Date.now() > deadline) throw error; await page.waitForTimeout(250); } }
  };
  const step = async (name: string, run: () => Promise<Record<string, unknown>>) => {
    const before = faults.length;
    let verdict: Verdict;
    try { verdict = { step: `${language}: ${name}`, pass: true, evidence: await run() }; }
    catch (error) { verdict = { step: `${language}: ${name}`, pass: false, evidence: { error: String(error).slice(0, 1200) } }; }
    const fresh = faults.slice(before);
    if (fresh.some(line => BAD.test(line))) verdict.pass = false;
    verdict.evidence.faults = fresh.slice(0, 6);
    await shot(`${verdicts.length}`).catch(() => undefined);
    verdicts.push(verdict);
  };
  return { context, page, faults, t, kinds: [...names_.kinds], reuse: names_.reuse, n, shot, pool, curated, preview, rowIn, countIn, names, body, settle, eventually, step };
}

/** 🏁️ Runs one journey per requested language in a fresh browser, writes `<name>.json` of all verdicts and sets the exit code. */
export async function runJourneys(name: string, journey: (browser: Browser, language: "en" | "de") => Promise<void>) {
  const browser = await chromium.launch();
  try {
    for (const language of languages) await journey(browser, language as "en" | "de").catch(error => verdicts.push({ step: `${language}: journey`, pass: false, evidence: { error: String(error).slice(0, 1500) } }));
  } finally {
    await browser.close();
    await writeFile(join(out, `${name}.json`), `${JSON.stringify(verdicts, null, 2)}\n`);
  }
  for (const verdict of verdicts) console.log(`${verdict.pass ? "PASS" : "FAIL"} ${verdict.step}`);
  process.exitCode = verdicts.every(verdict => verdict.pass) ? 0 : 1;
}
export { assert, join, writeFile };
export type { Browser, Locator, Page };
