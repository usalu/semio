//! 🕸️ 🕸️ Generation3d play app commands command — `reorganize`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::standards::v1::subsets::any::schema::{commit_fixture, with_host};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    with_host(fixture, |host| {
        if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
            Ok(Emit::mutations(commit_fixture(fixture, &host.fixture)))
        } else {
            Ok(Emit::default())
        }
    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
