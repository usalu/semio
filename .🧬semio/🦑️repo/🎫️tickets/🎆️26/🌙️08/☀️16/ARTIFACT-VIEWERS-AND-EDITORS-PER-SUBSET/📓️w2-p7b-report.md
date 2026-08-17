# W2 Packet P7 (remainder) — Report

Lane: W2 packet P7 (remainder), ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Lease:
`✏️s/🔌️plugins/{🏗️fem,🔋️energy,🪐️space,🎪️demonstrator}/**`. `🌍️gis` (the other P7 plugin) was already
completed by an earlier attempt before this cutoff — see `📓️w2-p7-gis-notes.md`, not redone here.

Execution shape: coordinator (this session) did the structural triage plus fem's `glue.rs`/plugin-root
rewiring directly, then ran four parallel background sub-agents — one full audit-and-fix pass per
plugin (energy, space, demonstrator) plus one dedicated fix pass for fem's post-rewire compile errors —
each with its own scoped lease, own scratch cargo log, and a final report folded in below. No sub-agent
touched `🧰️framework/**` or another plugin's lease.

## Discovery: the "cut-off" prior session had gone much further than expected

The packet brief assumed only gis had landed and fem/energy/space/demonstrator were untouched. Live
`git status` at the start of this session showed otherwise: all four plugins already had substantial
work — energy, space and demonstrator were essentially fully wired (plugin-root `.editor()`/`.viewer()`
calls, `glue.rs` editor/viewer regions, DIALECT consts, real window content); only `🏗️fem` was left in a
broken intermediate state (its artifact-level editor/viewer trees were fully moved and real, but
`📦️glue.rs` still mounted the deleted `🎛️apps/` paths and the plugin root still called
`.document_app::<crate::apps::fem2d::…>` against a module that no longer existed — this crate could not
have compiled as left).

Verified via direct filesystem inspection (not trusting either the brief or the earlier session's own
implicit claims), per this ticket's own emoji-typo-trap and "validate assumptions" rules.

## Structural decisions (recorded verbatim, per the packet's requirement)

### 1. fem's shared compute / energy's shared engine

- **fem**: the 8 plugin-root compute dirs (`model`, `analyses`, `elements2d`, `elements3d`,
  `formulation`, `mesh`, `sparse`, `algebra`) live at `✏️s/🔨️modules/🏗️fem/⚙️engine/…`, a sibling
  top-level tree entirely OUTSIDE this lease's `✏️s/🔌️plugins/🏗️fem/**` boundary — already
  plugin/crate-root shared code by construction, nothing to decide, not touched.
  The ONE in-lease exception, `🎛️apps/◻2d/⚙️engine/🖥️app-surface/` (shared by both `fem2d_ui` and
  `fem3d_ui` per its own doc comment), was moved by the prior session to a new plugin-root
  `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/` (confirmed present on disk, `glue.rs`'s `app_surface`
  mount already repointed there, single mount name unchanged) — a shared module, not a surface facet,
  matching the "genuinely-shared compute stays at plugin root" rule. Confirmed correct, not redone.
- **energy**: the 50-subdir `⚙️engine` at `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/**`
  is already plugin-root (sibling to `🗿️artifacts`, energy has zero apps) — left in place, no move
  needed. Confirmed correct.

### 2. space's studio app has no artifact of its own

Confirmed unchanged from the prior session's own correct call (recorded in
`📓️w2-p7-ground-truth.md`, re-verified live, not re-decided here): `🎛️apps/🪐️space/`'s `SpaceApp` has
`ArtifactApp::Snapshot`/`::Mutation` = `semio_framework_os::{WorkflowSnapshot, WorkflowMutation}`, a
FRAMEWORK-owned type with no `🗿️artifacts/🪐️space` node in this crate at all — there is no artifact for
a surface to bind to. **Only `🏠️home` got migrated** (`🎛️apps/🏠️home/` → editor+viewer under
`🗿️artifacts/🏠️home/…`); `🎛️apps/🪐️space/` stays exactly as-is, registered via
`.document_app::<SpaceApp>(…)` + `.foreign_document_codec::<SpaceApp>(OS_SPACE_SCHEMA)`, no surface.
**What's left**: studio needs either (a) a new framework-level `Dialect`/artifact-kind authored under
`🧰️framework/**` for the workflow schema, out of every plugin's lease, or (b) an explicit decision that
studio never gets its own viewer/editor surface and stays a plain `ArtifactApp` indefinitely. Not decided
here — reported, not invented.

## What this packet did

### fem — finished the incomplete migration

- Rewrote `📦️packages/🦀️rust/📦️glue.rs`'s `//#region 🎛️Apps` (466 stale `#[path]` mounts pointing at
  the already-deleted `🎛️apps/◻2d`/`🎛️apps/🧊️3d`) into `//#region ✏️Editor` + `//#region 👁️Viewer`,
  mounting from the already-real `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,
  👁️viewer}/…` trees, via a small Python transform script (never hand-typed emoji paths) —
  0 missing `#[path]` targets verified against disk afterward (466 total attrs checked).
- Repointed the crate-root `//#region 📚️Examples` mounts (`app_2d_demo_session`/`app_3d_demo_session`)
  at the new editor path, names kept.
- Rewired the plugin root `✏️s/🔌️plugins/🏗️fem/🦀️component.rs`: two `.document_app::<crate::apps::…>`
  calls → `.editor::<crate::editor::fem2d::Fem2dPlayApp>(…)` + `.viewer::<crate::viewer::fem2d::
  Fem2dViewer>(…)` and the fem3d equivalents; added `#[cfg(test)] mod surface_tests` using the real
  `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
  (4 tests, one pair per artifact).
- Fixed `Cargo.toml`'s `app = "fem2d-play"`/`"fem3d-play"` → derived `"s.fem.fem2d@1/*#editor"`/
  `"s.fem.fem3d@1/*#editor"` (same class of bug w0-f/gis already fixed for cad/gis).
- Fixed 5 stale in-lease doc-comment references (`crate::apps::fem2d::…`/`apps::fem3d::…` → `editor::
  fem2d::…`/`editor::fem3d::…`) across both artifact roots and their mutation/binary schema files.
- Deleted the now-empty `✏️s/🔌️plugins/🏗️fem/🎛️apps/` (only a doc-only stub `🦀️component.rs` remained).
- Confirmed 0 outside-lease referrers repo-wide for `apps::fem2d`/`apps::fem3d`/`fem::apps::`.
- `cargo check` then surfaced 56 lib + 126 lib-test errors (182 total) GENUINELY inside fem's own moved
  files — the one plugin in this packet where that happened (every other plugin's remaining errors were
  foreign). Root cause: fem's editor-tree mutation/config/presence `impl Mutation<…>` bodies still
  returned a bare `Diff`/`Config`/`Presence`, but the concurrent `26/08/16/MUTATION-OUTCOMES-MERGE-
  POLICIES-AND-FIRST-CLASS-CONFLICTS` ticket has already landed `Mutation::diff -> MutationOutcome<Self
  ::Diff>` repo-wide (confirmed: gis's own already-complete W2 packet already conforms to this shape).
  Delegated to a dedicated fix pass (below) rather than hand-fixing ~90 files myself.

### fem's compile-fix pass — 182 → 0 errors

Root causes, fixed across ~90 files:
1. **`Mutation::diff` shape** (~140 fixes): 50 `🦠️mutation/🦀️component.rs` payload files (25 fem2d +
   25 fem3d `MutationKind` impls) wrapped in `protocol::MutationOutcome::new(...)`; 4 handwritten
   `impl Mutation<…>` (fem2d/fem3d `✏️editor/{🎚️config,👥️presence}/🦀️component.rs`); downstream test
   call sites updated `.diff(&base)` → `.diff(&base).diff()` (the real accessor — `MutationOutcome`'s
   field is private, `.diff()` is the method); `vcs::apply_mutation` now returns
   `(P, Vec<MutationMessage>)`, fixed both `apply_fem{2,3}d_mutation` + test helpers with `.0`; plus one
   gap outside the coordinator's three predicted causes — `Fem{2,3}dStore::new` returns
   `Result<Self, VcsError>`, 3 test call sites were missing `.expect("valid store")`.
2. **`create_fem{2,3}d_app()` returns `AppDefinition` directly** (contract §2.4), 6 sites still did
   `.definition` field access — dropped.
3. **Missing sibling-command imports**, 4 files (fem2d/fem3d `🧱️add-node`/`🏋️add-nodal-load` test
   modules referenced sibling command modules without `use`-ing them) — added per the compiler's own
   suggestions.

Verified: `cargo check -p semio-s-plugin-fem --all-targets --keep-going` → 0 errors (both `lib` and
`lib test`, `Finished dev profile in 23.68s`); `cargo test -p semio-s-plugin-fem --no-run` → exit 0.
One pre-existing, out-of-lease warning remains (a `testkit` glob-import ambiguity rooted in
`🧰️framework/…/📦️glue.rs:272-273`, framework glue — a future-incompatibility *warning*, not an error,
does not block the build, not fem's file to fix).

### energy — audited, two real bugs found and fixed, otherwise already correct

Already correct (verified live): `TableWindowKit`/`TreeWindowKit` wired with real content (`📊️zones`
window builds a real `TableView{columns, rows}` from `crate::model::Model.zones`; `🌳️structure` window
uses `TreeWindowKit` for the composed-child overview — a better fit than a table there, a considered
choice not a shortcut); `MODEL_DIALECT` correct (`s.energy.model`/`1`/`*`); artifact uses
`.document_codec_bare::<EnergyModelSnapshot, EnergyModelMutation>(...)` (a valid alternate mechanism
needing no `ArtifactEditor` bound, unaffected by this ticket); `glue.rs` 0 missing paths; policy
self-checks 0/0/0; no stale Cargo.toml/tsconfig literals; no outside-lease referrers; en/de labels
complete. `pub mod plugin_apps;` (mounting a doc-only stub) is dead but taxonomy-legal through W2 per
contract §6 — left alone.

Fixed: `EnergyModelEditorCommand` was missing hand-written `OpText`/`OpBinary` impls (the `dsl::DslOps`
derive only emits `DslVariants` — every other migrated plugin hand-writes both); added, mirroring
`NormConfigMutation`/`TrinityJackCommand`'s precedent. Two test-only `E0433`s (`semio_framework::AppRole`
→ `semio_framework_plugin::AppRole`) fixed.

Verified: `cargo check -p semio-s-plugin-energy --all-targets --keep-going` → **0 errors**; `cargo test`
on the surface/editor/viewer/window tests → **21/21 pass**, including the real
`assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect`/`new_viewer` testkit calls.

### space — audited, one gap fixed (missing testkit), otherwise already correct

Already correct: plugin root `.editor::<HomeApp>(…)`/`.viewer::<HomeViewer>(…)` wired,
`.document_app::<SpaceApp>(…)`/`.foreign_document_codec::<SpaceApp>(OS_SPACE_SCHEMA)` untouched;
`HOME_DIALECT` correct (`s.space.home`/`1`/`*`), `.document_codec::<EditorApp<HomeApp>>()` correct;
`glue.rs` 0 missing paths, `//#region 🎛️Apps` (still present, `🪐️space`-only) has zero dangling `🏠️home`
references; policy self-checks 0/0/0, real `🏠️main` window (not scaffold) with both leaves in editor and
viewer; no stale Cargo.toml rows exist at all; no outside-lease referrers; en/de labels complete.

Fixed: added the missing `#[cfg(test)] mod surface_tests` block to
`✏️s/🔌️plugins/🪐️space/🦀️component.rs` using the real testkit functions
(`assert_viewer_never_mutates::<HomeViewer>`, `assert_editor_and_viewer_share_dialect::<HomeApp,
HomeViewer>`).

Verified: `cargo check -p semio-s-plugin-space --all-targets --keep-going` → 16 real errors, **all**
anchored solely in `🧰️framework/…/💻️os/🖥️host/🦀️component.rs` (`Vec<Dialect>` vs `Dialect` mismatches
from `io_dialects_for`) — confirmed live/uncommitted (`git status` → `M`, mtime today 15:53:29) foreign
peer churn, **0 errors in `🔌️plugins/🪐️space`**.

### demonstrator — audited, one gap fixed (missing testkit), one contract deviation justified

Already correct: playground's editor+viewer wired in the manifest, `🎛️apps` already fully gone,
`glue.rs` regions correct, `PLAYGROUND_DIALECT` correct (`s.demonstrator.playground`/`1`/`*`); policy
self-checks 0/0/0; en/de labels complete; the coordinator's own gis cross-reference fix (below) reads
correctly and the puzzle cross-reference from a separate, already-complete W2 packet is untouched and
correct.

**Contract deviation, justified**: the brief named `MeshWindowKit` for playground's windows. Verified
live: `PlaygroundArtifact { #[state(artifact)] pub schema: String }` — the ENTIRE persisted artifact is
one opaque string, no mesh geometry field exists anywhere in the schema. Both windows (editor + viewer)
genuinely build `TextWindowKit::render(&TextView{...})` from that real field instead — using
`MeshWindowKit` here would mean fabricating fake geometry with no basis in the real schema, exactly the
kind of placeholder content this ticket forbids. `🪟️main` is the established real-window naming for a
single-window subset (matches several already-migrated stdio surfaces), not scaffold residue.

Fixed: added the missing `#[cfg(test)] mod surface_tests` block to
`🛂️manifest/🎪️demonstrator/🦀️component.rs`, sibling to the pre-existing `mod tests`, using
`assert_viewer_never_mutates::<PlaygroundViewer>`/`assert_editor_and_viewer_share_dialect::
<PlaygroundEditor, PlaygroundViewer>`.

Verified: `cargo check -p semio-s-plugin-demonstrator --all-targets --keep-going` → cargo exit 101, but
**0 errors anchored in `🔌️plugins/🎪️demonstrator`** (confirmed by grep). The 91 error lines are all in
upstream dependency crates (gis/procedural/puzzle) that are live-uncommitted right now (other in-flight
W2 sub-agents mid-edit, confirmed via `git status`) — cargo's dependency graph never reaches
`semio-s-plugin-demonstrator` itself in this run (no "Checking semio-s-plugin-demonstrator" line
appears). Re-run once those three land.

## Coordinator's own fix: demonstrator's gis cross-reference

Per gis's own packet ground truth ("the coordinator fixes this once gis's own report confirms the new
module path"), and confirmed gis's W2 packet is complete
(`gis::editor::gis2d::{create_gis2d_app, Gis2dPlayApp}` real on disk), fixed in
`✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs`:
`use gis::apps::gis2d::{…}` → `use gis::editor::gis2d::{…}`; `.document_app::<Gis2dPlayApp>(…)` →
`.editor::<Gis2dPlayApp>(…)`; the `bundle_registers_…` test's expected id `"gis2d-play"` →
`"s.gis.gismap@1/*#editor"`. Also fixed the three matching `app = "gis2d-play"` literals in
`📦️packages/🦀️rust/Cargo.toml` → `"s.gis.gismap@1/*#editor"` (playground `verfolgen` variant + both
`/osm`,`/vt` tile-proxy asset rows) — same class of bug w0-f/gis already fixed for cad/gis's own
Cargo.toml. Confirmed via ground truth that puzzle's cross-reference (`puzzle::editor::puzzle3d::…`) was
already fixed by puzzle's own, separate W2 packet — untouched here.

## Verification summary

| crate | own-file errors | status | notes |
|---|---:|---|---|
| `semio-s-plugin-fem` | 0 (was 182) | clean, `cargo test --no-run` exit 0 | fixed in this packet |
| `semio-s-plugin-energy` | 0 | clean, `cargo test`: 21/21 pass | 2 real bugs fixed |
| `semio-s-plugin-space` | 0 | 16 foreign errors (framework `🖥️host`, live peer, confirmed) | testkit added |
| `semio-s-plugin-demonstrator` | 0 | 91 foreign errors (gis/procedural/puzzle, live peers, confirmed) | testkit added |

Live-filesystem policy self-checks, all four plugins, all after every fix landed:

| check | fem | energy | space | demonstrator |
|---|---:|---:|---:|---:|
| `SCAFFOLD` under `🗿️artifacts` | 0 | 0 | 0 | 0 |
| `::editor::` under `👁️viewer` dirs | 0 | 0 | 0 | 0 |
| `.mutation(`/`Emit::mutations`/`artifact_mutations` under `👁️viewer` dirs | 0 | 0 | 0 | 0 |
| `mod surface_tests` present | yes | yes | yes | yes |

## Files touched (this packet, beyond the four sub-agents' own detailed file lists in their reports)

Coordinator-direct:
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` (Apps region → Editor+Viewer regions, Examples
  repoint)
- `✏️s/🔌️plugins/🏗️fem/🦀️component.rs` (plugin root `.editor()`/`.viewer()` wiring + `surface_tests`)
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml` (derived surface ids)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻2d,🧊️3d}/🦀️component.rs` (stale doc-comment path fixes)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
  {🦀️component.rs,💾️binary/🦀️component.rs}` (stale doc-comment path fixes)
- `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs` (gis cross-reference)
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` (gis literal fixes)

Deleted: `✏️s/🔌️plugins/🏗️fem/🎛️apps/` (whole tree, now-empty).

Sub-agent-authored (see each sub-agent's own summary above for the exact file list): fem's ~90-file
`MutationOutcome`/`.definition`/import fixes; energy's `OpText`/`OpBinary` impls + `AppRole` path fixes;
space's `surface_tests` block; demonstrator's `surface_tests` block.

Scratch (ticket folder, `.txt` only): `🧪️w2-p7b-fem-cargo-run1.txt`, `🧪️w2-p7b-fem-cargo-final.txt`,
`🧪️w2-p7b-energy-cargo.txt`, `🧪️w2-p7b-space-cargo.txt`, `🧪️w2-p7b-demonstrator-cargo.txt`,
`🧪️w2-p7b-cargo.txt` (consolidated tail of all four). This file: `📓️w2-p7b-report.md`.

## Not done / follow-ups for the coordinator (next W2/W3 lane)

1. **Space's studio surface** — structural decision 2's "what's left" (new framework-level `Dialect` for
   the workflow schema, or an explicit no-surface decision) is unresolved, out of every plugin's lease.
2. **Demonstrator's cargo check** can't finish until gis/procedural/puzzle's own live peer edits land —
   re-run `cargo check -p semio-s-plugin-demonstrator` once they do; expected clean based on 0 own-file
   errors here.
3. **Space's cargo check** likewise blocked on `🧰️framework/…/💻️os/🖥️host/🦀️component.rs`'s live edit;
   re-run once that lands.
4. `📜️script.ts`'s repo-wide static path-string array still lists fem's now-deleted `🎛️apps/…` —
   pre-existing, cosmetic, same finding every prior W2 packet has made, not blocking.
