//! 🗂️ VCS app — DocumentApp impl, render, manifest (constitutional: ui).

use semio_framework_plugin::{
        build_graph_timeline_scene, create_default_layout, tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionDescriptor, App, AppLabels, ArtifactKindSpec, ConfigView, DocumentApp,
    DocumentView, Emit, GraphTimelineScene, HistoryView, Label, Locale, LocalizedLabel, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, SurfaceKind, Terminology, UiButtonNode, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiPresence, UiStackNode, UiTreeItemNode,
};
use serde_json::{json, Value};
use store::{DocumentCommand, DocumentStore};
use vcs::{VcsDemoProjection, VCS_DEMO_SCHEMA};
use vcs_engine::{empty_vcs_demo_projection, VcsDemoConfig};
use vcs_op::{VcsDemoConfigOperation, VcsDemoOperation};
use vcs_protocol::VcsDemoCommand;

//#region 🔖️Constants
pub const VCS_PLAY_APP_ID: &str = "vcs-play";
const VCS_PLAY_BODY_EDITOR: &str = "vcs.play.editor";
const VCS_PLAY_BODY_HISTORY: &str = "vcs.play.history";
const VCS_PLAY_BODY_DOCUMENT: &str = "vcs.play.document";
const VCS_PLAY_BODY_INSPECTION: &str = "vcs.play.inspection";
const VCS_PLAY_SURFACE_HISTORY: &str = "vcs.play.history";
const VCS_PLAY_WINDOW_EDITOR: &str = "vcs-editor";
const VCS_PLAY_WINDOW_HISTORY: &str = "vcs-history";
//#endregion 🔖️Constants

//#region 🔖️Types
type VcsDemoStore = DocumentStore<VcsDemoProjection, VcsDemoOperation>;
//#endregion 🔖️Types

//#region 🔖️Locale

//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn demo_authors() -> Vec<vcs_kernel::Author> {
    vec![
        vcs_kernel::Author { id: "author-alice".into(), name: "Alice".into(), avatar: None },
        vcs_kernel::Author { id: "author-bob".into(), name: "Bob".into(), avatar: None },
        vcs_kernel::Author { id: "author-carol".into(), name: "Carol".into(), avatar: None },
    ]
}

fn vcs_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(VCS_PLAY_APP_ID).action(action, args)
}

fn vcs_demo_projection_diff_operations(current: &VcsDemoProjection, next: &VcsDemoProjection) -> Vec<VcsDemoOperation> {
    let mut operations = Vec::new();
    if next.title != current.title {
        operations.push(VcsDemoOperation::SetTitle { title: next.title.clone() });
    }
    if next.counter != current.counter {
        operations.push(VcsDemoOperation::SetCounter { counter: next.counter });
    }
    if next.status != current.status {
        operations.push(VcsDemoOperation::SetStatus { status: next.status.clone() });
    }
    if next.notes != current.notes {
        operations.push(VcsDemoOperation::SetNotes { notes: next.notes.clone() });
    }
    for tag in &next.tags {
        if !current.tags.contains(tag) {
            operations.push(VcsDemoOperation::AddTag { tag: tag.clone() });
        }
    }
    for tag in &current.tags {
        if !next.tags.contains(tag) {
            operations.push(VcsDemoOperation::RemoveTag { tag: tag.clone() });
        }
    }
    operations
}

/// 🩹️ Builds the `VcsDemoOperation` for a `patchProjection` field write — mirrors
/// `shooting_ui::shot_patch_for_field`'s string-keyed field dispatch.
fn vcs_patch_operation_for_field(field: &str, value: &str) -> Option<VcsDemoOperation> {
    match field {
        "title" => Some(VcsDemoOperation::SetTitle { title: value.into() }),
        "counter" => value.parse::<i64>().ok().map(|counter| VcsDemoOperation::SetCounter { counter }),
        "status" => Some(VcsDemoOperation::SetStatus { status: value.into() }),
        "notes" => Some(VcsDemoOperation::SetNotes { notes: value.into() }),
        _ => None,
    }
}

/// 🌱️ Seeds a rich, forked checkpoint/alternative history directly against the store — this app's
/// whole point is exercising the history UI (swimlane graph, checkpoints, alternatives, undo/redo),
/// so its "initial document" is itself a populated history, not a bare projection. Dispatched via
/// `DocumentApp::seed`, called once by `VcsDocumentApp::new` right after store construction.
fn seed_vcs_demo_history(store: &mut VcsDemoStore) {
    let authors = demo_authors();
    let alice = authors[0].clone();
    let bob = authors[1].clone();
    let carol = authors[2].clone();
    let last_checkpoint_id = |store: &VcsDemoStore| -> String { store.envelope().vcs.checkpoints.last().expect("checkpoint just committed").id.clone() };

    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetCounter { counter: 1 }, VcsDemoOperation::SetTitle { title: "VCS Demo".into() }], description: Some("bootstrap".into()) });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Bootstrap".into()), authors: vec![alice.clone()] });
    let c1 = last_checkpoint_id(store);

    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetNotes { notes: "main line".into() }, VcsDemoOperation::SetStatus { status: "draft".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Annotate main draft".into()), authors: vec![alice.clone(), bob.clone()] });
    let c2 = last_checkpoint_id(store);

    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetCounter { counter: 2 }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Main milestone".into()), authors: vec![alice.clone(), bob.clone(), carol.clone()] });
    let c3 = last_checkpoint_id(store);

    let _ = store.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: c3.clone() });
    let _ = store.dispatch(DocumentCommand::CreateAlternative { name: "feature-a".into() });
    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetTitle { title: "Feature A".into() }, VcsDemoOperation::AddTag { tag: "feature-a".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Start feature A".into()), authors: vec![alice.clone()] });
    let c4 = last_checkpoint_id(store);
    let feature_a_id = store.envelope().active_alternative_id.clone().expect("feature-a alternative id");

    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetCounter { counter: 10 }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Feature A progress".into()), authors: vec![alice.clone(), bob.clone()] });

    let _ = store.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: c3.clone() });
    let _ = store.dispatch(DocumentCommand::CreateAlternative { name: "feature-b".into() });
    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetTitle { title: "Feature B".into() }, VcsDemoOperation::SetNotes { notes: "branch b".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Start feature B".into()), authors: vec![bob.clone()] });
    let feature_b_id = store.envelope().active_alternative_id.clone().expect("feature-b alternative id");

    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetCounter { counter: 20 }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Feature B try".into()), authors: vec![bob.clone(), carol.clone()] });

    let _ = store.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: c3.clone() });
    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetStatus { status: "active".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Resume main".into()), authors: vec![carol.clone()] });
    let c8 = last_checkpoint_id(store);

    let _ = store.dispatch(DocumentCommand::SwitchAlternative { alternative_id: feature_a_id.clone() });
    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetCounter { counter: 11 }, VcsDemoOperation::AddTag { tag: "wip".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Feature A sprint".into()), authors: vec![alice.clone(), carol.clone()] });

    let _ = store.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: c4 });
    let _ = store.dispatch(DocumentCommand::CreateAlternative { name: "feature-a-hotfix".into() });
    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetStatus { status: "hotfix".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Hotfix off feature A".into()), authors: vec![bob.clone()] });

    let _ = store.dispatch(DocumentCommand::SwitchAlternative { alternative_id: feature_b_id });
    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::AddTag { tag: "review".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Feature B review".into()), authors: vec![bob.clone()] });

    let _ = store.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: c8 });
    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetCounter { counter: 3 }, VcsDemoOperation::SetNotes { notes: "main polish".into() }, VcsDemoOperation::AddTag { tag: "release".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Main batch polish".into()), authors: vec![alice.clone(), bob.clone(), carol.clone()] });

    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetStatus { status: "done".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Main release".into()), authors: vec![alice.clone()] });

    let _ = store.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: c2 });
    let _ = store.dispatch(DocumentCommand::CreateAlternative { name: "docs".into() });
    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetNotes { notes: "documentation pass".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Docs branch".into()), authors: vec![carol.clone()] });

    let _ = store.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: c1 });
    let _ = store.dispatch(DocumentCommand::CreateAlternative { name: "spike".into() });
    let _ = store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetTitle { title: "Spike prototype".into() }], description: None });
    let _ = store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("Spike experiment".into()), authors: vec![bob, carol] });
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the VCS app; one field per label makes every locale combination compile-checked.
    struct VcsLabels {
        document: native_en "Document", native_de "Dokument", reuse_en "Document", reuse_de "Dokument";
        actions: native_en "Actions", native_de "Aktionen", reuse_en "Actions", reuse_de "Aktionen";
        counter: native_en "Counter", native_de "Zähler", reuse_en "Counter", reuse_de "Zähler";
        commit: native_en "Commit", native_de "Commit", reuse_en "Commit", reuse_de "Commit";
        branch: native_en "Branch", native_de "Branch", reuse_en "Branch", reuse_de "Branch";
        undo: native_en "Undo", native_de "Rückgängig", reuse_en "Undo", reuse_de "Rückgängig";
        redo: native_en "Redo", native_de "Wiederholen", reuse_en "Redo", reuse_de "Wiederholen";
        title: native_en "Title", native_de "Titel", reuse_en "Title", reuse_de "Titel";
        status: native_en "Status", native_de "Status", reuse_en "Status", reuse_de "Status";
        notes: native_en "Notes", native_de "Notizen", reuse_en "Notes", reuse_de "Notizen";
        tags: native_en "Tags", native_de "Schlagwörter", reuse_en "Tags", reuse_de "Schlagwörter";
        alternatives: native_en "Alternatives", native_de "Alternativen", reuse_en "Alternatives", reuse_de "Alternativen";
        no_checkpoints: native_en "(no checkpoints)", native_de "(keine Checkpoints)", reuse_en "(no checkpoints)", reuse_de "(keine Checkpoints)";
        checkpoints: native_en "checkpoints", native_de "Checkpoints", reuse_en "checkpoints", reuse_de "Checkpoints";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
/// 🌳️ Builds the document tree's checkpoint + alternative sections from `HistoryView` alone — the
/// swimlane graph's own `HistoryColumn`s carry everything needed (checkpoint id/description/timestamp,
/// and which alternative ids reference each row); alternative rows are labeled by id since
/// `HistoryColumn` doesn't carry alternative display names (`vcs::Alternative.name` isn't part of the
/// `DocumentApp`-visible `HistoryView` contract — a real gap, noted for whoever revisits this).
fn build_document_tree(history: &HistoryView, selected: &[String], labels: &VcsLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("vcs-play-document");
    let checkpoint_items: Vec<UiTreeItemNode> = history
        .columns
        .iter()
        .rev()
        .map(|column| UiTreeItemNode {
            icon_id: Some("git-commit".into()),
            menu: None,
            ..tree_item_with_action(
                builder.item_id("checkpoint", &column.checkpoint_id),
                Label::data(column.description.clone().unwrap_or_else(|| column.checkpoint_id.clone())),
                Some(column.timestamp.clone()),
                vcs_action("checkoutCheckpoint", Some(json!({ "checkpointId": column.checkpoint_id }))),
            )
        })
        .collect();
    let mut alternative_ids: Vec<String> = Vec::new();
    for column in &history.columns {
        for alternative_id in &column.alternative_ids {
            if !alternative_ids.contains(alternative_id) {
                alternative_ids.push(alternative_id.clone());
            }
        }
    }
    let alternative_items: Vec<UiTreeItemNode> = alternative_ids
        .iter()
        .map(|alternative_id| {
            let count = history.columns.iter().filter(|column| column.alternative_ids.contains(alternative_id)).count();
            UiTreeItemNode {
                icon_id: Some("git-branch".into()),
                menu: None,
                ..tree_item_with_action(
                    builder.item_id("alternative", alternative_id),
                    Label::data(alternative_id.clone()),
                    Some(format!("{count} {}", labels.checkpoints.as_str())),
                    vcs_action("switchAlternative", Some(json!({ "alternativeId": alternative_id }))),
                )
            }
        })
        .collect();
    let selected_ids: Vec<String> = selected.iter().map(|id| builder.item_id("checkpoint", id)).collect();
    builder
        .section_or_placeholder("vcs-play-document.checkpoints", Some(labels.document.into()), true, checkpoint_items, labels.no_checkpoints)
        .section("vcs-play-document.alternatives", Some(labels.alternatives.into()), true, alternative_items)
        .selected(selected_ids)
        .selection_change(vcs_action("setSelection", None))
        .build()
}

fn build_inspection_tree(projection: &VcsDemoProjection, labels: &VcsLabels) -> UiNode {
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "vcs-play-inspector".into(),
        label: labels.title.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "vcs-play-inspector.title".into(),
                label: labels.title.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "vcs-play-inspector.title.input".into(),
                    input_kind: "text".into(),
                    value: projection.title.clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: vcs_action("patchProjection", Some(json!({ "field": "title" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "vcs-play-inspector.counter".into(),
                label: labels.counter.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "vcs-play-inspector.counter.input".into(),
                    input_kind: "number".into(),
                    value: projection.counter.to_string(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: vcs_action("patchProjection", Some(json!({ "field": "counter" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "vcs-play-inspector.status".into(),
                label: labels.status.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "vcs-play-inspector.status.input".into(),
                    input_kind: "text".into(),
                    value: projection.status.clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: vcs_action("patchProjection", Some(json!({ "field": "status" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "vcs-play-inspector.notes".into(),
                label: labels.notes.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "vcs-play-inspector.notes.input".into(),
                    input_kind: "text".into(),
                    value: projection.notes.clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: vcs_action("patchProjection", Some(json!({ "field": "notes" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            ui_inspector_readonly_field("vcs-play-inspector.tags", labels.tags, projection.tags.join(", ")),
        ],
    }])
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn ui_stack_horizontal(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "horizontal".into(), gap: Some("tight".into()), padding: Some("none".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

fn editor_button(id: &str, icon_id: &str, label: impl Into<Label>, action: &str) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(format!("vcs-play-editor.{id}")), icon_id: icon_id.into(), label: label.into(), action: vcs_action(action, None), style: None, presence: UiPresence::default(), menu: None })
}

fn render_editor(projection: &VcsDemoProjection, labels: &VcsLabels) -> UiNode {
    // One button per row where the label is dynamic-width (counter), two per row otherwise: the
    // framework's horizontal stack gives every child an equal flex-1 share and buttons don't shrink
    // below their label width, so a wide/growing label overflows and overlaps its neighbor in the
    // (narrower) Editor panel of the default layout. A leading heading clears the window's
    // Action/Viewport tab chrome, which otherwise overlaps content placed flush at the panel top.
    let heading = ui_text(labels.actions);
    let increment_row = ui_stack_horizontal(vec![editor_button("increment", "plus", Label::data(format!("+ {} ({})", labels.counter.as_str(), projection.counter)), "incrementCounter")]);
    let commit_row = ui_stack_horizontal(vec![editor_button("commit", "git-commit", labels.commit, "commitCheckpoint"), editor_button("new-alternative", "git-branch", labels.branch, "createAlternative")]);
    let history_row = ui_stack_horizontal(vec![editor_button("undo", "undo", labels.undo, "undo"), editor_button("redo", "redo", labels.redo, "redo")]);
    let summary =
        ui_stack_vertical(vec![ui_text(Label::data(format!("{} · {} {}", projection.title, labels.counter.as_str(), projection.counter))), ui_text(Label::data(if projection.notes.is_empty() { "—".to_string() } else { projection.notes.clone() }))]);
    ui_stack_vertical(vec![heading, increment_row, commit_row, history_row, summary])
}

fn render_history(history: &HistoryView) -> UiNode {
    build_graph_timeline_scene(VCS_PLAY_SURFACE_HISTORY, VCS_PLAY_APP_ID, GraphTimelineScene { columns_json: serde_json::to_string(&history.columns).unwrap_or_else(|_| "[]".into()) })
}
//#endregion 🔖️Render

//#region 🔖️VcsPlayApp
/// 🧪️ B1: unit struct — the former `VcsPlayApp::selected_checkpoint_ids` `RefCell` field now lives in
/// `vcs_engine::VcsDemoConfig` (see `DocumentApp::Config`), written through `vcs_op::VcsDemoConfigOperation`s.
#[derive(Default)]
pub struct VcsPlayApp;

impl DocumentApp for VcsPlayApp {
    type Projection = VcsDemoProjection;
    type Operation = VcsDemoOperation;
    type Config = VcsDemoConfig;
    type ConfigOperation = VcsDemoConfigOperation;
    type Command = VcsDemoCommand;

    fn app_id(&self) -> &str {
        VCS_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        VCS_DEMO_SCHEMA
    }

    fn initial_projection(&self) -> VcsDemoProjection {
        empty_vcs_demo_projection()
    }

    fn seed(&self, store: &mut DocumentStore<VcsDemoProjection, VcsDemoOperation>) {
        seed_vcs_demo_history(store);
    }

    /// 🏷️ Maps each `VcsDemoCommand` variant back to the action id it was declared under in
    /// `create_vcs_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check. `setLocale` isn't declared in the manifest (mirrors
    /// `ShootingCommand::SetLocale` — see `shooting_ui`'s identical doc), so it skips enforcement.
    fn command_id(&self, command: &VcsDemoCommand) -> &str {
        match command {
            VcsDemoCommand::IncrementCounter => "incrementCounter",
            VcsDemoCommand::PatchProjection { .. } => "patchProjection",
            VcsDemoCommand::TextEdit { .. } => "textEdit",
            VcsDemoCommand::Edit { .. } => "edit",
            VcsDemoCommand::SetSelection { .. } => "setSelection",
            VcsDemoCommand::SetLocale { .. } => "setLocale",
            VcsDemoCommand::NoOperation => "noOperation",
            VcsDemoCommand::CanvasPointerDown => "canvasPointerDown",
            VcsDemoCommand::CanvasPointerMove => "canvasPointerMove",
            VcsDemoCommand::CanvasPointerUp => "canvasPointerUp",
            VcsDemoCommand::CanvasWheel => "canvasWheel",
        }
    }

    /// 🧩️ The former `handle_action` match arms, ported verbatim: document-mutating arms emit
    /// `document_operations`; former ephemeral-state arms (selection/locale) emit `config_operations`.
    /// "undo"/"redo"/"commitCheckpoint"/"createAlternative"/"switchAlternative"/"checkoutCheckpoint"
    /// never reach here — `VcsDocumentApp` intercepts those six history actions before dispatching a
    /// typed command, straight to `DocumentCommand`.
    fn handle(&self, command: &VcsDemoCommand, doc: &DocumentView<'_, VcsDemoProjection>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Emit<VcsDemoOperation, VcsDemoConfigOperation> {
        let projection = doc.projection;
        match command {
            VcsDemoCommand::IncrementCounter => Emit::operations(vec![VcsDemoOperation::SetCounter { counter: projection.counter + 1 }]),
            VcsDemoCommand::PatchProjection { field, value } => match vcs_patch_operation_for_field(field, value) {
                Some(operation) => Emit::operations(vec![operation]),
                None => Emit::default(),
            },
            VcsDemoCommand::TextEdit { text } | VcsDemoCommand::Edit { text } => match serde_json::from_str::<VcsDemoProjection>(text) {
                Ok(next_projection) => {
                    let operations = vcs_demo_projection_diff_operations(projection, &next_projection);
                    if operations.is_empty() {
                        Emit::default()
                    } else {
                        Emit::operations(operations)
                    }
                }
                Err(_) => Emit::default(),
            },
            VcsDemoCommand::SetSelection { ids } => Emit::config(vec![VcsDemoConfigOperation::SetSelection { checkpoint_ids: ids.clone() }]),
            VcsDemoCommand::SetLocale { value } => Emit::config(vec![VcsDemoConfigOperation::SetLocale { value: value.clone() }]),
            VcsDemoCommand::NoOperation | VcsDemoCommand::CanvasPointerDown | VcsDemoCommand::CanvasPointerMove | VcsDemoCommand::CanvasPointerUp | VcsDemoCommand::CanvasWheel => Emit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, VcsDemoProjection>, cfg: &ConfigView<'_, VcsDemoConfig>) -> UiNode {
        let labels = semio_framework_plugin::resolve_labels_for_locale::<VcsLabels>(&cfg.projection.locale);
        match body_key {
            VCS_PLAY_BODY_EDITOR => render_editor(doc.projection, labels),
            VCS_PLAY_BODY_HISTORY => render_history(doc.history),
            VCS_PLAY_BODY_DOCUMENT => build_document_tree(doc.history, &cfg.projection.selected_checkpoint_ids, labels),
            VCS_PLAY_BODY_INSPECTION => build_inspection_tree(doc.projection, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️VcsPlayApp

//#region 🔖️Manifest
pub fn create_vcs_app() -> App {
    App::from_builder(
        App::builder(VCS_PLAY_APP_ID, LocalizedLabel::native("VCS", "VCS")).document(["semio", "vcs"])
            .artifact_kind(ArtifactKindSpec {
                id: "vcs.document".into(),
                name: "VCS Document".into(),
                source_format: "vcs.demo".into(),
                component_kind: "vcs".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                schema: "vcs.demo".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("git-branch")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(VCS_PLAY_WINDOW_EDITOR, LocalizedLabel::native("Editor", "Editor"), VCS_PLAY_BODY_EDITOR, SurfaceKind::Canvas2d, "pen-tool")
            .window_kind(VCS_PLAY_WINDOW_HISTORY, LocalizedLabel::native("History", "Verlauf"), VCS_PLAY_BODY_HISTORY, SurfaceKind::GraphTimeline, "git-branch")
            .panel_tab("framework.panel.document", LocalizedLabel::native("Document", "Dokument"), PanelGroup::Workbench, VCS_PLAY_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", LocalizedLabel::native("Inspection", "Inspektion"), PanelGroup::Details, VCS_PLAY_BODY_INSPECTION)
            .operation("incrementCounter", LocalizedLabel::native("Increment Counter", "Zähler erhöhen"))
            .operation("patchProjection", LocalizedLabel::native("Patch Projection", "Projektion aktualisieren"))
            .operation("textEdit", LocalizedLabel::native("Edit Text", "Text bearbeiten"))
            .operation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("noOperation", LocalizedLabel::native("No-operation", "Keine Aktion"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            .view_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"))
            .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"))
            .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Leinwand-Mausrad"))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(create_default_layout(
                &[VCS_PLAY_WINDOW_EDITOR.into(), VCS_PLAY_WINDOW_HISTORY.into()],
                "row",
                Some(&[30.0, 70.0]),
                Some(&["Editor".into(), "History".into()]),
            ))
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1) —
            // this app has no user-visible sticky defaults, so `config_spec()` stays the trait default
            // `ConfigSpec::empty()`; declared anyway for parity with every other converted app.
            .config(VcsPlayApp::default().config_spec()),
    )
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, ViewState};
    use store::{DocumentEnvelope, HistoryColumn};

    /// 📦️ Parses `document_pack()` (the full envelope) for tests that need to inspect raw
    /// checkpoints/alternatives directly — safe here because none of these tests undo/redo, so every
    /// edit in the log is still applied.
    fn seeded_envelope(app: &VcsDocumentApp<VcsPlayApp>) -> DocumentEnvelope<VcsDemoProjection, VcsDemoOperation> {
        let files = app.document_pack().expect("document pack");
        store::parse_document_pack::<VcsDemoProjection, VcsDemoOperation>(&files.pack, &files.spr).expect("parse document pack").envelope
    }

    #[test]
    fn renders_editor_scene() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let node = app.render(VCS_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(!json.contains("text-editor"), "editor must no longer be a raw JSON editor: {json}");
        for action in ["incrementCounter", "commitCheckpoint", "undo", "redo", "createAlternative"] {
            assert!(json.contains(action), "missing editor button for {action}: {json}");
        }
        assert!(json.contains(" · Counter "), "missing title/counter summary: {json}");
    }

    #[test]
    fn renders_history_scene() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let node = app.render(VCS_PLAY_BODY_HISTORY, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("graph-timeline"), "missing graph-timeline surface kind: {json}");
        assert!(json.contains("lane"), "missing lane field in history columns: {json}");
        assert!(!json.contains("\"table\""), "history must not fall back to a generic table: {json}");
    }

    #[test]
    fn seeded_history_has_checkpoints() {
        let app = testkit::new_app::<VcsPlayApp>();
        let envelope = seeded_envelope(&app);
        assert!(envelope.vcs.alternatives.len() >= 5, "expected >=5 alternatives, got {}", envelope.vcs.alternatives.len());
        assert!(envelope.vcs.checkpoints.len() >= 14, "expected >=14 checkpoints, got {}", envelope.vcs.checkpoints.len());
        let mut children_by_parent: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for checkpoint in &envelope.vcs.checkpoints {
            if let Some(parent_id) = &checkpoint.parent_id {
                *children_by_parent.entry(parent_id.clone()).or_insert(0) += 1;
            }
        }
        assert!(children_by_parent.values().any(|count| *count >= 2), "seed must contain a real fork (a checkpoint with >=2 children)");
        let lanes: std::collections::HashSet<usize> = store::build_history_columns(&envelope).into_iter().map(|column: HistoryColumn| column.lane).collect();
        assert!(lanes.len() >= 3, "expected >=3 distinct swimlanes, got {lanes:?}");
    }

    #[test]
    fn checkout_then_commit_forks_across_actions() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let envelope_before = seeded_envelope(&app);
        let root_checkpoint_id = envelope_before.vcs.checkpoints[0].id.clone();
        let children_of_root_before = envelope_before.vcs.checkpoints.iter().filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(root_checkpoint_id.as_str())).count();

        let checkout = app.handle_action("checkoutCheckpoint", Some(&json!({ "checkpointId": root_checkpoint_id })), &testkit::meta("local")).expect("checkout");
        assert!(checkout.operations.is_empty(), "history actions never emit KernelOperations");

        app.dispatch_typed(VcsDemoCommand::IncrementCounter, &testkit::meta("local")).expect("increment");
        app.handle_action("commitCheckpoint", Some(&json!({ "message": "forked from root" })), &testkit::meta("local")).expect("commit");

        let envelope_after = seeded_envelope(&app);
        let children_of_root_after = envelope_after.vcs.checkpoints.iter().filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(root_checkpoint_id.as_str())).count();
        assert_eq!(children_of_root_after, children_of_root_before + 1, "checking out the root then committing through actions must add a new fork of the root, not extend the trunk");
    }

    #[test]
    fn increment_counter_action_updates_projection() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let before = app.projection().expect("materialize projection").counter;
        let result = app.dispatch_typed(VcsDemoCommand::IncrementCounter, &testkit::meta("local")).expect("increment");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("materialize projection").counter, before + 1);
    }

    #[test]
    fn document_lists_checkpoints() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let node = app.render(VCS_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("vcs-play-document.checkpoint"));
    }

    #[test]
    fn text_edit_action_persists_projection_changes() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let before = app.projection().expect("materialize projection");
        let mut edited = before.clone();
        edited.title = "Edited via JSON".into();
        edited.counter = before.counter + 41;
        edited.tags.push("edited-in-place".into());
        let text = serde_json::to_string_pretty(&edited).unwrap();
        let result = app.dispatch_typed(VcsDemoCommand::TextEdit { text }, &testkit::meta("local")).expect("text edit");
        assert!(!result.operations.is_empty());
        let after = app.projection().expect("materialize projection");
        assert_eq!(after.title, "Edited via JSON");
        assert_eq!(after.counter, before.counter + 41);
        assert!(after.tags.contains(&"edited-in-place".to_string()));
    }

    #[test]
    fn vcs_labels_resolve_native_english_by_default() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let node = app.render(VCS_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Actions"));
        assert!(json.contains("Commit"));
        assert!(json.contains("Branch"));
        assert!(json.contains("Undo"));
        assert!(json.contains("Redo"));
        assert!(json.contains("Counter"));

        let inspection = app.render(VCS_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("Title"));
        assert!(inspection_json.contains("Status"));
        assert!(inspection_json.contains("Notes"));
        assert!(inspection_json.contains("Tags"));

        let document_tree = app.render(VCS_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Alternatives"));
        assert!(document_json.contains("checkpoints"));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more passing
    /// a `ViewState` into `render`/`app_labels` for this purpose (mirrors `shooting_ui`'s identical test).
    #[test]
    fn vcs_labels_resolve_german_locale() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        app.dispatch_typed(VcsDemoCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");

        let node = app.render(VCS_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Aktionen"));
        assert!(json.contains("Rückgängig"));
        assert!(json.contains("Wiederholen"));
        assert!(json.contains("Zähler"));

        let inspection = app.render(VCS_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("Titel"));
        assert!(inspection_json.contains("Notizen"));
        assert!(inspection_json.contains("Schlagwörter"));

        let document_tree = app.render(VCS_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Alternativen"));
        assert!(document_json.contains("Checkpoints"));
        assert!(!document_json.contains("\"Alternatives\""));
    }

    #[test]
    fn edit_action_is_alias_for_text_edit() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let before = app.projection().expect("materialize projection");
        let mut edited = before.clone();
        edited.status = "reviewed".into();
        let text = serde_json::to_string(&edited).unwrap();
        let result = app.dispatch_typed(VcsDemoCommand::Edit { text }, &testkit::meta("local")).expect("edit");
        assert!(!result.operations.is_empty());
        assert_eq!(app.projection().expect("materialize projection").status, "reviewed");
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let before = app.projection().expect("materialize projection").counter;
        app.dispatch_typed(VcsDemoCommand::IncrementCounter, &testkit::meta("local")).expect("increment");
        assert_eq!(app.projection().expect("materialize projection").counter, before + 1);
        let undo = app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert!(undo.operations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.projection().expect("materialize projection").counter, before);
        app.handle_action("redo", None, &testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("materialize projection").counter, before + 1);
    }

    #[test]
    fn create_and_switch_alternative_round_trip_through_the_wrapper() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let create = app.handle_action("createAlternative", Some(&json!({ "name": "trying-something" })), &testkit::meta("local")).expect("create alternative");
        assert!(create.operations.is_empty());
        let envelope = seeded_envelope(&app);
        assert!(envelope.active_alternative_id.is_some(), "createAlternative must set an active alternative");
    }

    /// 👁️ `setSelection` is config-only: it must drive `cfg.selected_checkpoint_ids` (rendered into the
    /// document tree's `selected` ids) without ever touching the document store.
    #[test]
    fn set_selection_drives_config_and_emits_no_document_operations() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let checkpoint_id = seeded_envelope(&app).vcs.checkpoints[0].id.clone();
        let result = app.dispatch_typed(VcsDemoCommand::SetSelection { ids: vec![checkpoint_id.clone()] }, &testkit::meta("local")).expect("select");
        assert!(result.operations.is_empty(), "setSelection mutates only ephemeral config, never the document");
        let node = app.render(VCS_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&checkpoint_id));
    }

    /// 🎥️ Config-only commands (selection/locale) never create a document-store undo step — mirrors
    /// `shooting_ui`'s `camera_drag_never_creates_a_document_undo_step`.
    #[test]
    fn set_selection_never_creates_a_document_undo_step() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let before = app.projection().expect("materialize projection").counter;
        app.dispatch_typed(VcsDemoCommand::SetSelection { ids: vec!["checkpoint-1".into()] }, &testkit::meta("local")).expect("select");
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo (no-op: nothing on the document store to undo)");
        assert_eq!(app.projection().expect("materialize projection").counter, before, "document undo has nothing to revert — selection never touched the document");
    }
}
//#endregion 🧪️Tests
