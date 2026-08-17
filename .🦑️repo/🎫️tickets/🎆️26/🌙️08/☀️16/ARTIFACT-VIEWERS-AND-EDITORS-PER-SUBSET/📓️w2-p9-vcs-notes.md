# W2 Packet P9 (vcs) — Notes

Lane: W2 packet, plugin `🌿️vcs`, subset `s.vcs.vcs@1/*`. Followed the pilot's recipe
(`📓️w2-cad-report.md`), the frozen contract (`📋️contract-freeze.md`), and the closed SDK gaps
(`📓️w0-f-report.md`, `📓️w2-p8-report.md`).

## What landed

### Editor (`…/🪆️subsets/✳️any/✏️editor/`)

The entire retired `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/` app tree moved across intact: root
`🦀️component.rs` (now `impl ArtifactEditor for VcsPlayApp`), `🎚️config` (+schema), `👥️presence`
(+schema), `🗣️terminology`, `🎮️commands/*` (9 groups: increment-counter, patch-snapshot, text-edit,
edit, set-locale, no-operation, canvas-pointer-down/move/up, canvas-wheel), `📌️panels/*` (2:
`📄️artifact` document tree, `🔍️inspection`), `📚️examples/🎬️demo-session`, and the single mode
`🎭️modes/✏️edit/{component.rs, 🪟️windows/{📜️history,📝️editor}}`. Both windows gained a real
`🟦️component.ts` twin (`VcsHistoryViewModel`/`VcsHistoryColumnViewModel`/`VcsHistoryAuthor` mirroring
the framework `HistoryColumn`/`Author` wire shape for the history window; `VcsEditorViewModel`/
`VcsPlaySnapshotViewModel`/`VcsPlayLabelsViewModel` mirroring `render(projection: &VcsSnapshot,
labels: &VcsPlayLabels)` for the editor window) — the scaffold's single `🪟️main` placeholder was
deleted. The surface root also gained a real `🟦️component.ts` (namespaced re-export,
`historyWindow`/`editorWindow`, not a blanket `export *`).

`impl ArtifactApp for VcsPlayApp` → `impl ArtifactEditor for VcsPlayApp`; `const APP_ID` removed;
`const DIALECT: Dialect = crate::artifacts::vcs::VCS_DIALECT` added. `create_vcs_app()` now returns
`AppDefinition` (`Editor::builder(VCS_DIALECT)…build_definition()`) instead of `App`. The old
`create_vcs_app` had no `.example(...)`/`.workflow(...)` calls to begin with, so nothing was lost
there — noted inline anyway for parity with the other W2 packets' identical SDK-gap note.

Test-module fallout fixed: `VcsApp = VcsArtifactApp<VcsPlayApp>` → `VcsArtifactApp<EditorApp<VcsPlayApp>>`;
`new_app::<VcsPlayApp>()` → `new_app::<EditorApp<VcsPlayApp>>()`; `new_app_with_registry` needed a
local `vcs_app_manifest_for_testkit() -> App { App { definition: create_vcs_app(), examples: Vec::new() } }`
shim (the testkit fn's signature is still `fn() -> App`, unchanged for this ticket); the two tests that
read `create_vcs_app().definition` were repointed at `create_vcs_app()` directly (the function itself
now returns the definition).

### Viewer (`…/✳️any/👁️viewer/`)

A genuinely independent, minimal, real `VcsViewer: ArtifactViewer`:
- `Snapshot = VcsSnapshot`, `Mutation = crate::artifacts::vcs::VcsDemoMutation` (both artifact-level,
  shared with the editor — decode-only per contract §2.2).
- `Config`/`ConfigMutation` = framework `NoConfig`/`NoConfigMutation`; `Presence`/`Transient` likewise
  `NoPresence`/`NoTransient` — a viewer needs no persisted per-session state to render a read-only
  checkpoint tree.
- `Command` = a one-variant, `#[derive(Default)]` `VcsViewCommand::Noop` (viewer declares no
  actions); `handle` always returns `Ok(ViewEmit::default())`. `Default` is required by the real
  `testkit::assert_viewer_never_mutates::<V>()` (contract §2.5) to synthesize a representative
  command.
- One real window, `📜️history` (`🎭️modes/👁️view/🪟️windows/📜️history`), built on **`TreeWindowKit`**
  (`framework.window.tree`, contract §2.6) exactly as this packet's brief recommended — the checkpoint
  DAG is genuinely a forest (`HistoryColumn.parent_checkpoint_id: Option<String>` — exactly one
  parent per checkpoint), so it converts to a `TreeView` with zero cycle/multi-parent handling. Each
  checkpoint becomes one `TreeNodeView` nested under its parent; root checkpoints
  (`parent_checkpoint_id: None`) become tree roots. This is a deliberately simpler treatment than the
  editor's own swimlane-graph history window (`build_graph_timeline_scene`/`GraphTimelineScene`) —
  alternative names, lane/column layout and the per-row `checkoutCheckpoint`/`switchAlternative`
  navigation actions have no read-only counterpart (a viewer declares no actions at all), documented
  as an intentional simplification for a first-pass viewer, not a bug.
- `create_vcs_viewer() -> AppDefinition` via `Viewer::builder(VCS_DIALECT)…build_definition()`.

### DIALECT derivation

`VCS_DIALECT` lives at the ARTIFACT level (`🗿️artifacts/🌿️vcs/🦀️component.rs`), not under
`editor`/`viewer`, so a viewer file can read it without ever importing through the sibling editor
module. `artifact_kind: "s.vcs.vcs"` was **verified, not guessed**, against three independent sources
in the artifact's own `definition()`/schema:
1. The `"s.vcs.schema.artifact"` capability row's own descriptor: `.descriptor(b"s.vcs.vcs")?` in
   `🗿️artifacts/🌿️vcs/🦀️component.rs`'s `definition()`.
2. The subset's own schema id: `#[artifact_schema(id = "s.vcs.vcs")]` on `VcsSnapshot`
   (`…/🧬️schema/📸️snapshot/🦀️component.rs`) and on `vcs_artifact_schema_descriptor()`
   (`…/🧬️schema/🦀️component.rs`, `id: "s.vcs.vcs"`).
3. The subset's own `🔣️component.json`: `"artifact": "s.vcs.vcs"`
   (`…/🏅️standards/🔖️1/🪆️subsets/🔣️component.json`).

`standard: StandardId("1")`, `subset: SubsetId::ANY` mirror this file's own
`🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the canonical surface id is `s.vcs.vcs@1/*#editor` /
`s.vcs.vcs@1/*#viewer`, exactly the contract §1 grammar. (A separate, differently-scoped
`const VCS_DIALECT` already existed inside the subset's own `🚪️io/🦀️component.rs`
composer-registry module, value `"s.vcs"` — that is the pre-existing COMPOSER dialect, matching the
`"s.vcs.composer.native"` capability's own descriptor `s.vcs@1/*`, a different concept in a different,
private module; left untouched, no naming collision since it never leaves that module's scope.)

### `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::vcs::*` from `../../🎛️apps/🌿️vcs/…`) replaced by two
independent regions: `//#region ✏️Editor` (`pub mod editor { pub mod vcs { … } }`) and
`//#region 👁️Viewer` (`pub mod viewer { pub mod vcs { … } }`), every leaf `#[path]` mounted from the
real subset path on disk, deliberately never mounting anything under `✏️editor/` from the viewer
region. Every `crate::apps::vcs::` reference across the moved editor files became `crate::editor::vcs::`
(mechanical `sed`, editor tree only, 0 leftover). The bottom `//#region 📚️Examples` mount for
`app_vcs_demo_session` was repointed at the new editor path (name kept, only the `#[path]` string
changed). Verified with the recipe's Python path-resolution script: **145 `#[path]` attrs, 0
missing**.

**Emoji-typo trap hit and caught live**: while first drafting the glue.rs replacement region I
mistyped "🏅️standards" as the Chinese lookalike "🏅️标准" twice (config schema path, canvas-wheel
command path) — caught immediately by re-grepping the file for the Chinese substring right after
writing it (before running the path-resolution script), fixed both lines by copying the correct
"🏅️standards" string from an adjacent, already-correct line in the same file, then re-ran the
Python script to confirm 0 missing paths. A second, harmless instance of the same typo happened in a
throwaway `ls`/`cat` Bash probe while reading the cad pilot's files for reference (never touched any
real vcs file) — both instances are recorded here per the recipe's explicit ask to document the trap
when it fires.

### Plugin root (`✏️s/🔌️plugins/🌿️vcs/🦀️component.rs`)

`.document_app::<crate::apps::vcs::VcsPlayApp>(crate::apps::vcs::create_vcs_app())` → two calls:
`.editor::<crate::editor::vcs::VcsPlayApp>(crate::editor::vcs::create_vcs_app())` and
`.viewer::<crate::viewer::vcs::VcsViewer>(crate::viewer::vcs::create_vcs_viewer())`. Added
`#[cfg(test)] mod surface_tests` calling the REAL, now-landed
`semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
directly (per this packet's brief: the gap is closed, no local stand-ins written).

### `🗿️artifacts/🌿️vcs/🦀️component.rs`

Added `pub const VCS_DIALECT: Dialect = Dialect { artifact_kind: "s.vcs.vcs", standard:
StandardId("1"), subset: SubsetId::ANY }` (see DIALECT derivation above). Fixed the one real
(non-comment) `crate::apps::vcs::` reference: `.document_codec::<crate::apps::vcs::VcsPlayApp>()` →
`.document_codec::<semio_framework_plugin::EditorApp<crate::editor::vcs::VcsPlayApp>>()` (the runtime
`ArtifactApp` bound needs the SDK adapter, not the authoring trait implementor directly). Two stray
doc-comment references to the old `apps::vcs`/`🎛️apps` path fixed (one here, one in
`🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`), cosmetic.

### `Cargo.toml` / TS configs

Grepped the whole `📦️packages/**` tree for the literal old app path string (`🎛️apps/🌿️vcs`): **zero
hits**. Unlike cad, vcs's `Cargo.toml` never had a storybook `sourceRoots` entry or any `tsconfig.json`/
`vitest.config.ts` `include` pointing at the old app path, so nothing needed repointing there.

### Deletion

`✏️s/🔌️plugins/🌿️vcs/🎛️apps/` removed in full (it was the plugin's only app) once every real file had
a real destination — confirmed empty of anything but taxonomy-placeholder `📌️empty.md` files
immediately before deletion.

## Outside-lease referrers (repo-wide grep, report only)

Grepped the WHOLE repo for `apps::vcs::`/`crate::apps::vcs` (real Rust code, `.rs` files only, vcs's
own tree excluded): **zero real compile-dependency hits**. The only other repo-wide hits for the
literal string `🎛️apps/🌿️vcs` are inside historical ticket-folder JSON/scratch files under
`.🦑️repo/🎫️tickets/…` (records of past migrations, not live code) and one static path-string array
entry in root `📜️script.ts` (`…/🎮️commands/🖱️canvas/🦀️component.rs`, a path that was already stale
before this packet — pre-dates even this ticket, per the pilot's and W2-P8's identical finding that
this array isn't kept in sync with `🎛️apps` deletions and isn't this packet's job,
`policyTaxonomyDirsBreaches` not walking surface subtrees until W3).

## Verification

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-vcs --all-targets --keep-going`, output in
`🧪️w2-p9-vcs-cargo.txt` (final run kept; ran twice total):

- Run 1: 546 errors, ALL anchored in `semio-s-plugin-stdio`'s own files (checking that crate never
  even reached `semio-s-plugin-vcs`, which depends on it). Many were `error: expected item, found
  '+'` at line 1 of several stdio artifact files, with content like `++ ✏️s/🔌️plugins/🗄️stdio/…` —
  this is a transient read of a file mid-write by another live session (confirmed: `head -3` on the
  same file moments later showed completely normal Rust doc-comment content, and `git status
  --porcelain` on that file showed no uncommitted changes at check time — a torn read, not a real
  defect). **0 errors in `🌿️vcs` files.**
- Run 2 (re-run for a stable snapshot): 327 errors, still 100% anchored in `semio-s-plugin-stdio`
  (`grep -oE "✏️s/🔌️plugins/[^/]*"` on the log matches only `🗄️stdio`), now a consistent, reproducible
  class: `error[E0053]: method 'diff'/'mutate' has an incompatible type for trait … expected
  MutationOutcome<XDiff>, found XDiff` across dozens of stdio artifact standards (wav, mp4, mp3, epw,
  avi, …). Confirmed via `git status --porcelain -- ✏️s/🔌️plugins/🗄️stdio` (dozens of `M` entries,
  including `📦️glue.rs` and many `🧬️mutations/🦀️component.rs`/`🧬️schema/🦀️component.rs` leaves right
  now) and `git log --date=iso -3 -- ✏️s/🔌️plugins/🗄️stdio` (most recent real commit 2026-08-16
  14:18:35, message mentioning "Mutation Outcomes, Merge Policies and First-Class Conflicts") that
  this is the concurrent peer ticket
  `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` mid-sweep, converting
  `Mutation::diff`/`mutate` return types from bare `Diff` to `MutationOutcome<Diff>` repo-wide —
  exactly the class of change `📓️w0-f-report.md` documents hitting this same crate boundary earlier
  today. **0 errors anchored in `🌿️vcs` files in either run** (`grep -c "🔌️plugins/🌿️vcs"` on both
  logs reads 0). `semio-s-plugin-vcs` itself is never reached by `cargo check` because its own
  `Cargo.toml` dependency on `semio-s-plugin-stdio` fails first — expected, not this packet's bug, not
  fixed, not blocked on (documented per the recipe's step 16, not retried further since the failure
  class is stable/reproducible and clearly upstream).
- `cargo check --all-targets` also compiles test targets, so no separate `cargo test --no-run` run was
  needed to additionally probe test-target compileability — it is blocked by the identical upstream
  `semio-s-plugin-stdio` failure before ever reaching `semio-s-plugin-vcs`.

Policy/grep self-checks (all direct filesystem checks, not the CLI's cached `compose.json`):
- `grep -rl "SCAFFOLD"` under `✏️editor` + `👁️viewer`: **0**.
- `grep -rl "::editor::"` under `👁️viewer`: **0**.
- `grep -rln '\.mutation(\|Emit::mutations\|artifact_mutations'` under `👁️viewer`: **0**.
- `grep -rl "🎛️apps"` under `📦️packages`: **0** (nothing needed repointing).
- `grep -rl "apps::vcs\|crate::apps"` anywhere under the whole `🌿️vcs` plugin tree: **0**.
- `grep -rl "🏅️标准|🏅️标"` (the Chinese-lookalike typo) anywhere under the plugin: **0** (clean after
  the live fix documented above).
- `find … -type d -name 🎛️apps` under the plugin: **0** (directory fully deleted).

## Files touched

Created:
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (moved content +
  3 new real `🟦️component.ts` files: 2 windows + surface root)
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` (`🦀️component.rs`/
  `🟦️component.ts` at surface root, mode root, and the `📜️history` window; taxonomy facet dirs
  otherwise `📌️empty.md`)

Edited:
- `✏️s/🔌️plugins/🌿️vcs/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring, `surface_tests`
  using the real landed testkit helpers)
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️component.rs` (`VCS_DIALECT`,
  `.document_codec::<EditorApp<…>>()`, doc fixes)
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
  (doc fix)
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions)

Deleted:
- `✏️s/🔌️plugins/🌿️vcs/🎛️apps/` (whole tree — the plugin's only app)

Not touched (checked, nothing needed): `Cargo.toml`, `📦️packages/🟦️typescript/📦️index.ts`,
`📋️project.json`, `AGENTS.md`.

Scratch (ticket folder): `🧪️w2-p9-vcs-cargo.txt`.
