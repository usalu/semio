# PCG Retained Wire Owner Layout

## Current Acceptance — 2026-09-13 03:10 UTC

The retained PCG codec and all discovered consumers are implemented. The latest completed native run, green3, passed **12/12 tests** on the normal 2 MiB stack (974 filtered, 0.27 s Rust, 23.5 s Nx). It includes 52 malformed checkpoint cases, all 173 publication cut points and all 125 restore cut points across all 11 checkpoint owners. Refused grants preserve cursor and backing, first faults remain sticky, and actual allocated backing equals physical release after terminal close.

The 18 newly added non-finite/negative numerical cases first produced a real runtime RED: 10 passed and one corpus test failed because all 18 inputs were incorrectly accepted. The production restore now validates finite control accumulators and individual CSR/scalar values as they arrive, and rejects negative residual norms/squares. Green3 accepted the corrected behavior.

The composed FEM3D route also passed after the retained PCG switch: lower 1/1 and upper 14/14, normal 2 MiB, 35.3 s Nx. Its actual run reached ready/static/modal/terminal/closed, 249602 turns, PCG iteration 11, subspace iteration 8, close lane 22 and three child closes, with zero watchdog overruns. This predates the restore-only finiteness validation and subsequent test additions.

The new dense 64×64 checkpoint exercises real CSR index and value continuation pages and compares the resumed solution with an independently derived exact dyadic reference. The empty checkpoint law checks zero-order ownership and complete release. These fixtures and the private checkpoint serialization-derive removal pass parsing; neutral6 passed the full schema/oracle and strict TypeScript in 1.7 s. Native-green4 stopped during compilation at the concurrent WindowConfig loader API cutover (6.6 s Nx); neither new native law ran. The loader agent has corrected the observed generic capability bound and continues its declared cutover. A retry remains required. Neutral7 subsequently passed in 2.4 s with the seven RHS norm cases added; the RHS correction itself is schema/test-only pending actual RED.

| Evidence | Observed result |
| --- | --- |
| Native red1 | 5 passed / 4 failed: eager publication, repeated terminal transfer, wrong identity acceptance and allocation before refusal |
| Native green1 | 9/9; checkpoint 11 pages / 93 one-fuel turns, preview and complete each 5 pages / 40 turns |
| Native green2 | 11/11; initial 34 malformed cases and all 173 publication cuts |
| Scalar admission red1 | 10 passed / 1 failed; all 18 new numerical-invalid inputs wrongly accepted |
| Native green3 | 12/12; 52 malformed cases and 125 restore cuts |
| Neutral6 | Passed 1.7 s, including dense reference, Ajv, DataView, fast-json-patch and strict TypeScript |
| Composed FEM3D after PCG | Lower 1/1 and upper 14/14; actual terminal close and no watchdog overruns |

Logs are under `🗑️generated/fem-pcg-publication-*.log`, `🗑️generated/fem-pcg-scalar-admission-native-red-1.log` and `🗑️generated/fem3d-pcg-retained-native-1.log`.

## Implemented Ownership

The numerical checkpoint, optional publication writer and once-only terminal witness are separate owners. Each granted step initializes/adopts a page, emits one header/length/scalar/field transition, or transfers a completed payload. Preview reactions and contours borrow individual numerical values without copying whole vectors. Pending flags clear only after transfer. Writer pages close before numerical backing.

PcgRestoreCursor owns input pages and its partial candidate, checks exact operation/revision/generation/seed before allocating candidate backing, then validates control words, CSR structure and all seven scalar arrays incrementally. The first fault is sticky. Input pages and partial typed backing stay owned until caller-granted close.

Generic whole-Value PCG checkpoint/preview encode/decode entry points, the copied PcgPreview representation and private checkpoint serialization derives are removed. Native consumers now drive the same retained codec and explicitly close outputs and jobs.

The retained constructor and restore use the existing 4096-byte scalar backing policy with before-allocation refusal. The eager cold constructor supports larger inputs under a distinct path; this is not a universal numerical solver maximum. Local backing accounting is not a shared process allocation authority. The separate working-memory authority remains required. No stack, watchdog or allocator ceiling was increased.

## Checkpoint

Use the existing numerical32-byte page header and16KiB retained payload pages. The magic is `FEMPCP1\\0`, kind13, existing header version1. Field0 is a26-word control owner; fields1/2/3 own CSR indptr/indices/values; fields4–10 own b/x/diag/r/z/p/ap. All scalar arrays use the already accepted shared length/continuation cursor. One operation emits one header, length, scalar or completed-page transition; page reservation remains separate.

Each control word occupies8bytes. Integer narrowing is checked, booleans require0/1, stage requires a declared discriminant, and f64 values use IEEE754 bits. The control order is:

| Index | Field | Value |
| --- | --- | --- |
| 0 | operation | u64 |
| 1 | revision | u64 |
| 2 | generation | u64 |
| 3 | seed | u64 |
| 4 | order | usize |
| 5 | tolRel | f64 |
| 6 | maxIter | usize |
| 7 | batchUnits | usize |
| 8 | stage | u8 |
| 9 | bNorm | f64 |
| 10 | residualNorm | f64 |
| 11 | residualSq | f64 |
| 12 | rzOld | f64 |
| 13 | rzNew | f64 |
| 14 | dotAccum | f64 |
| 15 | alpha | f64 |
| 16 | beta | f64 |
| 17 | iteration | usize |
| 18 | cursor | usize |
| 19 | rowCursor | usize |
| 20 | entryCursor | usize |
| 21 | rowSum | f64 |
| 22 | converged | bool |
| 23 | coarsePublished | bool |
| 24 | previewDue | bool |
| 25 | checkpointDue | bool |

The checkpoint source currently has29 fields: one CSR, seven vectors and21 numerical/control values. Adding operation/revision/generation/seed plus explicit matrix order produces the26 control words above. No local writer, restore, close, allocation or physical page cursor is serialized as numerical state.

## Preview and Terminal Output

The `FEMPCG1\\0`, kind3 envelope has one ten-word control owner: operation/revision/generation/seed/order/stage/quality/iteration/residualNorm/converged. Four scalar owners carry displacement, residual, reactions and approximate contours. Displacement/residual borrow x/r. Reaction and contour write one derived scalar directly (-r and abs(x)) without constructing temporary vectors or PcgPreview.

The existing quality rule remains: converged→Final; otherwise coarsePublished→Coarse; otherwise Initializing. Terminal delivery needs its own local once-only witness. Due flags stay true while their writer is pending and clear only when the complete retained payload transfers.

## Restore and Cleanup

A checkpoint candidate owns the incoming payload, shared NumericalOwnerRestoreCursor, a fresh checkpoint state, identity and first fault. It validates coordinates, canonical lengths, exact kind/magic, identity, stage/bool/integer bounds and matrix/vector dimensions before publishing. CSR pages and vector allocation demand stay separate; every rejected/partial owner drains through bounded close. The cold convenience path, if retained, must drive this same codec and cleanup to completion rather than remain a generic Value decoder fallback.

Current interactive construction enforces a4096-byte vector capacity after allocation, corresponding to512 f64 values. That source limit is separate from16KiB serialization pages and the768-slot mounted RHS producer. Before fixing a retained restore limit, the constructor admission must move its bound before allocation and the writer/restore must share an explicit solver policy. Do not silently infer that the cold `PcgJob::new` already enforces the same order bound; it does not. CSR entry maxima and total payload admission must also be validated against the selected policy.

## Required Next Evidence

Actual runtime RED for the three admitted first-publication cases; canonical control and scalar byte fixtures with an independent DataView/JSON Patch oracle; multi-page and derived-scalar cases; exact resume and numeric reference parity; sticky faults; cancellation at every writer/restore phase; insufficient physical close grants; once-only terminal publication. Existing helper-based PCG tests must retire both original and restored owners explicitly. Full FEM3D numerical output/cleanup must pass after the codec switch.

This plan does not modify runtime limits, protocol constants, schemas or production code. The numerical writer/reader continuation prerequisite and paged-model allocation slice are already accepted separately.
