# 📓️ Wave 5B — FILL is a first-class Tool in puzzle 🖐️5d

Slice: 5B. Crate: `semio-s-artifact-puzzle-5d`. EDITOR5 =
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
Line numbers are as of the last read; siblings edited the same files all day, so treat them as anchors, not addresses.

---

## 1. What landed

### 1.1 The tool module (CREATE)
- **`EDITOR5/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`** (120 lines, new) — the `ToolDefinition`/`ToolRunDefinition`
  ported from EDITOR3 `🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` and trimmed against EDITOR2's precedent:
  - `TOOL_ID = "fill"` (deliberately the SAME string the old `UTILITY_ID` carried, so no document/config field,
    no `setFillCount` payload and no probe id had to be renamed).
  - `RUN_JOB_KIND = "s.puzzle.puzzle5d.fill.run"`, `REVALIDATE_JOB_KIND = "s.puzzle.puzzle5d.fill.revalidate"`
    (unchanged from the retired utility, so the framework ledger keeps resolving the same job kinds).
  - `run_definition()`: `mutating: true`, `rebase: Revalidate`, `reconfigure: Resume`, run + revalidate job pair,
    `ToolRunSettingsReads { config: ["/fillCount", "/contactTolerance", "/objectKindWeights", "/vortexKindWeights"] }`.
    **Config-field names, not verb names** — 5A1 renamed the *verbs* to `setPartKindWeight`/`setGripKindWeight`
    (landed, verified in `☑️options/🖌️brush/🦀️.rs:60,75`) but left `Puzzle5dConfig.object_kind_weights` /
    `.vortex_kind_weights` (`🎚️config/🦀️.rs`) as they were, so these four JSON pointers are the ones that exist.
  - `count_measure()` — `WindowMeasure::Number{id:"puzzle5d-fill-count", min:0, max:None, step:1,
    loading: live_fill_run(tool_run).map(|_| true), on_change: setFillCount}`. `max: None` is the product
    decision (visible stall, never a silent clamp); the `loading` tie is 3d's, which 2d deliberately lacks.
  - `distribution_group()` — the part-kind / grip-kind weight trees, and `measures()` = `[count, distribution]`.
  - `live_fill_run()` / `abort_action()` (moved here from the retired utility module).
- **`…/🛠️tools/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`** (100 lines, new) — 7 laws: mutating/Revalidate/Resume + both job
  kinds; declared trace kind == what the bridge emits; settings read only planner inputs (and no camera);
  the whole stage/counter/reason vocabulary is declared; unbounded count entry; **measures carry kind rows on a
  document with no `kindCatalogs` (concrete forest)**; abort offered only for a live run.

**Trace kind decision.** `Puzzle5dPlannerBoard::translate()` (`🧠️precompute/🦀️.rs`, the `ToolRunTraceOp::Upsert`
loop) emits an `Instance3d` subject as the PRIMARY record and pairs each one with a `Placement2d` **twin** under
`key | PUZZLE5D_PLANNER_TRACE_TWIN_BIT`. `ToolRunTraceKind` has no dual variant (`Instance3d | Placement2d |
Entity | None`, `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:1799-1804`), so the declaration stays
`ToolRunTraceKind::Instance3d` — exactly what the sibling brush run already declares — and BOTH panes still
paint, because the twins travel inside the tick, not inside the declaration.

### 1.2 The utility rail is gone (3d's split mirrored)
3d: fill = Tool; brush / transform / relocate = Utilities. 5d now matches.
- **DELETED** `EDITOR5/🎭️modes/✏️edit/🪟️windows/◻️2d/🪛️utilities/🪣️fill/` (the old `UtilityDefinition`) and
  **DELETED** `EDITOR5/🎭️modes/✏️edit/☑️options/🪣️fill/` (its Utility-Options group, which could never surface
  again once no `fill` utility can be active).
- `🗿️artifacts/🖐️5d/🦀️.rs`: the two `#[path]` mod rows removed; new `modes::edit::tools::fill` mod at `:2244`;
  new `commands::engagement_repeat_last` mod at `:2120`.
- `🪟️windows/◻️2d/🦀️.rs` / `🪟️windows/🧊️3d/🦀️.rs`: `fill` dropped from each window's `utilities` vec;
  `window_measures(envelope, labels)` lost its now-dead `tool_run` parameter and its `mode_options::fill` row.
- `🎭️modes/✏️edit/🦀️.rs`: imports `modes::edit::tools::fill`; Escape still binds `fill::abort_action(tool_run)`.
- `⚖️` `☑️options/🖌️brush/🦀️.rs`: `distribution_children` made `pub` so the tool and the brush options share ONE
  kind-row builder (3d shares `puzzle3d_distribution_group` the same way) instead of a copy.

### 1.3 Registration and dispatch (EDITOR5 `🦀️.rs`)
| registry | change |
|---|---|
| `create_puzzle5d_app` | `.tool(fill_tool::definition(...))` `:9787` + `.mode_tools(edit::PUZZLE5D_PLAY_MODE_EDIT, [ToolRef::new("fill")])` `:9788`; `.utility(board2d::utilities::fill::…)` removed |
| `build_tool_run_job` | `:9064` — explicit `fill_tool::TOOL_ID` arm, default arm is now `Ok(None)` (was: everything-that-isn't-brush fell through to fill) |
| `tool_measures` | NEW hook `:9437` — `{ "fill": fill_tool::measures(envelope, labels, doc.tool_run()) }` (3d's shape) |
| `puzzle5d_scene_active_utility` | `:~930` — now prefers `ViewModel::active_tool_id` (new `puzzle5d_fill_tool_active`, `:927`), so an armed fill TOOL outranks either pane's utility |
| `handle_action_impl` | new `tool_run` parameter; emits `Effect::SetActiveTool{fill}` on entering fill `:4429` and `SetActiveUtility` otherwise — 3d's exact rule (never an empty tool effect, which would bounce-disarm a just-armed run) |
| `Puzzle5dActionCtx` | new `tool_run: Option<&ToolRunView>` field |
| `Puzzle5dWindowCommandWork` | new `tool_run` field + `with_tool_run(...)`; `build_tool_job` binds it for `engagementAbort` |
| `puzzle5d_command_variants!` | `EngagementRepeatLast = "engagementRepeatLast"` `:4070`, `CycleBrushCandidateBack = "cycleBrushCandidateBack"` `:4097` |
| `PUZZLE5D_RETAINED_TOOL_IDS` | both ids added `:4535,:4542` |
| `PUZZLE5D_WINDOW_TOOL_IDS` | both ids added `:4608,:4612` |
| `TOOL_JOB_IDS` | now `&PUZZLE5D_TOOL_JOB_IDS` `:4147` — a const-fn concat `:4134` of the retained list **plus `setActiveTool`/`setActiveUtility`** (drift-proof; 3d spells its list out by hand) |
| `PUBLICATION_CONTRACTS` | `cycleBrushCandidateBack`→WindowTransient `:8523`; `engagementRepeatLast`→WindowTransient `:8530`; `engagementSubmit` widened to `[Artifact, WindowConfig, WindowTransient, Interaction]` (the new grammar moves parts, frames cameras and clears the selection) |
| `bounded_first_step_tool_proofs!` | the retained block moved out of the trait impl into `struct Puzzle5dRetainedCommandProofs` `:8883` (+ both new ids `:8897,:8904`); **NEW second block** `struct Puzzle5dHostConfigurationProofs` `:8975` — generic proofs (no `factory_type`), `factory: "BoundedFirstStepCommandJobFactory"`, `contract: resumable(8_192, 8, 1, 8_192, 7_500, 1, 1)`, `tools: ["setActiveTool","setActiveUtility"]`; the trait method `:9073` concatenates them, exactly like EDITOR3 `🦀️.rs` ≈7605-7641/7851 |
| `.action_with` / classification | `engagementRepeatLast` mutation `:9634` + Migrated `:9707`; `cycleBrushCandidateBack` view action `:9665` + Migrated `:9701` |
| keybindings | NEW `:9604-9610` — `escape`→engagementAbort, `delete`/`backspace`→deleteSelection, `mod+d`→duplicateSelection, `tab`→cycleBrushCandidate, `shift+tab`→cycleBrushCandidateBack, `f`→focusSelection (5d had ZERO keybindings before) |
| `command_from_action` | unchanged — it routes through the macro-generated `try_from_action`, so both new ids are covered automatically |

`host_configuration_mutation` is deliberately NOT overridden: the framework default already returns `Ok(None)`
and 3d's override is the identical no-op. The proofs, not the hook, were the missing half.

### 1.4 Engagement grammar, repeat-last, abort
- **`🎮️commands/📨️engagement-submit/🦀️.rs`** (rewritten): `PUZZLE5D_ENGAGEMENT_VERBS =
  ["select", "brush", "fill <n>", "clear", "zoom", "move dx dy [dz]", "rotate deg", "scale f"]`.
  `fill [<n>]` arms the TOOL, pushes `set_fill_count::request(count)` and dispatches
  `TOOL_RUN_START_ACTION_ID{toolId:"fill"}` — one submit both retargets and starts, 3d's behaviour.
  `select`/`brush` switch the utility (3D `select` → `move` gumball), `clear` empties the `vortex` domain through
  `ctx.clear_selection()`, `zoom` calls `focus_selection`, and `move`/`rotate`/`scale` reuse the EXISTING
  `translate_selection` / `rotate_selection` / `scale_selection` arms (synthesised args) so a typed transform and
  a dragged gumball are literally the same edit. `rotate <deg>` turns about world Z.
- **`🎭️modes/✏️edit/🦀️.rs`**: the placeholder is now `PUZZLE5D_ENGAGEMENT_VERBS.join(", ")` — derived, so an
  advertised verb no arm parses cannot exist (the defect 3d still carries with `pick`/`rectangle`/`lasso`).
  `on_repeat_last` is wired for the first time.
- **`🎮️commands/🔂️engagement-repeat-last/🦀️.rs`** (new): `fill_count + 1` via `set_fill_count::request`, so the
  `Resume` policy continues the same deterministic sequence for exactly one more part; aborts otherwise.
- **`🎮️commands/🛑️engagement-abort/🦀️.rs`** (rewritten): clears the engagement line AND the brush candidate
  index, and when the fill TOOL is armed dispatches `toolRunAbort{runId,generation}` for the live run plus
  `Effect::SetActiveTool{""}` — i.e. Escape **aborts a live run** instead of merely disarming.
- **`🎮️commands/🔁️cycle-brush-candidate/🦀️.rs`**: added `cycle_brush_candidate_back` (`shift+tab`) sharing one
  wrapped-ring helper with the forward cycle.
- **`🎮️commands/🧮️set-fill-count/🦀️.rs`**: added `request(count) -> Effect` (3d's helper) so the count always
  lands on the lane a live run's declared settings watch.

### 1.5 Fixtures
- `EDITOR5/🧠️precompute/🪣️fill/🧫️fixtures/🎞️fill-run.json` — `laws.reconfigure.lowered = 1` added.
- `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` — `toolIds` regenerated from `PUZZLE5D_RETAINED_TOOL_IDS`
  (30 → 65 ids). **This was a whole-fleet repair**, not only my two ids: the fixture had drifted behind 5A2/5D/5E.
  `evidenceToolIds` unioned to the same 65.
- `PLUGIN/🧫️fixtures/🔏️publication-authority/🔣️.json` — `cycleBrushCandidateBack` + `engagementRepeatLast` in the
  window-transient group; `engagementSubmit` widened to `artifact, window-config, window-transient, interaction`.
- **No schema twin change**: this slice added verbs and a tool declaration only. No config, transient or
  mutation SHAPE moved, so `🧬️schema` `.ts/.json/.graphql/.proto` were correctly left alone.

### 1.6 Laws for the fill bridge (`EDITOR5/🧠️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`)
The end-to-end bridge laws already existed and were re-pointed from `UTILITY_ID` to `fill_tool::TOOL_ID`
(`job()` builder, `start()` → `toolRunStart{toolId}`), which is exactly the "through the real ToolRun job pair"
path the slice asked for. Added:
- `lowering_the_fill_count_during_a_run_retracts_the_tail_of_the_same_run` — same `runId`, provisional count
  drops to the lowered value, committed document untouched (the raise twin already existed).
- `a_document_past_the_planner_capacity_refuses_with_a_visible_danger_step` — **capsule-dream (2 880 parts)
  against 3d's `DOCUMENT_OBJECT_SLOTS` = 2 048**: asserts the run emits a `Danger` step carrying
  `FillRunReason::ArtifactCapacity` BEFORE the declared `preparation-capacity:fixture-objects:2048` refusal, and
  places nothing. ⚠️ Note the contract 3d actually implements: the oversized document produces a **visible
  danger step and then a declared fault** — not a notice-and-continue. That is the shipped 3d semantics
  (`PreparationCapacityRefusal`), and the existing interactive law already pinned it; I pinned the ordering.
- `fill_run_finalize_publishes_one_edit_with_every_provisional_placement` strengthened: it now asserts every
  published part carries **BOTH poses** (`["2d"].x/.y` and a 3-element `["3d"].origin`) and its grips, and that
  `fasteners` grew by one per placement — still ONE undo entry (undo removes all, redo restores all).
- Concrete forest and nakagin both run through the fixture case list (`concrete-forest-8` → 8 placed,
  `nakagin-12` → 0 placed, stall `no-open-vortex`; nakagin has no open grip, so "parts created" is not
  assertable there — the planner-oracle equality and the interactive µs law cover it instead).

### 1.7 Editor-root laws touched (`EDITOR5/🧪️tests/🔬️unit/🦀️.rs`)
- `fill_and_brush_params_are_tagged_utility_options_not_engagement_controls` → renamed
  `fill_is_tool_options_and_brush_is_utility_options_never_engagement_controls`: fill count is now asserted to be
  IN `fill_tool::measures` and ABSENT from both windows' `window_measures`; brush stays a `"brush"`-tagged rail group.
- `fill_count_entry_is_unbounded_and_defaults_to_one_hundred` re-pointed at the tool measures.
- NEW `fill_is_registered_as_a_mode_tool_and_no_longer_as_a_utility` — one `.tool` with a run, listed in the edit
  mode's tools, and no window binds `fill` as a utility.
- NEW `engagement_placeholder_advertises_exactly_the_parsed_verbs` — placeholder ≡ `PUZZLE5D_ENGAGEMENT_VERBS`
  and `on_repeat_last` is present, for both windows.

---

## 2. Commands run — real verdicts

All outputs in `TICKET/🗑️generated/5B/`. Every invocation used `CARGO_INCREMENTAL=0`.

| command | verdict |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --message-format=short` | ✅ **EXIT 0**, `Finished dev profile in 1m 18s`, **0 errors**, `semio-s-artifact-puzzle-5d (lib) generated 3 warnings` (down from 9 earlier today; the one that was mine — `unused import: ToolRunView`, `🪟️windows/◻️2d/🦀️.rs:14` — is fixed. The remaining 3 are siblings' `unnecessary qualification` rows). `check-native-final.txt` |
| same `--target wasm32-wasip2` | ✅ **EXIT 0**, `Finished dev profile in 2.83s`, **0 errors**, 3 warnings. `check-wasm-final.txt` |
| `bun ./📜️script.ts publication-authority-audit Puzzle5dPlayApp` | ✅ **EXIT 0** — `validated Puzzle publication authority; owners=Puzzle5dPlayApp; … windowOwnershipCases=7; schema=Ajv; oracle=independent`; the admitted list contains `cycleBrushCandidateBack` and `engagementRepeatLast`, and `engagementSubmit` under its widened lane set. `publication-audit-final.txt` |
| `cargo test … --lib -- fill` | ⚠️ **EXIT 101 — 10 passed, 9 failed, 550 filtered out (3.40s).** The test target now COMPILES (it did not, earlier today). `test-fill-final.txt` |

### 2.1 Which fill laws pass and which are blocked
✅ **Passing (10)** — every law that does not need a shipped example document:
`tools::fill::tests::{fill_run_declaration_is_mutating_revalidated_and_resumable,
fill_run_declares_the_trace_kind_the_planner_bridge_emits, fill_run_settings_read_only_the_planner_inputs,
fill_run_declares_its_whole_step_vocabulary, fill_count_measure_is_unbounded_and_reads_the_shared_count,
abort_action_is_offered_only_for_a_live_fill_run}`,
`component::unit_tests::{fill_is_registered_as_a_mode_tool_and_no_longer_as_a_utility,
fill_is_tool_options_and_brush_is_utility_options_never_engagement_controls,
fill_count_entry_is_unbounded_and_defaults_to_one_hundred}`,
`terminology::tests::the_fill_run_vocabulary_equals_the_planner_schema_table_…`.

❌ **Blocked (9), all by ONE regression outside this slice.** Every failing law needs
`concrete_forest_example_document()` / `nakagin_example_document()` / `capsule_dream_example_document()`, and
**all three shipped `.dsl.semio` example assets no longer parse**:

```
examples::puzzle5d::concrete_forest_tests::dsl_asset_parses_and_round_trips ... FAILED
  example dsl parses: TextError { message: "expected List, found Absent",
                                  span: TextSpan { line: 1, column: 1, length: 0 } }
```
(identical for `nakagin_capsule_tower_tests` and `capsule_dream_tests`; `test-example.txt`.) The
`Puzzle5dSnapshot` DSL grammar gained a `#[dsl(table)] target_volumes` field
(`🧬️schema/📸️snapshot/🦀️.rs:~55`) and the three example assets under `📚️examples/*/🖼️assets/*/🗣️.dsl.semio`
were not regenerated with the new table, so `parse_example_dsl` yields an EMPTY document. That is why my fill
laws see zero parts, zero kind rows and zero trace records — not a fill defect. 15 other non-fill laws
(`set_active_example_*`, `clipboard_verbs_cover_every_part_of_the_largest_example`, `inference_determinism_law`,
`flatten_matches_golden_poses_to_1e4`, …) fail on the same root cause.

The failing nine: `precompute::fill::tests::{fill_run_job_matches_the_language_neutral_fill_run_fixture,
fill_revalidate_job_translates_provisional_placements_and_their_conflicts,
fill_run_job_step_stays_below_the_interactive_ceiling_on_the_largest_examples,
fill_run_finalize_publishes_one_edit_with_every_provisional_placement,
aborting_a_fill_run_leaves_the_document_byte_identical,
raising_the_fill_count_during_a_run_reconfigures_the_same_run,
lowering_the_fill_count_during_a_run_retracts_the_tail_of_the_same_run,
a_document_past_the_planner_capacity_refuses_with_a_visible_danger_step}` and
`tools::fill::tests::fill_measures_carry_the_count_and_kind_rows_even_without_authored_catalogs`.

### 2.2 One shared fix I did make
The first symptom on that path was a hard fault, not an empty document:
`puzzle5d-planner-snapshot: kindCompatibility.expected an array, found Null`. `Puzzle5dDocument`
(EDITOR5 `🦀️.rs:~379-382`) serialised `kindCatalogs` / `kindCompatibility` as JSON `null` when absent, while the
schema snapshot declares them as a non-`Option` `Vec`/child with `#[value(default)]` — a key that is omitted
decodes, a key that is `null` does not. I added `skip_serializing_if = "Option::is_none"` to both fields. That
removed the fault crate-wide; the DSL-asset regression above is what is left.

**NOT verified by me:** the eight end-to-end fill-bridge assertions (finalize = ONE edit with both poses,
abort, raise, lower, capacity refusal, planner-oracle equality, interactive µs ceiling) and the kind-rows law.
They compile and run; they cannot reach a non-empty document until the example assets parse again. **Owed to
integration**, and they should go green with no further 5B work once the assets are regenerated.

---

## 3. For the battery / probes — exact ids

- Tool activation control: `#tool.fill` (tool id **`fill`**), via `setActiveTool{toolId:"fill"}`.
- Tool run controls are framework-generic (`toolRunStart` / `Pause` / `Step` / `Abort` / `Finalize` / `Dismiss`).
- Tool options rail id: measure `puzzle5d-fill-count`; distribution group `puzzle5d-play-fill-distribution`,
  whose children are `puzzle5d-play-suggestion-parts` / `-grips` with rows
  `puzzle5d-play-part-kind-<kindId>` / `puzzle5d-play-grip-kind-<kindId>` (verbs `setPartKindWeight` /
  `setGripKindWeight`, 5A1's naming).
- Engagement input ids: `puzzle5d-engagement-puzzle5d-2d` and `puzzle5d-engagement-puzzle5d-3d`; placeholder
  text is exactly `select, brush, fill <n>, clear, zoom, move dx dy [dz], rotate deg, scale f`.
- Status line ids: `puzzle5d-status-<windowKind>`.
- New verbs to exercise: `engagementRepeatLast`, `cycleBrushCandidateBack`.
- Keys: `escape` (aborts a LIVE fill run, not just the input), `tab` / `shift+tab`, `delete`, `backspace`,
  `mod+d`, `f`.
- Run job kinds for trace/lane assertions: `s.puzzle.puzzle5d.fill.run`, `s.puzzle.puzzle5d.fill.revalidate`.

---

## 4. Hand-offs

- **5C (kind rows).** No `inferred_part_kind_rows` was needed and none was added: the fallback ALREADY exists on
  both sides. UI rows — `puzzle5d_kind_ids(document, "parts"|"grips")` (EDITOR5 `🦀️.rs`, `//#region 🔖️Distribution`)
  already derives one row per distinct `partKind` / `gripKind` present when `kindCatalogs` is absent. Planner —
  `puzzle3d_kind_catalogs(document, authored)` (`🧠️precompute/🦀️.rs`) already synthesises a 3d catalog from the
  document's own kinds when it authors none. My tool law
  `fill_measures_carry_the_count_and_kind_rows_even_without_authored_catalogs` pins the UI half on the concrete
  forest (which genuinely has no `kindCatalogs`). **5C: please do not add a third mechanism**; if the catalogue
  panel needs rows, call `puzzle5d_kind_ids`.
- **5A1.** I consume `☑️options/🖌️brush::distribution_children`, which I made `pub`. If you move it, repoint
  `🛠️tools/🪣️fill/🦀️.rs::distribution_group`. The tool's `settings.config` pointers are CONFIG FIELD names
  (`/objectKindWeights`, `/vortexKindWeights`) — they must track `Puzzle5dConfig`, not your verb renames.
- **5E.** Both `board2d::window_measures` and `world3d::window_measures` lost their `tool_run` parameter (fill was
  their only consumer). `✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:96,109,110,111` is still red on your new
  `grid_visible`/`selectable_kinds`/`voxel_dims`/`projection`/`up` fields.
- **5G — BLOCKING, please read.** Adding `#[dsl(table)] target_volumes` to `Puzzle5dSnapshot`
  (`🧬️schema/📸️snapshot/🦀️.rs`) broke the DSL grammar for all three shipped example assets
  (`📚️examples/{🌲️concrete-forest,🏢️nakagin-capsule-tower,💊️capsule-dream}/🖼️assets/*/🗣️.dsl.semio`):
  `expected List, found Absent` at 1:1. Every example-driven law in the crate — mine and ~15 others — is red
  until those assets carry the new table (or the field is declared optional in the grammar). This is the single
  biggest integration blocker I hit.
  Note also `🧠️precompute/🪣️fill/🦀️.rs::puzzle3d_ops` only maps `CreatePart`/`ConnectGrips`; any new provisional
  mutation class from target volumes needs an arm there or the bridge errors `puzzle5d-fill-run-provisional`.
- **Schema owner.** I added `skip_serializing_if = "Option::is_none"` to `Puzzle5dDocument.kind_catalogs` and
  `.kind_compatibility` (§2.2). If the editor document ever gains another `Option` field whose schema twin is a
  non-`Option` with `#[value(default)]`, it needs the same attribute or every round-trip faults on `null`.
- **Integration.** Run `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- fill`
  once the crate's test target is green; that filter covers this slice's laws plus the pre-existing
  fixture/oracle/interactive laws.
- **Shared-file note.** I regenerated `🗄️retained-jobs/🔣️.json`'s `toolIds` from the const for the WHOLE crate.
  Anyone who adds a retained tool id after me must re-sync it, or
  `language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle` fails.

---

## 5. Not done / deliberately out of scope

- **Target volumes for 5d** (E5 §3.4c) — not this slice; 5G owns them.
- The `capsule-dream` capacity path ends in a **declared fault after a visible danger step**, matching 3d. If the
  product wants a notice-and-stall instead, that is a change in 3d's `PreparationCapacityRefusal`, not in 5d.
- 3d's `ToolRunRetargetableJob` in-place retarget is reached for free: `🧠️precompute/🪣️fill` delegates to
  `fill3d::build_run_job`, which returns 3d's retargetable job wrapped by `Puzzle5dPlannerToolRunJob`.
