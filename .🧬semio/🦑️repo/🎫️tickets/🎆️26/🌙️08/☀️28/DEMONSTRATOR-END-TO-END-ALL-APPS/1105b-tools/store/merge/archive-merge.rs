//! 🏐️ Staged `archive-merge` members of the `instance-commands` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🗃️ The whole-document archive merge, one budgeted step per host poll.
use semio_plugin_document_media::ActiveDocumentArchiveLoadState;
use semio_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_FAILED_CODE, DOCUMENT_LOAD_MEMBERS_DIFFER_CODE, DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, DOCUMENT_LOAD_OTHER_DOCUMENT_CODE, document_load_fault};
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use crate::command_bundle::CommandCtx;
use semio_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use semio_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, PersistedMergePhase, ReferenceSpaceMember};
use protocol::OpText;
use semio_plugin_reactor_app::PluginApp;
use semio_ui_contract::*;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> CommandCtx<'ctx, A, M, X> {
    /// 🔀️ Advances the read-back merge (design §22.22) by ONE budgeted step in the host's own poll turn: the first poll opens the
    /// store's resumable merge, every poll spends at most the turn's remaining fuel on it (the work it really did is consumed from
    /// the turn), and the poll that finishes it requests the operation's terminal: `Ready` with what it took (`completed == total`)
    /// and is ahead by, or the fault that refused it. Nothing is replaced. A cancelled operation retires the cursor with the rest of
    /// its owners; what earlier steps took stays, like any remote batch.
    pub async fn merge_document_archive(&mut self, operation: u64, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let stepped = self.step_decoded_document_archive_merge(operation, identity, cx).await;
        let active = self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed during its merge"))?;
        match stepped {
            Ok(Some((taken, ahead))) => {
                active.completed = taken;
                active.total = taken;
                active.ahead = ahead;
                active.request_terminal(ActiveDocumentArchiveLoadState::Ready);
            }
            Ok(None) => {}
            Err(fault) => active.request_fault(&fault),
        }
        Ok(())
    }

    /// 🧬️ The members of the archive must be the live ones (a member merge is the member's own, not yet available); the decoded log
    /// then joins the store's through its resumable merge — an archive of another document is refused at open.
    async fn open_decoded_document_archive_merge(&mut self, operation: u64) -> Result<(), Fault> {
        let live = self.child_composition.archive_member_entries().await?;
        let generation = self.stores.store.generation();
        let active = self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed before its merge"))?;
        let archive = active.archive.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive merge input owner is absent"))?;
        if !live.iter().eq(archive.members.iter().rev()) {
            return Err(document_load_fault(DOCUMENT_LOAD_MEMBERS_DIFFER_CODE, "the archive's members differ from this document's live members; a member merge is not available"));
        }
        let pack = std::mem::take(&mut archive.parent_pack);
        let history = active.decoded_history.take().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive decoded history owner is absent"))?;
        match self.stores.store.persisted_merge_open(&pack, history) {
            Ok(cursor) => {
                active.merge_cursor = Some(cursor);
                active.merge_generation = generation;
                Ok(())
            }
            Err((error, history)) => {
                active.decoded_history = Some(history);
                Err(match error {
                    store::VcsError::ValidationFailed(detail) => document_load_fault(DOCUMENT_LOAD_OTHER_DOCUMENT_CODE, detail),
                    error => error.into_fault(),
                })
            }
        }
    }

    /// 🪜️ One merge step; answers the events taken and the events this program holds that the archive lacks once the merge finished,
    /// `None` while it has more to do. An open history edit hears the base move like after any remote edit, and derivable children
    /// follow, in the poll that finished the merge.
    async fn step_decoded_document_archive_merge(&mut self, operation: u64, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<(u64, u64)>, Fault> {
        if self.ingest.document_archive_loads.get(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed before its step"))?.merge_cursor.is_none() {
            self.open_decoded_document_archive_merge(operation).await?;
        }
        let budget = cx.fuel_remaining();
        let active = self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed during its step"))?;
        let cursor = active.merge_cursor.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive merge lost its cursor"))?;
        let turn = self.stores.store.persisted_merge_step(cursor, budget).await;
        cx.consume_fuel(turn.work);
        let progress = turn.result.map_err(|error| match error {
            store::VcsError::ValidationFailed(detail) => document_load_fault(DOCUMENT_LOAD_OTHER_DOCUMENT_CODE, detail),
            error => error.into_fault(),
        })?;
        active.completed = progress.completed;
        active.total = progress.total;
        if progress.phase != PersistedMergePhase::Complete {
            return Ok(None);
        }
        let (outcome, generation) = (cursor.outcome(), active.merge_generation);
        self.history.cache = None;
        if self.stores.store.generation() != generation {
            self.as_dispatch().queue_base_moved();
        }
        self.child_composition.follow_derivable_children(&mut *self.stores, identity).await?;
        Ok(Some((u64::try_from(outcome.merged).unwrap_or(u64::MAX), u64::try_from(outcome.ahead).unwrap_or(u64::MAX))))
    }
}
