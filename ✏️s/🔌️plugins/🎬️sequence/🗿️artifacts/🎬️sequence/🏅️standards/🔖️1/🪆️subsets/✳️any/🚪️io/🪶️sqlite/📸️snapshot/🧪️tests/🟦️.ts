import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";

import nativeSchema from "../../../../🧬️schema/📸️snapshot/🔣️.json";
import childSchema from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json";
import ioSchema from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json";
import references from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🧫️fixtures/🔣️.json";
import * as sqliteOwner from "../🟦️.ts";
import referenceSchema from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import * as owner from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type {ArtifactSqliteOptions} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
const own=sqliteOwner as unknown as {SEQUENCE_SQLITE_SCHEMA:string;sequenceSnapshotToSqliteDatabase:(snapshot:owner.SequenceSnapshot,options?:ArtifactSqliteOptions)=>Promise<SqliteDatabase>;sequenceSnapshotFromSqliteDatabase:(database:SqliteDatabase,options?:ArtifactSqliteOptions)=>Promise<owner.SequenceSnapshot>};
const snapshot=():owner.SequenceSnapshot=>structuredClone(fixture.snapshot);
test("Sequence neutral persisted-reference laws agree with independent JSON schema",()=>{
 const ajv=new Ajv({strict:false,validateFormats:false}).addSchema(referenceSchema).addSchema(ioSchema).addSchema(childSchema).addSchema(nativeSchema);
 expect(ajv.validate(nativeSchema,{...fixture.snapshot,steps:[]})).toBe(false);expect(ajv.validate(nativeSchema,{...fixture.snapshot,content:{...fixture.snapshot.content,localOwner:{}}})).toBe(false);
});
test("Sequence snapshot facade exposes its complete literal relational owner",()=>{expect(Object.hasOwn(sqliteOwner,"sequenceSnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(sqliteOwner,"sequenceSnapshotFromSqliteDatabase")).toBe(true)});
test("Sequence handcrafted SQL independently stores every reference component",async()=>{
 const sql=await Bun.file(new URL("../🗄️.sql",import.meta.url)).text(),db=new Database(":memory:",{safeIntegers:true}),expected=snapshot(),child=expected.content,target=child.target;
 try{db.run(sql);db.run("INSERT INTO sequence_document VALUES (1,?)",[expected.schema]);db.run("INSERT INTO sequence_content VALUES (1,1,?,?,?,?,?)",[child.childId,target.artifactId,target.dialect.artifactKind,target.dialect.standard,target.dialect.subset]);expect(db.query("SELECT schema,child_id,artifact_id,artifact_kind,standard,subset FROM sequence_document JOIN sequence_content ON sequence_document.id=sequence_content.document_id").get()).toEqual({schema:expected.schema,child_id:child.childId,artifact_id:target.artifactId,artifact_kind:target.dialect.artifactKind,standard:target.dialect.standard,subset:target.dialect.subset});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([])}finally{db.close()}
});
test("Sequence queryable SQLite files retain complete Unicode NUL and child identity",async()=>{
 const expected=snapshot(),database=await own.sequenceSnapshotToSqliteDatabase(expected);expect(database.tables.length).toBe(2);expect(own.SEQUENCE_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());
 const db=Database.deserialize(await exportSqliteDatabase(database),{safeIntegers:true});try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("SELECT child_id,artifact_id,artifact_kind,standard,subset FROM sequence_content").get()).toEqual({child_id:expected.content.childId,artifact_id:expected.content.target.artifactId,artifact_kind:expected.content.target.dialect.artifactKind,standard:expected.content.target.dialect.standard,subset:expected.content.target.dialect.subset});expect(await own.sequenceSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected)}finally{db.close()}
});
test("Sequence independent semantic edits and child surrogate renumbering reconstruct",async()=>{
 const expected=snapshot(),db=Database.deserialize(await exportSqliteDatabase(await own.sequenceSnapshotToSqliteDatabase(expected)),{safeIntegers:true});
 try{db.run("UPDATE sequence_document SET schema=?",["edited 世界"]);db.run("UPDATE sequence_content SET id=99,child_id=?,artifact_id=?,artifact_kind=?,standard=?,subset=?",["new child","new artifact","s.stdio.json","rfc8259","*"]);expected.schema="edited 世界";expected.content={childId:"new child",target:{artifactId:"new artifact",dialect:{artifactKind:"s.stdio.json",standard:"rfc8259",subset:"*"}}};expect(await own.sequenceSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected)}finally{db.close()}
});
test("Sequence every native string domain includes empty reference components",async()=>{
 const expected:owner.SequenceSnapshot={schema:"",content:{childId:"",target:{artifactId:"",dialect:{artifactKind:"",standard:"",subset:""}}}};
 expect(await own.sequenceSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(await own.sequenceSnapshotToSqliteDatabase(expected))))).toEqual(expected);
});
test("Sequence independently queryable references retain all shared delimiter and partial-empty vectors",async()=>{
 for(const reference of references.references){const expected=snapshot();expected.content.target=structuredClone(reference);const db=Database.deserialize(await exportSqliteDatabase(await own.sequenceSnapshotToSqliteDatabase(expected)));try{expect(db.query("SELECT artifact_id,artifact_kind,standard,subset FROM sequence_content").get()).toEqual({artifact_id:reference.artifactId,artifact_kind:reference.dialect.artifactKind,standard:reference.dialect.standard,subset:reference.dialect.subset});expect(await own.sequenceSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected)}finally{db.close()}}
});
test("Sequence malformed missing duplicate parents identities and column types refuse",async()=>{
 const bytes=await exportSqliteDatabase(await own.sequenceSnapshotToSqliteDatabase(snapshot()));
 for(const sql of fixture.malformedSql){const db=Database.deserialize(bytes);try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(own.sequenceSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow()}finally{db.close()}}
 const valid=await own.sequenceSnapshotToSqliteDatabase(snapshot()),wrong:SqliteDatabase={...valid,tables:valid.tables.map((table,index)=>index===0?{...table,rows:table.rows.map(row=>({...row,values:[...row.values,1n]}))}:table)};await expect(own.sequenceSnapshotFromSqliteDatabase(wrong)).rejects.toThrow();
});
test("Sequence caller row value and physical limits constrain real ownership",async()=>{
 const expected=snapshot();await expect(own.sequenceSnapshotToSqliteDatabase(expected,{maxRows:1})).rejects.toThrow();const database=await own.sequenceSnapshotToSqliteDatabase(expected);
 await expect(own.sequenceSnapshotFromSqliteDatabase(database,{maxValueBytes:1})).rejects.toThrow();await expect(exportSqliteDatabase(database,{maxFileBytes:fixture.control.tinyFileBytes})).rejects.toThrow();
 expected.content.childId="😀".repeat(fixture.control.largeTextBytes);await expect(own.sequenceSnapshotToSqliteDatabase(expected,{maxValueBytes:fixture.control.maxOwnedBytes})).rejects.toThrow();
});
test("Sequence semantic traversal cancels initially and within large persisted text",async()=>{
 const expected=snapshot(),initial=new AbortController();initial.abort();await expect(own.sequenceSnapshotToSqliteDatabase(expected,{signal:initial.signal})).rejects.toHaveProperty("kind","canceled");const original=await own.sequenceSnapshotToSqliteDatabase(expected);await expect(own.sequenceSnapshotFromSqliteDatabase(original,{signal:initial.signal})).rejects.toHaveProperty("kind","canceled");
 expected.content.childId="😀".repeat(fixture.control.largeTextBytes);const database=await own.sequenceSnapshotToSqliteDatabase(expected);
 for(const phase of["projectSnapshot","reconstructSnapshot"]as const){const c=new AbortController();let reached=false;const options:ArtifactSqliteOptions={signal:c.signal,onProgress:e=>{if(e.phase===phase&&e.completed>0&&e.total===0){reached=true;c.abort()}}};await expect(phase==="projectSnapshot"?own.sequenceSnapshotToSqliteDatabase(expected,options):own.sequenceSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true)}
});


import semanticCells from "../../../../🧬️schema/📸️snapshot/🧫️fixtures/📏️semantic-cells/🔣️.json";

import Ajv2020 from "ajv/dist/2020";
test("Sequence complete independent SQL semantic cell extent",async()=>{
 expect(semanticCells).toEqual({"schemaVersion":1,"tables":["sequence_document","sequence_content"],"cases":[{"name":"complete-owner","field":null,"text":null,"bytes":97,"rows":2},{"name":"empty-all","field":"all","text":"","bytes":24,"rows":2},{"name":"unicode-all","field":"all","text":"世界\u0000😀; id!kind@standard/subset%","bytes":246,"rows":2},{"name":"long-schema","field":"schema","text":"😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000","bytes":217,"rows":2},{"name":"long-childId","field":"childId","text":"😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000","bytes":211,"rows":2},{"name":"long-artifactId","field":"artifactId","text":"😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000","bytes":217,"rows":2},{"name":"long-artifactKind","field":"artifactKind","text":"😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000","bytes":220,"rows":2},{"name":"long-standard","field":"standard","text":"😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000","bytes":231,"rows":2},{"name":"long-subset","field":"subset","text":"😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000😀界\u0000","bytes":229,"rows":2}]});
 for(const c of semanticCells.cases){const s=snapshot();const set=(field:string,value:string)=>{switch(field){case "schema":s.schema=value;break;case "childId":s.content.childId=value;break;case "artifactId":s.content.target.artifactId=value;break;case "artifactKind":s.content.target.dialect.artifactKind=value;break;case "standard":s.content.target.dialect.standard=value;break;case "subset":s.content.target.dialect.subset=value;break;default:throw Error("Unknown semantic field")}};if(c.field!==null)for(const field of c.field==="all"?["schema","childId","artifactId","artifactKind","standard","subset"]:[c.field])set(field,c.text!);
  const d=await own.sequenceSnapshotToSqliteDatabase(s),oracle=Database.deserialize(await exportSqliteDatabase(d));try{expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(d.tables.map(t=>t.name)).toEqual(semanticCells.tables);let bytes=0,rows=0;for(const t of d.tables){const actual=oracle.query<Record<string,null|string|number|bigint|Uint8Array>,[]>('SELECT * FROM "'+t.name+'"').all();rows+=actual.length;for(const row of actual)for(const value of Object.values(row))bytes+=value===null?0:typeof value==="string"?Buffer.byteLength(value,"utf8"):value instanceof Uint8Array?value.length:8;}expect(rows).toBe(c.rows);expect(bytes).toBe(c.bytes);expect(await own.sequenceSnapshotToSqliteDatabase(s,{maxValueBytes:c.bytes})).toEqual(d);expect(await own.sequenceSnapshotFromSqliteDatabase(d,{maxValueBytes:c.bytes})).toEqual(s);await expect(own.sequenceSnapshotToSqliteDatabase(s,{maxValueBytes:c.bytes-1})).rejects.toThrow();await expect(own.sequenceSnapshotFromSqliteDatabase(d,{maxValueBytes:c.bytes-1})).rejects.toThrow();}finally{oracle.close()}
 }
});
