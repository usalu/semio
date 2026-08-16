# W2 Packet P8 (trinity) — Migration Notes

DRAFT — verification section pending final cargo check/test runs.

Lane: W2 packet P8, plugin `🔱️trinity`, TWO subsets: `s.trinity.jack@1/*` (app `🔌️jack`) and
`s.trinity.rewrite@1/*` (app `♻️rewrite`). Scope: migrate both retired `🎛️apps/*` trees into
`✏️editor`, author real `👁️viewer`s for both, rewire `📦️glue.rs`/plugin root/`Cargo.toml`, delete
`🎛️apps/`. Followed `📓️w2-cad-report.md`'s recipe (steps 1-16).

## What landed

### jack editor (`🗿️artifacts/🔌️jack/…/✳️any/✏️editor/`)

Old app had NO `🎭️modes` dir — its 3 windows (`📝️editor`, `📊️results`, `🌐️graph`) sat directly under
`🪟️windows/`. Moved whole: root `🦀️component.rs` (now `impl ArtifactEditor for TrinityJackPlayApp`),
`🎚️config` (+schema), `👥️presence` (+schema), `🎮️commands/*` (17 groups), `📌️panels/*` (3),
`🗣️terminology`, `🌉️wasm`, `📚️examples/🎬️demo-session`, and the 3 windows now under a newly-authored
`🎭️modes/✏️edit/{🦀️component.rs, 🪟️windows/{🌐️graph,📝️editor,📊️results}}` — the scaffold's single
`🪟️main` placeholder deleted. The app's old `.mode("explore", …)`/`.default_mode_id("explore")`/
inline `jack_layout()` were replaced by a real mode file (`TRINITY_JACK_MODE_EDIT = "edit"`,
`definition()`, `layout()` — the same row-graph/column-editor+results layout, just relocated) stitched
via `.mode_def(edit::definition())`/`.default_mode_id(edit::TRINITY_JACK_MODE_EDIT)`/
`.default_layout(edit::layout())`. `⚙️engine` was checked and found EMPTY on disk (no files, `mod
engine` never declared anywhere) — nothing to move.

`impl ArtifactApp for TrinityJackPlayApp` → `impl ArtifactEditor for TrinityJackPlayApp`; `const
APP_ID`/its backing const removed; `const DIALECT: Dialect = crate::artifacts::jack::TRINITY_JACK_DIALECT`
added. `create_trinity_jack_app()` now returns `AppDefinition` (`Editor::builder(TRINITY_JACK_DIALECT)
…build_definition()`); the trailing `.example("nakagin", …)`/`.workflow("trinity", …)` calls were
**dropped**, not ported (SDK gap, see below).

### rewrite editor (`🗿️artifacts/♻️rewrite/…/✳️any/✏️editor/`)

Same shape: old app had 6 windows (`🎛️parameters`, `⬅️before`, `👈️lhs`, `⏭️after`, `➡️rhs`, `🔎️jack`)
directly under `🪟️windows/`, moved to a new `🎭️modes/✏️edit/{🦀️component.rs, 🪟️windows/*}` (two-row
layout: LHS/RHS/Jack over Parameters/Before/After). `🌍️world` (app-only, no taxonomy slot, real
content — 50KB, the LOD-scale-table helper) moved whole into `✏️editor/🌍️world/`, mounted as a direct
child of `editor::rewrite` in glue.rs (matches the pilot recipe's step-4 guidance: app-only facet with
no taxonomy slot, only editor-side files reference it — **plus jack's own editor `🌐️graph` window
`crate::editor::rewrite::world::trinity_lod_scale_json()`, a same-plugin cross-artifact editor→editor
reference, not a viewer-purity concern**). `⚙️engine` again empty on disk, nothing moved.

Same `ArtifactApp`→`ArtifactEditor` swap, `DIALECT: Dialect =
crate::artifacts::rewrite::TRINITY_REWRITE_DIALECT`, `create_rewrite_app()` → `AppDefinition` via
`Editor::builder(TRINITY_REWRITE_DIALECT)…build_definition()`, `.example`/`.workflow` dropped.

### DIALECT constants

- `pub const TRINITY_JACK_DIALECT: Dialect = Dialect { artifact_kind: "s.trinity.jack", standard:
  StandardId("1"), subset: SubsetId::ANY }` — added at `🗿️artifacts/🔌️jack/🦀️component.rs` (ARTIFACT
  level, not under editor), matching `#[artifact_schema(id = "s.trinity.jack")]` confirmed in that
  subset's own `🧬️schema/🦀️component.rs:15`.
- `pub const TRINITY_REWRITE_DIALECT: Dialect = Dialect { artifact_kind: "s.trinity.rewrite",
  standard: StandardId("1"), subset: SubsetId::ANY }` — added at `🗿️artifacts/♻️rewrite/🦀️component.rs`,
  matching `#[artifact_schema(id = "s.trinity.rewrite")]` confirmed at
  `🧬️schema/🦀️component.rs:16`. Left the unrelated, pre-existing `const DIALECT: Dialect { artifact_kind:
  "s.rewrite", … }` inside `derived_analysis::RewriteAnalyzerAnalysis`
  (`🧬️schema/🦀️component.rs:583`, an `ArtifactAnalysis` impl, different trait/string/mechanism)
  completely alone.
- Both use bare `semio_framework_plugin::{Dialect, StandardId, SubsetId}` — confirmed reachable
  (`semio_framework_plugin` does `pub use semio_framework::*;` at its crate root, and
  `semio_framework`'s own glue.rs does `pub use io::{StandardId, SubsetId, Dialect, ArtifactDialect,
  …};` at ITS crate root) — no `::app::` qualification needed, unlike the pilot's now-stale workaround.

### jack viewer (`…/✳️any/👁️viewer/`)

Genuinely independent `TrinityJackViewer: ArtifactViewer`:
- `Snapshot = JackSnapshot`, `Mutation = crate::artifacts::jack::op::TrinityGraphMutation` (both
  artifact-level, decode-only per contract §2.2).
- `Config`/`Presence`/`Transient` = framework `NoConfig`/`NoPresence`/`NoTransient`.
- `Command = TrinityJackViewCommand::Noop` (one variant, `#[derive(Default, …)]` with `#[default]` —
  needed for `testkit::assert_viewer_never_mutates<V>()`'s `V::Command: Default` bound); `handle`
  always returns `Ok(ViewEmit::default())`.
- One real window, `🌐️graph` (`🎭️modes/👁️view/🪟️windows/🌐️graph`), a read-only node-graph render
  built from `JackSnapshot::nodes()`/`.edges()` (artifact-level pure accessors) +
  `semio_framework_plugin::build_node_graph_scene`/`NodeGraphScene` — the SAME framework helpers the
  editor's Graph window uses, but the node/edge/viewport conversion is a small duplicated pure
  function in the viewer's own window file (never calling into the sibling editor module — the
  editor's `fixture_to_workflow`/`node_to_workflow_record`/`split_endpoint` were NOT reused, matching
  cad's own precedent of accepting light duplication over a real refactor given the time budget).
- `create_trinity_jack_viewer() -> AppDefinition` via `Viewer::builder(TRINITY_JACK_DIALECT)…
  build_definition()`.

### rewrite viewer (`…/✳️any/👁️viewer/`)

Genuinely independent `TrinityRewriteViewer: ArtifactViewer`:
- `Snapshot = RewriteSnapshot`, `Mutation = crate::artifacts::rewrite::op::RewriteRuleMutation`
  (decode-only).
- `Config`/`Presence`/`Transient` = framework `NoConfig`/`NoPresence`/`NoTransient`.
- `Command = TrinityRewriteViewCommand::Noop` (same `Default` pattern as jack).
- One real window, `📜️rule` (`🎭️modes/👁️view/🪟️windows/📜️rule`), built on the framework's
  `TextWindowKit` (contract §2.6): renders the LHS pattern + RHS actions + live parameter bindings as
  one pretty-printed, read-only JSON `TextView`. `TextWindowKit::window_kind()` (the read-only variant,
  no `replace-text` action) is registered directly as the window kind — its id/body_key are the shared
  `framework.window.text` constant per the frozen kit grammar, not a per-app id.
- `create_trinity_rewrite_viewer() -> AppDefinition` via `Viewer::builder(TRINITY_REWRITE_DIALECT)…
  build_definition()`.

Both viewers ship real `🟦️component.ts` twins per window (typed ViewModel + window-kind
id/body-key/surface-id constants) and a surface-root `🟦️component.ts` namespaced re-export
(`export * as graphWindow`/`export * as ruleWindow from …`).

### `📦️glue.rs`

Old single `//#region 🔖️Apps` (`pub mod apps { pub mod jack { … } pub mod rewrite { … } }`, mounting
from `../../🎛️apps/<jack|rewrite>/…`) replaced by two independent regions:
- `//#region ✏️Editor` — `pub mod editor { pub mod jack { … } pub mod rewrite { … } }`, every leaf
  `#[path]`-mounted from `../../🗿️artifacts/<jack|rewrite>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
  The old flat `pub mod windows { pub(crate) mod <name> { … } }` (direct child of the app module) was
  relocated one level deeper, nested inside a new `pub mod modes { pub mod edit { mod component; pub
  use component::*; pub mod windows { … } } }`, matching the physical window relocation.
- `//#region 👁️Viewer` — `pub mod viewer { pub mod jack { … } pub mod rewrite { … } }`, same base but
  `…/👁️viewer/…`, deliberately never mounting anything under `✏️editor/`.

Every `crate::apps::jack::`/`crate::apps::rewrite::` reference across every moved Rust file became
`crate::editor::jack::`/`crate::editor::rewrite::` (mechanical `sed`, editor tree only, both apps in
one pass since jack's own `🌐️graph` window references `crate::editor::rewrite::world::…`). Built
programmatically (Python, brace-matched extraction + path-prefix substring replace on the ORIGINAL
`📦️glue.rs` text) rather than hand-typed, per the pilot's step-10 warning — 375 `#[path]` attributes
verified against disk after the edit, **0 missing**. The bottom `//#region 📚️Examples` mounts for
`app_jack_demo_session`/`app_rewrite_demo_session` repointed at the new editor paths (names kept, only
the `#[path]` strings changed).

### Plugin root (`✏️s/🔌️plugins/🔱️trinity/🦀️component.rs`)

```
.document_app::<crate::apps::jack::TrinityJackPlayApp>(crate::apps::jack::create_trinity_jack_app())
.document_app::<crate::apps::rewrite::TrinityRewritePlayApp>(crate::apps::rewrite::create_rewrite_app())
```
→ four calls:
```
.editor::<crate::editor::jack::TrinityJackPlayApp>(crate::editor::jack::create_trinity_jack_app())
.viewer::<crate::viewer::jack::TrinityJackViewer>(crate::viewer::jack::create_trinity_jack_viewer())
.editor::<crate::editor::rewrite::TrinityRewritePlayApp>(crate::editor::rewrite::create_rewrite_app())
.viewer::<crate::viewer::rewrite::TrinityRewriteViewer>(crate::viewer::rewrite::create_trinity_rewrite_viewer())
```
Added `#[cfg(test)] mod surface_tests` using the CANONICAL
`semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
(confirmed present at `🔌️plugin/🦀️component.rs:6753`/`6772`, closed by W0-F — no local stand-ins
needed, unlike the pilot) — 4 assertions, one pair per artifact.

### `🗿️artifacts/🔌️jack/🦀️component.rs` / `🗿️artifacts/♻️rewrite/🦀️component.rs`

Added the two `DIALECT` consts (above). Fixed the two real (non-comment) `crate::apps::…::` references:
`.document_codec::<crate::apps::jack::TrinityJackPlayApp>()` →
`.document_codec::<EditorApp<crate::editor::jack::TrinityJackPlayApp>>()`, same pattern for rewrite —
the runtime `ArtifactApp` bound needs the SDK adapter, not the authoring trait implementor directly.
Several stray doc-comment references to the old `apps::jack`/`apps::rewrite` paths fixed too
(cosmetic, 8 files total across both artifact trees).

### `📦️packages/🦀️rust/Cargo.toml`

Two `[[package.metadata.semio.playground]]` entries' `app = "trinity-jack-play"` /
`app = "trinity-rewrite-play"` (the old hand-written ids) repointed at the now-derived
`surface_app_id` values `"s.trinity.jack@1/*#editor"` / `"s.trinity.rewrite@1/*#editor"` — verified
`surface_app_id`'s grammar directly rather than assuming (contract §1). No `tsconfig.json` exists for
this plugin's TypeScript package (unlike cad) — nothing to repoint there; confirmed via `find`.

### Deletion

`✏️s/🔌️plugins/🔱️trinity/🎛️apps/` removed in full (both apps' own trees, plus one orphaned
plugin-level stub `🎛️apps/🦀️component.rs` — a doc-comment-only file never `#[path]`-mounted by
glue.rs, dead content, safe to remove alongside).

## Outside-lease referrers (checked, none found)

Grepped the WHOLE repo (not just trinity) for `apps::jack`, `apps::rewrite`, and the literal old path
strings `🎛️apps/🔌️jack`, `🎛️apps/♻️rewrite`:
- Zero real `.rs` hits outside `✏️s/🔌️plugins/🔱️trinity/` itself.
- Literal-path-string hits outside trinity are all historical ticket-folder scratch logs/reports from
  OTHER completed tickets (`.🦑️repo/🎫️tickets/**`), not live code — irrelevant.
- Two real crates DO depend on `semio-s-plugin-trinity` (`✏️s/🔌️plugins/✒️writer` via Cargo.toml, and
  trinity's own `🔨️modules/🔌️jack/{🧠️lsp,🐚️shell}` sub-crates) — both exclusively reference
  `trinity::core::*`/`trinity::lexer::*`/`trinity::artifacts::jack::*` (the unrelated `//#region
  🔤️Jack kernel` / `//#region 🔖️Artifacts` glue.rs regions, both untouched by this packet). Confirmed
  by grep: zero `apps::`/`editor::`/`viewer::` references from either dependent. Not a blocker.

## SDK gaps found (framework, outside this packet's lease — report to W1-A)

1. `TextWindowKit`/`TextView`/`WindowKit` (contract §2.6, the `//#region 🔖️WindowKits` region) are
   NOT in `semio_framework_plugin`'s curated crate-root `pub use app::{ … };` re-export list — same
   gap category W0-F's Gap 1 already fixed for `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/
   `EditorApp`/`ViewerApp`/`ViewEmit`, but the WindowKits region wasn't included in that fix. This
   packet is (as far as its own grep found) the first W2 packet to actually use a WindowKit rather
   than a fully custom window, so the gap hadn't surfaced yet. Workaround: `use
   semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};` — same `::app::` qualification
   the pilot used for the whole `ArtifactEditor` family before W0-F. Every future W2 packet reaching
   for `TableWindowKit`/`TreeWindowKit`/`ImageWindowKit`/`MeshWindowKit`/`DocumentWindowKit`/
   `MediaWindowKit` will hit the identical `E0432`.
2. `.example(...)`/`.workflow(...)` still don't exist on `EditorBuilder`/`ViewerBuilder` (contract
   §2.4's `App { definition, examples }` split) — same gap the pilot documented, confirmed still true
   here; both apps' example/workflow registration calls dropped, not ported.

## Verification

_(cargo check/test output pending — appended once the background run completes; see
`🧪️w2-p8-trinity-cargo.txt` / `🧪️w2-p8-trinity-test.txt` in this folder for the raw logs.)_

## Files touched

Created:
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (moved
  content + new `🎭️modes/✏️edit/🦀️component.rs`)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**`
  (`🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the `🌐️graph` window)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**`
  (moved content + new `🎭️modes/✏️edit/🦀️component.rs`, includes relocated `🌍️world/`)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**`
  (`🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the `📜️rule` window)

Edited:
- `✏️s/🔌️plugins/🔱️trinity/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring ×2, `surface_tests`)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs` (`TRINITY_JACK_DIALECT`, `.document_codec::<EditorApp<…>>()`, doc fixes)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🦀️component.rs` (`TRINITY_REWRITE_DIALECT`, `.document_codec::<EditorApp<…>>()`, doc fixes)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions, examples mounts)
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml` (playground `app =` ids)

Deleted:
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/` (whole tree — both apps + the orphaned stub file)

Scratch (ticket folder): `🧪️w2-p8-trinity-cargo.txt`, `🧪️w2-p8-trinity-test.txt`.
