//! 🪪 Serves the accepted first Store through the app's paid egress recipient.
use super::*;
use semio_os_kernel::{
    channel::{FirstArchiveAction, FirstArchiveKey, FirstArchiveReply, FirstArchiveRequest},
    os_spr::history::FirstGenesisHistory,
};
use semio_plugin_decl_media::original_media::natural_source::NaturalFileSourceGuest;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    /// 📤 Reacquires the Store Pack for this caller turn and carries only the first SPR and page cursor.
    pub(super) fn advance_first_archive(
        &mut self, source: &NaturalFileSourceGuest, request: FirstArchiveRequest, output: &mut Vec<u8>,
        cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        let refuse = |output: &mut Vec<u8>| {
            if !FirstArchiveReply::Refused.encode_into(request, output) {
                return Err(ValueError::invariant("first archive denial recipient is absent").into());
            }
            Ok(())
        };
        let Some(committed) = source.first_archive_frontier() else { return refuse(output); };
        let operation = source.loan().operation;
        if request.key != (FirstArchiveKey {
            instance: operation.instance, operation: operation.operation, generation: operation.generation, sequence: source.sequence(),
        }) || self.tools.live_runtime_instance_id != Some(operation.instance) {
            return refuse(output);
        }
        let store = &self.stores.store;
        let current = (store.generation_now(), store.content_revision_now());
        let envelope = store.envelope();
        let mounted = &mut self.media.first_archive;
        let history = if current == committed && request.action == FirstArchiveAction::Open && !mounted.has_resident() {
            if envelope.owner.is_some() || envelope.active_alternative_id.is_some() || envelope.viewer_checkpoint_id.is_some()
                || envelope.open_transaction.is_some() || !envelope.vcs.edits.is_empty() || !envelope.transitions.is_empty()
                || !envelope.conflicts.is_empty() {
                return refuse(output);
            }
            let dialect = envelope.dialect.as_ref().map(|dialect| [
                dialect.artifact_kind.as_str(), dialect.standard.as_str(), dialect.subset.as_str()
            ]);
            let Ok(plan) = FirstGenesisHistory::new(&envelope.id, &envelope.schema, dialect) else { return refuse(output); };
            Some(plan)
        } else {
            None
        };
        mounted.exchange(request, &envelope.id, current, committed, envelope.vcs.genesis.pack(), history.as_ref(), output, cx)
    }
}
