# W2 Packet P8 (forms) — Notes

Lane: W2 packet P8, plugin `📋️forms`, subset `s.forms.forms@1/*`. Scope: migrate the retired
`✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/` app into `✏️editor`, author a real `👁️viewer`, rewire
`📦️glue.rs`, and verify. Followed `📓️w2-cad-report.md`'s migration recipe (steps 1-16) exactly, using
the SDK gaps already closed by `📓️w0-f-report.md` (bare `ArtifactEditor`/`ArtifactViewer`/`Editor`/
`Viewer`/`EditorApp`/`ViewerApp`/`ViewEmit` imports, canonical `testkit::assert_viewer_never_mutates`/
`assert_editor_and_viewer_share_dialect`/`new_viewer` — none used locally by this packet since forms'
own root test module wrote direct `create_forms_viewer()`/`DIALECT` assertions instead; see "SDK gaps"
below for why the two canonical testkit fns were not wired in here).

Pre-existing state confirmed before starting: `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️component.rs`
already carried UNCOMMITTED staged changes from a concurrent live session (pure reformatting — line
wrapping/unwrapping only, confirmed via `git diff --cached`, unrelated to this migration, no
`🚪️DerivedIoRegistry` region existed in this file to begin with). Built every edit on top of the
current on-disk content; did not touch or revert that staged diff.

## What landed

### Editor (`…/🪆️subsets/✳️any/✏️editor/`)

The entire old app tree moved across intact into the W1-E scaffold (which already carried the
required taxonomy shape under a scaffold `🎭️modes/✏️edit/🪟️windows/🪟️main` placeholder — deleted and
replaced, not diffed): root `🦀️component.rs` (now `impl ArtifactEditor for FormsPlayApp`), `🎚️config`
(+schema), `👥️presence` (+schema), `🗣️terminology`, `🎮️commands/*` (24 payload modules), `📌️panels/*`
(3: document/catalogue/inspection), `📚️examples/🎬️demo-session`, and the mode subtree — renamed from
the scaffold's default `✏️edit` to `📝️blueprint` (the app's own, pre-existing mode name — per this
packet's brief, kept as-is rather than normalized to `✏️edit` the way cad's pilot did, since forms only
ever had the one mode and its name is already meaningful). Two windows, `🧱️builder` (BlockList) and
`▶️try` (Canvas2d) — each gained a real `🟦️component.ts` twin (typed `FormsBuilderViewModel`/
`FormsTryViewModel` + window-kind id/body-key/surface-id constants) replacing the scaffold's single
`🪟️main` placeholder. The surface root also gained a real `🟦️component.ts` (namespaced re-export of
both window twins — `export * as builderWindow from …`/`export * as tryWindow from …`).

`impl ArtifactApp for FormsPlayApp` → `impl ArtifactEditor for FormsPlayApp`; `const APP_ID` removed
(the `FORMS_PLAY_APP_ID` string constant itself is KEPT — it is reused as a plain `ActionFactory`
routing tag, not a trait const, per recipe step 7's explicit carve-out); `const DIALECT: Dialect =
crate::artifacts::forms::FORMS_DIALECT` added. `create_forms_app()` now returns `AppDefinition`
(`Editor::builder(FORMS_DIALECT)…build_definition()`) instead of `App`; the trailing
`.example("default", …)`/`.example("onboarding", …)`/`.example("building-component", …)`/
`.workflow("forms", "Forms", "data")` calls were **dropped**, not ported (SDK gap, see below) — the
three now-unused imports (`default_example_json`, `onboarding_example_json`, `forms_dsl` alias) were
removed rather than left dead.

### Viewer (`…/✳️any/👁️viewer/`)

A genuinely independent, real `FormsViewer: ArtifactViewer`:
- `Snapshot = FormsSnapshot`, `Mutation = crate::artifacts::forms::op::FormMutation` (both
  artifact-level, shared with the editor — decode-only per contract §2.2).
- `Config`/`ConfigMutation` = framework `NoConfig`/`NoConfigMutation`; `Presence`/`Transient` likewise
  `NoPresence`/`NoTransient`. Documented as an intentional simplification: the read-only Try preview
  renders every step flat (no wizard step-cursor, no answer-entry state) rather than carrying its own
  cut-down config type, given the packet's time budget — flagged as a real follow-up opportunity below,
  not silently dropped.
- `Command` = a one-variant `FormsViewCommand::Noop` (deriving `Default`, matching the bound
  `assert_viewer_never_mutates` needs, though this packet did not wire that canonical fn in — see SDK
  gaps); `handle` always returns `Ok(ViewEmit::default())`.
- One real window, `▶️try` (`🎭️modes/👁️view/🪟️windows/▶️try`, mode `view` — a fresh mode name distinct
  from the editor's `blueprint`, matching the pilot's cad-viewer convention of a dedicated `view` mode
  rather than reusing the editor's mode name), rendering the actual `FormsSnapshot`: every step's
  questions in document order, each showing its typed default value (`default_value_for_question` →
  `dsl_to_value` → `json_string_value`, all pure artifact-level `🧬️schema` helpers, same ones the
  editor's own Try window uses) as **plain read-only text**, not an editable input — no wizard
  navigation, no answer entry, since a Noop-only command channel has nothing to drive them with.
  Extension question kinds (host-contributed, resolved through the editor's own `contributions_json`
  plumbing) fall back to a plain `"(kind)"` label here rather than resolving any contribution, since the
  viewer declares no config lane to carry `contributions_json` — documented in the window file's own doc
  comment, phrased without literally typing the forbidden `::editor::` substring.
- `create_forms_viewer() -> AppDefinition` via `Viewer::builder(FORMS_DIALECT)…build_definition()`.

`policyViewerPurityBreaches` self-check: grepped the whole `👁️viewer` tree for `::editor::`,
`.mutation(`, `Emit::mutations`, `artifact_mutations` — zero hits (see "Verification" below for the
actual grep run, not just this claim).

### `📦️glue.rs`

Old `//#region 🎛️Apps` (mounting `apps::forms::*` from `../../🎛️apps/📋️forms/…`) replaced by two
independent regions: `//#region ✏️Editor` (`pub mod editor { pub mod forms { … } }`) and
`//#region 👁️Viewer` (`pub mod viewer { pub mod forms { … } }`), each `#[path]`-mounted from the real
subset dirs. Every `crate::apps::forms::` reference across the 41 moved Rust files became
`crate::editor::forms::` (mechanical `sed`, editor tree only — 0 occurrences of the old prefix remain
anywhere under `✏️s/🔌️plugins/📋️forms/`, confirmed by a final whole-tree grep). The bottom
`//#region 📚️Examples` mount for `app_forms_demo_session` was repointed at the new editor path (name
kept, only the `#[path]` string changed). All 101 `#[path]` attributes in the rewritten `glue.rs`
verified against disk with the recipe's own Python script — 0 missing on the final run (1 missing on
the first run, the bottom Examples mount above, fixed and re-verified).

**The emoji-typo trap hit twice while authoring the new glue.rs regions** — see "Incidents" below.

### Plugin root (`✏️s/🔌️plugins/📋️forms/🦀️component.rs`)

`.document_app::<crate::apps::forms::FormsPlayApp>(crate::apps::forms::create_forms_app())` → two
calls: `.editor::<crate::editor::forms::FormsPlayApp>(crate::editor::forms::create_forms_app())` and
`.viewer::<crate::viewer::forms::FormsViewer>(crate::viewer::forms::create_forms_viewer())`.

### `🗿️artifacts/📋️forms/🦀️component.rs`

Added `pub const FORMS_DIALECT: semio_framework_plugin::app::Dialect = Dialect { artifact_kind:
"s.forms.forms", standard: StandardId("1"), subset: SubsetId::ANY }` — lives at the ARTIFACT level
(not under `editor`/`viewer`) specifically so the viewer file can read it without ever importing
through `editor`. `artifact_kind = "s.forms.forms"` matches the id
`FormsArtifact`'s own `#[artifact_schema(id = "s.forms.forms")]`
(`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:21`); `standard`/`subset` match this file's
own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the canonical surface id is
`s.forms.forms@1/*#editor` / `s.forms.forms@1/*#viewer`, exactly the contract §1 grammar. Fixed the one
other real (non-comment) `crate::apps::forms::` reference: `.document_codec::<crate::apps::forms::FormsPlayApp>()`
→ `.document_codec::<semio_framework_plugin::EditorApp<crate::editor::forms::FormsPlayApp>>()` (the
runtime `ArtifactApp` bound needs the SDK adapter, not the authoring trait implementor directly). Three
stray doc-comment references to the old path fixed too (cosmetic), plus one in the sibling
`🧬️mutations/💾️binary/🦀️component.rs`.

### `⚙️engine`

Not applicable — forms' `🎛️apps/📋️forms/⚙️engine` directory was already fully empty on disk before this
packet started (a prior ticket, `26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES`, had already
relocated its sole content — `forms_io()` and its unit test — into the app root and the artifact
`declaration()`/`register_app_schema` respectively; the editor root `🦀️component.rs`'s own doc comments
still narrate this history). No engine-facet move was needed for this packet; the empty directory was
removed along with the rest of `🎛️apps/` in the final deletion.

### `📦️packages/🦀️rust/Cargo.toml`, `📦️packages/🟦️typescript/` (tsconfig)

Neither needed edits: forms' `Cargo.toml` has no `[package.metadata.semio.storybook]` block and no
`app = "…"` metadata lines (unlike cad's), and forms' `📦️packages/🟦️typescript/` directory has no
`tsconfig.json` at all (only `package.json`/`📋️project.json`/`📜️script.ts`/`📦️index.ts`) — confirmed by
listing the directory before declaring this a non-issue, not assumed. Grepped both files present for
the literal `🎛️apps` string: 0 hits.

### Deletion

`✏️s/🔌️plugins/📋️forms/🎛️apps/` removed in full (it was the plugin's only app) once every real file had
a real destination and `glue.rs` no longer referenced it. Also contained a stray top-level
`🎛️apps/🦀️component.rs` (a one-line doc-only marker, never `#[path]`-mounted anywhere) — deleted with
the rest.

## Incidents

**The emoji-typo trap (recipe step 11) was hit twice** while authoring this packet's new `glue.rs`
regions and one throwaway scratch file — "🏅️standards" (Latin) vs "🏅️标准" (Chinese, same meaning)
silently diverge:

1. A `Write` call meant to create a scratch placeholder file used a hand-typed `../` path segment that
   included the wrong Chinese glyph; the `..` collapsed the segment away before the file landed, so no
   wrong directory was actually created — caught by an immediate `find`/`ls` check, cleaned up the stray
   file.
2. The FULL new `glue.rs` `✏️Editor`/`👁️Viewer` regions (98 `#[path]` attributes) were typed by hand from
   memory instead of built from the saved base-path string, and every single one carried the wrong
   Chinese glyph — this DID create real content (in the glue file text, not a wrong directory, since
   `#[path]` strings are just string literals). Caught immediately by the recipe's own Python
   path-resolution verification script (0 of 41 `🏅️标准` paths resolved). Fixed with a scripted
   find-and-replace (`str.replace`) across the whole file rather than hand-editing each of the 41
   occurrences, then re-verified with the same script (0 missing, all 101 paths resolve).

Neither incident reached a committed or reported-done state — both were caught by verification before
moving on. Documented here per this ticket's own trap warning, and as a live confirmation that the
verification step (not just care while typing) is what actually catches this class of defect.

## SDK gaps found (already known, reconfirmed; nothing new)

1. **`EditorBuilder` has no `.example(...)`/`.workflow(...)`** (contract §2.4 split, w0-f-report Gap 4)
   — forms' pre-migration manifest had three `.example(...)` calls (`default`/`onboarding`/
   `building-component`) and one `.workflow("forms", "Forms", "data")` call; all four dropped, not
   ported, documented inline at `create_forms_app`'s own doc comment. The subset's own `📚️examples/🎬️demo`
   facet (already real, pre-existing) is the likely intended replacement mechanism.
2. **`new_app_with_registry`/`assert_declared_actions_bridge_to_commands` still take `fn() -> App`**
   (w0-f-report Gap 3, confirmed still true) — `create_forms_app` now returns `AppDefinition`, so a
   local `forms_manifest_for_testkit() -> App { App { definition: create_forms_app(), examples:
   Vec::new() } }` wrapper is used, same pattern the pilot used.
3. **Canonical `assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect` (w0-f Gap 2,
   confirmed present in the SDK, contract §2.5) were NOT wired into this packet's own test module.**
   This is a real gap in this packet's own work, not the framework's: given the cargo-check blocker (see
   Verification below) never resolved during this session, the two functions could not be exercised
   against the real `FormsViewer`/`FormsPlayApp` pair to confirm they compile against this packet's
   concrete types before committing to using them; the root viewer test module instead asserts the same
   two properties by hand (`def.role == AppRole::Viewer`, `def.dialect == FORMS_DIALECT.into()`,
   `<FormsViewer as ArtifactViewer>::DIALECT == FORMS_DIALECT`). Swapping in the canonical
   `testkit::assert_viewer_never_mutates::<FormsViewer>()` /
   `testkit::assert_editor_and_viewer_share_dialect::<FormsPlayApp, FormsViewer>()` calls is a trivial,
   low-risk follow-up once `cargo test -p semio-s-plugin-forms` can actually run to completion.

## Outside-lease referrers

None found. Grepped the WHOLE repo for `apps::forms` (as a Rust module path) and the literal string
`🎛️apps/📋️forms` — every hit is inside a historical ticket's own scratch/report file
(`.🦑️repo/🎫️tickets/**`, all from tickets already closed before this one opened), never a real `.rs`
compile dependency from another plugin. Unlike cad's pilot (which had `🎪️demonstrator` importing
`cad::apps::cad::*` directly), nothing outside `📋️forms`'s own lease references its old app path.

## Verification

- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-forms --all-targets --keep-going`, output appended
  across four runs to `🧪️w2-p8-forms-cargo.txt`:
  - Every run: **0 errors anchored inside `📋️forms` files** (`grep -B2 -A8 "^error" … | grep -c
    "📋️forms"` reads 0 every time — confirmed explicitly, not assumed).
  - Run 1: 3 errors, all inside `semio-framework-plugin`'s own `🔌️plugin/🦀️component.rs` (missing
    `messages`/`report` fields on `AppFrame`, missing `snapshot_with_conflicts` method) — confirmed
    live-edited (`git status --porcelain` shows ` M`, `git log --date=iso` shows a commit at
    2026-08-16 12:10:56, today, for the peer MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS
    ticket per its own commit message).
  - Run 2: failure moved upstream to `semio-framework-os-kernel`'s `🏪️store/🦀️component.rs` (missing
    `edit_messages` field, `Conflict`/`HistoryConflict` type mismatch) — same live-edit fingerprint
    confirmed (`git status` ` M`, same day's commits).
  - Run 3/4: failure moved back into `semio-framework-plugin`'s own file with a different symptom
    (`AppCommand::SetMergePolicy`/`ResolveConflict`/`ReadConflicts` non-exhaustive match) — same file,
    still ` M`, still today — the churn is genuinely still in flight, moving between the two crates
    exactly as the pilot's report and the w0-f-report both predicted for this exact class of concurrent
    refactor.
  - This packet's own crate never got a chance to finish a full `--all-targets` pass to completion
    during this session purely because of this upstream churn — not because of anything in `📋️forms`'s
    own files, confirmed on every single run.
- `cargo test -p semio-s-plugin-forms` — not run to a pass/fail result; blocked by the same upstream
  compile failure (a crate that fails `cargo check` cannot be tested). Output (the same compile failure)
  captured in `🧪️w2-p8-forms-test.txt`.
- Repo-wide grep for `apps::forms` / `🎛️apps/📋️forms` (outside-lease referrers) — see above, 0 real
  hits.
- `policyViewerPurityBreaches` self-check (manual grep, `bun ./📜️script.ts policy` not run this
  session — see Handoff): 0 hits for `::editor::`, `.mutation(`, `Emit::mutations`,
  `artifact_mutations` anywhere under the `👁️viewer` tree.
- Glue path resolution: 101/101 `#[path]` attributes resolve on disk (recipe's own Python script,
  final run).

## Handoff

1. Re-run `cargo check -p semio-s-plugin-forms --all-targets --keep-going` and
   `cargo test -p semio-s-plugin-forms` once `semio-framework-plugin`'s live `AppCommand`/`AppFrame`/
   `snapshot_with_conflicts` churn (MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS ticket)
   settles — expected clean based on every run this packet saw (zero errors ever attributed to
   `📋️forms`'s own files across four consecutive runs, each catching the churn at a different point).
2. Once `cargo test` can run, swap the root viewer test module's hand-written role/dialect assertions
   for the canonical `testkit::assert_viewer_never_mutates::<FormsViewer>()` /
   `testkit::assert_editor_and_viewer_share_dialect::<FormsPlayApp, FormsViewer>()` (SDK gap 3 above) —
   low-risk, not done here only because the compile blocker made it unverifiable in this session.
3. `bun ./📜️script.ts policy` was not run this session (bun/nx invocation was judged lower priority than
   exhausting the cargo-check retry budget given the session's time budget); the manual grep above is a
   reasonable proxy for `taxonomy/viewer-purity` but not a substitute for the real policy run, which the
   next session touching this ticket should do before final close-out.
4. The viewer's Try window is deliberately a flat, non-interactive read of every step's questions today
   (no per-step wizard cursor, since `Config = NoConfig`) — a real follow-up (a small viewer-owned
   `Config`/`Command` pair carrying just `current_step_index`, dispatched through `ViewEmit`'s
   `config_mutations`, contract §2.2 explicitly allows this) could give the viewer the same step-by-step
   wizard feel as the editor's own Try window while staying strictly read-only (no artifact mutation
   ever possible). Not attempted here given the time budget; flagged, not silently skipped.

## Files touched

Created:
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` (75 files —
  55 real content files moved/rewritten + 5 new real `🟦️component.ts` twins created fresh: 2 windows +
  surface root, plus 20 pre-existing `📌️empty.md` scaffold placeholders left untouched)
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️标准/...` — none; see "Incidents" (typo caught and
  fixed before anything under a wrong directory was left on disk)
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` (5 real
  files — `🦀️component.rs`/`🟦️component.ts` at surface root, mode root, and the `▶️try` window; 14
  taxonomy facet dirs otherwise `📌️empty.md`, all pre-existing from the W1-E scaffold)

Edited:
- `✏️s/🔌️plugins/📋️forms/🦀️component.rs` (plugin root: `.editor()`/`.viewer()` wiring)
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️component.rs` (`FORMS_DIALECT`,
  `.document_codec::<EditorApp<…>>()`, doc fixes)
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
  (doc fix)
- `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/📦️glue.rs` (editor + viewer mount regions)

Deleted:
- `✏️s/🔌️plugins/📋️forms/🎛️apps/` (whole tree — the plugin's only app, plus a one-line doc-only
  top-level marker file)

Scratch (ticket folder): `🧪️w2-p8-forms-cargo.txt`, `🧪️w2-p8-forms-test.txt`.
