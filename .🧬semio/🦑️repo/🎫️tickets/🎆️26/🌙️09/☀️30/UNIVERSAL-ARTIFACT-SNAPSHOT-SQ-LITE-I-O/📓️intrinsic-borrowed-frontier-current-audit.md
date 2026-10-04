# Intrinsic Borrowed Frontier Current Audit

Read-only review 2026-10-03 of ticket 🌲️intrinsic-borrowed-frontier/🦀️.rs and subsequently mounted root/kernel Pack Value 🛫️encode/🦀️.rs. No edits or Cargo. Root supplied authentic pre-repair actual-kernel run f0cfa5e8-57ce-4f4f-9abe-f9c7ec401f70:3 tests,2 pass/1 fail,1351 outside,0.074s;65-edge encoder DepthLimit versus accepted unchanged decoder. That is RED evidence for the removed dynamic64 ceiling, not a post-mount pass.

## Mount and semantics

Fresh reads now find DynamicFrame in both actual encoder facets: 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs and 🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs. Capsule replaces only dynamic discovery/emission with borrowed explicit stack frames. Root direct value entry still starts at semantic depth0. Generic FieldValue record/list/map grammar and its recursive64 policy are separate and remain source-bounded.

DynamicItems carries borrowed slice iterators; DynamicFrame stores next children's semantic depth. Each borrowed reference lives inside source Value and is never mutated, cloned or moved. Frame advancement restores parent sibling depth; empty containers do not create a child edge. dynamic_child_depth checked_add(1) refuses u16 overflow and excess declared depth before pushing a nonempty child frontier. A scalar or empty container at maximum is legal; its nonempty child is refused. No overflow wrap or dynamic64 clamp remains in the mounted sections.

Array/list length and ordered Object/map occurrences emit directly in slice order. Duplicate keys remain distinct; keys are deliberately inline (`text(key,true)`), so discovery ignoring Object keys is consistent with ordinary grammar and does not lose them. UInt/Int retain distinct tags, Float emits to_le_bytes, text and octets preserve bytes; empty arrays/objects retain their tags. Symbols discover only borrowed string values in the same depth-first occurrence order; finish uses existing sorted symbol grammar. No concrete new value loss was found in this source review.

## Control and allocation

Both phases use existing paid push helper9. Growth checks doubling/size overflow, charges the full new Vec backing request cumulatively, checkpoints before try_reserve_exact, then pushes only after admission. Each discovery, measure and physical-emission call has its own frontier; all three charge the exact same caller NativeEncodeControl ledger. Popping does not refund cumulative ownership. No hidden owned Value tree is allocated. Dropping a frontier drops borrowed iterators only, so refusal does not recursively drop the source tree through frames.

Discovery has checkpoint and step per visited node, stage total0 for an initially unknown full traversal; NativeEncodeControl.advance permits total0 while checking counter overflow and cancellation every256 advances. Emission checkpoints per node and existing text/byte output chunks65536 enforce interior copy cancellation. The control is not reconstructed inside either traversal. Immutable source equality is available after any refusal; no owned mirror needs special retirement here. Symbol Vec/frontier/output deallocation and allocator request equality still require actual System observer laws. Admission accounting alone is not that proof.

## Meaningful remaining laws

The current unchanged third law is root/kernel 🌱️value/🧪️tests/🎞️intrinsic-media/🦀️.rs64–81: fixture edge/maximum/accepted rows, ordinary grammar oracle bytes, exact controlled decoder and encoder, raw-word neutral comparison. It must remain unchanged through post-mount execution; do not infer GREEN from syntax or this review.

Its constructed frontier is Array-only. Extend separate laws for alternating Object/Array, ordered duplicate Object keys, maximum-depth empty container versus one deeper nonempty child, and late cancellation while ascending many exhausted frames. Prove original borrowed source raw-word/order identity after refusal. Add cumulative ownership one-byte-short and exact System-request observer coverage that includes **three** newly paid frontiers, plus decoder/source retirement qualifications separately. A checked u16::MAX child-overflow law should isolate the owner depth function or safely retire the very deep fixture; ordinary recursive reference construction/drop is not itself a trustworthy65535-depth harness.

No concrete new capsule bug was found. The retained generic64 ceiling is unchanged existing generic record policy, not the same direct dynamic Value defect. Post-mount actual kernel/native execution, full allocator settlement, and end-to-end Vcs dispatch remain unverified by this audit.
