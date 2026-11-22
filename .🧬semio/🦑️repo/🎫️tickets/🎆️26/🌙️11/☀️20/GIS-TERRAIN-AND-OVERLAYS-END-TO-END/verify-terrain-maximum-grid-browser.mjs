import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { chromium } from "playwright";
import { maximumTerrainBrowserRaster, terrainElevationFixture } from "./terrain-browser-fixtures.mjs";

const url = new URL(process.argv[2] ?? "http://127.0.0.1:6083/?example=demo");
assert.equal(url.hostname, "127.0.0.1");
async function missingRaster() {
  const { cases, exportNoDataMetres } = await terrainElevationFixture("🕳️nodata.json");
  const { asciiGrid, source } = cases.find(({ name }) => name === "mixed-extrema-and-holes");
  const { bounds: [west, south, east], rowsMillimetres, missingSamples } = source;
  const width = rowsMillimetres[0].length;
  const height = rowsMillimetres.length;
  const mask = Buffer.from(missingSamples, "hex");
  const rows = rowsMillimetres.map((row, southFirst) => row.map((sample, column) => {
    const ordinal = southFirst * width + column;
    return mask[Math.floor(ordinal / 8)] & (1 << (ordinal % 8)) ? exportNoDataMetres : sample / 1000;
  }).join(" ")).reverse();
  const spacing = (east - west) / (width - 1);
  const expected = [`NCOLS ${width}`, `NROWS ${height}`, `XLLCENTER ${west}`, `YLLCENTER ${south}`, `CELLSIZE ${spacing}`, `NODATA_VALUE ${exportNoDataMetres}`, ...rows, ""].join("\n");
  return { selected: Buffer.from(asciiGrid), expected, samples: width * height, chunks: 1, filename: "missing-samples.asc" };
}

const repair = process.argv.includes("--repair-nodata") ? await terrainElevationFixture("../../../📐️edit-elevation/🧪️tests/🌫️fill-missing/🔣️.json") : undefined;
const { selected, expected, samples, chunks, filename } = repair
  ? { selected: Buffer.from(repair.sourceAscii), expected: undefined, samples: 12, chunks: 1, filename: "repair-samples.asc" }
  : await (process.argv.includes("--nodata") ? missingRaster() : maximumTerrainBrowserRaster());

function rasterSamples(ascii, expectedSource) {
  const [, , , , , noData, ...lines] = ascii.trimEnd().split("\n");
  const sentinel = Number(noData.split(/\s+/).at(-1));
  const mask = Buffer.from(expectedSource.missingSamples ?? "", "hex");
  const rows = expectedSource.rowsMillimetres;
  const width = rows[0].length;
  const values = lines.reverse().map(line => line.trim().split(/\s+/).map(Number));
  assert.deepEqual(values, rows.map((row, y) => row.map((sample, x) => {
    const index = y * width + x;
    const missing = mask[index >> 3] & (1 << (index % 8));
    return missing ? sentinel : sample / 1000;
  })));
}

const browser = await chromium.launch({ channel: "chrome", headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] });
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1100, height: 760 }, acceptDownloads: true });
  await page.goto(url.toString(), { waitUntil: "domcontentloaded", timeout: 60_000 });
  const selector = "[data-semio-os-ready][data-history-json]";
  const shell = page.locator(selector);
  const viewport = page.locator(".semio-world-3d-host");
  await viewport.locator('[data-slot="terrain-feature-list"] li').first().waitFor({ state: "attached", timeout: 60_000 });
  assert.equal(await shell.locator('[data-slot="surface-role-chip"]').getAttribute("data-app-id"), "s.gis.gisterrain@1/*#editor");
  const window = shell.locator('[data-slot="window-body"]').filter({ has: viewport }).first();
  const overlay = window.locator('[data-slot="window-engagement-overlay"]');
  if (await overlay.getAttribute("data-folded") === "true") await overlay.locator('button[id$=".engagement.toggle"]').click();
  const row = (id) => overlay.locator(`[data-slot="tree-item-row"][id="action.${id}"]`);
  const history = async () => JSON.parse(await shell.getAttribute("data-history-json"));
  const waitAction = (before, action) => page.waitForFunction(({ selector, before, action }) => {
    const raw = document.querySelector(selector)?.getAttribute("data-history-json");
    const state = raw && JSON.parse(raw);
    return state && state.cursor > before && state.actionIds.at(-1) === action;
  }, { selector, before, action }, { timeout: 120_000 });
  const exportRaster = async () => {
    const [file] = await Promise.all([page.waitForEvent("download", { timeout: 120_000 }), row("exportElevationGrid").click()]);
    assert.equal(file.suggestedFilename(), "terrain-elevation.asc");
    assert.equal(await file.failure(), null);
    try {
      return await readFile(await file.path(), "utf8");
    } finally {
      await file.delete();
    }
  };
  const baseline = await exportRaster();
  const previous = await history();
  const [chooser] = await Promise.all([page.waitForEvent("filechooser"), row("importElevationGrid").click()]);
  assert.equal(await chooser.element().getAttribute("accept"), ".asc");
  await chooser.setFiles({ name: filename, mimeType: "text/plain", buffer: selected });
  await waitAction(previous.cursor, "importElevationGridFile");
  const imported = await history();
  assert.equal(imported.actionIds.filter(id => id === "importElevationGridFile").length, 1);
  const reexport = await exportRaster();
  if (repair) rasterSamples(reexport, repair.source);
  else assert.equal(reexport, expected);
  const modifier = process.platform === "darwin" ? "Meta" : "Control";
  await page.keyboard.press(`${modifier}+z`);
  await waitAction(imported.cursor, "undo");
  const undone = await history();
  assert.equal(await exportRaster(), baseline);
  await page.keyboard.press(`${modifier}+Shift+z`);
  await waitAction(undone.cursor, "redo");
  assert.equal(await exportRaster(), reexport);
  if (repair) {
    const original = await history();
    const { command, expected } = repair.cases[0];
    await row("editElevation").click();
    await overlay.getByRole("combobox").click();
    await page.getByRole("option", { name: "Fill Missing Samples", exact: true }).click();
    for (const [label, value] of [
      ["Column from West", command.column], ["Row from South", command.row],
      ["Radius in Samples", command.radius], ["Meters or Smoothing Percent", command.value],
    ]) {
      await overlay.getByRole("spinbutton", { name: label, exact: true }).fill(String(value));
    }
    await overlay.getByRole("button", { name: "Execute", exact: true }).click();
    await waitAction(original.cursor, "editElevation");
    const repaired = await history();
    assert.equal(repaired.entries, original.entries + 1);
    const repairedBytes = await exportRaster();
    rasterSamples(repairedBytes, expected);
    await page.keyboard.press(`${modifier}+z`);
    await waitAction(repaired.cursor, "undo");
    const beforeRedo = await history();
    assert.equal(await exportRaster(), reexport);
    await page.keyboard.press(`${modifier}+Shift+z`);
    await waitAction(beforeRedo.cursor, "redo");
    assert.equal(await exportRaster(), repairedBytes);
  }
  console.log(JSON.stringify({ editor: url.toString(), samples, bytes: selected.byteLength, chunks, filename, repaired: Boolean(repair), undo: true, redo: true }));
} finally {
  await browser.close();
}
