/** 🧪️ Checks actual paged download ordering and Blob ownership against independently framed PDF bytes. */
import assert from "node:assert/strict";
import {readFileSync,writeFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv2020 from "ajv/dist/2020";
import {encodePackValue} from "@semio-tech/framework/packs";
const {downloadDeclaredMediaFileV1} = await import(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/📤️file/🟦️.ts"));
const {segmentedDownloadSinkFactory} = await import(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📤️SegmentedDownload/🟦️.ts"));
const staged = process.argv.includes("--red") ? JSON.parse(readFileSync(new URL("./🗑️generated/progressive-media-download/overlay.json",import.meta.url),"utf8")) : [];
const path = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📤️checked-media-export/📜️declarations.json";
const read = (p:string) => JSON.parse(staged.find((row:{path:string}) => row.path === p)?.after ?? readFileSync(p,"utf8"));
const fixture = read(path);
assert.ok(new Ajv2020({strict:true}).compile(read(path.replace("📜️declarations.json","🧬️schema/📜️declarations.json")))(fixture));
const expected = fixture.deliveryCases.find((row:{id:string})=>row.id === "complete").transport;
const mediaType = "application/pdf", outputPort = "artifact:native", formatKind = "s.stdio.pdf@1.7", filename = "page-history.pdf";
const portMediaType = {class:"data",form:"document"};
const handle = {app_instance_id:13,parent_document_id:"document",base_revision:7n,operation_id:17n,generation:2n};
const original = readFileSync(new URL("./🗑️generated/pdf-page-history-source.pdf", import.meta.url));
const pages = [Uint8Array.from(original.subarray(0,431)),Uint8Array.from(original.subarray(431))];
const calls:string[] = [];
let pending: unknown;
const actualSink = segmentedDownloadSinkFactory((...args:unknown[]) => {pending=args.at(-1);});
const delivered = await downloadDeclaredMediaFileV1({
  instanceId:13,declaration:{filename,mediaType,outputPort,formatKind,portMediaType},
  port:{
    readAppDocumentIdentity:async()=>handle,submitMediaExport:async()=>handle,
    pollMediaExport:async()=>({handle,state:"complete",applied_progress:2n,checkpoint_available:false,mime_type:mediaType,total_bytes:BigInt(original.byteLength),detail:"",descriptor:[...encodePackValue({portId:outputPort, kindId:formatKind, mediaType:portMediaType, wire:{kind:"binary",format_kind:formatKind}})]}),
    takeMediaExportChunk:async()=>{calls.push("take");const data=pages.shift();return {handle,terminal:data===undefined,data:data??new Uint8Array()};},
    cancelMediaExport:async()=>assert.fail("exact operation is already terminal"),
  },
  signal:new AbortController().signal,current:()=>true,progress:()=>{},yieldTurn:async()=>{},
  sinkFactory:async(...args:readonly [string,string])=>{
    calls.push("open");
    const sink=await actualSink(...args);
    return {
      write: async (page:Uint8Array)=>{calls.push("write");await sink.write(page);page.fill(0);},
      close:async()=>{calls.push("close");await sink.close();},
      abort:sink.abort,
    };
  },
});
assert.ok(delivered);
assert.deepEqual({calls,blob:pending instanceof Blob},{calls:expected,blob:true});
const blob=pending as Blob;
assert.equal(blob.type,mediaType);
const bytes=Buffer.from(await blob.arrayBuffer());
assert.deepEqual(bytes,original);
writeFileSync(new URL("./🗑️generated/pdf-page-history-segmented-blob.pdf",import.meta.url),bytes);
console.log("[DEBUG] Exact ordered PDF Blob verified");
