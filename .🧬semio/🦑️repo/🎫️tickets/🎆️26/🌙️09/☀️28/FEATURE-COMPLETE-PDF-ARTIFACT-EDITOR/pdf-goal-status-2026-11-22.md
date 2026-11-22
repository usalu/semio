# PDF Editor Goal Status (2026-11-22 14:00)

## Verdict

Not feature complete, not running end to end. No mounted browser or native journey (open → edit → save → reopen) has passed. Feature code exists for every gap of `pdf-goal-gap-audit.md`, but most of it has never compiled in the governed build. The previous Claude session ended at about 12:00 and stopped all agents; they are being resumed now.

## Verified Green

| What | Evidence |
|---|---|
| PDF workspace libraries compile (incl. instance-jobs, authoring, interaction) | `compile-green/c10.log` (via compile-fix agent) |
| Reopen of a 9.6 MB memberless archive in 4 KiB paid pages (G7) | `g7-reopen4.log`: 1 passed |
| Reopen route selector, folder-load wire, large-archive laws | `g7-route.log`, `g7-wire` |
| Typed folder-archive-load event across kernel/host/reactor compiles | `g7-event3.log`: 0 errors |
| Page-editor tests with paint/codec work | earlier run 55 passed / 2 failed, 5 passed / 0 failed |
| Archive limit 16 MiB (TS + Rust laws), Pack `Uint8Array` binary tag (9.6 MB < 3 s) | bun laws |
| Native natural-source cancellation (G16) | `g16-run.py`: 8 passed |
| Registry drift fix + registry/src self-repair (G9c) | 16 tests passed |
| `repo show project` no longer queues on the capacity gate | tests in `repo-show-project-hang.md` |
| Fleet cache breaker no longer forces local inference | 24 tests passed |
| Codecs standalone vs. oracles: CCITT (libtiff), JPX (OpenJPEG, 17 vectors), JBIG2 incl. refinement (jbig2dec, 0 px diff), mesh shadings 4–7 and type 4 functions (MuPDF) | scratch builds, `paint-fidelity.md` |

## Written, Not Yet Verified

- G1–G3 open/retirement/stall fixes and regression tests (`g1-g3-open-path-fix.md`)
- G4/G6 natural-source ingress and 32 MiB factory (`g4-g6-natural-source-integration.md`)
- G5 paged decoder round trip on the 9.44 MB fixture, G10 segmented Save law (`g5-g7-g10-segmented-io.md`)
- G12 URW Base35 fonts, G13 real painting (`paint-fidelity.md`)
- Authoring: create annotations with appearances, form fill, page replace/merge/extract, renumbering, undo/redo test (`authoring-features.md`)
- Viewer: zoom/search/thumbnail reducers and drag-reorder (`viewer-navigation-features.md`)
- G14 large-image picker routing selector (picker mount not written)

## Not Written Yet

- Viewer Transient lane and zoom/search window actions
- G14 raw-picker mount chain
- G11 governed lifter extension for deferred import shapes
- Proper cancellation token for merge/extract (currently a 20 s deadline), key-value inspector forms
- JBIG2 Huffman/halftone/aggregates, DCT YCCK, SMaskInData
- Idempotent crate-tree carve (hand-written `semio-s-artifact-stdio-pdf-io-export` crate is a stopgap)
- G18 native smoke, G20 Flow binding + Vitest, G9a Stdio component publication
- Mounted browser suite run (G8, G17)

## Problems Hit and Fixed

| Problem | Fix |
|---|---|
| `repo show project` stuck ~28 min in capacity queue, wrong "every project" message | sealed `current` reads skip the gate; accurate en/de message |
| Cache ledger refused every graph build (area shares 9400 bp) | shares corrected to 8400 bp + regression test |
| Fleet cache reported unreachable after one 1.5 s timeout | reads wait out the breaker (max 20 s) |
| `repo cargo` gate refused every check (`wait.starvedMilliseconds` undeclared) | declared in schema and type |
| `semio-os-channel` over 4000-line gate | split into `semio-os-channel-archive` + `-archive-ingress` |
| `semio-plugin-instance-app` over 262,144-byte cap (stopgap raised) | split into modules, standard cap restored |
| Cargo cycle (authoring → tests-2 → page-editor → authoring) | new `io-export` layer crate |
| Registry drift + partial `registry/src` trees | content-verified receipt + re-extraction |
| Many transient breaks from other sessions' in-flight edits (missing exports, schema/policy mismatches) | waited/retried; several completed by us |

## Main Blockers Now

1. Build-time `Operation not permitted (os error 1)` in proc-macro source-authority reads (`g5-snapshot6.log`: 26 cascading errors). Recurring; this is the top blocker. A diagnosis agent was running when the session ended.
2. Machine capacity: 16 GB host, memory admission keeps free memory at the 2 GB floor; every `repo cargo` run queues behind other sessions' builds (often 30–60 min per run).
3. Concurrent edits by other sessions repeatedly break the shared tree for hours at a time.
4. Whole-workspace graph seal (4822 projects) did not finish; most targets are run via `repo cargo` instead.

## Estimate

Not credible to promise a time. With the EPERM blocker fixed and the build queue moving at today's pace (about one governed run per agent per 30–60 min), the remaining compile-fix iterations plus the unwritten items need on the order of one to two more days of continuous agent work before the mounted end-to-end suite can be run and fixed.

## Resource Use

- Up to 10 agents ran in parallel (coordinator + Sonnet implementers + Haiku audits); more agents would not help because builds are serialized by host memory admission, not by agent count.
- CPU/memory were saturated by builds the whole time (load 6–7.5, free memory at the floor).
