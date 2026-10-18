//! ⏪️ OS runtime of non-destructive history editing: the per-instance [`TimeTravelLedger`] hosting the pure
//! `⏪️time-travel` session reducer, the host-driven `historyEdit*` verbs, the session preview every render seam reads,
//! the Report replay stepped per reactor turn, the finalize commit, the per-mutation history overlay and the history
//! panel's time-travel sections.
//!
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §4, §7 and §10.
//! Domain-neutral session: `semio_framework_time_travel`; store read API: `ArtifactStore::{state_before,
//! begin_report_replay, replay_report, commit_finished_replay}`.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;
use semio_framework::ActionArgDef;
use semio_framework_plugin_decl_app::ArtifactApp;
use pack::snapshot::ArtifactPack;
use store::ArtifactStore;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use semio_framework_value::DslValue;
use semio_framework_value_derive::FromValue;
use std::collections::HashMap;
use std::collections::HashSet;
use semio_framework::kernel::HybridLogicalTimestamp;
use semio_framework_ui_locale::LocalizedLabel;
use store::Mutation;
use protocol::OpBinary;
use protocol::OpText;
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use store::MutationDiff;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;
use semio_framework::kernel::{HistoryReprojectionKind, HistoryTimeTravel, HistoryTimeTravelProblem, HistoryTimeTravelReview, HistoryTimeTravelStage, RequestId};
use semio_framework::{mutation_input_defs, reference_id_text, registered_input_schema_document, ArgSchema, InputSchemaError};
use semio_framework::history_edit::HISTORY_EDIT_ARG_GENERATION;
use semio_framework_time_travel::{TimeTravelBase, TimeTravelEvent, TimeTravelLabel, TimeTravelRefusal, TimeTravelReview, TimeTravelSession, TimeTravelStage, TimeTravelTarget, TIME_TRAVEL_BUSY_CODE, TIME_TRAVEL_EDITOR_CLOSED_CODE, TIME_TRAVEL_INVALID_INPUT_CODE, TIME_TRAVEL_NAME_INVALID_CODE, TIME_TRAVEL_NAME_REQUIRED_CODE, TIME_TRAVEL_NOT_EDITABLE_CODE, TIME_TRAVEL_NOT_WITHDRAWABLE_CODE, TIME_TRAVEL_NO_SELECTION_CODE, TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE, TIME_TRAVEL_UNKNOWN_INPUT_CODE, TIME_TRAVEL_UNKNOWN_MUTATION_CODE};
use std::collections::VecDeque;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
#[path="♻️retirement/🦀️.rs"]
mod original_retirement;
#[cfg(test)]
pub use crate::original_retirement::tests::assert_session_original;
#[cfg(test)]
pub use crate::original_retirement::tests::assert_pending_command_original;
#[cfg(test)]
pub use crate::original_retirement::tests::assert_discard_publication_original;
#[cfg(test)]
pub use crate::original_retirement::tests::native_original_actor_grant;

pub use semio_framework_plugin_time_travel_panel::panel_pointer::*;

pub use semio_framework_plugin_time_travel_panel::panel_sections::*;

pub use semio_framework_plugin_command::lifted_command::*;

//#region 🔖️Limits
/// ⏱️ Wall budget of one replay slice per reactor turn.
pub const TIME_TRAVEL_TURN_WALL_US: u64 = 4_000;
/// 📡️ Most operations one reactor turn replays of a history change before the document store adopts it — a remote change
/// or this replica's own interior undo/redo, checkout or alternative switch (design §16.6, gap N17) — the cap behind the
/// turn's wall budget ([`time_travel_replay_turn_budget`]).
pub const TIME_TRAVEL_REPLAY_OPERATIONS: usize = 256;

/// ⏱️ One reactor turn of a deferred reprojection: [`TIME_TRAVEL_TURN_WALL_US`] of the job clock first, at most
/// [`TIME_TRAVEL_REPLAY_OPERATIONS`] operations second (`ArtifactStore::defer_remote_replays`,
/// `ArtifactStore::defer_local_replays`).
pub fn time_travel_replay_turn_budget() -> store::os_store::store_schema::ReplayTurnBudget {
    store::os_store::store_schema::ReplayTurnBudget::wall(TIME_TRAVEL_TURN_WALL_US, TIME_TRAVEL_REPLAY_OPERATIONS)
}
/// 📢️ Least interval between two replay-progress deliveries (history body refresh plus `HistoryPatch.timeTravel`),
/// unless the replay advanced by [`TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE`] of its total since the last one.
pub const TIME_TRAVEL_PROGRESS_REFRESH_MS: u64 = 100;
/// 📢️ Replay advance (per mille of its total) that delivers progress before [`TIME_TRAVEL_PROGRESS_REFRESH_MS`] elapsed.
pub const TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE: u64 = 50;

/// 🧹️ Discarded draft operations cold-retired per driver or close turn.
const TIME_TRAVEL_DISCARD_OPS_PER_TURN: usize = 64;

/// 🗳️ The request id the finalize prompt opens under; nothing awaits a dialog.
#[doc(hidden)]
pub const TIME_TRAVEL_FINALIZE_REQUEST: RequestId = RequestId(0x7417_0001);

#[doc(hidden)]
pub fn history_planning_retirement_grant(demand:RetirementDemand)->RetainedCloneGrant {
    RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}

#[doc(hidden)]
pub fn history_retirement_frame_grant(bytes:usize)->RetainedCloneGrant {
    RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1}
}
//#endregion 🔖️Limits

//#region 🔖️Outcome
/// 🙅️ Why a history-edit verb changed nothing: a session refusal, or a plugin-side precondition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeTravelActionRefusal {
    Session(TimeTravelRefusal),
    Busy,
    UnknownMutation,
    NotEditable,
    UnknownInput,
    InvalidInput,
    NoSelection,
    NameRequired,
    NameInvalid,
    SchemaUnavailable,
    NotWithdrawable,
    EditorClosed,
    UnitSpansDocuments,
}

impl TimeTravelActionRefusal {
    /// 🔖️ The `rejected` code of the verb result.
    pub fn code(self) -> &'static str {
        match self {
            Self::Session(refusal) => refusal.code(),
            Self::Busy => TIME_TRAVEL_BUSY_CODE,
            Self::UnknownMutation => TIME_TRAVEL_UNKNOWN_MUTATION_CODE,
            Self::NotEditable => TIME_TRAVEL_NOT_EDITABLE_CODE,
            Self::UnknownInput => TIME_TRAVEL_UNKNOWN_INPUT_CODE,
            Self::InvalidInput => TIME_TRAVEL_INVALID_INPUT_CODE,
            Self::NoSelection => TIME_TRAVEL_NO_SELECTION_CODE,
            Self::NameRequired => TIME_TRAVEL_NAME_REQUIRED_CODE,
            Self::NameInvalid => TIME_TRAVEL_NAME_INVALID_CODE,
            Self::SchemaUnavailable => TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE,
            Self::NotWithdrawable => TIME_TRAVEL_NOT_WITHDRAWABLE_CODE,
            Self::EditorClosed => TIME_TRAVEL_EDITOR_CLOSED_CODE,
            Self::UnitSpansDocuments => HISTORY_UNIT_SPANS_DOCUMENTS_CODE,
        }
    }
}

/// 🎛️ Outcome of one history-edit verb: the stage it left the session in, or why it was refused (a silent no-op).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeTravelActionOutcome {
    Pending,
    Applied(TimeTravelStage),
    Rejected(TimeTravelActionRefusal),
}
//#endregion 🔖️Outcome

//#region 🔖️Ledger
/// ✏️ The mutation whose inputs are being drafted: its identity, label, input descriptors and the validator of its
/// payload schema, the current draft payload and the draft's own outcome against the state before it. The operation
/// whose kind the draft rebuilds lives with the typed owners of the store it belongs to ([`TimeTravelStoreState`]).
/// A mutation without editable inputs — opened by a history row's Withdraw (design §22.1) — has no `schema` and no
/// inputs: its editor admits no draft but the withdrawal. Whether the draft is withdrawn is the session's pending draft.
#[derive(semio_framework_value::RetireOwned)]
pub struct TimeTravelEditor {
    pub target: MutationId,
    pub position: u32,
    pub op_index: u32,
    pub label: LocalizedLabel,
    pub schema: Option<&'static str>,
    pub inputs: Vec<ActionArgDef>,
    pub inputs_refused: Option<InputSchemaError>,
    pub validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
    pub value: DslValue,
    pub outcome: Vec<protocol::MutationMessage>,
    pub refused: Option<(String, String)>,
}

impl TimeTravelEditor {
    /// 🧯️ Stands in for a payload schema that compiles no validator, for the fail-closed law.
    #[cfg(test)]
    pub fn refuse_validator(&mut self, reason: &str) {
        self.validator = Err(reason.to_string());
    }
}

/// 🧩️ The composed member store a session edits (design §12): its owner edge, owner path and dialect, the typed owners of the
/// edit on that store (created by the first visit, erased here), and the children view every render seam reads while
/// the member's preview or replayed head is shown.
#[doc(hidden)]
pub struct TimeTravelMemberSubject {
    pub key: MemberKey,
    pub path: MemberPath,
    pub dialect: ArtifactDialect,
    pub owners: Option<Box<dyn TimeTravelOwners>>,
    pub children: Option<ChildContentView>,
    pub review_children: Option<TimeTravelReviewChildren>,
    pub metadata_close: Option<ControlledRetirement<TimeTravelMemberMetadata>>,
    pub metadata_retired: bool,
}

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct TimeTravelMemberMetadata {pub key:MemberKey,pub path:MemberPath,pub dialect:ArtifactDialect}

#[doc(hidden)]
pub struct TimeTravelReviewChildren {
    pub view: ChildContentView,
    pub base: TimeTravelBase,
    pub generation: u32,
}

impl TimeTravelReviewChildren {
    pub fn capture(view:&ChildContentView,base:TimeTravelBase,generation:u32,grant:RetainedCloneGrant)->Option<(Self,RetainedCloneProgress)>{
        let copied_bytes=size_of::<Self>();if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return None}
        Some((Self{view:view.clone(),base,generation},RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}))
    }
}

impl TimeTravelMemberSubject {
    /// 🧩️ The member store id the history wire names it by: the text of its owner path (`<slot>/<childId>` for a member of the document).
    pub fn store(&self) -> String {
        self.path.to_string()
    }
}

/// 🕰️ The ephemeral local-only history edit of one document instance (design §4 and §7): the pure session, the draft
/// editor, and the typed owners of the store the session edits — the document's own, or one composed member's (§12):
/// the preview the renderer reads while editing (state before the target with the draft), the running Report replay,
/// its finished result (whose head is the reviewing preview and whose report finalizes without replaying again), and
/// the owners still retiring. Nothing here is persisted or shared; closing the instance discards it.
pub struct TimeTravelLedger<A: ArtifactApp> {
    pub session: TimeTravelSession,
    pub pending_command: Option<semio_framework_time_travel::TimeTravelCommandCustody>,
    pub discard_cursor: Option<Box<semio_framework_time_travel::TimeTravelDiscardCursor>>,
    pub command_retirements: semio_framework_value::retirement_queue::RetirementQueue,
    pub command_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    session_retirement: Option<ControlledRetirement<TimeTravelSession>>,
    session_retired: bool,
    pub editor: Option<TimeTravelEditor>,
    pub document: TimeTravelStoreState<A::Snapshot, A::Mutation>,
    pub member: Option<TimeTravelMemberSubject>,
    pub retiring: Vec<Box<dyn TimeTravelOwners>>,
    pub closing: bool,
    pub ui_dirty: bool,
    pub document_dirty: bool,
    pub refreshed_ms: u64,
    pub refreshed_done: u32,
    pub patch_due: bool,
    pub patch: Option<HistoryPatch>,
    pub reprojection_paused: bool,
    pub reprojection_fault: Option<(String, HistoryReprojectionKind)>,
    pub reprojection_refreshed_ms: u64,
    pub deferred_history_row: Option<String>,
    pub authoring: Option<SupersedeAuthoring<A::Snapshot, A::Mutation>>,
    pub history_unavailable: bool,
    pub turn_clock: fn() -> Option<u64>,
}

/// ✍️ The undo or redo of a finalize being authored resumably: the Report replay of its inputs over the applied history,
/// stepped per reactor turn, and how it finalizes (unscoped, or within the history edit's own line).
#[doc(hidden)]
pub struct SupersedeAuthoring<P, Mu: ::protocol::Mutation<P>> {
    pub replay: store::os_store::snapshot_read::ArtifactDerivedReplay<P, Mu>,
    pub finalization: store::HistoryFinalization,
}

/// 🧮️ Where authoring the undo or redo of a finalize stands.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupersedeAuthored {
    Landed,
    Pending,
    Refused,
    Busy,
}

impl<A: ArtifactApp> Default for TimeTravelLedger<A> {
    fn default() -> Self {
        Self {
            session: TimeTravelSession::default(),
            pending_command: None,
            discard_cursor: None,
            command_retirements: Default::default(),
            command_receipt: None,
            session_retirement: None,
            session_retired: false,
            editor: None,
            document: TimeTravelStoreState::new(time_travel_mutation_label::<A>),
            member: None,
            retiring: Vec::new(),
            closing: false,
            ui_dirty: false,
            document_dirty: false,
            refreshed_ms: 0,
            refreshed_done: 0,
            patch_due: false,
            patch: None,
            reprojection_paused: false,
            reprojection_fault: None,
            reprojection_refreshed_ms: 0,
            deferred_history_row: None,
            authoring: None,
            history_unavailable: false,
            turn_clock: semio_framework_job::default_now_us,
        }
    }
}

impl<A: ArtifactApp> TimeTravelLedger<A> {
    /// ⏱️ Replaces the clock every driver turn reads its wall deadline from (laws drive a counted clock).
    #[cfg(test)]
    pub fn set_turn_clock(&mut self, clock: fn() -> Option<u64>) {
        self.turn_clock = clock;
    }

    /// 🕰️ The pure session state.
    pub fn session(&self) -> &TimeTravelSession {
        &self.session
    }

    /// 🎟️ Transfers the original caller-funded command receipt to its mounted parent.
    pub fn take_command_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.command_receipt.take()}

    /// 📨️ Borrows whether the mounted caller still owns an unconsumed original command receipt.
    pub fn has_command_receipt(&self)->bool{self.command_receipt.is_some()}

    /// ✏️ The draft editor while a mutation is being edited.
    pub fn editor(&self) -> Option<&TimeTravelEditor> {
        self.editor.as_ref()
    }

    /// ✏️ The draft editor for a law that exercises a descriptor the fixture app's leaves do not declare.
    #[cfg(test)]
    pub fn editor_mut(&mut self) -> Option<&mut TimeTravelEditor> {
        self.editor.as_mut()
    }

    /// 🔗️ The ids every `Reference` input of the open draft holds, per selection domain the input declares (draft order,
    /// each id once) — what a render highlights while the draft targets entities; empty while no draft editor is open.
    pub fn draft_references(&self) -> BTreeMap<String, Vec<String>> {
        let mut references: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let Some(editor) = self.editor.as_ref() else { return references };
        for row in time_travel_input_rows(&editor.inputs, &editor.value) {
            let ArgSchema::Reference { domain: Some(domain), .. } = &row.input.schema else { continue };
            let ids = references.entry(domain.clone()).or_default();
            let held: Vec<String> = match &row.value {
                DslValue::Array(items) => items.iter().filter_map(reference_id_text).collect(),
                other => reference_id_text(other).into_iter().collect(),
            };
            for id in held {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
        references
    }

    /// 🚦️ Whether a history edit is open on this instance.
    pub fn is_active(&self) -> bool {
        self.session.stage != TimeTravelStage::Inactive
    }

    /// 🧊️ Artifact-lane app commands, owned-child edits, agent transactions and history-lane verbs are refused with
    /// `timeTravel.frozen` while a history edit is open; selection, view and remote ingest keep working.
    pub fn freezes_local_emits(&self) -> bool {
        self.is_active()
    }

    /// 🪞️ The document every render seam reads: while editing, the state before the edited mutation with its draft
    /// (downstream not applied); while reviewing, the replayed head; otherwise the tool run overlay or `committed`.
    /// A poll, a command and a context menu keep deciding over what landed, never over this preview.
    /// A session on a composed member leaves the parent document as it is: the member's preview reaches the render seams
    /// through [`Self::render_children_or`].
    pub fn render_snapshot_or<'a>(&'a self, tool_runs: &'a ToolRunLedger<A>, committed: &'a Arc<A::Snapshot>) -> &'a Arc<A::Snapshot> {
        match (self.session.stage, self.member.is_some()) {
            (TimeTravelStage::Inactive, _) | (_, true) => tool_runs.overlay_or(committed),
            (stage, false) => self.document.shown(stage).map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::snapshot_owner).unwrap_or(committed),
        }
    }

    /// 🪞️ The composed children every render seam reads: while a session on a member shows that member's preview or
    /// replayed head, the live children with that member in its place (design §12); otherwise the member tool run's composed
    /// read or `live` (`ToolRunLedger::children_or`).
    pub fn render_children_or<'a>(&'a self, tool_runs: &'a ToolRunLedger<A>, live: &'a ChildContentView) -> &'a ChildContentView {
        self.member.as_ref().and_then(|member| member.children.as_ref()).filter(|_| self.is_active()).unwrap_or_else(|| tool_runs.children_or(live))
    }

    /// ⏯️ Whether the store the session edits holds a running replay.
    pub fn replaying(&self) -> bool {
        match self.member.as_ref() {
            None => TimeTravelOwners::replaying(&self.document),
            Some(member) => member.owners.as_deref().is_some_and(TimeTravelOwners::replaying),
        }
    }

    pub fn preview_progress(&self) -> Option<store::os_store::replay::ReplayProgress> {
        match self.member.as_ref() {
            None => self.document.preview_progress(),
            Some(member) => member.owners.as_deref().and_then(TimeTravelOwners::preview_progress),
        }
    }

    pub fn processed(&self) -> Option<u32> {
        match self.member.as_ref() {
            None => self.document.processed(),
            Some(member) => member.owners.as_deref().and_then(TimeTravelOwners::processed),
        }
    }

    /// 🧩️ The composed member store the session edits (`<slot>/<childId>`), `None` for the document's own.
    pub fn member_store(&self) -> Option<String> {
        self.member.as_ref().map(TimeTravelMemberSubject::store)
    }

    /// 🧹️ Every typed owner set the ledger holds: the document's, the member's being edited, and the members' still
    /// retiring after the session left them.
    pub fn owners(&self) -> impl Iterator<Item = &dyn TimeTravelOwners> {
        std::iter::once(&self.document as &dyn TimeTravelOwners).chain(self.member.iter().filter_map(|member| member.owners.as_deref())).chain(self.retiring.iter().map(|owners| &**owners))
    }

    /// 🏃️ Whether a driver turn has work: a live replay, an undo or redo of a finalize being authored, retirement, an owed
    /// UI scope or an owed history patch.
    pub fn has_pending_work(&self) -> bool {
        let replaying = self.session.stage == TimeTravelStage::Replaying && self.owners().any(TimeTravelOwners::replaying);
        self.member.as_ref().is_some_and(|_|self.closing||self.session.stage==TimeTravelStage::Inactive) || self.review_children_pending() || self.command_receipt.is_some() || self.pending_command.is_some() || self.discard_cursor.is_some() || !self.command_retirements.terminal_is_empty() || replaying || self.authoring.is_some() || self.owners().any(TimeTravelOwners::has_pending_work) || self.is_ui_dirty() || self.patch_due || self.patch.is_some()
    }

    pub fn review_children_pending(&self)->bool{
        self.member.as_ref().is_some_and(|member|match member.review_children.as_ref(){Some(review)=>self.closing||review.base!=self.session.base||!matches!(self.session.stage,TimeTravelStage::Editing|TimeTravelStage::Reviewing)||self.session.stage==TimeTravelStage::Reviewing&&(review.generation!=self.session.generation||member.children.as_ref().is_none_or(|view|!review.view.same_owner(view))),None=>!self.closing&&self.session.stage==TimeTravelStage::Reviewing&&member.children.is_some()})
    }

    /// 📨️ Pays the original received command slot while retaining unadmitted input in its caller.
    pub fn stage_time_travel_command(&mut self,original:&mut Option<TimeTravelEvent>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
        if self.command_receipt.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"actor original command receipt awaits its caller"))}
        if self.closing||self.pending_command.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"actor already retains an original pending command"))}
        let parent_copy=size_of::<Option<semio_framework_time_travel::TimeTravelCommandCustody>>();
        if grant.maximum_copy_bytes<parent_copy||grant.maximum_depth<2{return Ok(None)}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};
        let Some((owner,mut receipt))=semio_framework_time_travel::TimeTravelCommandCustody::admit_original(original,child)?else{return Ok(None)};
        if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original command admission exceeded its caller"))}
        receipt.copied_bytes+=parent_copy;self.pending_command=Some(owner);Ok(Some(receipt))
    }

    /// 🫥️ Whether the document holds no history because a pure command hydrated its head alone
    /// (`📓️api-stepped-document-load.md` §4): history verbs and queries answer `pure.history-unavailable`.
    pub fn history_unavailable(&self) -> bool {
        self.history_unavailable
    }

    /// 🫥️ Marks the document as hydrated head-only (`true`) or loaded with its history (`false`).
    pub fn set_history_unavailable(&mut self, unavailable: bool) {
        self.history_unavailable = unavailable;
    }

    /// ⏸️ Whether the user paused the replay of a waiting remote history change.
    pub fn reprojection_paused(&self) -> bool {
        self.reprojection_paused
    }

    /// ✍️ Whether the undo or redo of a finalize is still replaying before it commits.
    pub fn authoring_pending(&self) -> bool {
        self.authoring.is_some()
    }

    /// 🚩️ Whether a session change still owes the host a UI scope.
    pub fn is_ui_dirty(&self) -> bool {
        self.ui_dirty || self.document_dirty
    }

    /// 🧾️ The history patch a driver turn prepared for the next unsolicited UI progress frame, so replay progress
    /// and stage changes reach every host's band without a dispatch.
    pub fn take_patch(&mut self) -> Option<HistoryPatch> {
        self.patch.take()
    }

    /// 🧾️ Whether a prepared history patch still waits for its unsolicited UI progress frame.
    pub fn has_patch(&self) -> bool {
        self.patch.is_some()
    }

    /// 🎯️ The scope a session change dirties: every body once when the rendered document was swapped, else the
    /// history body and the per-window engagement chips.
    pub fn dirty_scope(&self) -> UiDirtyScope {
        if self.document_dirty {
            return UiDirtyScope::Full;
        }
        UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: true, measures: false, labels: false }
    }

    /// ✍️ The accepted drafts keyed by target, the shape every store read takes.
    pub fn accepted_drafts(&self) -> protocol::HistoryInputDrafts {
        self.session.accepted.iter().map(|draft| (draft.target.mutation.clone(), draft.replacement.clone())).collect()
    }

    pub fn replace_editor(&mut self, editor: Option<TimeTravelEditor>) {
        self.editor = editor;
    }

    /// 🧹️ One bounded retirement unit of the first typed owner set still retiring; `None` when nothing retires. A member
    /// owner set the session already left is dropped once it is terminal-empty.
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let demand=self.pending_command_retirement_demands(body)?;
        if demand!=RetirementDemand::default(){return Ok(demand);}
        let demand=self.session_retirement_demands(body)?;
        if demand!=RetirementDemand::default(){return Ok(demand);}
        let demand=self.document.retirement_demands(body)?;
        if demand!=RetirementDemand::default(){return Ok(demand);}
        if let Some(owners)=self.member.as_ref().and_then(|member|member.owners.as_ref()){let demand=owners.retirement_demands(body)?;if demand!=RetirementDemand::default(){return Ok(demand);}}
        if let Some(owners)=self.retiring.last(){return if owners.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(owners.as_ref()),depth:1,..Default::default()})}else{owners.retirement_demands(body)};}
        if self.retiring.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.retiring.capacity()*size_of::<Box<dyn TimeTravelOwners>>(),depth:1,..Default::default()});}
        Ok(Default::default())
    }
    pub fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, Fault> {
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if let Some(step)=self.pending_command_retirement_step(grant).map_err(ValueError::into_fault)?{return Ok(Some(step));}
        if let Some(step)=self.session_retirement_step(grant).map_err(ValueError::into_fault)?{return Ok(Some(step));}
        if let Some(step)=self.document.retire_step(grant)?{return Ok(Some(RetainedCloneStep::Progress(step.progress())));}
        if let Some(owners)=self.member.as_mut().and_then(|member|member.owners.as_mut()){if let Some(step)=owners.retire_step(grant)?{return Ok(Some(RetainedCloneStep::Progress(step.progress())));}}
        if let Some(owners)=self.retiring.last_mut(){
            if !owners.terminal_is_empty(){return owners.retire_step(grant).map(|step|step.map(|step|RetainedCloneStep::Progress(step.progress())));}
            let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
            if grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            drop(self.retiring.pop());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})));
        }
        if self.retiring.capacity()!=0{let bytes=self.retiring.capacity()*size_of::<Box<dyn TimeTravelOwners>>();if grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}drop(std::mem::take(&mut self.retiring));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})));}
        Ok(None)
    }

    /// 🚪️ Document close, instance retirement or reload: the session vanishes with every draft (ephemeral state). The
    /// typed owners settle at the runtime's close step, which reaches their stores.
    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn terminal_is_empty(&self) -> bool {
        (!self.closing||self.session_retired) && self.command_receipt.is_none() && self.pending_command.is_none() && self.discard_cursor.is_none() && self.command_retirements.terminal_is_empty() && self.session_retirement.is_none() && !self.is_active() && self.session.accepted.capacity()==0 && self.session.pending.is_none() && self.session.report.is_none() && self.session.fault.is_none() && self.editor.is_none() && self.member.is_none() && self.authoring.is_none() && self.patch.is_none() && self.reprojection_fault.is_none() && self.deferred_history_row.is_none() && self.retiring.capacity()==0 && self.owners().all(TimeTravelOwners::terminal_is_empty)
    }

    /// 👉️ The first mutation, in replay order, whose outcome blocks finalizing (`MergePolicy::Normal`, the floor
    /// `ReplayReport::blocks_finalize` reads): what the status names as the next problem (design §22.2); `None` while the
    /// session's report does not block.
    pub fn next_problem(&self) -> Option<String> {
        self.session.report.as_ref()?.outcomes.iter().find(|outcome| outcome.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst))).map(|outcome| outcome.mutation_id.0.clone())
    }

    /// ⏪️ The session status every host renders its band from (design §10); `None` while inactive.
    pub fn status(&self) -> Option<HistoryTimeTravel> {
        let session = &self.session;
        let stage = match session.stage {
            TimeTravelStage::Inactive => return None,
            TimeTravelStage::Editing => HistoryTimeTravelStage::Editing,
            TimeTravelStage::Replaying => HistoryTimeTravelStage::Replaying,
            TimeTravelStage::Reviewing => HistoryTimeTravelStage::Reviewing,
            TimeTravelStage::Choosing => HistoryTimeTravelStage::Choosing,
            TimeTravelStage::Finalizing => HistoryTimeTravelStage::Finalizing,
        };
        let preview_progress = (session.stage == TimeTravelStage::Editing).then(|| self.preview_progress()).flatten();
        Some(HistoryTimeTravel {
            session_id: session.id.to_string(),
            generation: session.generation,
            stage,
            target: session.pending.as_ref().map(|pending| pending.target.mutation.0.clone()),
            target_label: self.editor.as_ref().map(|editor| editor.label.clone()),
            done: preview_progress.map(|progress| progress.done).or_else(|| session.progress.map(|progress| progress.done)),
            total: preview_progress.map(|progress| progress.total).or_else(|| session.progress.map(|progress| progress.total)),
            processed: (session.stage == TimeTravelStage::Replaying).then(|| self.processed()).flatten(),
            worst: session.report.as_ref().and_then(|report| report.worst),
            blocking: session.report.as_ref().is_some_and(protocol::ReplayReport::blocks_finalize),
            fault: session.fault.clone(),
            accepted_count: u32::try_from(session.accepted.len()).unwrap_or(u32::MAX),
            review: session.review().map(|review| match review {
                TimeTravelReview::NoChanges => HistoryTimeTravelReview::NoChanges,
                TimeTravelReview::NeedsReplay => HistoryTimeTravelReview::NeedsReplay,
                TimeTravelReview::Blocked => HistoryTimeTravelReview::Blocked,
                TimeTravelReview::Ready => HistoryTimeTravelReview::Ready,
            }),
            rerunnable: session.rerun_refusal().is_none(),
            next_problem: self.next_problem().map(|mutation_id| HistoryTimeTravelProblem { mutation_id, store: self.member_store() }),
        })
    }

    /// 👥️ The ephemeral shared summary of the open session a host stamps on `PresencePeer.history_edit`: the mutation
    /// being edited (else the newest accepted draft's), the stage and the accepted draft count; `None` while inactive.
    pub fn presence(&self) -> Option<protocol::PresenceHistoryEdit> {
        let session = &self.session;
        let stage = match session.stage {
            TimeTravelStage::Inactive => return None,
            TimeTravelStage::Editing => protocol::PresenceHistoryEditStage::Editing,
            TimeTravelStage::Replaying => protocol::PresenceHistoryEditStage::Replaying,
            TimeTravelStage::Reviewing => protocol::PresenceHistoryEditStage::Reviewing,
            TimeTravelStage::Choosing => protocol::PresenceHistoryEditStage::Choosing,
            TimeTravelStage::Finalizing => protocol::PresenceHistoryEditStage::Finalizing,
        };
        let mutation_id = self.editor.as_ref().map(|editor| editor.target.0.clone()).or_else(|| session.accepted.last().map(|draft| draft.target.mutation.0.clone()))?;
        Some(protocol::PresenceHistoryEdit { mutation_id, stage, drafts: u32::try_from(session.accepted.len()).unwrap_or(u32::MAX) })
    }

    /// 🪧️ The plain-data view the history panel and the history wire read; `None` while inactive.
    pub fn panel(&self) -> Option<TimeTravelPanel> {
        let status = self.status()?;
        let session = &self.session;
        let outcomes = session.report.as_ref().map(|report| report.outcomes.iter().map(|outcome| (outcome.mutation_id.0.clone(), outcome.clone())).collect()).unwrap_or_default();
        let accepted: BTreeSet<String> = session.accepted.iter().map(|draft| draft.target.mutation.0.clone()).collect();
        let mut edited = accepted.clone();
        if let Some(pending) = session.pending.as_ref().filter(|pending| !session.unchanged(pending)) {
            edited.insert(pending.target.mutation.0.clone());
        }
        let next_problem = self.next_problem();
        let editor = self.editor.as_ref().map(|editor| TimeTravelEditorPanel {
            target: editor.target.0.clone(),
            label: editor.label.clone(),
            rows: time_travel_input_rows(&editor.inputs, &editor.value),
            editable: editor.schema.is_some(),
            inputs_refused: editor.inputs_refused.as_ref().map(|error| error.detail.clone()),
            withdrawn: session.pending.as_ref().is_some_and(|pending| pending.replacement == protocol::InputReplacement::Withdrawn),
            outcome: editor.outcome.clone(),
            refused: editor.refused.clone(),
            changed: session.pending.as_ref().is_some_and(|pending| !session.unchanged(pending)),
            reference_labels: BTreeMap::new(),
        });
        Some(TimeTravelPanel {
            status,
            store: self.member_store(),
            stage: session.stage,
            pending_after: session.pending.as_ref().and(self.editor.as_ref()).map(|editor| (editor.position, editor.op_index)),
            finalize_refusal: session.finalize_refusal(),
            review: session.review(),
            rerun_refusal: session.rerun_refusal(),
            begin_refusal: session.begin_refusal(),
            next_problem,
            editor,
            outcomes,
            edited,
            accepted,
        })
    }
}
//#endregion 🔖️Ledger

//#region 🔖️StoreState
#[doc(hidden)]
#[path="🎮️decision/🗑️discard/🦀️.rs"]
pub mod discard_decision;
/// 🧹️ The store-free half of one store's history-edit owners: whether a replay runs, whether owners still retire, the
/// terminal witness and one bounded retirement unit — callable on a composed member's erased owners without its store.
#[doc(hidden)]
pub trait TimeTravelOwners: Send {
    fn replaying(&self) -> bool;
    fn preview_progress(&self) -> Option<store::os_store::replay::ReplayProgress>;
    fn processed(&self) -> Option<u32>;
    fn has_pending_work(&self) -> bool;
    fn terminal_is_empty(&self) -> bool;
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn command_pending(&self)->bool;
    fn cancel_prepared_command(&mut self);
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, Fault>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// 🗃️ The typed owners of a history edit on ONE store — the document's own or one composed member's (design §12): the
/// edited operation's kind (staged by an open until the session adopts it), the draft preview, the running Report replay,
/// its finished result and replayed head, the retirements of the snapshots they displaced (taken from the store at once,
/// so they retire without it) and the operations a draft discarded.
#[doc(hidden)]
pub struct TimeTravelStoreState<P, Mu: ::protocol::Mutation<P>> {
    discard_effects: Option<Box<discard_decision::TimeTravelDiscardStoreEffects<P,Mu>>>,
    pub label_of: fn(&Mu) -> LocalizedLabel,
    staged: Option<Mu>,
    kind: Option<Mu>,
    preview: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    preview_job: Option<store::os_store::snapshot_read::ArtifactDerivedHistoryPreview<P, Mu>>,
    preview_input: Option<protocol::InputReplacement>,
    replay: Option<store::os_store::snapshot_read::ArtifactDerivedReplay<P, Mu>>,
    finished: Option<store::os_store::replay::EditReplayResult<P, Mu>>,
    head: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    snapshot_aliases: ManuallyDrop<[Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>; 8]>,
    snapshot_issuers: ManuallyDrop<Option<(Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<P>>,Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mu>>)>>,
    snapshot_close: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    snapshot_issuer_close: [Option<semio_framework_value::FactoryAuthority>; 2],
    pub(crate) retirements: VecDeque<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(crate) discarded: Vec<Mu>,
    pub(crate) discarded_pending: Option<Mu>,
    discarded_active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(crate) discarded_factory: Option<Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mu>>>,
    discarded_factory_close: Option<semio_framework_value::FactoryAuthority>,
}

impl<P,Mu:Mutation<P>> Drop for TimeTravelStoreState<P,Mu>{
    fn drop(&mut self){
        let empty=self.snapshot_aliases.iter().all(Option::is_none)&&self.snapshot_close.is_none()&&self.snapshot_issuers.is_none()&&self.snapshot_issuer_close.iter().all(Option::is_none);
        assert!(std::thread::panicking()||empty,"original derived snapshot custody requires granted retirement");
        if empty{unsafe{ManuallyDrop::drop(&mut self.snapshot_aliases);ManuallyDrop::drop(&mut self.snapshot_close);ManuallyDrop::drop(&mut self.snapshot_issuers)}}
    }
}

/// ⏭️ What one replay slice did: progress, a completed report (the head is swapped in), or a fault.
#[doc(hidden)]
pub enum TimeTravelReplayStep {
    Pending { done: u32, total: u32 },
    Completed(protocol::ReplayReport),
    Faulted,
}

/// 🖼️ One preview slice leaves progress or publishes a complete selected-mutation projection.
#[doc(hidden)]
pub enum TimeTravelPreviewStep {
    Pending,
    Completed(Vec<protocol::MutationMessage>),
}

/// 🌿️ How a finalize ended on its store; a finalize carries the encoded envelopes of every transition it authored (an
/// overwrite's `Supersede`; an alternative's pending-edit commit, `Branch` and scoped `Supersede`), in authoring order.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeTravelCommit {
    Finalized { authored: Vec<u8> },
    Stale,
    Blocked,
    Failed,
}

/// 📨️ One typed history-edit operation against the store a session edits.
#[doc(hidden)]
pub enum TimeTravelStoreCommand {
    Open { target: MutationId, current: Option<protocol::InputReplacement>, withdraw: bool },
    Adopt(bool),
    Rebuild(DslValue),
    Preview { target: MutationId, replacement: protocol::InputReplacement, drafts: protocol::HistoryInputDrafts, deadline_us: u64, clock: fn() -> Option<u64> },
    StepPreview { deadline_us: u64, clock: fn() -> Option<u64> },
    StartReplay { drafts: protocol::HistoryInputDrafts, from: MutationId },
    CancelReplay,
    StepReplay { deadline_us: u64, clock: fn() -> Option<u64> },
    Read(TimeTravelStage),
}

/// 📬️ What a [`TimeTravelStoreCommand`] answered.
#[doc(hidden)]
pub enum TimeTravelStoreOutput {
    Opened(Result<(TimeTravelEditor, protocol::InputReplacement), TimeTravelActionRefusal>),
    Rebuilt(Result<protocol::InputReplacement, String>),
    Previewed(TimeTravelPreviewStep),
    ReplayStarted(bool),
    Stepped(TimeTravelReplayStep),
    Read(Option<(store::os_store::snapshot_read::ErasedSnapshotRead, [u8; 32])>),
    Done,
}

/// 🔎️ One store-only read a session takes synchronously.
#[doc(hidden)]
pub enum TimeTravelStoreQuery {
    Base,
    Positions(Vec<MutationId>),
    ShownValue(TimeTravelStage),
}

/// 📬️ What a [`TimeTravelStoreQuery`] answered.
#[doc(hidden)]
pub enum TimeTravelQueryOutput {
    Base(TimeTravelBase),
    Positions(Option<Vec<TimeTravelTarget>>),
    Value(Option<DslValue>),
}

impl<P, Mu> TimeTravelStoreState<P, Mu>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
{
    /// 🏗️ Owners at rest, labelling the store's operations with `label_of`.
    pub fn new(label_of: fn(&Mu) -> LocalizedLabel) -> Self {
        Self { discard_effects:None,label_of, staged: None, kind: None, preview: None, preview_job: None, preview_input: None, replay: None, finished: None, head: None, snapshot_aliases: ManuallyDrop::new(std::array::from_fn(|_|None)), snapshot_issuers: ManuallyDrop::new(None), snapshot_close: ManuallyDrop::new(None), snapshot_issuer_close: std::array::from_fn(|_|None), retirements: VecDeque::new(), discarded: Vec::new(), discarded_pending: None, discarded_active: None, discarded_factory: None, discarded_factory_close: None }
    }

    /// 🪞️ The snapshot the session shows at `stage`: the draft preview while editing; while replaying that preview — or,
    /// for a replay started from a review (Replay again, a restored draft, a moved base), the head reviewed last, so no
    /// window falls back to the committed document in between; the replayed head while reviewing.
    pub fn shown(&self, stage: TimeTravelStage) -> Option<&store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>> {
        match stage {
            TimeTravelStage::Inactive => None,
            TimeTravelStage::Editing => self.preview.as_ref(),
            TimeTravelStage::Replaying => self.preview.as_ref().or(self.head.as_ref()),
            TimeTravelStage::Reviewing | TimeTravelStage::Choosing | TimeTravelStage::Finalizing => self.head.as_ref(),
        }
    }

    fn snapshot_custody_available(&self, count:usize)->Result<(),Fault>{
        if self.snapshot_issuer_close.iter().any(Option::is_some){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original snapshot issuer tickets must close before accepting another source").into_fault())}
        if self.snapshot_aliases.iter().filter(|slot|slot.is_none()).count()<count{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original derived snapshots await their granted retirement turn").into_fault())}
        Ok(())
    }

    pub fn retire(&mut self, store: &ArtifactStore<P, Mu>, snapshot: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>) -> Result<(), Fault> {
        if let Some(snapshot)=snapshot {
            let slot=self.snapshot_aliases.iter_mut().find(|slot|slot.is_none()).expect("original snapshot custody was reserved before source publication");
            *slot=Some(snapshot);
            if self.snapshot_issuers.is_none() {
                match (store.owned_snapshot_retirement_factory(),store.owned_mutation_retirement_factory()) {
                    (Ok(snapshots),Ok(mutations))=>*self.snapshot_issuers=Some((Arc::clone(snapshots),Arc::clone(mutations))),
                    _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original derived snapshot retains its missing retirement issuer").into_fault()),
                }
            }
        }
        Ok(())
    }

    fn cancel_preview(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
        if let Some((retirement,progress)) = store.retire_derived_history_preview(&mut self.preview_job,grant).map_err(ValueError::into_fault)? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
        self.preview_input = None;
        Ok(())
    }

    fn cancel_replay(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
        if let Some((retirement,progress)) = store.retire_derived_report_replay(&mut self.replay,grant).map_err(ValueError::into_fault)? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
        Ok(())
    }

    fn cancel_finished(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
        if let Some((retirement,progress)) = store.retire_finished_history_replay(&mut self.finished,grant).map_err(ValueError::into_fault)? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
        Ok(())
    }

    pub fn discard(&mut self, op: Option<Mu>) {
        self.discarded.extend(op);
    }

    /// 🧽️ Drops what `stage` no longer shows: the kind outside `Editing`, the draft preview once the replay finished or
    /// the session left, the replay outside `Replaying`, every replay owner once it is inactive.
    pub fn settle(&mut self, store: &ArtifactStore<P, Mu>, stage: TimeTravelStage) -> Result<(), Fault> {
        self.snapshot_custody_available(2)?;
        if stage != TimeTravelStage::Editing {
            self.cancel_preview(store)?;
            let kind = self.kind.take();
            self.discard(kind);
            let staged = self.staged.take();
            self.discard(staged);
        }
        if !matches!(stage, TimeTravelStage::Editing | TimeTravelStage::Replaying) {
            let preview = self.preview.take();
            self.retire(store, preview)?;
        }
        if stage != TimeTravelStage::Replaying {
            self.cancel_replay(store)?;
        }
        if stage == TimeTravelStage::Inactive {
            self.cancel_finished(store)?;
            let head = self.head.take();
            self.retire(store, head)?;
        }
        Ok(())
    }

    /// 🔎️ A store-only read.
    pub fn query(owners: Option<&Self>, store: &ArtifactStore<P, Mu>, query: TimeTravelStoreQuery) -> TimeTravelQueryOutput {
        match query {
            TimeTravelStoreQuery::Base => TimeTravelQueryOutput::Base(TimeTravelBase { content_revision: store.content_revision() }),
            TimeTravelStoreQuery::Positions(targets) => TimeTravelQueryOutput::Positions(targets.into_iter().map(|target| store.mutation_position(&target).ok().map(|(position, _)| TimeTravelTarget { position: u32::try_from(position).unwrap_or(u32::MAX), mutation: target })).collect()),
            TimeTravelStoreQuery::ShownValue(stage) => TimeTravelQueryOutput::Value(owners.and_then(|owners| owners.shown(stage)).map(|shown| semio_framework_value::ToValue::to_value(shown.snapshot()))),
        }
    }

    /// 📨️ Runs one typed command against `store`.
    pub async fn run(&mut self, store: &mut ArtifactStore<P, Mu>, command: TimeTravelStoreCommand) -> Result<TimeTravelStoreOutput, Fault> {
        Ok(match command {
            TimeTravelStoreCommand::Open { target, current, withdraw } => TimeTravelStoreOutput::Opened(self.open(store, &target, current.as_ref(), withdraw)),
            TimeTravelStoreCommand::Adopt(adopt) => {
                let staged = self.staged.take();
                match adopt {
                    true => {
                        let previous = std::mem::replace(&mut self.kind, staged);
                        self.discard(previous);
                    }
                    false => self.discard(staged),
                }
                TimeTravelStoreOutput::Done
            }
            TimeTravelStoreCommand::Rebuild(candidate) => TimeTravelStoreOutput::Rebuilt(self.rebuild(store, candidate)),
            TimeTravelStoreCommand::Preview { target, replacement, drafts, deadline_us, clock } => {
                self.cancel_preview(store)?;
                self.preview_job = Some(store.begin_derived_history_preview(target, drafts).map_err(|error| error.into_fault())?);
                self.preview_input = Some(replacement);
                TimeTravelStoreOutput::Previewed(self.step_preview(store, deadline_us, clock)?)
            }
            TimeTravelStoreCommand::StepPreview { deadline_us, clock } => TimeTravelStoreOutput::Previewed(self.step_preview(store, deadline_us, clock)?),
            TimeTravelStoreCommand::StartReplay { drafts, from } => {
                self.cancel_replay(store)?;
                match store.begin_derived_report_replay(drafts, Some(from)) {
                    Ok(replay) => {
                        self.replay = Some(replay);
                        TimeTravelStoreOutput::ReplayStarted(true)
                    }
                    Err(_) => TimeTravelStoreOutput::ReplayStarted(false),
                }
            }
            TimeTravelStoreCommand::CancelReplay => {
                self.cancel_replay(store)?;
                TimeTravelStoreOutput::Done
            }
            TimeTravelStoreCommand::StepReplay { deadline_us, clock } => TimeTravelStoreOutput::Stepped(self.step_replay(store, deadline_us, clock)?),
            TimeTravelStoreCommand::Read(stage) => TimeTravelStoreOutput::Read(match self.shown(stage) {
                Some(shown) => Some((store.snapshot_read_derived(shown).map_err(|error| error.into_fault())?, store.content_revision())),
                None => None,
            }),
        })
    }

    /// 🪪️ Commits the original finished replay through the caller's existing identity recipient.
    pub async fn commit_finished(&mut self, store: &mut ArtifactStore<P, Mu>, drafts: protocol::HistoryInputDrafts, finalization: store::HistoryFinalization, actor: Option<&str>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelCommit, Fault> {
        if self.finished.as_ref().is_none_or(|finished| *finished.drafts() != drafts) {
            self.cancel_finished(store)?;
            return Ok(TimeTravelCommit::Stale);
        }
        if actor.is_some_and(|actor| actor != store.local_actor_id().0.as_str()) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("time-travel.actor"), "history edit actor differs from its opened session"));
        }
        let finished = self.finished.take().expect("admitted finished history replay");
        let known: BTreeSet<MutationId> = store.envelope().transitions.iter().map(|transition| transition.mutation_id.clone()).collect();
        Ok(match store.commit_finished_replay(finished, finalization, identity).await {
            Ok(_) => {
                let authored: Vec<_> = store.envelope().transitions.iter().filter(|transition| !known.contains(&transition.mutation_id)).cloned().collect();
                TimeTravelCommit::Finalized { authored: store::os_spr::encode_envelopes(&authored) }
            }
            Err(vcs::VcsError::Stale { .. }) => TimeTravelCommit::Stale,
            Err(vcs::VcsError::Rejected { .. }) => TimeTravelCommit::Blocked,
            Err(error) => return Err(error.into_fault()),
        })
    }

    /// ✏️ The editor of `target` and its effective input before the session (its payload schema, input descriptors,
    /// validator and the value it starts from: `current`, its accepted draft, else its input); its kind is staged until
    /// the session adopts it. Refused for an unknown operation and for one whose inputs cannot be edited: it plans foreign
    /// steps, declares no input schema, or its schema describes no input the editor shows as a row over that value (design
    /// §22.20: an editor with zero rows never opens; a schema that cannot be read still opens, naming why) — unless the
    /// editor opens to `withdraw` it (a history row's Withdraw, design §22.1): then the store's supersede law decides
    /// ([`time_travel_admits_withdrawal`]), and an operation without editable inputs opens on an editor without inputs.
    /// An operation of a cross-document unit takes no supersession in this document alone (`AppliedMutation::unit`, the
    /// store's authoring law): its Withdraw answers the store's own refusal, its inputs are not editable.
    pub fn open(&mut self, store: &ArtifactStore<P, Mu>, target: &MutationId, current: Option<&protocol::InputReplacement>, withdraw: bool) -> Result<(TimeTravelEditor, protocol::InputReplacement), TimeTravelActionRefusal> {
        let refusal = if withdraw { TimeTravelActionRefusal::NotWithdrawable } else { TimeTravelActionRefusal::NotEditable };
        let row = store.applied_mutation(target).map_err(|_| TimeTravelActionRefusal::UnknownMutation)?;
        if row.unit.is_some() {
            return Err(if withdraw { TimeTravelActionRefusal::UnitSpansDocuments } else { TimeTravelActionRefusal::NotEditable });
        }
        let schema_id = store.envelope().schema.clone();
        let original = match row.supersession {
            Some(supersession) => supersession.replacement.clone(),
            None => protocol::InputReplacement::Input { schema: schema_id, payload: <Mu as ::protocol::OpBinary>::encode_op(row.operation).map_err(|_| refusal)? },
        };
        let decode = |replacement: &protocol::InputReplacement| match replacement {
            protocol::InputReplacement::Input { payload, .. } => <Mu as ::protocol::OpBinary>::decode_op(payload).ok(),
            protocol::InputReplacement::Withdrawn => None,
        };
        let kind = decode(&original).unwrap_or_else(|| row.operation.clone());
        let value = match decode(current.unwrap_or(&original)) {
            Some(op) => {
                let value = op.payload_value();
                op.retire_cold();
                value
            }
            None => kind.payload_value(),
        };
        let read = kind.input_schema().filter(|_| !kind.may_emit_foreign_steps()).map(|schema| (schema, mutation_input_defs(schema, &registered_input_schema_document)));
        let read = read.filter(|(_, inputs)| inputs.as_ref().map_or(true, |inputs| !time_travel_input_rows(inputs, &value).is_empty()));
        let admitted = if withdraw { time_travel_admits_withdrawal::<P, Mu>(row.operation) } else { read.is_some() };
        if !admitted {
            kind.retire_cold();
            return Err(refusal);
        }
        let (schema, inputs, inputs_refused, validator) = match read {
            Some((schema, read)) => {
                let (inputs, inputs_refused) = match read {
                    Ok(inputs) => (inputs, None),
                    Err(error) => (Vec::new(), Some(error)),
                };
                let documents = time_travel_schema_documents(schema);
                (Some(schema), inputs, inputs_refused, semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema, &documents.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|error| error.to_string()))
            }
            None => (None, Vec::new(), None, Err(TimeTravelLabel::RefusalNotEditable.en().to_string())),
        };
        let editor = TimeTravelEditor {
            target: target.clone(),
            position: u32::try_from(row.position).unwrap_or(u32::MAX),
            op_index: row.op_index,
            label: (self.label_of)(&kind),
            schema,
            inputs,
            inputs_refused,
            validator,
            value,
            outcome: Vec::new(),
            refused: None,
        };
        let previous = self.staged.replace(kind);
        self.discard(previous);
        Ok((editor, original))
    }

    /// 🎚️ The validated draft payload `candidate` rebuilt into the edited kind and canonically encoded.
    pub fn rebuild(&mut self, store: &ArtifactStore<P, Mu>, candidate: DslValue) -> Result<protocol::InputReplacement, String> {
        let op = self.kind.as_ref().ok_or("the draft editor lost the operation kind it rebuilds")?.with_payload_value(candidate).map_err(|error| error.to_string())?;
        let encoded = <Mu as ::protocol::OpBinary>::encode_op(&op);
        self.discard(Some(op));
        Ok(protocol::InputReplacement::Input { schema: store.envelope().schema.clone(), payload: encoded.map_err(|error| error.to_string())? })
    }

    /// 🖼️ One bounded prefix slice; only a completed preview displaces the last complete projection.
    fn step_preview(&mut self, store: &mut ArtifactStore<P, Mu>, deadline_us: u64, clock: fn() -> Option<u64>) -> Result<TimeTravelPreviewStep, Fault> {
        self.snapshot_custody_available(2)?;
        let Some(job) = self.preview_job.as_mut() else { return Ok(TimeTravelPreviewStep::Pending) };
        let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
        loop {
            let grant=history_planning_retirement_grant(job.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(ValueError::into_fault)?);
            if matches!(store.step_derived_history_preview(job, grant, &mut deadline).map_err(|error| error.into_fault())?, store::os_store::replay::ReplayStep::Finished(_)) { break; }
            if deadline() { return Ok(TimeTravelPreviewStep::Pending); }
        }
        let base = store.finish_derived_history_preview(&mut self.preview_job).map_err(|error| error.into_fault())?;
        let replacement = self.preview_input.take().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.preview-input-absent"), "history preview lost its selected input"))?;
        self.cancel_preview(store)?;
        let (preview, outcome) = match replacement {
            protocol::InputReplacement::Withdrawn => (base, Vec::new()),
            protocol::InputReplacement::Input { payload, .. } => {
                let op = <Mu as ::protocol::OpBinary>::decode_op(&payload).map_err(|error| error.into_fault())?;
                let derived = store.derive_history_snapshot(&base, &op);
                self.discard(Some(op));
                let (next, messages) = derived.map_err(|error| error.into_fault())?;
                match next {
                    Some(next) => { self.retire(store, Some(base))?; (next, messages) }
                    None => (base, messages),
                }
            }
        };
        let previous = self.preview.replace(preview);
        self.retire(store, previous)?;
        Ok(TimeTravelPreviewStep::Completed(outcome))
    }

    /// ⏭️ Steps the replay until `deadline_us` on `clock`; completion swaps the replayed head in and keeps the finished result.
    fn step_replay(&mut self, store: &ArtifactStore<P, Mu>, deadline_us: u64, clock: fn() -> Option<u64>) -> Result<TimeTravelReplayStep, Fault> {
        self.snapshot_custody_available(1)?;
        let Some(replay) = self.replay.as_mut() else { return Ok(TimeTravelReplayStep::Faulted) };
        let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
        let grant=history_planning_retirement_grant(replay.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(ValueError::into_fault)?);
        Ok(match store.step_derived_report_replay(replay, grant, &mut deadline) {
            Ok(store::os_store::replay::ReplayStep::Pending(progress)) => TimeTravelReplayStep::Pending { done: progress.done, total: progress.total },
            Ok(store::os_store::replay::ReplayStep::Finished(_)) => {
                match store.finish_derived_report_replay(&mut self.replay).and_then(|(result, head)| store.replay_report(&result).map(|report| (result, head, report))) {
                    Ok((result, head, report)) => {
                        let previous = std::mem::replace(&mut self.head, head);
                        self.retire(store, previous)?;
                        self.cancel_finished(store)?;
                        self.finished = Some(result);
                        TimeTravelReplayStep::Completed(report)
                    }
                    Err(_) => TimeTravelReplayStep::Faulted,
                }
            }
            Err(_) => {
                self.cancel_replay(store)?;
                TimeTravelReplayStep::Faulted
            }
        })
    }
}

impl<P, Mu> TimeTravelOwners for TimeTravelStoreState<P, Mu>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
{
    fn replaying(&self) -> bool {
        self.replay.is_some()
    }

    fn preview_progress(&self) -> Option<store::os_store::replay::ReplayProgress> {
        self.preview_job.as_ref().map(store::os_store::snapshot_read::ArtifactDerivedHistoryPreview::progress)
    }

    fn processed(&self) -> Option<u32> {
        self.replay.as_ref().map(|replay| replay.operation_preparation_progress().items).filter(|items| *items > 0).map(|items| u32::try_from(items).unwrap_or(u32::MAX))
    }

    fn has_pending_work(&self) -> bool {
        self.snapshot_aliases.iter().any(Option::is_some) || self.snapshot_close.is_some() || self.snapshot_issuers.is_some() || self.snapshot_issuer_close.iter().any(Option::is_some) || self.discard_effects.as_ref().is_some_and(|owner|owner.closing) || self.preview_job.is_some() || self.retirements.capacity()!=0 || self.discarded.capacity()!=0 || self.discarded_pending.is_some() || self.discarded_active.is_some() || self.discarded_factory.is_some() || self.discarded_factory_close.is_some()
    }

    fn terminal_is_empty(&self) -> bool {
        self.snapshot_aliases.iter().all(Option::is_none) && self.snapshot_close.is_none() && self.snapshot_issuers.is_none() && self.snapshot_issuer_close.iter().all(Option::is_none) && self.discard_effects.is_none() && self.staged.is_none() && self.kind.is_none() && self.preview.is_none() && self.preview_job.is_none() && self.preview_input.is_none() && self.replay.is_none() && self.finished.is_none() && self.head.is_none() && self.retirements.capacity()==0 && self.discarded.capacity()==0 && self.discarded_pending.is_none() && self.discarded_active.is_none() && self.discarded_factory.is_none() && self.discarded_factory_close.is_none()
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.original_retirement_demands(body) }
    fn command_pending(&self)->bool{self.discard_effects.is_some()}
    fn cancel_prepared_command(&mut self){if let Some(owner)=self.discard_effects.as_mut(){owner.closing=true}}
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, Fault> { self.original_retirement_step(grant).map_err(ValueError::into_fault) }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// ✍️ One typed history-edit command against a composed member's store (design §12): creates the member's owners on the
/// first visit, then runs the command on them.
#[doc(hidden)]
pub struct TimeTravelMemberRun<'a> {
    pub owners: &'a mut Option<Box<dyn TimeTravelOwners>>,
    pub command: TimeTravelStoreCommand,
}

impl store::MemberStoreVisitorMut for TimeTravelMemberRun<'_> {
    type Output = Result<TimeTravelStoreOutput, Fault>;

    async fn visit_mut<P, Mu>(self, store: &mut ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        for documents in <Mu as ::protocol::Mutation<P>>::INPUT_SCHEMA_DOCUMENTS {
            semio_framework_schema_registry::register_referenced_schema_documents(documents);
        }
        let owners = self.owners.get_or_insert_with(|| Box::new(TimeTravelStoreState::<P, Mu>::new(|op| protocol::SemanticMutation::<P>::label(op))) as Box<dyn TimeTravelOwners>);
        let state = owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P, Mu>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.member-store-kind"), "a composed member's history-edit owners belong to another store kind"))?;
        state.run(store, self.command).await
    }
}

/// 🪪️ Finalizes the original composed member replay with the existing caller authority.
#[doc(hidden)]
pub struct TimeTravelMemberCommit<'a, 'identity> {
    pub owners: &'a mut Option<Box<dyn TimeTravelOwners>>,
    pub drafts: protocol::HistoryInputDrafts,
    pub finalization: store::HistoryFinalization,
    pub actor: Option<&'a str>,
    pub identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'identity>,
}

impl store::MemberStoreVisitorMut for TimeTravelMemberCommit<'_, '_> {
    type Output = Result<TimeTravelCommit, Fault>;

    async fn visit_mut<P, Mu>(self, store: &mut ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        let state = self.owners.as_deref_mut().and_then(|owners| owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P, Mu>>()).ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.member-store-kind"), "a composed member's history-edit owners belong to another store kind"))?;
        state.commit_finished(store, self.drafts, self.finalization, self.actor, self.identity).await
    }
}

/// 🧽️ Settles a composed member's owners for `stage` against its store (design §12).
#[doc(hidden)]
pub struct TimeTravelMemberSettle<'a> {
    pub owners: &'a mut dyn TimeTravelOwners,
    pub stage: TimeTravelStage,
}

impl store::MemberStoreVisitor for TimeTravelMemberSettle<'_> {
    type Output = Result<(), Fault>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        self.owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P, Mu>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.member-store-kind"), "a composed member's history-edit owners belong to another store kind"))?.settle(store, self.stage)
    }
}

/// 🔎️ A store-only read of a composed member's store (design §12).
#[doc(hidden)]
pub struct TimeTravelMemberQuery<'a> {
    pub owners: Option<&'a dyn TimeTravelOwners>,
    pub query: TimeTravelStoreQuery,
}

impl store::MemberStoreVisitor for TimeTravelMemberQuery<'_> {
    type Output = TimeTravelQueryOutput;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        TimeTravelStoreState::<P, Mu>::query(self.owners.and_then(|owners| owners.as_any().downcast_ref::<TimeTravelStoreState<P, Mu>>()), store, self.query)
    }
}
//#endregion 🔖️StoreState

//#region 🔖️Pointer
/// 🎯️ The draft value of reference input `input` read from `selection`, the current selection of its declared domain:
/// the selected ids typed by the input's id type (ids the type cannot spell are skipped), all of them up to `max_items`
/// for a many-reference, else the first. Refused `unknown-input` for an input that is no reference or names no domain,
/// `no-selection` when the domain selects nothing at the input's granularity.
#[doc(hidden)]
pub fn time_travel_selection_value(input: &ActionArgDef, selection: Option<&protocol::DomainSelection>) -> Result<DslValue, TimeTravelActionRefusal> {
    let ArgSchema::Reference { domain: Some(_), granularity, many, max_items, id_type, .. } = &input.schema else { return Err(TimeTravelActionRefusal::UnknownInput) };
    let selection = selection.filter(|selection| granularity.as_ref().is_none_or(|granularity| *granularity == selection.granularity)).ok_or(TimeTravelActionRefusal::NoSelection)?;
    let mut ids = selection.ids.iter().filter_map(|id| id_type.id_value(id));
    if !*many {
        return ids.next().ok_or(TimeTravelActionRefusal::NoSelection);
    }
    let ids: Vec<DslValue> = ids.take(max_items.map_or(usize::MAX, |max| max as usize)).collect();
    match ids.is_empty() {
        true => Err(TimeTravelActionRefusal::NoSelection),
        false => Ok(DslValue::Array(ids)),
    }
}
//#endregion 🔖️Pointer

//#region 🔖️Driver
/// 🧿️ The session generation a verb names, in every shape the wire hands it over; `None` addresses the live session.
#[doc(hidden)]
pub fn time_travel_arg_generation(args: Option<&DslValue>) -> Option<u32> {
    tool_run::tool_run_arg_u64(args, HISTORY_EDIT_ARG_GENERATION).and_then(|generation| u32::try_from(generation).ok())
}
//#endregion 🔖️Driver

//#region 🔖️Supersessions
/// 🔁️ What one `Supersede` transition does to its author's history edits: a new edit, the undo of an earlier one (it
/// restores every input that edit still holds to the input before it, the original input when there was none), or the
/// redo of an undone one (it re-authors that edit's inputs over the state its undo left).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupersedeRole {
    Edit,
    Undo,
    Redo,
}

/// ✏️ One `Supersede` transition of the event log: its id, author, HLC, scope and inputs, its [`SupersedeRole`] and the
/// history edit it acts for (`entry`, an index into [`SupersedeLedger::records`]) — itself for an edit, the undone edit
/// for an undo, the re-authored edit for a redo.
#[derive(Clone, Debug, PartialEq)]
pub struct SupersedeRecord {
    pub transition_id: String,
    pub actor: semio_framework_value::SharedUtf8,
    pub timestamp: HybridLogicalTimestamp,
    pub scope: Option<String>,
    pub inputs: Vec<protocol::SupersededInput>,
    pub role: SupersedeRole,
    pub entry: usize,
}

/// 📚️ Every `Supersede` transition of a document's event log in fold order `(hlc, id)`, classified per author, with each
/// author's history edits that are not undone, the undone ones a redo re-authors (each with the undo that undid it), each
/// author's newest document-edit undo (`Revert`) and the original input of every superseded operation. A pure function
/// of the transitions and the edits: every replica, and every reload, derives the same rows. Rebuilt only when the
/// transitions changed.
///
/// An input's effective supersession in the view of `scope` is the last one naming it whose scope is `None` or `scope` —
/// the store's fold law — and an undo restores in the view of the undone edit's own scope. Every effective input has an
/// owner: the history edit whose input it is (an edit's own, a redo's re-authored edit's, an undo's the owner of the
/// input it restored), or none for an original input.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SupersedeLedger {
    pub key: (usize, Option<String>),
    pub records: Vec<SupersedeRecord>,
    pub index: HashMap<String, usize>,
    pub originals: HashMap<MutationId, protocol::InputReplacement>,
    pub undo: HashMap<semio_framework_value::SharedUtf8, Vec<usize>>,
    pub redo: HashMap<semio_framework_value::SharedUtf8, Vec<(usize, usize)>>,
    reverts: HashMap<semio_framework_value::SharedUtf8, HybridLogicalTimestamp>,
}

impl SupersedeLedger {
    /// 🔄️ Rebuilds from `transitions` when they changed since the last build; `originals` answers the original input,
    /// encoded like a supersede replacement, of every operation a supersession names.
    pub fn refresh(&mut self, transitions: &[store::os_spr::MutationEnvelope], originals: impl FnOnce(&BTreeSet<MutationId>) -> HashMap<MutationId, protocol::InputReplacement>) {
        let key = (transitions.len(), transitions.last().map(|envelope| envelope.mutation_id.0.clone()));
        if key == self.key {
            return;
        }
        let mut records = Vec::new();
        let mut reverts: HashMap<semio_framework_value::SharedUtf8, HybridLogicalTimestamp> = HashMap::new();
        for envelope in transitions {
            match store::os_spr::history_transition_from_envelope(envelope) {
                Ok(Some(store::os_spr::HistoryTransition::Supersede(supersede))) => {
                    records.push(SupersedeRecord { transition_id: envelope.mutation_id.0.clone(), actor: envelope.actor.0.clone(), timestamp: envelope.timestamp, scope: supersede.scope, inputs: supersede.inputs, role: SupersedeRole::Edit, entry: 0 })
                }
                Ok(Some(store::os_spr::HistoryTransition::Revert { .. })) => {
                    let newest = reverts.entry(envelope.actor.0.clone()).or_insert(envelope.timestamp);
                    *newest = (*newest).max(envelope.timestamp);
                }
                _ => {}
            }
        }
        let originals = originals(&records.iter().flat_map(|record| record.inputs.iter().map(|input| input.target.clone())).collect());
        *self = Self { key, ..Self::from_records(records, reverts, originals) };
    }

    /// 🧮️ The ledger of decoded `Supersede` `records` (any order; sorted into fold order here, `role` and `entry`
    /// recomputed), each author's newest document-edit undo and the original input of every operation they name.
    pub fn from_records(mut records: Vec<SupersedeRecord>, reverts: HashMap<semio_framework_value::SharedUtf8, HybridLogicalTimestamp>, originals: HashMap<MutationId, protocol::InputReplacement>) -> Self {
        records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp).then_with(|| a.transition_id.cmp(&b.transition_id)));
        let index = records.iter().enumerate().map(|(index, record)| (record.transition_id.clone(), index)).collect();
        let mut ledger = Self { key: (0, None), records, index, originals, undo: HashMap::new(), redo: HashMap::new(), reverts };
        for index in 0..ledger.records.len() {
            ledger.classify(index);
        }
        ledger
    }

    /// 🎯️ The replacement effective for `target` over the first `before` records in the view of `scope`, with the record
    /// that installed it.
    pub fn effective(&self, before: usize, scope: Option<&str>, target: &MutationId) -> Option<(usize, &protocol::InputReplacement)> {
        self.records[..before]
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, record)| record.scope.is_none() || record.scope.as_deref() == scope)
            .find_map(|(index, record)| record.inputs.iter().find(|input| input.target == *target).map(|input| (index, &input.replacement)))
    }

    /// 👑️ The history edit whose input record `index` installs for `target`: the edit itself, a redo's re-authored edit,
    /// an undo's owner of the input it restored; `None` for a restored original input.
    pub fn owner(&self, index: usize, target: &MutationId) -> Option<usize> {
        let record = &self.records[index];
        match record.role {
            SupersedeRole::Edit => Some(index),
            SupersedeRole::Redo => Some(record.entry),
            SupersedeRole::Undo => self.effective(record.entry, self.records[record.entry].scope.as_deref(), target).and_then(|(installed, _)| self.owner(installed, target)),
        }
    }

    /// 👑️ The owner of `target`'s input over the first `before` records in the view of `scope`.
    fn owner_at(&self, before: usize, scope: Option<&str>, target: &MutationId) -> Option<usize> {
        self.effective(before, scope, target).and_then(|(installed, _)| self.owner(installed, target))
    }

    /// ✋️ The inputs history edit `entry` still owns over the first `before` records, in the view of its scope.
    pub fn held(&self, entry: usize, before: usize) -> Vec<&MutationId> {
        let scope = self.records[entry].scope.as_deref();
        self.records[entry].inputs.iter().map(|input| &input.target).filter(|target| self.owner_at(before, scope, target) == Some(entry)).collect()
    }

    /// ⏪️ The input `target` had before history edit `entry`: the effective one in the view of its scope, else the original.
    pub fn before(&self, entry: usize, target: &MutationId) -> Option<protocol::InputReplacement> {
        match self.effective(entry, self.records[entry].scope.as_deref(), target) {
            Some((_, replacement)) => Some(replacement.clone()),
            None => self.originals.get(target).cloned(),
        }
    }

    /// ⏩️ Whether the inputs undo `undo` restored still have, over the first `before` records, the owners it left them with.
    fn undo_stands(&self, undo: usize, before: usize) -> bool {
        let scope = self.records[undo].scope.as_deref();
        self.records[undo].inputs.iter().all(|input| self.owner_at(before, scope, &input.target) == self.owner(undo, &input.target))
    }

    /// 🔁️ Classifies record `index` against its author's stacks: the undo of a history edit that still owns exactly its
    /// targets and whose inputs before it it restores, else the redo of the newest undone edit whose undo still stands and
    /// whose inputs it re-authors, else a new history edit (clearing the author's redos).
    fn classify(&mut self, index: usize) {
        let record = &self.records[index];
        let targets: BTreeSet<&MutationId> = record.inputs.iter().map(|input| &input.target).collect();
        let undone = self.undo.get(&record.actor).and_then(|stack| {
            stack.iter().rev().copied().find(|&entry| {
                self.records[entry].scope == record.scope && self.held(entry, index).into_iter().collect::<BTreeSet<_>>() == targets && record.inputs.iter().all(|input| self.before(entry, &input.target).as_ref() == Some(&input.replacement))
            })
        });
        let redone = self.redo.get(&record.actor).and_then(|stack| stack.last().copied()).filter(|&(entry, undo)| {
            undone.is_none()
                && self.records[entry].scope == record.scope
                && self.records[undo].inputs.iter().map(|input| &input.target).collect::<BTreeSet<_>>() == targets
                && self.undo_stands(undo, index)
                && record.inputs.iter().all(|input| self.records[entry].inputs.iter().any(|edited| edited.target == input.target && edited.replacement == input.replacement))
        });
        let actor = record.actor.clone();
        let (role, entry) = match (undone, redone) {
            (Some(entry), _) => {
                if let Some(stack) = self.undo.get_mut(&actor) {
                    stack.retain(|held| *held != entry);
                }
                self.redo.entry(actor).or_default().push((entry, index));
                (SupersedeRole::Undo, entry)
            }
            (None, Some((entry, _))) => {
                if let Some(stack) = self.redo.get_mut(&actor) {
                    stack.pop();
                }
                self.undo.entry(actor).or_default().push(entry);
                (SupersedeRole::Redo, entry)
            }
            (None, None) => {
                self.redo.remove(&actor);
                self.undo.entry(actor).or_default().push(index);
                (SupersedeRole::Edit, index)
            }
        };
        self.records[index].role = role;
        self.records[index].entry = entry;
    }

    /// 🔎️ The record of transition `transition_id`, with its index.
    pub fn record(&self, transition_id: &str) -> Option<(usize, &SupersedeRecord)> {
        self.index.get(transition_id).map(|&index| (index, &self.records[index]))
    }

    /// ✅️ The history edits in effect now: the owners of the inputs the store folds with (`supersessions`, the effective
    /// supersessions of the final alternative).
    pub fn applied_entries<'a>(&self, supersessions: impl IntoIterator<Item = (&'a MutationId, &'a protocol::EffectiveSupersession)>) -> HashSet<usize> {
        supersessions.into_iter().filter_map(|(target, supersession)| self.index.get(supersession.transition_id.as_str()).and_then(|&index| self.owner(index, target))).collect()
    }

    /// ✅️ Whether the history row of record `index` reads as applied: an edit or a redo while its edit is in effect, an
    /// undo while the edit it undid is not redone.
    pub fn row_applied(&self, index: usize, applied: &HashSet<usize>) -> bool {
        let record = &self.records[index];
        match record.role {
            SupersedeRole::Undo => !self.undo.get(&record.actor).is_some_and(|stack| stack.contains(&record.entry)),
            SupersedeRole::Edit | SupersedeRole::Redo => applied.contains(&record.entry),
        }
    }

    /// ⏪️ Whether `actor` may take history edit `entry` back now: it is theirs, not undone, and in effect.
    pub fn undoable(&self, actor: &str, entry: usize, applied: &HashSet<usize>) -> bool {
        applied.contains(&entry) && self.undo.get(actor).is_some_and(|stack| stack.contains(&entry))
    }

    /// ⏪️ The undo of history edit `entry`: its scope and each input it still owns, in the view of that scope, back to
    /// the input before it (the original input when there was none).
    pub fn restore(&self, entry: usize) -> (Option<String>, Vec<protocol::SupersededInput>) {
        let inputs = self.held(entry, self.records.len()).into_iter().filter_map(|target| self.before(entry, target).map(|replacement| protocol::SupersededInput { target: target.clone(), replacement })).collect();
        (self.records[entry].scope.clone(), inputs)
    }

    /// ⏩️ `actor`'s next redo: the newest undone history edit and its undo, while that undo still stands.
    pub fn redo_top(&self, actor: &str) -> Option<(usize, usize)> {
        self.redo.get(actor).and_then(|stack| stack.last().copied()).filter(|&(_, undo)| self.undo_stands(undo, self.records.len()))
    }

    /// ⏩️ The redo of history edit `entry` undone by `undo`: its scope and its own input for every input the undo restored.
    pub fn redo(&self, entry: usize, undo: usize) -> (Option<String>, Vec<protocol::SupersededInput>) {
        let inputs = self.records[undo].inputs.iter().filter_map(|restored| self.records[entry].inputs.iter().find(|edited| edited.target == restored.target).cloned()).collect();
        (self.records[entry].scope.clone(), inputs)
    }

    /// ⏱️ `actor`'s newest undo of a document edit.
    pub fn newest_revert(&self, actor: &str) -> Option<HybridLogicalTimestamp> {
        self.reverts.get(actor).copied()
    }

    /// 🏷️ The history row of record `index` in every locale: its role, its scope (an overwrite, or the alternative
    /// `alternative` names) and how many mutations it supersedes.
    pub fn label(&self, index: usize, alternative: Option<&str>) -> LocalizedLabel {
        let record = &self.records[index];
        let count = record.inputs.len();
        let (en_role, de_role) = match record.role {
            SupersedeRole::Edit => ("History edited", "Verlauf bearbeitet"),
            SupersedeRole::Undo => ("History edit undone", "Verlaufsbearbeitung rückgängig"),
            SupersedeRole::Redo => ("History edit redone", "Verlaufsbearbeitung wiederhergestellt"),
        };
        let (en_scope, de_scope) = match (&record.scope, alternative) {
            (None, _) => ("overwrite".to_string(), "überschrieben".to_string()),
            (Some(_), Some(name)) => (format!("alternative {name}"), format!("Alternative {name}")),
            (Some(id), None) => (format!("alternative {id}"), format!("Alternative {id}")),
        };
        let (en_count, de_count) = if count == 1 { ("1 mutation".to_string(), "1 Mutation".to_string()) } else { (format!("{count} mutations"), format!("{count} Mutationen")) };
        LocalizedLabel::native(&format!("{en_role} — {en_scope}: {en_count}"), &format!("{de_role} — {de_scope}: {de_count}"))
    }
}
//#endregion 🔖️Supersessions
