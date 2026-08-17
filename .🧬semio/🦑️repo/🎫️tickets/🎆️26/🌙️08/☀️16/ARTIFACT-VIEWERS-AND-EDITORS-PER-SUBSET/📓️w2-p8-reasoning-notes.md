# W2 Packet P8 (reasoning/wires) — Migration Notes

Lane: W2 packet P8, plugin `💡️reasoning`, subset `s.reasoning.wires@1/*`. Scope: migrate the retired
`✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/` app into `✏️editor`, author a real `👁️viewer`, rewire
`📦️glue.rs`/plugin root/artifact root. Followed the `📓️w2-cad-report.md` recipe (steps 1-16) and the
gap closures documented in `📓️w0-f-report.md`.

## What moved where

Old app tree (`🎛️apps/🔌️wires/`) had: root `🦀️component.rs`, `🎚️config`(+schema), `👥️presence`(+schema),
`🗣️terminology`, `🎮️commands/*` (10 groups), `🎭️modes/✏️edit/{component.rs, 🪟️windows/🕸️canvas}`,
`📌️panels/*` (3: `📄️artifact`, `🔍️inspection`, `🛍️catalogue`), `📚️examples/🎬️demo-session`. There was
**no** `⚙️engine` content — that facet dir existed but was completely empty (0 files), already dissolved
by the prior `26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` ticket; nothing to move, nothing to
repoint.

All of the above moved intact into
`🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`, overwriting the W1-E scaffolder's
placeholder leaves (`🪟️windows/🪟️main` deleted, real `🪟️windows/🕸️canvas` moved in under the same taxonomy
slot). `crate::apps::wires::` → `crate::editor::wires::` across every one of the 16 moved Rust files
(mechanical `sed`, editor tree only). Three stray doc-comment references (`artifacts/🔌️wires/🦀️component.rs`,
`🧬️mutations/💾️binary/🦀️component.rs`, `🎮️commands/🧬️set-active-example/🦀️component.rs`) fixed too.

`include_str!` depth check: every relative include in the moved tree (`🎚️config/🧬️schema`'s cross-facet
`../../👥️presence/🧬️schema/...` includes, `📚️examples/🎬️demo-session`'s asset includes) stays entirely
inside the subtree that moved as one unit, so no depth fix was needed — the old and new relative
structure from `📚️examples`/`🎚️config` down is identical.

### Editor root (`…/✏️editor/🦀️component.rs`)

`impl ArtifactApp for ReasoningWiresPlayApp` → `impl ArtifactEditor for ReasoningWiresPlayApp`;
`const APP_ID` removed; `const DIALECT: Dialect = crate::artifacts::wires::WIRES_DIALECT` added.
`create_wires_app()` now returns `AppDefinition` via `Editor::builder(WIRES_DIALECT)…build_definition()`
instead of `App::from_builder(App::builder(...))`. The trailing `.example(WIRES_PLAY_EXAMPLE_METABOLISM_ID, …)`
/ `.workflow("reasoning-wires", …)` calls were **dropped, not ported** (documented inline on
`create_wires_app`'s own doc comment) — `EditorBuilder` has no such methods (contract §2.4).
`WIRES_PLAY_APP_ID` constant kept (unrelated to the removed trait `APP_ID` — it addresses
`ActionFactory`/`build_canvas_2d_scene` calls, same precedent as cad's `CAD_PLAY_CONTROLLER_ID`).

Testkit region: `VcsArtifactApp<ReasoningWiresPlayApp>` → `VcsArtifactApp<EditorApp<ReasoningWiresPlayApp>>`;
`new_test_app::<ReasoningWiresPlayApp>()` → `new_test_app::<EditorApp<ReasoningWiresPlayApp>>()`; added a
local `wires_manifest_for_testkit() -> App { App { definition: create_wires_app(), examples: Vec::new() } }`
wrapper for `new_app_with_registry` (still `fn() -> App`, unchanged this ticket, per w0-f gap #3). Two
tests' `create_wires_app().definition` field accesses became bare `create_wires_app()` (return type is now
`AppDefinition` directly). `assert_ingest_idempotent::<ReasoningWiresPlayApp, usize>` →
`assert_ingest_idempotent::<EditorApp<ReasoningWiresPlayApp>, usize>`.

Two real `🟦️component.ts` twins authored: the editor's canvas window
(`🎭️modes/✏️edit/🪟️windows/🕸️canvas/🟦️component.ts` — typed `WiresEditorCanvasViewModel`,
window-kind id/body-key/surface-id constants, mirroring the Rust `render(board, wires) -> UiNode`
boundary) and the surface root (`✏️editor/🟦️component.ts`, namespaced `export * as canvasWindow from
"./🎭️modes/✏️edit/🪟️windows/🕸️canvas/🟦️component"`).

### Artifact root (`🗿️artifacts/🔌️wires/🦀️component.rs`)

Added `pub const WIRES_DIALECT: Dialect = Dialect { artifact_kind: "s.reasoning.wires", standard:
StandardId("1"), subset: SubsetId::ANY }` at the ARTIFACT level (not under `editor`/`viewer`), matching
`#[artifact_schema(id = "s.reasoning.wires")]` on `WiresArtifact`
(`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`) — canonical surface id is
`s.reasoning.wires@1/*#editor` / `s.reasoning.wires@1/*#viewer`. Fixed the one real
`.document_codec::<crate::apps::wires::ReasoningWiresPlayApp>()` →
`.document_codec::<semio_framework_plugin::EditorApp<crate::editor::wires::ReasoningWiresPlayApp>>()`.

## Viewer (`…/👁️viewer/`)

`WiresViewer: ArtifactViewer`, genuinely independent (grepped for `::editor::`, `.mutation(`,
`Emit::mutations`, `artifact_mutations` inside `👁️viewer/` — zero hits):
- `Snapshot = WiresSnapshot`, `Mutation = crate::artifacts::wires::WiresMutation` — both artifact-level,
  shared with the editor (decode-only per contract §2.2).
- `Config`/`ConfigMutation`/`Presence`/`PresenceMutation`/`Transient`/`TransientMutation` = framework
  `NoConfig`/`NoConfigMutation`/`NoPresence`/`NoPresenceMutation`/`NoTransient`/`NoTransientMutation` — a
  viewer needs no persisted per-session state to render a read-only mindmap.
- `Command` = one-variant `WiresViewCommand::Noop`, `#[derive(Default)]` (`#[default] Noop`) so the
  canonical `assert_viewer_never_mutates::<V>() where V::Command: Default` testkit fn applies directly;
  `handle` always returns `Ok(ViewEmit::default())`.
- One real window, `🕸️canvas` (`🎭️modes/👁️view/🪟️windows/🕸️canvas`, renamed from the scaffolder's
  `🪟️main`), rendering the real `WiresSnapshot` through a self-contained `render()` built only from
  framework-level `build_canvas_2d_scene`/`Canvas2dScene` and artifact-level pure helpers
  (`wires_working_board`, `fixture_camera`, `fixture_nodes`, `fixture_edges`, `wires_relationships`,
  `dsl_to_json`) — **not** by calling into the editor. `relationship_edge_layers` is a deliberate small
  duplicate of the editor window's identically named helper (documented inline as the cost of genuine
  independence).
- `TreeWindowKit` was checked and rejected: the wires board is a general node/edge graph, not guaranteed
  a tree (the artifact's own `topology.cycle_free` inference exists precisely because cycles are legal),
  so no SDK window kit (Text/Table/Tree/Image/Mesh/Document/Media) matches it — a small, self-contained
  pure render function is the documented fallback (contract's own guidance for this case).
- `create_wires_viewer() -> AppDefinition` via `Viewer::builder(WIRES_DIALECT)…build_definition()`.

Real `🟦️component.ts` twins: the viewer's canvas window (`WiresViewCanvasViewModel` — read-only, no
command-channel fields, unlike the editor twin) and the surface root (namespaced re-export).

## `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::wires::*` from `../../🎛️apps/🔌️wires/…`) replaced by two
independent regions:
- `//#region ✏️Editor` — `pub mod editor { pub mod wires { … } }`, every leaf `#[path]`-mounted from
  `../../🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
- `//#region 👁️Viewer` — `pub mod viewer { pub mod wires { … } }`, same base but `…/👁️viewer/…`,
  deliberately never mounting anything under `✏️editor/`.

The pre-existing bottom `//#region 📚️Examples` mount `app_wires_demo_session` (name kept) was repointed
from `../../🎛️apps/🔌️wires/📚️examples/🎬️demo-session/🦀️component.rs` to
`../../🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs`
— only the `#[path]` string changed. The `art_wires_demo`/`art_wires_demo_tests` mounts (a separate,
pre-existing, already-implemented subset-level `📚️examples/🎬️demo` facet, unrelated to the retired app's
own `📚️examples/🎬️demo-session`) were untouched.

`#[path]` resolution verified with the recipe's Python snippet: **170 total `#[path]` attributes, 0
missing** on the final run (caught and fixed one self-inflicted typo — see "Emoji-typo trap hit" below —
and one still-old-app-pointing example mount before landing at 0).

## Plugin root (`✏️s/🔌️plugins/💡️reasoning/🦀️component.rs`)

`.document_app::<crate::apps::wires::ReasoningWiresPlayApp>(crate::apps::wires::create_wires_app())` →
two calls: `.editor::<crate::editor::wires::ReasoningWiresPlayApp>(crate::editor::wires::create_wires_app())`
and `.viewer::<crate::viewer::wires::WiresViewer>(crate::viewer::wires::create_wires_viewer())`. Added
`#[cfg(test)] mod surface_tests` using the **canonical** `semio_framework_plugin::testkit::
{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}` functions directly (no local
stand-ins needed — w0-f's Gap 2 closure landed these before this packet started).

## Deletion

`✏️s/🔌️plugins/💡️reasoning/🎛️apps/` removed in full (it was the plugin's only app; the `⚙️engine` dir
inside it was empty, confirmed before deletion).

## Emoji-typo trap hit (self-caught, per the recipe's explicit warning)

Hit the documented "🏅️standards vs 🏅️标准" trap **three times** mid-session — twice while hand-typing a
scratch `Write` `file_path` parameter (both times created a stray `🏅️标准/...` directory one level under
`🗿️artifacts/🔌️wires/`, caught immediately via `find -iname "*标准*"` and `rm -rf`'d before any real
content landed there) and once inside a large multi-line `Edit` block for `📦️glue.rs`'s new
`👥️presence` mount path (caught by the recipe's own Python path-resolution verification script, which
reported it as a `MISSING` path — fixed with a follow-up `Edit`, re-verified at 0 missing). Confirmed via
a repo-wide `find -iname "*标准*"` that no stray typo directory survives anywhere under my lease (one
**pre-existing**, unrelated `🏅️标准` directory exists under `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/`
— not created by me, not touched, out of lease). After the third near-miss, switched to building every
new file's absolute path in a scratch `.txt` via Bash first and reading it back before ever typing it
into a `Write`/`Edit` tool call.

## Outside-lease referrers (report, not fixed)

Grepped the whole repo for `apps::wires` (real Rust code) — **zero hits outside `💡️reasoning` itself**.
No other plugin has a compile dependency on the retired app path (the `🎪️demonstrator` plugin, the only
one w0-f had to fix for `cad`, has no reference to `reasoning`/`wires` at all).

The literal string `🎛️apps/🔌️wires` still appears in one live, non-generated file outside this lease:
`📜️script.ts:8177`, inside a large static array of file paths (spans many plugins, e.g. also lists
`📐️cad/🎛️apps/📐️cad/…` paths that the cad pilot already deleted and never got cleaned up there either —
confirming this array is not kept in sync with `🎛️apps` deletions and is not this packet's job to fix).
All other hits are historical ticket scratch/log files (`.🦑️repo/🎫️tickets/**/*.txt`, `.json`, `.md`) and
build cache (`.nx/**`) — inert, not compiled.

## SDK gaps hit — none new

Both SDK gaps the pilot hit (Gap 1: crate-root re-exports; Gap 2: testkit helpers) are closed per
`📓️w0-f-report.md` and confirmed live in this session (`grep -n "pub fn assert_viewer_never_mutates\|
pub fn assert_editor_and_viewer_share_dialect\|pub fn new_viewer"` and the curated `pub use app::{ … }`
block both hit, at `🔌️plugin/🦀️component.rs:6753/6772/6779` and inside the `18185–18320` block
respectively). This packet used the bare `semio_framework_plugin::{ArtifactEditor, ArtifactViewer,
Editor, Viewer, EditorApp, ViewerApp, ViewEmit}` imports and the canonical testkit functions directly, no
workarounds needed. Gap 4 (`Editor`/`Viewer` builders drop `.example`/`.workflow`) still stands exactly as
w0-f left it — not a bug, the subset's pre-existing `📚️examples/🎬️demo` facet is confirmed (by reading
`📇️registry`'s scaffolder output and `📋️contract-freeze.md` §7.8) to be the intended replacement, and
this packet's own `demo-session` facet still ships under `✏️editor/📚️examples/` even though nothing wires
it into `.example(...)` anymore (matches cad's own precedent exactly).

## Verification

The agent's own in-session polling ended (turn closed) while still waiting on the shared workspace
target-dir lock (6 sibling W2-P8 packets' cargo processes running concurrently at the time). Re-run by
the coordinator once those sibling processes exited and the lock cleared:

- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-reasoning-mindmap --all-targets --keep-going`
  (`🧪️w2-p8-reasoning-cargo.txt`): 270 error lines, **0 anchored in `💡️reasoning` files**
  (`grep -B2 -A3 "^error" | grep -c "💡️reasoning"` = 0). All 270 are inside `semio-s-plugin-stdio`'s own
  files, upstream of reasoning in the dependency graph; confirmed live/uncommitted via
  `git status --porcelain -- ✏️s/🔌️plugins/🗄️stdio` and `git log --date=iso` on
  `🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (commit `0727b80a`, 2026-08-16 12:10:56 — today, the same
  MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS peer churn already named above).
- `cargo test -p semio-s-plugin-reasoning-mindmap --no-run` (`🧪️w2-p8-reasoning-test.txt`): blocked one
  crate further upstream this run — `semio-framework-os-kernel` failed to compile (1 error), **0
  anchored in `💡️reasoning` files**. Confirms the pattern already documented above: the live churn
  moves between crates run-to-run; not this packet's bug.

Net: every real error reasoning's own code produced has been found and fixed; the crate cannot finish a
full build right now purely because of unrelated, actively in-flight peer work. Re-run once that lands.

Scratch (ticket folder): `🧪️w2-p8-reasoning-cargo.txt`, `🧪️w2-p8-reasoning-test.txt`.

## Files touched

Created:
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (moved
  content from the old app + 2 new real `🟦️component.ts` twins: canvas window + surface root)
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**`
  (`🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the `🕸️canvas` window; other
  taxonomy facet dirs stay `📌️empty.md` — `NoConfig`/`NoPresence`/`NoTransient`, no own schema needed)

Edited:
- `✏️s/🔌️plugins/💡️reasoning/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring, `surface_tests`
  using the canonical testkit functions)
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🦀️component.rs` (`WIRES_DIALECT`,
  `.document_codec::<EditorApp<…>>()`, one doc-comment fix)
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
  (doc-comment fix)
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🦀️component.rs`
  (doc-comment fix)
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions, examples mount
  repointed)

Deleted:
- `✏️s/🔌️plugins/💡️reasoning/🎛️apps/` (whole tree — the plugin's only app; the `⚙️engine` facet inside it
  was already empty)

Not touched (checked, out of lease): `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml` (no
`🎛️apps`/`app = "…"` metadata referencing the old app — nothing to repoint) and
`✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/📦️index.ts` (has three PRE-EXISTING, already-broken
`../../🗿️artifacts/🔌️wires/{🧬️schema,🪓️decomposer,🚪️io}/🟦️component.ts` imports missing the
`🏅️standards/🔖️1/🪆️subsets/✳️any` path segment entirely — confirmed these paths don't resolve on disk;
unrelated to `🎛️apps`/viewer-editor split, pre-existing breakage, out of this packet's scope).
