# Typed Read Ownership Receiving Red 38

The exact registered literal General38 physically closed Nx/Bun 1 and Cargo 101. Selected 313 source bodies exact=true; producer exact=true. Full original owning-package all-target and original compiler/deadline/cancellation controls retained. Whole dependency closure and runtime acceptance remain unaccepted.

```text
error[E0428]: the name `ReadOwnershipRegistry` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:147:1
    |
 11 | pub struct ReadOwnershipRegistry<T:RetireOwned+Sync>{state:ManuallyDrop<Mutex<ReadOwnershipState<T>>>}
    | ---------------------------------------------------- previous definition of the type `ReadOwnershipRegistry` here
...
147 | pub struct ReadOwnershipRegistry<T:RetireOwned+Sync> {state:StateLock<T>,returned:AtomicUsize,sequence:AtomicU64,generation:AtomicU...
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ReadOwnershipRegistry` redefined here
    |
    = note: `ReadOwnershipRegistry` must be defined only once in the type namespace of this module

error[E0252]: the name `RetireOwned` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:193
error[E0252]: the name `RetireOwned` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:193
    |
  3 | ...,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::SharedControlledRetirement}};
    |                                     ----------- previous import of the trait `RetireOwned` here
...
108 | ...tainedCloneSource,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::SharedControlledRetirement}};
    |                                                      ^^^^^^^^^^^-
    |                                                      |
    |                                                      `RetireOwned` reimported here
    |                                                      help: remove unnecessary import
    |
    = note: `RetireOwned` must be defined only once in the type namespace of this module
error[E0252]: the name `RetirementCursor` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:205
    |
  3 | ...ep},retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::SharedControlledRetirement}};
    |                                 ---------------- previous import of the trait `RetirementCursor` here
...
108 | ...e,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::SharedControlledRetirement}};
    |                                                  ^^^^^^^^^^^^^^^^-
    |                                                  |
    |                                                  `RetirementCursor` reimported here
    |                                                  help: remove unnecessary import
    |
    = note: `RetirementCursor` must be defined only once in the type namespace of this module
error[E0252]: the name `RetirementStep` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:222
    |
  3 | ...::{RetireOwned,RetirementCursor,RetirementStep,shared::SharedControlledRetirement}};
    |                                    -------------- previous import of the type `RetirementStep` here
...
108 | ...eStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::SharedControlledRetirement}};
    |                                                     ^^^^^^^^^^^^^^-
    |                                                     |
    |                                                     `RetirementStep` reimported here
    |                                                     help: remove unnecessary import
    |
    = note: `RetirementStep` must be defined only once in the type namespace of this module
error[E0252]: the name `SharedControlledRetirement` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:237
    |
  3 | ...,RetirementStep,shared::SharedControlledRetirement}};
    |                    ---------------------------------- previous import of the type `SharedControlledRetirement` here
...
108 | ...,RetirementCursor,RetirementStep,shared::SharedControlledRetirement}};
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SharedControlledRetirement` reimported here
    |
    = note: `SharedControlledRetirement` must be defined only once in the type namespace of this module

error[E0252]: the name `ManuallyDrop` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:109:28
error[E0252]: the name `ManuallyDrop` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:109:28
    |
  4 | use std::{mem::{size_of,ManuallyDrop},sync::{Arc,Mutex,TryLockError}};
    |                         ------------ previous import of the type `ManuallyDrop` here
...
109 | use std::{cell::UnsafeCell,mem::ManuallyDrop,sync::{Arc,atomic::{AtomicBool,AtomicU64,AtomicUsize,Ordering}}};
    |                            ^^^^^^^^^^^^^^^^^-
    |                            |
    |                            `ManuallyDrop` reimported here
    |                            help: remove unnecessary import
    |
    = note: `ManuallyDrop` must be defined only once in the type namespace of this module
error[E0252]: the name `Arc` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:109:53
    |
  4 | use std::{mem::{size_of,ManuallyDrop},sync::{Arc,Mutex,TryLockError}};
    |                                              --- previous import of the type `Arc` here
...
109 | use std::{cell::UnsafeCell,mem::ManuallyDrop,sync::{Arc,atomic::{AtomicBool,AtomicU64,AtomicUsize,Ordering}}};
    |                                                     ^^^-
    |                                                     |
    |                                                     `Arc` reimported here
    |                                                     help: remove unnecessary import
    |
    = note: `Arc` must be defined only once in the type namespace of this module
error[E0252]: the name `ErasedSnapshotRetirement` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:13
    |
  3 | use crate::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,Retained...
    |             ------------------------ previous import of the trait `ErasedSnapshotRetirement` here
...
108 | use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedClone...
    |             ^^^^^^^^^^^^^^^^^^^^^^^^-
    |             |
    |             `ErasedSnapshotRetirement` reimported here
    |             help: remove unnecessary import
    |
    = note: `ErasedSnapshotRetirement` must be defined only once in the type namespace of this module
error[E0252]: the name `RetainedCloneSource` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:141
    |
  3 | ...rant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,share...
    |                               ------------------- previous import of the type `RetainedCloneSource` here
...
108 | ...::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,Ret...
    |                                                ^^^^^^^^^^^^^^^^^^^-
    |                                                |
    |                                                `RetainedCloneSource` reimported here
    |                                                help: remove unnecessary import
    |
    = note: `RetainedCloneSource` must be defined only once in the type namespace of this module
error[E0252]: the name `RetainedCloneGrant` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:100
    |
  3 | ...efusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep},retirement::{RetireO...
    |                                ------------------ previous import of the type `RetainedCloneGrant` here
...
108 | ...ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep},ret...
    |                                                 ^^^^^^^^^^^^^^^^^^-
    |                                                 |
    |                                                 `RetainedCloneGrant` reimported here
    |                                                 help: remove unnecessary import
    |
    = note: `RetainedCloneGrant` must be defined only once in the type namespace of this module
error[E0252]: the name `RetainedCloneProgress` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:119
    |
  3 | ...one::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,...
    |                             --------------------- previous import of the type `RetainedCloneProgress` here
...
108 | ...lKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep},retirement::{RetireOwned,...
    |                                              ^^^^^^^^^^^^^^^^^^^^^-
    |                                              |
    |                                              `RetainedCloneProgress` reimported here
    |                                              help: remove unnecessary import
    |
    = note: `RetainedCloneProgress` must be defined only once in the type namespace of this module
error[E0252]: the name `RetainedCloneStep` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:161
    |
  3 | ...Progress,RetainedCloneSource,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,shared::SharedControlle...
    |                                 ----------------- previous import of the type `RetainedCloneStep` here
...
108 | ...ant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,shared...
    |                                                  ^^^^^^^^^^^^^^^^^ `RetainedCloneStep` reimported here
    |
    = note: `RetainedCloneStep` must be defined only once in the type namespace of this module

error[E0252]: the name `ValueError` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:55
error[E0252]: the name `ValueError` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:55
    |
  3 | use crate::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,Retained...
    |                                      ---------- previous import of the type `ValueError` here
...
108 | use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedClone...
    |                                                       ^^^^^^^^^^-
    |                                                       |
    |                                                       `ValueError` reimported here
    |                                                       help: remove unnecessary import
    |
    = note: `ValueError` must be defined only once in the type namespace of this module
error[E0252]: the name `ValueRefusalKind` is defined multiple times
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:108:66
    |
  3 | use crate::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,Retained...
    |                                                 ---------------- previous import of the type `ValueRefusalKind` here
...
108 | use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedClone...
    |                                                                  ^^^^^^^^^^^^^^^^-
    |                                                                  |
    |                                                                  `ValueRefusalKind` reimported here
    |                                                                  help: remove unnecessary import
    |
    = note: `ValueRefusalKind` must be defined only once in the type namespace of this module
error[E0119]: conflicting implementations of trait `RetireOwned` for type `read::ReadOwnershipRegistry<_>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:215:1
    |
100 | impl<T:RetireOwned+Sync> RetireOwned for ReadOwnershipRegistry<T>{
    | ----------------------------------------------------------------- first implementation here
...
215 | impl<T:RetireOwned+Sync> RetireOwned for ReadOwnershipRegistry<T> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `read::ReadOwnershipRegistry<_>`

error[E0119]: conflicting implementations of trait `Drop` for type `read::ReadOwnershipRegistry<_>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:191:1
    |
105 | impl<T:RetireOwned+Sync> Drop for ReadOwnershipRegistry<T>{fn drop(&mut self){let empty=self.state.get_mut().unwrap_or_else(|error|...
error[E0119]: conflicting implementations of trait `Drop` for type `read::ReadOwnershipRegistry<_>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:191:1
    |
105 | impl<T:RetireOwned+Sync> Drop for ReadOwnershipRegistry<T>{fn drop(&mut self){let empty=self.state.get_mut().unwrap_or_else(|error|...
    | ---------------------------------------------------------- first implementation here
...
191 | impl<T:RetireOwned+Sync> Drop for ReadOwnershipRegistry<T> {fn drop(&mut self){let state=self.state.state.get_mut();assert!(std::th...
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `read::ReadOwnershipRegistry<_>`

error[E0592]: duplicate definitions with name `constructor_capacity_bytes`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:13:5
    |
 13 |     pub fn constructor_capacity_bytes()->usize{crate::retirement::shared::arc_bytes::<Self>()}
error[E0592]: duplicate definitions with name `constructor_capacity_bytes`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:13:5
    |
 13 |     pub fn constructor_capacity_bytes()->usize{crate::retirement::shared::arc_bytes::<Self>()}
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `constructor_capacity_bytes`
...
149 |     pub fn constructor_capacity_bytes()->usize {crate::retirement::shared::arc_bytes::<Self>()+size_of::<[Option<Slot<T>>;READ_LEAS...
    |     ------------------------------------------ other definition for `constructor_capacity_bytes`

error[E0592]: duplicate definitions with name `admit`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:14:5
    |
 14 |     pub fn admit(grant:RetainedCloneGrant)->Result<(Arc<Self>,RetainedCloneProgress),ValueError>{
error[E0592]: duplicate definitions with name `admit`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:14:5
    |
 14 |     pub fn admit(grant:RetainedCloneGrant)->Result<(Arc<Self>,RetainedCloneProgress),ValueError>{
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `admit`
...
150 |     pub fn admit(grant:RetainedCloneGrant)->Result<(Arc<Self>,RetainedCloneProgress),ValueError> {
    |     -------------------------------------------------------------------------------------------- other definition for `admit`

error[E0592]: duplicate definitions with name `try_issue`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:22:5
    |
 22 |     pub fn try_issue(self:&Arc<Self>,root:Arc<T>,grant:RetainedCloneGrant)->Result<(ReadCustody<T>,RetainedCloneProgress),(ValueError,Arc<T>)>{
error[E0592]: duplicate definitions with name `try_issue`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:22:5
    |
 22 |     pub fn try_issue(self:&Arc<Self>,root:Arc<T>,grant:RetainedCloneGrant)->Result<(ReadCustody<T>,RetainedCloneProgress),(ValueError,Arc<T>)>{
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `try_issue`
...
160 |     pub fn try_issue(self:&Arc<Self>,owner:Arc<T>,grant:RetainedCloneGrant)->Result<(ReadLease<T>,RetainedCloneProgress),(ValueError,Arc<T>)> {
    |     ----------------------------------------------------------------------------------------------------------------------------------------- other definition for `try_issue`

error[E0592]: duplicate definitions with name `take_returned`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:34:5
    |
 34 |     pub fn take_returned(&self,grant:RetainedCloneGrant)->Result<(Option<Arc<T>>,RetainedCloneProgress),ValueError>{
error[E0592]: duplicate definitions with name `take_returned`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:34:5
    |
 34 |     pub fn take_returned(&self,grant:RetainedCloneGrant)->Result<(Option<Arc<T>>,RetainedCloneProgress),ValueError>{
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `take_returned`
...
173 |     pub fn take_returned(&self,grant:RetainedCloneGrant)->Result<(Option<Arc<T>>,RetainedCloneProgress),ValueError> {
    |     --------------------------------------------------------------------------------------------------------------- other definition for `take_returned`

error[E0277]: `?` couldn't convert the error to `refusal::ValueError`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:246:117
    |
246 | ...as_ref().unwrap();let mut state=registry.state.try_lock()?;if !state.contains(self.id){return Err(ValueError::literal(ValueRefus...
error[E0277]: `?` couldn't convert the error to `refusal::ValueError`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:246:117
    |
246 | ...as_ref().unwrap();let mut state=registry.state.try_lock()?;if !state.contains(self.id){return Err(ValueError::literal(ValueRefus...
    |                                                   ----------^ the trait `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>` is not implemented for `refusal::ValueError`
    |                                                   |
    |                                                   this can't be annotated with `?` because it has type `Result<_, std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>`
    |
note: `refusal::ValueError` needs to implement `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../⚠️refusal/🦀️.rs:14:1
    |
 14 | pub struct ValueError { pub kind: ValueRefusalKind, pub message: std::borrow::Cow<'static, str> }
    | ^^^^^^^^^^^^^^^^^^^^^
error[E0599]: no method named `contains` found for struct `std::sync::MutexGuard<'_, ReadOwnershipState<T>>` in the current scope
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:246:129
    |
246 | ...e.try_lock()?;if !state.contains(self.id){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"read generation lo...
    |                            ^^^^^^^^ method not found in `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following trait defines an item `contains`, perhaps you need to implement it:
            candidate #1: `RangeBounds`
help: some of the expressions' fields have a method of the same name
    |
246 |         if self.owner.is_some(){let registry=self.registry.as_ref().unwrap();let mut state=registry.state.try_lock()?;if !state.issued.contains(self.id){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"read generation lost original registered root"));}let slot=state.slots.as_mut().unwrap()[self.id.index as usize].as_mut().unwrap();assert!(Arc::ptr_eq(&slot.owner,self.owner.as_ref().unwrap()));drop(self.owner.take());slot.returned=true;registry.returned.fetch_add(1,Ordering::AcqRel);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}
    |                                                                                                                                 +++++++
error[E0609]: no field `slots` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:246:280
    |
246 | ...tion lost original registered root"));}let slot=state.slots.as_mut().unwrap()[self.id.index as usize].as_mut().unwrap();assert!(...
    |                                                          ^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `returned` on type `&Arc<read::ReadOwnershipRegistry<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:246:461
    |
246 | ...;drop(self.owner.take());slot.returned=true;registry.returned.fetch_add(1,Ordering::AcqRel);return Ok(RetainedCloneStep::Progres...
    |                                                         ^^^^^^^^ unknown field
error[E0609]: no field `returned` on type `&Arc<read::ReadOwnershipRegistry<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:246:461
    |
246 | ...;drop(self.owner.take());slot.returned=true;registry.returned.fetch_add(1,Ordering::AcqRel);return Ok(RetainedCloneStep::Progres...
    |                                                         ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `state` on type `ManuallyDrop<std::sync::Mutex<ReadOwnershipState<T>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:206:89
    |
206 |     fn terminal_is_empty(&self)->bool {self.active.is_none()&&unsafe{&*self.owner.state.state.get()}.slots.is_none()}
    |                                                                                         ^^^^^ unknown field
error[E0609]: no field `state` on type `ManuallyDrop<std::sync::Mutex<ReadOwnershipState<T>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:206:89
    |
206 |     fn terminal_is_empty(&self)->bool {self.active.is_none()&&unsafe{&*self.owner.state.state.get()}.slots.is_none()}
    |                                                                                         ^^^^^ unknown field

error[E0609]: no field `state` on type `ManuallyDrop<std::sync::Mutex<ReadOwnershipState<T>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:209:123
    |
209 | ...ref().map_or_else(||Some(if unsafe{&*self.owner.state.state.get()}.free_len==READ_LEASE_CAPACITY&&!self.terminal_is_empty(){size...
    |                                                          ^^^^^ unknown field

error[E0034]: multiple applicable items in scope
error[E0609]: no field `state` on type `ManuallyDrop<std::sync::Mutex<ReadOwnershipState<T>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:209:123
    |
209 | ...ref().map_or_else(||Some(if unsafe{&*self.owner.state.state.get()}.free_len==READ_LEASE_CAPACITY&&!self.terminal_is_empty(){size...
    |                                                          ^^^^^ unknown field

error[E0034]: multiple applicable items in scope
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:154:25
    |
154 | ...   let bytes=Self::constructor_capacity_bytes();if bytes>grant.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusa...
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ multiple `constructor_capacity_bytes` found
    |
note: candidate #1 is defined in an impl for the type `read::ReadOwnershipRegistry<T>`
error[E0034]: multiple applicable items in scope
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:154:25
    |
154 | ...   let bytes=Self::constructor_capacity_bytes();if bytes>grant.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusa...
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ multiple `constructor_capacity_bytes` found
    |
note: candidate #1 is defined in an impl for the type `read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:13:5
    |
 13 |     pub fn constructor_capacity_bytes()->usize{crate::retirement::shared::arc_bytes::<Self>()}
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in an impl for the type `read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:149:5
error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:157:40
    |
157 | ...elf {state:StateLock {held:AtomicBool::new(false),poisoned:AtomicBool::new(false),state:UnsafeCell::new(state)},returned:AtomicU...
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `ManuallyDrop<Mutex<_>>`, found `StateLock<_>`
    |
    = note: expected struct `ManuallyDrop<std::sync::Mutex<ReadOwnershipState<T>>>`
               found struct `StateLock<_>`

error[E0560]: struct `read::ReadOwnershipRegistry<T>` has no field named `returned`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:157:141
    |
157 | ...,state:UnsafeCell::new(state)},returned:AtomicUsize::new(0),sequence:AtomicU64::new(0),generation:AtomicU64::new(0),revision:std...
error[E0560]: struct `read::ReadOwnershipRegistry<T>` has no field named `returned`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:157:141
    |
157 | ...,state:UnsafeCell::new(state)},returned:AtomicUsize::new(0),sequence:AtomicU64::new(0),generation:AtomicU64::new(0),revision:std...
    |                                   ^^^^^^^^ `read::ReadOwnershipRegistry<T>` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `read::ReadOwnershipRegistry<T>` has no field named `sequence`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:157:170
    |
157 | ...},returned:AtomicUsize::new(0),sequence:AtomicU64::new(0),generation:AtomicU64::new(0),revision:std::array::from_fn(|_|AtomicU64...
    |                                   ^^^^^^^^ `read::ReadOwnershipRegistry<T>` does not have this field
error[E0560]: struct `read::ReadOwnershipRegistry<T>` has no field named `sequence`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:157:170
    |
157 | ...},returned:AtomicUsize::new(0),sequence:AtomicU64::new(0),generation:AtomicU64::new(0),revision:std::array::from_fn(|_|AtomicU64...
    |                                   ^^^^^^^^ `read::ReadOwnershipRegistry<T>` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `read::ReadOwnershipRegistry<T>` has no field named `generation`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:157:197
    |
157 | ...0),sequence:AtomicU64::new(0),generation:AtomicU64::new(0),revision:std::array::from_fn(|_|AtomicU64::new(0))});
    |                                  ^^^^^^^^^^ `read::ReadOwnershipRegistry<T>` does not have this field
error[E0560]: struct `read::ReadOwnershipRegistry<T>` has no field named `generation`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:157:197
    |
157 | ...0),sequence:AtomicU64::new(0),generation:AtomicU64::new(0),revision:std::array::from_fn(|_|AtomicU64::new(0))});
    |                                  ^^^^^^^^^^ `read::ReadOwnershipRegistry<T>` does not have this field
    |
    = note: all struct fields are already assigned

error[E0560]: struct `read::ReadOwnershipRegistry<T>` has no field named `revision`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:157:226
    |
157 | ...),generation:AtomicU64::new(0),revision:std::array::from_fn(|_|AtomicU64::new(0))});
    |                                   ^^^^^^^^ `read::ReadOwnershipRegistry<T>` does not have this field
error[E0560]: struct `read::ReadOwnershipRegistry<T>` has no field named `revision`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:157:226
    |
157 | ...),generation:AtomicU64::new(0),revision:std::array::from_fn(|_|AtomicU64::new(0))});
    |                                   ^^^^^^^^ `read::ReadOwnershipRegistry<T>` does not have this field
    |
    = note: all struct fields are already assigned

error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:163:92
    |
163 |         let mut state=match self.state.try_lock(){Ok(state)=>state,Err(error)=>return Err((error,owner))};
    |                                                                                            ^^^^^ expected `ValueError`, found `TryLockError<MutexGuard<'_, _>>`
error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:163:92
    |
163 |         let mut state=match self.state.try_lock(){Ok(state)=>state,Err(error)=>return Err((error,owner))};
    |                                                                                            ^^^^^ expected `ValueError`, found `TryLockError<MutexGuard<'_, _>>`
    |
    = note: expected struct `refusal::ValueError`
                 found enum `std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>`

error[E0609]: no field `free_len` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:164:18
    |
164 | ...   if state.free_len==0||state.slots.is_none(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"read registry h...
error[E0609]: no field `free_len` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:164:18
    |
164 | ...   if state.free_len==0||state.slots.is_none(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"read registry h...
    |                ^^^^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `slots` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:164:37
    |
164 | ...   if state.free_len==0||state.slots.is_none(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"read registry h...
    |                                   ^^^^^ unknown field
error[E0609]: no field `slots` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:164:37
    |
164 | ...   if state.free_len==0||state.slots.is_none(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"read registry h...
    |                                   ^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `generation` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:165:36
    |
165 | ...   let Some(generation)=state.generation.checked_add(1)else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,...
    |                                  ^^^^^^^^^^ unknown field
error[E0609]: no field `generation` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:165:36
    |
165 | ...   let Some(generation)=state.generation.checked_add(1)else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,...
    |                                  ^^^^^^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `free` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:25
    |
166 | ...   let index=state.free[state.free_read]as usize;state.free_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state...
    |                       ^^^^ unknown field
error[E0609]: no field `free` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:25
    |
166 | ...   let index=state.free[state.free_read]as usize;state.free_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state...
    |                       ^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `free_read` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:36
    |
166 | ...   let index=state.free[state.free_read]as usize;state.free_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state...
    |                                  ^^^^^^^^^ unknown field
error[E0609]: no field `free_read` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:36
    |
166 | ...   let index=state.free[state.free_read]as usize;state.free_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state...
    |                                  ^^^^^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `free_read` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:61
    |
166 | ...   let index=state.free[state.free_read]as usize;state.free_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state...
    |                                                           ^^^^^^^^^ unknown field
error[E0609]: no field `free_read` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:61
    |
166 | ...   let index=state.free[state.free_read]as usize;state.free_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state...
    |                                                           ^^^^^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `free_read` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:78
    |
166 | ...   let index=state.free[state.free_read]as usize;state.free_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state...
    |                                                                            ^^^^^^^^^ unknown field
error[E0609]: no field `free_read` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:78
    |
166 | ...   let index=state.free[state.free_read]as usize;state.free_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state...
    |                                                                            ^^^^^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `free_len` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:117
    |
166 | ...e_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state.generation=generation;
    |                                                         ^^^^^^^^ unknown field
error[E0609]: no field `free_len` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:117
    |
166 | ...e_read=(state.free_read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state.generation=generation;
    |                                                         ^^^^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `generation` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:135
    |
166 | ...read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state.generation=generation;
    |                                                        ^^^^^^^^^^ unknown field
error[E0609]: no field `generation` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:166:135
    |
166 | ...read+1)%READ_LEASE_CAPACITY;state.free_len-=1;state.generation=generation;
    |                                                        ^^^^^^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `slots` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:167:44
    |
167 | ...   let alias=Arc::clone(&owner);state.slots.as_mut().unwrap()[index]=Some(Slot {generation,owner,returned:false});state.occupied...
    |                                          ^^^^^ unknown field
error[E0609]: no field `slots` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:167:44
    |
167 | ...   let alias=Arc::clone(&owner);state.slots.as_mut().unwrap()[index]=Some(Slot {generation,owner,returned:false});state.occupied...
    |                                          ^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `occupied` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:167:126
    |
167 | ...]=Some(Slot {generation,owner,returned:false});state.occupied[index/64]|=1u64<<(index%64);
    |                                                         ^^^^^^^^ unknown field
error[E0609]: no field `occupied` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:167:126
    |
167 | ...]=Some(Slot {generation,owner,returned:false});state.occupied[index/64]|=1u64<<(index%64);
    |                                                         ^^^^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0277]: `?` couldn't convert the error to `refusal::ValueError`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:170:93
    |
170 |     pub fn contains(&self,id:ReadLeaseId)->Result<bool,ValueError> {Ok(self.state.try_lock()?.contains(id))}
    |                                                                                   ----------^ the trait `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>` is not implemented for `refusal::ValueError`
error[E0277]: `?` couldn't convert the error to `refusal::ValueError`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:170:93
    |
170 |     pub fn contains(&self,id:ReadLeaseId)->Result<bool,ValueError> {Ok(self.state.try_lock()?.contains(id))}
    |                                                                                   ----------^ the trait `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>` is not implemented for `refusal::ValueError`
    |                                                                                   |
    |                                                                                   this can't be annotated with `?` because it has type `Result<_, std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>`
    |
note: `refusal::ValueError` needs to implement `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../⚠️refusal/🦀️.rs:14:1
    |
 14 | pub struct ValueError { pub kind: ValueRefusalKind, pub message: std::borrow::Cow<'static, str> }
    | ^^^^^^^^^^^^^^^^^^^^^
error[E0599]: no method named `contains` found for struct `std::sync::MutexGuard<'_, ReadOwnershipState<T>>` in the current scope
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:170:95
    |
170 |     pub fn contains(&self,id:ReadLeaseId)->Result<bool,ValueError> {Ok(self.state.try_lock()?.contains(id))}
    |                                                                                               ^^^^^^^^ method not found in `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following trait defines an item `contains`, perhaps you need to implement it:
            candidate #1: `RangeBounds`
help: some of the expressions' fields have a method of the same name
    |
170 |     pub fn contains(&self,id:ReadLeaseId)->Result<bool,ValueError> {Ok(self.state.try_lock()?.issued.contains(id))}
    |                                                                                               +++++++
error[E0277]: `?` couldn't convert the error to `refusal::ValueError`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:171:105
    |
171 |     pub fn occupied_count(&self)->Result<usize,ValueError> {Ok(READ_LEASE_CAPACITY-self.state.try_lock()?.free_len)}
    |                                                                                               ----------^ the trait `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>` is not implemented for `refusal::ValueError`
    |                                                                                               |
    |                                                                                               this can't be annotated with `?` because it has type `Result<_, std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>`
    |
note: `refusal::ValueError` needs to implement `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../⚠️refusal/🦀️.rs:14:1
    |
 14 | pub struct ValueError { pub kind: ValueRefusalKind, pub message: std::borrow::Cow<'static, str> }
    | ^^^^^^^^^^^^^^^^^^^^^
error[E0609]: no field `free_len` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:171:107
    |
171 |     pub fn occupied_count(&self)->Result<usize,ValueError> {Ok(READ_LEASE_CAPACITY-self.state.try_lock()?.free_len)}
    |                                                                                                           ^^^^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0609]: no field `returned` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:172:44
    |
172 |     pub fn has_returned(&self)->bool {self.returned.load(Ordering::Acquire)!=0}
    |                                            ^^^^^^^^ unknown field
error[E0609]: no field `returned` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:172:44
    |
172 |     pub fn has_returned(&self)->bool {self.returned.load(Ordering::Acquire)!=0}
    |                                            ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0277]: `?` couldn't convert the error to `refusal::ValueError`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:176:44
    |
176 |         let mut state=self.state.try_lock()?;let Some(index)=state.next_index()else{return Ok((None,Default::default()));};state.cu...
    |                                  ----------^ the trait `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>` is not implemented for `refusal::ValueError`
error[E0277]: `?` couldn't convert the error to `refusal::ValueError`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:176:44
    |
176 |         let mut state=self.state.try_lock()?;let Some(index)=state.next_index()else{return Ok((None,Default::default()));};state.cu...
    |                                  ----------^ the trait `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>` is not implemented for `refusal::ValueError`
    |                                  |
    |                                  this can't be annotated with `?` because it has type `Result<_, std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>`
    |
note: `refusal::ValueError` needs to implement `From<std::sync::TryLockError<std::sync::MutexGuard<'_, ReadOwnershipState<T>>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../⚠️refusal/🦀️.rs:14:1
    |
 14 | pub struct ValueError { pub kind: ValueRefusalKind, pub message: std::borrow::Cow<'static, str> }
    | ^^^^^^^^^^^^^^^^^^^^^
error[E0599]: no method named `next_index` found for struct `std::sync::MutexGuard<'_, ReadOwnershipState<T>>` in the current scope
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:176:68
    |
176 | ...;let Some(index)=state.next_index()else{return Ok((None,Default::default()));};state.cursor=(index+1)%READ_LEASE_CAPACITY;
    |                           ^^^^^^^^^^ method not found in `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`

error[E0609]: no field `slots` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:177:28
    |
177 | ...   let owner=if state.slots.as_ref().unwrap()[index].as_ref().unwrap().returned {let slot=state.take(index);self.returned.fetch_...
    |                          ^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`
error[E0609]: no field `slots` on type `std::sync::MutexGuard<'_, ReadOwnershipState<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:177:28
    |
177 | ...   let owner=if state.slots.as_ref().unwrap()[index].as_ref().unwrap().returned {let slot=state.take(index);self.returned.fetch_...
    |                          ^^^^^ unknown field
    |
    = note: available fields are: `roots`, `issued`, `cursor`

error[E0599]: `std::sync::MutexGuard<'_, ReadOwnershipState<T>>` is not an iterator
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:177:102
    |
  7 | struct ReadOwnershipState<T>{roots:[Option<Arc<T>>;READ_OWNERSHIP_CAPACITY],issued:[bool;READ_OWNERSHIP_CAPACITY],cursor:usize}
    | ---------------------------- doesn't satisfy `ReadOwnershipState<T>: Iterator`
error[E0599]: `std::sync::MutexGuard<'_, ReadOwnershipState<T>>` is not an iterator
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:177:102
    |
  7 | struct ReadOwnershipState<T>{roots:[Option<Arc<T>>;READ_OWNERSHIP_CAPACITY],issued:[bool;READ_OWNERSHIP_CAPACITY],cursor:usize}
    | ---------------------------- doesn't satisfy `ReadOwnershipState<T>: Iterator`
...
177 |         let owner=if state.slots.as_ref().unwrap()[index].as_ref().unwrap().returned {let slot=state.take(index);self.returned.fetc...
    |                                                                                                      ^^^^ `std::sync::MutexGuard<'_, ReadOwnershipState<T>>` is not an iterator
    |
   ::: /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sync/poison/mutex.rs:277:1
    |
277 | pub struct MutexGuard<'a, T: ?Sized + 'a> {
    | ----------------------------------------- doesn't satisfy `_: Iterator`
error[E0609]: no field `returned` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:177:119
    |
177 | ...).unwrap().returned {let slot=state.take(index);self.returned.fetch_sub(1,Ordering::AcqRel);Some(slot.owner)}else{None};
    |                                                         ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:181:27
    |
181 | ...   let sequence=self.sequence.load(Ordering::Acquire);let Some(terminal)=sequence.checked_add(2)else{return false;};if sequence&...
    |                         ^^^^^^^^ unknown field
error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:181:27
    |
181 | ...   let sequence=self.sequence.load(Ordering::Acquire);let Some(terminal)=sequence.checked_add(2)else{return false;};if sequence&...
    |                         ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:181:145
    |
181 | ...ked_add(2)else{return false;};if sequence&1!=0||self.sequence.compare_exchange(sequence,sequence+1,Ordering::AcqRel,Ordering::Ac...
    |                                                         ^^^^^^^^ unknown field
error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:181:145
    |
181 | ...ked_add(2)else{return false;};if sequence&1!=0||self.sequence.compare_exchange(sequence,sequence+1,Ordering::AcqRel,Ordering::Ac...
    |                                                         ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `generation` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:182:14
    |
182 | ...   self.generation.store(generation,Ordering::Relaxed);for(index,word)in self.revision.iter().enumerate(){word.store(u64::from_l...
    |            ^^^^^^^^^^ unknown field
error[E0609]: no field `generation` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:182:14
    |
182 | ...   self.generation.store(generation,Ordering::Relaxed);for(index,word)in self.revision.iter().enumerate(){word.store(u64::from_l...
    |            ^^^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `revision` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:182:84
    |
182 | ...   self.generation.store(generation,Ordering::Relaxed);for(index,word)in self.revision.iter().enumerate(){word.store(u64::from_l...
    |                                                                                  ^^^^^^^^ unknown field
error[E0609]: no field `revision` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:182:84
    |
182 | ...   self.generation.store(generation,Ordering::Relaxed);for(index,word)in self.revision.iter().enumerate(){word.store(u64::from_l...
    |                                                                                  ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:182:217
    |
182 | ...x*8+8].try_into().unwrap()),Ordering::Relaxed);}self.sequence.store(terminal,Ordering::Release);true
    |                                                         ^^^^^^^^ unknown field
error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:182:217
    |
182 | ...x*8+8].try_into().unwrap()),Ordering::Relaxed);}self.sequence.store(terminal,Ordering::Release);true
    |                                                         ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:185:27
    |
185 | ...   let sequence=self.sequence.load(Ordering::Acquire);if sequence==0||sequence&1!=0||self.generation.load(Ordering::Relaxed)!=ge...
    |                         ^^^^^^^^ unknown field
error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:185:27
    |
185 | ...   let sequence=self.sequence.load(Ordering::Acquire);if sequence==0||sequence&1!=0||self.generation.load(Ordering::Relaxed)!=ge...
    |                         ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `generation` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:185:96
    |
185 | ...   let sequence=self.sequence.load(Ordering::Acquire);if sequence==0||sequence&1!=0||self.generation.load(Ordering::Relaxed)!=ge...
    |                                                                                              ^^^^^^^^^^ unknown field
error[E0609]: no field `generation` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:185:96
    |
185 | ...   let sequence=self.sequence.load(Ordering::Acquire);if sequence==0||sequence&1!=0||self.generation.load(Ordering::Relaxed)!=ge...
    |                                                                                              ^^^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `revision` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:186:32
    |
186 | ...   for(index,word)in self.revision.iter().enumerate(){if word.load(Ordering::Relaxed)!=u64::from_le_bytes(revision[index*8..inde...
    |                              ^^^^^^^^ unknown field
error[E0609]: no field `revision` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:186:32
    |
186 | ...   for(index,word)in self.revision.iter().enumerate(){if word.load(Ordering::Relaxed)!=u64::from_le_bytes(revision[index*8..inde...
    |                              ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:186:192
    |
186 | ...].try_into().unwrap()){return false;}}sequence==self.sequence.load(Ordering::Acquire)
    |                                                         ^^^^^^^^ unknown field
error[E0609]: no field `sequence` on type `&read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:186:192
    |
186 | ...].try_into().unwrap()){return false;}}sequence==self.sequence.load(Ordering::Acquire)
    |                                                         ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0609]: no field `state` on type `ManuallyDrop<std::sync::Mutex<ReadOwnershipState<T>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:191:101
    |
191 | impl<T:RetireOwned+Sync> Drop for ReadOwnershipRegistry<T> {fn drop(&mut self){let state=self.state.state.get_mut();assert!(std::th...
    |                                                                                                     ^^^^^ unknown field
error[E0609]: no field `state` on type `ManuallyDrop<std::sync::Mutex<ReadOwnershipState<T>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:191:101
    |
191 | impl<T:RetireOwned+Sync> Drop for ReadOwnershipRegistry<T> {fn drop(&mut self){let state=self.state.state.get_mut();assert!(std::th...
    |                                                                                                     ^^^^^ unknown field

error[E0609]: no field `state` on type `ManuallyDrop<std::sync::Mutex<ReadOwnershipState<T>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:202:36
    |
202 |         let state=self.owner.state.state.get_mut();
    |                                    ^^^^^ unknown field

error[E0609]: no field `returned` on type `ManuallyDrop<read::ReadOwnershipRegistry<T>>`
error[E0609]: no field `state` on type `ManuallyDrop<std::sync::Mutex<ReadOwnershipState<T>>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:202:36
    |
202 |         let state=self.owner.state.state.get_mut();
    |                                    ^^^^^ unknown field

error[E0609]: no field `returned` on type `ManuallyDrop<read::ReadOwnershipRegistry<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:203:102
    |
203 | ...   if let Some(index)=state.next_index(){let slot=state.take(index);if slot.returned{self.owner.returned.fetch_sub(1,Ordering::A...
    |                                                                                                    ^^^^^^^^ unknown field
    |
    = note: available field is: `state`
error[E0609]: no field `returned` on type `ManuallyDrop<read::ReadOwnershipRegistry<T>>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:203:102
    |
203 | ...   if let Some(index)=state.next_index(){let slot=state.take(index);if slot.returned{self.owner.returned.fetch_sub(1,Ordering::A...
    |                                                                                                    ^^^^^^^^ unknown field
    |
    = note: available field is: `state`

error[E0034]: multiple applicable items in scope
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:18:25
    |
 18 |         let bytes=Self::constructor_capacity_bytes();
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ multiple `constructor_capacity_bytes` found
error[E0034]: multiple applicable items in scope
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:18:25
    |
 18 |         let bytes=Self::constructor_capacity_bytes();
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ multiple `constructor_capacity_bytes` found
    |
note: candidate #1 is defined in an impl for the type `read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:13:5
    |
 13 |     pub fn constructor_capacity_bytes()->usize{crate::retirement::shared::arc_bytes::<Self>()}
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in an impl for the type `read::ReadOwnershipRegistry<T>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🦀️.rs:149:5
error: could not compile `semio-framework-value` (lib) due to 67 previous errors; 46 warnings emitted

```
