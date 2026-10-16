# Rust Scan Timing Independent Current Review

2026-10-15. Source-only review of the qualified diagnostic appendix, actual measurement owner/schema, CacheWalk/scanner wiring, retirement execution and timing fixture. No runtime, canonical diagnostic preview, retirement, image access or production edits were performed.

## Findings

No actionable diagnostic-epoch finding in the inspected path.

Execution reads the exact SEMIO_CARGO_RETIREMENT_TIMING=1 switch once per retirement operation. Absent/other values pass undefined timing and instantiate no recorder; existing recency diagnostics are a separate flag. Initial scan remains build then incremental, and recensus invokes both functions again with new walkers after original physical/descriptor admission. No reuse of initial rows or measurements was introduced. Preview returns after the initial scan, so it produces at most two scanner lines; actual retirement has two initial and two recensus calls, at most four lines. The four-summary bound follows this execution call graph, not a global budget on arbitrary direct scanner invocations.

Each scanner emits one summary from its finally block. Formatting and console delivery are enclosed in catch-all suppression, so optional delivery cannot replace a callback Error, thrown undefined or actual cancellation failure. Scanner operations remain serial and individually awaited, and timing adds no descriptors, processes, timers or unjoined promises. A failure before body scanning may produce fewer records, which is consistent with an upper bound.

The schema is closed, with fixed scanner/phase/status strings, bounded root and nonnegative bounded counters/timings. The repository validator explicitly uses Number.isFinite for number types; integer fields also validate as integers. Formatter measures UTF-8 bytes for JSON plus fixed DEBUG prefix plus newline before emission and refuses values above 2048 bytes. Invalid/oversized summaries are dropped by report rather than emitted or allowed to replace the operation result. Root is the only variable string in the record; errors/file names/content are never serialized. JSON escaping plus complete console.log delivery produces a single logical line for each valid record.

## Phase And Counter Meaning

Discovery measures the original findNamedDirs pass; enumeration measures root/package/crate/session classification and result aggregation; measurement measures each complete unit/session subtree plus the incremental sibling lock observation. The recorder begins in discovery and switches at actual scanner boundaries. These disjoint stage spans sum to elapsed wall time modulo measurement overhead/roundoff. Stage values include awaited filesystem, checkpoint/progress and scanner computation in the active span; they are not independent CPU or pure filesystem latency.

Operation call counters increment immediately before the actual lstat/readdir or continuation block, so they count attempts, including failed attempts, rather than successful rows. Stat/measurement/entries timers include awaited system operation and post-operation control checks, exclude the preceding checkpoint; continuation time includes awaited progress publication, yielding and their checks. Operation timings are nested within stage times and must not be added to stage totals as additional work. With recency diagnostics disabled, measurement delegates to ordinary stat, so its requests correctly appear under stat rather than BigInt measurement. Incremental units summary counts both aggregate crate rows and stale-session rows, while retirement's current scan array uses aggregate crate units only; that difference should be retained when interpreting canonical summaries.

Complete status means that scanner reached its normal return, not that an entire later retirement settled. Failed status retains the count of rows already accumulated. The fixture checks fresh changed-file observation, exact callback undefined/Error and cancellation marker identity, stage-sum arithmetic, actual continuation activity and default silence using real metadata plus independent Python/SQLite/Ajv oracles. Those are meaningful Source test assertions; this reviewer did not execute them or independently reproduce the reported BOTH results.

## Limits

The existing scanner still uses readdir arrays and its established discovery/measurement coverage; diagnostics neither repairs nor expands that separate behavior. No canonical latency, contention explanation, budget sufficiency, full retirement success or SDK identity continuity follows from this review. Root owns the separately authorized live read-only preview and its results.
