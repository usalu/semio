# Peer Selection-Stage Presentation Review

Observed 2026-10-21T00:57:55.328Z. The immediate development target was never launched. This review executed no commands, tests, production functions, classifiers or SQL. Peer schema/corpus/test/runtime edits were preserved. Root separately authorized one minimal test correction described below; no runtime API or other Source file was changed by this reviewer.

## Observation And Exact Prior Comparison

At first read, schema/corpus matched executor28346/4c39… and157948/ecbc… while the peer test had already advanced from58997/5d5a… to60543/94696…, adding a third selection-stage cancellation test. Runtime initially still matched the reviewed52179/dd74… and therefore lacked the new classifier required by every changed actual-AST selection. Subsequent reads observed the peer authoring its runtime; the initial pairing was an incomplete test-first stage, not a candidate suitable for launch. Current two complete content-read rounds agree on the five fingerprints below. The later runtime53606/a696…→53695/311556…89-byte difference only assigns a dedicated helper docstring and restores the existing writer docstring.

Deleting only new selectionStages/SelectionStageClockCases from the current parsed schema and preserving its JSON formatting reproduces exactly26414B/65b34dd758cb9429b2e08689d8e419170f66b6b301548ab7bf7abaa8377a3b15. Removing only the new corpus selectionStageClockCases reproduces149000B/92d43d8138a58573354deaf40047a740a47f0879ecb7077e58c343976cd074d2. Removing the appended three new test bodies, new same-module cancellation import and new classifier name in existing AST extraction lists reproduces exact53576B/4707e8f48fd2b42e710bdeb86bce6b83c754ae1a4b1a20fd0f495bd506e1c8ec. This supports an additive schema/corpus/test scope; no old18test body logic or dependencyWork policy/cases were silently replaced. No Source snapshot was copied into the ticket.

## New Runtime Semantics And Interaction

The added classifier admits only three-key plain events with owner repository:build, stage running and one of five schema-enumerated selected-declaration/catalog-targets/command-sources/native-closure/compile-closure command strings. Activity-bearing, foreign-owner, extra-field and other commands bypass it. This is a finite explicit bookkeeping category, rather than a general rule suppressing every bare-running event.

It retains at most five latest stage rows, uses compressed ranks and one250ms presentation window, emits the first frame row, and replaces a due slot before draining latest observations. The eight neutral vectors cover rotating slots, same-slot latest replacement, outside-frame flush, backward/nonfinite clocks, foreign owner, extra fields and activity-bearing fallthrough. An independent SQLite recursive relation derives stage acceptance and latest-per-command output indices from actual row owner/command/stage/key-count/timing facts.

The existing dependency scheduler keeps its11slots/5000ms rules and old provider/cache/tooling schedulers retain their own paths. Entering a selection frame first flushes prior pending evidence and clears the old single-family key plus current command identity. A non-selection record drains all pending selection rows, closes that stage frame and then enters the existing writer path. Returning to dependency work sees an admitted command boundary and opens its appropriate new dependency frame. Thus selected command changes inside the five-member group become coalescible; other owner/command/stage transitions, unknown/activity records and actual settlement remain boundaries. The new scope intentionally changes how these five bookkeeping command transitions are presented; it is outside the earlier four-family-only dd74decision and requires the current new schema/corpus/tests to be sealed with it.

Only contiguous runs of this category are paced together. Arbitrary alternation with other emitted categories can still produce boundary flushes and first observations; no fixed whole-run output-volume guarantee follows. The old measured log had only167bare-running records, so it does not by itself establish this category is the main earlier host volume. Root may accept the narrowly declared feature, but actual output/cancellation/terminal evidence remains necessary.

The generalized ordered/drain/remember functions operate on the bounded stage array with family undefined, so they do not read dependency family metadata for stage slots. New stage rows are removed before their first sink write; later untouched slots remain pending if a write throws, consistent with existing attempt semantics. Presentation still uses the same raw JSON and observation-time message; no new receipt or aggregate elapsed claim is fabricated. No timer or background worker is introduced.

## Concrete Cancellation Finding And Authorized Correction

The peer’s third new test initially asserted that raw alias current.control.publish and current.control.yieldContinuation reject after additionalSignal abort. Actual withScriptInvocationSignalV1 spreads parent control and replaces its signal; publish remains the synchronous presentation writer and yield remains the continuation callback. Guard checks enforce alias cancellation; those raw callbacks do not independently promise it. Root and independent Light corroborated this interface mismatch. It was not an executed failure.

Root authorized only adding requireIssuedScriptInvocationV1(current) inside the two existing post-abort Promise.then callbacks before their respective publish/yield calls. I freshly matched60543/94696… immediately before that patch. Reversing exactly those two replacements reproduces the whole peer test. Current test is60645B/b4e7b52b8515a1b089defdd706d9eeac65f2e5b2ff8a28ee2a4b1aac95b3abb8. Its title, valid loop, exact primary rejection assertions, two callbacks/two continuations and pending final-flush output remain unchanged. The test now proves cancellation at the authentic checked boundary and does not invent a new raw-callback cancellation API.

## Exact Collection Contract

The previous combined phase17 selection contains exactly the original18literal titles and excludes the three added selection-stage titles. Existing old tests now extract the new classifier as part of the actual writer closure; they can exercise shared runtime wiring but cannot substitute for the three new selections. The current authored whole progress file has21tests in the exact order below. Root intends to expand the phase17literal list/count to21and preserve the earlier phase scopes; no integration or seal edit occurred here.

The supported @semio-tech/framework-process:test-process-progress development target runs this whole file with no caller title selector. Its existing router requires exactly the suite segment, and its process-progress entry has no fixed testNamePattern. Therefore the supported development target would collect21at current Source, while an unchanged exact18combined phase would collect18through its anchored title pattern. An all21result must not be mislabeled as evidence for a stale exact18sealed contract, nor does an exact18pass prove the added3tests.

1. `process progress uses neutral long-activity words and preserves the actual structured publisher`
2. `provider presentation follows neutral monotonic clock rows and independent SQLite scheduling`
3. `provider actual Original publisher preserves every checkpoint and flushes before failure settlement`
4. `provider counters use bounded localized actual writer messages`
5. `source admission work has localized bounded counters and keeps actual progress authority`
6. `parser source work has localized bounded operation counters at the actual writer`
7. `queue wait names the head request and position in the selected language at the actual writer`
8. `cache scan presentation follows neutral monotonic rows and independent SQLite scheduling`
9. `cache scan formatter uses bounded localized typed paths and independent SQLite messages`
10. `cache scan actual Original retains all checkpoints and primary failure if final presentation fails`
11. `cache deletion presentation keeps unit elapsed resets inside one paced original stream`
12. `cache deletion formatter names actual work without inventing aggregate seconds`
13. `cache deletion actual Original preserves every checkpoint across unit resets and exact primary error`
14. `build tool presentation follows neutral unit clocks and independent SQLite scheduling`
15. `build tool formatter uses closed localized activities and independent SQLite messages`
16. `build tool actual Original preserves every checkpoint, cancellation and primary error`
17. `dependency work presentation follows bounded interleaved frames and independent SQLite scheduling`
18. `dependency work actual Original preserves checkpoints cancellation and exact failure settlement`
19. `selection stage presentation follows closed neutral clocks and independent SQLite slots`
20. `selection stage actual Original retains every checkpoint and primary failure on presentation settlement`
21. `selection stage pending presentation cannot postpone actual Original cancellation`

## Direct Imports And Reads

The only new import name is withScriptInvocationSignalV1 from the same existing process implementation import. All actual AST readers still directly read that same implementation through the existing literal relative URL; they add selectionStageSlotV1 to their extraction lists. Schema/corpus imports are unchanged paths, TypeScript remains the existing test-only dynamic import, and Bun/SQLite/Ajv/local-validator interfaces remain the same. No new local file, runtime dependency, dynamic child entry or Source pin/critical path is introduced. Feature4754B/06a433… is unchanged. The new plain-event schema/corpus are language-agnostic test inputs even though no new Gherkin scenario was added to that unchanged feature.

## Current Stable Five-File Observation

- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json — 28346 bytes; SHA-256 4c39c86e9fc74576e914d32b7597a8ac7ff1a6c10f656cf782df1d20c659bbe7
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🔣️.json — 157948 bytes; SHA-256 ecbc0b93059b72f3971df7318484b475a4dbbd83e692b81317658bea08e8f5b9
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts — 60645 bytes; SHA-256 b4e7b52b8515a1b089defdd706d9eeac65f2e5b2ff8a28ee2a4b1aac95b3abb8
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🥒️.feature — 4754 bytes; SHA-256 06a43316e03b9195c19fab06f4c77974c5a766fae55d2cbecc837d14779a68dd
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts — 53695 bytes; SHA-256 311556b2a0ff76660098928dfd50e53bef78916b0fc8192415c357ff30381c8d

## Protected Self-Consumer Repair

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs — 37282 bytes; SHA-256 5eb20e1d0252a2d1756dd8df67ab11dec7d41303aca42cd6018039e290417ddc; unchanged
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts — 103363 bytes; SHA-256 7607b9a6d4003023def0c2372ac8f2716fd2a73598823cf7c112c182b19d1064; DRIFT
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json — 172838 bytes; SHA-256 8547b60ed14fcb4259510738c90380a7c7c15c6d9c1b6554aab317f6cfa19bb7; DRIFT

Disposition: coherent current authoring observed twice, minimal checked-boundary test correction complete, exact21roster and existing direct dependency scope handed to Root/Light for final independent review, integration and seal. No executed pass, complete Source seal or output-budget proof is asserted.
