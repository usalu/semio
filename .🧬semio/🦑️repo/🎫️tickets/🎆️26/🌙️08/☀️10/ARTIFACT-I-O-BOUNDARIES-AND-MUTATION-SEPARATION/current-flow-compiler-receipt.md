# Current Procedural Dependency Compiler Receipt

Actual registered locked native check 82126 failed in the Flow dependency. These diagnostics supersede the earlier stale-lock failure. Root or assigned peer must repair direct first-party consumer contracts before native artifact laws can execute.

```text
error[E0308]: mismatched types
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/../🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:333:30
    |
333 |         match cursor.advance(Grant { maximum_items: 1, maximum_bytes: bytes }) {
    |                      ------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RetainedCloneGrant`, found `Grant`
    |                      |
    |                      arguments to this method are incorrect
    |
note: method defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/🦀️.rs:539:12
    |
539 |     pub fn advance(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
    |            ^^^^^^^


error[E0308]: mismatched types
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/../🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:352:116
    |
352 | ...rsor.is_complete(){cursor.advance_insert_controlled(grant,&mut native)?;}let next=cursor.take_result().ok_or_else(||ValueError::...
    |                              ------------------------- ^^^^^ expected `RetainedCloneGrant`, found `Grant`
    |                              |
    |                              arguments to this method are incorrect
    |
note: method defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/🦀️.rs:380:12
    |
380 |     pub fn advance_insert_controlled(&mut self, grant: RetainedCloneGrant, control: &mut crate::NativeDecodeControl<'_>) -> Result<...
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0308]: mismatched types
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/../🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs:353:67
    |
353 | ...   cursor.begin_close();loop{match cursor.close_step(grant){RetirementStep::OwnedValue(value)=>retire(value),RetirementStep::Com...
    |                                              ---------- ^^^^^ expected `RetainedCloneGrant`, found `Grant`
    |                                              |
    |                                              arguments to this method are incorrect
    |
note: method defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/🦀️.rs:393:12
    |
393 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> { self.state.close_step(grant) }
    |            ^^^^^^^^^^

[native:owner-command] running elapsedMs=150049

error[E0599]: no method named `close_page` found for mutable reference `&mut retained::FlowRetirement` in the current scope
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs:41:94
   |
41 |         self.close_step_with(maximum_items, maximum_bytes, |frontier, items, bytes| frontier.close_page(items, bytes))
   |                                                                                              ^^^^^^^^^^ method not found in `&mut retained::FlowRetirement`


error[E0599]: no method named `next_close_byte_demand` found for struct `retained::FlowRetirement` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🦀️.rs:237:38
    |
237 | ...   let demand = self.retirement.next_close_byte_demand().map_err(|message|semio_framework_value::ValueError::new(semio_framework...
    |                                    ^^^^^^^^^^^^^^^^^^^^^^
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
    |
 32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
    | ------------------------- method `next_close_byte_demand` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `ArtifactEnvelopeFieldDecoder`
            candidate #2: `ArtifactEnvelopeMutationFieldAuthority`
            candidate #3: `ArtifactEnvelopeSnapshotFieldAuthority`
            candidate #4: `ArtifactEnvelopeVcsFieldAuthority`
            candidate #5: `ArtifactOwnedHistoryEntryAuthority`
            candidate #6: `ArtifactStoreOneItemPreparation`
            candidate #7: `ArtifactStoreOwnedDisposer`
            candidate #8: `ErasedMemberStoreOneItemPublication`
            candidate #9: `MemberOpenOperation`
            candidate #10: `RetirementCursor`
            candidate #11: `dsl::ErasedSnapshotRetirement`

error[E0308]: mismatched types
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/../🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛬️decoding/🦀️.rs:19:278
    |
 19 | ...);match retirement.advance(Grant{maximum_items:1,maximum_bytes}){RetirementStep::OwnedValue(value)=>V::retire_decoded(value),Ret...
    |                       ------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RetainedCloneGrant`, found `Grant`
    |                       |
    |                       arguments to this method are incorrect
    |
note: method defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/🦀️.rs:539:12
    |
539 |     pub fn advance(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> {
    |            ^^^^^^^


error[E0308]: mismatched types
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/../🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛬️decoding/🦀️.rs:20:506
    |
 20 | ...date.is_complete(){update.advance_insert_controlled(grant,control)?;}let replacement=update.take_result().ok_or_else(||ValueErro...
    |                              ------------------------- ^^^^^ expected `RetainedCloneGrant`, found `Grant`
    |                              |
    |                              arguments to this method are incorrect
    |
note: method defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/🦀️.rs:380:12
    |
380 |     pub fn advance_insert_controlled(&mut self, grant: RetainedCloneGrant, control: &mut crate::NativeDecodeControl<'_>) -> Result<...
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0308]: mismatched types
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/../🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛬️decoding/🦀️.rs:20:741
     |
  20 | ..._removed(){if let Some(value)=Arc::into_inner(value){V::retire_decoded(value);}retire_map(replacement);return Err(invalid("Flow...
     |                                  --------------- ^^^^^ expected `Arc<_, _>`, found `SharedOwner<V>`
     |                                  |
     |                                  arguments to this function are incorrect
     |
     = note: expected struct `Arc<_, _>`
                found struct `SharedOwner<V>`
note: associated function defined here
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/sync.rs:1230:12
     |
1230 |     pub fn into_inner(this: Self) -> Option<T> {
     |            ^^^^^^^^^^
help: call `Into::into` on this expression to convert `SharedOwner<V>` into `Arc<_, _>`
     |
  20 | fn insert<V:FromValue>(map:&mut crate::OrderedMap<V>,key:String,value:V,control:&mut NativeDecodeControl<'_>)->Result<()>{use semio_framework_value::ordered::{Grant,RetirementStep};let value=semio_framework_value::DecodedValue::new(value,V::retire_decoded);control.charge(size_of::<String>()+size_of::<V>()+4*size_of::<usize>())?;let mut update=map.begin_set(key,value.take());let grant=Grant{maximum_items:1,maximum_bytes:4096};let result=(||{while !update.is_complete(){update.advance_insert_controlled(grant,control)?;}let replacement=update.take_result().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Flow native ordered result missing"))?;if let Some(value)=update.take_removed(){if let Some(value)=Arc::into_inner(value.into()){V::retire_decoded(value);}retire_map(replacement);return Err(invalid("Flow native ordered key repeated"))}retire_map(std::mem::replace(map,replacement));Ok(())})();update.begin_close();loop{match update.close_step(grant){RetirementStep::OwnedValue(value)=>V::retire_decoded(value),RetirementStep::Complete=>break,_=>{}}}result}
     |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          +++++++


error[E0308]: mismatched types
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/../🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🛬️decoding/🦀️.rs:20:962
    |
 20 | ....begin_close();loop{match update.close_step(grant){RetirementStep::OwnedValue(value)=>V::retire_decoded(value),RetirementStep::C...
    |                                     ---------- ^^^^^ expected `RetainedCloneGrant`, found `Grant`
    |                                     |
    |                                     arguments to this method are incorrect
    |
note: method defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/🦀️.rs:393:12
    |
393 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep<V> { self.state.close_step(grant) }
    |            ^^^^^^^^^^


error[E0599]: no method named `next_close_byte_demand` found for struct `retained::FlowRetirement` in the current scope
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs:66:36
   |
66 | ...   let demand = self.frontier.next_close_byte_demand().map_err(|message|semio_framework_value::ValueError::new(semio_framework_va...
   |                                  ^^^^^^^^^^^^^^^^^^^^^^
   |
  ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
   |
32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
   | ------------------------- method `next_close_byte_demand` not found for this struct
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
           candidate #1: `ArtifactEnvelopeFieldDecoder`
           candidate #2: `ArtifactEnvelopeMutationFieldAuthority`
           candidate #3: `ArtifactEnvelopeSnapshotFieldAuthority`
           candidate #4: `ArtifactEnvelopeVcsFieldAuthority`
           candidate #5: `ArtifactOwnedHistoryEntryAuthority`
           candidate #6: `ArtifactStoreOneItemPreparation`
           candidate #7: `ArtifactStoreOwnedDisposer`
           candidate #8: `ErasedMemberStoreOneItemPublication`
           candidate #9: `MemberOpenOperation`
           candidate #10: `RetirementCursor`
           candidate #11: `dsl::ErasedSnapshotRetirement`

error[E0599]: no method named `close_page` found for struct `retained::FlowRetirement` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🦀️.rs:238:34
    |
238 |         Ok(match self.retirement.close_page(maximum_items, maximum_bytes.max(demand))? {
    |                                  ^^^^^^^^^^ method not found in `retained::FlowRetirement`
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
    |
 32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
    | ------------------------- method `close_page` not found for this struct


error[E0599]: no method named `next_allocation_bytes` found for struct `retained::FlowRetirement` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🦀️.rs:373:31
    |
373 |         match self.retirement.next_allocation_bytes() {
    |                               ^^^^^^^^^^^^^^^^^^^^^ method not found in `retained::FlowRetirement`
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
    |
 32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
    | ------------------------- method `next_allocation_bytes` not found for this struct


error[E0277]: the trait bound `retained::FlowRetirement: dsl::ErasedSnapshotRetirement` is not satisfied
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🦀️.rs:249:58
    |
249 |         ErasedSnapshotRetirement::next_close_byte_demand(&self.retirement)
    |         ------------------------------------------------ ^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |         |
    |         required by a bound introduced by this call
    |
help: the trait `dsl::ErasedSnapshotRetirement` is not implemented for `retained::FlowRetirement`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
    |
 32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `dsl::ErasedSnapshotRetirement`:
              ArtifactEditMessageLedgerRejected
              ArtifactEnvelopeDecodeRejected<P, Mutation>
              ArtifactEnvelopeReturnedFieldDecoder<P, Mutation>
              ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation>
              ArtifactReplayPreparationRetirement<P, M>
              ArtifactStoreBatchPublication<P, Mutation>
              FlowHostSnapshotRetirement
              FlowMutationRetirement
            and 14 others


error[E0308]: `match` arms have incompatible types
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs:26:17
   |
20 | /         match mutation {
21 | |             FlowMutation::AddWidget(value) => self.frontier.push(FlowOwner::Widget(value.widget)),
   | |                                               --------------------------------------------------- this is found to be of type `std::result::Result<(), retained::FlowOwner>`
22 | |             FlowMutation::RemoveWidget(value) => self.frontier.text(value.id),
   | |                                                  ---------------------------- this is found to be of type `std::result::Result<(), retained::FlowOwner>`
23 | |             FlowMutation::MoveWidget(value) => self.frontier.text(value.id),
   | |                                                ---------------------------- this is found to be of type `std::result::Result<(), retained::FlowOwner>`
...  |
26 | |                 self.frontier.push(FlowOwner::Widget(value.widget));
   | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<(), FlowOwner>`, found `()`
...  |
36 | |             FlowMutation::ReplaceFlowHostSnapshot(value) => self.frontier.push(FlowOwner::HostSnapshot(value.host_snapshot)),
37 | |         }
   | |_________- `match` arms have incompatible types
   |
   = note:   expected enum `std::result::Result<(), retained::FlowOwner>`
           found unit type `()`
help: consider removing this semicolon
   |
26 -                 self.frontier.push(FlowOwner::Widget(value.widget));
26 +                 self.frontier.push(FlowOwner::Widget(value.widget))

error[E0599]: no method named `reserve_allocation` found for struct `retained::FlowRetirement` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🦀️.rs:377:46
    |
377 |                 return match self.retirement.reserve_allocation(grant.maximum_capacity_bytes) {
    |                                              ^^^^^^^^^^^^^^^^^^ method not found in `retained::FlowRetirement`
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
    |
 32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
    | ------------------------- method `reserve_allocation` not found for this struct


error[E0599]: no method named `close_page` found for struct `retained::FlowRetirement` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🦀️.rs:386:31
    |
386 |         match self.retirement.close_page(1, grant.maximum_copy_bytes) {
    |                               ^^^^^^^^^^ method not found in `retained::FlowRetirement`
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
    |
 32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
    | ------------------------- method `close_page` not found for this struct


error[E0599]: no method named `next_allocation_bytes` found for struct `retained::FlowRetirement` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🦀️.rs:401:77
    |
401 | ...tion<usize> { self.retirement.next_allocation_bytes().ok().map(|bytes| bytes.unwrap_or(0)) }
    |                                  ^^^^^^^^^^^^^^^^^^^^^ method not found in `retained::FlowRetirement`
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
    |
 32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
    | ------------------------- method `next_allocation_bytes` not found for this struct


error[E0599]: no method named `next_close_byte_demand` found for struct `retained::FlowRetirement` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/📑️copy/🦀️.rs:445:43
    |
445 | ...   let demand = state.retirement.next_close_byte_demand().map_err(|message|semio_framework_value::ValueError::new(semio_framewor...
    |                                     ^^^^^^^^^^^^^^^^^^^^^^
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
    |
 32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
    | ------------------------- method `next_close_byte_demand` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `ArtifactEnvelopeFieldDecoder`
            candidate #2: `ArtifactEnvelopeMutationFieldAuthority`
            candidate #3: `ArtifactEnvelopeSnapshotFieldAuthority`
            candidate #4: `ArtifactEnvelopeVcsFieldAuthority`
            candidate #5: `ArtifactOwnedHistoryEntryAuthority`
            candidate #6: `ArtifactStoreOneItemPreparation`
            candidate #7: `ArtifactStoreOwnedDisposer`
            candidate #8: `ErasedMemberStoreOneItemPublication`
            candidate #9: `MemberOpenOperation`
            candidate #10: `RetirementCursor`
            candidate #11: `dsl::ErasedSnapshotRetirement`

error[E0599]: no method named `close_page` found for struct `retained::FlowRetirement` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/📑️copy/🦀️.rs:446:41
    |
446 |             let step = state.retirement.close_page(1, maximum_bytes.max(demand))?;
    |                                         ^^^^^^^^^^ method not found in `retained::FlowRetirement`
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🦀️.rs:32:1
    |
 32 | pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
    | ------------------------- method `close_page` not found for this struct

```
