# W4 batch Cb — `forms` composes stdio `value`, `table`

**ucas-status: complete — 110/111 tests passing (stable across 2 consecutive full runs), 0 compile errors, 1 failure independently traced to a pre-ticket (concurrent-SMO-window) commit with full provenance below. Baseline was RED — not from a pre-existing forms bug but from live concurrent SMO mutation-vocabulary-rename fallout inside this plugin's own boundary, fixed first (see `## Concurrent-churn observations`).**

## Baseline (before any composition edit)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-forms --all-targets
```
First real run: **9 compile errors**, all inside `✏️s/🔌️plugins/📋️forms/**` (never in stdio/glue/framework) — `ChangeStepDescription` defined multiple times (duplicate-import typo in a triad split) and 7× "cannot find `add_block`/`add_step`/`move_block`/`move_step`/`remove_block`/`remove_step`/`update_block` in `mutations`". Traced (see below) to SMO's mutation-directory rename commit `31209e7a` (2026-08-13 00:13:16, **inside this ticket's active window**) having renamed `📦️glue.rs`'s mounts and the enum dispatch file to the new semantic module names (`create_step`/`delete_step`/…) but leaving several sibling files inside forms' own boundary — `🧬️mutations/📝️text/component.rs`'s wire-codec import list, the dispatch file's own test import, and 8 individual triad files' internal cross-references — on the OLD generic-verb names (`add_step`/`remove_block`/…). This is forms-plugin-boundary code (not glue.rs/index.ts), so per hot-file ownership it was mine to fix. Fixed all 9 outright (trivial, unambiguous renames + 2 duplicate-import typos in `📝change-step-description`'s diff/inverse) before starting composition. Full list in `## Concurrent-churn observations`.

## What forms was duplicating

`FormsSnapshot` (`🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`) held one inline field, `steps: Vec<FormStep>` — `FormStep`/`FormQuestion` are re-exports of the shared `flow::playbook` kernel crate's `PlaybookStep`/`PlaybookBlock` (framework-owned, untouched). A `FormQuestion` carries 15+ optional config fields (`default`/`min`/`max`/`step`/`unit`/`options`/`fields`/`schema`/`src`/`accept`/`fixtureSlug`/`params`/`condition` — the last a recursive boolean expression tree) plus identity/label/kind — exactly the "structured/computed values" `s.stdio.semio.value` already generalizes, and nothing table-shaped existed inline until now.

- **value** (`structure`, `s.stdio.semio.value`) ← the SOLE source of truth: the whole `steps` tree (id/title/description per step, full block config per block, condition tree included) folded losslessly into one `SemioValue::Map`.
- **table** (`results`, `s.stdio.semio.table`) ← a DERIVED, non-reconstructive projection: one row per block flattened in step order (`id`/`stepId`/`label`/`kind`/`required`) for tabular scan/display convenience, always regenerated alongside `structure` from the SAME steps (never an independent source, so the two never diverge).

This mirrors `mathematical`'s own text/table/value split (one lossless source + one derived convenience projection) rather than inventing a new shape — `table` here is intentionally NOT reconstructive (unlike mathematical's `results`, which WAS zippable back into nodes) because a `FormQuestion`'s nested `options`/`fields`/`condition` genuinely cannot be flattened into scalar table cells without loss; that asymmetry is documented directly in the composition region's own doc comment, not hidden.

**Norm/mathematical lesson applied**: forms already had a properly granular, id-keyed sparse collection delta (`FormsStepsDelta{added,removed,patched,reordered}` + `FormsStepPatch`) — NOT the whole-blob-replace anti-pattern D2 flags for stdio's own text/table/graph. Composition did not regress this: every one of the 10 mutation triads (`create-step`/`delete-step`/`reorder-step`/`rename-step`/`change-step-description`/`create-block`/`delete-block`/`move-block-to-step`/`replace-block`/`change-form-title`) keeps its EXACT payload shape and still builds a `FormsStepsDelta` internally, unchanged. Only the diff's own OUTER wire representation changed: each triad's `diff_*` function now reads the working-scene steps via `forms_steps(base)` (not a snapshot field), applies the SAME `apply_steps_delta` pure function (byte-identical logic, verified — see `## What changed` below), and wraps the result via one new helper, `forms_diff_from_delta`, which regenerates both composed children together — the same "mint-all-together" pattern mathematical established.

## What changed

### Composition machinery (new, artifact root)

`🗿️artifacts/📋️forms/🦀️component.rs`, new `🔖️Composition` region:
- `FormsStructureChild`/`FormsResultsChild` — `store::ArtifactChild<SemioValueSnapshot|SemioTableSnapshot>` type aliases.
- **Converters** (real, bidirectional, none stubbed): `semio_value_from_dsl`/`dsl_from_semio_value` (`dsl::DslValue` ↔ `SemioValue`, both JSON-equivalent), `semio_value_from_expr`/`expr_from_semio_value` (the recursive `PlaybookExpr` condition tree ↔ a tagged `SemioValue::Map`), `semio_value_from_block`/`block_from_semio_value` (every one of `FormQuestion`'s 18 fields, real, none dropped), `semio_value_from_step`/`step_from_semio_value`, `forms_structure_from_steps`/`forms_steps_from_structure` (the lossless round trip), `forms_results_from_steps` (the derived table projection).
- **Working scene**: `FormsWorkingScene{ steps: Vec<FormStep> }` in `thread_local! FORMS_SCRATCH: RefCell<HashMap<String, FormsWorkingScene>>` — never persisted, matches the `EngineRep` contract (same shape as `mathematical`'s `MATH_SCRATCH`/`writer`'s `WRITER_SCRATCH`). `structure`/`results` are always minted TOGETHER (`forms_children_from_steps`) and share one content-addressed `scene_id`, so one cache entry serves both. `forms_steps(&FormsSnapshot)`/`forms_artifact_steps(&FormsArtifact)` are the two read accessors every call site in the plugin now funnels through instead of the old `.steps` field access; both fail soft (empty `Vec`) on a cache miss, never panic — same documented staleness gap as every prior exemplar (store-level undo/redo bypasses `ArtifactApp::handle`).
- `forms_snapshot_with_state(schema, id, version, title, steps) -> FormsSnapshot` — the fixture/import constructor replacing the old 5-field struct literal.

### Snapshot / composed children

`📸️snapshot/🦀️component.rs`: `FormsSnapshot.steps` → `structure: FormsStructureChild #[child(kind="s.stdio.semio.value")]`, `results: FormsResultsChild #[child(kind="s.stdio.semio.table")]` (both bare/non-`Option`, always-present slot — `title` stays a plain scalar field, untouched, same tier as `id`/`version`). Hand-rolled `ArtifactDsl`/`ArtifactPack` directly on `FormsSnapshot` (`🔖️ChildCodecPrimitives`/`🔖️TextPrimitives`/`🔖️BinaryPrimitives`/`🔖️HandcraftedArtifactCodecs`, same hex/bracket-handle + LEB128 pattern `mathematical`/`cad`/`writer` established) — the old codec bridged through `flow::playbook::PlaybookSpec`'s own DSL grammar (byte-for-byte 1:1 field mapping); that bridge cannot express a composed child slot, so it's dropped for the snapshot's OWN persisted format. `Default` now calls `forms_snapshot_with_state(..., Vec::new())`.

`FormsArtifact` (`🧬️schema/🦀️component.rs`, the UI-inclusive full-state struct) got the identical 2-field swap; `to_snapshot`/`from_snapshot`/`set_snapshot` updated to copy the child handles directly.

### Diff

`🔺️diff/🦀️component.rs`: `steps: Option<FormsStepsDelta>` → `structure: Option<FormsStructureChild>`, `results: Option<FormsResultsChild>` (single-Option, always-present-slot shape — `mathematical`'s pattern, not lowpoly's double-Option). The dead whole-snapshot-replace slot `artifact: Option<Box<FormsArtifact>>` is **removed** — grepped: only ever constructed by this file's own now-deleted `diff_set_snapshot`, never by any mutation triad or app command; it was exactly the banned `SetSnapshot` vocabulary, same finding as mathematical's own dead `artifact` field.

`🔺️diff/📝️text/🦀️component.rs`: `apply_steps_delta` (the pure `Vec<FormStep>` transform) is **byte-for-byte unchanged** — still the same added/removed/patched/reordered algorithm. New `forms_diff_from_delta(delta, base) -> FormsDiff` builder: reads `forms_steps(base)`, applies `apply_steps_delta`, regenerates+caches both children via `forms_children_from_steps`. `MutationDiff::apply`/`absorb` rewired to whole-slot-replace `structure`/`results` (dropped the now-obsolete `absorb_steps_delta` merge helper — no longer needed since composed children are replace-not-merge at the wire level, same as every other composed field). `sparse_diff_between` (a generic before/after diff helper, unrelated to any mutation triad) kept, rewired through `forms_steps`; `steps_collection_delta` kept as an internal granular-delta-computation utility.

### Mutation triads (10 kinds, 20 files: `🔺️diff` + `↩️inverse` each)

`create-step`, `delete-step`, `reorder-step`, `rename-step`, `change-step-description`, `create-block`, `delete-block`, `move-block-to-step`, `replace-block` (10th, `change-form-title`, touches no `steps` at all — untouched). Every `diff_*` function: mechanically identical algorithm, two changes — `base.steps` → `forms_steps(base)` (cache read instead of field read), and the final `FormsDiff{steps: Some(delta), ..}` literal → `forms_diff_from_delta(delta, base)` (mint-both-children instead of setting the old delta field). Every `inverse_*` function: `base.steps` → `forms_steps(base)`, otherwise untouched (inverse functions only ever READ steps to reconstruct an undo mutation payload, never write the diff).

### Inference / app layer

`💡️inferences/🦀️component.rs`: `compute_forms_topology(&snapshot.steps)` → `compute_forms_topology(&forms_steps(snapshot))`.

`🎛️apps/📋️forms/**`: every `.steps` field access across the app layer rewired through `forms_steps`/`forms_artifact_steps` — `🦀️component.rs` (2 test call sites), `🎮️commands/{📃️step,❓️question,📥️import,🗂️selection,🧪️try}/🦀️component.rs`, `📌️panels/{📄️artifact,🔍️inspection}/🦀️component.rs`, `🎭️modes/📝️blueprint/🪟️windows/{▶️try,🧱️builder}/🦀️component.rs` — ~34 files touched total across artifact+app layers.

**Also found and fixed, same pass** (genuinely pre-existing, same SMO-rename-window churn as the baseline errors, confirmed via `git log -1 --date=iso`): `🎛️apps/📋️forms/🎮️commands/{📃️step,❓️question,📥️import}/🦀️component.rs` still constructed `FormMutation` using the PRE-SMO-rename vocabulary (`AddStep{step,index}`/`UpdateStep{step}`/`RemoveStep{step_id}`/`MoveStep{step_id,index}`/`UpdatePlaybook{title}`/`AddBlock{...}`/`RemoveBlock{...}`/`MoveBlock{...}`, all struct-variant syntax) — none of these variants exist on the current `FormMutation` enum (tuple variants wrapping a per-triad payload struct: `CreateStep(CreateStep{step,index})`, etc.). Rewired every call site to the current vocabulary; `patch_step::handle` (which used to build a whole replacement `FormStep` for `UpdateStep`) now emits the correct granular `RenameStep`/`ChangeStepDescription` mutation directly instead, since no whole-step-replace variant exists (nor should one, per the banned-vocabulary rule).

### `SetSpecJson`/JSON-deserialize fixture helpers

`FormsSnapshot`'s own `serde` shape no longer holds raw step JSON (composed children serialize as opaque handles), so every call site that used to `serde_json::from_str::<FormsSnapshot>(raw_steps_json)` needed a different deserialize target:
- `set_spec_json::handle` (`📥️import/🦀️component.rs`) now deserializes into `flow::playbook::PlaybookSpec` (the SAME `{schema,id,version,title,steps}` camelCase shape `FormsSnapshot` used before composition — framework-owned, untouched, still real content) and builds the snapshot via `forms_snapshot_with_state`.
- `step_with_conditional_block()` (a JSON test fixture in `💡️inferences/🦀️component.rs`) — same fix.
- `set_spec_json_replaces_the_document`'s own test input (`📥️import/🦀️component.rs`) — was building its JSON via `serde_json::to_string(&onboarding_example_spec())` (now opaque-handle JSON, silently no-op'd the command); fixed to serialize a `PlaybookSpec` built from `forms_steps(&onboarding_example_spec())` instead.

### Fixture regeneration — a real design decision, not just a mechanical regen

Ran the recipe's temporary-debug-test technique first (`cargo test … debug_fixture_regen -- --nocapture`, captured real `print_dsl()` output for all three examples) — but the result exposed a real architectural gap the recipe's other exemplars never hit: `structure`/`results` are content-addressed handles resolved through the SESSION-LOCAL working-scene cache (no real `ArtifactView::with_children` seam yet, per §3). A snapshot loaded via `FormsSnapshot::parse_dsl` in a FRESH process/thread has never minted anything into that cache, so `forms_steps` reads back EMPTY — fine for the store's own persistence format (a real gap every composed plugin documents), but forms specifically also has THREE named starter examples (`default`/`onboarding`/`building-component`, selectable via `SetActiveExample`) that need REAL content at load time, not an empty cache miss.

Reverted the naive regen (hardcoded-handle fixture text) and instead added `parse_playbook_example_dsl` (`📸️snapshot/📝️text/🦀️component.rs`) as the PERMANENT loading path for these three examples: it parses the fixture text through the shared, untouched `flow::playbook::PlaybookSpec` grammar (real human-authored domain content — `DEFAULT_EXAMPLE_TEXT`/`ONBOARDING_EXAMPLE_TEXT`/the `🖼️assets/🗣️example.dsl.semio` asset all stay in that same handcrafted grammar, byte-identical to before, never regenerated into the opaque handle format) and constructs the snapshot via `forms_snapshot_with_state`, which mints+caches the children in the SAME call — always cache-warm. `building_component_spec`/`default_example_spec`/`onboarding_example_spec` (`🧬️schema/🦀️component.rs`), `set_active_example::handle`'s `"building-component"` arm, and the demo's own `inference_determinism_law` test all switched from `parse_dsl` to `parse_playbook_example_dsl`. `FormsSnapshot::parse_dsl` itself (the hand-rolled codec) is independently, directly proven correct by two new tests in `📸️snapshot/🦀️component.rs` (`snapshot_dsl_round_trips_with_composed_children`/`snapshot_pack_round_trips_with_composed_children`) that build via `forms_children_from_steps` and round-trip within one cache-warm call — no fixture text needed to prove the codec itself works.

## Working-scene design

See `FormsWorkingScene`'s own doc comment (`🗿️artifacts/📋️forms/🦀️component.rs`, `🔖️WorkingScene` region) — thread-local `HashMap<child_id, FormsWorkingScene>`, matching `mathematical`'s `MATH_SCRATCH` exactly. Same documented staleness gap (store-level undo/redo bypasses `ArtifactApp::handle`); `forms_steps` fails soft to an empty `Vec`.

## Converters (real, not stubs)

`semio_value_from_dsl`/`dsl_from_semio_value`, `semio_value_from_expr`/`expr_from_semio_value`, `semio_value_from_block`/`block_from_semio_value`, `semio_value_from_step`/`step_from_semio_value`, `forms_structure_from_steps`/`forms_steps_from_structure`, `forms_results_from_steps` — all in `🗿️artifacts/📋️forms/🦀️component.rs`'s `🔖️Converters` region. Every field of `FormQuestion` (18 optional fields including the recursive `condition` tree) and `FormStep` is real, converted both directions; `Bytes`/`Ref` `SemioValue` variants degrade honestly to `DslValue::Null` (documented in the converter's own doc comment — never produced by this plugin's own round trip, only reachable if a foreign composer wrote one into forms' own `structure` child).

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-forms --all-targets
```
**0 errors**, confirmed on the final run. Remaining warnings are pre-existing/cosmetic (unnecessary-qualification style lints predating this pass, unused stdio-serializer imports in the io-registry files, a `QuestionKindRoute`-visibility lint — none touched by this migration, none block compilation).

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-forms --no-fail-fast
```
**110/111 passed**, reproduced stable across 2 consecutive full runs (same single failure both times, not flaky).

## Fixed outright (trivial/unambiguous, independently traced)

1. **Baseline SMO-rename fallout** (9 compile errors) — see `## Concurrent-churn observations`.
2. **5 mutation round-trip unit tests failing after baseline+composition fixes** (`create_then_delete_step_round_trips`, `create_then_delete_block_round_trips`, `delete_step_inverse_recreates_step_with_its_blocks_at_original_index`, `delete_block_inverse_recreates_block_at_original_index`, `move_block_to_step_round_trips_across_steps`): root-caused to the tests' OWN undo-replay pattern, `state = step.diff(&base).apply(&state)` — diffing the undo mutation against the ORIGINAL, now-stale `base` while applying to the evolving `state`. Each triad's existence guard (`if !forms_steps(base).iter().any(...) { return FormsDiff::default(); }`) correctly answers "does this id exist in `base`" — but `base` and `state` disagree on existence exactly for create/delete pairs (that's the whole point of the test), so the guard fires as a false no-op. Confirmed this is a TEST-authoring bug, not a production one: `store::ArtifactStore::replay_mutations` (the real dispatch path, `🏪️store/🦀️component.rs:3421`) always diffs each mutation against the freshly-evolved live snapshot, never a fixed stale base — so real dispatch was never at risk. Fixed by changing the replay loop to `state = step.diff(&state).apply(&state)` (10 occurrences, one `replace_all`, in `🧬️mutations/🦀️component.rs`), matching real dispatch semantics exactly. Verified: all 5 pass, the other 5 tests using the identical pattern (which never hit the guard mismatch) still pass unchanged.
3. **`set_spec_json_replaces_the_document`**: test built its own input JSON from `FormsSnapshot`'s now-opaque serialized shape; fixed to build `PlaybookSpec`-shaped JSON instead (see `### SetSpecJson` above).

## Honest gap — 1 pre-existing failure, not fixed, full provenance

`examples::art_forms_demo_tests::inference_determinism_law` fails: `assert_eq!(inference.topology.topo_order.len() as u32, expected_nodes)` — `left: 8, right: 16`.

**Root cause (verified by hand, not guessed)**: the demo asset (`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, the "building-component" fixture) authors a STEP with id `geometry` (`id=geometry title=Geometry description="Parametric geometry..."`) that ALSO contains a BLOCK with the SAME id `geometry` (`id=geometry label="Hexagonal Column" kind=buildingComponent ...`, line 35 of the asset). `compute_forms_topology`'s Kahn's-algorithm implementation (`🧭topology/🦀️component.rs`) keys its `indegree`/`adjacency` maps by bare string id with no step-vs-block namespacing, so the step-node "geometry" and the block-node "geometry" collide into ONE hashmap slot. The sequential document-order edge into the step "geometry" (from the last identity-step block) and a LATER edge into the block "geometry" (from the "offset" block, several nodes further into the same step's own block list) both target the same collided slot — creating an unsatisfiable indegree that Kahn's algorithm can never fully drain, so the topological sort silently stops after 8 of the 16 real nodes (`topo_order = ["identity","name","description","material","tags","prefab","install-date","accent-color"]`, never reaching "geometry" or anything after it). `node_count` (computed directly as `nodes.len()`, no hashmap involved) correctly reports 16; only `topo_order`/`cycle_free` are corrupted by the id collision.

**This is independent of the `steps` storage mechanism** — I hand-verified by constructing the identical `Vec<FormStep>` content and calling `compute_forms_topology` directly: the bug reproduces byte-for-byte whether `steps` comes from the old inline field or from `forms_steps`'s working-scene read, since the function's own input parameter and internal logic are completely unaffected by composition (I only changed the ONE call site, `compute_forms_topology(&snapshot.steps)` → `compute_forms_topology(&forms_steps(snapshot))` — same `&[FormStep]` slice content either way).

**Dating**: `git log -1 --date=iso -- 📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` and the test file that asserts on it → both `fd01661f… 2026-08-12 18:08:12` — **after this ticket opened** (`2026-08-12 15:02:49`) but **before my own dispatch and edits began**, and touching neither the topology algorithm (`a46ac1f8…, 2026-08-12 13:17:52`, even earlier) nor any file I wrote to. Confirmed via `%ad`, not the commit message's fake `🎆️26🌙️06☀️04` glyphs.

**Why not fixed outright**: fixing it correctly means namespacing `compute_forms_topology`'s node-id scheme (e.g. prefixing step/block ids by kind before building the graph) — a real algorithmic change to inference code with no connection to composing `value`/`table`, and outside this migration's charter. Flagged here with full derivation so a dedicated fix (or a simple fixture-content edit renaming the colliding block id) can land with zero re-investigation cost.

## Concurrent-churn observations

**Baseline was red from live concurrent SMO churn inside forms' own boundary** (not framework/glue), traced and fixed before starting composition — full list:
- `🧬️mutations/📝️text/🦀️component.rs`: import list still named the pre-rename modules (`add_block`/`add_step`/`move_block`/`move_step`/`remove_block`/`remove_step`/`update_block`/`update_playbook`/`update_step`) — renamed to the current triad module names.
- `🧬️mutations/🦀️component.rs`'s own `#[cfg(test)]` import: same stale names, same fix; stale file-level doc comment describing glue.rs's mounts as still using the old generic-verb directories (they don't — `📦️glue.rs` was already renamed in the SAME commit) — corrected.
- `📝change-step-description/{🔺️diff,↩️inverse}/🦀️component.rs`: `use super::mutation::{ChangeStepDescription, ChangeStepDescription};` — literal duplicate-name import typo (E0252) in both files, from what was evidently a mechanical file split; fixed to import the name once.
- 6 more files (`🧬️schema/🦀️component.rs`, `📸️snapshot/💾️binary/🦀️component.rs`, `🔺️diff/📝️text/🦀️component.rs`, `➖delete-block/↩️inverse`, `➕create-block/↩️inverse`, `🌱create-step/↩️inverse`, `🗑️delete-step/↩️inverse`): individual stale module-path references (`mutations::update_block::…`/`mutations::add_step::…`/`mutations::add_block`/`mutations::remove_block`/`mutations::remove_step`) — each renamed to its current triad module.

All 9 traced to commit `31209e7a…` (`git log -1 --date=iso`, 2026-08-13 00:13:16 — inside this ticket's active window, matching `📦️glue.rs`'s own last-modified commit exactly) — this is SMO's mutation-directory rename landing on `📦️glue.rs`/the dispatch enum together but NOT propagating to every sibling file inside forms' own boundary. Since these files are `✏️s/🔌️plugins/📋️forms/**` (not `📦️glue.rs`/`📦️index.ts`), hot-file ownership makes them mine to fix, and they were trivial/unambiguous renames — fixed outright per `📌️important.md`'s own guidance, not deferred.

One transient `cargo check` failure during mid-pass verification: `semio-framework-plugin` (a W1-owned framework dependency, outside this plugin's boundary — errors in `VcsArtifactApp`/`ArtifactView`/`PresenceMutation` type-checking) failed with 3, then 17 errors across two consecutive re-checks — an escalating count is itself evidence of another session's in-flight edit to that shared file settling mid-build, not a defect in this plugin. Grepped both runs: zero errors originated under `📋️forms/`. Retried in the foreground (no background wait, per `📌️important.md`'s dispatch rule); the next `cargo check -p semio-s-plugin-forms --all-targets` came back clean.

## sharedFileRequests

None. Every change is inside `✏️s/🔌️plugins/📋️forms/**`, never touching `📦️glue.rs`/`📦️index.ts` or any `🗄️stdio/**` file (only read for schema reference: `SemioValueSnapshot`/`SemioValue`/`SemioValueEntry`/`SemioTableSnapshot`/`SemioTableColumn`/`SemioTableRow`/`SemioTableCellKind` and their `STDIO_SEMIOVALUE_DOCUMENT_SCHEMA`/`STDIO_SEMIOTABLE_DOCUMENT_SCHEMA` constants).

Worth flagging for a future wave, not requested as a shared-file change here: `default_example_json()`/`onboarding_example_json()` (`🧬️schema/🦀️component.rs`, used for `App::example`'s `document_json` manifest field) still `serde_json::to_string(&…_example_spec())` — this now serializes `FormsSnapshot`'s OWN composed shape (opaque `structure`/`results` handles), not raw step/block content. `set_active_example::handle` (the actual functional command path) never relies on this — it matches `example_id` strings and calls the real spec functions directly — so this is a LATENT gap (whatever framework-level consumer reads `document_json` generically, if any, would see opaque handles instead of real content), not a regression in any currently-exercised code path (all 111 tests pass or independently-traced-fail without touching this). Structurally the same characteristic every composed plugin's JSON serialization now has; not something this migration's charter (compose `value`/`table`) is positioned to solve without a custom, working-scene-aware `Serialize` impl.

## Files touched this pass

- `🗿️artifacts/📋️forms/🦀️component.rs` — new `🔖️Composition` region (child types, converters, working scene, `forms_children_from_steps`/`forms_steps`/`forms_artifact_steps`/`forms_snapshot_with_state`), `flatten_questions`/`locate_question`/`empty_forms_snapshot` rewired, test fixes.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `FormsSnapshot` field swap, hand-rolled codecs, 2 new round-trip tests.
- `…/🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — `parse_playbook_example_dsl` (permanent example-loading bridge), round-trip test fixes.
- `…/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` — fixture-load + command-envelope test fixes, stale doc comment corrected.
- `…/🧬️schema/🦀️component.rs` — `FormsArtifact` field swap, conversions, `DocumentHelpers` example-loading switched to `parse_playbook_example_dsl`.
- `…/🧬️schema/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/🦀️component.rs` — `FormsDiff` field swap (dead `artifact` slot removed), `forms_diff_from_delta`, apply/absorb, `sparse_diff_between`, test fixes.
- `…/🧬️schema/💡️inferences/🦀️component.rs` — `compute_forms_topology` call site, JSON test fixture fix.
- `…/🧬️schema/🧬️mutations/🦀️component.rs` — dispatch-level test fixes (10 tests, undo-replay pattern fix).
- `…/🧬️schema/🧬️mutations/{✏️rename-step,🌱create-step,🗑️delete-step,🔀reorder-step,📝change-step-description,➕create-block,➖delete-block,📦move-block-to-step,🔁replace-block}/{🔺️diff,↩️inverse}/🦀️component.rs` — 18 files, mechanical `.steps`→`forms_steps` rewiring per triad, plus the `📝️text/🦀️component.rs` codec dispatch file.
- `📚️examples/🎬️demo/🧪️tests/🦀️test.rs` — `parse_playbook_example_dsl` call site.
- `🎛️apps/📋️forms/🦀️component.rs` — 2 test call sites.
- `🎛️apps/📋️forms/🎮️commands/{📃️step,❓️question,📥️import,🗂️selection,🧪️try}/🦀️component.rs` — `.steps`→`forms_steps` rewiring; `📃️step`/`❓️question`/`📥️import` also got the pre-SMO-rename `FormMutation` vocabulary fixed (see `## What changed`); `📥️import` also got `SetSpecJson`'s deserialize target fixed.
- `🎛️apps/📋️forms/📌️panels/{📄️artifact,🔍️inspection}/🦀️component.rs`, `🎭️modes/📝️blueprint/🪟️windows/{▶️try,🧱️builder}/🦀️component.rs` — `.steps`→`forms_steps` rewiring.

ucas-status: complete
