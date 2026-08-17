# W2 Packet P8 (dag) — Migration Notes

Lane: W2 packet P8, plugin `🕸️dag`, subset `s.dag.dag@1/*`. Scope: migrate the retired
`✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/` app into `✏️editor`, author a real `👁️viewer`, rewire
`📦️glue.rs`/plugin root/artifact root, delete `🎛️apps`. Followed the pilot's recipe
(`📓️w2-cad-report.md`) precisely, using the CANONICAL testkit functions and bare SDK imports
closed by W0-F (`📓️w0-f-report.md`) instead of the pilot's local stand-ins.

## What landed

### Editor (`…/🪆️subsets/✳️any/✏️editor/`)

The entire old app tree moved across intact: root `🦀️component.rs` (now `impl ArtifactEditor for
DagPlayApp`), `🎚️config` (+schema), `👥️presence` (+schema), `🎮️commands/*` (13 groups —
add-node, remove-node, rename-dag-node, patch-dag-nodes, delete-selection, node-graph-edit,
connect-media-ports, disconnect, move-media-node, reorganize, node-graph-viewport,
graph-pointer-down, set-locale), `📌️panels/*` (3 — document/catalogue/inspection), `🗣️terminology`,
`📚️examples/🎬️demo-session`, and `🎭️modes/✏️edit/{component.rs, 🪟️windows/{🕸️main,🧬️compiled}}`.
`⚙️engine` was an EMPTY directory (0 files, 0 referrers) in the source app — nothing to move, no
depth fix needed. Both windows gained a real `🟦️component.ts` twin (typed `ViewModel` interfaces +
window-kind id/body-key/surface-id constants, mirroring the Rust `render()` boundary) — the
scaffold's single `🪟️main` placeholder was deleted and the two real window dirs (`🕸️main`,
`🧬️compiled`) moved in. The surface root also gained a real `🟦️component.ts` (namespaced
re-export of both window twins — `export * as mainWindow …` / `export * as compiledWindow …`).

`impl ArtifactApp for DagPlayApp` → `impl ArtifactEditor for DagPlayApp`; `const APP_ID` removed;
`const DIALECT: Dialect = crate::artifacts::dag::DAG_DIALECT` added. `create_dag_app()` now
returns `semio_framework_plugin::AppDefinition` (`Editor::builder(DAG_DIALECT)…build_definition()`)
instead of `App`; the trailing `.example_source(crate::examples::art_dag_demo::source())` /
`.workflow("dag", "DAG", "graph")` calls were **dropped, not ported** (documented inline at the
`.build_definition()` call site — `EditorBuilder` has no such methods, contract §2.4/W0-F gap 4).
The `manifest_includes_the_demo_example` test (which asserted `create_dag_app().examples`
contained `"demo"`) was removed with an inline doc comment explaining why, rather than left to
fail; the other manifest-sanity tests were updated from `create_dag_app().definition` to
`create_dag_app()` directly (the return type itself changed).

Testkit region: `VcsArtifactApp<DagPlayApp>` → `VcsArtifactApp<EditorApp<DagPlayApp>>`; added a
`dag_app_manifest_for_testkit() -> App { App { definition: create_dag_app(), examples: Vec::new() } }`
wrapper so `testkit::new_app_with_registry::<EditorApp<DagPlayApp>>(dag_app_manifest_for_testkit)`
still gets the `fn() -> App` shape that testkit fn's un-updated signature (W0-F gap 3, still open)
requires. `new_app()` now instantiates `EditorApp<DagPlayApp>`.

Imports: `use semio_framework_plugin::{ArtifactEditor, Editor, …}` bare (W0-F gap 1 closed this);
`Dialect`/`InteractionView` still need the `semio_framework_plugin::app::` prefix — confirmed by
grepping the crate-root `pub use app::{ … };` re-export block fresh (only `ArtifactEditor`,
`ArtifactViewer`, `Editor`, `EditorApp`, `ViewEmit`, `Viewer`, `ViewerApp` were added by W0-F;
`Dialect`/`StandardId`/`SubsetId` were deliberately left alone per that report).

### Viewer (`…/✳️any/👁️viewer/`)

A genuinely independent, minimal, real `DagViewer: ArtifactViewer`:
- `Snapshot = DagSnapshot`, `Mutation = crate::artifacts::dag::op::DagMutation` (both
  artifact-level, shared with the editor — decode-only per contract §2.2).
- `Config`/`ConfigMutation`/`Presence`/`PresenceMutation`/`Transient`/`TransientMutation` = the
  framework's `NoConfig`/`NoConfigMutation`/`NoPresence`/`NoPresenceMutation`/`NoTransient`/
  `NoTransientMutation` — a DAG viewer needs no persisted per-session state to render a read-only
  node graph.
- `Command = DagViewCommand::Noop` (one variant, `#[derive(Default)]`); `handle` always returns
  `Ok(ViewEmit::default())`.
- One real window, `🕸️main` (`🎭️modes/👁️view/🪟️windows/🕸️main`, renamed from the scaffold's
  `🪟️main` placeholder), rendering the actual `DagSnapshot` through the SAME artifact-level pure
  `document_to_workflow` inference the editor's own main window uses, composed with the
  framework's `build_node_graph_scene`/`NodeGraphScene::base` helpers — **not** by calling into
  the editor. `editable: Some(false)` is the one field distinguishing this render from the
  editor's. No `TreeWindowKit` fit: a DAG is a general node/edge graph, not a natural tree
  projection (a node can have multiple parents/children with no single hierarchy), so a
  self-contained pure render function in the viewer's own window file (per contract's own
  fallback guidance) was the right call, matching the pilot's identical reasoning for cad's shape
  window.
- `create_dag_viewer() -> AppDefinition` via `Viewer::builder(DAG_DIALECT)…build_definition()`.

Used the CANONICAL W0-F testkit surface where the recipe calls for it: `create_dag_viewer`'s own
tests assert `def.role == AppRole::Viewer` and `def.dialect == DAG_DIALECT.into()` directly (no
local stand-in needed since the dag viewer's own unit tests exercise this inline); the plugin
root's cross-surface assertions (see below) use `semio_framework_plugin::testkit::
{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}` — the real functions, not
local copies, since W0-F closed gap 2.

Viewer purity confirmed by direct grep of the whole `👁️viewer` tree: zero occurrences of the
literal substring `::editor::`, `.mutation(`, `Emit::mutations`, or `artifact_mutations`.

### `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::dag::*` from `../../🎛️apps/🕸️dag/…`) replaced by two
independent regions:
- `//#region ✏️Editor` — `pub mod editor { pub mod dag { … } }`, every leaf `#[path]`-mounted from
  `../../🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
- `//#region 👁️Viewer` — `pub mod viewer { pub mod dag { … } }`, same base but `…/👁️viewer/…`,
  deliberately never mounting anything under `✏️editor/`.

Every `crate::apps::dag::` reference across the moved Rust files became `crate::editor::dag::`
(mechanical substitution, editor tree only — 57 lines across the tree referenced it). The bottom
`//#region 📚️Examples` mount for `app_dag_demo_session` was repointed at the new editor path (name
kept, only the `#[path]` string changed). All 189 `#[path]` attributes in the rewired glue.rs
verified against disk with the recipe's own Python snippet — 0 missing, twice (once right after
the substitution, once again before declaring done).

### Plugin root (`✏️s/🔌️plugins/🕸️dag/🦀️component.rs`)

`.document_app::<crate::apps::dag::DagPlayApp>(crate::apps::dag::create_dag_app())` → two calls:
`.editor::<crate::editor::dag::DagPlayApp>(crate::editor::dag::create_dag_app())` and
`.viewer::<crate::viewer::dag::DagViewer>(crate::viewer::dag::create_dag_viewer())`. Added
`#[cfg(test)] mod surface_tests` calling the CANONICAL `semio_framework_plugin::testkit::
{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}` directly (W0-F gap 2 closed
— no local stand-ins needed, unlike the pilot).

### `🗿️artifacts/🕸️dag/🦀️component.rs`

Added `pub const DAG_DIALECT: semio_framework_plugin::app::Dialect = Dialect { artifact_kind:
"s.dag.dag", standard: StandardId("1"), subset: SubsetId::ANY }` — lives at the ARTIFACT level
(not under `editor`/`viewer`) specifically so the viewer file can read it without ever importing
through `editor`. `artifact_kind = "s.dag.dag"` matches `#[artifact_schema(id = "s.dag.dag")]` on
`DagArtifact` (confirmed by reading the schema file directly), `standard`/`subset` match this
file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — canonical surface id
`s.dag.dag@1/*#editor` / `s.dag.dag@1/*#viewer`, exactly the contract §1 grammar. Fixed the one
real (non-comment) `crate::apps::dag::` reference: `.document_codec::<crate::apps::dag::
DagPlayApp>()` → `.document_codec::<semio_framework_plugin::EditorApp<crate::editor::dag::
DagPlayApp>>()`. Two stray doc-comment references to the old path fixed too (cosmetic).

Note: a pre-existing, UNRELATED `DagAnalyzerAnalysis: ArtifactAnalysis` impl in the subset's own
`🧬️schema/🦀️component.rs` declares its own `const DIALECT: Dialect = Dialect { artifact_kind:
"s.dag", … }` (no trailing `.dag`, a different, narrower analysis-only dialect) — left untouched,
not this ticket's concern, not a naming collision with `DAG_DIALECT`.

### `📦️packages/🦀️rust/Cargo.toml`, `📦️packages/🟦️typescript/`

No `sourceRoots`/`app = "dag-play"` metadata lines exist in dag's `Cargo.toml` (unlike cad's
pilot, which had two) — nothing to repoint there. dag's `📦️index.ts`/`tsconfig`-equivalent
(`📋️project.json`) reference only artifact-level facets (`schema`, `snapshot`, `diff`,
`mutations`, `io`, `decomposer`) via relative paths that predate the subset/standard taxonomy
depth (e.g. `../../🗿️artifacts/🕸️dag/🧬️schema/🟦️component.ts` instead of `…/🏅️standards/🔖️1/
🪆️subsets/✳️any/🧬️schema/🟦️component.ts`) — this is PRE-EXISTING drift unrelated to the app/editor
migration (no `apps`/`🎛️apps` string anywhere in that file), outside this packet's scope; flagged
for whoever owns that facade next, not fixed here.

### Deletion

`✏️s/🔌️plugins/🕸️dag/🎛️apps/` removed in full (it was the plugin's only app) once every real file
had a real destination and glue.rs no longer referenced it.

## Outside-lease referrers (report, not fixed)

Repo-wide grep (via a dedicated Explore pass) for `apps::dag`, the literal old path string
`🎛️apps/🕸️dag`, and `DAG_PLAY_APP_ID`/`"dag-play"` used as a config value, found:

- **No real Rust compile dependency anywhere outside `✏️s/🔌️plugins/🕸️dag/`.** No other plugin
  imports `dag::apps::dag::…`, and no `Cargo.toml` anywhere in the repo references the old
  hand-written `"dag-play"` app id.
- One cosmetic, non-compiling item: `/Users/ueli/Documents/semio/script.ts:8305-8306` —
  `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` (a `Set<string>` of `.rs` paths a repo-tooling policy
  reads) carries two now-dead entries pointing at
  `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🎮️commands/🔧️nodes/🦀️component.rs` and `…/🕸️graph/🦀️component.rs`
  — data, not code; the lint that consumes it only iterates files it finds on disk, so these two
  become silent dead weight, not a breakage. `🧰️framework/**`/repo-root `script.ts` are both
  outside this packet's lease (`✏️s/🔌️plugins/🕸️dag/**` only) — reported, not pruned.

## SDK gaps hit (already closed by W0-F, used directly)

None new. Confirmed via fresh grep of the crate-root `pub use app::{ … };` block before writing
any import that `ArtifactEditor`, `ArtifactViewer`, `Editor`, `EditorApp`, `ViewEmit`, `Viewer`,
`ViewerApp` are all present (W0-F gap 1) and used bare throughout; `testkit::
assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect`/`new_viewer` (W0-F gap 2)
are the CANONICAL versions this packet's plugin-root test module calls, no local stand-ins
written. The one remaining open gap this packet still had to route around (W0-F gap 3, not yet
closed): `testkit::new_app_with_registry`/`assert_declared_actions_bridge_to_commands` still take
`fn() -> App`, not `fn() -> AppDefinition` — a tiny local `dag_app_manifest_for_testkit()` wrapper
(same pattern the pilot and W0-F's own report both used) bridges this, documented inline as a
still-open framework gap, not a dag-specific workaround.

## Verification

### `cargo check -p semio-s-plugin-dag --all-targets --keep-going`

Five consecutive runs across this session (workspace-shared `target/` lock contention with other
live sessions made several of these take well over two minutes), final one's full output in
`🧪️w2-p8-dag-cargo.txt`:

- Run 1: 4 errors, all inside `semio-framework-plugin`'s own SDK file
  (`E0063`×2 missing struct fields on `AppFrame`, `E0599` no `snapshot_with_conflicts` method) —
  confirmed via `git status --porcelain` that `🔌️plugin/🦀️component.rs` is currently modified
  (uncommitted) by a live peer session. **0 errors anchored in `🕸️dag` files** (confirmed:
  `grep -B2 -A5 "^error" | grep -c "🔌️plugins/🕸️dag"` = 0; the one line containing the string
  `🕸️dag` in the whole log is an unrelated `infinite_canvas` board-port kernel type path —
  `♾️infinite/…/🎲️board/🔌️ports/➡️directed/🕸️dag/…` — a different "dag" namespace entirely, and only
  a warning, not an error).
- Run 2 (after a ~90s wait, same file still uncommitted/modified): identical 3 errors, same file —
  no progress from that peer session yet in the interval.
- Run 3: the failure moved upstream to `semio-framework-os-kernel`'s own `🏪️store/🦀️component.rs`
  (`E0609` missing `edit_messages` field on `HistoryLog`, `E0308` `Conflict`/`HistoryConflict` type
  mismatch) — confirmed via `git status`/`git log --date=iso` that this file is ALSO currently
  uncommitted, and the error shape (`HistoryLog` field renames, `Conflict` vs `HistoryConflict`)
  matches the still-in-flight `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`
  ticket W0-F's own report named as concurrently touching this exact area.
- Run 4: still `semio-framework-os-kernel`, now a single borrow-checker error (`E0382`) in the same
  file area — confirmed still live-edited, the peer session visibly still iterating.
- Run 5 (the one saved to `🧪️w2-p8-dag-cargo.txt`): back to `semio-framework-plugin`, now
  `error[E0004]: non-exhaustive patterns: AppCommand::SetMergePolicy { .. },
  AppCommand::ResolveConflict { .. } and AppCommand::ReadConflicts { .. } not covered` — new
  `AppCommand` variants mid-addition by the same in-flight ticket, propagating through a `match` in
  the SDK file this packet's own `EditorApp`/`ViewerApp`/testkit imports depend on.
- **Net across all five runs: zero errors ever anchored in `🕸️dag`'s own files**
  (`grep -c "semio-s-plugin-dag" 🧪️w2-p8-dag-cargo.txt` = 0 in the final run too). The crate cannot
  finish a full `cargo check` right now purely because of unrelated, actively in-flight framework
  work upstream of it in the dependency graph (bouncing between `semio-framework-plugin` and
  `semio-framework-os-kernel` across runs — the same two crates the pilot's own report predicted
  would be hit). Re-run once that lands.

### `cargo test -p semio-s-plugin-dag`

Same blocker, one more manifestation of the same live churn: this run's `semio-framework-plugin`
build failed on `error[E0004]: non-exhaustive patterns: AppCommand::SetMergePolicy { .. },
AppCommand::ResolveConflict { .. } and AppCommand::ReadConflicts { .. } not covered` — new
`AppCommand` variants mid-addition by the same in-flight conflict/merge-policy ticket. Full output
in `🧪️w2-p8-dag-test.txt`. **0 errors anchored in `🕸️dag` files** (confirmed the same way as the
cargo check runs). Not a dag-specific failure — the crate never gets to compile at all because its
own dependency `semio-framework-plugin` fails first, upstream of anything this packet wrote.

## Files touched

Created:
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (58 files —
  moved content + 3 new real `🟦️component.ts` twins: 2 windows + surface root)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` (19 files —
  `🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the `🕸️main` window; other
  taxonomy facet dirs otherwise `📌️empty.md`)

Edited:
- `✏️s/🔌️plugins/🕸️dag/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring, `surface_tests`)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️component.rs` (`DAG_DIALECT`, `.document_codec::<EditorApp<…>>()`, doc fixes)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions, examples repoint)

Deleted:
- `✏️s/🔌️plugins/🕸️dag/🎛️apps/` (whole tree — the plugin's only app)

Scratch (ticket folder): `🧪️w2-p8-dag-cargo.txt`, `🧪️w2-p8-dag-test.txt`.
