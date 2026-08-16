# W2 Packet P8 (imperative) — Notes

Lane: W2 packet P8, plugin `📜️imperative`, subset `s.imperative.imperative@1/*`. Scope (own lease
only): `✏️s/🔌️plugins/📜️imperative/**`. Followed `📓️w2-cad-report.md`'s "Migration recipe" (steps
1-16) and closed SDK gaps confirmed by `📓️w0-f-report.md`.

## What moved where

The plugin had exactly one app, `🎛️apps/📜️imperative/`, with a `🎭️modes/✏️edit/🪟️windows/{📋️main,
📝️script}` shape (2 windows) plus app-root `⚙️engine`, `📌️panels`, `🗣️terminology`, `🎚️config`,
`👥️presence`, `📚️examples`, `🌉️wasm`, `🎮️commands`. All of it moved intact into
`🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`, overwriting the W1-E
scaffold's disposable placeholder leaves first (per recipe step 3): root `🦀️component.rs`, `🎚️config`
(+schema), `👥️presence` (+schema), `🗣️terminology`, `🌉️wasm`, `📚️examples/🎬️demo-session`,
`📌️panels/*` (3: document/catalogue/inspection), `🎮️commands/*` (11 groups), and the mode subtree
`🎭️modes/✏️edit/{component.rs, 🪟️windows/{📋️main,📝️script}}`.

`⚙️engine` (not a `surfaceChildDirs` member) was checked for referrers first (recipe step 4): only
editor-side files reference `apps::imperative::engine` (root component, `🌉️wasm`, the `📝️script`
window, `👁️run`/`👁️set-locale` commands) — nothing in `🧬️schema`/`💡️inferences` depends on it — so it
moved whole into `✏️editor/⚙️engine/`, the simple case.

Each of the two windows (`📋️main`, `📝️script`) gained a real `🟦️component.ts` twin (typed
`ViewModel` interfaces + window-kind id/body-key/surface-id constants, mirroring the Rust `render()`
boundary — neither window had one before this packet). The surface root also gained a real
`🟦️component.ts` (namespaced re-export of both window twins, `export * as mainWindow from …` /
`export * as scriptWindow from …`).

`include_str!` audit (recipe step 5): every hit inside the moved tree references a sibling that moved
with it (`🎬️demo-session`'s `🖼️assets/…`, `🎚️config/🧬️schema`'s own five leaves plus a relative
`../../👥️presence/🧬️schema/…` reach) — no cross-boundary path, no depth fix needed.

`impl ArtifactApp for ImperativePlayApp` → `impl ArtifactEditor for ImperativePlayApp`; `const APP_ID`
removed; `const DIALECT: Dialect = crate::artifacts::imperative::IMPERATIVE_DIALECT` added.
`create_imperative_app()` now returns `AppDefinition` (`Editor::builder(IMPERATIVE_DIALECT)…
.build_definition()`); the trailing `.example_source(art_imperative_demo::source())` /
`.workflow("imperative", "Imperative", "graph")` calls were **dropped, not ported** (same SDK gap #4
the cad pilot hit — `EditorBuilder` has no such methods, `PluginBuilder::editor::<E>` only takes the
bare `AppDefinition`) — documented inline at the drop site, not silently lost.

## The `IMPERATIVE_DIALECT` constant

`pub const IMPERATIVE_DIALECT: semio_framework_plugin::app::Dialect = Dialect { artifact_kind:
"s.imperative.imperative", standard: StandardId("1"), subset: SubsetId::ANY };` — added at the
**artifact** level in `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🦀️component.rs`, not
under `editor`/`viewer`, so the viewer file can read it without ever importing through the sibling
editor module. `artifact_kind = "s.imperative.imperative"` matches the schema's own
`#[artifact_schema(id = "s.imperative.imperative")]` (`🧬️schema/🦀️component.rs:11`); `standard`/
`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — canonical surface ids
`s.imperative.imperative@1/*#editor` / `s.imperative.imperative@1/*#viewer`.

Pre-existing, unrelated wrinkle noticed but NOT touched (different constant, different purpose):
`derived_analysis::ImperativeAnalyzerAnalysis::DIALECT` in `🧬️schema/🦀️component.rs:176` uses
`artifact_kind: "s.imperative"` (no trailing `.imperative`) and `subset: SubsetId("*")` instead of
`SubsetId::ANY` — an `ArtifactAnalysis` trait dialect for source-sniffing, pre-dates this packet, out
of scope to "fix" (not a surface id, no `surface_app_id` grammar requirement applies to it).

Fixed the one real (non-comment) `crate::apps::imperative::` reference left in the artifact's own
`declaration()`: `.document_codec::<crate::apps::imperative::ImperativePlayApp>()` →
`.document_codec::<EditorApp<crate::editor::imperative::ImperativePlayApp>>()` (the runtime
`ArtifactApp` bound needs the SDK adapter). Two stray doc-comment references to the old path fixed too
(artifact root + the `🧬️mutations/💾️binary` facet's module doc, cosmetic).

## Viewer design

Genuinely independent `ImperativeViewer: ArtifactViewer` in `👁️viewer/🦀️component.rs` — never
imports through the sibling editor module (self-verified: `grep -rn "::editor::\|\.mutation(\|
artifact_mutations\|Emit::mutations"` over the whole `👁️viewer` tree returns nothing).

- `Snapshot = ImperativeSnapshot`, `Mutation = crate::artifacts::imperative::mutations::
  ImperativeMutation` — both artifact-level, shared with the editor (decode-only per contract §2.2).
- `Config`/`ConfigMutation` = `NoConfig`/`NoConfigMutation`; `Presence`/`Transient` likewise
  `NoPresence`/`NoTransient` — a viewer needs no persisted per-session state to render two read-only
  windows.
- `Command` = one-variant `ImperativeViewCommand::Noop` (`#[derive(Default)]`, needed by the
  canonical `assert_viewer_never_mutates::<V>() where V::Command: Default` — see below); `handle`
  always returns `Ok(ViewEmit::default())`.
- **Two real windows**, both built from contract §2.6 window kits rather than hand-rolled scenes:
  - `🎭️modes/👁️view/🪟️windows/📋️main` — `TableWindowKit` (kind id `framework.window.table`), one
    row per top-level step (`index`, `id`, `kind`), English-only headers (no `Config`, so no locale to
    read localized column labels from — an intentional simplification, matching the editor's own
    `col_index`/`col_id`/`col_kind` label *content* but not its localization axis).
  - `🎭️modes/👁️view/🪟️windows/📝️script` — `TextWindowKit` (kind id `framework.window.text`),
    `read_only: true`, text from `imperative_engine::compile_to_text(&path)` — the SAME shared-kernel
    free function the editor's `ImperativeHost::compile_text` wrapper calls, called directly here
    since `compile_to_text` is pure (`&Path -> String`, no app/editor state); the editor's
    `ImperativeHost` wrapper adds no logic a read-only render needs, it just owns `&mut self`
    execution state.
  - Both windows read `path` via the SAME artifact-level pure helper the editor uses,
    `crate::artifacts::imperative::imperative_working_scene(document).path` — never through the
    sibling editor module.
  - This deviates slightly from the packet brief's suggestion ("a plain read-only render, not a kit,
    for the flow/graph side") by using `TableWindowKit` for the steps window too, instead of
    hand-rolling a `build_table_scene` call the way the editor's own main window does: `TableWindowKit`
    exists precisely for this shape (flat columns/rows) and reusing it is less code than duplicating
    the editor's `TableRow`/`table_rows` pattern, consistent with CLAUDE.md's "use existing
    libraries/kits as much as possible."
- `create_imperative_viewer() -> AppDefinition` via `Viewer::builder(IMPERATIVE_DIALECT)…
  .build_definition()`.

## `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::imperative::*` from `../../🎛️apps/📜️imperative/…`) replaced
by two independent regions:
- `//#region ✏️Editor` — `pub mod editor { pub mod imperative { … } }`, every leaf `#[path]`-mounted
  from `../../🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
- `//#region 👁️Viewer` — `pub mod viewer { pub mod imperative { … } }`, same base but `…/👁️viewer/…`,
  never mounting anything under `✏️editor/`.

Every `crate::apps::imperative::`/`apps::imperative::` reference across the moved Rust files became
`crate::editor::imperative::`/`editor::imperative::` (mechanical `sed`, editor tree only — confirmed
0 remaining `apps::imperative` hits afterward). The `#[cfg(target_arch = "wasm32")] pub use
apps::imperative::wasm::ImperativeSession;` re-export and the bottom `//#region 📚️Examples` mount for
`app_imperative_demo_session` were repointed at the new editor path (names kept, only the `#[path]`
strings/module prefixes changed). All 141 `#[path]` attributes verified against disk with the recipe's
Python snippet (0 missing) after every file existed.

**Emoji-typo trap hit twice this packet** (recipe step 11's exact warning): once inside a large
`glue.rs` Edit (`🏅️标准` for `🏅️standards`, caught immediately by the path-verification script before
declaring done) and once via a `Write` to a brand-new file path (same substitution, caught by
immediately `ls`-ing the parent after the write, per the recipe's own mitigation, then `rm -rf`'d and
rewritten from a copy-pasted-correct path). Neither escaped into a landed commit; both are called out
here as the recipe asked, since this is exactly the failure mode it flags as easy to miss.

## Extension crates checked

`✏️s/🔌️plugins/📜️imperative/🧩️extensions/{🧠️logic, 📣️effect, 🧮️math, 🎮️control, 📝️text}/` — grepped
for `apps::imperative`/`🎛️apps/📜️imperative`: **0 hits, 0 changes needed.** They are independent
library crates with no `🎛️apps`/`🪆️subsets` of their own, unaffected by this migration.

## Plugin root (`✏️s/🔌️plugins/📜️imperative/🦀️component.rs`)

`.document_app::<crate::apps::imperative::ImperativePlayApp>(create_imperative_app())` → two calls:
`.editor::<crate::editor::imperative::ImperativePlayApp>(crate::editor::imperative::
create_imperative_app())` and `.viewer::<crate::viewer::imperative::ImperativeViewer>(crate::viewer::
imperative::create_imperative_viewer())`.

Added `#[cfg(test)] mod surface_tests` using the **canonical** framework testkit functions closed by
w0-f gap 2 (`semio_framework_plugin::testkit::{assert_viewer_never_mutates,
assert_editor_and_viewer_share_dialect}`) — no local stand-ins, unlike the cad pilot which predated
their landing.

## `📦️packages/🦀️rust/Cargo.toml`, `📦️packages/🟦️typescript`

No `[package.metadata.semio.storybook].sourceRoots` entry exists in this plugin's `Cargo.toml` (cad
had one; imperative doesn't) and there is no `tsconfig.json` in this plugin's TS package (it ships a
plain `package.json` + `📦️index.ts` instead) — grepped both for `🎛️apps` and the literal old path
string: 0 hits needing repointing. `📦️index.ts` already referenced (pre-existing, unrelated to this
migration) `../../🗿️artifacts/📜️imperative/🧬️schema/…` / `🪓️decomposer/…` / `🚪️io/…` paths that skip
the `🏅️standards/🔖️1/🪆️subsets/✳️any` segment entirely — those look like a pre-existing broken
reference outside this packet's scope (not an app-path reference, not touched).

## Deletion

`✏️s/🔌️plugins/📜️imperative/🎛️apps/` removed in full (it was the plugin's only app) once every real
file had a real destination and all glue.rs paths verified.

## Outside-lease referrers (report only, none found)

Repo-wide grep for `apps::imperative` (Rust) and the literal path `🎛️apps/📜️imperative`
(Rust/TS/TOML/JSON) found:
- **Zero real compile-time dependencies** from any other plugin, extension, or framework crate.
- One informational hit in root `📜️script.ts:8300` — a hardcoded fixture/inventory list entry
  `"✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎮️commands/🔧️step/🦀️component.rs"` referencing a
  command name (`🔧️step`) that doesn't even match any of imperative's actual command directories
  (`add-step`/`remove-step`/etc.) — clearly stale historical data in a list, not a live reference or
  compile dependency. Root `📜️script.ts` is outside this packet's lease regardless; not touched.
- The remaining hits are all inside other tickets' own historical `.🦑️repo/🎫️tickets/**` JSON/report
  files (snapshots from ticket `26/08/05`–`26/08/13` work), not live code.

No blocker to report to any other packet owner.

## SDK gaps hit (not already closed by w0-f)

None new. The two gaps the cad pilot hit and w0-f didn't close are the same ones this packet hit and
worked around identically:
1. `EditorBuilder` has no `.example_source(…)`/`.workflow(…)` (w2-cad-report SDK gap #4) — dropped,
   documented inline in `✏️editor/🦀️component.rs`'s manifest fn.
2. `testkit::new_app_with_registry<A: ArtifactApp + Default>(manifest: fn() -> App)` still expects the
   old `App { definition, examples }` shape (w2-cad-report SDK gap #3) — worked around with a local
   `imperative_app_manifest_for_testkit() -> App { App { definition: create_imperative_app(),
   examples: Vec::new() } }` wrapper in the editor's `testkit` submodule, same pattern cad used.

Gaps 1 and 2 from the cad pilot (`ArtifactEditor`/`ArtifactViewer`/etc. re-exports; `testkit::
assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect`/`new_viewer`) are confirmed
closed — used bare/canonical throughout this packet with no local stand-ins.

## Verification

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-imperative --all-targets --keep-going`, six full runs
over the session (final run's full output in `🧪️w2-p8-imperative-cargo.txt`). **Every run's error set
was 0-anchored in `🔌️plugins/📜️imperative` files** (`grep -c "🔌️plugins/📜️imperative"` on each run's
output returned 0), while the actual blocking errors moved upstream exactly the way both prior reports
predicted, confirmed live via `git status --porcelain`/`git log --date=iso` on each failing file before
attributing it away:
- Run 1: 3 errors in `semio-framework-plugin`'s own file (`AppFrame` missing fields, `ArtifactStore`
  missing method) — file `M` (modified, uncommitted).
- Run 2: same 3, unchanged (peer session mid-edit).
- Run 3: moved to `semio-framework-os-kernel`'s `🏪️store/🦀️component.rs` (`HistoryLog`/`Conflict`
  field-shape mismatch, 2 errors) — file `M`, last commit 2026-08-16 03:32:28 (today, this session).
- Run 4: moved back to `semio-framework-plugin` (1 error: `AppCommand::SetMergePolicy`/
  `ResolveConflict`/`ReadConflicts` not covered by a match) — the `26/08/16/MUTATION-OUTCOMES-MERGE-
  POLICIES-AND-FIRST-CLASS-CONFLICTS` ticket's channel-variant sweep landing incrementally, same
  ticket w0-f's own report names as still in flight.
- Runs 5 and 6: identical single error, stable — confirms the peer session is between edits, not that
  this packet caused anything. **`known-broken-by-live-peers` crates per the task brief
  (`semio-s-plugin-stdio`, `semio-framework-os-kernel`) plus `semio-framework-plugin` itself (the SDK
  crate this whole ticket is extending) are all mid-refactor by concurrent sessions right now; this
  packet's own crate has never once been reachable for a full typecheck this session.**

`cargo test -p semio-s-plugin-imperative`, output in `🧪️w2-p8-imperative-test.txt` — same blocker
(`semio-framework-plugin` fails to build before `semio-s-plugin-imperative` is even attempted).

Since a real compile could not be obtained, every trait-boundary/import edit in this packet was
additionally verified by hand against the pilot's own confirmed-compiling equivalents (cad's
`✏️editor`/`👁️viewer` files, read in full) line-by-line for signature/import shape, and by grepping
this packet's own tree for every leftover `ArtifactApp`/`VcsArtifactApp<`/`App::builder`/
`App::from_builder`/`IMPERATIVE_PLAY_APP_ID`-as-trait-const before stopping (all clear). This is
disclosed explicitly per CLAUDE.md's "must not say a test is passing when you didn't run it" —
the tests were RUN (`cargo test`), but did not reach a pass/fail verdict for this packet's own code;
re-run once `semio-framework-plugin`/`semio-framework-os-kernel` finish their concurrent landing.

## Files touched

Created:
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**`
  (moved content + 2 new real `🟦️component.ts` twins for `📋️main`/`📝️script` + 1 new surface-root
  `🟦️component.ts`)
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**`
  (`🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and both windows — `📋️main`
  renamed from the scaffold's generic `🪟️main`, `📝️script` authored fresh; taxonomy facet dirs
  otherwise `📌️empty.md`)

Edited:
- `✏️s/🔌️plugins/📜️imperative/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring,
  `surface_tests`)
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🦀️component.rs` (`IMPERATIVE_DIALECT`,
  `.document_codec::<EditorApp<…>>()`, doc fixes)
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
  🧬️mutations/💾️binary/🦀️component.rs` (doc fix)
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions)

Deleted:
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/` (whole tree — the plugin's only app)

Not touched (no changes needed, checked): `✏️s/🔌️plugins/📜️imperative/🧩️extensions/**`,
`📦️packages/🦀️rust/Cargo.toml`, `📦️packages/🟦️typescript/**`.

Scratch (ticket folder): `🧪️w2-p8-imperative-cargo.txt`, `🧪️w2-p8-imperative-test.txt`.
