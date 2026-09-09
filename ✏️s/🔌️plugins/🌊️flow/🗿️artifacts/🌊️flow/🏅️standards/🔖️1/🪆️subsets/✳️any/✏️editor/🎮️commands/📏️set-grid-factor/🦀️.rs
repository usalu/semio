//! 🌐️ 🔳️ Flow play app commands command — `set-grid-factor`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct SetGridFactor {
    pub value: f64,
}

/// 🔳️ Clamped to the slider's own `0.5..=50.0` range so a scripted dispatch can't desynchronize the
/// control from the config.
pub fn handle(_payload: &SetGridFactor, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
