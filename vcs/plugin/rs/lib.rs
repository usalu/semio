//! 🗂️ VCS plugin — declarative version-control play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{SurfaceKind,
    build_vcs_history_scene, create_default_layout, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, App, ActionDescriptor, PanelGroup, PluginApp, PluginBundle,
    UiButtonNode, UiFieldNode, UiInputNode, UiNode, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    VcsHistoryScene, ViewState, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
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
    /// @emoji 🧭 Checkout position, since the store is reconstructed fresh from this JSON document
    /// on every action call and would otherwise reset to the latest checkpoint on every dispatch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    current_checkpoint_id: Option<String>,
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
        current_checkpoint_id: store.current_checkpoint_id().map(str::to_string),
    }
}

fn parse_envelope(document_json: &str) -> VcsPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &VcsPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn vcs_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: VCS_PLAY_APP_ID.into(),
        action: action.into(),
        args,
    }
}

fn store_from_envelope(envelope: &VcsPlayEnvelope) -> VcsDemoStore {
    let mut store = VcsDemoStore::new(envelope.envelope.clone());
    store.set_envelope(envelope.envelope.clone(), envelope.applied_edit_ids.clone());
    store.set_current_checkpoint_id(envelope.current_checkpoint_id.clone());
    store
}

fn sync_store_to_envelope(store: &VcsDemoStore, selected: &[String]) -> VcsPlayEnvelope {
    VcsPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        selected_checkpoint_ids: selected.to_vec(),
        current_checkpoint_id: store.current_checkpoint_id().map(str::to_string),
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
    let last_checkpoint_id =
        |store: &VcsDemoStore| -> String { store.envelope().vcs.checkpoints.last().expect("checkpoint just committed").id.clone() };

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetCounter { counter: 1 }, VcsDemoOp::SetTitle { title: "VCS Demo".into() }],
        description: Some("bootstrap".into()),
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Bootstrap".into()),
        authors: vec![alice.clone()],
    });
    let c1 = last_checkpoint_id(store);

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetNotes { notes: "main line".into() }, VcsDemoOp::SetStatus { status: "draft".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Annotate main draft".into()),
        authors: vec![alice.clone(), bob.clone()],
    });
    let c2 = last_checkpoint_id(store);

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetCounter { counter: 2 }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Main milestone".into()),
        authors: vec![alice.clone(), bob.clone(), carol.clone()],
    });
    let c3 = last_checkpoint_id(store);

    let _ = store.dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: c3.clone() });
    let _ = store.dispatch(DocumentVcsCommand::CreateAlternative { name: "feature-a".into() });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetTitle { title: "Feature A".into() }, VcsDemoOp::AddTag { tag: "feature-a".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Start feature A".into()),
        authors: vec![alice.clone()],
    });
    let c4 = last_checkpoint_id(store);
    let feature_a_id = store.envelope().active_alternative_id.clone().expect("feature-a alternative id");

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetCounter { counter: 10 }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Feature A progress".into()),
        authors: vec![alice.clone(), bob.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: c3.clone() });
    let _ = store.dispatch(DocumentVcsCommand::CreateAlternative { name: "feature-b".into() });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetTitle { title: "Feature B".into() }, VcsDemoOp::SetNotes { notes: "branch b".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Start feature B".into()),
        authors: vec![bob.clone()],
    });
    let feature_b_id = store.envelope().active_alternative_id.clone().expect("feature-b alternative id");

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetCounter { counter: 20 }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Feature B try".into()),
        authors: vec![bob.clone(), carol.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: c3.clone() });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetStatus { status: "active".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Resume main".into()),
        authors: vec![carol.clone()],
    });
    let c8 = last_checkpoint_id(store);

    let _ = store.dispatch(DocumentVcsCommand::SwitchAlternative { alternative_id: feature_a_id.clone() });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetCounter { counter: 11 }, VcsDemoOp::AddTag { tag: "wip".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Feature A sprint".into()),
        authors: vec![alice.clone(), carol.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: c4 });
    let _ = store.dispatch(DocumentVcsCommand::CreateAlternative { name: "feature-a-hotfix".into() });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetStatus { status: "hotfix".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Hotfix off feature A".into()),
        authors: vec![bob.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::SwitchAlternative { alternative_id: feature_b_id });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::AddTag { tag: "review".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Feature B review".into()),
        authors: vec![bob.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: c8 });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![
            VcsDemoOp::SetCounter { counter: 3 },
            VcsDemoOp::SetNotes { notes: "main polish".into() },
            VcsDemoOp::AddTag { tag: "release".into() },
        ],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Main batch polish".into()),
        authors: vec![alice.clone(), bob.clone(), carol.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetStatus { status: "done".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Main release".into()),
        authors: vec![alice.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: c2 });
    let _ = store.dispatch(DocumentVcsCommand::CreateAlternative { name: "docs".into() });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetNotes { notes: "documentation pass".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Docs branch".into()),
        authors: vec![carol.clone()],
    });

    let _ = store.dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: c1 });
    let _ = store.dispatch(DocumentVcsCommand::CreateAlternative { name: "spike".into() });
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![VcsDemoOp::SetTitle { title: "Spike prototype".into() }],
        description: None,
    });
    let _ = store.dispatch(DocumentVcsCommand::CommitCheckpoint {
        message: Some("Spike experiment".into()),
        authors: vec![bob, carol],
    });
}
//#endregion 🔖Envelope

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the VCS app; one field per label makes every locale combination compile-checked.
struct VcsLabels {
    actions: &'static str,
    counter: &'static str,
    commit: &'static str,
    branch: &'static str,
    undo: &'static str,
    redo: &'static str,
    title: &'static str,
    status: &'static str,
    notes: &'static str,
    tags: &'static str,
    alternatives: &'static str,
    no_checkpoints: &'static str,
    checkpoints: &'static str,
    window_editor: &'static str,
    window_history: &'static str,
}

const VCS_LABELS_NATIVE_EN: VcsLabels = VcsLabels {
    actions: "Actions",
    counter: "Counter",
    commit: "Commit",
    branch: "Branch",
    undo: "Undo",
    redo: "Redo",
    title: "Title",
    status: "Status",
    notes: "Notes",
    tags: "Tags",
    alternatives: "Alternatives",
    no_checkpoints: "(no checkpoints)",
    checkpoints: "checkpoints",
    window_editor: "Editor",
    window_history: "History",
};

const VCS_LABELS_NATIVE_DE: VcsLabels = VcsLabels {
    actions: "Aktionen",
    counter: "Zähler",
    commit: "Commit",
    branch: "Branch",
    undo: "Rückgängig",
    redo: "Wiederholen",
    title: "Titel",
    status: "Status",
    notes: "Notizen",
    tags: "Schlagwörter",
    alternatives: "Alternativen",
    no_checkpoints: "(keine Checkpoints)",
    checkpoints: "Checkpoints",
    window_editor: "Editor",
    window_history: "Verlauf",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; no terminology variant exists for this app.
fn vcs_labels(view_state: &ViewState) -> &'static VcsLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de { &VCS_LABELS_NATIVE_DE } else { &VCS_LABELS_NATIVE_EN }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn build_document_tree(envelope: &VcsDemoEnvelope, selected: &[String], labels: &VcsLabels) -> UiNode {
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
            action: Some(vcs_action("checkoutCheckpoint", Some(json!({ "checkpointId": checkpoint.id })))),
            hover_action: None,
            unhover_action: None,
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
            description: Some(format!("{} {}", alt.checkpoint_ids.len(), labels.checkpoints)),
            icon_id: Some("git-branch".into()),
            selected: None,
            default_open: None,
            action: Some(vcs_action(
                "switchAlternative",
                Some(json!({ "alternativeId": alt.id })),
            )),
            hover_action: None,
            unhover_action: None,
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
                        label: labels.no_checkpoints.into(),
                        description: None,
                        icon_id: None,
                        selected: None,
                        default_open: None,
                        action: None,
        hover_action: None,
        unhover_action: None,
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
                label: Some(labels.alternatives.into()),
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
        selection_change: Some(vcs_action("setSelection", None)),
        drop_action: None,
    })
}

fn build_inspection_tree(projection: &VcsDemoProjection, labels: &VcsLabels) -> UiNode {
    ui_stack_vertical(vec![
        UiNode::Field(UiFieldNode {
            id: "vcs-play-inspector.title".into(),
            label: labels.title.into(),
            child: Box::new(UiNode::Input(UiInputNode {
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
            })),
            description: None,
            required: None,
            error: None,
        }),
        UiNode::Field(UiFieldNode {
            id: "vcs-play-inspector.counter".into(),
            label: labels.counter.into(),
            child: Box::new(UiNode::Input(UiInputNode {
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
            })),
            description: None,
            required: None,
            error: None,
        }),
        UiNode::Field(UiFieldNode {
            id: "vcs-play-inspector.status".into(),
            label: labels.status.into(),
            child: Box::new(UiNode::Input(UiInputNode {
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
            })),
            description: None,
            required: None,
            error: None,
        }),
        UiNode::Field(UiFieldNode {
            id: "vcs-play-inspector.notes".into(),
            label: labels.notes.into(),
            child: Box::new(UiNode::Input(UiInputNode {
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
            })),
            description: None,
            required: None,
            error: None,
        }),
        ui_inspector_readonly_field("vcs-play-inspector.tags", labels.tags, projection.tags.join(", ")),
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
fn ui_stack_horizontal(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "horizontal".into(),
        gap: Some("tight".into()),
        padding: Some("none".into()),
        id: None,
        selected: None,
        activate: None,
        children,
        drop_action: None,
    })
}

fn editor_button(id: &str, icon_id: &str, label: &str, action: &str) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(format!("vcs-play-editor.{id}")),
        icon_id: icon_id.into(),
        label: label.into(),
        action: vcs_action(action, None),
        style: None,
        disabled: None,
    })
}

fn render_editor(projection: &VcsDemoProjection, labels: &VcsLabels) -> UiNode {
    // One button per row where the label is dynamic-width (counter), two per row otherwise: the
    // framework's horizontal stack gives every child an equal flex-1 share and buttons don't shrink
    // below their label width, so a wide/growing label overflows and overlaps its neighbor in the
    // (narrower) Editor panel of the default layout. A leading heading clears the window's
    // Action/Viewport tab chrome, which otherwise overlaps content placed flush at the panel top.
    let heading = ui_text(labels.actions);
    let increment_row = ui_stack_horizontal(vec![editor_button(
        "increment",
        "plus",
        &format!("+ {} ({})", labels.counter, projection.counter),
        "incrementCounter",
    )]);
    let commit_row = ui_stack_horizontal(vec![
        editor_button("commit", "git-commit", labels.commit, "commitCheckpoint"),
        editor_button("new-alternative", "git-branch", labels.branch, "createAlternative"),
    ]);
    let history_row = ui_stack_horizontal(vec![
        editor_button("undo", "undo", labels.undo, "undo"),
        editor_button("redo", "redo", labels.redo, "redo"),
    ]);
    let summary = ui_stack_vertical(vec![
        ui_text(format!("{} · {} {}", projection.title, labels.counter, projection.counter)),
        ui_text(if projection.notes.is_empty() { "—".to_string() } else { projection.notes.clone() }),
    ]);
    ui_stack_vertical(vec![heading, increment_row, commit_row, history_row, summary])
}

fn render_history(envelope: &VcsDemoEnvelope) -> UiNode {
    let columns = vcs::build_history_columns(envelope);
    build_vcs_history_scene(
        VCS_PLAY_SURFACE_HISTORY,
        VCS_PLAY_APP_ID,
        VcsHistoryScene {
            columns_json: serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into()),
        },
    )
}
//#endregion 🔖Render

//#region 🔖VcsPlayApp
#[derive(Default)]
struct VcsPlayApp;

impl PluginApp for VcsPlayApp {
    fn app_id(&self) -> &str {
        VCS_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("vcs envelope json")
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let mut store = store_from_envelope(&play);
        match action {
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

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        let materialized = materialized_projection(&play);
        let labels = vcs_labels(view_state);
        match body_key {
            VCS_PLAY_BODY_EDITOR => render_editor(&materialized, labels),
            VCS_PLAY_BODY_HISTORY => render_history(&play.envelope),
            VCS_PLAY_BODY_DOCUMENT => build_document_tree(&play.envelope, &play.selected_checkpoint_ids, labels),
            VCS_PLAY_BODY_INSPECTION => build_inspection_tree(&materialized, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = vcs_labels(view_state);
        semio_framework_plugin::AppLabelsOverlay {
            app_label: None,
            window_kind_labels: std::collections::HashMap::from([
                (VCS_PLAY_WINDOW_EDITOR.to_string(), labels.window_editor.to_string()),
                (VCS_PLAY_WINDOW_HISTORY.to_string(), labels.window_history.to_string()),
            ]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::new(),
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
            .window_kind(VCS_PLAY_WINDOW_EDITOR, "Editor", VCS_PLAY_BODY_EDITOR, SurfaceKind::Canvas2d)
            .window_kind(VCS_PLAY_WINDOW_HISTORY, "History", VCS_PLAY_BODY_HISTORY, SurfaceKind::VcsHistory)
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, VCS_PLAY_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, VCS_PLAY_BODY_INSPECTION)
            .operation("incrementCounter", "Increment Counter")
            .operation("commitCheckpoint", "Commit Checkpoint")
            .operation("createAlternative", "Create Alternative")
            .operation("switchAlternative", "Switch Alternative")
            .operation("checkoutCheckpoint", "Checkout Checkpoint")
            .operation("patchProjection", "Patch Projection")
            .operation("textEdit", "Edit Text")
            .operation("edit", "Edit")
            .view_action("setSelection", "Set Selection")
            .view_action("noop", "No-op")
            .view_action("canvasPointerDown", "Canvas Pointer Down")
            .view_action("canvasPointerMove", "Canvas Pointer Move")
            .view_action("canvasPointerUp", "Canvas Pointer Up")
            .view_action("canvasWheel", "Canvas Wheel")
            .shell_action("setDocument", "Set Document")
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

fn register_vcs_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "vcs", label: "VCS", version: "0.1.0",
    setup: register_vcs_exports,
    apps: [ create_vcs_app => VcsPlayApp ],
}
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
        assert!(!json.contains("text-editor"), "editor must no longer be a raw JSON editor: {json}");
        for action in ["incrementCounter", "commitCheckpoint", "undo", "redo", "createAlternative"] {
            assert!(json.contains(action), "missing editor button for {action}: {json}");
        }
        assert!(json.contains(" · Counter "), "missing title/counter summary: {json}");
    }

    #[test]
    fn renders_history_scene() {
        let app = VcsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(VCS_PLAY_BODY_HISTORY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("vcs-history"), "missing vcs-history surface kind: {json}");
        assert!(json.contains("lane"), "missing lane field in history columns: {json}");
        assert!(!json.contains("\"table\""), "history must not fall back to a generic table: {json}");
    }

    #[test]
    fn seeded_history_has_checkpoints() {
        let envelope = default_envelope();
        assert!(envelope.envelope.vcs.alternatives.len() >= 5, "expected >=5 alternatives, got {}", envelope.envelope.vcs.alternatives.len());
        assert!(envelope.envelope.vcs.checkpoints.len() >= 14, "expected >=14 checkpoints, got {}", envelope.envelope.vcs.checkpoints.len());
        let mut children_by_parent: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for checkpoint in &envelope.envelope.vcs.checkpoints {
            if let Some(parent_id) = &checkpoint.parent_id {
                *children_by_parent.entry(parent_id.clone()).or_insert(0) += 1;
            }
        }
        assert!(children_by_parent.values().any(|count| *count >= 2), "seed must contain a real fork (a checkpoint with >=2 children)");
        let lanes: std::collections::HashSet<usize> =
            vcs::build_history_columns(&envelope.envelope).into_iter().map(|column| column.lane).collect();
        assert!(lanes.len() >= 3, "expected >=3 distinct swimlanes, got {lanes:?}");
    }

    #[test]
    fn checkout_then_commit_forks_across_actions() {
        let mut app = VcsPlayApp;
        let document = app.initial_document_json();
        let play = parse_envelope(&document);
        let root_checkpoint_id = play.envelope.vcs.checkpoints[0].id.clone();
        let children_of_root_before = play
            .envelope
            .vcs
            .checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(root_checkpoint_id.as_str()))
            .count();

        let checkout_ops = app.handle_action_patch_ops(
            "checkoutCheckpoint",
            Some(&json!({ "checkpointId": root_checkpoint_id })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(checkout_ops.len(), 1);
        let payload: Value = serde_json::from_str(&checkout_ops[0]).unwrap();
        let after_checkout = serde_json::to_string(&payload["document"]).unwrap();

        let increment_ops = app.handle_action_patch_ops("incrementCounter", None, &after_checkout, &ViewState::default());
        assert_eq!(increment_ops.len(), 1);
        let payload: Value = serde_json::from_str(&increment_ops[0]).unwrap();
        let after_increment = serde_json::to_string(&payload["document"]).unwrap();

        let commit_ops = app.handle_action_patch_ops(
            "commitCheckpoint",
            Some(&json!({ "message": "forked from root" })),
            &after_increment,
            &ViewState::default(),
        );
        assert_eq!(commit_ops.len(), 1);
        let payload: Value = serde_json::from_str(&commit_ops[0]).unwrap();
        let next: VcsPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let children_of_root_after = next
            .envelope
            .vcs
            .checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(root_checkpoint_id.as_str()))
            .count();
        assert_eq!(
            children_of_root_after,
            children_of_root_before + 1,
            "checking out the root then committing through actions must add a new fork of the root, not extend the trunk"
        );
    }

    #[test]
    fn increment_counter_action_updates_projection() {
        let mut app = VcsPlayApp;
        let document = app.initial_document_json();
        let before = materialized_projection(&parse_envelope(&document)).counter;
        let ops = app.handle_action_patch_ops("incrementCounter", None, &document, &ViewState::default());
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
    fn text_edit_action_persists_projection_changes() {
        let mut app = VcsPlayApp;
        let document = app.initial_document_json();
        let before = materialized_projection(&parse_envelope(&document));
        let mut edited = before.clone();
        edited.title = "Edited via JSON".into();
        edited.counter = before.counter + 41;
        edited.tags.push("edited-in-place".into());
        let text = serde_json::to_string_pretty(&edited).unwrap();
        let ops = app.handle_action_patch_ops("textEdit", Some(&json!({ "text": text })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: VcsPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        let after = materialized_projection(&next);
        assert_eq!(after.title, "Edited via JSON");
        assert_eq!(after.counter, before.counter + 41);
        assert!(after.tags.contains(&"edited-in-place".to_string()));
    }

    #[test]
    fn vcs_labels_resolve_native_english_by_default() {
        let app = VcsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(VCS_PLAY_BODY_EDITOR, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Actions"));
        assert!(json.contains("Commit"));
        assert!(json.contains("Branch"));
        assert!(json.contains("Undo"));
        assert!(json.contains("Redo"));
        assert!(json.contains("Counter"));

        let inspection = app.render(VCS_PLAY_BODY_INSPECTION, &document, &ViewState::default());
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("Title"));
        assert!(inspection_json.contains("Status"));
        assert!(inspection_json.contains("Notes"));
        assert!(inspection_json.contains("Tags"));

        let document_tree = app.render(VCS_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Alternatives"));
        assert!(document_json.contains("checkpoints"));
    }

    #[test]
    fn vcs_labels_resolve_german_locale() {
        let app = VcsPlayApp;
        let document = app.initial_document_json();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };

        let node = app.render(VCS_PLAY_BODY_EDITOR, &document, &view_state);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Aktionen"));
        assert!(json.contains("Rückgängig"));
        assert!(json.contains("Wiederholen"));
        assert!(json.contains("Zähler"));

        let inspection = app.render(VCS_PLAY_BODY_INSPECTION, &document, &view_state);
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("Titel"));
        assert!(inspection_json.contains("Notizen"));
        assert!(inspection_json.contains("Schlagwörter"));

        let document_tree = app.render(VCS_PLAY_BODY_DOCUMENT, &document, &view_state);
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Alternativen"));
        assert!(document_json.contains("Checkpoints"));
        assert!(!document_json.contains("\"Alternatives\""));
    }

    #[test]
    fn edit_action_is_alias_for_text_edit() {
        let mut app = VcsPlayApp;
        let document = app.initial_document_json();
        let before = materialized_projection(&parse_envelope(&document));
        let mut edited = before.clone();
        edited.status = "reviewed".into();
        let text = serde_json::to_string(&edited).unwrap();
        let ops = app.handle_action_patch_ops("edit", Some(&json!({ "text": text })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: Value = serde_json::from_str(&ops[0]).unwrap();
        let next: VcsPlayEnvelope = serde_json::from_value(payload["document"].clone()).unwrap();
        assert_eq!(materialized_projection(&next).status, "reviewed");
    }
}
//#endregion 🧪Tests
