/** 🗺️ Actual GIS map package boundary with independently queried intrinsic values. */
import {gisMapSnapshotToSqliteDatabase,gisMapSnapshotFromSqliteDatabase,validateGisMapSnapshotSqliteDialect,type GisMapArtifact} from "@semio-tech/gis-gismap-js";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import fixture from "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json";
test("GIS map public package preserves full intrinsic words and independent target identities",async()=>{
 const maximum=fixture.value.members[2]!.value.value,word=fixture.value.members[4]!.value.bits;
 if(typeof maximum!=="string"||typeof word!=="string")throw Error("Neutral intrinsic fixture differs");
 const snapshot:GisMapArtifact={positions:[{id:"maximum",data:{kind:"unsigned",value:BigInt(maximum)}},{id:"signal",data:{kind:"float",value:{bits:BigInt("0x"+word)}}}],routes:[],regions:[],drawing:fixture.child,value:fixture.child,image:fixture.child};
 const oracle=Database.deserialize(await exportSqliteDatabase(await gisMapSnapshotToSqliteDatabase(snapshot)));
 try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(oracle.query("SELECT high,low FROM gis_map_unsigned").get()).toEqual({high:4294967295,low:4294967295});
  oracle.run("UPDATE gis_map_position SET id=-id-100");
  const database=await importSqliteDatabase(oracle.serialize());
  expect(await gisMapSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
  expect(await validateGisMapSnapshotSqliteDialect(snapshot,fixture.dialect,database)).toEqual([]);
  oracle.run("UPDATE gis_map_position SET feature_id='edited' WHERE ordinal=0");
  const edited=await importSqliteDatabase(oracle.serialize());
  expect((await gisMapSnapshotFromSqliteDatabase(edited)).positions[0]!.id).toBe("edited");
  await expect(validateGisMapSnapshotSqliteDialect(snapshot,fixture.dialect,edited)).rejects.toThrow("identity");
 }finally{oracle.close();}
});
