import { strict as assert } from "node:assert";
import { appendFile, mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";
import type { Browser, Page } from "playwright";

/** 📘️ Standalone PDF 1.7 pane journey (port 6260): new document, edit the document title, insert text, save, reopen the saved file and see both edits again, open a real PDF, in English and German.
 * Usage: `repo <this file> [en|de|open|transfer ...]` with the host from `serve-pdf-native.sh` (or the repo route) running.
 * Writes screenshots, saved files and `journey.json` under `🗑️generated/pdf-journey`.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const fixture = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
const url = process.env.PDF_PANE_URL ?? "http://127.0.0.1:6260/";
const out = join(import.meta.dir, "🗑️generated", "pdf-journey");
const steps = process.argv.slice(2).length > 0 ? process.argv.slice(2) : ["en", "de", "open", "transfer"];
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
const labels = { en: { save: "Save File", open: "Open File…", insertPage: "Insert Page…", title: "Title", pageSize: ["Page width", "Page height"] }, de: { save: "Datei speichern", open: "Datei öffnen …", insertPage: "Seite einfügen …", title: "Titel", pageSize: ["Seitenbreite", "Seitenhöhe"] } };

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
  const insertText = async (value: string) => {
    const pressed = await press("insert-text");
    const filled: string[] = [];
    for (const [key, entry] of Object.entries({ page: "1", x: "72", y: "700", text: value, height: "14" })) {
      const input = page.locator(`[id$="action.insertText.arg.${key}.input"]`).first();
      await input.fill(entry);
      filled.push(`${key}=${entry}`);
    }
    const submitted = await clickUncovered(page, '[id$="action.insertText.execute"]');
    await page.waitForTimeout(1500);
    return { pressed, filled, submitted };
  };
  return { context, page, faults, notices, open, save, shot, text, press, insertText };
}

const pdfText = (bytes: Uint8Array) => new TextDecoder("latin1").decode(bytes);

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
    const inserted = await pane.insertText(body);
    await pane.shot(`${language}-2b-inserted`);
    const edited = await pane.text();
    const textShown = edited.includes(body);
    const sizes = await pane.page.evaluate((names) => names.map(name => (document.querySelector(`input[aria-label="${name}"]`) as HTMLInputElement | null)?.value), labels[language].pageSize);
    const pageSize = sizes[0] === "612" && sizes[1] === "792";
    const saved = await pane.save(`${language}-edited.pdf`).catch(error => ({ pressed: "failed", path: "", suggested: String(error).slice(0, 120), bytes: new Uint8Array() }));
    const content = pdfText(saved.bytes);
    const headerOk = content.startsWith("%PDF-1.7");
    const titleOk = content.includes(title) || content.includes(title.replace("ä", "\\344"));
    verdicts.push({ step: `${language}: new document shows a page and keeps the title`, pass: pageSize && headerOk && titleOk && textShown, evidence: { label: labels[language].save, pageSize, sizes, inserted, textShown, headerOk, titleOk, suggested: saved.suggested, bytes: saved.bytes.byteLength, header: content.slice(0, 8), faults: pane.faults.slice(0, 8) } });
    if (saved.bytes.byteLength > 0) {
      const again = await pane.open(saved.path);
      const titleAgain = await pane.page.getByRole("textbox", { name: labels[language].title, exact: true }).first().inputValue().catch(() => "");
      await pane.shot(`${language}-3-reopened`);
      const textAgain = (await pane.text()).includes(body);
      verdicts.push({ step: `${language}: reopen keeps the title and the inserted text`, pass: (again.notice ?? "").startsWith("shell.documentTransfer.file-opened|") && titleAgain.includes(title) && textAgain, evidence: { notice: again.notice, titleAgain, textAgain, faults: pane.faults.slice(0, 8) } });
    }
  } finally {
    await pane.context.close();
  }
}

/** 📂️ Opens the real two-page fixture, expects the file-opened notice and a rendered page canvas, then edits and reopens. */
async function openReal(browser: Browser) {
  const pane = await session(browser, "en-US");
  try {
    const opened = await pane.open(fixture);
    await pane.shot("open-1-fixture");
    const canvases = await pane.page.evaluate(() => [...document.querySelectorAll("canvas")].map(canvas => `${canvas.width}x${canvas.height}`));
    const painted = canvases.filter(size => size !== "0x0" && !size.startsWith("0x"));
    const body = await pane.text();
    const bothPages = body.includes("Page 1 / 2") && body.includes("Page 2 / 2");
    verdicts.push({ step: "open the two-page fixture", pass: opened.notice.startsWith("shell.documentTransfer.file-opened|") && painted.length > 0 && bothPages && body.includes("595") && body.includes("Original Title"), evidence: { opened, canvases, painted, bothPages, faults: pane.faults.slice(0, 8) } });
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
    const [download, pressed] = await Promise.all([pane.page.waitForEvent("download", { timeout: 90_000 }), pane.press("exportDocument")]);
    const path = join(out, "exported-document");
    await download.saveAs(path);
    const size = (await Bun.file(path).arrayBuffer()).byteLength;
    const cursor = (await pane.notices()).length;
    const [chooser] = await Promise.all([pane.page.waitForEvent("filechooser", { timeout: 30_000 }), pane.press("importDocument")]);
    await chooser.setFiles(path);
    await pane.page.waitForTimeout(8000);
    await pane.shot("transfer-1-imported");
    const text = (await pane.text()).includes(body);
    verdicts.push({ step: "export document then import document", pass: size > 0 && text, evidence: { inserted, pressed, suggested: download.suggestedFilename(), size, textAfterImport: text, notices: (await pane.notices()).slice(cursor), faults: pane.faults.slice(0, 8) } });
  } finally {
    await pane.context.close();
  }
}

const browser = await chromium.launch();
try {
  for (const step of steps) {
    try {
      if (step === "en" || step === "de") await newDocument(browser, step);
      else if (step === "open") await openReal(browser);
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
