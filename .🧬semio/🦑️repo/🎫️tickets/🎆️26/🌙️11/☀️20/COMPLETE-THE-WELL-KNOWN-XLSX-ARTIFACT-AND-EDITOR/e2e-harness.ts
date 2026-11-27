import {mkdirSync} from "node:fs";
import {createRequire} from "node:module";
import {join} from "node:path";
import type {Download, FileChooser, Page} from "playwright";

export const root = "/Users/ueli/Documents/semio";
const osDev = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev");
const {ensureRepositoryBrowserPathV1} = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
export const matrix = await import(join(osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
ensureRepositoryBrowserPathV1(root, process.env);
const {chromium} = createRequire(join(osDev, "📦️packages/🟦️typescript/package.json"))("playwright") as typeof import("playwright");

export type Verdict = {step: string; status: "pass" | "fail"; detail: string};

/** 🧭 One headless shell session with notice capture, a scripted file picker, downloads and fault capture, shared by the probe and the end-to-end journey. */
export async function session(locale: "en" | "de", serve: string, out: string) {
  mkdirSync(out, {recursive: true});
  const {awaitBeacon, dismissIntroduction, seatLocale, windowIds, readShell, unfoldActionsRail, clickUncovered} = matrix;
  const browser = await chromium.launch({args: ["--js-flags=--experimental-wasm-jspi", "--use-angle=swiftshader", "--in-process-gpu", "--disable-accelerated-2d-canvas"], env: {...process.env}});
  const page: Page = await browser.newPage({viewport: {width: 1600, height: 1000}, acceptDownloads: true, locale: locale === "de" ? "de-DE" : "en-US"});
  await page.addInitScript(() => {
    const events: string[] = [];
    (window as unknown as {__notices: string[]}).__notices = events;
    const seen = new WeakMap<Element, string>();
    new MutationObserver(() => { for (const el of document.querySelectorAll("[data-semio-transient-notice]")) { const text = `${el.getAttribute("data-notice-code") ?? ""}|${(el.textContent ?? "").trim()}`; if (seen.get(el) !== text) { seen.set(el, text); events.push(text); } } }).observe(document, {childList: true, subtree: true, attributes: true, characterData: true});
  });
  page.setDefaultNavigationTimeout(180_000);
  const downloads: Download[] = [], faults: string[] = [], verdicts: Verdict[] = [];
  const picker: {next: Uint8Array | null; name: string} = {next: null, name: ""};
  page.on("download", d => downloads.push(d));
  page.on("pageerror", e => faults.push("pageerror: " + e.message.slice(0, 300)));
  page.on("console", m => { const t = m.text(); if (/unreachable|trapped|panicked|Uncaught|dispatch-failed|refused:/i.test(t) && !/instance-(?:sealed|retired)\b/.test(t)) faults.push(`${m.type()}: ${t.slice(0, 300)}`); });
  page.on("filechooser", (c: FileChooser) => { const bytes = picker.next; picker.next = null; void c.setFiles(bytes === null ? [] : [{name: picker.name, mimeType: "application/octet-stream", buffer: Buffer.from(bytes)}]); });
  const record = (step: string, ok: boolean, detail: string) => { verdicts.push({step, status: ok ? "pass" : "fail", detail}); console.log(`[e2e ${locale}] ${ok ? "PASS" : "FAIL"} ${step}: ${detail}`); return ok; };
  const notices = (): Promise<string[]> => page.evaluate(() => (window as unknown as {__notices: string[]}).__notices ?? []);
  const shot = (name: string) => page.screenshot({path: join(out, `${name}.png`)}).catch(() => undefined);
  async function waitFor<T>(ms: number, probe: () => Promise<T | null | false | undefined>): Promise<T | null> {
    const end = Date.now() + ms;
    while (Date.now() < end) { const v = await probe(); if (v) return v as T; await page.waitForTimeout(300); }
    return null;
  }
  const railHas = async (id: string) => { await unfoldActionsRail(page).catch(() => undefined); return (await readShell(page)).actions.includes(`action.${id}`) as boolean; };
  const clickAction = async (id: string) => { await railHas(id); await clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${id}"]`); };
  const clearNotices = () => page.evaluate(() => { for (const el of document.querySelectorAll("[data-semio-transient-notice]")) el.querySelector("button")?.click(); });
  async function settleOpen(before: string[], seen: number) {
    const opened = await waitFor(120_000, async () => {
      const notes = (await notices()).slice(seen), fresh = (await windowIds(page)).filter((id: string) => !before.includes(id));
      if (fresh.length > 0 && notes.some(n => n.startsWith("shell.documentTransfer.file-opened|"))) return {opened: fresh as string[], notes};
      if (notes.some(n => n.startsWith("shell.openFile.") || n.startsWith("shell.documentTransfer.file-open-failed|"))) return {opened: [] as string[], notes};
      return null;
    });
    return opened ?? {opened: [] as string[], notes: (await notices()).slice(seen)};
  }
  async function spawnFromPalette(query: string, itemId: string) {
    const before = await windowIds(page);
    await dismissIntroduction(page);
    await page.evaluate(() => (document.activeElement instanceof HTMLElement ? document.activeElement.blur() : undefined));
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
    await input.waitFor({state: "visible", timeout: 15_000}).catch(() => undefined);
    await input.fill(query);
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${itemId}"]`).first();
    await item.waitFor({state: "visible", timeout: 8_000}).catch(() => undefined);
    if ((await item.count()) === 0) { await page.keyboard.press("Escape"); return [] as string[]; }
    await item.click({timeout: 8_000}).catch(() => item.click({force: true}));
    const fresh = await waitFor(120_000, async () => { const w = (await windowIds(page)).filter((id: string) => !before.includes(id)); return w.length > 0 ? w as string[] : null; });
    await page.waitForTimeout(3_000);
    return fresh ?? [];
  }
  async function openViaPicker(bytes: Uint8Array, name: string) {
    let bootstrap: string[] = [];
    if (!(await railHas("openArtifactFile"))) bootstrap = await spawnFromPalette("xlsx", "spawn.stdio-office.s.stdio.xlsx@ecma-376/transitional#editor");
    const before = await windowIds(page), seen = (await notices()).length;
    picker.next = bytes; picker.name = name;
    await clickAction("openArtifactFile");
    const result = await settleOpen(before, seen);
    return {...result, bootstrap};
  }
  async function closeWindows(ids: string[]) {
    for (const id of ids) await page.evaluate(wid => { const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find(el => el.getAttribute("data-window-id") === wid); (tab?.querySelector('[data-slot="mode-dock-tab-close"], button[aria-label*="lose"], button[aria-label*="chließen"]') as HTMLElement | null)?.click(); }, id);
    await waitFor(30_000, async () => (await windowIds(page)).every((id: string) => !ids.includes(id)));
  }
  async function download(trigger: () => Promise<void>, path: string) {
    const cursor = downloads.length, seen = (await notices()).length;
    await trigger();
    await waitFor(90_000, async () => downloads.length > cursor);
    const dl = downloads[cursor];
    if (!dl) return {ok: false as const, notes: (await notices()).slice(seen)};
    await dl.saveAs(path);
    return {ok: true as const, name: dl.suggestedFilename(), path, notes: (await notices()).slice(seen)};
  }
  const settingsOpen = () => page.evaluate(() => [...document.querySelectorAll('[id^="framework.settings.driver"]')].some(e => (e as HTMLElement).offsetParent !== null));
  async function boot() {
    await page.goto(serve, {waitUntil: "commit"});
    const beacon = await awaitBeacon(page, Date.now() + 300_000);
    record("boot", !!beacon?.startsWith("ready"), String(beacon));
    if (!beacon?.startsWith("ready")) throw new Error("shell not ready");
    await dismissIntroduction(page);
    await seatLocale(page, locale);
    await page.waitForTimeout(500);
    if (await settingsOpen()) await page.locator('[id="framework.settings"]').first().click({timeout: 8_000}).catch(() => undefined);
    await page.waitForTimeout(500);
    record("settings-closed", !(await settingsOpen()), "settings panel closed after seating the locale");
    const lang = await page.evaluate(() => document.documentElement.lang);
    record("locale", lang.startsWith(locale), `html.lang=${lang}`);
  }
  return {browser, page, downloads, faults, verdicts, record, notices, shot, waitFor, railHas, clickAction, clearNotices, openViaPicker, closeWindows, download, boot, spawnFromPalette, windowIds: () => windowIds(page) as Promise<string[]>};
}
