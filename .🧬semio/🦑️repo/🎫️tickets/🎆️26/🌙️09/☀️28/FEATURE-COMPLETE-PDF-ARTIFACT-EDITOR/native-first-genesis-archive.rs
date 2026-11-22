//! 🧳 Streams the authoritative paid Pack and first SPR as a version-1 memberless archive.
use semio_framework_hash::Sha256;
use semio_framework_job::{OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_value::{ByteReadSource, RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_os_spr::channel::{DOCUMENT_ARCHIVE_MAXIMUM_BYTES, DOCUMENT_ARCHIVE_VERSION};
use semio_os_vcs::io::binary::genesis::ArtifactGenesisPack;
use std::borrow::Borrow;

const PAGE_BYTES: usize = 4096;

/// 🎟️ Borrows one selected genesis and its canonical SPR until every exact page has been received.
pub struct FirstGenesisArchive<P, S> {
    pack: P,
    spr: S,
    cursor: FirstGenesisArchiveCursor,
}

/// 🧮 Retains framing, digest and one pending page with a borrowed source on each offer.
pub struct FirstGenesisArchiveCursor {
    pack_prefix: [u8; 11],
    pack_prefix_len: usize,
    spr_prefix: [u8; 10],
    spr_prefix_len: usize,
    extent: usize,
    accepted: usize,
    page: [u8; PAGE_BYTES],
    pending: usize,
    digest: Sha256,
    binding: Option<OriginalSourceBinding>,
    cancelled: bool,
}

impl<P: Borrow<ArtifactGenesisPack>, S: AsRef<[u8]>> FirstGenesisArchive<P, S> {
    /// 🧾 Fences the current wire extent before reading or flattening the original Pack.
    pub fn new(pack: P, canonical_first_spr: S) -> Result<Self, ValueError> {
        let cursor = FirstGenesisArchiveCursor::new(pack.borrow().byte_len(), canonical_first_spr.as_ref().len())?;
        Ok(Self { pack, spr: canonical_first_spr, cursor })
    }

    pub fn extent(&self) -> usize { self.cursor.extent() }
    pub fn accepted(&self) -> usize { self.cursor.accepted() }

    pub fn offer(&mut self, cx: &mut StepContext<'_>) -> Result<Option<(usize, &[u8])>, OriginalTurnFailure> {
        self.cursor.offer(self.pack.borrow(), self.spr.as_ref(), cx)
    }

    pub fn acknowledge(&mut self, offset: usize, length: usize, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.cursor.acknowledge(offset, length, cx)
    }

    pub fn digest(&self) -> Option<[u8; 32]> { self.cursor.digest() }
    pub fn cancel(&mut self) { self.cursor.cancel(); }
}

impl FirstGenesisArchiveCursor {
    pub fn extent_for(pack: usize, spr: usize) -> Result<usize, ValueError> {
        let mut scratch = [0; 10];
        let extent = 2usize.saturating_add(varint(pack as u64, &mut scratch))
            .saturating_add(varint(spr as u64, &mut scratch)).saturating_add(pack).saturating_add(spr);
        if extent > DOCUMENT_ARCHIVE_MAXIMUM_BYTES {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "first genesis archive exceeds its declared byte authority"));
        }
        Ok(extent)
    }

    pub fn new(pack_length: usize, spr_length: usize) -> Result<Self, ValueError> {
        let extent = Self::extent_for(pack_length, spr_length)?;
        let mut pack_prefix = [0; 11];
        pack_prefix[0] = DOCUMENT_ARCHIVE_VERSION;
        let pack_prefix_len = 1 + varint(pack_length as u64, &mut pack_prefix[1..]);
        let mut spr_prefix = [0; 10];
        let spr_prefix_len = varint(spr_length as u64, &mut spr_prefix);
        Ok(Self {
            pack_prefix, pack_prefix_len, spr_prefix, spr_prefix_len, extent,
            accepted: 0, page: [0; PAGE_BYTES], pending: 0, digest: Sha256::new(), binding: None, cancelled: false,
        })
    }

    pub fn extent(&self) -> usize { self.extent }
    pub fn accepted(&self) -> usize { self.accepted }
    pub fn has_pending(&self) -> bool { self.pending != 0 }

    /// 📄 Holds the same page through a refused grant or transport retry.
    pub fn offer(&mut self, pack: &ArtifactGenesisPack, spr: &[u8], cx: &mut StepContext<'_>) -> Result<Option<(usize, &[u8])>, OriginalTurnFailure> {
        self.check_context(cx)?;
        if self.cancelled || self.accepted == self.extent { return Ok(None); }
        if cx.is_cancelled() { self.cancel(); return Ok(None); }
        if self.pending == 0 {
            let count = (self.extent - self.accepted).min(PAGE_BYTES);
            if !can_process(cx, count, count) { return Ok(None); }
            let grant = cx.retained_grant();
            let Some((result, work)) = cx.with_original_effect(grant, |_, effect| {
                if !effect.performed(count as u64)? { return Ok(false); }
                self.fill(pack, spr, count)?;
                Ok::<bool, ValueError>(true)
            })? else { return Ok(None); };
            cx.consume_fuel(work as u64);
            if !result? { return Ok(None); }
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() })?;
            self.binding.get_or_insert_with(|| OriginalSourceBinding::borrow(cx));
            self.pending = count;
        }
        Ok(Some((self.accepted, &self.page[..self.pending])))
    }

    /// 🧮 Advances and hashes the exact pending page only after its matching receipt.
    pub fn acknowledge(&mut self, offset: usize, length: usize, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.check_context(cx)?;
        if cx.is_cancelled() { self.cancel(); return Ok(false); }
        if self.cancelled || self.pending == 0 || offset != self.accepted || length != self.pending {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "first genesis archive receipt does not match its pending page").into());
        }
        let copy = 2 * std::mem::size_of::<Sha256>() + 2 * std::mem::size_of::<usize>();
        if !can_process(cx, copy, length) { return Ok(false); }
        let Some((digest, work)) = cx.with_original_effect(cx.retained_grant(), |_, effect| {
            if !effect.performed(length as u64)? { return Ok(None); }
            let mut digest = self.digest.clone();
            digest.update(&self.page[..length]);
            Ok::<Option<Sha256>, ValueError>(Some(digest))
        })? else { return Ok(false); };
        cx.consume_fuel(work as u64);
        let Some(digest) = digest? else { return Ok(false); };
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })?;
        self.digest = digest;
        self.accepted += length;
        self.pending = 0;
        Ok(true)
    }

    /// 🔏 Gives the digest only after exact receipt of the archive's last byte.
    pub fn digest(&self) -> Option<[u8; 32]> {
        (!self.cancelled && self.accepted == self.extent && self.pending == 0).then(|| self.digest.clone().finalize())
    }

    /// 🛑 Abandons the offered page and prevents publication of an incomplete archive.
    pub fn cancel(&mut self) {
        self.pending = 0;
        self.cancelled = true;
    }

    fn check_context(&self, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if self.binding.is_some_and(|binding| !binding.matches(cx)) {
            return Err(ValueError::invariant("first genesis archive changed its original caller").into());
        }
        Ok(())
    }

    fn fill(&mut self, pack: &ArtifactGenesisPack, spr: &[u8], count: usize) -> Result<(), ValueError> {
        let mut offset = self.accepted;
        let mut filled = 0;
        let output = &mut self.page[..count];
        copy_segment(&self.pack_prefix[..self.pack_prefix_len], output, &mut offset, &mut filled)?;
        copy_segment(pack, output, &mut offset, &mut filled)?;
        copy_segment(&self.spr_prefix[..self.spr_prefix_len], output, &mut offset, &mut filled)?;
        copy_segment(spr, output, &mut offset, &mut filled)?;
        copy_segment(&[0u8][..], output, &mut offset, &mut filled)?;
        if filled != count { return Err(ValueError::invariant("first genesis archive ended early")); }
        Ok(())
    }
}

fn copy_segment(
    source: &(impl ByteReadSource + ?Sized), output: &mut [u8], offset: &mut usize, filled: &mut usize,
) -> Result<(), ValueError> {
    let length = source.byte_len();
    if *offset >= length {
        *offset -= length;
        return Ok(());
    }
    let count = (output.len() - *filled).min(length - *offset);
    if count != 0 && source.read_bytes(*offset, &mut output[*filled..*filled + count]) != count {
        return Err(ValueError::invariant("first genesis archive source changed its extent"));
    }
    *filled += count;
    *offset = 0;
    Ok(())
}

fn can_process(cx: &StepContext<'_>, copy: usize, work: usize) -> bool {
    let grant = cx.retained_grant();
    !cx.should_yield() && grant.maximum_items != 0 && grant.maximum_depth != 0
        && grant.maximum_copy_bytes >= copy && cx.fuel_remaining() >= work as u64
}

fn varint(mut value: u64, output: &mut [u8]) -> usize {
    for (index, slot) in output.iter_mut().enumerate() {
        let octet = (value & 127) as u8;
        value >>= 7;
        *slot = octet | if value == 0 { 0 } else { 128 };
        if value == 0 { return index + 1; }
    }
    unreachable!("u64 varints fit in ten octets")
}
