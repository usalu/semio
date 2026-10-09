pub use semio_framework_value::{native_encoding,native_decoding,ValueError,ValueRefusalKind};
#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs"]
mod allocation;

#[cfg(test)]
mod receiving {
 use super::native_encoding::{NativeEncodeControl,NativeEncodeAllocation};
 use super::{ValueError,ValueRefusalKind};
 #[test]
 fn original_decoder_refusal_causes_no_actual_allocator_call(){
  use super::native_decoding::{NativeDecodeControl,NativeDecodeAllocation};
  let source="雪";let pointer=source.as_ptr();for kind in [ValueRefusalKind::OwnershipLimit,ValueRefusalKind::Canceled]{let mut progress=|_|true;let mut allocate=|_:NativeDecodeAllocation|Err(ValueError::literal(kind,"actual original decoder refusal"));let mut control=NativeDecodeControl::new_forwarded(source.len(),&mut progress,&mut allocate);let(result,owned,released)=crate::test_allocation::observe_backing(||control.copy_text(source));assert_eq!((owned,released),(0,0));assert_eq!(result.unwrap_err().kind,kind);assert_eq!(control.owned_bytes(),0);assert_eq!(source.as_ptr(),pointer);}
  println!("[DEBUG] Actual decoder allocation frontier: refusalKinds=2 allocatorOwned=0 allocatorReleased=0 cumulativeOwned=0 originalPointerPreserved=true");
 }
 #[test]
 fn original_foreign_refusal_causes_no_actual_allocator_call(){
  let source="雪";let pointer=source.as_ptr();
  for kind in [ValueRefusalKind::OwnershipLimit,ValueRefusalKind::Canceled]{let mut progress=|_|true;let mut allocate=|_:NativeEncodeAllocation|Err(ValueError::literal(kind,"actual original allocation refusal"));let mut control=NativeEncodeControl::new_forwarded(source.len(),&mut progress,&mut allocate);let(result,owned,released)=crate::test_allocation::observe_backing(||control.copy_text(source));assert_eq!((owned,released),(0,0));assert_eq!(result.unwrap_err().kind,kind);assert_eq!(control.owned_bytes(),0);assert_eq!(source.as_ptr(),pointer);}
  println!("[DEBUG] Actual forwarded native allocation frontier: refusalKinds=2 allocatorOwned=0 allocatorReleased=0 cumulativeOwned=0 originalPointerPreserved=true");
 }
}

#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🛂️allocation/🧪️tests/🦀️.rs"]
mod decoder_allocation;

#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs"]
mod io_authority;

#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs"]
pub mod sqlite_snapshot;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs"]
mod original_operation_control;
pub use original_operation_control::{EntityIdentityAuthority,OriginalOperationReceiver};
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/📨️slot/🦀️.rs"]
mod operation_slot;

#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/📦️archive/🧬️schema/🦀️.rs"]
mod archive_schema;
#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/📦️archive/🛬️decode/🦀️.rs"]
mod archive_decoding;

#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🔨️modules/🚪️io/📦️payload/🧬️schema/🦀️.rs"]
pub mod io_schema;
#[cfg(test)]
pub mod os_store{pub use crate::io_authority::{NativeSnapshotEncodeOwner,NativeSnapshotDecodeOwner,NativeSnapshotBodyWallet};}
#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs"]
pub mod semio_format;
#[cfg(test)]
pub use pack::PackRefusal;
#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs"]
mod snapshot_record_encoding;

#[cfg(test)]
pub use pack::record::DecodeOptions as PackDecodeOptions;
#[cfg(test)]
pub use semio_framework_diagnostic::TextError;
#[cfg(test)]
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs"]
mod snapshot_record_decoding;
