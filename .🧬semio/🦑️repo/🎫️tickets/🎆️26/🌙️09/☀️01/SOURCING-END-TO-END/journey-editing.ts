import { createHash } from "node:crypto";
import { assert, awaitBeacon, dismissIntroduction, kinds, out, runJourneys, session, url, writeFile, type Browser, type Locator } from "./journey-kit.ts";

/** ✏️ Sourcing curation editing journey (6081): selection (click, Escape, Select All, selection modes, granularity, shift/ctrl/alt merges, grid marquee, hover), clipboard (Copy, Cut, Paste with every anchor), typed and stepped counts, the Curate row action, the row More menu, Restock, Curation Set Count, Set Active Example, Grid Instance Display and the contributions lane; English and German.
 * Usage: `JOURNEY=editing bun journey-editing.ts [en|de ...]` with the native sourcing host on 6081.
 * @see ./journey-kit.ts
 * @see ./✂️editing-features.md */

const KVH = "KVH C24 100×200";
const IPE = "Steel IPE 200";
const HEA = "Steel HEA 160";
const GLULAM = kinds[0];
const CASEMENT = "Casement Window 100×120";
const KVH_ID = "beam-kvh-c24";
const WINDOWS = { pool: "sourcingPool", curated: "sourcingCurated", preview: "sourcingPreview", grid: "sourcingGrid" } as const;
type Win = (typeof WINDOWS)[keyof typeof WINDOWS];

async function journey(browser: Browser, language: "en" | "de") {
  const s = await session(browser, language);
  const { page, t, pool, curated, preview, rowIn, countIn, names, settle, eventually } = s;
  await s.context.grantPermissions(["clipboard-read", "clipboard-write"]).catch(() => undefined);
  const boot = async () => {
    await page.goto(url);
    assert.match((await awaitBeacon(page, Date.now() + 300_000)) ?? "", /^ready/u);
    await dismissIntroduction(page);
    await settle(3000);
  };
  const total = () => names(pool()).then(rows => rows.length);
  const cell = (scope: Locator, name: string) => rowIn(scope, name).locator("td").nth(1);
  const active = (scope: Locator) => scope.locator('tbody tr[data-active="true"]').evaluateAll(rows => rows.map(row => row.querySelector("td")?.textContent?.replace(/^\s*(Drag to window|Verschieben)\s*/u, "").trim() ?? ""));
  const poolCount = (name: string) => countIn(pool(), name).inputValue();
  const curatedCount = (name: string) => countIn(curated(), name).inputValue().catch(() => "absent");
  const curatedNames = () => names(curated()).then(rows => rows.filter(row => row !== "No data" && row !== "Keine Daten"));
  const previewEmpty = async () => (await preview().innerText()).includes(t.noSelection);
  const toggle = (win: Win) => page.locator(`[id="framework.window.${win}.engagement.toggle"]`);
  let opened: Win = WINDOWS.pool;
  const closeAllActions = async () => {
    if ((await page.locator('[id="action.category.actions"]').count()) === 0) return;
    await toggle(opened).click();
    await settle(400);
  };
  const openActions = async (win: Win) => {
    await closeAllActions();
    opened = win;
    await toggle(win).click();
    await settle(500);
  };
  const closeActions = async (_win: Win) => closeAllActions();
  const step = (name: string, body: () => Promise<Record<string, unknown>>) => s.step(name, async () => { await closeAllActions().catch(() => undefined); return body(); });
  const act = async (win: Win, id: string) => {
    await openActions(win);
    await page.locator(`[id="action.${id}"]`).click();
    await settle(900);
  };
  const run = async (win: Win, id: string) => { await act(win, id); await closeActions(win); await settle(600); };
  const form = async (win: Win, id: string, args: Record<string, string | number>) => {
    await act(win, id);
    const prefix = `framework.window.${win}.action.${id}`;
    for (const [name, value] of Object.entries(args)) {
      const field = page.locator(`[id="${prefix}.arg.${name}.input"]`);
      if ((await field.getAttribute("role")) === "combobox") { await field.click(); await page.getByRole("option").nth(Number(value)).click(); }
      else await field.fill(String(value));
    }
    await settle(300);
    await page.locator(`[id="${prefix}.execute"]`).click();
    await settle(1200);
    await closeActions(win);
  };
  const stepper = (scope: Locator, name: string, control: "increment" | "decrement") => rowIn(scope, name).locator(`[data-stepper-control="${control}"]`);
  const type = async (scope: Locator, name: string, text: string) => {
    const box = countIn(scope, name);
    await box.click();
    await box.fill(text);
    await box.press("Enter");
    await settle(1500);
  };
  const setPool = async (name: string, count: number) => {
    await type(pool(), name, String(count));
    await eventually(async () => assert.equal(await poolCount(name), String(count)));
  };
  const grid = () => page.getByRole("tabpanel", { name: t.grid, exact: true });
  const canvasHash = async () => createHash("md5").update(await grid().locator("canvas").first().screenshot()).digest("hex");
  const fresh = async () => { await boot(); };
  const sameSet = (left: string[], right: string[]) => assert.deepEqual([...left].sort(), [...right].sort());
  const clickRow = async (name: string, modifiers: ("Shift" | "Control" | "Alt")[] = []) => { await cell(pool(), name).click({ modifiers }); await settle(900); };

  try {
    await boot();

    await step("selection: a row click selects and paints the row, Escape clears", async () => {
      await clickRow(KVH);
      await eventually(async () => { sameSet(await active(pool()), [KVH]); assert.equal(await previewEmpty(), false, "Preview still empty"); });
      await page.keyboard.press("Escape");
      await eventually(async () => { assert.deepEqual(await active(pool()), []); assert.equal(await previewEmpty(), true); });
      return {};
    });

    await step("selection: Set Selection Mode… Single and Multiple, Select All, Clear Selection", async () => {
      await form(WINDOWS.pool, "setSelectionMode", { domainId: "rows", mode: 0 });
      await run(WINDOWS.pool, "selectAll");
      const single = (await active(pool())).length;
      assert.ok(single <= 1, `single mode kept ${single} selected`);
      await run(WINDOWS.pool, "clearSelection");
      await form(WINDOWS.pool, "setSelectionMode", { domainId: "rows", mode: 1 });
      await run(WINDOWS.pool, "selectAll");
      const n = await total();
      await eventually(async () => assert.equal((await active(pool())).length, n, "Select All in Multiple mode"));
      await run(WINDOWS.pool, "clearSelection");
      await eventually(async () => assert.deepEqual(await active(pool()), []));
      return { single, rows: n };
    });

    await step("selection: Select All shortcut and Escape with the table focused", async () => {
      await clickRow(KVH);
      await page.keyboard.press("ControlOrMeta+a");
      const n = await total();
      await eventually(async () => assert.equal((await active(pool())).length, n, "mod+a selected all"));
      await page.keyboard.press("Escape");
      await eventually(async () => assert.deepEqual(await active(pool()), []));
      return {};
    });

    await step("selection: Set Granularity… accepts object and refuses an unknown granularity", async () => {
      await form(WINDOWS.pool, "setInteractionGranularity", { domainId: "rows", granularityId: "object" });
      await clickRow(IPE);
      await eventually(async () => sameSet(await active(pool()), [IPE]));
      await form(WINDOWS.pool, "setInteractionGranularity", { domainId: "rows", granularityId: "no-such-granularity" });
      await clickRow(HEA);
      await eventually(async () => sameSet(await active(pool()), [HEA]));
      return {};
    });

    await step("selection: shift range, ctrl invert and alt subtract in the Pool", async () => {
      await form(WINDOWS.pool, "setSelectionMode", { domainId: "rows", mode: 1 });
      await clickRow(KVH);
      await clickRow(HEA, ["Shift"]);
      await eventually(async () => sameSet(await active(pool()), [KVH, IPE, HEA]));
      await clickRow(CASEMENT, ["Control"]);
      await eventually(async () => sameSet(await active(pool()), [KVH, IPE, HEA, CASEMENT]));
      await clickRow(IPE, ["Control"]);
      await eventually(async () => sameSet(await active(pool()), [KVH, HEA, CASEMENT]));
      await clickRow(HEA, ["Alt"]);
      await eventually(async () => sameSet(await active(pool()), [KVH, CASEMENT]));
      return { selected: await active(pool()) };
    });

    await step("selection: the Pool selection is painted in Curated and in the Preview", async () => {
      await setPool(KVH, 2);
      await setPool(IPE, 1);
      await clickRow(KVH);
      await eventually(async () => { sameSet(await active(curated()), [KVH]); assert.equal(await previewEmpty(), false); });
      return {};
    });

    await step("selection: a Curated row click selects across Pool and Curated", async () => {
      await cell(curated(), IPE).click();
      await eventually(async () => { sameSet(await active(pool()), [IPE]); sameSet(await active(curated()), [IPE]); });
      await page.keyboard.press("Escape");
      return {};
    });

    await step("selection: Grid marquee selects the curated kinds, shift adds, ctrl subtracts, hover raises no fault", async () => {
      const box = (await grid().boundingBox())!;
      const marquee = async (modifier?: "Shift" | "Control") => {
        if (modifier) await page.keyboard.down(modifier);
        await page.mouse.move(box.x + 30, box.y + 60);
        await page.mouse.down();
        await page.mouse.move(box.x + box.width - 30, box.y + box.height - 120, { steps: 14 });
        await page.mouse.up();
        if (modifier) await page.keyboard.up(modifier);
        await settle(1200);
      };
      const before = s.faults.length;
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.mouse.move(box.x + box.width / 2 + 40, box.y + box.height / 2 + 20, { steps: 6 });
      await cell(pool(), GLULAM).hover();
      await settle(600);
      assert.equal(s.faults.slice(before).filter(line => /pageerror|dropped action/u.test(line)).length, 0, s.faults.slice(before).join("\n"));
      await marquee();
      await eventually(async () => sameSet(await active(pool()), [KVH, IPE]));
      await page.keyboard.press("Escape");
      await clickRow(KVH);
      await marquee("Shift");
      await eventually(async () => sameSet(await active(pool()), [KVH, IPE]));
      await marquee("Control");
      await eventually(async () => assert.deepEqual(await active(pool()), []));
      return {};
    });

    await step("clipboard: Copy, Cut and Paste move curated counts, every anchor pastes alike", async () => {
      await setPool(KVH, 3);
      await clickRow(KVH);
      await run(WINDOWS.curated, "copy");
      await settle(800);
      await run(WINDOWS.curated, "cut");
      await eventually(async () => assert.ok(!(await curatedNames()).includes(KVH), "cut left the kind curated"));
      let expected = 0;
      for (let anchor = 0; anchor < 7; anchor++) {
        await form(WINDOWS.curated, "paste", { anchor });
        expected += 3;
        await eventually(async () => assert.equal(await curatedCount(KVH), String(expected), `paste with anchor ${anchor}`));
      }
      return { pasted: expected };
    });

    await step("clipboard: copying an uncurated Pool row pastes one unit", async () => {
      await clickRow(GLULAM);
      await run(WINDOWS.pool, "copy");
      await form(WINDOWS.pool, "paste", { anchor: 0 });
      await eventually(async () => assert.equal(await curatedCount(GLULAM), "1"));
      return {};
    });

    await step("counts: typed entry in the Pool commits, zero removes, negative and invalid are refused, a large entry clamps", async () => {
      await fresh();
      await setPool(KVH, 5);
      assert.equal(await curatedCount(KVH), "5");
      await type(pool(), KVH, "-3");
      await eventually(async () => assert.equal(await poolCount(KVH), "5"));
      await type(pool(), KVH, "abc");
      await eventually(async () => assert.equal(await poolCount(KVH), "5"));
      await type(pool(), KVH, "99999");
      await eventually(async () => assert.equal(await poolCount(KVH), "60"));
      await type(pool(), KVH, "0");
      await eventually(async () => { assert.equal(await poolCount(KVH), "0"); assert.ok(!(await curatedNames()).includes(KVH)); });
      return {};
    });

    await step("counts: typed entry in Curated, Decrease and Increase buttons, bounds disable the buttons", async () => {
      await setPool(IPE, 2);
      await type(curated(), IPE, "4");
      await eventually(async () => assert.equal(await poolCount(IPE), "4"));
      await type(curated(), IPE, "-1");
      await eventually(async () => assert.equal(await curatedCount(IPE), "4"));
      await stepper(curated(), IPE, "decrement").click();
      await eventually(async () => assert.equal(await poolCount(IPE), "3"));
      await stepper(pool(), IPE, "decrement").click();
      await eventually(async () => assert.equal(await curatedCount(IPE), "2"));
      await stepper(pool(), IPE, "increment").click();
      await eventually(async () => assert.equal(await curatedCount(IPE), "3"));
      await setPool(HEA, 8);
      await eventually(async () => assert.ok(await stepper(pool(), HEA, "increment").isDisabled(), "increment enabled at availability"));
      await type(curated(), IPE, "0");
      await eventually(async () => assert.ok(!(await curatedNames()).includes(IPE)));
      await eventually(async () => assert.ok(await stepper(pool(), IPE, "decrement").isDisabled(), "decrement enabled at zero"));
      return {};
    });

    await step("counts: the Curate row action adds one until the availability is reached", async () => {
      await setPool(HEA, 7);
      await rowIn(curated(), HEA).getByRole("button", { name: t.curate, exact: true }).click();
      await eventually(async () => assert.equal(await curatedCount(HEA), "8"));
      await eventually(async () => assert.equal(await rowIn(curated(), HEA).getByRole("button", { name: t.curate, exact: true }).count(), 0, "Curate offered at availability"));
      return {};
    });

    await step("row More menu: opens, Curate, Curate One Less and Remove act on the row", async () => {
      const more = (scope: Locator, name: string) => rowIn(scope, name).locator("[data-context-menu-trigger]");
      const labels = language === "en" ? { more: "Curate", less: "Curate One Less", remove: "Remove" } : { more: "Kuratieren", less: "Eine weniger kuratieren", remove: "Entfernen" };
      const item = (label: string) => page.getByRole("menuitem", { name: label, exact: true });
      await setPool(KVH, 2);
      await more(pool(), KVH).click();
      await eventually(async () => assert.equal(await item(labels.more).count() + await item(labels.less).count() + await item(labels.remove).count(), 3, "menu items"));
      await item(labels.more).click();
      await eventually(async () => assert.equal(await curatedCount(KVH), "3"));
      await more(curated(), KVH).click();
      await item(labels.less).click();
      await eventually(async () => assert.equal(await poolCount(KVH), "2"));
      await more(pool(), KVH).click();
      await item(labels.remove).click();
      await eventually(async () => assert.ok(!(await curatedNames()).includes(KVH)));
      await more(pool(), KVH).click();
      await eventually(async () => assert.equal(await item(labels.less).count() + await item(labels.remove).count(), 0, "uncurated row offers only Curate"));
      await page.keyboard.press("Escape");
      return {};
    });

    await step("Stock From Catalogue: the action and the Restock button keep rows and curated counts", async () => {
      await setPool(KVH, 3);
      const n = await total();
      await run(WINDOWS.pool, "stockFromCatalogue");
      await eventually(async () => { assert.equal(await total(), n); assert.equal(await poolCount(KVH), "3"); });
      await page.locator('[id="window:sourcing-pool/sourcing-pool-restock"]').click();
      await settle(2500);
      await eventually(async () => { assert.equal(await total(), n); assert.equal(await poolCount(KVH), "3"); assert.equal(await curatedCount(KVH), "3"); });
      return { rows: n };
    });

    await step("Curation Set Count…: value, delta, negative value and unknown object", async () => {
      const set = (args: Record<string, string | number>) => form(WINDOWS.curated, "curationSetCount", args);
      await set({ objectId: KVH_ID, value: 7 });
      await eventually(async () => assert.equal(await curatedCount(KVH), "7"));
      await set({ objectId: KVH_ID, delta: -2 });
      await eventually(async () => assert.equal(await curatedCount(KVH), "5"));
      await set({ objectId: KVH_ID, value: -1 });
      await settle(1200);
      assert.equal(await curatedCount(KVH), "5", "a negative value must be refused");
      await set({ objectId: "no-such-kind", value: 4 });
      await settle(1200);
      assert.equal((await curatedNames()).length, 1);
      await set({ objectId: KVH_ID, value: 0 });
      await eventually(async () => assert.ok(!(await curatedNames()).includes(KVH)));
      return {};
    });

    await step("Set Active Example…: Demo reloads the catalogue and empties the curation", async () => {
      await setPool(KVH, 2);
      await form(WINDOWS.pool, "setActiveExample", { exampleId: 0 });
      await eventually(async () => { assert.equal(await curatedCount(KVH), "absent"); assert.ok((await names(pool())).includes(GLULAM)); }, 15_000);
      return {};
    });

    await step("Set Grid Instance Display…: all three options redraw the Grid canvas", async () => {
      await setPool(KVH, 3);
      await setPool(IPE, 2);
      await settle(2500);
      const hashes: Record<string, string> = {};
      for (const [index, key] of ["line", "representative", "count"].entries()) {
        await form(WINDOWS.grid, "setGridInstanceDisplay", { value: index });
        await settle(2500);
        hashes[key] = await canvasHash();
      }
      assert.equal(new Set(Object.values(hashes)).size, 3, `canvas hashes ${JSON.stringify(hashes)}`);
      return { hashes };
    });

    await step("setContributions: the installed module packs show as module filters", async () => {
      const toggles = await page.locator('[id^="window:sourcing-pool/sourcing-filter-module-"]').evaluateAll(items => items.map(item => item.id.replace(/^.*module-/u, "")));
      for (const module of ["beams", "windows", "slabs", "reuse"]) assert.ok(toggles.includes(module), `module ${module} missing from ${toggles}`);
      return { toggles };
    });

    await step("Note Shell Command and Record Tutorial are not user actions of the editor", async () => {
      await openActions(WINDOWS.pool);
      const ids = await page.evaluate(() => [...document.querySelectorAll('[id^="action."]')].map(item => item.id));
      await closeActions(WINDOWS.pool);
      assert.ok(!ids.includes("action.noteShellCommand") && !ids.includes("action.recordTutorial"), "palette exposes shell-only verbs");
      return { palette: ids.length };
    });
  } finally {
    await s.shot("final").catch(() => undefined);
    await writeFile(`${out}/editing-console-${language}.txt`, `${s.faults.join("\n")}\n`);
    await s.context.close();
  }
}

await runJourneys("editing", journey);
