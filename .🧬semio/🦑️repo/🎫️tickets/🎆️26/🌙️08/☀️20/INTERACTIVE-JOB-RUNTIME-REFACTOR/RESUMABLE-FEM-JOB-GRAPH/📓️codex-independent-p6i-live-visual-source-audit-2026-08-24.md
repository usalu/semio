# Codex Independent P6i Live Visual Source Audit

Date: 2026-08-24

Verdict: **RED — concrete production-path counterexamples remain.**

## Scope

Read-only, source-first audit of the P6i repair contract, coordinator pre-acceptance RED,
current remediation report, accepted P6g/P6h preservation, and the live implementation.
No Cargo, Nx, Wasm, browser, broad build, or runtime gate was run.

## Counterexamples

### 1. FEM3D Has No Production Solver-to-Visual Publication Caller

`Fem3dSolverView` begins with zero completed fields at
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️session/🦀️component.rs:827-844`.
The mounted step refuses to create a visual candidate until every node field has been published
(`:876-883`). Only `publish_scalar` advances the completion count (`:156-167`), but the only
repository occurrences of `publish_solver_scalar`, `publish_solver_progress`, and
`Fem3dSolverScalar` outside their definitions are in the module's `#[cfg(test)]` fixtures
(`:1426-1449`, `:1541-1668`). There is no production numerical solver caller.

Therefore a normal non-empty FEM3D mounted operation remains in `Running(None)` at the
fields-ready gate and never produces a visual lease. This is not a generation-qualified solver
view wired to an actual solver; it is a decorative publication API.

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

### 4. The P6i Structural Gate Is False-Green for These Paths

The predicate checks the existence of public 3D publication functions and test-law text
(`📜️script.ts:5528-5540`), but contains no production caller census for either publication
function. It verifies field-packet construction (`:5517-5521`) and prepared instance admission
(`:5586-5593`), but does not require a `Status` consumer.

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

1. Wire the actual FEM3D numerical execution path to generation-qualified scalar/progress
   publication; a non-test caller census must prove it. Do not allow candidate creation merely
   through an uninitialized or synthetic progress count.
2. Define and consume typed 3D field/progress result pages in the prepared World3d path (including
   localization/accessibility), or map them to bounded visual primitives. The numerical fixture
   must verify the rendered prepared result, not just stored packet fields.
3. Replace FEM2D render-time `materialize` with an immutable prepared Canvas2d packet/page lease
   path; prohibit whole retained output reconstruction at mounted render.
4. Strengthen `toolJobFemLiveVisualPublicationExact` and self-mutations to reject each of the
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
