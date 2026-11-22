import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const directory = new URL("../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/", import.meta.url);
export const terrainElevationFixture = async (name) => JSON.parse(await readFile(new URL(name, directory), "utf8"));

export async function maximumTerrainBrowserRaster() {
  const { maximumPicker: { side, west, south, cellSize, noData, exportNoData, northSample, otherSample, bytes, chunkCount } } = await terrainElevationFixture("🔣️.json");
  const top = new Array(side).fill(northSample).join(" ");
  const bottom = new Array(side).fill(otherSample).join(" ");
  const rows = [top, ...new Array(side - 1).fill(bottom)];
  const header = [`NCOLS ${side}`, `NROWS ${side}`, `XLLCENTER ${west}`, `YLLCENTER ${south}`, `CELLSIZE ${cellSize}`, `NODATA_VALUE ${noData}`];
  const selected = Buffer.from([...header, ...rows, ""].join("\n").padEnd(bytes));
  const expected = [...header.slice(0, -1), "NODATA_VALUE " + exportNoData, ...rows, ""].join("\n");
  assert.equal(selected.byteLength, bytes);
  assert.equal(Math.ceil(selected.byteLength / (32 * 1024)), chunkCount);
  return { selected, expected, samples: side * side, chunks: chunkCount, filename: "signed-maximum.asc" };
}

