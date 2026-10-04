//! 📥️ Retained typed pack session: the artifact-neutral mounted ingress for one canonical `.spk`
//! document. It gates the artifact's own header before any semantic allocation, pages the canonical
//! stream into the framework retained cursors (source → segment → catalog → value) and hands every
//! value token to an artifact-owned [`RetainedTypedPackOwner`], one bounded unit of work per grant.

use super::mounted_pack_rt as mounted;
use semio_framework_pack_error::PackError;
use semio_framework_value::{ValueRefusalKind,native_decoding::NativeDecodeControl};

fn fault(kind:ValueRefusalKind,detail:&'static str)->PackError{PackError::RetainedMalformed{kind,what:"mounted-pack",offset:0,detail}}

/// 🧾️ Keeps this reservation's actual charge separate from the original source fault.
#[derive(Debug)]
pub struct RetainedTypedPackAllocationError{pub allocated_bytes:usize,pub fault:PackError}

/// 🧬️ The artifact-specific half of a mounted pack session: consumes catalog/value events
/// straight into typed domain owners, never into a schema-erased record tree.
pub trait RetainedTypedPackOwner {
    type Value;
    fn accept(&mut self, token: mounted::RetainedValueToken, catalog: &mounted::RetainedPackCatalogCursor, control:&mut NativeDecodeControl<'_>) -> Result<(), PackError>;
    fn grant_symbol(&mut self, catalog: &mounted::RetainedPackCatalogCursor, control:&mut NativeDecodeControl<'_>) -> Result<bool, PackError>;
    fn take(&mut self) -> Option<Self::Value>;
    fn close_step(&mut self) -> bool;
    fn terminal_is_empty(&self) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedTypedPackPhase {
    Header,
    Ingress,
    Drive,
    Ready,
    Published,
    Closing,
    Closed,
}

/// 🧾️ One close grant's outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedTypedPackCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

/// 🧵️ Worker-owned mounted session over one exact byte count: `header` is matched byte by
/// byte and rejected before the owner or any cursor is allocated; every byte after it is handed
/// unchanged to the canonical retained page source.
pub struct RetainedTypedPackSession<O: RetainedTypedPackOwner> {
    phase: RetainedTypedPackPhase,
    header: Vec<u8>,
    header_len: usize,
    expected_bytes: usize,
    maximum_items: usize,
    maximum_depth: u16,
    owner_factory: fn(&mut NativeDecodeControl<'_>) -> Result<O, PackError>,
    page: [u8; mounted::RETAINED_PACK_PAGE_BYTES],
    page_len: usize,
    admitted: usize,
    source: std::mem::ManuallyDrop<Option<mounted::RetainedPackSourceCursor>>,
    anchor: std::mem::ManuallyDrop<Option<mounted::RetainedPackAnchorCursor>>,
    segment: std::mem::ManuallyDrop<Option<mounted::RetainedPackSegmentCursor>>,
    catalog: std::mem::ManuallyDrop<Option<mounted::RetainedPackCatalogCursor>>,
    value: std::mem::ManuallyDrop<Option<mounted::RetainedValueCursor>>,
    typed: std::mem::ManuallyDrop<Option<O>>,
    catalog_value: std::mem::ManuallyDrop<Option<mounted::RetainedPackCatalog>>,
    document_byte: Option<(u64, u8)>,
    source_complete: bool,
    segment_complete: bool,
    anchor_ready: bool,
    catalog_complete: bool,
    value_sealed: bool,
    value_complete: bool,
    fault: Option<PackError>,
}

impl<O: RetainedTypedPackOwner> RetainedTypedPackSession<O> {
    /// 🚪️ Preflights exact credits only; nothing semantic exists until `header` has passed.
    pub fn new(header: Vec<u8>, expected_bytes: usize, maximum_items: usize, maximum_depth: u16, owner_factory: fn(&mut NativeDecodeControl<'_>) -> Result<O, PackError>, control:&mut NativeDecodeControl<'_>) -> Result<Self, PackError> {
        control.checkpoint().map_err(PackError::from)?;
        if expected_bytes <= header.len() || maximum_items == 0 || maximum_depth == 0 {
            return Err(fault(ValueRefusalKind::InvalidValue,"mounted-pack.exact-credits"));
        }
        if expected_bytes>super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES{return Err(fault(ValueRefusalKind::WorkLimit,"mounted-pack.exact-credits"));}
        Ok(Self {
            phase: if header.is_empty() { RetainedTypedPackPhase::Ingress } else { RetainedTypedPackPhase::Header },
            header,
            header_len: 0,
            expected_bytes,
            maximum_items,
            maximum_depth,
            owner_factory,
            page: [0; mounted::RETAINED_PACK_PAGE_BYTES],
            page_len: 0,
            admitted: 0,
            source: std::mem::ManuallyDrop::new(None),
            anchor: std::mem::ManuallyDrop::new(None),
            segment: std::mem::ManuallyDrop::new(None),
            catalog: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            typed: std::mem::ManuallyDrop::new(None),
            catalog_value: std::mem::ManuallyDrop::new(None),
            document_byte: None,
            source_complete: false,
            segment_complete: false,
            anchor_ready: false,
            catalog_complete: false,
            value_sealed: false,
            value_complete: false,
            fault: None,
        })
    }

    fn allocate_after_header(&mut self,control:&mut NativeDecodeControl<'_>) -> Result<(), PackError> {
        control.checkpoint().map_err(PackError::from)?;
        let canonical = self.expected_bytes - self.header.len();
        let pages = canonical.div_ceil(mounted::RETAINED_PACK_PAGE_BYTES);
        let maximum_symbols = self.maximum_items.min(u32::MAX as usize) as u32;
        let decoded = super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES as u64;
        let limits = || mounted::PackLimits { max_file_len: decoded, max_segment_len: decoded, max_symbols: maximum_symbols, max_depth: self.maximum_depth, max_items: self.maximum_items as u64, max_total_alloc: decoded };
        let maximum_source_allocation_bytes = super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES.checked_mul(4).ok_or_else(||fault(ValueRefusalKind::OwnershipLimit,"mounted-pack.source-allocation-credits"))?;
        *self.source = Some(mounted::RetainedPackSourceCursor::try_new(pages, canonical, maximum_source_allocation_bytes).map_err(|fault|fault.into_pack_error("mounted-pack.source-preflight",0))?);
        *self.anchor = Some(mounted::RetainedPackAnchorCursor::new());
        *self.segment = Some(mounted::RetainedPackSegmentCursor::try_new(limits(), super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES)?);
        *self.catalog = Some(
            mounted::RetainedPackCatalogCursor::try_new(limits(), maximum_symbols as usize, decoded as usize, decoded as usize, self.maximum_items, super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES)
                .map_err(|fault|fault.into_pack_error("mounted-pack.catalog-preflight"))?,
        );
        *self.value = Some(mounted::RetainedValueCursor::try_new(limits(), super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES)?);
        *self.typed = Some((self.owner_factory)(control)?);
        self.phase = RetainedTypedPackPhase::Ingress;
        Ok(())
    }

    /// 📥️ Admits one byte; a refused byte is handed back unchanged.
    pub fn admit_byte(&mut self, value: u8, control:&mut NativeDecodeControl<'_>) -> Result<(), u8> {
        if let Err(error)=control.checkpoint(){self.remember_fault(PackError::from(error));return Err(value);}
        if self.fault.is_some()||!matches!(self.phase, RetainedTypedPackPhase::Header | RetainedTypedPackPhase::Ingress) || self.admitted == self.expected_bytes {
            return Err(value);
        }
        if self.header_len < self.header.len() {
            if value != self.header[self.header_len] {
                return Err(value);
            }
            self.header_len += 1;
            self.admitted += 1;
            if self.header_len == self.header.len() {
              if let Err(error)=self.allocate_after_header(control){
                self.remember_fault(error);
                self.admitted -= 1;
                self.header_len -= 1;
                return Err(value);
              }
            }
            return Ok(());
        }
        if self.source.is_none(){if let Err(error)=self.allocate_after_header(control){self.remember_fault(error);return Err(value);}}
        if !self.source.as_ref().is_some_and(mounted::RetainedPackSourceCursor::has_reserved_page) {
            return Err(value);
        }
        self.page[self.page_len] = value;
        self.page_len += 1;
        self.admitted += 1;
        if self.page_len == mounted::RETAINED_PACK_PAGE_BYTES {
          if let Err(error)=self.flush_page(){
            self.remember_fault(error);
            self.admitted -= 1;
            return Err(value);
          }
        }
        Ok(())
    }

    fn flush_page(&mut self) -> Result<(), PackError> {
        if self.page_len == 0 {
            return Ok(());
        }
        let len = self.page_len;
        let source = self.source.as_mut().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.source-owner"))?;
        source.preflight_page(len).map_err(|fault|fault.into_pack_error("mounted-pack.page-preflight",0))?;
        let page = mounted::RetainedPackPage::try_from_array(std::mem::replace(&mut self.page, [0; mounted::RETAINED_PACK_PAGE_BYTES]), len).map_err(|_|fault(ValueRefusalKind::InvariantViolated,"mounted-pack.page-owner"))?;
        self.page_len = 0;
        if let Err(page) = source.admit_page(page) {
            (self.page, self.page_len) = page.into_array();
            return Err(fault(ValueRefusalKind::InvariantViolated,"mounted-pack.producer-handback"));
        }
        Ok(())
    }

    /// 🔏️ Seals ingress after exactly `expected_bytes`.
    pub fn seal(&mut self,control:&mut NativeDecodeControl<'_>) -> Result<(), PackError> {
        control.checkpoint().map_err(PackError::from)?;
        if self.admitted != self.expected_bytes || self.header_len != self.header.len() {
            return Err(fault(ValueRefusalKind::InvariantViolated,"mounted-pack.exact-byte-seal"));
        }
        self.flush_page()?;
        self.source.as_mut().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.source-owner"))?.seal().map_err(|fault|fault.into_pack_error("mounted-pack.source-seal",0))?;
        self.phase = RetainedTypedPackPhase::Drive;
        Ok(())
    }

    /// ⏱️ One bounded unit of retained decode work; `true` once the typed value is ready.
    pub fn grant(&mut self,control:&mut NativeDecodeControl<'_>) -> Result<bool, PackError> {
        if let Some(error)=self.fault.take(){return Err(error);}
        control.checkpoint().map_err(PackError::from)?;
        if matches!(self.phase, RetainedTypedPackPhase::Ready | RetainedTypedPackPhase::Published) {
            return Ok(true);
        }
        if self.phase != RetainedTypedPackPhase::Drive {
            return Err(fault(ValueRefusalKind::InvariantViolated,"mounted-pack.missing-seal"));
        }
        if self.next_retained_allocation_bytes()?.is_some() {
            return Ok(false);
        }
        if self.typed.as_mut().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.typed-owner"))?.grant_symbol(self.catalog.as_ref().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.catalog-owner"))?,control)? {
            return Ok(false);
        }
        if let Some((index, byte)) = self.document_byte {
            if self.value.as_ref().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.value-owner"))?.ingress_ready() {
                self.document_byte = None;
                self.value.as_mut().expect("mounted value cursor retained").admit_byte(index, byte).map_err(|_|fault(ValueRefusalKind::InvariantViolated,"mounted-pack.value-backpressure"))?;
                return Ok(false);
            }
        }
        if !self.value_complete {
            if let Some(token) = self.value.as_mut().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.value-owner"))?.grant()? {
                self.value_complete = matches!(token, mounted::RetainedValueToken::Complete { .. });
                self.typed.as_mut().expect("mounted typed owner retained").accept(token, self.catalog.as_ref().expect("mounted catalog retained"),control)?;
                return Ok(false);
            }
        }
        if self.catalog_complete && !self.value_sealed {
            let bytes = self.catalog.as_ref().expect("mounted catalog retained").document_bytes();
            self.value.as_mut().expect("mounted value cursor retained").seal(bytes)?;
            self.value_sealed = true;
            return Ok(false);
        }
        if self.catalog.as_ref().is_some_and(mounted::RetainedPackCatalogCursor::has_pending_input) {
            let event = self.catalog.as_mut().expect("mounted catalog retained").grant().map_err(|fault|fault.into_pack_error("mounted-pack.catalog-malformed"))?;
            self.catalog_event(event);
            return Ok(false);
        }
        if self.document_byte.is_none() && !self.segment_complete && (self.segment.as_ref().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.segment-owner"))?.preflight().is_err() || self.source_complete) {
            if let Some(event) = self.segment.as_mut().expect("mounted segment retained").grant()? {
                self.segment_complete = matches!(event, mounted::RetainedPackSegmentEvent::PackComplete { .. });
                let catalog = self.catalog.as_mut().expect("mounted catalog retained");
                catalog.admit(event).map_err(|_|fault(ValueRefusalKind::InvariantViolated,"mounted-pack.catalog-backpressure"))?;
                let event = catalog.grant().map_err(|fault|fault.into_pack_error("mounted-pack.catalog-malformed"))?;
                self.catalog_event(event);
                return Ok(false);
            }
        }
        if !self.source_complete && self.segment.as_ref().expect("mounted segment retained").preflight().is_ok() {
            if let Some(event) = self.source.as_mut().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.source-owner"))?.grant().map_err(|fault|fault.into_pack_error("mounted-pack.source-grant",0))? {
                self.source_complete = matches!(event, mounted::RetainedPackSourceEvent::Complete { .. });
                self.anchor.as_mut().expect("mounted anchor retained").grant(Some(event))?;
                self.segment.as_mut().expect("mounted segment retained").admit(event).map_err(|_|fault(ValueRefusalKind::InvariantViolated,"mounted-pack.segment-handback"))?;
                return Ok(false);
            }
        }
        if self.source_complete && !self.anchor_ready {
            self.anchor_ready = self.anchor.as_mut().expect("mounted anchor retained").grant(None)?;
            return Ok(false);
        }
        if self.anchor_ready && self.catalog_complete && self.value_complete && self.catalog_value.is_none() {
            let superblock = self.anchor.as_mut().expect("mounted anchor retained").take().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.anchor-handoff"))?;
            *self.catalog_value = self.catalog.as_mut().expect("mounted catalog retained").take(superblock).map_err(|fault|fault.into_pack_error("mounted-pack.catalog-validation"))?;
            self.phase = RetainedTypedPackPhase::Ready;
            return Ok(true);
        }
        Ok(false)
    }

    fn catalog_event(&mut self, event: Option<mounted::RetainedPackCatalogEvent>) {
        match event {
            Some(mounted::RetainedPackCatalogEvent::DocumentByte { index, value, .. }) => self.document_byte = Some((index, value)),
            Some(mounted::RetainedPackCatalogEvent::Complete) => self.catalog_complete = true,
            _ => {}
        }
    }

    /// 🤝️ Hands the typed value over exactly once.
    pub fn take(&mut self) -> Option<O::Value> {
        if self.phase != RetainedTypedPackPhase::Ready {
            return None;
        }
        let value = self.typed.as_mut()?.take()?;
        self.phase = RetainedTypedPackPhase::Published;
        Some(value)
    }

    pub fn progress(&self) -> Option<mounted::RetainedPackSourceProgress> {
        self.source.as_ref().map(mounted::RetainedPackSourceCursor::progress)
    }

    pub fn semantic_allocated(&self) -> bool {
        self.typed.is_some()
    }

    /// 📏️ The exact allocation the next grant needs, bounded by the envelope close budget.
    pub fn next_retained_allocation_bytes(&mut self) -> Result<Option<usize>, PackError> {
        let requested = match self.phase {
            RetainedTypedPackPhase::Ingress => match self.source.as_ref() {
                Some(source) if !source.has_reserved_page() => Some(source.next_allocation_bytes().map_err(|fault|fault.into_pack_error("mounted-pack.source-allocation",0))?),
                _ => None,
            },
            RetainedTypedPackPhase::Drive => match self.segment.as_ref().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.segment-owner"))?.next_allocation_bytes() {
                Some(requested) => Some(requested),
                None => match self.value.as_mut().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.value-owner"))?.next_allocation_bytes()? {
                    Some(requested) => Some(requested),
                    None => self.catalog.as_mut().ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"mounted-pack.catalog-owner"))?.next_allocation_bytes().map_err(|fault|fault.into_pack_error("mounted-pack.catalog-allocation"))?,
                },
            },
            _ => None,
        };
        if let Some(requested) = requested {
            self.retained_allocated_bytes().checked_add(requested).filter(|total| *total <= super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES).ok_or_else(||fault(ValueRefusalKind::OwnershipLimit,"mounted-pack.retained-allocation-credits"))?;
        }
        Ok(requested)
    }

    /// 🧱️ Preserves the pending reservation's charge and its actual producer fault.
    pub fn reserve_retained_allocation(&mut self,maximum_bytes:usize,control:&mut NativeDecodeControl<'_>)->Result<mounted::RetainedPackSourceAllocationStep,RetainedTypedPackAllocationError>{
        control.checkpoint().map_err(|error|RetainedTypedPackAllocationError{allocated_bytes:0,fault:PackError::from(error)})?;
        let remaining=super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES.saturating_sub(self.retained_allocated_bytes());
        let missing=|detail|RetainedTypedPackAllocationError{allocated_bytes:0,fault:fault(ValueRefusalKind::InvariantViolated,detail)};
        let source_error=|error:mounted::RetainedPackSourceAllocationError|RetainedTypedPackAllocationError{allocated_bytes:error.allocated_bytes,fault:error.fault.into_pack_error("mounted-pack.source-allocation",0)};
        match self.phase{
            RetainedTypedPackPhase::Ingress=>self.source.as_mut().ok_or_else(||missing("mounted-pack.source-owner"))?.reserve_page(maximum_bytes.min(remaining)).map_err(source_error),
            RetainedTypedPackPhase::Drive=>{
                let segment=self.segment.as_mut().ok_or_else(||missing("mounted-pack.segment-owner"))?;
                if segment.next_allocation_bytes().is_some(){return segment.reserve_allocation(maximum_bytes.min(remaining)).map_err(source_error);}
                let value=self.value.as_mut().ok_or_else(||missing("mounted-pack.value-owner"))?;
                if value.next_allocation_bytes().map_err(|error|RetainedTypedPackAllocationError{allocated_bytes:0,fault:error})?.is_some(){
                    return value.reserve_allocation(maximum_bytes.min(remaining)).map(|step|mounted::RetainedPackSourceAllocationStep{progressed:step.progressed,allocated_bytes:step.allocated_bytes}).map_err(|error|RetainedTypedPackAllocationError{allocated_bytes:error.allocated_bytes,fault:error.fault});
                }
                self.catalog.as_mut().ok_or_else(||missing("mounted-pack.catalog-owner"))?.reserve_allocation(maximum_bytes.min(remaining)).map(|step|mounted::RetainedPackSourceAllocationStep{progressed:step.progressed,allocated_bytes:step.allocated_bytes}).map_err(|error|RetainedTypedPackAllocationError{allocated_bytes:error.allocated_bytes,fault:error.fault.into_pack_error("mounted-pack.catalog-allocation")})
            }
            _=>Ok(mounted::RetainedPackSourceAllocationStep::default()),
        }
    }

    pub fn retained_allocated_bytes(&self) -> usize {
        self.source.as_ref().map_or(0, mounted::RetainedPackSourceCursor::allocated_bytes)
            + self.segment.as_ref().map_or(0, mounted::RetainedPackSegmentCursor::allocated_bytes)
            + self.catalog.as_ref().map_or(0, mounted::RetainedPackCatalogCursor::allocated_bytes)
            + self.value.as_ref().map_or(0, mounted::RetainedValueCursor::allocated_bytes)
    }

    pub fn request_cancel(&mut self) {
        if let Some(source) = self.source.as_mut() {
            source.request_cancel();
        }
        self.phase = RetainedTypedPackPhase::Closing;
    }

    /// 🧹️ Releases one owner per grant, typed owner first, source last.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<RetainedTypedPackCloseStep, PackError> {
        let one = RetainedTypedPackCloseStep::Pending { released_items: 1, released_bytes: 0 };
        let retained = |step: mounted::RetainedPackCloseStep| match step {
            mounted::RetainedPackCloseStep::Pending { released_items, released_bytes } => Some(RetainedTypedPackCloseStep::Pending { released_items, released_bytes }),
            mounted::RetainedPackCloseStep::Complete => None,
        };
        if maximum_items == 0 {
            return Ok(RetainedTypedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.phase = RetainedTypedPackPhase::Closing;
        if self.document_byte.take().is_some() {
            return Ok(one);
        }
        if self.page_len != 0 {
            self.page.fill(0);
            self.page_len = 0;
            return Ok(one);
        }
        if self.catalog_value.take().is_some() {
            return Ok(one);
        }
        if let Some(typed) = self.typed.as_mut() {
            if typed.close_step() {
                drop(self.typed.take());
            }
            return Ok(one);
        }
        if let Some(value) = self.value.as_mut() {
            if let Some(step) = retained(value.close_step(1, maximum_bytes)?) {
                return Ok(step);
            }
            drop(self.value.take());
            return Ok(one);
        }
        if let Some(catalog) = self.catalog.as_mut() {
            if let Some(step) = retained(catalog.close_step(1, maximum_bytes).map_err(|error|PackError::from_paged_refusal(error,"mounted-pack.close",0))?) {
                return Ok(step);
            }
            drop(self.catalog.take());
            return Ok(one);
        }
        if let Some(segment) = self.segment.as_mut() {
            if let Some(step) = retained(segment.close_step(1, maximum_bytes)) {
                return Ok(step);
            }
            drop(self.segment.take());
            return Ok(one);
        }
        if let Some(anchor) = self.anchor.as_mut() {
            anchor.close_step();
            drop(self.anchor.take());
            return Ok(one);
        }
        if let Some(source) = self.source.as_mut() {
            if let Some(step) = retained(source.close_step(1, maximum_bytes).map_err(|error|PackError::from_paged_refusal(error,"mounted-pack.close",0))?) {
                return Ok(step);
            }
            drop(self.source.take());
            return Ok(one);
        }
        self.fault=None;
        self.phase = RetainedTypedPackPhase::Closed;
        Ok(RetainedTypedPackCloseStep::Complete)
    }

    pub fn next_retained_release_allocation_bytes(&self)->Result<Option<usize>,PackError>{
        if self.document_byte.is_some()||self.page_len!=0||self.catalog_value.is_some()||self.typed.is_some(){return Ok(None);}
        if let Some(value)=self.value.as_ref(){return Ok(value.next_release_allocation_bytes());}
        if let Some(catalog)=self.catalog.as_ref(){return catalog.next_release_allocation_bytes().map_err(|error|PackError::from_paged_refusal(error,"mounted-pack.catalog-release",0));}
        if let Some(segment)=self.segment.as_ref(){return Ok(segment.next_release_allocation_bytes());}
        self.source.as_ref().map(|source|source.next_release_allocation_bytes().map(Some).map_err(|error|PackError::from_paged_refusal(error,"mounted-pack.source-release",0))).unwrap_or(Ok(None))
    }

    /// 🪪️ Borrows the original ingress fault while the refused byte stays with its producer.
    pub fn fault(&self)->Option<&PackError>{self.fault.as_ref()}

    fn remember_fault(&mut self,error:PackError){if self.fault.is_none(){self.fault=Some(error);}self.phase=RetainedTypedPackPhase::Closing;}

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == RetainedTypedPackPhase::Closed
            && self.page_len == 0
            && self.source.is_none()
            && self.anchor.is_none()
            && self.segment.is_none()
            && self.catalog.is_none()
            && self.value.is_none()
            && self.typed.is_none()
            && self.catalog_value.is_none()
            && self.document_byte.is_none()
            && self.fault.is_none()
    }
}

impl<O: RetainedTypedPackOwner> Drop for RetainedTypedPackSession<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained typed pack session reached Drop before exact terminal-empty close");
    }
}

#[cfg(test)]
#[path="🧭️producer/🏪️store/🧪️tests/🦀️.rs"]
mod producer_authority;
