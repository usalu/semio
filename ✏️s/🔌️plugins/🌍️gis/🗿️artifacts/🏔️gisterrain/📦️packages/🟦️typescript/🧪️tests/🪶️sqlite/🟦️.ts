/** 🏔️ Actual GIS terrain package boundary preserves exact words and independent child identities. */
import {gisTerrainSnapshotToSqliteDatabase,gisTerrainSnapshotFromSqliteDatabase,validateGisTerrainSnapshotSqliteDialect,type GisTerrainArtifact} from "@semio-tech/gis-gisterrain";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import fixture from "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json";
import liege from "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🏔️elevation.json";
test("GIS terrain public package preserves signaling NaN and separately owned mesh handles",async()=>{
 const snapshot:GisTerrainArtifact={exaggeration:{bits:BigInt("0x"+fixture.words[5]!)},mesh:fixture.child};
 const oracle=Database.deserialize(await exportSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(snapshot)));
 try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(oracle.query("SELECT exaggeration,exaggeration_numeric_class FROM gis_terrain_parameters").get()).toEqual({exaggeration:null,exaggeration_numeric_class:"nan"});
  oracle.run("UPDATE gis_terrain_document SET id=-42;UPDATE gis_terrain_parameters SET id=-42;UPDATE gis_terrain_mesh_child SET id=-42");
  const database=await importSqliteDatabase(oracle.serialize());
  expect(await gisTerrainSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
  expect(await validateGisTerrainSnapshotSqliteDialect(snapshot,fixture.dialect,database)).toEqual([]);
  oracle.query("UPDATE gis_terrain_mesh_child SET child_id=?").run("independent child identity edit");
  const edited=await importSqliteDatabase(oracle.serialize());
  expect((await gisTerrainSnapshotFromSqliteDatabase(edited)).mesh?.childId).toBe("independent child identity edit");
  await expect(validateGisTerrainSnapshotSqliteDialect(snapshot,fixture.dialect,edited)).rejects.toThrow("identity");
 }finally{oracle.close();}
});

test("GIS elevation samples are normalized, ordered and independently editable in SQLite", async () => {
 const snapshot: GisTerrainArtifact = {
  exaggeration: {bits: 0x3ff0000000000000n},
  elevationSource: {bounds: [-0, 0, 0.04, 0.04], rows: liege.rows},
 };
 const oracle = Database.deserialize(await exportSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(snapshot)));
 try {
  expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(oracle.query("SELECT printf('%016x',west_ieee754_bits) AS word FROM gis_terrain_elevation_source").get()).toEqual({word: "8000000000000000"});
  expect(oracle.query("SELECT count(*) AS samples,min(elevation_meters) AS low,max(elevation_meters) AS high FROM gis_terrain_elevation_sample").get()).toEqual({samples: 1089, low: 47, high: 252});
  const unedited = await importSqliteDatabase(oracle.serialize());
  expect(await validateGisTerrainSnapshotSqliteDialect(snapshot, fixture.dialect, unedited)).toEqual([]);
  oracle.query("UPDATE gis_terrain_elevation_sample SET elevation_meters = -32768 WHERE ordinal = 34").run();
  const edited = await gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()));
  expect(edited.elevationSource?.rows[1]?.[1]).toBe(-32768);
  expect(Object.is(edited.elevationSource?.bounds[0], -0)).toBe(true);
  oracle.query("DELETE FROM gis_terrain_elevation_sample WHERE ordinal = 1088").run();
  await expect(gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow("sample count");
 } finally {
  oracle.close();
 }
});
