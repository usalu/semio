import sys
src,dst=sys.argv[1:3]
s=open(src).read()
R=[
('use crate::os_pack::{write_varint_i64, write_varint_u64, ByteReader, ChunkId, CodecId, PackError, PackLimits};\nuse std::collections::{HashMap, HashSet};\nuse protocol::value::ValueError;',
 'use crate::os_pack::{write_varint_i64, write_varint_u64, ByteReader, ChunkId, CodecId, PackError, PackLimits, PackRefusal};\nuse std::collections::{HashMap, HashSet};\nuse protocol::value::{ValueError,ValueRefusalKind};',1),
('PackError::Schema(format!("symbol {s:?} missing from precomputed table"))',
 'PackError::from(ValueError::new(ValueRefusalKind::InvariantViolated,format!("symbol {s:?} missing from precomputed table")))',1),
('        if limits.max_depth == 0 || limits.max_items == 0 {\n            return Err(PackError::LimitExceeded("retained value credits"));\n        }\n',
 '        if limits.max_depth == 0{return Err(PackError::Refusal(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"retained value depth credits"}));}\n        if limits.max_items == 0{return Err(PackError::Refusal(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"retained value item credits"}));}\n',1),
('let fault = PackError::LimitExceeded("retained value stack allocation");',
 'let fault = PackError::Refusal(PackRefusal::RetainedAllocation{kind:ValueRefusalKind::AllocationFailed,allocated_bytes:0,what:"retained-value-stack",offset:self.offset,detail:"retained value stack allocation"});',1),
('let fault = PackError::LimitExceeded("retained value stack allocation overgrant");',
 'let fault = PackError::Refusal(PackRefusal::RetainedAllocation{kind:ValueRefusalKind::OwnershipLimit,allocated_bytes,what:"retained-value-stack",offset:self.offset,detail:"retained value stack allocation overgrant"});',1),
('        if self.stack.len() == self.stack.capacity() || self.stack.len() == self.maximum_frames {\n            return Err(PackError::LimitExceeded("retained value owner stack"));\n        }\n',
 '        if self.stack.len() == self.maximum_frames { return Err(PackError::Refusal(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained value owner stack" })); }\n        if self.stack.len() == self.stack.capacity() { return Err(PackError::Refusal(PackRefusal::LimitExceeded { kind: ValueRefusalKind::InvariantViolated, limit: "retained value owner stack" })); }\n',1),
('        if maximum_symbols > limits.max_symbols as usize\n            || maximum_symbol_utf8_bytes as u64 > limits.max_file_len\n            || maximum_symbol_scalars > maximum_symbol_utf8_bytes\n            || maximum_allocation_bytes == 0\n            || maximum_allocation_bytes > isize::MAX as usize\n        {\n            return Err(PackError::LimitExceeded("retained record-body physical credits"));\n        }\n',
 '        if maximum_symbols > limits.max_symbols as usize || maximum_symbol_utf8_bytes as u64 > limits.max_file_len { return Err(PackError::Refusal(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body physical credits" })); }\n        if maximum_symbol_scalars > maximum_symbol_utf8_bytes { return Err(PackError::Refusal(PackRefusal::LimitExceeded { kind: ValueRefusalKind::InvalidValue, limit: "retained record-body physical credits" })); }\n        if maximum_allocation_bytes == 0 || maximum_allocation_bytes > isize::MAX as usize { return Err(PackError::Refusal(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body physical credits" })); }\n',1),
('.map_err(|_| PackError::LimitExceeded("retained record-body symbol credits"))?,',
 '.map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?,',1),
('.map_err(|fault| PackError::RetainedMalformed { what: "retained-record-body-symbol-allocation", offset: fault.offset, detail: fault.code })?',
 '.map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol-allocation"))?',2),
('fault: PackError::RetainedMalformed { what: "retained-record-body-symbol-allocation", offset: error.fault.offset, detail: error.fault.code },',
 'fault: PackError::Refusal(error.fault.into_pack_refusal("retained-record-body-symbol-allocation")),',2),
('self.symbols.symbol_chars(symbol).map_err(|_| PackError::RetainedMalformed { what: "retained-record-body-symbol", offset: self.offset, detail: "symbol is outside admitted registry" })',
 'self.symbols.symbol_chars(symbol).map_err(|fault| PackError::Refusal(fault.into_pack_refusal("retained-record-body-symbol")))',1),
('self.symbols.symbol_char(symbol, index).map_err(|_| PackError::RetainedMalformed { what: "retained-record-body-symbol", offset: self.offset, detail: "symbol is outside admitted registry" })',
 'self.symbols.symbol_char(symbol, index).map_err(|fault| PackError::Refusal(fault.into_pack_refusal("retained-record-body-symbol")))',1),
('value.admit_byte(self.value_offset, byte).map_err(|_| PackError::RetainedMalformed {\n                    what: "retained-record-body",\n                    offset,\n                    detail: "value producer handback",\n                })?;',
 'value.admit_byte(self.value_offset, byte).map_err(|_| PackError::Refusal(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated,\n                    what: "retained-record-body",\n                    offset,\n                    detail: "value producer handback",\n                }))?;',1),
('                    if length > self.limits.max_segment_len || length > usize::MAX as u64 {\n                        return Err(PackError::LimitExceeded("retained record-body symbol length"));\n                    }\n',
 '                    if length > self.limits.max_segment_len { return Err(PackError::Refusal(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body symbol length" })); }\n                    if length > usize::MAX as u64 { return Err(PackError::Refusal(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body symbol length" })); }\n',1),
('.map_err(|fault| PackError::RetainedMalformed { what: "retained-record-body-symbol", offset: fault.offset, detail: fault.code })?;',
 '.map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?;',3),
('{Err(PackError::Schema("chunk owner has no controlled admission".into()))}',
 '{Err(PackError::from(ValueError::new(ValueRefusalKind::UnsupportedOwner,"chunk owner has no controlled admission")))}',1),
('symbols.get(usize::try_from(symref).map_err(|_| PackError::LimitExceeded("symbol reference index"))?).map(String::as_str).ok_or_else(|| PackError::Malformed {\n            what: "symref",\n            offset: 0,\n            detail: format!("symref {symref} out of range for inline table of {}", symbols.len()),\n        })?,',
 'symbols.get(usize::try_from(symref).map_err(|_| PackError::Refusal(PackRefusal::LimitExceeded { kind: ValueRefusalKind::InvalidValue, limit: "symbol reference index" }))?).map(String::as_str).ok_or_else(|| PackError::Refusal(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue,\n            what: "symref",\n            offset: 0,\n            detail: format!("symref {symref} out of range for inline table of {}", symbols.len()),\n        }))?,',1),
('PackError::Schema("manifest not loaded".to_string())',
 'PackError::from(ValueError::new(ValueRefusalKind::InvariantViolated,"manifest not loaded"))',1),
('PackError::Schema("controlled manifest absent".into())',
 'PackError::from(ValueError::new(ValueRefusalKind::InvariantViolated,"controlled manifest absent"))',1),
]
for old,new,n in R:
    c=s.count(old)
    if c!=n: sys.exit(f'COUNT {c}!={n}: {old[:90]}')
    s=s.replace(old,new)
open(dst,'w').write(s)
print('ok')
