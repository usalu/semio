# W2 Packet P3-stdio-geometry Report

Lane: W2 packet P3-stdio-geometry, `🗄️stdio` kinds `🧿️semio`(19) `📐️step`(7) `🏗️ifc`(5) `🖊️dwg`(2)
`🖊️dxf`(1) `🧊️gltf`(1) `🧊️obj`(1) `🟪️stl`(1) `☁️ply`(1) `☁️las`(1) `💬️bcf`(1). Recipe followed:
`📓️w2-cad-report.md` (adapted — no app to migrate; both surfaces authored fresh from schema).
Contract: `📋️contract-freeze.md` §1, §2, §2.6.

## Subset count correction (measured, not assumed)

The brief's arithmetic said "44 subsets"; the sum of its own per-kind list is **40**
(19+7+5+2+1+1+1+1+1+1+1). Enumerated on disk with `find … -regex '.*🪆️subsets/[^/]*$'` per kind —
**40 real subset directories**, confirmed twice (before and after authoring). This report and all
counts below use 40.

## What landed

### Viewer (`👁️viewer`, all 40)

Every subset's viewer is a real `ArtifactViewer` impl: `Command` is a single `#[default] Noop`
variant (derives `Default`, matching the framework's own `SurfaceViewerCommand` testkit fixture —
required because `testkit::assert_viewer_never_mutates<V>()` needs `V::Command: Default`), `Config`/
`Presence`/`Transient` = `NoConfig`/`NoPresence`/`NoTransient`, `handle` always returns
`Ok(ViewEmit::default())`, one real `view` mode with one real Main window.

- **39 geometry-ish kinds** (`🧿️semio`, `📐️step`, `🏗️ifc`, `🖊️dwg`, `🖊️dxf`, `🧊️gltf`, `🧊️obj`,
  `🟪️stl`, `☁️ply`, `☁️las`) render through the shared `MeshWindowKit` (`window_kind()`). `render()`
  is a pure `Snapshot -> UiNode` read: one placeholder-box instance per entry of the snapshot's
  **largest top-level JSON array field** (via `serde_json::to_value` introspection, clamped 1–6),
  positioned in a row. This is a deliberately uniform, kind-agnostic signal — real (derived from the
  actual document, not fabricated) but not a bespoke per-kind geometry decoder. Rationale: the brief
  explicitly asks for "one exemplary surface pair and replicate" for semio's 19 subsets and groups
  all ten geometry kinds under one kit; 40 genuinely distinct per-kind vertex/entity extractors
  (`SemioMeshSnapshot.meshes[].primitives[].positions` vs `StepSnapshot`'s STEP entities vs
  `LasSnapshot.points` vs …) is not "thin," and several of these schemas are mid-refactor by the live
  FULL-STDIO peer ticket (see below) — coupling to their exact field names would be the first thing
  to break. Documented here as an intentional simplification, same spirit as the cad pilot's
  hardcoded-camera-defaults note.
- **`💬️bcf`** renders through `TableWindowKit` instead: one row per `BcfSnapshot.topics[]`
  (GUID/Title/Status/Priority/Author), a real, direct field read (no introspection needed — BCF's
  own shape is naturally tabular). Justification recorded in the window file's own doc comment:
  `TreeWindowKit` was considered and rejected because topics carry no parent/child nesting in the
  snapshot (comments/viewpoints are per-topic detail, not a topic tree).

### Editor (`✏️editor`, all 40)

Same window kit per subset, `editable_window_kind()` (frozen actions: `set-vertex` for the Mesh
kit, `set-cell` for Table). `Config`/`Presence`/`Transient`/`Draft` = the four `No*` framework types.

- **`🧿️semio` ✳️mesh is the one fully wired exemplar** contract §2.6 names ("set-vertex for
  meshes"): `SemioMeshEditCommand::SetVertex{mesh_index, primitive_index, vertex_index, point}`,
  `handle()` resolves the real mesh/primitive by index off `doc.snapshot`, bounds-checks the vertex
  index, and emits the subset's own real `SemioMeshMutation::MoveVertex` (found on disk at
  `🧬️schema/🧬️mutations/📍move-vertex/🦠️mutation/🦀️component.rs` — note the `📍` prefix, not a bare
  slug; caught via `find`, not guessed, avoiding the exact emoji-typo trap CLAUDE.md warns about).
  `command_from_action("set-vertex", args)` is also implemented for real (JSON arg parsing), not left
  at the trait default.
- **The other 39** declare `editable_window_kind()` for real (the window really advertises
  `set-vertex`/`set-cell` in its manifest) but `handle()` is a minimal, explicit no-op —
  `Ok(Emit::default())`, never `unreachable!()`. Checked every one of these 39 kinds'
  `🧬️schema/🧬️mutations` enum by hand: all expose only `NoMutation` / whole-document `SetSnapshot` /
  insert-or-remove-by-index variants (`InsertVertex`/`RemoveVertex`, `InsertTriangle`/
  `RemoveTriangle`, …) or (`obj`/`stl`/`las`) a genuine `SetVertex`/`SetTriangleVertices`/`SetPoint`
  by-index replace that a fuller pass could wire the same way `semio.mesh` was — flagged as a
  follow-up, not invented here, per the ticket's explicit instruction ("report, don't invent").

### Rust + TypeScript twins

Every one of the 40×2 windows carries both `🦀️component.rs` and `🟦️component.ts` (never empty) —
400 files total (40 subsets × 2 roles × 5 leaves: surface root ×2, mode root, window ×2). En/de
labels (mode "View"/"Ansicht", "Edit"/"Bearbeiten"; window kit labels are the frozen "Mesh"/"Netz" or
"Table"/"Tabelle" from contract §2.6), English first, no default language.

### `<KIND>_DIALECT` consts

Every viewer/editor root carries its own `pub const <SUBSET>_DIALECT: Dialect` — **duplicated
inline in both surfaces**, not hoisted to a shared artifact-root const the way the cad pilot did.
Deliberate deviation from the pilot's step 6, for two reasons: (1) `🧿️semio` and `🧊️gltf` are
"actively being edited right now" by the live FULL-STDIO peer ticket per this packet's own brief —
touching either artifact's root `🦀️component.rs` (a schema/declaration file, not a surface file) to
add a shared const would be a real collision risk; (2) applying the same non-touching discipline
uniformly to all 40 (rather than 2) is simpler and keeps every edit inside `👁️viewer/**`/`✏️editor/**`,
strictly inside this packet's lease. Every `artifact_kind`/`standard`/`subset` triple was read off
that subset's own `🧬️schema/🦀️component.rs` `impl ArtifactAnalysis for …AnalyzerAnalysis { const
DIALECT }` row (or, for named non-`any` subsets, the schema's own `pub const DIALECT`) — never
guessed. Two real multi-standard subtleties confirmed on disk, not assumed:
- `🖊️dwg` ac1018's own schema `pub use`s ac1024's wholesale, including a `DIALECT` that hardcodes
  `standard: StandardId("ac1024")` — a pre-existing schema-level simplification (ac1018 is documented
  dead code, superseded by real R2004+ decode). My `DWG_AC1018_DIALECT` const correctly says
  `StandardId("ac1018")`, independent of that schema quirk.
- `🏗️ifc` has two real standards (`2x3`, `4`) with different artifact-level schema ids
  (`s.stdio.ifc.2x3` vs `s.stdio.ifc`) and different Snapshot types (`Ifc2x3Snapshot` vs
  `IfcSnapshot`) — not one dialect with two subsets.

### Module wiring — `📦️glue.rs` (shared, 3 concurrent packets)

Re-read fresh immediately before editing (twice — the file grew from 9614 → 10443 → 13709 lines
across the session as two sibling W2 stdio packets, P1-stdio-media and P2-stdio-data, landed their
own regions concurrently). Found the **existing flat-module convention** the P1 packet had already
established (`pub mod editor { pub mod png { … } pub mod jpg_any { … } }`, one flat module per
subset, kind+subset joined by `_`, "any"-suffix dropped only when a kind has exactly one subset ever)
and matched it exactly rather than inventing a nested `kind::subset` tree, for a consistent
`crate::editor::<flat>::…`/`crate::viewer::<flat>::…` addressing scheme repo-wide. Inserted my 40×2
subset module trees as a new `//#region P3-stdio-geometry` / `//#endregion P3-stdio-geometry` block
**inside** the existing `pub mod editor { … }` and `pub mod viewer { … }` blocks (before their
closing brace), never touching P1's or P2's own entries. Disk-verification script (adapted from the
cad pilot's) ran against the whole file before and after insertion: **0 of 2052 `#[path]` attributes
unresolved**, both times.

### Plugin root — `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` (shared, tiny, high-contention)

Re-read fresh immediately before editing. Added a `//#region 👁️✏️SurfacesP3StdioGeometry` block with
`.editor()`/`.viewer()` calls, alongside the P1 packet's own pre-existing region (P2's region landed
after mine, cleanly, untouched by me).

**Critical finding, not in the frozen contract:** `PluginBuilder::editor::<E>`/`::viewer::<V>`
(`🏗️builder/🦀️component.rs:235,259`) require `E::Mutation: protocol::SemanticMutation<E::Snapshot>`
— a bound `ArtifactEditor`/`ArtifactViewer` themselves do **not** require (contract §2.1/§2.2 only
ask for `protocol::Mutation`). `SemanticMutation`'s own doc comment: "Implemented only by
`#[derive(Mutations)]`, never by hand." Checked all 40 subsets' `🧬️schema/🧬️mutations` enums: only
**8** carry that derive (pre-existing, schema-owned) — `semio` **brep/drawing/graph/kit/mesh/object/
table/text**. The other **32** (11 more `semio` subsets + all of `step`/`ifc`/`dwg`/`dxf`/`gltf`/
`obj`/`stl`/`ply`/`las`/`bcf`) carry a pre-existing **hand-rolled** `impl protocol::Mutation<Snapshot>
for XMutation` (predates this ticket) and therefore do not satisfy the bound and cannot be made to by
hand per that same doc comment. Registering any of the 32 in `plugin()` does not compile — confirmed
by attempting it first (see verification below) before trimming to the 8 that work.

**Resolution:** registered only the 8 qualifying subsets in `plugin()`. The other 32 have complete,
real `👁️viewer`/`✏️editor` trees (files exist, compile standalone, pass every taxonomy/policy check —
Rust only checks generic bounds when a generic fn is actually *called*, so an unregistered surface's
own code never hits the `SemanticMutation` bound) but are **not** wired into the plugin's builder
chain, and so are not yet resolvable through `AppRouter`/`OpeningResolver` at runtime. This is an SDK
gap, reported here rather than worked around: fixing it needs either (a) the schema owner migrating
32 pre-existing hand-rolled `Mutation` impls to `#[derive(Mutations)]` (outside this packet's lease,
and would collide with the live FULL-STDIO peer ticket's ongoing edits to several of the same files),
or (b) the framework loosening `PluginBuilder::editor`/`::viewer`'s bound back to plain
`Mutation<Snapshot>`, matching what `ArtifactEditor`/`ArtifactViewer` themselves actually require.
Flagging for W1-A/coordinator triage — this almost certainly also blocks the P1/P2 sibling packets'
own kinds wherever their Mutation types are similarly hand-rolled (their own `plugin()` regions
register many more than 8 of their subsets, which will need the same audit).

## Outside-lease referrers

None found. Nothing outside this lease referenced any symbol this packet created (all-new code), and
this packet's own additions never removed or renamed anything an outside file could have depended on.

## SDK gaps found (report to W1-A/coordinator)

1. **`PluginBuilder::editor`/`::viewer` requires `E::Mutation: SemanticMutation<E::Snapshot>`**, a
   bound not present on `ArtifactEditor`/`ArtifactViewer` themselves and not documented anywhere in
   `📋️contract-freeze.md` §2.1/§2.4. See "Plugin root" above for the full finding and its blast
   radius (32 of this packet's 40 subsets, likely many more across sibling packets).
2. **`TextWindowKit`/`TableWindowKit`/`TreeWindowKit`/`ImageWindowKit`/`DocumentWindowKit`/
   `MediaWindowKit`** are still only reachable via `semio_framework_plugin::app::{…}`, not the
   crate-root curated list (w2-fix's Job 3 added `MeshWindowKit`/`MeshView`/`WindowKit` bare but
   explicitly flagged the other six as out of that packet's scope). `💬️bcf`'s viewer/editor import
   `TableView`/`TableWindowKit` through `semio_framework_plugin::app::` for this reason — same class
   of gap, now confirmed to bite a second window kit.

## Verification actually run

- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --all-targets --keep-going`, three full runs
  (`🧪️w2-stdio-geometry-cargo-run1.txt` → `-run2.txt` → `-run3.txt` = `🧪️w2-stdio-geometry-cargo.txt`,
  ticket folder):
  - Run 1 (before any fix): 1491 errors. Investigated every error whose primary `-->` location fell
    under any of my 11 kinds' `👁️viewer`/`✏️editor` dirs (94 substring hits, narrowed to the real
    primary-location set) and fixed three genuine bugs in my own code: (a) `semio_framework::AppRole`
    should be `semio_framework_plugin::AppRole` (the crate isn't a direct dependency; the plugin crate
    re-exports it via `pub use semio_framework::*;`) — 80 occurrences across all 40 root files'
    tests; (b) `🧊️gltf`'s `GltfMutation` is not reachable through the subset schema path, only through
    `crate::artifacts::gltf::GltfMutation` (its own re-export) — 3 occurrences; (c)
    `protocol::ProtocolError::new(String)` does not exist (it's a closed enum) — fixed the one real
    caller (`semio_mesh`'s `decode_op`) to construct `ProtocolError::Malformed{…}`, matching the
    pattern every other hand-rolled `OpBinary`/`OpText` impl in the repo already uses.
  - Run 2 (after those three fixes): 1354 errors, **0 with a primary location inside any
    `👁️viewer`/`✏️editor` dir anywhere in the repo** (not just mine). Discovered the
    `SemanticMutation` SDK gap here (E0277 on `crate::editor::gltf::GltfAnyEditor` and
    `crate::editor::dxf::DxfAnyEditor` at the plugin-root call sites).
  - Trimmed the plugin root to the 8 qualifying subsets (see above).
  - Run 3 (final): 1337 errors. **0 anchored in any of my 40 subsets' `👁️viewer`/`✏️editor` files, 0
    in `📦️glue.rs`, 0 in the stdio plugin root.** The only errors still touching a `👁️viewer`/
    `✏️editor` path anywhere in the repo (54, all `E0308`/`E0053`/…) are inside `🎞️pptx` — the P2
    packet's own lease, confirmed via `git status --porcelain` (modified, uncommitted) at the time of
    this run. Every one of the remaining ~1283 errors is inside `🧬️schema/**`/`🚪️io/**` (never my
    lease) or framework files (`📡️spr/🎮️command/🦀️component.rs`, `🏗️builder/🦀️component.rs`,
    `🔌️plugin/🦀️component.rs`) — confirmed `git status --porcelain -- <file>` = modified/uncommitted
    for a sample of each category, matching the live `MUTATION-OUTCOMES-MERGE-POLICIES-AND-
    FIRST-CLASS-CONFLICTS` ticket's `.apply()` return-type refactor (`T` → `Result<T,
    MutationApplyError>`) breaking pre-existing test assertions repo-wide, exactly the class of
    breakage this packet's brief pre-warned about.
- Live-filesystem policy check, `bun ./📜️script.ts policy` (repo root, full run,
  `🧪️w2-stdio-geometry-policy-run1.txt`/`-run2.txt`), grepped for `taxonomy/surface-completeness`,
  `taxonomy/surface-scaffold-residue`, `taxonomy/viewer-purity` restricted to this packet's 11 kind
  paths: **0 breaches, both runs** (run2 taken after all Rust fixes, to catch anything a file rewrite
  might have reintroduced — none did). Independently confirmed with a direct Python walk of all 40
  subsets' `👁️viewer`/`✏️editor` trees: 0 files containing `SCAFFOLD`, 0 files containing `::editor::`
  under any `👁️viewer` dir.

## Files touched

Created (400 files — 40 subsets × {`👁️viewer`,`✏️editor`} × {surface root `🦀️/🟦️`, mode root `🦀️`,
window `🦀️/🟦️`}), one tree per subset at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<kind>/🏅️standards/<std>/🪆️subsets/<subset>/{👁️viewer,✏️editor}/**`
for: `🧿️semio` (animation, any, audio, brep, cad, document, drawing, flow, graph, image, kit, mesh,
model, object, presentation, table, text, value, video), `📐️step` (any, cc1–cc6), `🏗️ifc` (2x3/any,
2x3/cobie, 2x3/cv20, 2x3/sav, 4/any), `🖊️dwg` (ac1018/any, ac1024/any), `🖊️dxf` (r12/any), `🧊️gltf`
(2.0/any), `🧊️obj` (3.0/any), `🟪️stl` (ascii/any), `☁️ply` (1.0/any), `☁️las` (1.0/any), `💬️bcf`
(2.1/any).

Edited:
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` — new `P3-stdio-geometry` region inside the
  existing shared `pub mod editor {}`/`pub mod viewer {}` blocks.
- `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` — new `SurfacesP3StdioGeometry` region, 8 subsets registered
  (see SDK gap above for why not 40).

Not touched (outside this lease, by design): any `🧬️schema/**`, `🚪️io/**` file; any artifact-root
`🦀️component.rs` (declaration/registration files); the framework SDK files where the
`SemanticMutation` bound and the six not-yet-bare window kits live.

Scratch (ticket folder): `🧪️w2-stdio-geometry-cargo-run1.txt`, `-run2.txt`, `-run3.txt`,
`🧪️w2-stdio-geometry-cargo.txt` (= run3), `🧪️w2-stdio-geometry-policy-run1.txt`, `-run2.txt`.
Generator scripts (not part of the repo, session scratchpad only, kept for provenance):
`gen_subsets.py`, `write_surfaces.py`, `gen_glue_block.py`, `gen_plugin_root.py`.
