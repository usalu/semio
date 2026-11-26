import {mkdirSync, writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {join} from "node:path";
import type {Download, FileChooser, Page} from "playwright";

const root = "/Users/ueli/Documents/semio";
const osDev = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev");
const shellHost = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost");
const [locale = "en", serve = "http://127.0.0.1:6070/", outArg] = process.argv.slice(2) as [("en" | "de")?, string?, string?];
const out = outArg ?? join(import.meta.dir, "🗑️generated/claude-full-s/journey-" + locale);
mkdirSync(out, {recursive: true});

const {ensureRepositoryBrowserPathV1} = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const {awaitBeacon, dismissIntroduction, seatLocale, windowIds, readShell, unfoldActionsRail, clickUncovered} = await import(join(osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const {pickedFileRefusalTextV1} = await import(join(shellHost, "📂️picked-file-opening/🟦️.ts"));
const {readXlsx} = await import(join(root, "🌎️hub/🧪️tests/🚪️io-matrix/🟦️.ts"));
const require = createRequire(join(root, "🌎️hub/package.json"));
const {zipSync, strToU8} = require("fflate") as typeof import("fflate");
ensureRepositoryBrowserPathV1(root, process.env);
const {chromium} = createRequire(join(osDev, "📦️packages/🟦️typescript/package.json"))("playwright") as typeof import("playwright");

type Dialect = "strict" | "transitional";
const spaces: Record<Dialect, {main: string; rel: string}> = {
  transitional: {main: "http://schemas.openxmlformats.org/spreadsheetml/2006/main", rel: "http://schemas.openxmlformats.org/officeDocument/2006/relationships"},
  strict: {main: "http://purl.oclc.org/ooxml/spreadsheetml/main", rel: "http://purl.oclc.org/ooxml/officeDocument/relationships"},
};
const mainType = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
const sheetType = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
const pkg = "http://schemas.openxmlformats.org/package/2006";
function workbook(dialect: Dialect, a1: string): Uint8Array {
  const ns = spaces[dialect], head = '<?xml version="1.0" encoding="UTF-8"?>';
  const parts: Record<string, string> = {
    "[Content_Types].xml": `<Types xmlns="${pkg}/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="${mainType}"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="${sheetType}"/></Types>`,
    "_rels/.rels": `<Relationships xmlns="${pkg}/relationships"><Relationship Id="rId1" Type="${ns.rel}/officeDocument" Target="xl/workbook.xml"/></Relationships>`,
    "xl/workbook.xml": `<workbook xmlns="${ns.main}" xmlns:r="${ns.rel}"${dialect === "strict" ? ' conformance="strict"' : ""}><sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/></sheets></workbook>`,
    "xl/_rels/workbook.xml.rels": `<Relationships xmlns="${pkg}/relationships"><Relationship Id="rId1" Type="${ns.rel}/worksheet" Target="worksheets/sheet1.xml"/></Relationships>`,
    "xl/worksheets/sheet1.xml": `<worksheet xmlns="${ns.main}"><sheetData><row r="1"><c r="A1" t="inlineStr"><is><t xml:space="preserve">${a1}</t></is></c><c r="B1"><v>42</v></c></row></sheetData></worksheet>`,
  };
  return zipSync(Object.fromEntries(Object.entries(parts).map(([k, v]) => [k, strToU8(head + v)])), {level: 6, mtime: new Date(1980, 0, 1)});
}
const foreignDocx = zipSync({"[Content_Types].xml": strToU8(`<Types xmlns="${pkg}/content-types"><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>`), "word/document.xml": strToU8('<w:document xmlns:w="urn:w"/>')});
const notZip = strToU8("this is plainly not a workbook");

type Verdict = {step: string; status: "pass" | "fail"; detail: string};
const verdicts: Verdict[] = [];
const record = (step: string, ok: boolean, detail: string) => { verdicts.push({step, status: ok ? "pass" : "fail", detail}); console.log(`[journey ${locale}] ${ok ? "PASS" : "FAIL"} ${step}: ${detail}`); return ok; };

const browser = await chromium.launch({args: ["--js-flags=--experimental-wasm-jspi", "--use-angle=swiftshader", "--in-process-gpu", "--disable-accelerated-2d-canvas"], env: {...process.env}});
const page: Page = await browser.newPage({viewport: {width: 1440, height: 900}, acceptDownloads: true, locale: locale === "de" ? "de-DE" : "en-US"});
await page.addInitScript(() => {
  const events: string[] = [];
  (window as unknown as {__notices: string[]}).__notices = events;
  const seen = new WeakMap<Element, string>();
  new MutationObserver(() => { for (const el of document.querySelectorAll("[data-semio-transient-notice]")) { const text = `${el.getAttribute("data-notice-code") ?? ""}|${(el.textContent ?? "").trim()}`; if (seen.get(el) !== text) { seen.set(el, text); events.push(text); } } }).observe(document, {childList: true, subtree: true, attributes: true, characterData: true});
});
page.setDefaultNavigationTimeout(180_000);
const downloads: Download[] = [];
const faults: string[] = [];
const picker: {next: Uint8Array | null; name: string} = {next: null, name: ""};
page.on("download", d => downloads.push(d));
page.on("pageerror", e => faults.push("pageerror: " + e.message.slice(0, 300)));
page.on("console", m => { const t = m.text(); if (/unreachable|trapped|panicked|Uncaught|dispatch-failed|refused:/i.test(t)) faults.push(`${m.type()}: ${t.slice(0, 300)}`); });
page.on("filechooser", (c: FileChooser) => { const bytes = picker.next; picker.next = null; void c.setFiles(bytes === null ? [] : [{name: picker.name, mimeType: "application/octet-stream", buffer: Buffer.from(bytes)}]); });

const notices = () => page.evaluate(() => (window as unknown as {__notices: string[]}).__notices ?? []);
const spawned = () => page.evaluate(() => (window as unknown as {__semioOsCatalogProbe?: {spawned: {id: string; pluginId: string; appId: string}[]}}).__semioOsCatalogProbe?.spawned ?? []);
const shot = (name: string) => page.screenshot({path: join(out, `${name}.png`)}).catch(() => undefined);

async function waitFor<T>(label: string, ms: number, probe: () => Promise<T | null | false | undefined>): Promise<T | null> {
  const end = Date.now() + ms;
  while (Date.now() < end) { const v = await probe(); if (v) return v as T; await page.waitForTimeout(300); }
  return null;
}
async function railHas(id: string): Promise<boolean> { await unfoldActionsRail(page).catch(() => undefined); return (await readShell(page)).actions.includes(`action.${id}`); }

async function spawnEditor(appId: string): Promise<string[] | string> {
  const program = (await page.evaluate(() => (window as unknown as {__semioOsCatalogProbe?: {programs: {pluginId: string; appId: string}[]}}).__semioOsCatalogProbe?.programs ?? [])).find(p => p.appId === appId);
  if (!program) return `no catalog program ${appId}`;
  const before = await windowIds(page);
  await dismissIntroduction(page);
  await page.evaluate(() => (document.activeElement instanceof HTMLElement ? document.activeElement.blur() : undefined));
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({state: "visible", timeout: 15_000}).catch(() => undefined);
  if ((await input.count()) === 0) return "command palette never opened";
  await input.fill("xlsx");
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${program.pluginId}.${program.appId}"]`).first();
  await item.waitFor({state: "visible", timeout: 8_000}).catch(() => undefined);
  if ((await item.count()) === 0) { await page.keyboard.press("Escape"); return `no palette row spawn.${program.pluginId}.${program.appId}`; }
  await item.click({timeout: 8_000}).catch(() => item.click({force: true}));
  const fresh = await waitFor("spawned", 120_000, async () => { const w = (await windowIds(page)).filter(id => !before.includes(id)); return w.length > 0 ? w : null; });
  if (!fresh) return "no new window after spawn";
  await page.waitForTimeout(3_500);
  return fresh;
}
async function openViaPicker(bytes: Uint8Array, name: string, editorAppId = "s.stdio.xlsx@ecma-376/transitional#editor"): Promise<{opened: string[]; notes: string[]}> {
  let bootstrap: string[] = [];
  if (!(await railHas("openArtifactFile"))) {
    const spawnedWindows = await spawnEditor(editorAppId);
    if (typeof spawnedWindows === "string") return {opened: [], notes: [`no openArtifactFile rail row and bootstrap editor failed: ${spawnedWindows}`]};
    bootstrap = spawnedWindows;
    if (!(await railHas("openArtifactFile"))) return {opened: [], notes: ["no openArtifactFile rail row in the bootstrap editor"]};
  }
  const before = await windowIds(page), seen = (await notices()).length;
  picker.next = bytes; picker.name = name;
  await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.openArtifactFile"]');
  const result = await settleOpen(before, seen);
  if (bootstrap.length > 0) await closeWindows(bootstrap);
  return result;
}
async function openViaDrop(bytes: Uint8Array, name: string): Promise<{opened: string[]; notes: string[]}> {
  const before = await windowIds(page), seen = (await notices()).length;
  await page.evaluate(({b64, name}) => {
    const raw = Uint8Array.from(atob(b64), c => c.charCodeAt(0));
    const dt = new DataTransfer();
    dt.items.add(new File([raw], name, {type: ""}));
    for (const type of ["dragenter", "dragover", "drop"]) document.body.dispatchEvent(new DragEvent(type, {dataTransfer: dt, bubbles: true, cancelable: true}));
  }, {b64: Buffer.from(bytes).toString("base64"), name});
  return settleOpen(before, seen);
}
async function settleOpen(before: string[], seen: number) {
  const end = Date.now() + 120_000;
  while (Date.now() < end) {
    const notes = (await notices()).slice(seen);
    const opened = (await windowIds(page)).filter(id => !before.includes(id));
    if (opened.length > 0 && notes.some(n => n.startsWith("shell.documentTransfer.file-opened|"))) return {opened, notes};
    if (notes.some(n => n.startsWith("shell.openFile.") || n.startsWith("shell.documentTransfer.file-open-failed|"))) return {opened, notes};
    await page.waitForTimeout(300);
  }
  return {opened: [], notes: (await notices()).slice(seen)};
}
async function closeWindows(ids: string[]) {
  for (const id of ids) await page.evaluate(wid => { const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find(el => el.getAttribute("data-window-id") === wid); (tab?.querySelector('[data-slot="mode-dock-tab-close"], button[aria-label*="lose"], button[aria-label*="chließen"]') as HTMLElement | null)?.click(); }, id);
  await waitFor("closed", 30_000, async () => (await windowIds(page)).every(id => !ids.includes(id)));
}
async function cellValue() { return page.locator('input[aria-label="A1"]').first().inputValue().catch(() => null); }

try {
  await page.goto(serve, {waitUntil: "commit"});
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  record("boot", !!beacon?.startsWith("ready"), String(beacon));
  if (!beacon?.startsWith("ready")) throw new Error("shell not ready");
  await dismissIntroduction(page);
  const seated = await seatLocale(page, locale);
  const lang = await page.evaluate(() => document.documentElement.lang);
  record("locale", lang.startsWith(locale), `seat=${seated} html.lang=${lang}`);
  await shot("00-shell");
  const probe = await page.evaluate(() => (window as unknown as {__semioOsCatalogProbe?: {plugins: {pluginId: string; status: string}[]; programs: {pluginId: string; appId: string}[]}}).__semioOsCatalogProbe ?? null);
  record("catalog", !!probe && probe.programs.some(p => p.appId.startsWith("s.stdio.xlsx@")), JSON.stringify({plugins: probe?.plugins, xlsx: probe?.programs.filter(p => p.appId.includes("xlsx")).map(p => p.appId)}));

  for (const dialect of ["transitional", "strict"] as Dialect[]) {
    for (const via of ["picker", "drop"] as const) {
      const tag = `${dialect}-${via}`, name = `Quarterly-Budget-${dialect}-${via}.xlsx`, marker = `Original ${dialect} ${via}`, edited = `Edited ${dialect} ${via} ${locale}`;
      const {opened, notes} = await (via === "picker" ? openViaPicker(workbook(dialect, marker), name, `s.stdio.xlsx@ecma-376/${dialect}#editor`) : openViaDrop(workbook(dialect, marker), name));
      const wanted = `s.stdio.xlsx@ecma-376/${dialect}#editor`;
      const owner = (await spawned()).filter(s => s.appId === wanted);
      record(`${tag}:open`, opened.length > 0 && owner.length > 0, `windows=${opened.length} owner=${owner.map(o => o.appId).join(",") || "none"} notes=${JSON.stringify(notes.slice(0, 2))}`);
      if (opened.length === 0) { await shot(`${tag}-open-failed`); continue; }
      await page.waitForTimeout(1500);
      const a1 = await waitFor("a1", 60_000, async () => (await cellValue()) !== null ? true : null);
      const initial = await cellValue();
      record(`${tag}:mount`, !!a1 && initial === marker, `A1=${JSON.stringify(initial)} expected=${JSON.stringify(marker)}`);
      await shot(`${tag}-mounted`);
      const cell = page.locator('input[aria-label="A1"]').first();
      await cell.fill(edited); await cell.press("Enter");
      const clean = await waitFor("clean", 30_000, async () => page.evaluate(v => { const c = document.querySelector<HTMLInputElement>('input[aria-label="A1"]'); return c?.value === v && !c.hasAttribute("data-draft-dirty"); }, edited));
      record(`${tag}:edit`, !!clean, `A1=${JSON.stringify(await cellValue())}`);
      const cursor = downloads.length, seenSave = (await notices()).length;
      await railHas("saveArtifactFile");
      await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.saveArtifactFile"]');
      await waitFor("download", 60_000, async () => downloads.length > cursor);
      const dl = downloads[cursor];
      const saveNotes = (await notices()).slice(seenSave);
      if (dl) {
        const path = join(out, `saved-${tag}.xlsx`);
        await dl.saveAs(path);
        const wb = await readXlsx(page, new Uint8Array(await Bun.file(path).arrayBuffer()), "A1");
        record(`${tag}:save-name`, dl.suggestedFilename() === name, `suggested=${dl.suggestedFilename()} expected=${name}`);
        record(`${tag}:save-content`, wb.valid === true && wb.text === edited, `valid=${wb.valid} A1=${JSON.stringify(wb.text)} notes=${JSON.stringify(saveNotes.slice(0, 2))}`);
      } else record(`${tag}:save-name`, false, `no download; notes=${JSON.stringify(saveNotes.slice(0, 3))}`);
      const kept = await waitFor("kept", 60_000, async () => page.evaluate(() => document.querySelector('[data-semio-local-document-id][aria-current="true"]')?.getAttribute("data-semio-local-document-id") ?? null));
      await closeWindows(opened);
      if (!kept) { record(`${tag}:reopen`, false, "no kept document id"); continue; }
      const button = page.locator(`[data-semio-local-document-id=${JSON.stringify(kept)}]`).first();
      await button.waitFor({state: "visible", timeout: 120_000}).catch(() => undefined);
      const before = await windowIds(page), seenReopen = (await notices()).length;
      await button.click().catch(() => undefined);
      const reopened = await waitFor("reopen", 120_000, async () => { const w = (await windowIds(page)).filter(id => !before.includes(id)); const n = (await notices()).slice(seenReopen); return w.length > 0 && n.some(x => x.startsWith("shell.documentTransfer.file-opened|")) ? w : null; });
      await page.waitForTimeout(2000);
      const after = await waitFor("a1-reopen", 60_000, async () => (await cellValue()) === edited ? true : null);
      record(`${tag}:reopen`, !!reopened && !!after, `windows=${reopened?.length ?? 0} A1=${JSON.stringify(await cellValue())} expected=${JSON.stringify(edited)}`);
      await shot(`${tag}-reopened`);
      if (reopened) await closeWindows(reopened);
    }
  }

  for (const [label, bytes, fname, reason] of [["foreign-docx", foreignDocx, "Foreign-Document.xlsx", "content-mismatch"], ["not-a-zip", notZip, "Broken.xlsx", "content-mismatch"], ["unknown-extension", notZip, "mystery.zzz", "unresolved"]] as const) {
    for (const via of ["picker", "drop"] as const) {
      const windowsBefore = await windowIds(page);
      const res = await (via === "picker" ? openViaPicker : openViaDrop)(bytes, fname);
      const windowsAfter = await windowIds(page);
      const text = res.notes.join(" || ");
      const refused = res.notes.some(n => n.startsWith("shell.openFile.") || n.startsWith("shell.documentTransfer.file-open-failed|"));
      let expected = "";
      try { expected = pickedFileRefusalTextV1(reason as never, fname, locale); } catch { expected = ""; }
      const localized = expected !== "" && res.notes.some(n => n.includes(expected));
      record(`refusal:${label}:${via}`, refused && res.opened.length === 0 && windowsBefore.length === windowsAfter.length, `notices=${JSON.stringify(res.notes.slice(0, 2))} localizedExpected=${localized ? "match" : JSON.stringify(expected.slice(0, 80))}`);
      if (via === "drop") await shot(`refusal-${label}`);
      void text;
    }
  }
  record("faults", faults.length === 0, JSON.stringify([...new Set(faults)].slice(0, 6)));
} catch (error) {
  record("fatal", false, String(error).split("\n")[0]!.slice(0, 300));
  await shot("fatal");
} finally {
  writeFileSync(join(out, "verdicts.json"), JSON.stringify({locale, serve, verdicts}, null, 1));
  writeFileSync(join(out, "table.md"), ["| step | verdict | detail |", "| --- | --- | --- |", ...verdicts.map(v => `| ${v.step} | ${v.status} | ${v.detail.replace(/\|/g, "/")} |`)].join("\n") + "\n");
  await browser.close();
  process.exitCode = verdicts.every(v => v.status === "pass") ? 0 : 1;
}
