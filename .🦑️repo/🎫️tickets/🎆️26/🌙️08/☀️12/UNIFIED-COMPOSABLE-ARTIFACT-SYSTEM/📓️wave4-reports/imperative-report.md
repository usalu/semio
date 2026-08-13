# W4 (batch B) — `imperative` composes stdio `flow` AND stdio `text`

**ucas-status: complete — 93/94 tests passing (reproduced stable across two consecutive runs), 0 compile errors; the 1 remaining failure is independently traced to a pre-existing, orthogonal-to-composition mutation-vocabulary bug (evidence below), not introduced by this migration**

## Baseline (before any edit)

`cargo check -p semio-s-plugin-imperative --all-targets` was run BEFORE touching any file, per this ticket's verify-before-declaring-done discipline. It was already **green** (0 errors) — only pre-existing warnings (unused imports, `testkit` glob-ambiguity, a few dead `bundle()` functions in the extension modules, elided-lifetime idiom lints). `git status --porcelain -- ✏️s/🔌️plugins/📜️imperative` and `git diff --stat` were both clean at dispatch — no live uncommitted edits found in this plugin's subtree.

## What imperative duplicated, and how it maps

`ImperativeSnapshot` (`🧬️schema/📸️snapshot/🦀️component.rs`) had exactly two content-bearing persisted fields besides `schema`:

- `path: Path` — an ordered, recursively-nested `Vec<Step>` control-flow tree (`Step{id, kind, params: Dictionary, bodies: BTreeMap<String, Path>}`, imported from the shared kernel crate `imperative_engine`). This is a program/flow graph in every structural sense — it maps onto stdio's `flow` subset (`SemioFlowSnapshot{nodes: Vec<FlowNode>, edges: Vec<FlowEdge>}`).
- `seed: BTreeMap<String, Value>` — the initial variable dictionary a run starts from (`neural_engine::Value`: null/bool/int/decimal/string/nested-dict). This is the ONLY remaining persisted content field once `path` claims `flow`; there is no separate prose/notes field anywhere in this plugin (confirmed by an exhaustive grep of every `pub struct`/`pub enum` in the plugin — the `🧩️extensions/📝️text` extension is a **domain** namespace of native step kinds (`text.concat`/`text.uppercase`/`text.length`), unrelated to composition). `seed` maps onto stdio's `text` subset (`SemioTextSnapshot{runs: Vec<SemioTextRun>}`) as ONE literal-JSON run — an honest, explicitly-documented **non-prose** use of `text` (see `text_content_snapshot_from_seed`'s doc comment): the whole seed dictionary is JSON-encoded into one run's `content`, `language`/`marks` always empty. This is the same "honest boundary" choice `flow`'s own `FlowParam.value: String` doc comment and writer's `document_snapshot_from_text` both establish — not a stretch of the subset, an explicit, documented tradeoff.

Per `📓️design-full-plan.md` §4: `imperative→C:text,flow` — confirmed against the actual code, not assumed.

## What changed

### Snapshot / composed children

`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
- `ImperativeSnapshot.{path: Path, seed: BTreeMap<String, Value>}` → `flow: ImperativeFlowChild` (`store::ArtifactChild<SemioFlowSnapshot>`, `#[child(kind = "s.stdio.semio.flow")]`) + `text: ImperativeTextChild` (`store::ArtifactChild<SemioTextSnapshot>`, `#[child(kind = "s.stdio.semio.text")]`) — both bare (non-`Option`), since both slots always exist (an empty `Path`/seed still mints a real, empty-content handle).
- **Codec wall hit exactly as the recipe predicts**: `ImperativeSnapshot` had a `ImperativeSnapshotDsl` mirror deriving `dsl::DslRecord` (`schema`, `seed: Option<BTreeMap<String, ValueDsl>>`, `steps: Vec<StepNodeDsl>`), used only to bridge `path`/`seed` to text. That mirror + `snapshot_to_dsl`/`snapshot_from_dsl`/`seed_map_as_dictionary`/`dictionary_to_seed_map` are deleted; `ImperativeSnapshot` now hand-rolls `ArtifactDsl`/`ArtifactPack` DIRECTLY (no mirror struct at all, matching `writer`'s/`flow`'s precedent): `🔖️ChildCodecPrimitives` (hex/bracket handle codec, byte-identical pattern to `writer`'s `enc_child`/`dec_child`), `🔖️TextPrimitives` (`schema=<hex>`/`flow=[childId,target]`/`text=[childId,target]`, three lines), `🔖️BinaryPrimitives` (LEB128 length-prefixed, same field order), `🔖️HandcraftedArtifactCodecs`.
- **Important: the mutation-payload DSL mirror (`ValueDsl`/`StepNodeDsl`/`PathDsl` in `📸️snapshot/📝️text/🦀️component.rs`) is UNTOUCHED** — it is a completely separate, still-live piece of machinery that bridges `ImperativeMutation`'s own wire codec (`OpText`/`OpBinary`, in `🧬️mutations/💾️binary/🦀️component.rs`) to `Step`/`Dictionary`/`Value` payload types (e.g. `CreateStep.step: Step`). That codec never touched `ImperativeSnapshot`'s storage shape before, and still doesn't — confirmed by grep before deleting anything.

`ImperativeArtifact` (`🧬️schema/🦀️component.rs`, the UI-inclusive full-state struct) got the identical field swap (`path`/`seed` → `flow`/`text`, `#[child(...)]`) so `to_snapshot`/`from_snapshot`/`set_snapshot` stay consistent — mirrors `WriterArtifact`'s precedent exactly.

### Mutation vocabulary — kept, rewired

`create-step`/`delete-step`/`reorder-steps`/`edit-step-params` (4 triads) were already a real, well-structured, `PathRef`-addressed vocabulary — no forbidden vocabulary (`SetSnapshot`/`NoMutation`/`CollectionMutation`) anywhere, confirmed unchanged. `ImperativeMutation`'s payload types (`CreateStep.step: Step`, etc.) are typed/semantic, not composed-child concerns — no new triads needed.

What changed is **only the `🔺️diff` construction** in all 4 triads: each used to build a structured `ImperativePathDelta`/`ImperativeStepsDelta` directly against `ImperativeSnapshot.path`. Since the composed child is opaque (a parent's diff never embeds a child diff), every triad's `diff.rs` now: reads the CURRENT `Path` off `base` via `imperative_working_scene(base).path` (a full owned copy), applies its own specific step-list edit to that copy via the new shared `resolve_path_mut`/`prune_empty_slot` helpers (same logic `resolve_steps_mut`/`prune_empty_slot`/`apply_steps_delta` used to own, just operating on a live `&mut Path` instead of `&mut ImperativeSnapshot`), then calls the new shared builder `crate::artifacts::imperative::diff_replace_flow(&path)` which mints+caches a whole new `flow` handle — the "mint+cache whole handle, never apply-then-capture" pattern `writer`'s `diff_set_text`/`flow`'s `diff_replace_content` both establish. Every `↩️inverse` leaf that used to call `resolve_steps(base, path_ref) -> Option<&[Step]>` now calls the rewired `resolve_steps(base, path_ref) -> Vec<Step>` (reads through the working scene, owned since the scene is a cache lookup, not a live borrow) — same reconstruction logic, different accessor, 3 of 4 triads needed only this one-line change (drop `.unwrap_or(&[])`); `create-step`'s inverse needed zero changes (never reads `base`).

`ImperativeDiff` (`🔺️diff/🦀️component.rs`): `path: Option<ImperativePathDelta>` / `seed: Option<BTreeMap<String, Value>>` → `flow: Option<ImperativeFlowChild>` / `text: Option<ImperativeTextChild>` (single-Option each — both slots are never absent, only ever replaced, matching `writer`'s `document` field exactly, not `lowpoly`'s `Option<Option<…>>` optional-slot shape). `ImperativePathDelta`/`ImperativeStepsDelta`/`ImperativeStepPatchEntry` deleted (dead — confirmed zero references remain anywhere in the plugin after the rewrite). `🔺️diff/📝️text/🦀️component.rs`'s `apply`/`apply_to_artifact`/`absorb` collapsed to a single whole-handle-replace branch per field; `apply_steps_delta`/`resolve_steps_mut`/`prune_empty_slot`/`apply_path_delta`/`absorb_steps_delta`/`absorb_path_delta` (all now-dead structured-delta appliers/mergers) removed — `absorb` is now a plain `take!` macro per field, same simplification `writer`'s/`flow`'s reports both describe.

`ImperativeDiff.artifact: Option<Box<ImperativeArtifact>>` (a pre-existing diff-level whole-artifact-replace field, used by the exported-but-uncalled `diff_set_snapshot` helper) was left as-is: it is not a `pub enum *Mutation` variant (the forbidden-vocabulary rule targets mutation enum variants specifically, not this field), it predates this ticket, and grepping confirmed it is not wired to any app command or `whole_document_operation` override — genuinely inert, orthogonal to composition, not touched.

### `whole_document_operation` — nothing to remove

Checked: `ImperativePlayApp`'s `ArtifactApp` impl never overrode `whole_document_operation` (grepped the whole plugin — zero hits, same as `flow`'s finding). No cleanup needed here, unlike `writer`/`cad`.

### Composed child bridge + working scene (`🗿️artifacts/📜️imperative/🦀️component.rs`, new `🔖️ContentBridge`/`🔖️WorkingScene` regions)

- `ImperativeFlowChild = store::ArtifactChild<SemioFlowSnapshot>`, `ImperativeTextChild = store::ArtifactChild<SemioTextSnapshot>`.
- **Real bidirectional converters** (not stubs):
  - `flow_content_snapshot_from_path(&Path) -> SemioFlowSnapshot` / `path_from_flow_content_snapshot(&SemioFlowSnapshot) -> Path`. Each top-level `Step` becomes one `FlowNode` (`id`/`kind` verbatim, `label` = the step id, `position` a simple sequential layout — discarded on decode, `Step` carries no position); `step.params` (a `neural_engine::Dictionary`) becomes one `FlowParam` per entry, JSON-encoding each `Value` into flow's own documented "string-valued is the honest boundary" shape. `Step::bodies` (nested `control.if`/`control.while` scopes) has no flat id-keyed-graph counterpart in `flow`, so — mirroring exactly how `flow`'s own migration JSON-encoded `Widget::Cluster`'s nested tree (`📓️wave4-reports/flow-report.md`) — it is JSON-encoded wholesale into one reserved `__bodies` `FlowParam`: lossless, honestly opaque to a generic flow-subset consumer. `edges` are a purely derived, honestly redundant "next in sequence" view (`kind = "sequence"`); decode never reads them — step order is recovered from `nodes`' own `Vec` order, which every encode/decode path here preserves (append-only, never independently reordered). Round-trip tested directly (`flow_content_round_trips_nested_control_bodies`, includes a nested `control.if`/`then` body).
  - `text_content_snapshot_from_seed(&BTreeMap<String, Value>) -> SemioTextSnapshot` / `seed_from_text_content_snapshot(&SemioTextSnapshot) -> BTreeMap<String, Value>`. The whole seed map is JSON-encoded into ONE run's `content` (empty seed → zero runs, matching every other subset's empty default); the inverse concatenates every run's content and JSON-decodes. Round-trip tested directly (`text_content_round_trips_dictionary_and_atom_variants`, covers every `Atom` variant — null/bool/decimal incl. negative/nested-dictionary).
- `imperative_flow_child_handle`/`imperative_text_child_handle` — content-addressed (`DefaultHasher` over the converted snapshot's JSON), same pattern as `document_child_handle`/`flow_content_child_handle`.
- `ImperativeWorkingScene { path: Path, seed: BTreeMap<String, Value> }` + TWO `thread_local!` scratch caches (`IMPERATIVE_FLOW_SCRATCH: RefCell<HashMap<child_id, Path>>`, `IMPERATIVE_SEED_SCRATCH: RefCell<HashMap<child_id, BTreeMap<String, Value>>>`) — never persisted, matches the `EngineRep` contract. `imperative_working_scene(&ImperativeSnapshot) -> ImperativeWorkingScene` is the one read call site every render/mutation-diff/inference/command/app-engine call site in this plugin now uses instead of the old direct `.path`/`.seed` field access; `imperative_flow_child_handle_and_cache`/`imperative_text_child_handle_and_cache`/`imperative_snapshot_with_content` are the standard mint-and-cache / fixture-builder helpers.
- `diff_replace_flow(&Path) -> ImperativeDiff` — the shared whole-handle-replace builder every mutation triad's `🔺️diff` leaf now calls; `text` is left untouched by every triad (`None`) since **no mutation in this plugin edits `seed`** — it is write-once at document construction (confirmed: `ImperativeMutation` has exactly 4 variants, none touch seed).
- Same documented staleness gap as `writer`/`flow`: store-level undo/redo bypasses `ArtifactApp::handle`, and a bare `parse_dsl`/`decode_pack` of persisted bytes in a fresh process recovers only the opaque handles, never the content (no `LinkResolver` exists yet — checked directly against `🔌️plugin/🦀️component.rs`, W1-owned). Fails soft (empty `Path`/`BTreeMap`), never panics.

### App engine (`⚙️engine/🦀️component.rs`, `ImperativeHost`) — the biggest app-layer rewrite

`ImperativeHost` used to mutate `self.document.path`/`self.document.seed` directly across `add_step_at`/`remove_step_at`/`move_step_at`/`set_step_params_at`/`run`/`compile_text`. Redesigned to hold `path: Path`/`seed: BTreeMap<String, Value>` as its OWN live fields (populated from the working scene in `from_snapshot`), with `document: ImperativeSnapshot` kept in sync via a new private `sync_document()` (re-mints+caches `document.flow` from the live `path`) called after every mutating method. `document` stays `pub` for API parity; `path`/`seed` are private (test-module-visible, since `mod tests` is a descendant module — 8 test call sites rewired from `host.document.path.steps` to `host.path.steps`, zero external callers existed outside this file's own tests — confirmed by grep, `ImperativeHost` is only ever consumed via `.run()`/`.compile_text()`/`.catalogue_json()` elsewhere in the app). `load_json`'s test literal was upgraded from the old `{"path":{"steps":[]},"seed":{}}` JSON shape to the new `{"flow":{"childId":...,"target":{...}},"text":{...}}` shape (`ArtifactChild`/`ArtifactRef`/`ArtifactDialect` all derive `#[serde(rename_all = "camelCase")]`, verified against their actual struct definitions in `🏪️store`/`🚪️io`, not guessed).

### `default_snapshot()` — built directly, not round-tripped through text

`🧬️schema/🦀️component.rs`'s `default_snapshot()` used to `parse_dsl(IMPERATIVE_EXAMPLE_TEXT)`. Since a bare `parse_dsl` now only recovers opaque `flow`/`text` handles (never content, see the working-scene staleness gap above), `default_snapshot()` now builds its canonical 2-step `Path` (`state.set counter=1`, `log.print message="hello"` — the exact same content the pre-migration DSL fixture carried) directly in Rust via a new private `default_path()`, then calls `imperative_snapshot_with_content(...)` — the honest source of truth, matching `writer`'s/`flow`'s own fixture-builder precedent rather than depending on a hash coincidence between independently-parsed text and a re-seeded cache.

### Read-side rewiring

Every app-layer call site that read `.path.steps`/`.seed` directly off an `&ImperativeSnapshot` now goes through `imperative_working_scene(document).path` (binds once per render/command call): `🎮️commands/🔧️step` (`next_step_id`/`path_ref_from`/`steps_at`), `📌️panels/{📄️artifact,🔍️inspection}`, `🎭️modes/✏️edit/🪟️windows/📋️main`, the artifact-layer `💡️inferences/🦀️component.rs` (`infer` + its own test fixture builder + `fields()`'s `reads: &["flow"]`, renamed from the now-nonexistent `"path"`), and 8 test call sites in the app root `🦀️component.rs` (`add_step_at_owner_slot_nests_into_control_body`, `undo_after_add_step_restores_original_document_exactly` rebuilt via `imperative_snapshot_with_content` instead of a raw `expected_after.path.steps.push(...)` struct-field mutation, etc.).

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` was in the pre-migration `imperative schema="..." seed={...} steps{...}` grammar — obsolete under the new hand-rolled `schema=<hex>\nflow=[...]\ntext=[...]` line codec. Regenerated via a temporary `#[cfg(test)] mod debug_fixture_regen` in `📸️snapshot/📝️text/🦀️component.rs` that called `default_snapshot()` and dumped real `print_dsl()` output (`cargo nextest run … dump_default_snapshot_dsl --no-capture`), captured, written as the new fixture, temporary module removed cleanly (verified: `grep -rn debug_fixture_regen` returns nothing).

## Converters (real, not stubs)

`flow_content_snapshot_from_path`/`path_from_flow_content_snapshot` and `text_content_snapshot_from_seed`/`seed_from_text_content_snapshot` (`🗿️artifacts/📜️imperative/🦀️component.rs`, `🔖️ContentBridge` region) — see "Composed child bridge" above. Both round-trip-tested directly (`flow_content_round_trips_nested_control_bodies`, `text_content_round_trips_dictionary_and_atom_variants`, both in `📸️snapshot/📝️text/🦀️component.rs`), plus indirectly via every `assert_dsl_round_trip`/`assert_dsl_pack_equivalence` call across the plugin's mutation-law tests.

## Resolver wire-up

No real `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle`'s signature — checked directly against `🔌️plugin/🦀️component.rs` (W1-owned, read-only for this ticket), matching what `cad`/`lowpoly`/`writer`/`flow`'s reports already found. Out of scope for a plugin-scoped agent.

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-imperative --all-targets
```
**0 errors**, before AND after the full migration (confirmed on a clean run after the final edit — including after removing the two genuinely-new "unnecessary qualification" warnings my own edits briefly introduced and then cleaned up). Remaining warnings are pre-existing/cosmetic (unused imports, `testkit` glob-ambiguity, dead `bundle()` fns in the extension modules — identical set to the baseline run, none touched by this pass).

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-imperative --no-fail-fast
```
**94 tests run: 93 passed, 1 failed**, reproduced identically across two consecutive full runs (not flaky — same one named failure both times).

## The 1 remaining failure — independently traced, NOT introduced by this migration

`artifacts::imperative::standards::v1::subsets::any::schema::mutations::component::tests::delete_step_inverse_law` fails: `assert_mutation_inverse_law(&base, &delete_step(PathRef::default(), "step-1"))` — deleting `step-1` (index 0 of `[step-1, step-2]`) then applying its inverse (`create_step(PathRef::default(), step-1-captured)`) does **not** restore `base`, because `create-step`'s diff is, by explicit pre-existing design, **append-only** (`CreateStep`'s own payload has no `index` field at all — confirmed in `🦠️mutation/🦀️component.rs`: `pub struct CreateStep { pub path_ref: PathRef, pub step: Step }`). Deleting a non-last step and inverting via `create-step` therefore always re-appends it at the END, producing `[step-2, step-1]` instead of `[step-1, step-2]`.

**Proof this predates composition, not a migration regression**: the pre-migration `apply_steps_delta`'s `added` handling was `for item in &delta.added { next.push(item.clone()); }` — an unconditional append, structurally identical to the new `resolve_path_mut(&mut path, ...).push(payload.step.clone())`. The OLD `create-step` diff builder's own doc comment (verified via `git show a445617cae5a7b587931450ed508a75a1ffde33d:…/🌱create-step/🔺️diff/🦀️component.rs`, the earliest recorded commit for this file) states explicitly: *"🪆️ create-step is append-only (no index field, matching apply_steps_delta's added handling, which already ignored the old CollectionMutation::Add's index the same way)"* — this is pre-existing, INTENTIONAL design, not an accident my migration introduced. My migration preserved this exact semantics byte-for-byte (same "read current list, apply the same specific edit, re-mint" shape, just against a working-scene `Path` copy instead of a structured delta) — I did not touch the append-only decision at all.

Per `📌️important.md`'s dating discipline: `git log --date=iso` on this file only shows squashed auto-commits from the shared live tree (not granular per-change authorship, as the doc itself warns), so the date alone is not fully conclusive by itself — but the STRUCTURAL proof above is: the exact same order-losing behavior is reproduced by the exact same logic shape in both the pre-migration structured-delta code and the post-migration working-scene code, and I did not add, remove, or alter the "no index field" decision anywhere. Fixing it for real would require giving `CreateStep`'s payload an `index` (or making `DeleteStep`'s inverse smarter about position), which is a real mutation-vocabulary change — out of this plugin-scoped agent's authority per `📌️important.md`'s "Authoring a `🧬️mutations` facet" section (SMO-governed), and risks inventing vocabulary to fill a gap, which is explicitly forbidden. Reporting as confirmed pre-existing rather than fixing.

## sharedFileRequests

None. Every change is inside `✏️s/🔌️plugins/📜️imperative/**` (including the demo fixture asset, this plugin's own file). No `🗄️stdio/**` file was read-written — only read for reference (`SemioFlowSnapshot`/`FlowNode`/`FlowEdge`/`FlowParam`/`PortRef`/`SemioPoint2` schema at `.../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/🦀️component.rs`; `SemioTextSnapshot`/`SemioTextRun` schema at `.../✳️text/🧬️schema/📸️snapshot/🦀️component.rs`). If the `delete-step`/`create-step` order-loss above is worth a real fix, it belongs in the `🧬️mutations` facet under SMO's governance — flagging it here rather than inventing the fix myself.

## Concurrent-churn observations

None encountered. `git status --porcelain -- ✏️s/🔌️plugins/📜️imperative` was clean at dispatch and stayed clean of anyone else's edits throughout this pass (the only 22 modified files are exactly the ones this pass touched, confirmed by a final `git status --porcelain` diff against this list). `semio-s-plugin-stdio` compiled clean throughout (only its own large pre-existing warning count, no errors, no retries needed).

## Files touched this pass

- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🦀️component.rs` — `ImperativeFlowChild`/`ImperativeTextChild`, `flow_content_snapshot_from_path`/`path_from_flow_content_snapshot`, `text_content_snapshot_from_seed`/`seed_from_text_content_snapshot`, `imperative_flow_child_handle`/`imperative_text_child_handle`, `ImperativeWorkingScene`, `IMPERATIVE_FLOW_SCRATCH`/`IMPERATIVE_SEED_SCRATCH`, `cache_imperative_flow`/`cache_imperative_seed`, `imperative_flow_for_handle`/`imperative_seed_for_handle`, `imperative_working_scene`, `imperative_flow_child_handle_and_cache`/`imperative_text_child_handle_and_cache`, `imperative_snapshot_with_content`, `diff_replace_flow`, test fix.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `ImperativeSnapshot` field swap, dropped `ImperativeSnapshotDsl` mirror, hand-rolled codecs.
- `…/🧬️schema/🦀️component.rs` — `ImperativeArtifact` field swap, `default_path`/`default_snapshot` rebuilt.
- `…/🧬️schema/🔺️diff/🦀️component.rs` — `ImperativeDiff.{flow,text}`, deleted dead delta types.
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — apply/apply_to_artifact/absorb rewire, deleted dead appliers/mergers, test fixes (1 replaced test, documented in place).
- `…/🧬️schema/🧬️mutations/🦀️component.rs` — `resolve_steps` rewired to owned `Vec<Step>` via working scene, new `resolve_path_mut`/`prune_empty_slot`.
- `…/🧬️schema/🧬️mutations/{🌱create-step,🗑️delete-step,🔀reorder-steps,🔧edit-step-params}/{🔺️diff,↩️inverse}/🦀️component.rs` (7 files with real changes; `create-step`'s `↩️inverse` needed no changes) — all 4 triads rewired onto the working-scene + `diff_replace_flow` pattern.
- `…/🧬️schema/💡️inferences/🦀️component.rs` — `infer` rewired through the working scene, `fields()`'s `reads` updated, test fixture builder fixed.
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture.
- `…/📸️snapshot/📝️text/🦀️component.rs` — 2 tests replaced with real converter round-trip laws (documented in place), 3 rejection tests adapted to the new hand-rolled grammar's own error conditions.
- `🎛️apps/📜️imperative/⚙️engine/🦀️component.rs` (`ImperativeHost`) — `path`/`seed` live fields, `sync_document`, every mutating method rewired, 8 test fixes incl. the `load_json` JSON literal.
- `🎛️apps/📜️imperative/🎮️commands/🔧️step/🦀️component.rs` — `next_step_id`/`path_ref_from`/`steps_at` rewired through the working scene.
- `🎛️apps/📜️imperative/📌️panels/{📄️artifact,🔍️inspection}/🦀️component.rs` — working-scene rewiring.
- `🎛️apps/📜️imperative/🎭️modes/✏️edit/🪟️windows/📋️main/🦀️component.rs` — working-scene rewiring.
- `🎛️apps/📜️imperative/🦀️component.rs` — 8 test fixes (working-scene rewiring, `undo_after_add_step_restores_original_document_exactly` rebuilt via `imperative_snapshot_with_content`).

ucas-status: complete
