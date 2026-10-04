//! ⚠️ Unmounted canonical Pack producer-authority draft; no native runtime verdict.
use semio_framework_diagnostic::{FaultOrigin,TextError};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::list::{PagedListError,PagedListAllocationError,PagedListRefusalKind};
#[cfg(test)]
#[path="../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[path="🔌️capture/🦀️.rs"]
mod transport_capture;
pub use transport_capture::{OwnedTransportError,TransportAdmission,TransportCaptureRefusal,TransportCaptureStage,TransportReservation,reserve_transport,PackTransportContext,PackTransportPolicy,PackTransportPhase,PackTransportProgress,TransportContextRefusal,TransportContextRefusalCause,SharedTransportReservation};

/// 🔁️ The transport owner explicitly chooses whether one failure may be retried.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PackRetryDisposition{Never,Transient}

/// 🔌️ The actual provider operation owns the external failure category.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PackTransportCategory{NativeIo,HttpRequest,HttpBody}

/// 🪪️ External transport never invents a semantic Value refusal.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PackCauseKind{Refusal(ValueRefusalKind),Transport(PackTransportCategory)}

/// 🚨️ Source-owned container refusals retain their precise machine cause.
#[derive(Debug,Clone,PartialEq)]
pub enum PackError{
 BadMagic,
 UnsupportedVersion{major:u16,minor:u16},
 UnknownRequiredFlags(u32),
 Truncated(u64),
 ChecksumMismatch{segment:&'static str,offset:u64},
 ContentHashMismatch,
 LimitExceeded{kind:ValueRefusalKind,limit:&'static str},
 RetainedMalformed{kind:ValueRefusalKind,what:&'static str,offset:u64,detail:&'static str},
 RetainedAllocation{kind:ValueRefusalKind,allocated_bytes:usize,what:&'static str,offset:u64,detail:&'static str},
 Malformed{kind:ValueRefusalKind,what:&'static str,offset:u64,detail:String},
 NonCanonical(&'static str),
 UnsupportedCodec(u8),
 ValueRefusal(ValueError),
 TextRefusal(TextError),
 Io{error:ValueError,retry:PackRetryDisposition},
 TransportFailure(OwnedTransportError),
 TransportAdmission{category:PackTransportCategory,refusal:TransportCaptureRefusal},
}

impl PackError{
 /// 📋️ Preserves the lower borrowed source's kind and static reason.
 pub const fn from_paged_refusal(error:PagedListError,what:&'static str,offset:u64)->Self{
  Self::RetainedMalformed{kind:paged_kind(error.kind),what,offset,detail:error.reason}
 }
 /// 🧱️ Preserves the actual allocated byte witness alongside the borrowed cause.
 pub const fn from_paged_allocation(error:PagedListAllocationError,what:&'static str,offset:u64)->Self{
  Self::RetainedAllocation{kind:paged_kind(error.kind),allocated_bytes:error.allocated_bytes,what,offset,detail:error.reason}
 }
 /// 🧭️ Reads intrinsic syntax authority or the actual mandatory producer-authored kind.
 pub const fn cause_kind(&self)->PackCauseKind{
  match self{
   Self::TransportFailure(error)=>PackCauseKind::Transport(error.category()),
   Self::TransportAdmission{refusal,..}=>PackCauseKind::Refusal(refusal.kind),
   Self::BadMagic|Self::Truncated(_)|Self::ChecksumMismatch{..}|Self::ContentHashMismatch|Self::NonCanonical(_)=>PackCauseKind::Refusal(ValueRefusalKind::InvalidValue),
   Self::UnsupportedVersion{..}|Self::UnknownRequiredFlags(_)|Self::UnsupportedCodec(_)=>PackCauseKind::Refusal(ValueRefusalKind::UnsupportedOwner),
   Self::LimitExceeded{kind,..}|Self::RetainedMalformed{kind,..}|Self::RetainedAllocation{kind,..}|Self::Malformed{kind,..}=>PackCauseKind::Refusal(*kind),
   Self::ValueRefusal(error)|Self::Io{error,..}=>PackCauseKind::Refusal(error.kind),
   Self::TextRefusal(error)=>PackCauseKind::Refusal(error.kind),
  }
 }
 /// 🔎️ Returns a Value refusal only when the producer supplied semantic authority.
 pub const fn refusal_kind(&self)->Option<ValueRefusalKind>{match self.cause_kind(){PackCauseKind::Refusal(kind)=>Some(kind),PackCauseKind::Transport(_)=>None}}
}

const fn paged_kind(kind:PagedListRefusalKind)->ValueRefusalKind{
 match kind{PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated}
}

impl std::fmt::Display for PackError{
 fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
  match self{
   Self::BadMagic=>formatter.write_str("bad magic"),
   Self::UnsupportedVersion{major,minor}=>write!(formatter,"unsupported version {major}.{minor}"),
   Self::UnknownRequiredFlags(flags)=>write!(formatter,"unknown required feature bits {flags:#x}"),
   Self::Truncated(offset)=>write!(formatter,"truncated at offset {offset}"),
   Self::ChecksumMismatch{segment,offset}=>write!(formatter,"checksum mismatch in {segment} at offset {offset}"),
   Self::ContentHashMismatch=>formatter.write_str("content hash mismatch"),
   Self::LimitExceeded{limit,..}=>write!(formatter,"limit exceeded: {limit}"),
   Self::RetainedMalformed{what,offset,detail,..}|Self::RetainedAllocation{what,offset,detail,..}=>write!(formatter,"malformed {what} at offset {offset}: {detail}"),
   Self::Malformed{what,offset,detail,..}=>write!(formatter,"malformed {what} at offset {offset}: {detail}"),
   Self::NonCanonical(detail)=>write!(formatter,"non-canonical encoding: {detail}"),
   Self::UnsupportedCodec(codec)=>write!(formatter,"unsupported codec {codec}"),
   Self::ValueRefusal(error)=>write!(formatter,"schema error: {error}"),
   Self::TextRefusal(error)=>write!(formatter,"schema error: {error}"),
   Self::Io{error,..}=>write!(formatter,"io error: {error}"),
   Self::TransportFailure(error)=>write!(formatter,"io error: {error}"),
   Self::TransportAdmission{refusal,..}=>write!(formatter,"transport admission: {refusal}"),
  }
 }
}

impl std::error::Error for PackError{
 fn source(&self)->Option<&(dyn std::error::Error+'static)>{
  match self{Self::ValueRefusal(error)|Self::Io{error,..}=>Some(error),Self::TextRefusal(error)=>Some(error),Self::TransportFailure(error)=>Some(error.source_error()),Self::TransportAdmission{refusal,..}=>Some(refusal),_=>None}
 }
}
impl From<ValueError> for PackError{fn from(error:ValueError)->Self{Self::ValueRefusal(error)}}
impl From<TextError> for PackError{fn from(error:TextError)->Self{Self::TextRefusal(error)}}
semio_framework_diagnostic::fault_from_error!(PackError,FaultOrigin::Module,"module.pack");

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod producer_authority_tests;
