#!/usr/bin/env python3
"""🧩️ S19 one-off codemod (set `flow-extensions`, guest side): the flow editor RECEIVES the host's `flow.extension`
closure. Measured root cause of S17 2c: `s.flow.flow#editor` declared no `setContributions`, so the shell skipped it
(`app-owns-no-setContributions`) and its registry — the only source of every extension operator its catalogue lists
and its evaluation resolves — stayed empty. Adds the flow twin of generation2d's retained contributions route (raw
bound = the command ingress's `COMMAND_MAXIMUM_BYTES`, host-only, installs into the process-wide flow registry and
invalidates the instance's retained evaluation session, which `pending_effects` re-arms on the next refresh), its law,
and the `flow.extension` consumption of procedural + demonstrator (the host scopes packs by `consumes` alone once the
operator-reachability cut is gone). Idempotent. usage: s19-flow-extensions.py <root>
"""
import os
import sys

root = sys.argv[1]
ARTIFACT = "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow"
EDITOR = f"{ARTIFACT}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
COMMAND_DIR = f"{EDITOR}/🎮️commands/🧩️set-contributions"


def path(rel):
    return os.path.join(root, rel)


def edit(rel, change):
    before = open(path(rel), encoding="utf-8").read()
    after = change(before)
    if after != before:
        open(path(rel), "w", encoding="utf-8").write(after)
        print(f"edited {rel}")


def create(rel, text):
    if os.path.exists(path(rel)):
        if open(path(rel), encoding="utf-8").read() != text:
            raise SystemExit(f"{rel} exists with other content")
        return
    os.makedirs(os.path.dirname(path(rel)), exist_ok=True)
    open(path(rel), "w", encoding="utf-8").write(text)
    print(f"created {rel}")


def replace_once(old, new, marker=None):
    def change(text):
        if (marker or new) in text:
            return text
        assert text.count(old) == 1, f"anchor count {text.count(old)}: {old[:80]!r}"
        return text.replace(old, new)
    return change


create(f"{COMMAND_DIR}/🦀️.rs", '''//! 🧩️ Flow play app command — `set-contributions`: the host's `flow.extension` closure, installed into the
//! plugin's process-wide flow extension registry, the one source of every extension operator the catalogue
//! lists and every evaluation resolves.

use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧩️ One page of the shell's contributions pack — the flow twin of generation2d's row. The shell pushes
/// the pack whole as page 0 of 1 over the pack-encoded command ingress (bounded by `COMMAND_MAXIMUM_BYTES`);
/// `page`/`page_count` keep the registry's page-run addressing so a multi-page run assembles the same closure.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-contributions")]
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
''')

create(f"{COMMAND_DIR}/🧪️tests/🔬️unit/🦀️.rs", '''use super::*;
use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView, NoConfig};

const MANIFEST_ADMISSION_FIXTURE: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧫️fixtures/🔣️manifest-admission.json");

fn contributions(row_id: &str) -> (String, Vec<String>) {
    let fixture: serde_json::Value = serde_json::from_str(MANIFEST_ADMISSION_FIXTURE).expect("manifest admission fixture");
    let row = fixture["rows"].as_array().expect("rows").iter().find(|row| row["id"] == row_id).expect("fixture row");
    let operators = row["operators"].as_array().expect("operators").iter().map(|operator| operator.as_str().expect("operator id").to_string()).collect();
    let json = serde_json::json!([{ "pluginId": fixture["pluginId"], "topicContribution": { "topic": "flow.extension", "payload": { "manifestJson": row["manifestJson"] } } }]).to_string();
    (json, operators)
}

/// ⚖️ LAW: the flow editor's own `setContributions` installs the pushed closure into the registry its
/// catalogue and evaluation read, publishes no store lane (no History row, no config ledger entry) and
/// invalidates the session exactly once per registry generation.
#[test]
fn a_pushed_flow_extension_closure_reaches_the_flow_editor_registry_without_a_store_lane() {
    let (json, operators) = contributions("valid-full");
    assert!(!operators.is_empty(), "the fixture row must contribute an operator, else the law proves nothing");
    let snapshot = FlowSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &NoConfig {}, window: None };
    let mut session = FlowEvalSession::new();
    let emit = handle(&SetContributions { json: json.clone(), page: 0, page_count: 1 }, &doc, &cfg, &mut session).expect("a well-formed closure is admitted");
    assert!(emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty() && emit.draft_mutations.is_empty() && emit.child_emits.is_empty() && emit.effects.is_empty(), "a host contributions push publishes no store lane and no effect");
    let catalogue: serde_json::Value = serde_json::from_str(&flow::flow_neuron_kind_infos_json()).expect("operator catalogue JSON");
    let ids: Vec<&str> = catalogue.as_array().expect("catalogue array").iter().filter_map(|item| item["id"].as_str()).collect();
    for operator in &operators {
        assert!(ids.contains(&operator.as_str()), "contributed operator {operator} must be in the flow registry after setContributions; catalogue={ids:?}");
    }
    assert!(!install(&SetContributions { json, page: 0, page_count: 1 }, &mut session).expect("an unchanged re-push is admitted"), "an unchanged closure keeps the registry generation and owes no re-evaluation");
    flow::uninstall_flow_extension("admission").expect("the law leaves the process-wide registry as it found it");
    session.retire_cold();
}
''')

edit(f"{ARTIFACT}/🦀️.rs", replace_once(
    '''            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️toggle-extension/🦀️.rs"]
            pub mod toggle_extension;
''',
    '''            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️toggle-extension/🦀️.rs"]
            pub mod toggle_extension;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
'''))

E = f"{EDITOR}/🦀️.rs"
edit(E, replace_once("    set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension,\n};", "    set_contributions, set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension,\n};"))
edit(E, replace_once(
    '''        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
    }''',
    '''        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "setContributions" as "set-contributions" => set_contributions::SetContributions,
    }'''))
edit(E, replace_once(
    '''                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
            })),
            other => Err(''',
    '''                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
            })),
            "setContributions" => Ok(FlowCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json"]).unwrap_or_default(),
                page: u64_arg(&["page"]).unwrap_or_default(),
                page_count: u64_arg(&["pageCount", "page_count"]).unwrap_or(1),
            })),
            other => Err(''', marker='"setContributions" => Ok(FlowCommand::SetContributions'))

ROUTE = '''//#region 🧩️ContributionsRoute
/// 🧩️ The host→guest contributions route — the flow twin of generation2d's: the shell's `flow.extension`
/// closure is what makes this app's extension operators exist at all (`install_builtin_flow_extensions`
/// installs none).
const FLOW_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];
const FLOW_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "flow.contributions-command.v1";
/// 📐️ The real wire ceiling of one contributions push: the paged command ingress's assembled-command bound,
/// exactly as generation2d/generation3d declare it — the unscoped nine-extension closure (293 642 characters,
/// `flow-extension-brep` alone 190 656) crosses whole.
const FLOW_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::kernel::COMMAND_MAXIMUM_BYTES;

fn flow_contributions_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(FLOW_CONTRIBUTIONS_RAW_BYTES, 256, 1, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page against the app instance's retained evaluation session.
struct FlowContributionsWork {
    instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    completed: bool,
    closing: bool,
}

impl FlowContributionsWork {
    fn new(instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { instance_owner: Some(instance_owner), completed: false, closing: false }
    }
}

impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
    }

    fn extent(
        &self,
        command: &FlowCommand,
        _snapshot: &FlowSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        (!self.closing && !self.completed && matches!(command, FlowCommand::SetContributions(payload) if payload.json.len() <= FLOW_CONTRIBUTIONS_RAW_BYTES)).then_some(1)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, Fault> {
        if self.closing || self.completed {
            return Err(Fault::from("flow-contributions-work-terminal"));
        }
        let FlowCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("flow-contributions-route-rejected"));
        };
        let instance_owner = self.instance_owner.as_ref().ok_or_else(|| Fault::from("flow-contributions-instance-owner"))?;
        instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| owner.with_session(|session| set_contributions::install(payload, session))?)?;
        self.completed = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit::default()))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if self.instance_owner.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.instance_owner.is_none()
    }
}

struct FlowContributionsJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowContributionsJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_CONTRIBUTIONS_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FlowContributionsJobFactory {
    type Payload = ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<FlowPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_CONTRIBUTIONS_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        flow_contributions_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FLOW_CONTRIBUTIONS_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("Flow contributions command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowContributionsJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] =
        &[semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] }];
}

struct FlowContributionsJobFactoryProofs;

impl FlowContributionsJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "flow.host_snapshot",
        factory: "FlowContributionsJobFactory",
        factory_type: FlowContributionsJobFactory,
        contract: flow_contributions_contract(),
        tools: ["setContributions"]
    }
}
//#endregion 🧩️ContributionsRoute

//#region 🔖️FlowPlayApp
struct FlowInstanceOperationOwner {'''
edit(E, replace_once("//#region 🔖️FlowPlayApp\nstruct FlowInstanceOperationOwner {", ROUTE, marker="//#region 🧩️ContributionsRoute"))
edit(E, replace_once(
    "        proofs.extend(FlowGraphOperationJobFactoryProofs::bounded_first_step_tool_proofs());\n        proofs\n",
    "        proofs.extend(FlowGraphOperationJobFactoryProofs::bounded_first_step_tool_proofs());\n        proofs.extend(FlowContributionsJobFactoryProofs::bounded_first_step_tool_proofs());\n        proofs\n"))
edit(E, replace_once(
    "        registry.register(FlowGraphOperationJobFactory::new(&controller))?;\n        registry.register(FlowDirectStoreJobFactory::new(&controller))\n",
    "        registry.register(FlowGraphOperationJobFactory::new(&controller))?;\n        registry.register(FlowContributionsJobFactory::new(&controller))?;\n        registry.register(FlowDirectStoreJobFactory::new(&controller))\n"))
edit(E, replace_once(
    '''            && !FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str())
        {
            return Ok(None);
        }''',
    '''            && !FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str())
            && !FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&request.tool_id.as_str())
        {
            return Ok(None);
        }'''))
edit(E, replace_once(
    '''        if FLOW_CHILD_GROUP_TOOL_IDS.contains(&request.tool_id.as_str()) || FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&request.tool_id.as_str()) || FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            let tool_id = request.command.command_id();
            let work: Box<dyn ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {''',
    '''        if FLOW_CHILD_GROUP_TOOL_IDS.contains(&request.tool_id.as_str())
            || FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&request.tool_id.as_str())
            || FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str())
            || FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&request.tool_id.as_str())
        {
            let tool_id = request.command.command_id();
            let work: Box<dyn ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = if FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id) {
                Box::new(FlowContributionsWork::new(request.instance_operation_owner))
            } else if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {'''))
edit(E, replace_once(
    '''                FlowCommand::command_id,
                if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {
                    FLOW_CHILD_GROUP_RAW_BYTES
                }''',
    '''                FlowCommand::command_id,
                if FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id) {
                    FLOW_CONTRIBUTIONS_RAW_BYTES
                } else if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {
                    FLOW_CHILD_GROUP_RAW_BYTES
                }'''))
edit(E, replace_once(
    '''                if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {
                    1
                } else if FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&tool_id) {''',
    '''                if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) || FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id) {
                    1
                } else if FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&tool_id) {'''))
edit(E, replace_once(
    '''        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung auflösen"), "runtime", ActionKind::View) })
''',
    '''        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung auflösen"), "runtime", ActionKind::View) })
        .command(CommandDefinition {
            in_palette: false,
            ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([
                ActionArgDef::text("json", LocalizedLabel::native("Contributions Page", "Beiträge-Seite")),
                ActionArgDef::text("page", LocalizedLabel::native("Page", "Seite")),
                ActionArgDef::text("pageCount", LocalizedLabel::native("Page Count", "Seitenanzahl")),
            ])
        })
'''))
edit(E, replace_once(
    '''        .action_interactive_job("flowEvalResolve", semio_framework_plugin::InteractiveJobClassification::Migrated)
''',
    '''        .action_interactive_job("flowEvalResolve", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setContributions", semio_framework_plugin::InteractiveJobClassification::Migrated)
'''))

edit("✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml", replace_once('consumes = ["forms.questionKind"]', 'consumes = ["forms.questionKind", "flow.extension"]'))
edit("✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml", replace_once('consumes = ["forms.questionKind", "process.machines", "cad.computer", "sourcing.module"]', 'consumes = ["forms.questionKind", "process.machines", "cad.computer", "sourcing.module", "flow.extension"]'))
