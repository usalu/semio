# Codex Independent P6i Live Visual Source Audit

Date: 2026-08-24

Verdict: **RED — concrete production-path counterexamples remain.**

## Scope

Read-only, source-first audit of the P6i repair contract, coordinator pre-acceptance RED,
current remediation report, accepted P6g/P6h preservation, and the live implementation.
No Cargo, Nx, Wasm, browser, broad build, or runtime gate was run.

## Counterexamples

### 1. FEM3D Solver Publication Is Wired, But Its Allocation Admission Is Not

The current source contains a mounted numerical child: it publishes node scalars at
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️session/🦀️component.rs:1574-1579`
and `:1642-1650`, then publishes final progress at `:1652-1659`. `ready` requires a validated
final state, all scalar slots initialized, and the exact total (`:229-231`); final/converged
progress rejects an incomplete scalar set (`:217-226`). A sparse `MaybeUninit` view therefore
does not falsely authorize the candidate on the current tree.

The same implementation does not admit the solver or order owners through an exact process
item/byte ledger before allocating them. `ReserveSolverPages` directly calls `solver.admit_page`
(`:1038-1050`), which allocates each boxed scalar/initialization page (`:168-183`), and visual
`ReserveSnapshot` directly constructs both boxed `FixedOrder` owners (`:2262-2268`). The
preflight/credit structure has only packet counts; it contains no process admission for these
backings. This fails P6i's required exact backing admission before transfer/allocation.

### 2. FEM3D Numerical/Status Pages Are Never Rendered

The page classifier makes only pages 1 through 9 `Instance`; all field and label pages are
`Status` (`.../🧵️session/🦀️component.rs:311-318`). Displacement, residual, reaction,
contour, and mode values are placed on pages 10, 12, 14, 16, and 18 (`:475-484`, `:651-671`),
with labels/progress on page 20 (`:674-683`).

The prepared World3d consumer only acts on `Mesh`, `Instance`, and `Camera`; every other kind,
including `Status`, reaches the empty wildcard arm
(`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs:8829-8892`). Thus
the claimed non-zero field correspondence fixture checks producer packet bytes, but no mounted
editor result mode or viewer presents those values. This violates the contract's required
numerical-to-visual correspondence and accessible progress/result presentation.

### 3. FEM2D Reconstructs a Complete Monolithic String on the Render Path

The fixed page owner explicitly creates a full `String` with total retained capacity and appends
all four pages (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs:217-224`).
`Fem2dMountedVisualLease::layers_json` exposes that reconstruction (`:335-337`) and the mounted
render body invokes it directly (`:1125-1128`). The editor dispatch is live mounted authority
(`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:332`),
so this is not test-only legacy code.

The P6i contract forbids complete visual decode/clone/encoding on the mounted UI route and
requires immutable page/packet lease consumption. Fixed backing during construction does not make
a full render-time `String::with_capacity(self.len)` plus four-page append compliant.

### 4. Snapshot Draw Credits Are Counted, Not Reserved

`SnapshotPreflight` accumulates `draw_count` and `draw_bytes` as scalar counters
(`.../🧵️session/🦀️component.rs:2839-2876`), but `World3dSnapshotDescriptor` receives only
page, item, and byte credits. The renderer allocates its draw rebuild later, when it processes
the mesh header through `begin_world3d_draw_rebuild`
(`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs:8830-8854`). Thus
preflight does not reserve draw capacity; it merely predicts it, leaving a late renderer refusal
path after visual lease publication.

### 5. Fault and Ordinary-Drop Ownership Are Not Exact

`MountedState::fail` keeps one fault `Vec` and clones it into `JobStep::Failed`
(`.../🧵️session/🦀️component.rs:2621-2623`). The numerical child likewise converts a retained
fault payload to another `Vec` while retaining the source payload (`:610-612`). The mounted close
cursor retires only the retained owner; neither additional result/fault backing participates in
the exact close ledger.

There is no `Drop` implementation for `MountedState`, `Fem3dPageVisualJob`,
`Fem3dPageVisualLease`, or `Fem3dSolverView`. `MountedJob::terminal_drop_is_shallow` simply
returns `true` (`:3031-3063`). An ordinary drop before the mounted close/retirement route uses
Rust's deep drop and cannot perform the required one-owner recovery; this is not a shallow-drop
handoff to mounted recovery.

### 6. The P6i Structural Gate Is False-Green for These Paths

The predicate checks the existence of public 3D publication functions and test-law text
(`📜️script.ts:5528-5540`), but contains no production caller census for either publication
function. It verifies field-packet construction (`:5517-5521`) and prepared instance admission
(`:5586-5593`), but does not require a `Status` consumer.

It does not require process admission for solver/order backing, admitted draw resources before
lease publication, fault-result ownership accounting, or a real ordinary-drop recovery path.
For 2D it only forbids a `bytes: String` backing inside `Fem2dFixedJsonPages`
(`📜️script.ts:5454-5468`); it neither rejects `materialize() -> String` nor tests the mounted
render body. Correspondingly, all 24 hostile mutations can pass while each counterexample above
remains live.

## Positive Evidence Preserved

FEM3D editor model/results and viewer dispatches do share the neutral `with_live_visual` authority
and attach `World3dSnapshotLease` directly to `scene.snapshot`:

- editor dispatch: `.../🧊️3d/.../✏️editor/🦀️component.rs:526-533`;
- viewer dispatch: `.../🧊️3d/.../👁️viewer/🦀️component.rs:82-90`;
- model/result/view packet attachment: their respective window model components at lines 29-33,
  92-96, and 39-43.

The 3D packet store itself uses fixed pages and close steps one admitted page at a time. These
repairs do not close the two production semantic gaps above.

## Required Closure

1. Add exact pre-allocation process permits for every solver page and fixed order backing, and
   reserve the draw resource before a visual lease can be sealed/published.
2. Define and consume typed 3D field/progress result pages in the prepared World3d path (including
   localization/accessibility), or map them to bounded visual primitives. The numerical fixture
   must verify the rendered prepared result, not just stored packet fields.
3. Make faults/results and ordinary-drop handles shallow recovery owners with one-owner close
   accounting, instead of cloning/deep-dropping live backings.
4. Replace FEM2D render-time `materialize` with an immutable prepared Canvas2d packet/page lease
   path; prohibit whole retained output reconstruction at mounted render.
5. Strengthen `toolJobFemLiveVisualPublicationExact` and self-mutations to reject each of the
   above, including deletion/misrouting of the real solver caller and ignored field-page kinds.

## Executed Source Checks

```text
bun ./📜️script.ts verify interactivity tool-jobs --p6i-only --self-test
[verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=24.

bun ./📜️script.ts verify interactivity tool-jobs --p6h-only --self-test
[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=70.
```

Scoped FEM P6i Rust files passed `rustfmt --edition 2021 --check`; the broader framework-plugin
file has unrelated pre-existing formatting drift, so it was not counted as a P6i formatting pass.
`git diff --check HEAD -- <P6i scoped paths>` emitted no whitespace errors.

Those structural checks are recorded as passing but are insufficient to overturn this RED verdict.
