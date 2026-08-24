# Sol High P6i FEM Live Visual Publication Implementation

Date: 2026-08-24
Agent: `/root/p6h_audit_remediation`
Scope: P6i source implementation and isolated source acceptance

## Outcome

The mounted FEM route now has dimension-separated 2D and 3D retained visual builders. Both use the
nineteen contract stages, generation-qualified candidate/current/displaced ownership, stable
incremental ordering, pre-admitted output backing, freshness validation immediately before
publication, and incremental close. The exact P6i source verifier accepts the final source with 21
faithful hostile mutations. The accepted P6h verifier remains green with all 70 mutations.

## Production Mapping

### 2D

- `Fem2dVisualJob` owns fixed output backing plus retained region, element, and numerical-field
  order indexes. Each call advances one scalar, adjacent stable-order comparison, region fragment,
  element fragment, glyph, vector/contour/mode entry, label, output page, or control transition.
- `Fem2dVisualFreshness` qualifies operation, model revision, document generation, numerical
  preview sequence, surface generation, and renderer scene generation.
- The mounted session consumes fuel before each build/close opportunity, moves superseded work into
  `visual_rejected`/`visual_displaced`, and preserves `visual_current` until an admitted sealed
  lease passes snapshot commit authority and exact freshness.
- PCG exposes scalar immutable visual reads through `PcgVisualScalar`/`visual_scalar` and a
  generation-local progress tuple. The session copies one node scalar after each numerical preview
  opportunity and never clones the solver vector.
- English and German accessibility entries include stage, progress, residual/tolerance,
  provisional/final state, quality, and cancel/retry/discard controls.

### 3D

- A separate `semio.fem3d.mounted-live-visual` job, schema, registry, lease, and cap inventory were
  added; it does not reuse the 2D catalog.
- The 3D builder has retained cursors for solid-outline centroids, endpoint lookup, stable region and
  element ordering, mesh nodes/elements, assembly marks, load/support glyphs, three-component
  displacement/residual/reaction/mode entries, contours, eigen estimates, localized labels, page
  sealing, freshness, publication, and displaced close.
- Tetrahedron and hexahedron cell identities are explicit 3D-only schema values.
- `SnapshotPreflight` scans one nested owner per caller opportunity. Output/load +1 checks compare
  prospective credit before committing counters, so rejection leaves the exact producer and
  retained census unchanged.
- The fixed 16-active/32-shell registry mounts on the shared bounded-job reactor using isolated
  placement. Previous current leases transfer into the successor and remain visible until the new
  candidate is atomically swapped; stale, cancelled, faulted, and refused candidates retire
  incrementally.
- Snapshot return witnesses, candidate/current/displaced leases, page strings, order indexes, fault
  storage, and shell credit all participate in close and terminal-empty checks.
- Model and results windows borrow the immutable mounted lease and bypass the old synchronous
  solve/mesh/sort/encode helpers. The value-owned World3d scene boundary performs only its admitted
  bounded packet materialization.

### Mounting and Publication

- FEM plugin initialization registers both dimension job kinds.
- FEM3D editor hooks provide snapshot preflight, reconcile effects, maintenance close, app close,
  and terminal-empty witnesses through the accepted P6g surface.
- The mounted render route validates exact app instance, revision, and generation before exposing a
  lease. Completion rechecks live registry identity, snapshot commit authority, cancel state, and
  the complete visual freshness tuple before swapping.

## Counterexample Map

The exact verifier extracts the production job/reconcile/render bodies and checks ordered stage
presence, forbidden whole-operation helpers, loop absence in job bodies, pre-work fuel ordering,
stable adjacent comparison, output ownership transfer, freshness ordering, atomic swap, snapshot
transfer, shared placement, mounted renderer borrowing, close hooks, and law-body evidence.

| Mutation | Rejected invariant |
| --- | --- |
| 2D maximum guard | schema +1 must refuse |
| 2D field-order swap | deterministic stable numerical order |
| 2D freshness inversion | stale candidate cannot publish |
| 2D German accessibility replacement | locale separation |
| 2D post-work fuel | fuel must precede the opportunity |
| 2D reaction sign | numerical field identity |
| 2D load preflight removal | nested glyph owner admission |
| 3D hexahedron removal | 3D-only cell schema |
| 3D mode vector shrink | three-component ownership |
| 3D freshness inversion | exact generation publication |
| 3D direct overwrite | atomic current/displaced swap |
| 3D former transfer removal | last-valid ownership |
| 3D inline placement | shared bounded-job mounting |
| 3D output guard removal | pre-admitted packet capacity |
| 3D load guard removal | nested load maximum |
| 3D producer-handback law removal | unchanged +1 producer |
| 3D mounted-render bypass | renderer lease consumption |
| 3D whole-scene helper restoration | no render-time rebuild |
| 3D partial-close law removal | interrupted close |
| 3D cancel/fault/device-close law removal | last-valid preservation |
| 3D German accessibility replacement | dimension-local locale entries |

## Laws Added

2D production-path laws cover maximum +1 owner preservation, stale/fault/cancel/device-close
last-valid behavior, deterministic replay, English/German accessibility, and per-step timing.

3D production-path laws cover maximum +1 before job ownership, output/load census +1 exact producer
handback, stale freshness refusal, deterministic replay, incremental lease close,
cancel/deadline/fault/device-close last-valid behavior, accessibility, and per-step timing.

## Verification Ledger

Executed on the final source:

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test
[verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=21.
```

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test
[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=70.
```

Scoped `rustfmt --edition 2021` parsed and formatted every changed FEM Rust source. The final
handoff also ran scoped `rustfmt --check` and scoped diff whitespace checks.

Cargo, Nx, Wasm, native/browser rendering, and broad workspace builds were intentionally not run in
this source-only packet. Those remain the serialized runtime acceptance gates enumerated by the P6i
contract and are not claimed here.

## Coordinator Pre-acceptance Remediation

This section supersedes the original 21-mutation ledger for the coordinator counterexamples in
`📓️coordinator-p6i-pre-acceptance-counterexamples-2026-08-24.md`.

| Counterexample | Bounded repair | Production evidence |
| --- | --- | --- |
| Complete mounted-render clones | FEM3D publishes `World3dSnapshotLease`; editor model/results and viewer attach that lease directly to `World3dScene.snapshot` | Both editor result modes and the viewer render body contain no `to_string`, solve, mesh, sort, collect, or scene encoder |
| Viewer whole-scene bypass | The viewer implements the same snapshot-preflight, reconcile, maintenance, close, and terminal hooks as the editor | `ViewerApp<V>` forwards all mounted hooks and `pending_effects`; viewer dispatch enters `live_visual::with_live_visual` |
| Monolithic output/order owners | 2D owns four independently admitted 4 KiB pages plus fixed inline index slots; 3D owns 21 independently admitted `World3dSnapshotPage` values plus two fixed index backings | Admission advances one page/backing per stage opportunity; close retires one slot, page, or fixed backing per grant and terminal-empty checks every owner |
| Placeholder zero fields | The 3D solver view uses generation-qualified `MaybeUninit<Fem3dSolverScalar>` slots and exposes scalar/progress publication without cloning a solver vector | Non-zero displacement, residual, reaction, contour, mode, and eigen fixtures are read from the sealed field pages and compared to the exact producer values |
| Prepared-frame bypass | Prepared World3d consumes one typed page item per fuel opportunity, begins an admitted draw rebuild, admits one instance, seals atomically, then swaps the retained lease | Stale/page/capacity faults retain the cursor; close is incremental; the former scene is not replaced until seal |
| False-green verifier | The P6i verifier now extracts live storage, job, close, preflight, editor, viewer, framework-forwarding, and prepared-world bodies and proves ordered predicates | 24 no-op-guarded mutations cover monolithic backing, whole close, UTF-8 page boundaries, post-work fuel, reaction identity, zero solver backing, generation, field correspondence, atomic swap, editor/viewer bypass, hook forwarding, instance admission, seal, and lease swap |

### Remediation Laws

- 2D fixed output rejects the page maximum +1 before transfer, preserves UTF-8 page boundaries, and
  returns one actual page or index slot per close grant.
- 3D snapshot preflight rejects a page-byte maximum +1 without changing the producer pointer or
  committed counters.
- A generation-tagged non-zero solver scalar corresponds exactly to displacement page 10, residual
  page 12, reaction page 14, contour page 16, and mode/eigen page 18.
- Stale publication, cancellation, deadline exhaustion, injected fault, and interrupted device
  close preserve the prior valid snapshot and close every candidate page incrementally.
- Deterministic replay preserves both localized label pages, and one production call remains below
  the eight-millisecond source law threshold.

### Final Isolated Ledger

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test
[verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=24.
```

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test
[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=70.
```

```text
git diff --check HEAD -- <P6i scoped files>
clean
```

Scoped Rust formatting completed successfully. Cargo, Nx, Wasm, native/browser rendering, and broad
workspace builds were not run.

## Independent Terra RED Remediation

This section supersedes both earlier P6i ledgers for the concrete counterexamples in
`📓️codex-independent-p6i-live-visual-source-audit-2026-08-24.md`.

| Counterexample | Production repair | Source evidence |
| --- | --- | --- |
| No mounted production FEM3D numerical caller | `MountedState` retains and drives a generation-qualified `Fem3dNumericalChild` before visual candidacy; sparse or incomplete fields cannot become ready | The child owns assembly, PCG, modal construction, LDLT, Subspace iteration, solver pages, and terminal close state |
| Supported solids omitted from the solve | The numerical child copies one domain point per grant, drives accepted `MeshJob`, extrudes each retained triangle, and inserts one genuine `Tet4` per grant | Solid meshing, extrusion-node lookup/creation, material lookup, pressure, and self-weight all have retained scalar cursors |
| Decorative reaction and modal fields | Reactions are recovered from the retained full system as `K_full u - F_full`; modal input uses positive physical lumped member/Tet4 mass and publishes the genuine Subspace component/eigenvalue | Assembly exposes qualified full entries and compact indices; `ModalInputConstruction` validates and adopts the retained mass owner instead of manufacturing identity mass |
| Prepared World3d ignored typed result state | Prepared World3d consumes solver displacement, residual, reaction, contour, mode/eigen, status, and progress pages into visible and accessible retained instances | Generation and solver-count correspondence are rechecked before candidate seal and atomic lease swap |
| Mounted FEM2D reconstructed a whole string | The mounted Canvas route retains immutable prepared page/packet leases, and the native renderer consumes borrowed records before the legacy fallback | The mounted render body contains no `materialize`, `layers_json`, whole-string clone, solve, triangulate, sort, or collect path |
| Numerical owners could disappear on completion or fault | Completion retains the numerical child until mounted close; delegated children, solver pages, modal mass, mesh/assembly owners, IDs, strings, and retained fault payloads close incrementally | Every close call retires at most one credited page/backing/child opportunity and terminal-empty witnesses include the numerical child |
| Structural verifier could false-green caller/render gaps | The exact P6i verifier censes the mounted numerical caller, both editor modes, viewer authority, Canvas renderer, prepared status/field consumers, solver-page admission/close, solid/Tet4 execution, reaction identity, physical modal mass, genuine Subspace output, and numerical close ordering | Twenty-two faithful source mutations each weaken a live production predicate and are rejected |

### Production Laws

- `fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized` drives the real
  slab path with one fuel opportunity per turn, observes non-zero displacement, constrained
  reaction, mode component, and eigenvalue, enforces the per-turn timing law, and interrupts close.
- `fem3d_production_field_correspondence_rejects_sparse_and_zero_aliases` proves sparse fields are
  not ready, a complete generation-qualified field is ready, stale generations are rejected,
  reaction and residual remain distinct, mode and displacement remain distinct, and maximum +1 is
  refused.

### Superseding Isolated Ledger

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test
[verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=22.
```

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test
[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=70.
```

Scoped `rustfmt --edition 2021 --check` and scoped diff whitespace checks are clean. Cargo, Nx,
Wasm, native/browser rendering, and broad workspace builds were not run and are not claimed.

## Post-RED Fixed-owner Remediation

This section supersedes every earlier P6i ledger for the concrete findings in
`📓️terra-p6i-post-red-independent-source-static-audit-2026-08-24.md`.

| RED counterexample | Exact production repair | Counterexample evidence |
| --- | --- | --- |
| FEM3D document/model transfer used monolithic `Vec` allocation | The mounted numerical child admits and copies nodes, IDs, elements, supports, and meshed solids into `MountedAnalysisModel` and fixed inline `FixedSlots` one slot per pre-fueled opportunity | The P6i census rejects dynamic replacements independently for nodes, elements, supports, analysis IDs, and meshed solids |
| Solid outlines, holes, extrusion nodes, and Tet4 owners were dynamic | `MountedPlanarDomain` owns fixed outer/hole polygons; fixed solid point, triangle, node-ID, and analysis-index owners advance one scalar or slot per turn; `MeshJob::new_mounted_bounded` retains the domain | Independent mutations restore dynamic outline, hole, solid point, triangle, ID, and index owners and are rejected |
| RHS and modal mass used total-size reserve/copy | `MountedScalarSlots` admits, writes, updates, transfers, and retires one scalar slot per opportunity; PCG and modal constructors retain these exact owners through refusal and close | Separate RHS, modal-mass, maximum-guard, physical-mass-transfer, and refused-PCG-owner mutations fail |
| Fuel was consumed after local work and solid `MeshJob` construction | Every local numerical stage checks cancellation/deadline, consumes one fuel unit, rechecks cancellation, and only then performs one reserve/copy/update/construction action; accepted delegated jobs retain their own pre-work gates | Faithful mutations move fuel after `step_model` and after mounted `MeshJob` construction; both fail |
| Member/Tet4 mass updates could batch writes | `ElementMass` advances one scalar mass entry and `SolidTetMass` advances one of twelve translational lumped-mass writes per turn | Mutations that remove the stages or replace physical Tet4 mass fail the structural gate |
| Normal and terminal retirement could release whole owners | `SolidIndicesRetire` pops one live analysis index and one admission slot per opportunity before solid publication; terminal close advances one child, string backing, scalar, slot, admission, page, or fixed backing per admitted grant | Whole-close mutations for the normal solid-index path, numerical owner path, visual pages, Canvas pages, and fixed owners all fail |
| Capacity refusal could mutate or lose producers | Fixed slot/model/domain/scalar guards reject maximum +1 before admission; element, Tet4, meshed-solid, and PCG refusal branches restore the exact transferred owner | The maximum +1 law compares returned `String` and node backing pointers, proves counters unchanged, and closes one slot/admission per action; owner-restoration mutations fail |
| Earlier verifier did not cover these families | The exact verifier now extracts all four live admission helpers, ordered pre-work fuel, split mass stages, mounted construction, normal/terminal close, refusal restoration, and the law body | Forty-eight no-op-guarded faithful mutations are independently rejected; preserved P6h still rejects all seventy mutations |

### Fixed-owner Law

`fem3d_numerical_fixed_owner_maximum_plus_one_refuses_unchanged_and_closes_one_slot` exercises the
live fixed-slot, mounted-analysis, mounted-domain, and mounted-scalar owners. It proves maximum +1
leaves admission and length unchanged, full/unadmitted writes return the exact producer backing,
one close call retires one value or admission slot, and the bounded action remains below eight
milliseconds.

### Final Source-only Ledger

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test
[verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=48.
```

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test
[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=70.
```

The full Rust source census declared by the P6i and P6h gates passes
`rustfmt --check --edition 2021`. The scoped diff whitespace gate is clean. Cargo, Nx, Wasm,
native/browser rendering, and broad workspace builds were not run and are not claimed.

## Revoked-family Ownership Remediation

This section supersedes the 48-mutation source ledger for the four ownership families found by the
fresh current-tree audit after the fixed-owner acceptance was revoked.

| Revoked family | Exact production repair | Hostile evidence |
| --- | --- | --- |
| Solver and order backing allocated without process admission | Each mounted shell reserves exact process item/byte totals before `MountedState` construction. `Fem3dSolverView::admit_page` claims the scalar or initialization page before its `Box`, and `FixedOrder::new` claims its exact order bytes before allocation. | Separate mutations bypass initialized-page, scalar-page, and order claims; remove the mounted reservation; or zero the per-shell item/byte credits. All are rejected. |
| Draw credit counted but not reserved before prepared allocation | `World3dSnapshotDescriptor` carries draw, instance, and byte totals. `world3d_snapshot_begin` atomically reserves their global capacities before slot publication, and prepared World3d claims the exact typed draw permit before beginning its rebuild. Slot release returns all three totals. | Mutations zero the descriptor, omit the global draw-byte reservation, or bypass the prepared claim. The draw reservation/orphan-close law is also required. |
| Retained faults and payloads cloned | Mounted faults transfer the owned detail directly into `JobStep::Failed`; oversized details are replaced by a fixed diagnostic. The numerical child retains the exact fault payload and emits only a fixed code, without copying its retained page. | Clone-restoration mutations in both mounted failure and retained-payload paths fail body-level ownership predicates. |
| Ordinary Drop deep-reclaimed live state | Solver scalar/init pages, candidate snapshot pages/write token/order backings, and completed leases transfer to fixed recovery owners. `MountedJob` cancels the external retained shell; `MountedState` asserts that external recovery reached terminal empty. Maintenance drains one recovered backing, page, or snapshot control owner per admitted grant. | Independent mutations remove solver, lease, candidate, state, and mounted-job Drop lanes, replace one backing/page close with whole-array release, or batch orphan pages. All are rejected. |

The verifier false-green caused by two sequential `return` statements was removed: production and
visual predicates are now conjoined, and both mutation collections execute. Current Canvas packet
names and body evidence replaced stale predicates from the previously unreachable visual block.

### Revoked-family Laws

- `fem3d_process_permits_precede_solver_order_allocation_and_drop_handoff_closes_one_backing`
  proves maximum +1 process refusal leaves live totals and owner pointers unchanged, then exercises
  solver and candidate ordinary Drop handoff with one recovered backing per close grant.
- `draw_permit_is_reserved_before_publication_and_orphan_close_is_one_page` proves global draw
  item/instance/byte reservation, exact one-time prepared permit claim, zero-credit refusal, and one
  page or control owner per orphan close step.

### Final Revoked-family Ledger

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test
[verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=93.
```

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test
[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=70.
```

Exact scoped `rustfmt --edition 2021` plus `rustfmt --check` succeeded for the World3d snapshot,
prepared World3d consumer, and mounted FEM3D session sources. Scoped diff whitespace checks are
clean. Cargo, Nx, Wasm, native/browser rendering, and broad workspace builds were not run and are
not claimed.

## Narrow Ordinary-Drop Recovery Remediation

This section supersedes the ordinary-Drop row and 93-mutation ledger above for the sole RED in
`📓️codex-p6i-revoked-family-independent-reaudit-2026-08-25.md`.

| RED counterexample | Exact repair | Source evidence |
| --- | --- | --- |
| `MountedJob::drop` only cancelled the external shell | Every allocated shell now pre-reserves its own fixed `MountedRecoverySlot` for the exact `Identity`. Running or queued job Drop publishes `Recover` before attempting a shell borrow, cancels, and moves the exact `MountedState` when uncontended. If the shell is borrowed, the publication remains generation-qualified and the owner stays discoverable in the fixed shell until maintenance transfers it. | The Drop-body verifier requires publication before cancel/borrow, `shell.take()`, exact `publish_owner`, and exact restoration on refused transfer. Mutations independently delete each operation. |
| `MountedState::drop` asserted instead of handing ownership back | A nonterminal state replaces itself with an owner-empty terminal shell and transfers the exact populated state to its pre-reserved recovery slot. The slot uses a fixed mutex-backed owner cell, so borrow contention cannot lose or deep-drop the state. | The verifier rejects the former cancel/assert body and requires ordered replace plus `publish_owner`; a body mutation restores ordinary owner Drop and fails. |
| No state/job recovery drain | Mounted maintenance scans one fixed recovery slot, discovers a contended shell owner or takes the published owner, performs one existing `MountedState::close_step` opportunity, and restores the same generation-qualified state when nonterminal. Only terminal identity permits state Drop, recovery reservation release, process-credit release, and shell release. | Mutations bypass state close, restoration, maintenance mounting, credit release, or recovery publication and are rejected. `terminal_is_empty` includes every recovery reservation for the application. |
| Completed job Drop could destroy the valid visual | A job that observed `JobStep::Done` publishes `Retained`; maintenance validates the same identity and completed state, clears only the job-handoff publication, and leaves the valid mounted visual in its shell until normal replacement/close. | Factory construction validates the pre-reserved recovery identity, and normal retirement clears the retained publication only after state terminal-empty. |

### Drop Recovery Law

`fem3d_queued_running_and_state_drop_publish_exact_identity_and_drain_one_owner` covers all three
hostile paths. A queued job is dropped while its shell is borrowed and remains discoverable under
the exact identity; a running job with a live solver page transfers directly; and an independently
dropped populated state publishes itself. The shared drain proves every grant releases at most one
owner and at most one page-sized backing, then reaches zero shell item/byte credit and an empty
generation-qualified recovery authority.

### Narrow Final Ledger

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test
[verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=101.
```

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test
[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=70.
```

Scoped Rust formatting/check and scoped diff whitespace checks are clean. Cargo, Nx, Wasm,
native/browser rendering, and broad workspace builds were not run and are not claimed.
