# Receiving Custody Checks

The canonical Note profile attributed 1,574 of 41,131 exclusive samples to the matched `OriginalReceivingPrimary::is_empty` body and 2,889 to the enclosing `NativeReceivingEffect::has_no_retained_custody` body. See [Canonical Note Symbol Audit](canonical-note-symbol-audit-2026-11-21.md) for the attribution and its limits.

## Acknowledgement lifecycle

`NativeSchemaKeyFrame::idle` calls `terminal_is_empty()` before `reusable()`. For a bound reusable acknowledgement, the former used to scan the semantic frame and its receipt owner before rejecting the bound state. The latter then scanned both frames. Moving the inexpensive acknowledgement fields ahead of the frame checks removes one full semantic vacancy evaluation on that path. Moving these fields ahead in `reusable` also lets an occupied producer, owner, or pending Post reject reuse immediately.

`NativeRecordMatcher::selection` probes `rearm_authority_demand` before selecting the acknowledgement, semantic, or key action. Checking the seed, authority, destination, binding, and failure owner first prevents frame scans for probes that cannot rearm. `authority_close_demand` likewise avoids scanning either frame without an authority. Successful quotes still check both frames. The existing mutation methods still revalidate the quote and the exact Source, grant, and Work before making a paid change.

These four changes reorder pure Boolean tests on the same borrowed state. They preserve the predicate results, error strings, quote arithmetic, storage layout, and ownership checks. The call-count reduction is a source-level result. The new guest and speedup were not measured.

## Inline receipt owner

A vacant `ManuallyDrop<Option<OriginalReceivingPrimary>>` could avoid a full owner scan, but the current stage quotes only `2 * size_of::<Option<OriginalConstructionCompletion>>()`. A successful transition to `Some` must also initialize the other receipt-owner fields now born during fieldwise frame initialization. Existing terminal handoff materializes the full empty owner before stage validation and is unsuitable as a paid-construction precedent. The frame's slot size, frame transfer size, and retirement size are used in physical quotes. Layout equality alone would not cover the additional stage initialization.

The public host publication method receives `&mut OriginalReceivingPrimary`, whose other cause fields are writable. A structural retirement shortcut based only on receipt completion presence therefore also requires constraining mutation through that host boundary or proving custody of all independent causes whenever the frame transitions back to Empty. A future storage change should give the resident receipt its own narrow authority and quote the exact storage writes after validation.

## Verification

- Exhaustive evaluation of the old and new source predicates matched across 1,808 Boolean assignments. Source comparison confirmed identical error and quote bodies. `git diff --check` passed through `repo tools exec git`.
- The pinned Rust compiler parsed the receiving-frame source in Rust 2024 mode with `-Z parse-crate-root-only`. A linked Rust test build was not run.
- The pinned Bun 1.4.2 executable passed 31 tests from 12 existing language-neutral law files, including the Ajv and SQLite oracles for exact vacancy, bound reuse, acknowledgement rearm, refused payout, original receipt custody, and cancellation. The repo tool wrapper was waiting for host capacity, so the tests used the executable reported by `repo tools list --json`.
- Existing Rust cases cover a real Fresh action in an Empty phase, retained refusal even while Empty, receipt-cause pointer preservation on transfer, exact unreported work, zero-work denial, foreign Work denial, original payout refusal retry, authenticated publication, and paid retirement. The receipt tests are in `construction/effect/tests/retained-outcome.rs` and `outcome/receiving/tests/failure.rs`, alongside the receiving-frame marker tests.
