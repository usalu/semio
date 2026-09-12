# FEM3D Per-step Ownership Follow-up Audit

## Scope and evidence

This is a read-only source and existing-artifact audit. No Cargo command or test was run for this report.

The existing native artifact [fem3d-window-config-native-28.log](🗑️generated/fem3d-window-config-native-28.log) records one failure of `live_visual::tests::fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized`: the elapsed-time condition at the session test's former line 291. It records 18 of 19 selected FEM3D tests passing. It does **not** identify the elapsed value, numerical stage, nested cursor, or allocation/control operation that took longer than 8 ms. It is therefore runtime proof of an over-budget iteration, not proof that any source candidate below caused it.

The exercised fixture has one meshed solid, supports at its four document nodes, and self weight. It enters the mounted solid mesh route, model construction, assembly and CSR construction, PCG, reaction recovery, modal construction, LDLT, subspace iteration, publication, and bounded close. It has no Bar3 or Frame3 document element, so its `ElementCommit` route is not exercised.

Root has since added failure-only information to the same, unchanged 8 ms assertion: elapsed microseconds, `Fem3dNumericalStage`, `StepContext` stage, terminal state, and deadline state. The fuel assertion continues to run first and reportedly passed in FEM28. That is the right first discriminator; this audit does not recommend raising the threshold.

## Actual mounted route

The parent numerical loop is [editor/session/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs), in `Fem3dNumericalChild::step`. Non-delegated stages consume one parent fuel before their stage action. Delegated `SolidMesh`, `Assembly`, `Pcg`, `Ldlt`, and `Subspace` pass the context to their job.

| Route | Actual owner transition | Assessment |
| --- | --- | --- |
| `SolidMeshBegin → SolidMesh` | The parent gives a mounted domain to `MeshJob::new_mounted_bounded`, then drives `MeshJob::step(context)`. | The fixture reaches this route. It contains two source-proven multi-allocation candidates below. |
| `MountAssembly → PrepareAssembly → Assembly` | `MountAssembly` wraps the complete `MountedAnalysisModel` in `Arc::new(self.model.take())`; assembly construction then advances one construction cursor per call. | The initial Arc backing is a whole inline model allocation/move in one already-fuelled parent step. |
| `BuildCsr` | `AssemblyCsrBuild::step_one` advances its cursors. | No bulk action was found in this source review. |
| `BeginPcg → PreparePcg → Pcg` | The completed CSR and mounted RHS transfer into `PcgJobConstruction`; construction reserves/initializes one vector or scalar cursor per call; `PcgJob::step` runs the solve. | Construction is split. PCG control payloads are not. |
| `RecoverReaction` | One visual full-CSR entry is inspected per step. | Bounded by source. |
| `BeginModal → PrepareModal → Ldlt → Subspace` | Matrix/factor/mass owners move to modal, LDLT, then subspace jobs. | Modal construction is cursorized. LDLT and subspace output paths use retained page writers. |
| `Complete` and cancellation close | `MountedState::close_step` drives `Fem3dNumericalChild::close_step`; the child retires one live lane/owner at a time before it is removed. | No active production lifecycle that directly drops the complete live numerical child was found. |

The mounted assembly job deliberately emits no large preview/checkpoint payload in its mounted path. Modal construction advances a validation/reservation/copy cursor. The observed LDLT and subspace paths retain paged output rather than call a whole-state serializer in their interactive loop. Those paths are not current candidates from this review.

## Source-proven ownership gaps

### 1. Mounted mesh reserves three preparation backings in one fuelled job step

In [engine/mesh/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs), `MeshJobStage::ReservePreparation` calls `try_reserve_exact` for `preparation.points`, `preparation.point_indices`, and `preparation.constraints` in the same `MeshJob::step` action. The actual FEM3D solid fixture enters this mounted mesh stage.

One job fuel can therefore perform three backing-allocation attempts. An allocation failure is reported, but the successful operations are neither separately admitted nor represented by a cursor between them. This is a current source ownership gap and a plausible source of a slow `SolidMesh` iteration; it is not a runtime attribution for FEM28.

Smallest schema-first slice: replace the all-three reserve expression with three named preparation-reservation stages or a retained `PreparationReserveCursor`. Each successful allocation advances exactly one stage; zero fuel, expired deadline, and cancellation retain the prior owner and cursor.

Law target: `fem3d_mounted_mesh_reservation_and_insertion_use_one_backing_per_fuel`.

### 2. Mounted triangulation allocates three insertion workspaces in one fuelled action

`OwnedTriangulation::insert_next` creates `bad`, `boundary`, and `retained` work vectors and calls `try_reserve_exact` for all three when an insertion begins. `MeshJob` calls it from its mounted `InsertBoundary` route under the job's one-fuel context. The allocation bound derives from the mounted maximum triangle count; it is not split into individually admitted workspaces.

This is another actual `SolidMesh` multi-allocation burst. The earlier reservation candidate and this candidate are separate: splitting preparation reserves alone does not make insertion bootstrap one-owner-per-fuel.

Smallest schema-first slice: retain an `InsertionWorkspace` and give it `ReserveBad`, `ReserveBoundary`, and `ReserveRetained` cursors before the first triangulation insertion. Keep the existing insertion algorithm and its numerical output unchanged.

Law target: `fem3d_mounted_mesh_reservation_and_insertion_use_one_backing_per_fuel`, with an explicit case that reaches the first `InsertBoundary` bootstrap.

### 3. Assembly mount creates an Arc backing for the entire inline analysis model in one parent step

`Fem3dNumericalChild::step` at `MountAssembly` takes `MountedAnalysisModel` and performs `Arc::new(self.model.take()...)`. `MountedAnalysisModel` holds fixed inline arrays for 128 nodes, 128 elements, and 64 supports. The action happens after the parent consumed its one fuel, with no separate reservation/admission cursor for the Arc backing.

The fixture reaches `MountAssembly`. Source proves the whole aggregate allocation/move operation; it does not prove its size or timing caused FEM28.

Smallest schema-first slice: admit the analysis-model root as its own retained owner before numerical execution, then make `MountAssembly` transfer only an already-admitted handle. If the root must be created there, add a `ReserveAssemblyModelRoot` stage with a precise accepted backing bound and failure result before the transfer stage.

Law target: `fem3d_mount_assembly_transfers_one_admitted_model_root`.

### 4. PCG preview, checkpoint, and completion serialize whole solver state in one control step

In [engine/sparse/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs), the actual mounted `PcgJob::step` route serializes a complete `self.preview()` when preview or completion publication is due, and serializes `self.state` when a checkpoint is due. `preview()` clones complete `x` and `r` vectors and constructs derived reactions and contour vectors; `checkpoint_bytes()` encodes the full state. Each result is handed to `payload_from_bytes` in that same control step.

Unlike PCG construction, these operations have no page cursor. They can combine whole-vector allocation/derivation, encoding, and payload creation under one fuel. The solid self-weight fixture reaches PCG, so this is an actual production route and a plausible `Pcg`-stage explanation. It remains un-attributed until the enhanced timing assertion identifies a PCG control branch.

Smallest schema-first slice: define the preview/checkpoint envelope independently of native serialization, then make PCG use retained paged payload writers and cursors for source-vector copy, derived reaction/contour production, encoding, and payload retirement. Preserve the existing payload schema and numerical values. Do not call `encode_value(&self.preview())` or `encode_value(&self.state)` from an interactive one-fuel control action.

Law target: `fem3d_pcg_control_publication_is_paged_per_fuel`.

### 5. Solid tetrahedron identity creation performs multiple string allocations in one model step

The actual solid path's `SolidTet` stage clones four node identifiers and formats a tetrahedron identifier in one fuelled parent action. The fixture reaches it. `SolidNodeCreate` similarly formats an identifier and clones it into the model in one action. Existing generated solid identifiers are length checked, but the four direct node identifiers used by a tet are not revalidated here before their clones.

These actions are much smaller than the mesh and PCG candidates for the recorded fixture, but they violate the same per-step ownership rule. `ElementCommit` also clones a Bar3/Frame3 identifier and both endpoint identifiers in one action; it is production code but outside this fixture.

Smallest schema-first slice: use a pending fixed-slot element/tetrahedron identity record and transfer or validate one identifier per stage. Validate direct identifier bounds before an allocation/copy.

Law targets:

- `fem3d_solid_tet_identifier_transfer_one_per_step`
- `fem3d_element_identifier_transfer_one_per_step` for Bar3/Frame3 coverage.

## Diagnostics: what the current assertion resolves, and what remains

The new test assertion will distinguish an outer `SolidMesh`, `MountAssembly`, `Pcg`, and later numerical stage. Mesh already writes granular context stage names, so a mesh failure should identify whether it occurred during preparation reservation or insertion.

Two diagnostic gaps remain:

1. Non-delegated `Fem3dNumericalChild` stages do not consistently set a context stage. The enhanced message has the outer child stage, but it cannot expose the pending owner/cursor or inline model cardinality. A failure-only assertion should include the relevant child cursor/owner summary for `MountAssembly`, model construction, CSR/PCG/modal preparation, and publication stages.
2. `PcgJob` sets only `fem.pcg`. It does not distinguish normal iterative work from preview encoding, checkpoint encoding, or completion encoding. Add failure-only branch labels such as `fem.pcg.preview-encode`, `fem.pcg.checkpoint-encode`, and `fem.pcg.complete-encode`, together with PCG stage/cursor/iteration and due flags. This makes the first diagnostic rerun decisive without introducing hot-loop timing logs.

The root's new elapsed/stage/deadline/terminal message is sufficient to begin. Its output must not be treated as proof of a candidate until a rerun emits it.

## Bounded-close review

Root follow-up on the actual insertion code: the initial three work vectors are local variables until all reservations succeed. An intermediate failure therefore drops successful earlier backings rather than retaining them in the triangulation owner for governed cleanup. The Retain-to-Fan transition replaces the old triangle Vec, and the end of Fan sets insertion=None while its bad/boundary capacities remain live. Splitting the reserve calls alone will not close these ownership gaps. The correction must install a partial insertion owner before its first allocation, preserve the first fault and every successful backing, retain the displaced triangle backing, and retire all insertion workspaces one admitted physical backing at a time. This is source evidence only, additional to the Terra audit, not the attributed cause of native28.

`Fem3dNumericalChild::close_step` visits the retained child outcome, rejected owners, and then mesh, assembly, CSR, PCG, modal, LDLT, subspace, mass, domain, and model lanes. It returns after closing one live owner/lane; loops inside nested close routines skip already-empty lanes before returning from their next live owner. `MountedState::close_step` holds the numerical child in a `Box` and clears it only after its bounded close reaches terminal.

`SubspaceIterationJob::reserve_iteration_owner` calls a retiring work close with `usize::MAX`. The reviewed `SubspaceWork::close_step` still returns after one live vector owner and its live owners are created with the existing page cap, so this is not source proof of a current bulk drop. It bypasses the caller's explicit byte allowance and should be tightened in a separate hardening slice to pass the numerical owner-page cap and expose its retirement lane. It should not be used to explain FEM28 without a diagnostic result.

## Execution order and acceptance laws

1. Rerun the unchanged FEM28 test with the root diagnostic. Use its emitted child/context stage to select the first implementation slice; do not change its 8 ms threshold.
2. If it reports `SolidMesh`, split the preparation and insertion bootstrap reserve operations first. If it reports a PCG encode branch, cursorize only the identified PCG payload route. If it reports `MountAssembly`, admit the analysis-model root before the transfer. The string-transfer slice follows independently because it is source-proven but lower expected wall cost.
3. For every selected slice, use a schema-first closed fixture plus a language-neutral expected stage/owner trace. The existing numerical oracle remains responsible for result equivalence; the new ownership law must observe fuel, cancellation, deadline, cursor, and close semantics.

Each law must establish all of the following:

- Zero fuel, an expired deadline, and pre-cancel leave the same owner/cursor intact.
- One fuel executes one physical backing admission, one staged payload page, or one explicit handle transfer; it cannot complete a group of reserve operations.
- Close retires one retained page/owner under the configured page bound and becomes terminal only after all owners are gone.
- The existing FEM28 target still completes the solid, reaction, modal, and close lifecycle without changing its wall-time limit.

## Precise current and proposed targets

Existing runtime target:

```text
live_visual::tests::fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized
```

Proposed focused laws:

```text
fem3d_mounted_mesh_reservation_and_insertion_use_one_backing_per_fuel
fem3d_mount_assembly_transfers_one_admitted_model_root
fem3d_pcg_control_publication_is_paged_per_fuel
fem3d_solid_tet_identifier_transfer_one_per_step
fem3d_element_identifier_transfer_one_per_step
```

No test status is claimed beyond the pre-existing FEM28 artifact: this report did not execute tests.
