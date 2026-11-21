import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { PNG } from "pngjs";

const url = new URL(process.argv[2] ?? "http://127.0.0.1:6083/?example=demo");
assert(["127.0.0.1", "localhost", "[::1]"].includes(url.hostname), "Use the local s dev server");
const interactionOnly = process.argv.includes("--interaction-only");
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
await mkdir(generated, { recursive: true });

const expectedFeatures = ["Institut de Botanique", "Lycée Block 3000", "Sart Tilman", "Sart Tilman"];

async function readFeatures(viewport) {
  const rows = viewport.locator('[data-slot="terrain-feature-list"] li');
  await rows.filter({ hasText: "Institut de Botanique" }).waitFor({ state: "attached", timeout: 50_000 });
  const names = await rows.allTextContents();
  assert.deepEqual(names.toSorted(), expectedFeatures.toSorted());
  return names;
}

async function readTerrainData(page) {
  const dimensions = page.locator('[id="grid-size"]');
  if (!(await dimensions.isVisible())) {
    await page.getByRole("button", { name: "Terrain Data", exact: true }).click();
  }
  const rows = {
    "grid-size": "33 × 33",
    "grid-range": "47–252 m",
    "grid-mean": "117.0 m",
    "grid-longitude": "5.555000° → 5.608333°",
    "grid-latitude": "50.585000° → 50.638333°",
    "positions-0": "p_institut_de_botanique_ulg_liege",
    "positions-1": "p_lycee_block_3000",
    "routes-0": "r_sart_tilman",
    "regions-0": "g_sart_tilman",
  };
  for (const [id, expected] of Object.entries(rows)) {
    const row = page.locator(`[id="${id}"]`);
    await row.waitFor({ state: "visible", timeout: 15_000 });
    assert((await row.textContent()).includes(expected), `Terrain Data row ${id} includes ${expected}`);
  }
  return rows;
}

async function waitForTerrainPaint(canvas, screenshot, pageErrors) {
  await canvas.waitFor({ state: "visible", timeout: 20_000 });
  const limit = Date.now() + 25_000;
  do {
    const pixels = PNG.sync.read(await canvas.screenshot({ path: screenshot, timeout: 8_000 })).data;
    const samples = new Set();
    for (let offset = 0; offset < pixels.length; offset += 16) {
      samples.add(((pixels[offset] >> 4) << 8) | ((pixels[offset + 1] >> 4) << 4) | (pixels[offset + 2] >> 4));
    }
    if (samples.size >= 32) return samples.size;
    await canvas.page().waitForTimeout(200);
  } while (!pageErrors.length && Date.now() < limit);
  assert.fail("The terrain viewport never drew a varied frame");
}

const browser = await chromium.launch({ channel: "chrome", headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] });
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1100, height: 760 } });
  const pageErrors = [];
  page.on("pageerror", error => pageErrors.push(error.message));
  page.on("console", message => { if (message.text().startsWith("[DEBUG]")) console.log(message.text()); });
  const response = await page.goto(url.toString(), { waitUntil: "domcontentloaded", timeout: 50_000 });
  assert(response?.ok(), `s dev returned HTTP ${response?.status() ?? "unknown"}`);

  const shellSelector = "[data-semio-os-ready][data-history-json]";
  const history = page.locator(shellSelector);
  await history.waitFor({ state: "attached", timeout: 50_000 });
  const editorRole = page.locator('[data-slot="surface-role-chip"][data-role="editor"]');
  await editorRole.waitFor({ state: "attached" });
  assert.equal(await editorRole.getAttribute("data-app-id"), "s.gis.gisterrain@1/*#editor");
  const viewport = page.locator(".semio-world-3d-host");
  const names = await readFeatures(viewport);
  const canvas = viewport.locator("canvas").first();
  const screenshot = join(generated, `terrain-s-dev-${url.port}.png`);
  const colours = interactionOnly ? null : await waitForTerrainPaint(canvas, screenshot, pageErrors);
  if (colours !== null) console.log(`[DEBUG] The registered Terrain editor painted ${colours} colour bins and exposes ${names.length} feature labels.`);

  const brush = page.getByRole("button", { name: "Raise Terrain", exact: true });
  if (!(await brush.isVisible())) await page.locator('[id$=".utilityBar.unfold"]').click();
  try {
    await brush.waitFor({ state: "visible", timeout: 20_000 });
  } catch (error) {
    const controls = await page.getByRole("button").evaluateAll((buttons) =>
      buttons.map((button) => ({ id: button.id, label: button.getAttribute("aria-label"), title: button.getAttribute("title"), text: button.textContent?.trim() }))
        .filter(({ id, label, title, text }) => id || label || title || text));
    console.error(`[DEBUG] Visible editor controls ${JSON.stringify(controls)}`);
    throw error;
  }
  if (await brush.getAttribute("aria-pressed") !== "true") await brush.click();
  await page.getByRole("button", { name: "Raise Terrain", exact: true, pressed: true }).waitFor({ state: "visible", timeout: 20_000 });
  console.log("[DEBUG] The Utilities toolbar selected Raise Terrain.");
  await page.waitForFunction(() => {
    const interaction = document.querySelector(".semio-world-3d-host")?.getAttribute("data-interaction-json");
    return interaction && JSON.parse(interaction).terrainBrush?.arguments?.operation === "raise-lower";
  }).catch(async (error) => {
    console.error(`[DEBUG] World3d interaction after selecting Raise Terrain ${await viewport.getAttribute("data-interaction-json")}`);
    throw error;
  });
  const { terrainBrush } = JSON.parse(await viewport.getAttribute("data-interaction-json"));
  assert.equal(terrainBrush.action, "editElevation");
  assert(terrainBrush.arguments.value > 0, "The Raise Terrain brush has a positive height step");
  console.log("[DEBUG] World3d received the registered editElevation brush binding.");

  const { cursor: originalCursor, actionIds } = JSON.parse(await history.getAttribute("data-history-json"));
  assert(Number.isSafeInteger(originalCursor), "The editor exposes a history cursor");
  const previousEdits = actionIds.filter((id) => id === "editElevation").length;
  const size = await canvas.boundingBox();
  assert(size, "The terrain canvas has a visible area");
  await canvas.click({ position: { x: size.width / 2, y: size.height / 2 } });
  console.log(`[DEBUG] Clicked the authored terrain; starting history cursor ${originalCursor}.`);
  await page.waitForFunction(({ selector, cursor, edits }) => {
    const raw = document.querySelector(selector)?.getAttribute("data-history-json");
    if (!raw) return false;
    const current = JSON.parse(raw);
    return current.cursor > cursor && current.canUndo && current.actionIds.filter((id) => id === "editElevation").length > edits;
  }, { selector: shellSelector, cursor: originalCursor, edits: previousEdits });
  const { cursor: editedCursor } = JSON.parse(await history.getAttribute("data-history-json"));
  console.log(`[DEBUG] Terrain Store published the brush edit at history cursor ${editedCursor}.`);
  const modifier = process.platform === "darwin" ? "Meta" : "Control";
  const waitForCursor = (cursor) => page.waitForFunction(({ selector, expected }) => {
    const raw = document.querySelector(selector)?.getAttribute("data-history-json");
    return raw && JSON.parse(raw).cursor === expected;
  }, { selector: shellSelector, expected: cursor });
  await page.keyboard.press(`${modifier}+z`);
  await waitForCursor(originalCursor);
  assert(JSON.parse(await history.getAttribute("data-history-json")).canRedo);
  await page.keyboard.press(`${modifier}+Shift+z`);
  await waitForCursor(editedCursor);
  await page.keyboard.press(`${modifier}+z`);
  await waitForCursor(originalCursor);

  const cameraBefore = await viewport.getAttribute("data-camera-json");
  assert(cameraBefore && JSON.parse(cameraBefore).position?.length === 3, "Terrain publishes its initial camera");
  await canvas.hover({ position: { x: size.width / 2, y: size.height / 2 } });
  await page.mouse.wheel(0, -240);
  await page.waitForFunction((before) => {
    const camera = document.querySelector(".semio-world-3d-host")?.getAttribute("data-camera-json");
    return camera && camera !== before;
  }, cameraBefore, { timeout: 20_000 });
  const cameraAfter = await viewport.getAttribute("data-camera-json");
  assert.equal(JSON.parse(await history.getAttribute("data-history-json")).cursor, originalCursor, "Camera navigation uses WindowConfig");

  const terrainData = await readTerrainData(page);

  const viewerUrl = new URL(url);
  viewerUrl.searchParams.set("role", "viewer");
  const viewerResponse = await page.goto(viewerUrl.toString(), { waitUntil: "domcontentloaded", timeout: 50_000 });
  assert(viewerResponse?.ok(), `s dev viewer returned HTTP ${viewerResponse?.status() ?? "unknown"}`);
  await page.locator(shellSelector).waitFor({ state: "attached", timeout: 50_000 });
  const viewerRole = page.locator('[data-slot="surface-role-chip"][data-role="viewer"]');
  await viewerRole.waitFor({ state: "attached" });
  assert.equal(await viewerRole.getAttribute("data-app-id"), "s.gis.gisterrain@1/*#viewer");
  const viewerNames = await readFeatures(viewport);
  assert.equal(await brush.count(), 0);
  assert(JSON.parse(await viewport.getAttribute("data-interaction-json")).terrainBrush == null, "The viewer should be read-only");
  const viewerScreenshot = join(generated, `terrain-viewer-s-dev-${url.port}.png`);
  const viewerColours = interactionOnly ? null : await waitForTerrainPaint(canvas, viewerScreenshot, pageErrors);

  assert.deepEqual(pageErrors, []);
  console.log(JSON.stringify({ url: url.toString(), features: names, terrainData, camera: { before: JSON.parse(cameraBefore), after: JSON.parse(cameraAfter) }, activeUtility: "Raise Terrain", edit: { originalCursor, editedCursor, undo: true, redo: true }, colours, screenshot: interactionOnly ? undefined : screenshot, viewer: { url: viewerUrl.toString(), features: viewerNames, colours: viewerColours, screenshot: interactionOnly ? undefined : viewerScreenshot } }));
} finally {
  await browser.close();
}
