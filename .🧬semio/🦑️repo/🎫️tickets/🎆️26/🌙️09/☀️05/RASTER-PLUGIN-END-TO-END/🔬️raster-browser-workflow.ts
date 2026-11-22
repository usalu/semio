import assert from "node:assert/strict";
import { mkdir, mkdtemp, rm, rmdir } from "node:fs/promises";
import { join } from "node:path";
import { chromium, type Browser, type Download, type Locator, type Page } from "playwright";
import sharp from "sharp";
import { importPayloadChunks } from "@semio-tech/framework";
import { decodeDocumentArchiveBytes } from "@semio-tech/framework-os";
import { EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID } from "@semio-tech/framework-manifest/definitions-action-ids-and-shell-brand";
import { dismissIntroduction, seatLocale, unfoldActionsRail, windowIds } from "@semio-tech/framework-os-dev/testing/program-matrix";

const address = process.argv[2] ?? "http://127.0.0.1:6060/?plugin=raster";
const composite = '[data-surface-id$=":raster-composite"]';

async function boot(page: Page) {
  const timeout = 180_000;
  const response = await page.goto(address, { waitUntil: "domcontentloaded" });
  assert(response?.ok(), "Raster preview did not return a successful response");
  await page.waitForFunction(() => document.querySelector("[data-semio-os-ready],[data-semio-os-error]"), null, { timeout });
  assert.equal(await page.locator("[data-semio-os-error]").count(), 0, "Raster shell reported a startup error");
  await dismissIntroduction(page);
  await seatLocale(page, "en");
  await page.locator(composite).first().locator("canvas").first().waitFor({ state: "visible", timeout });
  if (process.env.RASTER_WORKFLOW_TRACE_TRANSPORT !== "1") {
    await page.waitForFunction((selector) => {
      const assets = document.querySelector(selector)?.getAttribute("data-assets-json") ?? "{}";
      return Object.hasOwn(JSON.parse(assets), "semio-emblem");
    }, composite, { timeout });
    await page.getByText(/^Image rendering failed:/).first().waitFor({ state: "hidden", timeout });
  }
}

async function action(page: Page, verb: string) {
  const selector = `[data-slot="window-action-pane"] [id="action.${verb}"]:visible`;
  const row = page.locator(selector).first();
  if (!(await row.isVisible())) await unfoldActionsRail(page);
  await row.waitFor({ state: "visible" });
  const staged = (await row.textContent())?.trim().endsWith("…");
  let pressed = false;
  if (verb === "importPng") {
    await row.locator("button[data-slot=tree-label]").first().scrollIntoViewIfNeeded({ timeout: 4000 }).catch(() => undefined);
    const geometry = await row.evaluate((element) => {
      const details = (node: Element | null) => {
        if (!node) return null;
        const box = node.getBoundingClientRect(), style = getComputedStyle(node);
        return { id: node.id, slot: node.getAttribute("data-slot"), box: [box.x, box.y, box.width, box.height],
          clip: style.clipPath, hit: style.pointerEvents, scroll: [node.scrollTop, node.clientHeight, node.scrollHeight] };
      };
      const label = element.querySelector("button[data-slot=tree-label]");
      const bounds = (label ?? element).getBoundingClientRect();
      const y = bounds.y + bounds.height / 2;
      const hits = [0.16, 0.5, 0.85].map((share) => {
        const x = bounds.x + bounds.width * share, top = document.elementFromPoint(x, y);
        return { x, y, contained: Boolean(top && element.contains(top)), id: top?.closest("[id]")?.id,
          slot: top?.getAttribute("data-slot") };
      });
      return { row: details(element), label: details(label), pane: details(element.closest("[data-slot=window-action-pane]")),
        body: details(element.closest("[data-slot=window-engagement-body]")), chrome: details(element.closest(`[id$=".engagement"]`)), hits };
    });
    console.log("[DEBUG] Raster import command geometry", geometry);
    const hit = geometry.hits.find((candidate) => candidate.contained);
    if (process.env.RASTER_WORKFLOW_TRACE_TRANSPORT !== "1") assert(hit, "Visible Import PNG cannot be reached by the pointer");
    if (hit) {
      await page.mouse.click(hit.x, hit.y);
      pressed = true;
    }
  }
  if (!pressed) await pressActionRow(row);
  if (staged) {
    const stage = page.locator(`[data-slot="window-action-pane"] [id$=".action.${verb}.execute"]:visible`).first();
    await stage.waitFor({ state: "visible", timeout: 8_000 });
    await pressActionRow(stage);
  }
}

async function pressActionRow(row: Locator) {
  const label = row.locator("button[data-slot=tree-label]").first();
  if (await label.count()) await label.press("Enter", { timeout: 20_000 });
  else await row.click({ timeout: 20_000 });
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

async function waitForCanvasColor(surface: Locator, target: readonly [number, number, number]) {
  const canvas = surface.locator("canvas").first();
  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    const screenshot = await canvas.screenshot({ animations: "disabled", scale: "css", timeout: 10_000 });
    const { data, info } = await sharp(screenshot).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
    let matches = 0, x = 0, y = 0;
    for (let index = 0; index < data.length; index += 4) {
      if (!target.every((channel, offset) => Math.abs(data[index + offset]! - channel) <= 3)) continue;
      const pixel = index / 4;
      matches += 1;
      x += pixel % info.width + 0.5;
      y += Math.floor(pixel / info.width) + 0.5;
    }
    if (matches >= 16) return { x: x / matches / info.width, y: y / matches / info.height };
  }
  assert.fail(`The visible Raster canvas did not show RGB ${target.join(", ")}`);
}

async function importPng(page: Page) {
  const diagnostic = process.env.RASTER_WORKFLOW_DIAGNOSTIC_COMPRESSED === "1";
  const width = diagnostic && process.env.RASTER_WORKFLOW_DIAGNOSTIC_SMALL === "1" ? 32 : 256;
  const image = await sharp({
    create: { width, height: width, channels: 4, background: { r: 23, g: 120, b: 80, alpha: 1 } },
  }).png({ compressionLevel: diagnostic ? 9 : 0 }).toBuffer();
  const payload = `data:image/png;base64,${image.toString("base64")}`;
  if (diagnostic) {
    assert(payload.length < 65_536, "The compressed diagnostic must fit the previous command-wire limit");
    console.log("[DEBUG] Raster compressed diagnostic input", image.length);
  } else assert(importPayloadChunks(payload).length > 1, "The file picker must dispatch multiple PNG chunks");
  const [chooser] = await Promise.all([page.waitForEvent("filechooser"), action(page, "importPng")]);
  await chooser.setFiles({ name: "raster-workflow.png", mimeType: "image/png", buffer: image });
  await waitForImportedAsset(page, "", true);
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
  }, { selector: surfaceSelector, expected: key, changed }, { timeout: process.env.RASTER_WORKFLOW_TRACE_TRANSPORT === "1" ? 120_000 : 60_000 });
  const imageKey = await witness.jsonValue();
  await witness.dispose();
  assert(imageKey, "The renderer did not publish the expected image asset");
  return imageKey;
}

async function stroke(page: Page, surface: Locator, point: { x: number; y: number }, color: string) {
  const tools = surface.getByRole("toolbar", { name: "Paint tools" });
  if (!(await tools.isVisible())) await surface.getByRole("button", { name: /^Painting/ }).click();
  const layers = surface.getByRole("combobox", { name: "Layer", exact: true });
  const importedLayer = layers.getByRole("option", { name: "Imported Image", exact: true });
  await importedLayer.waitFor({ state: "attached" });
  const importedValue = await importedLayer.getAttribute("value");
  assert(importedValue, "The imported layer has no editable identity");
  await layers.selectOption(importedValue);
  await layers.getByRole("option", { name: "Imported Image", exact: true, selected: true }).waitFor({ state: "attached", timeout: 30_000 });
  await tools.getByRole("button", { name: "Brush", exact: true }).click();
  await surface.locator('input[type="color"]').fill(color);
  await surface.locator(`input[type="color"][value=${JSON.stringify(color)}]`).waitFor({ state: "attached", timeout: 30_000 });
  const editor = surface.getByRole("application", { name: "Image editor" });
  const box = await surface.locator("canvas").first().boundingBox();
  assert(box, "Raster canvas is not visible");
  const x = box.x + box.width * point.x, y = box.y + box.height * point.y;
  assert(await editor.evaluate((element, point) => element.contains(document.elementFromPoint(point.x, point.y)), { x, y }), "The imported image is covered by another control");
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
const browserDiagnostics: string[] = [];
const transportDiagnostics: string[] = [];
const errors: string[] = [];
async function recordBrowserDiagnostics(page: Page) {
  const preview = new URL(address);
  await page.routeWebSocket((url) => url.host === preview.host && url.pathname === "/" && url.searchParams.has("token"), (socket) => {
    const upstream = socket.connectToServer();
    upstream.onMessage((message) => {
      if (typeof message === "string" && /"type"\s*:\s*"(?:update|full-reload|prune)"/.test(message)) return;
      socket.send(message);
    });
  });
  if (process.env.RASTER_WORKFLOW_TRACE_DRIVE === "1") {
    await page.route((url) => decodeURIComponent(url.pathname).endsWith("/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts"), async (route) => {
      const response = await route.fetch();
      let body = await response.text();
      const replace = (old: string, next: string) => {
        assert(body.includes(old), `The running turn driver does not contain ${old}`);
        body = body.replace(old, next);
      };
      replace("await shardClient.startJob(actorId, job, kind, input);", `await shardClient.startJob(actorId, job, kind, input);
        console.warn("[DEBUG] Raster drive reserved start", job.toString(), kind);
        let rasterJobSteps = 0;`);
      replace("const outcome = await shardClient.stepJob(actorId, job, jobStepBudget);", `const outcome = await shardClient.stepJob(actorId, job, jobStepBudget);
        rasterJobSteps++;
        if (rasterJobSteps <= 4 || rasterJobSteps % 128 === 0 || outcome.status !== "running") {
          console.warn("[DEBUG] Raster drive reserved step", job.toString(), rasterJobSteps, outcome.status,
            outcome.status === "failed" ? new TextDecoder().decode(outcome.value).slice(0, 300) : "");
        }`);
      replace("const previousIngress = terminal;", `const previousIngress = terminal;
        if (continuations < 4 || continuations % 128 === 0) {
          console.warn("[DEBUG] Raster drive ingress", inspected.actionId, continuations, terminal,
            typeof continued.status === "string" ? continued.status : continued.status?.tag,
            continued.effects.map(effect => effect.tag).join(","), continued.deferredEvents ?? 0);
        }`);
      await route.fulfill({ response, body });
    });
  }
  if (process.env.RASTER_WORKFLOW_TRACE_TRANSPORT === "1") {
    await page.route((url) => decodeURIComponent(url.pathname).includes("/🔌️PluginRuntime/📥️invocation-frames/"), async (route) => {
      const response = await route.fetch();
      const body = await response.text();
      const close = /closeInvoke\(\{\s*failed:\s*true\s*\}\);/;
      assert.match(body, close, "The live invocation failure trace could not be found");
      const diagnostic = `console.warn("[DEBUG] Raster invocation failed", String(actionId), error instanceof Error ? error.message : String(error));`;
      const done = /return response;/;
      assert.match(body, done, "The live invocation reply trace could not be found");
      const result = `if (actionId === "importPngFile" || actionId === "setCompositeViewport") console.warn(
        "[DEBUG] Raster invocation reply", String(actionId), JSON.stringify({
          frames: frames.map(frame => Object.keys(frame)),
          mutations: response.mutations.length,
          output: response.output && typeof response.output === "object" ? Object.keys(response.output) : String(response.output),
          diagnostics: response.diagnostics.map(value => ({code: value?.code, message: value?.message, stage: value?.stage})),
        }));`;
      await route.fulfill({ response, body: body.replace(close, (failure) => failure + diagnostic).replace(done, (reply) => result + reply) });
    });
  }
  const record = (line: string) => {
    browserDiagnostics.push(line.slice(0, 500));
    if (browserDiagnostics.length > 30) browserDiagnostics.shift();
  };
  page.on("console", (message) => {
    if (message.type() === "error" || message.type() === "warning") {
      const line = message.text();
      record(line);
      if (/importPngFile|file-open|typed command|command ingress|refused|invocation (failed|reply)|Raster drive/i.test(line)) {
        if (process.env.RASTER_WORKFLOW_TRACE_DRIVE === "1" && line.includes("Raster drive")) console.log(line);
        transportDiagnostics.push(line.slice(0, 700));
        if (transportDiagnostics.length > 18) transportDiagnostics.shift();
      }
    }
  });
  page.on("requestfailed", (request) => {
    record(`${new URL(request.url()).pathname}: ${request.failure()?.errorText ?? "failed"}`);
  });
  if (process.env.RASTER_WORKFLOW_TRACE_TRANSPORT === "1") await page.addInitScript(() => {
    document.addEventListener("click", (event) => {
      if (!(event.target instanceof Element)) return;
      const target = event.target.closest("button, input[type=file]");
      if (target) console.warn("[DEBUG] Raster click", target.closest("[id]")?.id, target.getAttribute("data-slot"), event.isTrusted);
    }, true);
    const click = HTMLInputElement.prototype.click;
    HTMLInputElement.prototype.click = function () {
      if (this.type === "file") console.warn("[DEBUG] Raster file input click", this.accept, navigator.userActivation.isActive);
      return click.call(this);
    };
  });
}
try {
  browser = await chromium.launch({ headless: true, downloadsPath, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
  const options = { viewport: { width: 1600, height: 1050 }, locale: "en-US", acceptDownloads: true };
  const page = await (await browser.newContext(options)).newPage();
  page.on("pageerror", (error) => errors.push(error.message));
  await recordBrowserDiagnostics(page);
  await boot(page);
  const canvasWitness = await page.locator(composite).first().evaluate((element) => {
    const assets = JSON.parse(element.getAttribute("data-assets-json") ?? "{}");
    const layers = JSON.parse(element.getAttribute("data-layers-json") ?? "[]");
    return { assetKeys: Object.keys(assets), layers: Array.isArray(layers)
      ? layers.map(({ name, imageKey }: { name: string; imageKey?: string }) => ({ name, imageKey }))
      : Object.keys(layers) };
  });
  console.log("[DEBUG] Raster initial compositor", canvasWitness);
  await importPng(page);
  const originalAsset = await importedAsset(page);
  const surface = page.locator(composite).first();
  const importedPoint = await waitForCanvasColor(surface, [23, 120, 80]);
  const before = await png(page);
  const importedColorPresent = before.data.some((red, index, rgba) => index % 4 === 0 && red === 23
    && rgba[index + 1] === 120 && rgba[index + 2] === 80 && rgba[index + 3] === 255);
  assert(importedColorPresent, "The imported PNG color did not reach the exported composite");
  await stroke(page, surface, importedPoint, "#dc1937");
  const paintedAsset = await waitForImportedAsset(page, originalAsset, true);
  await waitForCanvasColor(page.locator(composite).first(), [220, 25, 55]);
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
  await recordBrowserDiagnostics(fresh);
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
  const reopenedPoint = await waitForCanvasColor(reopenedSurface, [23, 120, 80]);
  await stroke(fresh, reopenedSurface, reopenedPoint, "#247be8");
  await waitForImportedAsset(fresh, paintedAsset, true, reopenedSelector);
  await waitForCanvasColor(reopenedSurface, [36, 123, 232]);
  assert(!(await png(fresh)).data.equals(edited.data), "Pixels in the reopened archive are not editable");
  for (const openedPage of [page, fresh]) assert.equal(await openedPage.getByText("WebGPU did not initialize", { exact: true }).count(), 0);
  assert.deepEqual(errors, []);
  console.log("[DEBUG] Raster browser verified PNG import, paint, undo, redo, editable archive reopen and PNG pixel equality");
} catch (error) {
  for (const page of browser?.contexts().flatMap((context) => context.pages()) ?? []) {
    const status = await page.locator("[data-semio-os-error], [role=status]:visible, [role=alert]:visible").allTextContents().catch(() => []);
    const notices = await page.locator("[data-notice-code]").evaluateAll((items) => items
      .map((item) => item.getAttribute("data-notice-code"))).catch(() => []);
    const inputs = await page.locator("input[type=file]").evaluateAll((items) => items.map((item) => ({
      id: item.id, accept: item.getAttribute("accept"),
    }))).catch(() => []);
    const invocations = await page.evaluate(() => (performance.getEntriesByType("measure") as PerformanceMeasure[])
      .filter((entry) => entry.name.startsWith("semio.hop.") && String(entry.detail?.actionId).startsWith("importPng"))
      .slice(-18).map(({ name, duration, detail }) => ({ name, duration, action: detail?.actionId, effects: detail?.effects, failed: detail?.failed }))).catch(() => []);
    console.error("[DEBUG] Raster browser failure", { status: status.map((value) => value.trim().slice(0, 300)), notices, inputs, invocations, transportDiagnostics, errors, browserDiagnostics });
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
