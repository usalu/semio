# Wave B9 — the five guest halves of B6's lane diagnosis

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, 2026-09-11. Continues
`📓️2026-09-11-wave-B6-lane-diagnosis.md` (host halves) and `📓️2026-09-10-wave-P5-lanes-render.md`.
Everything below was run in the foreground on the live tree; every command's tail is quoted.

## 0 Verdict summary

| Lane | Verdict | Root cause (file:line) | Law | Probe verdict expected to flip on #45 |
|---|---|---|---|---|
| 1 utility publication instance scope | **FIXED (guest)** | `✏️editor/🦀️.rs:7988` — `render_body` resolved the window from the roster's FIRST pane | `each_window_instance_publishes_its_own_armed_utility_into_its_world_lane` | `brush-preview-place` |
| 2 first pick invisible to the render | **FIXED (guest), class removed** | `✏️editor/🦀️.rs:216` + `:7755` — `meta` projected `null`, one refused typed decode emptied the WHOLE document the topology is built from | 3 laws (below) | `inspection-object-fields`, `inspection-locked-flag-row` |
| 3 `importFixture` fold | **NOT a guest fold** — excluded by construction; regression law added | see §3 | `exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity` | `import-distinct` (expected already-correct) |
| 4 world surface not dirtied after a mutation | **FIXED (host)** | `🏛️ShellHost/🟦️.tsx:4844` — the direct browser-actor route dropped the dispatch's dirty scope entirely | `dirties the whole shell when a direct browser-actor dispatch applied a mutation` | `clipboard`, `gumball-scene-delta` |
| 5 `framework.panel.history: AliasCapacity` | **FIXED (host Rust)** | `🔌️plugin/🦀️.rs` `ui_history_panel` — the alias table was not paged with the rows | `ui_history_panel_bounds_revert_row_actions_to_the_arena_page` | the `AliasCapacity` fault itself |

---

## 1 Utility publication is keyed by window INSTANCE now

### Root cause

`plugin_refresh_ui` (`🔌️plugin/🦀️.rs:32089`) renders EVERY window instance under the window KIND's one
body key — the shell sends `windowKinds[].n` per instance (`🛠️ShellHelpers/🟦️.tsx`'s
`buildUiRefreshRequest`: `windows = windowInstances.map((instance) => ({ key: instance.id, n: instance.n, … }))`) —
and carries the instance identity ONLY through `ViewModel::for_window_instance(entry.key)`, i.e. in
`view_state.window_id`.

`Puzzle3dPlayApp::render_body` (`✏️editor/🦀️.rs:7988`) ignored that field:

```rust
let wid = window_id_from_key.or_else(|| config.window_ids.first().map(String::as_str)).unwrap_or(main::WINDOW_KIND_ID);
```

`config.window_ids` is the whole live roster (`🪟️window/🦀️.rs:267` — `view.window_instances`), so every
pane resolved the FIRST pane and looked `active_utility_by_window_id` up under it.
`puzzle3d_scene_active_utility`'s `.or_else(|| view.active_utility_id)` fallback hid this whenever the
first pane was unarmed (map miss → the narrowed view's own utility), which is why it only presented as
`utility=select preview=null` once more than one pane was armed. The same defect ran on the reducer side:
`ArtifactEditor::handle` and `puzzle3d_retained_reduce` passed `command.window_id()` — the `windowId`
ARGUMENT, which B6 measured as the bare kind `puzzle3d-main` — instead of the host's own
`view_state.window_id`.

### Fix

`✏️editor/🦀️.rs` — new `puzzle3d_addressed_window_id(view_state, keyed, fallback, roster)`: the explicit
`<body>:<instance>` body key, then `ViewModel::window_id`, then the caller's own fallback, then the
roster's first pane. Used by `render_body`, `ArtifactEditor::handle` and `puzzle3d_retained_reduce`.
`Puzzle3dWindowCommandWork::step` already used that order and is unchanged.

### Law

`🧪️tests/🔬️unit/🦀️.rs` → `each_window_instance_publishes_its_own_armed_utility_into_its_world_lane`,
plus the testkit's new `render_window_refresh(app, body_key, window_id)` — the ONLY render shape that
reproduces the host's own `plugin_refresh_ui` call (kind body key + instance-narrowed view; the
`<body>:<instance>` form the pre-existing `render_window` uses is the other host route and hides this).
Three panes, each with its own utility; the armed pane must publish `brush` AND a `brushPreviewJson`,
the unarmed pane `select` and no preview.

```
test editor::puzzle3d::component::tests::each_window_instance_publishes_its_own_armed_utility_into_its_world_lane ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 676 filtered out; finished in 1.75s
```

Real coverage — with `render_body`'s line reverted the same law FAILS on the exact browser symptom:

```
[DEBUG] puzzle3d.brushPreview.lane utility=volumeBrush preview=0 vortices=2639
assertion `left == right` failed: the pane the user armed must publish its OWN utility
  left: "volumeBrush"
 right: "brush"
test result: FAILED. 0 passed; 1 failed
```

---

## 2 First pick invisible to the guest render

### Root cause — two decodings of one document, one of which fails whole

B6 proved the boundary (the leftover carries the pick, the panel renders the pre-pick hash for 73 s) and
left the last line open between "topology pruning" and "interaction-store round trip". It is the first,
and the mechanism is a silent whole-document substitution inside the guest:

- `Puzzle3dPlaySnapshot::new` (`🧬️schema/🧬️mutations/🦀️.rs:429`) builds the TYPED authority with
  `dsl::FromValue::from_value(...).unwrap_or_default()` while keeping the supplied projection verbatim.
  A refused typed decode therefore yields a snapshot whose `value()` is the whole document and whose
  `typed()` is the EMPTY document.
- `Puzzle3dPlayApp::interaction_topology` (`✏️editor/🦀️.rs:7755`) read `doc.snapshot.typed()`, while every
  render reads `puzzle3d_projection_value(doc.snapshot.value())`. `protocol::validate_state`
  (`📡️replication/📡️wire/🦀️.rs:2616`) prunes every selected id the topology does not contain — so an empty
  typed half means the world paints the object and the pick is dropped one hop later, with no error
  anywhere. That is exactly the workaround `leftover_interaction_view_from`'s own comment describes.
- `PartialEq for Puzzle3dPlaySnapshot` compares the TYPED half only, so `assert_dsl_pack_equivalence`
  reads two empty documents as equal and no round-trip law in the tree could see this.

The live trigger: `Puzzle3dFixtureMeta` (`✏️editor/🦀️.rs:216`) typed both catalog members as
`Option<DslValue>` without `skip_serializing_if`, so an ABSENT member projected as `null`, and the
persisted twin `Puzzle3dMeta` types `kindCompatibility` as a real array and refuses a `Null` for it.
Measured directly:

```
[DEBUG] b9 meta empty meta={"kindCatalogs":null,"kindCompatibility":null}
        err=Some(ValueError("meta.kindCompatibility.expected an array, found Null"))
[DEBUG] b9 seeded typed objects=0 projection objects=1
```

`empty_fixture()` produces exactly that shape (and the pre-existing
`puzzle3d_typed_fixture_matches_the_projection_bridge_for_every_example` docstring already records the
symptom without naming its consequence). Any document reaching the guest through `parse_dsl` /
`decode_pack_with` / `FromValue` / `initial_snapshot` with a meta the typed side refuses loses its whole
pickable universe.

### Fix

- `✏️editor/🦀️.rs` — `Puzzle3dFixtureMeta`'s two members gain `skip_serializing_if = "Option::is_none"`:
  an absent member projects as absent, never as `null`.
- `✏️editor/🦀️.rs` — new `puzzle3d_fixture_from_projection(&Value)`, now the ONE read behind both
  `Puzzle3dPlayApp::render_fixture` and `interaction_topology`. The pickable universe and the painted
  document are one answer; no typed-decode failure can narrow one without narrowing the other.

### Laws

`🧪️tests/🔬️unit/🦀️.rs`:

- `an_absent_fixture_meta_member_never_empties_the_typed_authority`
- `interaction_topology_names_every_id_the_world_lane_paints` — measured against a deliberately
  divergent snapshot (typed half refused, projection whole)
- `play_snapshot_typed_authority_survives_every_store_round_trip` — pack, dsl and projection rebuild
- `first_pick_of_a_fresh_session_renders_the_object_and_its_lock_row` — the end-to-end hop: first
  `interactionSelect` of a fresh session → the object id and `puzzle3d-play-inspector.object.locked`
  reach the panel, and a `setSelectionFlag locked` on that same selection lands on that object

```
test editor::puzzle3d::component::tests::an_absent_fixture_meta_member_never_empties_the_typed_authority ... ok
test editor::puzzle3d::component::tests::interaction_topology_names_every_id_the_world_lane_paints ... ok
test editor::puzzle3d::component::tests::play_snapshot_typed_authority_survives_every_store_round_trip ... ok
test editor::puzzle3d::component::tests::first_pick_of_a_fresh_session_renders_the_object_and_its_lock_row ... ok
```

Real coverage — with both fixes reverted:

```
test ..::an_absent_fixture_meta_member_never_empties_the_typed_authority ... FAILED
test ..::interaction_topology_names_every_id_the_world_lane_paints ... FAILED
```

B6's host witnesses (`interaction selection lost dispatched=… validated=… readback=…` and
`interaction-store snapshot unavailable`) are KEPT for #45 — they are the one-line confirmation that the
browser's instance of this was the pruning branch and not the store branch.

---

## 3 `importFixture` — the fold is excluded by construction

B6 measured `performInvocation settled {actionId:"importFixture", frames:2, historyUpserts:0, effects:0}`
with the document unchanged. **`effects:0` rules out every refusal branch**: `import_fixture`'s three
aborts all go through `Puzzle3dActionCtx::notice` (`✏️editor/🦀️.rs:2628`), which pushes an
`Effect::Notify` unconditionally, and `dispatch_step`'s abort arm deliberately KEEPS `effects` while
dropping the mutations. A settled `importFixture` with zero effects therefore parsed its payload, ran to
`ctx.scene.fixture = fixture`, and produced an empty delta — i.e.
`puzzle3d_operations_from_fixture_change`'s `before_snapshot == after_snapshot`. That is the IDENTITY
signature, not a fold, and it agrees with the coordinator's own 19:24 note
(`📓️2026-09-10-cursor-coordination.md`): *"Import: NOT a defect … Distinct 2-object fixture
browser-proven: instances 1→2 + create-object history row. Import round trip fully CLOSED."*

The native round trip through the app's OWN export bytes — the exact browser gesture, not a
hand-assembled projection — is correct in both directions:

```
[DEBUG] puzzle3d.import.ingress args=true payload_len=7542
[DEBUG] puzzle3d.import.parsed objects=1 before=2
[DEBUG] puzzle3d.import.apply ops=1 after_objects=1
[DEBUG] puzzle3d.import.ingress args=true payload_len=7750
[DEBUG] puzzle3d.import.parsed objects=2 before=1
[DEBUG] puzzle3d.import.apply ops=1 after_objects=2
[DEBUG] puzzle3d.import.ingress args=true payload_len=7750
[DEBUG] puzzle3d.import.parsed objects=2 before=2
[DEBUG] puzzle3d.import.apply ops=0 after_objects=2
```

### Law

`exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity`: export A (1 object),
export B (2 objects), restore A, import B → no `Notify`, one history row, two objects; re-import B →
no `Notify`, the document is unchanged.

```
test editor::puzzle3d::component::tests::exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 681 filtered out; finished in 9.68s
```

Note on the law's shape: the third import's `history_patch` is NOT a usable identity witness in-process —
a retained verb answers before it runs, so an admission carries the PREVIOUS row. The document (object
cores) is the honest measurement and is what the law asserts.

**Open on #45, one line:** B6's two ingress/parse taps decide whether the probe's `…-distinct.json` was
distinct at all. `objects=N before=M` with `N != M` and `apply ops=0` would be a real fold; `N == M` with
equal content is the correct identity no-op and closes the lane. **Kept deliberately** — see §7.

---

## 4 A document mutation now dirties the shell in the same turn

### Root cause

`ShellHost.dispatchDirectBrowserActorCommand` (`🏛️ShellHost/🟦️.tsx:4821`) — the route every live
document action actually takes — awaited the actor result, published its host effects, and returned.
It never applied a dirty scope. Every other dispatch path ends in
`applyHostEffects(…, resolveUiDirtyScope(response.uiScope), …)` → `refreshUi(nextSession, uiScope)`.

A typed operation still repaints later through `subscribeOperationCompletions` (`:5236`), but a
framework-reserved verb that commits INLINE — `paste`, `undo`, `interactionSelect`, a gumball commit —
publishes no `OperationCompleted` frame at all, so nothing on this route ever asked for a refresh. That
is B6 §3's +12 s world lane and the `gumball-scene-delta FAIL sceneDelta=false` verdict: the document was
right the whole time and the surface was never re-taken.

The actor handoff's reply (`BrowserActorActionResultV1`,
`🌐️browser-bundle/🎯️action-handoff/🟦️.ts:30`) carries `{outcome, mutationCount, hostEffects}` and NO
`UiDirtyScope` — the guest's own scope does not cross it — so `mutationCount` is the honest signal.

### Fix

- `🛠️ShellHelpers/🟦️.tsx` — new pure `browserActorDispatchUiScopeV1(result)`: `{kind:"full"}` for a
  `guest-applied` result with `mutationCount > 0`, `{kind:"none"}` otherwise. `refreshUi` is
  hash-conditional, so unchanged sections still cost no payload; what this buys is the changed window
  body being re-taken in the same turn.
- `🏛️ShellHost/🟦️.tsx` — `dispatchDirectBrowserActorCommand` applies it through `refreshUi` after
  re-checking `current()`; `refreshUi` joins the callback's deps.

### Law

`🧪️tests/🔬️engine-contract/🟦️.ts` → `"dirties the whole shell when a direct browser-actor dispatch applied a mutation"`.

```
SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/…/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts \
  --testNamePattern='direct browser-actor dispatch applied a mutation'
 Test Files  1 passed | 22 skipped (23)
      Tests  1 passed | 867 skipped (868)
```

Host-side and therefore **vite-live now** — it does not wait on #45.

---

## 5 `framework.panel.history: AliasCapacity` — the alias table pages with the rows

### Root cause

`SurfaceReconcileFault::AliasCapacity` is `UiValue::credited_clone()` returning `None`
(`♻️reconcile.rs:801`). The `UiValue` argument arena is sized by its own declared law
(`🎬️action.rs:54`): `UI_VALUE_PAGE_ROWS = UI_BUILT_CHILDREN_MAX - 1 = 31` INTERACTIVE rows, backed
`UI_VALUE_LIVE_PAGES = 1` page deep, deliberately, because the backing is charged against the resident
aggregate ceiling.

`ui_history_panel` (`🔌️plugin/🦀️.rs:9901`) gives every revertible row a `RowAction` whose
`args: Some(UiValue::Map(...))` claims one of those rows — and the command log grows without bound within
a session. W-AB's earlier fix paged the row NODES by `UI_BUILT_CHILDREN_MAX` arity
(`page_history_command_nodes`, the 32+7 law), but nothing paged the ALIASES. B6 measured the fault at
**31 entries** — exactly `UI_BUILT_CHILDREN_MAX - 1`, the arena's page.

### Fix

`🔌️plugin/🦀️.rs` — the Commands section opens a `PanelRowBudget::new(panel_page_rows())` (the framework's
own sanctioned virtualised-panel mechanism, §🔖️PanelPaging) and spends one credit per inline revert.
Every row still renders and stays reachable under the paged tree; only the inline revert affordance is
bounded by the page the arena actually backs, and `revertToCommand` stays dispatchable for older rows
through the command palette. `panel_page_rows()` clamps on the LIVE headroom, so a process whose other
panels already hold credit yields fewer inline reverts instead of a refusal mid-row.

### Law

`🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` →
`ui_history_panel_bounds_revert_row_actions_to_the_arena_page`: 200 revertible entries assemble, every
row stays reachable, inline reverts stay `<= UI_VALUE_PAGE_ROWS`.

```
test ..::ui_history_panel_bounds_revert_row_actions_to_the_arena_page ... ok
test ..::ui_history_panel_clips_an_oversized_command_label_and_its_folded_count ... ok
test ..::ui_history_panel_clips_an_oversized_operation_description ... ok
test ..::ui_history_panel_filters_rows_and_gates_the_backwards_action ... ok
test ..::ui_history_panel_pages_command_rows_from_the_live_count ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 651 filtered out; finished in 0.04s
```

Real coverage — with `revert_budget.spend()` removed the panel does not assemble at all:

```
panicked at …/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:4160:76:
  200 revertible entries must assemble without an alias refusal
test result: FAILED. 0 passed; 1 failed
```

i.e. the browser's `1:framework.panel.history: AliasCapacity` reproduced natively, and gone.

---

## 6 Verification (all foreground)

| Command | Result |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished dev profile … in 23.47s`, 0 errors, 88 warnings (proof the tree expanded) |
| `cargo check -p semio-framework-plugin` | `Finished dev profile … in 43.10s`, 0 errors, 6 warnings |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Finished dev profile … in 47.27s`, 0 errors |
| `RUST_MIN_STACK=… cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1` | `687 passed; 2 failed` — both pre-existing, see below |
| `… --lib leftover_` | `8 passed; 0 failed` |
| `… --lib inspection` | `5 passed; 0 failed` |
| `… --lib inspector` | `10 passed; 0 failed` |
| `… --lib import` | `8 passed; 0 failed` |
| `… --lib paste` | `3 passed; 0 failed` |
| `… --lib history` | `2 passed; 0 failed` |
| `… --lib interaction` | `10 passed; 0 failed` |
| `… --lib residue` | `1 passed; 0 failed` (B6's law still green) |
| `… --lib puzzle3d_typed_fixture_matches` | `1 passed` (the meta-projection change does not move the differential bridge) |
| `… --lib example_switch` | `6 passed; 0 failed` |
| `cargo test -p semio-framework-plugin --lib ui_history_panel -- --test-threads=1` | `5 passed; 0 failed` |
| `SEMIO_TEST_LEVEL=long bun x vitest … --testNamePattern='direct browser-actor dispatch applied a mutation\|carries the armed utility\|leftover activeUtility'` | `Test Files 1 passed \| 22 skipped`, `Tests 3 passed \| 867 skipped` |
| `SEMIO_TEST_LEVEL=long bun x vitest … '🔌️PluginRuntime' '🔬️engine-contract'` | `Tests 3 failed \| 620 passed (623)` — the same three peer-owned failures B6 and B2 recorded verbatim (`buildNoteShellCommandAction`, `binds two instances of one body to distinct surfaces`, `readAppDocumentPack()` extra `ops` field); none is in this wave's diff |

### The two puzzle3d lib failures, both pre-existing

- `two_instances_converge_disjoint_object_edits_via_backbone` — the fail-closed VCS/backbone remote-merge
  stub, recorded unchanged since `📓️2026-09-09-wave-X-test-suite.md` §7 and again in
  `📓️2026-09-09-remaining-test-failures-audit.md` line 268.
- `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` — a wall-clock
  step-budget law. **A/B-proven not mine**: with all four of this wave's `✏️editor/🦀️.rs` edits reverted
  the same law fails WORSE.

```
with the wave's edits:     worst turn 18 at 4.487916ms / 4.440375ms / 4.389625ms  (budget 2ms)
with the edits reverted:   worst turn 18 at 6.912209ms                            (budget 2ms)
```

## 7 `[DEBUG]` taps

No tap was added by this wave; the two temporary probes used to prove §2 were deleted after they had
produced the quoted output.

Kept for the #45 readout, deliberately:

| Tap | Where | Decides |
|---|---|---|
| `puzzle3d.import.ingress` / `puzzle3d.import.parsed` (B6 §4) | `🎮️commands/📥️import-fixture/🦀️.rs` | §3's one open question: was the probe's file distinct at all (`objects=N before=M`) |
| `puzzle3d.import.apply` (pre-existing) | `✏️editor/🦀️.rs:3280` | the delta the parse produced |
| `interaction selection lost …` / `interaction-store snapshot unavailable …` (B6 §1, host) | `🔌️plugin/🦀️.rs` | confirms the browser's lane-2 instance was the pruning branch this wave fixed |
| `puzzle3d.utility.publish action=… window=… map_hit=…` (pre-existing) | `✏️editor/🦀️.rs:3187` | confirms lane 1's flip in the browser (`map_hit=true` on the armed pane) |
| `puzzle3d.brushPreview.lane` / `.gate` / `.cache` (pre-existing) | `🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` | the brush-preview half of `brush-preview-place` |

Sweep all of them once #45's battery has read them.

## 8 Files touched

| File | Change |
|---|---|
| `✏️s/…/✳️any/✏️editor/🦀️.rs` | `puzzle3d_addressed_window_id` + its three call sites; `Puzzle3dFixtureMeta` projects absent members as absent; `puzzle3d_fixture_from_projection` as the ONE projection read; `interaction_topology` reads it |
| `✏️s/…/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | `render_window_refresh` (the host's `plugin_refresh_ui` render shape) + `rendered_body_value` extraction |
| `✏️s/…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 6 laws (lanes 1, 2 ×4, 3) |
| `🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `browserActorDispatchUiScopeV1` |
| `🧰️framework/…/🧱️elements/🏛️ShellHost/🟦️.tsx` | the direct browser-actor route dirties the shell on an applied mutation |
| `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts` | lane 4 law |
| `🧰️framework/…/🔨️modules/🔌️plugin/🦀️.rs` | `ui_history_panel` pages its alias table with `PanelRowBudget`/`panel_page_rows` |
| `🧰️framework/…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | lane 5 law |
