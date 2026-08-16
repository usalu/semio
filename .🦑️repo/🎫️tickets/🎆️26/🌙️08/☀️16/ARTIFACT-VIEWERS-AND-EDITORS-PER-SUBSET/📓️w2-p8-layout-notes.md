# W2 Packet P8 (layout) — Migration Notes

Lane: W2 packet P8, plugin `📏️layout`, subset `s.layout.layout@1/*`. Scope: migrate the retired
`✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/` app into `✏️editor`, author a real `👁️viewer`, rewire
`📦️glue.rs`, and verify. Followed `📓️w2-cad-report.md`'s migration recipe (steps 1-16) and used the
w0-f-closed canonical SDK names/testkit functions per `📓️w0-f-report.md`.

## What moved where

### Editor (`…/🪆️subsets/✳️any/✏️editor/`)

The entire old app tree moved across intact: root `🦀️component.rs` (now `impl ArtifactEditor for
LayoutPlayApp`), `🎚️config` (+schema), `👥️presence` (+schema), `🎮️commands/*` (19 payload modules),
`📌️panels/*` (4: artifact/document, catalogue, inspection, preflight), `🗣️terminology`, `🌉️wasm`,
`📚️examples/🎬️demo-session`, and the mode subtree `🎭️modes/✏️edit/{component.rs, 🪟️windows/{📐️blueprint,
👁️preview}}`. Both windows gained a real `🟦️component.ts` twin (typed `LayoutBlueprintViewModel`/
`LayoutPreviewViewModel` + window-kind id/body-key/surface-id constants) — they had none before this
packet (the pre-existing app predates the `windowLeafLangs` requirement). The surface root also gained
a real `🟦️component.ts` (namespaced re-export of both windows' twins — `export * as blueprintWindow
from …`/`export * as previewWindow from …`, not a blanket `export *`, since Preview's view-model
imports Blueprint's `LayoutCameraViewModel`).

Two non-standard, app-only facets (no `surfaceChildDirs` slot): `⚙️engine` (the headless
`LayoutEngine`/scene/export module, split `component.rs` + `🎬️scene/component.rs`) and `🖼️canvas`
(the shared `canvas_layers` host-layer builder both windows call). Both moved WHOLE into
`✏️editor/⚙️engine/` and `✏️editor/🖼️canvas/` respectively (recipe step 4's "only editor-side files
reference it" branch — confirmed by grep before moving: nothing under the subset's own `🧬️schema`/
`💡️inferences` referenced either).

`impl ArtifactApp for LayoutPlayApp` → `impl ArtifactEditor for LayoutPlayApp`; `const APP_ID` removed;
`const DIALECT: Dialect = crate::artifacts::layout::LAYOUT_DIALECT` added. `create_layout_app()` now
returns `AppDefinition` (`Editor::builder(LAYOUT_DIALECT)…build_definition()`) instead of `App`; the
trailing `.example("sample", …, "cylinder")` / `.workflow("layout", "Layout", "layout")` calls were
**dropped**, not ported — see "SDK gaps" below (already closed/documented by w0-f, not a new gap).

### Viewer (`…/✳️any/👁️viewer/`)

A genuinely independent, minimal, real `LayoutViewer: ArtifactViewer`:
- `Snapshot = LayoutSnapshot`, `Mutation = crate::artifacts::layout::mutations::LayoutMutation` (both
  artifact-level, shared with the editor — decode-only per contract §2.2).
- `Config`/`ConfigMutation` = framework `NoConfig`/`NoConfigMutation`; `Presence`/`Transient` likewise
  `NoPresence`/`NoTransient` — a viewer needs no persisted per-session state to render (fixed default
  camera every render, documented as an intentional simplification, not a bug).
- `Command` = a one-variant `LayoutViewCommand::Noop` deriving `Default` (needed by the canonical
  `assert_viewer_never_mutates::<V>()`'s `V::Command: Default` bound — see w0-f Gap 2); `handle` always
  returns `Ok(ViewEmit::default())`.
- One real window, `👁️preview` (`🎭️modes/👁️view/🪟️windows/👁️preview`), rendering the actual
  `LayoutSnapshot`'s first page through a small, self-contained, PURE render function written directly
  in the viewer's own window file (recipe step 8's option "b" — not relocated into
  `🧬️schema/💡️inferences`, given the time budget, same posture the cad pilot took for its own
  WorldScene helpers). It reuses the artifact-level `resolve_page` (already pure, already outside both
  surfaces) for per-page frame inheritance, and the framework's `build_canvas_2d_scene`/`Canvas2dScene`
  — but does **not** call the sibling editor's `LayoutEngine`/`canvas_layers`. Concretely: real page
  background + real fill/stroke rects for `Frame::Rect` (straight off the document, no synthetic
  data), an outline rect for `Frame::Text` (no glyph layout — the parley/fontique-backed
  `LayoutEngine` stays editor-only; documented simplification, not a bug, mirroring cad's viewer's own
  "default camera/sun, fallback-box mesh" documented gap), and a placeholder tint for `Frame::Image`
  (same placeholder color the editor's own unresolved-link path uses). No chrome (no guides/margins/
  dashed inherited-frame strokes — those are the editor's Blueprint authoring affordances only).
- `create_layout_viewer() -> AppDefinition` via `Viewer::builder(LAYOUT_DIALECT)…build_definition()`.

### `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::layout::*` from `../../🎛️apps/📏️layout/…`) replaced by two
independent regions, built PROGRAMMATICALLY (a Python string-substitution over the existing region
text, never hand-typed) to avoid the emoji-typo trap:
- `//#region ✏️Editor` — `pub mod editor { pub mod layout { … } }`, every leaf `#[path]`-mounted from
  `../../🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…` (same shape the old
  `🎛️apps` region already had, base path swapped).
- `//#region 👁️Viewer` — `pub mod viewer { pub mod layout { … } }`, freshly authored (mirrors only the
  4 real files on disk: root, mode, window), `…/👁️viewer/…`, deliberately never mounting anything
  under `✏️editor/`.

Every `crate::apps::layout::` reference across the moved Rust files became `crate::editor::layout::`
(mechanical `sed`, editor tree only, then spot-checked file-by-file). The `🕸️Wasm` region's
`pub use apps::layout::wasm::LayoutSession;` → `pub use editor::layout::wasm::LayoutSession;`. The
bottom `//#region 📚️Examples` mount for `app_layout_demo_session` was repointed at the new editor path
(name kept, only the `#[path]` string changed). `resolveAll #[path]` attrs verified against disk with
the recipe's Python snippet: **247 total, 0 missing**.

### Plugin root (`✏️s/🔌️plugins/📏️layout/🦀️component.rs`)

`.document_app::<crate::apps::layout::LayoutPlayApp>(create_layout_app())` → two calls:
`.editor::<crate::editor::layout::LayoutPlayApp>(crate::editor::layout::create_layout_app())` and
`.viewer::<crate::viewer::layout::LayoutViewer>(crate::viewer::layout::create_layout_viewer())`. Added
`#[cfg(test)] mod surface_tests` calling the CANONICAL `semio_framework_plugin::testkit::
{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}` directly (no local stand-ins —
w0-f Gap 2 closed these before this packet started).

### `🗿️artifacts/📏️layout/🦀️component.rs`

Added `pub const LAYOUT_DIALECT: Dialect = Dialect { artifact_kind: "s.layout.layout", standard:
StandardId("1"), subset: SubsetId::ANY }` — lives at the ARTIFACT level (not under `editor`/`viewer`)
specifically so a viewer file can read it without ever importing through `editor`. `artifact_kind =
"s.layout.layout"` matches `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`'s own
`#[artifact_schema(id = "s.layout.layout")]`, `standard`/`subset` match this file's own
`🏅️standards/🔖️1/🪆️subsets/✳️any` location — canonical surface id `s.layout.layout@1/*#editor` /
`s.layout.layout@1/*#viewer`. Left the pre-existing local `store::os_io::ArtifactDialect { artifact_kind:
"s.stdio.semio", … }` (used by `background_drawing_child_handle` for STDIO composition) completely
alone — documented in a doc-comment on `LAYOUT_DIALECT` why the two are not the same type/concept.
Fixed the one real (non-comment) `crate::apps::layout::` reference: `.document_codec::<crate::apps::
layout::LayoutPlayApp>()` → `.document_codec::<semio_framework_plugin::EditorApp<crate::editor::layout::
LayoutPlayApp>>()`. Three stray doc-comment references to the old path fixed too (this file, plus
`🚪️io/🦀️component.rs` and `🧬️schema/🧬️mutations/🧾change-data-fields/🦠️mutation/{🦀️component.rs,
🟦️component.ts}` — all within this plugin's own lease).

### Deletion

`✏️s/🔌️plugins/📏️layout/🎛️apps/` removed in full (it was the plugin's only app) once every real file
had a real destination — confirmed hollowed out to only scaffold-era `📌️empty.md` leftovers first.

## Outside-lease referrers (report, not fixed)

- Repo-wide `apps::layout`/`🎛️apps/📏️layout` grep found no real Rust compile dependency outside this
  plugin's own lease — every other hit is either historical ticket JSON/log scratch data (unrelated
  past tickets, harmless) or a stale literal-string entry inside root `📜️script.ts`
  (`"✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎮️commands/✏️author/🦀️component.rs"`, line ~8181) — a
  hardcoded allowlist/fixture array in shared root infrastructure, outside this packet's lease. That
  literal path doesn't even match any file this packet ever saw (no `✏️author` command existed in the
  app this packet migrated), so it was very likely already stale/unreachable before this packet
  started; flagged for the root-script owner, not touched here.
- `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📦️index.ts` — pre-existing, unrelated breakage
  found in passing: all 12 of its `export * as … from "../../🗿️artifacts/📏️layout/🧬️schema/…"` /
  `…/🚪️io/…` / `…/🪓️decomposer/…` paths are missing the `🏅️standards/🔖️1/🪆️subsets/✳️any/` segment
  and none of them resolve on disk (`🧬️schema`/`🚪️io`/`🪓️decomposer` don't exist directly under
  `🗿️artifacts/📏️layout/`). Confirmed via `git log` this predates the ticket (last touched
  2026-08-12, four days before this ticket opened) and is unrelated to the apps/editor/viewer region
  this packet owns — not touched, flagged here for whoever owns the plugin's WASM/TS facade next.

## SDK gaps found

None new. Both gaps `📓️w2-cad-report.md` found (crate-root re-export list, missing testkit helpers)
were already closed by `📓️w0-f-report.md` before this packet started, and this packet used the
canonical bare `semio_framework_plugin::{ArtifactEditor, ArtifactViewer, Editor, Viewer, EditorApp,
ViewerApp, ViewEmit}` imports and `semio_framework_plugin::testkit::{assert_viewer_never_mutates,
assert_editor_and_viewer_share_dialect}` directly — verified by grepping the SDK file fresh (not
trusting the w0-f report's cached line numbers) before relying on it. `Dialect`/`StandardId`/
`SubsetId` are still only reachable via `semio_framework::*` (not `semio_framework_plugin::*`),
confirmed unchanged from w0-f's note — used `use semio_framework::Dialect;` accordingly, no gap.

`EditorBuilder`/`ViewerBuilder` still have no `.example(...)`/`.workflow(...)` methods (contract §2.4,
confirmed by reading the SDK fresh) — the old `.example("sample", …)`/`.workflow("layout", …)` calls
were dropped from `create_layout_app()`, documented inline with a comment pointing at the subset's own
`📚️examples/🎬️demo` facet as the modern replacement, same posture cad's pilot took.

## Verification

- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-layout --all-targets --keep-going`, three runs,
  output for the last one in `🧪️w2-p8-layout-cargo.txt`:
  - Run 1: 4 errors, all inside `semio-framework-plugin`'s own `🔌️plugin/🦀️component.rs`
    (`protocol::AppFrame` missing `messages`/`report` fields, `ArtifactStore::snapshot_with_conflicts`
    missing) — confirmed live-edited: `git status --porcelain` showed it modified, uncommitted, and
    repo-wide `git status` showed a whole concurrent ticket
    (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`) mid-flight across dozens
    of files (including an unrelated plugin, `➗️mathematical`). **0 errors in `📏️layout` files.**
  - Run 2: 3 errors, now inside `semio-framework-os-kernel`'s own `🏪️store/🦀️component.rs`
    (`HistoryLog` missing `edit_messages`, `Vec<Conflict>`/`Vec<HistoryConflict>` mismatch) — same
    peer ticket's churn, moved one crate further upstream exactly as the cad pilot's report and the
    w0-f report both predicted it would. Confirmed via `git status`/`git log` on the newly-failing
    file. **0 errors in `📏️layout` files.**
  - Run 3: 1 error, again inside `semio-framework-plugin`'s own `🔌️plugin/🦀️component.rs`
    (`AppCommand`'s exhaustive `match` now missing arms for `SetMergePolicy`/`ResolveConflict`/
    `ReadConflicts` — the same peer ticket's own new variants, name-matched:
    `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`). **0 errors in `📏️layout`
    files.** `semio-s-plugin-layout` itself was never reached in any of the three runs — its own
    dependency graph never got past `semio-framework-plugin`/`semio-framework-os-kernel`, both
    unrelated, both actively mid-edit by the peer ticket the whole session (`git status --porcelain`
    showed both modified/uncommitted on every check; the failure signature moved from
    `semio-framework-plugin` → `semio-framework-os-kernel` → back to `semio-framework-plugin` across
    the three runs — the peer ticket's own sweep is still landing, not stuck).
- `cargo test -p semio-s-plugin-layout`, output in `🧪️w2-p8-layout-test.txt`: launched after the third
  `cargo check`; did not reach a pass/fail result within this session — `ps aux | grep "^ueli.*cargo "`
  showed **31 concurrent cargo processes** contending on the shared `target/` lock at the time (matches
  memory note `feedback-concurrent-cargo-workspace-churn.md`'s documented 30-90+ minute range for
  repo-wide contention), and the process's own CPU time stayed flat (0:00.59 unchanged across repeated
  checks) — genuinely lock-blocked, not crashed or hung on real work. Same upstream blocker as
  `cargo check`'s three runs, not this lane's own code; untouched (outside this packet's lease —
  `🧰️framework/**`). Whoever next touches this plugin should re-run both commands once the peer
  ticket's sweep and the target-dir contention both clear.

Net: every real compile error seen across all three runs is anchored in `semio-framework-plugin` or
`semio-framework-os-kernel`, never in `📏️layout`'s own files (`grep -B2 -A8 "^error" … | grep -c
"📏️layout"` reads 0 on every run) — confirmed, not assumed. Every manual review this packet could do
in the meantime (every command/panel/window file's `use crate::editor::layout::…` grepped and
spot-read; every `#[path]` attr in `📦️glue.rs` verified against disk; every `App::builder`/
`ArtifactApp`/`VcsArtifactApp<` residue grepped across the whole moved tree; the `include_bytes!` depth
fix verified to resolve on disk; `policyViewerPurityBreaches`'s three trip-wire substrings grepped
under `👁️viewer` and found clean) turned up nothing outstanding in `📏️layout`'s own files. Re-run
`cargo check -p semio-s-plugin-layout --all-targets --keep-going` and `cargo test -p
semio-s-plugin-layout` once the MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS sweep
finishes landing — expected clean based on every run this lane saw (zero errors ever attributed to
this plugin's own files across three full passes at three different upstream blocking points).

## Files touched

Created:
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (70 files
  total — moved content + 3 new real `🟦️component.ts` twins: 2 windows + surface root)
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` (19 files — `🦀️component.rs`/`🟦️component.ts` at surface root, mode root,
  and the `👁️preview` window; taxonomy facet dirs otherwise `📌️empty.md`)

Edited:
- `✏️s/🔌️plugins/📏️layout/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring, `surface_tests`)
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️component.rs` (`LAYOUT_DIALECT`,
  `.document_codec::<EditorApp<…>>()`, doc fixes)
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾change-data-fields/🦠️mutation/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾change-data-fields/🦠️mutation/🟦️component.ts` (doc fix)
- `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions)

Deleted:
- `✏️s/🔌️plugins/📏️layout/🎛️apps/` (whole tree — the plugin's only app)

Scratch (ticket folder): `🧪️w2-p8-layout-cargo.txt`, `🧪️w2-p8-layout-test.txt`.

