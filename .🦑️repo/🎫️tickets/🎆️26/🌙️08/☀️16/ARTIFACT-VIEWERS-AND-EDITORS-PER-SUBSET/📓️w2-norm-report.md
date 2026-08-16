# W2 Packet P3 (norm) — Report

Lane: W2 packet P3, plugin `📕️norm`, 15 subsets (`s.norm.<variant>@1/*`, variant ∈ {din4108,
din16798, din18599, en1990–en1999, iso16757, vdi3805}). Scope: migrate all 15 retired
`✏️s/🔌️plugins/📕️norm/🎛️apps/<app>/` trees into their subset's `✏️editor/`, author a real `👁️viewer/`
for each, rewire `📦️glue.rs`/plugin root/`Cargo.toml`, delete `🎛️apps/`, verify.

Followed the pilot's recipe (`📓️w2-cad-report.md`) and reused the landed W0-F SDK gap fixes
(`📓️w0-f-report.md`) directly — no local workarounds for Gap 1/2 were needed except for the
still-open gaps documented below.

## Why one script, not 15 hand-edits

norm's `🖥️app-surface/🦀️component.rs` doc-comment states the 15 apps are "structurally identical by
construction … and differ only in their per-standard `Document` type, ids and labels." Verified this
empirically before writing anything: normalized diffs (`sed s/din4108/XXX/` etc.) between din4108 and
en1990's window/panel/root files showed **zero** structural differences outside naming, except one
real one — `en1990`'s (and several other EN apps') `set-snapshot` command payload is `text: String`
(a DSL-text field) rather than din4108's `#[dsl(block)] snapshot: XSnapshot`, because some snapshot
types no longer implement `DslField` after an unrelated composed-child migration. That difference
lives entirely inside the *moved, unchanged* command file and the root file's own `#[cfg(test)]`
call-sites — both survive verbatim through this migration since the transform never rewrites command
bodies, only moves them and fixes `apps::<app>::` → `editor::<app>::` references.

Given that, every editor migration step was driven by three ticket-scratch Python scripts (kept per
CLAUDE.md's "temporary scripts live in the ticket folder" rule, not committed as permanent tooling):
`🐍️norm-migrate-editor.py` (move + regex-transform each app's own root `component.rs` **in place on
its own real text**, never a synthesized template), `🐍️norm-migrate-ts-and-viewer.py` (editor window
TS twins, artifact-root `DIALECT`/`DOCUMENT_SCHEMA` consts, the whole real viewer surface),
`🐍️norm-glue-rewrite.py` (glue.rs `🎛️Apps` region → `✏️Editor`/`👁️Viewer` regions + `📚️Examples`
repoint). Every transform was dry-run against all 15 apps' real text with assertions before any file
was touched, and the manifest/testkit regex asserted on literal pre-migration substrings copied from
the actual files — not hand-typed guesses.

## What landed, per subset (×15)

### Editor (`…/🪆️subsets/✳️any/✏️editor/`)

Whole app tree moved intact: root `component.rs` (now `impl ArtifactEditor for <X>PlayApp`), `🎮️commands/*`
(3: evaluate/set-snapshot/selected-check), `🎭️modes/✏️edit/{component.rs, 🪟️windows/{📥️inputs,📊️results}}`,
`📌️panels/*` (3: document/catalogue/inspection), `📚️examples/🎬️demo-session/*`. Every `crate::apps::<app>::`
reference across the moved tree became `crate::editor::<app>::` (mechanical, verified 0 leftover
`apps::` references across all 15 trees after the pass).

Root `component.rs` transform (applied to each app's own real content, not a template):
- `impl ArtifactApp for <X>PlayApp` → `impl ArtifactEditor for <X>PlayApp`.
- Top-level `pub const APP_ID` and the trait's `const APP_ID` both removed; `const DIALECT: Dialect =
  crate::artifacts::<variant>::<VARIANT>_DIALECT;` added.
- `create_<variant>_app()`: `App::from_builder(App::builder(APP_ID, LocalizedLabel::data(LABEL))…)` →
  `Editor::builder(<VARIANT>_DIALECT)…build_definition()`; the old `.example(...)`/`.workflow(...)`
  tail calls dropped (SDK gap, documented inline — see below).
- Testkit: `VcsArtifactApp<X>` → `VcsArtifactApp<EditorApp<X>>`, `new_app::<X>()` →
  `new_app::<EditorApp<X>>()`, and each app's `new_app_with_registry::<X>(create_x_app)` call (which
  needs `fn() -> App`, not the new `AppDefinition`) got a small `<variant>_manifest_for_testkit()`
  wrapper (`App { definition: create_x_app(), examples: Vec::new() }`), same pattern the pilot used.
- The one stale `assert_eq!(definition.id, APP_ID)` test line per app was dropped (the id is now
  derived via `surface_app_id`, already proven by the plugin-root `surface_tests`; re-deriving it here
  would need a new `semio-framework` Cargo dependency norm doesn't otherwise need — cad's own migrated
  editor doesn't re-check `.id` either).

Every window (`📥️inputs`, `📊️results`) gained a real `🟦️component.ts` twin (typed `*ViewModel`
interface + window-kind-id/body-key constants mirroring the Rust `render()` boundary) — norm had zero
`.ts` twins pre-migration, so this is new content, not a move. The editor surface root also gained a
real `🟦️component.ts` (namespaced `export * as inputsWindow`/`export * as resultsWindow`).

### Viewer (`…/✳️any/👁️viewer/`)

A genuinely independent, real `<X>Viewer: ArtifactViewer` per subset:
- `Snapshot`/decode-only `Mutation` = the same artifact-level `<X>Snapshot`/`crate::artifacts::<v>::op::<X>Mutation`
  the editor uses (shared, artifact-level, safe for a viewer to reference).
- `Config`/`Presence`/`Transient` = framework `NoConfig`/`NoPresence`/`NoTransient` — a viewer needs no
  persisted per-session state to render a read-only compliance table.
- `Command` = a one-variant `<X>ViewCommand::Noop` (`#[derive(Default)]`, required by
  `assert_viewer_never_mutates`'s `V::Command: Default` bound).
- One real window, `📊️report` (`🎭️modes/👁️view/🪟️windows/📊️report`), using the SDK's
  **`TableWindowKit`** (contract §2.6 — the task brief's explicit recommendation for compliance/report
  data): `render()` recomputes the `CheckReport` straight off the snapshot via the subset's own pure
  `🧬️schema/💡️inferences::evaluate` function (the same one the editor's results window reaches through
  `NormHost`), then tables it via two new shared pure helpers in `app_surface`
  (`report_table_columns`/`report_table_rows`) — real content, not a placeholder, and reused across all
  15 viewers rather than duplicated 15 times. This file imports nothing from the sibling editor module.
- `create_<variant>_viewer() -> AppDefinition` via `Viewer::builder(DIALECT)…build_definition()`.

Two new shared helpers added to plugin-level `app_surface` (in-lease, same "shallowest common
ancestor" pattern the file already documents for `edit_mode_definition`): `view_mode_definition()` +
`single_window_layout(...)` (identical `view` mode / single-full-pane layout shape for all 15
viewers), plus `report_table_columns()`/`report_table_rows()`. Each has its own unit test.

### `🗿️artifacts/<app>/🦀️component.rs` (×15)

Added `pub const <VARIANT>_DIALECT: Dialect = Dialect { artifact_kind: "s.norm.<variant>", standard:
StandardId("1"), subset: SubsetId::ANY }` and `pub const <VARIANT>_DOCUMENT_SCHEMA: &str =
"semio.norm.<variant>/v1"` at the ARTIFACT level (not under `editor`), so the viewer can read them
without ever importing through the editor — mirrors cad's `CAD_DIALECT`/`CAD_DOCUMENT_SCHEMA`
placement exactly. `artifact_kind` string (`s.norm.<variant>`) matches each artifact's own
`definition()`'s `schema` capability descriptor, not guessed. Fixed
`.document_codec::<crate::apps::<variant>::<X>PlayApp>()` → `.document_codec::<EditorApp<crate::editor::<variant>::<X>PlayApp>>()`
(runtime `ArtifactApp` bound needs the SDK adapter).

**Pre-existing repo damage found and fixed in passing** (in-lease, so fixed, not just reported): three
of the 15 artifact-root files (`din4108`, `din16798`, `din18599`) had a broken `//#region 🔖️ArtifactKind`
— a concurrent session's removal of a since-relocated `🚪️DerivedIoRegistry` region (unrelated ticket,
confirmed via `git diff --cached` before this packet touched anything) had also eaten the preceding
doc-comment/region-open line, leaving a dangling `// `)` so the` orphan comment fragment. Since these
three files needed editing anyway (DIALECT/DOCUMENT_SCHEMA/document_codec), restored the doc-comment
verbatim from the 12 unaffected siblings' identical text.

### Plugin root (`✏️s/🔌️plugins/📕️norm/🦀️component.rs`)

Fifteen `.document_app::<X>(create_x_app())` calls → thirty `.editor::<X>(create_x_app())` +
`.viewer::<V>(create_v_viewer())` pairs. Added `#[cfg(test)] mod surface_tests` calling the now-landed
`semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
(W0-F Gap 2, no local stand-in needed — the pilot's version predates that landing) once per subset via
a small macro, 15 test functions total.

### `📦️glue.rs`

Old `//#region 🎛️Apps` (15 `pub mod <variant> { … mounting ../../🎛️apps/<app>/… }` blocks) replaced by
independent `//#region ✏️Editor` / `//#region 👁️Viewer` regions, each `pub mod editor { pub mod
<variant> { … } }` / `pub mod viewer { pub mod <variant> { … } }`, every leaf `#[path]` derived
programmatically from the same app table the other two scripts used (never hand-typed — the pilot's
"emoji-typo trap" is structurally impossible here since no path segment was retyped by hand). The
bottom `//#region 📚️Examples`'s 15 `app_<variant>_demo_session` mounts repointed at the new editor
path (names kept, only the `#[path]` string changed).

**Verified against disk**: every one of the glue.rs's 1670 non-`"."` `#[path]` attributes resolves to
a real file (`re.findall` + `os.path.isfile`, run after the rewrite — 0 missing).

### `📦️packages/🦀️rust/Cargo.toml`

Fifteen `[[package.metadata.semio.playground]]` entries' `app = "norm-<x>-play"` (old hand-written
ids) updated to the new derived surface ids `"s.norm.<variant>@1/*#editor"` — same fix class as W0-F's
demonstrator-side `"cad-play"` → `"s.cad.cad@1/*#editor"` repoint, but this one is inside this
packet's own lease (norm's own Cargo.toml), so fixed here rather than flagged. No storybook
`sourceRoots` or tsconfig `include` entries pointed at `🎛️apps` (norm has no `tsconfig.json`; its TS
package's `📦️index.ts` only re-exports artifact-level schema facets, never app-level ones) — nothing
else needed there.

### Deletion

`✏️s/🔌️plugins/📕️norm/🎛️apps/` removed in full, including one stray plugin-root-facet marker file
(`🎛️apps/🦀️component.rs`, a bare docstring-only stub) the migration script's directory-emptiness check
caught and flagged rather than silently deleting.

## Decision recorded: shared `NormConfig`/`NormPresence` stay plugin-level

`crate::config::{NormConfig, NormConfigMutation, NormHost}` / `crate::presence::{NormPresence,
NormPresenceMutation}` were **not** relocated into each of the 30 surfaces' own `🎚️config`/`👥️presence`
facets. Their own doc-comments are explicit that this is intentional, pre-existing design: "all
fifteen compliance apps have the identical config shape … unlike `shooting`'s per-app `ShootingConfig`
this is ONE type reused by every app," at "the shallowest taxonomy node common to every consumer" —
exactly the same placement rule `app_surface`/`document` already use, both of which also stayed at the
plugin root through this migration. Each of the 30 surfaces' own `🎚️config`/`👥️presence`/`🫧️transient`
dirs (`surfaceRequiredChildDirs`) still carry the required `📌️empty.md` taxonomy leaf — the editor's
`ArtifactEditor::Config = NormConfig` is the shared plugin-level type; the viewer's `Config = NoConfig`
needs no per-subset config at all. Physically duplicating the shared type into 15 (or 30) copies would
have broken `app_surface`'s existing generic helpers (`commit_selected_check_index<M>`, `norm_io`,
etc., which name `NormConfigMutation` directly) for 14 of the 15 apps and re-introduced the exact
per-app duplication the pre-existing design deliberately avoids. Flagging for the coordinator in case
"distribute per the four-lane rule" was meant more literally than this reading — happy to revisit if so.

## SDK gaps found (framework, outside this packet's lease — same class W0-F already reported)

1. `Dialect`, `StandardId`, `SubsetId`, `WindowKit`, `TableWindowKit`, `TableView`, `InteractionView`
   are still only reachable via `semio_framework_plugin::app::…`, not the curated crate-root `pub use`
   list — unlike `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/`EditorApp`/`ViewerApp`/`ViewEmit`,
   which W0-F's Gap 1 already fixed and this packet used bare, directly. `TableWindowKit`/`TableView`/
   `WindowKit` (contract §2.6, landed after the pilot's packet) were never added to that list at all.
   Every W2 packet reaching for a window kit will hit the identical `app::` workaround this packet used.
2. `EditorBuilder`/`ViewerBuilder::build_definition()` still take no `.example(...)`/`.workflow(...)` —
   same gap #4 from the pilot's report, hit identically here for all 15 apps; each subset's own
   `📚️examples/🎬️demo-session` facet (moved verbatim, real content) is the replacement surface.
3. `testkit::new_app_with_registry`'s `manifest: fn() -> App` signature is still not
   `AppDefinition`-aware — same gap #3 from w0-f-report, hit for all 15 apps (each needed the small
   `<variant>_manifest_for_testkit()` wrapper).

## Outside-lease referrers (report, not fixed)

- Repo-root `📜️script.ts`'s `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` (a hardcoded `Set<string>` of
  file paths, ~line 8067) still lists 30 entries under the old `🎛️apps/📕️norm/…` paths (now deleted).
  Harmless as dead allowlist rows (they just match nothing now), but the new `✏️editor/…` paths this
  packet created are *not* in that allowlist — checked this did not create a live breach (the
  `handcrafted-grammar/spec-distinctness` category dominating the repo-wide policy run's 24k count is
  pre-existing and unrelated to norm's command/mutation grammars), but flagging the stale rows for
  whoever next touches that allowlist, since `🧰️framework/**`/repo-root is outside this lease.
- No other plugin's Rust or Cargo.toml referenced `apps::<norm-variant>::` or `semio-s-plugin-norm`
  anywhere in the repo (grepped repo-wide before declaring done) — unlike cad, no demonstrator-style
  fix was needed for norm.

## Verification run

- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-norm --all-targets --keep-going`, run **six times**
  over roughly 30 minutes (output for the last run in `🧪️w2-norm-cargo.txt`); every single run's error
  set was 100% inside `semio-framework-plugin`'s own file, **zero** ever anchored inside `📕️norm`
  (`grep -c "📕️norm" <run output>` = 0, all six times). See "Concurrent workspace churn" below —
  never got a clean compile of the shared framework crate long enough to observe norm's own files
  actually get checked. **Re-run `cargo check -p semio-s-plugin-norm --all-targets --keep-going` once
  the framework crate settles — expected clean based on every static/structural check this packet ran
  (see the sanity nets below), but not yet confirmed by the compiler.**
- `cargo test -p semio-s-plugin-norm` — not attempted; blocked upstream of even `cargo check` by the
  same `semio-framework-plugin` churn (a crate cannot be tested before it compiles). Re-run after the
  `cargo check` above passes clean.
- `bun ./📜️script.ts policy` (repo root — the registry-local `📇️registry/📜️script.ts policy` command
  errors immediately with "must export const policy = defineLint(...)", it is a different subcommand;
  the pilot's own precedent, confirmed in `📓️w2-cad-report.md`, ran this at the repo root too), full
  run in `🧪️w2-norm-policy.txt`, cross-checked directly against `.🦑️repo/⚡️cache/breaches/compose.json`
  (same method the pilot used — the top-level "24420 high-priority breach(es)" summary is repo-wide
  noise across 34 unrelated rule families, not a norm signal):
  - `taxonomy/surface-completeness`: 7 total repo-wide, **0 for norm's 15 dialects**.
  - `taxonomy/surface-scaffold-residue`: 227 total repo-wide, **0 for norm's 15 dialects**.
  - `taxonomy/viewer-purity`: 0 total repo-wide (no plugin has a violation yet), **0 for norm**.
  - `plugin-dependency/contributed-surface-target`, `taxonomy/os-config-shape`,
    `taxonomy/missing-owner-surface`: 0, unaffected.
  - **Target met**: all 15 `s.norm.<variant>@1/*` subsets show 0 breaches and 0 scaffold-residue rows
    across all three new surface policies.
  - Pre-existing, unrelated to this packet (found while filtering, not fixed —
    `plugin-dependency/parity`, 2 rows): norm's Cargo.toml already Cargo-depends on
    `semio-s-plugin-stdio`/`semio-s-plugin-fem` without a matching `.depends_on(...)` declaration;
    predates this ticket, untouched by this packet's edits, not a viewers/editors concern.
- Sanity nets run before/alongside cargo (since the shared framework crate was mid-edit by concurrent
  sessions for most of this packet's session, see below): brace/paren balance across all 30 new
  `.rs` surface files (0 unbalanced), 0 leftover `apps::` references across all 15 editor trees
  (grepped per-app and in aggregate), 0 stray `SCAFFOLD` markers, 0 stray `🪟️main` placeholder dirs,
  all `surfaceRequiredChildDirs` facets present with `📌️empty.md` for all 15×2 = 30 surfaces.

### Concurrent workspace churn (confirmed live, not this packet's bug)

`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-norm --all-targets --keep-going` never got a clean run
across six attempts spanning roughly 30 minutes: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
(the SDK file itself, a dependency of every plugin including norm) showed as `M` (modified,
uncommitted) in `git status` throughout, mtime advancing between every run. This is not a surprise —
the ticket's own `📌️important.md`, written at ticket start, names this exact file as owned by a live
peer session (`26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS`) with
mtimes "within 60s of this ticket's start commit." Each of the six runs surfaced a **different** error
class purely inside that one framework file, never inside `📕️norm`'s own files
(`grep -c "📕️norm" <run output>` = 0, all six times, confirmed explicitly per run):
  1. `AppFrame` missing field `report`; `ArtifactStore::snapshot_with_conflicts` missing method.
  2. `AppFrame` missing field `messages`; same `snapshot_with_conflicts` gap.
  3. `AppCommand` non-exhaustive match (`SetMergePolicy`/`ResolveConflict`/`ReadConflicts` not
     covered) — a different, unrelated concurrent ticket's (MUTATION-OUTCOMES-MERGE-POLICIES-AND-
     FIRST-CLASS-CONFLICTS) channel additions.
  4. Same `AppCommand` non-exhaustive match, unchanged.
  5. Same `AppCommand` non-exhaustive match, unchanged.
  6. Same `AppCommand` non-exhaustive match plus a new `E0046` (trait item not implemented) —
     genuinely moving forward (a new, different symptom of the same in-flight refactor), not stuck.
`ps aux` throughout this session showed a large number of concurrent `cargo check -p semio-s-plugin-<x>`
processes (block, puzzle, imperative, trinity, energy, draw, playbook, forms, lowpoly, and more) plus
`cargo check -p semio-framework-os-kernel`/`cargo test -p semio-framework-os-kernel` invocations from
the MUTATION-OUTCOMES ticket's own session actively working through the exact `AppCommand`/
`MutationOutcome` refactor causing these errors — i.e. its owner is live and mid-fix, not stalled.
This is `feedback-concurrent-cargo-workspace-churn.md`'s scenario at a larger scale than previously
seen (dozens of parallel W2-packet sessions on the same shared, actively-mutating framework crate at
once). Given every run's error set stayed 100% inside that one non-`📕️norm` file across all six
attempts, and given the extremely wide correctness net this packet ran in its place (brace/paren
balance across all 30 new `.rs` and 105 `.ts` surface files, 0 leftover `apps::` references across all
15 editor trees, 0 stray SCAFFOLD markers, all `surfaceRequiredChildDirs` facets present for all 30
surfaces, 0 policy breaches on all three target policies, glue.rs's 1670 `#[path]` attributes all
resolved on disk), this packet's own code is not what's blocking. **Re-run `cargo check -p
semio-s-plugin-norm --all-targets --keep-going` and `cargo test -p semio-s-plugin-norm` once the
framework crate settles.**

## Files touched

Created (per subset ×15, `🗿️artifacts/<app>/🏅️standards/🔖️1/🪆️subsets/✳️any/`):
- `✏️editor/**` — moved content (commands/modes/windows/panels/examples) + 3 new real `🟦️component.ts`
  twins each (2 windows + surface root) = 45 new TS files total.
- `👁️viewer/**` — real `🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the
  `📊️report` window (5 new Rust files + 3 new TS files per subset = 120 total).

Edited:
- `✏️s/🔌️plugins/📕️norm/🦀️component.rs` (plugin root: 15×`.editor()`+`.viewer()` wiring,
  `surface_tests` module).
- `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️component.rs` (`MODE_VIEW`, `view_mode_definition`,
  `single_window_layout`, `report_table_columns`, `report_table_rows` + tests; one doc-comment fix).
- `✏️s/🔌️plugins/📕️norm/🎚️config/🦀️component.rs` (one doc-comment fix).
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<app>/🦀️component.rs` (×15 — `<VARIANT>_DIALECT`/
  `<VARIANT>_DOCUMENT_SCHEMA` consts, `.document_codec::<EditorApp<…>>()` fix; 3 of the 15 also got the
  pre-existing broken-doc-comment restoration).
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` (`✏️Editor`/`👁️Viewer` regions, `📚️Examples`
  repoint).
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml` (15 playground `app = "…"` metadata values).

Deleted:
- `✏️s/🔌️plugins/📕️norm/🎛️apps/` (whole tree — all 15 apps + the stray root marker file).

Scratch (ticket folder): `🧪️w2-norm-cargo.txt`, `🧪️w2-norm-policy.txt`, `🐍️norm-migrate-editor.py`,
`🐍️norm-migrate-ts-and-viewer.py`, `🐍️norm-glue-rewrite.py` (kept per CLAUDE.md, not deleted).
