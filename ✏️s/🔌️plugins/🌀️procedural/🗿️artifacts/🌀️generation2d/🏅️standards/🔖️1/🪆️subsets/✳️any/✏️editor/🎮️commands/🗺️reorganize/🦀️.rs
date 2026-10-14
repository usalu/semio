//! 🕸️ 🕸️ Generation2d play app commands command — `reorganize`.

use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::host_reorganize;
use semio_s_artifact_procedural_2d_model::with_host;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.host_snapshot;
    Ok(Emit::mutations(with_host(fixture, |host| host_reorganize(host, r#"{"orientation":"leftRight"}"#)).unwrap_or_default()))
}

//#region 🧪️Tests
//#endregion 🧪️Tests
