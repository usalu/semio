//! 🔭️ 🔭️ Flow play app commands command — `set-lod-mode`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[derive(semio_framework_value::RetireOwned)]
pub struct SetLodMode {
    pub value: String,
}

/// 🎚️ Unknown lod ids are rejected outright (rather than clamped) — the select control only ever
/// offers `FLOW_LOD_MODE_AUTOMATIC` plus the real `DagDrawLod` ids.
pub fn handle(_payload: &SetLodMode, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}

//#region 🪢️TaxonomyMounts

//#endregion 🪢️TaxonomyMounts
