/** 🏔️ Exact terrain words and independently editable SQL parameters. */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import {gisTerrainSnapshotToSqliteDatabase,gisTerrainSnapshotFromSqliteDatabase,validateGisTerrainSnapshotSqliteDialect} from "../../🪶️sqlite/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
function snapshot(word:string,mesh=true){return {exaggeration:{bits:BigInt("0x"+word)},...(mesh?{mesh:fixture.child}:{})};}
test("GIS terrain retains every native IEEE word, absent imported map and independent child handle",async()=>{for(const word of fixture.words)for(const present of [false,true]){const value=snapshot(word,present),db=await gisTerrainSnapshotToSqliteDatabase(value);expect(await gisTerrainSnapshotFromSqliteDatabase(db)).toEqual(value);const sql=Database.deserialize(await exportSqliteDatabase(db));try{expect(sql.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(sql.query("SELECT count(*) AS count FROM gis_terrain_imported_map").get()).toEqual({count:0});const query=sql.query("SELECT exaggeration_numeric_class AS kind,exaggeration FROM gis_terrain_parameters").get()as{kind:string;exaggeration:number|null};if(word.includes("7ff0000000000001")||word.startsWith("fff8")){expect(query.kind).toBe("nan");expect(query.exaggeration).toBe(null);}expect(await gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())))).toEqual(value);}finally{sql.close();}}});
test("GIS terrain accepts consistent surrogate renumbering and valid independent edits but rejects mismatched words and missing ownership",async()=>{const value=snapshot(fixture.words[2]!);const sql=Database.deserialize(await exportSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(value)));try{sql.run("UPDATE gis_terrain_document SET id=42; UPDATE gis_terrain_parameters SET id=42; UPDATE gis_terrain_mesh_child SET id=42");const db=await importSqliteDatabase(new Uint8Array(sql.serialize()));expect(await validateGisTerrainSnapshotSqliteDialect(value,fixture.dialect,db)).toEqual([]);sql.query("UPDATE gis_terrain_mesh_child SET child_id=?").run("independently edited mesh identity");const changed=await importSqliteDatabase(new Uint8Array(sql.serialize()));expect((await gisTerrainSnapshotFromSqliteDatabase(changed)).mesh?.childId).toBe("independently edited mesh identity");await expect(validateGisTerrainSnapshotSqliteDialect(value,fixture.dialect,changed)).rejects.toThrow("identity");sql.run("UPDATE gis_terrain_parameters SET exaggeration=2");await expect(gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())))).rejects.toThrow("IEEE");}finally{sql.close();}for(const mutation of ["DELETE FROM gis_terrain_parameters","UPDATE gis_terrain_mesh_child SET id=9","INSERT INTO gis_terrain_document VALUES(9)"]){const sql=Database.deserialize(await exportSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(value)));try{sql.run(mutation);await expect(gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(sql.serialize())))).rejects.toThrow();}finally{sql.close();}}});
test("GIS terrain admits budgets before copies and reports cancellation through both directions",async()=>{const value=snapshot(fixture.words[1]!),db=await gisTerrainSnapshotToSqliteDatabase(value);for(const options of [{maxRows:2},{maxValueBytes:16}]){await expect(gisTerrainSnapshotToSqliteDatabase(value,options)).rejects.toThrow("limit");await expect(gisTerrainSnapshotFromSqliteDatabase(db,options)).rejects.toThrow("limit");}const controller=new AbortController();controller.abort();await expect(gisTerrainSnapshotToSqliteDatabase(value,{signal:controller.signal})).rejects.toMatchObject({kind:"canceled"});await expect(gisTerrainSnapshotFromSqliteDatabase(db,{signal:controller.signal})).rejects.toMatchObject({kind:"canceled"});await expect(validateGisTerrainSnapshotSqliteDialect(value,{...fixture.dialect,subset:"invented"},db)).rejects.toThrow("subset");});
import {parseGisTerrainArtifact} from "../../../🟦️.ts";
import {Buffer} from "node:buffer";
import Ajv from "ajv";
import importedMapFixture from "../../🧫️fixtures/🪶️sqlite/🗺️imported-map/🔣️.json";
import importedMapSchema from "../../🧫️fixtures/🪶️sqlite/🗺️imported-map/🧬️schema/🔣️.json";
import {parseIntrinsicValue,type IntrinsicValue,type IntrinsicMember} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
type MapWireValue={kind:string;value?:string|boolean;word?:string;hex?:string;items?:MapWireValue[];members?:{name:string;value:MapWireValue}[]};
function importedMapValue(wire:MapWireValue):IntrinsicValue{
 switch(wire.kind){
  case "null":return{kind:"null"};
  case "boolean":return{kind:"boolean",value:wire.value as boolean};
  case "unsigned":{const decimal=wire.value as string;if(!/^(0|[1-9][0-9]*)$/.test(decimal))throw Error("canonical unsigned decimal required");return parseIntrinsicValue({kind:"unsigned",value:BigInt(decimal)});}
  case "signed":{const decimal=wire.value as string;if(!/^(0|[1-9][0-9]*|-[1-9][0-9]*)$/.test(decimal))throw Error("canonical signed decimal required");return parseIntrinsicValue({kind:"signed",value:BigInt(decimal)});}
  case "float":return{kind:"float",value:{bits:BigInt("0x"+wire.word!)}};
  case "text":return{kind:"text",value:wire.value as string};
  case "bytes":return{kind:"bytes",value:Uint8Array.from(wire.hex!.match(/../g)??[],byte=>parseInt(byte,16))};
  case "array":return{kind:"array",items:wire.items!.map(importedMapValue)};
  case "object":return{kind:"object",members:wire.members!.map(member=>({name:member.name,value:importedMapValue(member.value)}))};
  default:throw Error("closed imported map fixture variant required");
 }
}
test("terrain owns complete typed imported map roles and every intrinsic domain as independently editable SQL entities",async()=>{
 const valid=new Ajv({allErrors:true,strict:true}).compile(importedMapSchema);expect(valid(importedMapFixture)).toBe(true);
 expect(valid({...importedMapFixture,hiddenCarrier:"opaque"})).toBe(false);
 const words=new DataView(new ArrayBuffer(8));for(const word of importedMapFixture.words){words.setBigUint64(0,BigInt("0x"+word),true);expect(words.getBigUint64(0,true).toString(16).padStart(16,"0")).toBe(word);}
 const properties:IntrinsicMember[]=importedMapFixture.properties.map(member=>({name:member.name,value:importedMapValue(member.value)}));
 const feature=(wire:MapWireValue)=>{const value=importedMapValue(wire);if(value.kind!=="object")throw Error("neutral feature object required");return value;};const importedMap={positions:[feature(importedMapFixture.position)],routes:[feature(importedMapFixture.route)],regions:[feature(importedMapFixture.region)],properties};
 const snapshot=parseGisTerrainArtifact({exaggeration:{bits:BigInt("0x"+importedMapFixture.words[6]!)},importedMap,mesh:importedMapFixture.child});
 expect(snapshot).toEqual({exaggeration:{bits:BigInt("0x"+importedMapFixture.words[6]!)},importedMap,mesh:importedMapFixture.child});
 const oracle=Database.deserialize(await exportSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(snapshot)));
 try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const names=oracle.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all().map(row=>(row as{name:string}).name);
  for(const name of importedMapFixture.semanticTables)expect(names).toContain(name);
  const columns=oracle.query("PRAGMA table_info(gis_terrain_parameters)").all().map(row=>(row as{name:string}).name);expect(columns).not.toContain("imported_features_json");
  for(const role of ["position","route","region"])expect(oracle.query(`SELECT count(*) AS count FROM gis_terrain_${role}`).get()).toEqual({count:1});
  const restored=await gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()));expect(restored).toEqual(snapshot);
  expect(await validateGisTerrainSnapshotSqliteDialect(snapshot,importedMapFixture.dialect,await importSqliteDatabase(oracle.serialize()))).toEqual([]);
  oracle.query("UPDATE gis_terrain_text SET value=? WHERE value=?").run("independently edited latitude label", "Latitude\nBreite");
  const changed=await gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()));const edited=structuredClone(importedMap);const position=edited.positions[0]!;if(position.kind!=="object")throw Error("closed position object required");position.members.find(member=>member.name==="label")!.value={kind:"text",value:"independently edited latitude label"};
  expect(changed).toEqual({...snapshot,importedMap:edited});
  await expect(validateGisTerrainSnapshotSqliteDialect(snapshot,importedMapFixture.dialect,await importSqliteDatabase(oracle.serialize()))).rejects.toThrow("identity");
 }finally{oracle.close();}
 for(const overlay of [undefined,{positions:[],routes:[],regions:[],properties:[]}]){const value=parseGisTerrainArtifact({exaggeration:{bits:0n},...(overlay?{importedMap:overlay}:{})});expect(await gisTerrainSnapshotFromSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(value))).toEqual(value);}
});
test("terrain canonical native UTF8 fields refuse independently proven lossy UTF16 states",()=>{for(const units of fixture.invalidUtf16CodeUnits){const invalid=String.fromCharCode(...units);expect(Buffer.from(invalid,"utf8").toString("utf8")).not.toBe(invalid);expect(()=>parseGisTerrainArtifact({...snapshot(fixture.words[0]!),importedMap:{positions:[],routes:[],regions:[],properties:[{name:invalid,value:{kind:"null"}}]}})).toThrow();}});
function completeSnapshot(){return parseGisTerrainArtifact({exaggeration:{bits:0n},mesh:importedMapFixture.child,importedMap:{positions:[importedMapValue(importedMapFixture.position)],routes:[importedMapValue(importedMapFixture.route)],regions:[importedMapValue(importedMapFixture.region)],properties:importedMapFixture.properties.map(m=>({name:m.name,value:importedMapValue(m.value)}))}});}
test("neutral intrinsic integer decoder refuses overflow and noncanonical decimal before native admission",()=>{
 for(const[kind,values]of[["unsigned",["18446744073709551616","-1","00","+1"]],["signed",["9223372036854775808","-9223372036854775809","-0","01","+1"]]]as const)for(const value of values)expect(()=>importedMapValue({kind,value})).toThrow();
 for(const[kind,value]of[["unsigned","18446744073709551615"],["unsigned","0"],["signed","9223372036854775807"],["signed","-9223372036854775808"],["signed","0"]]as const)expect(importedMapValue({kind:kind!,value})).toEqual({kind,value:BigInt(value!)});
});
test("unsigned SQL magnitudes are independent canonical decimal text with exact full range",async()=>{
 const value=completeSnapshot(),oracle=Database.deserialize(await exportSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(value)));
 try{expect(oracle.query("SELECT value,typeof(value) AS storage FROM gis_terrain_unsigned").get()).toEqual({value:"18446744073709551615",storage:"text"});expect(oracle.query("SELECT CAST(value AS TEXT) AS value FROM gis_terrain_signed").get()).toEqual({value:"-9223372036854775808"});
  oracle.query("UPDATE gis_terrain_unsigned SET value=?").run("9007199254740993");const changed=await gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()));expect(changed.importedMap!.properties.find(m=>m.name==="uint64")!.value).toEqual({kind:"unsigned",value:9007199254740993n});
  for(const decimal of["18446744073709551616","-1","00","+1"," 1","1.0",""]){oracle.query("UPDATE gis_terrain_unsigned SET value=?").run(decimal);await expect(gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow();}
 }finally{oracle.close();}
});
test("complete imported relational trees reject kind mismatches sharing orphan rows and ordinal gaps",async()=>{
 const bytes=await exportSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(completeSnapshot()));
 for(const mutation of["UPDATE gis_terrain_route SET value_id=(SELECT value_id FROM gis_terrain_position LIMIT 1)","UPDATE gis_terrain_value SET kind='text' WHERE id=(SELECT value_id FROM gis_terrain_position LIMIT 1)","INSERT INTO gis_terrain_boolean SELECT id,1 FROM gis_terrain_value WHERE kind='null' LIMIT 1","UPDATE gis_terrain_member SET ordinal=ordinal+1","UPDATE gis_terrain_position SET ordinal=1","INSERT INTO gis_terrain_value VALUES(100000,'null')","UPDATE gis_terrain_array SET value_id=parent_id","INSERT INTO gis_terrain_document VALUES(8)"]){const oracle=Database.deserialize(bytes);try{oracle.run(mutation);await expect(gisTerrainSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow();}finally{oracle.close();}}
});
import {importedMapFromMedia,intrinsicFromJson,intrinsicToJson} from "../../../🗺️imported-map/🟦️.ts";
test("typed map admission retains broad records repeated IDs unknown properties and every intrinsic domain",async()=>{
 const value=completeSnapshot(),map=value.importedMap!,all=map.properties.map(m=>m.value);for(const role of["positions","routes","regions"]as const){map[role][0]!.members.push({name:"all domains",value:{kind:"array",items:structuredClone(all)}});map[role].push(structuredClone(map[role][0]!));}
 map.positions.push({kind:"object",members:[{name:"lon",value:{kind:"unsigned",value:1n}},{name:"lat",value:{kind:"signed",value:-1n}}]});expect(await gisTerrainSnapshotFromSqliteDatabase(await gisTerrainSnapshotToSqliteDatabase(value))).toEqual(value);
 const empty={positions:[],routes:[],regions:[],properties:[]};expect(importedMapFromMedia(intrinsicFromJson({}))).toEqual(empty);expect(importedMapFromMedia(intrinsicFromJson({positions:[],extra:"retained"})).properties).toEqual([{name:"extra",value:{kind:"text",value:"retained"}}]);
 for(const bad of[{positions:null},{routes:null},{regions:[null]},null])expect(()=>importedMapFromMedia(intrinsicFromJson(bad))).toThrow();expect(()=>parseGisTerrainArtifact({exaggeration:{bits:0n},importedMap:{...empty,properties:[{name:"positions",value:{kind:"null"}}]}})).toThrow();
 for(const member of map.properties.filter(m=>["octets","uint64","exact words"].includes(m.name)))expect(()=>intrinsicToJson(member.value)).toThrow();expect(()=>intrinsicToJson({kind:"signed",value:1n})).toThrow();expect(()=>intrinsicToJson({kind:"object",members:[{name:"x",value:{kind:"null"}},{name:"x",value:{kind:"null"}}]})).toThrow();
});

import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
test("GIS native literal semantic bytes are distinct from backing and codec framing",()=>{
 const path=resolve(import.meta.dir,"../../🧫️fixtures/🪶️sqlite/📏️native-values/🔣️.json");expect(existsSync(path),"closed native semantic byte contract").toBe(true);
 const f=JSON.parse(readFileSync(path,"utf8")),schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧫️fixtures/🪶️sqlite/📏️native-values/🧬️schema/🔣️.json"),"utf8"));
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(f)).toBe(true);expect(validate({...f,physicalAsSemantic:true})).toBe(false);
 const text=f.text.repeat(f.repetitions);expect(Buffer.byteLength(text)).toBe(f.textBytes);expect(new TextEncoder().encode(text).length).toBe(f.textBytes);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE leaves(bytes INTEGER NOT NULL CHECK(bytes>=0));");for(const bytes of[8,Buffer.byteLength(f.property),Buffer.byteLength(text)])db.query("INSERT INTO leaves VALUES(?)").run(bytes);
  expect(db.query("SELECT sum(bytes) AS bytes FROM leaves").get()).toEqual({bytes:f.totalBytes});expect(f.totalBytes).toBeGreaterThan(f.maximumValueBytes);expect(f.maximumAllocationBytes).toBeGreaterThan(f.totalBytes);
 }finally{db.close();}
});
