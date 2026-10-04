import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import fixture from"../../🧫️fixtures/🪶️sqlite/🔣️.json";
import lawsSchema from"../../🧫️fixtures/🪶️sqlite/🧬️schema/🔣️.json";
import nativeSchema from"../../🔣️.json";
import artifactSchema from"../../../🔣️.json";
import childSchema from"../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json";
import ioSchema from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json";
import * as owner from"../../🟦️.ts";
import{binary64,type Binary64}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type{ArtifactSqliteOptions}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
type Reference=Omit<owner.CadReference,"origin"|"orientation"|"scale"|"widthWorld"|"opacity">&{origin:[Binary64,Binary64,Binary64];orientation:[Binary64,Binary64,Binary64,Binary64]|null;scale:Binary64|null;widthWorld:Binary64;opacity:Binary64|null};
type Snapshot=Omit<owner.CadSnapshot,"referencesByModelDefinitionId">&{referencesByModelDefinitionId:Record<string,Reference[]>};
type Ports={cadSnapshotToSqliteDatabase(snapshot:Snapshot,options?:ArtifactSqliteOptions):Promise<SqliteDatabase>;cadSnapshotFromSqliteDatabase(database:SqliteDatabase,options?:ArtifactSqliteOptions):Promise<Snapshot>};
const ports=owner as unknown as Ports;
function snapshot():Snapshot{
 const value=structuredClone(fixture.snapshot);
 return{...value,referencesByModelDefinitionId:Object.fromEntries(Object.entries(value.referencesByModelDefinitionId).map(([key,rows])=>[key,rows.map(row=>({...row,origin:row.origin.map(binary64)as Reference["origin"],orientation:row.orientation===null?null:row.orientation.map(binary64)as Reference["orientation"],scale:row.scale===null?null:binary64(row.scale),widthWorld:binary64(row.widthWorld),opacity:row.opacity===null?null:binary64(row.opacity)}))]))};
}
const bytes=async(value:Snapshot)=>exportSqliteDatabase(await ports.cadSnapshotToSqliteDatabase(value));
const restore=async(value:Uint8Array,options:ArtifactSqliteOptions={})=>ports.cadSnapshotFromSqliteDatabase(await importSqliteDatabase(value),options);
test("CAD independent neutral schema admits every persisted domain",()=>{const validator=new Ajv({strict:false,validateFormats:false}).addSchema([ioSchema,childSchema,artifactSchema,nativeSchema]);expect(validator.validate(lawsSchema,fixture)).toBe(true)});
test("CAD independent SQLite declares six complete domain tables without implicit indices",async()=>{const db=new Database(":memory:");try{db.run(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());expect(db.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:6});expect(db.query("PRAGMA table_info(cad_reference)").all().length).toBe(39);expect(db.query("SELECT name FROM sqlite_schema WHERE type='index'").all()).toEqual([])}finally{db.close()}});
test("CAD document parser preserves independent local and target child identities",()=>{expect(owner.parseCadSnapshot(fixture.snapshot) as unknown).toEqual(fixture.snapshot)});
test("CAD public persisted facade exposes both semantic directions",()=>{expect(Object.hasOwn(owner,"cadSnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(owner,"cadSnapshotFromSqliteDatabase")).toBe(true)});
test("CAD complete independently queryable files preserve all eighteen rows and literal duplicates",async()=>{const expected=snapshot(),data=await bytes(expected),db=Database.deserialize(data,{safeIntegers:true});try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);for(const[name,count]of Object.entries(fixture.tableRows))expect(db.query('SELECT COUNT(*) AS n FROM "'+name+'"').get()).toEqual({n:BigInt(count)});expect(db.query("SELECT child_id,artifact_id FROM cad_model_child WHERE slot='shapeModel'").get()).toEqual({child_id:"shape local\0",artifact_id:"target shape"});expect(db.query("SELECT node_id FROM cad_node ORDER BY ordinal").all()).toEqual([{node_id:""},{node_id:""},{node_id:"node"}]);expect(await restore(data)).toEqual(expected)}finally{db.close()}});
for(const hex of fixture.binary64Words)test("CAD all reference binary64 words "+hex,async()=>{const expected=snapshot(),word={bits:BigInt("0x"+hex)};for(const rows of Object.values(expected.referencesByModelDefinitionId))for(const row of rows){row.origin=[word,word,word];row.orientation=[word,word,word,word];row.scale=word;row.widthWorld=word;row.opacity=word}const data=await bytes(expected),db=Database.deserialize(data,{safeIntegers:true});try{const buffer=Buffer.alloc(8);buffer.writeBigUInt64BE(word.bits);const value=buffer.readDoubleBE(),kind=Number.isNaN(value)?"nan":value===Infinity?"positiveInfinity":value===-Infinity?"negativeInfinity":"finite";expect(db.query("SELECT origin_x_bits,origin_x_class,orientation_x_bits,scale_bits,opacity_bits FROM cad_reference ORDER BY id").all()).toEqual(Array.from({length:4},()=>({origin_x_bits:BigInt.asIntN(64,word.bits),origin_x_class:kind,orientation_x_bits:BigInt.asIntN(64,word.bits),scale_bits:BigInt.asIntN(64,word.bits),opacity_bits:BigInt.asIntN(64,word.bits)})));expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected)}finally{db.close()}});
test("CAD empty children, empty literal map groups and absent optional numerics are distinct",async()=>{const expected=snapshot();delete expected.shapeModel;delete expected.buildingModel;delete expected.energyModel;delete expected.structureClassicModel;expected.drawings=[];expected.nodes=[];expected.referencesByModelDefinitionId=Object.fromEntries([["",[]],["__proto__",[]]]);expect(await restore(await bytes(expected))).toEqual(expected);expected.referencesByModelDefinitionId={};expect(await restore(await bytes(expected))).toEqual(expected)});
test("CAD independent surrogate renumbering and domain edits retain relationships",async()=>{const expected=snapshot(),db=Database.deserialize(await bytes(expected));try{for(const table of["model_child","drawing_child","node"])db.run("UPDATE cad_"+table+" SET id=id+100");db.run("UPDATE cad_reference_group SET id=id+100");db.run("UPDATE cad_reference SET group_id=group_id+100,id=id+100,source_url='edited 世界'");for(const rows of Object.values(expected.referencesByModelDefinitionId))for(const row of rows)row.sourceUrl="edited 世界";expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected)}finally{db.close()}});
for(const[index,sql]of fixture.malformedSql.entries())test("CAD independently malformed semantic edit refuses "+index,async()=>{const db=Database.deserialize(await bytes(snapshot()));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(restore(new Uint8Array(db.serialize()))).rejects.toThrow()}finally{db.close()}});
test("CAD exact eighteen-row admission passes and one below is refused",async()=>{const expected=snapshot(),database=await ports.cadSnapshotToSqliteDatabase(expected);expect(database.tables.reduce((n,t)=>n+t.rows.length,0)).toBe(18);await expect(ports.cadSnapshotToSqliteDatabase(expected,{maxRows:18})).resolves.toBeDefined();await expect(ports.cadSnapshotToSqliteDatabase(expected,{maxRows:17})).rejects.toThrow();await expect(ports.cadSnapshotFromSqliteDatabase(database,{maxRows:17})).rejects.toThrow()});
for(const phase of["projectSnapshot","reconstructSnapshot"]as const)test("CAD interior ordered-reference cancellation in "+phase,async()=>{const expected=snapshot(),row=expected.referencesByModelDefinitionId[""]![0]!;expected.referencesByModelDefinitionId[""]=Array.from({length:fixture.control.collectionLength},()=>structuredClone(row));const database=await ports.cadSnapshotToSqliteDatabase(expected),controller=new AbortController();let reached=false;const options:ArtifactSqliteOptions={signal:controller.signal,onProgress:event=>{if(event.phase===phase&&event.completed>=fixture.control.cancelAt&&event.completed<event.total){reached=true;controller.abort()}}};await expect(phase==="projectSnapshot"?ports.cadSnapshotToSqliteDatabase(expected,options):ports.cadSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);const canceled=new AbortController();canceled.abort();await expect(phase==="projectSnapshot"?ports.cadSnapshotToSqliteDatabase(snapshot(),{signal:canceled.signal}):ports.cadSnapshotFromSqliteDatabase(database,{signal:canceled.signal})).rejects.toHaveProperty("kind","canceled")});
test("CAD lookup workspace is admitted before reconstruction maps",async()=>{const expected=snapshot(),row=expected.referencesByModelDefinitionId[""]![0]!;expected.referencesByModelDefinitionId[""]=Array.from({length:fixture.control.collectionLength},()=>structuredClone(row));const database=await ports.cadSnapshotToSqliteDatabase(expected);expect(database.tables.reduce((n,t)=>n+t.rows.length,0)*512).toBeGreaterThan(fixture.control.maxOwnedBytes);await expect(ports.cadSnapshotFromSqliteDatabase(database,{maxValueBytes:fixture.control.maxOwnedBytes})).rejects.toThrow()});
test("CAD cumulative value and physical file bounds refuse oversized ownership",async()=>{const expected=snapshot(),database=await ports.cadSnapshotToSqliteDatabase(expected);await expect(exportSqliteDatabase(database,{maxFileBytes:fixture.control.tinyFileBytes})).rejects.toThrow();await expect(ports.cadSnapshotFromSqliteDatabase(database,{maxValueBytes:1})).rejects.toThrow();expected.referencesByModelDefinitionId[""]![0]!.sourceUrl="😀".repeat(fixture.control.largeCharacters);await expect(ports.cadSnapshotToSqliteDatabase(expected,{maxValueBytes:fixture.control.maxOwnedBytes})).rejects.toThrow()});
test("CAD independently measured semantic scalar grant passes exactly and refuses one below",async()=>{
 const expected=snapshot(),database=await ports.cadSnapshotToSqliteDatabase(expected),db=Database.deserialize(await exportSqliteDatabase(database),{safeIntegers:true});let semanticBytes=0;
 try{for(const table of database.tables){const columns=db.query('PRAGMA table_info("'+table.name+'")').all() as {name:string}[];for(const row of db.query('SELECT * FROM "'+table.name+'"').all() as Record<string,unknown>[]){for(const column of columns){const value=row[column.name];semanticBytes+=value===null?0:typeof value==="string"?Buffer.byteLength(value,"utf8"):8}}}}finally{db.close()}
 expect(semanticBytes).toBeGreaterThan(0);expect(semanticBytes).toBeLessThan(database.tables.reduce((sum,table)=>sum+table.rows.length,0)*512);
 await expect(ports.cadSnapshotToSqliteDatabase(expected,{maxValueBytes:semanticBytes})).resolves.toEqual(database);
 await expect(ports.cadSnapshotFromSqliteDatabase(database,{maxValueBytes:semanticBytes})).resolves.toEqual(expected);
 await expect(ports.cadSnapshotFromSqliteDatabase(database,{maxValueBytes:semanticBytes-1})).rejects.toHaveProperty("kind","ownershipLimit");
});

for(const phase of fixture.ownership.phases)test("CAD captures caller controls before complete "+phase,async()=>{
 const expected=snapshot(),database=await ports.cadSnapshotToSqliteDatabase(expected);let changed=false;
 const options={maxColumns:1024,maxValueBytes:1_000_000,onProgress:()=>{if(!changed){changed=true;options.maxColumns=1;options.maxValueBytes=0}}};
 await expect(phase==="projectSnapshot"?ports.cadSnapshotToSqliteDatabase(expected,options):ports.cadSnapshotFromSqliteDatabase(database,options)).resolves.toEqual(phase==="projectSnapshot"?database:expected);expect(changed).toBe(true);
});
for(const phase of fixture.ownership.phases)test("CAD neutral typed interior cancellation retains phase "+phase,async()=>{
 const expected=snapshot(),row=expected.referencesByModelDefinitionId[""]![0]!;expected.referencesByModelDefinitionId[""]=Array.from({length:fixture.control.collectionLength},()=>structuredClone(row));const database=await ports.cadSnapshotToSqliteDatabase(expected),controller=new AbortController();let reached=false;
 const options:ArtifactSqliteOptions={signal:controller.signal,onProgress:event=>{if(event.phase===phase&&event.completed>=fixture.ownership.checkpointRows&&event.completed<event.total){reached=true;controller.abort()}}};
 await expect(phase==="projectSnapshot"?ports.cadSnapshotToSqliteDatabase(expected,options):ports.cadSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);
});

test("CAD independently edited owned domain corpus preserves typed refusal authority",async()=>{
 const original=await bytes(snapshot());
 for(const sql of fixture.malformedSql){const db=Database.deserialize(original);try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(restore(new Uint8Array(db.serialize()))).rejects.toHaveProperty("kind",fixture.ownership.malformedRefusal)}finally{db.close()}}
});

test("CAD literal reference-index contract is closed and independently orders UTF-8 keys",()=>{
 const validate=new Ajv({strict:true}).compile(lawsSchema.properties.referenceIndex);
 expect(validate(fixture.referenceIndex)).toBe(true);
 expect(validate({...fixture.referenceIndex,normalizeCase:true})).toBe(false);
 expect(validate({...fixture.referenceIndex,keyComparison:"utf16Units"})).toBe(false);
 const db=new Database(":memory:");try{db.run("CREATE TABLE literal_key(value TEXT NOT NULL)");for(const key of fixture.referenceIndex.keys)db.run("INSERT INTO literal_key VALUES(?)",[key]);expect(db.query("SELECT value FROM literal_key ORDER BY CAST(value AS BLOB)").all()).toEqual(fixture.referenceIndex.orderedKeys.map(value=>({value})))}finally{db.close()}
});
test("CAD actual parent preserves numeric-looking NUL astral and prototype literal groups",async()=>{
 const expected=snapshot(),reference=expected.referencesByModelDefinitionId[""]![0]!;
 expected.referencesByModelDefinitionId=Object.fromEntries(fixture.referenceIndex.keys.map(key=>[key,key==="__proto__"?[]:[{...structuredClone(reference),id:key,sourceUrl:key}]]));
 const data=await bytes(expected),db=Database.deserialize(data);try{
  expect(db.query("SELECT model_definition_id AS value FROM cad_reference_group ORDER BY CAST(model_definition_id AS BLOB)").all()).toEqual(fixture.referenceIndex.orderedKeys.map(value=>({value})));
  expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected);
  db.run("UPDATE cad_reference_group SET id=id+200");db.run("UPDATE cad_reference SET group_id=group_id+200,id=id+200");
  expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected);
 }finally{db.close()}
});

test("CAD intrinsic geometry media preserves the declared file payload through independent OBJ parsing",async()=>{
 const policy=(fixture as unknown as {intrinsicGeometry?:unknown}).intrinsicGeometry;
 expect(policy).toEqual({accepted:["structuredText","intrinsicText","intrinsicBytes"],refused:["intrinsicNull","addressedBinary"],childPublication:"unavailable",positionCount:6,bounds:[[0,0,0],[3,3,2]]});
 const {OBJLoader}=await import("three/examples/jsm/loaders/OBJLoader.js");
 const text=await Bun.file(new URL("../../../../../../../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧫️fixtures/📦️set-object-applied/⬅️before.obj",import.meta.url)).text();
 const bytes=Buffer.from(text,"utf8");
 expect(new TextDecoder("utf-8",{fatal:true}).decode(bytes)).toBe(text);
 const positions:number[]=[];
 const parsed=new OBJLoader().parse(text);
 parsed.traverse(node=>{const geometry=(node as unknown as {geometry?:{getAttribute(name:string):{array:ArrayLike<number>}}}).geometry;if(geometry)positions.push(...Array.from(geometry.getAttribute("position").array));});
 expect(positions.length/3).toBe(6);
 expect([0,1,2].map(axis=>[Math.min(...positions.filter((_,index)=>index%3===axis)),Math.max(...positions.filter((_,index)=>index%3===axis))])).toEqual([[0,3],[0,3],[0,2]]);
});
