/** 🧪️ Wires intrinsic ownership agrees with independent SQLite integer and IEEE queries. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import{exportSqliteDatabase,importSqliteDatabase}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type{WiresValue}from"../../../🌱️value/🟦️.ts";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json" with {type:"json"};
import {parseWiresSnapshot} from "../../🟦️.ts";
const schema=await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text();
function snapshot():import("../../🟦️.ts").WiresSnapshot{return{content:fixture.content,wiresFixture:{kind:"object",members:[
 {name:fixture.duplicateNames[0]!,value:{kind:"unsigned",value:BigInt(fixture.unsigned[2]!)}},
 {name:fixture.duplicateNames[1]!,value:{kind:"signed",value:BigInt(fixture.signed[0]!)}},
 {name:fixture.duplicateNames[2]!,value:{kind:"float",value:{bits:BigInt("0x"+fixture.floatWords[5]!)}}},
 {name:fixture.duplicateNames[3]!,value:{kind:"bytes",value:Uint8Array.from(fixture.byteValues)}}]},
 meta:{kind:"array",items:[{kind:"null"},{kind:"boolean",value:true},{kind:"text",value:fixture.texts[1]!}]}};}
test("Wires canonical parser preserves complete native intrinsic states and independent child identity",()=>{
 const expected=snapshot(),independent=new Database(":memory:");try{
  independent.exec(schema);
  independent.exec("INSERT INTO wires_document VALUES(1);INSERT INTO wires_value VALUES(1,'unsigned');INSERT INTO wires_unsigned VALUES(1,4294967295,4294967295);INSERT INTO wires_value VALUES(2,'float');INSERT INTO wires_float VALUES(2,NULL,9221120237041095220,'nan');");
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  const unsigned=independent.query("SELECT high,low FROM wires_unsigned").get()as{high:number;low:number};
  expect((BigInt(unsigned.high)<<32n)|BigInt(unsigned.low)).toBe(BigInt(fixture.unsigned[2]!));
  expect(parseWiresSnapshot(expected)).toEqual(expected);
 }finally{independent.close();}
});
test("Wires explicit intrinsic entities survive independent SQLite reserialization, aliases and edits",async()=>{
 const own=await import("../../🪶️sqlite/🟦️.ts"),expected=snapshot(),database=await own.wiresSnapshotToSqliteDatabase(expected);
 expect(database.tables.map(table=>table.sql+";\n").join("")).toBe(schema);
 const independent=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(independent.query("SELECT name FROM wires_object_member ORDER BY ordinal").all()).toEqual(fixture.duplicateNames.map(name=>({name})));
  const statement=independent.query("SELECT value_ieee754_bits AS bits,value_numeric_class AS kind,value FROM wires_float")as unknown as{safeIntegers(enabled:boolean):void;get():{bits:bigint;kind:string;value:null}};statement.safeIntegers(true);const word=statement.get();
  expect(word).toEqual({bits:BigInt("0x"+fixture.floatWords[5]!),kind:"nan",value:null});
  independent.exec("BEGIN;PRAGMA defer_foreign_keys=ON;UPDATE wires_document SET id=-17;UPDATE wires_fixture_root SET id=-17,value_id=-value_id-100;UPDATE wires_meta_root SET id=-17,value_id=-value_id-100;UPDATE wires_content_child SET id=-17;UPDATE wires_value SET id=-id-100;");
  for(const table of["wires_null","wires_boolean","wires_unsigned","wires_signed","wires_float","wires_text","wires_bytes","wires_array","wires_object"])independent.exec("UPDATE "+table+" SET id=-id-100");
  independent.exec("UPDATE wires_octet SET id=-id-100,bytes_id=-bytes_id-100;UPDATE wires_array_element SET id=-id-100,array_id=-array_id-100,value_id=-value_id-100;UPDATE wires_object_member SET id=-id-100,object_id=-object_id-100,value_id=-value_id-100;COMMIT;");
  const renamed=await importSqliteDatabase(independent.serialize());expect(await own.wiresSnapshotFromSqliteDatabase(renamed)).toEqual(expected);expect(await own.validateWiresSnapshotSqliteDialect(expected,fixture.dialect,renamed)).toEqual([]);
  independent.run("UPDATE wires_text SET value=?",[fixture.editedText]);const edited=await own.wiresSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()));expect(edited.meta).toEqual({kind:"array",items:[{kind:"null"},{kind:"boolean",value:true},{kind:"text",value:fixture.editedText}]});
  await expect(own.validateWiresSnapshotSqliteDialect(expected,fixture.dialect,await importSqliteDatabase(independent.serialize()))).rejects.toThrow();
 }finally{independent.close();}
});
test("Wires consumes all reachable intrinsic ownership and refuses independent SQL cycles",async()=>{
 const own=await import("../../🪶️sqlite/🟦️.ts"),database=await own.wiresSnapshotToSqliteDatabase(snapshot());
 const independent=Database.deserialize(await exportSqliteDatabase(database));try{
  independent.exec("INSERT INTO wires_value VALUES(5000,'array');INSERT INTO wires_array VALUES(5000);INSERT INTO wires_array_element VALUES(5000,5000,0,5000);");
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
  await expect(own.wiresSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()))).rejects.toThrow();
 }finally{independent.close();}
});
test("Wires exact coordinates and controlled materialization retain deep full native values",async()=>{
 const own=await import("../../🪶️sqlite/🟦️.ts");let deep:WiresValue={kind:"float",value:{bits:BigInt("0x"+fixture.floatWords[6]!)}};for(let i=0;i<fixture.deepLevels;i++)deep={kind:"array",items:[deep]};
 const expected={...snapshot(),meta:deep},database=await own.wiresSnapshotToSqliteDatabase(expected);const restored=await own.wiresSnapshotFromSqliteDatabase(database);let value=restored.meta;for(let i=0;i<fixture.deepLevels;i++){if(value.kind!=="array")throw Error("deep value differs");value=value.items[0]!;}expect(value).toEqual({kind:"float",value:{bits:BigInt("0x"+fixture.floatWords[6]!)}});
 for(const dialect of fixture.invalidSqliteDialects)await expect(own.validateWiresSnapshotSqliteDialect(expected,dialect,database)).rejects.toThrow();
 await expect(own.wiresSnapshotToSqliteDatabase(expected,{maxRows:30})).rejects.toThrow();await expect(own.wiresSnapshotToSqliteDatabase(expected,{maxValueBytes:30})).rejects.toThrow();await expect(own.wiresSnapshotFromSqliteDatabase(database,{maxRows:30})).rejects.toThrow();await expect(own.wiresSnapshotFromSqliteDatabase(database,{maxValueBytes:30})).rejects.toThrow();
 for(const phase of["projectSnapshot","reconstructSnapshot"]as const){const abort=new AbortController();let interior=false;const options={signal:abort.signal,onProgress:(progress:import("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts").ArtifactSqliteProgress)=>{if(progress.phase===phase&&progress.completed>=256&&progress.completed<progress.total){interior=true;abort.abort();}}};await expect(phase==="projectSnapshot"?own.wiresSnapshotToSqliteDatabase(expected,options):own.wiresSnapshotFromSqliteDatabase(database,options)).rejects.toThrow();expect(interior).toBe(true);}
});
test("Wires independent IEEE queries retain every neutral binary64 class and reject mismatched companions",async()=>{
 const own=await import("../../🪶️sqlite/🟦️.ts");for(const word of fixture.floatWords){const expected={...snapshot(),meta:{kind:"float"as const,value:{bits:BigInt("0x"+word)}}},database=await own.wiresSnapshotToSqliteDatabase(expected),independent=Database.deserialize(await exportSqliteDatabase(database));try{
  const statement=independent.query("SELECT f.value_ieee754_bits AS bits FROM wires_float f JOIN wires_meta_root m ON m.value_id=f.id")as unknown as{safeIntegers(enabled:boolean):void;get():{bits:bigint}};statement.safeIntegers(true);expect(statement.get().bits).toBe(BigInt.asIntN(64,BigInt("0x"+word)));
  expect((await own.wiresSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()))).meta).toEqual(expected.meta);
  independent.exec("UPDATE wires_float SET value_numeric_class=CASE WHEN value_numeric_class='nan' THEN 'finite' ELSE 'nan' END");await expect(own.wiresSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()))).rejects.toThrow();
 }finally{independent.close();}}
});
test("Wires borrowed Unicode admission rejects invalid scalars and cancels a long semantic field internally",async()=>{
 const own=await import("../../🪶️sqlite/🟦️.ts");for(const value of["\ud800","\udfff"])await expect(own.wiresSnapshotToSqliteDatabase({...snapshot(),meta:{kind:"text",value}})).rejects.toThrow();
 const expected={...snapshot(),meta:{kind:"text"as const,value:"世界 🪐".repeat(30000)}},database=await own.wiresSnapshotToSqliteDatabase(expected);expect((await own.wiresSnapshotFromSqliteDatabase(database)).meta).toEqual(expected.meta);
 for(const phase of["projectSnapshot","reconstructSnapshot"]as const){const abort=new AbortController();let interior=false;const options={signal:abort.signal,onProgress:(progress:import("../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts").ArtifactSqliteProgress)=>{if(progress.phase===phase&&progress.total===expected.meta.value.length&&progress.completed>=65536&&progress.completed<progress.total){interior=true;abort.abort();}}};await expect(phase==="projectSnapshot"?own.wiresSnapshotToSqliteDatabase(expected,options):own.wiresSnapshotFromSqliteDatabase(database,options)).rejects.toThrow();expect(interior).toBe(true);}
});
test("Wires admits reconstruction lookup ownership before materializing independently valid SQL rows",async()=>{
 const own=await import("../../🪶️sqlite/🟦️.ts"),expected={...snapshot(),meta:{kind:"array"as const,items:Array.from({length:fixture.metadataItems},()=>({kind:"null"as const}))}},database=await own.wiresSnapshotToSqliteDatabase(expected),bytes=await exportSqliteDatabase(database),independent=Database.deserialize(bytes);try{
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("SELECT COUNT(*) AS n FROM wires_array_element").get()).toEqual({n:fixture.metadataItems});
  const physical=await importSqliteDatabase(independent.serialize(),{maxValueBytes:fixture.metadataValueBudget});expect(physical.tables.reduce((n,table)=>n+table.rows.length,0)).toBeGreaterThan(fixture.metadataItems*3);
  await expect(own.wiresSnapshotFromSqliteDatabase(physical,{maxValueBytes:fixture.metadataValueBudget})).rejects.toThrow();
 }finally{independent.close();}
});
