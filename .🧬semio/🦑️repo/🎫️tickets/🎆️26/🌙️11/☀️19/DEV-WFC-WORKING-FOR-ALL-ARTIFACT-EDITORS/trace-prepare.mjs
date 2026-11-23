import { readFileSync } from "node:fs";
const root = new URL(".", import.meta.url);
const component = readFileSync(new URL("../../../../../../../.cache/b/r/m/components/runs/w0GhkIl/w/175d210bc175a557befaf287.linked/component.wasm", root));
const core = component.subarray(11692, 234990681);
const rows = JSON.parse(readFileSync(new URL("./imports.json", root), "utf8"));
const imports = {};
for (const row of rows) {
  const bucket = imports[row.module] ??= {};
  bucket[row.name] = (...args) => {
    const error = new Error(`host ${row.module}::${row.name} args ${args.length}`);
    throw error;
  };
}
Error.stackTraceLimit = 80;
const started = Date.now();
console.log(`[DEBUG] compiling ${core.byteLength} bytes`);
const mod = new WebAssembly.Module(core);
console.log(`[DEBUG] compiled in ${Date.now() - started} ms`);
let instance;
try {
  instance = new WebAssembly.Instance(mod, imports);
} catch (error) {
  console.log(`[DEBUG] instantiate failed ${error && error.stack || error}`);
  process.exit(1);
}
const prepare = instance.exports.semio_owned_prepare_actor_v1;
console.log(`[DEBUG] calling prepare`);
try {
  const value = prepare();
  console.log(`[DEBUG] prepare returned ${value}`);
} catch (error) {
  console.log(String(error && error.stack || error));
}
