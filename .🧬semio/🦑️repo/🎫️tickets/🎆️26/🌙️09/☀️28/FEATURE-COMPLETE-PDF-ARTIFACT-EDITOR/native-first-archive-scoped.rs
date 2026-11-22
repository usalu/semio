//! 🔏 Couples one borrowed first-genesis archive to its exact source and receipt.
use super::FirstGenesisArchive;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_os_spr::channel::{
    FirstGenesisArchiveOffer, FirstGenesisArchivePageReceipt, FirstGenesisArchiveScope, FirstGenesisArchiveSeal,
    FirstArchiveAction, FirstArchiveReply, FirstArchiveRequest,
};
use semio_os_spr::AppDocumentIdentity;
use semio_os_vcs::io::binary::genesis::ArtifactGenesisPack;
use std::borrow::Borrow;

/// 🎟️ Keeps the accepted source scope across each physical archive page.
pub struct ScopedFirstGenesisArchive<P, S> {
    offer: FirstGenesisArchiveOffer,
    archive: FirstGenesisArchive<P, S>,
    closed: bool,
}

impl<P: Borrow<ArtifactGenesisPack>, S: AsRef<[u8]>> ScopedFirstGenesisArchive<P, S> {
    pub fn new(scope: FirstGenesisArchiveScope, archive: FirstGenesisArchive<P, S>) -> Result<Self, ValueError> {
        let offer = FirstGenesisArchiveOffer::new(scope, archive.extent()).ok_or_else(invalid_receipt)?;
        Ok(Self { offer, archive, closed: false })
    }

    pub fn header(&self) -> &FirstGenesisArchiveOffer { &self.offer }
    pub fn offer_page(&mut self, cx: &mut StepContext<'_>) -> Result<Option<(u32, &[u8])>, OriginalTurnFailure> {
        self.archive.offer(cx).map(|page| page.map(|(offset, bytes)| (offset as u32, bytes)))
    }

    pub fn acknowledge(&mut self, receipt: &FirstGenesisArchivePageReceipt, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.closed { return Err(invalid_receipt().into()); }
        let offset = self.archive.accepted() as u32;
        if receipt.offset.checked_add(receipt.length) == Some(offset)
            && receipt.matches(&self.offer, receipt.offset, receipt.length as usize) {
            return Ok(true);
        }
        let Some(length) = self.offer.page_length(offset) else { return Err(invalid_receipt().into()); };
        if !receipt.matches(&self.offer, offset, length) { return Err(invalid_receipt().into()); }
        self.archive.acknowledge(offset as usize, length, cx)
    }

    pub fn seal(&self) -> Option<FirstGenesisArchiveSeal> {
        (!self.closed).then(|| self.archive.digest()).flatten()
            .map(|sha256| FirstGenesisArchiveSeal { offer: self.offer.clone(), sha256 })
    }

    pub fn cancel(&mut self) { self.archive.cancel(); self.closed = true; }

    /// 📬 Serves the same mounted source with one exact pending page on the bounded Shell lane.
    pub fn exchange(
        &mut self, request: FirstArchiveRequest, document: &AppDocumentIdentity, generation: u64,
        revision: &[u8; 32], output: &mut Vec<u8>, cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        let scope = &self.offer.scope;
        let reply = if !scope.matches_key(request.key) || !scope.matches_current(document, generation, revision) {
            FirstArchiveReply::Refused
        } else if self.closed {
            match request.action {
                FirstArchiveAction::Close | FirstArchiveAction::Cancel => FirstArchiveReply::Closed,
                _ => FirstArchiveReply::Refused,
            }
        } else {
            match request.action {
                FirstArchiveAction::Open => FirstArchiveReply::Opened {
                    extent: self.offer.extent, document: document.parent_document_id.as_deref().ok_or_else(invalid_receipt)?,
                    generation, revision: *revision,
                },
                FirstArchiveAction::Preview { offset }
                    if offset as usize == self.archive.accepted() && self.offer.page_length(offset).is_some() =>
                    self.archive.offer(cx)?.map_or(FirstArchiveReply::Pending, |(_, bytes)| FirstArchiveReply::Page(bytes)),
                FirstArchiveAction::Acknowledge { offset, length }
                    if self.offer.page_length(offset) == Some(length as usize) => {
                    let current = self.archive.accepted();
                    if offset as usize + length as usize == current
                        || offset as usize == current && self.archive.acknowledge(offset as usize, length as usize, cx)?
                    { FirstArchiveReply::Acknowledged } else { FirstArchiveReply::Pending }
                }
                FirstArchiveAction::Seal => self.archive.digest().map_or(FirstArchiveReply::Refused, FirstArchiveReply::Sealed),
                FirstArchiveAction::Cancel => {
                    self.archive.cancel();
                    self.closed = true;
                    FirstArchiveReply::Closed
                }
                FirstArchiveAction::Close if self.archive.digest().is_some() => {
                    self.closed = true;
                    FirstArchiveReply::Closed
                }
                _ => FirstArchiveReply::Refused,
            }
        };
        if !reply.encode_into(request, output) { return Err(invalid_receipt().into()); }
        Ok(())
    }
}

fn invalid_receipt() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "first-genesis archive receipt does not match its accepted source")
}
