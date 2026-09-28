//! 📥 Replaces the document with an imported fixture JSON file. The host's chunked inbound lane
//! (`Effect::RequestFileOpen` → one `importFixture {payload, name, chunk, chunkCount}` per
//! `IMPORT_CHUNK_BYTES` page) is reassembled by the framework before this action runs
//! (`semio_framework::kernel::ImportStaging`, admitted in the SDK's `dispatch_action`), so the action reads
//! ONE whole `payload`; an agent may hand the fixture inline as a `json`/`fixture` object instead.

use crate::editor::puzzle2d::{Puzzle2dActionCtx, PUZZLE2D_FIXTURE_SCHEMA};
use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;
use semio_framework::kernel::{UiDirtyScope, IMPORT_ARGUMENT_PAYLOAD};
use serde_json::Value;

//#region 🔖️Vocabulary
/// 🚫️ Why one import was refused — every arm becomes a localized shell notice, never a silent no-op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dImportFault {
    /// 📦️ The file is larger than [`PUZZLE_IMPORT_TOTAL_BYTES`], the budget one export may stream.
    Capacity,
    /// 🔤️ The file is not one JSON object, or not a puzzle 2d fixture.
    Payload,
}

impl Puzzle2dImportFault {
    /// 🏷️ Stable code, mirrored by the unit laws.
    pub fn code(self) -> &'static str {
        match self {
            Self::Capacity => "puzzle2d-import-capacity",
            Self::Payload => "puzzle2d-import-payload",
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Command
/// 📥 The fixture an import carries: the whole picked file as `payload`, or an inline `json`/`fixture` object —
/// accepted only when it is a puzzle 2d fixture (its `schema` or a `nodes` array).
pub fn puzzle2d_import_value(args: &Value) -> Result<Value, Puzzle2dImportFault> {
    let value = match args.get(IMPORT_ARGUMENT_PAYLOAD).and_then(Value::as_str) {
        Some(text) if text.len() > PUZZLE_IMPORT_TOTAL_BYTES => return Err(Puzzle2dImportFault::Capacity),
        Some(text) => serde_json::from_str::<Value>(text).map_err(|_| Puzzle2dImportFault::Payload)?,
        None => args.get("json").or_else(|| args.get("fixture")).or_else(|| args.get(IMPORT_ARGUMENT_PAYLOAD)).cloned().filter(Value::is_object).ok_or(Puzzle2dImportFault::Payload)?,
    };
    let is_fixture = value.get("schema").and_then(Value::as_str) == Some(PUZZLE2D_FIXTURE_SCHEMA) || value.get("nodes").is_some_and(Value::is_array);
    if !is_fixture {
        return Err(Puzzle2dImportFault::Payload);
    }
    Ok(value)
}

/// 📥 Replaces the document with the imported fixture as one document edit; every refusal publishes a named,
/// localized notice and changes nothing.
pub fn import_fixture(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let mut fixture = match args.ok_or(Puzzle2dImportFault::Payload).and_then(puzzle2d_import_value) {
        Ok(fixture) => fixture,
        Err(fault) => {
            match fault {
                Puzzle2dImportFault::Capacity => ctx.notice(|labels| labels.import_too_large.as_str()),
                Puzzle2dImportFault::Payload => ctx.notice(|labels| labels.import_invalid.as_str()),
            }
            *ctx.ui_scope = UiDirtyScope::None;
            return;
        }
    };
    if let Some(object) = fixture.as_object_mut() {
        object.entry("schema").or_insert_with(|| Value::String(PUZZLE2D_FIXTURE_SCHEMA.into()));
        object.entry("edges").or_insert_with(|| Value::Array(Vec::new()));
    }
    ctx.scene.fixture = fixture;
}
//#endregion 🔖️Command
