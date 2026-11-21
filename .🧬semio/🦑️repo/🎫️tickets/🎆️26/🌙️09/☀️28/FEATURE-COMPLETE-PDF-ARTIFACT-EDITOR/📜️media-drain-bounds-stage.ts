/** 🛑 Stages one extra terminal probe after the maximum admitted data pages. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const stage=join(import.meta.dir,"🗑️generated","media-drain-bounds.json");
const paths={"life":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/🚚️lifecycle/🟦️.ts","tests":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📤️checked-media-export/🟦️.ts","fixture":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📤️checked-media-export/📜️declarations.json","schema":"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📤️checked-media-export/🧬️schema/📜️declarations.json"};
const hash=(s:string)=>createHash("sha256").update(s).digest("hex");
const edit=(s:string,a:string,b:string)=>{
  assert.equal(s.split(a).length,2,a);
  return s.replace(a,b);
};
if(process.argv.includes("--apply")){
  const rows=JSON.parse(readFileSync(stage,"utf8")) as {path:string;before:string;after:string}[];
  for(const row of rows)assert.equal(hash(readFileSync(row.path,"utf8")),row.before,row.path);
  for(const row of rows){
    assert.equal(hash(readFileSync(row.path,"utf8")),row.before,row.path);
    writeFileSync(row.path,row.after);
    assert.equal(readFileSync(row.path,"utf8"),row.after,row.path);
  }
  console.log("[DEBUG] Applied "+rows.length+" media drain cap paths");
}else{
  const rows:{path:string;before:string;after:string}[]=[];
  function update(path:string,patch:(source:string)=>string){
    const source=readFileSync(path,"utf8"),after=patch(source);
    assert.notEqual(source,after,path);
    rows.push({path,before:hash(source),after});
  }
  update(paths.schema,source=>{
    const result=JSON.parse(source);
    result.required.push("drainBounds");
    const natural={type:"integer",minimum:0,maximum:8193};
    result.properties.drainBounds={type:"array",minItems:3,maxItems:3,items:{type:"object",additionalProperties:false,required:["id","dataPages","pageBytes","terminal","expectedDelivery","expectedPages","expectedTakes","expectedCancel","expectedError"],properties:{id:{type:"string"},dataPages:natural,pageBytes:{type:"integer",minimum:1,maximum:4096},terminal:{type:"boolean"},expectedDelivery:{type:"boolean"},expectedPages:natural,expectedTakes:natural,expectedCancel:{type:"integer",minimum:0,maximum:1},expectedError:{type:["null","string"]}}}};
    return JSON.stringify(result,null,2)+"\n";
  });
  update(paths.fixture,source=>JSON.stringify({...JSON.parse(source),drainBounds:[{"id":"at-data-page-cap","dataPages":8192,"pageBytes":4096,"terminal":true,"expectedDelivery":true,"expectedPages":8192,"expectedTakes":8193,"expectedCancel":0,"expectedError":null},{"id":"extra-data-page","dataPages":8193,"pageBytes":1,"terminal":true,"expectedDelivery":false,"expectedPages":8192,"expectedTakes":8193,"expectedCancel":1,"expectedError":"media-export.pages"},{"id":"missing-terminal-after-cap","dataPages":8192,"pageBytes":4096,"terminal":false,"expectedDelivery":false,"expectedPages":8192,"expectedTakes":8193,"expectedCancel":1,"expectedError":"media-export.pages"}]},null,2)+"\n");
  update(paths.tests,source=>edit(source,"import { collectMediaExportBytes, MEDIA_EXPORT_OPERATION_POLICY,","import { collectMediaExportBytes, receiveMediaExportBytes, MEDIA_EXPORT_OPERATION_POLICY,").trimEnd()+"\n\n"+"type DrainBoundsRow = Readonly<{id:string;dataPages:number;pageBytes:number;terminal:boolean;expectedDelivery:boolean;expectedPages:number;expectedTakes:number;expectedCancel:number;expectedError:string|null}>;\nasync function exerciseDrainBounds(row: DrainBoundsRow) {\n  const handle: MediaExportHandle = { app_instance_id: 4, parent_document_id: \"saved\", operation_id: 9n, generation: 6n, base_revision: 8n };\n  const reusable = new Uint8Array(row.pageBytes), empty = new Uint8Array();\n  let takes = 0, pages = 0, cancelled = 0, delivered = false, error = null;\n  try {\n    delivered = await receiveMediaExportBytes({\n      instanceId: handle.app_instance_id, parentDocumentId: handle.parent_document_id, revision: handle.base_revision,\n      outputPort: \"artifact:native\", mediaType: \"application/pdf\", cancelled: () => false, progress: () => {},\n      now: () => 0, yieldTurn: async () => {},\n      port: {\n        submitMediaExport: async () => handle,\n        pollMediaExport: async () => ({ handle, state: \"complete\", applied_progress: 1n, checkpoint_available: false, mime_type: \"application/pdf\", total_bytes: BigInt(row.dataPages * row.pageBytes), detail: \"\", descriptor: [1] }),\n        cancelMediaExport: async () => { cancelled++; },\n        takeMediaExportChunk: async () => {\n          const data = ++takes <= row.dataPages;\n          return { handle, data: data ? reusable : empty, terminal: !data && row.terminal };\n        },\n      },\n    }, { begin: () => {}, page: () => { pages++; } });\n  } catch (failure) {\n    error = failure instanceof Error ? failure.message : String(failure);\n  }\n  return { pages, takes, cancelled, delivered, error };\n}\nfor (const row of declarations.drainBounds) it(\"verified media page limit \" + row.id, async () => {\n  expect(await exerciseDrainBounds(row)).toEqual({ pages: row.expectedPages, takes: row.expectedTakes, cancelled: row.expectedCancel, delivered: row.expectedDelivery, error: row.expectedError });\n});\n");
  update(paths.life,source=>{
    let next=edit(source,"    for (let page = 1; page <= SEGMENTED_DOWNLOAD_CONTRACT.maximumOutstandingChunks; page += 1) {","    const maximumPages = SEGMENTED_DOWNLOAD_CONTRACT.maximumOutstandingChunks;\n    for (let page = 1; page <= maximumPages + 1; page += 1) {");
    return edit(next,'      if (!sameHandle(chunk.handle, handle)) refusal ??= new Error("media-export.chunk-owner");','      if (!sameHandle(chunk.handle, handle)) refusal ??= new Error("media-export.chunk-owner");\n      if (page > maximumPages && (!chunk.terminal || chunk.data.byteLength !== 0)) refusal ??= new Error("media-export.pages");');
  });
  writeFileSync(stage,JSON.stringify(rows,null,2)+"\n");
  console.log("[DEBUG] Staged "+rows.length+" media drain cap paths");
}
