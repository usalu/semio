//! 🧩️ Generation2d play app commands command — `set-contributions`: the host's `flow.extension`
//! closure, installed into the plugin's process-wide flow extension registry one page at a time.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧩️ One page of the shell's `contributionsJson` — the exact twin of generation3d's row. The
/// payload cannot cross whole: every string in a public command invocation is capped at
/// `semio_framework::PUBLIC_INVOCATION_STRING_BYTES` by `validate_public_json_envelope`, which runs
/// before the addressed tool's own wire contract.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-contributions")]
pub struct SetContributions {
    pub json: String,
    pub page: u64,
    pub page_count: u64,
}

/// 🧩️ Buffers this page and, on the run's last one, installs the assembled closure into the flow
/// extension registry and invalidates every `sessions` evaluation the missing registry had already
/// faulted. Answers whether any session was invalidated, which is what owes the attached previews a
/// fresh evaluation. The key is [`semio_framework_os_flow::flow_extension_registry_generation`], so a
/// re-push of an unchanged closure owes nothing while any later contribution change re-evaluates.
pub fn install(payload: &SetContributions, sessions: &mut [&mut FlowEvalSession]) -> Result<bool, Fault> {
    let page = u32::try_from(payload.page).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    let page_count = u32::try_from(payload.page_count).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    semio_framework_os_flow::sync_host_flow_extension_contributions_page(page, page_count, &payload.json).map_err(Fault::from)?;
    let generation = semio_framework_os_flow::flow_extension_registry_generation();
    Ok(sessions.iter_mut().fold(false, |invalidated, session| session.invalidate_for_flow_extension_registry(generation) | invalidated))
}

/// 🧩️ The `app_commands!` row. Its framework-fixed signature carries no attached-window roster, so it
/// installs the page and invalidates the session it is handed; the SERVED route
/// (`Generation2dContributionsWork::step`) is the one that owes the attached previews an evaluation.
pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    install(payload, &mut [session])?;
    Ok(Emit::default())
}
