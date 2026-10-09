# Current Native Compiler Receipt

Actual native owner exited 1. This is a compiler refusal, not a timeout. The target did not reach its own runtime law.

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:123:5
    |
123 |       fn next_close_byte_demand(&self) -> usize {
    |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
    |  _____|
    | |
124 | |         self.retirement.as_ref().map_or_else(|| self.pending.as_ref().map_or(1, |pending| pending.capacity().max(1)), |retirement...
125 | |     }
    | |_____^ not a member of trait `ErasedSnapshotRetirement`

error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:22:5
   |
22 |     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`

error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:22:5
   |
22 |     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
   |     not a member of trait `ErasedSnapshotRetirement`

error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:71:5
   |
71 |     fn next_close_byte_demand(&self)->usize{if self.bytes.len()!=0{1}else{self.bytes.next_release_allocation_bytes().unwrap_or(0)}}
   |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
   |     not a member of trait `ErasedSnapshotRetirement`


error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:71:5
   |
71 |     fn next_close_byte_demand(&self)->usize{if self.bytes.len()!=0{1}else{self.bytes.next_release_allocation_bytes().unwrap_or(0)}}
   |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
   |     not a member of trait `ErasedSnapshotRetirement`

error[E0425]: cannot find function `owned_retirement` in module `crate::retirement`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:111:94
    |
111 | ...f.retirement = Some(crate::retirement::owned_retirement(owner)); return Ok(SnapshotRetirementStep::Pending { released_items: 1, ...
    |                                           ^^^^^^^^^^^^^^^^ not found in `crate::retirement`

error[E0425]: cannot find function `owned_retirement` in module `super::retirement`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🦀️.rs:338:51
    |

error[E0425]: cannot find function `owned_retirement` in module `crate::retirement`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:111:94
    |
111 | ...f.retirement = Some(crate::retirement::owned_retirement(owner)); return Ok(SnapshotRetirementStep::Pending { released_items: 1, ...
    |                                           ^^^^^^^^^^^^^^^^ not found in `crate::retirement`

error[E0425]: cannot find function `owned_retirement` in module `super::retirement`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🦀️.rs:338:51
    |
338 |         self.retirement = Some(super::retirement::owned_retirement(value));
    |                                                   ^^^^^^^^^^^^^^^^ not found in `super::retirement`

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:106:88
    |
106 |     fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
    |                                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^
    |

error[E0425]: cannot find function `owned_retirement` in module `super::retirement`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🦀️.rs:338:51
    |
338 |         self.retirement = Some(super::retirement::owned_retirement(value));
    |                                                   ^^^^^^^^^^^^^^^^ not found in `super::retirement`

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:106:88
    |
106 |     fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
    |                                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
    |
106 -     fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
106 +     fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
    |

error[E0050]: method `close_step` has 3 parameters but the declaration in trait `retirement_contract::ErasedSnapshotRetirement::close_step` has 2
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:119:19
    |
119 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
    |
   ::: 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:19
    |
 19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    |                   ------------------------------------ trait requires 2 parameters

error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:118:1
    |
118 | impl ErasedSnapshotRetirement for PagedUtf8AppendCursor {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
   ::: 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:21:5

error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:118:1
    |
118 | impl ErasedSnapshotRetirement for PagedUtf8AppendCursor {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
   ::: 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:21:5
    |
 21 |     fn next_copy_byte_demand(&self) -> usize;
    |     ----------------------------------------- `next_copy_byte_demand` from trait
 22 |     fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    |     -------------------------------------------------------------------------------------------- `next_capacity_byte_demand` from trait
 23 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
    |     ---------------------------------------------------------------- `next_release_byte_demand` from trait
 24 |     fn next_depth_demand(&self) -> Result<usize, ValueError>;
    |     --------------------------------------------------------- `next_depth_demand` from trait

error[E0050]: method `close_step` has 3 parameters but the declaration in trait `retirement_contract::ErasedSnapshotRetirement::close_step` has 2

error[E0050]: method `close_step` has 3 parameters but the declaration in trait `retirement_contract::ErasedSnapshotRetirement::close_step` has 2
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:13:19
   |
13 |     fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
   |
  ::: 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:19
   |
19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
   |                   ------------------------------------ trait requires 2 parameters

error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:12:1
   |
12 | impl ErasedSnapshotRetirement for NativeDecodeRetirementRecipient{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
   |
  ::: 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:21:5

error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:12:1
   |
12 | impl ErasedSnapshotRetirement for NativeDecodeRetirementRecipient{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
   |
  ::: 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:21:5
   |
21 |     fn next_copy_byte_demand(&self) -> usize;
   |     ----------------------------------------- `next_copy_byte_demand` from trait
22 |     fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
   |     -------------------------------------------------------------------------------------------- `next_capacity_byte_demand` from trait
23 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
   |     ---------------------------------------------------------------- `next_release_byte_demand` from trait
24 |     fn next_depth_demand(&self) -> Result<usize, ValueError>;
   |     --------------------------------------------------------- `next_depth_demand` from trait

error[E0050]: method `close_step` has 3 parameters but the declaration in trait `retirement_contract::ErasedSnapshotRetirement::close_step` has 2

error[E0050]: method `close_step` has 3 parameters but the declaration in trait `retirement_contract::ErasedSnapshotRetirement::close_step` has 2
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:63:19
   |
63 |     fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
   |
  ::: 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:19
   |
19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
   |                   ------------------------------------ trait requires 2 parameters

error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:62:1
   |
62 | impl<const N:usize> ErasedSnapshotRetirement for PagedText<N>{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
   |
  ::: 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:21:5

error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:62:1
   |
62 | impl<const N:usize> ErasedSnapshotRetirement for PagedText<N>{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
   |
  ::: 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:21:5
   |
21 |     fn next_copy_byte_demand(&self) -> usize;
   |     ----------------------------------------- `next_copy_byte_demand` from trait
22 |     fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
   |     -------------------------------------------------------------------------------------------- `next_capacity_byte_demand` from trait
23 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
   |     ---------------------------------------------------------------- `next_release_byte_demand` from trait
24 |     fn next_depth_demand(&self) -> Result<usize, ValueError>;
   |     --------------------------------------------------------- `next_depth_demand` from trait

    Checking wgpu v29.0.4

error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:105:30
    |
105 |             let step = owner.close_step(1, maximum_bytes)?;
    |                              ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
    |                                         |
    |                                         expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:8
    |
 19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    |        ^^^^^^^^^^            -----
help: remove the extra argument
    |
105 -             let step = owner.close_step(1, maximum_bytes)?;
105 +             let step = owner.close_step(/* retained_clone::RetainedCloneGrant */)?;
    |

error[E0308]: mismatched types
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:106:24
    |
106 |             if step != SnapshotRetirementStep::Complete { return Ok(step); }
    |                ----    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RetainedCloneStep`, found `SnapshotRetirementStep`
    |                |
    |                expected because this is `retained_clone::RetainedCloneStep`

error[E0308]: mismatched types
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:106:69
    |
106 |             if step != SnapshotRetirementStep::Complete { return Ok(step); }
    |                                                                  -- ^^^^ expected `SnapshotRetirementStep`, found `RetainedCloneStep`
    |                                                                  |
    |                                                                  arguments to this enum variant are incorrect
    |
help: the type constructed contains `retained_clone::RetainedCloneStep` due to the type of the argument passed
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:106:66

error[E0308]: mismatched types
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:106:69
    |
106 |             if step != SnapshotRetirementStep::Complete { return Ok(step); }
    |                                                                  -- ^^^^ expected `SnapshotRetirementStep`, found `RetainedCloneStep`
    |                                                                  |
    |                                                                  arguments to this enum variant are incorrect
    |
help: the type constructed contains `retained_clone::RetainedCloneStep` due to the type of the argument passed
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:106:66
    |
106 |             if step != SnapshotRetirementStep::Complete { return Ok(step); }
    |                                                                  ^^^----^
    |                                                                     |
    |                                                                     this argument influences the type of `Ok`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
    |

error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn retirement_contract::ErasedSnapshotRetirement>` in the current scope
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:124:143
    |
124 | ... pending.capacity().max(1)), |retirement| retirement.next_close_byte_demand())
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `RetirementCursor` defines an item `next_close_byte_demand`, perhaps you need to implement it
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:25:1
    |
 25 | pub trait RetirementCursor: Send {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: there is a method `next_copy_byte_demand` with a similar name
    |
124 -         self.retirement.as_ref().map_or_else(|| self.pending.as_ref().map_or(1, |pending| pending.capacity().max(1)), |retirement| retirement.next_close_byte_demand())
124 +         self.retirement.as_ref().map_or_else(|| self.pending.as_ref().map_or(1, |pending| pending.capacity().max(1)), |retirement| retirement.next_copy_byte_demand())
    |


error[E0061]: this method takes 1 argument but 2 arguments were supplied
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:16:52
   |
16 | ...   if !owner.terminal_is_empty(){return owner.close_step(1,maximum_bytes).map(|step|match step{SnapshotRetirementStep::Complete=>...
   |                                                  ^^^^^^^^^^ - ------------- unexpected argument #2 of type `usize`
   |                                                             |
   |                                                             expected `RetainedCloneGrant`, found integer
   |
note: method defined here
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:8
   |
19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
   |        ^^^^^^^^^^            -----
help: remove the extra argument
   |
16 -         if !owner.terminal_is_empty(){return owner.close_step(1,maximum_bytes).map(|step|match step{SnapshotRetirementStep::Complete=>SnapshotRetirementStep::Pending{released_items:0,released_bytes:0},other=>other});}
16 +         if !owner.terminal_is_empty(){return owner.close_step(/* retained_clone::RetainedCloneGrant */).map(|step|match step{SnapshotRetirementStep::Complete=>SnapshotRetirementStep::Pending{released_items:0,released_bytes:0},other=>other});}
   |

error[E0308]: mismatched types
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:16:101
   |
16 | ...ap(|step|match step{SnapshotRetirementStep::Complete=>SnapshotRetirementStep::Pending{released_items:0,released_bytes:0},other=>o...
   |                   ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RetainedCloneStep`, found `SnapshotRetirementStep`
   |                   |
   |                   this expression has type `retained_clone::RetainedCloneStep`

error[E0308]: `match` arms have incompatible types
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:16:209
   |
16 | ...ep|match step{SnapshotRetirementStep::Complete=>SnapshotRetirementStep::Pending{released_items:0,released_bytes:0},other=>other});}
   |       ----------                                   ------------------------------------------------------------------        ^^^^^ expected `SnapshotRetirementStep`, found `RetainedCloneStep`
   |       |                                            |
   |       |                                            this is found to be of type `retirement_contract::SnapshotRetirementStep`
   |       `match` arms have incompatible types

error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn retirement_contract::ErasedSnapshotRetirement>` in the current scope

error[E0308]: `match` arms have incompatible types
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:16:209
   |
16 | ...ep|match step{SnapshotRetirementStep::Complete=>SnapshotRetirementStep::Pending{released_items:0,released_bytes:0},other=>other});}
   |       ----------                                   ------------------------------------------------------------------        ^^^^^ expected `SnapshotRetirementStep`, found `RetainedCloneStep`
   |       |                                            |
   |       |                                            this is found to be of type `retirement_contract::SnapshotRetirementStep`
   |       `match` arms have incompatible types

error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn retirement_contract::ErasedSnapshotRetirement>` in the current scope
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:22:148
   |
22 | ...s_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `RetirementCursor` defines an item `next_close_byte_demand`, perhaps you need to implement it
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:25:1

error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn retirement_contract::ErasedSnapshotRetirement>` in the current scope
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:22:148
   |
22 | ...s_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `RetirementCursor` defines an item `next_close_byte_demand`, perhaps you need to implement it
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:25:1
   |
25 | pub trait RetirementCursor: Send {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: there is a method `next_copy_byte_demand` with a similar name
   |
22 -     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
22 +     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_copy_byte_demand()}).unwrap_or(0)}
   |


error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn retirement_contract::ErasedSnapshotRetirement>` in the current scope
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🦀️.rs:308:129
    |
308 | ...empty() { size_of_val(owner.as_ref()) } else { owner.next_close_byte_demand() }))
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `RetirementCursor` defines an item `next_close_byte_demand`, perhaps you need to implement it
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:25:1
    |
 25 | pub trait RetirementCursor: Send {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: there is a method `next_copy_byte_demand` with a similar name
    |
308 -         Ok(self.retirement.as_ref().map_or(0, |owner| if owner.terminal_is_empty() { size_of_val(owner.as_ref()) } else { owner.next_close_byte_demand() }))
308 +         Ok(self.retirement.as_ref().map_or(0, |owner| if owner.terminal_is_empty() { size_of_val(owner.as_ref()) } else { owner.next_copy_byte_demand() }))
    |


error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🦀️.rs:351:63
    |
351 | ...ne_retirement(retirement.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained clone owner retireme...
    |                             ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
    |                                        |
    |                                        expected `RetainedCloneGrant`, found `usize`
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:8
    |
 19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    |        ^^^^^^^^^^            -----
help: remove the extra argument
    |
351 -         let step = admit_retained_clone_retirement(retirement.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained clone owner retirement")?;
351 +         let step = admit_retained_clone_retirement(retirement.close_step(/* retained_clone::RetainedCloneGrant */)?, maximum_items, maximum_bytes, "retained clone owner retirement")?;
    |

error[E0308]: `?` operator has incompatible types
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🦀️.rs:351:52
    |
351 | ...rement(retirement.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained clone owner retirement")?;
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `SnapshotRetirementStep`, found `RetainedCloneStep`
    |
    = note: `?` operator cannot convert from `retained_clone::RetainedCloneStep` to `retirement_contract::SnapshotRetirementStep`

[artifact-rust:semio-framework-artifact-flow-flow:check] running elapsedMs=10001
Some errors have detailed explanations: E0046, E0050, E0061, E0308, E0407, E0425, E0599.
For more information about an error, try `rustc --explain E0046`.
warning: `semio-framework-value` (lib) generated 20 warnings (20 duplicates)
error: could not compile `semio-framework-value` (lib) due to 22 previous errors; 20 warnings emitted
warning: build failed, waiting for other jobs to finish...
warning: `semio-framework-value` (lib) generated 20 warnings
error: could not compile `semio-framework-value` (lib) due to 22 previous errors; 20 warnings emitted
53 |         accept(1);
54 |       });
```
