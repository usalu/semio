//! 🎛️ Norm plugin — the app-surface machinery every one of the fifteen compliance apps shares.
//!
//! 📌️ The fifteen norm apps are structurally identical by construction (one `edit` mode, an
//! inputs/results window pair, the framework document/catalogue/inspection panel trio, the same
//! `model:in`/`report:out` media ports, the same three commands) and differ only in their per-standard
//! `Document` type, ids and labels. Everything that does NOT vary lives here, ONCE; every taxonomy node
//! under each subset's `✏️editor/`/`👁️viewer/` states only what genuinely varies and calls into this module. That is the
//! "shared declarations belong at the shallowest common ancestor" rule taken to its conclusion — the
//! shallowest common ancestor of fifteen sibling apps is the plugin's own `🫀️core`.
//!
//! Nothing here depends on any app or artifact module: every entry point is either a plain constructor
//! or generic over the artifact's `Document`/`NormFamily`, so `🫀️core` stays a leaf of the dependency
//! graph exactly as the artifacts require.

use crate::document::{CheckReport, NormFamily, NormHost};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasChildren};
use semio_framework::ToolExecutionContract;
use semio_framework_plugin::{
    AppIo, ArtifactKindSpec, ArtifactPresentation, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, BuiltNode, ConfigView, Emit, Fault, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaPortDirection, MediaPortSpec, MediaType, ModeDefinition, OsMediaCapability,
    PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, PortMultiplicity, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowOptions,
};

//#region 🔖️Ids
/// 🆔️ The single mode every norm app's editor declares.
pub const MODE_EDIT: &str = "edit";
/// 🆔️ The single mode every norm app's viewer declares (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET).
pub const MODE_VIEW: &str = "view";
//#endregion 🔖️Ids

//#region 🔖️ViewerManifest
/// ✏️ The `view` mode definition — identical for all fifteen viewers, the read-only counterpart of
/// `edit_mode_definition`.
pub fn view_mode_definition() -> ModeDefinition {
    ModeDefinition { id: MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane window layout — every norm viewer has exactly one window (the compliance
/// report table), so there is no quadrant/split layout to allocate.
pub fn single_window_layout(window_kind_id: &str, title: &str) -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}

/// 📊️ `TableWindowKit` column headers for a norm `CheckReport` — shared by all fifteen viewers'
/// report windows so the table shape is declared exactly once.
pub fn report_table_columns() -> Vec<String> {
    vec!["Clause".into(), "Status".into(), "Utilization".into(), "Message".into()]
}

/// 📊️ `TableWindowKit` rows for a norm `CheckReport` — one row per computed check, columns matching
/// `report_table_columns`.
pub fn report_table_rows(report: &CheckReport) -> Vec<Vec<String>> {
    report.checks.iter().map(|check| vec![check.clause.to_string(), format!("{:?}", check.status), format!("{:.2}", check.utilization), check.message.clone()]).collect()
}
//#endregion 🔖️ViewerManifest

//#region 🔖️Render
fn render_text(value: impl Into<String>) -> UiAssemblyResult<BuiltNode> {
    let label = ui::Label::try_from(value.into()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI label admission failed"))?;
    ui::text(label).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text build failed"))
}

/// 📑️ Renders a whole `CheckReport` as one line per computed check.
pub fn render_report(report: &CheckReport) -> UiAssemblyResult<BuiltNode> {
    if report.checks.is_empty() {
        return render_text("No checks computed.");
    }
    let children = report.checks.iter().enumerate().map(|(index, check)| render_text(format!("{}. {} — {:?} u={:.2} — {}", index + 1, check.clause, check.status, check.utilization, check.message))).collect::<UiAssemblyResult<Vec<_>>>()?;
    ui::column()
        .try_children(children)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm report children admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm report build failed"))
}

/// 📄️ Renders a document as pretty-printed JSON — the inputs window's surface.
pub fn render_document_json<D: dsl::ToValue>(document: &D) -> UiAssemblyResult<BuiltNode> {
    let json = pack::json::to_string_pretty(&pack::json::from_dsl_value(&dsl::ToValue::to_value(document)));
    render_text(json)
}

/// 🧾️ Renders a one-line headline for a family's current session — the document panel's surface.
pub fn render_summary<F: NormFamily>(host: &NormHost<F>) -> UiAssemblyResult<BuiltNode> {
    let report = host.report();
    render_text(format!("{} — {} checks, worst u={:.2}, all pass={}", F::family_id().label(), report.checks.len(), report.worst_utilization(), report.all_pass()))
}

/// 📚️ Renders the catalogue panel's placeholder headline for a family.
pub fn render_catalogue(label: &str) -> UiAssemblyResult<BuiltNode> {
    render_text(format!("{label} catalogue"))
}

/// 🔍️ Renders the inspection panel — the `selected_check_index` row of the report, falling back to the
/// first check when the index is unset or out of range (and to a placeholder when there are no checks).
pub fn render_inspection(report: &CheckReport, selected_check_index: Option<u32>) -> UiAssemblyResult<BuiltNode> {
    let checks = &report.checks;
    let index = selected_check_index.map(|value| value as usize).filter(|index| *index < checks.len()).unwrap_or(0);
    match checks.get(index) {
        Some(check) => render_text(format!("{check:?}")),
        None => render_text("No checks"),
    }
}

/// ❓️ The unknown-body-key fallback every norm app's `render` ends with.
pub fn render_unknown_body(body_key: &str) -> UiAssemblyResult<BuiltNode> {
    render_text(format!("Unknown body: {body_key}"))
}
//#endregion 🔖️Render

//#region 🔖️Manifest
/// ✏️ The `edit` mode definition — identical for all fifteen apps.
pub fn edit_mode_definition() -> ModeDefinition {
    ModeDefinition { id: MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A norm window kind — both windows of every app are plain `Canvas2d` surfaces with no measures,
/// engagement, actions or utilities, so only id/label/body/icon vary.
pub fn window_definition(id: &str, label: LocalizedLabel, body_key: &str, icon_id: &str) -> WindowKindDefinition {
    WindowKindDefinition {
        id: id.into(),
        label,
        body_key: body_key.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: icon_id.into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}

/// 📌️ A norm panel tab — every one is a framework-predefined leaf id bound to this app's body key.
/// Byte-identical to the `AppBuilder::panel_tab(id, label, group, body_key)` scalar call it replaces
/// (`PanelTabSpec::leaf` builds exactly this shape).
pub fn panel_definition(id: &str, label: LocalizedLabel, group: PanelGroup, body_key: &str) -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(id.into()), label, group, body_key: Some(body_key.into()), children: Vec::new() }
}

/// 🗿️ A norm artifact kind — Data × Value document per owner-table (IO coverage lattice).
pub fn artifact_kind_spec(variant: &str, label: &str) -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: artifact_kind_id(variant),
        name: label.into(),
        source_format: format!("norm.{variant}.document"),
        component_kind: "norm".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: format!("norm.{variant}.document"),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}

/// 🆔️ `computation.norm.<variant>` — the artifact kind id `report:out` pins itself to.
pub fn artifact_kind_id(variant: &str) -> String {
    format!("computation.norm.{variant}")
}

/// 🔌️ Every norm family's typed media I/O surface — the implicit `document:in`/`document:out` pair
/// (auto-injected by `AppIo::all_ports`) plus the two extra workflow ports every norm app gets:
/// `model:in` (a generic upstream-model input — an honest pass-through, no family `Document` shape has
/// a generic "raw model" field to receive one into yet) and `report:out` (the computed `CheckReport`,
/// pinned to this family's own already-declared `computation.norm.{variant}` artifact kind via
/// `kind_id`). One function serves both the builder's `.io(...)` declaration and each app's
/// `ArtifactApp::io` override, so the two never drift apart.
pub fn norm_io(variant: &str, document_schema: &str) -> AppIo {
    let artifact_kind_id = artifact_kind_id(variant);
    AppIo {
        document_schema: document_schema.into(),
        document_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        ports: vec![
            MediaPortSpec {
                id: "model:in".into(),
                label: "Model".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                kind_id: None,
                required: false,
                multiplicity: PortMultiplicity::One,
            },
            MediaPortSpec {
                id: "report:out".into(),
                label: "Report".into(),
                direction: MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
                kind_id: Some(artifact_kind_id.clone()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: ArtifactPresentation { id: artifact_kind_id, name: variant.into(), dimension: "data".into(), component_kind: "norm".into() },
    }
}
//#endregion 🔖️Manifest

//#region 🔖️MediaPorts
/// 🎞️ `"report:out"` dumps the currently computed `CheckReport`, pinned to this family's declared
/// artifact kind; `"document:out"` replicates the SDK default (whole-document pack) since overriding
/// `export_media` shadows it entirely. Any other port is `NotImplemented`.
pub fn export_media<F>(port: &str, variant: &str, document_schema: &str, document: &F::Document) -> Result<Media, MediaError>
where
    F: NormFamily,
    F::Document: store::ArtifactPack,
{
    if port == "report:out" {
        let host = NormHost::<F>::from_document(document.clone());
        let json = pack::json::to_json_string(host.report());
        return Ok(Media { media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: artifact_kind_id(variant), json } });
    }
    if port != "document:out" {
        return Err(MediaError::NotImplemented);
    }
    let bytes = store::ArtifactPack::encode_pack(document);
    Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: document_schema.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
}

/// 🎞️ `"model:in"` is an honest generic pass-through: a payload that happens to decode as this family's
/// own `Document` shape becomes a bundle of targeted `change-<field>` mutations (one per persistent
/// field, via each migrated facet's `XMutation::from_snapshot`) rather than a single whole-document
/// replace mutation — the banned whole-document-replace escape hatch has no 1:1 replacement, so `wrap` now
/// decomposes the imported document into the closed semantic vocabulary instead. Bundling them into one
/// `Emit::mutations` call keeps the import atomic (one edit, one undo entry), matching the old
/// single-mutation commit's history shape. Anything that doesn't decode is accepted but inert (no norm
/// family document has a generic "raw model" field to stash a foreign shape into yet). `"document:in"`
/// replicates the SDK default (decodes the base64 pack).
pub fn import_media<D, M, F>(port: &str, media: &Media, wrap: F) -> Result<Emit<M, crate::config::NormConfigMutation>, MediaError>
where
    D: Clone + Default + PartialEq + dsl::ToValue + dsl::FromValue + store::ArtifactPack,
    F: Fn(D) -> Vec<M>,
{
    if port == "model:in" {
        if let MediaPayload::Structured { json, .. } = &media.payload {
            if let Ok(document) = pack::json::from_json_str::<D>(json) {
                return Ok(Emit::mutations(wrap(document)));
            }
        }
        return Ok(Emit::default());
    }
    if port != "document:in" {
        return Err(MediaError::NotImplemented);
    }
    let MediaPayload::Structured { json, .. } = &media.payload else {
        return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
    };
    let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
    let document = <D as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
    Ok(Emit::mutations(wrap(document)))
}
//#endregion 🔖️MediaPorts

//#region 🔖️Commands
/// 📤️ The whole-document replace every app's `set-document` and `evaluate` commands emit — `description`
/// is the manifest action id the command was declared under, which the command log labels the edit with.

/// 📤️ Commit a typed document mutation (kept for the norm sub-lane's not-yet-migrated sibling facets;
/// migrated facets use `commit_snapshot_fields` below instead, since the whole-document-replace
/// variant this helper used to construct is banned with no 1:1 replacement).
pub fn commit_snapshot<M>(mutation: M, description: &str) -> Result<Emit<M, crate::config::NormConfigMutation>, Fault> {
    Ok(Emit::commit(vec![mutation], description))
}

/// 📤️ Commit a bundle of targeted semantic mutations as one described edit — the migrated facets'
/// replacement for `commit_snapshot`'s old single whole-document-replace commit: a `set-snapshot`
/// command payload (or a re-evaluation re-commit) decomposes into one `change-<field>` mutation per
/// persistent field via `XMutation::from_snapshot`, bundled here into a single undo entry.
pub fn commit_snapshot_fields<M>(mutations: Vec<M>, description: &str) -> Result<Emit<M, crate::config::NormConfigMutation>, Fault> {
    Ok(Emit::commit(mutations, description))
}

/// ☑️ The one config-only edit every app's `selected-check` command emits.
pub fn commit_selected_check_index<M>(index: Option<u32>) -> Result<Emit<M, crate::config::NormConfigMutation>, Fault> {
    Ok(Emit::config(vec![crate::config::ChangeSelectedCheckIndex { index }.into()]))
}

/// 🎯️ Builds the args-side of an app's `command_from_action` bridge for `selected-check` — the shells
/// still speak `{action,args}` for chrome actions.
pub fn selected_check_index_arg(args: Option<&dsl::DslValue>) -> Option<u32> {
    args.and_then(|value| value.get("index")).and_then(dsl::DslValue::as_u64).map(|value| value as u32)
}
//#endregion 🔖️Commands

//#region 🔖️Views
/// 👁️ Reads the config's selected check index out of a `ConfigView` — the one field norm apps read.
pub fn selected_check_index(cfg: &ConfigView<'_, crate::config::NormConfig>) -> Option<u32> {
    cfg.snapshot.selected_check_index
}

/// 📄️ Reads the document out of a `ArtifactView` — spelled once so every app's `render`/`handle` reads
/// it the same way.
pub fn snapshot<'a, D>(doc: &'a ArtifactView<'_, D>) -> &'a D {
    doc.snapshot
}
//#endregion 🔖️Views

//#region 🧵️RetainedCommands
/// 🧾️ Every norm tool id, in `app_commands!` row order. All fifteen apps declare exactly this set, so
/// the list, [`NORM_PUBLICATION_CONTRACTS`], every factory key set and every `bounded_first_step_tool_proofs!`
/// block are driven from this one constant.
pub const NORM_RETAINED_TOOL_IDS: &[&str] = &["setSnapshot", "evaluate", "setSelectedCheckIndex"];
/// 🧬️ The payload schema id every norm retained command job is admitted under.
pub const NORM_RETAINED_PAYLOAD_SCHEMA: &str = "norm.tool-command.v1";
/// 🎒️ Wire ceiling for one norm tool dispatch: the largest payload is `setSnapshot`'s whole compliance
/// document, a few dozen scalar quantities plus an ordered layer list — kilobytes, never megabytes.
pub const NORM_RETAINED_RAW_BYTES: usize = 8_192;
/// 🎒️ Real bound for one Artifact-lane edit: a single `change-<field>`/`insert-layer`/`remove-layer`
/// leaf, the only artifact mutations any norm command emits.
pub const NORM_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;
/// 🎒️ Real bound for one Config-lane edit: `NormConfigMutation` carries a single `Option<u32>` index.
pub const NORM_CONFIG_STORE_MAXIMUM_BYTES: usize = 4_096;

/// 🚦️ Per-tool publication lanes, read straight off the three command bodies: `set-snapshot` commits
/// artifact mutations, `evaluate` emits nothing at all (the report is derived on every read), and
/// `selected-check` writes view state through the config lane.
pub const NORM_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setSnapshot", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "evaluate", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setSelectedCheckIndex", lanes: &[ArtifactToolPublicationLane::Config] },
];

/// ⏱️ The one bounded-first-step contract all forty-five norm tool identities share.
pub fn norm_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(NORM_RETAINED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

/// 🧵️ The per-app half of the shared factory: an editor states only how its own aggregated command enum
/// dispatches, and inherits every retained-command constant, reducer, factory and store preparation
/// below. `dispatch_retained` MUST route into the app's `🎮️commands/*` bodies, which stay the sole
/// authority for what a norm command does.
pub trait NormRetainedEditor: semio_framework_plugin::ArtifactEditor<Config = crate::config::NormConfig, ConfigMutation = crate::config::NormConfigMutation, DraftMutation = semio_framework_plugin::NoDraftMutation> {
    fn dispatch_retained(command: &Self::Command, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault>;
}

/// 🧵️ The retained reducer shared by all fifteen apps — no norm command reads selection or hover, so the
/// interaction owners are unused and the reduction is exactly the ordinary `handle` path.
///
/// 🔁️ `artifact_mutations` is reversed on the way out because the retained publication lane drains the
/// bundle LIFO (one `begin_apply_one` per `Vec::pop`, one store edit each), whereas `Emit::commit`'s
/// ordinary dispatch applies the same vector front-to-back inside one edit. `XMutation::from_snapshot`
/// emits ordered `remove-layer`/`insert-layer` runs, so handing the bundle back-to-front is what makes
/// the published document identical to the one the ordinary path produces.
pub fn norm_retained_reduce<A: NormRetainedEditor>(
    command: &A::Command,
    snapshot: &A::Snapshot,
    config: &A::Config,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, Fault> {
    if !NORM_RETAINED_TOOL_IDS.contains(&A::command_id(command)) {
        return Err(Fault::from("norm-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let mut emit = A::dispatch_retained(command, &doc, &ConfigView { snapshot: config })?;
    emit.artifact_mutations.reverse();
    Ok(emit)
}

/// 📏️ Every norm command is one bounded step; none of the three walks a collection incrementally.
pub fn norm_bounded_extent<A: NormRetainedEditor>(_command: &A::Command, _snapshot: &A::Snapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 🏭️ The one owned bounded tool-job factory serving all fifteen norm editors — generic over the app, so
/// the per-standard `Snapshot`/`Mutation`/`Command` types are the only thing that varies and the owner
/// witness stays each app's own concrete `EditorApp<A>`.
pub struct NormBoundedCommandJobFactory<A: NormRetainedEditor> {
    keys: Vec<semio_framework::ToolFactoryKey>,
    owner: std::marker::PhantomData<fn() -> A>,
}

impl<A: NormRetainedEditor> NormBoundedCommandJobFactory<A> {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: NORM_RETAINED_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect(), owner: std::marker::PhantomData }
    }
}

impl<A: NormRetainedEditor> semio_framework::ToolJobFactory for NormBoundedCommandJobFactory<A> {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<A>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<A>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        NORM_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework_plugin::InteractiveJobClassification {
        semio_framework_plugin::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        norm_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > NORM_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("norm retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl<A: NormRetainedEditor> semio_framework_plugin::ArtifactOwnedToolJobFactory for NormBoundedCommandJobFactory<A> {
    type Owner = semio_framework_plugin::EditorApp<A>;
    const TOOL_IDS: &'static [&'static str] = NORM_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = <A as semio_framework_plugin::ArtifactEditor>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = NORM_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
/// 🧬️ Builds one `protocol::Edit<M>` for either lane's `advance()` — the artifact and config lanes differ
/// only in `M`, their id prefix and their byte ceiling, so one generic preparation serves both.
fn norm_next_edit<M>(prefix: &'static str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

fn norm_mutation_retained_bytes<M: ::protocol::OpBinary>(mutation: &M) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "norm-mutation-encode-failed".to_string())
}

fn admit_norm_mutation<M: ::protocol::OpBinary>(mutation: &M, maximum_bytes: usize) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = norm_mutation_retained_bytes(mutation)?;
    if retained_bytes > maximum_bytes {
        return Err("norm-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_norm_one_item<P, M>(base: &P, mutation: M, maximum_bytes: usize) -> Result<(P, Vec<M>, M), String>
where
    M: ::protocol::Mutation<P> + ::protocol::OpBinary,
{
    admit_norm_mutation(&mutation, maximum_bytes)?;
    let inverse = ::protocol::Mutation::inverse(&mutation, base);
    let diff = ::protocol::Mutation::diff(&mutation, base).into_parts().0;
    let post = ::protocol::MutationDiff::apply(&diff, base).map_err(|_| "norm-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

/// 🏭️ The exact one-item Store preparation authority both norm lanes need — the Artifact lane is
/// unavailable to a migrated tool without it (`interactive-job.publication-contract` at app construction),
/// and so is the Config lane.
pub struct NormOneItemPreparationFactory<P, M> {
    prefix: &'static str,
    maximum_bytes: usize,
    lane: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> NormOneItemPreparationFactory<P, M> {
    pub const fn new(prefix: &'static str, maximum_bytes: usize) -> Self {
        Self { prefix, maximum_bytes, lane: std::marker::PhantomData }
    }
}

struct NormOneItemPreparation<P, M> {
    prefix: &'static str,
    maximum_bytes: usize,
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for NormOneItemPreparationFactory<P, M>
where
    P: Send + Sync + 'static,
    M: ::protocol::Mutation<P> + ::protocol::OpBinary + Send + 'static,
{
    fn preflight(&self, mutation: &M, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("norm-lane-or-description-envelope".into());
        }
        admit_norm_mutation(mutation, self.maximum_bytes)
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        let retained_bytes = norm_mutation_retained_bytes(&request.mutation).unwrap_or(self.maximum_bytes.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > self.maximum_bytes
        {
            return Err(request);
        }
        Ok(Box::new(NormOneItemPreparation {
            prefix: self.prefix,
            maximum_bytes: self.maximum_bytes,
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for NormOneItemPreparation<P, M>
where
    P: Send + Sync + 'static,
    M: ::protocol::Mutation<P> + ::protocol::OpBinary + Send + 'static,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "norm-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "norm-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_norm_one_item(base.get(), mutation, self.maximum_bytes)?;
        let authority = self.authority.as_ref().ok_or_else(|| "norm-authority-missing".to_string())?;
        let edit = norm_next_edit(self.prefix, forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("norm-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

//#region 🔌️EditorOverrides
/// 📬️ `ArtifactEditor::build_artifact_store_one_item_preparation_factory` for every norm editor.
pub fn norm_artifact_store_preparation<A: NormRetainedEditor>() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>> {
    Some(std::sync::Arc::new(NormOneItemPreparationFactory::<A::Snapshot, A::Mutation>::new("norm-artifact-retained", NORM_ARTIFACT_STORE_MAXIMUM_BYTES)))
}

/// 📬️ `ArtifactEditor::build_config_store_one_item_preparation_factory` for every norm editor.
pub fn norm_config_store_preparation<A: NormRetainedEditor>() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<A::Config, A::ConfigMutation>>> {
    Some(std::sync::Arc::new(NormOneItemPreparationFactory::<A::Config, A::ConfigMutation>::new("norm-config-retained", NORM_CONFIG_STORE_MAXIMUM_BYTES)))
}

/// 🏭️ Declares one norm app's concrete owned factory as a newtype over the shared generic
/// [`NormBoundedCommandJobFactory`], plus its `register_tool_job_factories` entry point. The newtype is
/// required, not decorative: `ArtifactBoundedFirstStepProof` joins the `factory:` literal against the
/// last `::` segment of `std::any::type_name`, which for a generic instantiation is the owner app's own
/// name followed by `>`. Every behavioural line still lives once, in the generic base this delegates to.
#[macro_export]
macro_rules! norm_owned_tool_job_factory {
    ($factory:ident, $app:ty) => {
        pub struct $factory($crate::app_surface::NormBoundedCommandJobFactory<$app>);

        impl $factory {
            pub fn new(controller_id: &str) -> Self {
                Self($crate::app_surface::NormBoundedCommandJobFactory::<$app>::new(controller_id))
            }

            pub fn register(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<$app>>) -> Result<(), semio_framework_plugin::Fault> {
                let controller = registry.controller_id().to_string();
                registry.register(Self::new(&controller))
            }
        }

        impl semio_framework::ToolJobFactory for $factory {
            type Payload = <$crate::app_surface::NormBoundedCommandJobFactory<$app> as semio_framework::ToolJobFactory>::Payload;
            type Job = <$crate::app_surface::NormBoundedCommandJobFactory<$app> as semio_framework::ToolJobFactory>::Job;

            fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
                semio_framework::ToolJobFactory::keys(&self.0)
            }

            fn payload_schema_id(&self) -> &str {
                semio_framework::ToolJobFactory::payload_schema_id(&self.0)
            }

            fn classification(&self) -> semio_framework_plugin::InteractiveJobClassification {
                semio_framework::ToolJobFactory::classification(&self.0)
            }

            fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
                semio_framework::ToolJobFactory::execution_contract(&self.0)
            }

            fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
                semio_framework::ToolJobFactory::create_job(&mut self.0, operation, payload)
            }

            fn create_job_from_wire_pages_with_payload(
                &mut self,
                operation: semio_framework_job::Operation,
                payload: Self::Payload,
                input: semio_framework::action_bus::RetainedToolWireInput,
                checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
            ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
                semio_framework::ToolJobFactory::create_job_from_wire_pages_with_payload(&mut self.0, operation, payload, input, checkpoint)
            }
        }

        impl semio_framework_plugin::ArtifactOwnedToolJobFactory for $factory {
            type Owner = semio_framework_plugin::EditorApp<$app>;
            const TOOL_IDS: &'static [&'static str] = $crate::app_surface::NORM_RETAINED_TOOL_IDS;
            const DOCUMENT_SCHEMA: &'static str = <$app as semio_framework_plugin::ArtifactEditor>::DOCUMENT_SCHEMA;
            const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = $crate::app_surface::NORM_PUBLICATION_CONTRACTS;
        }
    };
}

/// 🧵️ `ArtifactEditor::build_tool_job` for every norm editor.
pub fn build_norm_tool_job<A: NormRetainedEditor>(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<A>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
    if !NORM_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
        return Ok(None);
    }
    let tool_id = A::command_id(&request.command);
    if tool_id != request.tool_id {
        return Err(Fault::from("norm-command-tool-mismatch"));
    }
    let work = Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, norm_retained_reduce::<A>, norm_bounded_extent::<A>));
    let operation = semio_framework_plugin::AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
    };
    let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new_with_context(
        *request.command,
        request.snapshot,
        request.config,
        request.history,
        request.interaction_state,
        request.interaction_hover,
        request.context,
        operation,
        request.completion,
        A::command_id,
        NORM_RETAINED_RAW_BYTES,
        1,
        work,
    )?;
    Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}
//#endregion 🔌️EditorOverrides

#[cfg(test)]
//#region 🧵️RetainedDispositionOracle
pub(crate) mod retained_disposition_oracle {
    /// 🧵️ Framework-neutral summary exposed by the owned oracle boundary.
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct NormRetainedDispositionSummary {
        pub(crate) app_count: usize,
        pub(crate) route_count: usize,
        pub(crate) retained_count: u64,
        pub(crate) batch_only_count: u64,
        pub(crate) publication_contract_count: usize,
    }

    /// 🧪️ Owned boundary hiding the test-only JSON implementation.
    pub(crate) trait NormRetainedDispositionOracle {
        fn summarize(&self, source: &str) -> Result<NormRetainedDispositionSummary, String>;
    }

    /// 🧪️ Third-party `serde_json` implementation kept behind the owned boundary.
    pub(crate) struct SerdeJsonNormRetainedDispositionOracle;

    impl NormRetainedDispositionOracle for SerdeJsonNormRetainedDispositionOracle {
        fn summarize(&self, source: &str) -> Result<NormRetainedDispositionSummary, String> {
            let value: serde_json::Value = serde_json::from_str(source).map_err(|error| error.to_string())?;
            let routes = value["routes"].as_array().ok_or("routes must be an array")?;
            let expected_ids = super::NORM_RETAINED_TOOL_IDS;
            if routes.len() != expected_ids.len() {
                return Err("route count must be exactly three".into());
            }
            for (route, expected_id) in routes.iter().zip(expected_ids) {
                if route["id"] != *expected_id || route["admission"] != "migrated" {
                    return Err(format!("invalid route disposition for {expected_id}"));
                }
            }
            if routes[0]["emittedLanes"] != serde_json::json!(["artifact"])
                || routes[1]["emittedLanes"] != serde_json::json!([])
                || routes[2]["emittedLanes"] != serde_json::json!(["config"])
            {
                return Err("route lane audit does not match the command bodies".into());
            }
            let apps = value["apps"].as_array().ok_or("apps must be an array")?;
            let publication_contracts = value["publicationContracts"].as_array().ok_or("publicationContracts must be an array")?;
            let declared = super::NORM_PUBLICATION_CONTRACTS.iter().map(|contract| serde_json::json!({ "toolId": contract.tool_id, "lanes": contract.lanes.iter().map(|lane| format!("{lane:?}")).collect::<Vec<_>>() })).collect::<Vec<_>>();
            if publication_contracts != &declared {
                return Err("publication contracts do not match the live factory declaration".into());
            }
            for (route, contract) in routes.iter().zip(publication_contracts) {
                if route["publicationLanes"] != contract["lanes"] {
                    return Err("route publication lanes diverge from the factory publication contract".into());
                }
            }
            if value["factory"]["payloadSchema"] != super::NORM_RETAINED_PAYLOAD_SCHEMA || value["factory"]["maximumRawBytes"] != serde_json::json!(super::NORM_RETAINED_RAW_BYTES) {
                return Err("factory identity does not match the live shared factory".into());
            }
            let summary = NormRetainedDispositionSummary {
                app_count: apps.len(),
                route_count: routes.len(),
                retained_count: value["expected"]["retained"].as_u64().ok_or("retained must be an integer")?,
                batch_only_count: value["expected"]["batchOnlyPendingRewrite"].as_u64().ok_or("batchOnlyPendingRewrite must be an integer")?,
                publication_contract_count: publication_contracts.len(),
            };
            if summary.app_count != 15 || summary.retained_count != 45 || summary.batch_only_count != 0 || summary.publication_contract_count != 3 {
                return Err("cohort totals or publication contracts are not fully migrated".into());
            }
            Ok(summary)
        }
    }

    /// 🧪️ Pins the canonical fixture and rejects forged admission, lane, and publication claims.
    pub(crate) fn assert_fixture(variant: &str) {
        let source = include_str!("../🧪️fixtures/🧫️retained-command-dispositions/🔣️.json");
        let oracle = SerdeJsonNormRetainedDispositionOracle;
        let summary = oracle.summarize(source).expect("canonical Norm retained disposition fixture");
        assert_eq!(summary, NormRetainedDispositionSummary { app_count: 15, route_count: 3, retained_count: 45, batch_only_count: 0, publication_contract_count: 3 });
        let canonical: serde_json::Value = serde_json::from_str(source).expect("canonical fixture JSON");
        assert!(canonical["apps"].as_array().expect("apps").iter().any(|app| app["variant"] == variant), "missing Norm app fixture row for {variant}");

        let mut forged_admission = canonical.clone();
        forged_admission["routes"][0]["admission"] = serde_json::json!("batchOnlyPendingRewrite");
        assert!(oracle.summarize(&forged_admission.to_string()).is_err());
        let mut forged_lane = canonical.clone();
        forged_lane["routes"][2]["emittedLanes"] = serde_json::json!(["host"]);
        assert!(oracle.summarize(&forged_lane.to_string()).is_err());
        let mut forged_publication = canonical.clone();
        forged_publication["publicationContracts"][1]["lanes"] = serde_json::json!(["Artifact"]);
        assert!(oracle.summarize(&forged_publication.to_string()).is_err());
        let mut forged_factory = canonical;
        forged_factory["factory"]["payloadSchema"] = serde_json::json!("norm.tool-command.v2");
        assert!(oracle.summarize(&forged_factory.to_string()).is_err());
    }
}
//#endregion 🧵️RetainedDispositionOracle

//#region 🧪️Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[semio_framework_async_macros::async_test]
    fn the_edit_mode_is_the_same_for_every_app() {
        let mode = edit_mode_definition();
        assert_eq!(mode.id, MODE_EDIT);
        assert!(mode.tools.is_empty() && mode.commands.is_empty() && mode.layout_id.is_none());
    }

    #[semio_framework_async_macros::async_test]
    fn a_window_definition_is_a_plain_canvas2d_surface() {
        let window = window_definition("norm-x-inputs", LocalizedLabel::native("Inputs", "Eingaben"), "norm.x.play.inputs", "download");
        assert_eq!(window.body_key, "norm.x.play.inputs");
        assert!(matches!(window.surface_kind, SurfaceKind::Canvas2d));
        assert!(window.actions.is_empty() && window.utilities.is_empty() && window.options.measures.is_empty());
    }

    /// 📌️ Proves `panel_definition` reproduces the scalar `AppBuilder::panel_tab` shape exactly (an
    /// `App`-kind leaf carrying the body key) — the property that keeps the manifest byte-identical
    /// after the panel declarations moved into `📌️panels/*` nodes.
    #[semio_framework_async_macros::async_test]
    fn a_panel_definition_is_an_app_kind_leaf_carrying_its_body_key() {
        let panel = panel_definition("document", LocalizedLabel::native("Document", "Dokument"), PanelGroup::Workbench, "norm.x.play.document");
        assert!(matches!(&panel.kind, PanelTabKind::App(id) if id == "document"));
        assert_eq!(panel.body_key.as_deref(), Some("norm.x.play.document"));
        assert!(panel.children.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    fn norm_io_declares_model_in_and_report_out_beside_the_implicit_document_ports() {
        let io = norm_io("din4108", "semio.norm.din4108/v1");
        assert!(io.ports.iter().any(|port| port.id == "model:in" && port.direction == MediaPortDirection::In));
        let report_out = io.ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
        assert_eq!(report_out.direction, MediaPortDirection::Out);
        assert_eq!(report_out.kind_id.as_deref(), Some("computation.norm.din4108"));
        assert_eq!(io.artifact.id, "computation.norm.din4108");
    }

    #[semio_framework_async_macros::async_test]
    fn the_artifact_kind_spec_is_a_data_value_document() {
        let spec = artifact_kind_spec("en1990", "EN 1990");
        assert_eq!(spec.id, "computation.norm.en1990");
        assert_eq!(spec.source_format, "norm.en1990.document");
        assert_eq!(spec.dimension, "data");
        assert_eq!(spec.component_kind, "norm");
        assert_eq!(spec.media_type.class, MediaClass::Data);
        assert_eq!(spec.media_type.form, MediaForm::Value);
    }

    #[semio_framework_async_macros::async_test]
    fn render_report_falls_back_to_a_placeholder_when_nothing_was_computed() {
        let json = serde_json::to_string(&render_report(&CheckReport::default())).expect("json");
        assert!(json.contains("No checks computed."), "{json}");
    }

    #[semio_framework_async_macros::async_test]
    fn render_inspection_falls_back_to_the_first_check_for_an_out_of_range_index() {
        let mut report = CheckReport::default();
        report.push(crate::document::CheckResult::from_utilization(
            crate::document::ClauseId::new("demo", "§1", "1.1"),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 0.5),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
            "demo check",
            crate::document::AnnexChoice::De,
        ));
        let inside = serde_json::to_string(&render_inspection(&report, Some(0))).expect("json");
        let outside = serde_json::to_string(&render_inspection(&report, Some(99))).expect("json");
        assert_eq!(inside, outside, "an out-of-range index must fall back to the first check");
        assert!(serde_json::to_string(&render_inspection(&CheckReport::default(), None)).expect("json").contains("No checks"));
    }

    #[semio_framework_async_macros::async_test]
    fn the_view_mode_is_the_same_for_every_viewer() {
        let mode = view_mode_definition();
        assert_eq!(mode.id, MODE_VIEW);
        assert!(mode.tools.is_empty() && mode.commands.is_empty() && mode.layout_id.is_none());
    }

    #[semio_framework_async_macros::async_test]
    fn single_window_layout_stacks_exactly_one_window() {
        let layout = single_window_layout("framework.window.table", "Report");
        let WindowLayoutRoot::Stack(stack) = layout.root else { panic!("expected a stack root") };
        assert_eq!(stack.children.len(), 1);
        assert_eq!(stack.children[0].window_kind_id, "framework.window.table");
    }

    #[semio_framework_async_macros::async_test]
    fn report_table_columns_and_rows_line_up_with_the_check_report() {
        let mut report = CheckReport::default();
        report.push(crate::document::CheckResult::from_utilization(
            crate::document::ClauseId::new("demo", "§1", "1.1"),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 0.5),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
            "demo check",
            crate::document::AnnexChoice::De,
        ));
        let columns = report_table_columns();
        let rows = report_table_rows(&report);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].len(), columns.len());
        assert!(rows[0][3].contains("demo check"));
    }

    #[semio_framework_async_macros::async_test]
    fn selected_check_index_arg_reads_the_shell_wire_shape() {
        assert_eq!(selected_check_index_arg(Some(&dsl::DslValue::from(&serde_json::json!({ "index": 3 })))), Some(3));
        assert_eq!(selected_check_index_arg(Some(&dsl::DslValue::from(&serde_json::json!({})))), None);
        assert_eq!(selected_check_index_arg(None), None);
    }
}
//#endregion 🧪️Tests
