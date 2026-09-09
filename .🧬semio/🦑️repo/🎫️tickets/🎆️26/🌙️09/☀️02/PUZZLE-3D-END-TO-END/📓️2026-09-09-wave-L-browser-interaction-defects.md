# Wave L — Browser interaction defects (puzzle 3d, React target)

Scope: the six defects measured 2026-09-09 20:35–21:05 in `📓️2026-09-09-runtime-verification.md`.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached `HEAD` (`9b605a4550`), concurrent peers editing the same
  tree throughout. Wave W-K owns the scene payload encoding (`🧊️main/🦀️.rs` scene JSON,
  `🌐️World3dHost` mesh resolution, plugin host `scene-surface.encode`) — not touched here.
- Cargo foreground only, `RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 CARGO_TARGET_DIR=…/scratchpad/target-p3d`
  (seeded private target). macOS has no `timeout`.
- TS: the renderer's react-target vitest project
  (`🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`), `SEMIO_TEST_LEVEL=standard` — the default
  `fundamental` level collects only `⚡️quick` and would have silently skipped every suite below.
- No browser run in this wave; the coordinator rebuilds and verifies. Everything in §5 is explicitly
  NOT verified.
- One blocker cleared before anything could be run at all: the puzzle3d testkit's close drain did not
  cover `PluginCloseStep::AwaitingInput`, so `cargo test -p semio-s-artifact-puzzle-3d` did not compile
  (`E0004`). Fixed in place (`🔬️testkit/🦀️.rs:241`); pre-existing, unrelated to this wave.

## 1 Per-defect cause + evidence

### 1.1 Duplicate Selection from the context menu is a no-op

Two independent causes stack, and the measurement cannot separate them without a browser probe.

**(a) The menu shown was not the plugin's.** `openSurfaceContextMenu`
(`🗣️Interpreter/🟦️.tsx:528`) falls back to the SHELL menu whenever the guest returns zero specs:
`items: specs.length > 0 ? mapSpecs(specs) : (shellFallback?.() ?? [])`. The rows the report lists —
"Set Active Example, Add Object…, Duplicate Selection ⌘D, Translate Selection, Rotate Selection,
Delete Selection ⌫" — are window-action LABELS (`ActionDefinition::bounded_catalog("duplicateSelection",
… "Duplicate Selection" …)`, editor `🦀️.rs:7599`), not the plugin's own context-menu vocabulary, which
for an object selection is "Duplicate / Select all of same kind / Zoom to selection / Hide / Lock /
Delete (1 object)" (`puzzle3d_context_menu_items`, editor `🦀️.rs:2391`). So the guest answered `[]`
and the React shell silently substituted `buildShellContextMenuItems` (`🏛️ShellHost/🟦️.tsx:9080`).
That also fully explains defect 3 (below) — the shell fallback lists every `inPalette` window action,
`deleteAttraction`/`deleteTargetVolume` included.
`requestContextMenu` (`🏛️ShellHost/🟦️.tsx:3892`) has three separate `return []` branches (no session,
no `plugin.contextMenu`, unresolvable window view context) and the guest has four more
(`refresh_cache` error, `window_config_store.capture` error — `🔌️plugin/🦀️.rs:24889`, no authored
labels, empty selection). NONE of them is distinguishable from "this surface has no rows here".

**(b) The command itself refused silently.** Whatever menu carried it, `duplicateSelection` reached the
guest (`handleAction` ok, completion `operation 355`) and produced no clone and no edit. Every
selection-scoped arm ran to completion over an EMPTY id list rather than refusing:
`duplicate_selection` cloned nothing and then called `replace_selection` with no targets (which
`Puzzle3dActionCtx::replace_selection` drops, editor `🦀️.rs:2259`), `delete_selection` retained
everything, `translate/rotate/scale_selection` moved nothing. The emitted delta was empty, so no
command row, no outliner change, no notice — indistinguishable from a dead menu row. W-D4's
`ctx.notice` existed but no arm called it, and `dispatch_step`'s abort branch
(editor `🦀️.rs:2947`) returned `Emit::default()`, which DISCARDS effects — so even an arm that had
noticed would have been silenced by its own abort.

Natively `duplicate_selection_reselects_the_created_clones` passes because the test selects first;
it never exercised the empty-selection path.

### 1.2 Context-menu group labels render as raw ids

The guest emits `menu.group.<category>` rows with `label: None` by contract — the taxonomy is CHROME
vocabulary the host owns (`🔌️plugin/🦀️.rs:11913`; asserted by
`plugin-runtime-plugin-builder-contract/🦀️.rs:3894` "group rows travel with no label"). The wgpu
target resolves it (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:304` via `ribbon_parent_label`); the React
target's `mapContextMenuSpecs` (`🌐️World3dHost/🟦️.tsx:1189`) mapped
`label: spec.label === undefined ? undefined : wireLabel(spec.label)` and left the row unlabeled, so
the menu renderer fell back to the raw id.

There was no TS twin of `ribbon_parent_label`. The EN/DE table itself already existed exactly once, as
the `ui.ribbon.parent.*` chrome bundle (`uiRibbonParentEn`/`uiRibbonParentDe`,
`🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx:2208`/`:2237`) — and it was being read through
two hand-copied lookups in `🛠️ShellHelpers` (`resolveUtilityGroupLabel`, `actionCategoryLabel`).
`menu.group.more` is the one group id NOT in the 20-id taxonomy (`organizeContextMenu` synthesizes it
when the row budget overflows, `🔺️mesh/🟦️.ts:235`), so it had no key to resolve through at all — it
renders blank in the wgpu target for the same reason.

### 1.3 `Delete Attraction` / `Delete Target Volume` offered for a plain object selection

Same root as 1.1(a): those rows came from the shell fallback, not from
`puzzle3d_context_menu_items`, which already gates strictly by selection kind (object branch returns
before the attraction/target-volume branches are reached, editor `🦀️.rs:2394-2445`).

The gate that was missing is one level up: both actions act on an `id` their own context-menu row
supplies (`delete_attraction(ctx, args)` reads `args.id`), yet they declare NO `action_args` — so any
generic surface (command palette, shell fallback menu) can only ever dispatch them empty. They were
`in_palette: true` (the `ActionDefinition::new` default, `🛂️manifest/🦀️.rs:905`).
`setTargetVolumeFlag` is the same shape and had the same defect.

### 1.4 Fill tick loop over-queues the actor

Two causes.

**(a) The in-flight gate gated nothing.** `createInFlightSkippingInterval`
(`🛠️ShellHelpers/🟦️.tsx:2450`) holds its flag for exactly as long as the promise `run` RETURNS. The
tick body was

```ts
() => { if (interactivePluginActionInFlight()) return; dispatch("fillBuildTick"); }
```

— a block body, so `dispatch`'s return value was discarded and the flag cleared on the next microtask,
long before the next 120 ms tick. Hence 252 ticks queued in 35 s and 38 rejections with
`serializePerActor: queue is full (>256 pending turns)`. `ComponentSceneHostProps.onAction` was typed
`=> void`, so returning it was not even expressible.

**(b) A returned promise still would not have meant "settled".** `plugin.handleAction` answers on the
guest's FIRST reactor turn — `dispatch_typed_command_inner` only ADMITS a typed command there and
replies `{ operationId, generation }` with `UiDirtyScope::None` (`🔌️plugin/🦀️.rs:22374`,`:22582`).
The work runs on later turns and lands as an `OperationCompleted` frame, which only the
`subscribeOperationCompletions` subscription saw (`🏛️ShellHost/🟦️.tsx` completion effect).

**(c) The `readHistory` half.** 90 `readHistory` round trips in the same 35 s did NOT come from a
history patch. The shell has exactly ONE `readHistory` call site — a `useEffect` keyed
`[applyHistoryPatch, session]` (`🏛️ShellHost/🟦️.tsx:1698`), and `applyHistoryPatch` is
`useCallback(…, [])`. So it re-fired once per new `session` OBJECT. `applyHostEffects`'s terminal
`SET_SESSION` guard was `current.viewState === nextViewState` — but every dispatch enters
`applyHostEffects` with a freshly built per-call projection (`onAction`'s `dispatchViewState`, built at
`🏛️ShellHost/🟦️.tsx:5409-5416`; `windowViewContext`/`panelViewContext` always return a new object,
`🛂️manifest/🟦️.ts:847-857`), so the guard was false on EVERY action and minted a new session every
time. The comment above that guard already stated the intent it was failing to enforce.

Separately, `finish_recorded` (`🔌️plugin/🦀️.rs`) ignored its `verb` parameter entirely and attached a
history patch whenever the log generation advanced — and `dispatch_emit` records one command-log row
per dispatch by design, `ActionKind::View` included (`🔌️plugin/🦀️.rs:20105-20116`). So a
`fillBuildTick` dirtied history. This is exactly the gap
`fill_build_tick_is_ignored_when_fill_tool_is_inactive`'s doc comment reported as unreachable; that
note also correctly identified `dispatch_typed`'s literal `"typed-command"` verb, though the browser
path enters through `handle_action`, which does pass the real id.

### 1.5 `registerBrushMesh` boot cost

`register_brush_mesh` never touched `ctx.ui_scope`, so it inherited `dispatch_step`'s
`UiDirtyScope::Full` default (editor `🦀️.rs:2915`) and its terminal `Emit` carried `Full` into the
completion witness — one whole batched `refresh-ui` guest round trip per page. Its publication lane is
already `ArtifactToolPublicationLane::HostOnly` (editor `🦀️.rs:6472`) and it is a `view_action`
(`:7626`), but nothing in the framework ties the lane to the scope: the lane declares "publishes to no
store", the scope is entirely the emit's own. Its sibling `fill_build_tick` sets
`UiDirtyScope::None` explicitly on its idle paths — that is the established pattern it was missing.
The matching `readHistory` per page is the same session-identity churn as 1.4(c).

### 1.6 Inspection panel empty — NOT settled

Established, not fixed:

- The guest CANNOT emit an empty inspection body: `inspection::render` returns either
  `selected_section` or the `summary` fallback (schema/domain/objects rows), and `PanelTreeBuilder`
  fails loudly rather than dropping children. So an empty body is host-side.
- Body keys match end to end (`puzzle.3d.play.inspector` in `🔍️inspection/🦀️.rs:23`, editor
  `🦀️.rs:7392`, `🔣️.json:175`, request built in `🛠️ShellHelpers/🟦️.tsx:3975`).
- A pick is `interactionSelect`, which always returns `UiDirtyScope::Full`
  (`🔌️plugin/🦀️.rs:20661`), matching the measured "one full" — so the panel IS in the request.
  (All four puzzle3d partial scopes carry `panel_bodies: Vec::new()`, editor `🦀️.rs:1952-1970`, so
  every OTHER selection-changing command leaves the inspector stale by construction; that is a real
  second defect, listed in §5.)
- A guest render fault would have rejected the whole `refreshUi`, and `refreshUi` was ok — so it is not
  a swallowed `ui.fixed-capacity`.
- What IS silently swallowed: a requested body whose retained surface is absent or unrooted is dropped
  from the response with no diagnostic, and the shell then keeps its loading placeholder.

Only a diagnostic was added, not a fix — see §2.8 and the one-probe decision procedure in §5.

## 2 Changes (file:line)

1. **`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx:371-375`** — `ui.contextMenu.more` added to
   `UiTranslationSchema` (the one group id outside `UiRibbonParentCategory`).
   **`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx:2725`,`:3552`** —
   `More`/`Mehr` in both bundles.
2. **`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:59-67`** — `CONTEXT_MENU_GROUP_ID_PREFIX` and
   `CONTEXT_MENU_OVERFLOW_CATEGORY` exported from the module that OWNS `organizeContextMenu`; its four
   internal literal `"menu.group."`/`"menu.group.more"` uses now read them (`:82`,`:86`,`:210`,`:219`,`:235`).
3. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:1757-1776`** —
   new `ribbonParentLabel(category)` (the TS twin of `ribbon_parent_label`) and `contextMenuGroupLabel(id)`.
   `resolveUtilityGroupLabel` (`:1609`) and `actionCategoryLabel` (`:3123`) collapsed onto the former,
   removing the two hand-copied lookups.
4. **`…/🧱️elements/🌐️World3dHost/🟦️.tsx:1188-1206`** — `mapContextMenuSpecs` resolves a label-less
   `menu.group.*` row through `contextMenuGroupLabel` and gives it the default `folder` icon (wgpu parity).
5. **`…/🧱️elements/🏛️ShellHost/🟦️.tsx:4839`,`:4846`,`:4852`** — `applyHostEffects`'s `SET_SESSION`
   guard now asks whether an EFFECT rewrote the view state (`nextViewState !== baseSession.viewState`)
   instead of comparing a per-call projection against the live session, so a plain dispatch no longer
   mints a new session. **`:1722`,`:1727`** — the history-snapshot effect re-keyed on
   `session?.pluginId`/`session?.instanceId`.
6. **`…/🧱️elements/🏛️ShellHost/🟦️.tsx:741-765`** (`OPERATION_SETTLE_*`, `startedTypedOperationId`),
   **`:1785-1855`** (`settleOperation`, `awaitOperationSettle`), **`:4884`** (completion resolves
   waiters), **`:5563`** (`onAction` awaits the settle), **`:5583`** (dep). `onAction`'s promise now
   settles on the dispatched action's own `OperationCompleted` frame, bounded by a 30 s watchdog and a
   fixed waiter/ring capacity, race-safe against a completion that lands before the waiter registers.
   **`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:656-664`** — `ComponentSceneHostProps.onAction` retyped
   `=> void | Promise<void>` and documented as that contract.
7. **`…/🧱️elements/🌐️World3dHost/🟦️.tsx:4504-4527`** — both tick loops (`suggestionsTick`,
   `fillBuildTick`) `return dispatch(...)` so `createInFlightSkippingInterval` actually gates.
8. **`…/🧱️elements/🔌️PluginRuntime/🟦️.tsx`** (`project`, in `ownedUiRefreshResponse`) — a requested
   body dropped for want of a retained surface, or for want of a root, now emits one permanent
   `console.error` naming the body key instead of vanishing. Diagnostic only — see §5.
9. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22740-22770`** — `finish_recorded` reads
   its `verb`, resolves the declared `ActionKind` through the action then command registry, and skips
   the history patch for `View`/`Interaction` (`verb_dirties_history_panel`). **`:22690`** —
   `dispatch_typed` passes the real verb instead of the literal `"typed-command"`.
10. **`✏️editor/🦀️.rs:2296-2314`** — `Puzzle3dActionCtx::refuse_without_selection(&ids)`: one localized
    notice plus `abort`. **`:2947`** — the abort branch now returns `Emit { effects, ui_scope: None }`
    so a refusal notice survives its own abort. **`🗣️terminology/🦀️.rs:42`** — `nothing_selected`
    (EN/DE × native/reuse).
11. Arms wired to it: `🎮️commands/👯️duplicate-selection/🦀️.rs`, `🗑️delete-selection/🦀️.rs`,
    `🚀️translate-selection/🦀️.rs`, `🔄️rotate-selection/🦀️.rs`, `📏️scale-selection/🦀️.rs`,
    `🧬️select-same-kind/🦀️.rs`.
12. **`🎮️commands/📋️register-brush-mesh/🦀️.rs:32`** — `*ctx.ui_scope = UiDirtyScope::None` at entry,
    covering every exit including the shared-mesh fast path and the refusal.
13. **`✏️editor/🦀️.rs:7608`,`:7610`,`:7611`** — `deleteAttraction`, `deleteTargetVolume`,
    `setTargetVolumeFlag` marked `.in_palette(false)`.
14. **`🔬️testkit/🦀️.rs:241`** — `PluginCloseStep::AwaitingInput` folded into the `Blocked` arm of
    `drain_close` (compile blocker, pre-existing).

New laws:
- **`✏️editor/🧪️tests/🔬️unit/🦀️.rs:2679`** `selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice`.
- **`✏️editor/🧪️tests/🔬️unit/🦀️.rs`** `fill_build_tick_is_ignored_when_fill_tool_is_inactive` — the
  `history_patch.is_none()` assertion its own doc called unreachable is now asserted (and passes).
- **`🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:4650`** "resolves taxonomy group rows from the chrome
  ribbon-parent bundle in both locales".
- **`🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:2136`** "gates on exactly what run returns — a discarded
  dispatch promise gates nothing".

## 3 Laws

1. **A menu that cannot be answered is not an empty menu.** Every silent-`[]` branch on the
   context-menu path is a lie the shell then dresses up as a shell fallback. Partly addressed
   (the fallback now cannot offer entity-scoped rows); the `[]`-vs-failure conflation itself
   is §5.
2. **A selection-scoped command with no selection refuses visibly.** One notice, `abort`, no empty
   edit, no empty interaction write — never a silent completion. `refuse_without_selection` is the one
   place that decides it.
3. **An abort suppresses mutations, not the refusal.** `Emit::default()` on abort discarded the very
   notice that made the refusal visible; effects travel on their own lane and survive.
4. **A group row's label belongs to the host, and the taxonomy has exactly one string table.** Every
   ribbon-parent lookup in the React shell now goes through `ribbonParentLabel`, reading the same
   `ui.ribbon.parent.*` bundle the ribbon does; ids outside the closed taxonomy resolve through their
   own declared key (`ui.contextMenu.more`) or not at all.
5. **An action a generic surface cannot dispatch correctly is not palette vocabulary.** An action whose
   handler needs a caller-supplied entity id, but which declares no args, must be `in_palette(false)`.
6. **A background tick is gated on the previous tick's completion, never on a wall clock**, and
   "completion" means the guest's own `OperationCompleted` frame — which is why
   `ComponentSceneHostProps.onAction` returns a promise that means it.
7. **A view/interaction verb never dirties the history panel.** `finish_recorded` decides by the
   verb's DECLARED kind, so every caller must pass the real action id.
8. **A per-call view-state projection is never written back into the session.** The session's identity
   is a dependency of the whole shell; only an effect that genuinely changed the view state may change it.
9. **A HostOnly-lane upload paints nothing.** The publication lane says where a tool publishes; the
   emit must say what it dirtied, and for a host-only upload that is `UiDirtyScope::None`.
10. **A refresh that drops a body it was asked for says so.** Silence there is indistinguishable from
    success.

## 4 Commands + tails

```
cargo check -p semio-framework-plugin
    Checking semio-framework-plugin v0.1.0 (…/🔌️plugin/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 5.05s

cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
    Checking semio-s-artifact-puzzle-3d v0.1.0 (…/🧊️3d/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 54.24s

cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle
    Checking semio-s-plugin-puzzle v0.1.0 (…/🧩️puzzle/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 18.48s
    (warnings present — `unused_qualifications` in puzzle 2d/5d, peer-owned; proof the crates really compiled)

cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib
    test result: FAILED. 590 passed; 23 failed; 0 ignored; 0 measured; 0 filtered out; finished in 41.78s

cargo test … --lib -- selection_scoped_commands_with_no_selection
    test editor::puzzle3d::component::tests::selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice ... ok
    test result: ok. 1 passed; 0 failed; 612 filtered out

cargo test … --lib -- duplicate_selection select_same_kind fill_build_tick context_menu   (before the full run)
    9 passed / 3 failed → after the fixes: fill_build_tick_is_ignored_when_fill_tool_is_inactive ... ok
    (now passing WITH the added `history_patch.is_none()` assertion — the §1.4 history fix, executed)

SEMIO_TEST_LEVEL=standard bun x vitest run 🔬️engine-contract     (renderer react target)
     Test Files  1 passed (1)
          Tests  457 passed (457)

SEMIO_TEST_LEVEL=standard bun x vitest run                        (renderer react target, all 15 suites)
     Test Files  1 failed | 14 passed (15)
          Tests  569 passed (569)
    — the one failure is a suite that cannot LOAD: `🧩️package-integration` → `ReferenceError: self is
      not defined` from `🎯️targets/🧊️wgpu/…/🐚️plugin-bridge.ts:159`. Pre-existing, no file of mine.

bun x vitest run                                                  (ui-react)
     Test Files  5 failed | 17 passed (22)
          Tests  4 failed | 697 passed (701)
    — all pre-existing and unrelated: `bun:sqlite` bundling, the package self-alias check, a UIDialog
      focus assertion, a 20 000-node Diagram timeout, a CSS-content assertion. No i18n/schema failure,
      which is what the new `ui.contextMenu.more` key would have broken.

bun x tsc --noEmit -p tsconfig.json   (whole repo)
    4036 errors, 308 of them outside generated `.d.ts` — every one of those 308 in
    `✏️s/🔌️plugins/🏗️fem/…/🚪️io/…` (peer-owned). ZERO in any file this wave touched, and zero anywhere
    under `🧰️framework/`.
```

The 23 Rust failures are the documented pre-existing families, not regressions:

- `fill_build_tick_is_a_view_action_with_narrow_ui_scope`, `fill_build_tick_only_plans_available_slider_range`,
  `fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job`,
  `fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances`,
  `fill_render_reveals_the_full_available_plan_tagged_with_reveal_index`,
  `set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up` — `📓️2026-09-09-remaining-test-failures-audit.md` §3.2.
- `gumball_translate_drag_coalesces_into_one_edit` — same audit §3.3 (`addObjectKind` no-op on the
  empty example). It passes its own "three ticks accumulate" assertion and fails on the UNDO, so it is
  not the new refusal path (the test supplies explicit `ids`).
- `hover_suggestion_updates_the_brush_candidate_index_and_live_preview` — `📓️2026-09-09-user-feature-checklist.md` §Hover-to-preview.
- `two_instances_converge_disjoint_object_edits_via_backbone` — same audit §268, an explicit
  fail-closed VCS stub.
- 14 × `precompute::…::fill_worker_*` — the precompute module is staged-modified by a peer
  (`⏳️precompute/🦀️.rs`, `⏳️precompute/🧪️tests/🔬️unit/🦀️.rs`) and its own suite is mid-refactor.

`duplicate_selection_reselects_the_created_clones` and `select_same_kind_with_no_selection_leaves_the_selection_untouched`
both pass — the refusal did not break the happy paths.

## 5 Not verified / left open

1. **No browser run.** Nothing here is confirmed against the React target at runtime. Defects 1, 2, 3,
   4 and 5 are fixed at their causes and covered by native/TS laws; only a rebuild + pick + right-click
   + fill run settles them in the browser.
2. **Defect 6 (inspection panel) is NOT fixed** — only instrumented. The one probe that decides it, on
   the pick's full refresh: read the console for the new
   `refreshUi dropped requested body "puzzle.3d.play.inspector"` record.
   - If it appears → the panel's retained surface is absent/unrooted; the fix is in `refreshUi`'s
     `missingSurfaceIds` (which today waits only for surfaces that have NEVER published, so a surface
     being re-rendered is read back mid-flight). Deliberately not changed here: widening it is exactly
     the shape that produced the 20:35 `4096 continuations` failures.
   - If it does NOT appear and the panel is still blank → the guest rendered `summary` (three rows) or
     `selected_section`, and the empty body is downstream in `TreeView`
     (`🗣️Interpreter/🟦️.tsx:1072-1098` renders `<Tree sections={[]}/>` with no fallback when no child
     resolves to a `treeSection`).
3. **Every puzzle3d partial scope carries `panel_bodies: Vec::new()`** (editor `🦀️.rs:1952-1970`), so
   `duplicateSelection`, the gumball verbs, `setSelectionFlag` etc. leave the inspection panel stale by
   construction. Not changed here — it needs a scope decision per command, and defect 6's own cause
   must be settled first or the two fixes will be confused for one another.
4. **`translateSelection`/`rotateSelection`/`scaleSelection` refusals are not observed in-process.**
   They carry a `coalesce_key`, so they enter the latest-wins channel whose accepted invocation answers
   before the command runs (`dispatch_typed_command_inner`'s `latest_wins_target` arm); the notice
   travels on that operation's completion lane, which the in-process fixture does not drain. The law
   asserts only that they emit no mutation. A browser check of "Translate Selection with nothing
   selected shows a notice" is still owed.
5. **wgpu parity for `menu.group.more`.** `shell_context_menu_item_from_spec`
   (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:304`) resolves the 20 taxonomy ids and renders the overflow row
   with an EMPTY label. Left alone: it is a different target, not a measured defect, and touching
   `ui_wgpu` would have pulled another crate into an already-contended build. One-line follow-up next
   to `ribbon_parent_label`.
6. **`🔌️plugin-runtime` vitest suite is unreachable.** `🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`
   is referenced by no vitest project in the repo, so the `🔌️PluginRuntime` change in §2.8 has no
   suite to run against.
7. **`onAction` now settles late by design.** Every caller that awaits it waits for the guest's
   completion (bounded at 30 s). The framework-reserved verbs (`undo`/`redo`/`commitCheckpoint`/…)
   start no typed operation and still resolve immediately, and the direct-browser-actor branch is
   unchanged, but a caller that relied on the old early resolve would now be slower. Nothing in the
   tree awaits it in a hot loop today.
8. **`dispatch_emit` still records a command-log row for a refused mutation** (by design — see its own
   doc), so a refused `duplicateSelection` appears in history with no edit id. Deliberately not
   changed; noted in the new test's doc comment.
9. **`handle_command_frame` still passes the literal `"typed-command"`** to `finish_recorded`
   (`🔌️plugin/🦀️.rs:24331`). That path currently faults before it can matter
   (`dispatch_command_frame` rejects bare frames), so it was left as is.
