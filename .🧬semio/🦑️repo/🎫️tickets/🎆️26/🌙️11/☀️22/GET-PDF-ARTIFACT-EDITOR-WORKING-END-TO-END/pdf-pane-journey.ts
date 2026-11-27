import { strict as assert } from "node:assert";
import { appendFile, mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";
import type { Browser, Page } from "playwright";

/** 📘️ Standalone PDF 1.7 pane journey (port 6260): new document, edit the document title, insert text, save, reopen the saved file and see both edits again, open a real PDF, in English and German.
 * Usage: `repo <this file> [en|de|open-en|open-de|transfer ...]` with the host from `serve-pdf-native.sh` (or the repo route) running.
 * Writes screenshots, saved files and `journey.json` under `🗑️generated/pdf-journey`.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const fixture = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
const url = process.env.PDF_PANE_URL ?? "http://127.0.0.1:6260/";
const out = join(import.meta.dir, "🗑️generated", process.env.PDF_JOURNEY_OUT ?? "pdf-journey");
const steps = process.argv.slice(2).length > 0 ? process.argv.slice(2) : ["en", "de", "open-en", "open-de", "transfer-en", "transfer-de", "details-en", "details-de", "a11y-en", "a11y-de", "ops-en", "ops-de"];
const OPEN_BOUND_MS = Number(process.env.PDF_OPEN_BOUND_MS ?? 120_000);
const OPENED = /^shell\.(?:documentTransfer\.(?:file-opened|file-open-failed|open-cancelled|transfer-busy)|openFile\.[^|]+)\|/u;

const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
await mkdir(out, { recursive: true });

type Verdict = { step: string; pass: boolean; evidence: Record<string, unknown> };
const verdicts: Verdict[] = [];
const labels = { en: { save: "Save File", open: "Open File…", insertPage: "Insert Page…", title: "Title", pageSize: ["Page width", "Page height"], range: /^Page 99 does not exist\. Enter a page number from 1 to \d+\.$/u, unknownField: /^The document has no field nonexistent\.$/u }, de: { save: "Datei speichern", open: "Datei öffnen …", insertPage: "Seite einfügen …", title: "Titel", pageSize: ["Seitenbreite", "Seitenhöhe"], range: /^Seite 99 existiert nicht\. Geben Sie eine Seitenzahl von 1 bis \d+ ein\.$/u, unknownField: /^Das Dokument hat kein Feld nonexistent\.$/u } };

/** 🪟️ One pane session in one browser locale with notice, console-fault and download recorders. */
async function session(browser: Browser, locale: "en-US" | "de-DE", step: string) {
  const context = await browser.newContext({ locale, acceptDownloads: true, viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  await page.addInitScript(() => {
    const seen: string[] = [];
    const announced: { code: string; role: string | null; live: string | null; text: string }[] = [];
    Object.defineProperty(window, "__journeyNotices", { value: seen });
    Object.defineProperty(window, "__journeyAnnounced", { value: announced });
    new MutationObserver(() => {
      for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
        const text = (element.firstChild?.textContent ?? element.textContent ?? "").trim();
        const line = `${element.getAttribute("data-notice-code") ?? ""}|${text}`.slice(0, 300);
        if (!seen.includes(line)) { seen.push(line); announced.push({ code: element.getAttribute("data-notice-code") ?? "", role: element.getAttribute("role"), live: element.getAttribute("aria-live"), text }); }
      }
    }).observe(document, { subtree: true, childList: true, characterData: true });
  });
  const faults: string[] = [];
  const trace = join(out, `console-${step}-${locale}.log`);
  await writeFile(trace, "");
  page.on("pageerror", error => { faults.push(`pageerror: ${error.message}`.slice(0, 300)); void appendFile(trace, `pageerror: ${error.stack ?? error.message}\n`); });
  page.on("console", message => { if (!/^\[DEBUG\] (?:admit|scope|end)/u.test(message.text())) void appendFile(trace, `${message.type()}: ${message.text()}\n`); if ((message.type() === "error" || /failed|refused/u.test(message.text())) && !/semio-stream-mux|^\[DEBUG\]/u.test(message.text())) faults.push(`${message.type()}: ${message.text()}`.slice(0, 400)); });
  const notices = () => page.evaluate(() => [...((window as unknown as { __journeyNotices?: string[] }).__journeyNotices ?? [])]);
  await page.goto(url);
  const beacon = await awaitBeacon(page, Date.now() + 240_000);
  assert.match(beacon ?? "", /^ready/u, `the pane must report ready, got ${beacon}`);
  await dismissIntroduction(page);
  const press = async (id: string) => {
    const selector = `[data-slot="window-action-pane"] [id="action.${id}"]`;
    if (await page.locator(selector).count() === 0) await unfoldActionsRail(page);
    return clickUncovered(page, selector);
  };
  const open = async (path: string) => {
    const cursor = (await notices()).length;
    const [chooser, pressed] = await Promise.all([page.waitForEvent("filechooser", { timeout: 30_000 }), press("openArtifactFile")]);
    await chooser.setFiles(path);
    let notice: string | undefined;
    for (const deadline = Date.now() + OPEN_BOUND_MS; notice === undefined && Date.now() < deadline; await page.waitForTimeout(500)) notice = (await notices()).slice(cursor).find(line => OPENED.test(line));
    return { pressed, notice: notice ?? "no notice within the bound" };
  };
  const save = async (name: string) => {
    const [download, pressed] = await Promise.all([page.waitForEvent("download", { timeout: 90_000 }), press("saveArtifactFile")]);
    const path = join(out, name);
    await download.saveAs(path);
    return { pressed, path, suggested: download.suggestedFilename(), bytes: new Uint8Array(await Bun.file(path).arrayBuffer()) };
  };
  const shot = (name: string) => page.screenshot({ path: join(out, `${name}.png`) });
  const text = () => page.evaluate(() => document.body.innerText);
  const pageRows = () => page.evaluate(() => [...new Set((document.body.textContent ?? "").match(/(?:Page|Seite) \d+ \/ \d+/gu) ?? [])]);
  const canvasHash = () => page.evaluate(() => [...document.querySelectorAll("canvas")].map(canvas => { try { const data = canvas.getContext("2d")?.getImageData(0, 0, canvas.width, canvas.height).data; if (data === undefined) return `${canvas.width}x${canvas.height}:gl`; let hash = 0; for (let index = 0; index < data.length; index += 4) hash = (hash * 31 + data[index] + data[index + 1] * 3 + data[index + 2] * 7) | 0; return `${canvas.width}x${canvas.height}:${hash}`; } catch { return `${canvas.width}x${canvas.height}:unreadable`; } }));
  const insertText = async (value: string, pageNumber = "1") => {
    let pressed = await press("insert-text");
    if (await page.locator('[id$="action.insertText.arg.page.input"]').count() === 0) pressed = await press("insert-text");
    const filled: string[] = [];
    for (const [key, entry] of Object.entries({ page: pageNumber, x: "72", y: "700", text: value, height: "14" })) {
      const input = page.locator(`[id$="action.insertText.arg.${key}.input"]`).first();
      await input.fill(entry);
      filled.push(`${key}=${entry}`);
    }
    const submitted = await clickUncovered(page, '[id$="action.insertText.execute"]');
    const started = Date.now();
    while (Date.now() - started < Number(process.env.INSERT_WAIT_MS ?? 20_000) && !(await text()).includes(value)) await page.waitForTimeout(250);
    return { pressed, filled, submitted, visibleAfterMs: Date.now() - started };
  };
  const alerts = () => page.evaluate(() => [...document.querySelectorAll('[role="alert"],[role="status"],[aria-live],[data-semio-transient-notice]')].map(element => (element.textContent ?? "").trim().slice(0, 300)).filter(line => line.length > 0));
  const noticeWithCode = async (code: string, pattern: RegExp) => {
    let found: { role: string | null; live: string | null; text: string } | undefined;
    for (const deadline = Date.now() + 15_000; found === undefined && Date.now() < deadline; await page.waitForTimeout(500)) {
      const candidates = await page.evaluate((wanted) => [...document.querySelectorAll(`[data-semio-transient-notice][data-notice-code="${wanted}"]`)].map(element => ({ role: element.getAttribute("role"), live: element.getAttribute("aria-live"), text: (element.firstChild?.textContent ?? "").trim() })), code);
      found = candidates.find(candidate => pattern.test(candidate.text)) ?? (await page.evaluate((wanted) => ((window as unknown as { __journeyAnnounced?: { code: string; role: string | null; live: string | null; text: string }[] }).__journeyAnnounced ?? []).filter(entry => entry.code === wanted), code)).find(candidate => pattern.test(candidate.text));
    }
    return found ?? null;
  };
  const setTitle = async (language: "en" | "de", title: string) => {
    const field = page.getByRole("textbox", { name: labels[language].title, exact: true }).first();
    await field.waitFor({ state: "visible", timeout: 30_000 });
    await field.fill(title);
    await field.press("Enter");
    await page.waitForTimeout(1000);
  };
  const act = async (id: string, args: Record<string, string>) => {
    const camel = id.replace(/-(\w)/gu, (_, letter: string) => letter.toUpperCase());
    const first = Object.keys(args)[0];
    let pressed = await press(id);
    if (await page.locator(`[id$="action.${camel}.arg.${first}.input"]`).count() === 0) pressed = await press(id);
    for (const [key, entry] of Object.entries(args)) await page.locator(`[id$="action.${camel}.arg.${key}.input"]`).first().fill(entry);
    const form = await page.evaluate((suffix) => [...document.querySelectorAll(`[id$="${suffix}"],[id*="action."][id$=".input"]`)].map(element => `${element.id.slice(-48)}=${(element as HTMLInputElement).value}`), `action.${camel}.execute`);
    const executeState = await page.evaluate((suffix) => [...document.querySelectorAll(`[id$="${suffix}"]`)].map(element => `${(element as HTMLButtonElement).disabled}|${element.getAttribute("aria-disabled")}|${(element.textContent ?? "").trim()}`), `action.${camel}.execute`);
    const hit = await page.evaluate((suffix) => [...document.querySelectorAll(`[id$="${suffix}"]`)].map(element => { const box = element.getBoundingClientRect(); const top = document.elementFromPoint(box.left + box.width / 2, box.top + box.height / 2); return `${Math.round(box.left)},${Math.round(box.top)},${Math.round(box.width)}x${Math.round(box.height)} top=${top === element || element.contains(top) ? "self" : `${top?.tagName}#${top?.id}.${(top?.className as string | undefined)?.toString().slice(0, 60)}`}`; }), `action.${camel}.execute`);
    const covered = hit.some(line => !line.endsWith("top=self"));
    const submitted = await clickUncovered(page, `[id$="action.${camel}.execute"]`);
    await page.waitForTimeout(Number(process.env.ACT_WAIT_MS ?? 2500));
    const after = await page.evaluate(() => [...document.querySelectorAll("input")].filter(input => /rotation|drehung/iu.test(`${input.getAttribute("aria-label") ?? ""} ${input.id}`)).map(input => `${input.getAttribute("aria-label")}=${input.value}`));
    return { id, pressed, submitted, covered, hit, form, executeState, after, notices: (await notices()).slice(-4), alerts: await alerts() };
  };
  const titleValue = (language: "en" | "de") => page.getByRole("textbox", { name: labels[language].title, exact: true }).first().inputValue().catch(() => "");
  return { context, page, faults, notices, alerts, noticeWithCode, open, save, shot, text, pageRows, press, act, insertText, setTitle, titleValue, canvasHash };
}

const pdfText = (bytes: Uint8Array) => new TextDecoder("latin1").decode(bytes);
const titleInFile = (content: string, title: string) => content.includes(title) || content.includes(title.replace("ä", "\\344")) || content.includes(Buffer.from(title, "latin1").toString("hex").toUpperCase());
const isPrefix = (prefix: Uint8Array, bytes: Uint8Array) => bytes.byteLength > prefix.byteLength && prefix.every((value, index) => bytes[index] === value);
/** 🎯️ The input ledger's one console line for the refusals a step provokes on purpose and checks in its own verdict (the out-of-range insert answers `stdio.pdf.page-number-out-of-range`, shown as an announced localized notice). */
const intended = /^warning: input #\d+ insert-text refused: dispatch-failed \(user[^)]*\) — stdio\.pdf\.page-number-out-of-range$/u;
const consoleVerdict = (language: string, pane: { faults: string[] }) => {
  const unexpected = pane.faults.filter(fault => !intended.test(fault));
  verdicts.push({ step: `${language}: no console errors beyond known noise`, pass: unexpected.length === 0, evidence: { faults: unexpected.slice(0, 12), intended: pane.faults.filter(fault => intended.test(fault)) } });
};

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
    const oracle = saved.bytes.byteLength > 0 ? Bun.spawnSync(["uv", "run", "--offline", "--with", "pymupdf==1.26.7", "--with", "pypdf", "python", "-I", join(import.meta.dir, "journey-oracle.py"), "ops", saved.path, title, marker], { stdout: "pipe", stderr: "pipe" }) : undefined;
    const oracleOk = oracle?.exitCode === 0;
    verdicts.push({ step: `${language}: insert page, rectangle, text and rotation survive Save in a third-party reader`, pass: pagesShown && textShown && oracleOk && ![inserted, rectangle, rotated].some(step => step.covered), evidence: { inserted, pagesShown, rectangle, text, rotated, textShown, bytes: saved.bytes.byteLength, oracle: `${oracle?.stdout.toString() ?? ""}${oracle?.stderr.toString() ?? ""}`.slice(-1500), faults: pane.faults.slice(0, 8) } });
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

/** 🧭️ Every step gets its own browser process, so one step's tree footprint never carries into the next (the host warden cancels a headless browser tree above its cap); a disconnect that the journey did not ask for is recorded with the step it hit. */
const signals: string[] = [];
for (const signal of ["SIGTERM", "SIGHUP", "SIGINT"] as const) process.on(signal, () => signals.push(`${new Date().toISOString()} ${signal}`));
try {
  for (const step of steps) {
    const browser = await chromium.launch({ handleSIGHUP: false, handleSIGINT: false, handleSIGTERM: false });
    let closing = false;
    browser.on("disconnected", () => { if (!closing) signals.push(`${new Date().toISOString()} ${step}: browser disconnected`); });
    try {
      if (step === "en" || step === "de") await newDocument(browser, step);
      else if (step === "open-en" || step === "open-de") await openReal(browser, step === "open-en" ? "en" : "de");
      else if (step === "details-en" || step === "details-de") await details(browser, step === "details-en" ? "en" : "de");
      else if (step === "a11y-en" || step === "a11y-de") await accessibility(browser, step === "a11y-en" ? "en" : "de");
      else if (step === "probe") await probe(browser);
      else if (step === "expand") await expandProbe(browser);
      else if (step === "ops-en" || step === "ops-de") await pageOperations(browser, step === "ops-en" ? "en" : "de");
      else if (step === "transfer-en" || step === "transfer-de") await transfer(browser, step === "transfer-en" ? "en" : "de");
      else throw new Error(`Unknown journey step: ${step}`);
    } catch (error) {
      verdicts.push({ step, pass: false, evidence: { error: String(error).slice(0, 2000) } });
    } finally {
      closing = true;
      await browser.close().catch(() => undefined);
    }
  }
} finally {
  if (signals.length > 0) verdicts.push({ step: "driver signals and browser disconnects", pass: false, evidence: { signals } });
  await writeFile(join(out, "journey.json"), `${JSON.stringify(verdicts, (_, value) => value instanceof Uint8Array ? `${value.byteLength} bytes` : value, 2)}\n`);
}
for (const verdict of verdicts) console.log(`${verdict.pass ? "PASS" : "FAIL"} ${verdict.step}`);
process.exitCode = verdicts.every(verdict => verdict.pass) ? 0 : 1;
