/** 🌐 Drives local PNG insertion and selected replacement through the mounted PDF catalog. */
import assert from "node:assert/strict";
import {mkdirSync, readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import type {Locator, Page} from "playwright";
import {awaitBeacon, clickUncovered, dismissIntroduction, seatLocale, unfoldActionsRail, windowIds} from "@semio-tech/framework-os-dev/testing/program-matrix";

const [url, directory, sourcePng] = process.argv.slice(2);
assert(url && directory, "Expected staged PDF shell URL and output directory");
const output = resolve(directory);
const scratch = join(output, "browser-temp");
mkdirSync(scratch, {recursive: true});
const browserOwner = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser");
const {ensureRepositoryBrowserPathV1} = await import(join(browserOwner, "🧰️preparation/🟦️.ts"));
const {PLAYWRIGHT_MODULE_SPECIFIER} = await import(join(browserOwner, "🟦️.ts"));
ensureRepositoryBrowserPathV1(process.cwd(), process.env);
const {chromium}: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const flags = process.platform === "darwin" ? ["--in-process-gpu", "--disable-accelerated-2d-canvas", "--use-angle=swiftshader"] : [];
const browser = await chromium.launch({
  args: flags, headless: true, downloadsPath: output,
  env: {...process.env, TMPDIR: scratch, TMP: scratch, TEMP: scratch},
});
const page = await browser.newPage({locale: "en-US", viewport: {width: 1600, height: 1000}, acceptDownloads: true});
const fixture = JSON.parse(readFileSync(new URL("./image-authoring.json", import.meta.url), "utf8"));
const png = sourcePng ? resolve(sourcePng) : {name: "sample.png", mimeType: "image/png", buffer: Buffer.from(fixture.png, "base64")};
const actionRail = '[data-slot="window-action-pane"]';
const row = (id: string) => actionRail + ' [id="action.' + id + '"]';
const imageLabel = (id: number) => "Image: SemioImage" + id;

async function clickAction(target: Page, id: string) {
  await unfoldActionsRail(target);
  await clickUncovered(target, row(id));
}

async function choose(target: Page, activate: () => Promise<unknown>, file: string | typeof png) {
  const [picker] = await Promise.all([target.waitForEvent("filechooser", {timeout: 60_000}), activate()]);
  await picker.setFiles(file);
}

async function freshPdfWindows(target: Page, previous: string[]): Promise<Locator> {
  const handle = await target.waitForFunction(known => {
    const added = [...document.querySelectorAll('[data-slot="window"]')].filter(window => window.id && !known.includes(window.id));
    const found = added.some(window => window.querySelector('[data-ui-node-key="pdf-text-layer-page-1"]'));
    return found && added.map(window => window.id);
  }, previous, {timeout: 120_000});
  const ids = await handle.jsonValue();
  assert(Array.isArray(ids));
  return target.locator(ids.map(id => '[data-slot="window"][id=' + JSON.stringify(id) + ']').join(","));
}

async function openArtifact(target: Page, name: string): Promise<Locator> {
  const previous = await windowIds(target);
  await choose(target, () => clickAction(target, "openArtifactFile"), join(output, name));
  return freshPdfWindows(target, previous);
}

async function reopenKept(target: Page, identity: string): Promise<Locator> {
  await target.reload({waitUntil: "commit"});
  const beacon = await awaitBeacon(target, Date.now() + 300_000);
  assert(beacon?.startsWith("ready"), "Shell did not boot after reload: " + beacon);
  await dismissIntroduction(target);
  assert.equal(await seatLocale(target, "en"), "en=en");
  const previous = await windowIds(target);
  const selector = "[data-semio-local-document-id=" + JSON.stringify(identity) + "]";
  await target.locator(selector).first().click({timeout: 120_000});
  const restored = await freshPdfWindows(target, previous);
  await target.locator(selector + '[aria-current="true"][data-semio-local-persisted="true"]').first().waitFor({timeout: 120_000});
  return restored;
}

async function save(target: Page, name: string) {
  const [download] = await Promise.all([
    target.waitForEvent("download", {timeout: 60_000}),
    clickAction(target, "saveArtifactFile"),
  ]);
  await download.saveAs(join(output, name));
  assert.equal(await download.failure(), null);
}

function imageRows(owned: Locator, pageIndex: number) {
  return owned.locator('[data-ui-node-key^="pdf-text-layer-p' + pageIndex + '-"]')
    .and(owned.getByRole("button", {name: /^Image(?:$|: )/u}));
}

async function insert(target: Page) {
  await clickAction(target, "insert-image");
  const matrix: number[] = fixture.existing.insertPlacement;
  for (const [field, value] of Object.entries({page: 0, x: matrix[4], y: matrix[5], width: matrix[0], height: matrix[3]})) {
    const input = target.locator(row("insert-image.arg." + field)).locator("input").first();
    await input.fill(String(value));
    await input.blur();
  }
  await choose(target, () => target.locator(actionRail).getByRole("button", {name: "Execute", exact: true}).click(), png);
}

async function selectedReplace(target: Page, owned: Locator, index: number) {
  await imageRows(owned, 0).nth(index).press("Enter", {timeout: 60_000});
  const button = owned.getByRole("button", {name: "Replace Image with PNG", exact: true});
  await choose(target, () => button.click(), png);
}

try {
  await page.goto(url, {waitUntil: "commit", timeout: 300_000});
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  assert(beacon?.startsWith("ready"), "Shell did not boot: " + beacon);
  await dismissIntroduction(page);
  assert.equal(await seatLocale(page, "en"), "en=en");
  await unfoldActionsRail(page);
  if (!await page.locator(row("openArtifactFile")).first().isVisible()) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    await page.locator("[role=dialog] [data-slot=command-input]").first().fill("pdf");
    await page.locator('[data-command-item-id="spawn.stdio-pdf.s.stdio.pdf@1.7/*#editor"]').click();
  }
  const owned = await openArtifact(page, "image-source.pdf");
  await imageRows(owned, 0).nth(1).waitFor({timeout: 60_000});
  assert.equal(await imageRows(owned, 0).count(), 2);
  assert.equal(await imageRows(owned, 1).count(), 1);

  await insert(page);
  await owned.getByRole("button", {name: imageLabel(1), exact: true}).waitFor({timeout: 60_000});
  assert.equal(await imageRows(owned, 0).count(), 3);
  await save(page, "image-inserted.pdf");

  await selectedReplace(page, owned, 0);
  const replacement = owned.getByRole("button", {name: imageLabel(3), exact: true});
  await replacement.waitFor({timeout: 60_000});
  await save(page, "image-replaced.pdf");

  await clickAction(page, "undo");
  await replacement.waitFor({state: "detached", timeout: 60_000});
  assert.equal(await imageRows(owned, 0).count(), 3);
  await save(page, "image-undone.pdf");

  await clickAction(page, "redo");
  await replacement.waitFor({timeout: 60_000});
  await save(page, "image-redone.pdf");

  await selectedReplace(page, owned, 1);
  await owned.getByRole("button", {name: imageLabel(5), exact: true}).waitFor({timeout: 60_000});
  await save(page, "image-inline.pdf");

  await imageRows(owned, 0).nth(0).press("Enter");
  const [cancelled] = await Promise.all([
    page.waitForEvent("filechooser", {timeout: 60_000}),
    owned.getByRole("button", {name: "Replace Image with PNG", exact: true}).click(),
  ]);
  await cancelled.element().evaluate(node => node.dispatchEvent(new Event("cancel")));
  await save(page, "image-cancelled.pdf");

  const kept = sourcePng ? await page.locator('[data-semio-local-document-id][aria-current="true"][data-semio-local-persisted="true"]').first().getAttribute("data-semio-local-document-id") : null;
  const reopened = await openArtifact(page, "image-cancelled.pdf");
  await imageRows(reopened, 0).nth(2).waitFor({timeout: 60_000});
  assert.equal(await imageRows(reopened, 0).count(), 3);
  assert.equal(await imageRows(reopened, 1).count(), 1);
  if (sourcePng) {
    assert(kept, "Save must confirm the current kept document before reload");
    const archive = await reopenKept(page, kept);
    const restoredInline = archive.getByRole("button", {name: imageLabel(5), exact: true});
    await restoredInline.waitFor({timeout: 60_000});
    await save(page, "image-archive-reloaded.pdf");
    await clickAction(page, "undo");
    await restoredInline.waitFor({state: "detached", timeout: 60_000});
    assert.equal(await imageRows(archive, 0).count(), 3);
    await save(page, "image-archive-undo.pdf");
    await clickAction(page, "redo");
    await restoredInline.waitFor({timeout: 60_000});
    await save(page, "image-archive-redo.pdf");
    console.log("[DEBUG] Kept PDF reopened by the same UUID after reload and repeated the persisted undo and redo");
  }
  console.log("[DEBUG] Mounted PDF picker inserted PNG, replaced one shared and one inline placement, verified undo and redo, cancelled without edits, and reopened");
} finally {
  await browser.close();
}
