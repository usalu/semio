# W2 Packet P5 (flow) — Notes

Lane: W2 packet P5, plugin `🌊️flow`, subset `s.flow.flow@1/*`. Recipe followed:
`📓️w2-cad-report.md` (16-step migration recipe). Contract: `📋️contract-freeze.md` §1, §2, §2.6.
SDK gaps from `📓️w0-f-report.md`/`📓️w2-fix-report.md` (crate-root re-exports, testkit helpers)
confirmed already landed — used bare `semio_framework_plugin::{ArtifactEditor, ArtifactViewer, Editor,
Viewer, EditorApp, ViewerApp, ViewEmit, Dialect, StandardId, SubsetId}` throughout, no `::app::`
qualification needed.

## What moved

The whole `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/` app tree moved into
`🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`, preserving internal structure: root
`🦀️component.rs` (990 lines), `🎚️config` (+schema), `👥️presence` (+schema), `🎮️commands/*` (22 groups
at app root + 5 more under `🎭️modes/🧬️generate/🎮️commands/*`), `📌️panels/*` (3: `📄️artifact`,
`🔍️inspection`, `🛍️catalogue`), `🗣️terminology`, `📚️examples/🎬️demo-session` (+ its `🧪️tests`), and
both mode subtrees:
- `🎭️modes/✏️edit/` — windows `🌊️main` (node-graph canvas), `🗣️compiled` (read-only wire-literal text
  editor)
- `🎭️modes/🧬️generate/` — windows `👁️preview`, `📝️form`, `🗂️generations`, plus its own 5 commands
  (`add-generation`/`remove-generation`/`select-generation`/`rename-generation`/
  `update-generation-values`)

Every `crate::apps::flow::` self-reference across the moved tree (51 lines, all files) became
`crate::editor::flow::` via a recursive sed, verified 0 remaining afterward. One extra hand-fix beyond
the mechanical sed: `🎮️commands/👁️node-graph-viewport/🦀️component.rs`'s test helper
`fn preview_off_ids(app: &mut semio_framework_plugin::VcsArtifactApp<crate::apps::flow::FlowPlayApp>)`
— the sed alone would have left `VcsArtifactApp<crate::editor::flow::FlowPlayApp>`, missing the
`EditorApp<…>` adapter wrap now required by the runtime `ArtifactApp` bound; changed to use the
testkit's own `FlowApp` type alias instead (`VcsArtifactApp<EditorApp<FlowPlayApp>>`).

`include_str!`/`include_bytes!` check (recipe step 5): the app tree has exactly two, both relative to
siblings that moved WITH the file (`📚️examples/🎬️demo-session/🦀️component.rs` → its own
`🖼️assets/🎮️demo.cmd.semio`; the demo's own `🧪️tests/🦀️test.rs` → `../🖼️assets/…`), plus
`🎚️config/🧬️schema/🦀️component.rs`'s five self-includes and its five `../../👥️presence/🧬️schema/…`
includes (presence moved to the same new depth alongside config). No cross-boundary include existed;
none needed a depth fix.

`⚙️engine`: checked, confirmed **empty** (`find` returns 0 files) — nothing to migrate, nothing
referencing it (`grep -rn "apps::flow::engine"` — 0 hits). Not a real facet in this plugin, unlike cad's.

`🔨️modules/🧮️compute`: searched the whole `✏️s/🔌️plugins/🌊️flow/` tree — **does not exist** in this
plugin. Not fabricated, not touched.

## Editor (`…/🪆️subsets/✳️any/✏️editor/`)

`impl ArtifactApp for FlowPlayApp` → `impl ArtifactEditor for FlowPlayApp`
(`✏️editor/🦀️component.rs:250`); `const APP_ID` removed, `const DIALECT: Dialect =
crate::artifacts::flow::FLOW_DIALECT;` added (`:263`). `create_flow_app()` (`:433`) now returns
`AppDefinition` via `Editor::builder(crate::artifacts::flow::FLOW_DIALECT)…build_definition()` instead
of `App::from_builder(App::builder(FLOW_PLAY_APP_ID, …))`; the trailing
`.example_source(crate::examples::art_flow_demo::source())` / `.workflow("flow", "Flow", "graph")`
calls were **dropped**, not ported (`EditorBuilder` has no such methods — see "SDK gaps" below), noted
inline at `:535-539`. Two `create_flow_app().definition` call sites (the two manifest-sanity tests,
`:753`/`:773`) had the now-redundant `.definition` stripped since `create_flow_app()` returns
`AppDefinition` directly.

Testkit fallout (`✏️editor/🦀️component.rs:544-632`): `pub type FlowApp = VcsArtifactApp<FlowPlayApp>`
→ `VcsArtifactApp<EditorApp<FlowPlayApp>>`; both `new_app::<FlowPlayApp>()` call sites →
`new_app::<EditorApp<FlowPlayApp>>()`; `new_app_with_registry::<FlowPlayApp>(create_flow_app)` →
`new_app_with_registry::<EditorApp<FlowPlayApp>>(flow_manifest_for_testkit)` with a new local wrapper
`fn flow_manifest_for_testkit() -> App { App { definition: create_flow_app(), examples: Vec::new() } }`
(same framework testkit gap the cad pilot found — `new_app_with_registry` still takes `fn() -> App`).

Every window (5 total, both modes) got a real `🟦️component.ts` twin — typed view-model interfaces
mirroring each window's actual Rust `render()` signature, never an empty file:
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🟦️component.ts` — `FlowMainViewModel` (mirrors
  `render(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession)`,
  `SurfaceKind::NodeGraph`)
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🟦️component.ts` — `FlowCompiledViewModel` (same
  render signature, text-editor scene of the compiled wire literal)
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🟦️component.ts` — `FlowGeneratePreviewViewModel`
  (mirrors `render(config: &FlowConfig)`)
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🟦️component.ts` — `FlowGenerateFormViewModel`
  (mirrors `render(fixture: &FlowSnapshot, config: &FlowConfig)`)
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🟦️component.ts` — `FlowGenerationsViewModel`
  (mirrors `render(config: &FlowConfig, locale: Locale, terminology: Terminology)`)

Surface root `✏️editor/🟦️component.ts` also rewritten (was the scaffold `SCAFFOLD = true` stub) —
namespaced re-exports (`export * as mainWindow from …`, etc., not a blanket `export *`) for all five
windows, mirroring the cad pilot's precedent.

## Viewer (`…/✳️any/👁️viewer/`)

A genuinely independent, minimal, real `FlowViewer: ArtifactViewer`
(`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs`):
- `Snapshot = FlowSnapshot`, `Mutation = crate::artifacts::flow::op::FlowMutation` (both
  artifact-level, shared with the editor — decode-only per contract §2.2).
- `Config`/`ConfigMutation` = framework `NoConfig`/`NoConfigMutation`; `Presence`/`Transient` likewise
  `NoPresence`/`NoTransient`. No persisted per-session camera/canvas state — LOD/grid/proximity use the
  artifact's own pure default constants (`FLOW_DEFAULT_PROXIMITY_DISTANCE`/`FLOW_DEFAULT_GRID_FACTOR`
  from `🧬️schema/🦀️component.rs`, `FLOW_LOD_MODE_AUTOMATIC` from the `flow` kernel crate), documented
  as an intentional simplification.
- `Command` = one-variant `FlowViewCommand::Noop` (`#[derive(Default)]` with `#[default]` on the
  variant, satisfying `assert_viewer_never_mutates`'s `V::Command: Default` bound); `handle` always
  returns `Ok(ViewEmit::default())`.
- One real window, `🌊️main` (`🎭️modes/👁️view/🪟️windows/🌊️main`), rendering the actual `FlowSnapshot`
  through the SAME `SurfaceKind::NodeGraph` / `build_node_graph_scene` shape the editor's own `🌊️main`
  window renders — **not** by calling into the editor module. `fixture_to_workflow`/`split_endpoint`
  (the node/edge → `NodeGraphNodeRecord`/`NodeGraphEdgeRecord` projection) are OWN copies in the
  viewer's window file, duplicated on purpose (contract §2.2's stated cost of genuine independence) —
  built from `flow::flow_host_with_session`/`FlowEvalSession::new()` (kernel-crate-level, not the
  editor module) and a fresh, throwaway, never-persisted `FlowEvalSession` created inside `render`
  itself.
- `create_flow_viewer() -> AppDefinition` via `Viewer::builder(FLOW_DIALECT)…build_definition()`.

`policyViewerPurityBreaches` self-check: `grep -rn "::editor::\|\.mutation(\|Emit::mutations\|artifact_mutations"` over the
whole `👁️viewer/` tree — one hit found and fixed during authoring (a doc comment in the viewer root
file's `🔖️Manifest` region literally typed the forbidden substring while explaining the parity with the
editor's manifest stitch); re-checked clean after rephrasing without the substring.

## `FLOW_DIALECT`

Added at the ARTIFACT level, `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️component.rs`, new
`//#region 🔖️Dialect`:

```rust
pub const FLOW_DIALECT: Dialect = Dialect { artifact_kind: "s.flow.flow", standard: StandardId("1"), subset: SubsetId::ANY };
```

`artifact_kind = "s.flow.flow"` matches this same file's own `definition()` capability row
(`ArtifactCapability::new(ArtifactIdentity::parse("s.flow.schema.artifact")?, …).descriptor(b"s.flow.flow")?`,
verified by reading the row directly, not guessed); `standard`/`subset` match this file's own on-disk
location `🏅️standards/🔖️1/🪆️subsets/✳️any`. Canonical surface ids: `s.flow.flow@1/*#editor` /
`s.flow.flow@1/*#viewer`. Also fixed in the same file: `.document_codec::<crate::apps::flow::FlowPlayApp>()`
→ `.document_codec::<EditorApp<crate::editor::flow::FlowPlayApp>>()` (the runtime `ArtifactApp` bound
needs the SDK adapter). One stray doc-comment reference to the old path fixed too (cosmetic,
`artifact_kind()`'s doc comment).

One more cosmetic doc fix, in the file under live concurrent peer edit
(`🧬️schema/🦀️component.rs`, the MUTATION-OUTCOMES-MERGE-POLICIES ticket's `Mutation::diff` →
`MutationOutcome<Diff>` refactor): `FLOW_DEFAULT_PROXIMITY_DISTANCE`'s doc comment said
"`crate::apps::flow::config`" — updated to "`crate::editor::flow::config`". Did NOT touch anything
else in that file; the peer's `MutationOutcome`-shaped `Mutation` impls elsewhere in it are untouched,
not reverted.

## What I did NOT do (left for the coordinator)

- **`.example_source(…)`/`.workflow("flow", "Flow", "graph")` dropped, not ported** — `EditorBuilder`
  has no such methods (contract §2.4's `App { definition, examples }` split; `PluginBuilder::editor::<E>`
  only takes an `AppDefinition`, so `App.examples` has no carrier through this builder — same SDK gap
  #4 the cad pilot documented, confirmed still present).
- **`#[cfg(test)] mod surface_tests` NOT added by me** — belongs in the plugin root
  (`✏️s/🔌️plugins/🌊️flow/🦀️component.rs`), which is out of my lease. The coordinator should add, next
  to wherever the `.editor()`/`.viewer()` wiring lands:
  ```rust
  #[cfg(test)]
  mod surface_tests {
      use crate::editor::flow::FlowPlayApp;
      use crate::viewer::flow::FlowViewer;
      #[test]
      fn flow_viewer_never_mutates() {
          semio_framework_plugin::testkit::assert_viewer_never_mutates::<FlowViewer>();
      }
      #[test]
      fn flow_editor_and_viewer_share_dialect() {
          semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<FlowPlayApp, FlowViewer>();
      }
  }
  ```
  (both real framework functions now, per `📓️w0-f-report.md` — no local stand-ins needed.)
- **Plugin root `component.rs` and `📦️packages/🦀️rust/📦️glue.rs`** — out of lease, untouched. Plugin
  root currently has exactly one call to fix:
  `.document_app::<crate::apps::flow::FlowPlayApp>(crate::apps::flow::create_flow_app())` (line 11) →
  needs `.editor::<crate::editor::flow::FlowPlayApp>(crate::editor::flow::create_flow_app())` +
  `.viewer::<crate::viewer::flow::FlowViewer>(crate::viewer::flow::create_flow_viewer())`. `glue.rs`
  currently has one `//#region 🎛️Apps`-style region mounting `apps::flow::*` from
  `../../🎛️apps/🌊️flow/…` that needs splitting into `editor`/`viewer` regions pointing at
  `../../🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer}/…`, mirroring the cad/
  puzzle precedent exactly (the editor region's internal directory structure is byte-identical to the
  old apps tree's own listing, so the same "old apps-region text + scoped string substitution" shortcut
  those two packets used should work here too).
- **`Cargo.toml`/`tsconfig.json`/`vitest.config.ts`** — checked (not edited, out of lease):
  `📦️packages/🦀️rust/Cargo.toml` has **zero** references to `🎛️apps/🌊️flow` (only an unrelated
  `variant = "flow"` string at line 19); no `tsconfig*.json` or `vitest*.ts` under
  `📦️packages/🟦️typescript/` references the old path either. Nothing for the coordinator to repoint in
  those three files for this plugin — `📦️glue.rs` is the only build-config file with a stale reference.
- **`✏️s/🔌️plugins/🌊️flow/🎛️apps/` NOT deleted** — per instructions. Left with only two now-empty
  directories (`🎭️modes`, `⚙️engine`) plus the untouched `🎛️apps/🦀️component.rs` mod stub and the
  sibling `🎛️apps/🦀️component.rs` — coordinator's job once glue.rs/plugin root are rewired.

## Outside-lease referrers (checked, none found)

`grep -rln "crate::apps::flow\|apps::flow::\|flow::apps::flow"` across the whole repo, excluding this
plugin's own tree: **zero hits**. The 9 extension crates under `🧩️extensions/{🏗️bim,📃️list,📐️brep,
📖️dictionary,📝️text,🔤️primitive,🖍️draw,🧠️logic,🧮️math}/` — explicitly re-checked, zero references to
`apps::flow`/`crate::apps::flow`, confirming the coordinator's own pre-check. No other plugin (e.g.
demonstrator) imports anything from this plugin's app tree, unlike cad's/puzzle3d's demonstrator
dependency.

## SDK gaps found

None new. Confirmed the W0-F/W2-FIX fixes are live and sufficient for this packet: `ArtifactEditor`,
`ArtifactViewer`, `Editor`, `Viewer`, `EditorApp`, `ViewerApp`, `ViewEmit`, `Dialect`, `StandardId`,
`SubsetId` all resolved bare from `semio_framework_plugin::{…}`, no `::app::` qualification needed
anywhere in this packet's new/edited files (only `InteractionView` needed the `semio_framework_plugin::app::`
qualifier, matching the SAME pattern the editor's ORIGINAL pre-migration file already used for it —
not a new gap, `InteractionView` was never in scope for the W0-F/W2-FIX curated-list additions). This
plugin's windows use `SurfaceKind::NodeGraph`/`Canvas2d` via bare helpers
(`build_node_graph_scene`/`build_text_editor_scene`/`render_generation_preview_text`/
`render_generation_form_body`/`render_generations_tree`) — no window-kit SDK types (`MeshWindowKit`/
`TreeWindowKit`/etc.) were needed, so this packet did not exercise or hit that gap class at all.

The `.example_source(…)`/`.workflow(…)` drop is the same pre-existing SDK gap #4 from
`📓️w2-cad-report.md`, re-confirmed present, not re-discovered.

## Verification

- `rustfmt --edition 2021 --check` run individually against every `🦀️component.rs` in both new trees
  (there is no mounted crate to run a real `cargo check` against yet — my new files aren't reachable
  from `glue.rs` until the coordinator wires them in, exactly as the brief expects). **Zero parse
  errors** across all 59 files — confirms every file is syntactically valid Rust. Brace/paren balance
  double-checked programmatically on top of that (all balanced). The only rustfmt diffs remaining are
  either (a) rustfmt's own line-collapsing preference for long chains/lines, which this codebase
  evidently does not run in its own convention (very long lines are the norm throughout every file I
  read, including in already-shipped cad/puzzle/wires/lowpoly precedent files), or (b) pre-existing
  import-order/brace-style deviations that predate this migration (verified by diffing against what the
  original, unmigrated file already looked like) — neither touched, since fixing (a) would fight the
  established repo style and (b) isn't something this migration introduced. The one class of diff this
  migration DID introduce — `crate::artifacts::flow::` vs `crate::editor::flow::` import ordering
  flipping because "editor" sorts after "artifacts" while "apps" sorted before it — was fixed across
  all 41 affected files (mechanical, safe, whitespace-only reordering; verified with a second full
  rustfmt pass showing 0 remaining `+`/`-` diff lines touching either import path).
- `grep -rln "crate::apps::flow"` across the whole plugin: only `✏️s/🔌️plugins/🌊️flow/🦀️component.rs`
  (plugin root, out of lease) remains. Zero elsewhere, including inside the moved/edited trees.
- `find … -name "🟦️component.ts"` re-run after every write to confirm each twin landed at the intended
  path (never trusted a `Write` result alone).
- Full repo-wide `apps::flow` grep (see "Outside-lease referrers" above): zero hits outside this
  plugin.

## Files touched

Created (moved + real content):
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (114 files —
  moved app tree + 6 new real `🟦️component.ts` twins: 5 windows + surface root)
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` (19 files —
  `🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the `🌊️main` window; remaining
  taxonomy facet dirs otherwise `📌️empty.md` from the W1-E scaffolder, untouched)

Edited:
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️component.rs` (`FLOW_DIALECT`,
  `.document_codec::<EditorApp<…>>()`, 2 doc-comment fixes)
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
  (1 doc-comment fix only, on a file under live concurrent peer edit — that peer's changes untouched)

Deleted: nothing (scaffold placeholder LEAVES were removed only where real content replaced them at
the same or a renamed path — `🎛️apps/🌊️flow/` itself is explicitly not deleted by this packet).

Scratch (ticket folder): none needed beyond this report — all working state lived in the session
scratchpad (`/private/tmp/…/scratchpad/flow-*.txt`), not copied into the ticket folder since none of
it is a durable artifact (path-variable caches and a disk-verification report, all superseded by this
document).
