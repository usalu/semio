//! 🚪️ mathematical <- json — foreign `Deserializer<MathematicalSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Every `MathematicalSnapshot`
//! field round-trips through `serde_json` untouched, so this hop is `IoFidelity::Exact`.
//!
//! 🌱️ `JsonSnapshot::from_value`/`.to_serde_value()` (`semio_s_plugin_stdio::artifacts::json`) are
//! still hard-typed to `serde_json::Value` — a foreign plugin's own API this crate does not own,
//! `🗄️stdio`'s own ~563-file deferred wave (ticket
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`'s `verified-outcomes.md`).
//! But the LAST hop, decoding into `MathematicalSnapshot` itself, no longer needs
//! `serde_json::from_value::<MathematicalSnapshot>` — the framework's reverse bridge
//! (`impl From<&serde_json::Value> for DslValue`, `🧰️framework/🔨️modules/🌱️value/🦀️component.rs`)
//! crosses from `JsonSnapshot`'s serde-typed output into `DslValue`, and `MathematicalSnapshot`
//! already implements `FromValue` — so this file only needs `serde_json` for the ONE call it
//! cannot avoid (`JsonSnapshot::from_value`/`.to_serde_value()`), not for `MathematicalSnapshot`
//! itself.

use crate::artifacts::mathematical::MathematicalSnapshot;
use dsl::FromValue;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoMathematical;

impl Deserializer<MathematicalSnapshot> for JsonIntoMathematical {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn deserialize(payload: &IoPayload) -> IoResult<MathematicalSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "JsonIntoMathematical: expected a binary json payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let text = std::str::from_utf8(bytes).map_err(|error| IoError { message: format!("JsonIntoMathematical: not valid utf-8: {error}"), diagnostics: Vec::new() })?;
        let value: serde_json::Value = serde_json::from_str(text).map_err(|error| IoError { message: format!("JsonIntoMathematical: not valid json: {error}"), diagnostics: Vec::new() })?;
        let json = JsonSnapshot::from_value(value);
        let dsl_value = dsl::DslValue::from(&json.to_serde_value());
        let snapshot = MathematicalSnapshot::from_value(dsl_value).map_err(|error| IoError { message: format!("JsonIntoMathematical: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
