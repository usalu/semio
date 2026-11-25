import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import assert from "node:assert/strict";

const {createFlowHost, FlowOperation, FLOW_MAX_REQUEST_BYTES} = await import(pathToFileURL(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js")));
const memory = new WebAssembly.Memory({initial:32});
const text = "ü".repeat((FLOW_MAX_REQUEST_BYTES - 12) / 2);
let frame;
const host = createFlowHost({memory,schedule:()=>{},exports:{memory,
  flow_bridge_allocate:()=>8,flow_bridge_release:()=>{},
  flow_bridge_send(offset,length){frame=Buffer.from(new Uint8Array(memory.buffer,offset,length)); return 1;},
  flow_bridge_begin_close(){},flow_bridge_terminal_is_empty(){return 0;},
  flow_bridge_step_progress:()=>0n,flow_bridge_poll:()=>0
}});
const task = host.start(FlowOperation.setCatalogueJson,{json:text},{slot:1,generation:1});
if (!frame) await task.result;
assert.equal(frame.readUInt32LE(16),FLOW_MAX_REQUEST_BYTES);
assert.equal(frame.readUInt32LE(28),Buffer.byteLength(text));
assert.deepEqual(frame.subarray(32),Buffer.from(text));
console.log("Full-size UTF-8 catalogue frame passed");
