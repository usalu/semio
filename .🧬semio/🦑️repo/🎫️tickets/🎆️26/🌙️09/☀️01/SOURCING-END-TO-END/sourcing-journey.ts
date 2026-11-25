import { existsSync } from "node:fs";
import { assert, awaitBeacon, dismissIntroduction, join, kinds, out, reuse, root, runJourneys, session, url, writeFile, type Browser, type Locator } from "./journey-kit.ts";

/** 🪵️ Sourcing curation editor journey (6081): boot, windows, pool, curation add/count/remove, drag both ways, filters, sort, example switch, undo/redo, grid canvas, preview, reload; English and German.
 * Usage: `bun sourcing-journey.ts [en|de ...]` with the native sourcing host answering on 6081 (override with `SOURCING_URL`).
 * Writes screenshots and `journey.json` under `🗑️generated/journey`.
 * @see ./journey-kit.ts */

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
    const wedged = () => s.faults.some(line => /prior-outcome|runtime live cleanup faulted|publication-stalled|zero-progress/u.test(line));
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
        if (wedged()) { await restart(); throw new Error(`${error}; instance wedged by the edit, page reloaded to isolate later steps`); }
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

await runJourneys("journey", journey);
