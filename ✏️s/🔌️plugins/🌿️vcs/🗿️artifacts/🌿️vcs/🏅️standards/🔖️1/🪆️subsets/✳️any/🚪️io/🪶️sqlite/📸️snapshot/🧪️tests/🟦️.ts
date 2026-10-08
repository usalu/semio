import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";

import nativeSchema from "../../../../🧬️schema/📸️snapshot/🔣️.json";
import type {VcsSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import * as owner from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type {ArtifactSqliteOptions} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
type Snapshot=Omit<VcsSnapshot,"counter">&{counter:bigint};
const own=owner as unknown as {VCS_SQLITE_SCHEMA:string;vcsSnapshotToSqliteDatabase:(snapshot:Snapshot,options?:ArtifactSqliteOptions)=>Promise<SqliteDatabase>;vcsSnapshotFromSqliteDatabase:(database:SqliteDatabase,options?:ArtifactSqliteOptions)=>Promise<Snapshot>};
const snapshot=():Snapshot=>({...fixture.snapshot,counter:BigInt(fixture.snapshot.counter),tags:[...fixture.snapshot.tags]});
test("VCS complete neutral ownership laws agree with independent JSON schema",()=>{const ajv=new Ajv({strict:false,validateFormats:false}).addSchema(nativeSchema);expect(fixture["control"]["tagCount"]).toEqual(2048);expect(fixture["control"]["cancelAt"]).toEqual(256);expect(fixture["control"]["largeTextBytes"]).toEqual(131073);expect(fixture["control"]["maxOwnedBytes"]).toEqual(65536);expect(fixture["control"]["tinyFileBytes"]).toEqual(1024);expect(fixture["semanticCells"]["tableWidths"]).toEqual({"vcs_document":6,"vcs_tag":4});expect(fixture["semanticCells"]["fullRows"]).toEqual(7);expect(ajv.validate(nativeSchema,{...fixture.snapshot,unexpected:true})).toBe(false);expect(ajv.validate(nativeSchema,{...fixture.snapshot,tags:[1]})).toBe(false)});
test("VCS facade declares the full semantic SQLite owner",()=>{expect(Object.hasOwn(owner,"vcsSnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(owner,"vcsSnapshotFromSqliteDatabase")).toBe(true)});
test("VCS handwritten schema is independently queryable without native codec knowledge",async()=>{
 const sql=await Bun.file(new URL("../🗄️.sql",import.meta.url)).text(),db=new Database(":memory:",{safeIntegers:true});
 try{db.run(sql);db.run("INSERT INTO vcs_document VALUES (1,?,?,?,?,?)",[fixture.snapshot.schema,fixture.snapshot.title,-9223372036854775808n,fixture.snapshot.notes,fixture.snapshot.status]);fixture.snapshot.tags.forEach((tag,index)=>db.run("INSERT INTO vcs_tag VALUES (?,?,?,?)",[BigInt(index+1),1n,BigInt(index),tag]));expect(db.query("SELECT counter FROM vcs_document").get()).toEqual({counter:-9223372036854775808n});expect(db.query("SELECT value FROM vcs_tag JOIN vcs_document ON vcs_document.id=vcs_tag.document_id ORDER BY ordinal").all()).toEqual(fixture.snapshot.tags.map(value=>({value})));expect(db.query("PRAGMA foreign_key_check").all()).toEqual([])}finally{db.close()}
});
test("VCS owned files preserve full fields, tag order, duplicates, empty tags and Unicode",async()=>{
 const expected=snapshot(),database=await own.vcsSnapshotToSqliteDatabase(expected);expect(own.VCS_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());expect(database.tables.length).toBe(2);
 const db=Database.deserialize(await exportSqliteDatabase(database),{safeIntegers:true});try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT counter,title,status FROM vcs_document").get()).toEqual({counter:expected.counter,title:expected.title,status:expected.status});expect(db.query("SELECT ordinal,value FROM vcs_tag ORDER BY ordinal").all()).toEqual(expected.tags.map((value,index)=>({ordinal:BigInt(index),value})));expect(await own.vcsSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected)}finally{db.close()}
});
for(const word of fixture.signed64)test(`VCS independently stores signed64 counter ${word}`,async()=>{
 const expected={...snapshot(),counter:BigInt(word)},bytes=await exportSqliteDatabase(await own.vcsSnapshotToSqliteDatabase(expected)),db=Database.deserialize(bytes,{safeIntegers:true});
 try{expect(db.query("SELECT counter FROM vcs_document").get()).toEqual({counter:BigInt(word)});expect(await own.vcsSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected)}finally{db.close()}
});
test("VCS independent semantic edits and consistent tag surrogate renumbering reconstruct",async()=>{
 const expected=snapshot(),db=Database.deserialize(await exportSqliteDatabase(await own.vcsSnapshotToSqliteDatabase(expected)),{safeIntegers:true});
 try{db.run("UPDATE vcs_document SET counter=?,title=?,status=?",[9223372036854775807n,"Geändert 世界","any native status"]);db.run("UPDATE vcs_tag SET id=id+100,value=? WHERE ordinal=0",["edited; tag"]);expected.counter=9223372036854775807n;expected.title="Geändert 世界";expected.status="any native status";expected.tags[0]="edited; tag";expect(await own.vcsSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected)}finally{db.close()}
});
test("VCS empty tags retain the complete queryable schema",async()=>{const expected={...snapshot(),tags:[]},database=await own.vcsSnapshotToSqliteDatabase(expected);expect(database.tables.length).toBe(2);expect(await own.vcsSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(database)))).toEqual(expected)});
test("VCS refuses malformed native widths, parents, dense ordinals and exact columns",async()=>{
 const bytes=await exportSqliteDatabase(await own.vcsSnapshotToSqliteDatabase(snapshot()));
 for(const sql of fixture.malformedSql){const db=Database.deserialize(bytes,{safeIntegers:true});try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(own.vcsSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow()}finally{db.close()}}
 for(const word of fixture.invalidSigned64)await expect(own.vcsSnapshotToSqliteDatabase({...snapshot(),counter:BigInt(word)})).rejects.toThrow();
 await expect(own.vcsSnapshotToSqliteDatabase(snapshot(),{maxRows:1})).rejects.toThrow();await expect(own.vcsSnapshotFromSqliteDatabase(await own.vcsSnapshotToSqliteDatabase(snapshot()),{maxValueBytes:1})).rejects.toThrow();
});
test("VCS semantic traversals cancel initially, within tags and before large textual ownership",async()=>{
 const expected=snapshot(),initial=new AbortController();initial.abort();await expect(own.vcsSnapshotToSqliteDatabase(expected,{signal:initial.signal})).rejects.toHaveProperty("kind","canceled");const original=await own.vcsSnapshotToSqliteDatabase(expected);await expect(own.vcsSnapshotFromSqliteDatabase(original,{signal:initial.signal})).rejects.toHaveProperty("kind","canceled");
 expected.tags=Array.from({length:fixture.control.tagCount},(_,index)=>`tag ${index}`);const large=await own.vcsSnapshotToSqliteDatabase(expected);
 for(const phase of ["projectSnapshot","reconstructSnapshot"] as const){const c=new AbortController();let reached=false;const options:ArtifactSqliteOptions={signal:c.signal,onProgress:e=>{if(e.phase===phase&&e.completed>=fixture.control.cancelAt){reached=true;c.abort()}}};await expect(phase==="projectSnapshot"?own.vcsSnapshotToSqliteDatabase(expected,options):own.vcsSnapshotFromSqliteDatabase(large,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true)}
 expected.notes="😀".repeat(fixture.control.largeTextBytes);await expect(own.vcsSnapshotToSqliteDatabase(expected,{maxValueBytes:fixture.control.maxOwnedBytes})).rejects.toThrow();const c=new AbortController();let reached=false;await expect(own.vcsSnapshotToSqliteDatabase(expected,{signal:c.signal,onProgress:e=>{if(e.phase==="projectSnapshot"&&e.completed>0&&e.total===0){reached=true;c.abort()}}})).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);
});


test("VCS complete semantic cells agree with independent SQLite at exact and one-short caller limits",async()=>{
 const ajv=new Ajv({strict:false,validateFormats:false}).addSchema(nativeSchema);expect(fixture["control"]["tagCount"]).toEqual(2048);expect(fixture["control"]["cancelAt"]).toEqual(256);expect(fixture["control"]["largeTextBytes"]).toEqual(131073);expect(fixture["control"]["maxOwnedBytes"]).toEqual(65536);expect(fixture["control"]["tinyFileBytes"]).toEqual(1024);expect(fixture["semanticCells"]["tableWidths"]).toEqual({"vcs_document":6,"vcs_tag":4});expect(fixture["semanticCells"]["fullRows"]).toEqual(7);expect(fixture.semanticCells.tableWidths).toEqual({vcs_document:6,vcs_tag:4});
 for(const word of fixture.signed64){const expected={...snapshot(),counter:BigInt(word)},database=await own.vcsSnapshotToSqliteDatabase(expected),bytes=await exportSqliteDatabase(database),db=Database.deserialize(bytes,{safeIntegers:true});let cells=0,rows=0;
  try{for(const table of db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()as{name:string}[]){const name='"'+table.name.replaceAll('"','""')+'"',columns=db.query("PRAGMA table_info("+name+")").all()as{name:string}[];expect(columns.length).toBe(fixture.semanticCells.tableWidths[table.name as keyof typeof fixture.semanticCells.tableWidths]);rows+=Number((db.query("SELECT COUNT(*) count FROM "+name).get()as{count:bigint}).count);for(const column of columns){const key='"'+column.name.replaceAll('"','""')+'"';cells+=Number((db.query("SELECT COALESCE(SUM(CASE typeof("+key+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+key+" AS BLOB)) WHEN 'blob' THEN length("+key+") ELSE 0 END),0) bytes FROM "+name).get()as{bytes:bigint}).bytes);}}expect(rows).toBe(fixture.semanticCells.fullRows);}finally{db.close()}
  expect(cells).toBeGreaterThan(0);expect(await own.vcsSnapshotToSqliteDatabase(expected,{maxRows:rows,maxValueBytes:cells})).toEqual(database);await expect(own.vcsSnapshotToSqliteDatabase(expected,{maxRows:rows-1,maxValueBytes:cells})).rejects.toThrow();await expect(own.vcsSnapshotToSqliteDatabase(expected,{maxRows:rows,maxValueBytes:cells-1})).rejects.toThrow();
 }console.log("[DEBUG] VCS complete two-table independent exact semantic cell and row boundaries");
});

import NormSemanticAjv from "ajv/dist/2020.js";
import normSemanticContract from "../🧫️fixtures/🎛️semantic.json";

import {vcsSnapshotToSqliteDatabase as normSemanticProject,vcsSnapshotFromSqliteDatabase as normSemanticRestore} from "../🟦️.ts";
import {exportSqliteDatabase as normSemanticExport,importSqliteDatabase as normSemanticImport} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {independentSqliteExtent as normIndependentExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";
test("VCS complete independent cells preserve copied limits and required empty metadata",async()=>{
 expect(normSemanticContract["schemaBytes"]).toEqual(420);expect(normSemanticContract["tableWidths"]).toEqual({"vcs_document":6,"vcs_tag":4});expect(normSemanticContract["cases"]).toEqual([{"id":"full","rows":7,"valueBytes":250},{"id":"emptyTags","rows":1,"valueBytes":72}]);
 for(const item of normSemanticContract.cases){const source=snapshot();if(item.id==="emptyTags"){source.tags=[];}const database=await normSemanticProject(source),bytes=await normSemanticExport(database);expect(normIndependentExtent(bytes)).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:normSemanticContract.schemaBytes,tableWidths:normSemanticContract.tableWidths});const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:normSemanticContract.schemaBytes,maxTables:2,maxColumns:6};expect(await normSemanticProject(source,limits)).toEqual(database);expect(await normSemanticRestore(await normSemanticImport(bytes),limits)).toEqual(source);
  for(const short of[{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:1},{...limits,maxColumns:5}]){await expect(normSemanticProject(source,short)).rejects.toThrow();await expect(normSemanticRestore(database,short)).rejects.toThrow();}
 }console.log("[DEBUG] VCS complete independent SQLite cells and metadata preserve full and empty semantic limits");
});
