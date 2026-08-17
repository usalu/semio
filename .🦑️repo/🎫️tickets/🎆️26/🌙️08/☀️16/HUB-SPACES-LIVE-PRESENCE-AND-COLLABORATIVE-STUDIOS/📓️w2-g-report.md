# Lane 2-G report — fix the 15 pre-existing `engine::space::*` test failures

## Result

`cargo test -p semio-s-plugin-space --lib`: **198 passed; 1 failed** (baseline was 124 passed / 15
failed). All 14 fixable failures are green. The one remaining failure is a genuine, pre-existing
framework bug outside this lease (see "Blocked" below), left failing with a full diagnosis and a
`sharedFileRequest`, per the brief's explicit instruction to never force green by deleting/ignoring a
test. Final tail: `🧪️2-g-final2.txt`. (Total test count moved from 139→199 between baseline and now —
not a regression; lanes 2-A/2-B kept landing new `🗿️artifacts/🏠️home`/`🗿️artifacts/🪐️space` tests on the
live tree the whole time I worked, per the brief's own warning.)

## Root cause (one shared cause, as the brief predicted)

14 of the 15 failures shared one cause: `PluginBuilder::build_definition()`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:5167`) now hard-requires every
`App::builder(...)` id to parse via `semio_framework::parse_surface_app_id` as
`<artifact_kind>@<standard>/<subset>#<role>` (landed by the concurrent
`26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` ticket, commit `07873f842a`, hours before this
ticket started — confirmed via `git log --date=iso`). `⚙️engine/🪐️space` (the pre-existing "studio"
workflow engine) predates that convention: its own manifest id and every test-only synthetic
"other plugin" registration used bare slugs (`"studio"`, `"draw"`, `"puzzle5d"`, `"root-tool"`).

Fixing that panic then exposed **two more, previously-masked, genuinely separate bugs** in two of the
14 tests (`set_app_registrations_command_registers_app_and_surfaces_empty_document_apps_in_catalogue`,
`export_media_emits_download_effect_and_import_requests_file_open`) and confirmed a third, unrelated
pre-existing bug in a 15th test (`commit_checkpoint_round_trips_projection`) — all masked until now
because the crate could not link, so none of these tests had ever actually run past their setup code.

## Fixes

1. **`S_PLAY_APP_ID`** (`⚙️engine/🪐️space/🦀️component.rs:44`): `"studio"` → `"s.space.studio@1/*#editor"`
   (canonical, mirrors the contract freeze's own `s.space.home@1/*#editor` example). Fixes the 4 tests
   that call `create_space_app()` directly: `space_manifest_uses_studio_app_id` (assertion now compares
   against the constant instead of a hardcoded literal), `space_declares_expected_actions_and_examples`,
   `space_window_kind_actions_scope_editing_to_workflow`,
   `space_workflow_context_menu_stays_within_budget_with_destructive_tail`.

2. **`testkit::seed_app`** (same file, `pub(crate) mod testkit`): added a shared
   `pub(crate) fn test_surface_id(slug: &str) -> String`, built via
   `semio_framework::surface_app_id(&ArtifactDialect{artifact_kind: format!("s.{slug}"), standard:"1",
   subset:"*"}, AppRole::Editor)` — mirrors the framework's own `canonical_test_app_id` test fixture
   helper (`🔌️plugin/🦀️component.rs`). `seed_app` now builds its `App::builder(...)` id through this
   helper; `register_app_io` therefore registers under the canonical id. Every
   `SpawnApp{app_id: "draw"/"puzzle5d"/"shooting"}` literal that dispatches against that registry (not
   the ones that only round-trip op-text encoding) was updated to `test_surface_id("draw"/…)` so the
   registry lookup still hits: `⚙️engine/🪐️space/🦀️component.rs` (3 call sites),
   `🎮️commands/🧩️spawn-app/🦀️component.rs` (4 tests), `🎮️commands/🧩️delete-selection/🦀️component.rs` (1
   test). `open_instance`/`export_media`'s node lookups needed no further change — they read
   `plugin_id`/`app_id` off the bundled demo DSL document (framework-owned fixture, document DATA not a
   manifest id, never subject to `parse_surface_app_id`), not the registry.

3. **`set-active-panel-tab`'s hand-built `AppDefinition`** (own test, not `seed_app`): threaded
   `testkit::test_surface_id("root-tool")` through `App::builder(...)`, the `os_app_registration`/
   `workflow_palette` lookups, and the catalogue-tree JSON assertion. **Second bug found here**: the
   test blanked `app_json["document"] = json!([])` to simulate an empty breadcrumb, but
   `AppDefinition`'s wire field is `breadcrumb` — `.document(...)` is only the *builder method* name,
   `#[serde(rename_all = "camelCase")]` leaves the single-word field `breadcrumb` unchanged on the wire.
   The mutation was a silent no-op; the real `"breadcrumb": ["root-tool"]` (from the `.document(...)`
   call) survived untouched, so the app landed nested one level under a `"root-tool"` breadcrumb segment
   instead of at the catalogue root — proven with a temporary `[DEBUG]` trace of `entry`/`registration`
   inside `build_catalogue_tree` (removed before finishing). Fixed: `app_json["breadcrumb"] = json!([])`.

4. **`commit_checkpoint_round_trips_projection`** (unrelated pre-existing bug, own test): constructed a
   bare `VcsArtifactApp::new(SpaceApp::default())` and called `commitCheckpoint` with ZERO prior edits.
   `ArtifactStore::dispatch_inner`'s `CommitCheckpoint` arm
   (`🏪️store/🦀️component.rs:4789-4793`, last touched 2026-08-14 — two days before this ticket, confirmed
   via `git log --date=iso`, genuinely unrelated to the canonical-surface-id migration) rejects an empty
   checkpoint. `SpaceApp` has no `genesis()` override and its `initial_snapshot()` is genuinely empty, so
   `applied_edit_ids` is `[]` at construction — this test, as written, could never have produced a
   non-empty checkpoint. Fixed by spawning a "draw" node before committing, mirroring the sibling
   `checkout_checkpoint_restores_projection` test right below it.

5. **`export_media_emits_download_effect_and_import_requests_file_open`**: **third bug found here**.
   The test dispatched `format: "stdio.dwg"` and registered its export/import handlers under
   `format_artifact_kind = "dwg"`. Stdio's format registry now keys every `FormatDescriptor` by its full
   schema `representation.id` (`kind_id == short_id`, no alias —
   `🗄️stdio/🗿️artifacts/🖊️dwg/🧬️schema/📜️artifact-definition.json`), so neither bare string resolves.
   Printed every registered `dwg` descriptor at runtime (temporary `[DEBUG]`, removed) to find the real
   id rather than guessing from the JSON — it is
   `"s.stdio.dwg.standard.ac1018.representation.document"` (the `ac1024`-is-the-only-live-standard doc
   comment in `🗄️stdio/🗿️artifacts/🖊️dwg/🦀️component.rs` is about the encode/decode `io_registry`, a
   *different* registry from the format-descriptor catalog this test reads). Introduced a local
   `DWG_FORMAT_ID` const and used it consistently for the dispatch payload, the handler registrations,
   and the `SetPendingImport.format` assertion (all `🗄️stdio/**`-adjacent literals, never touching
   `🗄️stdio/**` itself — same pattern lane 2-0 already used for the `stdio_format_descriptors()` rename).

## Blocked — sharedFileRequest

**`two_instances_converge_on_disjoint_edits_via_backbone`** (`⚙️engine/🪐️space/🦀️component.rs`) still
fails:

```
pump b: Fault { origin: Module, code: FaultCode("module.vcs"), severity: Error, message:
"validation failed: change change-b052db4007cd5fd3 has an invalid edit reference edit-3486f95a59709941" }
```

- **First bug in this test** (fixed, within my lease): the doc comment and setup claimed "two
  independent instances start from the same deterministic demo projection" and dispatched a rename
  against `demo_space_projection().graph.nodes.first().id` — but `assert_two_instances_converge`'s
  `paired_apps`/`new_app::<A>()` (`🔌️plugin/🦀️component.rs:5771-5778`) construct each instance from
  `A::initial_snapshot()`, which for `SpaceApp` is genuinely EMPTY (no `genesis()` override). That id
  could never exist in either instance; the "missing-target" guard correctly rejected it. Rewrote the
  test so both commands are `SpawnApp` (A spawns "draw", B spawns "shooting") — this is what `document.id`
  in `paired_apps` actually supports, and still proves the intended property (disjoint concurrent
  creates from independent instances converge over the backbone).
- **Second bug, NOT fixable from this lease**: with the corrected setup, dispatch succeeds on both
  sides, but `instance_b`'s `commitCheckpoint` (`assert_two_instances_converge`'s own "pump b" step,
  `🔌️plugin/🦀️component.rs:5852`) now fails validation inside `🏪️store/🦀️component.rs`'s
  `validate_durable_history`/`CommitCheckpoint` handling: a `Change` ends up referencing an edit id that
  isn't present in the local `envelope.vcs.edits`. **Confirmed via two independent isolated
  (`--test-threads=1 --exact`) reproductions that this is not specific to "shooting" or to using two
  different plugins**: substituting a second `draw` spawn at B (different position, same plugin)
  reproduces the identical fault. This is a genuine, pre-existing bug in the backbone-relay +
  checkpoint path for CREATE-type mutations (`WorkflowMutation::AddNode`), never exercised by any test
  before this lane — the ORIGINAL, differently-broken test always failed *earlier*, at `instance_b`'s
  own `dispatch_typed` (missing-target), so this checkpoint code path was never reached until fix #1
  above let the test actually run its full body for the first time.
- **File/region**: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`, the
  `CommitCheckpoint` arm of `ArtifactStore::dispatch_inner` (~line 4789) and/or
  `validate_durable_history` (~line 2650) — root cause not fully isolated (would need instrumenting
  private `ArtifactStore` fields I cannot reach from a `#[cfg(test)]` block in another crate). Both
  `🏪️store/**` and `🔌️plugin/**` are under `🧰️framework/**` — forbidden to this lane.
- Left failing per the brief ("never delete/`#[ignore]` to force green; leave failing + name exactly
  why"). The failing test's own doc comment (`⚙️engine/🪐️space/🦀️component.rs`, right above the `#[test]`)
  carries this same diagnosis for whoever picks up the `sharedFileRequest`.

**sharedFileRequest**: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — someone with that
lease should reproduce with `cargo test -p semio-s-plugin-space --lib
engine::space::component::tests::two_instances_converge_on_disjoint_edits_via_backbone --
--test-threads=1 --exact` and trace why a remotely-ingested `AddNode` edit's id ends up referenced by a
`Change` without a matching `envelope.vcs.edits` entry when `commitCheckpoint` runs on the receiving
side of a `MemoryBackbone` pair.

## Changed files

- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🧩️spawn-app/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🧩️delete-selection/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🧭️set-active-panel-tab/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🖼️export-media/🦀️component.rs`

(`⚙️engine/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs` was touched with a temporary debug trace during
diagnosis and restored to its original content before finishing — `git status` shows it unmodified.)

All five are inside the granted lease (`✏️s/🔌️plugins/🪐️space/⚙️engine/**`). No forbidden path
(`🗿️artifacts/🏠️home/**`, `🗿️artifacts/🪐️space/**`, `🧰️framework/**`, `✏️s/🔌️plugins/🗄️stdio/**`) was
edited.

## Attribution of transient failures encountered while working

Multiple `cargo test`/`cargo check` runs during this lane hit red compiles from concurrent peer work,
all confirmed via `git log --date=iso` / `git status --porcelain` to be live, uncommitted edits by other
sessions, never mine:
- `🗿️artifacts/🏠️home/**`, `🗿️artifacts/🪐️space/**` (lanes 2-A/2-B, per the brief) — repeated
  struct-field/import churn in `editor::space_index::commands::*`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (`store::SpaceConflict` briefly
  unresolved) — a different concurrent session actively editing `🏪️store` (confirmed `M` in
  `git status`).
Each cleared on retry within a few minutes; none required any action from this lane beyond waiting.

## Logs (this ticket folder)

`🧪️2-g-baseline.txt` (starting 124/15), `🧪️2-g-blocked-1.txt` (transient peer-churn compile failure,
for the record), `🧪️2-g-final.txt` (intermediate: 197/2, before the DWG format id was corrected),
`🧪️2-g-final2.txt` (final: 198/1).

## What is NOT done

- `two_instances_converge_on_disjoint_edits_via_backbone` still fails — a genuine, pre-existing
  framework bug outside this lease's lease boundary (see "Blocked" above and the `sharedFileRequest`).
- Everything else in this lane's scope (the 15 `engine::space::*` failures named in the brief) is done.
