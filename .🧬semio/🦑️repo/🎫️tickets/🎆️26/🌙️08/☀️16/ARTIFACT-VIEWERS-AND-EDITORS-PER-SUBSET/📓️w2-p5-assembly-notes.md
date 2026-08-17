# W2 Packet P5 (assembly) — Notes

Lane: W2 packet P5, plugin `🌀️procedural`, artifact kind `🧩️assembly` ONLY (siblings own
`🌀️procedural2d`/`🧊️procedural3d` in the same plugin tree, disjoint subtrees, not touched). Harder than
a normal W2 packet: no `🎛️apps` tree ever existed, no artifact-root `component.rs` existed, and
assembly was not yet registered on the plugin. Recipe followed: `📓️w2-cad-report.md` steps 1, 2, 6, 8,
9, 10, 11. Closest real precedent found and mirrored closely: `🔋️energy`'s `🔋️model` artifact — also
authored fresh under this same ticket, also schema-first with zero apps to migrate, also a
`TreeWindowKit`-based single-window-per-surface first pass.

## What `AssemblySnapshot` / the WFC schema actually models

Read in full before designing anything (`🧬️schema/📸️snapshot/🦀️component.rs`,
`🧬️schema/🔺️diff/🦀️component.rs`, `🧬️schema/🧬️mutations/🦀️component.rs`, all nine mutation triads,
and `🧬️schema/💡️inferences/🦀️component.rs`'s `compile_and_solve`):

- `AssemblySnapshot` persists only the WFC **problem spec**, never the solved assignment:
  - `seed: u64` — deterministic solve seed, authored only via `change-seed`.
  - `slots: Vec<AssemblySlot>` — WFC solver variables, each `{ id, x, y, z, pinned_module_id }`. The
    `x`/`y`/`z` fields are real coordinates (so a future spatial/mesh view is plausible), but no module
    ASSIGNMENT is ever stored per slot — `pinned_module_id` is a hard pre-assignment fed INTO the
    solver, not an output.
  - `edges: Vec<AssemblySlotEdge>` — generic adjacency graph (`{ id, from_slot_id, to_slot_id }`) the
    solver propagates constraints over; no 2D/3D regular-grid assumption baked in despite the slot
    coordinates.
  - `modules: Vec<store::ArtifactChild<SemioKitSnapshot>>` — the catalog of placeable content, composed
    via `kit` (owned per this wave's design ruling: no private closed `Module` type). Each child exposes
    `.child_id: String`.
  - `weights: Vec<AssemblyModuleWeight>` — per-module selection bias (`{ module_id, weight: f64 }`).
  - `rules: Vec<AssemblyRule>` — adjacency constraints between modules (`{ id, module_a_id, module_b_id,
    allowed: bool, params: SemioValue }`); `params` is generic `value`-shaped structured data, not a
    bespoke struct per constraint kind.
- The SOLVE itself (`AssemblySolve`/contradiction verdict/entropy map) is an INFERENCE
  (`compile_and_solve` in `💡️inferences/🦀️component.rs`), derived via `wfc_engine::solver_graph::
  GraphSolver`, never mutation-authored persisted state. This is why the editor/viewer render the
  problem spec as a tree, not a solved spatial layout — there is nothing solved to place without
  running the inference, which is out of a first-pass surface's scope.
- Nine real mutation kinds, each a real `{🦠️mutation, 🔺️diff, ↩️inverse}` triad with a builder fn
  returning `AssemblyMutation` directly: `create_slot(index: usize, slot: AssemblySlot)`,
  `delete_slot(id: String)` (cascades incident edges), `create_rule(index: usize, rule: AssemblyRule)`,
  `delete_rule(id: String)`, `change_weight(module_id: String, weight: f64)` (upserts),
  `remove_weight(module_id: String)`, `connect_slots(index: usize, edge: AssemblySlotEdge)`,
  `disconnect_slots(id: String)`, `change_seed(seed: u64)`. All nine round-trip through `inverse()`
  (tested in the existing schema tree, not by this packet).
- `🌊️flow` under `💡️inferences/🧩️wfc-engine/🌊️flow/🦀️component.rs` is confirmed (read in full) to be an
  internal WFC constraint-propagation-flow concept — nothing to do with the sibling `🌊️flow` plugin or
  `semio-framework-os-flow`. Not touched, not imported by anything I wrote.

## `ASSEMBLY_DIALECT` — exact value and why

```rust
pub const ASSEMBLY_DIALECT: Dialect = Dialect { artifact_kind: ASSEMBLY_DOCUMENT_SCHEMA, standard: StandardId("1"), subset: SubsetId::ANY };
```
(`🗿️artifacts/🧩️assembly/🦀️component.rs:24`)
where `ASSEMBLY_DOCUMENT_SCHEMA = "s.assembly"` (re-exported from `🧬️schema/📸️snapshot/🦀️component.rs`'s
own `pub const ASSEMBLY_DOCUMENT_SCHEMA: &str = "s.assembly";`). **This deliberately does NOT follow the
brief's illustrative `"s.procedural.assembly"` guess** — grepped the schema tree first (per the brief's
own escape clause) and found the REAL, already-chosen convention: `AssemblySnapshot`/`AssemblyDiff` both
carry `#[artifact_schema(id = "s.assembly")]` (bare, not nested under `procedural`), predating this
ticket. Followed what's on disk, not the illustrative guess. Canonical surface ids are therefore
`s.assembly@1/*#editor` / `s.assembly@1/*#viewer`.

Contrast `procedural2d`/`procedural3d`: their OWN real `#[artifact_schema(id=...)]` values ARE
`"s.procedural.procedural2d"` / `"s.procedural.procedural3d"` (nested under the `procedural` plugin id)
— so the brief's naming guess was right for THOSE two, just not for assembly, whose schema tree was
authored with a different, bare convention by whichever earlier wave built it.

## Files created

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🦀️component.rs` (NEW, artifact root) —
  `artifact_kind()` (`ArtifactKindSpec`, `id: "data.assembly"`, `dimension: "data"`,
  `MediaClass::Data`/`MediaForm::Value` — follows `energy.model`'s "headless data artifact" precedent,
  not `procedural2d`/`3d`'s `Flow` shape, since assembly has no flow-graph fixture), `definition()`
  (single `schema.artifact` capability, `ArtifactIdentity::parse("s.assembly")`, descriptor
  `b"s.assembly"` claimed as `"s.assembly"` under `ArtifactIdentityNamespace::schema()` — exactly the
  brief's template, since the escape-clause check confirmed `"s.assembly"` IS the real convention),
  `ASSEMBLY_DIALECT` const (region `🔖️Dialect`, :13-25), re-exports of `AssemblyDiff`/`AssemblyMutation`/
  `AssemblySnapshot`/`ASSEMBLY_DOCUMENT_SCHEMA`. **No `declaration()`** — see "Blocking gap" below,
  documented in-file at the `🔖️Declaration` region's trailing comment (:63-77).
- `✏️editor/🦀️component.rs` (surface root) — `AssemblyEditorCommand` enum at :28 (nine variants, one
  per real mutation kind), `impl ArtifactEditor for AssemblyEditor` at :54, `Snapshot = AssemblySnapshot`,
  `Mutation = AssemblyMutation`, `Config`/`Draft`/`Presence`/`Transient` all `No*` (the
  `👥️presence/🧬️schema` facet is still scaffold-empty, confirmed before choosing `NoPresence`),
  `fn handle` at :77 translating each command 1:1 via the schema tree's own builder fns — no synthetic
  "set field" indirection, since assembly's mutations are already exactly this granular (unlike
  `energy.model`'s two-field `SetStructureField`). `create_assembly_editor()` at :115.
- `✏️editor/🎭️modes/✏️edit/🦀️component.rs` — mode `definition()`/`layout()`, single full-pane
  `structure` window (mirrors `🌿️vcs`'s single-window layout precedent, not `energy.model`'s two-column
  split, since assembly has one window today).
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️component.rs` + `🟦️component.ts` — real
  `TreeWindowKit::editable_window_kind()` window; `render()` builds one tree branch per collection
  (slots/edges/modules/weights/rules), each leaf showing real field values.
- `✏️editor/🟦️component.ts` — namespaced re-export (`export * as structureWindow from …`), dialect/mode
  id consts.
- `👁️viewer/🦀️component.rs` — `impl ArtifactViewer for AssemblyViewer` at :35, same `Snapshot`/
  `Mutation`, `AssemblyViewCommand` one-variant no-op enum, `handle` always `Ok(ViewEmit::default())`,
  `create_assembly_viewer()` at :75. Verified zero `::editor::`/`.mutation(`/`artifact_mutations`
  substrings anywhere under `👁️viewer` (grepped after writing, not assumed).
- `👁️viewer/🎭️modes/👁️view/🦀️component.rs` — mirrors the editor mode, single `structure` window.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🦀️component.rs` + `🟦️component.ts` — same tree shape
  as the editor's window, `TreeWindowKit::window_kind()` (read-only variant), reads
  `crate::artifacts::assembly::AssemblySnapshot` directly, never through the editor.
- `👁️viewer/🟦️component.ts` — namespaced re-export, no command-payload types (viewer declares none).

Deleted (scaffold, disposable per contract §7.8): both surfaces' `🎭️modes/<mode>/🪟️windows/🪟️main/`
placeholder trees, replaced with `🌳️structure`.

## Window-kit choice and reasoning

`TreeWindowKit` for both surfaces, ONE window each (`🌳️structure`). Considered a spatial/`MeshWindowKit`
render instead (the brief explicitly raised this, since `AssemblySlot` carries real `x`/`y`/`z`) but
rejected it for a first pass: the snapshot never stores a solved module ASSIGNMENT per slot — only the
PROBLEM (unsolved slots/edges/rules/weights) — so a mesh view would have nothing solved to place without
running the `compile_and_solve` inference first, which is real, expensive WFC solving, not a pure render
concern. A rule/slot/weight/module tree is the honest representation of what this artifact actually
persists today, and directly mirrors `energy.model`'s own `🌳️structure` precedent (also authored fresh
under this ticket, also schema-first). A spatial view over the raw slot coordinates, or a rendered
SOLUTION view once an inference-invocation seam exists from `render()`, are both plausible, documented
follow-ups — not required for surface-completeness today.

## What the coordinator must wire (plugin/glue level — NOT done here, outside this lease)

1. **`📦️glue.rs`** (`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`):
   - Mount the new artifact-root file: inside the existing `pub mod assembly { … }` block (currently
     starts at line ~855, already has `pub mod standards { … }` + the `schema`/`diff`/`mutations`/
     `inferences` shims), add — mirroring `procedural2d`'s/`procedural3d`'s own top-of-block shape
     exactly (`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs:33-36`):
     ```rust
     #[path = "../../🗿️artifacts/🧩️assembly/🦀️component.rs"]
     mod component;
     pub use component::*;
     ```
     right after `pub mod assembly {` and before `pub mod standards {`.
   - Add two new independent regions, `//#region ✏️Editor` and `//#region 👁️Viewer`, each
     `pub mod editor { pub mod assembly { … } }` / `pub mod viewer { pub mod assembly { … } }`, mounting:
     - surface root: `mod component; pub use component::*;` from `…/✏️editor/🦀️component.rs` /
       `…/👁️viewer/🦀️component.rs`
     - `pub mod modes { pub mod edit { mod component; pub use component::*; pub mod windows { pub mod
       structure { mod component; pub use component::*; } } } }` (viewer: `view` instead of `edit`,
       same shape) — this is the EXACT module path my files already assume
       (`crate::editor::assembly::modes::edit::windows::structure`, `crate::viewer::assembly::modes::
       view::windows::structure`) — verify against the real file once mounted, don't hand-derive again.
   - Follow the disk-verification script from `📓️w2-cad-report.md` step 10 before declaring done.
2. **Plugin root** (`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`): add
   `.editor::<crate::editor::assembly::AssemblyEditor>(crate::editor::assembly::create_assembly_editor())`
   and `.viewer::<crate::viewer::assembly::AssemblyViewer>(crate::viewer::assembly::create_assembly_viewer())`
   — analogous to the two existing `.artifact(crate::artifacts::procedural2d::declaration())`-style
   calls, EXCEPT assembly has no `declaration()` yet (see blocking gap below), so `.artifact(…)` for
   assembly cannot be added until that lands. `.editor()`/`.viewer()` for the surfaces themselves have
   no such dependency and CAN be wired now once glue.rs mounts the files.
3. **`#[cfg(test)] mod surface_tests`** on the plugin root: add
   `semio_framework_plugin::testkit::{assert_viewer_never_mutates::<AssemblyViewer>, assert_editor_and_viewer_share_dialect::<AssemblyEditor, AssemblyViewer>}` calls (the REAL framework testkit fns, landed by
   W0-F — no local stand-in needed, unlike the cad pilot).

## Blocking gap: `declaration()` is NOT authored — verified, not guessed

`ArtifactDeclaration::builder(definition()?)` requires `.schema(descriptor: ArtifactSchemaDescriptor)`
before `.try_build()` is reachable — this is TYPESTATE-mandatory
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:2829-2903`, `NeedsSchema` ->
`DeclarationReady`, unlocked ONLY by `.schema(...)`). `ArtifactSchemaDescriptor` needs FOUR facets
(`artifact`/`snapshot`/`diff`/`mutations`), each carrying FIVE mandatory `&'static str` leaves (rust/
typescript/graphql/json_schema/proto) via `include_str!` — all non-optional (`FacetLeaves` has no
`Option` fields, `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:159-165`).

Verified on disk (`ls`, twice): assembly's `🧬️schema/📸️snapshot/`, `🔺️diff/`, `🧬️mutations/` each carry
ONLY `🦀️component.rs` + `🟦️component.ts` — no `.json`/`.graphql`/`.proto` leaf anywhere in this
artifact's schema tree, and there is no `🧬️schema/🦀️component.rs` artifact-facet file (the composite
`XArtifact` type + `..._artifact_schema_descriptor()` fn every OTHER migrated artifact has) at all.

Compare `energy.model`, the closest real precedent: its equivalent facet (`🧬️schema/🦀️component.rs`,
with `EnergyModelArtifact`, `energy_model_artifact_schema_descriptor()`, and full 5-leaf sets at every
facet including `🔣️component.json`/`🔗️component.graphql`/`🛰️component.proto`) was built by an EARLIER,
separate wave (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM / ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE)
BEFORE this ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET ticket ever touched it — `energy`'s own
`declaration()` in `🦀️component.rs:313-321` simply CALLS that pre-existing descriptor fn, it doesn't
author it. This ticket's own migration recipe (`📓️w2-cad-report.md`) never asks a W2 surface-authoring
packet to hand-craft schema-descriptor leaves, and confirms every other W2 packet's artifact schema tree
was ALREADY complete before its own packet started.

**This packet's own brief scoped "fully real and already implemented" to snapshot/diff/mutations/
inferences specifically** (verified true — all four are real, tested, compiling) — not to the
descriptor/artifact-facet layer, which turns out to be the one piece missing. Authoring 14 new
hand-crafted GraphQL SDL / JSON-Schema / Protobuf files for a WFC domain this packet did not design,
under a ticket scoped to editor/viewer surface authoring, was judged out of this packet's named
5-item deliverable and a real design-authority question (wrong content here would be a worse defect than
an honest gap) — not silently resolved.

**Consequence**: `crate::artifacts::assembly::declaration()` does not exist. The coordinator's own brief
anticipated wiring `.artifact(crate::artifacts::assembly::declaration())` on the plugin root — that
specific call cannot land until either (a) a follow-up authors the missing schema-facet + 14 leaf files
(recommend a new, explicitly-scoped ticket, NOT bundled into a W2 surface packet), or (b) the framework
gains a lower-ceremony declaration path for schema-first artifacts with a minimal descriptor. Everything
else in this packet — `artifact_kind()`, `definition()`, `ASSEMBLY_DIALECT`, both full surfaces — is
complete, self-contained, and unaffected by this gap (they depend only on `AssemblySnapshot`/
`AssemblyMutation`/`AssemblyDiff`, which are real).

## Open questions for the coordinator

1. **Should assembly be registered as a live artifact/plugin surface now?** Given the `declaration()`
   gap above, `.artifact(...)` cannot be added yet regardless of intent — but `.editor()`/`.viewer()`
   CAN be wired independently (they don't route through `ArtifactDeclaration`). Recommend wiring
   `.editor()`/`.viewer()` once glue.rs mounts these files, and leaving `.artifact()` for whoever closes
   the schema-facet gap — but this is the coordinator's call, not decided here, per the brief's own
   instruction not to silently narrow this either way.
2. **Schema-facet gap ownership.** Recommend a new, narrowly-scoped ticket (schema/protocol authoring,
   not surface authoring) to build assembly's missing `🧬️schema/🦀️component.rs` artifact facet + the
   nine missing JSON-Schema/GraphQL/Protobuf leaf sets, using `energy.model`'s real, complete schema
   tree as the template. Not attempted here.
3. **`AssemblyRule.params: SemioValue` is left at its default in the editor's `CreateRule` command** —
   the underlying `create_rule` mutation builder supports the full field, but no editor affordance for
   authoring arbitrary structured `SemioValue` constraint params exists yet (mirrors `energy.model`'s
   own documented `SetStructureField` two-leaf narrowing). A follow-up, not a defect.
4. **A spatial/mesh view of `AssemblySlot`'s real `x`/`y`/`z` coordinates** (or of a SOLVED assignment,
   once a render-time inference-invocation seam exists) is a plausible upgrade over the tree view — not
   attempted here, see "Window-kit choice" above for the reasoning this first pass used instead.

## Verification performed

- Every new directory/file path verified with `find`/`ls` immediately after creation (caught and fixed
  the `🏅️standards` vs `🏅️标准` typo trap TWICE mid-session, both times before it reached a real
  content file — cleaned up with `rm -rf` immediately, confirmed clean with `find … -iname "*标*"`
  returning empty).
- `//#region`/`//#endregion` balance checked per file (all matched).
- Viewer-purity substrings (`::editor::`, `.mutation(`, `artifact_mutations`, `Emit::mutations`) grepped
  across the whole `👁️viewer` tree post-write: zero matches.
- Every Rust type/fn signature used (mutation builders, `AssemblySlot`/`AssemblyRule`/`AssemblySlotEdge`
  field shapes, `TreeWindowKit`/`TreeNodeView`/`TreeView`, `ArtifactEditor`/`ArtifactViewer` trait member
  shapes, `Editor`/`Viewer`/`EditorBuilder`/`ViewerBuilder` methods) was read from real, on-disk,
  already-compiling source (`energy.model`'s real editor/viewer, the SDK trait definitions in
  `🔌️plugin/🦀️component.rs`, assembly's own schema tree) — none guessed.
- Caught a live concurrent fix in `energy.model`'s viewer file (a peer session correcting
  `semio_framework::AppRole` → `semio_framework_plugin::AppRole`, mid-session, shown via the harness's
  own file-changed notice) and proactively applied the SAME correction to both my new test modules
  before it could become a real error — `semio_framework` is not a direct extern-crate dependency of
  `semio-s-plugin-procedural` (confirmed against this plugin's own `📦️glue.rs` preamble), so
  `semio_framework_plugin::AppRole` (reachable via its blanket `pub use semio_framework::*;` at
  `🔌️plugin/🦀️component.rs:18557`) is the only spelling that resolves.
- Could NOT run `cargo check` — these files are not mounted in `📦️glue.rs` yet (see "What the
  coordinator must wire" above), so there is no compilation unit to check. Every signature was instead
  cross-checked by reading the real callee source, not assumed.

## Files touched

Created:
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
  (+ `🟦️component.ts`)
- `…/✏️editor/🎭️modes/✏️edit/🦀️component.rs`
- `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️component.rs` (+ `🟦️component.ts`, + six
  `📌️empty.md` facet dirs: `🎚️config`/`🎚️options`/`🎬️actions`/`👥️presence`/`🪛️utilities`/`🫧️transient`)
- `…/👁️viewer/🦀️component.rs` (+ `🟦️component.ts`)
- `…/👁️viewer/🎭️modes/👁️view/🦀️component.rs`
- `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🦀️component.rs` (+ `🟦️component.ts`, + six
  `📌️empty.md` facet dirs, same set as above)

Deleted (scaffold placeholders, both surfaces):
- `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/` (whole tree)
- `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/` (whole tree)

NOT touched (outside this lease, per instructions): `📦️glue.rs`, plugin root `🦀️component.rs`,
`Cargo.toml`, tsconfig, `🎛️apps/**`, `🧰️framework/**`, `🗿️artifacts/🌀️procedural2d/**`,
`🗿️artifacts/🧊️procedural3d/**`, assembly's own `🧬️schema/**` (read-only — the missing schema-facet
leaves are a documented gap, not silently authored here).

Scratch (ticket folder): none needed beyond this report — no cargo run was possible (files not yet
mounted), so no `.txt` log to attach.
