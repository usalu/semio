# Typed Read Lease Owning Red 37

Exact published literal General37 physically closed Nx/Bun 1 and Cargo 101 (signal null, reason exit). The full original owning-package all-target selection and compiler/deadline controls were retained. Selected source count 313; exact sources true; exact producer true. Runtime targets did not execute; whole dependency closure is not accepted.

Actual compiler boundary:

```text
error[E0053]: method `next_copy_byte_demand` has an incompatible type for trait
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🛬️decode/🧪️tests/🦀️.rs:80:42
   |
80 |         fn next_copy_byte_demand(&self)->usize{0}
   |                                          ^^^^^ expected `Result<usize, ValueError>`, found `usize`
   |
error[E0433]: cannot find type `ReadLeaseRegistry` in this scope
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🧪️tests/🦀️.rs:38:94
   |
38 | ...xt.as_str().unwrap();let capacity=ReadLeaseRegistry::<String>::constructor_capacity_bytes();
   |                                      ^^^^^^^^^^^^^^^^^ use of undeclared type `ReadLeaseRegistry`

error[E0433]: cannot find type `ReadLeaseRegistry` in this scope
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🧪️tests/🦀️.rs:39:304
   |
39 | ...=observe_retirement_allocations(||ReadLeaseRegistry::<String>::admit(denied));assert!(refusal.is_err());assert_eq!(heap,(0,0));}
   |                                      ^^^^^^^^^^^^^^^^^ use of undeclared type `ReadLeaseRegistry`

error[E0433]: cannot find type `ReadLeaseRegistry` in this scope
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🧪️tests/🦀️.rs:40:177
   |
40 | ...=observe_retirement_allocations(||ReadLeaseRegistry::<String>::admit(g).unwrap());assert!(p.fits(g));assert_eq!(heap,(capacity,0)...
   |                                      ^^^^^^^^^^^^^^^^^ use of undeclared type `ReadLeaseRegistry`

error[E0433]: cannot find type `ReadLeaseRegistry` in this scope
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🧪️tests/🦀️.rs:22:25
   |
22 | ...   let(registry,p)=ReadLeaseRegistry::<u64>::admit(RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ReadLeaseRegistry::...
   |                       ^^^^^^^^^^^^^^^^^ use of undeclared type `ReadLeaseRegistry`

error[E0433]: cannot find type `ReadLeaseRegistry` in this scope
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🧪️tests/🦀️.rs:22:116
   |
22 | ...um_items:1,maximum_capacity_bytes:ReadLeaseRegistry::<u64>::constructor_capacity_bytes(),maximum_depth:1,..Default::default()}).u...
   |                                      ^^^^^^^^^^^^^^^^^ use of undeclared type `ReadLeaseRegistry`

error[E0433]: cannot find type `ReadLeaseRegistry` in this scope
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../🔗️read/🧪️tests/🦀️.rs:22:256
   |
22 | ...ert_eq!(p.retained_capacity_bytes,ReadLeaseRegistry::<u64>::constructor_capacity_bytes());
   |                                      ^^^^^^^^^^^^^^^^^ use of undeclared type `ReadLeaseRegistry`

error[E0308]: mismatched types
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🧪️tests/🦀️.rs:32:102
   |
32 | ...ementDemand {copy_bytes:owner.next_copy_byte_demand(),capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.n...
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `usize`, found `Result<usize, ValueError>`
   |
error[E0308]: mismatched types
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/📋️queue/🧪️tests/🎮️ownership/🦀️.rs:36:80
   |
36 | ..._bytes>0 && queue.next_copy_byte_demand()==0 {let (small,(a,r))=observe_retirement_allocations(||queue.step(RetainedCloneGrant {m...
   |                -----------------------------  ^ expected `Result<usize, ValueError>`, found integer
   |                |
error[E0308]: mismatched types
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:59:241
   |
59 | ..._capacity_byte_demand(if copy != 0 { law["maximumCopyBytes"].as_u64().unwrap() as usize } else { release }).unwrap(), release, ow...
   |                             ----    ^ expected `Result<usize, ValueError>`, found integer
   |                             |
error[E0308]: mismatched types
  --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:72:33
   |
72 | ...   for denied in [(copy != 0).then_some(RetainedCloneGrant { maximum_copy_bytes: copy.saturating_sub(1), ..grant }), (capacity !=...
   |                       ----    ^ expected `Result<usize, ValueError>`, found integer
   |                       |
error[E0599]: no method named `saturating_sub` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:72:92
     |
  72 | ...mum_copy_bytes: copy.saturating_sub(1), ..grant }), (capacity != 0).then_some(RetainedCloneGrant { maximum_capacity_bytes: capa...
     |                         ^^^^^^^^^^^^^^ method not found in `std::result::Result<usize, refusal::ValueError>`
     |
error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:173:75
    |
173 | ...capacity_byte_demand(if copy == 0 { release } else { frontier_law["maximumCopyBytes"].as_u64().unwrap() as usize }).unwrap();
    |                            ----    ^ expected `Result<usize, ValueError>`, found integer
    |                            |
error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:107:63
    |
107 | ...   (copy, owner.next_capacity_byte_demand(if copy == 0 { release } else { law["maximumCopyBytes"].as_u64().unwrap() as usize })....
    |                                                 ----    ^ expected `Result<usize, ValueError>`, found integer
    |                                                 |
error[E0369]: binary operation `<=` cannot be applied to type `std::result::Result<usize, refusal::ValueError>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:112:26
    |
112 |             assert!(copy <= law["maximumCopyBytes"].as_u64().unwrap() as usize);
    |                     ---- ^^ -------------------------------------------------- usize
    |                     |
error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:126:26
    |
126 |                 (copy != 0).then_some(RetainedCloneGrant { maximum_copy_bytes: copy.saturating_sub(1), ..grant }),
    |                  ----    ^ expected `Result<usize, ValueError>`, found integer
    |                  |
error[E0599]: no method named `saturating_sub` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:126:85
     |
 126 |                 (copy != 0).then_some(RetainedCloneGrant { maximum_copy_bytes: copy.saturating_sub(1), ..grant }),
     |                                                                                     ^^^^^^^^^^^^^^ method not found in `std::result::Result<usize, refusal::ValueError>`
     |
error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:201:56
    |
201 |                 (copy, owner.next_capacity_byte_demand(copy).unwrap(), owner.next_release_byte_demand().unwrap())
    |                              ------------------------- ^^^^ expected `usize`, found `Result<usize, ValueError>`
    |                              |
error[E0369]: cannot add `usize` to `std::result::Result<usize, refusal::ValueError>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:204:26
    |
204 |             assert!(copy + capacity + release <= 4096);
    |                     ---- ^ -------- usize
    |                     |
error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:205:84
    |
205 | ...ximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 };
    |                                        ^^^^ expected `usize`, found `Result<usize, ValueError>`
    |
error[E0369]: binary operation `>` cannot be applied to type `std::result::Result<usize, refusal::ValueError>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:209:21
    |
209 |             if copy > 0 {
    |                ---- ^ - {integer}
    |                |
error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:211:34
    |
211 |                 assert_eq!(copy, row["minimumCopyBytes"].as_u64().unwrap() as usize);
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<usize, ValueError>`, found `usize`
    |
error[E0369]: cannot subtract `{integer}` from `std::result::Result<usize, refusal::ValueError>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:213:129
    |
213 | ...ainedCloneGrant { maximum_copy_bytes: copy - 1, maximum_capacity_bytes: 0, ..grant }).unwrap());
    |                                          ---- ^ - {integer}
    |                                          |
error[E0369]: binary operation `>` cannot be applied to type `std::result::Result<usize, refusal::ValueError>`
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:221:21
    |
221 |             if copy > 0 { assert_eq!(step.progress().released_bytes, 0); }
    |                ---- ^ - {integer}
    |                |
error[E0308]: mismatched types
   --> 🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧪️tests/🔬️unit/🦀️.rs:228:51
    |
228 |         assert_eq!(owner.next_copy_byte_demand(), 0);
    |                                                   ^ expected `Result<usize, ValueError>`, found integer
    |
error: could not compile `semio-framework-value` (lib test) due to 25 previous errors; 105 warnings emitted

```
