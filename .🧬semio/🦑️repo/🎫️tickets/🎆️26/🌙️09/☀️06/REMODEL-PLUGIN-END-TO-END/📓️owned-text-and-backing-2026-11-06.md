# Owned Text, Exact Backing, and Receiver Coordination

Checkpoint: 2026-11-06 09:38 UTC. The Remodel goal and ticket remain active. The plugin has **not** been demonstrated end to end.

## Concrete Changes

The generic `FundedText` implementation and its actual Original caller/decoder laws now belong to `semio-framework-job-owned-text` at `🧰️framework/🔨️modules/🧵️job/📝️text/🧳️owned`. The former Flow leaf and its laws/JSON fixtures were moved, not copied. Flow's authoring module imports this defining package and retains its explicit product export `FlowTextCopyWork`. The framework workspace lists the new package, and Flow read declares its direct first-party dependency.

Text turns now quote copying from source metadata before work admission, report byte copying and bounded UTF-8 boundary selection, reject short independent work budgets before advancing, and quote the fixed output handoff. Convenience calls borrow the initiating StepContext work/deadline loan and check its cumulative remaining stock. Cleanup exposes its selected child's real work quote and preserves the full Original retirement outcome. This is not a persistent job construction recovery implementation.

The canonical raw allocator is `semio_framework_value::retained_clone::allocation::backing`, defined in the value control contract. It exports `exact_vector_bytes<T>`, `allocate_exact_vector<T>`, and `allocate_exact_string`. One checked `Layout::array` defines the allocation and reconstructed Vec capacity. Zero-size and zero-sized-element cases allocate nothing; String wraps an empty initialized UTF-8 payload. These primitives do not supply admission, an original work loan, or an allocation marker. Their callers still own those responsibilities.

Root migrated owned text and Remodel's `OwnedRowsBuffer` to the canonical allocator. The browser owner adopted it in indexed changes; the native owner adopted it in native construction backing. Rows-buffer construction now also rejects an unaddressable layout before taking its original vector. The new refusal test checks exact original pointer/custody preservation.

## Authored Tests and Actual Validation

- Six portable UTF-8 chunk fixtures, strict JSON schema, and a lodash/Node UTF-8 oracle: actual normal Bun execution exited 0. Cases cover ASCII, two-byte/four-byte boundaries, mixed characters and empty input. This validates the expected fixture chunks, not Rust execution.
- Seven portable vector layouts and four text layouts, strict schema, decimal.js size arithmetic and actual Node Buffer backing: actual normal Bun execution exited 0.
- Rust allocator laws now install the existing first-party RequestedAllocator, observe exact allocation/release requests, preserve pointers through in-capacity writes, and force real vector/string allocation failures. These laws are authored and **unexecuted**.
- Rust owned-text laws now install the actual HeapWitness in their new test binary. Both raw and convenience paths cover zero/short work, exact quote admission, retained denial, UTF-8 chunks, output handoff and paid cancellation cleanup. They are authored and **unexecuted**.
- Rustfmt `--emit stdout` exited 0 for the latest text/backing sources and all eight external receiver files. That is a parser check, not compilation or a formatting pass.
- The manifest path probe checked 24 relative paths across the new text, Flow read and control-contract manifests, and confirmed one framework member entry. It does not prove Cargo resolution.

The latest actual source-closure parser reported: job-owned-text 1 production source / 9,145 bytes and 4 test sources / 27,824 bytes; control-contract 9 production sources / 33,814 bytes and 15 test sources / 54,720 bytes; Flow host read 22 production sources / 228,162 bytes and 27 test sources / 253,076 bytes; Remodel diff 9 production sources / 180,104 bytes and 17 test sources / 224,921 bytes. These are parser observations with the explicit cfg sets, not a claim about every Cargo feature closure. Limits were unchanged.

## Actual Managed Gate Results

| Gate | Outcome | Meaning |
| --- | --- | --- |
| Framework lock 52518 | terminal 0 | Normal managed generate-lockfile completed; current lock contains job-owned-text. |
| All-scope lock 85519, browser-owned | terminal 0 | Managed lock update only; no test proof. |
| Raster lock 21352, native-owned | terminal 0 | Managed lock update only. |
| Key 2574 | terminal 1 | Cargo/source admission failed; includes framework manifest extent drift and earlier admitted projection diagnostics. Current inspected projection signatures already match their children. |
| Owned rows buffer 50488 | terminal 1 | Reached rustc, then ordered value E0599 at removed receive_retained_work; no row test body. |
| Owned rows edit 23112 | terminal 1 | Same known ordered receiving compiler failure; no edit test body. |
| WASI component 97795 | terminal 1 | Older admitted duplicate close_started diagnostic, repaired in current source by native owner; no current component link. |
| Old Flow text 7804 | terminal 1 | Incorrect framework-workspace package selection for semio-os-flow-host. Host belongs to the repository workspace. New text tests belong to the new framework package. |
| Exact backing 9056 | terminal 1 | Planning failed before compiler: async-executor 1.14.1 was unavailable under the planning --offline request. |

Root started canonical explicit framework `repo cargo ... fetch --locked --manifest-path 🧰️framework/Cargo.toml`, session **48088**, after confirming `fetch` is a declared resolve-kind command. It uses the same authored network grant and managed registry custody; no authority/offline bypass was introduced. It was live at this checkpoint. A fresh backing test and a correctly selected job-owned-text test should follow only after the fetch prerequisites and owned cleanup are verified.

Browser equality 9905 is now terminal 1: its interleaved log contains a native-decoding missing OriginalRetirementOutcome spelling and ends in manifest extent refusal. The native owner corrected that spelling in source; it is not a fresh compile pass. Earlier equality E0689 work accumulator types were also corrected in source. No equality test-body pass has been observed. Native presentation/broker/JPG/mesh-source attempts also terminated before their test bodies on the known receiving or older admitted projection errors; see the native reports for their exact captures.

## External Receiver Propagation

Root updated the eight handwritten external receivers in graph properties, UI values/text, expanded presentation paths, stored child headers, neural input, Raster coverage and Part21 receiving. They forward the actual tagged NativeRecoveryLoan through their live children, aggregate pending state, preserve a fallible canonical close permit gate and full OriginalRetirementOutcome. Inline fixed UI text truthfully has no local pending allocation frame.

These files are **mid-migration and not compile-ready**. Native then added required advance/take/close work quotes and the maximum_work_units axis to demand/advance. Root's external owners still need that final contract propagated. UiValue phase 3 and several Raster phases still call the removed post-effect helper. No helper shim or guessed arbitrary child allowance was supplied.

The current defining contract and its actual dependency matrix are recorded in [Native Receiver Phase Matrix](📓️native-receiver-phase-matrix.md) and [Native Work Phase Matrix](📓native-work-phase-matrix.md). Source accessor costs, NativeDefault, bounded matcher work, actual attempt markers, and single-loan nesting remain required work. Native-owned and native-decoding test closures also need genuine public integration-law ownership splits; source limits must not be raised.

## Original Custody Findings

A simple post-address commit check would be wrong: genuine OriginalPostRead handback can move the same S. Central diff commit needs a carried original mutable-post witness from that authority. The current assigned-applier source has not gained this witness. The earlier audit recommendation was corrected before Root implemented an invalid pointer check.

Job receiving identity is persistent across turns: OriginalDecodeState owns the same construction/work issuers, and a receiving effect records that issuer identity rather than a per-turn stack recipient address. Therefore an actual retained native effect can in principle be reloaned to the same original issuer. However current receiving checkpoint/cancellation gates block the normal operation before a post-cancellation construction recovery callback. The legacy admit_original_construction is callback-local and stores no persistent action frame. A restricted original construction recovery seam is still needed; exact allocation and work loans alone do not prove interrupted-callback recovery.

Root next owns the substantive funded PagedList reserve/empty-page selection prerequisite; the audit lane retains read-only design and low-complexity fixture/oracle work. Existing reserve selection and release child scans cannot honestly report one unit for variable traversal. This is required before the native matcher can use a real bounded page quote. Root's external receiver quote propagation and the separate job construction recovery seam remain open.

The complete acceptance remains the prior coordinated native/WASI/descriptor/catalog/host and real UI journey matrix, with current-source native tests, real reconstruction/export/cancellation/reload and multi-instance checks. None of those broad requirements is replaced by the narrow oracle/parser results above.
