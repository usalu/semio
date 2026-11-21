//! 📦️ Private PDF JPEG admission over the exact paged octets retained by the picker.
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::PdfDctResourceIdentity;
use semio_framework_artifact_reference::ArtifactRef;
use semio_plugin::__value::{
    paged::{ByteReadSource, PagedBytes, PAGED_BYTES_CHUNK_BYTES},
    retirement::controlled::ControlledRetirement,
    OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand, ValueError, ValueRefusalKind,
};
use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::{
    BaselineJpgCursor, BaselineJpgMetadata, JpgByteSource,
};

pub const MAX_PICKED_JPEG_BYTES: usize = 4 * 1024 * 1024;
const BASELINE_STEP_WORK: u64 = 4096;

pub type PdfPickedJpegBytes = PagedBytes<MAX_PICKED_JPEG_BYTES>;

struct JpegSource<'a>(&'a PdfPickedJpegBytes);

impl JpgByteSource for JpegSource<'_> {
    fn len(&self) -> usize { self.0.byte_len() }
    fn byte(&self, index: usize) -> Option<u8> { self.0.byte_at(index) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdfJpegAdmissionProgress {
    Checking { position: usize, length: usize },
    Hashing { position: usize, length: usize },
    Admitted,
}

pub struct PdfJpegAdmission {
    source: ControlledRetirement<PdfPickedJpegBytes>,
    proof: BaselineJpgCursor,
    hash: Option<PdfDctResourceIdentity>,
    closing: bool,
    faulted: bool,
}

pub struct AdmittedPdfJpeg {
    metadata: BaselineJpgMetadata,
    digest: [u8; 32],
    source: ControlledRetirement<PdfPickedJpegBytes>,
}

impl AdmittedPdfJpeg {
    /// 📐️ Lends the profile and color metadata attested by the complete entropy scan.
    pub fn metadata(&self) -> BaselineJpgMetadata { self.metadata }

    /// 📖️ Lends the exact original bytes to a decoder or sequential serializer.
    pub fn encoded(&self) -> Option<&dyn ByteReadSource> {
        self.source.original().map(|bytes| bytes as &dyn ByteReadSource)
    }

    /// 🔐️ Gives the fixed-size native identity for the consuming graph transaction.
    pub fn digest(&self) -> [u8; 32] { self.digest }

    /// 🧾️ Materializes the resource reference under the caller's publication allocation grant.
    pub fn reference(&self) -> ArtifactRef { PdfDctResourceIdentity::reference_for_digest(self.digest) }

    /// 📤️ Transfers the admitted retained owner into graph custody or original retirement.
    pub fn into_source(self) -> ControlledRetirement<PdfPickedJpegBytes> { self.source }
}

impl PdfJpegAdmission {
    /// 🛬️ Takes an already admitted original picker owner without copying its encoded pages.
    pub fn new(source: ControlledRetirement<PdfPickedJpegBytes>) -> Result<Self, ControlledRetirement<PdfPickedJpegBytes>> {
        let Some(length) = source.original().map(ByteReadSource::byte_len) else { return Err(source); };
        if !source.original_is_untouched() || !(4..=MAX_PICKED_JPEG_BYTES).contains(&length) {
            return Err(source);
        }
        Ok(Self { source, proof: BaselineJpgCursor::new(length), hash: None, closing: false, faulted: false })
    }

    /// 📈️ Reports byte position for the currently active pass.
    pub fn progress(&self) -> PdfJpegAdmissionProgress {
        let length = self.source.original().map_or(0, ByteReadSource::byte_len);
        match &self.hash {
            None => PdfJpegAdmissionProgress::Checking { position: self.proof.position(), length },
            Some(hash) if hash.complete(length) => PdfJpegAdmissionProgress::Admitted,
            Some(hash) => PdfJpegAdmissionProgress::Hashing { position: hash.position(), length },
        }
    }

    /// 🪜️ Performs one bounded baseline unit or hashes one bounded compressed page.
    pub fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<PdfJpegAdmissionProgress, ValueError> {
        if self.closing || self.faulted {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "PDF JPEG import is closing or faulted"));
        }
        let source = self.source.original().ok_or_else(|| ValueError::invariant("PDF JPEG source owner is empty"))?;
        let allowance = effect.remaining_work_units()?;
        if let Some(hash) = &mut self.hash {
            let count = source.byte_len().saturating_sub(hash.position())
                .min(PAGED_BYTES_CHUNK_BYTES).min(usize::try_from(allowance).unwrap_or(usize::MAX));
            if count > 0 && effect.performed(count as u64)? {
                let mut page = [0; PAGED_BYTES_CHUNK_BYTES];
                if source.read_bytes(hash.position(), &mut page[..count]) != count {
                    self.faulted = true;
                    return Err(ValueError::invariant("PDF JPEG source page changed while hashing"));
                }
                hash.append(&page[..count]);
            }
        } else if allowance >= BASELINE_STEP_WORK && effect.performed(BASELINE_STEP_WORK)? {
            if let Err(error) = self.proof.step(&JpegSource(source), &mut |_| {}) {
                self.faulted = true;
                return Err(error.into());
            }
            if let Some(metadata) = self.proof.complete() {
                self.hash = Some(PdfDctResourceIdentity::new(metadata.color_transform)
                    .ok_or_else(|| ValueError::invariant("PDF JPEG profile emitted an unsupported color transform"))?);
            }
        }
        Ok(self.progress())
    }

    /// ✅️ Returns the exact owner only after successful validation and hashing.
    pub fn finish(self) -> Result<AdmittedPdfJpeg, Self> {
        if self.closing || self.faulted || self.progress() != PdfJpegAdmissionProgress::Admitted {
            return Err(self);
        }
        let Self { source, proof, hash, .. } = self;
        Ok(AdmittedPdfJpeg {
            metadata: proof.complete().expect("admitted PDF JPEG proof"),
            digest: hash.expect("admitted PDF JPEG identity").digest(),
            source,
        })
    }

    /// ♻️ Selects controlled retirement after cancellation, rejection, or lost publication.
    pub fn begin_close(&mut self) { self.closing = true; }

    /// 📏️ Quotes the next native page retirement without moving the original owner.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.source.next_demand(body)
    }

    /// 🧹️ Retires the exact original through the repository's paid page-owner cursor.
    pub fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        self.closing = true;
        self.source.step(body, effect)
    }

    /// ⏹️ Indicates that every original retained page has been retired.
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.source.terminal_is_empty() }
}
