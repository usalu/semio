# W2 Packet P7 (gis) — Notes

Lane: W2 packet P7, plugin `🌍️gis`, ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Lease:
`✏️s/🔌️plugins/🌍️gis/**` only. Followed the pilot's recipe (`📓️w2-cad-report.md`), the frozen contract
(`📋️contract-freeze.md`), the closed SDK gaps (`📓️w0-f-report.md`), the parallel-subagent shape
(`📓️w2-p8-report.md`), and this packet's own coordinator-verified ground truth (`📓️w2-p7-ground-truth.md`).

Two apps, two artifacts, confirmed by reading each app's root `🦀️component.rs` before moving anything:

- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/` → `Gis2dPlayApp` (`ArtifactApp::Snapshot = GisMapSnapshot`) →
  artifact `🗺️gismap`.
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/` → `Gis3dPlayApp` (`ArtifactApp::Snapshot = GisTerrainSnapshot`) →
  artifact `🏔️gisterrain`.

Ground truth's app↔artifact pairing confirmed correct.

## DIALECT strings used (verified against each artifact's own `definition()` schema capability, not
the module-private `🚪️io` const of the same name)

- `🗺️gismap`: `pub const GISMAP_DIALECT: Dialect = Dialect { artifact_kind: "s.gis.gismap", standard:
  StandardId("1"), subset: SubsetId::ANY }` — added at
  `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️component.rs`.
- `🏔️gisterrain`: `pub const GISTERRAIN_DIALECT: Dialect = Dialect { artifact_kind: "s.gis.gisterrain",
  standard: StandardId("1"), subset: SubsetId::ANY }` — added at
  `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🦀️component.rs`.

Both match ground truth exactly. Canonical surface ids: `s.gis.gismap@1/*#editor` /
`s.gis.gismap@1/*#viewer`, `s.gis.gisterrain@1/*#editor` / `s.gis.gisterrain@1/*#viewer`. Confirmed the
pre-existing module-PRIVATE `GISTERRAIN_DIALECT`/`GISMAP_DIALECT` consts in each artifact's own
`🚪️io/🦀️component.rs` (2-part `artifact_kind`, unrelated older io/composer purpose) were left
untouched — different file, different scope, no Rust name collision.

## What landed

### Editors (`…/🪆️subsets/✳️any/✏️editor/`)

Both apps' entire retired trees moved across intact into their artifact's `✳️any/✏️editor/`, overwriting
the scaffolder's placeholder leaves first (`rm -rf` the scaffold, then `mv` the real content — no
diff/merge):

- **gismap editor** (from `🎛️apps/◻2d/`): root `🦀️component.rs`, `🎚️config` (+schema), `👥️presence`
  (+schema), `🎮️commands/*` (5 groups: example, features, view, locale, shell), `📌️panels/*` (3:
  artifact, catalogue, inspection), `🗣️terminology`, `🌉️wasm`, `📚️examples/🎬️demo-session`, and
  `🎭️modes/✏️edit/{component.rs, 🪟️windows/🗺️map/{component.rs, 🎚️options/*×5}}`. `🗺️maphost/`
  (an app-only, non-`surfaceChildDirs` facet — `MapHost` projection needing both document and
  `Gis2dConfig`) moved whole into `✏️editor/🗺️maphost/` per pilot step 4 (only editor-side command
  files reference it). 52 files total under `✳️any/✏️editor/`.
- **gisterrain editor** (from `🎛️apps/🧊️3d/`): root `🦀️component.rs`, `🎚️config` (+schema),
  `👥️presence` (+schema), `🎮️commands/*` (3: exaggeration, view, locale), `📚️examples/🎬️demo-session`,
  and `🎭️modes/👁️view/{component.rs, 🪟️windows/🏔️terrain/component.rs}`. No panels/terminology/wasm/
  maphost facet (gis3d never had one). 35 files total under `✳️any/✏️editor/`.

Both `⚙️engine` dirs each app had were confirmed EMPTY (`find … -type f` returned nothing) — nothing to
relocate there, unlike fem's genuinely-shared `app_surface`.

Each app's window that previously had ONLY a `🦀️component.rs` (the map window, the terrain window)
gained a real `🟦️component.ts` twin (typed ViewModel + window-kind id/body-key constants mirroring the
Rust `render()` signature): `✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🟦️component.ts` (namespaced
re-export of the 5 pre-existing option-level `.ts` twins plus its own `Gis2dMapWindowViewModel`),
`✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🟦️component.ts` (`Gis3dTerrainWindowViewModel` +
`Gis3dTerrainPin`). Both surface roots also gained a real `🟦️component.ts` (namespaced re-export, not a
blanket `export *`, mirroring cad's ambiguity-avoidance precedent).

`impl ArtifactApp for X` → `impl ArtifactEditor for X`; `const APP_ID` removed (the plain string tag
consts `GIS2D_PLAY_APP_ID`/`GIS3D_PLAY_APP_ID` were KEPT — still used as `ActionFactory`/controller-id
plain string tags, not a trait const); `const DIALECT: Dialect = crate::artifacts::<artifact>::<X>_DIALECT`
added. `create_gis2d_app()`/`create_gis3d_app()` now return `AppDefinition`
(`Editor::builder(DIALECT)…build_definition()`) instead of `App`; the trailing `.example(...)`/
`.workflow(...)` calls were dropped, not ported (same SDK gap the pilot documented, still open — see
below), noted inline at each `create_*_app()`'s doc comment.

Test-module fallout fixed in both root files per pilot step 7: `VcsArtifactApp<X>` →
`VcsArtifactApp<EditorApp<X>>`, `testkit::new_app::<X>()` → `testkit::new_app::<EditorApp<X>>()`, a new
local `gis2d_app_manifest_for_testkit()`/`gis3d_app_manifest_for_testkit()` wrapper (`App { definition:
create_x_app(), examples: Vec::new() }`) feeding both `new_app_with_registry` and
`testkit::assert_declared_actions_bridge_to_commands::<EditorApp<X>>`. Both `mod tests` blocks got an
explicit `use semio_framework_plugin::EditorApp;` (and the manifest-wrapper fn added to their existing
`use crate::editor::<app>::testkit::{...}` import) rather than relying on `mod testkit`'s own private
`use` being reachable through a sibling `mod tests`'s `use super::testkit::*;` glob — that reachability
is NOT guaranteed by Rust's module-privacy rules for a private (non-`pub`) `use` item across sibling
modules, so this packet made both test modules self-sufficient instead of copying a pattern that may be
latent-broken elsewhere.

`crate::apps::gis2d::`/`crate::apps::gis3d::` → `crate::editor::gis2d::`/`crate::editor::gis3d::` across
every moved file (107 hits found, all rewritten); one cross-app doc-comment reference in gisterrain's
moved `🎚️config/🦀️component.rs` (mirroring gis2d's config) and one in gisterrain's editor root
(`command_from_action`'s doc comment) also fixed, since gis2d itself moved too. Final repo-wide grep for
`crate::apps::`/`apps::gis2d`/`apps::gis3d` inside the whole `🌍️gis` tree: 0 hits.

`include_str!`/`include_bytes!` audit: every macro in both moved trees uses a same-directory or
sibling-relative path (`"🦀️component.rs"`, `"../../👥️presence/🧬️schema/…"`, `"🖼️assets/🎮️demo.cmd.semio"`)
— all targets moved WITH their referrer inside the same subtree, so none needed a depth-delta fix.

### Viewers (`…/✳️any/👁️viewer/`)

Genuinely independent, minimal, real viewers — never importing the sibling editor module:

- **`GisMapViewer: ArtifactViewer`** — `Snapshot = GisMapSnapshot`, `Mutation =
  crate::artifacts::gismap::op::GisMapMutation` (artifact-level, shared with the editor, decode-only
  per contract §2.2). `Config`/`Presence`/`Transient` = framework `NoConfig`/`NoPresence`/`NoTransient`
  (no persisted per-session viewer state needed — camera/render-mode use hardcoded defaults matching
  `Gis2dConfig::default()`, an intentional simplification per contract §2.2/pilot step 8, not a bug).
  `Command = GisMapViewCommand::Noop` (single variant, `#[derive(Default)]` with `#[default]` on the
  variant — required by the real `testkit::assert_viewer_never_mutates<V>()`'s `V::Command: Default`
  bound). One real window, `🗺️map` (`🎭️modes/👁️view/🪟️windows/🗺️map`), rendering the actual
  `GisMapSnapshot` via `crate::artifacts::gismap::schema::gis_map_descriptor_json` (pure, artifact-level)
  through `TiledMapScene::base`/`build_tiled_map_scene` — the SAME framework/schema helpers the editor's
  map window uses, never the editor module itself. `create_gismap_viewer() -> AppDefinition` via
  `Viewer::builder(GISMAP_DIALECT)…build_definition()`.
- **`GisTerrainViewer: ArtifactViewer`** — `Snapshot = GisTerrainSnapshot`, `Mutation =
  crate::artifacts::gisterrain::op::GisTerrainMutation`. Same `NoConfig`/`NoPresence`/`NoTransient` +
  `GisTerrainViewCommand::Noop` shape. One real window, `🏔️terrain`
  (`🎭️modes/👁️view/🪟️windows/🏔️terrain`), rendering the actual `GisTerrainSnapshot` via
  `crate::artifacts::gisterrain::schema::{parse_descriptor, build_terrain_scene_json}` (pure,
  artifact-level) through `world3d_scene_extended`/`build_world_3d_scene` — real imported-overlay pins
  (`instances_json`, mirroring the editor's own pin-instance construction verbatim, duplicated on
  purpose per `policyViewerPurityBreaches`) and the real terrain descriptor JSON, default hardcoded
  camera matching `Gis3dConfig::default()`. `create_gisterrain_viewer() -> AppDefinition`.

Both viewer window placeholder dirs (scaffolder's `🪟️main`) were renamed to the real window names
(`🗺️map`, `🏔️terrain`) before being filled with real content, matching each editor's own window naming.
Each viewer window got a real `🟦️component.ts` twin too. Live filesystem self-check (not the cached
CLI): `grep -rl "SCAFFOLD"` under `🗿️artifacts` → 0 hits; `grep -rl "::editor::"` and
`grep -rln "\.mutation(\|Emit::mutations\|artifact_mutations"` restricted to both `👁️viewer` dirs → 0
hits, confirmed AFTER every edit (re-ran the sweep as the final step below, not just once mid-session).

### `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::gis2d::*`/`apps::gis3d::*` from `../../🎛️apps/◻2d|🧊️3d/…`)
mechanically transformed (Python script: rename region markers, `pub mod apps` → `pub mod editor`,
replace `../../🎛️apps/◻2d/` → `../../🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` and
the gisterrain equivalent — mechanical text substitution over the EXISTING correct nesting, not
hand-retyped) into `//#region ✏️Editor` (`pub mod editor { pub mod gis2d { … } pub mod gis3d { … } }`,
33 `#[path]` leaves, verified against disk before AND after insertion). A new `//#region 👁️Viewer`
(`pub mod viewer { pub mod gismap { … } pub mod gisterrain { … } }`, 6 `#[path]` leaves) was hand-built
(much smaller tree) and inserted right after, mounting from the `👁️viewer/…` siblings — deliberately
never mounting anything under `✏️editor/`. Names: `editor::gis2d`/`editor::gis3d` (kept matching the
apps' own pre-existing names, per pilot step 10's "match the existing naming" rule); `viewer::gismap`/
`viewer::gisterrain` (artifact-name-based, since the viewer tree is new and has no prior app name to
match — self-consistent with the artifact-level `crate::artifacts::gismap`/`crate::artifacts::gisterrain`
paths the viewer files themselves already use).

The crate-root `//#region 📚️Examples` block's `app_2d_demo_session`/`app_3d_demo_session` mounts
repointed from `../../🎛️apps/◻2d|🧊️3d/📚️examples/🎬️demo-session/🦀️component.rs` to the new
`../../🗿️artifacts/<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs`
paths (names kept, only the `#[path]` string changed, matching cad's precedent).

Path-resolution script (pilot step 10's python snippet) run against the WHOLE final `📦️glue.rs`: **154
`#[path]` attributes checked, 0 missing.**

### Plugin root (`✏️s/🔌️plugins/🌍️gis/🦀️component.rs`)

Two `.document_app::<X>(create_x_app())` calls → four calls: `.editor::<crate::editor::gis2d::Gis2dPlayApp>(…)`
+ `.viewer::<crate::viewer::gismap::GisMapViewer>(…)`, `.editor::<crate::editor::gis3d::Gis3dPlayApp>(…)`
+ `.viewer::<crate::viewer::gisterrain::GisTerrainViewer>(…)`. Added `#[cfg(test)] mod surface_tests`
using the REAL `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
assert_editor_and_viewer_share_dialect}` (closed by w0-f gap 2 — NOT local stand-ins, per this packet's
ground-truth instructions) — 4 tests, one pair per artifact.

### Artifact roots

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️component.rs`: added `pub const GISMAP_DIALECT`;
  `.document_codec::<crate::apps::gis2d::Gis2dPlayApp>()` →
  `.document_codec::<semio_framework_plugin::EditorApp<crate::editor::gis2d::Gis2dPlayApp>>()`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🦀️component.rs`: added `pub const GISTERRAIN_DIALECT`;
  same `.document_codec::<EditorApp<crate::editor::gis3d::Gis3dPlayApp>>()` fix.

### `📦️packages/🦀️rust/Cargo.toml`

No literal `🎛️apps/<plugin>` PATH strings found anywhere under `✏️s/🔌️plugins/🌍️gis/📦️packages/`
(unlike cad, gis's own package config never referenced `🎛️apps/…` directly). However, six
`[package.metadata.semio.{playground,assets}]` entries keyed `app = "gis2d-play"` / `app = "gis3d-play"`
— the OLD hand-written `APP_ID` literal, now retired from the trait entirely and replaced by the
DERIVED `surface_app_id` value. Left unfixed this would silently resolve to no app (exactly the class
of bug w0-f's handoff flagged for cad's identical, still-unfixed `app = "cad-play"` lines). Since this
is inside my own lease (unlike the demonstrator's copy of the same string), fixed all six:
`app = "gis2d-play"` → `app = "s.gis.gismap@1/*#editor"` (both playground-variant + both `/osm`,`/vt`
tile-proxy asset rows), `app = "gis3d-play"` → `app = "s.gis.gisterrain@1/*#editor"` (playground variant
+ `/dem` tile-proxy asset row). No `tsconfig.json`/`include` array under `🟦️typescript` referenced the
apps path either — nothing to fix there.

`📜️script.ts`'s repo-wide static path-string array still lists gis's now-deleted `🎛️apps/…` paths
alongside every other already-migrated plugin's — confirmed pre-existing, cosmetic, not this packet's
job (same finding w2-p8's report already made; not re-flagging as new).

### Deletion

`✏️s/🔌️plugins/🌍️gis/🎛️apps/` removed in full (both apps' only remaining content was the two empty
`⚙️engine` dirs and the doc-only `🎛️apps/🦀️component.rs` stub) once every real file had a real
destination, confirmed via a final `find … -depth` listing before `rm -rf`.

## Outside-lease referrer (reported, NOT fixed — per explicit instruction)

`✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs` currently has
`use gis::apps::gis2d::{create_gis2d_app, Gis2dPlayApp};` and
`.document_app::<Gis2dPlayApp>(create_gis2d_app())` — untouched, exactly as instructed (the demonstrator
sub-agent/coordinator's own follow-up, not this lane's). **Exact new path for the coordinator to copy
verbatim**: `gis::editor::gis2d::{create_gis2d_app, Gis2dPlayApp}`, with the builder call becoming
`.editor::<Gis2dPlayApp>(create_gis2d_app())` (`create_gis2d_app` now returns `AppDefinition`, matching
`PluginBuilder::editor::<E: ArtifactEditor>(def: AppDefinition)`'s signature — verified against
`🔌️plugin/🏗️builder/🦀️component.rs`, same as w0-f's cad fix). The demonstrator's own
`📦️packages/🦀️rust/Cargo.toml` also has three `app = "gis2d-play"` metadata lines (`:65`, `:94`, `:101`)
that will need the same `"s.gis.gismap@1/*#editor"` literal once the coordinator's fix lands — mirrors
exactly what w0-f did for cad's two analogous lines in the same file. The demonstrator manifest's own
test `bundle_registers_the_six_demonstrator_surfaces` (or whatever it's now named) has a comment at
`:62` that already anticipates this: `"gis2d-play" stays the coordinator's own follow-up` — confirms the
demonstrator sub-agent was told the same thing this packet's brief tells me.

## SDK gaps (confirms w0-f/pilot findings, nothing new found)

1. `EditorBuilder`/`ViewerBuilder::build_definition()` still has no `.example(...)`/`.workflow(...)` —
   both apps' old example/workflow registrations (`"reuse-map"`, `"reuse-terrain"`,
   `.workflow("gis2d"/"gis3d", …)`) dropped, not ported, noted inline. Each artifact's own
   `📚️examples/🎬️demo-session` facet (pre-existing, real content) is the modern replacement surface,
   per contract.
2. `testkit::assert_declared_actions_bridge_to_commands`'s signature is still `fn(manifest: fn() -> App)`
   — both apps' pre-existing tests calling it needed the same local `App { definition, examples:
   Vec::new() }` wrapper w0-f's Gap 3 note describes.
3. Confirmed the real `assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect` (w0-f gap 2
   closure) work as documented — both used directly in `plugin root`'s `surface_tests`, no local
   stand-ins needed this time (unlike the pilot, which predates the fix).

## Verification

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-gis --all-targets --keep-going`, output in
`🧪️w2-p7-gis-cargo.txt` (three runs across the session):

- Run 1: 1 real error, `error[E0004]: non-exhaustive patterns … OpsHeaderLine::Inverse/Metadata/Message`
  inside `semio-framework-os-kernel`'s `🏪️store/🦀️component.rs:3102`. **0 errors in `🌍️gis` files.**
  Confirmed via `git status --porcelain -- 🧰️framework/…/🏪️store/🦀️component.rs` → `M` (uncommitted) and
  `git log --date=iso -3` on that path → most recent real commit 2026-08-16 14:18:35, tied to the
  concurrent `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` ticket; `stat` mtime showed
  the file modified ~43 seconds before the check ran — a genuinely live, in-progress edit.
- Run 2 (immediate re-run): identical error, same file — the peer session hadn't advanced yet.
- Run 3 (after the last doc-comment fix): the SAME upstream file now fails with a DIFFERENT error,
  `error[E0502]: cannot borrow 'edits' as mutable because it is also borrowed as immutable`, confirming
  the peer session is actively iterating on that exact file between runs (matching the pilot's and
  w0-f's documented "failure moves upstream/changes shape run to run" pattern). **0 errors in `🌍️gis`
  files across all three runs** — `semio-s-plugin-gis` itself was never reached by rustc (the failing
  dependency `semio-framework-os-kernel` blocks the whole build graph before gis's own files are ever
  type-checked), same class of blocker every other W2 packet in this ticket has hit and documented.
  Not fixed — outside this lease (`🧰️framework/**` explicitly forbidden), not gis-specific, confirmed
  live via git evidence rather than assumed from a commit message.

Live filesystem policy self-check (not the cached CLI, per w2-p8's warning): `grep -rl "SCAFFOLD"` under
`🗿️artifacts` → 0; `grep -rl "::editor::"` and `grep -rln "\.mutation(\|Emit::mutations\|artifact_mutations"`
restricted to both `👁️viewer` dirs → 0; repo-wide-within-lease `crate::apps::`/`apps::gis2d`/`apps::gis3d`
→ 0; `🎛️apps/` directory → confirmed deleted (`ls` → No such file or directory).

`cargo test -p semio-s-plugin-gis` was not run separately — it hits the identical upstream blocker
before reaching gis's own test code (same dependency-graph short-circuit as `cargo check`), so a
separate run would add no new signal; will pass once `semio-framework-os-kernel` finishes landing.

## Files touched

Created:
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (52 files —
  moved content + 2 new real `🟦️component.ts` twins: map window + surface root)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️标准/../✳️any/👁️viewer/**` — see corrected path below
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` (19 files —
  `🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the `🗺️map` window; taxonomy
  facet dirs otherwise `📌️empty.md`)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (35 files —
  moved content + 2 new real `🟦️component.ts` twins: terrain window + surface root)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` (19 files)

Edited:
- `✏️s/🔌️plugins/🌍️gis/🦀️component.rs` (plugin root: `.editor()`/`.viewer()`×2 wiring, `surface_tests`)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️component.rs` (`GISMAP_DIALECT`, `.document_codec::<EditorApp<…>>()`)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🦀️component.rs` (`GISTERRAIN_DIALECT`, `.document_codec::<EditorApp<…>>()`)
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions, examples repoint)
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml` (6 `app = "gis2d-play"/"gis3d-play"` → derived surface ids)

Deleted:
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/` (whole tree — both apps, now fully migrated)

Scratch (ticket folder): `🧪️w2-p7-gis-cargo.txt`. This file: `📓️w2-p7-gis-notes.md`.

## Not done / follow-ups for the coordinator

1. Demonstrator's `gis::apps::gis2d::…` import/builder call and its 3 `app = "gis2d-play"` Cargo.toml
   lines — explicitly out of scope, exact replacement strings given above.
2. `📜️script.ts`'s static path array still lists gis's deleted `🎛️apps/…` — pre-existing, cosmetic,
   not blocking, same finding as every prior W2 packet's report.
3. Full `cargo check`/`cargo test` pass for `semio-s-plugin-gis` blocked by the live
   `semio-framework-os-kernel` refactor; re-run once that lands — expected clean based on 0 own-file
   errors across three separate runs at two different points in the peer session's own iteration.
