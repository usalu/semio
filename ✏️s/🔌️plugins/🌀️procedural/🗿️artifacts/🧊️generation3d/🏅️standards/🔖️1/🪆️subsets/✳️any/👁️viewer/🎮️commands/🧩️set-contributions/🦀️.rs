//! 🧩️ Generation3d viewer command — `set-contributions`: the host's `flow.extension` closure,
//! installed into the plugin's process-wide flow extension registry one page at a time. Read-only by
//! construction: the emitted `ViewEmit` is empty, because the registry is runtime state and not a
//! store lane at all.

use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧩️ One page of the shell's `contributionsJson`. The viewer evaluates its own preview through
/// `FlowHost`, so it needs the same contributed operators the editor does; `validate_public_json_envelope`
/// caps every string in a public command invocation at `semio_framework::PUBLIC_INVOCATION_STRING_BYTES`,
/// so the closure always crosses as a page run.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-contributions")]
#[value(rename_all = "camelCase")]
pub struct SetContributions {
    pub json: String,
    pub page: u64,
    pub page_count: u64,
}

/// 🧩️ Buffers this page and, on the run's last one, installs the assembled closure.
pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    let page = u32::try_from(payload.page).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    let page_count = u32::try_from(payload.page_count).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    semio_framework_os_flow::sync_host_flow_extension_contributions_page(page, page_count, &payload.json).map_err(Fault::from)?;
    Ok(ViewEmit::default())
}
