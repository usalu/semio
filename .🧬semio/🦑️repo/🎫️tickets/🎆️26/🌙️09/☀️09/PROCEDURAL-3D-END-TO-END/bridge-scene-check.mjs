import {readFile} from "node:fs/promises";
import {resolve, join} from "node:path";
import {pathToFileURL} from "node:url";
import assert from "node:assert/strict";

if (process.env.FLOW_LOGICAL_CLOCK) {
  let time = 0;
  Object.defineProperty(globalThis, "performance", {value:{now:() => (time+=0.001)},configurable:true});
}
const publication = resolve(process.argv[2]);
const bindings = await import(pathToFileURL(join(publication, "flow_core.js")));
const hostModule = await import(pathToFileURL(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js")));
const wasm = await bindings.default({module_or_path: await readFile(join(publication, "flow_core_bg.wasm"))});
let calls=0, quiet=0;
const bridge={...wasm,flow_bridge_poll(...args){
  const length=wasm.flow_bridge_poll(...args);
  calls++;
  const progress=[0,1,2,3].map(axis=>Number(wasm.flow_bridge_step_progress(axis)));
  if(length || progress.some(x=>x)) {
    if(quiet>16 || length<0) console.log("quiet polls",quiet,"at",calls,"result",length,"progress",progress);
    quiet=0;
  } else if (++quiet === 4096) console.log("4096 empty polls at",calls);
  return length;
}};
const host = hostModule.createFlowHost({exports: bridge, memory: wasm.memory, schedule: setImmediate, retainedGrant: {...hostModule.FLOW_RETAINED_GRANT,maximumCopyBytes:Number(process.env.COPY ?? hostModule.FLOW_RETAINED_GRANT.maximumCopyBytes),maximumItems:Number(process.env.ITEMS ?? 1)}});
const timeout = setTimeout(() => { console.error("Flow scene timed out"); process.exit(1); }, 30000);
const features = await hostModule.createFlowFeatures(host);
const inputs = [
  ["attachSurface", {surface:1, surfaceGeneration:1, width:800, height:600, dpr:2}],
  ["surfaceStatus", {surface:1, surfaceGeneration:1, status:"created"}],
  ["setSelection", {json:JSON.stringify({nodes:["profile"],edges:[],handles:[]})}],
  ["setComputingProgress", {json:JSON.stringify({active:"profile",stale:["extrude"]})}],
  ["setNodeStatuses", {json:JSON.stringify({profile:{status:"queued"},extrude:{status:"computing"}})}],
  ["setSelectedChannels", {json:JSON.stringify([{widgetId:"profile",port:"out",direction:"out"}])}]
];
for (const [name,args] of inputs) {
  const group = Object.values(features).find(group => typeof group[name] === "function");
  assert.ok(group, name);
  console.log(name);
  const task = group[name](args);
  task.subscribe(event => { if(event.event === 2656) console.log("terminal", Buffer.from(event.body).toString("hex")); });
  await task.result;
}
console.log("Closing session",calls);
await features.lifetime.close();
await host.close();
assert.ok(host.terminalIsEmpty());
clearTimeout(timeout);
console.log("Scene inputs and session closure passed");
