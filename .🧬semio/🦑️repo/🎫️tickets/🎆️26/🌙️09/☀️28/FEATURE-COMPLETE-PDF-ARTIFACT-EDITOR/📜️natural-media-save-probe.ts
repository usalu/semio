/** 🧪️ Replays archive delivery refusals through the live segmented helper. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv2020 from "ajv/dist/2020";
import {encodePackValue} from "@semio-tech/framework/packs";
import {EXPORT_MEDIA_FILE_ACTION_ID} from "@semio-tech/framework";
const {downloadDeclaredMediaFileV1, declaredNaturalMediaFileExportV1} = await import(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/📤️file/🟦️.ts"));
const staged = process.argv.includes("--red") ? JSON.parse(readFileSync(new URL("./🗑️generated/natural-media-save/overlay.json",import.meta.url),"utf8")) : [];
const fixtureFile = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📤️checked-media-export/📜️declarations.json";
const schemaFile = fixtureFile.replace("📜️declarations.json","🧬️schema/📜️declarations.json");
const read = (p:string) => JSON.parse(staged.find((row:{path:string}) => row.path === p)?.after ?? readFileSync(p,"utf8"));
const fixture = read(fixtureFile);
assert.ok(new Ajv2020({strict:true}).compile(read(schemaFile))(fixture));
const decl = {...fixture.defaults,portMediaType:fixture.port.mediaType};
const binary = {...decl,binary:true};
const native = {...binary,outputPort:"artifact:native"};
const io = {...fixture.port,id:native.outputPort};
if (!process.argv.includes("--red")) {
  const app = {actions:[{id:EXPORT_MEDIA_FILE_ACTION_ID,args:Object.entries(native).map(([id,value]) => ({id,default:value}))}],windowKinds:[],io:{ports:[io]}};
  assert.deepEqual(declaredNaturalMediaFileExportV1(app,native,"natural.pdf"), {outputPort:"artifact:native",filename:"natural.pdf",mediaType:native.mediaType,formatKind:native.formatKind,portMediaType:io.mediaType});
  for(const option of [{formatKind:"another"},{mediaType:"application/pdf"},{binary:false}]) assert.equal(declaredNaturalMediaFileExportV1(app,{...native,...option},"natural.pdf"),null);
}
const failures: string[] = [];
for(const row of fixture.deliveryCases.filter((row:{id:string})=>row.id.startsWith("archive-"))){
  const calls:string[] = [];
  const abort = new AbortController();
  const handle = {app_instance_id:13,parent_document_id:"document",base_revision:7n,operation_id:17n,generation:2n};
  let delivered = false, error = null, remaining = [new Uint8Array([37,80]),new Uint8Array([68,70])];
  try {
    delivered = await downloadDeclaredMediaFileV1({
      instanceId: 13, declaration: decl,
      port: {
        readAppDocumentIdentity: async () => handle,
        submitMediaExport: async (instance:number,output:string,parent:string,revision:bigint) => {assert.deepEqual([instance,output,parent,revision],[13,decl.outputPort,"document",7n]);return handle;},
        pollMediaExport: async () => ({handle,state:"complete",applied_progress:4n,checkpoint_available:false,mime_type:decl.mediaType,total_bytes:4n,detail:"",descriptor:[...encodePackValue(fixture.descriptor)]}),
        cancelMediaExport: async () => assert.fail("completed producer should already be drained"),
        takeMediaExportChunk: async () => { const data = remaining.shift(); return {handle,data:data??new Uint8Array(),terminal:data===undefined}; },
      },
      signal: abort.signal, current: () => true, progress: () => {}, yieldTurn: async () => {},
      sinkFactory: async () => {calls.push("open");return {write:async () => {calls.push("write");},close:async () => {calls.push("close");},abort:async () => {calls.push("abort");}};},
      beforeDelivery: async () => {
        assert.deepEqual(calls,["open","write","write"]);
        if (row.id === "archive-refused-before-delivery") throw Error("natural-file.saved-revision-changed");
        abort.abort();
      },
    });
  } catch(cause) {error=cause instanceof Error?cause.message:String(cause);}
  try {assert.deepEqual({delivered,error,sink:calls},{delivered:row.delivered,error:row.error,sink:row.sink});}
  catch(cause){failures.push(row.id + " " + JSON.stringify({delivered,error,sink:calls}));}
}
assert.deepEqual(failures,[]);
console.log("[DEBUG] Verified two exact archive delivery refusals and native port selection");
