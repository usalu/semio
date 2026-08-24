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
