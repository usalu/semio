import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { assert, awaitBeacon, dismissIntroduction, join, out, root, runJourneys, session, url, verdicts, type Browser } from "./journey-kit.ts";

/** ⏪️ Sourcing curation editor journey (6081) for HISTORY and IMPORT/EXPORT: checkpoint, checkout, alternatives, revert to command, the history edit chain with its finish dialog, command filter, undo/redo, archive export and import, folder sync with a second browser context; English and German.
 * Every step starts from a freshly reloaded editor so one refused feature cannot hide the next.
 * Usage: `JOURNEY=history [STEPS=<regex of step names>] bun journey-history.ts [en|de ...]` with the native sourcing host answering on 6081.
 * Writes screenshots, downloads and `history.json` under `🗑️generated/journey`.
 * @see ./journey-kit.ts
 * @see ./🕰️history-io-features.md */

const labels = {
  en: { backwards: "Backwards", edit: "Edit", withdraw: "Withdraw", attach: "Attach", detach: "Detach", panelNoise: /Panel|Example/u, current: /Current/u },
  de: { backwards: "Zurück bis hier", edit: "Bearbeiten", withdraw: "Zurückziehen", attach: "Verbinden", detach: "Trennen", panelNoise: /Panel|Beispiel|Fenster/u, current: /Aktuell/u },
} as const;

const historyPrefix = "panel:framework.panel.history/";
const checkpoints = (shown: string[]) => shown.filter(row => /Checkpoint/u.test(row)).length;
const kvh = "KVH C24 100×200";

async function journey(browser: Browser, language: "en" | "de") {
  const s = await session(browser, language);
  const { page, t, pool, curated, rowIn, names, settle, eventually, step: baseStep } = s;
  const l = labels[language];
  const ipe = language === "en" ? "Steel IPE 200" : "Stahl IPE 200";
  const historyOpen = () => page.locator(`[id='${historyPrefix}framework.history.actions']`).isVisible().catch(() => false);
  const openHistory = async () => { if (!(await historyOpen())) await page.locator("[id='framework.panel.history']").click(); await eventually(async () => assert.ok(await historyOpen(), "History panel not open")); };
  const rows = () => page.evaluate(prefix => [...document.querySelectorAll(`[id^='${prefix}'][role=treeitem]`)].map(element => (element as HTMLElement).innerText.replace(/\s+/gu, " ").trim()), historyPrefix);
  const rowIds = () => page.evaluate(prefix => [...document.querySelectorAll(`[id^='${prefix}'][role=treeitem]`)].map(element => element.id.slice(prefix.length)), historyPrefix);
  const curatedNames = async () => (await names(curated())).filter(name => name !== "No data" && name !== "Keine Daten");
  const add = async (kind: string) => { await rowIn(pool(), kind).getByRole("button", { name: t.increase, exact: true }).click(); await eventually(async () => assert.ok((await curatedNames()).includes(kind), `${kind} not curated`)); };
  const sameSet = (left: string[], right: string[]) => assert.deepEqual([...left].sort(), [...right].sort());
  const entry = (hint: RegExp) => page.locator(`[id^='${historyPrefix}framework.history.commands␟'],[id^='${historyPrefix}framework.history.entry.']`).filter({ hasText: hint }).first();
  const menuOpen = () => page.locator("[id='action.exportArtifactDocument']").isVisible().catch(() => false);
  const menu = async (actionId: string) => {
    if (!(await menuOpen())) await page.locator("[id='framework.window.sourcingPool.engagement.toggle']").click();
    const item = page.locator(`[id='action.${actionId}']`);
    await eventually(async () => assert.ok(await item.isVisible(), `menu item ${actionId} not visible`));
    await item.click();
    await settle(900);
  };
  const historyAction = (key: string) => page.locator(`[id='${historyPrefix}framework.history.${key}'] button`).first();
  const alternatives = () => page.evaluate(prefix => [...document.querySelectorAll(`[id^='${prefix}framework.history.alternative.']`)].map(element => ({ id: element.id.slice(`${prefix}framework.history.alternative.`.length), text: (element as HTMLElement).innerText.replace(/\s+/gu, " ").trim() })), historyPrefix);
  const load = async () => {
    await page.goto(url);
    assert.match((await awaitBeacon(page, Date.now() + 300_000)) ?? "", /^ready/u);
    await dismissIntroduction(page);
    await settle(3000);
    await openHistory();
  };
  const wanted = (name: string) => process.env.STEPS === undefined || new RegExp(process.env.STEPS, "iu").test(name);
  const step = async (name: string, seed: string[], run: () => Promise<Record<string, unknown>>) => {
    if (!wanted(name)) return;
    await baseStep(name, async () => {
      await load();
      for (const kind of seed) await add(kind);
      return run();
    });
    const last = verdicts.at(-1)!;
    console.log(`${last.pass ? "PASS" : "FAIL"} ${last.step}${last.pass ? "" : ` ${JSON.stringify(last.evidence).slice(0, 300)}`}`);
  };
  const mutationRow = (hint: string) => page.locator(`[id^='${historyPrefix}framework.history.mutation.']`).filter({ hasText: hint }).first();
  const beginEdit = async (hint: string) => {
    await entry(new RegExp(hint, "u")).locator("button").first().click();
    const row = mutationRow(hint);
    await eventually(async () => assert.ok(await row.isVisible(), "mutation row not shown"));
    await row.getByRole("button", { name: l.edit, exact: true }).click();
  };

  try {
    await step("history panel lists the actions and the recorded commands", [kvh, ipe], async () => {
      const ids = await rowIds();
      for (const key of ["undo", "redo", "commitCheckpoint", "createAlternative", "filter"]) assert.ok(ids.includes(`framework.history.${key}`), `missing ${key}`);
      await eventually(async () => assert.ok((await rows()).filter(row => /beam-kvh-c24|beam-steel-ipe200/u.test(row)).length >= 2, `no curation entries in ${await rows()}`));
      return { rows: await rows() };
    });
    await step("undo and redo via the Actions menu", [kvh, ipe], async () => {
      await menu("undo");
      await eventually(async () => assert.deepEqual(await curatedNames(), [kvh], "undo did not take the last row back"));
      await menu("redo");
      await eventually(async () => sameSet(await curatedNames(), [kvh, ipe]));
      return { curated: await curatedNames() };
    });
    await step("revert to command takes back only the newer commands", [kvh, ipe], async () => {
      await entry(/kvh/u).click({ button: "right" });
      await page.getByRole("menuitem", { name: new RegExp(l.backwards, "u") }).first().click();
      await eventually(async () => assert.deepEqual(await curatedNames(), [kvh], "revert kept the newer command"), 15_000);
      await menu("redo");
      await eventually(async () => sameSet(await curatedNames(), [kvh, ipe]));
      return { rows: await rows() };
    });
    await step("history command filter shows only mutations or only other commands", [kvh, ipe], async () => {
      const combo = page.locator(`[id='${historyPrefix}framework.history.filter/framework.history.filter.control']`);
      const choose = async (index: number) => { await combo.click(); await page.getByRole("option").nth(index).click(); await settle(900); };
      const mutations = (shown: string[]) => shown.some(row => /beam-kvh-c24/u.test(row)), others = (shown: string[]) => shown.some(row => l.panelNoise.test(row));
      await combo.click();
      const offered = await page.getByRole("option").allInnerTexts();
      await page.keyboard.press("Escape");
      assert.equal(offered.length, 3, `filter options ${offered}`);
      await choose(1);
      await eventually(async () => { const shown = await rows(); assert.ok(!mutations(shown) && others(shown), `without mutations ${shown}`); });
      const without = await rows();
      await choose(2);
      await eventually(async () => { const shown = await rows(); assert.ok(mutations(shown) && !others(shown), `only mutations ${shown}`); });
      const only = await rows();
      await choose(0);
      await eventually(async () => { const shown = await rows(); assert.ok(mutations(shown) && others(shown), `all ${shown}`); });
      return { offered, without, only };
    });
    await step("commit checkpoint keeps the curated state and records a checkpoint", [kvh], async () => {
      const before = checkpoints(await rows());
      await historyAction("commitCheckpoint").click();
      await eventually(async () => assert.equal(checkpoints(await rows()), before + 1, `no checkpoint row in ${await rows()}`));
      assert.deepEqual(await curatedNames(), [kvh]);
      return { rows: await rows() };
    });
    await step("checkout checkpoint returns the committed state", [kvh], async () => {
      const before = checkpoints(await rows());
      await historyAction("commitCheckpoint").click();
      await eventually(async () => assert.equal(checkpoints(await rows()), before + 1, "checkpoint not recorded"));
      await add(ipe);
      await menu("checkoutCheckpoint");
      await eventually(async () => assert.deepEqual(await curatedNames(), [kvh], "checkout did not return the checkpoint state"), 15_000);
      return { curated: await curatedNames(), rows: await rows() };
    });
    await step("create alternative branches from the checkpoint and switch alternative moves between the lines", [kvh], async () => {
      const marks = checkpoints(await rows());
      await historyAction("commitCheckpoint").click();
      await eventually(async () => assert.equal(checkpoints(await rows()), marks + 1, "checkpoint not recorded"));
      await add(ipe);
      const before = (await alternatives()).length;
      assert.equal(before, 1, `only the main line before creation ${JSON.stringify(await alternatives())}`);
      await historyAction("createAlternative").click();
      await eventually(async () => assert.equal((await alternatives()).length, before + 1, `alternative rows ${JSON.stringify(await alternatives())}`));
      await eventually(async () => assert.deepEqual(await curatedNames(), [kvh], "the alternative kept the post-checkpoint edit"));
      const lines = await alternatives();
      const alternative = lines.find(row => l.current.test(row.text))!, main = lines.find(row => row.id !== alternative.id)!;
      const switchTo = (id: string) => page.locator(`[id='${historyPrefix}framework.history.alternative.${id}'] button`).first().click();
      await switchTo(main.id);
      await eventually(async () => sameSet(await curatedNames(), [kvh, ipe]), 15_000);
      await switchTo(alternative.id);
      await eventually(async () => assert.deepEqual(await curatedNames(), [kvh]), 15_000);
      await menu("switchAlternative");
      await page.locator("[id='framework.window.sourcingPool.action.switchAlternative.arg.alternativeId.input']").fill(main.id);
      await page.locator("[id='framework.window.sourcingPool.action.switchAlternative.execute']").click();
      await eventually(async () => sameSet(await curatedNames(), [kvh, ipe]), 15_000);
      return { alternatives: lines };
    });
    const exportArchive = async () => {
      const download = page.waitForEvent("download", { timeout: 20_000 });
      await menu("exportArtifactDocument");
      const file = await download;
      const path = join(out, `history-${language}-${file.suggestedFilename()}`);
      await file.saveAs(path);
      return { name: file.suggestedFilename(), path, bytes: readFileSync(path) };
    };
    const importArchive = async (path: string) => {
      const chooser = page.waitForEvent("filechooser", { timeout: 20_000 });
      await menu("importArtifactDocument");
      await (await chooser).setFiles(path);
    };
    await step("viewer role opens the same document and its history panel lists the commands", [kvh], async () => {
      await page.locator("[id='playground.navbar.roles.viewer']").click();
      await settle(5000);
      assert.deepEqual(s.faults.filter(line => /role switch/u.test(line)).map(line => line.slice(0, 300)), [], "the role switch to the viewer failed");
      await openHistory();
      await eventually(async () => assert.ok((await rows()).some(row => /beam-kvh-c24/u.test(row)), `no curation entry in ${await rows()}`));
      return { rows: await rows() };
    });
    await step("export document downloads a semio archive that carries the curated rows", [kvh, ipe], async () => {
      const { name, bytes } = await exportArchive();
      assert.match(name, /^curation-\d{8}T\d{6}Z\.semio-archive$/u);
      assert.equal(bytes.subarray(4, 7).toString("latin1"), "SEM", "archive magic");
      for (const id of ["beam-kvh-c24", "beam-steel-ipe200"]) assert.ok(bytes.includes(id), `archive lacks ${id}`);
      await eventually(async () => assert.match(await s.body(), language === "en" ? /Exported/u : /exportiert/u));
      return { name, size: bytes.length };
    });
    await step("import document restores the curated rows into the Empty example", [kvh, ipe], async () => {
      const { path } = await exportArchive();
      await load();
      const selector = page.locator('[id="playground.navbar.fixture"]');
      await selector.click();
      await page.getByRole("option", { name: /^(no example|kein)/iu }).first().click();
      await settle(2500);
      assert.deepEqual(await curatedNames(), [], "the Empty example still holds rows");
      await importArchive(path);
      await eventually(async () => assert.match(await s.body(), language === "en" ? /Opened .* as a new document/u : /als neues Dokument geöffnet/u), 60_000);
      await eventually(async () => sameSet(await curatedNames(), [kvh, ipe]), 20_000);
      return { curated: await curatedNames() };
    });
    await step("a garbage file is refused as not a document archive and changes nothing", [kvh], async () => {
      const garbage = join(out, `history-${language}-garbage.semio-archive`);
      writeFileSync(garbage, "not an archive");
      await importArchive(garbage);
      await eventually(async () => assert.match(await s.body(), language === "en" ? /is not a document archive/u : /ist kein Dokumentarchiv/u), 20_000);
      assert.deepEqual(await curatedNames(), [kvh]);
      return {};
    });
    await step("Save File writes the JSON rfc8259 natural file that an independent parser reads back to the curated rows", [kvh, ipe], async () => {
      const download = page.waitForEvent("download", { timeout: 20_000 });
      await menu("saveArtifactFile");
      const file = await download;
      const path = join(out, `history-${language}-${file.suggestedFilename()}`);
      await file.saveAs(path);
      assert.match(file.suggestedFilename(), /\.json$/u);
      const parsed = JSON.parse(readFileSync(path, "utf8"));
      const rows = (parsed.curated as { objectId: string; count: number }[]).map(row => `${row.objectId}:${row.count}`).sort();
      assert.deepEqual(rows, ["beam-kvh-c24:1", "beam-steel-ipe200:1"]);
      return { name: file.suggestedFilename(), rows };
    });
    await step("Open File re-imports the JSON natural file into the Empty example", [kvh, ipe], async () => {
      const download = page.waitForEvent("download", { timeout: 20_000 });
      await menu("saveArtifactFile");
      const file = await download;
      const path = join(out, `history-${language}-reopen-${file.suggestedFilename()}`);
      await file.saveAs(path);
      await load();
      await page.locator('[id="playground.navbar.fixture"]').click();
      await page.getByRole("option", { name: /^(no example|kein)/iu }).first().click();
      await settle(2500);
      assert.deepEqual(await curatedNames(), [], "the Empty example still holds rows");
      const chooser = page.waitForEvent("filechooser", { timeout: 20_000 });
      await menu("openArtifactFile");
      await (await chooser).setFiles(path);
      await eventually(async () => sameSet(await curatedNames(), [kvh, ipe]), 60_000);
      return { curated: await curatedNames() };
    });
    await step("text utf-8 and ZIP 2.0 ioEntries stay agent-only: the editor offers the archive pair and the JSON natural file", [], async () => {
      const manifest = await page.evaluate(async () => (await fetch("/%F0%9F%94%8C%EF%B8%8Fplugin-modules/%F0%9F%AA%B5%EF%B8%8Fsourcing/%F0%9F%94%A3%EF%B8%8F.json")).json());
      const entries = manifest.contributions.ioEntries.map((entry: { direction: string; counterpart: { artifactKind: string; standard: string } }) => `${entry.direction} ${entry.counterpart.artifactKind}@${entry.counterpart.standard}`).sort();
      assert.deepEqual(entries, ["export framework.io.text@utf-8", "export s.stdio.json@rfc8259", "export s.stdio.zip@2.0", "import framework.io.text@utf-8", "import s.stdio.json@rfc8259", "import s.stdio.zip@2.0"]);
      await page.locator("[id='framework.window.sourcingPool.engagement.toggle']").click();
      await eventually(async () => assert.ok(await menuOpen()));
      const offered = await page.evaluate(() => [...document.querySelectorAll("[id^='action.'][id]")].map(element => element.id.slice("action.".length)).filter(id => !id.startsWith("category.")));
      for (const id of ["exportArtifactDocument", "importArtifactDocument", "saveArtifactFile", "openArtifactFile"]) assert.ok(offered.includes(id), `the menu lacks ${id}: ${offered}`);
      assert.deepEqual(offered.filter(id => /zip|txt|text|loadDocumentJson|catalog:out/iu.test(id)), []);
      assert.equal(await page.locator("[id*='catalog:out']").count(), 0);
      return { declared: entries, offered };
    });
    await step("folder sync survives a short offline spell without freezing the editor", [], async () => {
      const folder = join(root, ".cache/b/p/m/sourcing-data", `history-${language}-offline-${Date.now()}`);
      await attachFolder(s, folder);
      await add(kvh);
      await eventually(async () => assert.ok(existsSync(folderFile(folder)), "no archive saved"), 20_000);
      await s.context.setOffline(true);
      const before = statSync(folderFile(folder)).size;
      await add(ipe);
      await settle(3000);
      sameSet(await curatedNames(), [kvh, ipe]);
      await s.context.setOffline(false);
      await eventually(async () => assert.ok(statSync(folderFile(folder)).size > before, "the offline edit never reached the folder"), 30_000);
      return { folder, grew: statSync(folderFile(folder)).size - before };
    });
    await step("detach is offered once a folder is attached", [], async () => {
      const folder = join(root, ".cache/b/p/m/sourcing-data", `history-${language}-detach-${Date.now()}`);
      await attachFolder(s, folder);
      await page.locator("[id='framework.sync.folder']").click();
      await eventually(async () => assert.ok(await page.getByRole("button", { name: l.detach, exact: true }).isVisible(), "no detach control for the attached folder"));
      return {};
    });
    await step("detach stops saving to the folder and attaching the same folder again resumes it", [], async () => {
      const folder = join(root, ".cache/b/p/m/sourcing-data", `history-${language}-resume-${Date.now()}`);
      await attachFolder(s, folder);
      await add(kvh);
      await eventually(async () => assert.ok(existsSync(folderFile(folder)), "no archive saved"), 20_000);
      await page.locator("[id='framework.sync.folder']").click();
      await page.getByRole("button", { name: l.detach, exact: true }).click();
      await settle(6000);
      const size = statSync(folderFile(folder)).size;
      await add(ipe);
      await settle(4000);
      assert.equal(statSync(folderFile(folder)).size, size, "a detached folder kept receiving events");
      await attachFolder(s, folder);
      await settle(4000);
      const afterAttach = statSync(folderFile(folder)).size;
      await add(language === "en" ? "Steel HEA 160" : "Stahl HEA 160");
      await eventually(async () => assert.ok(statSync(folderFile(folder)).size > afterAttach, "the re-attached folder did not resume saving"), 30_000);
      return { folder, caughtUpOnAttach: afterAttach - size, resumed: statSync(folderFile(folder)).size - afterAttach };
    });
    await step("a second browser context attached to the same folder sees the first context's rows", [], async () => {
      const folder = join(root, ".cache/b/p/m/sourcing-data", `history-${language}-shared-${Date.now()}`);
      await attachFolder(s, folder);
      await add(kvh);
      await eventually(async () => assert.ok(existsSync(folderFile(folder)), "no archive saved"), 20_000);
      await settle(3000);
      const peer = await session(browser, language);
      try {
        await peer.page.goto(url);
        assert.match((await awaitBeacon(peer.page, Date.now() + 300_000)) ?? "", /^ready/u);
        await dismissIntroduction(peer.page);
        await peer.settle(3000);
        await attachFolder(peer, folder);
        await peer.eventually(async () => assert.ok((await peer.names(peer.curated())).includes(kvh), "the second context does not see the first context's row"), 20_000);
        await add(ipe);
        await peer.eventually(async () => assert.ok((await peer.names(peer.curated())).includes(ipe), "the second context does not see a later edit"), 20_000);
      } finally {
        await peer.context.close();
      }
      return { folder, files: readdirSync(join(folder, ".semio")) };
    });
    let blocked: string | null = null;
    const control = (id: string) => page.locator(`[id='${id}']`);
    const editorInputs = () => page.locator(`[id*='/framework.history.editor.input.']:not([id$='.row']) input, input[id*='/framework.history.editor.input.']`);
    const countCell = (kind: string) => rowIn(curated(), kind).getByRole("spinbutton").first();
    const chain = async (name: string, run: () => Promise<Record<string, unknown>>, seed: string[] = [kvh]) => {
      if (!wanted(name)) return;
      if (blocked !== null) {
        await baseStep(name, async () => { throw new Error(`blocked by the refused history edit begin: ${blocked}`); });
        console.log(`FAIL ${verdicts.at(-1)!.step} blocked`);
        return;
      }
      await step(name, seed, async () => {
        await beginEdit("beam-kvh-c24");
        await eventually(async () => assert.ok((await rowIds()).some(id => id.includes("framework.history.editor")), "no editor rows after Edit"), 12_000);
        return run();
      });
      if (!verdicts.at(-1)!.pass && blocked === null) blocked = JSON.stringify(verdicts.at(-1)!.evidence).slice(0, 700);
    };
    const finalizeToDialog = async () => {
      await editorInputs().first().fill("3");
      await control("ui.timeTravel.accept").click();
      await eventually(async () => assert.ok(await control("shell.time-travel.finalize").isEnabled(), "finalize not enabled"), 60_000);
      await control("shell.time-travel.finalize").click();
    };
    await chain("history edit begin opens the editor for one mutation and shows the band", async () => {
      assert.ok(await control("ui.timeTravel.accept").isVisible(), "no Accept control");
      return { rows: await rowIds() };
    });
    await chain("history edit input changes the draft value", async () => {
      await editorInputs().first().fill("3");
      await eventually(async () => assert.equal(await editorInputs().first().inputValue(), "3"));
      return {};
    });
    await chain("history edit use selection takes the selected row as the reference", async () => {
      await rowIn(pool(), ipe).locator("td").nth(1).click();
      await page.getByRole("button", { name: /Use selection|Auswahl verwenden/u }).first().click();
      await eventually(async () => assert.ok((await s.body()).includes("beam-steel-ipe200") || (await s.body()).includes(ipe)));
      return {};
    });
    await chain("history edit withdraw removes a mutation from the replay and restore returns it", async () => {
      await control("ui.timeTravel.exit").click();
      const row = mutationRow("beam-kvh-c24");
      await row.getByRole("button", { name: l.withdraw, exact: true }).click();
      await eventually(async () => assert.match((await rows()).join("|"), /Withdrawn|Zurückgezogen/u));
      await row.getByRole("button", { name: /Restore|Wiederherstellen/u }).click();
      await eventually(async () => assert.doesNotMatch((await rows()).join("|"), /Withdrawn|Zurückgezogen/u));
      return {};
    });
    await chain("history edit accept replays the edit and discard drops a draft", async () => {
      await editorInputs().first().fill("3");
      await control("ui.timeTravel.discard").click();
      await eventually(async () => assert.equal(await editorInputs().count(), 0, "discard kept the draft"));
      await beginEdit("beam-kvh-c24");
      await editorInputs().first().fill("3");
      await control("ui.timeTravel.accept").click();
      await eventually(async () => assert.ok(await control("shell.time-travel.finalize").isVisible()), 60_000);
      return {};
    });
    await chain("history edit finalize opens the finish dialog with Overwrite and New alternative", async () => {
      await finalizeToDialog();
      await eventually(async () => assert.ok(await page.getByRole("button", { name: /Overwrite|Überschreiben/u }).isVisible() && await page.getByRole("button", { name: /New alternative|Neue Alternative/u }).isVisible(), "dialog options missing"));
      return {};
    });
    await chain("history edit commit with Overwrite rewrites the history in place", async () => {
      await finalizeToDialog();
      await page.getByRole("button", { name: /Overwrite|Überschreiben/u }).click();
      await eventually(async () => assert.equal(await countCell(kvh).inputValue(), "3"), 60_000);
      return {};
    });
    await chain("history edit commit as New alternative keeps the original line", async () => {
      await finalizeToDialog();
      const before = (await alternatives()).length;
      await page.getByRole("button", { name: /New alternative|Neue Alternative/u }).click();
      await eventually(async () => assert.ok((await alternatives()).length > before, "no new alternative"), 60_000);
      return {};
    });
    await chain("history edit back returns from the finish dialog to the review", async () => {
      await finalizeToDialog();
      await control("shell.time-travel.back").click();
      await eventually(async () => assert.ok(await control("shell.time-travel.finalize").isVisible()));
      return {};
    });
    await chain("history edit cancel replay, rerun and exit leave a consistent session", async () => {
      await editorInputs().first().fill("3");
      await control("ui.timeTravel.accept").click();
      await control("shell.time-travel.cancel-replay").click({ timeout: 3000 }).catch(() => undefined);
      await eventually(async () => assert.ok(await control("shell.time-travel.rerun").isVisible() || await control("shell.time-travel.finalize").isVisible()), 60_000);
      if (await control("shell.time-travel.rerun").isEnabled().catch(() => false)) await control("shell.time-travel.rerun").click();
      await control("ui.timeTravel.exit").click();
      await eventually(async () => assert.equal(await control("ui.timeTravel.exit").count(), 0, "session stayed open"));
      assert.deepEqual(await curatedNames(), [kvh]);
      return {};
    });
  } finally {
    await s.shot("final").catch(() => undefined);
    await s.context.close();
  }
}

await runJourneys("history", journey);
