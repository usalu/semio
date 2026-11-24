# value-native-owned split (2026-11-24)

## Cause
The gate counted the crate closure at about 305000 of 262144 bytes. HEAD was 252043. The growth is the new untracked `🏗️record-machine/🦀️.rs` (52391 bytes, a peer's derive refactor), not the job-context rebind or the drop-guard rewrite (34 asserts, under 1 KB).

## Why the split is not a plain lower layer
`NativeReceive` has impls for types foreign to the crate (`Option`, `Vec`, `String`, tuples, `Result`, arrays, `Box`, `OrderedMap`, `PersistentList`, `SealedShared`, `ValueError` and the six `value_codec!` records). By the orphan rule every one of those impls and its receivers must stay in the trait crate. `NativeRecordFields` has the same constraint. So tree, variant, array, map, set, mapped and refusal cannot move out. Only trait-independent code can go below.

## Change
New crate `semio-framework-value-native-record-machine` (`🧳️owned/🏗️record-machine/📦️packages/🦀️rust`), below native-owned:
- machine core `🏗️record-machine/🦀️.rs` (file stays in place, now compiled in the new crate);
- `🏗️record-machine/🪡️field/🦀️.rs`: `NativeScalarValue`, `NativeScalarReceiver`, the primitive reads, `NativeFieldPlanSlot` and the plan fns, cut from the tree file;
- the adapters for `NativeReceiveSlot` stay in the owner as `🏗️record-machine/🧩️slot/🦀️.rs`.
native-owned re-exports `machine` and the field items, so derive paths (`native_decoding::owned::machine`) are unchanged.
Also: `NativeRecordArms::schema` (derive and machine) replaces the machine's use of `NativeRecordFields`; `N:'static` bounds added to the machine fns that take `dyn NativeRecordArms`; inline tests of array and refusal moved to `🧪️tests` and `🧪️unit` files (out of the source closure).

## Sizes (closure, tests excluded)
- native-owned: about 305000 before, 237436 now (15 files of 32, 9.4 percent headroom). The target of 200000 is not reachable without redesigning the trait (generic receivers over the receiver type instead of `T:NativeReceive`).
- record-machine: 62943 (2 files).
- Next candidates inside native-owned if it grows: `unit` enum receiver (9.4K, trait-independent), `NativeStringReceiver` (about 6K).

## Crates above 255000 bytes (approximate scan, same closure reader as the gate, tests excluded)
| bytes | files | crate root |
|---|---|---|
| 339502 | 41 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/📦️packages/🦀️rust` |
| 324221 | 87 | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📦️packages/🦀️rust` |
| 270743 | 34 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust` |
| 262195 | 32 | `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📐️model/📦️packages/🦀️rust` |
| 262088 | 16 | `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust` |
| 261923 | 9 | `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🖨️printing/📦️packages/🦀️rust` |
| 261920 | 16 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/📦️packages/🦀️rust` |
| 260544 | 18 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📦️packages/🦀️rust` |
| 259783 | 21 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🎞️session/📦️packages/🦀️rust` |
| 258842 | 29 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️program-bridge/📦️packages/🦀️rust` |
| 257800 | 23 | `🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust` |
| 257721 | 27 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔀️shell-actions/📦️packages/🦀️rust` |
| 257524 | 21 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📦️packages/🦀️rust` |
| 256227 | 17 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/🧱️owner/📦️packages/🦀️rust` |

Some of these (vcs, dsl, interpreter) read above 262144 here, so they either carry another boundary or the scan over-counts; verify with the gate before splitting.

## Follow-up (hub-10 admission: 262180 of 262144)
The peer added `🏗️record-machine/🔀️enum/🦀️.rs` (24220 bytes) to native-owned after the first measurement, so the crate crossed the limit again (the count was right, the file was new). Moved to the record-machine crate: that enum machine and the variant selector (`🏗️record-machine/🎯️selector/🦀️.rs`, cut from `🌳️value/🔀️variant/🦀️.rs`). native-owned re-exports `enum_machine` and the selector types. The unit-enum receiver stays in native-owned because `NativeUnitVariants` is implemented for `ValueRefusalKind` (orphan rule).
Closure sizes, tests excluded: native-owned 218499 (15 files, 43 KB margin), record-machine 107464 (5 files). `cargo check` passes for both.
