//! 🗂️ VCS plugin — declarative version-control play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{SurfaceKind,
    build_graph_timeline_scene, create_default_layout, is_de_locale, localized_label_map, resolve_labels, selection_ids,
    tree_item_with_action, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionEmit, App, ActionDescriptor,
    AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView, HistoryView, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder,
    ResourceKindSpec, UiButtonNode, UiFieldNode, UiInputNode, UiNode, UiStackNode,
    UiTreeItemNode, GraphTimelineScene, ViewState, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use vcs::{DocumentVcsCommand, DocumentVcsStore, Operation, OperationDiff};

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

//#region 🔖Types
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

type VcsDemoStore = DocumentVcsStore<VcsDemoProjection, VcsDemoOp>;

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
//#endregion 🔖Types

//#region 🔖DocumentHelpers
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

fn vcs_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: VCS_PLAY_APP_ID.into(),
        action: action.into(),
        args,
    }
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

/// 🌱 Seeds a rich, forked checkpoint/alternative history directly against the store — this app's
/// whole point is exercising the history UI (swimlane graph, checkpoints, alternatives, undo/redo),
/// so its "initial document" is itself a populated history, not a bare projection. Dispatched via
/// `DocumentApp::seed`, called once by `VcsDocumentApp::new` right after store construction.
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
//#endregion 🔖DocumentHelpers

//#region 🔖Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the VCS app; one field per label makes every locale combination compile-checked.
    struct VcsLabels {
        actions: &'static str = en: "Actions", de: "Aktionen";
        counter: &'static str = en: "Counter", de: "Zähler";
        commit: &'static str = en: "Commit", de: "Commit";
        branch: &'static str = en: "Branch", de: "Branch";
        undo: &'static str = en: "Undo", de: "Rückgängig";
        redo: &'static str = en: "Redo", de: "Wiederholen";
        title: &'static str = en: "Title", de: "Titel";
        status: &'static str = en: "Status", de: "Status";
        notes: &'static str = en: "Notes", de: "Notizen";
        tags: &'static str = en: "Tags", de: "Schlagwörter";
        alternatives: &'static str = en: "Alternatives", de: "Alternativen";
        no_checkpoints: &'static str = en: "(no checkpoints)", de: "(keine Checkpoints)";
        checkpoints: &'static str = en: "checkpoints", de: "Checkpoints";
        window_editor: &'static str = en: "Editor", de: "Editor";
        window_history: &'static str = en: "History", de: "Verlauf";
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_vcs_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the whole
/// builder chain; mirrors `puzzle3d_action_labels`.
fn vcs_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("incrementCounter", "Increment Counter", "Zaehler erhoehen"),
        ("patchProjection", "Patch Projection", "Projektion aktualisieren"),
        ("textEdit", "Edit Text", "Text bearbeiten"),
        ("edit", "Edit", "Bearbeiten"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("noop", "No-op", "Keine Aktion"),
        ("canvasPointerDown", "Canvas Pointer Down", "Leinwand-Zeiger gedrueckt"),
        ("canvasPointerMove", "Canvas Pointer Move", "Leinwand-Zeiger bewegt"),
        ("canvasPointerUp", "Canvas Pointer Up", "Leinwand-Zeiger losgelassen"),
        ("canvasWheel", "Canvas Wheel", "Leinwand-Mausrad"),
    ];
    localized_label_map(is_de, ENTRIES)
}

/// 🗣️ (utility id) -> localized toolbar-button label, for every `.utility(...)` declared in `create_vcs_app`;
/// currently empty since this manifest declares no utilities, kept for parity with `puzzle3d_utility_labels`.
fn vcs_utility_labels(_is_de: bool) -> std::collections::HashMap<String, String> {
    std::collections::HashMap::new()
}
//#endregion 🔖CommandLabels

//#region 🔖Panels
/// 🌳 Builds the document tree's checkpoint + alternative sections from `HistoryView` alone — the
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
            ..tree_item_with_action(
                builder.item_id("checkpoint", &column.checkpoint_id),
                column.description.clone().unwrap_or_else(|| column.checkpoint_id.clone()),
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
                ..tree_item_with_action(
                    builder.item_id("alternative", alternative_id),
                    alternative_id.clone(),
                    Some(format!("{count} {}", labels.checkpoints)),
                    vcs_action("switchAlternative", Some(json!({ "alternativeId": alternative_id }))),
                )
            }
        })
        .collect();
    let selected_ids: Vec<String> = selected.iter().map(|id| builder.item_id("checkpoint", id)).collect();
    builder
        .section_or_placeholder(
            "vcs-play-document.checkpoints",
            Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            true,
            checkpoint_items,
            labels.no_checkpoints,
        )
        .section("vcs-play-document.alternatives", Some(labels.alternatives.into()), true, alternative_items)
        .selected(selected_ids)
        .selection_change(vcs_action("setSelection", None))
        .build()
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
        loading: None,
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
        loading: None,
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

fn render_history(history: &HistoryView) -> UiNode {
    build_graph_timeline_scene(
        VCS_PLAY_SURFACE_HISTORY,
        VCS_PLAY_APP_ID,
        GraphTimelineScene {
            columns_json: serde_json::to_string(&history.columns).unwrap_or_else(|_| "[]".into()),
        },
    )
}
//#endregion 🔖Render

//#region 🔖VcsPlayApp
/// 🎛️ Ephemeral view state: the multi-selected checkpoint ids in the document tree.
#[derive(Default)]
struct VcsPlayApp {
    selected_checkpoint_ids: Vec<String>,
}

impl DocumentApp for VcsPlayApp {
    type Projection = VcsDemoProjection;
    type Op = VcsDemoOp;

    fn app_id(&self) -> &str {
        VCS_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        VCS_DEMO_SCHEMA
    }

    fn initial_projection(&self) -> VcsDemoProjection {
        empty_vcs_demo_projection()
    }

    fn seed(&self, store: &mut DocumentVcsStore<VcsDemoProjection, VcsDemoOp>) {
        seed_vcs_demo_history(store);
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, VcsDemoProjection>,
        _view_state: &ViewState,
    ) -> ActionEmit<VcsDemoOp> {
        // "undo"/"redo"/"commitCheckpoint"/"createAlternative"/"switchAlternative"/"checkoutCheckpoint"
        // never reach here — `VcsDocumentApp` intercepts those six history actions before calling
        // `handle_action`, dispatching them straight to `DocumentVcsCommand`.
        match action {
            "setSelection" => {
                self.selected_checkpoint_ids = selection_ids(args);
                ActionEmit::default()
            }
            "incrementCounter" => ActionEmit::ops(vec![VcsDemoOp::SetCounter { counter: doc.projection.counter + 1 }]),
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
                match operation {
                    Some(operation) => ActionEmit::ops(vec![operation]),
                    None => ActionEmit::default(),
                }
            }
            "textEdit" | "edit" => {
                if let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) {
                    if let Ok(next_projection) = serde_json::from_str::<VcsDemoProjection>(text) {
                        let operations = vcs_demo_projection_diff_ops(doc.projection, &next_projection);
                        if !operations.is_empty() {
                            return ActionEmit::ops(operations);
                        }
                    }
                }
                ActionEmit::default()
            }
            "noop" | "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" | "canvasWheel" => ActionEmit::default(),
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, VcsDemoProjection>, view_state: &ViewState) -> UiNode {
        let labels = resolve_labels::<VcsLabels>(view_state);
        match body_key {
            VCS_PLAY_BODY_EDITOR => render_editor(doc.projection, labels),
            VCS_PLAY_BODY_HISTORY => render_history(doc.history),
            VCS_PLAY_BODY_DOCUMENT => build_document_tree(doc.history, &self.selected_checkpoint_ids, labels),
            VCS_PLAY_BODY_INSPECTION => build_inspection_tree(doc.projection, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<VcsLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(VCS_PLAY_WINDOW_EDITOR, labels.window_editor)
            .window_kind_label(VCS_PLAY_WINDOW_HISTORY, labels.window_history)
            .action_labels(vcs_action_labels(is_de))
            .utility_labels(vcs_utility_labels(is_de))
    }
}
//#endregion 🔖VcsPlayApp

//#region 🔖Manifest
fn create_vcs_app() -> App {
    App::from_builder(
        App::builder(VCS_PLAY_APP_ID, "VCS").document(["semio", "vcs"])
            .resource_kind(ResourceKindSpec {
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
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(VCS_PLAY_WINDOW_EDITOR, "Editor", VCS_PLAY_BODY_EDITOR, SurfaceKind::Canvas2d)
            .window_kind(VCS_PLAY_WINDOW_HISTORY, "History", VCS_PLAY_BODY_HISTORY, SurfaceKind::GraphTimeline)
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, VCS_PLAY_BODY_DOCUMENT)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, VCS_PLAY_BODY_INSPECTION)
            .operation("incrementCounter", "Increment Counter")
            .operation("patchProjection", "Patch Projection")
            .operation("textEdit", "Edit Text")
            .operation("edit", "Edit")
            .view_action("setSelection", "Set Selection")
            .view_action("noop", "No-op")
            .view_action("canvasPointerDown", "Canvas Pointer Down")
            .view_action("canvasPointerMove", "Canvas Pointer Move")
            .view_action("canvasPointerUp", "Canvas Pointer Up")
            .view_action("canvasWheel", "Canvas Wheel")
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
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};
    use vcs::{DocumentVcsEnvelope, HistoryColumn};

    /// 📦 Parses `document_json()` (the full envelope) for tests that need to inspect raw
    /// checkpoints/alternatives directly — safe here because none of these tests undo/redo, so every
    /// edit in the log is still applied.
    fn seeded_envelope(app: &VcsDocumentApp<VcsPlayApp>) -> DocumentVcsEnvelope<VcsDemoProjection, VcsDemoOp> {
        serde_json::from_str(&app.document_json().expect("document json")).expect("envelope")
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
        let lanes: std::collections::HashSet<usize> =
            vcs::build_history_columns(&envelope).into_iter().map(|column: HistoryColumn| column.lane).collect();
        assert!(lanes.len() >= 3, "expected >=3 distinct swimlanes, got {lanes:?}");
    }

    #[test]
    fn checkout_then_commit_forks_across_actions() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let envelope_before = seeded_envelope(&app);
        let root_checkpoint_id = envelope_before.vcs.checkpoints[0].id.clone();
        let children_of_root_before = envelope_before
            .vcs
            .checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(root_checkpoint_id.as_str()))
            .count();

        let checkout = app.handle_action("checkoutCheckpoint", Some(&json!({ "checkpointId": root_checkpoint_id })), &ViewState::default(), &testkit::meta("local")).expect("checkout");
        assert!(checkout.operations.is_empty(), "history actions never emit KernelOperations");

        app.handle_action("incrementCounter", None, &ViewState::default(), &testkit::meta("local")).expect("increment");
        app.handle_action("commitCheckpoint", Some(&json!({ "message": "forked from root" })), &ViewState::default(), &testkit::meta("local")).expect("commit");

        let envelope_after = seeded_envelope(&app);
        let children_of_root_after = envelope_after
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
        let mut app = testkit::new_app::<VcsPlayApp>();
        let before = app.projection().expect("materialize projection").counter;
        let result = app.handle_action("incrementCounter", None, &ViewState::default(), &testkit::meta("local")).expect("increment");
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
        let result = app.handle_action("textEdit", Some(&json!({ "text": text })), &ViewState::default(), &testkit::meta("local")).expect("text edit");
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

    #[test]
    fn vcs_labels_resolve_german_locale() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };

        let node = app.render(VCS_PLAY_BODY_EDITOR, None, &view_state).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Aktionen"));
        assert!(json.contains("Rückgängig"));
        assert!(json.contains("Wiederholen"));
        assert!(json.contains("Zähler"));

        let inspection = app.render(VCS_PLAY_BODY_INSPECTION, None, &view_state).expect("render");
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("Titel"));
        assert!(inspection_json.contains("Notizen"));
        assert!(inspection_json.contains("Schlagwörter"));

        let document_tree = app.render(VCS_PLAY_BODY_DOCUMENT, None, &view_state).expect("render");
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
        let result = app.handle_action("edit", Some(&json!({ "text": text })), &ViewState::default(), &testkit::meta("local")).expect("edit");
        assert!(!result.operations.is_empty());
        assert_eq!(app.projection().expect("materialize projection").status, "reviewed");
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let before = app.projection().expect("materialize projection").counter;
        app.handle_action("incrementCounter", None, &ViewState::default(), &testkit::meta("local")).expect("increment");
        assert_eq!(app.projection().expect("materialize projection").counter, before + 1);
        let undo = app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert!(undo.operations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.projection().expect("materialize projection").counter, before);
        app.handle_action("redo", None, &ViewState::default(), &testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("materialize projection").counter, before + 1);
    }

    #[test]
    fn create_and_switch_alternative_round_trip_through_the_wrapper() {
        let mut app = testkit::new_app::<VcsPlayApp>();
        let create = app.handle_action("createAlternative", Some(&json!({ "name": "trying-something" })), &ViewState::default(), &testkit::meta("local")).expect("create alternative");
        assert!(create.operations.is_empty());
        let envelope = seeded_envelope(&app);
        assert!(envelope.active_alternative_id.is_some(), "createAlternative must set an active alternative");
    }
}
//#endregion 🧪Tests
