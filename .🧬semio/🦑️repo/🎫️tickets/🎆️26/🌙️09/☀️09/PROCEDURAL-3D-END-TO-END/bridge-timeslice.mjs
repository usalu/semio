import { readFile } from "node:fs/promises";
import { resolve, join } from "node:path";
import { pathToFileURL } from "node:url";
import assert from "node:assert/strict";

const root = resolve(process.argv[2]);
const start = process.hrtime.bigint();
let reading = 0, samples = 0, injected = false, polling = false;
const timeout = setTimeout(() => {console.error("Timed out waiting for turn", samples, injected);process.exit(1);}, 10000);
Object.defineProperty(globalThis, "performance", { configurable:true, value:{ now() {
  if (polling && !injected && ++samples === 2) { injected = true; reading += 20; }
  return Number(process.hrtime.bigint()-start)/1e6 + reading;
}}});
const bindings = await import(pathToFileURL(join(root, "flow_core.js")));
const wasm = await bindings.default({module_or_path:await readFile(join(root,"flow_core_bg.wasm"))});
const {createFlowHost,createFlowFeatures} = await import(pathToFileURL(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js")));
injected = true;
const bridge = {...wasm,flow_bridge_poll(...args) {
  polling=true;
  try {return wasm.flow_bridge_poll(...args);} finally {polling=false;}
}};
const host = createFlowHost({exports:bridge,memory:wasm.memory,schedule:setImmediate});
const features = await createFlowFeatures(host);
samples=0;injected=false;
console.log("Scheduling surface attachment");
const result=await features.surface.attachSurface({surface:1,surfaceGeneration:1,width:400,height:300,dpr:1}).result;
console.log("Attachment answered",result);
assert.ok(injected);
await features.lifetime.close();
console.log("Session closed");
await host.close();
clearTimeout(timeout);
console.log("Surface attachment survived one expired poll and closed");
