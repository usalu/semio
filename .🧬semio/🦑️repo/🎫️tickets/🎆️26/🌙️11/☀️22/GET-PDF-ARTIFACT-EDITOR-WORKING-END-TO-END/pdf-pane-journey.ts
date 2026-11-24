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
const steps = process.argv.slice(2).length > 0 ? process.argv.slice(2) : ["en", "de", "open-en", "open-de", "transfer", "details-en", "details-de"];
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
const labels = { en: { save: "Save File", open: "Open File…", insertPage: "Insert Page…", title: "Title", pageSize: ["Page width", "Page height"], range: /^Page 99 does not exist\. Enter a page number from 1 to \d+\.$/u, unknownField: /^The document has no field \/info\/nonexistent\.$/u }, de: { save: "Datei speichern", open: "Datei öffnen …", insertPage: "Seite einfügen …", title: "Titel", pageSize: ["Seitenbreite", "Seitenhöhe"], range: /^Seite 99 existiert nicht\. Geben Sie eine Seitenzahl von 1 bis \d+ ein\.$/u, unknownField: /^Das Dokument hat kein Feld \/info\/nonexistent\.$/u } };

/** 🪟️ One pane session in one browser locale with notice, console-fault and download recorders. */
async function session(browser: Browser, locale: "en-US" | "de-DE") {
  const context = await browser.newContext({ locale, acceptDownloads: true, viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  await page.addInitScript(() => {
    const seen: string[] = [];
    Object.defineProperty(window, "__journeyNotices", { value: seen });
    new MutationObserver(() => {
      for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
        const line = `${element.getAttribute("data-notice-code") ?? ""}|${(element.firstChild?.textContent ?? element.textContent ?? "").trim()}`.slice(0, 300);
        if (!seen.includes(line)) seen.push(line);
      }
    }).observe(document, { subtree: true, childList: true, characterData: true });
  });
  const faults: string[] = [];
  page.on("pageerror", error => faults.push(`pageerror: ${error.message}`.slice(0, 300)));
  const trace = join(out, `console-${locale}.log`);
  await writeFile(trace, "");
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
      found = candidates.find(candidate => pattern.test(candidate.text));
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
  const titleValue = (language: "en" | "de") => page.getByRole("textbox", { name: labels[language].title, exact: true }).first().inputValue().catch(() => "");
  return { context, page, faults, notices, alerts, noticeWithCode, open, save, shot, text, press, insertText, setTitle, titleValue, canvasHash };
}

const pdfText = (bytes: Uint8Array) => new TextDecoder("latin1").decode(bytes);
const titleInFile = (content: string, title: string) => content.includes(title) || content.includes(title.replace("ä", "\\344")) || content.includes(Buffer.from(title, "latin1").toString("hex").toUpperCase());
const isPrefix = (prefix: Uint8Array, bytes: Uint8Array) => bytes.byteLength > prefix.byteLength && prefix.every((value, index) => bytes[index] === value);
const consoleVerdict = (language: string, pane: { faults: string[] }) => verdicts.push({ step: `${language}: no console errors beyond known noise`, pass: pane.faults.length === 0, evidence: { faults: pane.faults.slice(0, 12) } });

/** ✍️ New document: shell labels in the locale, edit the title, save, check the PDF header and the title in the file. */
async function newDocument(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE");
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
    for (const deadline = Date.now() + 15_000; rangeLines.length === 0 && Date.now() < deadline; await pane.page.waitForTimeout(500)) rangeLines = [...await pane.alerts(), ...await pane.notices(), (await pane.text())].filter(line => labels[language].range.test(line));
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
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE");
  try {
    const original = new Uint8Array(await Bun.file(fixture).arrayBuffer());
    const opened = await pane.open(fixture);
    await pane.shot(`${language}-open-1-fixture`);
    const canvases = await pane.page.evaluate(() => [...document.querySelectorAll("canvas")].map(canvas => `${canvas.width}x${canvas.height}`));
    const painted = canvases.filter(size => size !== "0x0" && !size.startsWith("0x"));
    const body = await pane.text();
    const bothPages = language === "en" ? body.includes("Page 1 / 2") && body.includes("Page 2 / 2") : body.includes("Seite 1 / 2") && body.includes("Seite 2 / 2");
    verdicts.push({ step: `${language}: open the two-page fixture`, pass: opened.notice.startsWith("shell.documentTransfer.file-opened|") && painted.length > 0 && bothPages && body.includes("595") && body.includes("Original Title"), evidence: { opened, canvases, painted, bothPages, faults: pane.faults.slice(0, 8) } });
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
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE");
  try {
    const title = language === "en" ? "Details Title" : "Detailtitel";
    const before = await pane.titleValue(language);
    await pane.press("setSnapshotValue");
    for (const [key, entry] of Object.entries({ path: "/info/title", value: JSON.stringify(title) })) await pane.page.locator(`[id$="action.setSnapshotValue.arg.${key}.input"]`).first().fill(entry);
    await clickUncovered(pane.page, '[id$="action.setSnapshotValue.execute"]');
    let shown = "";
    for (const deadline = Date.now() + 20_000; !shown.includes(title) && Date.now() < deadline; await pane.page.waitForTimeout(500)) shown = await pane.titleValue(language);
    await pane.shot(`${language}-details-1-title`);
    const layer = (await pane.text()).includes(title);
    const saved = await pane.save(`${language}-details.pdf`).catch(error => ({ pressed: "failed", path: "", suggested: String(error).slice(0, 120), bytes: new Uint8Array() }));
    const inFile = titleInFile(pdfText(saved.bytes), title);
    verdicts.push({ step: `${language}: the Details pane sets the title of an untitled document`, pass: before === "" && shown.includes(title) && layer && inFile, evidence: { before, shown, layer, inFile, bytes: saved.bytes.byteLength, alerts: await pane.alerts(), faults: pane.faults.slice(0, 8) } });
    await pane.press("setSnapshotValue");
    for (const [key, entry] of Object.entries({ path: "/info/nonexistent", value: JSON.stringify("x") })) await pane.page.locator(`[id$="action.setSnapshotValue.arg.${key}.input"]`).first().fill(entry);
    await clickUncovered(pane.page, '[id$="action.setSnapshotValue.execute"]');
    const refused = await pane.noticeWithCode("stdio.pdf.unknown-field", labels[language].unknownField);
    await pane.shot(`${language}-details-2-refused`);
    verdicts.push({ step: `${language}: a refused Details-pane value shows the localized unknown-field notice as an announced alert`, pass: refused !== null && refused.role === "alert" && refused.live === "assertive", evidence: { refused, alerts: await pane.alerts(), notices: await pane.notices() } });
    consoleVerdict(`${language} details`, pane);
  } finally {
    await pane.context.close();
  }
}

/** 📤️ Export Document then Import Document: the exported archive imports back with the inserted text. */
async function transfer(browser: Browser) {
  const pane = await session(browser, "en-US");
  try {
    const body = "Transfer Round Trip";
    const inserted = await pane.insertText(body);
    const [download, pressed] = await Promise.all([pane.page.waitForEvent("download", { timeout: 90_000 }), pane.press("exportArtifactDocument")]);
    const path = join(out, "exported-document");
    await download.saveAs(path);
    const size = (await Bun.file(path).arrayBuffer()).byteLength;
    const cursor = (await pane.notices()).length;
    const [chooser] = await Promise.all([pane.page.waitForEvent("filechooser", { timeout: 30_000 }), pane.press("importArtifactDocument")]);
    await chooser.setFiles(path);
    await pane.page.waitForTimeout(8000);
    await pane.shot("transfer-1-imported");
    const text = (await pane.text()).includes(body);
    verdicts.push({ step: "export document then import document", pass: size > 0 && text, evidence: { inserted, pressed, suggested: download.suggestedFilename(), size, textAfterImport: text, notices: (await pane.notices()).slice(cursor), faults: pane.faults.slice(0, 8) } });
    consoleVerdict("transfer", pane);
  } finally {
    await pane.context.close();
  }
}

/** 🔬️ Diagnostic: after insert-text, which gesture makes the pane show the text. */
async function probe(browser: Browser) {
  const pane = await session(browser, "en-US");
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

const browser = await chromium.launch();
try {
  for (const step of steps) {
    try {
      if (step === "en" || step === "de") await newDocument(browser, step);
      else if (step === "open-en" || step === "open-de") await openReal(browser, step === "open-en" ? "en" : "de");
      else if (step === "details-en" || step === "details-de") await details(browser, step === "details-en" ? "en" : "de");
      else if (step === "probe") await probe(browser);
      else if (step === "transfer") await transfer(browser);
      else throw new Error(`Unknown journey step: ${step}`);
    } catch (error) {
      verdicts.push({ step, pass: false, evidence: { error: String(error).slice(0, 2000) } });
    }
  }
} finally {
  await browser.close();
  await writeFile(join(out, "journey.json"), `${JSON.stringify(verdicts, (_, value) => value instanceof Uint8Array ? `${value.byteLength} bytes` : value, 2)}\n`);
}
for (const verdict of verdicts) console.log(`${verdict.pass ? "PASS" : "FAIL"} ${verdict.step}`);
process.exitCode = verdicts.every(verdict => verdict.pass) ? 0 : 1;
