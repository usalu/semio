# Flow Retained Member-Open Integration Audit

## Scope and verdict

Read-only current-tree audit of the real Flow retained snapshot decoder, the shared request/open authority, and `VcsArtifactApp` child publication. This is a **RED integration boundary**. The decoder is a bounded, request-owning Flow snapshot decoder, but no `MemberOpenOperation` implementation or `MemberFactory` entry point adopts it. The live child restore still accepts a borrowed `&[u8]` and awaits the legacy whole-pack `M::open` path. Therefore the source does not yet establish retained input ownership across full history/replay/initialization, cancellation after typed decode, or a final cancel/root-generation fence before publication.

This report makes no native/runtime-pass claim. Provider-owned cold gate `37645` was active while this audit was made.

## Current source evidence

| Boundary | Current evidence | Classification |
| --- | --- | --- |
| Request authority | [`MemberOpenRequest`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:39) owns sealed pages, expected coordinate, optional owner, operation, generation, expiry, and has authority checks before and during page copies. Its close path retires pages and identity under grants. | Source-supported primitive |
| Operation seam | [`MemberOpenOperation`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:32) has the required stepping/close shape, but `rg` finds no implementation in the framework, stdio, or Flow tree. | RED: dormant abstraction |
| Typed partial-owner holder | private [`MemberStoreOpenRetained`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:273) keeps request, owner bundle, history, initial, edit, envelope, runtime, and active retirement through bounded close. | Reusable internal lifecycle, not an activated decode pipeline |
| Flow snapshot decoder | [`SemioFlowSnapshotDecode`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:25) frames request bytes, checks authority around every byte, has fixed node/edge/string bounds, exact EOF, `take_ready` rechecks authority, and boundedly retires partial snapshot plus original request. | Source-supported snapshot-only component |
| Closed request input | The decoder constructor calls infallible [`request.expected()`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:50); that accessor `expect`s retained identity at [open/🦀️.rs:63](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:63). A fully closed request therefore panics instead of returning its terminal request in `MemberOpenAdmissionError`. Provider confirmed and will correct this after its active gate. | Source RED, repair already coordinated |
| Factory selection | [`MemberFactory`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17962) only exposes async `open(expected, owner, envelope_pack)`. Generated [`space_members!` arms](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18285) select from borrowed `expected` then call whole-pack [`open_member_store`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3137). | RED: no request-owned selection/open |
| Legacy decoder order | `open_member_store` correctly validates parsed `HistoryLog` identity before `P` decoding, but first calls whole-byte `decode_document_pack_bytes` and its async history decoder ([store/🦀️.rs:3144](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3144)). It has no retained page owner/cancellation or close result. | Not a safe replacement target |
| Public child restore | [`open_child`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20004) reserves the child-map slot, then awaits legacy `M::open` at [20015](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20015). It only compares parent generation before an awaited `prepare_child_member`, then commits after further awaits ([20022-20039](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20022)). No request cancel token is present. | RED: no final authority fence/linearized retained operation |
| Existing local admission | `ChildMemberRegistry` reservation has an exact generation and `cancel_admission` ([plugin/🦀️.rs:8190](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8190), [8329](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8329)). `OwnsAdmission` is an uncommitted authority token checked at commit ([store/🦀️.rs:18803](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18803)). `ChildContentView::admit_member` and publication generation preflight are non-mutating ([plugin/🦀️.rs:19833](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19833)). | Useful transaction pieces, but no composite abort/final fence |

`load_child_pack` remains the actual archive/open caller and delegates directly to the legacy method at [plugin/🦀️.rs:24630](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24630). The public stdio wrapper also continues to expose the same legacy opening API at [stdio/🦀️.rs:1149](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1149).

## Safe source-first integration seam

1. Replace, rather than supplement, `MemberFactory::open`. Give it an associated, closed operation type and `begin_open(request: MemberOpenRequest) -> Result<Self::Open, MemberOpenAdmissionError>`, where `Self::Open: MemberOpenOperation<Member = Self>`. Have `space_members!` generate a concrete closed operation enum with one variant per full `(kind, standard, subset)` arm. Selection consumes the request only after inspecting its still-retained exact coordinate. An unknown coordinate returns the original request with `Identity`; it must never become an erased or borrowed byte slice.

2. Repair request inspection before any decoder construction. Add a non-panicking identity inspection/validation API on `MemberOpenRequest`; a closed/closing request returns `Stale` with the original request, while a present non-Flow coordinate returns `Identity` with the original request. The Flow constructor must not call `expected()` until that check succeeds.

3. Let the Flow operation own `SemioFlowSnapshotDecode` until its `take_ready` transfer. Only then move the original request, snapshot, and exact member-store owner bundle into a store-private retained operation based on `MemberStoreOpenRetained`. That operation must perform history, replay, typed envelope construction, and store initialization incrementally from request chunks. Every stage must retain its just-created typed value before its next fallible/awaited stage; all rejection/cancel paths remain app-owned until bounded `close_step` reaches its terminal witness. Do not expose the private holder as a cross-crate generic convenience API and do not route Flow through JSON `ArtifactEnvelopeDecodeAuthority`.

4. Add one app-owned fixed registry for the resulting `M::Open`, the immutable `MemberOpenRequest` authority, the exact `ChildStoreAdmission`, captured parent-store generation, and the request cancel scope. This is the correct place for a reserved child slot: outside an async future and subject to the existing maintenance/close machinery. The input/open job must be admitted only after all fixed job, child-abort, child-root-retirement, and map-slot capacities are preflighted.

5. On every driven page, use the request `StepContext`; do not materialize/commit after cancellation, operation mismatch, request generation mismatch, expiry, or close. After `Ready`, revalidate the request authority, parent store generation, declared `child_restore_projection`, map admission generation, graph admission authority, and child-root publication generation. Repeat the check after every awaited preparation/checkpoint/snapshot capture.

6. Make the final publication a single non-suspending transaction. The current `commit_child_member` awaits `set_owner` and `graph_mut` before map/root publication ([plugin/🦀️.rs:19950](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19950)); that cannot be the final linearization point. Prepare all async work first, then enter one fixed-order app/graph transaction which checks all tokens and publishes graph ownership, child map, immutable content root, generation, and pending-pin removal with no await. If `SpaceMember::set_owner` cannot become a pre-commit, private operation with an exact rollback/retirement rule, its async contract is the remaining blocker to this packet.

7. Add a single composite abort path that cancels the child registry reservation and returns/drops the uncommitted `OwnsAdmission`, non-mutating root index/publication preflight, and retains the finished member or active open job in the existing bounded retirement registry. Do not merely cancel the map slot. On failure before a finished member exists, the operation itself is the retained owner; on failure after it exists, hand it to `ChildMemberRetirement` only after the map reservation is canceled.

## Required neutral acceptance corpus

Extend the existing Flow binary fixture, not a source-string check, with these language-neutral rows and bind the Rust/independent implementation to the same output:

- closed request handed to Flow selection: `stale`, no panic, original request terminally retired;
- non-Flow coordinate: `identity`, original request byte count and full identity retired; no snapshot allocation/decode;
- wrong persisted document id/schema/dialect/parent/slot/child after a valid Flow snapshot: exact identity/owner rejection before typed `ArtifactStore` publication;
- cancellation at a snapshot page boundary, a history/replay page boundary, and after operation-ready but before app transaction: map empty, no graph edge, unchanged root and parent generation, exact close total;
- parent-generation change after decode and after awaited preparation: same no-publication result; operation/member is retained and drained;
- stale child-map admission or changed graph authority: cannot commit into a reused slot or graph;
- capacity refusal before intake and failure after a completed member: no lost request/member, next independent request can use its own slot;
- a multi-page valid Flow frame whose page retirement is driven under variable positive byte grants; total released input/identity/typed bytes must equal the owned values, not merely satisfy a per-step ceiling.

The current decoder law covers two valid snapshots, twelve hostile decoder rows, byte-boundary cancellation, and bounded close, but does **not** exercise a full member factory or app commit ([stdio Flow test](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/💾️binary/🧪️tests/🦀️.rs:55)). The framework’s registered `member-open-protocol-check` explicitly logs that typed parser/factory activation is not claimed ([host script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:238)). Existing child laws prove legacy factory failure cleanup and success publication, not retained request cancellation ([plugin/🦀️.rs:34907](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34907)).

## Gate registration after implementation

Keep and extend these exact existing gates rather than crediting their current green/source-only state:

- `bun nx run semio-s-plugin-stdio:flow-retained-decode-check` (the stdio script executes its one exact Rust law after the AJV/third-party LEB128 oracle at [stdio script](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:430));
- `bun nx run semio-framework-os-host:member-open-protocol-check` extended beyond request-only staging to the concrete request-owned factory path;
- `bun nx run semio-framework-os:member-dialect-check` extended with the new retained child-open app law and exact-one selected test names.

Only a registered gate which executes the neutral rows through `load_child_pack`/the replacement request-owned entry point may qualify this boundary. The current package scripts and unit laws are useful component evidence, not an end-to-end child restore pass.
