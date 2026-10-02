/** 🏔️ Actual GIS terrain package boundary preserves exact words and semantic text. */
import {gisTerrainSnapshotToSqliteDatabase,gisTerrainSnapshotFromSqliteDatabase,validateGisTerrainSnapshotSqliteDialect,type GisTerrainArtifact} from "@semio-tech/gis-gisterrain";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import fixture from "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json";
test("GIS terrain public package preserves signaling NaN and separately owned mesh handles",async()=>{
 const snapshot:GisTerrainArtifact={exaggeration:{bits:BigInt("0x"+fixture.words[5]!)},importedFeaturesJson:fixture.importedFeaturesJson,mesh:fixture.child};
 const oracle=Database.deserialize(await exportSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(snapshot)));
 try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(oracle.query("SELECT exaggeration,exaggeration_numeric_class FROM gis_terrain_parameters").get()).toEqual({exaggeration:null,exaggeration_numeric_class:"nan"});
  oracle.run("UPDATE gis_terrain_document SET id=-42;UPDATE gis_terrain_parameters SET id=-42;UPDATE gis_terrain_mesh_child SET id=-42");
  const database=await importSqliteDatabase(oracle.serialize());
  expect(await gisTerrainSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
  expect(await validateGisTerrainSnapshotSqliteDialect(snapshot,fixture.dialect,database)).toEqual([]);
  oracle.query("UPDATE gis_terrain_parameters SET imported_features_json=?").run("independent plain text edit");
  const edited=await importSqliteDatabase(oracle.serialize());
  expect((await gisTerrainSnapshotFromSqliteDatabase(edited)).importedFeaturesJson).toBe("independent plain text edit");
  await expect(validateGisTerrainSnapshotSqliteDialect(snapshot,fixture.dialect,edited)).rejects.toThrow("identity");
 }finally{oracle.close();}
});
