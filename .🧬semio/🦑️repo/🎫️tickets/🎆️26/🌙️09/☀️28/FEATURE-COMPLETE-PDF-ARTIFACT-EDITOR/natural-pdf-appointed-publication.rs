//! 🌐 Advances the staged natural input through the mounted import job and paid Store receipt.

use super::*;
use semio_plugin_decl_media::original_media::natural_source::NaturalFileSourceGuest;
use semio_plugin_instance_docs::docs_components::natural_publication::NaturalFilePublication;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    pub(super) fn natural_source_can_close(&self) -> bool {
        self.tools.tool_operations.is_empty() && self.pending.reserved_preparation.is_none()
            && self.pending.reserved_birth.terminal_is_empty() && self.media.reserved_mount.is_none()
            && self.media.reserved_receipt.is_none() && self.media.reserved_mount_construction.terminal_is_empty()
            && self.media.pending_reserved_producer_permit.is_none()
    }

    pub(super) async fn advance_natural_publication(
        &mut self, source: &mut NaturalFileSourceGuest, cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        if self.media.natural_source.is_none() {
            if !source.input_ready() { return Ok(()); }
            let instance = source.loan().operation.instance;
            if self.tools.live_runtime_instance_id != Some(instance) {
                return Err(ValueError::invariant("natural source named another mounted instance").into());
            }
            self.media.natural_source = NaturalFilePublication::admit(instance, &self.stores.store.local_actor_id().0, cx)?;
            return Ok(());
        }
        let mut owner = self.media.natural_source.take();
        let result = self.advance_natural_owner(source, &mut owner, cx).await;
        self.media.natural_source = owner;
        result
    }

    async fn advance_natural_owner(
        &mut self, source: &mut NaturalFileSourceGuest, slot: &mut Option<Box<NaturalFilePublication>>,
        cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        let owner = slot.as_mut().expect("natural original publication owner");
        if !owner.source.matches(cx) {
            return Err(ValueError::invariant("natural source changed its Original publication authority").into());
        }
        let cancelled = source.cancellation_pending();
        if owner.operation.is_none() && cancelled {
            if let Some(receipt) = self.media.reserved_receipt {
                owner.operation = Some(receipt.operation(cx)?);
            }
        }
        if let Some(operation) = owner.operation {
            if let Some(mounted) = self.tools.tool_operations.get_mut(operation.operation.0) {
                if owner.mount_id != Some(mounted.operation.operation) {
                    return Err(ValueError::invariant("natural source mounted operation changed its route").into());
                }
                if cancelled && !mounted.published_artifact && !mounted.user_cancel_requested {
                    let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<bool>(), ..Default::default() };
                    if !cx.retirement_should_yield() && cx.retained_grant().maximum_depth > 0
                        && progress.fits(cx.retained_grant()) && cx.fuel_remaining() != 0 {
                        cx.consume_retained(progress)?;
                        cx.consume_fuel(1);
                        mounted.user_cancel_requested = true;
                    }
                }
                return Ok(());
            }
        } else if !cancelled {
            if !owner.take_source(source, cx)? { return Ok(()); }
            let meta = owner.meta.as_ref().expect("natural source mount owns action metadata");
            self.dispatch_ctx().advance_original_reserved_natural_source(&mut owner.input, meta, &mut owner.operation, cx).await?;
            if owner.mount_id.is_none() {
                if let Some(operation) = owner.operation {
                    owner.mount_id = Some(operation.operation);
                } else if let Some(receipt) = self.media.reserved_receipt {
                    owner.mount_id = Some(receipt.operation(cx)?.operation);
                }
            }
            return Ok(());
        }
        if !self.maintenance_source_terminal_is_empty(cx) {
            let grant = cx.retained_grant();
            self.maintenance_source_close_step(cx, grant).map_err(OriginalTurnFailure::from)?;
            return Ok(());
        }
        if !owner.close_input(cx)? { return Ok(()); }
        if !cancelled && !source.finish_format(source.loan().operation, owner.committed, cx)? { return Ok(()); }
        cx.with_original_retirement(|cx| NaturalFilePublication::retire(slot, cx))?;
        Ok(())
    }
}
