import {mkdirSync, readFileSync, writeFileSync} from "node:fs";
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
const genuineStrict = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧪️tests/🧫️fixtures/🔒️reuse-marketplaces-strict.xlsx");
const dialectWorkbook = (dialect: Dialect, marker: string): Uint8Array => dialect === "strict" ? new Uint8Array(readFileSync(genuineStrict)) : workbook(dialect, marker);
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
const sealedRefusals: string[] = [];
const reopenLoop = Number(process.env.REOPEN_LOOP ?? 0);
let lastSaved: {text: string | null; path: string} | null = null;
const picker: {next: Uint8Array | null; name: string} = {next: null, name: ""};
page.on("download", d => downloads.push(d));
page.on("pageerror", e => faults.push("pageerror: " + e.message.slice(0, 300)));
page.on("console", m => { const t = m.text(); if (/instance-(?:sealed|retired)\b/.test(t)) { sealedRefusals.push(t.slice(0, 300)); return; } if (/unreachable|trapped|panicked|Uncaught|dispatch-failed|refused:/i.test(t)) faults.push(`${m.type()}: ${t.slice(0, 300)}`); });
let chooserEvents = 0;
page.on("filechooser", (c: FileChooser) => { chooserEvents += 1; const bytes = picker.next; picker.next = null; void c.setFiles(bytes === null ? [] : [{name: picker.name, mimeType: "application/octet-stream", buffer: Buffer.from(bytes)}]); });

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
  const choosersBefore = chooserEvents;
  const result = await settleOpen(before, seen);
  if (result.opened.length === 0) console.log(`[journey ${locale}] DIAG picker open ${name}: choosers=${chooserEvents - choosersBefore} bootstrap=${JSON.stringify(bootstrap)} rail=${JSON.stringify((await readShell(page)).actions.filter(id => /open|save/i.test(id)))} windows=${JSON.stringify((await windowIds(page)).slice(0, 8))}`);
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

page.on("console", m => { const t = m.text(); if (/os-shell|export-failed|Error:/i.test(t) && !/cft0|\[DEBUG\]|TRACE/.test(t)) console.log("[probe console]", m.type(), t.slice(0, 600)); });
try {
  await page.goto(serve, {waitUntil: "commit"});
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  console.log("beacon", beacon);
  const file = process.env.PROBE_FILE!;
  const r = await openViaPicker(new Uint8Array(readFileSync(file)), "sample.xlsx");
  console.log("PROGRAMS", JSON.stringify(await page.evaluate(()=>(window as any).__semioOsCatalogProbe?.programs?.filter((p:any)=>/xlsx/.test(p.appId)))));
  console.log("SPAWNED", JSON.stringify(await spawned()));
  const {clickUncovered,unfoldActionsRail}=await import(join(osDev,"🧪️tests/🧮️program-matrix/🟦️.ts")); await unfoldActionsRail(page);
  const acts=(await readShell(page)).actions; console.log("ACTIONS", JSON.stringify(acts.filter(a=>/xport|ave/i.test(a))));
  const exp="action.exportArtifactDocument";
  const progs=()=>page.evaluate(()=>JSON.stringify((window as any).__semioOsCatalogProbe?.spawned));
  console.log("BEFORE-CLOSE spawned", await progs());
  const wid=(await windowIds(page)); console.log("WIN", JSON.stringify(wid));
  await page.evaluate(()=>{for(const b of document.querySelectorAll('[data-slot="mode-dock-tab-close"]')) (b as HTMLElement).click();}); await page.waitForTimeout(3000);
  console.log("AFTER-CLOSE win", JSON.stringify(await windowIds(page)), "spawned", await progs());
  await unfoldActionsRail(page);
  console.log("ACTS2", JSON.stringify((await readShell(page)).actions.filter(a=>/xport/i.test(a))));
  console.log("EXPORT", exp, await clickUncovered(page, `[data-slot="window-action-pane"] [id="${exp}"]`));
  await page.waitForTimeout(8000);
  console.log("NOTICES", JSON.stringify(await notices()));
  console.log("RESULT", JSON.stringify(r).slice(0, 800));
} finally { await browser.close(); }
