//! 🗂️ VCS plugin — declarative version-control play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{SurfaceKind, 
    build_table_scene, build_text_editor_scene, create_default_layout, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, App, CommandDescriptor, PanelGroup, PluginApp, PluginBundle, TableScene, TextEditorScene,
    UiControlNode, UiFieldNode, UiInputNode, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;
use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation,
    OperationDiff,
};

//#region 🔖Constants
const VCS_PLAY_APP_ID: &str = "vcs-play";
const VCS_PLAY_BODY_EDITOR: &str = "vcs.play.editor";
const VCS_PLAY_BODY_HISTORY: &str = "vcs.play.history";
const VCS_PLAY_BODY_DOCUMENT: &str = "vcs.play.document";
const VCS_PLAY_BODY_INSPECTION: &str = "vcs.play.inspection";
const VCS_PLAY_SURFACE_EDITOR: &str = "vcs.play.editor";
const VCS_PLAY_SURFACE_HISTORY: &str = "vcs.play.history";
const VCS_PLAY_WINDOW_EDITOR: &str = "vcs-editor";
const VCS_PLAY_WINDOW_HISTORY: &str = "vcs-history";
const VCS_DEMO_SCHEMA: &str = "vcs.demo";
//#endregion 🔖Constants

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VcsDemoProjection {
    schema: String,
    title: String,
    counter: i64,
    notes: String,
    status: String,
    tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
enum VcsDemoOp {
    SetCounter { counter: i64 },
    SetTitle { title: String },
    SetNotes { notes: String },
    SetStatus { status: String },
    AddTag { tag: String },
    RemoveTag { tag: String },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum VcsDemoDiff {
    #[default]
    Empty,
    SetCounter { counter: i64 },
    SetTitle { title: String },
    SetNotes { notes: String },
    SetStatus { status: String },
    AddTag { tag: String },
    RemoveTag { tag: String },
}

type VcsDemoEnvelope = DocumentVcsEnvelope<VcsDemoProjection, VcsDemoOp>;
type VcsDemoStore = DocumentVcsStore<VcsDemoProjection, VcsDemoOp>;

fn empty_vcs_demo_projection() -> VcsDemoProjection {
    VcsDemoProjection {
        schema: VCS_DEMO_SCHEMA.into(),
        title: "VCS Demo".into(),
        counter: 0,
        notes: String::new(),
        status: "new".into(),
        tags: Vec::new(),
    }
}

fn demo_authors() -> Vec<vcs::Author> {
    vec![
        vcs::Author {
            id: "author-alice".into(),
            name: "Alice".into(),
            avatar: None,
        },
        vcs::Author {
            id: "author-bob".into(),
            name: "Bob".into(),
            avatar: None,
        },
        vcs::Author {
            id: "author-carol".into(),
            name: "Carol".into(),
            avatar: None,
        },
    ]
}

impl OperationDiff<VcsDemoProjection> for VcsDemoDiff {
    fn apply(&self, projection: &VcsDemoProjection) -> VcsDemoProjection {
        let op = match self {
            VcsDemoDiff::Empty => return projection.clone(),
            VcsDemoDiff::SetCounter { counter } => VcsDemoOp::SetCounter { counter: *counter },
            VcsDemoDiff::SetTitle { title } => VcsDemoOp::SetTitle { title: title.clone() },
            VcsDemoDiff::SetNotes { notes } => VcsDemoOp::SetNotes { notes: notes.clone() },
            VcsDemoDiff::SetStatus { status } => VcsDemoOp::SetStatus { status: status.clone() },
            VcsDemoDiff::AddTag { tag } => VcsDemoOp::AddTag { tag: tag.clone() },
            VcsDemoDiff::RemoveTag { tag } => VcsDemoOp::RemoveTag { tag: tag.clone() },
        };
        apply_vcs_demo_op(projection, &op)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, VcsDemoDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<VcsDemoProjection> for VcsDemoOp {
    type Diff = VcsDemoDiff;

    fn diff(&self, _projection: &VcsDemoProjection) -> Self::Diff {
        match self {
            VcsDemoOp::SetCounter { counter } => VcsDemoDiff::SetCounter { counter: *counter },
            VcsDemoOp::SetTitle { title } => VcsDemoDiff::SetTitle { title: title.clone() },
            VcsDemoOp::SetNotes { notes } => VcsDemoDiff::SetNotes { notes: notes.clone() },
            VcsDemoOp::SetStatus { status } => VcsDemoDiff::SetStatus { status: status.clone() },
            VcsDemoOp::AddTag { tag } => VcsDemoDiff::AddTag { tag: tag.clone() },
            VcsDemoOp::RemoveTag { tag } => VcsDemoDiff::RemoveTag { tag: tag.clone() },
        }
    }

    fn backwards(&self, projection: &VcsDemoProjection) -> Vec<Self> {
        match self {
            VcsDemoOp::SetCounter { .. } => vec![VcsDemoOp::SetCounter {
                counter: projection.counter,
            }],
            VcsDemoOp::SetTitle { .. } => vec![VcsDemoOp::SetTitle {
                title: projection.title.clone(),
            }],
            VcsDemoOp::SetNotes { .. } => vec![VcsDemoOp::SetNotes {
                notes: projection.notes.clone(),
            }],
            VcsDemoOp::SetStatus { .. } => vec![VcsDemoOp::SetStatus {
                status: projection.status.clone(),
            }],
            VcsDemoOp::AddTag { tag } => vec![VcsDemoOp::RemoveTag { tag: tag.clone() }],
            VcsDemoOp::RemoveTag { tag } => vec![VcsDemoOp::AddTag { tag: tag.clone() }],
        }
    }
}

fn apply_vcs_demo_op(projection: &VcsDemoProjection, operation: &VcsDemoOp) -> VcsDemoProjection {
    let mut next = projection.clone();
    match operation {
        VcsDemoOp::SetCounter { counter } => next.counter = *counter,
        VcsDemoOp::SetTitle { title } => next.title = title.clone(),
        VcsDemoOp::SetNotes { notes } => next.notes = notes.clone(),
        VcsDemoOp::SetStatus { status } => next.status = status.clone(),
        VcsDemoOp::AddTag { tag } => {
            if !next.tags.contains(tag) {
                next.tags.push(tag.clone());
            }
        }
        VcsDemoOp::RemoveTag { tag } => next.tags.retain(|entry| entry != tag),
    }
    next
}
//#endregion 🔖Domain

//#region 🔖Envelope
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VcsPlayEnvelope {
    envelope: VcsDemoEnvelope,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    selected_checkpoint_ids: Vec<String>,
}

fn default_envelope() -> VcsPlayEnvelope {
    let mut store = VcsDemoStore::new(create_document_vcs_envelope(
        VCS_DEMO_SCHEMA,
        "vcs-demo",
        empty_vcs_demo_projection(),
        None,
    ));
    seed_vcs_demo_history(&mut store);
    VcsPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        selected_checkpoint_ids: Vec::new(),
    }
}

fn parse_envelope(document_json: &str) -> VcsPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &VcsPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn vcs_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: VCS_PLAY_APP_ID.into(),
        command: command.into(),
        args,
    }
}

fn store_from_envelope(envelope: &VcsPlayEnvelope) -> VcsDemoStore {
    let mut store = VcsDemoStore::new(envelope.envelope.clone());
    store.set_envelope(envelope.envelope.clone(), envelope.applied_edit_ids.clone());
    store
}

fn sync_store_to_envelope(store: &VcsDemoStore, selected: &[String]) -> VcsPlayEnvelope {
    VcsPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        selected_checkpoint_ids: selected.to_vec(),
    }
}

fn materialized_projection(play: &VcsPlayEnvelope) -> VcsDemoProjection {
    vcs::materialize_document_projection(&play.envelope, &play.applied_edit_ids)
        .unwrap_or_else(|_| play.envelope.vcs.initial_projection.clone())
}

fn vcs_demo_projection_diff_ops(current: &VcsDemoProjection, next: &VcsDemoProjection) -> Vec<VcsDemoOp> {
    let mut operations = Vec::new();
    if next.title != current.title {
        operations.push(VcsDemoOp::SetTitle { title: next.title.clone() });
    }
    if next.counter != current.counter {
        operations.push(VcsDemoOp::SetCounter { counter: next.counter });
    }
    if next.status != current.status {
        operations.push(VcsDemoOp::SetStatus { status: next.status.clone() });
    }
    if next.notes != current.notes {
        operations.push(VcsDemoOp::SetNotes { notes: next.notes.clone() });
    }
    for tag in &next.tags {
        if !current.tags.contains(tag) {
            operations.push(VcsDemoOp::AddTag { tag: tag.clone() });
        }
    }
    for tag in &current.tags {
        if !next.tags.contains(tag) {
            operations.push(VcsDemoOp::RemoveTag { tag: tag.clone() });
        }
    }
    operations
}

fn seed_vcs_demo_history(store: &mut VcsDemoStore) {
    let authors = demo_authors();
    let alice = authors[0].clone();
    let bob = authors[1].clone();
    let carol = authors[2].clone();

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![
            VcsDemoOp::SetCounter { counter: 1 },
            VcsDemoOp::SetTitle {
                title: "VCS Demo".into(),
            },
        ],
        description: Some("bootstrap".into()),
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Bootstrap".into()),
        authors: vec![alice.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![
            VcsDemoOp::SetNotes {
                notes: "main line".into(),
            },
            VcsDemoOp::SetStatus {
                status: "draft".into(),
            },
        ],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Annotate main draft".into()),
        authors: vec![alice.clone(), bob.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetCounter { counter: 2 }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Main milestone".into()),
        authors: vec![alice.clone(), bob.clone(), carol.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::CreateAlternative {
        name: "feature-a".into(),
    });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![
            VcsDemoOp::SetTitle {
                title: "Feature A".into(),
            },
            VcsDemoOp::AddTag {
                tag: "feature-a".into(),
            },
        ],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Start feature A".into()),
        authors: vec![alice.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetCounter { counter: 3 }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Main release".into()),
        authors: vec![alice],
    });
}
//#endregion 🔖Envelope

//#region 🔖History
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HistoryRow {
    checkpoint_id: String,
    timestamp: String,
    labels: Vec<String>,
    authors: Vec<vcs::Author>,
    parent_checkpoint_id: Option<String>,
    description: Option<String>,
    lane: usize,
}

fn history_rows(envelope: &VcsDemoEnvelope) -> Vec<HistoryRow> {
    envelope
        .vcs
        .checkpoints
        .iter()
        .enumerate()
        .rev()
        .map(|(index, checkpoint)| {
            let alternative_ids: Vec<String> = envelope
                .vcs
                .alternatives
                .iter()
                .filter(|alt| alt.checkpoint_ids.contains(&checkpoint.id))
                .map(|alt| alt.id.clone())
                .collect();
            let mut labels: Vec<String> = envelope
                .vcs
                .alternatives
                .iter()
                .filter(|alt| alternative_ids.contains(&alt.id))
                .map(|alt| alt.name.clone())
                .collect();
            if labels.is_empty() && index == 0 {
                labels.push("main".into());
            }
            HistoryRow {
                checkpoint_id: checkpoint.id.clone(),
                timestamp: checkpoint.timestamp.clone(),
                labels,
                authors: checkpoint.authors.clone(),
                parent_checkpoint_id: checkpoint.parent_id.clone(),
                description: checkpoint.message.clone(),
                lane: alternative_ids.len(),
            }
        })
        .collect()
}
//#endregion 🔖History

//#region 🔖Panels
fn build_document_tree(envelope: &VcsDemoEnvelope, selected: &[String]) -> UiNode {
    let checkpoint_items: Vec<UiTreeItemNode> = envelope
        .vcs
        .checkpoints
        .iter()
        .rev()
        .map(|checkpoint| UiTreeItemNode {
            id: format!("vcs-play-document.checkpoint.{}", checkpoint.id),
            label: checkpoint.message.clone().unwrap_or_else(|| checkpoint.id.clone()),
            description: Some(checkpoint.timestamp.clone()),
            icon_id: Some("git-commit".into()),
            selected: None,
            default_open: None,
            command: Some(vcs_cmd("checkoutCheckpoint", Some(json!({ "checkpointId": checkpoint.id })))),
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        })
        .collect();
    let alternative_items: Vec<UiTreeItemNode> = envelope
        .vcs
        .alternatives
        .iter()
        .map(|alt| UiTreeItemNode {
            id: format!("vcs-play-document.alternative.{}", alt.id),
            label: alt.name.clone(),
            description: Some(format!("{} checkpoints", alt.checkpoint_ids.len())),
            icon_id: Some("git-branch".into()),
            selected: None,
            default_open: None,
            command: Some(vcs_cmd(
                "switchAlternative",
                Some(json!({ "alternativeId": alt.id })),
            )),
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "vcs-play-document.checkpoints".into(),
                label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
                default_open: Some(true),
                items: if checkpoint_items.is_empty() {
                    vec![UiTreeItemNode {
                        id: "vcs-play-document.empty".into(),
                        label: "(no checkpoints)".into(),
                        description: None,
                        icon_id: None,
                        selected: None,
                        default_open: None,
                        command: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    }]
                } else {
                    checkpoint_items
                },
            },
            UiTreeSectionNode {
                id: "vcs-play-document.alternatives".into(),
                label: Some("Alternatives".into()),
                default_open: Some(true),
                items: alternative_items,
            },
        ],
        selected_ids: Some(
            selected
                .iter()
                .map(|id| format!("vcs-play-document.checkpoint.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(vcs_cmd("setSelection", None)),
    })
}

fn build_inspection_tree(projection: &VcsDemoProjection) -> UiNode {
    ui_stack_vertical(vec![
        UiNode::Field(UiFieldNode {
            id: "vcs-play-inspector.title".into(),
            label: "Title".into(),
            child: UiControlNode::Input(UiInputNode {
                id: "vcs-play-inspector.title.input".into(),
                input_kind: "text".into(),
                value: projection.title.clone(),
                placeholder: None,
                commit: Some("blur".into()),
                on_change: vcs_cmd("patchProjection", Some(json!({ "field": "title" }))),
            }),
        }),
        UiNode::Field(UiFieldNode {
            id: "vcs-play-inspector.counter".into(),
            label: "Counter".into(),
            child: UiControlNode::Input(UiInputNode {
                id: "vcs-play-inspector.counter.input".into(),
                input_kind: "number".into(),
                value: projection.counter.to_string(),
                placeholder: None,
                commit: Some("blur".into()),
                on_change: vcs_cmd("patchProjection", Some(json!({ "field": "counter" }))),
            }),
        }),
        UiNode::Field(UiFieldNode {
            id: "vcs-play-inspector.status".into(),
            label: "Status".into(),
            child: UiControlNode::Input(UiInputNode {
                id: "vcs-play-inspector.status.input".into(),
                input_kind: "text".into(),
                value: projection.status.clone(),
                placeholder: None,
                commit: Some("blur".into()),
                on_change: vcs_cmd("patchProjection", Some(json!({ "field": "status" }))),
            }),
        }),
        UiNode::Field(UiFieldNode {
            id: "vcs-play-inspector.notes".into(),
            label: "Notes".into(),
            child: UiControlNode::Input(UiInputNode {
                id: "vcs-play-inspector.notes.input".into(),
                input_kind: "text".into(),
                value: projection.notes.clone(),
                placeholder: None,
                commit: Some("blur".into()),
                on_change: vcs_cmd("patchProjection", Some(json!({ "field": "notes" }))),
            }),
        }),
        ui_inspector_readonly_field("vcs-play-inspector.tags", "Tags", projection.tags.join(", ")),
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_editor(projection: &VcsDemoProjection) -> UiNode {
    build_text_editor_scene(
        VCS_PLAY_SURFACE_EDITOR,
        VCS_PLAY_APP_ID,
        TextEditorScene::base(
            serde_json::to_string_pretty(projection).unwrap_or_else(|_| "{}".into()),
            Some("json".into()),
            None,
        ),
    )
}

fn render_history(envelope: &VcsDemoEnvelope) -> UiNode {
    let rows = history_rows(envelope);
    build_table_scene(
        VCS_PLAY_SURFACE_HISTORY,
        VCS_PLAY_APP_ID,
        TableScene {
            columns_json: json!([
                {"id":"checkpointId","label":"Checkpoint"},
                {"id":"timestamp","label":"When"},
                {"id":"labels","label":"Labels"},
                {"id":"description","label":"Message"}
            ])
            .to_string(),
            rows_json: serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()),
        },
    )
}
//#endregion 🔖Render

//#region 🔖VcsPlayApp
struct VcsPlayApp;

impl PluginApp for VcsPlayApp {
    fn app_id(&self) -> &str {
        VCS_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("vcs envelope json")
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let mut store = store_from_envelope(&play);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<VcsPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| value.as_array()) {
                    play.selected_checkpoint_ids = ids
                        .iter()
                        .filter_map(|value| value.as_str().map(str::to_string))
                        .collect();
                    return vec![set_document_op(&play)];
                }
            }
            "incrementCounter" => {
                let projection = store.projection().unwrap_or_else(|_| empty_vcs_demo_projection());
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![VcsDemoOp::SetCounter {
                        counter: projection.counter + 1,
                    }],
                    description: None,
                });
                return vec![set_document_op(&sync_store_to_envelope(&store, &play.selected_checkpoint_ids))];
            }
            "commitCheckpoint" => {
                let projection = store.projection().unwrap_or_else(|_| empty_vcs_demo_projection());
                let message = args
                    .and_then(|value| value.get("message"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("Checkpoint @ {}", projection.counter));
                let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
                    message: Some(message),
                    authors: demo_authors().into_iter().take(1).collect(),
                });
                return vec![set_document_op(&sync_store_to_envelope(&store, &play.selected_checkpoint_ids))];
            }
            "undo" => {
                let _ = store.dispatch(DocumentVcsCommand::Undo);
                return vec![set_document_op(&sync_store_to_envelope(&store, &play.selected_checkpoint_ids))];
            }
            "redo" => {
                let _ = store.dispatch(DocumentVcsCommand::Redo);
                return vec![set_document_op(&sync_store_to_envelope(&store, &play.selected_checkpoint_ids))];
            }
            "createAlternative" => {
                let count = store.envelope().vcs.alternatives.len();
                let name = args
                    .and_then(|value| value.get("name"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let name = if name.is_empty() {
                    format!("alt-{}", count + 1)
                } else {
                    name.to_string()
                };
                let _ = store.dispatch(DocumentVcsCommand::CreateAlternative { name });
                return vec![set_document_op(&sync_store_to_envelope(&store, &play.selected_checkpoint_ids))];
            }
            "switchAlternative" => {
                if let Some(alternative_id) = args.and_then(|value| value.get("alternativeId")).and_then(|value| value.as_str()) {
                    let _ = store.dispatch(DocumentVcsCommand::SwitchAlternative {
                        alternative_id: alternative_id.into(),
                    });
                    return vec![set_document_op(&sync_store_to_envelope(&store, &play.selected_checkpoint_ids))];
                }
            }
            "checkoutCheckpoint" => {
                if let Some(checkpoint_id) = args.and_then(|value| value.get("checkpointId")).and_then(|value| value.as_str()) {
                    let _ = store.dispatch(DocumentVcsCommand::CheckoutCheckpoint {
                        checkpoint_id: checkpoint_id.into(),
                    });
                    return vec![set_document_op(&sync_store_to_envelope(&store, &play.selected_checkpoint_ids))];
                }
            }
            "patchProjection" => {
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                let operation = match field {
                    "title" => value.and_then(|entry| entry.as_str()).map(|title| VcsDemoOp::SetTitle { title: title.into() }),
                    "counter" => value.and_then(|entry| entry.as_i64()).map(|counter| VcsDemoOp::SetCounter { counter }),
                    "status" => value.and_then(|entry| entry.as_str()).map(|status| VcsDemoOp::SetStatus { status: status.into() }),
                    "notes" => value.and_then(|entry| entry.as_str()).map(|notes| VcsDemoOp::SetNotes { notes: notes.into() }),
                    _ => None,
                };
                if let Some(operation) = operation {
                    let _ = store.dispatch(DocumentVcsCommand::Apply {
                        operations: vec![operation],
                        description: None,
                    });
                    return vec![set_document_op(&sync_store_to_envelope(&store, &play.selected_checkpoint_ids))];
                }
            }
            "textEdit" | "edit" => {
                if let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) {
                    if let Ok(next_projection) = serde_json::from_str::<VcsDemoProjection>(text) {
                        let current = store.projection().unwrap_or_else(|_| empty_vcs_demo_projection());
                        let operations = vcs_demo_projection_diff_ops(&current, &next_projection);
                        if !operations.is_empty() {
                            let _ = store.dispatch(DocumentVcsCommand::Apply { operations, description: None });
                            return vec![set_document_op(&sync_store_to_envelope(&store, &play.selected_checkpoint_ids))];
                        }
                    }
                }
            }
            "noop" | "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "canvasWheel" => {}
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        let materialized = materialized_projection(&play);
        match body_key {
            VCS_PLAY_BODY_EDITOR => render_editor(&materialized),
            VCS_PLAY_BODY_HISTORY => render_history(&play.envelope),
            VCS_PLAY_BODY_DOCUMENT => build_document_tree(&play.envelope, &play.selected_checkpoint_ids),
            VCS_PLAY_BODY_INSPECTION => build_inspection_tree(&materialized),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖VcsPlayApp

//#region 🔖AppFactory
fn create_vcs_app() -> App {
    App::from_builder(
        App::builder(VCS_PLAY_APP_ID, "VCS").document(["semio", "vcs"])
            .icon_id("git-branch")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(VCS_PLAY_WINDOW_EDITOR, "Editor", VCS_PLAY_BODY_EDITOR, SurfaceKind::TextEditor)
            .window_kind(VCS_PLAY_WINDOW_HISTORY, "History", VCS_PLAY_BODY_HISTORY, SurfaceKind::Canvas2d)
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, VCS_PLAY_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, VCS_PLAY_BODY_INSPECTION)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(create_default_layout(
                &[VCS_PLAY_WINDOW_EDITOR.into(), VCS_PLAY_WINDOW_HISTORY.into()],
                "row",
                Some(&[30.0, 70.0]),
                Some(&["Editor".into(), "History".into()]),
            )),
    )
}

fn vcs_bundle() -> PluginBundle {
    PluginBundle::new("vcs", "VCS", "0.1.0").register_app(create_vcs_app(), || Box::new(VcsPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(vcs_bundle()));

semio_framework_plugin::plugin_exports!();
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_editor_scene() {
        let app = VcsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(VCS_PLAY_BODY_EDITOR, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn renders_history_table() {
        let app = VcsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(VCS_PLAY_BODY_HISTORY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("table"));
    }

    #[test]
    fn seeded_history_has_checkpoints() {
        let envelope = default_envelope();
        assert!(!envelope.envelope.vcs.checkpoints.is_empty());
        assert!(!envelope.envelope.vcs.alternatives.is_empty());
    }

    #[test]
    fn increment_counter_command_updates_projection() {
        let mut app = VcsPlayApp;
        let document = app.initial_document_json();
        let before = materialized_projection(&parse_envelope(&document)).counter;
        let ops = app.handle_command_patch_ops("incrementCounter", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: VcsPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(materialized_projection(&next).counter, before + 1);
    }

    #[test]
    fn document_lists_checkpoints() {
        let app = VcsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(VCS_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("vcs-play-document.checkpoint"));
    }

    #[test]
    fn text_edit_command_persists_projection_changes() {
        let mut app = VcsPlayApp;
        let document = app.initial_document_json();
        let before = materialized_projection(&parse_envelope(&document));
        let mut edited = before.clone();
        edited.title = "Edited via JSON".into();
        edited.counter = before.counter + 41;
        edited.tags.push("edited-in-place".into());
        let text = serde_json::to_string_pretty(&edited).unwrap();
        let ops = app.handle_command_patch_ops("textEdit", Some(&json!({ "text": text })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: VcsPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let after = materialized_projection(&next);
        assert_eq!(after.title, "Edited via JSON");
        assert_eq!(after.counter, before.counter + 41);
        assert!(after.tags.contains(&"edited-in-place".to_string()));
    }

    #[test]
    fn edit_command_is_alias_for_text_edit() {
        let mut app = VcsPlayApp;
        let document = app.initial_document_json();
        let before = materialized_projection(&parse_envelope(&document));
        let mut edited = before.clone();
        edited.status = "reviewed".into();
        let text = serde_json::to_string(&edited).unwrap();
        let ops = app.handle_command_patch_ops("edit", Some(&json!({ "text": text })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: VcsPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(materialized_projection(&next).status, "reviewed");
    }
}
//#endregion 🧪Tests
