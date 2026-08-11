//! 🧪 W5B prototype host: loads the real compiled `w5b-extension-echo` wasm component through
//! `ExtensionRuntime` and drives a full `manifest()` → `activate()` → `invoke("add", ...)` round
//! trip, printing each step so failure is visible exactly where it happens.

use semio_framework_plugin_host::ExtensionRuntime;

fn main() {
    let wasm_path = "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/w5b-extension-prototype/target/wasm32-wasip2/release/w5b_extension_echo.wasm";

    println!("[w5b] building ExtensionRuntime (engine + linker)...");
    let runtime = ExtensionRuntime::new().expect("build extension runtime");

    println!("[w5b] loading component: {wasm_path}");
    let extension_id = runtime.load(wasm_path).expect("load_bytes: instantiate + manifest() + activate()");
    println!("[w5b] loaded extension_id = {extension_id:?}");

    let manifest = runtime.manifest(&extension_id).expect("manifest must be present after load");
    println!("[w5b] manifest = {manifest:?}");
    assert_eq!(manifest.extension_id, "w5b.echo");
    assert_eq!(manifest.label, "W5B Echo Extension");
    assert_eq!(manifest.version, "0.1.0");
    assert_eq!(manifest.extends, "w5b.host");

    let request = serde_json::to_vec(&serde_json::json!({ "a": 19, "b": 23 })).unwrap();
    println!("[w5b] invoking capability \"add\" with request = {:?}", String::from_utf8_lossy(&request));
    let result_bytes = runtime.extension_invoke(&extension_id, "add", &request).expect("extension_invoke(\"add\", ...) must succeed");
    let result: serde_json::Value = serde_json::from_slice(&result_bytes).expect("result must be valid json");
    println!("[w5b] invoke result = {result}");
    assert_eq!(result, serde_json::json!({ "sum": 42 }));

    println!("[w5b] invoking unknown capability \"nope\" (expect a fault, not a panic)...");
    let error = runtime.extension_invoke(&extension_id, "nope", &[]).expect_err("unknown capability must fault");
    println!("[w5b] unknown-capability fault = {error:?}");
    assert_eq!(error.code.0, "extension.unknown-capability");

    println!("[w5b] ALL ROUND-TRIP ASSERTIONS PASSED — extension-world ABI works end-to-end with a real compiled component.");
}
