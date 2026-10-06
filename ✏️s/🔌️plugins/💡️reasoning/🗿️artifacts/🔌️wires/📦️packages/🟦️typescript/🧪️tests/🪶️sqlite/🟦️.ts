/** 🔌️ Named Wires package exposes complete exact intrinsic state to independent SQL. */
import{wiresSnapshotToSqliteDatabase,wiresSnapshotFromSqliteDatabase,validateWiresSnapshotSqliteDialect,decodeWiresJsonValue,type WiresSnapshot}from"@semio-tech/reasoning-wires";
import{exportSqliteDatabase,importSqliteDatabase}from"@semio-tech/framework";
import{Database}from"bun:sqlite";import{expect,test}from"bun:test";
import fixture from"../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json";
test("Wires public package retains intrinsic octets and exact negative IEEE word",async()=>{
 const snapshot:WiresSnapshot={content:fixture.content,wiresSnapshot:{kind:"bytes",value:Uint8Array.from(fixture.byteValues)},meta:{kind:"float",value:{bits:BigInt("0x"+fixture.floatWords[6]!)}}},database=await wiresSnapshotToSqliteDatabase(snapshot),oracle=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("SELECT value FROM wires_octet ORDER BY ordinal").all()).toEqual(fixture.byteValues.map(value=>({value})));
  expect(await wiresSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).toEqual(snapshot);expect(await validateWiresSnapshotSqliteDialect(snapshot,fixture.dialect,database)).toEqual([]);
  oracle.run("UPDATE wires_octet SET value=42 WHERE ordinal=0");const edited=await wiresSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()));expect(edited.wiresSnapshot).toEqual({kind:"bytes",value:Uint8Array.from([42,...fixture.byteValues.slice(1)])});await expect(validateWiresSnapshotSqliteDialect(snapshot,fixture.dialect,await importSqliteDatabase(oracle.serialize()))).rejects.toThrow();
 }finally{oracle.close();}
});
test("Wires JSON wire decoder retains finite JSON meaning separately from full intrinsic canonical state",()=>{
 const original=JSON.parse('{"a":1,"b":-2,"c":0.5,"d":[null,true,"Unicode 🪐"]}');expect(decodeWiresJsonValue(original)).toEqual({kind:"object",members:[{name:"a",value:{kind:"unsigned",value:1n}},{name:"b",value:{kind:"signed",value:-2n}},{name:"c",value:{kind:"float",value:{bits:4602678819172646912n}}},{name:"d",value:{kind:"array",items:[{kind:"null"},{kind:"boolean",value:true},{kind:"text",value:"Unicode 🪐"}]}}]});expect(()=>decodeWiresJsonValue(Number.NaN)).toThrow();expect(decodeWiresJsonValue(JSON.parse("18446744073709551616"))).toEqual({kind:"float",value:{bits:4895412794951729152n}});
});
