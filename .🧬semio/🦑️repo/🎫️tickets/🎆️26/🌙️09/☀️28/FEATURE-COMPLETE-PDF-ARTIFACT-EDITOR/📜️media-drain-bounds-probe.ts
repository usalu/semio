/** 🧭 Exercises exact data and terminal limits without retaining output bytes. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv2020 from "ajv/dist/2020";
import type {MediaExportHandle} from "@semio-tech/framework-os";
const {receiveMediaExportBytes}=await import(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/🚚️lifecycle/🟦️.ts"));
type DrainBoundsRow = Readonly<{id:string;dataPages:number;pageBytes:number;terminal:boolean;expectedDelivery:boolean;expectedPages:number;expectedTakes:number;expectedCancel:number;expectedError:string|null}>;
async function exerciseDrainBounds(row: DrainBoundsRow) {
  const handle: MediaExportHandle = { app_instance_id: 4, parent_document_id: "saved", operation_id: 9n, generation: 6n, base_revision: 8n };
  const reusable = new Uint8Array(row.pageBytes), empty = new Uint8Array();
  let takes = 0, pages = 0, cancelled = 0, delivered = false, error = null;
  try {
    delivered = await receiveMediaExportBytes({
      instanceId: handle.app_instance_id, parentDocumentId: handle.parent_document_id, revision: handle.base_revision,
      outputPort: "artifact:native", mediaType: "application/pdf", cancelled: () => false, progress: () => {},
      now: () => 0, yieldTurn: async () => {},
      port: {
        submitMediaExport: async () => handle,
        pollMediaExport: async () => ({ handle, state: "complete", applied_progress: 1n, checkpoint_available: false, mime_type: "application/pdf", total_bytes: BigInt(row.dataPages * row.pageBytes), detail: "", descriptor: [1] }),
        cancelMediaExport: async () => { cancelled++; },
        takeMediaExportChunk: async () => {
          const data = ++takes <= row.dataPages;
          return { handle, data: data ? reusable : empty, terminal: !data && row.terminal };
        },
      },
    }, { begin: () => {}, page: () => { pages++; } });
  } catch (failure) {
    error = failure instanceof Error ? failure.message : String(failure);
  }
  return { pages, takes, cancelled, delivered, error };
}

const live=process.argv.includes("--live");
const fromLive = (path: string)=>JSON.parse(readFileSync(path,"utf8"));
const source = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📤️checked-media-export/📜️declarations.json", reference="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📤️checked-media-export/🧬️schema/📜️declarations.json";
const stage = live ? [] : fromLive(resolve(import.meta.dir,"🗑️generated/media-drain-bounds.json"));
const read=(path:string)=>stage.length===0 ? fromLive(path) : JSON.parse(stage.find((row:{path:string})=>row.path===path).after);
const corpus=read(source);
assert.ok(new Ajv2020({ strict:true }).compile(read(reference))(corpus));
const failures = [];
for(const row of corpus.drainBounds) {
  const result=await exerciseDrainBounds(row);
  try { assert.deepEqual(result,{pages:row.expectedPages,takes:row.expectedTakes,cancelled:row.expectedCancel,delivered:row.expectedDelivery,error:row.expectedError}); }
  catch { failures.push({id:row.id,...result}); }
}
assert.deepEqual(failures,[]);
console.log("[DEBUG] Exact page cap, overrun and missing terminal laws passed");
