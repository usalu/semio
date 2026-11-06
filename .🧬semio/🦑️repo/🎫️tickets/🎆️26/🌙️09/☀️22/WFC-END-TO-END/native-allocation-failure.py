from pathlib import Path

p=Path('🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/🦀️.rs')
s=p.read_text()
anchor='/// 🧳️ Body failure ends production;'
definition='''/// 📦️ Keeps every original allocation outcome without reducing its independent refusals.
pub enum NativeSnapshotAllocationFailure{Encode(semio_framework_value::native_encoding::NativeEncodeAllocationOutcome),Decode(semio_framework_value_native_owned::native_decoding::NativeDecodeAllocationOutcome)}
impl std::fmt::Debug for NativeSnapshotAllocationFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Encode(v)=>f.debug_struct("Encode").field("primary",&v.primary).field("extent_refusal",&v.extent_refusal).field("forwarded_refusal",&v.forwarded_refusal).field("scoped_refusal",&v.scoped_refusal).field("actual",&v.actual).finish(),Self::Decode(v)=>f.debug_struct("Decode").field("primary",&v.primary).field("extent_refusal",&v.extent_refusal).field("settlement_refusal",&v.settlement_refusal).field("actual",&v.actual).finish()}}}
impl PartialEq for NativeSnapshotAllocationFailure{fn eq(&self,other:&Self)->bool{match(self,other){(Self::Encode(a),Self::Encode(b))=>a.primary==b.primary&&a.extent_refusal==b.extent_refusal&&a.forwarded_refusal==b.forwarded_refusal&&a.scoped_refusal==b.scoped_refusal&&a.actual==b.actual,(Self::Decode(a),Self::Decode(b))=>a.primary==b.primary&&a.extent_refusal==b.extent_refusal&&a.settlement_refusal==b.settlement_refusal&&a.actual==b.actual,_=>false}}}
impl RetireOwned for NativeSnapshotAllocationFailure{fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Encode(value)=>value.retirement(),Self::Decode(value)=>value.retirement()}}fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Encode(value)=>value.retirement_birth_bytes(),Self::Decode(value)=>value.retirement_birth_bytes()}}fn controlled_retirement_supported()->bool{true}}
'''
assert anchor in s;s=s.replace(anchor,definition+anchor,1)
s=s.replace(' pub(super) cause:Option<NativeSnapshotCause>,',' pub(super) cause:Option<NativeSnapshotCause>,\n pub(super) allocation:Option<NativeSnapshotAllocationFailure>,',1)
s=s.replace('impl NativeSnapshotFailure{\n', '''impl NativeSnapshotFailure{
 /// 📦️ Borrows the actual original allocation outcome with every independent refusal.
 pub fn allocation(&self)->Option<&NativeSnapshotAllocationFailure>{self.allocation.as_ref()}
 /// 🧳️ Adopts one original allocation outcome or returns both owners if already occupied.
 pub fn with_allocation(mut self,allocation:NativeSnapshotAllocationFailure)->Result<Self,(Self,NativeSnapshotAllocationFailure)>{if self.allocation.is_some(){return Err((self,allocation))}self.allocation=Some(allocation);Ok(self)}
 /// 🏗️ Creates an original failure retaining the complete allocation outcome.
 pub fn from_allocation(allocation:NativeSnapshotAllocationFailure)->Self{Self{cause:None,allocation:Some(allocation),receipt_refusal:None,context_refusal:None,work_refusal:None,settlement_refusal:None,progress:Default::default()}}
''',1)
s=s.replace('Self{cause:', 'Self{allocation:None,cause:')
s=s.replace('Self{allocation:None,cause:None,allocation:Some(allocation)', 'Self{cause:None,allocation:Some(allocation)')
s=s.replace('self.cause==other.cause&&self.receipt_refusal', 'self.cause==other.cause&&self.allocation==other.allocation&&self.receipt_refusal')
s=s.replace('deferred(self.cause),deferred(self.receipt_refusal)', 'deferred(self.cause),deferred(self.allocation),deferred(self.receipt_refusal)')
s=s.replace('deferred_birth_bytes_for(&self.cause),deferred_birth_bytes_for(&self.receipt_refusal)', 'deferred_birth_bytes_for(&self.cause),deferred_birth_bytes_for(&self.allocation),deferred_birth_bytes_for(&self.receipt_refusal)')
p.write_text(s)
p=Path('🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs')
s=p.read_text().replace('pub use failure::{NativeSnapshotFailure,NativeSnapshotCause,NativeSnapshotWork,native_snapshot_turn};','pub use failure::{NativeSnapshotFailure,NativeSnapshotCause,NativeSnapshotAllocationFailure,NativeSnapshotWork,native_snapshot_turn};').replace('NativeSnapshotFailure{cause,context_refusal:None','NativeSnapshotFailure{cause,allocation:None,context_refusal:None')
p.write_text(s)
