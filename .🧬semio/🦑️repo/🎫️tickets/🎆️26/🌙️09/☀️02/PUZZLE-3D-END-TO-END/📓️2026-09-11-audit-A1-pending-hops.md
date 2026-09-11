# Audit A1 — #44 pending hops vs HEAD `46c3cb9de0`

Read-only source audit. No builds/tests run (per instructions); verdicts are static-analysis reads of
the working tree at HEAD `46c3cb9de0` (2026-09-11 12:39:02, auto-commit). Cross-checked against
`📓️2026-09-11-claude-coordination.md`'s 14:40 baseline battery (`🗑️generated/probe-2026-09-11T12-10-48.md`,
served wasm still #43 — pre-#44), which is consistent evidence, not contradicting evidence: browser
failures there are expected while the served wasm predates these source changes.

## Git evidence

```
$ git log --date=iso --format='%h %ad' -3 -- '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d'
46c3cb9de0 2026-09-11 12:39:02 +0200
f39d4b0db3 2026-09-10 13:54:54 +0200
6ad7b0e7bc 2026-09-10 01:31:40 +0200

$ git status --porcelain -- '✏️s/🔌️plugins/🧩️puzzle' '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
 M ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📋️project.json
 M ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📋️project.json
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📋️project.json
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📋️project.json
 M 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx
 M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️testkit/🌊️actor-import/📋️project.json
 M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/vitest.config.ts
 M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json
 M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📋️project.json
 M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📋️project.json
 M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🧪️tests/🔬️unit/🦀️.rs
 M 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts
```

Everything in the puzzle 3d guest tree and the plugin host `🦀️.rs` is already **committed** at HEAD
(clean vs HEAD per the coordination doc's own ground-truth line) — the `git status` above shows only
unrelated `project.json`/`vitest.config.ts`/ShellHost noise, confirming `git diff f39d4b0db3 46c3cb9de0`
is the real diff window for everything below.

## Verdict table

| # | Hop | Verdict |
|---|---|---|
| 1 | Guest/host `interactionSelect` leftover publishes `selected` on first pick | **PRESENT** |
| 2 | mesh-mode `translateSelection` pre-admit of the exact operation slot | **ABSENT** |
| 3 | Guest brush preview live-target latch (`session.brush_live_target`) | **PRESENT** |
| 4 | `importFixture` apply/fold — distinct payload upserts + history row | **PRESENT** |
| 5 | Inspection `selected_section` fall-through (object id + vortex uuid) + lock `flag_row` | **PRESENT** |
| 6 | Host `1:window` alias (`windowHostContextBindings`) + leftover Inspection full refresh | **PRESENT** |
| 7 | Host bridge worker fault `v102_1` fix | **PRESENT** |
| 8 | History-panel command paging (32+7 law) | **PRESENT** |

7 of 8 hops are landed in source at HEAD. Only hop 2 (mesh-mode translate pre-admit) has no
corresponding code change — and the 14:40 baseline battery run in this same session (still on the #43
wasm) shows this exact fault (`fixed typed-operation and segmented-output authorities did not
pre-admit the exact operation slot`) firing 16× today (suggestionsTick ×10, engagementAbort ×3,
setCamera ×2, transformBegin ×1) and names it "the convergent blocker" — **Wave B0 has just been
launched on it in this same coordination session**, in parallel with this audit.

---

## 1. Guest/host `interactionSelect` leftover — PRESENT

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:

- `dispatch_interaction_action` (line 21605), `INTERACTION_SELECT_ACTION_ID` arm (line 21610–21622)
  mutates `state.selection` via `protocol::next_selection` on every pick, including the first.
- Line 21682: `let leftover = Self::leftover_interaction_view_from(&state, &self.interaction_hover);`
  is built from that **same, just-updated** `state` (not the post-revalidate snapshot).
- Line 21690: `result.output = leftover;` — the leftover view is now the reserved invocation output for
  EVERY `interactionSelect`/`interactionHover`/etc. call (was previously discarded by
  `Self::empty_result(...)` with no interactionView at all).
- `fn leftover_interaction_view_from` (line 21697) builds `selected_ids` by walking
  `state.selection.values()` and pushes `("selectedIds", protocol::ToValue::to_value(&selected_ids))`
  into the published `interactionView` object (line ~21730).

This is a **brand-new function** — confirmed via `git diff f39d4b0db3 46c3cb9de0` on this file: the
whole `leftover_interaction_view_from` body plus the `let leftover = …` / `result.output = leftover`
wiring is new (`+` lines only, no removal of an older equivalent).

Host consumption confirmed wired end-to-end:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:408`
  `export function interactionViewFromLeftoverOutput(output: unknown)` reads `output.interactionView`
  and extracts `selectedIds` from it.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1888`
  `const published = interactionViewFromLeftoverOutput(output);` then
  `publishLeftoverWorldSelectionV1({ ids: published.selectedIds, … })` and bumps
  `leftoverInspectionEpoch` when `published.selectedIds.length > 0` — directly feeding hop 6's
  Inspection refresh.

## 2. Mesh-mode `translateSelection` pre-admit — ABSENT

Searched for the exact fault text `"fixed typed-operation and segmented-output authorities did not
pre-admit the exact operation slot"` (only occurrence: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23484`,
inside `dispatch_typed_command_inner`'s generic capacity gate — `can_admit_typed_operation` /
`latest_wins_commands.can_insert` / `latest_wins_order` / `segmented_downloads.can_insert` /
`segmented_closures.can_insert`). This gate is **unchanged** between `f39d4b0db3` and HEAD (no diff
hunk touches lines 23400–23530).

Guest `translate_selection` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️translate-selection/🦀️.rs`)
DID change in this window, but only for the *locked-object refusal* hop (already landed pre-#43, not
this one):

```rust
let unlocked: Vec<String> = ids.iter().filter(|id| ctx.scene.fixture.objects.iter().any(|object| &object.id == *id && !object.locked)).cloned().collect();
if unlocked.is_empty() && !ids.is_empty() {
    ctx.refuse_when_locked();
    return;
}
```

No code anywhere (guest editor, `World3dHost/🟦️.tsx`, `PluginRuntime/🟦️.tsx`, `plugin.🦀️.rs`) reserves
a typed-operation slot ahead of a mesh-mode `translateSelection` dispatch, nor special-cases the
capacity gate for it. `World3dHost/🟦️.tsx`'s `dispatchGumballPoseDelta` still only *synthesizes* a
translate when a move-axis pose delta rounds to zero (`§8.27`/`§8.28`, pre-existing, unchanged this
window) — that is a different bug (near-zero drag never dispatching) from this one (a real drag's
dispatch answering with the capacity fault).

**Corroboration**: the 14:40 baseline battery in `📓️2026-09-11-claude-coordination.md` (this same
session) reproduces this exact fault 16× on the still-#43 wasm and just launched **Wave B0** against
it — i.e. it is independently confirmed live and already assigned, in parallel with this audit.

**Implementation brief for Wave B0 / whoever picks this up:**
- Function: `Puzzle3dPlayApp::dispatch_typed_command_inner` (plugin.🦀️.rs:23470-23484) — the capacity
  gate that emits `interactive-job.typed-operation-capacity`.
- The likely fix is NOT loosening this gate but ensuring gumball-drag-initiated `translateSelection` in
  mesh mode doesn't collide with slots already held open by `suggestionsTick`/`setCamera`/
  `engagementAbort` turns in the same battery run — i.e. either (a) release/short-circuit those other
  reserved slots faster (their own turns should complete and free the slot before the next drag frame),
  or (b) give gumball-drag a reserved/dedicated slot class so it never competes for the shared
  `ARTIFACT_LIVE_OUTPUT_SLOTS` pool mid-drag.
- Now that hop 6 (`1:window` alias) has landed, the `setCamera`/`suggestionsTick` "no host context"
  storm that was separately consuming slots is gone — re-probe with `--battery` (or the targeted
  `--gumball --locked` lane) once B0 lands, since some of the 16 faults observed today may already
  clear as a side effect and only a genuine remainder needs the dedicated fix.
- Existing law to extend: `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` already has
  `leftover_overlay_translate_selection_moves_unlocked_object` (new this window, line ~326) proving the
  *unlocked* mesh-mode translate path at the Rust level — that law does not exercise the host capacity
  gate (it dispatches directly through the guest test harness, bypassing `plugin.🦀️.rs`'s slot
  admission), so it stays green regardless of this bug. The probe verdict name to watch is
  `gumball-scene-delta` (battery step; currently `poseLen=266`/`sceneDelta=false`).

## 3. Guest brush preview live-target latch — PRESENT

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs`:
- Line 2366: `brush_live_target: Option<String>` field on `Puzzle3dPrecomputeSession`.
- Line 2635-2636: `pub fn set_brush_live_target(&mut self, vortex_full_id: Option<String>) { self.brush_live_target = vortex_full_id.filter(|id| !id.is_empty()); }`
- Line 2639-2640: `pub fn brush_live_target(&self) -> Option<&str> { self.brush_live_target.as_deref() }`

Latched from `suggestions_tick`
(`✏️s/…/✏️editor/🎮️commands/⏱️suggestions-tick/🦀️.rs:31`):
`precompute.set_brush_live_target(Some(target.to_string()));`

Consumed by the world encode
(`✏️s/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`), new function `world_brush_preview_target`
(line ~443-452):
```rust
pub fn world_brush_preview_target(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> Option<String> {
    envelope.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty())
        .or_else(|| crate::editor::puzzle3d::puzzle3d_brush_target_vortex(envelope, interaction))
        .or_else(|| session.brush_live_target().map(str::to_string))
}
```
`world_brush_preview_json` calls this and only gates on `reason=no-target` when even the latched
fallback is empty — so a `setCamera`/hover-empty render after a warm preview keeps the last target.

Law test, same file, line 710-716:
```rust
fn brush_preview_target_falls_back_to_session_live_target() {
    let mut session = Puzzle3dPrecomputeSession::new();
    session.set_brush_live_target(Some("seed-left-001:v0".into()));
    let envelope = Puzzle3dScene { fixture: forest_store(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
    assert_eq!(world_brush_preview_target(&session, &envelope, &Puzzle3dInteractionSnapshot::default()).as_deref(), Some("seed-left-001:v0"));
}
```
This is exactly the "last published body keeps `brushPreviewJson`" hop from the W-AB #43 readout. Note
this is guest-source-only proof (`world_brush_preview_target` in isolation) — the wasm build to observe
it in the browser (`brush-preview-place` battery step) hasn't happened yet (still #43 wasm per the
coordination doc), consistent with the "Needs #44 wasm" note it was filed under.

## 4. `importFixture` apply/fold — PRESENT

Guest fold logic (`✏️s/…/✏️editor/🎮️commands/📥️import-fixture/🦀️.rs`) is **byte-identical** to before —
`git diff f39d4b0db3 46c3cb9de0` on this file is empty. It already replaces
`ctx.scene.fixture = fixture` unconditionally for a parsed payload, and `importFixture` is already in
`puzzle3d_action_document_intent` (editor `🦀️.rs:702`), so the generic before/after diff
(`puzzle3d_operations_from_fixture_change`, editor `🦀️.rs:589-598`, also unchanged this window) already
turns a genuinely distinct fixture into history mutations. Static reading says this path was already
correct; the reported browser failure (`map_hit=false`, `historyUpserts=0`) traced in the W-AB #43
readout to the **host** side, and that IS what changed:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — new
`openImportFixture` host-arm handler (lines 5961-5970, all `+` in the diff):
```tsx
if (action.action === "openImportFixture") {
  console.warn("[DEBUG] import-picker hop host-arm openImportFixture");
  void requestFileOpen("application/json,.json", "text", false)
    .then(async (opened) => {
      console.warn(`[DEBUG] import-picker opened=${opened.length} name=${opened[0]?.name ?? "none"} bytes=${opened[0]?.contents.length ?? 0}`);
      if (!opened[0]) return;
      onAction({ controllerId: action.controllerId, action: "importFixture", args: { payload: opened[0].contents, name: opened[0].name } });
    })
    .catch((error) => console.error("[DEBUG] import-picker host-arm failed", error));
  return;
}
```
This is exactly the missing half the W-AB readout named: *"`openImportFixture` does not settle on the
guest (host-arm `return`s)"* — before this, the host-arm branch for `openImportFixture` fell through
without ever re-dispatching `importFixture` with the picked payload. Plus a second related fix at
lines 4948/4958 (`resolvedImport = importAction || "importFixture"`) hardening the
`RequestFileOpen`-effect path's fallback.

New guest laws prove the fold explicitly, exercising the real `openImportFixture` → `importFixture`
sequence (`✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`):
- `file_menu_import_row_opens_the_file_picker` (line 4419) — Import is `openImportFixture` in the file
  menu; `importFixture` stays dispatchable but out of the palette.
- `open_import_fixture_requests_file_open_then_import_applies_payload` (line 4438) — clears the
  document, then imports a **distinct** payload (the original exported fixture) and asserts
  `imported.history_patch.is_some()`.
- `import_fixture_of_the_live_document_records_whether_identical_content_is_an_edit` (line 4463) — the
  companion negative case: re-importing the identical live fixture asserts `history_patch.is_none()`
  (store-identity no-op, not a guest bug) — this directly explains the "Concrete→Concrete is the
  documented store identity no-op; distinct must not be" framing from the W-AB readout, and pins that
  the fold DOES distinguish identical vs distinct.

Not yet re-verified against the browser (still #43 wasm) — but the specific gap named in the readout
(host-arm not forwarding the picker result into `importFixture`) has a direct, named fix.

## 5. Inspection `selected_section` fall-through + lock `flag_row` — PRESENT (confirm)

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`:
- `fn selected_section` (line 196) matches on `interaction.granularity` first, then two `.or_else`
  fallbacks:
  - Line 224-227: object-id fall-through — `let ids = &interaction.selected; fixture.objects.iter().find(|object| ids.iter().any(|id| id == &object.id))…`
  - Line 228-233: **vortex-uuid** fall-through — matches `interaction.selected` against either the bare
    vortex id or `puzzle3d_vortex_full_id(&object.id, &vortex.id)`, then renders the PARENT object's
    `object_fields` (so lock chrome/flag_row rides along).
- `fn flag_row` (line 104) assembles the `patchInspector` toggle row (`object.hidden`/`object.locked`)
  used by `object_fields` (line 118) for every fall-through path — so hop 5's "lock flag_row chrome"
  is the same code, not a separate gap.

Laws present, same file:
- `leftover_browser_shaped_snapshot_wires_namespaced_object_fields_and_lock_row` (line ~306)
- `leftover_vortex_granularity_unresolved_falls_back_to_object_fields` (line ~327)
- `leftover_selected_vortex_uuid_falls_through_to_object_fields` (line ~342) — asserts both
  `object.id` and `object.locked` render, and `.empty` does NOT.

This code was landed before #43 (§8.26/§8.27) and is unchanged in the `f39d4b0db3`→HEAD diff — confirmed
present and intact at HEAD.

## 6. Host `1:window` alias + leftover Inspection full refresh — PRESENT

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
(NOTE: this file has extremely long single-source-lines; plain `grep -rn` silently returns 0 hits on it
— use `rg`/python substring search, not `grep`, when re-auditing this file):
- `export const DEFAULT_LEFTOVER_WINDOW_SURFACE = "window";`
- `export function windowHostContextBindings(instanceId, windows, viewState)` — binds each authored
  window AND aliases the **last** bound window's surface onto `DEFAULT_LEFTOVER_WINDOW_SURFACE`
  (`pluginSurfaceRef(instanceId, DEFAULT_LEFTOVER_WINDOW_SURFACE)`), so `surface-visible` establishes
  host context for `1:window` before a leftover Viewport patch dirties it.
- `export function leftoverInspectionRefreshScope(selectedIds)` — `{ kind: "full" } : null` when
  `selectedIds.length > 0`.

Consumed in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
around line 8837 (inside the `leftoverInspectionEpoch` effect): the tab-switch logic still runs, but the
`const scope = leftoverInspectionRefreshScope(["leftover"]); if (scope && currentSession) { … void
refreshUi(currentSession, scope); }` call is now **unconditional** on the epoch firing — no more
early-return when the Inspection tab was already current (the #43 bug: "epoch only switched the
Inspection tab and returned early when already on that tab").

## 7. Host bridge worker fault `v102_1` fix — PRESENT

`🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts`:
```ts
export type ShardCommandIngressPage = { readonly cursor: ShardCommandPageCursor; readonly bytes: Uint8Array; readonly page: ActorBytePage };
```
and in `createShardCommandIngressPages`, each pushed page carries all three fields:
```ts
pages.push({ cursor: { … }, bytes: bytes.slice(), page: createActorBytePage(bytes) });
```
This is exactly the §8.28 fix: the `{ cursor, bytes }` shape (W-AB's `reactor.stageCommandPage` rework)
now ALSO mints `page: createActorBytePage(bytes)` so the still-live #42/#43 wasm's `poll$1` can
destructure `length` off `page` without hitting `Cannot destructure property 'length' of 'v102_1' as it
is undefined`.

## 8. History-panel command paging (32+7 law) — PRESENT

Landed 2026-09-10 19:24 (before #41), well before this audit's diff window, still intact at HEAD:
- Production code: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9878-9888` — the
  `UI_BUILT_CHILDREN_MAX`-ary paging tree over the live filtered command-row count (mirrors
  `paged_text_carrier`, line 451-471, same pattern).
- Fixture: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/history-panel-command-pages/🔣️.json`
  → `{"pageArity":32,"overflowPastArity":7,"commandRowCount":39,"expectedCommandSectionChildren":2,"entryKeyPrefix":"framework.history.entry."}`.
- Law: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:4114`
  `async fn ui_history_panel_pages_command_rows_from_the_live_count()`.

---

## Recommended wave split

Only **one** hop is absent (#2), so there is exactly one independent implementation wave needed —
everything else is already landed and should ride the next wasm rebuild + `--battery` re-verify instead
of new implementation work:

**Wave 1 (the only absent hop — already claimed by Wave B0 in this same session, do not duplicate):**
- Fix mesh-mode `translateSelection` typed-operation slot admission.
- Touches: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`dispatch_typed_command_inner` /
  `can_admit_typed_operation`, lines ~23466-23484) and/or
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
  (`dispatchGumballPoseDelta`).
- Does not touch any file the other 7 (already-landed) hops touch, so it is safe to land alone.
- Re-probe target: `--gumball --locked` lane, verdict name `gumball-scene-delta`
  (`sceneDelta`/`poseLen`).

No other wave is needed for source changes. The real remaining work across all 8 hops is **verification**,
not implementation:
1. Rebuild the puzzle 3d wasm (currently still serving #43, 2026-09-10 23:23) so hops 1/3/4/5/6/7/8
   (source-present) are actually exercised in the browser.
2. Re-run `--battery` and confirm `first-pick selectedIds`, `inspection-object-fields`,
   `inspection-locked-flag-row`, `locked-flag-row`, `locked-refusal-notice`, `brush-preview-place`,
   `import-distinct`, and `gumball-scene-delta` (contingent on Wave B0) all flip to PASS.
3. If `gumball-scene-delta` still fails after Wave B0 + hop 6's `1:window` fix are both in the served
   wasm, that confirms the capacity gate needs the dedicated fix rather than the host-context storm
   having been the sole cause.
