//! 🪝️ Retained record decoding: the value and record-body cursors that admit a wire body one granted step at a time, plus source decode, receiving and the stack storage they own.

use semio_framework_value::{DslValue, Number, ValueError, ValueRefusalKind};
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldValue, RecordValue, WireNode};
use semio_framework_replication_codec::PackLimits;
use semio_framework_pack_record_schema::{ELEM_BOOL, ELEM_ENUM, ELEM_F64, ELEM_FALLBACK, ELEM_INT, ELEM_STR, ELEM_UINT, TAG_ABSENT, TAG_BLOCK, TAG_BYTES, TAG_BYTES_CHUNKED, TAG_ENUM, TAG_EXPR, TAG_F64, TAG_FALSE, TAG_INT, TAG_LIST, TAG_MAP, TAG_NULL, TAG_PACKED_F64, TAG_PACKED_VARINT, TAG_RECORD, TAG_STATEMENTS, TAG_STR, TAG_STR_INLINE, TAG_TABLE_SOA, TAG_TRUE, TAG_TUPLE, TAG_UINT, TAG_VALUE, TAG_WIRE};

#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[path = "📖️source/🦀️.rs"]
mod original_record_source;
pub use original_record_source::{RetainedRecordBodySourceDecode,RetainedRecordBodyReadLimits,RetainedRecordBodySourceStep};

#[path = "🫴️receiving/🦀️.rs"]
mod original_record_receiving;
pub use original_record_receiving::RetainedRecordReceiving;

#[path = "../🎟️storage/🦀️.rs"]
mod original_storage;

//#region 🔖️RetainedValue
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedValueRole {
    Count,
    FieldId,
    Integer,
    Unsigned,
    Enum,
    Symbol,
    Chunk,
    StringLength,
    BytesLength,
    TableRows,
    TableField,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedValueContainer {
    Record,
    Tuple,
    List,
    Statements,
    Map,
    ChunkedBytes,
    Table,
    PackedF64,
    PackedVarint,
    Wire,
}
semio_framework_value::artifact_retire_leaf!(RetainedValueContainer);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedValueToken {
    Tag { offset: u64, value: u8 },
    Begin { kind: RetainedValueContainer, count: u64 },
    Unsigned { role: RetainedValueRole, value: u64 },
    Signed(i64),
    F64(u64),
    StringChar(char),
    Byte(u8),
    WirePresence(u8),
    WireNodePresence(u8),
    WireLabelPresence(u8),
    TablePresence { rows: u64, value: u8 },
    TableBitmap { first_row: u64, value: u8 },
    End(RetainedValueContainer),
    Complete { bytes: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedContext {
    Field,
    Dsl,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RetainedVarint {
    value: u64,
    bytes: u8,
}

impl RetainedVarint {
    fn admit(&mut self, byte: u8, offset: u64) -> Result<Option<u64>, PackRefusal> {
        if self.bytes >= 10 || (self.bytes == 9 && ((byte & 0x80) != 0 || byte & 0x7f > 1)) {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-varint", offset: offset - self.bytes as u64, detail: "overlong varint" });
        }
        let payload = (byte & 0x7f) as u64;
        self.value |= payload << (self.bytes as u32 * 7);
        self.bytes += 1;
        if byte & 0x80 != 0 {
            return Ok(None);
        }
        if self.bytes > 1 && payload == 0 {
            return Err(PackRefusal::NonCanonical("non-minimal retained varint"));
        }
        Ok(Some(self.value))
    }

    fn preview(self, byte: u8, offset: u64) -> Result<Option<u64>, PackRefusal> {
        let mut cursor = self;
        cursor.admit(byte, offset)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RetainedUtf8 {
    value: u32,
    minimum: u32,
    remaining: u8,
}

impl RetainedUtf8 {
    fn admit(&mut self, byte: u8, offset: u64) -> Result<Option<char>, PackRefusal> {
        if self.remaining == 0 {
            match byte {
                0x00..=0x7f => return Ok(Some(byte as char)),
                0xc2..=0xdf => (self.value, self.minimum, self.remaining) = ((byte & 0x1f) as u32, 0x80, 1),
                0xe0..=0xef => (self.value, self.minimum, self.remaining) = ((byte & 0x0f) as u32, 0x800, 2),
                0xf0..=0xf4 => (self.value, self.minimum, self.remaining) = ((byte & 7) as u32, 0x10000, 3),
                _ => return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "invalid leading byte" }),
            }
            return Ok(None);
        }
        if byte & 0xc0 != 0x80 {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "invalid continuation" });
        }
        self.value = (self.value << 6) | (byte & 0x3f) as u32;
        self.remaining -= 1;
        if self.remaining != 0 {
            return Ok(None);
        }
        if self.value < self.minimum || (0xd800..=0xdfff).contains(&self.value) || self.value > 0x10ffff {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "invalid scalar" });
        }
        char::from_u32(self.value).map(Some).ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "invalid scalar" })
    }

    fn preview(self, byte: u8, offset: u64) -> Result<Option<char>, PackRefusal> {
        let mut cursor = self;
        cursor.admit(byte, offset)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AfterVarint {
    Record(u16),
    Field,
    Scalar(RetainedValueRole),
    Sequence(RetainedValueContainer, u16, RetainedContext),
    Statements(u16),
    Map(u16, RetainedContext),
    String,
    Bytes,
    Chunks,
    TableRows(u16),
    TableColumns(u64, u16),
    TableField(u64, u64, u16),
    Packed(RetainedValueContainer),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expect {
    Finish,
    End(RetainedValueContainer),
    Value(u16, RetainedContext),
    StringTag,
    Varint(RetainedVarint, AfterVarint),
    F64([u8; 8], usize),
    Utf8(u64, RetainedUtf8),
    Bytes(u64),
    Record(u64, u16),
    Values(u64, u16, RetainedContext),
    Varints(u64, RetainedValueRole),
    F64s(u64),
    Statements(u64, u16),
    Map(u64, u16, RetainedContext),
    Wire(u16),
    WireNode,
    WireLabel,
    TableColumns(u64, u64, u16),
    TablePresence(u64, u64, u16),
    TableBitmap(u64, u64, u64, u64, u64, u16),
    TableElem(u64, u64, u64, u16),
}

pub struct RetainedValueCursor {
    limits: PackLimits,
    intrinsic_field: bool,
    stack: std::mem::ManuallyDrop<Vec<Expect>>,
    maximum_frames: usize,
    maximum_allocation_bytes: usize,
    initialized_roots: u8,
    pending: Option<(u64, u8)>,
    offset: u64,
    sealed: bool,
    closing: bool,
    fault: Option<PackRefusal>,
    closed: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedValueAllocationStep {
    pub progressed: bool,
    pub allocated_bytes: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RetainedValueAllocationError {
    pub allocated_bytes: usize,
    pub fault: PackRefusal,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedValueProgress {
    pub frames: usize,
    pub frame_capacity: usize,
    pub allocated_bytes: usize,
    pub initialized_roots: u8,
    pub pending_input: bool,
    pub closed: bool,
}

impl RetainedValueCursor {
    /// 📏️ Quotes the original scalar frame or whole stack allocation before close.
    pub fn retirement_demands(&self) -> semio_framework_value::RetirementDemand {
        use semio_framework_value::RetirementDemand;
        if self.terminal_is_empty() { return Default::default(); }
        let copy_bytes = if self.pending.is_some() { size_of::<(u64, u8)>() } else if !self.stack.is_empty() { size_of::<Expect>() } else { 0 };
        RetirementDemand { copy_bytes, release_bytes: if copy_bytes == 0 { self.stack_allocation_bytes() } else { 0 }, depth: 1, ..Default::default() }
    }
    pub fn try_new(limits: PackLimits, maximum_allocation_bytes: usize) -> Result<Self, PackRefusal> {
        if limits.max_depth == 0{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"retained value depth credits"});}
        if limits.max_items == 0{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"retained value item credits"});}
        let maximum_frames = usize::from(limits.max_depth).checked_mul(8).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained value stack"})?;
        let requested = maximum_frames.checked_mul(size_of::<Expect>()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained value stack allocation"})?;
        if maximum_allocation_bytes == 0 || maximum_allocation_bytes > isize::MAX as usize || requested > maximum_allocation_bytes {
            return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained value stack allocation credits"});
        }
        Ok(Self {
            limits,
            intrinsic_field:false,
            stack: std::mem::ManuallyDrop::new(Vec::new()),
            maximum_frames,
            maximum_allocation_bytes,
            initialized_roots: 0,
            pending: None,
            offset: 0,
            sealed: false,
            closing: false,
            fault: None,
            closed: false,
        })
    }

    /// 🎞️ Counts intrinsic value depth from its declared Value field rather than physical framing.
    pub fn try_new_intrinsic(limits:PackLimits,maximum_allocation_bytes:usize)->Result<Self,PackRefusal>{
        let mut cursor=Self::try_new(limits,maximum_allocation_bytes)?;cursor.intrinsic_field=true;Ok(cursor)
    }

    fn stack_allocation_bytes(&self) -> usize {
        self.stack.capacity().saturating_mul(size_of::<Expect>())
    }

    pub fn allocated_bytes(&self) -> usize {
        self.stack_allocation_bytes()
    }

    pub fn next_allocation_bytes(&self) -> Result<Option<usize>, PackRefusal> {
        if let Some(fault) = self.fault.clone() {
            return Err(fault);
        }
        if self.closed || self.closing {
            return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated,what: "retained-value", offset: self.offset, detail: "allocation after close" });
        }
        if self.stack.capacity() >= self.maximum_frames {
            return Ok(None);
        }
        self.maximum_frames
            .checked_mul(size_of::<Expect>())
            .filter(|requested| *requested <= self.maximum_allocation_bytes)
            .map(Some)
            .ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained value stack allocation credits"})
    }

    pub fn reserve_allocation(&mut self, maximum_bytes: usize) -> Result<RetainedValueAllocationStep, RetainedValueAllocationError> {
        let requested = self.next_allocation_bytes().map_err(|fault| RetainedValueAllocationError { allocated_bytes: 0, fault })?;
        let Some(requested) = requested else { return Ok(RetainedValueAllocationStep::default()) };
        if maximum_bytes < requested {
            return Ok(RetainedValueAllocationStep::default());
        }
        if self.stack.try_reserve_exact(self.maximum_frames).is_err() {
            let fault = PackRefusal::RetainedAllocation{kind:ValueRefusalKind::AllocationFailed,allocated_bytes:0,what:"retained-value-stack",offset:self.offset,detail:"retained value stack allocation"};
            let first = self.fault.get_or_insert_with(|| fault.clone()).clone();
            return Err(RetainedValueAllocationError { allocated_bytes: 0, fault: first });
        }
        let allocated_bytes = self.stack_allocation_bytes();
        if allocated_bytes > maximum_bytes || allocated_bytes > self.maximum_allocation_bytes {
            let fault = PackRefusal::RetainedAllocation{kind:ValueRefusalKind::OwnershipLimit,allocated_bytes,what:"retained-value-stack",offset:self.offset,detail:"retained value stack allocation overgrant"};
            self.fault.get_or_insert_with(|| fault.clone());
            return Err(RetainedValueAllocationError { allocated_bytes, fault });
        }
        Ok(RetainedValueAllocationStep { progressed: true, allocated_bytes })
    }

    fn initialize_root(&mut self) -> Result<bool, PackRefusal> {
        if self.initialized_roots == 2 {
            return Ok(false);
        }
        if self.stack.capacity() < self.maximum_frames {
            return Ok(false);
        }
        let frame = if self.initialized_roots == 0 { Expect::Finish } else { Expect::Varint(RetainedVarint::default(), AfterVarint::Record(0)) };
        self.stack.push(frame);
        self.initialized_roots += 1;
        Ok(true)
    }

    pub fn admit_byte(&mut self, offset: u64, byte: u8) -> Result<(), (u64, u8)> {
        if self.closed || self.closing || self.fault.is_some() || self.sealed || self.initialized_roots != 2 || self.stack.is_empty() || self.pending.is_some() || offset != self.offset {
            return Err((offset, byte));
        }
        self.pending = Some((offset, byte));
        Ok(())
    }

    pub fn seal(&mut self, bytes: u64) -> Result<(), PackRefusal> {
        if self.closed || self.closing || self.fault.is_some() || self.initialized_roots != 2 || self.pending.is_some() || bytes != self.offset {
            return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated,what: "retained-value", offset: self.offset, detail: "seal position mismatch" });
        }
        self.sealed = true;
        Ok(())
    }

    fn push(&mut self, value: Expect) -> Result<(), PackRefusal> {
        if self.stack.len() == self.maximum_frames { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained value owner stack" }); }
        if self.stack.len() == self.stack.capacity() { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::InvariantViolated, limit: "retained value owner stack" }); }
        self.stack.push(value);
        Ok(())
    }

    fn count(&self, count: u64) -> Result<(), PackRefusal> {
        if count > self.limits.max_items {
            Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"retained value item count"})
        } else {
            Ok(())
        }
    }

    fn string(&mut self) -> Result<(), PackRefusal> {
        self.push(Expect::StringTag)
    }

    fn control(&mut self, value: Expect) -> Result<Option<RetainedValueToken>, PackRefusal> {
        match value {
            Expect::End(kind) => Ok(Some(RetainedValueToken::End(kind))),
            Expect::Finish if self.sealed => Ok(Some(RetainedValueToken::Complete { bytes: self.offset })),
            Expect::Finish => {
                self.push(Expect::Finish)?;
                Ok(None)
            }
            Expect::Record(0, _) | Expect::Values(0, _, _) | Expect::Varints(0, _) | Expect::F64s(0) | Expect::Statements(0, _) | Expect::Map(0, _, _) | Expect::TableColumns(0, _, _) | Expect::Utf8(0, _) | Expect::Bytes(0) => Ok(None),
            Expect::TableBitmap(0, _, rows, present, columns, depth) => {
                self.push(Expect::TableElem(present, rows, columns, depth))?;
                Ok(None)
            }
            Expect::Record(count, depth) => {
                self.push(Expect::Record(count - 1, depth))?;
                self.push(Expect::Value(depth + 1, RetainedContext::Field))?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Field))?;
                Ok(None)
            }
            Expect::Values(count, depth, context) => {
                self.push(Expect::Values(count - 1, depth, context))?;
                self.push(Expect::Value(depth + 1, context))?;
                Ok(None)
            }
            Expect::Varints(count, role) => {
                self.push(Expect::Varints(count - 1, role))?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(role)))?;
                Ok(None)
            }
            Expect::F64s(count) => {
                self.push(Expect::F64s(count - 1))?;
                self.push(Expect::F64([0; 8], 0))?;
                Ok(None)
            }
            Expect::Statements(count, depth) => {
                self.push(Expect::Statements(count - 1, depth))?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Record(depth + 1)))?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Symbol)))?;
                Ok(None)
            }
            Expect::Map(count, depth, context) => {
                self.push(Expect::Map(count - 1, depth, context))?;
                self.push(Expect::Value(depth + 1, context))?;
                self.string()?;
                Ok(None)
            }
            Expect::TableColumns(count, rows, depth) => {
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::TableField(rows, count - 1, depth)))?;
                Ok(None)
            }
            other => {
                self.push(other)?;
                Ok(None)
            }
        }
    }

    fn after(&mut self, value: u64, after: AfterVarint) -> Result<RetainedValueToken, PackRefusal> {
        match after {
            AfterVarint::Record(depth) => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::Record))?;
                self.push(Expect::Record(value, depth))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Record, count: value })
            }
            AfterVarint::Field => {
                if value > u16::MAX as u64 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "field-id", offset: self.offset, detail: "exceeds u16" });
                }
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::FieldId, value })
            }
            AfterVarint::Scalar(role) => {
                if role == RetainedValueRole::Integer {
                    Ok(RetainedValueToken::Signed(((value >> 1) as i64) ^ -((value & 1) as i64)))
                } else {
                    Ok(RetainedValueToken::Unsigned { role, value })
                }
            }
            AfterVarint::Sequence(kind, depth, context) => {
                self.count(value)?;
                self.push(Expect::End(kind))?;
                self.push(Expect::Values(value, depth, context))?;
                Ok(RetainedValueToken::Begin { kind, count: value })
            }
            AfterVarint::Statements(depth) => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::Statements))?;
                self.push(Expect::Statements(value, depth))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Statements, count: value })
            }
            AfterVarint::Map(depth, context) => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::Map))?;
                self.push(Expect::Map(value, depth, context))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Map, count: value })
            }
            AfterVarint::String => {
                if value > self.limits.max_segment_len {
                    return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained string length" });
                }
                self.push(Expect::Utf8(value, RetainedUtf8::default()))?;
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::StringLength, value })
            }
            AfterVarint::Bytes => {
                if value > self.limits.max_segment_len {
                    return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained bytes length" });
                }
                self.push(Expect::Bytes(value))?;
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::BytesLength, value })
            }
            AfterVarint::Chunks => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::ChunkedBytes))?;
                self.push(Expect::Varints(value, RetainedValueRole::Chunk))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::ChunkedBytes, count: value })
            }
            AfterVarint::TableRows(depth) => {
                self.count(value)?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::TableColumns(value, depth)))?;
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::TableRows, value })
            }
            AfterVarint::TableColumns(rows, depth) => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::Table))?;
                self.push(Expect::TableColumns(value, rows, depth))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Table, count: rows })
            }
            AfterVarint::TableField(rows, columns, depth) => {
                if value > u16::MAX as u64 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "table-field", offset: self.offset, detail: "exceeds u16" });
                }
                self.push(Expect::TablePresence(columns, rows, depth))?;
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::TableField, value })
            }
            AfterVarint::Packed(kind) => {
                self.count(value)?;
                self.push(Expect::End(kind))?;
                self.push(match kind {
                    RetainedValueContainer::PackedF64 => Expect::F64s(value),
                    RetainedValueContainer::PackedVarint => Expect::Varints(value, RetainedValueRole::Integer),
                    _ => unreachable!(),
                })?;
                Ok(RetainedValueToken::Begin { kind, count: value })
            }
        }
    }

    fn tag(&mut self, offset: u64, tag: u8, depth: u16, context: RetainedContext) -> Result<RetainedValueToken, PackRefusal> {
        if depth > self.limits.max_depth {
            return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"retained value depth"});
        }
        if context == RetainedContext::Dsl && !matches!(tag, TAG_FALSE | TAG_TRUE | TAG_INT | TAG_UINT | TAG_F64 | TAG_STR | TAG_STR_INLINE | TAG_BYTES | TAG_BYTES_CHUNKED | TAG_LIST | TAG_MAP | TAG_NULL) {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "dsl-value", offset, detail: "field-only tag" });
        }
        if context == RetainedContext::Field && tag == TAG_NULL {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "field-value", offset, detail: "DSL-only null" });
        }
        match tag {
            TAG_ABSENT | TAG_FALSE | TAG_TRUE | TAG_NULL => {}
            TAG_INT => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Integer)))?,
            TAG_UINT => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Unsigned)))?,
            TAG_F64 => self.push(Expect::F64([0; 8], 0))?,
            TAG_STR => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Symbol)))?,
            TAG_STR_INLINE => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::String))?,
            TAG_BYTES => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Bytes))?,
            TAG_BYTES_CHUNKED => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Chunks))?,
            TAG_ENUM => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Enum)))?,
            TAG_TUPLE | TAG_LIST => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Sequence(if tag == TAG_TUPLE { RetainedValueContainer::Tuple } else { RetainedValueContainer::List }, depth, context)))?,
            TAG_RECORD => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Record(depth + 1)))?,
            TAG_BLOCK => self.push(Expect::Value(depth + 1, context))?,
            TAG_STATEMENTS => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Statements(depth)))?,
            TAG_MAP => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Map(depth, context)))?,
            TAG_VALUE => self.push(Expect::Value(if self.intrinsic_field&&depth==1{0}else{depth+1}, RetainedContext::Dsl))?,
            TAG_WIRE => {
                self.push(Expect::End(RetainedValueContainer::Wire))?;
                self.push(Expect::Wire(depth))?;
            }
            TAG_TABLE_SOA => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::TableRows(depth)))?,
            TAG_PACKED_F64 | TAG_PACKED_VARINT => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Packed(if tag == TAG_PACKED_F64 { RetainedValueContainer::PackedF64 } else { RetainedValueContainer::PackedVarint })))?,
            TAG_EXPR => self.string()?,
            _ => return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-tag", offset, detail: "unknown tag" }),
        }
        if tag == TAG_WIRE {
            Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Wire, count: 1 })
        } else {
            Ok(RetainedValueToken::Tag { offset, value: tag })
        }
    }

    fn grant_inner(&mut self) -> Result<Option<RetainedValueToken>, PackRefusal> {
        let Some(expectation) = self.stack.pop() else { return Ok(None) };
        let control = matches!(
            expectation,
            Expect::Finish
                | Expect::End(..)
                | Expect::Record(..)
                | Expect::Values(..)
                | Expect::Varints(..)
                | Expect::F64s(..)
                | Expect::Statements(..)
                | Expect::Map(..)
                | Expect::TableColumns(..)
                | Expect::Utf8(0, _)
                | Expect::Bytes(0)
                | Expect::TableBitmap(0, ..)
        );
        if control {
            return self.control(expectation);
        }
        let Some((offset, byte)) = self.pending.take() else {
            self.stack.push(expectation);
            return if self.sealed { Err(PackRefusal::Truncated(self.offset)) } else { Ok(None) };
        };
        self.offset += 1;
        match expectation {
            Expect::Value(depth, context) => self.tag(offset, byte, depth, context).map(Some),
            Expect::StringTag => match byte {
                TAG_STR => {
                    self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Symbol)))?;
                    Ok(Some(RetainedValueToken::Tag { offset, value: byte }))
                }
                TAG_STR_INLINE => {
                    self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::String))?;
                    Ok(Some(RetainedValueToken::Tag { offset, value: byte }))
                }
                _ => Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-string", offset, detail: "expected string tag" }),
            },
            Expect::Varint(mut cursor, after) => match cursor.admit(byte, offset)? {
                Some(value) => self.after(value, after).map(Some),
                None => {
                    self.push(Expect::Varint(cursor, after))?;
                    Ok(None)
                }
            },
            Expect::F64(mut bytes, index) => {
                bytes[index] = byte;
                if index == 7 {
                    Ok(Some(RetainedValueToken::F64(u64::from_le_bytes(bytes))))
                } else {
                    self.push(Expect::F64(bytes, index + 1))?;
                    Ok(None)
                }
            }
            Expect::Utf8(remaining, mut cursor) => {
                let token = cursor.admit(byte, offset)?.map(RetainedValueToken::StringChar);
                if remaining > 1 {
                    self.push(Expect::Utf8(remaining - 1, cursor))?;
                } else if cursor.remaining != 0 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "truncated scalar" });
                }
                Ok(token)
            }
            Expect::Bytes(remaining) => {
                if remaining > 1 {
                    self.push(Expect::Bytes(remaining - 1))?;
                }
                Ok(Some(RetainedValueToken::Byte(byte)))
            }
            Expect::Wire(depth) => {
                if byte & !7 != 0 || (byte & 2 != 0 && byte & 1 == 0) {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "wire-presence", offset, detail: "invalid bits" });
                }
                self.push(Expect::Value(depth + 1, RetainedContext::Dsl))?;
                if byte & 4 != 0 {
                    self.push(Expect::WireLabel)?;
                }
                if byte & 1 != 0 {
                    self.push(Expect::WireNode)?;
                }
                self.push(Expect::WireNode)?;
                Ok(Some(RetainedValueToken::WirePresence(byte)))
            }
            Expect::WireNode => {
                if byte & !3 != 0 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "wire-node", offset, detail: "invalid bits" });
                }
                if byte & 2 != 0 {
                    self.string()?;
                }
                if byte & 1 != 0 {
                    self.string()?;
                }
                self.string()?;
                Ok(Some(RetainedValueToken::WireNodePresence(byte)))
            }
            Expect::WireLabel => {
                if byte & !3 != 0 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "wire-label", offset, detail: "invalid bits" });
                }
                if byte & 2 != 0 {
                    self.string()?;
                }
                if byte & 1 != 0 {
                    self.string()?;
                }
                Ok(Some(RetainedValueToken::WireLabelPresence(byte)))
            }
            Expect::TablePresence(columns, rows, depth) => {
                match byte {
                    0 => self.push(Expect::TableElem(rows, rows, columns, depth))?,
                    1 => self.push(Expect::TableBitmap(rows.div_ceil(8), 0, rows, 0, columns, depth))?,
                    _ => return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "table-presence", offset, detail: "expected 0 or 1" }),
                }
                Ok(Some(RetainedValueToken::TablePresence { rows, value: byte }))
            }
            Expect::TableBitmap(bytes, row, rows, mut present, columns, depth) => {
                let used = (rows - row).min(8) as u8;
                let mask = if used == 8 { u8::MAX } else { ((1u16 << used) - 1) as u8 };
                if byte & !mask != 0 {
                    return Err(PackRefusal::NonCanonical("table bitmap padding bits"));
                }
                present += (byte & mask).count_ones() as u64;
                if bytes > 1 {
                    self.push(Expect::TableBitmap(bytes - 1, row + 8, rows, present, columns, depth))?;
                } else {
                    self.push(Expect::TableElem(present, rows, columns, depth))?;
                }
                Ok(Some(RetainedValueToken::TableBitmap { first_row: row, value: byte }))
            }
            Expect::TableElem(present, rows, columns, depth) => {
                self.push(Expect::TableColumns(columns, rows, depth))?;
                self.push(match byte {
                    ELEM_FALLBACK => Expect::Values(present, depth, RetainedContext::Field),
                    ELEM_BOOL => Expect::Bytes(rows.div_ceil(8)),
                    ELEM_INT => Expect::Varints(present, RetainedValueRole::Integer),
                    ELEM_UINT => Expect::Varints(present, RetainedValueRole::Unsigned),
                    ELEM_F64 => Expect::F64s(present),
                    ELEM_STR => Expect::Varints(present, RetainedValueRole::Symbol),
                    ELEM_ENUM => Expect::Varints(present, RetainedValueRole::Enum),
                    _ => return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "table-element", offset, detail: "unknown tag" }),
                })?;
                Ok(Some(RetainedValueToken::Tag { offset, value: byte }))
            }
            _ => unreachable!(),
        }
    }

    pub fn grant(&mut self) -> Result<Option<RetainedValueToken>, PackRefusal> {
        if let Some(fault) = self.fault.clone() {
            return Err(fault);
        }
        if self.closed || self.closing {
            return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated,what: "retained-value", offset: self.offset, detail: "grant after close" });
        }
        match self.initialize_root() {
            Ok(true) => return Ok(None),
            Ok(false) => {}
            Err(fault) => {
                self.fault.get_or_insert_with(|| fault.clone());
                return Err(self.fault.clone().expect("retained value first fault"));
            }
        }
        if self.initialized_roots != 2 {
            return Ok(None);
        }
        match self.grant_inner() {
            Ok(value) => Ok(value),
            Err(fault) => {
                self.fault.get_or_insert_with(|| fault.clone());
                Err(self.fault.clone().expect("retained value first fault"))
            }
        }
    }

    pub fn next_release_allocation_bytes(&self) -> Option<usize> {
        if self.pending.is_some() || !self.stack.is_empty() {
            return None;
        }
        let allocated = self.stack_allocation_bytes();
        (allocated != 0).then_some(allocated)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_pack_format_retained::RetainedPackCloseStep, PackRefusal> {
        if !self.closed && maximum_items == 0 && maximum_bytes == 0 {
            return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if maximum_items == 0 && (self.pending.is_some() || !self.stack.is_empty()) {
            return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.closing = true;
        if maximum_items != 0 {
            if self.pending.take().is_some() {
                return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.stack.pop().is_some() {
                if self.stack.is_empty() {
                    self.initialized_roots = 0;
                }
                return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        if self.pending.is_some() || !self.stack.is_empty() {
            return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.initialized_roots = 0;
        let allocated_bytes = self.stack_allocation_bytes();
        if allocated_bytes != 0 {
            if maximum_bytes < allocated_bytes {
                return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            *self.stack = Vec::new();
            return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: allocated_bytes });
        }
        self.closed = true;
        Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Complete)
    }

    /// 🚦️ Whether [`Self::admit_byte`] would accept another byte right now. One admitted byte can
    /// take SEVERAL [`Self::grant`] turns to be consumed — every control frame on the expectation
    /// stack (`Expect::Record`, `Expect::Map`, `Expect::End`, …) is popped without touching the
    /// pending slot — so a producer that admits on a fixed cadence rather than on this predicate
    /// hands back a byte the cursor is still holding.
    pub fn ingress_ready(&self) -> bool {
        !self.closed && !self.closing && self.fault.is_none() && !self.sealed && self.initialized_roots == 2 && !self.stack.is_empty() && self.pending.is_none()
    }

    /// 📏️ Reports validated bytes independently of admitted pending ingress.
    pub fn consumed_bytes(&self)->u64{self.offset}

    pub fn progress(&self) -> RetainedValueProgress {
        RetainedValueProgress {
            frames: self.stack.len(),
            frame_capacity: self.stack.capacity(),
            allocated_bytes: self.stack_allocation_bytes(),
            initialized_roots: self.initialized_roots,
            pending_input: self.pending.is_some(),
            closed: self.closed,
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.stack.is_empty() && self.stack.capacity() == 0 && self.stack_allocation_bytes() == 0 && self.initialized_roots == 0 && self.pending.is_none()
    }
}

impl Drop for RetainedValueCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained value cursor reached Drop before terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedRecordBodyToken {
    Catalog { symbols: u64 },
    SymbolChar { symbol: u64, character: char },
    CatalogComplete,
    Value(RetainedValueToken),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedRecordBodyPhase {
    SymbolCount(RetainedVarint),
    SymbolLength { symbol: u64, cursor: RetainedVarint },
    SymbolText { symbol: u64, scalar_start: usize, utf8_len: usize, remaining: u64, cursor: RetainedUtf8 },
    Value,
    Closing,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedRecordBodyClosePhase {
    ValueLogical,
    SymbolsLogical,
    ValuePhysical,
    SymbolsPhysical,
}

/// 🧵️ Retained cursor for the canonical container-less record-body wire used by mutations.
/// It owns a fixed symbol registry and forwards the record value without constructing a
/// schema-erased record.
pub struct RetainedRecordBodyCursor {
    limits: PackLimits,
    intrinsic_field: bool,
    phase: RetainedRecordBodyPhase,
    symbols: std::mem::ManuallyDrop<semio_framework_pack_format_catalog::RetainedPackSymbolTable>,
    maximum_allocation_bytes: usize,
    expected_symbols: u64,
    symbol_utf8_bytes: usize,
    offset: u64,
    value_offset: u64,
    pending: Option<(u64, u8)>,
    catalog_complete_pending: bool,
    sealed: bool,
    value_sealed: bool,
    value: std::mem::ManuallyDrop<Option<RetainedValueCursor>>,
    fault: Option<PackRefusal>,
    closing: bool,
    close_phase: RetainedRecordBodyClosePhase,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedRecordBodyAllocationStep {
    pub progressed: bool,
    pub allocated_bytes: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RetainedRecordBodyAllocationError {
    pub allocated_bytes: usize,
    pub fault: PackRefusal,
}

#[derive(Clone, Copy)]
enum RetainedRecordBodyAllocationOwner {
    SymbolSpans { target: usize, offset: u64 },
    SymbolScalars { target: usize, offset: u64 },
    Value,
}

impl RetainedRecordBodyCursor {
    pub fn try_new(
        limits: PackLimits,
        maximum_symbols: usize,
        maximum_symbol_utf8_bytes: usize,
        maximum_symbol_scalars: usize,
        maximum_allocation_bytes: usize,
    ) -> Result<Self, PackRefusal> {
        if limits.max_symbols == 0 || limits.max_items == 0 || limits.max_depth == 0 {
            return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body credits" });
        }
        if maximum_symbols > limits.max_symbols as usize || maximum_symbol_utf8_bytes as u64 > limits.max_file_len { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body physical credits" }); }
        if maximum_symbol_scalars > maximum_symbol_utf8_bytes { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::InvalidValue, limit: "retained record-body physical credits" }); }
        if maximum_allocation_bytes == 0 || maximum_allocation_bytes > isize::MAX as usize { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body physical credits" }); }
        Ok(Self {
            limits,
            intrinsic_field:false,
            phase: RetainedRecordBodyPhase::SymbolCount(RetainedVarint::default()),
            symbols: std::mem::ManuallyDrop::new(
                semio_framework_pack_format_catalog::RetainedPackSymbolTable::try_new(maximum_symbols, maximum_symbol_utf8_bytes, maximum_symbol_scalars, maximum_allocation_bytes)
                    .map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?,
            ),
            maximum_allocation_bytes,
            expected_symbols: 0,
            symbol_utf8_bytes: 0,
            offset: 0,
            value_offset: 0,
            pending: None,
            catalog_complete_pending: false,
            sealed: false,
            value_sealed: false,
            value: std::mem::ManuallyDrop::new(None),
            fault: None,
            closing: false,
            close_phase: RetainedRecordBodyClosePhase::ValueLogical,
        })
    }

    /// 🎞️ Validates intrinsic logical depth independently from the single-field Body envelope.
    pub fn try_new_intrinsic(limits:PackLimits,maximum_symbols:usize,maximum_symbol_utf8_bytes:usize,maximum_symbol_scalars:usize,maximum_allocation_bytes:usize)->Result<Self,PackRefusal>{
        let mut cursor=Self::try_new(limits,maximum_symbols,maximum_symbol_utf8_bytes,maximum_symbol_scalars,maximum_allocation_bytes)?;cursor.intrinsic_field=true;Ok(cursor)
    }

    pub fn admit_byte(&mut self, offset: u64, value: u8) -> Result<(), (u64, u8)> {
        if self.sealed
            || self.closing
            || self.fault.is_some()
            || self.pending.is_some()
            || self.catalog_complete_pending
            || offset != self.offset
            || matches!(self.phase, RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed)
        {
            return Err((offset, value));
        }
        self.pending = Some((offset, value));
        Ok(())
    }

    pub fn seal(&mut self, bytes: u64) -> Result<(), PackRefusal> {
        if self.closing || self.fault.is_some() || self.pending.is_some() || bytes != self.offset {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "seal position mismatch" });
        }
        self.sealed = true;
        Ok(())
    }

    fn allocated_bytes_checked(&self) -> Option<usize> {
        self.symbols.allocated_bytes().checked_add(self.value.as_ref().map_or(0, RetainedValueCursor::allocated_bytes))
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated_bytes_checked().unwrap_or(usize::MAX)
    }

    fn allocation_need(&self) -> Result<Option<(RetainedRecordBodyAllocationOwner, usize)>, PackRefusal> {
        if let Some(fault) = self.fault.clone() {
            return Err(fault);
        }
        if self.closing || matches!(self.phase, RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed) {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "allocation after close" });
        }
        if self.phase == RetainedRecordBodyPhase::Value {
            let value = self.value.as_ref().ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "value owner missing" })?;
            return value.next_allocation_bytes().map(|need| need.map(|bytes| (RetainedRecordBodyAllocationOwner::Value, bytes)));
        }
        let Some((offset, byte)) = self.pending else { return Ok(None) };
        let need = match self.phase {
            RetainedRecordBodyPhase::SymbolCount(cursor) => match cursor.preview(byte, offset)? {
                None => None,
                Some(count) => {
                    let target = usize::try_from(count)
                        .ok()
                        .filter(|target| *target <= self.symbols.maximum_symbols())
                        .ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body symbol count" })?;
                    {let bytes=self.symbols.source_reserve_demands(true,target,offset).map_err(|fault|fault.into_pack_refusal("retained-record-body-symbol-allocation"))?.capacity_bytes;(bytes!=0).then_some((RetainedRecordBodyAllocationOwner::SymbolSpans{target,offset},bytes))}
                }
            },
            RetainedRecordBodyPhase::SymbolLength { cursor, .. } => {
                if let Some(length) = cursor.preview(byte, offset)? {
                    let length = usize::try_from(length).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body symbol bytes" })?;
                    self.symbol_utf8_bytes
                        .checked_add(length)
                        .filter(|total| *total <= self.symbols.maximum_utf8_bytes())
                        .ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body cumulative symbol bytes" })?;
                }
                None
            }
            RetainedRecordBodyPhase::SymbolText { cursor, .. } => match cursor.preview(byte, offset)? {
                None => None,
                Some(_) => {
                    let target = self.symbols.scalar_len().checked_add(1).filter(|target| *target <= self.symbols.maximum_scalars()).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body symbol scalars" })?;
                    {let bytes=self.symbols.source_reserve_demands(false,target,offset).map_err(|fault|fault.into_pack_refusal("retained-record-body-symbol-allocation"))?.capacity_bytes;(bytes!=0).then_some((RetainedRecordBodyAllocationOwner::SymbolScalars{target,offset},bytes))}
                }
            },
            RetainedRecordBodyPhase::Value | RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed => None,
        };
        Ok(need)
    }

    pub fn next_allocation_bytes(&mut self) -> Result<Option<usize>, PackRefusal> {
        let need = match self.allocation_need() {
            Ok(need) => need,
            Err(fault) => {
                self.fault.get_or_insert_with(|| fault.clone());
                return Err(self.fault.clone().expect("record body first fault"));
            }
        };
        let Some((_, requested)) = need else { return Ok(None) };
        let result = self
            .allocated_bytes_checked()
            .and_then(|allocated| allocated.checked_add(requested))
            .filter(|total| *total <= self.maximum_allocation_bytes)
            .map(|_| Some(requested))
            .ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body allocation credits" });
        if let Err(fault) = &result {
            self.fault.get_or_insert_with(|| fault.clone());
        }
        result
    }

    /// 🔍️ Quotes the original next physical allocation or scalar write without moving pending source.
    pub fn source_step_demands(&self)->Result<semio_framework_value::RetirementDemand,PackRefusal>{
        use semio_framework_value::RetirementDemand;
        let mut demand=if let Some((owner,bytes))=self.allocation_need()?{match owner{
            RetainedRecordBodyAllocationOwner::SymbolSpans{target,offset}=>self.symbols.source_reserve_demands(true,target,offset).map_err(|error|error.into_pack_refusal("retained-record-body-source"))?,
            RetainedRecordBodyAllocationOwner::SymbolScalars{target,offset}=>self.symbols.source_reserve_demands(false,target,offset).map_err(|error|error.into_pack_refusal("retained-record-body-source"))?,
            RetainedRecordBodyAllocationOwner::Value=>RetirementDemand{capacity_bytes:bytes,depth:1,..Default::default()},
        }}else{match(self.phase,self.pending){
            (RetainedRecordBodyPhase::SymbolText{remaining,cursor,..},Some((offset,byte)))if cursor.preview(byte,offset)?.is_some()=>{let mut next=self.symbols.source_write_demands(false)?;if remaining==1{next.depth=next.depth.max(self.symbols.source_write_demands(true)?.depth)}next},
            (RetainedRecordBodyPhase::SymbolLength{cursor,..},Some((offset,byte)))if cursor.preview(byte,offset)?==Some(0)=>self.symbols.source_write_demands(true)?,
            _=>RetirementDemand{depth:1,..Default::default()},
        }};
        demand.depth=demand.depth.checked_add(1).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"record-body source physical depth"})?;Ok(demand)
    }
    /// 🧹️ Scalar symbol and native index removal has no payload copy; actual backing release stays distinct.
    pub fn source_close_demands(&self)->Result<semio_framework_value::RetirementDemand,PackRefusal>{
        use semio_framework_value::RetirementDemand;
        if self.terminal_is_empty(){return Ok(Default::default())}
        let mut demand=if self.pending.is_some()||self.catalog_complete_pending{RetirementDemand{depth:1,..Default::default()}}
        else{match self.close_phase{
            RetainedRecordBodyClosePhase::ValueLogical=>RetirementDemand{depth:if self.value.is_some(){2}else{1},..Default::default()},
            RetainedRecordBodyClosePhase::SymbolsLogical=>{let mut next=self.symbols.retirement_demands()?;next.copy_bytes=0;next.release_bytes=0;next},
            RetainedRecordBodyClosePhase::SymbolsPhysical=>{let mut next=self.symbols.retirement_demands()?;next.copy_bytes=0;next},
            RetainedRecordBodyClosePhase::ValuePhysical=>RetirementDemand{release_bytes:self.value.as_ref().and_then(RetainedValueCursor::next_release_allocation_bytes).unwrap_or(0),depth:2,..Default::default()},
        }};
        demand.depth=demand.depth.checked_add(1).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"record-body source physical depth"})?;Ok(demand)
    }

    pub fn reserve_allocation(&mut self, maximum_bytes: usize) -> Result<RetainedRecordBodyAllocationStep, RetainedRecordBodyAllocationError> {
        let need = match self.allocation_need() {
            Ok(need) => need,
            Err(fault) => {
                let first = self.fault.get_or_insert_with(|| fault.clone()).clone();
                return Err(RetainedRecordBodyAllocationError { allocated_bytes: 0, fault: first });
            }
        };
        let Some((owner, requested)) = need else { return Ok(RetainedRecordBodyAllocationStep::default()) };
        if maximum_bytes < requested {
            return Ok(RetainedRecordBodyAllocationStep::default());
        }
        let remaining = self.maximum_allocation_bytes.saturating_sub(self.allocated_bytes());
        let step = match owner {
            RetainedRecordBodyAllocationOwner::SymbolSpans { target, offset } => self
                .symbols
                .reserve_symbol_capacity(target, maximum_bytes.min(remaining), offset)
                .map(|step| RetainedRecordBodyAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
                .map_err(|error| RetainedRecordBodyAllocationError {
                    allocated_bytes: error.allocated_bytes,
                    fault: error.fault.into_pack_refusal("retained-record-body-symbol-allocation"),
                }),
            RetainedRecordBodyAllocationOwner::SymbolScalars { target, offset } => self
                .symbols
                .reserve_scalar_capacity(target, maximum_bytes.min(remaining), offset)
                .map(|step| RetainedRecordBodyAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
                .map_err(|error| RetainedRecordBodyAllocationError {
                    allocated_bytes: error.allocated_bytes,
                    fault: error.fault.into_pack_refusal("retained-record-body-symbol-allocation"),
                }),
            RetainedRecordBodyAllocationOwner::Value => self
                .value
                .as_mut()
                .ok_or_else(|| RetainedRecordBodyAllocationError {
                    allocated_bytes: 0,
                    fault: PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "value owner missing" },
                })?
                .reserve_allocation(maximum_bytes.min(remaining))
                .map(|step| RetainedRecordBodyAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
                .map_err(|error| RetainedRecordBodyAllocationError { allocated_bytes: error.allocated_bytes, fault: error.fault }),
        };
        match step {
            Ok(step) if self.allocated_bytes() <= self.maximum_allocation_bytes => Ok(step),
            Ok(step) => {
                let fault = PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body allocation overgrant" };
                self.fault.get_or_insert_with(|| fault.clone());
                Err(RetainedRecordBodyAllocationError { allocated_bytes: step.allocated_bytes, fault })
            }
            Err(error) => {
                let first = self.fault.get_or_insert_with(|| error.fault.clone()).clone();
                Err(RetainedRecordBodyAllocationError { allocated_bytes: error.allocated_bytes, fault: first })
            }
        }
    }

    pub fn symbol_chars(&self, symbol: u64) -> Result<usize, PackRefusal> {
        self.symbols.symbol_chars(symbol).map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))
    }

    /// 🧮️ Borrows the validated exact UTF-8 size before any symbol copy is allocated.
    pub fn symbol_utf8_bytes(&self, symbol:u64)->Result<usize,PackRefusal>{
        let span=self.symbols.symbol_span(symbol).map_err(|fault|fault.into_pack_refusal("retained-record-body-symbol"))?;
        usize::try_from(span.utf8_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained symbol UTF-8 size"})
    }

    /// 📍️ Counts consumed wire bytes without counting any still-pending ingress byte.
    pub fn consumed_bytes(&self)->u64{
        self.offset-self.value_offset+self.value.as_ref().map_or(0,|value|value.offset)
    }

    pub fn symbol_char(&self, symbol: u64, index: usize) -> Result<Option<char>, PackRefusal> {
        self.symbols.symbol_char(symbol, index).map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))
    }

    fn begin_value(&mut self) -> Result<(), PackRefusal> {
        *self.value = Some(if self.intrinsic_field{RetainedValueCursor::try_new_intrinsic(self.limits.clone(),self.maximum_allocation_bytes)?}else{RetainedValueCursor::try_new(self.limits.clone(), self.maximum_allocation_bytes)?});
        self.phase = RetainedRecordBodyPhase::Value;
        Ok(())
    }

    fn grant_inner(&mut self) -> Result<Option<RetainedRecordBodyToken>, PackRefusal> {
        if matches!(self.phase, RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed) {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "grant after close" });
        }
        if self.catalog_complete_pending {
            self.catalog_complete_pending = false;
            return Ok(Some(RetainedRecordBodyToken::CatalogComplete));
        }
        if self.phase == RetainedRecordBodyPhase::Value {
            if let Some((offset, byte)) = self.pending.take() {
                let value = self.value.as_mut().ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset, detail: "value owner missing" })?;
                if !value.ingress_ready() {
                    self.pending = Some((offset, byte));
                    return Ok(None);
                }
                value.admit_byte(self.value_offset, byte).map_err(|_| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated,
                    what: "retained-record-body",
                    offset,
                    detail: "value producer handback",
                })?;
                self.offset += 1;
                self.value_offset += 1;
            } else if self.sealed && !self.value_sealed {
                self.value.as_mut().ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "value owner missing" })?.seal(self.value_offset)?;
                self.value_sealed = true;
            }
            return self.value.as_mut().ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "value owner missing" })?.grant().map(|token| token.map(RetainedRecordBodyToken::Value));
        }
        let Some((offset, byte)) = self.pending.take() else {
            return if self.sealed { Err(PackRefusal::Truncated(self.offset)) } else { Ok(None) };
        };
        self.offset += 1;
        match self.phase {
            RetainedRecordBodyPhase::SymbolCount(mut cursor) => match cursor.admit(byte, offset)? {
                None => self.phase = RetainedRecordBodyPhase::SymbolCount(cursor),
                Some(symbols) => {
                    if symbols > self.symbols.maximum_symbols() as u64 {
                        return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body symbol count" });
                    }
                    self.expected_symbols = symbols;
                    if symbols == 0 {
                        self.begin_value()?;
                        return Ok(Some(RetainedRecordBodyToken::CatalogComplete));
                    }
                    self.phase = RetainedRecordBodyPhase::SymbolLength { symbol: 0, cursor: RetainedVarint::default() };
                    return Ok(Some(RetainedRecordBodyToken::Catalog { symbols }));
                }
            },
            RetainedRecordBodyPhase::SymbolLength { symbol, mut cursor } => match cursor.admit(byte, offset)? {
                None => self.phase = RetainedRecordBodyPhase::SymbolLength { symbol, cursor },
                Some(length) => {
                    if length > self.limits.max_segment_len { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body symbol length" }); }
                    if length > usize::MAX as u64 { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body symbol length" }); }
                    let length = length as usize;
                    self.symbol_utf8_bytes = self
                        .symbol_utf8_bytes
                        .checked_add(length)
                        .filter(|total| *total <= self.symbols.maximum_utf8_bytes())
                        .ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body cumulative symbol bytes" })?;
                    if length == 0 {
                        let scalar_start = self.symbols.scalar_len() as u64;
                        self.symbols
                            .push_symbol_reserved(semio_framework_pack_format_catalog::RetainedPackSymbolSpan { scalar_start, scalar_len: 0, utf8_len: 0 }, offset)
                            .map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?;
                        if symbol + 1 == self.expected_symbols {
                            self.begin_value()?;
                            return Ok(Some(RetainedRecordBodyToken::CatalogComplete));
                        }
                        self.phase = RetainedRecordBodyPhase::SymbolLength { symbol: symbol + 1, cursor: RetainedVarint::default() };
                    } else {
                        self.phase = RetainedRecordBodyPhase::SymbolText {
                            symbol,
                            scalar_start: self.symbols.scalar_len(),
                            utf8_len: length,
                            remaining: length as u64,
                            cursor: RetainedUtf8::default(),
                        };
                    }
                }
            },
            RetainedRecordBodyPhase::SymbolText { symbol, scalar_start, utf8_len, remaining, mut cursor } => {
                let character = cursor.admit(byte, offset)?;
                let next = remaining - 1;
                if let Some(character) = character {
                    self.symbols
                        .push_scalar_reserved(character, offset)
                        .map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?;
                }
                if next == 0 {
                    if cursor.remaining != 0 {
                        return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-record-body-symbol", offset, detail: "truncated UTF-8 scalar" });
                    }
                    let scalar_len = self
                        .symbols
                        .scalar_len()
                        .checked_sub(scalar_start)
                        .ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body-symbol", offset, detail: "scalar span underflow" })?;
                    self.symbols
                        .push_symbol_reserved(
                            semio_framework_pack_format_catalog::RetainedPackSymbolSpan { scalar_start: scalar_start as u64, scalar_len: scalar_len as u64, utf8_len: utf8_len as u64 },
                            offset,
                        )
                        .map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?;
                    if symbol + 1 == self.expected_symbols {
                        self.begin_value()?;
                        self.catalog_complete_pending = character.is_some();
                    } else {
                        self.phase = RetainedRecordBodyPhase::SymbolLength { symbol: symbol + 1, cursor: RetainedVarint::default() };
                    }
                } else {
                    self.phase = RetainedRecordBodyPhase::SymbolText { symbol, scalar_start, utf8_len, remaining: next, cursor };
                }
                if let Some(character) = character {
                    return Ok(Some(RetainedRecordBodyToken::SymbolChar { symbol, character }));
                }
                if self.phase == RetainedRecordBodyPhase::Value {
                    return Ok(Some(RetainedRecordBodyToken::CatalogComplete));
                }
            }
            RetainedRecordBodyPhase::Value | RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed => unreachable!(),
        }
        Ok(None)
    }

    pub fn grant(&mut self) -> Result<Option<RetainedRecordBodyToken>, PackRefusal> {
        if let Some(fault) = self.fault.clone() {
            return Err(fault);
        }
        match self.next_allocation_bytes() {
            Ok(Some(_)) => return Ok(None),
            Ok(None) => {}
            Err(fault) => return Err(fault),
        }
        match self.grant_inner() {
            Ok(value) => Ok(value),
            Err(fault) => {
                self.fault.get_or_insert_with(|| fault.clone());
                Err(self.fault.clone().expect("record body first fault"))
            }
        }
    }

    pub fn next_release_allocation_bytes(&self) -> Result<Option<usize>, PackRefusal> {
        if self.pending.is_some() || !self.closing {
            return Ok(None);
        }
        match self.close_phase {
            RetainedRecordBodyClosePhase::ValuePhysical => {
                Ok(self.value.as_ref().and_then(RetainedValueCursor::next_release_allocation_bytes))
            }
            RetainedRecordBodyClosePhase::SymbolsPhysical => self.symbols.next_release_allocation_bytes().map_err(|error|PackRefusal::from_paged_refusal(error,"retained-record-body-symbol-release",self.offset)),
            RetainedRecordBodyClosePhase::ValueLogical | RetainedRecordBodyClosePhase::SymbolsLogical => Ok(None),
        }
    }

    pub fn close_step(
        &mut self,
        maximum_items: usize,
        maximum_bytes: usize,
    ) -> Result<semio_framework_pack_format_retained::RetainedPackCloseStep, PackRefusal> {
        use semio_framework_pack_format_retained::RetainedPackCloseStep;
        if self.phase == RetainedRecordBodyPhase::Closed {
            return Ok(RetainedPackCloseStep::Complete);
        }
        if maximum_items == 0 && maximum_bytes == 0 {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.closing = true;
        if maximum_items != 0 && self.pending.take().is_some() {
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if maximum_items != 0 && self.catalog_complete_pending {
            self.catalog_complete_pending = false;
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        loop {
            match self.close_phase {
                RetainedRecordBodyClosePhase::ValueLogical => {
                    if let Some(value) = self.value.as_mut() {
                        match value.close_step(maximum_items, 0)? {
                            RetainedPackCloseStep::Pending { released_items, released_bytes } if released_items != 0 || released_bytes != 0 => {
                                return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                            }
                            RetainedPackCloseStep::Pending { .. } if value.next_release_allocation_bytes().is_none() && !value.terminal_is_empty() => {
                                return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                            }
                            RetainedPackCloseStep::Pending { .. } | RetainedPackCloseStep::Complete => {}
                        }
                    }
                    if maximum_items == 0 {
                        return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    self.phase = RetainedRecordBodyPhase::Closing;
                    self.expected_symbols = 0;
                    self.symbol_utf8_bytes = 0;
                    self.close_phase = RetainedRecordBodyClosePhase::SymbolsLogical;
                    return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
                }
                RetainedRecordBodyClosePhase::SymbolsLogical => match self.symbols.close_step(maximum_items, 0).map_err(|error|PackRefusal::from_paged_refusal(error,"retained-record-body-symbol-release",self.offset))? {
                    RetainedPackCloseStep::Pending { released_items, released_bytes } if released_items != 0 || released_bytes != 0 => {
                        return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                    }
                    RetainedPackCloseStep::Pending { .. } if self.symbols.next_release_allocation_bytes().map_err(|error|PackRefusal::from_paged_refusal(error,"retained-record-body-symbol-release",self.offset))?.is_none() && !self.symbols.terminal_is_empty() => {
                        return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    RetainedPackCloseStep::Pending { .. } | RetainedPackCloseStep::Complete => {
                        if maximum_items == 0 {
                            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                        }
                        self.close_phase = RetainedRecordBodyClosePhase::ValuePhysical;
                        return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                },
                RetainedRecordBodyClosePhase::ValuePhysical => {
                    if let Some(value) = self.value.as_mut() {
                        let owner_items = if value.next_release_allocation_bytes().is_none() { maximum_items } else { 0 };
                        match value.close_step(owner_items, maximum_bytes)? {
                            RetainedPackCloseStep::Pending { released_items, released_bytes } => {
                                return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                            }
                            RetainedPackCloseStep::Complete => {
                                if maximum_items == 0 {
                                    return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                                }
                                drop(self.value.take());
                                self.close_phase = RetainedRecordBodyClosePhase::SymbolsPhysical;
                                return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
                            }
                        }
                    }
                    self.close_phase = RetainedRecordBodyClosePhase::SymbolsPhysical;
                }
                RetainedRecordBodyClosePhase::SymbolsPhysical => match self.symbols.close_step(maximum_items, maximum_bytes).map_err(|error|PackRefusal::from_paged_refusal(error,"retained-record-body-symbol-release",self.offset))? {
                    RetainedPackCloseStep::Pending { released_items, released_bytes } => {
                        return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                    }
                    RetainedPackCloseStep::Complete => {
                        self.phase = RetainedRecordBodyPhase::Closed;
                        return Ok(RetainedPackCloseStep::Complete);
                    }
                },
            }
        }
    }

    /// 🚦️ Whether [`Self::admit_byte`] would accept another byte AND be able to forward it on the
    /// next [`Self::grant`]. Once the value producer is live every admitted byte is handed straight
    /// to it, so this cursor's own empty ingress slot is not sufficient: a producer still chewing on
    /// the previous byte rejects the handoff as `retained-record-body/value producer handback`.
    pub fn ingress_ready(&self) -> bool {
        !self.sealed
            && !self.closing
            && self.fault.is_none()
            && self.pending.is_none()
            && !self.catalog_complete_pending
            && !matches!(self.phase, RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed)
            && self.value.as_ref().is_none_or(RetainedValueCursor::ingress_ready)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == RetainedRecordBodyPhase::Closed
            && self.value.is_none()
            && self.symbols.terminal_is_empty()
            && self.symbols.allocated_bytes() == 0
            && self.pending.is_none()
            && !self.catalog_complete_pending
    }
}

impl Drop for RetainedRecordBodyCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained record-body cursor reached Drop before terminal-empty close");
    }
}
//#endregion 🔖️RetainedValue

#[cfg(test)]
#[path = "../🎟️storage/🧪️tests/🦀️.rs"]
mod original_storage_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
