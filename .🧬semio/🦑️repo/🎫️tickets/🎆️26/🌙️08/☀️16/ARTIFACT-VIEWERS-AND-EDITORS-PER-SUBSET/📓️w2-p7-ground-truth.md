# W2 Packet P7 — Ground Truth (coordinator-verified, read before writing any code)

Read in this order before touching files: `📓️w2-cad-report.md` (recipe, 16 steps), `📋️contract-freeze.md`
§1/§2/§2.6, `📓️w0-f-report.md` (SDK gaps closed — use bare `semio_framework_plugin::{ArtifactEditor,
ArtifactViewer, Editor, Viewer, EditorApp, ViewerApp, ViewEmit}`, no `::app::` prefix needed anymore),
`📓️w2-p8-report.md` (parallel-subagent shape + live-cache warning: NEVER trust `bun ./📜️script.ts policy`'s
cached `compose.json` — call the policy functions directly against the live filesystem, or grep).

## Verified artifact_kind strings (grep'd from each subset's own `#[artifact_schema(id = "…")]`, NOT the
2-part `ArtifactIdentity::parse(…)` string, NOT any pre-existing per-file io `Dialect` const of the same
name — matches the cad pilot's own `CAD_DIALECT.artifact_kind = "s.cad.cad"` precedent, which is the
3-part schema id, not the 2-part identity)

| plugin | artifact (kind dir) | schema id (= DIALECT.artifact_kind) | standard | subset |
|---|---|---|---|---|
| gis | 🏔️gisterrain | `s.gis.gisterrain` | `1` | `*` |
| gis | 🗺️gismap | `s.gis.gismap` | `1` | `*` |
| fem | ◻2d (fem2d) | `s.fem.fem2d` | `1` | `*` |
| fem | 🧊️3d (fem3d) | `s.fem.fem3d` | `1` | `*` |
| energy | 🔋️model | `s.energy.model` | `1` | `*` |
| space | 🏠️home | `s.space.home` | `1` | `*` |
| demonstrator | 🎪️playground | `s.demonstrator.playground` | `1` | `*` |

Name the new pub consts `GISTERRAIN_DIALECT`, `GISMAP_DIALECT`, `FEM2D_DIALECT`, `FEM3D_DIALECT`,
`ENERGY_MODEL_DIALECT` (or `MODEL_DIALECT`, plugin already unambiguous — check sibling `📕️norm` per-kind
naming precedent from w2-p8 which used e.g. `DAG_DIALECT` bare, no plugin prefix, when the plugin has one
artifact — energy has one artifact so `MODEL_DIALECT` is fine), `HOME_DIALECT`, `PLAYGROUND_DIALECT`.
Each lives at the ARTIFACT root `component.rs` (NOT under editor/viewer), per pilot step 6.

CAUTION: gis's `🚪️io/🦀️component.rs` already has module-PRIVATE `const GISTERRAIN_DIALECT`/
`GISMAP_DIALECT` (fem likewise `FEM2D_DIALECT`/`FEM3D_DIALECT`) using the 2-part `artifact_kind` string
("s.gismap" not "s.gis.gismap") for an OLDER, unrelated io/composer purpose. Different file, different
scope (module-private `const` vs `pub const` at artifact root) — no Rust name collision, but do not
confuse the two; do not edit those pre-existing io consts.

## Structural decision 1 — fem's shared compute, energy's shared engine (record verbatim in the report)

**fem**: the "8 plugin-root compute dirs" (`model`, `analyses`, `elements2d`, `elements3d`,
`formulation`, `mesh`, `sparse`, `algebra`) mounted at `📦️glue.rs` crate root are physically at
`✏️s/🔨️modules/🏗️fem/⚙️engine/…` — a SIBLING top-level tree, entirely OUTSIDE this lease's
`✏️s/🔌️plugins/🏗️fem/**` boundary. Not moved, not touched — already plugin/crate-root shared code by
construction, nothing to decide.

The ONE exception genuinely inside the lease: `🎛️apps/◻2d/⚙️engine/🖥️app-surface/🦀️component.rs`
(`pub mod app_surface` at `📦️glue.rs:59-60`). Its own doc comment says it is used by BOTH `fem2d_ui` and
`fem3d_ui`. **Decision: move it to a new plugin-root `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/` dir**
(sibling to `🎮️commands`, `🗿️artifacts`, `📦️packages` at the plugin root), update the single glue.rs
`#[path]` for `app_surface`. It is a shared module, not a surface facet — same "keep genuinely-shared
compute at plugin root" rule the packet brief states, applied to the one in-lease case that needed it.

**energy**: the 50-subdir `⚙️engine` lives at `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/**`
— already plugin-root (sibling to `🗿️artifacts`, never nested under any app; energy has zero apps).
**Decision: leave in place, no move.** It already satisfies the same rule fem's `app_surface` needed
fixing for.

## Structural decision 2 — space's studio app has no artifact of its own

`✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs` (`SpaceApp`) has `ArtifactApp::Snapshot` /
`::Mutation` = `semio_framework_os::{WorkflowSnapshot, WorkflowMutation}` — a FRAMEWORK-owned type
(`🧰️framework/🛍️products/💻️os/**`, outside every plugin's lease, outside `🔌️plugins` entirely).
Registered via `.foreign_document_codec::<SpaceApp>(OS_SPACE_SCHEMA)`, not `.artifact(…)` — there is no
`🗿️artifacts/🪐️space` node anywhere in this crate (confirmed: `find … -name '🪐️space'` under `🗿️artifacts`
returns nothing). `🏠️home`'s own document type is `SHomeSnapshot`/`SHomeMutation` (`s.space.home` schema,
completely different shape) — binding studio's surface to home's subset would be a type-level lie, not a
"defer" convenience.

**Decision: migrate ONLY `🏠️home` in this packet.** `🎛️apps/🏠️home/` moves into
`🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer}/` per the pilot recipe.
`🎛️apps/🪐️space/` (studio) is LEFT IN PLACE, untouched, still registered via the existing
`.document_app::<SpaceApp>(create_space_app())` — do NOT delete the whole `🎛️apps` dir, only remove
`🎛️apps/🏠️home/` once its content has a real new home. **What is left, exactly:** studio needs either
(a) a new framework-level `Dialect`/artifact-kind for the workflow schema (`S_WORKFLOW_SCHEMA`/
`OS_SPACE_SCHEMA`) authored under `🧰️framework/**`, out of every plugin's lease, or (b) the plan to
decide studio doesn't get its own viewer/editor surface at all and stays a plain `ArtifactApp`
indefinitely. Report this, do not invent an artifact.

## Plugin() builder note for energy

Energy's `plugin()` currently ends `.try_library()` (no document app — a headless-library plugin,
`crate::artifacts::model::declaration()` is its only content). Adding `.editor::<E>(…)`/`.viewer::<V>(…)`
calls means it is no longer library-only — check `PluginBuilder`'s `try_library` vs `try_build` (and
whatever typestate gate exists between them) in `🔌️plugin/🏗️builder/🦀️component.rs` before deciding
which terminal call is correct; do not guess, read the builder.

## demonstrator's foreign referrers — do NOT touch the gis import/call yet

`✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs` currently has
`use gis::apps::gis2d::{create_gis2d_app, Gis2dPlayApp};` and
`.document_app::<Gis2dPlayApp>(create_gis2d_app())`. Once gis's own migration (this same packet, a
sibling sub-agent) lands, this becomes `gis::editor::gis2d::{…}` /
`.editor::<Gis2dPlayApp>(create_gis2d_app())`, exactly like w0-f fixed the cad reference. **The
demonstrator sub-agent must NOT touch these two lines or the `bundle_registers_the_six_demonstrator_surfaces`
test** — the coordinator fixes them itself, once, after gis's own sub-agent's report confirms the new
module path, to avoid two concurrent agents editing the same manifest file's same lines from stale
context. The demonstrator sub-agent's own job is authoring `🎪️playground`'s FIRST editor+viewer surfaces
(scaffolded, not migrated from any app — `🎛️apps` is empty for this plugin) and wiring
`.editor::<PlaygroundEditor>(…)` / `.viewer::<PlaygroundViewer>(…)` for ITS OWN artifact into the same
`plugin()` function, which is unavoidably the same file — just leave the gis lines byte-for-byte as they
are today.

Also confirmed: puzzle (`use puzzle::apps::puzzle3d::{…}`, `.document_app::<Puzzle3dPlayApp>(…)`) is
**NOT actually migrated** despite the packet brief's "depends on cad + puzzle, both already migrated"
— verified live: `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/` still holds all three real apps (`◻2d`, `🧊️3d`, `🖐️5d`),
`📦️glue.rs` still has `pub mod apps { pub mod puzzle3d { … } }`, no `pub mod editor` anywhere in that
file. The brief's claim was stale/wrong — leave the puzzle reference exactly as-is (it is still valid,
not broken), report the discrepancy, do not attempt to fix or route around it.
