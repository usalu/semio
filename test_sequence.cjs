const fs = require('fs');

const rs_js = fs.readFileSync("/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio/coda/client/bin/assistant/sketchpad-rs/pkg/sketchpad_rs.js", "utf8").replace("script_src = new URL(document.currentScript.src, location.href).toString();", "try { script_src = new URL(document.currentScript.src, location.href).toString(); } catch(e) { script_src = \"\"; }");
const wasmBinary = fs.readFileSync('/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio/coda/client/bin/assistant/sketchpad-rs/pkg/sketchpad_rs_bg.wasm');
const wasmBase64 = wasmBinary.toString('base64');

const inlined_rs = `
window.__wasmLogs = [];
function initLog(msg, isErr=false) {
    window.__wasmLogs.push({msg: msg, err: isErr});
    if (window.logToConsole) window.logToConsole(msg);
}
${rs_js}
try {
    initLog("[WASM-Init] Decoding wasmBase64...");
    const wasmBase64 = "${wasmBase64}";
    const wasmBinary = Uint8Array.from(atob(wasmBase64), c => c.charCodeAt(0));
    initLog("[WASM-Init] Decoded size: " + wasmBinary.length);
    window.wasmInitPromise = wasm_bindgen(wasmBinary.buffer).then(() => {
        wasm_bindgen.init_engine();
        window.sketchpadRs = wasm_bindgen;
        initLog("[WASM-Init] WASM Engine initialized successfully!");
    }).catch(e => {
        initLog("[WASM-Init] Error in wasm_bindgen: " + e.message, true);
    });
} catch (err) {
    initLog("[WASM-Init] Error decoding or initializing WASM: " + err.message, true);
}
`;

const jsdom = require("jsdom");
const { JSDOM } = jsdom;

const dom = new JSDOM(`<body>
    <div id="log-console"></div>
</body>`, { runScripts: "dangerously" });

const window = dom.window;
window.TextDecoder = TextDecoder;
window.TextEncoder = TextEncoder;

// Run the WASM loading script
const wasmScript = window.document.createElement("script");
wasmScript.textContent = inlined_rs;
window.document.body.appendChild(wasmScript);
window.TextDecoder = TextDecoder;
window.TextEncoder = TextEncoder;

// Now simulate main_mcp.js execution
window.eval(`
    function logToConsole(msg) {
        console.log("FROM LOGTOCONSOLE: " + msg);
    }
    window.logToConsole = logToConsole;

    if (window.__wasmLogs) {
        window.__wasmLogs.forEach(l => logToConsole(l.msg));
        window.__wasmLogs = [];
    }
`);

// Wait for promises
setTimeout(() => {
    console.log("Finished waiting");
}, 1000);
