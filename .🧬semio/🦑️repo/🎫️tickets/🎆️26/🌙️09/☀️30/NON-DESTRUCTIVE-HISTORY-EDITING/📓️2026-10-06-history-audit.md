# 📓️ Core History Audit — 2026-10-06

Read-only source audit. Root `AGENTS.md` read. No tests, builds, browser runs, generated outputs, or modifying Git operations performed. Findings distinguish current source from older reports; no runtime correctness claim is made.

Paths below are repository-relative. `STORE` = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`; `TT` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`; `PLG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`.

## Existing Implementation

- Artifact-neutral draft editing exists in `TT:746–816`: obtains `input_schema`, reads descriptors, validates, rebuilds through `with_payload_value`, and encodes the native mutation. Foreign-step and cross-document-unit mutations intentionally refuse local edit; withdrawal has its own admission law.
- Preview excludes downstream operations: `TT:818–839` requests `derived_snapshot_before(target, acceptedDrafts)` and folds only the edited operation. Store owner is `STORE:19377–19403`.
- Accepted replacements replay downstream incrementally: `TT:694–708`, `TT:842–866`; completion retains the derived head and a finished replay token. Finalize consumes that token and refuses mismatched drafts (`TT:715–731`).
- Store finalization fences content revision and open transactions, rejects blocking outcomes, and authors overwrite or alternative (`STORE:21581–21622`). The authoring path checks that the finished replay agrees with the prospective fold; otherwise it reruns/defer-replays (`STORE:21625+`).
- Pure lifecycle reducer has accept/discard, withdrawal/restore, rerun, generation fences, finalization prompt, overwrite/alternative, and fault/cancel paths in `🧰️framework/🔨️modules/⏪️time-travel/🟦️.ts:289–430`. Tool runner rest-with-open-transaction refusal and host abort/reset exist in `🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs:287–436`.

## Actionable Gaps

### P1 — Opening/Changing a Draft Still Performs Unbounded Synchronous History Work

`TT:818–819` synchronously obtains the preview base. `STORE:19382–19386` copies the applied-history IDs and calls `prefix_state_recorded`. That helper (`STORE:20795–20824`) computes prefix digests for both the changed prefix and the entire live history before choosing a cached snapshot. On a cold/mismatching prefix it calls synchronous `fold_recorded` with no deadline or cancellation parameter. Replaying downstream is sliced, but constructing its prefix/preview remains capable of freezing the editor. This is a current source finding, not merely the S4 performance report.

First execution task: introduce a bounded preview/prefix cursor using existing derived snapshot ownership and retirement, with progress/cancellation and generation fencing; keep the prior visible snapshot while pending. Add a language-neutral corpus plus independent oracle measuring examined operations/digested units for small and large histories, interrupted draft replacement, cancel, and remote base movement. A wall-clock-only test would not establish size independence.

### P1 — Supersession Planning Scans All History Before Sliced Replay Can Begin

`STORE:21755–21770` builds a map of every applied mutation for `supersede_inputs`; `STORE:24109–24131` scans every applied edit to find changed supersession positions; `fill_supersession_targets` begins at `STORE:12880`. `begin_report_replay` (`STORE:19409–19417`) invokes replay-window planning synchronously. These support the same editor responsiveness risk independently of downstream stepping. Replace scans with maintained mutation-to-edit/position indices and changed-target lookups, or make planning itself a cancellable cursor. Add work-count laws, not only result parity.

### P1 — Prove Current Runtime and Tool Ownership, Do Not Treat Old Counts as Closure

`📓️s5-resume.md:89–129` quotes S4's runtime law run as 75 passed / 15 failed, then documents fixes written after that run. `📓️s5-nested-report.md` records a later S5 composed-family run and explains remaining tool-run abort/retirement and row-label failures. The root should use the latest `📓️s5-plugin-law-run.md` and a fresh current-tree filtered run to decide which remain; this audit did not run tests and does not assert those reds remain today. Prioritize abort settling, one gesture/one transaction, retained child retirement, and actor-correct undo before claiming every editor end to end.

## Stale Audit Findings That Must Not Be Reimplemented Blindly

- S4 resume says nested owner paths were open, but current `PLG:9969–10090` contains `MemberKey.owner`, `MemberPath`, canonical escaped path parsing, and registry resolution at `PLG:10276`. `📓️s5-nested-report.md` records N1/N2 landing. Audit the current recursion laws rather than adding another path representation.
- S4 report says finalization actor assignment was open; current `TT:717–718` explicitly sets the store local actor before commit. Actor routing still merits end-to-end verification, but this exact historical omission is fixed in source.
- The pure time-travel/tool-machine mechanisms already exist; a rewrite would discard substantial schema, corpus, Rust/TypeScript twin, and oracle work. Focus on bounded planning and current verification.

## Recommended Root Work Order

1. Run the current schema/input editability census and existing lifecycle/tool/store conformance through registered Bun/Nx targets; record real commands/counts in this ticket.
2. Fix bounded prefix/preview and supersession planning, with independent work-count laws and cancellation tests.
3. Re-prove current runtime families: draft preview → discard/accept → warning/fatal replay → edit/withdraw conflict → overwrite/alternative → undo/reload, including remote base changes and composed members.
4. Coordinate with editor coverage audit for browser-level per-editor witnesses. No source-only finding here proves UI end-to-end completion.
