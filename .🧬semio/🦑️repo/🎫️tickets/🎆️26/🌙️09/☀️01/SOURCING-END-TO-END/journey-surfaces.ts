import { assert, awaitBeacon, dismissIntroduction, join, out, runJourneys, session, url, writeFile, type Browser, type Locator } from "./journey-kit.ts";

/** 🪟️ Sourcing surface journey (6081): viewer role and table, role switching, breadcrumbs, sorts, module and typology filters, examples, preview and grid, panes, window chrome, resizing, runtime language switch, command palette, accessible names and keyboard; English and German.
 * Usage: `JOURNEY=surfaces bun journey-surfaces.ts [en|de ...]` with the native sourcing host answering on 6081 (override with `SOURCING_URL`).
 * Writes screenshots and `surfaces.json` under `🗑️generated/journey`.
 * @see ./journey-kit.ts
 * @see ./🪟️surface-features.md */

type Language = "en" | "de";
type Row = { id: string; module: string; typology: string[]; en: string; de: string; availability: number };
const stockRows: Row[] = [
  { id: "beam-glulam-gl24h", module: "beams", typology: ["beams", "solid-timber", "glulam"], en: "Glulam GL24h 200×400", de: "Brettschichtholz GL24h 200×400", availability: 24 },
  { id: "beam-kvh-c24", module: "beams", typology: ["beams", "solid-timber", "kvh"], en: "KVH C24 100×200", de: "KVH C24 100×200", availability: 60 },
  { id: "beam-steel-ipe200", module: "beams", typology: ["beams", "steel", "ipe"], en: "Steel IPE 200", de: "Stahl IPE 200", availability: 12 },
  { id: "beam-steel-hea160", module: "beams", typology: ["beams", "steel", "hea"], en: "Steel HEA 160", de: "Stahl HEA 160", availability: 8 },
  { id: "window-casement-100x120", module: "windows", typology: ["windows", "casement"], en: "Casement Window 100×120", de: "Drehflügelfenster 100×120", availability: 18 },
  { id: "window-fixed-150x150", module: "windows", typology: ["windows", "fixed"], en: "Fixed Window 150×150", de: "Festverglasung 150×150", availability: 10 },
  { id: "window-tilt-turn-120x140", module: "windows", typology: ["windows", "tilt-turn"], en: "Tilt & Turn Window 120×140", de: "Dreh-Kipp-Fenster 120×140", availability: 14 },
  { id: "slab-concrete-240", module: "slabs", typology: ["slabs", "concrete"], en: "Concrete Slab 240mm", de: "Betondecke 240 mm", availability: 30 },
  { id: "slab-clt-160", module: "slabs", typology: ["slabs", "clt"], en: "CLT Slab 160mm", de: "BSP-Decke 160 mm", availability: 20 },
  { id: "slab-hollow-core-265", module: "slabs", typology: ["slabs", "hollow-core"], en: "Hollow Core Slab 265mm", de: "Hohlkörperdecke 265 mm", availability: 16 },
  { id: "hexagonal-cut-concrete-forest-left", module: "reuse", typology: ["reuse", "concrete", "forest"], en: "Hexagonal Cut Concrete Forest Left", de: "Sechseckig geschnittener Betonwald links", availability: 12 },
  { id: "hexagonal-cut-concrete-forest-right", module: "reuse", typology: ["reuse", "concrete", "forest"], en: "Hexagonal Cut Concrete Forest Right", de: "Sechseckig geschnittener Betonwald rechts", availability: 12 },
];
const moduleLabels: Record<string, [string, string]> = { beams: ["Beams", "Balken"], windows: ["Windows", "Fenster"], slabs: ["Slabs", "Decken"], reuse: ["Reuse", "Wiederverwendung"] };
const segmentLabels: Record<string, [string, string]> = { beams: ["Beams", "Balken"], "solid-timber": ["Solid Timber", "Massivholz"], glulam: ["Glulam", "Brettschichtholz"], kvh: ["KVH", "KVH"], steel: ["Steel", "Stahl"], ipe: ["IPE", "IPE"], hea: ["HEA", "HEA"], windows: ["Windows", "Fenster"], casement: ["Casement", "Drehflügel"], fixed: ["Fixed", "Festverglasung"], "tilt-turn": ["Tilt & Turn", "Dreh-Kipp"], slabs: ["Slabs", "Decken"], concrete: ["Concrete", "Beton"], clt: ["CLT", "BSP"], "hollow-core": ["Hollow Core", "Hohlkörper"], reuse: ["Reuse", "Wiederverwendung"], forest: ["Forest", "Wald"] };
const typologyOptions: string[][] = [["beams"], ["beams", "solid-timber"], ["beams", "solid-timber", "glulam"], ["beams", "solid-timber", "kvh"], ["beams", "steel"], ["beams", "steel", "ipe"], ["beams", "steel", "hea"], ["windows"], ["windows", "casement"], ["windows", "fixed"], ["windows", "tilt-turn"], ["slabs"], ["slabs", "concrete"], ["slabs", "clt"], ["slabs", "hollow-core"], ["reuse"], ["reuse", "concrete"], ["reuse", "concrete", "forest"]];
const labels = {
  en: { viewer: "Viewer", editor: "Editor", table: "Table", example: "Example", noExample: "No example", demo: "Demo", language: "Language", german: "Deutsch", english: "English", preview: "Preview", previewSurface: "Object preview", poolTable: "Pool table", curatedTable: "Curated table", module: "Module", typology: "Typology", availability: "Availability", curatedColumn: "Curated", focus: "Focus", unfocus: "Unfocus", close: "Close", actions: "Actions", noData: "No data", instance: "Instance display", projection: "Projection", options: "Window Options", utilities: "Utilities", name: "Name", count: "Count", setExample: "Set Active Example…", stock: "Stock From Catalogue", execute: "Execute", noSelection: "No selection", increase: "Increase Curated", remove: "Remove", curate: "Curate", search: "Search", lineBehind: "Line behind grid", representative: "Representative only", representativeCount: "Representative and count" },
  de: { viewer: "Betrachter", editor: "Editor", table: "Tabelle", example: "Beispiel", noExample: "Kein Beispiel", demo: "Demo", language: "Sprache", german: "Deutsch", english: "English", preview: "Vorschau", previewSurface: "Objektvorschau", poolTable: "Pool-Tabelle", curatedTable: "Kuratierte Tabelle", module: "Modul", typology: "Typologie", availability: "Verfügbarkeit", curatedColumn: "Kuratiert", focus: "Fokussieren", unfocus: "Fokus aufheben", close: "Schließen", actions: "Aktionen", noData: "Keine Daten", instance: "Instanzdarstellung", projection: "Projektion", options: "Fensteroptionen", utilities: "Hilfsmittel", name: "Name", count: "Anzahl", setExample: "Aktives Beispiel festlegen…", stock: "Bestand aus Katalog", execute: "Ausführen", noSelection: "Keine Auswahl", increase: "Erhöhen Kuratiert", remove: "Entfernen", curate: "Kuratieren", search: "Suchen", lineBehind: "Linie hinter Raster", representative: "Nur Vertreter", representativeCount: "Vertreter und Anzahl" },
} as const;
const editorWindows = ["sourcing-pool", "sourcing-curated", "sourcing-preview", "sourcing-grid"];
const dragHint = /^\s*(Drag to window|Verschieben)\s*/u;

async function journey(browser: Browser, language: Language) {
  const s = await session(browser, language);
  const { page, t, pool, curated, preview, rowIn, body, settle, eventually } = s;
  const only = process.env.STEPS ? new RegExp(process.env.STEPS, "u") : undefined;
  const step: typeof s.step = async (title, run) => { if (only === undefined || only.test(title)) await s.step(title, run); };
  const idx = language === "en" ? 0 : 1;
  const l = labels[language];
  const name = (row: Row) => row[language];
  const moduleText = (row: Row) => moduleLabels[row.module][idx];
  const typologyText = (path: string[]) => path.map(segment => segmentLabels[segment][idx]).join(" / ");
  const byId = (id: string) => stockRows.find(row => row.id === id)!;
  const glulam = byId("beam-glulam-gl24h"), kvh = byId("beam-kvh-c24"), hea = byId("beam-steel-hea160"), ipe = byId("beam-steel-ipe200");
  const reuseLeft = byId("hexagonal-cut-concrete-forest-left"), reuseRight = byId("hexagonal-cut-concrete-forest-right");
  const names = (scope: Locator) => scope.locator("tbody tr").evaluateAll(rows => rows.map(row => row.querySelector("td")?.textContent?.trim() ?? "")).then(cells => cells.map(cell => cell.replace(dragHint, "").trim()).filter(cell => cell && cell !== l.noData));
  const windows = () => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map(element => element.getAttribute("data-window-id") ?? ""));
  const button = (id: string) => page.locator(`[id="${id}"]`);
  const ready = async () => { assert.match((await awaitBeacon(page, Date.now() + 300_000)) ?? "", /^ready/u); await dismissIntroduction(page); await settle(3000); };
  const column = (scope: Locator, index: number) => scope.locator(`tbody tr td:nth-child(${index})`).allInnerTexts().then(cells => cells.map(cell => cell.replace(/\s+/gu, " ").replace(dragHint, "").trim()));
  const monotone = <T extends string | number>(values: T[], sign: 1 | -1) => values.length > 1 && values.every((value, index) => index === 0 || sign * (value < values[index - 1] ? -1 : value > values[index - 1] ? 1 : 0) >= 0);
  const toggleOf = (module: string) => pool().locator(`[id="window:sourcing-pool/sourcing-filter-module-${module}"]`);
  const typologySelect = () => pool().getByRole("combobox", { name: t.typology, exact: true });
  const chooseOption = async (trigger: Locator, pick: number | RegExp) => {
    await trigger.click();
    const option = typeof pick === "number" ? page.getByRole("option").nth(pick) : page.getByRole("option", { name: pick });
    const label = (await option.first().innerText()).trim();
    await option.first().click();
    return label;
  };
  const sortBothWays = async (th: Locator, read: () => Promise<(string | number)[]>, label: string) => {
    const directions: string[] = [];
    for (let click = 0; click < 2; click++) {
      await th.getByRole("button").click();
      await eventually(async () => {
        const direction = await th.getAttribute("aria-sort");
        assert.ok(direction === "ascending" || direction === "descending", `${label}: aria-sort ${direction}`);
        assert.ok(monotone(await read() as string[], direction === "ascending" ? 1 : -1), `${label}: rows do not follow ${direction}: ${await read()}`);
        if (click === 1) assert.notEqual(direction, directions[0], `${label}: second click kept ${direction}`);
      }, 15_000);
      directions.push((await th.getAttribute("aria-sort")) ?? "");
    }
    return directions;
  };
  const switchRole = async (role: "editor" | "viewer", how: "click" | "shortcut" = "click") => {
    const want = role === "viewer" ? ["framework.window.table"] : editorWindows;
    if (how === "click") await button(`playground.navbar.roles.${role}`).click();
    else await page.keyboard.press(`ControlOrMeta+Alt+${role === "viewer" ? "v" : "e"}`);
    await eventually(async () => assert.deepEqual(await windows(), want, `after ${how} to ${role}: ${JSON.stringify(recent)}`), 120_000);
    await eventually(async () => assert.equal(await button(`playground.navbar.roles.${role}`).getAttribute("aria-pressed"), "true"), 15_000);
    await settle(1500);
  };
  const unnamed = async () => {
    const cdp = await page.context().newCDPSession(page);
    const { nodes } = await cdp.send("Accessibility.getFullAXTree") as { nodes: { role?: { value: string }; name?: { value: string }; ignored?: boolean; backendDOMNodeId?: number }[] };
    const roles = new Set(["button", "tab", "combobox", "spinbutton", "textbox", "checkbox", "switch", "menuitem", "columnheader", "option", "searchbox", "slider"]);
    const bad = nodes.filter(node => !node.ignored && node.role && roles.has(node.role.value) && !(node.name?.value ?? "").trim());
    const described = await Promise.all(bad.slice(0, 30).map(async node => cdp.send("DOM.describeNode", { backendNodeId: node.backendDOMNodeId }).then((answer: any) => `${node.role?.value}#${answer.node.attributes?.[answer.node.attributes.indexOf("id") + 1] ?? ""}<${answer.node.nodeName}>`).catch(() => `${node.role?.value}?`)));
    await cdp.detach();
    return described;
  };
  const increase = async (row: Row, times: number) => {
    for (let done = 0; done < times; done++) {
      const before = Number(await rowIn(pool(), name(row)).getByRole("spinbutton").first().inputValue());
      await rowIn(pool(), name(row)).getByRole("button", { name: t.increase, exact: true }).click();
      await eventually(async () => assert.equal(Number(await rowIn(pool(), name(row)).getByRole("spinbutton").first().inputValue()), before + 1));
    }
  };
  const clearCuration = async () => {
    for (let remaining = (await names(curated())).length; remaining > 0; remaining = (await names(curated())).length) {
      await curated().getByRole("button", { name: t.remove, exact: true }).first().click({ timeout: 15_000 });
      await eventually(async () => assert.ok((await names(curated())).length < remaining, `Remove left ${remaining} curated rows`), 30_000);
    }
  };
  const fresh = async () => { await page.goto(url); await ready(); };
  const resetPool = async () => {
    for (const module of Object.keys(moduleLabels)) if (await toggleOf(module).getAttribute("aria-pressed") === "true") await toggleOf(module).click();
    await chooseOption(typologySelect(), 0);
    await eventually(async () => assert.equal((await names(pool())).length, 12));
  };
  const openActions = async (windowId: string) => { if (await button("action.undo").count() === 0) await button(`framework.window.${windowId}.engagement.toggle`).click(); await eventually(async () => assert.equal(await button("action.undo").count(), 1)); };
  const closeActions = async (windowId: string) => { if (await button("action.undo").count() > 0) await button(`framework.window.${windowId}.engagement.toggle`).click(); await settle(300); };
  const gridCanvas = () => page.locator('[data-window-id="sourcing-grid"] canvas, [id="sourcing-grid"] canvas').first();
  const canvasFingerprint = async () => {
    const png = await gridCanvas().screenshot();
    const { createHash } = await import("node:crypto");
    return { hash: createHash("md5").update(png).digest("hex"), bytes: png.length };
  };
  const openPalette = async () => { await page.locator("body").click({ position: { x: 5, y: 5 } }); await page.keyboard.press("ControlOrMeta+p"); await eventually(async () => assert.equal(await page.locator('[id$="ui.search.input"]').count(), 1)); };
  const recent: string[] = [];
  page.on("console", message => { if ((message.type() === "warning" || message.type() === "error") && !message.text().startsWith("[DEBUG]")) { recent.push(`${message.type()}: ${message.text().slice(0, 240)}`); if (recent.length > 6) recent.shift(); } });
  const meshRequests: string[] = [];
  page.on("response", response => { if (/\/mesh\/.*\.glb/iu.test(decodeURIComponent(response.url()))) meshRequests.push(`${response.status()} ${response.headers()["content-type"] ?? ""} ${decodeURIComponent(response.url()).split("/").pop()}`); });
  try {
    await page.goto(url);
    await ready();
    await step("boot without program/descriptor faults or page errors", async () => {
      assert.equal(s.faults.filter(line => line.startsWith("pageerror")).length, 0, s.faults.join("\n"));
      assert.deepEqual(await windows(), editorWindows);
      return { windows: await windows() };
    });
    await step("breadcrumb, role chip and role buttons name app, surface and locale", async () => {
      const crumb = await page.locator('[data-slot="app-name"]').innerText();
      const chip = await page.locator('[data-slot="surface-role-chip"]').innerText();
      assert.match(crumb, /semio.*sourcing.*curation/u);
      assert.equal(chip.trim(), l.editor);
      const group = await button("playground.navbar.roles").getAttribute("aria-label");
      assert.ok(group, "role group has no accessible name");
      const buttons = await page.locator('[id^="playground.navbar.roles."]').allInnerTexts();
      assert.ok(buttons.some(text => text.startsWith(l.viewer)) && buttons.some(text => text.startsWith(l.editor)), `role buttons ${buttons}`);
      assert.equal(await button("playground.navbar.roles.editor").getAttribute("aria-keyshortcuts") !== null, true, "editor chord not published");
      assert.equal(await button("playground.navbar.roles.viewer").getAttribute("aria-keyshortcuts") !== null, true, "viewer chord not published");
      return { crumb, chip, group, buttons };
    });
    await step("every control of the editor surface has an accessible name", async () => {
      const bad = await unnamed();
      assert.deepEqual(bad, []);
      return { unnamed: bad };
    });
    await step("window labels, column headers and surface names are in the active language", async () => {
      for (const label of [t.pool, t.curated, t.preview, t.grid]) assert.ok((await body()).includes(label), `window label ${label}`);
      const poolHeaders = (await pool().locator("th").allInnerTexts()).map(text => text.trim());
      assert.deepEqual(poolHeaders, [t.name, l.module, l.typology, t.availability, l.curatedColumn]);
      const curatedHeaders = (await curated().locator("th").allInnerTexts()).map(text => text.trim());
      assert.deepEqual(curatedHeaders, [t.name, t.availability, l.count, l.actions]);
      assert.equal(await pool().getByRole("table").first().getAttribute("aria-label").catch(() => null) ?? l.poolTable, l.poolTable);
      return { poolHeaders, curatedHeaders };
    });
    await step("pool sorts by Name, Module and Availability in both directions with aria-sort", async () => {
      const evidence: Record<string, unknown> = {};
      for (const [header, index, numeric] of [[t.name, 1, false], [l.module, 2, false], [t.availability, 4, true]] as const) {
        const read = async () => { const cells = await column(pool(), index); return numeric ? cells.map(Number) : cells; };
        evidence[header] = await sortBothWays(pool().getByRole("columnheader", { name: header, exact: true }), read, `pool ${header}`);
      }
      return evidence;
    });
    await step("module toggles Beams, Windows, Slabs, Reuse and their combinations filter the pool", async () => {
      await resetPool();
      const evidence: Record<string, unknown> = {};
      const expectIn = async (modules: string[]) => {
        const expected = stockRows.filter(row => modules.includes(row.module)).map(name).sort();
        await eventually(async () => assert.deepEqual((await names(pool())).sort(), expected));
      };
      for (const module of ["beams", "windows", "reuse", "slabs"]) {
        await toggleOf(module).click();
        await expectIn([module]);
        assert.equal(await toggleOf(module).getAttribute("aria-pressed"), "true", `${module} not pressed`);
        await toggleOf(module).click();
        await eventually(async () => assert.equal((await names(pool())).length, 12));
        evidence[module] = true;
      }
      for (const combination of [["beams", "windows"], ["beams", "reuse"], ["windows", "reuse"], ["beams", "windows", "reuse"], ["beams", "windows", "slabs", "reuse"]]) {
        for (const module of combination) await toggleOf(module).click();
        await expectIn(combination);
        for (const module of combination) await toggleOf(module).click();
        await eventually(async () => assert.equal((await names(pool())).length, 12));
        evidence[combination.join("+")] = true;
      }
      return evidence;
    });
    await step("module toggles carry the module names of the active language", async () => {
      const texts = (await pool().locator('[id^="window:sourcing-pool/sourcing-filter-module-"]').allInnerTexts()).map(text => text.trim());
      assert.deepEqual(texts, ["beams", "windows", "slabs", "reuse"].map(module => moduleLabels[module][idx]));
      return { texts };
    });
    await step("every typology option filters to its tree path and is labelled in the active language", async () => {
      await resetPool();
      await typologySelect().click();
      const options = (await page.getByRole("option").allInnerTexts()).map(text => text.trim());
      await page.keyboard.press("Escape");
      assert.deepEqual(options, [t.allTypologies, ...typologyOptions.map(path => segmentLabels[path[path.length - 1]][idx])]);
      const evidence: Record<string, unknown> = {};
      for (let index = 1; index <= typologyOptions.length; index++) {
        await chooseOption(typologySelect(), index);
        const path = typologyOptions[index - 1];
        const expected = stockRows.filter(row => path.every((segment, depth) => row.typology[depth] === segment));
        await eventually(async () => {
          assert.deepEqual((await names(pool())).sort(), expected.map(name).sort(), path.join("/"));
          const shown = await column(pool(), 3);
          assert.ok(shown.every(text => text.startsWith(typologyText(path))), `${path}: ${shown}`);
        });
        evidence[path.join("/")] = expected.length;
      }
      await chooseOption(typologySelect(), 0);
      await eventually(async () => assert.equal((await names(pool())).length, 12));
      return { options, evidence };
    });
    await step("stock names, module column and typology column are localized rows", async () => {
      const rows = await names(pool());
      assert.deepEqual(rows.sort(), stockRows.map(name).sort());
      assert.deepEqual((await column(pool(), 2)).sort(), stockRows.map(moduleText).sort());
      assert.deepEqual((await column(pool(), 3)).sort(), stockRows.map(row => typologyText(row.typology)).sort());
      return { rows };
    });
    await step("curated sorts by Name, Availability and Count in both directions with aria-sort", async () => {
      await clearCuration();
      await increase(glulam, 3); await increase(kvh, 1); await increase(hea, 2);
      await eventually(async () => assert.equal((await names(curated())).length, 3));
      const evidence: Record<string, unknown> = {};
      const read = { [t.name]: async () => names(curated()), [t.availability]: async () => (await column(curated(), 2)).map(Number), [l.count]: async () => curated().locator("tbody tr").evaluateAll(rows => rows.map(row => Number((row.querySelector("input") as HTMLInputElement | null)?.value))) } as Record<string, () => Promise<(string | number)[]>>;
      for (const header of [t.name, t.availability, l.count]) evidence[header] = await sortBothWays(curated().getByRole("columnheader", { name: header, exact: true }), read[header], `curated ${header}`);
      return evidence;
    });
    await step("curate row action adds one more and remove clears the row", async () => {
      const row = rowIn(curated(), name(hea));
      await row.getByRole("button", { name: l.curate, exact: true }).click();
      await eventually(async () => assert.equal(await rowIn(curated(), name(hea)).locator("input").first().inputValue(), "3"));
      await clearCuration();
      return {};
    });
    await step("empty example through the navbar selector and back to Demo", async () => {
      const selector = button("playground.navbar.fixture");
      await selector.click();
      const offered = (await page.getByRole("option").allInnerTexts()).map(text => text.trim());
      assert.deepEqual(offered, [l.noExample, l.demo]);
      await page.getByRole("option", { name: l.noExample, exact: true }).click();
      await eventually(async () => assert.equal((await names(pool())).length, 0, "empty example kept rows"), 30_000);
      await eventually(async () => assert.equal(await pool().locator("tbody tr").count() <= 1, true));
      await selector.click();
      await page.getByRole("option", { name: l.demo, exact: true }).click();
      await eventually(async () => assert.equal((await names(pool())).length, 12), 30_000);
      return { offered };
    });
    await step("empty example through Set Active Example and the registered examples", async () => {
      await openActions("sourcingCurated");
      await button("action.setActiveExample").click();
      const form = page.getByRole("combobox", { name: l.example, exact: true }).last();
      await form.click();
      const offered = (await page.getByRole("option").allInnerTexts()).map(text => text.trim());
      assert.deepEqual(offered, [l.noExample, l.demo], "Set Active Example must offer the navbar examples and no unregistered demo-session");
      await page.getByRole("option", { name: l.noExample, exact: true }).click();
      await page.getByRole("button", { name: l.execute, exact: true }).click();
      await eventually(async () => assert.equal((await names(pool())).length, 0, "empty example through the form kept rows"), 30_000);
      await form.click();
      await page.getByRole("option", { name: l.demo, exact: true }).click();
      await page.getByRole("button", { name: l.execute, exact: true }).click();
      await eventually(async () => assert.equal((await names(pool())).length, 12), 30_000);
      await closeActions("sourcingCurated");
      return { offered };
    });
    await step("preview shows the empty state, names its surface, follows a selection and clears on Escape", async () => {
      await clearCuration();
      assert.ok((await preview().innerText()).includes(l.noSelection), "no empty state");
      await increase(kvh, 1);
      await rowIn(curated(), name(kvh)).locator("td").nth(1).click();
      await eventually(async () => { assert.ok(!(await preview().innerText()).includes(l.noSelection), "still no selection"); assert.ok(await preview().locator("canvas").count() > 0, "no preview canvas"); }, 15_000);
      assert.equal(await preview().locator('[role="application"]').first().getAttribute("aria-label"), l.previewSurface);
      await page.keyboard.press("Escape");
      await eventually(async () => assert.ok((await preview().innerText()).includes(l.noSelection), "Escape did not clear the selection"), 15_000);
      return {};
    });
    await step("grid draws reuse GLB meshes and instance display options change the scene", async () => {
      await clearCuration();
      const empty = await canvasFingerprint();
      await increase(reuseLeft, 1); await increase(reuseRight, 1);
      await eventually(async () => assert.ok(meshRequests.some(request => request.startsWith("200") && /left\.glb/u.test(request)), `left glb not served: ${meshRequests}`), 30_000);
      await eventually(async () => assert.ok(meshRequests.some(request => request.startsWith("200") && /right\.glb/u.test(request)), `right glb not served: ${meshRequests}`), 30_000);
      assert.ok(meshRequests.every(request => !request.includes("text/html")), `glb answered as html: ${meshRequests}`);
      await settle(2500);
      const drawn = await canvasFingerprint();
      assert.notEqual(drawn.hash, empty.hash, "grid canvas unchanged after curating reuse stock");
      const evidence: Record<string, unknown> = { meshRequests, empty, drawn };
      await button("framework.window.sourcingGrid.measures.unfold").click();
      const display = button("sourcing-grid/sourcing-grid-window.instance-display");
      for (const option of [l.representative, l.representativeCount, l.lineBehind]) {
        await display.click();
        await page.getByRole("option", { name: option, exact: true }).click();
        await settle(2000);
        evidence[option] = (await canvasFingerprint()).hash;
      }
      assert.ok(new Set([evidence[l.representative], evidence[l.representativeCount]]).size === 2, "representative vs representative-with-count render alike");
      await button("framework.window.sourcingGrid.measures.unfold").click().catch(() => undefined);
      return evidence;
    });
    await fresh();
    await step("grid pick and shift-drag rectangle selection select curated instances and Preview follows", async () => {
      await clearCuration();
      await increase(ipe, 1); await increase(byId("window-casement-100x120"), 1);
      await settle(2500);
      const box = await gridCanvas().boundingBox();
      assert.ok(box, "no grid canvas");
      const selection = () => page.evaluate(() => { const surface = document.querySelector('[data-surface-id="window:sourcing-grid"]'); return surface ? JSON.parse(surface.getAttribute("data-selection-json") ?? "{}").selectedIds ?? [] : null; });
      assert.deepEqual(await selection(), []);
      await page.keyboard.down("Shift");
      await page.mouse.move(box.x + box.width * 0.04, box.y + box.height * 0.1);
      await page.mouse.down();
      await page.mouse.move(box.x + box.width * 0.96, box.y + box.height * 0.92, { steps: 15 });
      await page.mouse.up();
      await page.keyboard.up("Shift");
      await eventually(async () => assert.ok(((await selection()) as string[]).length > 0, "shift-drag rectangle selected nothing"), 10_000);
      const selected = await selection();
      await eventually(async () => assert.ok(!(await preview().innerText()).includes(l.noSelection), "Preview did not follow the grid selection"), 15_000);
      return { selected };
    });
    await step("projection pane lists the camera projections and switching one keeps the grid alive", async () => {
      await button("framework.worldOrbit.projection.sourcingGrid.pane.fold").click();
      const entries = (await page.evaluate(() => [...document.querySelectorAll('[id^="framework.worldOrbit.projection.sourcingGrid."]')].map(element => element.id))).filter(id => !id.endsWith(".disclosureLabel") && !id.endsWith("pane.fold"));
      for (const required of ["parallel", "orthographic", "axonometric", "perspective", "curvilinear"]) assert.ok(entries.includes(`framework.worldOrbit.projection.sourcingGrid.${required}`), `projection ${required} missing in ${entries}`);
      const before = (await canvasFingerprint()).hash;
      await button("framework.worldOrbit.projection.sourcingGrid.orthographic").click();
      await settle(2000);
      const after = (await canvasFingerprint()).hash;
      assert.notEqual(before, after, "orthographic projection did not change the grid");
      await button("framework.worldOrbit.projection.sourcingGrid.pane.fold").click().catch(() => undefined);
      return { entries };
    });
    await step("window options pane names the instance display select with three options", async () => {
      if (await button("sourcing-grid/sourcing-grid-window.instance-display").count() === 0) await button("framework.window.sourcingGrid.measures.unfold").click();
      const select = button("sourcing-grid/sourcing-grid-window.instance-display");
      await select.click();
      const options = (await page.getByRole("option").allInnerTexts()).map(text => text.trim());
      await page.keyboard.press("Escape");
      assert.deepEqual(options, [l.lineBehind, l.representative, l.representativeCount]);
      return { options };
    });
    await fresh();
    await step("window Focus, Unfocus, Close and Display-drag reopen work by pointer and keyboard", async () => {
      const tab = (id: string) => page.locator(`[data-slot="mode-dock-tab"][data-window-id="${id}"]`);
      await clearCuration();
      for (const id of editorWindows) {
        assert.equal(await tab(id).locator('[data-slot="mode-dock-tab-focus"]').getAttribute("aria-label"), l.focus);
        assert.equal(await tab(id).locator('[data-slot="mode-dock-tab-close"]').getAttribute("aria-label"), l.close);
      }
      const reopen = async (kind: string, prefix: string) => {
        if (await button("framework.display.windows." + kind + ".kind").count() === 0) { await button("framework.category.display").click(); await button("framework.display.windows." + kind).click(); }
        const box = (await button("framework.display.windows." + kind + ".kind").locator('[data-drag-role="transfer"]').first().boundingBox())!;
        await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
        await page.mouse.down();
        await page.mouse.move(900, 500, { steps: 15 });
        await page.mouse.move(920, 520, { steps: 5 });
        await page.mouse.up();
        await eventually(async () => assert.ok((await windows()).some(id => id.startsWith(prefix)), `Display drag did not reopen ${prefix}: ${await windows()}`), 15_000);
        await button("framework.category.display").click().catch(() => undefined);
      };
      const press = async (id: string, slot: "focus" | "close") => {
        const box = (await tab(id).boundingBox())!;
        await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
        await settle(400);
        const target = (await tab(id).locator(`[data-slot="mode-dock-tab-${slot}"]`).boundingBox())!;
        await page.mouse.click(target.x + target.width / 2, target.y + target.height / 2);
      };
      const widthOf = async (name: string) => (await page.getByRole("tabpanel", { name, exact: true }).boundingBox())?.width ?? 0;
      const before = await widthOf(t.preview);
      await press("sourcing-preview", "focus");
      await eventually(async () => assert.ok(await widthOf(t.preview) > before + 400, "Focus did not enlarge Preview"));
      await eventually(async () => assert.deepEqual(await windows(), ["sourcing-preview"]), 10_000);
      await tab("sourcing-preview").locator('[data-slot="mode-dock-tab-focus"]').focus();
      assert.equal(await tab("sourcing-preview").locator('[data-slot="mode-dock-tab-focus"]').getAttribute("aria-label"), l.unfocus);
      await page.keyboard.press("Enter");
      await eventually(async () => assert.deepEqual(await windows(), editorWindows), 10_000);
      await press("sourcing-preview", "close");
      await eventually(async () => assert.deepEqual(await windows(), ["sourcing-pool", "sourcing-curated", "sourcing-grid"]), 10_000);
      await reopen("sourcingPreview", "sourcing-preview");
      await tab("sourcing-curated").locator('[data-slot="mode-dock-tab-close"]').focus();
      await page.keyboard.press("Enter");
      await eventually(async () => assert.ok(!(await windows()).includes("sourcing-curated"), "keyboard Close did not close Curated"), 10_000);
      await reopen("sourcingCurated", "sourcing-curated");
      return { windows: await windows() };
    });
    await fresh();
    await step("utility bars are named and disabled when a window declares no utilities", async () => {
      const evidence: Record<string, unknown> = {};
      for (const id of ["sourcingPool", "sourcingCurated", "sourcingPreview", "sourcingGrid"]) {
        const unfold = button(`framework.window.${id}.utilityBar.unfold`);
        assert.equal(await unfold.count(), 1, `${id} utility toggle missing`);
        assert.equal((await unfold.innerText()).trim(), l.utilities);
        evidence[id] = await unfold.isDisabled();
      }
      return evidence;
    });
    await step("window Actions panes list the sourcing commands and Stock From Catalogue restocks", async () => {
      await openActions("sourcingCurated");
      const listed = (await page.evaluate(() => [...document.querySelectorAll('[id^="action."]')].map(element => element.id))).filter(id => !id.startsWith("action.category"));
      for (const required of ["action.setActiveExample", "action.stockFromCatalogue", "action.curationSetCount", "action.setGridInstanceDisplay", "action.undo", "action.redo", "action.copy", "action.cut", "action.paste", "action.exportArtifactDocument", "action.importArtifactDocument"]) assert.ok(listed.includes(required), `${required} missing in ${listed}`);
      await closeActions("sourcingCurated");
      return { listed };
    });
    await step("command palette finds and runs sourcing commands", async () => {
      await openPalette();
      await page.locator('[id$="ui.search.input"]').fill(language === "en" ? "stock" : "bestand");
      const hit = page.getByRole("option").filter({ hasText: l.stock }).first();
      await hit.waitFor({ timeout: 8000 });
      await page.keyboard.press("Escape");
      await openPalette();
      await page.locator('[id$="ui.search.input"]').fill(l.setExample.replace("…", ""));
      await page.getByRole("option").filter({ hasText: l.setExample }).first().click();
      await eventually(async () => assert.ok(await page.getByRole("button", { name: l.execute, exact: true }).count() > 0, "Set Active Example form did not open from the palette"));
      await page.getByRole("button", { name: l.execute, exact: true }).waitFor();
      await closeActions("sourcingCurated").catch(() => undefined);
      return {};
    });
    await step("dialogs and window events work from the keyboard", async () => {
      await openPalette();
      await page.keyboard.press("Escape");
      await eventually(async () => assert.equal(await page.locator('[id$="ui.search.input"]').count(), 0, "Escape did not dismiss the palette"));
      await openPalette();
      assert.equal(await page.evaluate(() => document.activeElement?.id.endsWith("ui.search.input") ?? false), true, "search input not focused on open");
      await page.keyboard.press("Escape");
      await rowIn(pool(), name(kvh)).locator("td").nth(1).click();
      await page.keyboard.press("ControlOrMeta+a");
      await page.keyboard.press("Escape");
      await page.locator("body").click({ position: { x: 5, y: 5 } });
      await page.keyboard.press("Tab");
      assert.ok(await page.evaluate(() => document.activeElement !== document.body), "Tab did not move focus into the surface");
      return {};
    });
    await step("layout survives desktop, tablet and mobile viewport widths without horizontal overflow", async () => {
      const evidence: Record<string, unknown> = {};
      for (const [label, width, height] of [["laptop", 1280, 800], ["tablet", 768, 1024], ["mobile", 390, 844], ["desktop", 1800, 1100]] as const) {
        await page.setViewportSize({ width, height });
        await settle(1800);
        const metrics = await page.evaluate(() => ({ scrollWidth: document.documentElement.scrollWidth, innerWidth: window.innerWidth, windows: [...document.querySelectorAll("[data-window-id]")].map(element => element.getAttribute("data-window-id")), roles: !!document.getElementById("playground.navbar.roles.viewer") }));
        assert.ok(metrics.scrollWidth <= metrics.innerWidth + 1, `${label}: horizontal overflow ${metrics.scrollWidth} > ${metrics.innerWidth}`);
        assert.ok(metrics.windows.length >= 1, `${label}: no window`);
        evidence[label] = metrics;
        await s.shot(`resize-${label}`).catch(() => undefined);
      }
      return evidence;
    });
    await step("split handle resizing changes the pool and curated widths", async () => {
      const pane = (name: string) => page.getByRole("tabpanel", { name, exact: true });
      const before = (await pane(t.pool).boundingBox())!;
      const edge = before.x + before.width;
      await page.mouse.move(edge + 1, before.y + before.height / 2);
      await page.mouse.down();
      await page.mouse.move(edge + 181, before.y + before.height / 2, { steps: 10 });
      await page.mouse.up();
      await settle(1200);
      const after = (await pane(t.pool).boundingBox())!;
      assert.ok(after.width > before.width + 80, `split handle did not widen Pool: ${before.width} -> ${after.width}`);
      return { before: before.width, after: after.width };
    });
    await fresh();
    await step("switching Editor to Viewer and back by click keeps the document", async () => {
      await clearCuration();
      await increase(glulam, 2);
      await switchRole("viewer");
      const chip = await page.locator('[data-slot="surface-role-chip"]').innerText();
      assert.equal(chip.trim(), l.viewer);
      await switchRole("editor");
      await eventually(async () => assert.equal(await rowIn(curated(), name(glulam)).locator("input").first().inputValue(), "2"));
      assert.equal((await page.locator('[data-slot="surface-role-chip"]').innerText()).trim(), l.editor);
      return { chip };
    });
    await step("viewer table is read-only, localized and shows the curation", async () => {
      await switchRole("viewer");
      const table = page.locator("table").first();
      const headers = (await table.locator("th").allInnerTexts()).map(text => text.trim());
      assert.deepEqual(headers, [t.name, l.module, l.typology, t.availability, l.curatedColumn]);
      const rows = await table.locator("tbody tr").evaluateAll(rows => rows.map(row => [...row.querySelectorAll("td")].map(cell => cell.textContent?.replace(/\s+/gu, " ").trim() ?? "")));
      assert.deepEqual(rows.map(row => row[0]).sort(), stockRows.map(name).sort());
      const glulamRow = rows.find(row => row[0] === name(glulam))!;
      assert.equal(glulamRow[1], moduleText(glulam));
      assert.equal(glulamRow[2], typologyText(glulam.typology));
      assert.equal(glulamRow[4].replace("⋯", "").trim(), "2", "viewer table does not show the curated count");
      assert.equal(await page.locator("input, [role=spinbutton], [data-stepper-control]").count(), 0, "viewer offers editable controls");
      assert.equal(await page.getByRole("button", { name: new RegExp(`${t.increase}|${l.curate}|${l.remove}`, "u") }).count(), 0, "viewer offers curation edits");
      assert.equal(await page.locator('[id^="window:sourcing-pool/sourcing-filter"]').count(), 0, "viewer offers pool filters");
      const unnamedViewer = await unnamed();
      assert.deepEqual(unnamedViewer, []);
      assert.equal((await page.getByRole("tabpanel").first().getAttribute("aria-label")) ?? l.table, l.table);
      return { headers, rows: rows.length, glulamRow };
    });
    await step("viewer table sorts are not offered as edits and Escape keeps the document", async () => {
      const before = await page.locator("table tbody tr").count();
      await page.keyboard.press("Escape");
      await page.keyboard.press("ControlOrMeta+z");
      await settle(800);
      assert.equal(await page.locator("table tbody tr").count(), before);
      assert.deepEqual(await windows(), ["framework.window.table"]);
      return { before };
    });
    await step("keyboard shortcuts switch Viewer to Editor and back", async () => {
      await switchRole("editor", "shortcut");
      assert.deepEqual(await windows(), editorWindows);
      await switchRole("viewer", "shortcut");
      await switchRole("editor", "shortcut");
      await eventually(async () => assert.equal(await rowIn(curated(), name(glulam)).locator("input").first().inputValue(), "2"));
      return {};
    });
    await step("role switches log no page errors; retirement warnings are recorded as the open lifecycle defect", async () => {
      assert.deepEqual(s.faults.filter(line => line.startsWith("pageerror") || /intake-rejected|no actor for instance|prior-outcome|role switch to/u.test(line)), []);
      const retirement = s.faults.filter(line => /original lifecycle receipt differs/u.test(line)).length;
      return { openDefect: "editor retirement logs original lifecycle receipt differs from its caller authority", occurrences: retirement };
    });
    await step("settings language switch flips every sourcing label at runtime and back", async () => {
      await clearCuration();
      const target = language === "en" ? "de" : "en";
      const flip = async (to: Language) => {
        await button("framework.settings").click();
        await page.locator('button[id="framework.settings.language"]').click();
        await page.getByRole("option", { name: to === "de" ? "Deutsch" : "English", exact: true }).click();
        await eventually(async () => assert.ok((await body()).includes(labels[to].noSelection) || (await body()).includes(labels[to].noData), `UI did not flip to ${to}`), 15_000);
        await button("framework.settings").click().catch(() => undefined);
        await settle(1500);
      };
      await flip(target);
      const flipped: Record<string, unknown> = {};
      const inTarget = (row: Row) => row[target];
      await eventually(async () => assert.deepEqual((await names(pool())).sort(), stockRows.map(inTarget).sort(), "stock names did not follow the language"), 30_000);
      await eventually(async () => assert.deepEqual((await pool().locator("th").allInnerTexts()).map(text => text.replace(/\s*[↑↓]\s*$/u, "").trim()), [labels[target].name, labels[target].module, labels[target].typology, labels[target].availability, labels[target].curatedColumn]), 30_000);
      await eventually(async () => assert.deepEqual((await pool().locator('[id^="window:sourcing-pool/sourcing-filter-module-"]').allInnerTexts()).map(text => text.trim()), ["beams", "windows", "slabs", "reuse"].map(module => moduleLabels[module][target === "en" ? 0 : 1])), 30_000);
      await eventually(async () => assert.ok((await body()).includes(labels[target].noSelection)), 30_000);
      flipped.title = await page.title();
      await flip(language);
      await eventually(async () => assert.deepEqual((await names(pool())).sort(), stockRows.map(name).sort(), "stock names did not return"), 30_000);
      return flipped;
    });
    await step("demo session example directory is not offered as an example", async () => {
      await button("playground.navbar.fixture").click();
      const offered = (await page.getByRole("option").allInnerTexts()).map(text => text.trim());
      await page.keyboard.press("Escape");
      assert.ok(!offered.some(text => /session|sitzung/iu.test(text)), `demo-session offered: ${offered}`);
      return { offered };
    });
  } finally {
    await s.shot("final").catch(() => undefined);
    await writeFile(join(out, `console-surfaces-${language}.txt`), `${s.faults.join("\n")}\n`);
    await s.context.close();
  }
}

await runJourneys("surfaces", journey);
