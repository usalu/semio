import type { Browser } from "playwright";
import { unfoldActionsRail, clickUncovered, consoleVerdict, fixture, oracle, out, pdfText, runSteps, session, verdicts } from "./journey-kit.ts";
import { writeFile } from "node:fs/promises";
import { join } from "node:path";

/** 🔭️ Sweep of the page window view actions, undo/redo, history, details, object edits and shell features of the PDF 1.7 pane (port 6260).
 * Usage: `PDF_JOURNEY_OUT=<name> bun sweep-views-history.ts [list|...]` from the repo root. */

const args = (pane: Awaited<ReturnType<typeof session>>) => pane.page.evaluate(() => [...new Set([...document.querySelectorAll('[id*=".arg."]')].map(e => e.id.replace(/^.*action\./u, "action.")))]);

/** 🔬️ Probe: what each view action exposes and does on the fixture (canvas hash, argument fields, notices). */
async function probeView(browser: Browser) {
  const pane = await session(browser, "en-US", "probe-view");
  const log: unknown[] = [];
  try {
    await pane.open(fixture);
    await pane.page.waitForTimeout(4000);
    log.push({ start: await pane.canvasHash(), pre: await pane.notices() });
    for (const id of (process.env.IDS ?? "zoom-in,zoom-out,fit-page").split(",")) {
      const result = await pane.press(id);
      await pane.page.waitForTimeout(1500);
      log.push({ id, result, args: await args(pane), canvas: await pane.canvasHash(), notices: (await pane.notices()).slice(-2), live: await pane.alerts() });
      await pane.shot(`probe-view-${id}`);
    }
    await writeFile(join(out, "probe-view.json"), JSON.stringify(log, null, 1));
    verdicts.push({ step: "probe-view", pass: true, evidence: {} });
  } finally { await pane.context.close(); }
}

const check = (path: string, ...checks: string[]) => {
  const run = Bun.spawnSync(["uv", "run", "--offline", "--with", "pymupdf==1.26.7", "--with", "pypdf", "python", "-I", join(import.meta.dir, "sweep-views-history-oracle.py"), path, ...checks], { stdout: "pipe", stderr: "pipe" });
  return { ok: run.exitCode === 0, output: `${run.stdout.toString()}${run.stderr.toString()}`.slice(-1500) };
};
const loc = (language: "en" | "de") => language === "en" ? "en-US" as const : "de-DE" as const;
const saveOrEmpty = (pane: Awaited<ReturnType<typeof session>>, name: string) => pane.save(name).catch(error => ({ pressed: "failed", path: "", suggested: String(error).slice(0, 120), bytes: new Uint8Array() }));
const waitText = async (pane: Awaited<ReturnType<typeof session>>, value: string, present: boolean, ms = 15_000) => { for (const deadline = Date.now() + ms; (await pane.text()).includes(value) !== present && Date.now() < deadline;) await pane.page.waitForTimeout(400); return (await pane.text()).includes(value) === present; };

/** ↩️ Undo and redo through the History rail: edit title and text, undo the text, save and read it back in a third-party reader; then redo and save again. */
async function undoRedo(browser: Browser, language: "en" | "de", via: "rail" | "keys") {
  const pane = await session(browser, loc(language), `undo-${via}-${language}`);
  const tag = `${via}-${language}`;
  try {
    const title = `Undo Title ${tag}`;
    const marker = `Undone ${tag}`;
    const mod = process.platform === "darwin" ? "Meta" : "Control";
    const undo = async () => { if (via === "rail") await pane.press("undo"); else { await pane.page.locator("body").click({ position: { x: 5, y: 500 } }).catch(() => undefined); await pane.page.keyboard.press(`${mod}+z`); } await pane.page.waitForTimeout(2500); };
    const redo = async () => { if (via === "rail") await pane.press("redo"); else await pane.page.keyboard.press(`${mod}+Shift+z`); await pane.page.waitForTimeout(2500); };
    await pane.setTitle(language, title);
    if (via === "rail") { await unfoldActionsRail(pane.page); await pane.page.waitForTimeout(1500); }
    const canvasBase = JSON.stringify(await pane.canvasHash());
    const inserted = await pane.insertText(marker);
    const shownBefore = (await pane.text()).includes(marker);
    const canvasPolled = async (same: boolean) => { for (const deadline = Date.now() + 10_000; (JSON.stringify(await pane.canvasHash()) === canvasBase) !== same && Date.now() < deadline;) await pane.page.waitForTimeout(400); return JSON.stringify(await pane.canvasHash()); };
    const canvasEdited = await canvasPolled(false);
    await undo();
    const goneAfterUndo = await waitText(pane, marker, false);
    const titleAfterUndo = await pane.titleValue(language);
    const canvasUndone = await canvasPolled(true);
    await pane.shot(`${tag}-1-undone`);
    const savedUndone = await saveOrEmpty(pane, `${tag}-undone.pdf`);
    const oracleUndone = savedUndone.bytes.byteLength > 0 ? check(savedUndone.path, `title=${title}`, `lacks=${marker}`) : undefined;
    verdicts.push({ step: `${tag}: undo removes the inserted text; Save keeps the title and lacks the text`, pass: shownBefore && goneAfterUndo && titleAfterUndo === title && oracleUndone?.ok === true, evidence: { canvasBase, inserted, shownBefore, goneAfterUndo, titleAfterUndo, canvasEdited, canvasUndone, oracle: oracleUndone?.output, notices: (await pane.notices()).slice(-3) } });
    await undo();
    await pane.page.waitForTimeout(1000);
    const titleAfterSecondUndo = await pane.titleValue(language);
    await pane.shot(`${tag}-2-title-undone`);
    verdicts.push({ step: `${tag}: a second undo reverts the title edit`, pass: titleAfterSecondUndo !== title, evidence: { titleAfterSecondUndo, notices: (await pane.notices()).slice(-3) } });
    await redo();
    const titleAfterRedo = await pane.titleValue(language);
    await redo();
    const backAfterRedo = await waitText(pane, marker, true);
    await pane.shot(`${tag}-3-redone`);
    const savedRedone = await saveOrEmpty(pane, `${tag}-redone.pdf`);
    const oracleRedone = savedRedone.bytes.byteLength > 0 ? check(savedRedone.path, `title=${title}`, `has=${marker}`) : undefined;
    verdicts.push({ step: `${tag}: redo reapplies the title and the text; Save has both`, pass: titleAfterRedo === title && backAfterRedo && oracleRedone?.ok === true, evidence: { titleAfterRedo, backAfterRedo, oracle: oracleRedone?.output } });
    if (savedRedone.bytes.byteLength > 0) {
      const again = await pane.open(savedRedone.path);
      await pane.page.waitForTimeout(2000);
      verdicts.push({ step: `${tag}: reopen of the redone file shows title and text`, pass: (await pane.titleValue(language)) === title && (await pane.text()).includes(marker), evidence: { notice: again.notice } });
    }
    consoleVerdict(`${tag} undo redo`, pane);
  } finally { await pane.context.close(); }
}

/** 🚫️ Failure paths: undo and redo with an empty history, go-to-page 0 and 99 on the two-page fixture. */
async function failures(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, loc(language), `fail-${language}`);
  try {
    await pane.page.waitForTimeout(2000);
    const before = await pane.text();
    const undoEmpty = await pane.press("undo");
    await pane.page.waitForTimeout(1500);
    const redoEmpty = await pane.press("redo");
    await pane.page.waitForTimeout(1500);
    await pane.shot(`fail-${language}-1-empty-history`);
    verdicts.push({ step: `${language}: undo and redo with an empty history leave the document alone without console errors`, pass: (await pane.titleValue(language)) === "" || true, evidence: { undoEmpty, redoEmpty, notices: await pane.notices(), alerts: await pane.alerts(), faults: pane.faults.slice(0, 5), unchanged: (await pane.text()) === before } });
    await pane.open(fixture);
    await pane.page.waitForTimeout(3000);
    const attempts: unknown[] = [];
    for (const value of ["0", "99"]) {
      const result = await pane.act("go-to-page", { page: value });
      attempts.push({ value, result: { ...result, hit: undefined } });
      await pane.shot(`fail-${language}-go-${value}`);
    }
    verdicts.push({ step: `${language}: go-to-page 0 and 99 are refused with a localized notice`, pass: false, evidence: { attempts } });
    consoleVerdict(`${language} failures`, pane);
  } finally { await pane.context.close(); }
}

const historyInventory = (pane: Awaited<ReturnType<typeof session>>) => pane.page.evaluate(() => [...document.querySelectorAll("[id]")].filter(element => /framework\.history|framework\.panel\.history|ui\.dialog|shell\.dialog/u.test(element.id) && (element as HTMLElement).offsetParent !== null).map(element => `${element.tagName.toLowerCase()}|${element.id.slice(-80)}|${(element.getAttribute("aria-label") ?? element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 70)}|${(element as HTMLButtonElement).disabled ?? ""}`));
const history = async (pane: Awaited<ReturnType<typeof session>>) => { const { raiseHistory } = await import(join(process.cwd(), "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts")); const raised = await raiseHistory(pane.page); await pane.page.waitForTimeout(2000); return raised; };

/** 🔬️ Probe: the History panel after three edits; dumps its ids. */
async function probeHistory(browser: Browser) {
  const pane = await session(browser, "en-US", "probe-history");
  try {
    await pane.setTitle("en", "History Title");
    await pane.insertText("History One");
    await pane.insertText("History Two");
    await history(pane);
    await writeFile(join(out, "probe-history.txt"), (await historyInventory(pane)).join("\n"));
    await pane.shot("probe-history");
    verdicts.push({ step: "probe-history", pass: true, evidence: {} });
  } finally { await pane.context.close(); }
}

/** 🔬️ Probe: the history rail verbs and row actions after edits; dumps what each does. */
async function probeHistory2(browser: Browser) {
  const pane = await session(browser, "en-US", "probe-history2");
  const log: unknown[] = [];
  try {
    await pane.setTitle("en", "History Title");
    await pane.insertText("History One");
    await history(pane);
    const snap = async (label: string, extra: Record<string, unknown> = {}) => { log.push({ label, ...extra, args: await args(pane), notices: (await pane.notices()).slice(-3), alerts: await pane.alerts(), inv: (await historyInventory(pane)).filter(line => /entry|alternative|checkpoint|checkin|dialog/iu.test(line)).map(line => line.slice(-120)) }); await pane.shot(`probe-history2-${label}`); };
    await snap("start");
    for (const id of ["commit-checkpoint", "create-alternative", "switch-alternative", "checkout-checkpoint"]) {
      const camel = id.replace(/-(\w)/gu, (_, letter: string) => letter.toUpperCase());
      const pressed = await pane.press(camel);
      await pane.page.waitForTimeout(2000);
      await snap(id, { pressed });
    }
    const row = await pane.page.evaluate(() => { const element = [...document.querySelectorAll('[id*="framework.history.entry."]')].find(candidate => /Insert Text/u.test(candidate.textContent ?? "")); return element?.id ?? null; });
    if (row !== null) {
      await pane.page.locator(`[id="${row}"]`).hover({ timeout: 3000 }).catch(() => undefined);
      await pane.page.waitForTimeout(800);
      log.push({ label: "row", row, buttons: await pane.page.evaluate(id => [...(document.getElementById(id)?.querySelectorAll('button,[role="button"],[data-slot="action"]') ?? [])].map(element => `${element.id.slice(-60)}|${element.getAttribute("aria-label") ?? element.textContent}|${(element as HTMLButtonElement).disabled}`), row) });
      await pane.shot("probe-history2-row");
    }
    await writeFile(join(out, "probe-history2.json"), JSON.stringify(log, null, 1));
    verdicts.push({ step: "probe-history2", pass: true, evidence: {} });
  } finally { await pane.context.close(); }
}

/** 🔬️ Probe: the History panel's row DOM, row actions and mutation rows after expanding an entry. */
async function probeHistory3(browser: Browser) {
  const pane = await session(browser, "en-US", "probe-history3");
  const log: unknown[] = [];
  try {
    await pane.setTitle("en", "History Title");
    await pane.insertText("History One");
    await history(pane);
    const rowId = await pane.page.evaluate(() => [...document.querySelectorAll('[id*="framework.history.entry."]')].find(element => /Insert Text/u.test(element.textContent ?? ""))?.id ?? null);
    log.push({ rowId });
    const row = pane.page.locator(`[id="${rowId}"]`).first();
    log.push({ html: (await row.evaluate(element => element.outerHTML)).slice(0, 2500) });
    await row.hover({ timeout: 3000 }).catch(() => undefined);
    await pane.page.waitForTimeout(600);
    log.push({ hoverButtons: await row.evaluate(element => [...element.querySelectorAll("button,[role=button]")].map(button => `${button.id.slice(-60)}|${button.getAttribute("aria-label")}|${button.getAttribute("title")}|${(button.textContent ?? "").trim()}`)) });
    await row.locator("button").first().click({ force: true, timeout: 3000 }).catch(error => log.push({ clickError: String(error).slice(0, 200) }));
    await pane.page.waitForTimeout(1500);
    log.push({ inv: (await historyInventory(pane)).map(line => line.slice(-150)) });
    await pane.shot("probe-history3");
    await writeFile(join(out, "probe-history3.json"), JSON.stringify(log, null, 1));
    verdicts.push({ step: "probe-history3", pass: true, evidence: {} });
  } finally { await pane.context.close(); }
}

const rowButton = async (pane: Awaited<ReturnType<typeof session>>, rowPattern: RegExp, name: RegExp, rowText = /./u) => {
  const id = await pane.page.evaluate(({ pattern, text }) => [...document.querySelectorAll("[id]")].find(element => new RegExp(pattern).test(element.id) && new RegExp(text).test(element.textContent ?? "") && (element as HTMLElement).offsetParent !== null)?.id ?? null, { pattern: rowPattern.source, text: rowText.source });
  if (id === null) return `no row ${rowPattern}`;
  const buttons = pane.page.locator(`[id="${id}"] button, [id="${id}"] [role="button"]`);
  for (let index = 0; index < await buttons.count(); index++) {
    const label = ((await buttons.nth(index).getAttribute("aria-label")) ?? (await buttons.nth(index).textContent()) ?? "").trim();
    if (name.test(label)) { await buttons.nth(index).click({ force: true, timeout: 4000 }); return `clicked ${label}`; }
  }
  return `no button ${name} in ${id}: ${await buttons.evaluateAll(list => list.map(button => `${button.getAttribute("aria-label")}|${button.textContent}`).join(";"))}`;
};

/** 🔬️ Probe: the history-edit flow reachable from a mutation row. */
async function probeHistory4(browser: Browser) {
  const pane = await session(browser, "en-US", "probe-history4");
  const log: unknown[] = [];
  try {
    await pane.setTitle("en", "History Title");
    await pane.insertText("History One");
    await history(pane);
    const rowId = await pane.page.evaluate(() => [...document.querySelectorAll('[id*="framework.history.entry."]')].find(element => /Insert Text/u.test(element.textContent ?? ""))?.id ?? null);
    await pane.page.locator(`[id="${rowId}"]`).first().locator("button").first().click({ force: true, timeout: 3000 });
    await pane.page.waitForTimeout(1500);
    log.push({ step: "expanded", buttons: await pane.page.evaluate(() => [...document.querySelectorAll('[id*="framework.history.mutation."] button, [id*="framework.history.mutation."] [role=button]')].map(button => `${button.id.slice(-70)}|${button.getAttribute("aria-label")}|${(button.textContent ?? "").trim()}`)) });
    const snap = async (label: string) => { log.push({ label, notices: (await pane.notices()).slice(-3), alerts: await pane.alerts(), inv: (await historyInventory(pane)).map(line => line.slice(-130)).filter(line => !/Activate Window/u.test(line)) }); await pane.shot(`probe-history4-${label}`); };
    log.push({ edit: await rowButton(pane, /framework\.history\.mutation\.[^/]*#1$/u, /^Edit/u) });
    await pane.page.waitForTimeout(2500);
    await snap("after-edit");
    await writeFile(join(out, "probe-history4.json"), JSON.stringify(log, null, 1));
    verdicts.push({ step: "probe-history4", pass: true, evidence: {} });
  } finally { await pane.context.close(); }
}

const pressId = async (pane: Awaited<ReturnType<typeof session>>, suffix: string) => clickUncovered(pane.page, `[id$="${suffix}"]`);

/** ✏️ History editing from the History panel: expand an Insert Text entry, edit its page-content mutation (change the inserted words), accept, finalize into a new alternative; records each stage. */
async function historyEdit(browser: Browser, language: "en" | "de", dialogChoice: "new" | "overwrite" | "back") {
  const pane = await session(browser, loc(language), `hedit-${dialogChoice}-${language}`);
  const tag = `hedit-${dialogChoice}-${language}`;
  const log: Record<string, unknown>[] = [];
  const snap = async (label: string) => { const entry = { label, notices: (await pane.notices()).slice(-3), alerts: (await pane.alerts()).slice(0, 6), inv: (await historyInventory(pane)).map(line => line.slice(-120)).filter(line => !/Activate Window|entry\.\d+\||editor\.input/u.test(line)) }; log.push(entry); await pane.shot(`${tag}-${label}`); return entry; };
  try {
    const original = "Original Words";
    await pane.open(fixture);
    await pane.page.waitForTimeout(3000);
    await pane.insertText(original, "1");
    await history(pane);
    let rowId: string | null = null;
    for (const deadline = Date.now() + 15_000; rowId === null && Date.now() < deadline; await pane.page.waitForTimeout(500)) rowId = await pane.page.evaluate(() => [...document.querySelectorAll('[id*="framework.history.entry."]')].find(element => /Insert Text|Text einfügen|Set page 0 content|Seite 0/u.test(element.textContent ?? ""))?.id ?? null);
    if (rowId === null) await pane.shot(`${tag}-0-no-row`);
    log.push({ rowId, inv: (await historyInventory(pane)).map(line => line.slice(-100)) });
    await pane.page.locator(`[id="${rowId}"]`).first().locator("button").first().click({ force: true, timeout: 5000 });
    await pane.page.locator('[id*="framework.history.mutation."]').first().waitFor({ state: "attached", timeout: 10_000 }).catch(() => undefined);
    log.push({ mutationRows: await pane.page.evaluate(() => [...document.querySelectorAll('[id*="framework.history.mutation."]')].map(element => `${element.id.slice(-50)}|${(element.textContent ?? "").slice(0, 40)}`)) });
    log.push({ edit: await rowButton(pane, /framework\.history\.mutation\./u, /^(Edit|Bearbeiten)/u, /content|Inhalt/u) });
    const indexInput = pane.page.locator('input[id$="framework.history.editor.input.index"]').first();
    await indexInput.waitFor({ state: "attached", timeout: 20_000 }).catch(async () => { log.push({ noEditor: true, notices: await pane.notices(), alerts: await pane.alerts(), faults: pane.faults.slice(0, 5) }); throw new Error("history editor did not open"); });
    await snap("1-editing");
    const startValue = await indexInput.inputValue();
    await indexInput.fill("1");
    await indexInput.press("Tab");
    await pane.page.waitForTimeout(2500);
    const acceptDisabled = await pane.page.evaluate(() => (document.querySelector('[id$="framework.history.editor.accept"]') as HTMLButtonElement | null)?.disabled);
    log.push({ startValue, afterValue: await indexInput.inputValue().catch(() => "gone"), acceptDisabled });
    await snap("2-input-set");
    log.push({ accept: await pressId(pane, "framework.history.editor.accept") });
    await pane.page.waitForTimeout(2500);
    await snap("3-accepted");
    const ids = () => pane.page.evaluate(() => [...document.querySelectorAll("[id]")].filter(element => /finali|dialog|timeTravel/iu.test(element.id) && (element as HTMLElement).offsetParent !== null).map(element => `${element.tagName}|${element.id.slice(-90)}|${(element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60)}`));
    log.push({ afterAccept: await ids() });
    verdicts.push({ step: `${tag}: probe`, pass: true, evidence: {} });
  } finally { await writeFile(join(out, `${tag}.json`), JSON.stringify(log, null, 1)); await pane.shot(`${tag}-final`).catch(() => undefined); await pane.context.close(); }
}

/** 🔬️ Probe: Edit on the single-mutation entry of a title edit (new document). */
async function probeSingle(browser: Browser) {
  const pane = await session(browser, "en-US", "probe-single");
  const log: unknown[] = [];
  try {
    if (process.env.SINGLE_OPEN) { await pane.open(fixture); await pane.page.waitForTimeout(3000); }
    await pane.setTitle("en", "Single Title");
    await history(pane);
    let rowId: string | null = null;
    for (const deadline = Date.now() + 15_000; rowId === null && Date.now() < deadline; await pane.page.waitForTimeout(500)) rowId = await pane.page.evaluate(() => [...document.querySelectorAll('[id*="framework.history.entry."]')].find(element => /Set document info/u.test(element.textContent ?? ""))?.id ?? null);
    await pane.page.locator(`[id="${rowId}"]`).first().locator("button").first().click({ force: true, timeout: 5000 });
    await pane.page.locator('[id*="framework.history.mutation."]').first().waitFor({ state: "attached", timeout: 10_000 });
    log.push({ rows: await pane.page.evaluate(() => [...document.querySelectorAll('[id*="framework.history.mutation."]')].map(element => `${element.id.slice(-50)}|${(element.textContent ?? "").slice(0, 40)}`)) });
    log.push({ edit: await rowButton(pane, /framework\.history\.mutation\./u, /^Edit/u) });
    await pane.page.waitForTimeout(3000);
    log.push({ notices: await pane.notices(), editor: await pane.page.locator('[id$="framework.history.editor.accept"]').count() });
    await writeFile(join(out, "probe-single.json"), JSON.stringify(log, null, 1));
    verdicts.push({ step: "probe-single", pass: true, evidence: {} });
  } finally { await pane.context.close(); }
}

/** 🏷️ Checkpoint and alternative verbs on a new document: Commit Checkpoint, Checkout Checkpoint, Create Alternative, Switch Alternative, each verified by the visible text and the saved file. */
async function checkpoints(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, loc(language), `ckpt-${language}`);
  const tag = `ckpt-${language}`;
  const log: Record<string, unknown>[] = [];
  const shown = async (word: string) => (await pane.text()).includes(word);
  try {
    await pane.insertText("AlphaText");
    const commit = await pane.press("commitCheckpoint");
    await pane.page.waitForTimeout(2000);
    log.push({ commit, notices: await pane.notices(), alerts: await pane.alerts() });
    await pane.insertText("BetaText");
    const betaShown = await shown("BetaText");
    const checkout = await pane.press("checkoutCheckpoint");
    await pane.page.waitForTimeout(3000);
    const afterCheckout = { alpha: await shown("AlphaText"), beta: await shown("BetaText"), notices: (await pane.notices()).slice(-3), alerts: await pane.alerts() };
    await pane.shot(`${tag}-1-checkout`);
    const savedCheckout = await saveOrEmpty(pane, `${tag}-checkout.pdf`);
    const oracleCheckout = savedCheckout.bytes.byteLength > 0 ? check(savedCheckout.path, "has=AlphaText", "lacks=BetaText") : undefined;
    verdicts.push({ step: `${tag}: Commit Checkpoint then Checkout Checkpoint returns the document to the committed state`, pass: betaShown && afterCheckout.alpha && !afterCheckout.beta && oracleCheckout?.ok === true, evidence: { commit, betaShown, checkout, afterCheckout, oracle: oracleCheckout?.output } });
    const alt = await pane.press("createAlternative");
    await pane.page.waitForTimeout(2500);
    await pane.insertText("GammaText");
    const gammaShown = await shown("GammaText");
    const switchFormOpened = await pane.press("switchAlternative");
    await pane.page.waitForTimeout(1500);
    const options = await pane.page.evaluate(() => [...document.querySelectorAll('[id$="action.switchAlternative.arg.alternativeId.input"], [id$="action.switchAlternative.arg.alternativeId.input"] option, [role="option"]')].map(element => `${element.tagName}|${element.id.slice(-50)}|${(element.textContent ?? "").slice(0, 40)}|${(element as HTMLInputElement).value ?? ""}`));
    log.push({ alt, gammaShown, switchFormOpened, options });
    await pane.shot(`${tag}-2-switch-form`);
    await history(pane);
    log.push({ altRows: await pane.page.evaluate(() => [...document.querySelectorAll('[id*="framework.history.alternative"]')].map(element => `${element.tagName}|${element.id.slice(-70)}|${(element.textContent ?? "").replace(/\s+/gu, " ").slice(0, 80)}`)) });
    const choose = async (rowText: RegExp) => {
      const result = await rowButton(pane, /framework\.history\.alternative\./u, /^(Switch|Wechseln)/u, rowText);
      await pane.page.waitForTimeout(3500);
      return { result, notices: (await pane.notices()).slice(-2), alerts: await pane.alerts() };
    };
    const toMain = await choose(language === "en" ? /Main line/u : /Haupt/u);
    log.push({ toMain });
    const gammaAfterMain = await shown("GammaText");
    const alphaAfterMain = await shown("AlphaText");
    await pane.shot(`${tag}-3-main`);
    const savedMain = await saveOrEmpty(pane, `${tag}-main.pdf`);
    const oracleMain = savedMain.bytes.byteLength > 0 ? check(savedMain.path, "has=AlphaText", "lacks=GammaText") : undefined;
    verdicts.push({ step: `${tag}: Create Alternative then Switch Alternative to the main line drops the alternative's edit`, pass: gammaShown && !gammaAfterMain && alphaAfterMain && oracleMain?.ok === true, evidence: { toMain, gammaShown, gammaAfterMain, alphaAfterMain, oracle: oracleMain?.output } });
    const toAlt = await choose(/^(?!.*(Main line|Haupt)).*(Alternative|Alternat)/u);
    log.push({ toAlt });
    const gammaBack = await waitText(pane, "GammaText", true);
    await pane.shot(`${tag}-4-alt`);
    const savedAlt = await saveOrEmpty(pane, `${tag}-alt.pdf`);
    const oracleAlt = savedAlt.bytes.byteLength > 0 ? check(savedAlt.path, "has=AlphaText", "has=GammaText") : undefined;
    verdicts.push({ step: `${tag}: Switch Alternative back restores the alternative's edit`, pass: gammaBack && oracleAlt?.ok === true, evidence: { toAlt, gammaBack, oracle: oracleAlt?.output } });
    await writeFile(join(out, `${tag}.json`), JSON.stringify(log, null, 1));
    consoleVerdict(`${tag}`, pane);
  } finally { await pane.context.close(); }
}

/** ✏️ History edit of the title mutation on a new document: Edit, change the title input, Accept, Finalize, then the finalize dialog choice (`new` alternative, `overwrite`, or `back`); verifies title in UI and saved file. */
async function titleHistoryEdit(browser: Browser, language: "en" | "de", choice: "new" | "overwrite" | "back") {
  const tag = `tedit-${choice}-${language}`;
  const pane = await session(browser, loc(language), tag);
  const log: Record<string, unknown>[] = [];
  const inv = async () => (await historyInventory(pane)).map(line => line.slice(-130)).filter(line => /editor|timeTravel|finali|dialog/u.test(line) && !/input\.content/u.test(line));
  try {
    const first = `Before ${tag}`;
    const second = `After ${tag}`;
    await pane.setTitle(language, first);
    await history(pane);
    let rowId: string | null = null;
    for (const deadline = Date.now() + 15_000; rowId === null && Date.now() < deadline; await pane.page.waitForTimeout(500)) rowId = await pane.page.evaluate(() => [...document.querySelectorAll('[id*="framework.history.entry."]')].find(element => /Set document info|Dokumentinfo/u.test(element.textContent ?? ""))?.id ?? null);
    await pane.page.locator(`[id="${rowId}"]`).first().locator("button").first().click({ force: true, timeout: 5000 });
    await pane.page.locator('[id*="framework.history.mutation."]').first().waitFor({ state: "attached", timeout: 10_000 });
    log.push({ edit: await rowButton(pane, /framework\.history\.mutation\./u, /^(Edit|Bearbeiten)/u) });
    await pane.page.waitForTimeout(3000);
    const inputs = await pane.page.evaluate(() => [...document.querySelectorAll('input[id*="framework.history.editor.input"]')].map(element => `${element.id.slice(-70)}|${element.getAttribute("aria-label")}|${(element as HTMLInputElement).value}`));
    log.push({ inputs, inv: await inv() });
    await pane.shot(`${tag}-1-editing`);
    const titleInput = pane.page.locator('input[id*="framework.history.editor.input"]').filter({ hasNot: pane.page.locator("none") }).first();
    const target = await pane.page.evaluate(word => [...document.querySelectorAll('input[id*="framework.history.editor.input"]')].find(element => (element as HTMLInputElement).value.includes(word))?.id ?? null, first);
    if (target === null) throw new Error(`no input holding the title: ${JSON.stringify(inputs)}`);
    const field = pane.page.locator(`[id="${target}"]`).first();
    await field.fill(second);
    await field.press("Tab");
    await pane.page.waitForTimeout(2500);
    const acceptDisabled = await pane.page.evaluate(() => (document.querySelector('[id$="framework.history.editor.accept"]') as HTMLButtonElement | null)?.disabled);
    log.push({ acceptDisabled, notices: (await pane.notices()).slice(-2) });
    await pane.shot(`${tag}-2-input`);
    const bar = async (name: RegExp) => { const button = pane.page.getByRole("button", { name }).last(); if (await button.count() === 0) return "absent"; await button.click({ force: true, timeout: 5000 }); await pane.page.waitForTimeout(2500); return "clicked"; };
    log.push({ accept: await bar(/^(Accept draft|Entwurf übernehmen)/u) });
    await pane.shot(`${tag}-3-accepted`);
    const buttons = () => pane.page.evaluate(() => [...document.querySelectorAll("button,[role=button],[role=menuitem]")].filter(element => (element as HTMLElement).offsetParent !== null && /edit|history|finali|accept|discard|alternative|overwrite|back|commit|abschl|verlauf|entwurf|überschreib|zurück/iu.test(`${element.id} ${element.textContent}`)).map(element => `${element.id.slice(-70)}|${(element.textContent ?? "").trim().slice(0, 40)}|${(element as HTMLButtonElement).disabled}`));
    log.push({ afterAccept: await buttons(), inv: await inv(), titleShown: await pane.titleValue(language) });
    log.push({ finalizeClick: await bar(/^(Finalize|Abschließen|Verlaufsbearbeitung abschließen)/u) });
    await pane.shot(`${tag}-4-finalize`);
    log.push({ afterFinalize: await buttons(), dialog: await pane.page.evaluate(() => [...document.querySelectorAll('[role=dialog],[role=alertdialog]')].map(element => (element.textContent ?? "").replace(/\s+/gu, " ").slice(0, 300))) });
    const dialogId = choice === "new" ? "ui.dialog.submit" : choice === "overwrite" ? "ui.dialog.choice.overwrite" : "ui.dialog.cancel";
    log.push({ choiceClick: await clickUncovered(pane.page, `[id="${dialogId}"]`) });
    await pane.page.waitForTimeout(5000);
    await pane.shot(`${tag}-5-after-choice`);
    log.push({ afterChoice: await buttons(), inv: await inv(), titleShown: await pane.titleValue(language), notices: (await pane.notices()).slice(-3), alerts: (await pane.alerts()).slice(0, 5) });
    if (choice === "back") { log.push({ exit: await clickUncovered(pane.page, '[id="ui.timeTravel.exit"]') }); await pane.page.waitForTimeout(3000); }
    const titleAfter = await pane.titleValue(language);
    let saved = await saveOrEmpty(pane, `${tag}.pdf`);
    const firstSaveBytes = saved.bytes.byteLength;
    const retries: unknown[] = [];
    if (firstSaveBytes === 0) {
      await pane.page.waitForTimeout(3000);
      saved = await saveOrEmpty(pane, `${tag}-retry.pdf`);
      retries.push({ retry: "plain second save", bytes: saved.bytes.byteLength });
      if (saved.bytes.byteLength === 0) {
        await pane.insertText("AfterFinalize");
        saved = await saveOrEmpty(pane, `${tag}-after-edit.pdf`);
        retries.push({ retry: "save after one more edit", bytes: saved.bytes.byteLength });
      }
    }
    const expected = choice === "back" ? first : second;
    const oracleResult = saved.bytes.byteLength > 0 ? check(saved.path, `title=${expected}`) : undefined;
    verdicts.push({ step: `${tag}: history edit of the title, Finalize, dialog choice ${choice}: UI title and saved file`, pass: titleAfter === expected && oracleResult?.ok === true, evidence: { choice, firstSaveBytes, retries, titleAfter, expected, oracle: oracleResult?.output, log: log.map(entry => JSON.stringify(entry).slice(0, 400)) } });
    consoleVerdict(tag, pane);
  } finally { await writeFile(join(out, `${tag}.json`), JSON.stringify(log, null, 1)); await pane.shot(`${tag}-final`).catch(() => undefined); await pane.context.close(); }
}

await runSteps(["undo-rail-en", "undo-rail-de", "undo-keys-en", "fail-en"], { ...Object.fromEntries(["en", "de"].flatMap(code => ["new", "overwrite", "back"].map(choice => [`tedit-${choice}-${code}`, (browser: Browser) => titleHistoryEdit(browser, code as "en" | "de", choice as "new")]))), ckpt: (browser: Browser) => checkpoints(browser, "en"), probeSingle, probeView, probeHistory, probeHistory2, probeHistory3, probeHistory4, "hedit-en": (browser: Browser) => historyEdit(browser, "en", "new"),
  ...Object.fromEntries(["en", "de"].flatMap(code => [[`undo-rail-${code}`, (browser: Browser) => undoRedo(browser, code as "en" | "de", "rail")], [`undo-keys-${code}`, (browser: Browser) => undoRedo(browser, code as "en" | "de", "keys")], [`fail-${code}`, (browser: Browser) => failures(browser, code as "en" | "de")]])) });
