# 🔎️ 1107 Audit Review 7: Ported 3D, Kernel and Neural Crates

Read-only audit, lane `review-7`. No sub-agents, no cargo, no edits, no modifying git commands. Brief rulings read from `C/1107-brief.md`: single charge, honest quotes, close liveness (W4), chunk rather than cap, no `reborrow(g, maximum_work_units())`, wallets only on no-turn entry points (W1).

## Scope and paths

Each crate's `[lib] path` points at a module root `🦀️.rs` one or more directories up from its `📦️packages/🦀️rust` folder. Paths below are relative to `🧰️framework/🔨️modules/` unless marked otherwise.

| Crate (Cargo `name`) | Source root |
|---|---|
| `semio-framework-3d` | `🧊️3d/📦️packages/🦀️rust/🦀️.rs` (re-exports only, 44 lines) |
| `semio-framework-3d-mesh-document` | `🧊️3d/🥽️mesh/📄️document/` (6 files, 1325 lines) |
| `semio-framework-3d-brep-engine` | `🧊️3d/📐️brep/⚙️engine/` (33 files, 6004 lines) |
| `semio-framework-kernel-json` | `🎠️kernel/📤️return/🔤️json/` (6 files, 1782 lines) |
| `semio-framework-kernel-ui` | `🎠️kernel/🖼️ui/` (4 files, 1071 lines) |
| `semio-os-kernel-neural-engine-model` | `🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/` (10 files, 1815 lines) |

Read in full or at the relevant ranges: brep `⏱️operation-jobs`, `♻️retention`, `🧹️retirement`, `🪅️shape-value/🎟️original`, `📦️mesh-io` (90-270), `🩺️healing-exchange-and-tessellation` (55-90); kernel-ui `🦀️.rs` (200-262, 280-380, 770-900), `🛬️receive`, `♻️retirement`; mesh-document `⏱️tessellation-job-and-obj-export` (40-240), `🕸️halfedge-mesh-operations` (280-292); neural `🧵️retirement` (200-214, 330-475), `📚️dictionary` (150-185).

Not read in depth: kernel-json `🖼️ui` (macro projections), most of neural `operator-record`, `dictionary`, `evaluator`, brep `shape-exchange`, `analysis`, `boolean`, and the value crate's retirement primitives. Findings there are absent from this report, not cleared.

## Findings

### 1. Top: failed or dropped mesh import leaves `pending_mutations` open, so retention close stalls forever

- `🧊️3d/📐️brep/⚙️engine/🩺️healing-exchange-and-tessellation/🦀️.rs:69` increments `self.pending_mutations` on activation. The only decrements are `:71` and `:78`, and both require `cursor.cleanup_complete() && cursor.is_active()`.
- `🧊️3d/📐️brep/⚙️engine/📦️mesh-io/🦀️.rs:156`: when `advance` errors, the cursor sets `cancelled=true` and returns `Err`. Cleanup has not run (`retired` is false), so `:71` never decrements.
- `MeshImportCursor` has no `Drop` impl (checked: no `impl Drop` anywhere in the brep engine), so a host that drops an active cursor leaks the count permanently.
- `🧊️3d/📐️brep/⚙️engine/♻️retention/🦀️.rs:50`: `if kernel.pending_mutations!=0&&!self.cancelled {return Ok(Progress(default))}`. Every non-cancelled retention step returns zero-work Progress. There is no `Blocked`, no typed refusal, and no budget. Only `cancel()` escapes.
- Concrete failure: a document import fails on triangle k and the host drops the cursor. Every later retention close for that session returns Progress with zero work on each turn, indefinitely. This violates W4 and the "Pending{work:0} that can spin" check.
- Fix direction: decrement on cursor cancel and on drop (or make the counter owned by the cursor), and return `Blocked{reason: open import}` from retention instead of zero-work Progress.

### 2. Re-stepping a completed mesh import panics, or publishes the shell twice

- `📦️mesh-io/🦀️.rs:155`: after `retired`, `step` returns `Done(Solid)` by taking `self.solid` and leaves `self.shell` set. A second call takes the `else` branch and returns `Done(Shell)`, registering the shell a second time. A third call reaches `self.shell.take().expect("original completed import shell")` and panics.
- For open meshes (`phase` 4, no solid), the first `Done` already takes the shell, so the next call panics on the `expect` at `:155`.
- Reachability: `healing-exchange:69` re-activates the cursor on any re-step (`is_active` was cleared at `:71`, and `retirement_complete()` is false because `cancelled` is false). The call reaches `step`, and then `:155`.
- Concrete failure: a host polls `step_mesh_import_sync` once more after `Done` for an open mesh and the process panics. Confidence: high on mechanism, depends on the host re-polling.
- Fix direction: a terminal cursor returns a typed error or `Done` once, with no re-publication.

### 3. Work charged with no action in the UI turn-patch reservation probe

- `🎠️kernel/🖼️ui/🛬️receive/🦀️.rs:19` calls `control.prepare_work(1)?`. Then `:21` returns `Ok(Progress(default))` on `TryLockError::WouldBlock`, or `Err` on poisoned, epoch-exhausted, or slot-exhausted. `:22` runs `control.commit_work(1)` unconditionally and returns `work:1`.
- Concrete failure: under arena contention, or with a poisoned mutex, the turn charges one unit per call while nothing is reserved. The brief requires `work = performed` and "an unadmitted quote runs nothing and charges nothing". This is an over-charge, not a double charge.
- Confidence: high (read directly from the code).

### 4. Retention: zero-work refusals that never refuse, and unrecorded physical moves

- `♻️retention/🦀️.rs:57`: when the grant cannot admit the next demand (`maximum_items==0`, or any byte or depth bound short), the step returns `Progress(default)`. It should return a typed `WorkLimit` or `DepthLimit` refusal. If the host's grant never admits the demand, the close never completes and never refuses (W4).
- `:32`: while cancelled, the demand is `depth:1`. A turn with `maximum_depth==0` stalls the cancel forever, through the same `:57` path.
- `:59`, `:63`, `:64`, `:65`, `:67`, `:68`: these steps move rows (`extract_slot_if`), begin root walks, and admit compaction, then return `copied_items:1` without calling `effect.performed(..)`. The turn's recorded work is 0, so these physical moves are unrecorded. Scanning many empty slots costs zero fuel, bounded only by `slot_count`.
- `:59`, `:63`, `:65`: `ControlledRetirement::new(..)` and `BodyCompactionJob::new(..)` failures call `panic!`. Reachability is uncertain: I did not read the constructors' refusal conditions.
- `:66`: `if self.decision.is_none() {return Progress(default)}` waits for the caller to call `decide`. This is a caller contract; a caller that never decides spins with zero work.

### 5. Import close builds its own authoring wallet with caps

- `📦️mesh-io/🦀️.rs:150-158`: `MeshImportCursor::step` creates `AuthoringWallet::new(grant, 1_000_000_000)` and `wallet.run(grant, 1<<20, ..)` for each close step. W1 forbids wallets on turn-reachable paths; `1<<20` is a per-call cap, which the brief rules against ("chunk rather than cap").
- The grep for callers of `step_mesh_import_sync`, `close_mesh_import_sync`, and `close_mesh_import_original` under `🧰️framework` and `🛍️products` returned no hits, so the impact is limited unless a plugin turn calls these.
- Severity: low to medium, depending on the external callers, which I did not check.

### 6. Orphaned body entities on a triangle error in mesh import

- `📦️mesh-io/🦀️.rs:238-240`: `make_vertex` and `line_edge` record their entities into `local` and insert into `edge_map` and `vertex_map`.
- `:241`: `make_planar_face_from_wire(..)?` returns early. `local` is dropped, and `self.recorder.append_disjoint(local)` at `:242` never runs. The vertices and edges created for that triangle are owned by no retirement, so they stay in the session `self.body` arena.
- Concrete failure: a triangle whose planar face fails after its edges were created leaves those entities in the session with no owner.
- Confidence: medium. It requires `make_planar_face_from_wire` to fail after `:239-240`.

### 7. Panics on malformed attribute data in mesh document

- `🧊️3d/🥽️mesh/📄️document/🕸️halfedge-mesh-operations/🦀️.rs:289`: `attribute.values[source as usize].as_array().unwrap()`, `original[axis]` (index, panics if length is under 3), and `.as_f64().unwrap()`. Reachable from any normal-channel transform whose values were authored or imported.
- `🧊️3d/🥽️mesh/📄️document/⏱️tessellation-job-and-obj-export/🦀️.rs:237`: `pub fn corner_uv` calls `value[0].as_f64().unwrap()` and `value[1]` on a value returned by `as_array()` without a length check. A malformed UV sample panics the public API.
- Confidence: high on mechanism. These are panics on reachable input paths, not invariants.

### 8. Operation admission is unbounded and cannot be cancelled

- `🧊️3d/📐️brep/⚙️engine/⏱️operation-jobs/🦀️.rs:413-421`: `LinearPattern` builds `count-1` transforms eagerly, with no cap and no budget. `:433-447` builds `count_x * count_y` transforms for `GridPattern` the same way.
- `:229`: `extraction` copies the whole reachable body into a private working body before the job exists. That work is not budgeted and cannot be cancelled, because `BrepOperationJob` does not exist yet.
- Concrete failure: a pattern count of 10^9 allocates and runs for a long time before `operation_job_sync` returns, with no progress and no cancel.
- Severity: medium. The fix is to admit counts in budgeted steps or cap them with a typed refusal.

### 9. Kernel-UI close panics on caller-chosen grants

- `🎠️kernel/🖼️ui/🦀️.rs:325`: `contents.close_step(items, bytes).expect("exact returned patch retirement")`.
- `:853`: `close_step()` calls `close_step_with_grant(1, 4096).expect(..)`.
- `:351`: `close_ui_turn_patch_owner_with_grant(items, bytes)` accepts any `bytes > 0`, so a caller can pass a grant smaller than one atom.
- `🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/🦀️.rs:47-52` returns `Err` from `retirement.advance` in some cases. I did not read `advance`. If it refuses a grant below one atom, these `expect`s panic.
- Uncertain: depends on `advance` semantics.

### 10. Kernel-UI patch retirement can return BudgetExhausted forever

- `🎠️kernel/🖼️ui/♻️retirement/🦀️.rs:25`: `if grant.maximum_copy_bytes < demand.copy_bytes` returns `BudgetExhausted` with zero work. If a patch's copy demand is larger than any grant the host ever offers, the close never completes and never refuses (W4).
- Uncertain: whether the reactor's per-turn grant can ever be below a single patch's demand.

### 11. Neural dictionary: unbounded synchronous loop and a zero-work branch

- `🧱️model/📚️dictionary/🦀️.rs:164-177`: `FromValue::from_value` runs an unbounded `loop` around `cold_input_step`, with no budget, no cancellation, and no work accounting of its own.
- `🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:441`: the final `else` returns `Progress(default)` with no state change when there is no pending value, no frames, and no shared owner. If that state is reachable before `done` is set, the loop spins forever. I did not trace the whole state machine, so this is unverified.
- `:172` (dictionary): `panic!` if `cold_input_close` returns `Err` during error cleanup. Reachable if `cold_effect` refuses the close quote.
- `🧵️retirement/🦀️.rs:466`: moves ownership and reports `copied_items:1` with no `effect.performed(..)`. Unrecorded work, same pattern as finding 4.

### 12. Mesh tessellation whole-mesh loop depends on an unverified progress guarantee

- `🧊️3d/🥽️mesh/📄️document/⏱️tessellation-job-and-obj-export/🦀️.rs:233`: `tessellate()` runs `loop { wallet.run(..); match step { Done => return, Working => {}, Cancelled => Err } }` with no iteration bound and no cancellation. Termination requires every `job.step` to advance or finish.
- `:151`: phases 8 to 10 do nothing when `choose_triangle()` returns `None`. Only `reject_ear` (`:180-182`) changes phase on a miss. I did not verify that `choose_triangle` always makes progress.
- Uncertain. A `Working` result with zero work would spin here forever.

## Checked and not reported

- `semio-framework-3d` root: re-exports only, no logic.
- Operation job step, commit, and cancel (`⏱️operation-jobs` 529-569): one commit point, cancel does not touch the session, and the two `expect`s at `:539` and `:563` hold by construction.
- Kernel-UI retirement `♻️retirement/🦀️.rs`: charges go through `record_retirement_work` once per step (`:51-53`). No double charge found in the kernel-UI close path.
- Kernel-JSON turn receipt: `work_units_spent` and `remaining_fuel` are `u64` at the projection boundary, while the actor ledger accumulates in `u128` (`🎭️actor/🏛️kernel-core/🦀️.rs:83`). No truncation path found.

## Summary

Top finding: a failed or dropped mesh import leaves `Brep.pending_mutations` incremented (`🩺️healing-exchange-and-tessellation/🦀️.rs:69-72`, `📦️mesh-io/🦀️.rs:156`), so every non-cancelled retention close returns zero-work Progress forever (`♻️retention/🦀️.rs:50`).
