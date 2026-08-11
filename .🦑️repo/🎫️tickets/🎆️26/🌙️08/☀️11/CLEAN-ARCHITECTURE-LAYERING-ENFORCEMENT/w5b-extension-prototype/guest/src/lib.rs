//! 🐚 W5B prototype: the simplest possible real `extension-world` guest component — one
//! capability handler ("add") that sums two i64s from a JSON payload. Exists purely to prove the
//! WIT extension-world ABI round-trips through a REAL compiled wasm component (cargo build →
//! wasm32-wasip2 component → wasmtime `ExtensionRuntime`), not just that it type-checks.

use semio_framework::Fault;
use semio_framework_plugin::plugin_runtime::ExtensionBundle;

fn bundle() -> ExtensionBundle {
    ExtensionBundle::new("w5b.echo", "W5B Echo Extension", "0.1.0").extends("w5b.host").handler("add", |request: &[u8]| -> Result<Vec<u8>, Fault> {
        let payload: serde_json::Value = serde_json::from_slice(request)
            .map_err(|error| Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("w5b.bad-request"), error.to_string()))?;
        let a = payload.get("a").and_then(serde_json::Value::as_i64).ok_or_else(|| {
            Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("w5b.bad-request"), "missing field `a`")
        })?;
        let b = payload.get("b").and_then(serde_json::Value::as_i64).ok_or_else(|| {
            Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("w5b.bad-request"), "missing field `b`")
        })?;
        Ok(serde_json::to_vec(&serde_json::json!({ "sum": a + b })).expect("json encode never fails for a plain object"))
    })
}

semio_framework_plugin::extension_exports!(bundle);
