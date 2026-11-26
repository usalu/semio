# Modal and buckling in the 3D FEM browser (F-M)

## Causes

1. **"Mode n: 0.000 Hz", nothing deforms, caption ignores the mode index.** `with_result_fields` (session-api) lent the *retained static generation* for a Modal/Buckling display (its `_request` argument was ignored). Its child has `request.mode == None`, so `mode_values` returned `None` for every node and the render fell through with `eigenvalue = 0.0`: caption `Mode k: 0.000 Hz`, all-zero shape, same for every index.
2. **Buckling "Analysis failed".** The mounted numerical child had no buckling at all: `step_publish_stress_scalar` returned the fault `fem3d.numerical-buckling-owner-missing` for any `request.buckling`, which the caption maps to the generic `Analysis failed`.
3. **Modal "Computing modes" for minutes, root cause in the Subspace stage.** Per-unit counters on the demo (mode 0) showed: (c) the iteration never converged (residual 0.99 after 22 iterations), because for every pencil the job first computes a shift `sigma = ||K^-1||_inf * ||B||_inf` (an n-solve inverse-norm estimate plus a pessimistic bound) and iterates `K^-1 B + sigma I`; with `sigma` far above the wanted eigenvalues the convergence ratio is about 1. After 30 iterations the block `m` doubles and restarts (9, 18, 36, 72, 133), so every iteration and every checkpoint grows; and (a) a checkpoint was queued after EVERY iteration and rewrote the whole factor, `B`, `x` and five work matrices (385 M page-write units against 3 M units of arithmetic). A mass matrix is positive semidefinite, so the modal child needs no shift.
4. An unconverged or non-positive eigen solve left the visual state at `SolvingUnconverged` forever (never `ready`), i.e. an endless "Computing modes" instead of a result.

Observation (not changed): the dense engine's frame mass has no torsional inertia, so for open sections (HEA200, J = 2.1e-7) the mounted job finds a real torsion mode near 13 Hz that the dense `fem3d_modal` omits. The laws use the SHS150 section, where torsion (about 61 Hz) is above the compared modes.

## Fixes

- `session-api/🦀️.rs`: `with_result_fields` lends fields only when the child solved the requested mode or buckling index (`request.mode/buckling` must match); `mode_values` reads `request.mode.or(request.buckling)`.
- `📊️results/🦀️.rs`: a mode display with no resolved mode values renders the pending frame (`Computing modes`) instead of zeros; an eigenvalue <= 0 shows a localized caption (`no such mode` / `no buckling factor (no compressive load)`, German too); `Fem3dPageVisualLease` is re-exported for the live-session contract.
- `🗣️terminology/🦀️.rs`: labels `mode_unavailable`, `buckling_unavailable` (EN, DE).
- Visual job (`🎞️visual-job`): new stages `BeginBuckling`/`Buckling` assemble the negated geometric stiffness (`SecondaryAssembly` + `mounted_geometric_stiffness_cell`) and pair it with the elastic stiffness (`SymmetricPencilConstruction::new_pair`), then reuse the modal LDLT/subspace/read stages (`eigen_index()`); `eigen_unavailable()` ends a solve without a positive converged mode as a finished, shapeless `ValidatedFinal` result (no fault, no hang); the secondary owner closes through a pre-check in the close ladder (no lane renumbering).
- Subspace crate (`🔢️sparse/🎵️subspace`): `with_positive_semidefinite_right()` (zero shift: no inverse-norm bound, plain inverse iteration) used by the modal child only (buckling keeps the shift, its `Kg` is indefinite); `without_publication()` (the mounted child discards checkpoints and previews) and `with_publication_floor(n)` plus a `PublicationPace` ledger so a checkpoint or preview starts only after the arithmetic since the previous one has paid for its measured cost. Existing laws that need a checkpoint after every iteration use `with_publication_floor(1)` and reset the pace; all 17 subspace laws pass.
- Files: `🎞️visual-job/🦀️.rs`, `🧮️solver-view`, `🧾️contract`, `🧱️model-step`, `🪜️solve-step/🍓️staged-step/{🦀️,🎼️modal-and-publication-stages,🧮️solve-and-recovery-stages}`, `🪜️solve-step/🥦️close-ladder`.

## Laws and verdicts (native, private dir C:\t\mb)

| Law | Where | Verdict |
|---|---|---|
| mounted cantilever modes vs Euler-Bernoulli closed form and the dense engine, ascending, finite, positive (SHS150, 3 elements, lumped mass, mode 0 8.04 Hz vs 8.49, mode 1 9.85 vs 10.40) | visual-job `modal_buckling` | pass |
| two mode indices select different, non-trivial shapes | visual-job `modal_buckling` | pass |
| mounted buckling vs Euler load (fixed-free 201771 vs 201668 N, pinned-pinned 812739 vs 806671 N, 2 elements) | visual-job `modal_buckling` | pass |
| transverse load: finished shapeless result, no fault | visual-job `modal_buckling` | pass |
| 8-element cantilever dense frequencies within 2 % of closed form | engine `modal-buckling` | pass |
| demo modal: finite, positive, ascending, per-index shapes differ | engine `modal-buckling` | pass |
| demo buckling: positive ascending factors or explicit refusal | engine `modal-buckling` | pass |
| static-only generation never shows `0.000 Hz`; mode index and frequency reach the caption; buckling factor shown | results `🎵️mode-wiring` (editor_app_2) | see status below |
| `results_window_renders_modal_mode_shape_3d`, `results_window_renders_buckling_mode_shape_3d` | editor_app_2 | see status below |

The mounted modal laws take minutes in a debug build under load (the whole visual-job module took 1306 s with contention).

## Status

- Existing laws `results_window_renders_modal_mode_shape_3d` and `results_window_renders_buckling_mode_shape_3d` (editor_app_2): both pass (run one at a time).
- New results laws `mode_wiring` (editor_app_2, three tests): written, NOT yet run; the editor build kept being broken by concurrent edits (last: 2D `LdltJob::new` now takes 3 arguments in `eigen-stages`). Run: `mb-law.sh mode_wiring <tag> semio-s-artifact-fem-3d-editor editor_app_2`.
- Not verified: the browser (no rebuilt wasm). Demo modal through the mounted job was not run natively to completion; its cost is the subspace stage (see cause 3). Pre-fix evidence: Subspace units dominated LDLT 550x on small beams, so check the subspace for the same per-iteration checkpoint cost the LDLT had.
- Debug prints removed; `C:	\mb` is shared with another agent's runs and was left in place.

## Subspace cost (follow-up result)

Demo, mode 0, mounted child, native: before 400 k turns without terminal (Subspace 389 k+); after the zero shift and `without_publication()`: terminal in 28,115 turns in the settle law (demo static 4,960, slab 37,025), 8 iterations, residual 2.6e-7, no block doubling. The four mounted modal and buckling laws dropped from 1306 s to 46 s; the 17 subspace crate laws pass (`subspace-law.sh`). The `[DEBUG]` counters were removed. The earlier-seen failure of `positive_spectrum` did not reproduce once the laws that need a checkpoint use `with_publication_floor(1)`.
