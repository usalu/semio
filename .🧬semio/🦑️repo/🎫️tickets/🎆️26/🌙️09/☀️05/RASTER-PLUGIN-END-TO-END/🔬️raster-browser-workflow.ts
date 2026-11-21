import assert from "node:assert/strict";
import { mkdir, mkdtemp, rm, rmdir } from "node:fs/promises";
import { join } from "node:path";
import { chromium, type Browser, type Download, type Locator, type Page } from "playwright";
import sharp from "sharp";
import { importPayloadChunks } from "@semio-tech/framework";
import { decodeDocumentArchiveBytes } from "@semio-tech/framework-os";
import { EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID } from "@semio-tech/framework-manifest/definitions-action-ids-and-shell-brand";
import { clickUncovered, dismissIntroduction, seatLocale, unfoldActionsRail, windowIds } from "@semio-tech/framework-os-dev/testing/program-matrix";

const address = process.argv[2] ?? "http://127.0.0.1:6060/?plugin=raster";
const composite = '[data-surface-id$=":raster-composite"]';

async function boot(page: Page) {
  const response = await page.goto(address, { waitUntil: "domcontentloaded" });
  assert(response?.ok(), "Raster preview did not return a successful response");
  await page.waitForFunction(() => document.querySelector("[data-semio-os-ready],[data-semio-os-error]"), null, { timeout: 90_000 });
  assert.equal(await page.locator("[data-semio-os-error]").count(), 0, "Raster shell reported a startup error");
  await dismissIntroduction(page);
  await seatLocale(page, "en");
  await page.locator(composite).first().locator("canvas").first().waitFor({ state: "visible" });
}

async function action(page: Page, verb: string) {
  const selector = `[data-slot="window-action-pane"] [id="action.${verb}"]:visible`;
  const row = page.locator(selector).first();
  if (!(await row.isVisible())) await unfoldActionsRail(page);
  await row.waitFor({ state: "visible" });
  const staged = (await row.textContent())?.trim().endsWith("…");
  assert.equal(await clickUncovered(page, selector), "ok", `Missing Raster action ${verb}`);
  if (staged) {
    const stage = `[data-slot="window-action-pane"] [id$=".action.${verb}.execute"]:visible`;
    await page.locator(stage).first().waitFor({ state: "visible", timeout: 8_000 });
    assert.equal(await clickUncovered(page, stage), "ok", `Could not execute ${verb}`);
  }
}

async function readDownload(download: Download) {
  const chunks: Buffer[] = [];
  for await (const chunk of await download.createReadStream()) chunks.push(Buffer.from(chunk));
  return Buffer.concat(chunks);
}

async function downloadAction(page: Page, verb: string) {
  const [download] = await Promise.all([page.waitForEvent("download", { timeout: 90_000 }), action(page, verb)]);
  return { name: download.suggestedFilename(), bytes: await readDownload(download) };
}

async function png(page: Page) {
  const { name, bytes } = await downloadAction(page, "exportPng");
  assert.match(name, /\.png$/i);
  const decoded = await sharp(bytes).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
  assert(decoded.info.width > 0 && decoded.info.height > 0);
  return decoded;
}

async function importPng(page: Page) {
  const image = await sharp({
    create: { width: 256, height: 256, channels: 4, background: { r: 23, g: 120, b: 80, alpha: 1 } },
  }).png({ compressionLevel: 0 }).toBuffer();
  const payload = `data:image/png;base64,${image.toString("base64")}`;
  assert(importPayloadChunks(payload).length > 1, "The file picker must dispatch multiple PNG chunks");
  const [chooser] = await Promise.all([page.waitForEvent("filechooser"), action(page, "importPng")]);
  await chooser.setFiles({ name: "raster-workflow.png", mimeType: "image/png", buffer: image });
  await page.locator(composite).first().getByRole("option", { name: "Imported Image", exact: true }).waitFor({ state: "attached", timeout: 60_000 });
}

async function importedAsset(page: Page) {
  const layers = await page.locator(composite).first().getAttribute("data-layers-json");
  const key = (JSON.parse(layers ?? "[]") as Array<{ name: string; imageKey?: string }>).find((layer) => layer.name === "Imported Image")?.imageKey;
  assert(key, "The rendered imported layer has no image asset");
  return key;
}

async function waitForImportedAsset(page: Page, key: string, changed = false, surfaceSelector = composite) {
  const witness = await page.waitForFunction(({ selector, expected, changed }) => {
    const layers = document.querySelector(selector)?.getAttribute("data-layers-json");
    const imageKey = (JSON.parse(layers ?? "[]") as Array<{ name: string; imageKey?: string }>).find((layer) => layer.name === "Imported Image")?.imageKey;
    return imageKey && (changed ? imageKey !== expected : imageKey === expected) ? imageKey : false;
  }, { selector: surfaceSelector, expected: key, changed }, { timeout: 30_000 });
  const imageKey = await witness.jsonValue();
  await witness.dispose();
  assert(imageKey, "The renderer did not publish the expected image asset");
  return imageKey;
}

async function stroke(page: Page, color: string, surface: Locator = page.locator(composite).filter({ visible: true }).last()) {
  const tools = surface.getByRole("toolbar", { name: "Paint tools" });
  if (!(await tools.isVisible())) await surface.getByRole("button", { name: /^Painting/ }).click();
  const layers = surface.getByRole("combobox", { name: "Layer", exact: true });
  const importedLayer = layers.getByRole("option", { name: "Imported Image", exact: true });
  await importedLayer.waitFor({ state: "attached" });
  const importedValue = await importedLayer.getAttribute("value");
  assert(importedValue, "The imported layer has no editable identity");
  await layers.selectOption(importedValue);
  await tools.getByRole("button", { name: "Brush", exact: true }).click();
  await surface.locator('input[type="color"]').fill(color);
  const editor = surface.getByRole("application", { name: "Image editor" });
  const box = await editor.boundingBox();
  assert(box, "Raster canvas is not visible");
  const x = box.x + box.width / 2, y = box.y + box.height / 2;
  assert(await editor.evaluate((element, point) => element.contains(document.elementFromPoint(point.x, point.y)), { x, y }), "Canvas center is covered by another control");
  await page.mouse.move(x - 6, y);
  await page.mouse.down();
  await page.mouse.move(x + 6, y + 6, { steps: 6 });
  await page.mouse.up();
  await surface.getByRole("status").filter({ hasText: /^Edit applied$/ }).waitFor({ timeout: 30_000 });
}

const generated = join(import.meta.dir, "🗑️generated");
await mkdir(generated, { recursive: true });
const downloadsPath = await mkdtemp(join(generated, "raster-browser-"));
let browser: Browser | undefined;
try {
  browser = await chromium.launch({ headless: true, downloadsPath, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
  const options = { viewport: { width: 1600, height: 1050 }, locale: "en-US", acceptDownloads: true };
  const page = await (await browser.newContext(options)).newPage();
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await boot(page);
  await importPng(page);
  const originalAsset = await importedAsset(page);
  const before = await png(page);
  const importedColorPresent = before.data.some((red, index, rgba) => index % 4 === 0 && red === 23
    && rgba[index + 1] === 120 && rgba[index + 2] === 80 && rgba[index + 3] === 255);
  assert(importedColorPresent, "The imported PNG color did not reach the exported composite");
  await stroke(page, "#dc1937");
  const paintedAsset = await waitForImportedAsset(page, originalAsset, true);
  const edited = await png(page);
  assert.deepEqual(edited.info, before.info);
  assert(!edited.data.equals(before.data), "The completed stroke did not change exported pixels");
  await action(page, "undo");
  await waitForImportedAsset(page, originalAsset);
  assert.deepEqual((await png(page)).data, before.data, "Undo did not restore the original pixels");
  await action(page, "redo");
  await waitForImportedAsset(page, paintedAsset);
  assert.deepEqual((await png(page)).data, edited.data, "Redo did not restore the edited pixels");

  const archive = await downloadAction(page, EXPORT_ARTIFACT_DOCUMENT_ACTION_ID);
  assert.match(archive.name, /\.semio-archive$/);
  assert(decodeDocumentArchiveBytes(archive.bytes));
  const fresh = await (await browser.newContext(options)).newPage();
  fresh.on("pageerror", (error) => errors.push(error.message));
  await boot(fresh);
  const originalWindows = await windowIds(fresh);
  const [chooser] = await Promise.all([fresh.waitForEvent("filechooser"), action(fresh, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID)]);
  await chooser.setFiles({ name: archive.name, mimeType: "application/octet-stream", buffer: archive.bytes });
  await fresh.locator('[data-notice-code="shell.documentTransfer.imported"]').waitFor({ timeout: 90_000, state: "attached" });
  const reopenedWindow = await fresh.waitForFunction((original) => [...document.querySelectorAll('[data-slot="mode-dock-tab"][data-window-id]')]
    .map((tab) => tab.getAttribute("data-window-id"))
    .find((id): id is string => id !== null && id.endsWith("::raster-composite") && !original.includes(id)), originalWindows, { timeout: 90_000 });
  const windowId = await reopenedWindow.jsonValue();
  await reopenedWindow.dispose();
  assert(windowId, "The archive import did not open a document window");
  const tab = fresh.locator(`[data-slot="mode-dock-tab"][data-window-id=${JSON.stringify(windowId)}]`).getByRole("tab");
  await tab.click();
  const panel = await tab.getAttribute("aria-controls");
  assert(panel, "The reopened window has no accessible editor panel");
  const reopenedSelector = `[id=${JSON.stringify(panel)}] ${composite}`;
  const reopenedSurface = fresh.locator(reopenedSelector);
  await reopenedSurface.waitFor({ state: "visible" });
  await waitForImportedAsset(fresh, paintedAsset, false, reopenedSelector);
  assert.deepEqual((await png(fresh)).data, edited.data, "Opening the editable archive did not restore the edited image");
  await stroke(fresh, "#247be8", reopenedSurface);
  await waitForImportedAsset(fresh, paintedAsset, true, reopenedSelector);
  assert(!(await png(fresh)).data.equals(edited.data), "Pixels in the reopened archive are not editable");
  for (const openedPage of [page, fresh]) assert.equal(await openedPage.getByText("WebGPU did not initialize", { exact: true }).count(), 0);
  assert.deepEqual(errors, []);
  console.log("[DEBUG] Raster browser verified PNG import, paint, undo, redo, editable archive reopen and PNG pixel equality");
} catch (error) {
  for (const page of browser?.contexts().flatMap((context) => context.pages()) ?? []) {
    const status = await page.locator("[data-semio-os-error], [role=status]:visible, [role=alert]:visible").allTextContents().catch(() => []);
    const notices = await page.locator("[data-notice-code]").evaluateAll((items) => items
      .map((item) => item.getAttribute("data-notice-code"))).catch(() => []);
    console.error("[DEBUG] Raster browser failure", { status: status.map((value) => value.trim().slice(0, 300)), notices });
  }
  throw error;
} finally {
  try {
    await browser?.close();
  } finally {
    await rm(downloadsPath, { recursive: true, force: true });
    await rmdir(generated).catch((error: NodeJS.ErrnoException) => {
      if (!["ENOTEMPTY", "ENOENT", "EEXIST"].includes(error.code ?? "")) throw error;
    });
  }
}
