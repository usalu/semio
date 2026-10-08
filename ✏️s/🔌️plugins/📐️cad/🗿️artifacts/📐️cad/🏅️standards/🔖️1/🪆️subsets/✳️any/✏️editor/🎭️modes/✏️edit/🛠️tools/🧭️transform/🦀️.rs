//! 🧭️ CAD edit-mode tool — Transform: the `🔄️machine` statechart (effects `ToolYield<CadToolLeaf>`, driven by the
//! `🛠️tool-machine` runner) every object gesture commits through. A gumball translate / rotate / scale and the commit of
//! every `spatial.interaction` that lands objects (a construction, `transform.move`/`copy`/`rotate`/`scale*`, a mirror)
//! enter as [`CadToolEntry`]s and leave as ONE `ToolTransaction` of child-lane leaves on the panes' composed
//! `s.stdio.semio@v1/model` children: one relative `drag-`/`rotate-`/`scale-elements` per touched pane plus the
//! `insert-element`s a construction or copy lands (design §12, §20.15). Tool state is never history; the yielded leaves are
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5).
#![allow(unexpected_cfgs)]

use crate::editor::cad::config::CadConfigMutation;
use crate::{cad_pane_model, cad_pane_model_slot, CadMutation, CadPaneId, CadSnapshot};
use machine::Command;
use semio_framework_diagnostic::{Fault, FaultFrom};
use semio_framework_plugin::app::ChildEmitPreparation;
use semio_framework_plugin::{ArtifactView, ChildContentView, Emit};
use std::collections::VecDeque;
use semio_framework_tool_machine::{ToolMachineRunner, ToolStep, ToolYield};
use semio_s_artifact_stdio_semio::standards::v1::subsets::model::schema::mutations::{drag_elements::DragElements, insert_element::InsertElement, rotate_elements::RotateElements, scale_elements::ScaleElements, SemioModelMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::model::schema::snapshot::{SemioModelElement, SemioModelSnapshot};
use std::sync::Arc;

/// 🪪️ The editor app id every transform-tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const CAD_EDITOR_APP_ID: &str = "s.cad.cad@1/*#editor";

//#region 🎬️Record
/// 🎬️ How one transform moves its objects — the parameters of the leaf it yields.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CadTransformMotion {
    Drag { offset: [f64; 3] },
    Rotate { axis: [f64; 3], angle: f64 },
    Scale { factors: [f64; 3] },
}

/// 🎬️ One object transform the tool yields: the literal target ids, deduplicated in first-seen order, and the motion.
#[derive(Clone, Debug, PartialEq)]
pub struct CadTransformRecord {
    pub targets: Vec<String>,
    pub motion: CadTransformMotion,
}

impl CadTransformRecord {
    /// ✋️ A drag of `targets` by `offset` — the gumball translate, a `transform.move` commit, an inspector origin delta.
    pub fn drag(targets: impl IntoIterator<Item = String>, offset: [f64; 3]) -> Self {
        Self { targets: cad_unique_targets(targets), motion: CadTransformMotion::Drag { offset } }
    }

    /// 🔄️ A turn of `targets`, each in place, about `axis` by `angle` radians.
    pub fn rotate(targets: impl IntoIterator<Item = String>, axis: [f64; 3], angle: f64) -> Self {
        Self { targets: cad_unique_targets(targets), motion: CadTransformMotion::Rotate { axis, angle } }
    }

    /// 🔍️ A scaling of `targets`, each in place, by `factors`.
    pub fn scale(targets: impl IntoIterator<Item = String>, factors: [f64; 3]) -> Self {
        Self { targets: cad_unique_targets(targets), motion: CadTransformMotion::Scale { factors } }
    }

    /// 🎚️ Whether the motion is one a leaf admits and that moves: finite numbers, a non-zero axis, positive factors,
    /// and never the identity.
    pub fn moves(&self) -> bool {
        match self.motion {
            CadTransformMotion::Drag { offset } => offset.iter().all(|component| component.is_finite()) && offset != [0.0; 3],
            CadTransformMotion::Rotate { axis, angle } => axis.iter().all(|component| component.is_finite()) && axis != [0.0; 3] && angle.is_finite() && angle != 0.0,
            CadTransformMotion::Scale { factors } => factors.iter().all(|factor| factor.is_finite() && *factor > 0.0) && factors != [1.0; 3],
        }
    }

    /// 🧮️ The child leaf this record yields over one pane's share of its targets.
    pub fn leaf(&self, targets: Vec<String>) -> SemioModelMutation {
        match self.motion {
            CadTransformMotion::Drag { offset } => SemioModelMutation::DragElements(DragElements { targets, offset }),
            CadTransformMotion::Rotate { axis, angle } => SemioModelMutation::RotateElements(RotateElements { targets, axis, angle }),
            CadTransformMotion::Scale { factors } => SemioModelMutation::ScaleElements(ScaleElements { targets, factors }),
        }
    }
}

/// 🧱️ One entry of a transform-tool request: an object transform, or one element a construction or a copy lands in a
/// pane's model child.
#[derive(Clone, Debug, PartialEq)]
pub enum CadToolEntry {
    Transform(CadTransformRecord),
    Create { pane: CadPaneId, element: SemioModelElement },
}

/// 🧱️ One leaf the transform tool yields: a child-lane leaf on `pane`'s composed model child.
#[derive(Clone, Debug, PartialEq)]
pub struct CadToolLeaf {
    pub pane: CadPaneId,
    pub leaf: SemioModelMutation,
}

/// 🧹️ `targets` without repeats, in first-seen order — the one target list every leaf the tool yields carries.
pub fn cad_unique_targets(targets: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    targets.into_iter().filter(|id| seen.insert(id.clone())).collect()
}
//#endregion 🎬️Record

//#region 🪆️PaneModels
/// 🪆️ The composed model child of every pane that has one, in pane order: the pane, its child id and its content now —
/// what every object edit is decided against and lands in (design §20.15).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CadPaneModels(pub Vec<(CadPaneId, String, SemioModelSnapshot)>);

impl CadPaneModels {
    /// 🔎️ The pane whose model holds `element_id` — element ids are the `"cad"` domain's own object ids, unique across panes.
    pub fn pane_of(&self, element_id: &str) -> Option<CadPaneId> {
        self.0.iter().find(|(_, _, model)| model.elements.iter().any(|element| element.id == element_id)).map(|(pane, _, _)| *pane)
    }

    /// 🔎️ `pane`'s model content.
    pub fn model(&self, pane: CadPaneId) -> Option<&SemioModelSnapshot> {
        self.0.iter().find(|(candidate, _, _)| *candidate == pane).map(|(_, _, model)| model)
    }
}

/// 🪆️ Reads every pane's composed `s.stdio.semio@v1/model` child out of `children`; a pane without a handle, or whose
/// child is not composed as that dialect, contributes nothing.
pub fn cad_pane_models(snapshot: &CadSnapshot, children: &ChildContentView) -> CadPaneModels {
    CadPaneModels(
        CadPaneId::all()
            .into_iter()
            .filter_map(|pane| {
                let child_id = cad_pane_model(snapshot, pane)?.child_id.clone();
                let slot = cad_pane_model_slot(pane);
                let dialect = children.dialect(slot, &child_id)?;
                if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "model" {
                    return None;
                }
                let model = children.typed_read::<SemioModelSnapshot>(slot, &child_id).ok()?.clone();
                Some((pane, child_id, model))
            })
            .collect(),
    )
}

/// 📮️ `leaves` as one edit per touched pane child, in pane order, each pane's leaves in their own order — stamped with
/// `transaction` (ONE composite group, one history row, design §12) when a committed tool transaction yielded them, a
/// plain child edit otherwise; nothing touched is the empty emit.
pub fn cad_child_leaves_emit(models: &CadPaneModels, transaction: Option<protocol::TransactionRef>, leaves: Vec<CadToolLeaf>) -> Emit<CadMutation, CadConfigMutation> {
    let mut remaining = leaves;
    let child_preparations: VecDeque<ChildEmitPreparation> = models
        .0
        .iter()
        .filter_map(|(pane, child_id, _)| {
            let (owned, rest): (Vec<CadToolLeaf>, Vec<CadToolLeaf>) = std::mem::take(&mut remaining).into_iter().partition(|leaf| leaf.pane == *pane);
            remaining = rest;
            let ops: Vec<SemioModelMutation> = owned.into_iter().map(|leaf| leaf.leaf).collect();
            (!ops.is_empty()).then(|| ChildEmitPreparation::of_owned::<SemioModelSnapshot, SemioModelMutation>(cad_pane_model_slot(*pane), child_id.clone(), ops))
        })
        .collect();
    match child_preparations.is_empty() {
        true => Emit::default(),
        false => Emit { child_preparations, transaction, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() },
    }
}
//#endregion 🪆️PaneModels

//#region 🛠️TransformTool
/// 📨️ What one transform-tool event carries: the composed pane models it yields against and the entries — dispatch
/// inputs, never tool state.
#[derive(Clone, Debug)]
pub struct CadToolRequest {
    pub models: Arc<CadPaneModels>,
    pub entries: Vec<CadToolEntry>,
}

/// 🧰️ The transform tool's context: empty, because every CAD object gesture reaches the guest as ONE record at its
/// release (the hosts preview the drag locally) — tool state, never history.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CadTransformToolContext;

fn cad_transform_tool_context(input: CadTransformToolContext) -> CadTransformToolContext {
    input
}

fn entries_apply(_context: &CadTransformToolContext, event: Option<&cad_transform_tool::Event>) -> bool {
    matches!(event, Some(cad_transform_tool::Event::Records(request)) if !cad_tool_yields(&request.models, &request.entries).is_empty())
}

fn yield_entries(_context: &mut CadTransformToolContext, event: Option<&cad_transform_tool::Event>, sink: &mut Vec<Command<cad_transform_tool::CadTransformTool>>) {
    let Some(cad_transform_tool::Event::Records(request)) = event else { return };
    sink.extend(cad_tool_yields(&request.models, &request.entries).into_iter().map(|(key, leaf)| Command::Effect(ToolYield::upsert(key, leaf))));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine cad_transform_tool {
        context: CadTransformToolContext;
        event Event { Records(CadToolRequest) }
        input: CadTransformToolContext;
        output: ();
        effect: ToolYield<CadToolLeaf>;
        context_from_input: cad_transform_tool_context;
        initial: idle;
        state idle {
            on Records if entries_apply => idle do yield_entries;
        }
    }
}

/// 🧷️ The transform tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct CadTransformToolHost;

impl machine::Host<cad_transform_tool::CadTransformTool> for CadTransformToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<CadToolLeaf>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🧮️ What the tool yields for `entries` on `models`, keyed and in order: each transform's leaf per touched pane, then
/// each created element — every one folded onto its pane's running model, so a later entry sees the earlier ones. A
/// leaf whose outcome is a no-op, an Error or a Fatal on that model is not yielded (the tool never commits an edit it
/// knows fails), and an entry for a pane without a model child lands nothing.
pub fn cad_tool_yields(models: &CadPaneModels, entries: &[CadToolEntry]) -> Vec<(String, CadToolLeaf)> {
    let mut states: Vec<(CadPaneId, SemioModelSnapshot)> = models.0.iter().map(|(pane, _, model)| (*pane, model.clone())).collect();
    let mut yields = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        let leaves: Vec<(String, CadPaneId, SemioModelMutation)> = match entry {
            CadToolEntry::Transform(record) if record.moves() => states
                .iter()
                .filter_map(|(pane, state)| {
                    let owned: Vec<String> = record.targets.iter().filter(|id| state.elements.iter().any(|element| element.id == **id)).cloned().collect();
                    (!owned.is_empty()).then(|| (format!("transform:{index}:{}", pane.model_definition_id()), *pane, record.leaf(owned)))
                })
                .collect(),
            CadToolEntry::Transform(_) => Vec::new(),
            CadToolEntry::Create { pane, element } => vec![(format!("create:{index}"), *pane, SemioModelMutation::InsertElement(InsertElement { element: element.clone() }))],
        };
        for (key, pane, leaf) in leaves {
            let Some((_, state)) = states.iter_mut().find(|(candidate, _)| *candidate == pane) else { continue };
            let outcome = <SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::diff(&leaf, state);
            if outcome.messages().iter().any(|message| message.level >= semio_framework_diagnostic::Severity::Error || message.code.0 == "mutation.no-op") {
                continue;
            }
            let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), state) else { continue };
            *state = next;
            yields.push((key, CadToolLeaf { pane, leaf }));
        }
    }
    yields
}

/// 🛠️ Runs `entries` through a transform tool at rest as ONE one-shot transaction — the ref minted from the admission's
/// `authoring_seed`, the host clock and `<appId>#<verb>`, and the yielded leaves in order. `None` when nothing moves or
/// lands: a stranger id, an identity motion or an empty request leaves zero trace.
pub fn cad_transform_tool_commit(verb: &str, authoring_seed: &str, models: &CadPaneModels, entries: Vec<CadToolEntry>) -> Option<(protocol::TransactionRef, Vec<CadToolLeaf>)> {
    let mut runner = ToolMachineRunner::<cad_transform_tool::CadTransformTool, CadTransformToolHost>::start(format!("{CAD_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), CadTransformToolContext, CadTransformToolHost).ok()?;
    match runner.send(cad_transform_tool::Event::Records(CadToolRequest { models: Arc::new(models.clone()), entries }), semio_framework_tool_machine::authoring_clock(0)).ok()? {
        ToolStep::Committed(transaction, leaves) => Some((transaction, leaves)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    }
}

/// 🧾️ The emit one committed transform-tool transaction publishes on the panes' model children: ONE composite group
/// whose member edits carry its ref, labelled from their leaves. A view without command authority (no authoring seed)
/// publishes the yielded leaves plainly; nothing yielded is the empty emit.
pub fn cad_transform_tool_emit(doc: &ArtifactView<'_, CadSnapshot>, verb: &str, entries: Vec<CadToolEntry>) -> Emit<CadMutation, CadConfigMutation> {
    let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
    let models = cad_pane_models(doc.snapshot, &doc.children);
    match cad_transform_tool_commit(verb, authoring_seed, &models, entries) {
        Some((transaction, leaves)) if !authoring_seed.is_empty() => cad_child_leaves_emit(&models, Some(transaction), leaves),
        Some((_, leaves)) => cad_child_leaves_emit(&models, None, leaves),
        None => Emit::default(),
    }
}

/// 🧊️ Publishes actual topology and its model reference in the same exact tool transaction.
pub fn cad_import_object_emit(doc: &ArtifactView<'_, CadSnapshot>, pane: CadPaneId, verb: &str, mut imported: crate::standards::v1::subsets::any::io::CadImportedObject) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::{schema::{snapshot::SemioBrepSnapshot, mutations::{SemioBrepMutation, set_snapshot::SetSnapshot}}};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::model::schema::snapshot::GeometryRef;
    let refused = |message| Fault::new(semio_framework_diagnostic::FaultOrigin::App, semio_framework_diagnostic::FaultCode::new("cad.import-object-refused"), message);
    let operation = doc.operation_optional().filter(|operation| !operation.authoring_seed.is_empty()).ok_or_else(|| refused("geometry import requires exact operation authoring authority"))?;
    let id = semio_framework_os_kernel::content_id("cad-geometry", format!("{}:{pane:?}", operation.authoring_seed).as_bytes());
    let child_id = format!("brep-{id}");
    if doc.snapshot.breps.iter().any(|child| child.child_id == child_id) { return Err(refused("geometry import requires a fresh topology identity")); }
    let index = u32::try_from(doc.snapshot.breps.len()).map_err(|_| refused("topology sibling count exceeds the portable u32 domain"))?;
    imported.element.id = format!("object-{id}");
    imported.element.geometry = GeometryRef::Brep { brep_id: child_id.clone() };
    let mut emit = cad_transform_tool_emit(doc, verb, vec![CadToolEntry::Create { pane, element: imported.element }]);
    if emit.child_preparations.is_empty() {
        return Err(refused("geometry import requires an available composed model"));
    }
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "brep".into() } };
    let genesis=semio_framework_plugin::app::ChildEmitGenesis{reference:target.clone(),initial_pack:<SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&SemioBrepSnapshot::default())};
    emit.artifact_mutations.push(CadMutation::CreateBrep(crate::mutations::create_brep::CreateBrep { child_id: child_id.clone(), target, index }));
    emit.child_preparations.push_front(ChildEmitPreparation::with_genesis::<SemioBrepSnapshot, SemioBrepMutation>("breps", child_id, genesis, vec![SemioBrepMutation::SetSnapshot(SetSnapshot { snapshot: imported.geometry })]));
    Ok(emit)
}
//#endregion 🛠️TransformTool

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
