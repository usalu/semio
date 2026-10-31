# Paged Append Caller Inventory

Bounded literal framework Rust census of actual cursor type and quote method; no runtime/Source-fit claim. Every actual append normal caller must pass original body and forward full outcome work/receipt before Err. Unrelated next_advance_demand methods are excluded by source review, not renamed mechanically.

- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🪪️identity/🦀️.rs`:4 use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,paged::{PagedUtf8,PagedUtf8AppendCursor,Utf8Text},re
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🪪️identity/🦀️.rs`:17 pub struct ArtifactCanonicalEditIdentityCursor<M:Send+Sync+'static>{authority:Option<RetainedOwnedProjection<dyn ArtifactCanonicalEditAuthority>>,edit
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🪪️identity/🦀️.rs`:29    if owner.terminal_is_empty(){return Ok(copy(size_of::<Option<PagedUtf8AppendCursor>>()));}
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🪪️identity/🦀️.rs`:31    let text=self.text()?;let chunk=if text.text_bytes()==0{""}else{text.text_chunk(self.chunk).ok_or_else(||refusal("canonical identity native input c
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🪪️identity/🦀️.rs`:36   }else if self.field<3{return Ok(copy(size_of::<PagedUtf8AppendCursor>()));}
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧵️canonical/🪪️identity/🦀️.rs`:49   else if !self.closing&&self.field<3{self.append=Some(PagedUtf8AppendCursor::default());RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🧪️tests/🔬️unit/🦀️.rs`:55         let (mut cursor, birth) = observe_retirement_allocations(crate::paged::PagedUtf8AppendCursor::default);
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🦀️.rs`:169 pub use append::PagedUtf8AppendCursor;
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs`:7 pub struct PagedUtf8AppendCursor {
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs`:21 impl PagedUtf8AppendCursor {
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs`:28         if let Some(message)=self.advance_fault{return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,m
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs`:75 impl ErasedSnapshotRetirement for PagedUtf8AppendCursor {
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs`:78     fn terminal_is_empty(&self) -> bool { PagedUtf8AppendCursor::terminal_is_empty(self) }
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs`:85 impl Drop for PagedUtf8AppendCursor {
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🧪️tests/🦀️.rs`:11   let source=text.as_str().unwrap().repeat(repeat);let pointer=source.as_ptr();let mut destination=PagedUtf8::<{usize::MAX}>::new();let(mut cursor,hea
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🧪️tests/🦀️.rs`:12   loop{assert!(turns<100000);if pause==Some(turns){break;}let(demand,heap)=observe(||cursor.next_advance_demand(&source,&destination).unwrap());assert
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🎟️work/🧪️tests/🦀️.rs`:5 fn finish(mut append:PagedUtf8AppendCursor,value:PagedUtf8<8192>,bank:&mut Bank,max:u64){append.begin_close();for _ in 0..max{if append.terminal_is_em
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🎟️work/🧪️tests/🦀️.rs`:6 #[test]fn actual_append_normal_uses_real_work_with_partial_utf8_and_original_stock_through_close(){let f:serde_json::Value=serde_json::from_str(includ
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🎟️work/🧪️tests/🦀️.rs`:7 #[test]fn actual_append_rejects_changed_source_utf8_before_publishing_and_caches_only_the_real_fault(){let f:serde_json::Value=serde_json::from_str(in
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/♻️close/🧪️tests/🦀️.rs`:5  let mut owner=PagedUtf8AppendCursor{source:Some((17,3)),destination:Some((29,7)),..Default::default()};
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs`:2 use super::{PagedUtf8,PagedUtf8AppendCursor};
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs`:6 pub struct PagedTextReceiver<I:NativeValueSource+?Sized,const N:usize>{binding:Option<RetainedCloneBinding>,append:PagedUtf8AppendCursor,output:Manual
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🛬️receive/🦀️.rs`:9  fn demands(&self,input:RetainedCloneRef<'_,I>,_:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.ready||self.taken{return Err(invali
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs`:463         let (mut cursor, allocation) = observe_retirement_allocations(crate::paged::PagedUtf8AppendCursor::default);
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs`:171     pub handoff_cursors: [Option<semio_framework_value::paged::PagedUtf8AppendCursor>; 2],
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs`:430                 let copy=size_of::<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>()+size_of::<semio_framework_value::paged::PagedUtf8AppendCur
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs`:438                 let demand=cursor.next_advance_demand(source,output)?;
