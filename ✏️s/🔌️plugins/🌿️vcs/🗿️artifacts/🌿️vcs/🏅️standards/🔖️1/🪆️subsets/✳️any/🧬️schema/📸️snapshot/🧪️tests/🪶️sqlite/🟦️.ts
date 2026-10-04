import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import lawSchema from "../../🧫️fixtures/🪶️sqlite/🧬️schema/🔣️.json";
import nativeSchema from "../../🔣️.json";
import * as owner from "../../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type {ArtifactSqliteOptions} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
type Snapshot=Omit<owner.VcsSnapshot,"counter">&{counter:bigint};
const own=owner as unknown as {VCS_SQLITE_SCHEMA:string;vcsSnapshotToSqliteDatabase:(snapshot:Snapshot,options?:ArtifactSqliteOptions)=>Promise<SqliteDatabase>;vcsSnapshotFromSqliteDatabase:(database:SqliteDatabase,options?:ArtifactSqliteOptions)=>Promise<Snapshot>};
const snapshot=():Snapshot=>({...fixture.snapshot,counter:BigInt(fixture.snapshot.counter),tags:[...fixture.snapshot.tags]});
test("VCS complete neutral ownership laws agree with independent JSON schema",()=>{const ajv=new Ajv({strict:false,validateFormats:false}).addSchema(nativeSchema);expect(ajv.validate(lawSchema,fixture)).toBe(true);expect(ajv.validate(nativeSchema,{...fixture.snapshot,unexpected:true})).toBe(false);expect(ajv.validate(nativeSchema,{...fixture.snapshot,tags:[1]})).toBe(false)});
test("VCS facade declares the full semantic SQLite owner",()=>{expect(Object.hasOwn(owner,"vcsSnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(owner,"vcsSnapshotFromSqliteDatabase")).toBe(true)});
test("VCS handwritten schema is independently queryable without native codec knowledge",async()=>{
 const sql=await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text(),db=new Database(":memory:",{safeIntegers:true});
 try{db.run(sql);db.run("INSERT INTO vcs_document VALUES (1,?,?,?,?,?)",[fixture.snapshot.schema,fixture.snapshot.title,-9223372036854775808n,fixture.snapshot.notes,fixture.snapshot.status]);fixture.snapshot.tags.forEach((tag,index)=>db.run("INSERT INTO vcs_tag VALUES (?,?,?,?)",[BigInt(index+1),1n,BigInt(index),tag]));expect(db.query("SELECT counter FROM vcs_document").get()).toEqual({counter:-9223372036854775808n});expect(db.query("SELECT value FROM vcs_tag JOIN vcs_document ON vcs_document.id=vcs_tag.document_id ORDER BY ordinal").all()).toEqual(fixture.snapshot.tags.map(value=>({value})));expect(db.query("PRAGMA foreign_key_check").all()).toEqual([])}finally{db.close()}
});
test("VCS owned files preserve full fields, tag order, duplicates, empty tags and Unicode",async()=>{
 const expected=snapshot(),database=await own.vcsSnapshotToSqliteDatabase(expected);expect(own.VCS_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());expect(database.tables.length).toBe(2);
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

