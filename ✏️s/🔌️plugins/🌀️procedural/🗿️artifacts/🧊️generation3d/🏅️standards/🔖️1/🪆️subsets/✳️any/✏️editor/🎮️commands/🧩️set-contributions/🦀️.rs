//! 🧩️ Generation3d play app commands command — `set-contributions`: the host's `flow.extension`
//! closure, installed into the plugin's process-wide flow extension registry one page at a time.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧩️ One page of the shell's `contributionsJson`. The payload cannot cross whole: every string in
/// a public command invocation is capped at `semio_framework::PUBLIC_INVOCATION_STRING_BYTES`
/// (4 KiB) by `validate_public_json_envelope`, which runs BEFORE the addressed tool's own wire
/// contract, and the generation3d closure is 293 642 characters. `page`/`page_count` address this
/// page inside the run the shell's `publicInvocationStringPages` cut.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-contributions")]
pub struct SetContributions {
    pub json: String,
    pub page: u64,
    pub page_count: u64,
}

/// 🧩️ Buffers this page and, on the run's last one, installs the assembled closure into the flow
/// extension registry AND re-arms every evaluation the missing registry had already faulted.
///
/// The install alone is not delivery. The host pushes this run AFTER the example is loaded, so the
/// first evaluation has already run against an EMPTY registry, cached its
/// `flow.extension-not-contributed` miss in the session's neural cache and incremental baseline,
/// published a `faulted` preview and given up its `flowEvalTick` chain — and installing operators
/// into a process-wide registry publishes nothing, so without this nothing in the app ever looks
/// again and the surface reads `faulted` forever with no user action able to change it
/// (`📓️runtime-verification-2026-09-09.md` boot #11).
///
/// The key is [`semio_framework_os_flow::flow_extension_registry_generation`], not "this was the
/// last page": a re-push of an unchanged closure leaves the generation where it was and re-arms
/// nothing, while ANY later contribution change — a plugin enabled, disabled or hot-swapped —
/// bumps it and re-evaluates. Each attached preview window owns its OWN retained evaluation
/// publication, so each gets its own chain back through the SAME addressed self-dispatch the tick
/// uses (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// Emits no store lane: the registry is process-wide runtime state, never a document, config or
/// transient lane — which is why this command's publication lane is `HostOnly`, and effects are not
/// a store lane (see `flow_eval_resolve`, `HostOnly` and self-re-arming for the same reason).
pub fn apply(
    payload: &SetContributions,
    _doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    session: &mut FlowEvalSession,
    preview_windows: &[(&str, &str)],
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let page = u32::try_from(payload.page).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    let page_count = u32::try_from(payload.page_count).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    semio_framework_os_flow::sync_host_flow_extension_contributions_page(page, page_count, &payload.json).map_err(Fault::from)?;
    let generation = semio_framework_os_flow::flow_extension_registry_generation();
    if !session.invalidate_for_flow_extension_registry(generation) {
        return Ok(Emit::default());
    }
    Ok(Emit { effects: preview_windows.iter().map(|(window_id, window_kind_id)| super::flow_eval_tick::rearm(window_id, window_kind_id, 105)).collect(), ..Default::default() })
}

/// 🧩️ The `app_commands!` row. Its `handle(payload, doc, cfg, ctx)` signature is framework-fixed and
/// carries no attached-window roster, exactly as it carries no `interaction` for
/// `delete_selection` — so it installs the page and invalidates the session, and the SERVED route
/// (`Generation3dContributionsWork::step`, which is handed the shell's trusted `ViewModel`) is the
/// one that re-arms. Reached only by the marks-free `handle`/`dispatch` fallbacks, which own no
/// preview window to arm a chain into anyway.
pub fn handle(payload: &SetContributions, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    apply(payload, doc, cfg, session, &[] as &[(&str, &str)])
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
