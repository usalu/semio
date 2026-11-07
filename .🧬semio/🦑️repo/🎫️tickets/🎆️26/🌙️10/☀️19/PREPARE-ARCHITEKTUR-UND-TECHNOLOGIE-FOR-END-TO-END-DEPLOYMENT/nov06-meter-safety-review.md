# Native Meter Safety Review — November 6

Bounded read-only source review requested by the coordinator. No implementation, test execution, process restart, capacity change or filesystem tree traversal was performed. This report is the only created file. Existing JS 55996, Python 15268 and Tectonic 70762 remain outside this review's mutations.

## Decision

Do not approve a timestamp-only change as a complete safe correction. The explicit meter-local completed-interval observation contract is defensible as a deliberate bounded reuse policy, provided the coordinator accepts the extra external-writer observation lag and the tests below pass. It does not establish a hard instantaneous physical ceiling against arbitrary concurrent external writes. Existing scans are non-atomic and already accept early-entry observations on first return. The proposed change adds up to two seconds after completion to that existing observation age; it cannot honestly be called unchanged freshness semantics.

The proposal correctly preserves usage.fresh, commit's post-arrival measurement, trailing singleflight, physical mapping checks and independent cancellation. However, its assertion that admitted deltas remain charged needs a separate concurrency correction or proof: the current meter can subtract the same admitted baseline more than once. Usage singleflight does not serialize meter state transitions.

## Concrete Concurrent Accounting Counterexample

Directory meter line 101 snapshots before=admitted before awaiting measure, then applies admitted=max(0,admitted-before) at completion. There is no meter mutex. Borrower operation admission tracks pending operations and runs them concurrently; it is not a serializer. Usage serializes filesystem walks, while concurrent fresh callers queue for the next walk and still retain their own earlier meter snapshots.

Use units scaled to valid byte limits, requests below the sixteen-MiB threshold, and a fake clock keeping the final check young. Start with physical usage 10 and admitted 10 from a completed reserve/write. Concurrent refreshes A and B both capture baseline 10. A returns measured 10 and subtracts 10, leaving admitted 0. B's trailing scan observes the relevant entry before a new writer's materialization and will also return 10. Between A and B completion a reserve of 20 succeeds and records admitted 20. B completes and subtracts its old baseline 10 again, leaving admitted 10 instead of 20. A following reserve of 80 against maximum 100 can be admitted from estimate 10+10+80=100 although retained estimate should be 10+20+80=110. The reservation's write can occur after the scan observed that entry; no atomic snapshot is assumed. A larger old baseline can erase the entire new reservation. These are ordinary asynchronous interleavings, not overlapping filesystem walks.

This is source-proven meter accounting risk, not a demonstrated live failure. It predates the proposed completion timestamp. A completion cache makes the lost balance reusable for the declared window, so it must not be waved away as unrelated when asserting preserved accounting.

A possible remedy is a monotonic cumulative admission watermark with a monotonically advanced covered watermark, so concurrent observations never retire the same debit twice. Precisely define when a debit becomes covered; completion alone does not prove an earlier reserved but unwritten debit materialized. Do not install an unreviewed formula merely because it fixes this arithmetic example. Alternatively serialize meter transitions, but prove that commits joining that queue still receive post-arrival scans, reserve state is updated atomically, cancellation cannot strand the queue, and simultaneous commits retain the existing bounded two-walk behavior. A blanket mutex may change that performance contract.

## Required Neutral Regression Vectors

Retain all vectors in Proposed Slow-Scan Contract For Independent Review, with schema-validated deferred scans and a deterministic clock. Add explicit interleavings for two refreshes sharing one old admitted baseline, a new reservation between their completions, and several callers sharing one trailing result. Assert retained bytes and final capacity refusal, not only walk counts. Repeat with cancelled and failed callers, and a commit interleaved with reserves. A third-party SQLite oracle should independently track admission identities or cumulative watermarks and covered debit sets, rather than reproduce mutable subtraction. Scaling the counterexample must remain below refresh thresholds and within schema limits so refusal cannot accidentally be rescued by an unrelated forced scan.

Also cover reservations admitted before a refresh but not yet physically written, to state whether the current contract requires serialized writer materialization or whether outstanding reservations must survive. The proposal's phrase reservations admitted since observation began is insufficient to resolve this older outstanding-debit question.

Verify exact completion+1999/completion+2000 boundaries; long first scan plus three rechecks; post-own-write commit while another scan runs; final over-cap refresh/refusal; unsuccessful refresh preserving timestamp and charge state; one subscriber cancellation versus last-subscriber abort/drain; identity substitution despite young observation. Do not replace fresh with recent. Existing tests do not cover the old-baseline double subtraction: the simultaneous commit case contains no interleaved admitted reservation.

## Alternative Scope

Same-reservation reuse can reduce the affected pre-hook sequence, but only with explicit lifetime and accepted-write invalidation coverage. Image shared custody is not proof that no writer exists. This alternative does not inherently fix the concurrent counter subtraction and still needs an external-writer validity policy. It is therefore not established safer from the inspected source alone.

## Inspected Sources And Imports

All paths below are relative to D:/semio. Native root is 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage.

- Native root/📁️directory/🟦️.ts, lines 97–114: meter construction, refresh snapshots, reserve/commit/recheck gates. Imports ./📏️usage/🟦️.ts and ./📋️contract/🟦️.ts.
- Native root/📁️directory/📏️usage/🟦️.ts, especially lines 101–140: running/queued slots, fresh and subscriber cancellation; inspected actual bounded walker source as part of this module.
- Native root/🧱️fixed/🟦️.ts, createFixedReceiverImage around lines 100–105: image-local meter; physical recheck before and after meter.admit; operation delegates to borrower admission. No meter serialization at this boundary.
- Native root/📥️borrower/🟦️.ts, line 26: concurrent pending borrower work with lifetime drain, no mutual exclusion around operation bodies.
- Native root/📁️directory/🧬️schema/🔣️.json: closed Protocol meter interval 2000 ms / 16777216 bytes, MeterRequest and MeterLimits.
- Native root/📁️directory/🧪️tests/🟦️.ts and 🧫️fixtures/🔣️.json: sequential capacity/lifecycle vectors and independent SQLite comparisons.
- Native root/📁️directory/📏️usage/🧪️tests/🟦️.ts: own-write trailing scan, recent cache, concurrent commits/cancellation, and independent metadata/drain oracles. Concurrent commit test uses a 20-ms delayed scan and does not interleave reserve debits.
- Ticket/nov06-prerequisite-native-routes.md: Bounded Pre-Hook Trace; Meter Age And Finite Repeated Work; Proposed Slow-Scan Contract For Independent Review.

No live-job awaited function, timing, measured bytes, physical-cap violation or livelock is claimed. Coordinator retains the adoption decision and all runtime authority.

## Targeted Review Of Implemented Correction

Re-read the current meter implementation, directory protocol schema, usage fresh boundary, and all three new 📁️directory/🧪️tests/📏️meter files after the owner's reported green. This was source-only; the reported 10 tests / 94 assertions and 6 tests / 78 assertions were not independently executed here.

No concrete correctness defect was found in this bounded correction. Monotonic BigInt admitted and covered watermarks remove the earlier double-subtraction counterexample: overlapping successful refreshes advance coverage to the maximum captured watermark rather than each subtracting a mutable balance. New admissions remain retained until a later observation captures their watermark. The requested/observed sequence prevents an older out-of-order refresh from replacing newer installed usage, coverage, or completion time. Success installation follows measurement validation and signal.throwIfAborted, so cancellation/failure does not publish a fresh epoch. Admission and coverage remain exact beyond cumulative Number precision, while the schema bounds individual byte scalars.

The source still requests usage.fresh for every commit. It does not replace fresh with recent, merge new callers into the already-running scan, change usage trailing singleflight or waiter cancellation, or alter the physical checks around meter invocation. Recheck completion reuse is explicitly declared by freshnessAnchor=completion, observationKind=completed-interval and admissionAccounting=monotone-watermark. The neutral slow vectors cover immediate three-check reuse, 1999 ms reuse and exact 2000 ms refresh, including a 90-second observation. Overlap regression now requires the extra final capacity-confirmation walk and refusal, with a separate SQLite debit-identity oracle. Newer-result ordering and unsafe/exact scalar cases have direct vectors.

The owner and coordinator explicitly retain the existing pre-scan unwritten-admission boundary. Its new direct test documents that a successful observation can retire an admission captured before the scan even when the caller has not materialized its write. Thus this correction is not a general outstanding-reservation ledger and must not be presented as guaranteeing instantaneous ceilings against delayed or unregistered writers. That agreed boundary and the non-atomic external-write observation lag remain the material limits recorded above. No new canary or live-process action was performed by this review. Within those stated limits, no additional blocker to the coordinator's bounded current-Source canary was identified.
