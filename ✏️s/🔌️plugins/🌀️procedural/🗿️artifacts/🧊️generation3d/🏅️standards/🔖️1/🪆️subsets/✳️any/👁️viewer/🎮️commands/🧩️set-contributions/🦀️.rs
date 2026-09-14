//! 🧩️ Generation3d viewer command — `set-contributions`: the host's `flow.extension` closure,
//! installed into the plugin's process-wide flow extension registry one page at a time. Read-only
//! by construction: the registry is runtime state, not a store lane at all.

use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, ViewEmit};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧩️ One page of the shell's `contributionsJson`. The viewer evaluates its own preview through the
/// shared `previewEval` run, so it needs the same contributed operators the editor does;
/// `validate_public_json_envelope` caps every string in a public command invocation at
/// `semio_framework::PUBLIC_INVOCATION_STRING_BYTES`, so the closure always crosses as a page run.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-contributions")]
#[value(rename_all = "camelCase")]
pub struct SetContributions {
    pub json: String,
    pub page: u64,
    pub page_count: u64,
}

/// 🧩️ Buffers this page and, on the run's last one, installs the assembled closure into the flow
/// extension registry and invalidates every evaluation the missing registry had already faulted.
/// Answers whether the session was invalidated, which is what owes the attached previews a fresh
/// evaluation.
///
/// The install alone is not delivery. The host pushes this run AFTER the document is loaded, so the
/// first evaluation has already run against an EMPTY registry, cached its
/// `flow.extension-not-contributed` miss in the session's neural cache and incremental baseline and
/// given up — and installing operators into a process-wide registry publishes nothing, so without the
/// owed evaluation nothing in the app ever looks again (`📓️runtime-verification-2026-09-09.md` boot
/// #11).
///
/// The key is [`semio_framework_os_flow::flow_extension_registry_generation`], not "this was the
/// last page": a re-push of an unchanged closure leaves the generation where it was and owes nothing,
/// while ANY later contribution change re-evaluates (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn install(payload: &SetContributions, session: &mut FlowEvalSession) -> Result<bool, Fault> {
    let page = u32::try_from(payload.page).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    let page_count = u32::try_from(payload.page_count).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    semio_framework_os_flow::sync_host_flow_extension_contributions_page(page, page_count, &payload.json).map_err(Fault::from)?;
    Ok(session.invalidate_for_flow_extension_registry(semio_framework_os_flow::flow_extension_registry_generation()))
}

/// 🧩️ The `view_commands!` row. Its `handle(payload, doc, cfg)` signature is framework-fixed and
/// carries neither the retained session nor the attached-window roster, so it installs the page and
/// nothing else; the SERVED route (`Generation3dViewContributionsWork::step`, which is handed both)
/// is the one that invalidates and owes the attached previews an evaluation.
pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    let page = u32::try_from(payload.page).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    let page_count = u32::try_from(payload.page_count).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    semio_framework_os_flow::sync_host_flow_extension_contributions_page(page, page_count, &payload.json).map_err(Fault::from)?;
    Ok(ViewEmit::default())
}
