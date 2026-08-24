# Terra P6i Post-RED Independent Source Static Audit

Date: 2026-08-24

Verdict: **RED — the mounted FEM3D numerical caller still makes monolithic, post-admission allocations and releases their full backing in one close grant.**

## Scope and Evidence

Read-only independent review after the coordinator and Codex RED reports. Reviewed the governing P6i contract, both preceding RED reports, accepted P6h acceptance/audits, the implementation report, the P6i/P6h source verifier, and the live 2D/3D session, editor, viewer, retained renderer, World3d snapshot, sparse, analysis, and mesh callers.

The production caller is real and mounted:

`Fem3d editor both result modes` / `Fem3d viewer` → `live_visual::with_live_visual` → `MountedState::step` → `Fem3dNumericalChild::step` → `MeshJob` / `AssemblyJob` / `PcgJob` / `LdltJob` / `SubspaceIterationJob` → `Fem3dSolverView` → `Fem3dPageVisualJob` → `World3dSnapshotLease` → prepared World3d.

The visual side is materially repaired: the editor’s model and results modes and the viewer borrow the typed snapshot lease; the prepared World3d consumer reads typed pages; field pages carry non-zero solver values; and the numerical child retains solid meshing, Tet4 insertion, physical loading, `K_full*u-F_full` reaction recovery, and modal jobs. Those facts do not cure the following live ownership and scheduling counterexamples.

## Concrete Counterexamples

### 1. Numerical model owners are monolithic `Vec` allocations, not pre-admitted fixed pages

`Fem3dNumericalChild::reserve_owner` calls `Vec::try_reserve_exact(count)` for the *entire* requested owner before it compares observed capacity with one snapshot-page maximum:

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️session/🦀️component.rs:470-475`.

The mounted caller invokes it directly for full document node, node-ID, element, support, and solid collections at `:534-554`; then again for entire solid outlines, hole lists, every hole, and computed solid node/Tet4 owners at `:647-724`. The same route reserves the whole compact modal mass and RHS vectors at `:971-1010`.

`MountedState::step` calls this child at `:2378-2392`, before any visual candidate can exist. Thus maximum+1 is not refused by a page inventory before the backing allocation; a single scheduler opportunity can allocate an input-sized contiguous owner. This directly contradicts P6i Admission and Close, and it is reachable from all mounted 3D surface authorities.

**Bounded repair:** replace every listed numerical `Vec` owner with an exact fixed-page or bounded-slot owner. Retain a reserve-page cursor; prove each page credit before allocation/transfer; copy at most one scalar/slot per numerical opportunity; return an unaccepted page to its producer unchanged. Modal mass and RHS need the same page representation rather than `try_reserve_exact(free_order)`.

### 2. Numerical work consumes fuel after it performs the opportunity

The general numerical stage path evaluates `self.step_model(doc)` and only then calls `context.consume_fuel(1)` at `:907-946`, specifically `:936-937`. `step_model` performs the full-owner reserve operations above, model pushes/clones, and the multi-write element/Tet4 mass updates at `:611-645` and `:780-810`.

The special solid-mesh constructor likewise takes the domain, instantiates `MeshJob::new_bounded`, and only then consumes fuel at `:869-881`, specifically `:877-880`. This is the mounted caller, not a fixture. It violates the contract’s pre-work fuel rule and means cancellation/deadline enforcement cannot prevent the allocation/construction that already occurred.

**Bounded repair:** make every numerical stage obtain one fuel unit *before* entering a producer, reserve, copy, scalar update, job construction, or close opportunity. Split element/Tet4 mass updates into retained component cursors when their ownership is credited independently.

### 3. Close still frees a whole dynamically-reserved backing in one grant

After popping items, `close_vec_step` computes the entire remaining `Vec` capacity and replaces the owner with `Vec::new()` in one call at `:1430-1440`, especially `:1434-1439`. This is reached by the numerical child’s mounted close state machine (beginning `:1466`). For the full-capacity owners allocated by finding 1, that final replacement releases the complete backing in one grant rather than one actual credited page/backing.

**Bounded repair:** the fixed-page representation from finding 1 makes close exact: one call removes one admitted page or one fixed index owner, with no capacity-preserving truncate and no whole-owner replacement.

## Verifier Gap

The P6i 22-mutation self-test passes because its structural predicate checks the visual job body for no `while`/`for` and checks textual numerical markers, but it does not reject `Fem3dNumericalChild::reserve_owner`, `ReserveModalMass`, `ReserveRhs`, the post-work `step_model` fuel order, or `close_vec_step` whole backing disposal. See `📜️script.ts:5500-5778`, particularly the visual-only `threeStep` assertions and the 22 mutation list. The 70-mutation P6h verifier also passes, but it governs its P6h source packet rather than this new FEM3D child.

Add faithful mutations that restore each forbidden `try_reserve_exact`, move fuel after `step_model`/`MeshJob::new_bounded`, and replace a page close with `Vec::new()`. Each must fail the exact P6i predicate.

## Narrow Verification Ledger

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test` | Pass: `live-source clean; hostile-mutations=22` — false-green for findings 1–3. |
| `bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test` | Pass: `live-source clean; hostile-mutations=70`. |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` | Fails. It reports formatting diffs in the P6i FEM2D model source and also several scoped framework sources. No formatter write was made. |
| Scoped `git diff --check HEAD -- <P6i/P6h files>` | Pass: no whitespace errors. |

No Cargo, Nx, Wasm, browser, broad build, formatter write, or production source edit was run. This audit only created this ticket record.
