//! 🧩️ Flow play app command — `set-contributions`: the host's `flow.extension` closure, installed into the
//! plugin's process-wide flow extension registry, the one source of every extension operator the catalogue
//! lists and every evaluation resolves.

use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧩️ One page of the shell's contributions pack — the flow twin of generation2d's row. The shell pushes
/// the pack whole as page 0 of 1 over the pack-encoded command ingress (bounded by `COMMAND_MAXIMUM_BYTES`);
/// `page`/`page_count` keep the registry's page-run addressing so a multi-page run assembles the same closure.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-contributions")]
#[derive(semio_framework_value::RetireOwned)]
pub struct SetContributions {
    pub json: String,
    pub page: u64,
    pub page_count: u64,
}

/// 🧩️ Buffers this page and, on the run's last one, installs the assembled closure into the flow extension
/// registry and invalidates the evaluation an empty registry had already faulted; the next host refresh
/// re-arms it through `FlowPlayApp::pending_effects`. Answers whether the session was invalidated. The key is
/// the registry generation, so a re-push of an unchanged closure owes nothing.
pub fn install(payload: &SetContributions, session: &mut FlowEvalSession) -> Result<bool, Fault> {
    let page = u32::try_from(payload.page).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    let page_count = u32::try_from(payload.page_count).map_err(|_| Fault::from("flow.contributions-page-address-invalid"))?;
    flow::sync_host_flow_extension_contributions_page(page, page_count, &payload.json).map_err(Fault::from)?;
    Ok(session.invalidate_for_flow_extension_registry(flow::flow_extension_registry_generation()))
}

/// 🧩️ The `app_commands!` row: installs the page against the session it is handed. The served route
/// (`FlowContributionsWork::step`) installs against the instance's retained session.
pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    install(payload, session)?;
    Ok(Emit::default())
}
