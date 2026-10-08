/** 🖼️ Full-width bitmap problem fixture and independent SQLite edits. */
import {fileURLToPath} from "node:url";
import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import type {BitmapSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {WFC_BITMAP_SQLITE_SCHEMA,bitmapSnapshotToSqliteDatabase,bitmapSnapshotFromSqliteDatabase} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {decodeBase64,encodeBase64,bitmapJsonBind,bitmapJsonValue} from "../../../📝️text/📸️snapshot/🟦️.ts";
const input:BitmapSnapshot={schema:fixture.schema,seed:BigInt(fixture.seed),input:{...fixture.input,pixels:decodeBase64(fixture.input.pixels)},output:fixture.output,model:fixture.model,pinned:fixture.pinned};
test("bitmap sparse owned native state has literal independent SQL fields",async()=>{
 const state=fixture.ownedSparseState;const sparse:BitmapSnapshot={...input,schema:state.schema,input:{...input.input,pixels:Uint8Array.from(state.pixels),palette:state.palette},model:{...input.model,ground:state.ground},pinned:state.pinned};
 const database=Database.deserialize(await exportSqliteDatabase(await bitmapSnapshotToSqliteDatabase(sparse)));
 try{expect(database.query("SELECT schema_id,palette_indices,ground_palette_index FROM wfc_bitmap_document JOIN wfc_bitmap_input ON wfc_bitmap_document.id=wfc_bitmap_input.id JOIN wfc_bitmap_model ON wfc_bitmap_document.id=wfc_bitmap_model.id").get()).toEqual({schema_id:state.schema,palette_indices:Uint8Array.from(state.pixels),ground_palette_index:null});expect(database.query("SELECT count(*) AS total FROM wfc_bitmap_palette").get()).toEqual({total:0});expect(database.query("SELECT count(*) AS total FROM wfc_bitmap_pin").get()).toEqual({total:0});expect(await bitmapSnapshotFromSqliteDatabase(await importSqliteDatabase(database.serialize()))).toEqual(sparse);}finally{database.close()}
});
test("bitmap all native widths and independent relational query/edit",async()=>{expect(WFC_BITMAP_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());const original=await bitmapSnapshotToSqliteDatabase(input);const db=Database.deserialize(await exportSqliteDatabase(original));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT red,palette_indices FROM wfc_bitmap_palette JOIN wfc_bitmap_input ON document_id=wfc_bitmap_input.id ORDER BY ordinal").all()).toEqual([{red:4294967295,palette_indices:Uint8Array.from([0,1])},{red:2,palette_indices:Uint8Array.from([0,1])}]);expect(await bitmapSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(input);db.query("UPDATE wfc_bitmap_pin SET x=7 WHERE ordinal=1").run();const edited=structuredClone(input);edited.pinned[1]!.x=7;expect(await bitmapSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(edited);}finally{db.close();}});
test("bitmap malformed relations and before-allocation control laws",async()=>{const db=await bitmapSnapshotToSqliteDatabase(input);for(const edit of [(d:any)=>d.tables[5].rows[0].values[1]=2n,(d:any)=>d.tables[2].rows[0].values[2]=9n,(d:any)=>d.tables[3].rows[0].values[3]=2n,(d:any)=>d.tables[0].rows[0].values[2]="01"]){const malformed=structuredClone(db);edit(malformed);await expect(bitmapSnapshotFromSqliteDatabase(malformed)).rejects.toThrow();}await expect(bitmapSnapshotToSqliteDatabase({...input,seed:18446744073709551616n})).rejects.toThrow();await expect(bitmapSnapshotToSqliteDatabase(input,{maxRows:1})).rejects.toThrow();await expect(bitmapSnapshotFromSqliteDatabase(db,{maxValueBytes:1})).rejects.toThrow();const c=new AbortController();c.abort();await expect(bitmapSnapshotToSqliteDatabase(input,{signal:c.signal})).rejects.toThrow();const arbitrary={...input,input:{...input.input,palette:[],pixels:new TextEncoder().encode("invalid native scalar 🧬")},model:{...input.model,ground:null}};expect(await bitmapSnapshotFromSqliteDatabase(await bitmapSnapshotToSqliteDatabase(arbitrary))).toEqual(arbitrary);});

test("unlinked public artifact resolves its own exports in independent Bun runtime",async()=>{const root=fileURLToPath(new URL("../../../../../../../../📦️packages/🟦️typescript",import.meta.url));const code="const artifact=await import(Bun.resolveSync(process.argv[1],process.argv[2]));if(typeof artifact.bitmapSnapshotToSqliteDatabase!==\"function\"||artifact.WFC_BITMAP_DOCUMENT_SCHEMA!==\"s.wfc.bitmap\")throw Error(\"public facet\");console.log(\"owned-package-self-reference\");";const child=Bun.spawn([process.execPath,"-e",code,"@semio-tech/wfc-bitmap",root],{stdout:"pipe",stderr:"pipe"});expect(await child.exited).toBe(0);expect(await new Response(child.stdout).text()).toBe("owned-package-self-reference\n");});


test("bitmap controlled native example has independent collection and text extents",async()=>{const d=new Database(":memory:");try{const result=d.query("WITH RECURSIVE items(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM items WHERE n<?) SELECT count(*) AS total,sum(n<=?) AS canceled FROM items").get(fixture.controlledAdmission.collectionItems,fixture.controlledAdmission.cancelAfter);expect(result).toEqual({total:1024,canceled:256});expect(d.query("SELECT length(CAST(? AS BLOB)) AS bytes").get("x".repeat(fixture.controlledAdmission.largeTextBytes))).toEqual({bytes:100000});}finally{d.close();}});

type OwningTarget={project:string;manifest:string;target:string;command:string};
const workspaceRoot=fileURLToPath(new URL("../".repeat(13),import.meta.url));
function expectOwningTargets(rows:OwningTarget[]){expect(rows.length).toBe(15);for(const expected of rows){const manifest=JSON.parse(readFileSync(join(workspaceRoot,expected.manifest),"utf8")),target=manifest.targets[expected.target];expect(manifest.name).toBe(expected.project);expect(target.executor).toBe("nx:run-commands");expect(target.options.command).toBe(expected.command);expect(target.options.cwd).toBe(dirname(expected.manifest));}}
test("WFC fifteen exact owning SQLite commands are declared by their owner Nx targets and package routers",()=>{
 expectOwningTargets(fixture.owningSqliteTargets);
 for(const expected of fixture.owningSqliteTargets)expect(readFileSync(join(workspaceRoot,dirname(expected.manifest),"📜️script.ts"),"utf8")).toMatch(/snapshotSqliteTests:\s*\["/u);
});

test("WFC fifteen owning Source build check test commands are declared by their owner Nx targets",()=>{expectOwningTargets(fixture.owningSourceTargets);});

import byteFields from "../../../📝️text/📸️snapshot/🧫️fixtures/byte-fields/🔣️.json";
test("bitmap intrinsic bytes and physical base64 agree with independent Buffer",()=>{
 for(const vector of byteFields.vectors){const bytes=Uint8Array.from(vector.bytes);expect(Buffer.from(bytes).toString("base64")).toBe(vector.base64);expect(encodeBase64(bytes)).toBe(vector.base64);expect(decodeBase64(vector.base64)).toEqual(bytes);expect(bitmapJsonBind<{pixels:Uint8Array}>({pixels:vector.base64}).pixels).toEqual(bytes);expect(bitmapJsonValue({pixels:bytes})).toEqual({pixels:vector.base64});}
 for(const text of byteFields.refused)expect(()=>decodeBase64(text)).toThrow();
});
