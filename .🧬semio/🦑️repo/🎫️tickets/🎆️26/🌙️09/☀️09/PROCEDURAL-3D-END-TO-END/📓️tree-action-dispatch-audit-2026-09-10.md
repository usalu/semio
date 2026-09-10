# Tree-row action dispatch audit — generation3d "Add Generation" (2026-09-10)

Static, read-only audit. No build/serve/browser was run for this pass — see
`📓️runtime-verification-2026-09-09.md` boot #11 for the live observation this explains.

## 1. How a `UiNode` tree row with an action binding renders, and what DOM event dispatches it

**Rust side — the row's binding.** `generation_tree()`
(`✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs:110`) builds the "Add Generation" row via
`tree_item_with_action(format!("{surface_prefix}.add-generation"), label("add"), None,
factory.action("addGeneration", None)?)`. `tree_item_with_action`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5699-5709`) binds it as
`Trigger::Activate` with no args (`builder.try_on(Trigger::Activate, action)`, the `None`-args arm).
`Trigger`/`ActionBinding` are defined at
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:1505-1532`. The row's
Rust-authored string id (`node.key` on the wire) is
`procedural3d-play-generate.add-generation` (from `render()` in
`✏️s/…/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs:32-34`, which calls
`generation_tree(GENERATION_3D_PLAY_APP_ID, "procedural3d-play-generate", …)`).

**TS side — snapshot → DOM.** `builtNodeToSnapshot`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx:383-405`)
mints one `UiNodeRecord` per node via `let nextId = 1; const id = nextId++`, DFS order, **on every
full-body reconciliation** (doc comment at line 383: "Node ids are DFS-local to this full-body
reconciliation"). `record.id` (this small sequential integer) is what
`treeItemToTreeData` (`🧰️framework/…/🗣️Interpreter/🟦️.tsx:1201-1228`) puts into
`TreeDataItem.id: String(record.id)` — this is the coordinator's observed `DIV id="5"`. The
Rust-authored semantic string id survives only as `record.key`, which is **never** written to a DOM
attribute by the tree renderer (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`, "leaf" row
at lines 1954-1984: `id={id}` is the numeric one, plus `role="treeitem"`, `data-slot="tree-item-row"`,
`data-tree-row-kind="leaf"` — no `data-ui-key`/`data-ui-node-key` anywhere in the file). **Gap vs
Puzzle 3D's `#tool.fill`-style ids** (`…PUZZLE-3D-END-TO-END/📓️2026-09-09-runtime-verification.md`):
puzzle's tree/tool rows are driven by hand-authored `TreeDataItem`s whose `id` is the caller's own
stable string (see `Catalogue`, `🌳️Tree/🟦️.tsx:876-878`, `id: item.id`); the semantic-UI bridge used
by procedural plugins instead always substitutes the volatile per-reconciliation integer. **A DOM
`id` selector here is not stable across a doc refresh** — the row that was `id="5"` this boot can be a
different number, or a different row, next boot.

**Click wiring.** `treeItemToTreeData` line 1224: `onClick: activateBinding ? () =>
dispatchTrigger(context, record, "activate") : …` — `activateBinding` is found because the row's
`bindings` include `{trigger:"activate", …}` (confirmed above), so `onClick` **is** wired. The
"leaf" row's actual DOM handler (`🌳️Tree/🟦️.tsx:1965-1968`):
```
onClick={(event) => { if (event.detail > 1) return; onClick?.(event); }}
```
`event.detail` is the browser's click-count-in-sequence (2 for the second click of a dblclick, 3 for
a triple-click); a synthetic/programmatic `.click()` reports `detail === 0`, so `0 <= 1` and the
handler fires — **a plain `.click()` on the row DOES reach the binding**, both for a real single
click and for a scripted one. **A double-click does NOT**: the row's `onDoubleClick` prop is never
set by `treeItemToTreeData` (only `onClick`/`onPointerEnter`/`actions` are populated at lines
1224-1226 — no `onDoubleClick:` key at all), so `🌳️Tree/🟦️.tsx`'s own
`onDoubleClick={(event) => { if (!onDoubleClick) return; … }}` (line 1969) is a no-op for this row,
**and** the first click of that sequence already fired the binding once (detail 1 on click #1) — so a
double-click both under- and over-fires relative to intent. **Enter does nothing**: no `tabIndex` and
no `onKeyDown` are set on any `role="treeitem"` row anywhere in `🌳️Tree/🟦️.tsx` (the file's one
`onKeyDown`, line 3635, belongs to the unrelated chat-panel textarea) — the row is not natively
focusable and carries no keyboard activation handler, so Enter is a structural no-op, not a bug
specific to this row.

## 2. Dispatch path from the click to the plugin

`dispatchTrigger` (`…🗣️Interpreter/🟦️.tsx:875-878`) calls `emitIntent` →
`store.buildIntent(record, binding)` (`…📃️UiDocumentStore/🟦️.tsx:507-519`), which mints a `UiIntent
{ surface, revision, node: record.id, nodeKey: record.key, trigger, action: {scope, name, version},
args: binding.args /* null for addGeneration */, input: null, seq }`. `UiNodeView` wires
`onIntent={onIntentStable}` for window bodies
(`🏛️ShellHost/🟦️.tsx:8962`), and `onIntentStable` (line 5822) is
`onActionStable(uiIntentToActionDescriptor(intent))`.

`uiIntentToActionDescriptor` (`🧰️framework/…/🛠️ShellHelpers/🟦️.tsx:1704-1710`):
`controllerId = intent.action.scope` (= the `controller_id` passed to `ActionFactory::new`, here
`GENERATION_3D_PLAY_APP_ID`), `action = intent.action.name` (`"addGeneration"`, version 1 so no
`@version` suffix), `args` omitted entirely when `uiIntentPayload` resolves to `undefined` — which it
does here: `intent.input === null` → `return intent.args ?? undefined` → `intent.args` is `null`
(the Rust `None`) → `undefined`. **Confirms: `addGeneration` truly dispatches with no `args` field**,
matching the Rust binding (`factory.action("addGeneration", None)`).

This `ActionDescriptor` reaches `onAction` (`🏛️ShellHost/🟦️.tsx:5401`), the single funnel every
window-body action goes through (`handleAction`, not `handleCommand` — `handleCommand` is reserved for
shell-originated app/mode/window-chrome commands like `setContributions`/`setAppRegistrations`, see
e.g. `🏛️ShellHost/🟦️.tsx:4280`, `:8102-8110`). Inside `onAction`, **before** any
`plugin.handleAction` call, there is a declared-action gate
(`🏛️ShellHost/🟦️.tsx:5689-5692`):
```
const declaredAction = targetSession.app.windowKinds.some((kind) => (kind.actions ?? []).some((entry) => entry.id === action.action));
if (!declaredAction && !FRAMEWORK_RESERVED_ACTION_IDS.has(action.action)) {
  console.warn("[DEBUG] skipping undeclared action", action.action, targetSession.app.id);
  return;
}
```
Only if this passes does `plugin.handleAction(instanceId, encodeWindowActionInvocation(…), viewState)`
run (line 5730-5734), followed by `applyHistoryPatch`, `applyHostEffects(response.requestedEffects,
…, resolveUiDirtyScope(response.uiScope), …)` and `awaitOperationSettle(response.output)`. See §3 for
why `declaredAction` is false for `addGeneration` here.

`applyHostEffects` (`🏛️ShellHost/🟦️.tsx:4673`) is what drives the refresh: it calls `refreshUi(nextSession,
uiScope)` (line 5025) after logging (now gated) `[DEBUG] applyHostEffects refresh`. A later typed-operation
**completion** (the retained-command frame landing async) re-enters at line 5052-5053, logging (gated)
`[DEBUG] completion apply` and calling `applyHostEffects(completion.requestedEffects,
resolveUiDirtyScope(completion.uiScope), …)` again. `UiDirtyScope`
(`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1273-1287`) is `{kind:"full"}` / `{kind:"none"}` /
`{kind:"partial", windowBodies?, …}`; `resolveUiDirtyScope(undefined)` → `{kind:"full"}` (line
1288-1290 doc comment: "missing … means `full`" — a whole-shell refresh is the safe default when a
program doesn't emit an explicit scope).

## 3. `addGeneration` specifically — command handler, publication lane, republish scope

Handler: `✏️s/…/🎮️commands/➕️add-generation/🦀️.rs:28-30`:
```rust
pub fn handle(_payload: &AddGeneration, doc: &ArtifactView<…>, cfg: &ConfigView<…>, _session: &mut FlowEvalSession) -> Result<Emit<…>, Fault> {
    Ok(generation_command_result("addGeneration", None, doc.snapshot, cfg.snapshot).map(|result| result.emit).unwrap_or_default())
}
```
`generation_command_result` (`✏️s/…/🎮️commands/🧬️generation/🦀️.rs:18-43`) builds `state` from the
projection, runs `generation_operations("addGeneration", None, …)`, applies each op to `state`, and
returns `Emit { artifact_mutations: […], config_mutations: [SetSelectedGeneration{…}],
coalesce_key: None /* only "updateGenerationValues" coalesces */, .. }`. This is a genuine document
mutation (artifact lane) plus a config-lane selection update — matching the earlier-audited
`ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[Artifact, Config, Transient] }`
(`✏️s/…/✏️editor/🦀️.rs:691`).

**But `addGeneration` is not a synchronous `handle_action` command here — it is a *retained/typed*
command.** `Generation3dPreviewCommandWork::step`
(`✏️s/…/✏️editor/🦀️.rs:393-412`) is the `ArtifactCommandWork` implementor that owns
`AddGeneration`/`RemoveGeneration`/`RenameGeneration`/`UpdateGenerationValues`/`SelectGeneration`
(line 388): it calls the same `generation_command_result_for`, stores the `Emit` on `self.emit`, and
if the mutation produced a `preview_fixture` it spins up a `FlowHost`/`FlowEvalSession` and returns
`ArtifactCommandWorkStep::Progress{…}` to keep evaluating the generation preview across further
`step()` turns — i.e. the operation only *completes* (and only then does its `requestedEffects`/
`uiScope` reach `applyHostEffects` per §2) once that preview evaluation finishes. This is the same
typed-operation/retained-command family whose "exceeds semantic work capacity" and "batched item
candidate … fold contract" failures blocked *every* action in earlier boots (`📓️runtime-verification-2026-09-09.md`
boots #2, #5) — by boot #11 those are fixed, so a `Progress`/complete transition should be reachable,
but I found no per-command `uiScope` assignment specific to `addGeneration` anywhere in this file or
`🧬️generation/🦀️.rs`; absent an explicit scope the completion falls back to the TS default
`{kind:"full"}` (§2), i.e. a whole-shell refresh, not a scoped re-render of just the Generations
window.

Whether any of this ever runs for `addGeneration` in the boot #11 session is moot per §4 below: the
click never reaches `plugin.handleAction` in the first place.

## 4. The confirmed gap — `addGeneration`'s window kind never declares the action

`WindowKindDefinition` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3086-3122`) carries its own
`actions: Vec<ActionDefinition>` field, doc-commented "Actions owned by this window kind. Mandatory,
may be empty, never absent." — this is exactly `kind.actions` that the `declaredAction` gate in §2
reads. The generations window's `definition()`
(`✏️s/…/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs:13-29`) sets `actions: Vec::new()` and is
registered via the bare `.window_kind_def(generations::definition())`
(`✏️s/…/✏️editor/🦀️.rs:1795`) with **no** follow-up `.window_kind_actions("generation3d-generations",
…)` or `.window_kind_action_refs("generation3d-generations", …)` call anywhere in the file (`grep -n
"window_kind_action" ✏️editor/🦀️.rs` → zero hits). Those two post-hoc builder methods
(`🧰️framework/…/🔌️plugin/🦀️.rs:4966-4980`) are the *only* way a window kind's `actions` list gets
populated after `.window_kind_def()` — `window_kind_actions` "Replaces the action definitions
structurally owned by one window kind", `window_kind_action_refs` "Assigns centrally declared builder
actions to one structural window owner." The app-level `.action_with(categorized_action("addGeneration",
…))` (`✏️editor/🦀️.rs:1815`) only registers the action in the app-wide palette/menu action registry —
it does **not** attach it to any window kind's `actions` list by itself. Sibling apps that dispatch
correctly do call the post-hoc builder: e.g. puzzle 5d's editor
(`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:8436-8437`)
calls `.window_kind_action_refs(board2d::WINDOW_KIND_ID, vec![…])`. `layout`, `space/home`, and
`lowpoly`'s editors do the same (`grep -rln "window_kind_actions(\|window_kind_action_refs("
✏️s/🔌️plugins` → those four apps only; generation3d/generation2d are absent from that list).

**Consequence, per §2's exact gate**: `targetSession.app.windowKinds.some(kind =>
(kind.actions??[]).some(entry => entry.id === "addGeneration"))` is `false` for every window kind in
this app (`generation3d-generations`, `-generate-form`, `-generate-preview`, and the edit-mode windows
all set `actions: Vec::new()` the same way — confirmed by reading `🪟️windows/👁️preview/🦀️.rs:20-35`
and `🪟️windows/📝️form/🦀️.rs:15-29`, same pattern). `"addGeneration"` is not in
`FRAMEWORK_RESERVED_ACTION_IDS` (`🛠️ShellHelpers/🟦️.tsx:254-273`: undo/redo/checkpoint/clipboard/
history-filter/tutorial/utility/tool/tick ids only). So `onAction` returns at line 5692 **before**
calling `plugin.handleAction` — no request ever reaches the guest, no typed operation is admitted, and
nothing in §3 ever runs. The one line this path does emit,
`console.warn("[DEBUG] skipping undeclared action", "addGeneration", "generation3d")`, is
**unconditional** (not gated by `SEMIO_RUNTIME_DIAGNOSTICS`) — it should be visible in a live console
capture; the boot #11 note ("no console failure") did not report checking for a `warn`-level
`[DEBUG] skipping undeclared action` line specifically, and the tab's console buffer is independently
known to drop thousands of lines per boot (`📓️runtime-verification-2026-09-09.md` boots #5/#7/#11).

This same gate blocks **every** window-body action of this app the same way, not just
`addGeneration` — including `selectGeneration`, `renameGeneration`, `removeGeneration`,
`updateGenerationValues`, and `setActiveExample` (all declared only via app-level `.action_with(…)`,
never attached to a window kind). This is consistent with, and likely the root cause of, both symptoms
in the ticket brief: the inert "Add Generation" row **and** the example picker whose `setActiveExample`
dispatch visibly changes the picker's own local label (a client-side control-value echo, independent of
whether the action ever reaches the plugin) but never touches the document.

## 5. DOM selectors + event sequence per action, and the diagnostics markers to prove dispatch/completion

Given §1's finding that the tree renderer never exposes a stable `data-ui-key`/`data-ui-node-key`
attribute, and `id` is a per-reconciliation-volatile integer, **the only selector that survives a doc
refresh is the row's rendered label text**, scoped to the section it lives in:

- **Add Generation** — the Generations window (`generation3d-generations`, body key
  `procedural.play.generations`), Actions section, single leaf row labelled "Add Generation" /
  "Generierung hinzufügen": `[role="treeitem"][data-tree-row-kind="leaf"]` whose
  `[data-slot="tree-label"]` text equals the locale label. Event: a single `.click()` (or a real mouse
  click with `detail<=1`) on that row `DIV` — reaches the binding per §1, but is dropped by the §4 gate
  before any plugin call.
- **selectGeneration** — a generation-list leaf row (icon `layers`, label = generation name, only
  present once `generation.generations` is non-empty — currently it never is, since `addGeneration`
  can't run): same `[role="treeitem"][data-tree-row-kind="leaf"]` selector scoped to the "Generations"
  section, single click.
- **renameGeneration** / **removeGeneration** — row actions (menu placement) on that same generation
  row: `RowAction` entries pushed at `🖼️semantic-ui/🦀️.rs:87-104`, rendered via
  `renderTreeHeaderActions`/`actions.map(...)` in `treeItemToTreeData` (line 1226) as
  `{kind:"button", onClick: () => context.onIntent(context.store.buildIntent(record, action.action))}`
  — these bypass `dispatchTrigger`/the `activate` lookup entirely and call `buildIntent` directly with
  the row-action's own `ActionBinding` (already carrying `id`/`name` args per §"Rust side" above); DOM:
  a header-action button inside `[data-slot="tree-header-actions"]` for that row (menu placement means
  it's likely behind a kebab/overflow trigger — `RowActionPlacement::Menu` at
  `🧬️contract/📦️packages/🦀️rust/🧩️component.rs:106`), single click on the menu item.
- **updateGenerationValues** — the slider in `generation3dGenerateForm` (`generation_form`,
  `🖼️semantic-ui/🦀️.rs:131+`), a `SliderView` (`…🗣️Interpreter/🟦️.tsx:1118-1141`):
  `onValueChange` dispatches `Trigger::Change`; DOM id is `node-${record.id}` (same volatile-integer
  caveat), so select via the form window's DOM subtree + a slider role (`role="slider"` from the
  underlying primitive) rather than `#node-<n>`. Event sequence for a scripted drag-equivalent: focus
  the thumb, then arrow-key or a `pointerdown`/`pointermove`/`pointerup` sequence (a bare `.click()`
  will not move a range slider).

**All four of the above are moot until §4's gate is fixed** — none of `addGeneration`,
`selectGeneration`, `renameGeneration`, `removeGeneration`, `updateGenerationValues` will leave
`onAction`'s early-return for this app today.

**Diagnostics.** `SEMIO_RUNTIME_DIAGNOSTICS` is resolved once per page
(`🏛️ShellHost`'s `//#region 🩺️RuntimeDiagnostics`, documented in
`📓️extension-addressing-2026-09-10.md` §5): explicit `setRuntimeDiagnostics` override → build's
`import.meta.env.VITE_SEMIO_RUNTIME_DIAGNOSTICS` → `localStorage["SEMIO_RUNTIME_DIAGNOSTICS"]`.
Arm a live tab with `localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1")` then reload (no
rebuild needed). Gated markers that prove the path ran: `[DEBUG] applyHostEffects refresh`,
`[DEBUG] applyHostEffects skipped refresh: session not current`, `[DEBUG] completion apply` (all
in `🏛️ShellHost/🟦️.tsx`, §2 above). **Important correction to the ticket brief**: per
`📓️extension-addressing-2026-09-10.md` §5, the markers named there as gated
(`thunk start`/`thunk done`/`command-ingress settled`) **do not exist in the tree any more** — a
sibling lane removed them between boot #7 and that lane's audit, and `🔌️PluginRuntime/🟦️.tsx` now
has zero `console.*` calls. The one marker that **matters most for this specific bug** is not gated
at all: `console.warn("[DEBUG] skipping undeclared action", action.action, app.id)`
(`🏛️ShellHost/🟦️.tsx:5691`) — it fires unconditionally and should be searched for directly
(`read_console_messages` with `pattern: "skipping undeclared action"`) before assuming a publication
gap; if it appears for `addGeneration`, §4 is confirmed live, not just statically.

## Summary of gaps found

1. **Root cause (high confidence, static + cross-app comparison)**: `generation3d-generations` (and
   every other generation3d window kind) declares `actions: Vec::new()` and is never followed by
   `.window_kind_actions`/`.window_kind_action_refs` in `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs`, so
   `🏛️ShellHost/🟦️.tsx:5689`'s `declaredAction` check is false for `addGeneration` (and
   `selectGeneration`/`renameGeneration`/`removeGeneration`/`updateGenerationValues`/
   `setActiveExample`), and `onAction` returns before ever calling `plugin.handleAction`. Sibling apps
   (puzzle 5d, layout, space/home, lowpoly) all wire this via `window_kind_action_refs`; generation3d
   and generation2d do not.
2. **DOM instability**: the tree renderer assigns `id` as a DFS-order integer re-minted on every
   full-body reconciliation (`builtNodeToSnapshot`, `UiDocumentStore/🟦️.tsx:387-389`) and never
   exposes the Rust-authored stable string id (`record.key`) as a DOM attribute — unlike Puzzle 3D's
   hand-authored `#tool.fill`-style ids. Any coordinator script keyed on `#5` will silently target the
   wrong row (or none) after the next refresh.
3. **No keyboard activation**: tree rows carry no `tabIndex`/`onKeyDown`, so "press Enter on the row"
   can never work by design, independent of §1's gap — this is not a regression to chase.
4. **Double-click is not wired** for `dispatchTrigger`-driven leaf rows (`onDoubleClick` never set in
   `treeItemToTreeData`); only a single `.click()`/real click is meaningful, and it also fires on the
   first click of an attempted double-click.
5. **Diagnostics mismatch**: the ticket brief's named console markers
   (`thunk start`/`done`/`command-ingress settled`) no longer exist per
   `📓️extension-addressing-2026-09-10.md` §5; the actionable, ungated signal for this specific bug is
   `[DEBUG] skipping undeclared action`, not a gated `applyHostEffects`/`completion apply` line.
