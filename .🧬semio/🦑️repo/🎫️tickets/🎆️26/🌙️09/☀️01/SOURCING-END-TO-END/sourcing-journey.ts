import { strict as assert } from "node:assert";
import { existsSync } from "node:fs";
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
  en: { pool: "Pool", curated: "Curated", preview: "Preview", grid: "Grid", search: "Search", minimum: "Minimum availability", remove: "Remove", curate: "Curate", availability: "Availability", name: "Name", noSelection: "No selection", increase: "Increase Curated", increaseCount: "Increase Count", moduleSlabs: "Slabs", typology: "Typology", allTypologies: "All Typologies" },
  de: { pool: "Pool", curated: "Kuratiert", preview: "Vorschau", grid: "Raster", search: "Suchen", minimum: "Mindestverfügbarkeit", remove: "Entfernen", curate: "Kuratieren", availability: "Verfügbarkeit", name: "Name", noSelection: "Keine Auswahl", increase: "Erhöhen Kuratiert", increaseCount: "Erhöhen Anzahl", moduleSlabs: "Slabs", typology: "Typologie", allTypologies: "Alle Typologien" },
} as const;
const kinds = ["Glulam GL24h 200×400", "KVH C24 100×200", "Steel IPE 200", "Steel HEA 160", "Casement Window 100×120", "Fixed Window 150×150", "Tilt & Turn Window 120×140", "Concrete Slab 240mm", "CLT Slab 160mm", "Hollow Core Slab 265mm"];
const reuse = "Hexagonal Cut Concrete Forest";
const BAD = /program load failed sourcing|plugin\.descriptor-invalid|ui\.fixed-capacity|program module unavailable sourcing|Plugin Recovery/u;
const DEBUG = /\[DEBUG\]/u;

/** 🪟️ One browser session in one locale with console-fault recording and per-step verdicts. */
async function session(browser: Browser, language: "en" | "de") {
  const context = await browser.newContext({ locale: language === "en" ? "en-US" : "de-DE", viewport: { width: 1800, height: 1100 } });
  const page = await context.newPage();
  const faults: string[] = [];
  page.on("pageerror", error => faults.push(`pageerror: ${error.message}`.slice(0, 400)));
  page.on("console", message => { if (!DEBUG.test(message.text()) && (message.type() === "error" || BAD.test(message.text()))) faults.push(`${message.type()}: ${message.text()}`.slice(0, 400)); });
  const t = text[language];
  const shot = (name: string) => page.screenshot({ path: join(out, `${language}-${name}.png`) });
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
  return { context, page, faults, t, shot, pool, curated, preview, rowIn, countIn, names, body, settle, eventually, step };
}

async function journey(browser: Browser, language: "en" | "de") {
  const s = await session(browser, language);
  const { page, t, pool, curated, preview, rowIn, countIn, names, body, settle, eventually, step } = s;
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
    await step("Pool lists the ten demo stock kinds and the reuse rows", async () => {
      const listed = await names(pool());
      const missing = kinds.filter(kind => !listed.includes(kind));
      assert.deepEqual(missing, []);
      assert.ok(listed.some(name => name.startsWith(reuse)), "reuse rows missing");
      return { rows: listed.length, reuse: listed.filter(name => name.startsWith(reuse)).length };
    });
    await step("Grid renders a 3D canvas", async () => {
      const canvases = await page.evaluate(() => [...document.querySelectorAll("canvas")].map(canvas => `${canvas.width}x${canvas.height}`));
      assert.ok(canvases.some(size => !size.startsWith("0x") && !size.endsWith("x0")), `canvases ${canvases}`);
      return { canvases };
    });
    const total = (await names(pool())).length;
    const target = "KVH C24 100×200";
    const curatedNames = () => names(curated());
    const poolCount = () => countIn(pool(), target).inputValue();
    const curatedCount = () => countIn(curated(), target).inputValue().catch(() => "absent");
    await step("curation add: Curated gains the row", async () => {
      await rowIn(pool(), target).getByRole("button", { name: t.increase, exact: true }).click();
      await eventually(async () => { assert.ok((await curatedNames()).includes(target), "row absent in Curated"); assert.equal(await poolCount(), "1"); });
      return {};
    });
    await step("curation set count", async () => {
      for (const expected of ["2", "3"]) {
        await rowIn(curated(), target).getByRole("button", { name: t.increaseCount, exact: true }).click();
        await eventually(async () => assert.equal(await curatedCount(), expected));
      }
      await eventually(async () => assert.equal(await poolCount(), "3"));
      return {};
    });
    await step("undo and redo of the count edit", async () => {
      await page.locator("body").click({ position: { x: 5, y: 5 } });
      await page.keyboard.press("ControlOrMeta+z");
      await eventually(async () => assert.notEqual(await curatedCount(), "3"));
      const undone = await curatedCount();
      await page.keyboard.press("ControlOrMeta+Shift+z");
      await eventually(async () => assert.equal(await curatedCount(), "3"));
      return { undone, redone: await curatedCount() };
    });
    await step("Preview renders the selected kind", async () => {
      await rowIn(curated(), target).locator("td").nth(1).click();
      await eventually(async () => {
        assert.ok(!(await preview().innerText()).includes(t.noSelection), "still no selection");
        assert.ok(await preview().locator("canvas").count() > 0, "no preview canvas");
      }, 8000);
      return { canvases: await preview().locator("canvas").count() };
    });
    await step("curation remove", async () => {
      await rowIn(curated(), target).getByRole("button", { name: t.remove, exact: true }).click();
      await eventually(async () => { assert.ok(!(await curatedNames()).includes(target), "row still in Curated"); assert.equal(await poolCount(), "0"); });
      return {};
    });
    await step("drag pool to curated and back", async () => {
      const handle = (scope: Locator, name: string) => rowIn(scope, name).locator('[data-drag-role="transfer"]').first();
      await handle(pool(), kinds[0]).dragTo(curated());
      await eventually(async () => assert.ok((await curatedNames()).includes(kinds[0]), "not added by drag"));
      await handle(curated(), kinds[0]).dragTo(pool());
      await eventually(async () => assert.ok(!(await curatedNames()).includes(kinds[0]), "not removed by drag back"));
      return {};
    });
    await step("filter by query", async () => {
      const box = pool().getByRole("textbox", { name: t.search, exact: true });
      await box.fill("steel");
      await eventually(async () => { const listed = await names(pool()); assert.ok(listed.includes("Steel IPE 200") && !listed.includes(kinds[0]), `query filter ${listed}`); });
      const listed = await names(pool());
      await box.fill("");
      await eventually(async () => assert.equal((await names(pool())).length, total, "query not cleared"));
      return { listed };
    });
    await step("filter by module", async () => {
      const toggle = pool().locator('[id="window:sourcing-pool/sourcing-filter-module-slabs"]');
      await toggle.click();
      await eventually(async () => { const listed = await names(pool()); assert.ok(listed.includes("CLT Slab 160mm") && !listed.includes(kinds[0]), `module filter ${listed}`); });
      const listed = await names(pool());
      await toggle.click();
      await eventually(async () => assert.equal((await names(pool())).length, total, "module not cleared"));
      return { listed };
    });
    await step("filter by typology", async () => {
      const select = pool().getByRole("combobox", { name: t.typology, exact: true });
      await select.click();
      const labels = await page.getByRole("option").allInnerTexts();
      await page.getByRole("option").nth(1).click();
      await eventually(async () => { const filtered = await names(pool()); assert.ok(filtered.length > 0 && filtered.length < 12, `typology filter ${filtered}`); });
      const filtered = await names(pool());
      await select.click(); await page.getByRole("option").first().click();
      await eventually(async () => assert.equal((await names(pool())).length, total, "typology not cleared"));
      return { labels, filtered };
    });
    const wedged = () => s.faults.some(line => /prior-outcome|runtime live cleanup faulted/u.test(line));
    const restart = async () => {
      await page.reload();
      assert.match((await awaitBeacon(page, Date.now() + 300_000)) ?? "", /^ready/u);
      await dismissIntroduction(page); await settle(3000);
    };
    await step("filter by minimum availability", async () => {
      const input = pool().getByRole("spinbutton", { name: t.minimum, exact: true });
      try {
        await input.fill("25"); await input.press("Enter");
        await eventually(async () => { const listed = await names(pool()); assert.ok(!listed.includes("Steel HEA 160") && listed.includes("KVH C24 100×200"), `min availability ${listed}`); });
        const listed = await names(pool());
        await input.fill("0"); await input.press("Enter");
        await eventually(async () => assert.equal((await names(pool())).length, total, "min availability not cleared"));
        return { listed };
      } catch (error) {
        if (wedged()) { await restart(); throw new Error(`${error}; instance wedged by the edit (plugin.internal.prior-outcome), page reloaded to isolate later steps`); }
        throw error;
      }
    });
    await step("sort by availability", async () => {
      const header = pool().getByRole("columnheader", { name: t.availability, exact: true });
      const column = async () => (await pool().locator("tbody tr td:nth-child(4)").allInnerTexts()).map(Number);
      const monotone = (values: number[], sign: number) => values.length > 1 && values.every((value, index) => index === 0 || sign * (value - values[index - 1]) >= 0);
      await header.getByRole("button").click();
      await eventually(async () => { const values = await column(); assert.ok(monotone(values, 1) || monotone(values, -1), `unsorted ${values}`); });
      const first = await column();
      const ascending = monotone(first, 1);
      await header.getByRole("button").click();
      await eventually(async () => assert.ok(monotone(await column(), ascending ? -1 : 1), "second click did not reverse"));
      return { first, second: await column(), ariaSort: await header.getAttribute("aria-sort") };
    });
    await step("example selector Demo to Empty to Demo", async () => {
      const selector = page.locator('[id="playground.navbar.fixture"]');
      await selector.click();
      const offered = await page.getByRole("option").allInnerTexts();
      const empty = page.getByRole("option", { name: /^(no example|empty|kein|leer)/iu });
      let wasEmpty: boolean | "not offered" = "not offered";
      if (await empty.count() > 0) {
        await empty.first().click();
        await eventually(async () => assert.ok(!(await names(pool())).includes(kinds[0]), "empty example kept rows"), 15000);
        wasEmpty = true;
        await selector.click();
      }
      await page.getByRole("option", { name: /demo/iu }).first().click();
      await eventually(async () => assert.ok((await names(pool())).includes(kinds[0]), `demo not loaded; offered ${offered}`), 15000);
      assert.notEqual(wasEmpty, "not offered", `the example selector offers only ${offered}: no empty example to switch to`);
      return { offered, wasEmpty };
    });
    await step("folder save, reload and reconnect keep the curated state", async () => {
      const folder = join(root, ".cache/b/p/m/sourcing-data", `journey-${language}-${Date.now()}`);
      await page.locator("#s-sync-status").click();
      await page.locator("[id='ui.utilities.group.sync']").click();
      await page.locator("[id='framework.sync.folder']").click();
      await page.locator("[id='framework.sync.folder.path']").fill(folder);
      await page.getByRole("button", { name: language === "en" ? "Attach" : "Verbinden", exact: true }).click();
      await settle(2500);
      await rowIn(pool(), kinds[2]).getByRole("button", { name: t.increase, exact: true }).click();
      await eventually(async () => assert.ok((await curatedNames()).includes(kinds[2]), "not curated"));
      await eventually(async () => assert.ok(existsSync(join(folder, ".semio", "events.semio")), "folder holds no saved archive"), 20000);
      await settle(3000);
      await page.reload();
      assert.match((await awaitBeacon(page, Date.now() + 300_000)) ?? "", /^ready/u);
      await dismissIntroduction(page);
      await eventually(async () => assert.equal(await page.locator("#s-folder-reconnect").count(), 1, "reload offered no folder reconnect"), 30000);
      assert.ok(!(await curatedNames()).includes(kinds[2]), "curated state present before reconnect");
      await page.locator("#s-folder-reconnect").click();
      await eventually(async () => assert.ok((await curatedNames()).includes(kinds[2]), "curated row lost after reconnect"), 30000);
      await eventually(async () => assert.equal(await page.locator("[data-semio-folder-reconnect]").count(), 0, "reconnect offer stayed after reconnect"), 15000);
      return { folder, curated: await curatedNames() };
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
