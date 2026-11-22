extern crate self as semio_framework_value;
extern crate self as semio_s_artifact_stdio_pdf_inferences;
extern crate self as semio_s_artifact_stdio_pdf_xref_stream;

pub use lexer::{NormalObject, NormalObjectStatus, PdfIndirectLengthCursor, PdfNormalObjectCursor, PdfStreamDictionaryCursor, StreamSpan, StreamSpanCursor};
pub use model::{PdfIndirectObject, PdfPredictor, PdfStreamFilter};
pub use lexer::{HeaderStep, IndirectHeaderCursor, PdfHeader, PdfHeaderCursor, PdfHeaderStep, XrefSectionKind, XrefSectionProbe};
pub use links::{XrefLinkCursor, XrefPrimaryCycle, XrefRevisionSchedule, XrefTarget};
pub use section::{ClassicSectionCursor, ClassicSectionEvent, ClassicSectionReceipt};
pub use footer::{FooterCursor, FooterStep};
pub use xref::StructuredTrailer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueRefusalKind { InvalidValue, UnsupportedOwner, InvariantViolated, WorkLimit, AllocationFailed, OwnershipLimit }

#[derive(Debug)]
pub struct ValueError { pub kind: ValueRefusalKind, pub message: &'static str }

impl ValueError {
    pub fn literal(kind: ValueRefusalKind, message: &'static str) -> Self { Self { kind, message } }
    pub fn invariant(message: &'static str) -> Self { Self::literal(ValueRefusalKind::InvariantViolated, message) }
    pub fn into_message(self) -> String { self.message.to_owned() }
}

#[derive(Debug)]
pub enum PdfEngineError { Malformed(String) }

pub type PResult<T> = Result<T, PdfEngineError>;

pub fn malformed<T>(reason: &str) -> PResult<T> { Err(PdfEngineError::Malformed(reason.to_owned())) }

#[derive(Clone, Copy)]
pub struct FlateGrant { pub work: usize, pub allocation_bytes: usize }

pub enum FlateStatus { NeedsHistory { bytes: usize }, NeedsOutput, Yield, Complete, Cancelled }

pub struct FlateStep {
    pub work: usize,
    pub allocated_bytes: usize,
    pub written_bytes: usize,
    pub status: Result<FlateStatus, ValueError>,
}

pub struct PdfFlateCursor;
pub struct PdfLzwCursor;
pub struct PdfPredictorCursor;

pub struct CodecRetirement { pub work: usize, pub released_bytes: usize, pub complete: bool }
pub struct CodecDemand { pub release_bytes: usize }

impl PdfFlateCursor {
    pub fn new(_: usize) -> Self { unreachable!("this law exercises unfiltered or packet-coded sources") }
}

impl PdfLzwCursor {
    pub fn new(_: usize, _: bool) -> Self { unreachable!("this law exercises unfiltered or packet-coded sources") }
}

impl PdfPredictorCursor {
    pub fn flate(_: usize, _: usize, _: u32, _: u32, _: u32, _: u32) -> Result<Self, ValueError> { unreachable!() }
    pub fn lzw(_: usize, _: usize, _: bool, _: u32, _: u32, _: u32, _: u32) -> Result<Self, ValueError> { unreachable!() }
}

macro_rules! inactive_codec {
    ($($codec:ty),+) => {$(
        impl $codec {
            pub fn progress(&self) -> (usize, usize) { unreachable!() }
            pub fn next_reserves_history(&self) -> bool { unreachable!() }
            pub fn advance(&mut self, _: &[u8], _: &mut [u8], _: FlateGrant, _: bool) -> FlateStep { unreachable!() }
            pub fn begin_close(&mut self) { unreachable!() }
            pub fn retirement_demand(&self) -> CodecDemand { unreachable!() }
            pub fn close_step(&mut self, _: usize, _: usize) -> CodecRetirement { unreachable!() }
        }
    )+};
}

inactive_codec!(PdfFlateCursor, PdfLzwCursor, PdfPredictorCursor);

pub struct XrefStreamSectionCursor;
pub struct XrefStreamReceipt {
    pub event: Result<XrefStreamEvent, xref::XrefStreamError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

pub enum XrefStreamEvent {
    More, Row { number: u32, entry: Option<xref::XrefEntry> }, Complete, Cancelled, Closed,
    NeedsAllocation { bytes: usize }, NeedsRelease { bytes: usize },
}

impl XrefStreamSectionCursor {
    pub fn new(_: usize, _: CosLimits, _: usize, _: usize) -> Self { Self }
    pub fn advance(&mut self, _: &[u8], _: CosGrant, _: bool) -> XrefStreamReceipt { unreachable!() }
    pub fn ready_trailer(&self) -> Option<&[PdfDictEntry]> { unreachable!() }
    pub fn take_trailer(&mut self) -> Option<Vec<PdfDictEntry>> { unreachable!() }
    pub fn recover_compatible(&mut self, _: &[u8]) -> PResult<bool> { unreachable!() }
    pub fn begin_close(&mut self) {}
    pub fn close_step(&mut self, _: CosGrant) -> XrefStreamReceipt {
        XrefStreamReceipt { event: Ok(XrefStreamEvent::Closed), work: 0, allocated_bytes: 0, released_bytes: 0 }
    }
}

