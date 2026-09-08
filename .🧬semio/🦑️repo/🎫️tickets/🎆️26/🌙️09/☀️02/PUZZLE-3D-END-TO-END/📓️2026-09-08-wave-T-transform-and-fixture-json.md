# 🧲️ Wave T — Gumball Transform Bracket + `setFixtureJson` Removal

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-T. Date 2026-09-08/09.

## 📊️ Counts

| | before | after |
|---|---|---|
| declared `action_interactive_job` routes (puzzle3d) | 63 | **62** |
| `InteractiveJobClassification::Migrated` | 60 | **62** |
| `BatchOnlyPendingRewrite` | 3 (`transformBegin`, `transformEnd`, `setFixtureJson`) | **0** |
| `PUZZLE3D_RETAINED_TOOL_IDS` | 60 | **62** |
| `bounded_first_step_tool_proofs!` `tools:` | 60 | **62** |
| `PUBLICATION_CONTRACTS` | 60 | **62** |
| publication-authority fixture migrated routes (Puzzle3dPlayApp) | 60 | **62** |

Puzzle3d is now 100 % `Migrated`: no route is rejected by
`validate_ui_dispatch_classification` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`,
fault `interactive-job.not-ui-safe`) any more.

## 1️⃣ Gumball transform bracket

`transformBegin` / `transformEnd` are now honest `Migrated` + `HostOnly` no-op brackets:

- both added to `PUZZLE3D_RETAINED_TOOL_IDS`, to the `bounded_first_step_tool_proofs!` `tools:` list
  and to `PUBLICATION_CONTRACTS` with `lanes: &[ArtifactToolPublicationLane::HostOnly]` (the
  `worldPointerDown` precedent);
- `build_tool_job` already routed `"worldPointerDown" | "transformBegin" | "transformEnd"` to
  `crate::retained_command::NoopPuzzleCommandWork` (wave S) — unchanged;
- both added to the `Some(1)` early return in `puzzle3d_retained_extent` next to `worldPointerDown`;
- `.action_interactive_job(…, BatchOnlyPendingRewrite)` → `Migrated` for both;
- `transformEnd`'s manifest declaration moved from `.mutation(...)` to `.view_action(...)` (same
  shape as `transformBegin` and `worldPointerDown`): it commits nothing, so declaring it a mutation
  was dishonest. `transformEnd` was consequently dropped from `puzzle3d_action_document_intent` and
  from the document-items arm of `puzzle3d_retained_extent`.

### 🗑️ Deleted dead scratch session

The `Puzzle3dPlayApp` gumball scratch-commit session could never work: `with_puzzle3d_app_for`
constructs a fresh `Puzzle3dPlayApp::default()` per call and restores only `config.fill_checkpoint`,
so nothing carried from `transformBegin` to `transformEnd`. Meanwhile the host
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`)
keeps the drag entirely local and dispatches ONE absolute start→end delta on drag end, which
`Puzzle3dScaleWork` already commits directly. Removed:

- fields `transform_drag_active`, `transform_base`, `transform_scratch`, `preview_seq` (+ their
  `Default` initialisers);
- methods `begin_transform_session`, `clear_transform_session`, `transform_drag_tick`,
  `commit_transform`, `gesture_preview` (an inherent `#[allow(dead_code)]` method, NOT an
  `ArtifactEditor`/`ArtifactApp` trait method — verified by reading the `impl Puzzle3dPlayApp`
  inherent block — so it was deleted outright rather than stubbed) and the whole
  `//#region 🔖️GesturePreview` block;
- the `transform_drag_active` gate plus the `transformBegin`/`transformEnd` arms in
  `handle_action_impl`, and the now-unused `transform_object_ids` / `transform_volume_ids` bindings;
- the scratch branch of `render_fixture` (now a plain projection decode, with an honest docstring);
- `puzzle3d_transform_drag_scope()` (its only two callers were the deleted tick/commit);
- `ctx.app.clear_transform_session();` in `🎮️commands/🧰️set-active/🦀️.rs`;
- the stale `//#region 🔖️PlayApp` header comment block describing the scratch-commit session, and
  the stale `with_puzzle3d_app_for` doc comment — both rewritten to describe the real design (one
  absolute delta per drag, committed directly; begin/end are host-only brackets).

`mesh_selection_ids`, `puzzle3d_apply_translate/rotate/scale`, `resolve_puzzle3d_attractions`,
`puzzle3d_rederive_moved_attractions` and `puzzle3d_operations_from_fixture_change` all keep real
callers (the `🎮️commands/{🚀️translate,🔄️rotate,📏️scale}-selection` free functions and the generic
delta bridge) and were left alone.

## 2️⃣ `setFixtureJson` removed from puzzle3d

Repo-wide grep for `setFixtureJson|set_fixture_json|set-fixture-json` found **no dispatcher in any
TypeScript, storybook story or fixture** for the 3d route, and the argument (a whole fixture;
128 755 bytes for Nakagin) cannot fit the shared 8 192-byte `PUZZLE_COMMAND_RAW_BYTES` cap
(`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:10`). No genuine consumer exists, so the
route was removed rather than rewired. Removed:

- the `ActionDefinition::bounded_catalog("setFixtureJson", …)` declaration and its
  `.action_interactive_job("setFixtureJson", …)` row;
- the `SetFixtureJson` row in `puzzle3d_command_variants!` and `"setFixtureJson"` in
  `Puzzle3dCommand::TOOL_JOB_IDS`;
- the `dispatch_puzzle3d_action` arm and the `set_fixture_json` import;
- the `#[path] pub mod set_fixture_json;` declaration in `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️.rs`;
- the whole directory `…/✏️editor/🎮️commands/🧪️set-fixture-json/`;
- `"setFixtureJson"` from `puzzle3d_action_document_intent`;
- the `batch-only-pending-rewrite` group holding it in the publication-authority fixture.

Untouched on purpose: `setActiveExample` and the `📚️examples` fixtures (the working way to load an
example); puzzle**5d**'s own `setFixtureJson` (already `Migrated`, out of this wave's scope); the
`🧪️set-fixture-json` entry in `🧰️framework/…/📚️library/🔣️taxonomy.json`
`semanticDirectoryMemberKinds/members-of-commands/memberNames`, because 5d still provides that
member name on disk.

No audit needed updating: `grep setFixtureJson|set_fixture_json|Fixture JSON|Fixture Json` in the
root `📜️script.ts` and in `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` returns
zero hits, and the `🗣️terminology/🦀️.rs` file carries no fixture-json label. No other invariant was
weakened.

## 🧪️ Tests

In `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`:

- `transform_begin_and_end_real_dispatch_is_already_the_noop_the_work_emits` →
  `transform_brackets_are_migrated_host_only_routes_that_complete_empty`: asserts, for both ids,
  membership in `PUZZLE3D_RETAINED_TOOL_IDS`, exactly one manifest declaration,
  `InteractiveJobClassification::Migrated`, `lanes == [HostOnly]`, and an empty
  `puzzle3d_retained_reduce` emit.
- `gumball_transform_session_commits_once_on_end` →
  `gumball_gesture_commits_one_absolute_delta_between_its_host_brackets`: the real gesture
  (`transformBegin` → one absolute `translateSelection` delta → `transformEnd`) moves the object by
  exactly that delta, one `undo` restores it, and a second gesture starts from the restored pose.
- the three `//#region 🔖️GesturePreview` tests deleted (they drove the removed
  `transform_drag_tick` / `commit_transform` / `gesture_preview` API directly).
- `gumball_translate_drag_coalesces_into_one_edit` kept; its stale "compat path without
  transformBegin" comment corrected.
- `transform_lifecycle_is_an_explicit_bounded_retained_boundary` kept as-is — its `build_tool_job`
  route string is unchanged.

## ✅️ Verification actually run

- `rustfmt --edition 2021 --emit stdout <file> > /dev/null` exits 0 on all four edited Rust files:
  the editor `🦀️.rs`, `🗿️artifacts/🧊️3d/🦀️.rs`, `🎮️commands/🧰️set-active/🦀️.rs`, and
  `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.
- Grep sweep: `transform_drag_active|transform_scratch|transform_base|commit_transform|transform_drag_tick|begin_transform_session|clear_transform_session|gesture_preview|puzzle3d_transform_drag_scope|setFixtureJson|set_fixture_json` returns **zero hits** under `🗿️artifacts/🧊️3d/`.
- `//#region` / `//#endregion` balance is unchanged from HEAD (the apparent delta at HEAD came from a
  doc comment quoting a region name, not a real marker).
- `🔍️verify-wave-T-publication-authority.ts` (in this ticket folder) replicates every clause of the
  puzzle-js audit's `ownerOracle` for `Puzzle3dPlayApp`. Result:
  `declared 62, migratedInSource 62, batchOnlyInSource [], fixtureMigrated 62,
  manifestBijection true, statusesAgree true, retainedExact true, proofsExact true,
  contractsExact true, hostOnlyExclusive true, transformLanes [HostOnly, HostOnly],
  missingOtherAnchors [], negativeStoreAnchors [], ownerTypeAlias true,
  factoryNamedInProofs true, noBatchOnlyGroupLeft true`.

### ❌️ `publication-authority-audit` still fails — pre-existing, not this wave

`bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts publication-authority-audit Puzzle3dPlayApp`
→ `Puzzle3dPlayApp publication authority diverged from the fixture`.

Cause, isolated: the audit's Puzzle3d branch requires two locale/terminology anchors in the editor
source that a peer removed when moving locale/terminology into the OS `ViewModel`:

- `matches!(value.as_str(), "en" | "en-US" | "de" | "de-DE")`
- `matches!(value.as_str(), "native" | "reuse")`

Proof it predates this wave: `git show HEAD:<editor 🦀️.rs> | grep -c en-US` → `0`, and that file's
last commit is `de617a7c17` (2026-09-08 23:25:57 +0200), the peer's terminology move. The Ajv schema
validation and the fixture `fixtureOracle` both PASS on the edited fixture (the audit throws later,
at `ownerOracle`, line 207). The audit is red for all three owners right now, including the two this
wave never touched:

- `Puzzle2dPlayApp`: missing `Ok(puzzle2d_dispatch_emit(command, snapshot.0.clone(), config, &selection, None))`
  and `Ok(puzzle2d_dispatch_emit(command, before, config, interaction.selection(PUZZLE2D_INTERACTION_DOMAIN), doc.operation_optional().cloned()))`
  (peer's `view_state: &ViewModel` signature change).
- `Puzzle5dPlayApp`: all its source anchors and its reserved-route guard order are present, so its
  divergence is in the contract-lane comparison — also not this wave.

Resolving those anchor expectations belongs to whoever owns the terminology/`ViewModel` migration;
relaxing them here would weaken invariants outside this wave's scope.

### ❌️ `verify interactivity tool-jobs` aborts before reaching puzzle

`bun ./📜️script.ts verify interactivity tool-jobs` →
`[verify interactivity tool-jobs shared-action-fixture] recordTutorial arms before the accepted
retained route.` (`📜️script.ts:6081`). `toolJobSharedFrameworkActionFixtureRun(root)` is the FIRST
call inside `toolJobCoverageRun` (`📜️script.ts:6755`), so no puzzle assertion was reached at all.
The file it inspects,
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`,
is staged-modified by the same peer commit `de617a7c17` and was never touched by this wave.

## ⚠️ Could NOT be verified

- **`cargo check` / `cargo build` / `cargo test` were NOT run at all** (machine at load ~60 with swap
  full; the coordinator runs one consolidated private check after this wave). Nothing here is
  compile- or runtime-confirmed. The only Rust instrument used was `rustfmt` (a parse check), plus
  reading and grep.
- **Pre-existing fixture divergence, NOT introduced and NOT fixed here**:
  `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json` declares only 2
  `toolIds` (`openAddObjectDialog`, `worldPointerDown`) while `PUZZLE3D_RETAINED_TOOL_IDS` now has
  62. Two tests assert set equality against that list —
  `retained_publication_contracts_are_an_exact_nonempty_tool_bijection` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:53`)
  and `language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`
  (`🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs:226`) — so both were already failing at 60 vs 2 and
  still fail at 62 vs 2. `🖐️5d`'s fixture has the same 2-vs-N staleness, so this is a systemic
  puzzle3d+puzzle5d fixture repair, not a wave-T regression; it was deliberately left to the ticket
  owner. (The same fixture's `evidenceToolIds`, `semanticCursors` — `transformBegin`/`transformEnd`
  as `boundedGestureBoundary`/`publishEmpty`/`closeOwner` — and `transformBeginBounded` /
  `transformEndBounded` / `transformLifecycleCancelFault` vectors already describe both brackets as
  retained routes expecting `terminalEmpty`, which independently corroborates this wave's design.)
- `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` still carries a `setFixtureJson` action row for the 3d window
  kind. It is a generated plugin-descriptor snapshot (it embeds `descriptorSha256`), regenerated by a
  wasm/plugin build, so it was deliberately NOT hand-edited; it stays stale until the plugin is
  rebuilt.
- No runtime/console confirmation of the gumball drag in the running app — no dev boot was performed.

## 📁️ Files changed

| path | change |
|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | brackets → Migrated/HostOnly/retained/proofs/extent; scratch session, `gesture_preview`, `puzzle3d_transform_drag_scope` deleted; `setFixtureJson` route removed; PlayApp + `with_puzzle3d_app_for` docs rewritten (7 402 → 7 277 lines) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️.rs` | `#[path] pub mod set_fixture_json;` removed |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active/🦀️.rs` | `clear_transform_session()` call removed |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-fixture-json/🦀️.rs` | **deleted** (directory removed) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 2 tests rewritten, 3 `GesturePreview` tests deleted, 1 stale comment fixed |
| `✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json` | `transformBegin`/`transformEnd` moved into the migrated `host-only` group; the `setFixtureJson` batch-only group deleted; Puzzle3d now 4 groups / 62 routes / 0 blockers |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` | one stale comment ("onto the transform scratch") corrected — comment only, no behaviour |
| `.🧬semio/🦑️repo/🎫️tickets/…/PUZZLE-3D-END-TO-END/🔍️verify-wave-T-publication-authority.ts` | **added** — audit-oracle replication used as evidence above |
| `.🧬semio/🦑️repo/🎫️tickets/…/PUZZLE-3D-END-TO-END/📓️2026-09-08-wave-T-transform-and-fixture-json.md` | **added** — this report |
