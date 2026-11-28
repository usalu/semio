import type { Browser, Page } from "playwright";
import { clickUncovered, consoleVerdict, fixture, isPrefix, labels, oracle, out, pdfText, runSteps, session, titleInFile, verdicts } from "./journey-kit.ts";
import { join } from "node:path";

/** 📘️ Standalone PDF 1.7 pane journey (port 6260): new document, edit the document title, insert text, save, reopen the saved file and see both edits again, open a real PDF, export/import, Details, accessibility and page operations, in English and German.
 * Usage: `repo <this file> [en|de|open-en|open-de|transfer-en ...]` with the host from `serve-pdf-native.sh` running; `PDF_JOURNEY_OUT` names the evidence folder under `🗑️generated`.
 * @see ./🌐️pdf-browser-journey.md */

/** ✍️ New document: shell labels in the locale, edit the title, save, check the PDF header and the title in the file. */
async function newDocument(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE", "new");
  try {
    await pane.text();
    await pane.shot(`${language}-1-new`);
    const title = language === "en" ? "Edited in English" : "Geändert auf Deutsch";
    const field = pane.page.getByRole("textbox", { name: labels[language].title, exact: true }).first();
    await field.waitFor({ state: "visible", timeout: 30_000 });
    await field.fill(title);
    await field.press("Enter");
    await pane.page.waitForTimeout(1000);
    await pane.shot(`${language}-2-edited`);
    const body = `Hello ${language === "en" ? "World" : "Welt"}`;
    const canvasBefore = await pane.canvasHash();
    const inserted = await pane.insertText(body);
    const canvasAfter = await pane.canvasHash();
    const canvasChanged = JSON.stringify(canvasBefore) !== JSON.stringify(canvasAfter);
    await pane.shot(`${language}-2b-inserted`);
    const edited = await pane.text();
    const textShown = edited.includes(body);
    const sizes = await pane.page.evaluate((names) => names.map(name => (document.querySelector(`input[aria-label="${name}"]`) as HTMLInputElement | null)?.value), labels[language].pageSize);
    const pageSize = sizes[0] === "612" && sizes[1] === "792";
    const rangeAttempt = await pane.insertText("Out Of Range", "99");
    let rangeLines: string[] = [];
    for (const deadline = Date.now() + 15_000; rangeLines.length === 0 && Date.now() < deadline; await pane.page.waitForTimeout(500)) rangeLines = [...await pane.alerts(), ...(await pane.notices()).map(line => line.slice(line.indexOf("|") + 1)), (await pane.text())].filter(line => labels[language].range.test(line));
    await pane.shot(`${language}-2c-range-fault`);
    const rangeLeaked = (await pane.text()).includes("Out Of Range");
    const rangeNotice = await pane.noticeWithCode("stdio.pdf.page-number-out-of-range", labels[language].range);
    const announced = rangeNotice !== null && rangeNotice.role === "alert" && rangeNotice.live === "assertive";
    verdicts.push({ step: `${language}: an out-of-range page number shows the localized range fault as an announced notice`, pass: rangeLines.length > 0 && !rangeLeaked && rangeNotice !== null && announced, evidence: { rangeAttempt, rangeLines: rangeLines.map(line => line.slice(0, 200)), rangeNotice, announced, rangeLeaked, notices: await pane.notices(), alerts: await pane.alerts() } });
    const saved = await pane.save(`${language}-edited.pdf`).catch(error => ({ pressed: "failed", path: "", suggested: String(error).slice(0, 120), bytes: new Uint8Array() }));
    const content = pdfText(saved.bytes);
    const headerOk = content.startsWith("%PDF-1.7");
    const titleOk = titleInFile(content, title);
    verdicts.push({ step: `${language}: new document shows a page and keeps the title`, pass: pageSize && headerOk && titleOk && textShown && canvasChanged, evidence: { canvasBefore, canvasAfter, canvasChanged, label: labels[language].save, pageSize, sizes, inserted, textShown, headerOk, titleOk, suggested: saved.suggested, bytes: saved.bytes.byteLength, header: content.slice(0, 8), faults: pane.faults.slice(0, 8) } });
    if (saved.bytes.byteLength > 0) {
      const again = await pane.open(saved.path);
      const titleAgain = await pane.page.getByRole("textbox", { name: labels[language].title, exact: true }).first().inputValue().catch(() => "");
      await pane.shot(`${language}-3-reopened`);
      const textAgain = (await pane.text()).includes(body);
      verdicts.push({ step: `${language}: reopen keeps the title and the inserted text`, pass: (again.notice ?? "").startsWith("shell.documentTransfer.file-opened|") && titleAgain.includes(title) && textAgain, evidence: { notice: again.notice, titleAgain, textAgain, faults: pane.faults.slice(0, 8) } });
    }
    consoleVerdict(`${language} new document`, pane);
  } finally {
    await pane.context.close();
  }
}

/** 📂️ Opens the real two-page fixture (both pages render), edits its title and inserts text on page 1, saves: the incremental route appends to the original bytes (the original is a byte prefix of the saved file), then reopens the saved file and sees both edits again. */
async function openReal(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE", "open");
  try {
    const original = new Uint8Array(await Bun.file(fixture).arrayBuffer());
    const opened = await pane.open(fixture);
    await pane.shot(`${language}-open-1-fixture`);
    const canvases = await pane.page.evaluate(() => [...document.querySelectorAll("canvas")].map(canvas => `${canvas.width}x${canvas.height}`));
    const painted = canvases.filter(size => size !== "0x0" && !size.startsWith("0x"));
    const both = (rows: string) => language === "en" ? rows.includes("Page 1 / 2") && rows.includes("Page 2 / 2") : rows.includes("Seite 1 / 2") && rows.includes("Seite 2 / 2");
    for (const deadline = Date.now() + 15_000; !both(await pane.text()) && Date.now() < deadline;) await pane.page.waitForTimeout(500);
    const body = await pane.text();
    const bothPages = both(body);
    const rows = await pane.pageRows();
    const stale = rows.filter(row => !row.endsWith("/ 2"));
    const width = await pane.page.evaluate(() => (document.body.textContent ?? "").includes("595") || [...document.querySelectorAll("input")].some(input => input.value === "595"));
    const openedTitle = await pane.titleValue(language);
    verdicts.push({ step: `${language}: open the two-page fixture`, pass: opened.notice.startsWith("shell.documentTransfer.file-opened|") && painted.length > 0 && bothPages && stale.length === 0 && width && openedTitle === "Original Title", evidence: { opened, canvases, painted, bothPages, pageRows: rows, staleRows: stale, width, openedTitle, faults: pane.faults.slice(0, 8) } });
    const marker = `Edited ${language}`;
    const title = language === "en" ? "Edited Original" : "Geändertes Original";
    await pane.setTitle(language, title);
    const inserted = await pane.insertText(marker);
    await pane.shot(`${language}-open-2-edited`);
    const textShown = (await pane.text()).includes(marker);
    const cursor = (await pane.notices()).length;
    const saved = await pane.save(`${language}-open-edited.pdf`).catch(error => ({ pressed: "failed", path: "", suggested: String(error).slice(0, 120), bytes: new Uint8Array() }));
    await pane.page.waitForTimeout(2000);
    await pane.shot(`${language}-open-3-saved`);
    const content = pdfText(saved.bytes);
    const saveNotices = (await pane.notices()).slice(cursor);
    const valid = content.startsWith("%PDF-") && content.trimEnd().endsWith("%%EOF");
    const incremental = isPrefix(original, saved.bytes);
    const markerInFile = content.includes(marker) || content.includes(Buffer.from(marker, "latin1").toString("hex").toUpperCase()) || /\(Edited \w+\)\s*Tj/u.test(content);
    verdicts.push({ step: `${language}: edit the opened PDF and save it incrementally`, pass: textShown && valid && incremental && titleInFile(content.slice(original.byteLength), title), evidence: { inserted, textShown, valid, incremental, markerInFile, originalBytes: original.byteLength, bytes: saved.bytes.byteLength, saveNotices, alerts: await pane.alerts(), faults: pane.faults.slice(0, 8) } });
    if (saved.bytes.byteLength > 0) {
      const again = await pane.open(saved.path);
      await pane.page.waitForTimeout(1500);
      await pane.shot(`${language}-open-4-reopened`);
      const titleAgain = await pane.titleValue(language);
      const bodyAgain = await pane.text();
      const pagesAgain = language === "en" ? bodyAgain.includes("Page 2 / 2") : bodyAgain.includes("Seite 2 / 2");
      verdicts.push({ step: `${language}: reopen the incrementally saved file`, pass: again.notice.startsWith("shell.documentTransfer.file-opened|") && titleAgain.includes(title) && bodyAgain.includes(marker) && pagesAgain, evidence: { notice: again.notice, titleAgain, textAgain: bodyAgain.includes(marker), pagesAgain, faults: pane.faults.slice(0, 8) } });
    }
    consoleVerdict(`${language} opened PDF`, pane);
  } finally {
    await pane.context.close();
  }
}

/** 🗂️ Sets the title of an untitled document from the Details pane (`setSnapshotValue` on `/info/title`), then sees it in the Object pane and the Text layer and in the saved file. */
async function details(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE", "details");
  try {
    const title = language === "en" ? "Details Title" : "Detailtitel";
    const before = await pane.titleValue(language);
    await pane.press("setSnapshotValue");
    for (const [key, entry] of Object.entries({ path: "/info/title", value: title })) await pane.page.locator(`[id$="action.setSnapshotValue.arg.${key}.input"]`).first().fill(entry);
    await clickUncovered(pane.page, '[id$="action.setSnapshotValue.execute"]');
    let shown = "";
    for (const deadline = Date.now() + 20_000; !shown.includes(title) && Date.now() < deadline; await pane.page.waitForTimeout(500)) shown = await pane.titleValue(language);
    await pane.shot(`${language}-details-1-title`);
    const layer = (await pane.text()).includes(title);
    const saved = await pane.save(`${language}-details.pdf`).catch(error => ({ pressed: "failed", path: "", suggested: String(error).slice(0, 120), bytes: new Uint8Array() }));
    const inFile = titleInFile(pdfText(saved.bytes), title);
    verdicts.push({ step: `${language}: the Details pane sets the title of an untitled document`, pass: before === "" && shown.includes(title) && layer && inFile, evidence: { before, shown, layer, inFile, bytes: saved.bytes.byteLength, alerts: await pane.alerts(), faults: pane.faults.slice(0, 8) } });
    await pane.press("setSnapshotValue");
    if (await pane.page.locator('[id$="action.setSnapshotValue.arg.path.input"]').count() === 0) await pane.press("setSnapshotValue");
    for (const [key, entry] of Object.entries({ path: "/info/nonexistent", value: "x" })) await pane.page.locator(`[id$="action.setSnapshotValue.arg.${key}.input"]`).first().fill(entry);
    await clickUncovered(pane.page, '[id$="action.setSnapshotValue.execute"]');
    const refused = await pane.noticeWithCode("stdio.pdf.unknown-field", labels[language].unknownField);
    await pane.shot(`${language}-details-2-refused`);
    verdicts.push({ step: `${language}: a refused Details-pane value shows the localized unknown-field notice as an announced alert`, pass: refused !== null && refused.role === "alert" && refused.live === "assertive", evidence: { refused, alerts: await pane.alerts(), notices: await pane.notices() } });
    consoleVerdict(`${language} details`, pane);
  } finally {
    await pane.context.close();
  }
}

/** 📤️ Export Document then Import Document in `language`: the exported archive imports back with the inserted text. */
async function transfer(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE", `transfer-${language}`);
  try {
    const body = "Transfer Round Trip";
    const inserted = await pane.insertText(body);
    const [download, pressed] = await Promise.all([pane.page.waitForEvent("download", { timeout: 90_000 }), pane.press("exportArtifactDocument")]);
    const path = join(out, `exported-document-${language}`);
    await download.saveAs(path);
    const size = (await Bun.file(path).arrayBuffer()).byteLength;
    const cursor = (await pane.notices()).length;
    const [chooser] = await Promise.all([pane.page.waitForEvent("filechooser", { timeout: 30_000 }), pane.press("importArtifactDocument")]);
    await chooser.setFiles(path);
    await pane.page.waitForTimeout(8000);
    await pane.shot(`${language}-transfer-1-imported`);
    const text = (await pane.text()).includes(body);
    verdicts.push({ step: `${language}: export document then import document`, pass: size > 0 && text, evidence: { inserted, pressed, suggested: download.suggestedFilename(), size, textAfterImport: text, notices: (await pane.notices()).slice(cursor), faults: pane.faults.slice(0, 8) } });
    consoleVerdict(`${language} transfer`, pane);
  } finally {
    await pane.context.close();
  }
}

/** ♿️ The accessible names the accessibility step looks up, per language: the page strip row of page 1 and the two move rows the strip offers. */
const a11yNames = {
  en: { row: /^Page 1 \/ 2\b/u, rowSecond: /^Page 2 \/ 2\b/u, later: "Move page 1 later", earlier: "Move page 2 earlier" },
  de: { row: /^Seite 1 \/ 2\b/u, rowSecond: /^Seite 2 \/ 2\b/u, later: "Seite 1 nach hinten verschieben", earlier: "Seite 2 nach vorne verschieben" },
} as const;

/** ♿️ Accessibility tree and keyboard: the inspector titles are real headings (`getByRole("heading")` with a level, `<h2>` for the object kind), the page strip rows are tree items that start collapsed and expand with ArrowRight, the move rows are named after their page, focus is visible on every stop, and a move is committed with Enter alone (no pointer). Reads Playwright aria snapshots (`page.ariaSnapshot()`), since `page.accessibility` no longer exists. */
async function accessibility(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE", "a11y");
  const names = a11yNames[language];
  const focusState = (page: Page) => page.evaluate(() => {
    const element = document.activeElement as HTMLElement | null;
    if (element === null) return { role: null, name: "", focusVisible: false, indicator: false };
    const style = getComputedStyle(element);
    return { role: element.getAttribute("role"), name: element.getAttribute("aria-label") ?? (element.textContent ?? "").trim().slice(0, 120), focusVisible: element.matches(":focus-visible"), indicator: (style.outlineStyle !== "none" && Number.parseFloat(style.outlineWidth) > 0) || style.boxShadow !== "none" || document.querySelector('[data-shared-border-segment][data-shared-border-state="focus"]') !== null };
  });
  const rowSizes = () => pane.page.evaluate(() => [...new Set((document.body.textContent ?? "").match(/(?:Page|Seite) \d+ \/ \d+ \(\d+x\d+\)/gu) ?? [])]);
  const markerPage = () => pane.page.evaluate(() => { const lines = document.body.innerText.split("\n").map(line => line.trim()); const at = lines.findIndex(line => line.includes("Move Marker")); const heading = lines.slice(0, Math.max(at, 0)).reverse().find(line => /^(?:Page|Seite) \d+ \/ \d+$/u.test(line)); return heading === undefined ? null : Number(heading.match(/\d+/u)![0]); });
  try {
    const opened = await pane.open(fixture);
    for (const deadline = Date.now() + 30_000; !(await pane.pageRows()).some(row => /\/ 2$/u.test(row)) && Date.now() < deadline;) await pane.page.waitForTimeout(500);
    await pane.shot(`${language}-a11y-1-opened`);

    const headings = await pane.page.getByRole("heading").evaluateAll(elements => elements.map(element => ({ tag: element.tagName, level: Number(element.getAttribute("aria-level") ?? element.tagName.slice(1)), name: (element.textContent ?? "").trim(), key: element.getAttribute("data-ui-node-key") })));
    const objectKind = pane.page.locator('[data-ui-node-key="pdf-object-kind"]');
    const objectKindTag = await objectKind.count() > 0 ? await objectKind.first().evaluate(element => element.tagName) : null;
    const tree = await pane.page.ariaSnapshot();
    const headingInTree = /- heading "[^"]+" \[level=\d\]/u.test(tree);
    const levelTwo = await pane.page.getByRole("heading", { level: 2 }).count();
    verdicts.push({ step: `${language}: the inspector titles are real headings with a level in the accessibility tree`, pass: headings.length > 0 && headings.every(heading => heading.name.length > 0 && heading.level >= 1 && heading.level <= 6) && levelTwo > 0 && headingInTree && (objectKindTag === null || objectKindTag === "H2"), evidence: { opened: opened.notice, headings, levelTwo, objectKindTag, headingInTree } });

    const marker = await pane.insertText("Move Marker", "1");
    await pane.page.waitForTimeout(1500);
    const strip = pane.page.getByRole("tree").first();
    const row = strip.getByRole("treeitem", { name: names.row }).first();
    await row.waitFor({ state: "visible", timeout: 30_000 });
    const before = { expanded: await row.getAttribute("aria-expanded"), rows: await rowSizes(), markerPage: await markerPage() };
    await row.focus();
    await pane.page.keyboard.press("ArrowRight");
    const rowFocus = await focusState(pane.page);
    const expanded = await row.getAttribute("aria-expanded");
    const later = strip.getByRole("treeitem", { name: names.later, exact: true });
    await later.waitFor({ state: "visible", timeout: 10_000 }).catch(() => undefined);
    const laterShown = await later.count() === 1;
    const stripTree = await strip.ariaSnapshot();
    const expandedInTree = (language === "en" ? /treeitem "Page 1 \/ 2[^"]*"[^\n]*\[expanded\]/u : /treeitem "Seite 1 \/ 2[^"]*"[^\n]*\[expanded\]/u).test(stripTree);
    const namedInTree = stripTree.includes(`treeitem "${names.later}"`);
    verdicts.push({ step: `${language}: a page strip row starts collapsed, expands with ArrowRight and exposes its move rows by name`, pass: before.expanded === "false" && expanded === "true" && laterShown && expandedInTree && namedInTree && rowFocus.focusVisible && rowFocus.indicator, evidence: { before, expanded, rowFocus, laterShown, expandedInTree, namedInTree, stripTree: stripTree.slice(0, 1500) } });

    await pane.page.keyboard.press("ArrowDown");
    const moveFocus = await focusState(pane.page);
    await pane.shot(`${language}-a11y-2-move-row-focused`);
    await pane.page.keyboard.press("Enter");
    let moved = false;
    let after = { rows: before.rows, markerPage: before.markerPage };
    for (const deadline = Date.now() + 20_000; !moved && Date.now() < deadline; await pane.page.waitForTimeout(500)) {
      after = { rows: await rowSizes(), markerPage: await markerPage() };
      moved = before.markerPage === 1 && after.markerPage === 2;
    }
    await pane.shot(`${language}-a11y-3-moved`);
    verdicts.push({ step: `${language}: a page move row is focusable, named, visibly focused and committed with the keyboard alone`, pass: moveFocus.name.includes(names.later) && moveFocus.focusVisible && moveFocus.indicator && moved, evidence: { marker, moveFocus, moved, before, after } });

    const secondRow = strip.getByRole("treeitem", { name: names.rowSecond }).first();
    await secondRow.focus();
    await pane.page.keyboard.press("ArrowRight");
    const earlier = strip.getByRole("treeitem", { name: names.earlier, exact: true });
    await earlier.waitFor({ state: "visible", timeout: 10_000 }).catch(() => undefined);
    const earlierCount = await earlier.count();
    const expandedSecond = await secondRow.getAttribute("aria-expanded");
    const secondTree = await strip.ariaSnapshot();
    await pane.page.keyboard.press("ArrowLeft");
    const collapsed = await secondRow.getAttribute("aria-expanded");
    verdicts.push({ step: `${language}: the last page offers only its earlier move and ArrowLeft collapses its row again`, pass: earlierCount === 1 && expandedSecond === "true" && collapsed === "false", evidence: { earlierCount, expandedSecond, collapsed, secondTree: secondTree.slice(0, 1200) } });
    consoleVerdict(`${language} accessibility`, pane);
  } finally {
    await pane.context.close();
  }
}

/** 🔬️ Diagnostic: how a page strip row reacts to a pointer click, a click on its disclosure button and ArrowRight; prints the tree row DOM after each. */
async function expandProbe(browser: Browser) {
  const pane = await session(browser, "en-US", "expand");
  try {
    await pane.open(fixture);
    for (const deadline = Date.now() + 30_000; !(await pane.pageRows()).some(row => /\/ 2$/u.test(row)) && Date.now() < deadline;) await pane.page.waitForTimeout(500);
    const strip = pane.page.getByRole("tree").first();
    const row = strip.getByRole("treeitem", { name: /^Page 1 \/ 2/u }).first();
    const state = async (label: string) => ({ label, expanded: await row.getAttribute("aria-expanded"), items: await strip.getByRole("treeitem").count(), html: (await row.evaluate(element => element.outerHTML)).slice(0, 700) });
    const log: unknown[] = [await state("initial")];
    await row.locator("button").first().evaluate(element => (element as HTMLElement).click());
    await pane.page.waitForTimeout(1500);
    log.push(await state("disclosure click"));
    await row.locator("button").first().evaluate(element => (element as HTMLElement).click());
    await pane.page.waitForTimeout(1000);
    log.push(await state("disclosure click again"));
    await row.focus();
    await pane.page.keyboard.press("ArrowRight");
    await pane.page.waitForTimeout(1500);
    log.push(await state("ArrowRight"));
    await pane.insertText("Move Marker", "1");
    await pane.page.waitForTimeout(1000);
    const later = strip.getByRole("treeitem", { name: "Move page 1 later", exact: true });
    await later.waitFor({ state: "visible", timeout: 10_000 });
    const context = () => pane.page.evaluate(() => { const lines = document.body.innerText.split("\n").map(line => line.trim()); const at = lines.findIndex(line => line.includes("Move Marker")); return lines.slice(Math.max(0, at - 4), at + 3); });
    log.push({ label: "marker context before", lines: await context() });
    const moveBefore = { canvas: await pane.canvasHash(), html: (await later.evaluate(element => element.outerHTML)).slice(0, 900) };
    await later.evaluate(element => (element as HTMLElement).click());
    await pane.page.waitForTimeout(3000);
    log.push({ label: "move row click", before: moveBefore, canvasAfter: await pane.canvasHash(), rows: await strip.getByRole("treeitem").allTextContents(), after: await context() });
    const other = pane.page.locator('[role="treeitem"][aria-expanded]:not([id*="pdf-thumbnail"])').first();
    const otherId = await other.getAttribute("id");
    const otherBefore = await other.getAttribute("aria-expanded");
    await other.locator("button").first().evaluate(element => (element as HTMLElement).click());
    await pane.page.waitForTimeout(1500);
    log.push({ label: "other tree row", id: otherId, before: otherBefore, after: await pane.page.locator(`[id="${otherId}"]`).first().getAttribute("aria-expanded") });
    verdicts.push({ step: "expand probe", pass: false, evidence: { log } });
  } finally {
    await pane.context.close();
  }
}

/** 📑️ Page operations on a new document: insert a second A4 page, draw a rectangle and write text on it, rotate it by 90 degrees, save, then let a third-party reader (pymupdf and pypdf, `journey-oracle.py`) confirm two pages, the A4 box, the rotation, the text and the title, and reopen the saved file in the pane. */
async function pageOperations(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE", `ops-${language}`);
  try {
    const title = language === "en" ? "Page Operations" : "Seitenoperationen";
    const marker = language === "en" ? "Second Page" : "Zweite Seite";
    await pane.setTitle(language, title);
    const two = (body: string) => language === "en" ? body.includes("Page 2 / 2") : body.includes("Seite 2 / 2");
    const inserted = await pane.act("insert-page", { page: "2", width: "595", height: "842" });
    for (const deadline = Date.now() + 15_000; !two(await pane.text()) && Date.now() < deadline;) await pane.page.waitForTimeout(500);
    const pagesShown = two(await pane.text());
    const rectangle = await pane.act("insert-rectangle", { page: "2", x: "72", y: "600", width: "200", height: "100", red: "1", green: "0", blue: "0" });
    const text = await pane.insertText(marker, "2");
    const rotated = await pane.act("set-page-rotation", { page: "2", x: "90" });
    await pane.shot(`${language}-ops-1-edited`);
    const textShown = (await pane.text()).includes(marker);
    const saved = await pane.save(`${language}-ops.pdf`).catch(error => ({ pressed: "failed", path: "", suggested: String(error).slice(0, 120), bytes: new Uint8Array() }));
    const checked = saved.bytes.byteLength > 0 ? oracle("ops", saved.path, title, marker) : undefined;
    const oracleOk = checked?.ok === true;
    verdicts.push({ step: `${language}: insert page, rectangle, text and rotation survive Save in a third-party reader`, pass: pagesShown && textShown && oracleOk && ![inserted, rectangle, rotated].some(step => step.covered), evidence: { inserted, pagesShown, rectangle, text, rotated, textShown, bytes: saved.bytes.byteLength, oracle: checked?.output, faults: pane.faults.slice(0, 8) } });
    if (saved.bytes.byteLength > 0) {
      const again = await pane.open(saved.path);
      for (const deadline = Date.now() + 15_000; !two(await pane.text()) && Date.now() < deadline;) await pane.page.waitForTimeout(500);
      await pane.shot(`${language}-ops-2-reopened`);
      const body = await pane.text();
      const titleAgain = await pane.titleValue(language);
      verdicts.push({ step: `${language}: reopen keeps both pages, the text and the title`, pass: again.notice.startsWith("shell.documentTransfer.file-opened|") && two(body) && body.includes(marker) && titleAgain.includes(title), evidence: { notice: again.notice, pages: two(body), textAgain: body.includes(marker), titleAgain, faults: pane.faults.slice(0, 8) } });
    }
    consoleVerdict(`${language} page operations`, pane);
  } finally {
    await pane.context.close();
  }
}

/** 🔬️ Diagnostic: after insert-text, which gesture makes the pane show the text. */
async function probe(browser: Browser) {
  const pane = await session(browser, "en-US", "probe");
  try {
    const marker = "Probe Text";
    if (process.env.PROBE_TITLE && process.env.PROBE_TITLE !== "after") {
      const field = pane.page.getByRole("textbox", { name: "Title", exact: true }).first();
      await field.fill("Probe Title");
      await field.press(process.env.PROBE_TITLE === "blur" ? "Tab" : "Enter");
      await pane.page.waitForTimeout(Number(process.env.PROBE_WAIT ?? 1000));
      await pane.shot("probe-0-title");
    }
    if (process.env.PROBE_CLICK) {
      await pane.page.getByText(process.env.PROBE_CLICK, { exact: true }).first().click();
      await pane.page.waitForTimeout(3000);
    }
    if (process.env.PROBE_TWICE) {
      await pane.insertText("First Probe");
      if (process.env.PROBE_TITLE === "after") {
        const field = pane.page.getByRole("textbox", { name: "Title", exact: true }).first();
        await field.fill("Probe Title");
        await field.press("Enter");
        await pane.page.waitForTimeout(3000);
        const body = await pane.text();
        console.log("after title:", body.includes("First Probe"), body.includes("Title: Probe Title"));
      }
    }
    const inserted = await pane.insertText(marker);
    if (process.env.PROBE_POKE) {
      await pane.page.getByText(process.env.PROBE_POKE, { exact: true }).first().click();
      await pane.page.waitForTimeout(5000);
    }
    const seen: Record<string, boolean> = { afterInsert: (await pane.text()).includes(marker), first: (await pane.text()).includes("First Probe") };
    await pane.page.getByRole("button", { name: /^Viewer/u }).first().click().catch(() => undefined);
    await pane.page.waitForTimeout(3000);
    seen.afterViewer = (await pane.text()).includes(marker);
    await pane.page.getByRole("button", { name: /^Editor/u }).first().click().catch(() => undefined);
    await pane.page.waitForTimeout(3000);
    seen.afterEditor = (await pane.text()).includes(marker);
    await pane.shot("probe-1");
    verdicts.push({ step: "probe", pass: false, evidence: { inserted, seen, canvas: await pane.canvasHash() } });
  } finally {
    await pane.context.close();
  }
}


const language = (step: string) => step.endsWith("de") ? "de" : "en";
await runSteps(["en", "de", "open-en", "open-de", "transfer-en", "transfer-de", "details-en", "details-de", "a11y-en", "a11y-de", "ops-en", "ops-de"], {
  en: browser => newDocument(browser, "en"), de: browser => newDocument(browser, "de"),
  ...Object.fromEntries(["en", "de"].flatMap(code => [[`open-${code}`, (browser: Browser) => openReal(browser, language(code))], [`details-${code}`, (browser: Browser) => details(browser, language(code))], [`a11y-${code}`, (browser: Browser) => accessibility(browser, language(code))], [`ops-${code}`, (browser: Browser) => pageOperations(browser, language(code))], [`transfer-${code}`, (browser: Browser) => transfer(browser, language(code))]])),
  probe: browser => probe(browser), expand: browser => expandProbe(browser),
});
