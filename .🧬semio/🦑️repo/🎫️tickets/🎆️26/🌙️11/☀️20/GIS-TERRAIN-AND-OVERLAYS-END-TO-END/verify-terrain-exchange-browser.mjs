import { PNG } from "pngjs";
import assert from "node:assert/strict";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { profileTerrainArchiveWorkers } from "./profile-terrain-archive-worker.mjs";

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
  assert(samples.every((row) => row.length === oracle.side && row.every((height) => {
    const millimetres = Math.round(height * 1000);
    return Number.isSafeInteger(millimetres) && millimetres / 1000 === height;
  })));
  return samples.reverse();
}

function verifyRaster(samples) {
  for (const [row, column, metres] of oracle.checkpoints) {
    assert.equal(samples[row][column], metres, `Elevation at row ${row}, column ${column}`);
  }
  for (let row = 0; row < oracle.side; row++) {
    for (let column = 0; column < oracle.side; column++) {
      assert.equal(samples[row][column], 32 + 3 * column - 2 * row, `Slope at row ${row}, column ${column}`);
    }
  }
  const elevations = samples.flat();
  assert.equal(elevations.reduce((sum, height) => sum + height, 0), oracle.checksum);
  assert.equal(elevations.reduce((sum, height) => sum + height ** 2, 0), oracle.squaredChecksum);
}

function verifyRaise(before, after, radius, metres, samplesPerMetre) {
  assert(Number.isSafeInteger(radius) && radius >= 0 && radius < oracle.side);
  const units = (height) => Math.round(height * samplesPerMetre);
  const step = units(metres);
  assert(step > 0 && step / samplesPerMetre === metres);
  const changes = after.flatMap((row, y) => row.map((value, x) => units(value) - units(before[y][x])));
  const squaredRadius = (radius + 1) ** 2;
  const changedSamples = changes.filter((change) => change !== 0).length;
  assert(changedSamples > 0, "The downloaded grid should contain the raised terrain");
  const matches = changes.some((change, index) => {
    if (change !== step) return false;
    const centerX = index % oracle.side;
    const centerY = Math.floor(index / oracle.side);
    return changes.every((actual, sample) => {
      const dx = sample % oracle.side - centerX;
      const dy = Math.floor(sample / oracle.side) - centerY;
      const area = Math.max(0, squaredRadius - dx ** 2 - dy ** 2);
      return actual === Math.round(step * area / squaredRadius);
    });
  });
  assert(matches, "The exported elevations should match a single circular raise at the declared radius and step");
  return changedSamples;
}

const profilePort = process.argv.includes("--archive-cpu") ? 6336 : null;
let archiveProfile;
const browser = await chromium.launch({
  channel: "chrome",
  headless: true,
  downloadsPath: generated,
  args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", ...(profilePort === null ? [] : [`--remote-debugging-port=${profilePort}`])],
});
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1100, height: 760 }, acceptDownloads: true });
  page.setDefaultTimeout(20_000);
  if (process.argv.includes("--trace-cleanup")) {
    await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
  }
  const pageErrors = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));
  if (process.argv.includes("--diagnose")) page.on("console", (message) => {
    if (message.type() === "error" || message.type() === "warning" || message.text().startsWith("[DEBUG]") || message.text().includes("cleanup continuation")) console.log("[DEBUG] Browser", message.text());
  });
  if (process.argv.includes("--archive") && process.argv.includes("--diagnose")) {
    await page.route((routeUrl) => decodeURIComponent(routeUrl.pathname).endsWith("/🧳️archive-transfer/🟦️.ts"), async (route) => {
      const response = await route.fetch();
      const source = await response.text();
      const body = source.replace(/progress\?\.\(answer\.status\);/, `progress?.(answer.status);
        const signature = JSON.stringify(answer.status);
        if (signature !== globalThis.terrainArchiveTrace || seq % 100 === 0) {
          globalThis.terrainArchiveTrace = signature;
          console.debug("[DEBUG] Archive progress", seq, signature);
        }`);
      assert.notEqual(body, source);
      await route.fulfill({ response, body });
    });
  }
  if (profilePort !== null) {
    let previousArchiveStatus;
    let repeats = 0;
    page.on("console", (message) => {
      const match = message.text().match(/^\[DEBUG\] Archive progress \d+ (.+)$/);
      if (!match) return;
      const status = JSON.parse(match[1]);
      if (status.state !== "running" || !status.completed) return;
      repeats = match[1] === previousArchiveStatus ? repeats + 1 : 0;
      if (!archiveProfile && repeats >= 3) {
        archiveProfile = profileTerrainArchiveWorkers(
          profilePort,
          new URL("./🗑️generated/terrain-archive-workers-late-profile.json", import.meta.url),
        ).catch((error) => console.log("[DEBUG] Terrain worker profile failed", error));
      }
      previousArchiveStatus = match[1];
    });
  }
  if (process.argv.includes("--archive-trace-guest")) {
    const context = page.context();
    await context.route((requestUrl) => requestUrl.pathname === "/__terrain_guest_trace", async (route) => {
      console.log("[DEBUG] Terrain guest", route.request().postData());
      await route.fulfill({ status: 204, body: "" });
    });
    await context.route((requestUrl) => decodeURIComponent(requestUrl.pathname).endsWith("/🌍️gis/🌉️bridge.js"), async (route) => {
      const response = await route.fetch();
      const source = await response.text();
      const anchor = "result = await reactor.poll(slice);";
      const body = source.replace(anchor, `${anchor}
        const count = globalThis.__terrainArchiveGuestTurns = (globalThis.__terrainArchiveGuestTurns ?? 0) + 1;
        if (count <= 4 || count % 16 === 0) {
          const receipt = result.retainedReceipt;
          const trace = JSON.stringify({
            count, actorId, activationGeneration, commandBytes: commandPage?.bytes?.length,
            eventKinds: events.map(({ kind }) => kind), nextWake: optionValue(result.nextWake),
            status: result.status, units: receipt?.workUnitsSpent, spent: receipt?.spent,
            grant: turn.input.grant, fuel: slice.fuel, keys: Object.keys(result)
          }, (_, value) => typeof value === "bigint" ? value.toString() : value);
          fetch(new URL("/__terrain_guest_trace", import.meta.url), { method: "POST", body: trace }).catch(() => {});
        }`);
      assert.notEqual(body, source);
      console.log("[DEBUG] Terrain guest bridge intercepted");
      await route.fulfill({ response, body });
    });
  }
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
  try {
    await waitForAction(original.cursor, importAction);
  } catch (error) {
    const announcements = await page.locator('[role="alert"], [role="status"]').allTextContents();
    console.log("[DEBUG] Import history", JSON.stringify({ original, current: await history(), announcements, pageErrors }));
    console.log("[DEBUG] Visible shell tail", (await page.locator("body").innerText()).slice(-3200));
    throw error;
  }
  const imported = await history();
  assert.equal(imported.entries, original.entries + 2, "The picker request and typed file callback should each publish one entry");
  assert.deepEqual(imported.actionIds.slice(-2), ["importElevationGrid", importAction]);

  const downloadRaster = () => download(actionRow("exportElevationGrid"), "terrain-elevation.asc");
  const exported = await downloadRaster();
  const baseline = readRaster(exported);
  verifyRaster(baseline);
  assert.equal((await history()).canUndo, true, "Export should leave the import available to Undo");

  async function replay(action) {
    const before = await history();
    await page.keyboard.press(action === "undo" ? `${modifier}+z` : `${modifier}+Shift+z`);
    await page.waitForFunction(({ selector, before, action }) => {
      const raw = document.querySelector(selector)?.getAttribute("data-history-json");
      const history = raw && JSON.parse(raw);
      return history && history.cursor > before.cursor && history.entries === before.entries + 1 && history.actionIds.at(-1) === action;
    }, { selector: shellSelector, before, action }).catch(async (error) => {
      console.log("[DEBUG] Terrain replay stalled", JSON.stringify({ action, before, current: await history() }));
      throw error;
    });
    return history();
  }
  assert((await replay("undo")).canRedo);
  assert(!(await replay("redo")).canRedo);
  assert.equal(await downloadRaster(), exported, "Redo should restore the same grid bytes");
  console.log("[DEBUG] Browser raster picker wrote the expected samples and reproduced the export bytes after Undo and Redo.");

  let paletteEdit;
  if (process.argv.includes("--palette-edit")) {
    const beforeEdit = await history();
    await actionRow("editElevation").click();
    await pane.getByText("Column from West", { exact: true }).waitFor({ state: "attached" });
    await pane.getByRole("button", { name: "Execute", exact: true }).click();
    await waitForAction(beforeEdit.cursor, "editElevation");
    const edited = await history();
    assert.equal(edited.entries, beforeEdit.entries + 1);
    const changedBytes = await downloadRaster();
    const changed = readRaster(changedBytes);
    const editedSamples = verifyRaise(baseline, changed, 4, 25);
    assert.equal(changed[16][16], baseline[16][16] + 25);
    await replay("undo");
    assert.equal(await downloadRaster(), exported);
    await replay("redo");
    assert.equal(await downloadRaster(), changedBytes);
    await replay("undo");
    paletteEdit = { editedSamples, undo: true, redo: true };
  }

  let contours;
  const downloadGeoJson = () => download(actionRow("exportGeoJson"), "terrain-overlays.geojson");
  let archive;
  if (process.argv.includes("--archive")) {
    const originalFeatures = await downloadGeoJson();
    const [saved] = await Promise.all([page.waitForEvent("download"), actionRow("exportArtifactDocument").click()]);
    assert.equal(await saved.failure(), null);
    const archiveBytes = await readFile(await saved.path());
    const filename = saved.suggestedFilename();
    assert(archiveBytes.length > 0);
    if (process.argv.includes("--capture-archive")) {
      await writeFile(new URL("./🗑️generated/mounted-terrain.semio-archive", import.meta.url), archiveBytes);
    }
    await saved.delete();
    const beforeClear = await history();
    await actionRow("clearOverlays").click();
    await waitForAction(beforeClear.cursor, "clearOverlays");
    const features = terrain.locator('[data-slot="terrain-feature-list"] li');
    await features.first().waitFor({ state: "detached" });
    assert.equal(await features.count(), 0);
    const [chooser] = await Promise.all([page.waitForEvent("filechooser"), actionRow("importArtifactDocument").click()]);
    await chooser.setFiles({ name: filename, mimeType: "application/octet-stream", buffer: archiveBytes });
    await terrain.locator('[data-slot="terrain-feature-list"] li').filter({ hasText: "Institut de Botanique" }).waitFor({ state: "attached", timeout: process.argv.includes("--long-archive") ? 420_000 : 90_000 }).catch(async (error) => {
      console.log("[DEBUG] Terrain archive current UI", JSON.stringify({ url: page.url(), history: await history(), announcements: await page.locator('[role="alert"], [role="status"]').allTextContents() }));
      console.log("[DEBUG] Terrain archive shell tail", (await page.locator("body").innerText()).slice(-4000));
      throw error;
    });
    assert.equal(await downloadRaster(), exported);
    assert.equal(await downloadGeoJson(), originalFeatures);
    archive = { filename, bytes: archiveBytes.length };
  }

  let grade;
  if (process.argv.includes("--grade")) {
    const [west, south, east, north] = oracle.bounds;
    const point = (x, y) => [west + (east - west) * x / 32, south + (north - south) * y / 32];
    const square = (lo, hi) => [point(lo, lo), point(hi, lo), point(hi, hi), point(lo, hi), point(lo, lo)];
    const features = [{ type: "Feature", id: "grading-lot", geometry: { type: "Polygon", coordinates: [square(4.5, 27.5), square(11.5, 20.5).reverse()] }, properties: { name: "Grading lot" } }];
    const beforeRegion = await history();
    const [picker] = await Promise.all([page.waitForEvent("filechooser"), actionRow("importGeoJson").click()]);
    await picker.setFiles({ name: "grading-lot.geojson", mimeType: "application/geo+json", buffer: Buffer.from(JSON.stringify({ type: "FeatureCollection", features })) });
    await waitForAction(beforeRegion.cursor, "importGeoJsonFile");
    const beforeGrade = await history();
    await actionRow("gradeTerrainRegion").click();
    const regionId = pane.getByRole("textbox", { name: "Region ID", exact: true });
    if (process.argv.includes("--diagnose")) {
      await regionId.waitFor({ state: "attached" });
      console.log("[DEBUG] Grade Region input", await regionId.evaluate((node) => node.parentElement?.parentElement?.outerHTML.slice(0, 3200)));
    }
    await regionId.fill("grading-lot").catch(async (error) => {
      console.log("[DEBUG] Grade Region form", await pane.innerText(), await pane.locator("input, textarea, label").evaluateAll((nodes) =>
        nodes.map((node) => ({ tag: node.tagName, id: node.id, for: node.getAttribute("for"), role: node.getAttribute("role"), type: node.getAttribute("type"), label: node.getAttribute("aria-labelledby") ?? node.getAttribute("aria-label"), text: node.textContent?.trim() }))));
      throw error;
    });
    await pane.getByRole("button", { name: "Execute", exact: true }).click();
    await waitForAction(beforeGrade.cursor, "gradeTerrainRegion");
    const gradedFile = await downloadRaster();
    const graded = readRaster(gradedFile);
    let changedSamples = 0;
    for (let y = 0; y < oracle.side; y++) {
      for (let x = 0; x < oracle.side; x++) {
        const inside = x >= 5 && x <= 27 && y >= 5 && y <= 27;
        const hole = x >= 12 && x <= 20 && y >= 12 && y <= 20;
        assert.equal(graded[y][x], inside && !hole ? 100 : baseline[y][x], `Graded cell ${x}, ${y}`);
        if (graded[y][x] !== baseline[y][x]) changedSamples++;
      }
    }
    await replay("undo");
    assert.equal(await downloadRaster(), exported);
    await replay("redo");
    assert.equal(await downloadRaster(), gradedFile);
    await replay("undo");
    await replay("undo");
    grade = { changedSamples, holePreserved: true, undo: true, redo: true };
  }

  if (process.argv.includes("--contours")) {
    const originalFeatures = await downloadGeoJson();
    const beforeContours = await history();
    await actionRow("generateTerrainContours").click();
    await pane.getByText("Contour Interval (m)", { exact: true }).waitFor({ state: "attached" });
    await pane.getByRole("button", { name: "Execute", exact: true }).click();
    await waitForAction(beforeContours.cursor, "generateTerrainContours");
    const contourBytes = await downloadGeoJson();
    const lines = JSON.parse(contourBytes).features.filter((feature) => feature.properties.semioTerrainContour === true);
    assert.deepEqual([...new Set(lines.map((line) => line.properties.elevationMetres))].sort((a, b) => a - b), [-25, 0, 25, 50, 75, 100, 125]);
    const [west, south, east, north] = oracle.bounds;
    for (const line of lines) {
      assert.equal(line.geometry.type, "LineString");
      for (const [longitude, latitude] of line.geometry.coordinates) {
        const expected = 32 + 96 * (longitude - west) / (east - west) - 64 * (latitude - south) / (north - south);
        assert(Math.abs(expected - line.properties.elevationMetres) < 1e-7);
      }
    }
    console.log("[DEBUG] Terrain contours matched the source heights", lines.length);
    await replay("undo");
    assert.equal(await downloadGeoJson(), originalFeatures);
    await replay("redo");
    assert.equal(await downloadGeoJson(), contourBytes);
    await replay("undo");
    contours = { lines: lines.length, undo: true, redo: true };
  }

  const viewport = terrain.locator(".semio-world-3d-host");
  let brushResult;
  let strokeResult;
  if (!process.argv.includes("--exchange-only")) {
    const fractionalBrush = process.argv.includes("--fractional-brush");
    if (fractionalBrush) {
      await setActionsOpen(true);
      await actionRow("configureTerrainBrush").click();
      const step = pane.getByRole("spinbutton", { name: "Elevation Step (m)", exact: true });
      await step.fill("0.031");
      await pane.getByRole("button", { name: "Execute", exact: true }).click();
    }
    await setActionsOpen(false);
    const brush = page.getByRole("button", { name: "Raise Terrain", exact: true });
    if (!(await brush.isVisible())) await terrain.locator('[id$=".utilityBar.unfold"]').click();
    if (await brush.getAttribute("aria-pressed") !== "true") await brush.click();
    await page.waitForFunction((expectedStep) => {
      const interaction = document.querySelector(".semio-world-3d-host")?.getAttribute("data-interaction-json");
      const args = interaction && JSON.parse(interaction).terrainBrush?.arguments;
      return args && args.operation === "raise-lower" && (expectedStep === null || args.value === expectedStep);
    }, fractionalBrush ? 0.031 : null);
    const { terrainBrush } = JSON.parse(await viewport.getAttribute("data-interaction-json"));
    assert.equal(terrainBrush.action, "editElevation");
    const beforeRaise = await history();
    await viewport.locator("canvas").first().click();
    await waitForAction(beforeRaise.cursor, terrainBrush.action);
    const raised = await history();
    await setActionsOpen(true);
    const sculpted = await downloadRaster();
    const samplesPerMetre = fractionalBrush || process.argv.includes("--millimetre-grid") ? 1000 : 1;
    const changedSamples = verifyRaise(baseline, readRaster(sculpted), terrainBrush.arguments.radius, terrainBrush.arguments.value, samplesPerMetre);
    await replay("undo");
    assert.equal(await downloadRaster(), exported, "Undo should restore the original imported heights");
    await replay("redo");
    assert.equal(await downloadRaster(), sculpted, "Redo should restore the raised heights");
    await replay("undo");
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

      const beforeStroke = await history();
      await drag(true);
      assert.equal(await downloadRaster(), exported, "Escape discards the full stroke");
      assert.equal((await history()).cursor, beforeStroke.cursor, "Cancelled strokes leave History untouched");

      await drag(false);
      await waitForAction(beforeStroke.cursor, terrainBrush.action);
      const committed = await history();
      assert.equal(committed.entries, beforeStroke.entries + 1, "One drag publishes one history entry");
      const strokeBytes = await downloadRaster();
      const differences = readRaster(strokeBytes).flatMap((row, y) => row.map((height, x) => height - baseline[y][x]));
      const raisedCells = differences.filter((height) => height > 0).length;
      assert(differences.every((height) => height >= 0), "Raising cannot lower any sample");
      assert(raisedCells > changedSamples, "Dragging edits more samples than a single click");
      await replay("undo");
      assert.equal(await downloadRaster(), exported, "One Undo restores the grid before the full drag");
      await replay("redo");
      assert.equal(await downloadRaster(), strokeBytes, "One Redo restores the exact stroke");
      await replay("undo");
      strokeResult = { raisedCells, cancelled: true, undo: true, redo: true };
    }
    brushResult = { changedSamples, radius: terrainBrush.arguments.radius, elevationStep: terrainBrush.arguments.value, raisedCursor: raised.cursor };
  }
  const beforeOverlays = await replay("undo");

  await pick(actionRow("importGeoJson"), geoJsonSource, ".geojson,.json,application/geo+json");
  await waitForAction(beforeOverlays.cursor, "importGeoJsonFile");
  const overlays = await history();
  const featureList = viewport.locator('[data-slot="terrain-feature-list"] li');
  const yard = featureList.filter({ hasText: "Survey yard" });
  const demoPin = featureList.filter({ hasText: "Institut de Botanique" });
  await yard.waitFor({ state: "attached" });
  await demoPin.waitFor({ state: "detached" });
  const exportedOverlays = await downloadGeoJson();
  assert.deepEqual(JSON.parse(exportedOverlays), geoJsonOracle);
  assert.match(exportedOverlays, /"id"\s*:\s*9007199254740993(?=\s*[,}])/u);
  assert.equal((await history()).canUndo, true);
  await replay("undo");
  await demoPin.waitFor({ state: "attached" });
  await replay("redo");
  await yard.waitFor({ state: "attached" });
  assert.equal(await downloadGeoJson(), exportedOverlays, "Redo should restore the imported GeoJSON bytes");
  let movePosition;
  if (process.argv.includes("--move-position")) {
    await setActionsOpen(false);
    const utility = page.getByRole("button", { name: "Move Position", exact: true });
    if (!(await utility.isVisible())) await terrain.locator('[id$=".utilityBar.unfold"]').click();
    await utility.click();
    await page.waitForFunction(() => {
      const interaction = document.querySelector(".semio-world-3d-host")?.getAttribute("data-interaction-json");
      return interaction && JSON.parse(interaction).terrainMovePosition?.action === "moveOverlayPosition";
    });
    const canvas = viewport.locator("canvas").first();
    const bounds = await canvas.boundingBox();
    assert(bounds);
    const marker = async () => {
      const raster = PNG.sync.read(await canvas.screenshot({ animations: "disabled" }));
      let column = 0;
      let row = 0;
      let samples = 0;
      for (let y = 0; y < raster.height; y++) {
        for (let x = 0; x < raster.width; x++) {
          const index = 4 * (y * raster.width + x);
          const [red, green, blue] = raster.data.subarray(index, index + 3);
          if (Math.abs(red - 248) + Math.abs(green - 217) + Math.abs(blue - 78) >= 8) continue;
          column += x;
          row += y;
          samples++;
        }
      }
      return samples < 4 ? null : [
        bounds.x + bounds.width * column / (raster.width * samples),
        bounds.y + bounds.height * row / (raster.height * samples),
      ];
    };
    let start;
    for (let frame = 0; frame < 10 && !start; frame++) {
      start = await marker();
      if (!start) await canvas.evaluate(() => new Promise(resolve => requestAnimationFrame(resolve)));
    }
    assert(start, "The imported marker should paint in the real terrain viewport");
    const [x, y] = start;
    const end = [
      x + Math.sign(bounds.x + bounds.width / 2 - x) * 30,
      y + Math.sign(bounds.y + bounds.height / 2 - y) * 18,
    ];
    const beforeMove = await history();
    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(end[0], end[1], { steps: 12 });
    await page.mouse.up();
    await waitForAction(beforeMove.cursor, "moveOverlayPosition");
    assert.equal((await history()).entries, beforeMove.entries + 1);
    await setActionsOpen(true);
    const movedBytes = await downloadGeoJson();
    const features = JSON.parse(movedBytes).features;
    const originalMarker = geoJsonOracle.features.find(feature => feature.geometry.type === "Point");
    const movedMarker = features.find(feature => feature.geometry.type === "Point");
    assert.notDeepEqual(movedMarker.geometry.coordinates.slice(0, 2), originalMarker.geometry.coordinates.slice(0, 2));
    assert.deepEqual(movedMarker.geometry.coordinates.slice(2), originalMarker.geometry.coordinates.slice(2));
    assert.deepEqual(movedMarker.properties, originalMarker.properties);
    assert.deepEqual(features.filter(feature => feature.geometry.type !== "Point"), geoJsonOracle.features.filter(feature => feature.geometry.type !== "Point"));
    assert.match(movedBytes, /"id"\s*:\s*9007199254740993(?=\s*[,}])/u);
    await replay("undo");
    assert.equal(await downloadGeoJson(), exportedOverlays);
    await replay("redo");
    assert.equal(await downloadGeoJson(), movedBytes);
    await replay("undo");
    movePosition = { coordinates: movedMarker.geometry.coordinates, undo: true, redo: true };
  }
  await replay("undo");

  let roleSwitch;
  if (process.argv.includes("--roles")) {
    const beforeRole = await history();
    const beforeRoleRaster = await downloadRaster();
    const beforeRoleFeatures = (await featureList.allTextContents()).toSorted();
    const roles = page.locator('[id="playground.navbar.roles"]');
    const chip = (role) => page.locator(`[data-slot="surface-role-chip"][data-role="${role}"][data-app-id="s.gis.gisterrain@1/*#${role}"]`);
    const switchTo = async (role) => {
      await roles.locator(`[data-role="${role}"]`).click();
      await chip(role).waitFor({ state: "attached", timeout: 90_000 }).catch(async (error) => {
        console.log("[DEBUG] Terrain role switch", JSON.stringify({ role, controls: await roles.innerText(), announcement: await page.locator('[role="alert"], [role="status"]').allTextContents() }));
        throw error;
      });
      await demoPin.waitFor({ state: "attached" });
      assert.deepEqual((await featureList.allTextContents()).toSorted(), beforeRoleFeatures);
    };
    const editorIdentity = await terrain.getAttribute("data-surface-document-id");
    await switchTo("viewer");
    if (editorIdentity !== null) assert.equal(await terrain.getAttribute("data-surface-document-id"), editorIdentity);
    await switchTo("editor");
    assert.deepEqual((await history()).actionIds, beforeRole.actionIds);
    assert.equal(await downloadRaster(), beforeRoleRaster);
    roleSwitch = { document: editorIdentity, viewer: true, editor: true };
  }

  assert.deepEqual(pageErrors, []);
  console.log(JSON.stringify({
    url: url.toString(),
    file: "terrain-elevation.asc",
    grid: { side: oracle.side, bounds: oracle.bounds, samples: oracle.side ** 2, checksum: oracle.checksum },
    brush: brushResult,
    paletteEdit,
    contours,
    archive,
    roleSwitch,
    movePosition,
    grade,
    stroke: strokeResult,
    overlays: { file: "terrain-overlays.geojson", features: geoJsonOracle.features.length, cursor: overlays.cursor },
    history: { originalCursor: original.cursor, importedCursor: imported.cursor, undo: true, redo: true },
  }));
} finally {
  await archiveProfile;
  await browser.close();
}
