/** 🌐️ Drives the mounted PDF editor using the shell file chooser and page inspector. */
import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Locator, Page } from "playwright";
import { awaitBeacon, clickUncovered, dismissIntroduction, seatLocale, unfoldActionsRail, windowIds } from "@semio-tech/framework-os-dev/testing/program-matrix";

const [base, source, generated, mode] = process.argv.slice(2);
assert(base && source && generated, "Expected shell URL, PDF fixture and generated output directory");
assert(mode === undefined || mode === "large-vector" || mode === "large-text" || mode === "edit-content",
  "Expected optional large-vector, large-text or edit-content case");
type ContentEdit = {
  page: {crop: {before: number[]; after: number[]}; trim: {before: number[]; after: number[]}};
  text: {before: string; after: string};
  vector: {at: [number, number]; newX: number};
  image: {width: number; newWidth: number};
  info: {before: string; after: string; author: string};
};
const contentEdit: ContentEdit | undefined = mode === "edit-content"
  ? JSON.parse(readFileSync(new URL("./🔣️mounted-content-edit.json", import.meta.url), "utf8"))
  : undefined;
const large: {readonly byteLength: number; readonly pageCount: number} | undefined = mode === "large-vector"
  ? JSON.parse(readFileSync("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json", "utf8")).largeVector
  : mode === "large-text" ? {byteLength: 33_038, pageCount: 48} : undefined;
const reportName = contentEdit ? "pdf-mounted-content" :
  mode === "large-text" ? "pdf-mounted-large-text" : large ? "pdf-mounted-large" : "pdf-mounted";
if (large) assert.equal(readFileSync(source).byteLength, large.byteLength, "Expected the independently verified large PDF fixture");
const output = resolve(generated);
mkdirSync(output, { recursive: true });
const scratch = join(output, `${reportName}-browser-tmp`);
mkdirSync(scratch, { recursive: true });
const browserOwner = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser");
const { ensureRepositoryBrowserPathV1 } = await import(join(browserOwner, "🧰️preparation/🟦️.ts"));
const { PLAYWRIGHT_MODULE_SPECIFIER } = await import(join(browserOwner, "🟦️.ts"));
ensureRepositoryBrowserPathV1(process.cwd(), process.env);
const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const args = process.platform === "darwin" ? ["--in-process-gpu", "--disable-accelerated-2d-canvas", "--use-angle=swiftshader"] : [];
const browser = await chromium.launch({args, headless: true, downloadsPath: output, env: {...process.env, TMPDIR: scratch, TMP: scratch, TEMP: scratch}});
const context = await browser.newContext({locale: "en-US", viewport: {width: 1600, height: 1000}, acceptDownloads: true});
const page = await context.newPage();
const diagnostics: string[] = [];
page.on("pageerror", error => diagnostics.push(error.message));
page.on("console", message => {
  if (message.type() === "error" || message.type() === "warning") diagnostics.push(message.text());
});

async function windowAction(page: Page, id: string) {
  await unfoldActionsRail(page);
  return `[data-slot="window-action-pane"] [id="action.${id}"]`;
}

async function openFile(page: Page, action: string, path: string) {
  const before = await windowIds(page);
  const [picker] = await Promise.all([page.waitForEvent("filechooser", {timeout: 30_000}), clickUncovered(page, action)]);
  await picker.setFiles(resolve(path));
  const windows = await page.waitForFunction((known) => {
    const added = [...document.querySelectorAll('[data-slot="window"]')].filter(window => window.id && !known.includes(window.id));
    return added.some(window => window.querySelector('[id="pdf-page-rotation"]')) ? added.map(window => window.id) : false;
  }, before, {timeout: 60_000});
  return page.locator((await windows.jsonValue() as string[]).map(id => `[data-slot="window"][id=${JSON.stringify(id)}]`).join(","));
}

async function verifyRotation(owned: Locator, name: string, degrees: string) {
  await owned.locator('[id="pdf-object-kind"]').filter({hasText: new RegExp(`^${name}$`)}).waitFor({timeout: 60_000});
  const rotation = owned.locator('[id="pdf-page-rotation"]');
  await rotation.waitFor({state: "visible"});
  assert.equal(await rotation.inputValue(), degrees, `${name} should be at ${degrees} degrees`);
  return rotation;
}

async function inspectorField(owned: Locator, id: string, value: string) {
  const field = owned.locator(`[id="${id}"]`);
  await field.waitFor({state: "visible", timeout: 60_000});
  assert.equal(await field.inputValue(), value, id);
  return field;
}

async function commitInspectorField(owned: Locator, id: string, before: string, after: string) {
  const field = await inspectorField(owned, id, before);
  await field.fill(after);
  await field.blur();
  await field.and(owned.locator(':not([data-draft-dirty="true"])')).waitFor({timeout: 60_000});
  assert.equal(await field.inputValue(), after, id);
}

async function selectInspector(owned: Locator, name: string | RegExp, kind: string) {
  await owned.getByRole("button", {name, exact: true}).press("Enter", {timeout: 60_000});
  await owned.locator('[id="pdf-object-kind"]')
    .filter({hasText: new RegExp(`^${kind}$`)}).waitFor({timeout: 60_000});
}

async function verifyOrEditContent(owned: Locator, spec: ContentEdit, editing: boolean) {
  const phase = editing ? "before" : "after";
  const field = (id: string, before: string, after: string) => editing
    ? commitInspectorField(owned, id, before, after)
    : inspectorField(owned, id, after);
  await selectInspector(owned, `Text: ${spec.text[phase]}`, "Text");
  await field("pdf-object-text", spec.text.before, spec.text.after);
  await owned.getByRole("button", {name: `Text: ${spec.text.after}`, exact: true}).waitFor({timeout: 60_000});
  await selectInspector(owned, "Vector graphic", "Vector");
  await field("pdf-object-x", String(spec.vector.at[0]), String(spec.vector.newX));
  await selectInspector(owned, /^Image(?:$|: )/u, "Image");
  await field("pdf-object-width", String(spec.image.width), String(spec.image.newWidth));

  await selectInspector(owned, "Page 1 / 2", "Page 1");
  await field("pdf-page-title", spec.info.before, spec.info.after);
  await owned.getByText(`Title: ${spec.info.after}`, {exact: true}).waitFor({timeout: 60_000});
  await field("pdf-page-crop-box", spec.page.crop.before.join(" "), spec.page.crop.after.join(" "));
  await field("pdf-page-trim-box", spec.page.trim.before.join(" "), spec.page.trim.after.join(" "));
  await inspectorField(owned, "pdf-page-author", spec.info.author);
}

async function verifyTrimHistory(page: Page, owned: Locator, spec: ContentEdit) {
  const ids = await owned.evaluateAll(windows => windows.map(window => window.id));
  for (const [action, trim] of [["undo", spec.page.trim.before], ["redo", spec.page.trim.after]] as const) {
    await clickUncovered(page, await windowAction(page, action));
    await selectInspector(owned, "Page 1 / 2", "Page 1");
    await page.waitForFunction(({ids, expected}) => ids.some(id => {
      const field = document.getElementById(id)?.querySelector('[id="pdf-page-trim-box"]');
      return field instanceof HTMLInputElement && field.value === expected;
    }), {ids, expected: trim.join(" ")}, {timeout: 60_000});
    await inspectorField(owned, "pdf-page-crop-box", spec.page.crop.after.join(" "));
    await inspectorField(owned, "pdf-page-title", spec.info.after);
  }
}

async function saveAndReopen(page: Page) {
  const save = await windowAction(page, "saveArtifactFile");
  const [download] = await Promise.all([page.waitForEvent("download", {timeout: 60_000}), clickUncovered(page, save)]);
  const saved = join(output, `${reportName}.pdf`);
  await download.saveAs(saved);
  assert.equal(await download.failure(), null);
  assert.equal(readFileSync(saved).subarray(0, 5).toString("ascii"), "%PDF-");
  return openFile(page, await windowAction(page, "openArtifactFile"), saved);
}

async function selectSecondPage(page: Page, owned: Locator) {
  const canvas = owned.locator(".semio-canvas-2d-host canvas");
  await canvas.waitFor({state: "visible"});
  const bounds = await canvas.boundingBox();
  assert(bounds, "The page canvas has no bounds");
  const middle = {x: bounds.width / 2, y: bounds.height / 2};
  await page.mouse.move(bounds.x + middle.x, bounds.y + middle.y);
  const steps = 22;
  for (let i = 0; i < steps; i++) {
    await page.mouse.wheel(0, 100);
    await page.evaluate(() => new Promise<void>(resolveFrame => requestAnimationFrame(() => resolveFrame())));
  }
  const zoom = Math.pow(0.9, steps);
  await canvas.click({position: {x: middle.x + 150 * zoom, y: middle.y + 1010 * zoom}});
}

try {
  await page.goto(base, {waitUntil: "commit", timeout: 300_000});
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  assert(beacon?.startsWith("ready"), `The shell did not boot: ${beacon}`);
  await dismissIntroduction(page);
  assert.equal(await seatLocale(page, "en"), "en=en", "Could not select English in the shell");
  const open = await windowAction(page, "openArtifactFile");
  if (!await page.locator(open).first().isVisible()) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    const input = page.locator("[role=dialog] [data-slot=command-input]").first();
    await input.fill("pdf");
    await page.locator("[data-command-item-id=\"spawn.stdio-pdf.s.stdio.pdf@1.7/*#editor\"]").click();
    await windowAction(page, "openArtifactFile");
  }
  const owned = await openFile(page, open, source);
  await verifyRotation(owned, "Page 1", "0");
  if (contentEdit) {
    await verifyOrEditContent(owned, contentEdit, true);
    await verifyTrimHistory(page, owned, contentEdit);
    const reopened = await saveAndReopen(page);
    await verifyOrEditContent(reopened, contentEdit, false);
    await page.screenshot({path: join(output, `${reportName}.png`)});
    console.log("[DEBUG] Mounted PDF edited text, vector position, image width, title and page boxes, verified trim undo/redo, saved and reopened");
  } else if (large) {
    const lastPage = owned.getByRole("button", {name: `Page ${large.pageCount} / ${large.pageCount}`, exact: true});
    await lastPage.press("Enter", {timeout: 60_000});
    await verifyRotation(owned, `Page ${large.pageCount}`, "0");
    if (mode === "large-text") {
      const text = `Text: Page ${large.pageCount} of ${large.pageCount}`;
      await owned.getByRole("button", {name: text, exact: true}).waitFor({timeout: 60_000});
      console.log(`[DEBUG] Verified the accessible final page text: ${text}`);
    } else {
      const finalPageGraphics = owned.locator(`[id^="pdf-text-layer-p${large.pageCount - 1}-"]`)
        .and(owned.getByRole("button", {name: "Vector graphic", exact: true}));
      await finalPageGraphics.first().waitFor({timeout: 60_000});
    }
    await page.screenshot({path: join(output, `${reportName}.png`)});
    console.log(`[DEBUG] Mounted ${large.byteLength}-byte PDF and selected its final page ${large.pageCount} through the Text Layer`);
  } else {
    await selectSecondPage(page, owned);
    const rotation = await verifyRotation(owned, "Page 2", "0");
    await rotation.fill("90");
    await rotation.blur();
    await page.screenshot({path: join(output, `${reportName}.png`)});

    const reopened = await saveAndReopen(page);
    await verifyRotation(reopened, "Page 1", "0");
    const pageTwo = reopened.getByRole("button", {name: "Page 2 / 2", exact: true});
    await pageTwo.press("Enter");
    await verifyRotation(reopened, "Page 2", "90");
    console.log("[DEBUG] Mounted PDF opened, rotated page 2, saved and reopened with page 1 unchanged");
  }
} catch (error) {
  await page.screenshot({path: join(output, `${reportName}-failed.png`)}).catch(() => undefined);
  diagnostics.push(await page.locator("body").innerText({timeout: 1_500}).catch(() => ""));
  throw error;
} finally {
  writeFileSync(join(output, `${reportName}-console.json`), JSON.stringify(diagnostics, null, 2));
  await browser.close();
}
