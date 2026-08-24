# P6i Fixed-Owner Independent Re-Audit

Date: 2026-08-24
Auditor: Codex `/root/p6h_audit_remediation/p6i_fixed_owner_reaudit`
Scope: current live tree; read-only P6i/P6h source/static re-audit.

## Verdict

**GREEN — no concrete P6i fixed-owner source/static counterexample remains in the mounted FEM3D route.**

This is limited to the requested source/static gate. It does not claim Cargo, Nx, Wasm,
browser, native rendering, or broad runtime verification.

## Governing Material Reviewed

- Root `AGENTS.md`.
- `p6i-fem-live-visual-publication-repair-contract-2026-08-24.md`.
- Terra's preceding RED: `terra-p6i-post-red-independent-source-static-audit-2026-08-24.md`.
- Current implementation report: `sol-high-p6i-fem-live-visual-publication-implementation-2026-08-24.md`.
- Accepted P6h materials: `coordinator-third-p6h-source-acceptance-2026-08-24.md` and `terra-phase-6-p6h-fresh-acceptance-audit-2026-08-24.md`.

## Mounted Production Trace

FEM3D editor model/results dispatches both borrow the mounted lease through
`live_visual::with_live_visual` (`editor/component.rs:529-530`). The retained state invokes the
real numerical child (`session/component.rs:2650-2664`), which retains the mounted mesh at
`:1061-1075`, mounted analysis construction at `:1169-1184`, PCG/LDLT/subspace, physical modal
mass, reaction recovery, and then builds the typed World3d lease. The World3d consumer advances
one typed page item through `step_world3d_snapshot` (`world/component.rs:8844-8872`).

The numerical graph is genuine: Tet4 insertion and physical lumped mass are retained at
`session/component.rs:908-958`; reaction is `K_full*u-F_full` at `:1556-1572`; the modal result
comes from `SubspaceIterationJob` rather than PCG aliases at `:1612-1650`; and visual fields are
written to the distinct displacement/residual/reaction/contour/mode pages.

## Former RED Findings Rechecked

| Owner/invariant | Current evidence | Result |
| --- | --- | --- |
| Document nodes, IDs, elements, supports, solids | `MountedAnalysisModel` and `FixedSlots` own bounded arrays; one-slot admission precedes one copy (`session/component.rs:397-475`, `:616-656`; `analyses/component.rs:100-177`). | Pass |
| Outline, holes, meshed points/triangles, solid IDs/indices, Tet4 | `MountedPlanarDomain` and all solid owners use retained fixed slots; copy/admission and Tet4/mass stages are separate (`session/component.rs:748-982`; `mesh/component.rs:36-165`). | Pass |
| RHS and modal mass | `MountedScalarSlots` admits, initializes, updates, transfers, and closes scalar slots one at a time (`session/component.rs:1196-1238`; `sparse/component.rs:429-493`). | Pass |
| Maximum +1 / producer handback | Exact guards reject before mutation; pending element, Tet4, meshed-solid, and PCG rejection paths restore their producers (`session/component.rs:341-386`, `:739-746`, `:947-952`, `:969-982`, `:1508-1518`). | Pass |
| Fuel/cancel/deadline before local numerical work | Non-delegated work checks cancellation/yield, consumes fuel, rechecks cancellation, and only then reserves/copies/updates/constructs (`session/component.rs:1023-1065`, `:1124-1160`). Delegated P6h jobs own their step contexts. | Pass |
| No model flattening at mesh/assembly boundary | Mesh receives `MountedPlanarDomain` via `new_mounted_bounded`; assembly receives `Arc<MountedAnalysisModel>` via `new_mounted`, preserving the mounted representation (`session/component.rs:1061-1063`, `:1169-1172`; `analyses/component.rs:1120-1123`, `:1395-1398`). | Pass |
| Normal and terminal retirement | Normal solid-index retirement pops one index then one admission per turn (`session/component.rs:960-968`). Numerical/model/domain/scalar/visual close paths retire one retained owner per returned close action (`session/component.rs:1675-1968`, `:2504-2548`, `:2702-2807`). | Pass |
| Former dynamic owner markers | In the bounded numerical region, searches found no `reserve_owner`, `try_reserve_exact`, `close_vec_step`, or `Vec::new()`. The searched former owners are fixed-slot/fixed-page representations. | Pass |

## Reproduced Gates

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test` | PASS — `live-source clean; hostile-mutations=48.` |
| `bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test` | PASS — `live-source clean; hostile-mutations=70.` |
| Declared-scope `rustfmt --edition 2021 --check --config skip_children=true` across the P6i/P6h source census | PASS |
| Scoped `git diff --check` across that census and `📜️script.ts` | PASS |

No production source was modified. Cargo, Nx, Wasm, browser, native/runtime, and broad build commands were not run.
