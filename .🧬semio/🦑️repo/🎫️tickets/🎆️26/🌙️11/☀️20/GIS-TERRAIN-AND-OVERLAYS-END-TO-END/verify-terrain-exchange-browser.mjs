import assert from "node:assert/strict";
import { mkdir, readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const url = new URL(process.argv[2] ?? "http://127.0.0.1:6083/?example=demo");
assert(["127.0.0.1", "localhost", "[::1]"].includes(url.hostname), "Use the local s Dev server");

const fixture = new URL("../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/", import.meta.url);
const oracle = JSON.parse(await readFile(new URL("🔣️.json", fixture), "utf8"));
const source = fileURLToPath(new URL("🔼️slope.asc", fixture));
const geoJsonSource = fileURLToPath(new URL("../../../🌍️geojson/🧪️tests/🧫️fixtures/🔣️.geojson", fixture));
const geoJsonOracle = JSON.parse(await readFile(geoJsonSource, "utf8"));
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
await mkdir(generated, { recursive: true });

const shellSelector = "[data-semio-os-ready][data-history-json]";
const importAction = "importElevationGridFile";
const modifier = process.platform === "darwin" ? "Meta" : "Control";

function readRaster(text) {
  const [columns, rows, xOrigin, yOrigin, cellSize, noData, ...values] = text.trim().split(/\r?\n/);
  const [west, south, east, north] = oracle.bounds;
  const pitch = (east - west) / (oracle.side - 1);
  assert.equal(pitch, (north - south) / (oracle.side - 1));
  assert.deepEqual([columns, rows, xOrigin, yOrigin, cellSize, noData], [
    `NCOLS ${oracle.side}`,
    `NROWS ${oracle.side}`,
    `XLLCENTER ${west}`,
    `YLLCENTER ${south}`,
    `CELLSIZE ${pitch}`,
    "NODATA_VALUE -32769",
  ]);
  assert.equal(values.length, oracle.side);
  const samples = values.map((row) => row.trim().split(/\s+/).map(Number));
  assert(samples.every((row) => row.length === oracle.side && row.every(Number.isSafeInteger)));
  return samples.reverse();
}

function verifyRaster(samples) {
  for (const [row, column, metres] of oracle.checkpoints) {
    assert.equal(samples[row][column], metres, `Elevation at row ${row}, column ${column}`);
  }
  const elevations = samples.flat();
  assert.equal(elevations.reduce((sum, height) => sum + height, 0), oracle.checksum);
  assert.equal(elevations.reduce((sum, height) => sum + height ** 2, 0), oracle.squaredChecksum);
}

function verifyRaise(before, after, radius, metres) {
  assert(Number.isSafeInteger(radius) && radius >= 0 && radius < oracle.side);
  assert(Number.isSafeInteger(metres) && metres > 0);
  const changes = after.flatMap((row, y) => row.map((value, x) => value - before[y][x]));
  const squaredRadius = (radius + 1) ** 2;
  const changedSamples = changes.filter((change) => change !== 0).length;
  assert(changedSamples > 0, "The downloaded grid should contain the raised terrain");
  const matches = changes.some((change, index) => {
    if (change !== metres) return false;
    const centerX = index % oracle.side;
    const centerY = Math.floor(index / oracle.side);
    return changes.every((actual, sample) => {
      const dx = sample % oracle.side - centerX;
      const dy = Math.floor(sample / oracle.side) - centerY;
      const area = Math.max(0, squaredRadius - dx ** 2 - dy ** 2);
      return actual === Math.round(metres * area / squaredRadius);
    });
  });
  assert(matches, "The exported elevations should match a single circular raise at the declared radius and step");
  return changedSamples;
}

const browser = await chromium.launch({
  channel: "chrome",
  headless: true,
  downloadsPath: generated,
  args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
});
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1100, height: 760 }, acceptDownloads: true });
  page.setDefaultTimeout(20_000);
  const pageErrors = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));
  const response = await page.goto(url.toString(), { waitUntil: "domcontentloaded", timeout: 50_000 });
  assert(response?.ok(), `s Dev returned HTTP ${response?.status() ?? "unknown"}`);

  const shell = page.locator(shellSelector);
  const terrain = shell.locator('[data-slot="window-body"]').filter({ has: page.locator(".semio-world-3d-host") });
  const overlay = terrain.locator('[data-slot="window-engagement-overlay"]');
  const pane = overlay.locator('[data-slot="window-action-pane"]');
  const toggle = overlay.locator('button[id$=".engagement.toggle"]');
  const history = async () => JSON.parse(await shell.getAttribute("data-history-json"));
  const waitForAction = (cursor, action) => page.waitForFunction(({ selector, cursor, action }) => {
    const raw = document.querySelector(selector)?.getAttribute("data-history-json");
    if (!raw) return false;
    const state = JSON.parse(raw);
    return state.cursor > cursor && state.canUndo && state.actionIds.at(-1) === action;
  }, { selector: shellSelector, cursor, action });
  const setActionsOpen = async (open) => {
    await toggle.waitFor({ state: "visible" });
    const folded = await overlay.getAttribute("data-folded") === "true";
    if (folded === open) await toggle.click();
  };
  const actionRow = (id) => pane.locator(`[data-slot="tree-item-row"][id="action.${id}"]`);
  const pick = async (row, path, accept) => {
    const [chooser] = await Promise.all([page.waitForEvent("filechooser"), row.click()]);
    assert.equal(chooser.isMultiple(), false);
    assert.equal(await chooser.element().getAttribute("accept"), accept);
    await chooser.setFiles(path);
  };
  const download = async (row, filename) => {
    const [file] = await Promise.all([page.waitForEvent("download"), row.click()]);
    assert.equal(file.suggestedFilename(), filename);
    assert.equal(await file.failure(), null);
    try {
      return await readFile(await file.path(), "utf8");
    } finally {
      await file.delete();
    }
  };

  await terrain.locator('[data-slot="terrain-feature-list"] li').filter({ hasText: "Institut de Botanique" }).waitFor({ state: "attached", timeout: 50_000 });
  await setActionsOpen(true);

  const importRow = actionRow("importElevationGrid");
  await importRow.waitFor({ state: "visible" });
  const original = await history();
  assert(Number.isSafeInteger(original.cursor));
  assert(Number.isSafeInteger(original.entries));
  await pick(importRow, source, ".asc");
  await waitForAction(original.cursor, importAction);
  const imported = await history();
  assert.equal(imported.entries, original.entries + 1, "File import should publish one history entry");

  const downloadRaster = () => download(actionRow("exportElevationGrid"), "terrain-elevation.asc");
  const exported = await downloadRaster();
  const baseline = readRaster(exported);
  verifyRaster(baseline);
  assert.equal((await history()).cursor, imported.cursor, "Export should leave history at the imported state");

  const waitForCursor = (cursor) => page.waitForFunction(({ selector, cursor }) => {
    const raw = document.querySelector(selector)?.getAttribute("data-history-json");
    return raw && JSON.parse(raw).cursor === cursor;
  }, { selector: shellSelector, cursor });
  await page.keyboard.press(`${modifier}+z`);
  await waitForCursor(original.cursor);
  assert.equal((await history()).canRedo, true);
  await page.keyboard.press(`${modifier}+Shift+z`);
  await waitForCursor(imported.cursor);
  assert.equal(await downloadRaster(), exported, "Redo should restore the same grid bytes");

  await setActionsOpen(false);
  const viewport = terrain.locator(".semio-world-3d-host");
  const brush = page.getByRole("button", { name: "Raise Terrain", exact: true });
  if (!(await brush.isVisible())) await terrain.locator('[id$=".utilityBar.unfold"]').click();
  if (await brush.getAttribute("aria-pressed") !== "true") await brush.click();
  await page.waitForFunction(() => {
    const interaction = document.querySelector(".semio-world-3d-host")?.getAttribute("data-interaction-json");
    return interaction && JSON.parse(interaction).terrainBrush?.arguments?.operation === "raise-lower";
  });
  const { terrainBrush } = JSON.parse(await viewport.getAttribute("data-interaction-json"));
  assert.equal(terrainBrush.action, "editElevation");
  await viewport.locator("canvas").first().click();
  await waitForAction(imported.cursor, terrainBrush.action);
  const raised = await history();
  await setActionsOpen(true);
  const sculpted = await downloadRaster();
  const changedSamples = verifyRaise(baseline, readRaster(sculpted), terrainBrush.arguments.radius, terrainBrush.arguments.value);
  await page.keyboard.press(`${modifier}+z`);
  await waitForCursor(imported.cursor);
  assert.equal(await downloadRaster(), exported, "Undo should restore the original imported heights");
  await page.keyboard.press(`${modifier}+Shift+z`);
  await waitForCursor(raised.cursor);
  assert.equal(await downloadRaster(), sculpted, "Redo should restore the raised heights");
  await page.keyboard.press(`${modifier}+z`);
  await waitForCursor(imported.cursor);
  let strokeResult;
  if (process.argv.includes("--strokes")) {
    const canvas = viewport.locator("canvas").first();
    const drag = async (cancel) => {
      await setActionsOpen(false);
      const view = await canvas.boundingBox();
      assert(view, "The Terrain viewport must be visible for direct sculpting");
      const centerX = view.x + view.width / 2;
      const centerY = view.y + view.height / 2;
      const span = Math.min(65, view.width * 0.12);
      await page.mouse.move(centerX - span, centerY);
      await page.mouse.down();
      await page.mouse.move(centerX + span, centerY, { steps: 12 });
      if (cancel) await page.keyboard.press("Escape");
      await page.mouse.up();
      await setActionsOpen(true);
    };

    await drag(true);
    assert.equal(await downloadRaster(), exported, "Escape discards the full stroke");
    assert.equal((await history()).cursor, imported.cursor, "Cancelled strokes leave History untouched");

    await drag(false);
    await waitForAction(imported.cursor, terrainBrush.action);
    const committed = await history();
    assert.equal(committed.entries, imported.entries + 1, "One drag publishes one history entry");
    const strokeBytes = await downloadRaster();
    const differences = readRaster(strokeBytes).flatMap((row, y) => row.map((height, x) => height - baseline[y][x]));
    const raisedCells = differences.filter((height) => height > 0).length;
    assert(differences.every((height) => height >= 0), "Raising cannot lower any sample");
    assert(raisedCells > changedSamples, "Dragging edits more samples than a single click");
    await page.keyboard.press(`${modifier}+z`);
    await waitForCursor(imported.cursor);
    assert.equal(await downloadRaster(), exported, "One Undo restores the grid before the full drag");
    await page.keyboard.press(`${modifier}+Shift+z`);
    await waitForCursor(committed.cursor);
    assert.equal(await downloadRaster(), strokeBytes, "One Redo restores the exact stroke");
    await page.keyboard.press(`${modifier}+z`);
    await waitForCursor(imported.cursor);
    strokeResult = { raisedCells, cancelled: true, undo: true, redo: true };
  }
  await page.keyboard.press(`${modifier}+z`);
  await waitForCursor(original.cursor);

  await pick(actionRow("importGeoJson"), geoJsonSource, ".geojson,.json,application/geo+json");
  await waitForAction(original.cursor, "importGeoJsonFile");
  const overlays = await history();
  const featureList = viewport.locator('[data-slot="terrain-feature-list"] li');
  const yard = featureList.filter({ hasText: "Survey yard" });
  const demoPin = featureList.filter({ hasText: "Institut de Botanique" });
  await yard.waitFor({ state: "attached" });
  await demoPin.waitFor({ state: "detached" });
  const downloadGeoJson = () => download(actionRow("exportGeoJson"), "terrain-overlays.geojson");
  const exportedOverlays = await downloadGeoJson();
  assert.deepEqual(JSON.parse(exportedOverlays), geoJsonOracle);
  assert.match(exportedOverlays, /"id"\s*:\s*9007199254740993(?=\s*[,}])/u);
  assert.equal((await history()).cursor, overlays.cursor);
  await page.keyboard.press(`${modifier}+z`);
  await waitForCursor(original.cursor);
  await demoPin.waitFor({ state: "attached" });
  await page.keyboard.press(`${modifier}+Shift+z`);
  await waitForCursor(overlays.cursor);
  await yard.waitFor({ state: "attached" });
  assert.equal(await downloadGeoJson(), exportedOverlays, "Redo should restore the imported GeoJSON bytes");
  await page.keyboard.press(`${modifier}+z`);
  await waitForCursor(original.cursor);

  assert.deepEqual(pageErrors, []);
  console.log(JSON.stringify({
    url: url.toString(),
    file: "terrain-elevation.asc",
    grid: { side: oracle.side, bounds: oracle.bounds, samples: oracle.side ** 2, checksum: oracle.checksum },
    brush: { changedSamples, radius: terrainBrush.arguments.radius, elevationStep: terrainBrush.arguments.value },
    stroke: strokeResult,
    overlays: { file: "terrain-overlays.geojson", features: geoJsonOracle.features.length, cursor: overlays.cursor },
    history: { originalCursor: original.cursor, importedCursor: imported.cursor, raisedCursor: raised.cursor, undo: true, redo: true },
  }));
} finally {
  await browser.close();
}
