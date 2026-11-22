import { strict as assert } from "node:assert";
import { mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";
import type { Browser, Page } from "playwright";

/** 🧭️ Standalone XLSX pane journey (port 6272): open, edit A1, save, reopen in English and German, German decimal input,
 * Strict and Transitional workbooks, and the localized refusal of a foreign file named `.xlsx`.
 * Usage: `repo <this file> [en|de|dialects|refusal ...]`. Writes screenshots, saved workbooks and `journey.json` under
 * `🗑️generated/standalone-journey`.
 * @see ./standalone-pane-journey.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const subsets = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets";
const fixtures = {
  transitional: join(root, subsets, "🧱️base/🧪️tests/🧫️fixtures/📕️reuse-marketplaces.xlsx"),
  strict: join(root, subsets, "🔒️strict/🧪️tests/🧫️fixtures/🌐️set-main-namespace-applied/➡️after.xlsx"),
};
const url = process.env.XLSX_PANE_URL ?? "http://127.0.0.1:6272/";
const out = join(import.meta.dir, "🗑️generated", "standalone-journey");
const steps = process.argv.slice(2).length > 0 ? process.argv.slice(2) : ["en", "de", "dialects", "refusal"];
const OPENED = /^shell\.(?:documentTransfer\.(?:file-opened|file-open-failed|open-cancelled|transfer-busy)|openFile\.[^|]+)\|/u;

const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
const { unzipSync, zipSync, strToU8 } = require("fflate") as typeof import("fflate");
await mkdir(out, { recursive: true });

type Verdict = { step: string; pass: boolean; evidence: Record<string, unknown> };
const verdicts: Verdict[] = [];

/** 📗️ Reads one cell of the first worksheet with fflate and Chromium's DOMParser, keeping its stored type. */
async function readCell(page: Page, bytes: Uint8Array, address: string) {
  const entries = unzipSync(bytes);
  const text = (path: string) => entries[path] === undefined ? "" : new TextDecoder().decode(entries[path]);
  const sheet = Object.keys(entries).filter(path => /^xl\/worksheets\/[^/]+\.xml$/u.test(path)).sort()[0] ?? "";
  return page.evaluate(({ workbook, worksheet, strings, address }) => {
    const parser = new DOMParser();
    const cell = Array.from(parser.parseFromString(worksheet, "application/xml").getElementsByTagNameNS("*", "c")).find(entry => entry.getAttribute("r") === address);
    const raw = cell?.getElementsByTagNameNS("*", "v")[0]?.textContent ?? null;
    const runs = (entry: Element | undefined) => Array.from(entry?.getElementsByTagNameNS("*", "t") ?? []).map(part => part.textContent).join("");
    const type = cell?.getAttribute("t") ?? (cell === undefined ? null : "n");
    const value = type === "inlineStr" ? runs(cell) : type === "s" ? runs(parser.parseFromString(strings, "application/xml").getElementsByTagNameNS("*", "si")[Number(raw)]) : raw;
    return { type, value, namespace: parser.parseFromString(workbook, "application/xml").documentElement.namespaceURI };
  }, { workbook: text("xl/workbook.xml"), worksheet: text(sheet), strings: text("xl/sharedStrings.xml"), address });
}

/** 🪟️ One pane session in one browser locale with notice, fault and download recorders. */
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
  page.on("console", message => { if (message.type() === "error" || /refused: /u.test(message.text())) faults.push(`${message.type()}: ${message.text()}`.slice(0, 300)); });
  const notices = () => page.evaluate(() => [...((window as unknown as { __journeyNotices?: string[] }).__journeyNotices ?? [])]);
  await page.goto(url);
  const beacon = await awaitBeacon(page, Date.now() + 180_000);
  assert.match(beacon ?? "", /^ready/u, `the pane must report ready, got ${beacon}`);
  await dismissIntroduction(page);
  const press = async (id: string) => {
    const selector = `[data-slot="window-action-pane"] [id="action.${id}"]`;
    if (await page.locator(selector).count() === 0) await unfoldActionsRail(page);
    return clickUncovered(page, selector);
  };
  const open = async (path: string) => {
    const cursor = (await notices()).length;
    const cells = await page.locator('input[aria-label="A1"]').count();
    const [chooser, pressed] = await Promise.all([page.waitForEvent("filechooser", { timeout: 30_000 }), press("openArtifactFile")]);
    await chooser.setFiles(path);
    let notice: string | undefined;
    for (const deadline = Date.now() + 180_000; notice === undefined && Date.now() < deadline; await page.waitForTimeout(250)) notice = (await notices()).slice(cursor).find(line => OPENED.test(line));
    if (notice?.startsWith("shell.documentTransfer.file-opened|")) await page.waitForFunction(count => document.querySelectorAll('input[aria-label="A1"]').length > count, cells, { timeout: 60_000 }).catch(() => undefined);
    return { pressed, notice, cells: await page.locator('input[aria-label="A1"]').count() };
  };
  const a1 = () => page.locator('input[aria-label="A1"]').last();
  const commit = async (text: string, expected: string) => {
    await a1().fill(text);
    await a1().press("Enter");
    await page.waitForFunction(value => {
      const cells = document.querySelectorAll<HTMLInputElement>('input[aria-label="A1"]');
      const cell = cells[cells.length - 1];
      return cell?.value === value && !cell.hasAttribute("data-draft-dirty");
    }, expected, { timeout: 30_000 });
  };
  const save = async (name: string) => {
    const [download, pressed] = await Promise.all([page.waitForEvent("download", { timeout: 90_000 }), press("saveArtifactFile")]);
    const path = join(out, name);
    await download.saveAs(path);
    return { pressed, path, suggested: download.suggestedFilename(), bytes: new Uint8Array(await Bun.file(path).arrayBuffer()) };
  };
  const shot = (name: string) => page.screenshot({ path: join(out, `${name}.png`), fullPage: false });
  const windows = () => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map(element => `${element.getAttribute("data-window-id")}|${element.getAttribute("aria-label") ?? ""}`).slice(0, 20));
  return { context, page, faults, notices, open, a1, commit, save, shot, windows };
}

/** ✍️ Opens a fixture, edits A1, saves, reopens the saved file and checks the cell in the file and in the editor. */
async function roundTrip(browser: Browser, language: "en" | "de") {
  const pane = await session(browser, language === "en" ? "en-US" : "de-DE");
  try {
    const text = language === "en" ? "Edited in English" : "Geändert auf Deutsch";
    const opened = await pane.open(fixtures.transitional);
    await pane.shot(`${language}-1-opened`);
    const original = await pane.a1().inputValue();
    await pane.commit(text, text);
    await pane.shot(`${language}-2-edited`);
    const saved = await pane.save(`${language}-edited.xlsx`);
    const savedCell = await readCell(pane.page, saved.bytes, "A1");
    const reopened = await pane.open(saved.path);
    const reopenedValue = await pane.a1().inputValue();
    await pane.shot(`${language}-3-reopened`);
    verdicts.push({ step: `${language}: open, edit A1, save, reopen`, pass: opened.notice?.startsWith("shell.documentTransfer.file-opened|") === true && original === "ID" && savedCell.value === text && reopened.notice?.startsWith("shell.documentTransfer.file-opened|") === true && reopenedValue === text, evidence: { opened, original, saved: { suggested: saved.suggested, path: saved.path, cell: savedCell }, reopened, reopenedValue, faults: pane.faults.slice(0, 8) } });
    if (language === "de") {
      await pane.commit("1,5", "1,5");
      await pane.shot("de-4-decimal-entered");
      const decimal = await pane.save("de-decimal.xlsx");
      const decimalCell = await readCell(pane.page, decimal.bytes, "A1");
      const again = await pane.open(decimal.path);
      const shown = await pane.a1().inputValue();
      await pane.shot("de-5-decimal-reopened");
      verdicts.push({ step: "de: `1,5` stored as number 1.5 and shown as `1,5`", pass: decimalCell.type === "n" && decimalCell.value === "1.5" && again.notice?.startsWith("shell.documentTransfer.file-opened|") === true && shown === "1,5", evidence: { decimalCell, again, shown, faults: pane.faults.slice(0, 8) } });
    }
  } finally {
    await pane.context.close();
  }
}

/** 🔒️ Opens the Strict and the Transitional workbook and records the notice, the editor window and the first cell. */
async function dialects(browser: Browser) {
  const pane = await session(browser, "en-US");
  try {
    for (const [dialect, path] of Object.entries(fixtures)) {
      const before = await pane.windows();
      const opened = await pane.open(path);
      const value = await pane.a1().inputValue().catch(() => null);
      const after = await pane.windows();
      await pane.shot(`dialect-${dialect}`);
      verdicts.push({ step: `open ${dialect} workbook`, pass: opened.notice?.startsWith("shell.documentTransfer.file-opened|") === true && value === (dialect === "strict" ? "Hello" : "ID"), evidence: { opened, value, newWindows: after.filter(entry => !before.includes(entry)), faults: pane.faults.slice(0, 8) } });
    }
  } finally {
    await pane.context.close();
  }
}

/** 🚫️ A plain text file and a foreign ZIP package, both named `.xlsx`, must be refused with the localized notice. */
async function refusal(browser: Browser) {
  const text = join(out, "not-a-workbook.xlsx");
  const foreign = join(out, "foreign-package.xlsx");
  await writeFile(text, "This is plain text, not a SpreadsheetML package.\n");
  await writeFile(foreign, zipSync({ "[Content_Types].xml": strToU8('<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>'), "word/document.xml": strToU8('<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>') }));
  const expected = {
    en: (file: string) => `“${file}” has the name of a known format, but its content does not match that format, so nothing was opened.`,
    de: (file: string) => `„${file}“ trägt den Namen eines bekannten Formats, aber der Inhalt passt nicht zu diesem Format, daher wurde nichts geöffnet.`,
  };
  for (const language of ["en", "de"] as const) {
    const pane = await session(browser, language === "en" ? "en-US" : "de-DE");
    try {
      for (const path of [text, foreign]) {
        const file = path.split("/").at(-1)!;
        const before = await pane.page.locator('input[aria-label="A1"]').count();
        const opened = await pane.open(path);
        await pane.shot(`refusal-${language}-${file.replace(/\.xlsx$/u, "")}`);
        verdicts.push({ step: `${language}: refuse ${file}`, pass: opened.notice === `shell.openFile.content-mismatch|${expected[language](file)}` && opened.cells === before, evidence: { opened, before, faults: pane.faults.slice(0, 8) } });
      }
    } finally {
      await pane.context.close();
    }
  }
}

const browser = await chromium.launch();
try {
  for (const step of steps) {
    try {
      if (step === "en" || step === "de") await roundTrip(browser, step);
      else if (step === "dialects") await dialects(browser);
      else if (step === "refusal") await refusal(browser);
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
