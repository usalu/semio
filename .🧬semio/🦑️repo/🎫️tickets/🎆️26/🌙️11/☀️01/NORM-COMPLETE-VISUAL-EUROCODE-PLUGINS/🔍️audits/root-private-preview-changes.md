# Root Private Preview Changes

2026-11-05. This is a scoped inventory for this continuation, not the final complete ticket inventory.

Production change:
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs — ColdReplayEdit now admits actual outcomes, refuses Error/Fatal before application/inverse, preserves original diagnostics, stamps the real request operation index and retires its original cold diff once. Concurrent publication/work-reporting edits in this same source belong to the native worker and are preserved.

Tests:
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧪️tests/❄️cold-replay/🦀️.rs — existing full public-factory helper now exposes actual refused/native-message results while retaining all source/decoder/candidate close steps; new regression covers existing neutral severity cases at indices0/7, exact callbacks/diagnostics/next projection/inverse and original delta retirement.
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs — observes real cold Diff retirement through an independent thread-local counter.
- ✏️s/🔌️plugins/📕️norm/🧪️tests/🔮️catalogue/🟦️.ts — strict valid-neutral-case expectation now explicitly refuses a fixture without its expected missing-kind list; no fallback hides missing expectations.

The earlier root catalogue implementation/reconciliation is documented separately in oracle-catalogue-ownership.md and generated/catalogue-reconciliation.json; the low worker records its exact fixture, registration and source repairs in current-audit-changed-files.md. SDK borrowed source changes remain documented in borrowed-choice-source.md.

Verification: native54126 terminal0 passed the new regression; native62122 terminal0 passed all4 cold replay tests. Structured native catalogue12103 terminal0 passed1 test, all15 registration76585 terminal0 passed3/61; low worker's strict TypeScript successor passed with zero diagnostics. These are focused proofs, not all15 native or browser end-to-end acceptance.

Scheduling: matrix17230 was deliberately SIGINT-cancelled at its own door, terminal130, to unblock focused prerequisite tests after real head-of-line memory contention. No permission prompt, source-cap change, global resource override, modifying git command or other agent process action occurred.

Persistent reports updated: STATUS.md, EXECUTION-PRESSURE.md, private-preview-semantics.md, oracle-catalogue-ownership.md and this inventory.
