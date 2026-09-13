# FEM PCG Retained Publication Next Slice

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

## Execution Contract

Define the preview/checkpoint/terminal wire envelope and scalar ordering in a closed language-neutral fixture/schema before production changes. A retained PCG writer must use the existing RetainedJobPayloadWriter and numerical page/cursor infrastructure, with one explicit admission or scalar write per fuel. Derive negative residuals and absolute displacement directly from the borrowed scalar, without temporary reactions/contour Vecs or a whole PcgPreview.

The checkpoint writer must cover the actual CSR owner, all seven scalar vectors, configuration, stage, convergence accumulators, numerical cursors, iteration and due flags. A retained restore cursor must reserve/copy these exact owners under bounded grants and return the same next numerical state. The cold checkpoint helper can drive the same codec to completion for test/batch use; it must not remain a synchronous fallback in interactive execution. Replace all actual consumers and fixtures together, without a legacy wire adapter.

Output writer state is a distinct owner from numerical state. Zero fuel, expired deadline and pre-cancel preserve numerical cursors, due flags, writer cursor, staged bytes and physical backing. Completion consumes the due state only after the retained output can transfer. Faults preserve the first fault and retain every partial writer/input/decoded owner until close. begin_close, close_step and terminal_is_empty must include checkpoint, preview, terminal and restore owners.

Preserve the shared8ms per-step deadline and watchdog policy. The original full-child200000-opportunity harness exhausted at genuine modal iteration6/30 before any PCG publication replacement. Its separate, source-dimension-derived completion allowance is documented in fem3d-shared-watchdog-authority.md; it is not a change to a production scheduler grant or an excuse for broken cursor progress.

## Required Evidence

Use strict Ajv and an independent existing byte/numeric reference for the same neutral envelope and scalar values. Cover empty and multi-page vectors, negative residual and contour derivation, exact page boundary, maximum-plus-one admission refusal, every publication cut, insufficient/zero cleanup grants, cancellation and exact physical release.

Drive actual PcgJob::step and retained restore from native tests. Preserve batch-equivalent convergence, exact checkpoint resume, coarse preview semantics, initial search-direction backing preservation and the publication grant corpus. Update old helper-based tests to explicitly close all retained outcomes and job owners. The full FEM3D route must still publish displacement, reaction and actual subspace eigen fields before exact bounded close.

The source belongs in sparse/🦀️.rs and its colocated schema/fixtures/tests. Existing root/ticket FEM routes and launch entries should include the new law, using Bun/Nx. No new script file or runtime library is needed.

## Root Source Refinement

PCG's actual state has one CSR plus seven scalar vectors: b, x, diag, r, z, p and ap. Its 29 checkpoint fields include numerical cursors, convergence accumulators and preview/checkpoint flags. The only direct PcgPreview decoding found under s is the sparse coarse-preview native test; FEM3D reads scalar views and later transfers the completed CSR. A new explicit numerical page envelope can replace the old generic Value envelope coherently, with no legacy decoder. Existing checkpoint-resume and preview tests must be updated together.

The shared NumericalPageCursor already encodes field, owner and item into a32-byte page header. However, advance_f64_owner and its u32/u64/paged/pair/matrix siblings write until a field ends without first committing a full page. The actual JobPayload page is16384 bytes:2043 f64 values plus length/header fill it, and the2044th requires continuation. The earlier4096-byte assumption confused this owner with PagedList/mesh backing and is corrected in fem-numerical-page-owner-implementation.md. Before reusing these helpers for PCG, complete the neutral2043/2044/4092-scalar and20 entry-width laws plus native byte/header/cursor observations, then make the shared helpers commit a full page before any next scalar or metadata write. The next header must describe the unchanged field/owner/item cursor. This is a shared numerical serialization boundary, not a PCG-specific workaround. Existing LDLT and subspace checkpoint owners are affected by source and need focused continuation coverage.

The current PCG preview_due and checkpoint_due flags are cleared before materializing their output; the retained replacement should freeze numerical work while its independent writer owns the due publication and clear that due flag only on successful transfer. The current Complete stage republishes on repeated step calls; the replacement needs an explicit terminal-publication witness. All writer states must be included in begin_close/close_step/terminal_is_empty, and diagnostic test helpers must close both the original and restored PCG owners.
