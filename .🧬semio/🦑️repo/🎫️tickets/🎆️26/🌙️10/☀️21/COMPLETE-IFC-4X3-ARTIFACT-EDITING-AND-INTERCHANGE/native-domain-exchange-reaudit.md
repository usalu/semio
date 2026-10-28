# Native Domain Exchange Reaudit

Read-only review of the clean checkpoint in `native-domain-exchange-fixes.md`, focused on the new owned FEM routes, owner receiving/writer, normal entry execution and the narrow nested IO selection fix. Production and shared generated dependencies were not changed. Direct source-read/Python fallback follows the existing ticket's documented repo CLI limitation. Native tests remain unexecuted because authentic compiler ingress is unavailable. The generic decoded IFC source/final IFC target pack wrapper, fixed warning metadata and ArchiveChildren ownership gaps are already explicitly open and are not counted here as new findings.

## P1 — Successful FEM Pack Encoding Enters the Work API After Starting Closure

File: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/write-pack.rs`, `PackWriting::demand` and `advance_owned`.

Both select cursor retirement only when `cursor.is_complete()`. The first call to `cursor.close(g)` changes its phase from Complete to Closing and closes one retained child frontier. It does not close all source/schema/Deflate/catalog/hash owners in one call. On the next turn `is_complete()` is false; the writer chooses normal cursor demands and ultimately calls `cursor.advance`. That API explicitly refuses Closing/Closed. Thus the successful writer cannot drain scratch to terminal and cannot publish the final FEM payload. This affects the normal registered `IfcIntoFem` route, not only the default reference codec.

Independent source contract: `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs`: `is_complete` tests exactly Complete (line 43), `advance` refuses Closing/Closed (lines 96–97), `close` changes phase to Closing and closes one child (lines 150–153). Child scratch contains several distinct owners, so stopping after one close is not terminal even with a generous grant.

Reproduce once authentic native execution is available: run `actual_pack_receiving_retains_original_wallet_and_persistent_output` and `actual_registered_fem_host_pack_roundtrip_keeps_shared_definitions`; both call `encode_exchange_pack(...).unwrap()` and must successfully pass beyond cursor scratch closure. Add a lower-level test that stops immediately after Complete, takes the first funded close step, verifies nonterminal closure and drives multiple additional close demands without calling `advance` again.

Repair: record an explicit successful-drain phase/flag in PackWriting before the first close. In that phase both `demand` and `advance_owned` must exclusively use `cursor.retirement_demands`/`cursor.close` until `terminal_is_empty`; only then remove the cursor and permit output take. Do not overload the existing `closing` flag if that means cancellation retirement, because `ready()` currently rejects it. Keep final wire custody and paid receipts through every close frontier, and test denied/zero-release grants during successful draining as well as cancellation.

## P2 — New Roundtrip Test Expects a Nested Direction to Escape Its Scope

File: IFC `io/🧪️tests/native_fem_owned.rs`, `actual_registered_fem_host_pack_roundtrip_keeps_shared_definitions`: it asserts outer `control.direction() == Encode` after the exporter entry returns.

`IoRunControl::receive_nested` in `🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs` creates a scoped child run with the parent's copied direction and shares progress/pending custody; the child changes its own active direction, while the parent remains bound to its original receiving frame. `IfcIntoFem` selects encoder inside that scoped child. Generic `serializer_entry` does not select the parent's encoder after `T::serialize`. Therefore the new assertion cannot pass under the current scoped contract even after P1 is repaired.

This is a verification defect, not a recommendation to blindly propagate child.active. Existing `original_receiving_nested_io_dual_control_empty_phase_change_keeps_original_wallet` deliberately checks that nested frame birth/closure remains charged to the parent's original direction while the child can use its companion. Scoped restoration fits that ownership contract. Document/assert the intended outer direction separately and test that the original encoder really receives payload production. Preserve original parent retirement authority when correcting the new expectation. The narrower `select` occupied-versus-empty pending-slot change itself does not replace controls or wallets.

## P2 — Quadratic Identity and Reference Admission Defeats the New Sorted Projection Indexes

Files: FEM `schema/📸️snapshot/native.rs::validate_authoring`; IFC `io/native_fem_build.rs::admission`, `node`, `material`, `section`.

Actual receiving reruns namespace uniqueness by comparing every identity against every preceding row. It then linearly scans nodes/materials/sections per member and nodes per support. The graph builder similarly checks preceding rows, scans all supports per node, and scans source definitions per member. These operations are checkpointed and funded, but asymptotically quadratic. The new IFC-to-FEM funded sorted indexes do not prevent the quadratic scan when its projected authoring owner enters `receive_authoring`.

Exact reproducible cost for N distinct rows in `validate_authoring` is N(N−1)/2 previous-row iterations, each charged as an item even across different namespaces. For 10,000 nodes this is 49,995,000 iterations before persistent row receiving; 64,000 rows requires 2,047,968,000. A minimal analytical source with 10,000 isolated point connections fits the 64,000 IFC-entity source bound (roughly five entities per node) yet exceeds the test wallet's 10,000,000-item budget on duplicate checking alone. The operation can refuse valid admitted datasets from work inflation rather than useful conversion demand. Builder `admission` computes its output extent bound only after these comparisons.

Repair: source-owned funded sorted namespace identity indexes and node/material/section/support indexes, with cancellable heap/in-place sorting or existing owner-compatible structures. Check coarse cardinality/extent bounds before scanning identities, binary-search references, and share the validated canonical index/receipt with the actual receiving boundary where appropriate. Do not skip owner validation merely because IFC happened to index its input. Add a larger language-neutral isolated-node/member-chain case and compare exact validation-work growth for N and 2N, plus budget/cancellation closure assertions.

## Confirmed Improvements and Remaining Limits

The former uncontrolled authoring-record/default-document/envelope bridge is gone from production. Fixture-only permissive authority is visibly cfg(test). Actual receiving constructs persistent sealed node/material rows, retains plain source inputs, and uses original snapshot wallet/capture/close frontiers. The actual writer borrows canonical fields and builds one final envelope buffer; buffer initialization is bounded to 65,536 bytes and the original text-copy helper itself copies in cancellable 64 KiB UTF-8 spans while retaining partial output. No new whole-authoring clone is used by the controlled route.

Canonical material/section keys are funded and sorted, endpoint relationships are checked against topology, full engineering dimension/scale checks and physical value checks exist, unsupported loads/releases/roll/local axes refuse. All newly owned intermediate row/graph slots examined remain in retained publication frames on refusal. No additional native type/physical-allocation correctness claim follows from this source review.

Normal domain IO entries invoke owned FEM receiving/writing and owned FEM-pack receiving/IFC construction under paired original controls. The narrow SQLite lane is separate. Generic IFC source/final target wrapper closure remains the next stage already recorded in the fixes report.

Retained probe: `native-domain-reaudit-probes.py`; output `🗑️generated/native-domain-reaudit-probes.json`. It reproduces the source-derived state transition and exact quadratic work counts, explicitly `nativeExecuted:false` and `modelOnly:true`. It is not executable Rust evidence or a native emitted-artifact oracle. No native outputs were fabricated. Existing authentic native tests should expose P1 and the incorrect direction expectation once ingress is restored.
