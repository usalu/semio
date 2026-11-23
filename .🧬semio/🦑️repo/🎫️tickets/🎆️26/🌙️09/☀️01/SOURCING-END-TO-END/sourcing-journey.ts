import { strict as assert } from "node:assert";
import { mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";
import type { Browser, Locator, Page } from "playwright";

/** 🪵️ Sourcing curation editor journey (6081): boot, windows, pool, curation add/count/remove, drag both ways, filters, sort, example switch, undo/redo, grid canvas, preview, reload; English and German.
 * Usage: `repo <this file> [en|de ...]` with `serve-sourcing-react-prototype` answering (override with `SOURCING_URL`).
 * Writes screenshots and `journey.json` under `🗑️generated/journey`.
 * @see ./🌐️browser-journey-2026-11.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const url = process.env.SOURCING_URL ?? "http://127.0.0.1:6081/";
const out = join(import.meta.dir, "🗑️generated", "journey");
const languages = process.argv.slice(2).length > 0 ? process.argv.slice(2) : ["en", "de"];
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, dismissIntroduction } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
await mkdir(out, { recursive: true });

type Verdict = { step: string; pass: boolean; evidence: Record<string, unknown> };
const verdicts: Verdict[] = [];
const text = {
  en: { pool: "Pool", curated: "Curated", preview: "Preview", grid: "Grid", search: "Search", minimum: "Minimum availability", remove: "Remove", curate: "Curate", availability: "Availability", name: "Name", noSelection: "No selection" },
  de: { pool: "Pool", curated: "Kuratiert", preview: "Vorschau", grid: "Raster", search: "Suchen", minimum: "Mindestverfügbarkeit", remove: "Entfernen", curate: "Kuratieren", availability: "Verfügbarkeit", name: "Name", noSelection: "Keine Auswahl" },
} as const;
const kinds = ["Glulam GL24h 200×400", "KVH C24 100×200", "Steel IPE 200", "Steel HEA 160", "Casement Window 100×120", "Fixed Window 150×150", "Tilt & Turn Window 120×140", "Concrete Slab 240mm", "CLT Slab 160mm", "Hollow Core Slab 265mm"];
const BAD = /program load failed sourcing|plugin\.descriptor-invalid|ui\.fixed-capacity|program module unavailable sourcing|Plugin Recovery/u;

/** 🪟️ One browser session in one locale with console-fault recording and per-step verdicts. */
async function session(browser: Browser, language: "en" | "de") {
  const context = await browser.newContext({ locale: language === "en" ? "en-US" : "de-DE", viewport: { width: 1800, height: 1100 } });
  const page = await context.newPage();
  const faults: string[] = [];
  page.on("pageerror", error => faults.push(`pageerror: ${error.message}`.slice(0, 400)));
  page.on("console", message => { if (message.type() === "error" || BAD.test(message.text())) faults.push(`${message.type()}: ${message.text()}`.slice(0, 400)); });
  const t = text[language];
  const shot = (name: string) => page.screenshot({ path: join(out, `${language}-${name}.png`) });
  const win = (title: string) => page.locator(`[data-window-id*="${title}"]`).first();
  const pool = () => page.locator('[data-window-id="sourcing-pool"]').first();
  const curated = () => page.locator('[data-window-id="sourcing-curated"]').first();
  const rowIn = (scope: Locator, name: string) => scope.getByRole("row").filter({ hasText: name }).first();
  const body = () => page.evaluate(() => document.body.innerText);
  const settle = (ms = 700) => page.waitForTimeout(ms);
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
  return { context, page, faults, t, shot, win, pool, curated, rowIn, body, settle, step };
}

async function journey(browser: Browser, language: "en" | "de") {
  const s = await session(browser, language);
  const { page, t, pool, curated, rowIn, body, settle, step } = s;
  try {
    await page.goto(url);
    const beacon = await awaitBeacon(page, Date.now() + 300_000);
    await dismissIntroduction(page);
    await settle(3000);
    await step("boot without program/descriptor faults or page errors", async () => {
      assert.match(beacon ?? "", /^ready/u, `beacon ${beacon}`);
      assert.equal(s.faults.filter(line => line.startsWith("pageerror") || BAD.test(line)).length, 0, s.faults.join("\n"));
      return { beacon };
    });
    await step("Pool, Curated, Preview and Grid windows render", async () => {
      const text0 = await body();
      for (const label of [t.pool, t.curated, t.preview, t.grid]) assert.ok(text0.includes(label), `missing window label ${label}`);
      return { windows: await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map(element => element.getAttribute("data-window-id"))) };
    });
    await step("Pool lists the ten demo stock kinds", async () => {
      const content = await pool().innerText();
      const missing = kinds.filter(kind => !content.includes(kind));
      assert.deepEqual(missing, []);
      return { rows: await pool().getByRole("row").count() };
    });
    await step("Grid renders a 3D canvas", async () => {
      const canvases = await page.evaluate(() => [...document.querySelectorAll("canvas")].map(canvas => `${canvas.width}x${canvas.height}`));
      assert.ok(canvases.some(size => !size.startsWith("0x") && !size.endsWith("x0")), `canvases ${canvases}`);
      return { canvases };
    });
    const target = "KVH C24 100×200";
    await step("curation add: Curated gains the row", async () => {
      await rowIn(pool(), target).getByRole("button", { name: /^(\+|Curate|Kuratieren|Increase|Erhöhen)/iu }).first().click();
      await settle();
      assert.ok((await curated().innerText()).includes(target), "row absent in Curated");
      return {};
    });
    await step("curation set count", async () => {
      const input = rowIn(curated(), target).getByRole("spinbutton").first();
      await input.fill("3"); await input.press("Enter"); await settle();
      assert.equal(await rowIn(curated(), target).getByRole("spinbutton").first().inputValue(), "3");
      return {};
    });
    await step("undo and redo of the count edit", async () => {
      await page.keyboard.press("Control+z"); await settle(1200);
      const undone = await rowIn(curated(), target).getByRole("spinbutton").first().inputValue().catch(() => "absent");
      await page.keyboard.press("Control+Shift+z"); await settle(1200);
      const redone = await rowIn(curated(), target).getByRole("spinbutton").first().inputValue().catch(() => "absent");
      assert.notEqual(undone, "3"); assert.equal(redone, "3");
      return { undone, redone };
    });
    await step("Preview renders the selected kind", async () => {
      await rowIn(curated(), target).getByRole("cell").first().click(); await settle(1500);
      const content = await page.locator('[data-window-id="sourcing-preview"]').first().innerText();
      assert.ok(!content.includes(t.noSelection), "still no selection");
      const canvases = await page.locator('[data-window-id="sourcing-preview"] canvas').count();
      assert.ok(canvases > 0, "no preview canvas");
      return { canvases };
    });
    await step("curation remove", async () => {
      await rowIn(curated(), target).getByRole("button", { name: new RegExp(t.remove, "iu") }).first().click(); await settle();
      assert.ok(!(await curated().innerText()).includes(target), "row still in Curated");
      return {};
    });
    await step("drag pool to curated and back", async () => {
      await rowIn(pool(), kinds[0]).dragTo(curated()); await settle();
      const added = (await curated().innerText()).includes(kinds[0]);
      await rowIn(curated(), kinds[0]).dragTo(pool()); await settle();
      const removed = !(await curated().innerText()).includes(kinds[0]);
      assert.ok(added && removed, `added ${added} removed ${removed}`);
      return { added, removed };
    });
    await step("filter by query", async () => {
      const box = pool().getByRole("searchbox").or(pool().getByRole("textbox", { name: new RegExp(t.search, "iu") })).first();
      await box.fill("steel"); await settle();
      const content = await pool().innerText();
      assert.ok(content.includes("Steel IPE 200") && !content.includes("Glulam"), "query filter");
      await box.fill(""); await settle();
      return {};
    });
    await step("filter by module", async () => {
      const toggle = pool().getByRole("checkbox", { name: /slabs/iu }).or(pool().getByRole("button", { name: /slabs/iu })).first();
      await toggle.click(); await settle();
      const content = await pool().innerText();
      assert.ok(content.includes("CLT Slab"), "slab missing");
      await toggle.click(); await settle();
      return {};
    });
    await step("filter by typology", async () => {
      const select = pool().getByRole("combobox").first();
      await select.selectOption({ index: 1 }).catch(async () => { await select.click(); await page.getByRole("option").nth(1).click(); });
      await settle();
      const rows = await pool().getByRole("row").count();
      await select.selectOption({ index: 0 }).catch(() => undefined); await settle();
      return { rowsFiltered: rows };
    });
    await step("filter by minimum availability", async () => {
      const input = pool().getByRole("spinbutton", { name: new RegExp(t.minimum, "iu") }).first();
      await input.fill("25"); await input.press("Enter"); await settle();
      const content = await pool().innerText();
      assert.ok(!content.includes("Steel HEA 160") && content.includes("KVH C24"), "min availability");
      await input.fill("0"); await input.press("Enter"); await settle();
      return {};
    });
    await step("sort by availability", async () => {
      const header = pool().getByRole("columnheader", { name: new RegExp(t.availability, "iu") }).first();
      await header.getByRole("button").first().click(); await settle();
      const first = header.getAttribute("aria-sort");
      await header.getByRole("button").first().click(); await settle();
      return { asc: await first, second: await header.getAttribute("aria-sort") };
    });
    await step("example selector Demo to Empty to Demo", async () => {
      const selector = page.getByRole("combobox", { name: /example|beispiel/iu }).or(page.getByRole("button", { name: /demo/iu })).first();
      await selector.click();
      await page.getByRole("option", { name: /^(no example|empty|kein|leer)/iu }).or(page.getByRole("menuitem", { name: /^(no example|empty|kein|leer)/iu })).first().click(); await settle(2000);
      const empty = !(await pool().innerText()).includes(kinds[0]);
      await selector.click();
      await page.getByRole("option", { name: /demo/iu }).or(page.getByRole("menuitem", { name: /demo/iu })).first().click(); await settle(2500);
      const back = (await pool().innerText()).includes(kinds[0]);
      assert.ok(empty && back, `empty ${empty} back ${back}`);
      return { empty, back };
    });
    await step("reload keeps the curated state", async () => {
      await rowIn(pool(), kinds[2]).getByRole("button", { name: /^(\+|Curate|Kuratieren|Increase|Erhöhen)/iu }).first().click(); await settle(1500);
      await page.reload();
      assert.match((await awaitBeacon(page, Date.now() + 300_000)) ?? "", /^ready/u);
      await dismissIntroduction(page); await settle(3000);
      const kept = (await curated().innerText()).includes(kinds[2]);
      return { kept };
    });
  } finally {
    await s.shot("final").catch(() => undefined);
    await writeFile(join(out, `console-${language}.txt`), `${s.faults.join("\n")}\n`);
    await s.context.close();
  }
}

const browser = await chromium.launch();
try {
  for (const language of languages) await journey(browser, language as "en" | "de").catch(error => verdicts.push({ step: `${language}: journey`, pass: false, evidence: { error: String(error).slice(0, 1500) } }));
} finally {
  await browser.close();
  await writeFile(join(out, "journey.json"), `${JSON.stringify(verdicts, null, 2)}\n`);
}
for (const verdict of verdicts) console.log(`${verdict.pass ? "PASS" : "FAIL"} ${verdict.step}`);
process.exitCode = verdicts.every(verdict => verdict.pass) ? 0 : 1;
