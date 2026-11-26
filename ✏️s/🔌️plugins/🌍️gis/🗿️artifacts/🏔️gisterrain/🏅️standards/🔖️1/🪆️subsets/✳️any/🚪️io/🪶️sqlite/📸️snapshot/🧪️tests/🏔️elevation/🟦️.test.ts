import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework/io/sqlite-snapshot";
import { gisTerrainSnapshotFromSqliteDatabase, gisTerrainSnapshotToSqliteDatabase } from "../../🟦️.ts";
import { parseElevationSource } from "../../../../../🧬️schema/🗺️imported-map/🏔️elevation/🟦️.ts";
import schema from "../../../../../🧬️schema/🗺️imported-map/🏔️elevation/🔣️.json" with { type: "json" };
import fixture from "../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🏔️replace-elevation-source/🏔️authors/🦠️mutation/🔣️.json" with { type: "json" };

const authored = fixture.ReplaceElevationSource.newElevationSource;
const surface = { ...authored, bounds: authored.bounds as [number, number, number, number] };
const validate = new Ajv().addKeyword("x-semio-ui").compile(schema);

test("authored fractional elevations have rectangular 2–65 dimensions and signed millimetres", () => {
  expect(validate(surface)).toBe(true);
  expect(parseElevationSource(surface)).toEqual(surface);
  const maximum = Array.from({ length: 65 }, () => Array(65).fill(123));
  const small = [[-2147483648, 0], [12, 2147483647]];
  for (const rowsMillimetres of [maximum, small]) {
    const value = { bounds: surface.bounds, rowsMillimetres };
    expect(validate(value)).toBe(true);
    expect(parseElevationSource(value)).toEqual(value);
  }
  const refused = [
    [[0, 1], [2, 3, 4]],
    [[0, 1]],
    [[0], [1]],
    [...maximum, maximum[0]],
    [Array(66).fill(0), Array(66).fill(0)],
    [[1, 2], [3, 2147483648]],
    [[1, 2], [3, -2147483649]],
    [[1, 2], [3, 0.125]],
  ];
  for (const rowsMillimetres of refused) {
    const value = { bounds: surface.bounds, rowsMillimetres };
    expect(validate(value)).toBe(false);
    expect(() => parseElevationSource(value)).toThrow();
  }
  const former = { bounds: surface.bounds, rows: surface.rowsMillimetres };
  expect(validate(former)).toBe(false);
  expect(() => parseElevationSource(former)).toThrow();
});

test("the largest grid fits its exact normalized row budget", async () => {
  const rowsMillimetres = Array.from({ length: 65 }, () => Array<number>(65).fill(125));
  rowsMillimetres[64]![64] = -2147483648;
  const elevationSource = { bounds: surface.bounds, rowsMillimetres };
  const snapshot = { exaggeration: { bits: 0x3ff0000000000000n }, elevationSource };
  const maxRows = 2 + 1 + 65 * 65;
  await expect(gisTerrainSnapshotToSqliteDatabase(snapshot, { maxRows: maxRows - 1 })).rejects.toThrow("row");
  const projected = await gisTerrainSnapshotToSqliteDatabase(snapshot, { maxRows });
  expect((await gisTerrainSnapshotFromSqliteDatabase(projected, { maxRows })).elevationSource).toEqual(elevationSource);
});

test("SQLite recovers shape, signed elevations and dense ownership independently", async () => {
  const snapshot = { exaggeration: { bits: 0x3ff0000000000000n }, elevationSource: surface };
  const projected = await gisTerrainSnapshotToSqliteDatabase(snapshot);
  expect((await gisTerrainSnapshotFromSqliteDatabase(projected)).elevationSource).toEqual(surface);
  const bytes = await exportSqliteDatabase(projected);
  const oracle = Database.deserialize(bytes);
  try {
    expect(oracle.query("SELECT column_count FROM gis_terrain_elevation_source").get()).toEqual({ column_count: 4 });
    const heights = oracle.query("SELECT elevation_millimetres AS height FROM gis_terrain_elevation_sample ORDER BY ordinal").all() as { height: number }[];
    expect(heights.map(({ height }) => height)).toEqual(surface.rowsMillimetres.flat());
    expect(() => oracle.exec("UPDATE gis_terrain_elevation_sample SET elevation_millimetres = 2147483648")).toThrow();
    oracle.exec("DELETE FROM gis_terrain_elevation_sample WHERE ordinal = 11");
    await expect(gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow("sample count");
  } finally {
    oracle.close();
  }
});

test("SQLite saves exact missing ordinals independently of valid zero and signed extrema", async () => {
  const elevationSource = parseElevationSource({
    bounds: [5, 50, 5.375, 50.25],
    rowsMillimetres: [[-2147483648, 0, 0, 2147483647], [-9999000, 20000, 30000, 40000], [1000, 2000, 0, 4000]],
    missingSamples: "0204",
  });
  expect(validate(elevationSource)).toBe(true);
  for (const missingSamples of [null, "", "0000", "020F", "02f4", "0104", "020400"]) {
    expect(() => parseElevationSource({ ...elevationSource, missingSamples })).toThrow();
  }
  const projected = await gisTerrainSnapshotToSqliteDatabase({ exaggeration: { bits: 0x3ff0000000000000n }, elevationSource });
  const oracle = Database.deserialize(await exportSqliteDatabase(projected));
  try {
    expect(oracle.query("SELECT missing_samples FROM gis_terrain_elevation_source").get()).toEqual({ missing_samples: "0204" });
    expect((await gisTerrainSnapshotFromSqliteDatabase(projected)).elevationSource).toEqual(elevationSource);
    oracle.exec("UPDATE gis_terrain_elevation_source SET missing_samples = '02f4'");
    await expect(gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow("padding");
  } finally {
    oracle.close();
  }
});
