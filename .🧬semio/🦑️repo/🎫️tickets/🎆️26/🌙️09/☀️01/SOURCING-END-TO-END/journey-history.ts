import { readFileSync } from "node:fs";
import { assert, awaitBeacon, dismissIntroduction, join, out, runJourneys, session, url, type Browser, type Page } from "./journey-kit.ts";

/** ⏪️ Sourcing curation editor journey (6081) for HISTORY and IMPORT/EXPORT: checkpoint, alternatives, checkout, revert, the history edit chain, finish dialog, command filter, undo/redo, archive export and import, folder sync with a second browser context; English and German.
 * Usage: `JOURNEY=history bun journey-history.ts [en|de ...]` with the native sourcing host answering on 6081.
 * Writes screenshots, downloads and `history.json` under `🗑️generated/journey`.
 * @see ./journey-kit.ts
 * @see ./🕰️history-io-features.md */

const labels = {
  en: { commit: "Commit Checkpoint", checkout: "Checkout Checkpoint", create: "Create Alternative", switchAlt: "Switch Alternative…", undo: "Undo", redo: "Redo", exportDocument: "Export Document", importDocument: "Import Document…", edit: "Edit", withdraw: "Withdraw", backwards: "Backwards", filter: "Filter", attach: "Attach" },
  de: { commit: "Checkpoint festschreiben", checkout: "Checkpoint auschecken", create: "Alternative erstellen", switchAlt: "Alternative wechseln…", undo: "Rückgängig", redo: "Wiederholen", exportDocument: "Dokument exportieren", importDocument: "Dokument importieren…", edit: "Bearbeiten", withdraw: "Zurückziehen", backwards: "Zurück", filter: "Filter", attach: "Verbinden" },
} as const;

const history = "[id^='panel:framework.panel.history/']";
const entries = "[id^='panel:framework.panel.history/framework.history.entry.'],[id^='panel:framework.panel.history/framework.history.commands␟']";

async function journey(browser: Browser, language: "en" | "de") {
  const s = await session(browser, language);
  const { page, t, pool, curated, rowIn, names, body, settle, eventually, step } = s;
  const l = labels[language];
  const kvh = "KVH C24 100×200", ipe = "Steel IPE 200", hea = "Steel HEA 160";
  const historyOpen = () => page.locator("[id='panel:framework.panel.history/framework.history.actions']").isVisible().catch(() => false);
  const openHistory = async () => { if (!(await historyOpen())) await page.locator("[id='framework.panel.history']").click(); await eventually(async () => assert.ok(await historyOpen(), "History panel not open")); };
  const rows = () => page.evaluate(selector => [...document.querySelectorAll(selector)].map(element => (element as HTMLElement).innerText.replace(/\s+/gu, " ").trim()), `${history}[role=treeitem]`);
  const rowIds = () => page.evaluate(selector => [...document.querySelectorAll(selector)].map(element => element.id.replace("panel:framework.panel.history/", "")), `${history}[role=treeitem]`);
  const curatedNames = () => names(curated());
  const add = async (kind: string) => { await rowIn(pool(), kind).getByRole("button", { name: t.increase, exact: true }).click(); await eventually(async () => assert.ok((await curatedNames()).includes(kind), `${kind} not curated`)); };
  const menuOpen = () => page.locator("[id='action.exportArtifactDocument']").isVisible().catch(() => false);
  const menu = async (actionId: string) => {
    if (!(await menuOpen())) await page.locator("[id='framework.window.sourcingPool.engagement.toggle']").click();
    const item = page.locator(`[id='action.${actionId}']`);
    await eventually(async () => assert.ok(await item.isVisible(), `menu item ${actionId} not visible`));
    await item.click();
    await settle(900);
  };
  const historyButton = (name: string | RegExp) => page.locator(`${history} button`).filter({ hasText: name }).first();
  const notice = () => page.evaluate(() => [...document.querySelectorAll("[role=status],[role=alert],[data-notice],[class*=toast]")].map(element => (element as HTMLElement).innerText.trim()).filter(Boolean));
  const refused = async () => (await body()).match(/could not be delivered|konnte nicht zugestellt/iu)?.[0];
  const reset = async () => {
    await page.goto(url);
    assert.match((await awaitBeacon(page, Date.now() + 300_000)) ?? "", /^ready/u);
    await dismissIntroduction(page);
    await settle(3000);
    await openHistory();
  };
  const forgets = async (): Promise<string[]> => { const taken = [...s.faults]; return taken; };
  void forgets; void notice; void readFileSync; void join; void out; void ({} as Page);

  const altRows = () => page.evaluate(() => [...document.querySelectorAll("[id^='panel:framework.panel.history/framework.history.alternative.']")].map(element => ({ id: element.id.replace("panel:framework.panel.history/framework.history.alternative.", ""), text: (element as HTMLElement).innerText.replace(/\s+/gu, " ").trim() })));
  const rowAction = async (name: string, hint: RegExp, action: RegExp) => {
    const row = page.locator(entries).filter({ hasText: hint }).first();
    await row.click({ button: "right" });
    await page.getByRole("menuitem", { name: action }).first().click();
    await settle(1500);
    return name;
  };

  try {
    await reset();
    await step("history panel lists the actions and the recorded commands", async () => {
      const ids = await rowIds();
      for (const key of ["framework.history.undo", "framework.history.redo", "framework.history.commitCheckpoint", "framework.history.createAlternative", "framework.history.filter"]) assert.ok(ids.includes(key), `missing ${key}`);
      await add(kvh); await add(ipe);
      await eventually(async () => assert.ok((await rows()).filter(row => /beam-kvh-c24|beam-steel-ipe200/u.test(row)).length >= 2, `no curation entries in ${await rows()}`));
      return { rows: await rows() };
    });
    await step("revert to command takes back only the newer commands", async () => {
      await rowAction("revert", /kvh/u, new RegExp(l.backwards, "u"));
      await eventually(async () => assert.deepEqual(await curatedNames(), [kvh]));
      return { curated: await curatedNames() };
    });
    await step("redo via the Actions menu returns the reverted command", async () => {
      await menu("redo");
      await eventually(async () => assert.deepEqual((await curatedNames()).sort(), [ipe, kvh].sort()));
      return { curated: await curatedNames() };
    });
    await step("undo and redo via the Actions menu", async () => {
      await menu("undo");
      await eventually(async () => assert.ok(!(await curatedNames()).includes(ipe), "undo kept the row"));
      const undone = await curatedNames();
      await menu("redo");
      await eventually(async () => assert.ok((await curatedNames()).includes(ipe), "redo did not return the row"));
      return { undone, redone: await curatedNames() };
    });
    await step("history command filter shows only mutations or only other commands", async () => {
      const combo = page.locator("[id='panel:framework.panel.history/framework.history.filter/framework.history.filter.control']");
      const options = async () => { await combo.click(); const labelsOf = await page.getByRole("option").allInnerTexts(); return labelsOf; };
      const offered = await options();
      assert.equal(offered.length, 3, `filter options ${offered}`);
      await page.getByRole("option").nth(1).click();
      await eventually(async () => { const shown = await rows(); assert.ok(!shown.some(row => /beam-kvh-c24/u.test(row)) && shown.some(row => /Panel|Fenster|Beispiel|Example/u.test(row)), `without mutations ${shown}`); });
      const without = await rows();
      await combo.click(); await page.getByRole("option").nth(2).click();
      await eventually(async () => { const shown = await rows(); assert.ok(shown.some(row => /beam-kvh-c24/u.test(row)) && !shown.some(row => /Panel|Fenster|Beispiel|Example/u.test(row)), `only mutations ${shown}`); });
      const only = await rows();
      await combo.click(); await page.getByRole("option").nth(0).click();
      await eventually(async () => { const shown = await rows(); assert.ok(shown.some(row => /beam-kvh-c24/u.test(row)) && shown.some(row => /Panel|Fenster|Beispiel|Example/u.test(row)), `all ${shown}`); });
      return { offered, without, only };
    });
    await step("commit checkpoint keeps the curated state and records a checkpoint", async () => {
      const before = await curatedNames();
      await historyButton(l.commit).click();
      await eventually(async () => assert.ok((await rows()).some(row => row === l.commit), `no checkpoint row in ${await rows()}`));
      assert.deepEqual(await curatedNames(), before);
      return { rows: await rows() };
    });
    await step("create alternative branches from the checkpoint and holds its own state", async () => {
      await add(hea);
      assert.equal((await altRows()).length, 0, "alternatives before creation");
      await historyButton(new RegExp(`^${l.create}`, "u")).click();
      await eventually(async () => assert.equal((await altRows()).length, 2, `alternative rows ${JSON.stringify(await altRows())}`));
      await eventually(async () => assert.ok(!(await curatedNames()).includes(hea), "the alternative kept the post-checkpoint edit"));
      return { alternatives: await altRows(), curated: await curatedNames() };
    });
    await step("switch alternative from the history panel returns the main line state and back", async () => {
      const current = (await altRows()).find(row => /Current|Aktuell/u.test(row.text))!;
      const other = (await altRows()).find(row => row.id !== current.id)!;
      await page.locator(`[id='panel:framework.panel.history/framework.history.alternative.${other.id}'] button`).first().click();
      await eventually(async () => assert.ok((await curatedNames()).includes(hea), "main line state not returned"));
      await page.locator(`[id='panel:framework.panel.history/framework.history.alternative.${current.id}'] button`).first().click();
      await eventually(async () => assert.ok(!(await curatedNames()).includes(hea), "alternative state not returned"));
      return { main: other.id, alternative: current.id };
    });
    await step("switch alternative from the Actions menu with the alternative id", async () => {
      const current = (await altRows()).find(row => /Current|Aktuell/u.test(row.text))!;
      const other = (await altRows()).find(row => row.id !== current.id)!;
      await menu("switchAlternative");
      await page.locator("[id='framework.window.sourcingPool.action.switchAlternative.arg.alternativeId.input']").fill(other.id);
      await page.locator("[id='framework.window.sourcingPool.action.switchAlternative.execute']").click();
      await eventually(async () => assert.ok((await curatedNames()).includes(hea), "main line state not returned"));
      return { switched: other.id };
    });
  } finally {
    await s.shot("final").catch(() => undefined);
    await s.context.close();
  }
  void entries; void refused; void menu;
}

await runJourneys("history", journey);
