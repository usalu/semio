# Current Root Native Dependency Receipt

Architect 47474: terminal exit 1; eight Value compilation errors; no artifact test runtime.
Sourcing 30381: terminal exit 1; nine Value compilation errors; no artifact test runtime.

The active independent retirement interface migration is retained. No obsolete methods or compatibility facade was restored by this ticket.

## architect-native-csv-current2.log

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:108:5
    |
108 |       fn next_close_byte_demand(&self) -> usize {
    |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
    |  _____|
    | |
109 | |         self.retirement.as_ref().map_or_else(|| self.pending.as_ref().map_or(1, |pending| pending.capacity().max(1)), |retirement...
110 | |     }
    | |_____^ not a member of trait `ErasedSnapshotRetirement`
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:15:5
   |
15 |     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
   |     not a member of trait `ErasedSnapshotRetirement`
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:65:5
   |
65 |     fn next_close_byte_demand(&self)->usize{if self.bytes.len()!=0{1}else{self.bytes.next_release_allocation_bytes().unwrap_or(0)}}
   |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
   |     not a member of trait `ErasedSnapshotRetirement`

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:106:88
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

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:114:58
    |
114 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
114 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
114 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,Retirement<V>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:119:58
    |
119 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,SharedRoot<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
119 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,SharedRoot<V>>>())}
119 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,SharedRoot<V>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:124:58
    |
124 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<(),Retirement<()>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
124 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<(),Retirement<()>>>())}
124 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<(),Retirement<()>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:129:58
    |
129 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
129 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
129 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,Retirement<V>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:134:58
    |
134 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,UpdateCursor<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
134 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,UpdateCursor<V>>>())}
134 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,UpdateCursor<V>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:139:58
    |
139 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,LookupCursor<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
139 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,LookupCursor<V>>>())}
139 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,LookupCursor<V>>>())}
    |

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:4:119
  |
4 | ...ogress, RetainedCloneStep}, ErasedSnapshotRetirement, SnapshotRetirementStep, ValueError, ValueRefusalKind};
  |                                                          ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:64:21
   |
64 |         let bytes = std::mem::size_of::<String>();
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
64 -         let bytes = std::mem::size_of::<String>();
64 +         let bytes = size_of::<String>();
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:83:40
   |
83 |         Ok(if self.pending.is_some() { std::mem::size_of::<crate::retirement::controlled::ControlledRetirement<String>>() } else { 0 })
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
83 -         Ok(if self.pending.is_some() { std::mem::size_of::<crate::retirement::controlled::ControlledRetirement<String>>() } else { 0 })
83 +         Ok(if self.pending.is_some() { size_of::<crate::retirement::controlled::ControlledRetirement<String>>() } else { 0 })
   |

warning: unused imports: `SnapshotRetirementStep` and `ValueError`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:2:38
  |
2 | use crate::{ErasedSnapshotRetirement,SnapshotRetirementStep,ValueError};
  |                                      ^^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^^

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:15:105
   |
15 |     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**own...
   |                                                                                                         ^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
15 -     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
15 +     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🦀️.rs:24:148
   |
24 | ...l FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn crate::ErasedSnapshotRetirement>>))->Result<T,E>{
   |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
24 -     pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn crate::ErasedSnapshotRetirement>>))->Result<T,E>{
24 +     pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🦀️.rs:75:108
   |
75 |     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError> { let bytes=count.checked_mul(std::mem::size_of::<T>())...
   |                                                                                                            ^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
75 -     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError> { let bytes=count.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native decoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native decoding collection allocation failed"))?;Ok(output) }
75 +     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError> { let bytes=count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native decoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native decoding collection allocation failed"))?;Ok(output) }
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛫️encode/🦀️.rs:51:106
   |
51 |     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(std::mem::size_of::<T>()).f...
   |                                                                                                          ^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
51 -     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native encoding collection allocation failed"))?;Ok(output)}
51 +     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native encoding collection allocation failed"))?;Ok(output)}
   |

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:2:86
  |
2 | use crate::{NativeDecodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,SnapshotRetirementStep,list::{PagedList,PagedLis...
  |                                                                                      ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:3:103
  |
3 | use crate::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};
  |                                                                                                       ^^^^^^^^^^^^^^^^^^^^^^

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/📋️queue/🦀️.rs:75:59
   |
75 |     fn retirement_birth_bytes(&self)->Option<usize> {Some(std::mem::size_of::<Self>())}
   |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
75 -     fn retirement_birth_bytes(&self)->Option<usize> {Some(std::mem::size_of::<Self>())}
75 +     fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/📋️queue/🦀️.rs:94:89
   |
94 |     fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
   |                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
94 -     fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
94 +     fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:54:83
   |
54 | pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {std::mem::size_of::<FactoryTicket<T>>()}
   |                                                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
54 - pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {std::mem::size_of::<FactoryTicket<T>>()}
54 + pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {size_of::<FactoryTicket<T>>()}
   |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:100:77
    |
100 |     if ticket.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:std::mem::size_of_val(ticket.as_ref()),depth:1,..Defaul...
    |                                                                             ^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
100 -     if ticket.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:std::mem::size_of_val(ticket.as_ref()),depth:1,..Default::default()});}
100 +     if ticket.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:size_of_val(ticket.as_ref()),depth:1,..Default::default()});}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:111:19
    |
111 |         let bytes=std::mem::size_of_val(ticket.as_ref());
    |                   ^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
111 -         let bytes=std::mem::size_of_val(ticket.as_ref());
111 +         let bytes=size_of_val(ticket.as_ref());
    |

warning: unused imports: `admit_retained_clone_retirement` and `admit_retained_clone_scaffold_retirement`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🗺️ordered-map/🦀️.rs:3:230
  |
3 | ...it_retained_clone_progress, admit_retained_clone_retirement, admit_retained_clone_scaffold_retirement, close_retained_binding};
  |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🗺️ordered-map/🦀️.rs:4:13
  |
4 | use crate::{SnapshotRetirementStep, retirement::RetireOwned};
  |             ^^^^^^^^^^^^^^^^^^^^^^

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🗺️ordered-map/🦀️.rs:130:24
    |
130 | ...   let progress = super::admit_retained_clone_close(grant, step, terminal_is_empty, &format!("retained ordered-map {label} scaff...
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
130 -         let progress = super::admit_retained_clone_close(grant, step, terminal_is_empty, &format!("retained ordered-map {label} scaffold close"))?.progress();
130 +         let progress = admit_retained_clone_close(grant, step, terminal_is_empty, &format!("retained ordered-map {label} scaffold close"))?.progress();
    |

warning: unused import: `admit_retained_clone_retirement`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/📦️paged/🦀️.rs:3:202
  |
3 | ...etainedCloneStep, admit_retained_clone_progress, admit_retained_clone_retirement};
  |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/📦️paged/🦀️.rs:5:5
  |
5 |     SnapshotRetirementStep, ValueError, ValueRefusalKind,
  |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `admit_retained_clone_retirement`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/📋️paged-list/🦀️.rs:3:226
  |
3 | ...etainedCloneStep, admit_retained_clone_progress, admit_retained_clone_retirement};
  |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/📋️paged-list/🦀️.rs:6:5
  |
6 |     SnapshotRetirementStep,
  |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🔗️projection/🦀️.rs:4:13
  |
4 | use crate::{SnapshotRetirementStep, ValueError, ValueRefusalKind};
  |             ^^^^^^^^^^^^^^^^^^^^^^
error[E0046]: not all trait items implemented, missing: `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:62:1
   |
62 | impl<const N:usize> ErasedSnapshotRetirement for PagedText<N>{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
   |
  ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:5
   |
19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
   |     --------------------------------------------------------------------------------------------- `close_step` from trait
20 |     fn terminal_is_empty(&self) -> bool;
21 |     fn next_copy_byte_demand(&self) -> usize;
   |     ----------------------------------------- `next_copy_byte_demand` from trait
22 |     fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
   |     -------------------------------------------------------------------------------------------- `next_capacity_byte_demand` from trait
23 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
   |     ---------------------------------------------------------------- `next_release_byte_demand` from trait
24 |     fn next_depth_demand(&self) -> Result<usize, ValueError>;
   |     --------------------------------------------------------- `next_depth_demand` from trait
error[E0046]: not all trait items implemented, missing: `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:12:1
   |
12 | impl ErasedSnapshotRetirement for NativeDecodeRetirementRecipient{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
   |
  ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:5
   |
19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
   |     --------------------------------------------------------------------------------------------- `close_step` from trait
20 |     fn terminal_is_empty(&self) -> bool;
21 |     fn next_copy_byte_demand(&self) -> usize;
   |     ----------------------------------------- `next_copy_byte_demand` from trait
22 |     fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
   |     -------------------------------------------------------------------------------------------- `next_capacity_byte_demand` from trait
23 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
   |     ---------------------------------------------------------------- `next_release_byte_demand` from trait
24 |     fn next_depth_demand(&self) -> Result<usize, ValueError>;
   |     --------------------------------------------------------- `next_depth_demand` from trait
error[E0046]: not all trait items implemented, missing: `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:105:1
    |
105 | impl ErasedSnapshotRetirement for PagedUtf8AppendCursor {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:5
    |
 19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    |     --------------------------------------------------------------------------------------------- `close_step` from trait
 20 |     fn terminal_is_empty(&self) -> bool;
 21 |     fn next_copy_byte_demand(&self) -> usize;
    |     ----------------------------------------- `next_copy_byte_demand` from trait
 22 |     fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    |     -------------------------------------------------------------------------------------------- `next_capacity_byte_demand` from trait
 23 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
    |     ---------------------------------------------------------------- `next_release_byte_demand` from trait
 24 |     fn next_depth_demand(&self) -> Result<usize, ValueError>;
    |     --------------------------------------------------------- `next_depth_demand` from trait
error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn retirement_contract::ErasedSnapshotRetirement>` in the current scope
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:15:148
   |
15 | ...s_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `RetirementCursor` defines an item `next_close_byte_demand`, perhaps you need to implement it
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:25:1
   |
25 | pub trait RetirementCursor: Send {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: there is a method `next_copy_byte_demand` with a similar name
   |
15 -     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
15 +     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_copy_byte_demand()}).unwrap_or(0)}
   |
error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn retirement_contract::ErasedSnapshotRetirement>` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:109:143
    |
109 | ... pending.capacity().max(1)), |retirement| retirement.next_close_byte_demand())
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `RetirementCursor` defines an item `next_close_byte_demand`, perhaps you need to implement it
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:25:1
    |
 25 | pub trait RetirementCursor: Send {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: there is a method `next_copy_byte_demand` with a similar name
    |
109 -         self.retirement.as_ref().map_or_else(|| self.pending.as_ref().map_or(1, |pending| pending.capacity().max(1)), |retirement| retirement.next_close_byte_demand())
109 +         self.retirement.as_ref().map_or_else(|| self.pending.as_ref().map_or(1, |pending| pending.capacity().max(1)), |retirement| retirement.next_copy_byte_demand())
    |

Some errors have detailed explanations: E0046, E0407, E0599.
For more information about an error, try `rustc --explain E0046`.
warning: `semio-framework-value` (lib) generated 30 warnings
```

## sourcing-parent-diff-current2.log

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:108:5
    |
108 |       fn next_close_byte_demand(&self) -> usize {
    |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
    |  _____|
    | |
109 | |         self.retirement.as_ref().map_or_else(|| self.pending.as_ref().map_or(1, |pending| pending.capacity().max(1)), |retirement...
110 | |     }
    | |_____^ not a member of trait `ErasedSnapshotRetirement`
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:15:5
   |
15 |     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
   |     not a member of trait `ErasedSnapshotRetirement`
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:65:5
   |
65 |     fn next_close_byte_demand(&self)->usize{if self.bytes.len()!=0{1}else{self.bytes.next_release_allocation_bytes().unwrap_or(0)}}
   |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
   |     not a member of trait `ErasedSnapshotRetirement`

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:106:88
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

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:114:58
    |
114 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
114 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
114 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,Retirement<V>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:119:58
    |
119 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,SharedRoot<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
119 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,SharedRoot<V>>>())}
119 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,SharedRoot<V>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:124:58
    |
124 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<(),Retirement<()>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
124 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<(),Retirement<()>>>())}
124 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<(),Retirement<()>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:129:58
    |
129 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
129 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,Retirement<V>>>())}
129 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,Retirement<V>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:134:58
    |
134 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,UpdateCursor<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
134 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,UpdateCursor<V>>>())}
134 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,UpdateCursor<V>>>())}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🗂️ordered/♻️retirement/🦀️.rs:139:58
    |
139 |     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,LookupCursor<V>>>())}
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
139 -     fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<TypedOwner<V,LookupCursor<V>>>())}
139 +     fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<TypedOwner<V,LookupCursor<V>>>())}
    |

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:4:119
  |
4 | ...ogress, RetainedCloneStep}, ErasedSnapshotRetirement, SnapshotRetirementStep, ValueError, ValueRefusalKind};
  |                                                          ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:64:21
   |
64 |         let bytes = std::mem::size_of::<String>();
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
64 -         let bytes = std::mem::size_of::<String>();
64 +         let bytes = size_of::<String>();
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:83:40
   |
83 |         Ok(if self.pending.is_some() { std::mem::size_of::<crate::retirement::controlled::ControlledRetirement<String>>() } else { 0 })
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
83 -         Ok(if self.pending.is_some() { std::mem::size_of::<crate::retirement::controlled::ControlledRetirement<String>>() } else { 0 })
83 +         Ok(if self.pending.is_some() { size_of::<crate::retirement::controlled::ControlledRetirement<String>>() } else { 0 })
   |

warning: unused imports: `SnapshotRetirementStep` and `ValueError`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:2:38
  |
2 | use crate::{ErasedSnapshotRetirement,SnapshotRetirementStep,ValueError};
  |                                      ^^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^^

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:15:105
   |
15 |     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**own...
   |                                                                                                         ^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
15 -     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
15 +     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🦀️.rs:24:148
   |
24 | ...l FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn crate::ErasedSnapshotRetirement>>))->Result<T,E>{
   |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
24 -     pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn crate::ErasedSnapshotRetirement>>))->Result<T,E>{
24 +     pub fn with_retirement_owner<T,E:From<ValueError>>(&mut self,wrapper_bytes:usize,operation:impl FnOnce(&mut Self)->(Result<T,E>,Option<Box<dyn ErasedSnapshotRetirement>>))->Result<T,E>{
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🦀️.rs:75:108
   |
75 |     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError> { let bytes=count.checked_mul(std::mem::size_of::<T>())...
   |                                                                                                            ^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
75 -     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError> { let bytes=count.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native decoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native decoding collection allocation failed"))?;Ok(output) }
75 +     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError> { let bytes=count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native decoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native decoding collection allocation failed"))?;Ok(output) }
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛫️encode/🦀️.rs:51:106
   |
51 |     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(std::mem::size_of::<T>()).f...
   |                                                                                                          ^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
51 -     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native encoding collection allocation failed"))?;Ok(output)}
51 +     pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{let bytes=count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native encoding collection size overflow"))?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native encoding collection allocation failed"))?;Ok(output)}
   |

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:2:86
  |
2 | use crate::{NativeDecodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,SnapshotRetirementStep,list::{PagedList,PagedLis...
  |                                                                                      ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:3:103
  |
3 | use crate::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};
  |                                                                                                       ^^^^^^^^^^^^^^^^^^^^^^

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/📋️queue/🦀️.rs:75:59
   |
75 |     fn retirement_birth_bytes(&self)->Option<usize> {Some(std::mem::size_of::<Self>())}
   |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
75 -     fn retirement_birth_bytes(&self)->Option<usize> {Some(std::mem::size_of::<Self>())}
75 +     fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/📋️queue/🦀️.rs:94:89
   |
94 |     fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
   |                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
94 -     fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
94 +     fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:54:83
   |
54 | pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {std::mem::size_of::<FactoryTicket<T>>()}
   |                                                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
54 - pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {std::mem::size_of::<FactoryTicket<T>>()}
54 + pub const fn factory_retirement_frame_bytes<T:FactoryPayloadRetirement>()->usize {size_of::<FactoryTicket<T>>()}
   |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:100:77
    |
100 |     if ticket.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:std::mem::size_of_val(ticket.as_ref()),depth:1,..Defaul...
    |                                                                             ^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
100 -     if ticket.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:std::mem::size_of_val(ticket.as_ref()),depth:1,..Default::default()});}
100 +     if ticket.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:size_of_val(ticket.as_ref()),depth:1,..Default::default()});}
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:111:19
    |
111 |         let bytes=std::mem::size_of_val(ticket.as_ref());
    |                   ^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
111 -         let bytes=std::mem::size_of_val(ticket.as_ref());
111 +         let bytes=size_of_val(ticket.as_ref());
    |

warning: unused imports: `admit_retained_clone_retirement` and `admit_retained_clone_scaffold_retirement`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🗺️ordered-map/🦀️.rs:3:230
  |
3 | ...it_retained_clone_progress, admit_retained_clone_retirement, admit_retained_clone_scaffold_retirement, close_retained_binding};
  |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🗺️ordered-map/🦀️.rs:4:13
  |
4 | use crate::{SnapshotRetirementStep, retirement::RetireOwned};
  |             ^^^^^^^^^^^^^^^^^^^^^^

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🗺️ordered-map/🦀️.rs:130:24
    |
130 | ...   let progress = super::admit_retained_clone_close(grant, step, terminal_is_empty, &format!("retained ordered-map {label} scaff...
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
130 -         let progress = super::admit_retained_clone_close(grant, step, terminal_is_empty, &format!("retained ordered-map {label} scaffold close"))?.progress();
130 +         let progress = admit_retained_clone_close(grant, step, terminal_is_empty, &format!("retained ordered-map {label} scaffold close"))?.progress();
    |

warning: unused import: `admit_retained_clone_retirement`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/📦️paged/🦀️.rs:3:202
  |
3 | ...etainedCloneStep, admit_retained_clone_progress, admit_retained_clone_retirement};
  |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/📦️paged/🦀️.rs:5:5
  |
5 |     SnapshotRetirementStep, ValueError, ValueRefusalKind,
  |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `admit_retained_clone_retirement`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/📋️paged-list/🦀️.rs:3:226
  |
3 | ...etainedCloneStep, admit_retained_clone_progress, admit_retained_clone_retirement};
  |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/📋️paged-list/🦀️.rs:6:5
  |
6 |     SnapshotRetirementStep,
  |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `SnapshotRetirementStep`
 --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🔗️projection/🦀️.rs:4:13
  |
4 | use crate::{SnapshotRetirementStep, ValueError, ValueRefusalKind};
  |             ^^^^^^^^^^^^^^^^^^^^^^

[native:owner-command] running elapsedMs=60006
error[E0046]: not all trait items implemented, missing: `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📝️text/📦️paged/🦀️.rs:62:1
   |
62 | impl<const N:usize> ErasedSnapshotRetirement for PagedText<N>{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
   |
  ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:5
   |
19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
   |     --------------------------------------------------------------------------------------------- `close_step` from trait
20 |     fn terminal_is_empty(&self) -> bool;
21 |     fn next_copy_byte_demand(&self) -> usize;
   |     ----------------------------------------- `next_copy_byte_demand` from trait
22 |     fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
   |     -------------------------------------------------------------------------------------------- `next_capacity_byte_demand` from trait
23 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
   |     ---------------------------------------------------------------- `next_release_byte_demand` from trait
24 |     fn next_depth_demand(&self) -> Result<usize, ValueError>;
   |     --------------------------------------------------------- `next_depth_demand` from trait
error[E0046]: not all trait items implemented, missing: `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:12:1
   |
12 | impl ErasedSnapshotRetirement for NativeDecodeRetirementRecipient{
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
   |
  ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:5
   |
19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
   |     --------------------------------------------------------------------------------------------- `close_step` from trait
20 |     fn terminal_is_empty(&self) -> bool;
21 |     fn next_copy_byte_demand(&self) -> usize;
   |     ----------------------------------------- `next_copy_byte_demand` from trait
22 |     fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
   |     -------------------------------------------------------------------------------------------- `next_capacity_byte_demand` from trait
23 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
   |     ---------------------------------------------------------------- `next_release_byte_demand` from trait
24 |     fn next_depth_demand(&self) -> Result<usize, ValueError>;
   |     --------------------------------------------------------- `next_depth_demand` from trait
error[E0046]: not all trait items implemented, missing: `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:105:1
    |
105 | impl ErasedSnapshotRetirement for PagedUtf8AppendCursor {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `close_step`, `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:19:5
    |
 19 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    |     --------------------------------------------------------------------------------------------- `close_step` from trait
 20 |     fn terminal_is_empty(&self) -> bool;
 21 |     fn next_copy_byte_demand(&self) -> usize;
    |     ----------------------------------------- `next_copy_byte_demand` from trait
 22 |     fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    |     -------------------------------------------------------------------------------------------- `next_capacity_byte_demand` from trait
 23 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
    |     ---------------------------------------------------------------- `next_release_byte_demand` from trait
 24 |     fn next_depth_demand(&self) -> Result<usize, ValueError>;
    |     --------------------------------------------------------- `next_depth_demand` from trait
error[E0599]: no method named `next_cold_byte_demand` found for struct `RetainedCloneClose` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🗺️ordered-map/🦀️.rs:820:23
    |
820 |         Ok(self.close.next_cold_byte_demand()?.max(usize::from(!self.terminal_is_empty())))
    |                       ^^^^^^^^^^^^^^^^^^^^^
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🧬️retained-clone/🦀️.rs:282:1
    |
282 | pub struct RetainedCloneClose {retirement:Option<Box<dyn ErasedSnapshotRetirement>>}
    | ----------------------------- method `next_cold_byte_demand` not found for this struct
    |
help: there is a method `next_copy_byte_demand` with a similar name
    |
820 -         Ok(self.close.next_cold_byte_demand()?.max(usize::from(!self.terminal_is_empty())))
820 +         Ok(self.close.next_copy_byte_demand()?.max(usize::from(!self.terminal_is_empty())))
    |
error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn retirement_contract::ErasedSnapshotRetirement>` in the current scope
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🫴️recipient/🦀️.rs:15:148
   |
15 | ...s_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `RetirementCursor` defines an item `next_close_byte_demand`, perhaps you need to implement it
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:25:1
   |
25 | pub trait RetirementCursor: Send {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: there is a method `next_copy_byte_demand` with a similar name
   |
15 -     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_close_byte_demand()}).unwrap_or(0)}
15 +     fn next_close_byte_demand(&self)->usize{self.owner.as_ref().map(|owner|if owner.terminal_is_empty(){std::mem::size_of_val(&**owner)}else{owner.next_copy_byte_demand()}).unwrap_or(0)}
   |
error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn retirement_contract::ErasedSnapshotRetirement>` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../📦️paged/🎮️append/🦀️.rs:109:143
    |
109 | ... pending.capacity().max(1)), |retirement| retirement.next_close_byte_demand())
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `RetirementCursor` defines an item `next_close_byte_demand`, perhaps you need to implement it
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:25:1
    |
 25 | pub trait RetirementCursor: Send {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: there is a method `next_copy_byte_demand` with a similar name
    |
109 -         self.retirement.as_ref().map_or_else(|| self.pending.as_ref().map_or(1, |pending| pending.capacity().max(1)), |retirement| retirement.next_close_byte_demand())
109 +         self.retirement.as_ref().map_or_else(|| self.pending.as_ref().map_or(1, |pending| pending.capacity().max(1)), |retirement| retirement.next_copy_byte_demand())
    |

Some errors have detailed explanations: E0046, E0407, E0599.
For more information about an error, try `rustc --explain E0046`.
warning: `semio-framework-value` (lib) generated 30 warnings
```
