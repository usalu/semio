import { createWebGpuSurfacePort } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🧊️webgpu/📦️packages/🟨️javascript/🟨️webgpu-surface.js";

const port = createWebGpuSurfacePort({ maximumQueue: 0 });
port.bindMemory(new WebAssembly.Memory({ initial: 1 }));
const results = [];
for (let value = 0; value < 9; value += 1) results.push(port.cancel(BigInt(value + 1), 1));
const observed = { accepted: results.map((result) => result.accepted), controls: port.state.controls, queued: port.state.queue.length };
console.log(JSON.stringify(observed));
if (observed.controls !== 0 || observed.queued !== 0 || observed.accepted.some(Boolean)) process.exitCode = 1;
