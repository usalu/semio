import { strict as assert } from "node:assert";
import { appendFile, mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";
import type { Browser, Page } from "playwright";

/** 🧰️ Shared kit of the PDF 1.7 pane journeys (port 6260): one browser session per step with notice, console-fault and download recorders, form-driven actions, Save and Open through the real Actions rail, and the verdict runner.
 * Used by `pdf-pane-journey.ts` and the action sweeps (`sweep-*.ts`); host from `serve-pdf-native.sh`.
 * @see ./🌐️pdf-browser-journey.md */


export const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
export const fixture = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
const url = process.env.PDF_PANE_URL ?? "http://127.0.0.1:6260/";
export const out = join(import.meta.dir, "🗑️generated", process.env.PDF_JOURNEY_OUT ?? "pdf-journey");
const OPEN_BOUND_MS = Number(process.env.PDF_OPEN_BOUND_MS ?? 120_000);
const OPENED = /^shell\.(?:documentTransfer\.(?:file-opened|file-open-failed|open-cancelled|transfer-busy)|openFile\.[^|]+)\|/u;

const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
export const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
export const { chromium } = require("playwright") as typeof import("playwright");
await mkdir(out, { recursive: true });

export type Verdict = { step: string; pass: boolean; evidence: Record<string, unknown> };
export const verdicts: Verdict[] = [];
export const labels = { en: { save: "Save File", open: "Open File…", insertPage: "Insert Page…", title: "Title", pageSize: ["Page width", "Page height"], range: /^Page 99 does not exist\. Enter a page number from 1 to \d+\.$/u, unknownField: /^The document has no field nonexistent\.$/u }, de: { save: "Datei speichern", open: "Datei öffnen …", insertPage: "Seite einfügen …", title: "Titel", pageSize: ["Seitenbreite", "Seitenhöhe"], range: /^Seite 99 existiert nicht\. Geben Sie eine Seitenzahl von 1 bis \d+ ein\.$/u, unknownField: /^Das Dokument hat kein Feld nonexistent\.$/u } };

/** 🪟️ One pane session in one browser locale with notice, console-fault and download recorders. */
export async function session(browser: Browser, locale: "en-US" | "de-DE", step: string) {
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

export const pdfText = (bytes: Uint8Array) => new TextDecoder("latin1").decode(bytes);
export const titleInFile = (content: string, title: string) => content.includes(title) || content.includes(title.replace("ä", "\\344")) || content.includes(Buffer.from(title, "latin1").toString("hex").toUpperCase());
export const isPrefix = (prefix: Uint8Array, bytes: Uint8Array) => bytes.byteLength > prefix.byteLength && prefix.every((value, index) => bytes[index] === value);
/** 🎯️ The input ledger's one console line for the refusals a step provokes on purpose and checks in its own verdict (the out-of-range insert answers `stdio.pdf.page-number-out-of-range`, shown as an announced localized notice). */
export const intended = /^warning: input #\d+ insert-text refused: dispatch-failed \(user[^)]*\) — stdio\.pdf\.page-number-out-of-range$/u;
export const consoleVerdict = (language: string, pane: { faults: string[] }) => {
  const unexpected = pane.faults.filter(fault => !intended.test(fault));
  verdicts.push({ step: `${language}: no console errors beyond known noise`, pass: unexpected.length === 0, evidence: { faults: unexpected.slice(0, 12), intended: pane.faults.filter(fault => intended.test(fault)) } });
};

/** 🔮️ Runs the third-party oracle (`journey-oracle.py`, pymupdf and pypdf) with `args`; returns its exit and output. */
export const oracle = (...args: string[]) => { const run = Bun.spawnSync(["uv", "run", "--offline", "--with", "pymupdf==1.26.7", "--with", "pypdf", "python", "-I", join(import.meta.dir, "journey-oracle.py"), ...args], { stdout: "pipe", stderr: "pipe" }); return { ok: run.exitCode === 0, output: `${run.stdout.toString()}${run.stderr.toString()}`.slice(-2000) }; };

/** 🧭️ Runs `steps` (argv, else `defaults`) with `handlers`, each step in its own browser process, so one step's tree footprint never carries into the next (the host warden cancels a headless browser tree above its cap); a disconnect that the journey did not ask for is recorded with the step it hit. Writes `journey.json` and sets the exit code. */
export async function runSteps(defaults: string[], handlers: Record<string, (browser: Browser) => Promise<void>>) {
  const steps = process.argv.slice(2).length > 0 ? process.argv.slice(2) : defaults;
  const signals: string[] = [];
  for (const signal of ["SIGTERM", "SIGHUP", "SIGINT"] as const) process.on(signal, () => signals.push(`${new Date().toISOString()} ${signal}`));
  try {
    for (const step of steps) {
      const handler = handlers[step];
      const browser = await chromium.launch({ handleSIGHUP: false, handleSIGINT: false, handleSIGTERM: false });
      let closing = false;
      browser.on("disconnected", () => { if (!closing) signals.push(`${new Date().toISOString()} ${step}: browser disconnected`); });
      try {
        if (handler === undefined) throw new Error(`Unknown journey step: ${step}`);
        await handler(browser);
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
}
