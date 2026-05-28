const fs = require('fs');
const jsCode = fs.readFileSync('./coda/client/bin/assistant/sketchpad-rs/pkg/sketchpad_rs.js', 'utf8');
const wasmBinary = fs.readFileSync('./coda/client/bin/assistant/sketchpad-rs/pkg/sketchpad_rs_bg.wasm');

const sandbox = { fetch: null, Request: null, URL: URL, WebAssembly };
require('vm').createContext(sandbox);
require('vm').runInContext(jsCode, sandbox);

sandbox.wasm_bindgen(wasmBinary.buffer).then((wasm) => {
    console.log("WASM Initialized successfully in Node!", Object.keys(wasm));
    sandbox.wasm_bindgen.init_engine();
    console.log("Engine initialized!");
}).catch(err => {
    console.error("WASM ERROR:", err);
});
