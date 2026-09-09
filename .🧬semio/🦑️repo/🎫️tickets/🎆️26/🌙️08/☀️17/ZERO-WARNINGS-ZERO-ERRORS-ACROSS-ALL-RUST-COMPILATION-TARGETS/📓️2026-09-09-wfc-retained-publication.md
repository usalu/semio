# WFC Retained Publication Failure

The fresh procedural assembly/WFC test build completed without compiler warnings or errors. Seven selected laws passed. The eighth, checkpoint_resume_preserves_rng_trail_and_progress, aborted while publishing its commit. The first panic was JobPayloadRejectedPage's exact-source-handback guard; dropping an already retained commit-state payload during unwinding caused the second panic.

The current WfcJob helper calls StepContext::payload_from_bytes and discards the rejection. The commit branch calls that helper twice in one step for CommitState and CommitOutput. StepContext admits only one payload page per step, so the second call returns OpportunityExhausted and the helper drops its source. The helper also only supports a single 16384-byte page, while WFC checkpoint and commit limits are each 1048576 bytes.

The existing framework RetainedJobPayloadWriter provides write_slice_page, retained rejection ownership, finish, and incremental close. The intended repair is a pending publication owned by WfcJob and WfcRestore, with separate state/output writers and byte cursors. It must copy at most one page per step, keep both streams until the outcome is complete, retain admission failures, and incrementally close partial writers, finished payloads, and source bytes on cancellation. It must not silently substitute an empty payload. The checkpoint/commit materialization and wire bytes remain the producers of the publication input.

Add neutral JSON vectors for payloads spanning page boundaries and single-page preview/checkpoint/fault cases, plus zero-budget and positive-budget close checks. Use the real BatchJobSession for multi-step publication tests so all pages share the operation ledger. StepContext::new creates a fresh private ledger per call, while BatchJobSession owns the persistent ledger. Its operation budget is 256 pages (4 MiB), above WFC's combined 2 MiB checkpoint/commit maximum.

No publication repair has been applied yet. The active laws770 runner has moved on to Writer artifact verification. Source and test repairs must be validated with a fresh WFC build and the exact failing checkpoint law before claiming a fix.

## Retained Publication Implementation

Replaced single-page payload conversion with an owned publisher used by both WFC execution and restore. It copies at most one page per step, retains quota-rejected source pages, holds completed commit state while output advances, and incrementally closes payloads and source buffers. Invalid operation identity returns an intrinsic fault without publishing under foreign authority. Preview cadence now includes its bounded publication step. Added neutral boundary vectors and three exact session laws for both commit streams, zero-fuel/rejected-source retry, and cancellation with one complete and one partial stream. Syntax parsing passed for four Rust sources; fresh compilation and runtime verification are pending.

- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/📤️publication/🧫️fixtures/🔣️.json
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/📤️publication/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/📤️publication/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/📜️script.ts

## Constrained Reference Search Cancellation

The two neutral grid solver laws failed because initial propagation completed the pinned checkerboard, and the reference search checked all-singleton before checking an already-cancelled token. Moved the existing cancellation check to the top of the drive loop, before completion, constraint rejection, and budget decisions. This changes the test-only shared search reference used by both grid oracle laws. Rust syntax passed; both native regression reruns remain pending. The skipped extraction law also reached the repaired WFC commit publisher and will be checked in the fresh run.

- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔍️search/🦀️.rs

## Native Stack Regression

The fresh WFC build succeeded, but the first checkpoint/resume runtime regression aborted with stack overflow on the ordinary test stack. Publication contains multiple fixed retained-page tables; embedding it directly enlarged both the WFC job and the restore job that owns a WFC job. Moved the pending publication behind Box ownership in the job, restore, and probe fixture. Payload ownership and incremental close stay unchanged. Parsed three Rust sources; default-stack native verification remains pending. No stack-limit override is used as a passing gate.

- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/📤️publication/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/📤️publication/🦀️.rs

## Retained Publisher Runtime Evidence

The fresh unboxed WFC build emitted 0 errors and 0 warnings. Across the original run and the SHA-verified continuation, 31 of 35 selected exact tests passed. All three new publication laws passed on the normal test stack, including two-page state/output, rejection retry, and cancellation with both finished and partial streams. The periodic extraction solve and both constrained grid cancellation laws now pass. Four restore-related tests aborted with stack overflow. The Box ownership change must be verified in a fresh build; these results are not claimed for it.

Executable SHA-256: ac3cf2dca071b0f7626de2dc56f361a1951ec49ef330ab4b032ff2e537a025bb.

## WFC Boxed Publication Fresh Runtime Result

The fresh build completed with 0 compiler errors and 0 warnings. All 35 selected exact native tests passed, including all four restore regressions that previously overflowed the stack, all three retained publication laws, extraction, and constrained grid cancellation. Executable SHA-256: 0fee5ecadabf06fb0755bdcb4ece9bebca47de50c376ce7a19b94d4f699d2fa1.
