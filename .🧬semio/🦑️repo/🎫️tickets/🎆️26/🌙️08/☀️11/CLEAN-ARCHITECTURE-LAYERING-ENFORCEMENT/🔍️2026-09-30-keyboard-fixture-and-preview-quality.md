# Keyboard Fixture and Preview Quality Audit

Read-only source and receipt inspection, 2026-09-30. No production sources, fixtures, or tests edited; no tests executed by this audit. The parent owns the actual repair and runtime validation.

## Keyboard Finding and Exact Row

The public editor AppDefinition builder in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` declares `mod+shift+m` → `editMeshSelection` at line2720. The migrated law `the_editor_binds_every_keyboard_verb_the_fixture_names` in the composition owner's editor unit suite compares the authored fixture against every app-authored keybinding. Its failure was a missing fixture row, not missing production reachability.

The exact required row is:

```json
{ "chord": "mod+shift+m", "action": "editMeshSelection", "framework": false, "windowScoped": true, "ownerWindowKindId": "procedural-preview", "liveInModes": ["edit"], "why": "Mesh — edit the selected faces, edges, or vertices using the staged operation controls.", "staged": true }
```

The parent added this row during inspection; the current authored fixture contains it at line20. It matches the inspected public builder:

- `operation` is required despite its default `extrude`. Its options are extrude, inset, subdivide, flip, deleteFaces, moveVertices, and loopCut. Therefore `staged: true` is required by the migrated law, which inspects `.required` and does not treat a default as removing the requirement.
- Optional arguments are amount default0.1, cuts default1 (integer1–256), and dx/dy/dz default0. They need not be added to the keyboard row; the public definition owns staged controls.
- `window_kind_action_refs` assigns `editMeshSelection` only to the edit preview kind. Its constant is `procedural-preview`.
- The edit layout mounts `procedural-main` and `procedural-preview`. Generate mounts its separate generations, form, and generate-preview kinds, so `liveInModes: ["edit"]` is correct.
- The existing knife row has the same owner/mode and `staged:true`; its public start and end vector arguments are both required. No knife change is needed.

Every one of the 15 literal `.keybinding` declarations under the current editor is now represented by the fixture: the two mesh verbs, undo/redo, reorganize, import/export, deleteSelection, addGeneration, cycleShowMode/cycleLodMode, and four arrows. `toolRunAbort` is an additional intentionally framework-minted fixture row. Framework-minted clipboard/history/interaction bindings are separately handled by the law. No other missing app-authored chord was found by this source inventory. The canvas activate/Enter contract remains in `surfaceBindings`, not app chord bindings.

The migrated exactness law permits an omitted binding only when its declared action kind is History, Clipboard, or Interaction. That is its current declared exception; source enumeration above independently confirms the current authored mesh/action chords rather than relying on that allowance. This audit did not run public AppDefinition construction or browser dispatch and makes no independent runtime reachability claim.

## Fuse Delivery Quality Scope

Current `✏️s/🧑‍💻dev/🧩️composition-laws/🧪️tests/🧩️generation3d-example-geometry/🦀️.rs:930` runs the newly added coarse quality checks only when `fixture.example == "sphere-box-fuse"`. It requires zero quantized-position boundary edges, nonmanifold edges, and orientation defects; positive signed volume; and agreement of signed-divergence volume with Parry3D within `max(volume,1) * 1e-5`. These are real additional coarse checks for that case, not evidence of general coarse quality for all18 laws.

Parent diagnostic receipt `🗑️generated/canonical-preview-topology-scoped/exact-cargo-laws-hrkJC1/00/law-16.stdout` confirms the actual398 mesh has zero boundary/nonmanifold/orientation defects, signed volume9.445376600714702, Parry volume9.445382118225098, and the preserved bounds. Fine geometry at tolerance0.0025 remains6006 triangles with volume9.692821424 against analytic9.70845078963702 tolerance0.02.

Limitations matter:

- The coarse check compares two computations of the same triangle soup, not coarse volume against the analytic union or a declared coarse geometric error bound. Positive closed volume and matching extrema cannot exclude an incorrect closed shape between those extrema.
- The coarse measured volume differs from the analytic union by approximately0.263074189 (2.71%). The test does not admit or reject this difference; no coarse analytic-volume tolerance is authored in the fixture.
- The predicate is a hardcoded example name, not schema/fixture-declared quality applicability. Other solid deliveries do not receive these new checks. The earlier `canonical-preview-current-fixture.log` records that a broader version encountered a sphere-cut-with-torus winding failure. Current18-law success must not be described as proving all coarse meshes have sound winding.
- Incidence uses positional quantization to1e-6 and checks edge incidence/winding. It does not independently prove absence of self-intersections, one connected component, genus, or Hausdorff/chord error compliance.

## Manual Floor Change and Provenance

The current sphere-box authored delivery floor is398; `git diff HEAD` changes only that numeric row from historical428. All edge/bounds/role/chunk/round-trip/time rows are unchanged. The parent reports a fresh exact18-law run green in10.7s after this change; this audit did not execute it. That establishes current declared expectations, not preservation of the historical428 density assertion.

The preceding audit `🔍️2026-09-30-sphere-box-delivery-tessellation.md` establishes that the428 floor was authored Sep24 before the Sep30 committed pole-ring meshing correction. It identifies a concrete source change capable of changing triangulation inputs, but no controlled old/new ring execution has yet proven that it caused the exact428→398 delta. The manual floor should be documented as a reviewed update to measured current coarse density supported by topology/volume receipts, with causal uncertainty retained. It is not valid to claim that the old floor survived or that Parry agreement by itself proves the requested coarse fidelity.
