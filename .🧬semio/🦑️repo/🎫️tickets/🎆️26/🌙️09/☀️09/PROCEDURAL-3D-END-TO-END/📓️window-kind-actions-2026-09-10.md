# Window-kind action declarations for generation3d / generation2d (2026-09-10)

Lane: window-kind action declarations of the generation3d/generation2d editors + viewers, plus the
React engine's stable tree-row DOM ids and the undeclared-action diagnostic.

Status: DONE — declarations landed, laws added and green, descriptor regenerated. `restage required: yes` (§6).

## 0. Correction to `📓️tree-action-dispatch-audit-2026-09-10.md` §4

The audit's root-cause claim ("`kind.actions` is empty for every generation3d window, so
`declaredAction` is false for `addGeneration`") is **not what the built manifest contains**.
`AppDefinitionBuilder::build_definition`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5315-5339`) resolves ownership *after*
`.window_kind_def()`:

```rust
let explicitly_owned_action_ids: HashSet<String> = self.window_kinds.iter()
    .flat_map(|window| window.actions.iter().map(|a| a.id.clone())
        .chain(window.action_refs.iter().map(|a| a.as_str().to_string()))).collect();
for window in &mut self.window_kinds {
    for action_ref in &window.action_refs { /* … push the referenced ActionDefinition … */ }
    …
    for action in &actions {
        if !explicitly_owned_action_ids.contains(&action.id) { window.actions.push(action.clone()); }
    }
}
```

Because **no** generation3d/generation2d window kind owned anything explicitly,
`explicitly_owned_action_ids` was empty and the last loop copied *every* app-level action onto
*every* window kind. `kind.actions` was therefore the full app action list, not `[]`, and the
`declaredAction` gate (`🏛️ShellHost/🟦️.tsx:5691`) passed for `addGeneration`. The already-green
`examples_match_set_active_example_select_options` test
(`…🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1147`) reads `setActiveExample` straight out of
`definition.window_kinds[..].actions` and is the standing proof of that.

(Introduced 21fbcd3538 2026-09-02, i.e. it predates the audit — the audit read
`WindowKindDefinition::definition()`'s literal `actions: Vec::new()` and stopped before the builder.)

So this lane is **not** a "make the dead button work" fix. It is the structural one the blanket
fallback was papering over: every window advertising every action of the app (window chrome menus,
`ShellHost:7441`'s focused-window keybinding table, `ShellHost:5703`'s viewer-role mutation guard all
read `windowKind.actions`), with no law anywhere tying a window's declared actions to the actions its
own bindings actually emit.

### Measured before the change

`✏️s/🔌️plugins/🌀️procedural/🔣️.json` (`manifest.apps[…].windowKinds[…].actions.length`), read with `bun`:

| app | window kind | declared actions (before) |
| --- | --- | --- |
| `s.procedural.generation3d@1/*#editor` | `procedural-main` | 44 |
| `s.procedural.generation3d@1/*#editor` | `procedural-preview` | 44 |
| `s.procedural.generation3d@1/*#editor` | `generation3d-generations` | 44 |
| `s.procedural.generation3d@1/*#editor` | `generation3d-generate-form` | 44 |
| `s.procedural.generation3d@1/*#editor` | `generation3d-generate-preview` | 44 |

Every window carried the identical 44-entry app action list — `addGeneration` INCLUDED. The
`declaredAction` gate was therefore passing, and the inert "Add Generation" row and inert example
picker reported in `📓️runtime-verification-2026-09-09.md` boot #11 have a different cause. What was
actually wrong is the absence of any scoping at all, and the absence of any law tying a window's
declared actions to the ones its own bindings emit.

## 2. Per-window action table (after)

Owned explicitly via `.window_kind_action_refs(...)`; "emits" is what the law test observed by
rendering the window for every bundled example and walking every `UiNode` binding plus every
`WindowMeasure` of that window kind. "declared" is the built `WindowKindDefinition.actions` length —
owned verbs plus the app-scoped/framework verbs `build_definition` still copies onto every window.

### `s.procedural.generation3d@1/*#editor`

| window kind | owned (`window_kind_action_refs`) | emits (measured) | declared |
| --- | --- | --- | --- |
| `procedural-main` | `nodeGraphEdit`, `nodeGraphViewport`, `setLodMode` | `setLodMode` | 30 |
| `procedural-preview` | `setCamera`, `setShowMode`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`, `translateSelection`, `rotateSelection`, `scaleSelection` | `setShowMode`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity` | 36 |
| `generation3d-generations` | `addGeneration`, `selectGeneration`, `renameGeneration`, `removeGeneration` | `addGeneration`, `selectGeneration`, `renameGeneration`, `removeGeneration` | 31 |
| `generation3d-generate-form` | `updateGenerationValues` | `updateGenerationValues` | 28 |
| `generation3d-generate-preview` | `setCamera`, `setShowMode`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity` | `setShowMode`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity` | 33 |

`nodeGraphEdit`/`nodeGraphViewport` come from the node-graph host's own verb table
(`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:575` `nodeGraphActions`), `setCamera` and the transform trio from
`🌐️World3dHost/🟦️.tsx:4374`/`:1782` (viewport gesture and gumball) — neither is a `UiNode` binding or a
measure, so the render/measure walk cannot observe them and the law is silent about them; they are
declared from the renderer contract instead.

### `s.procedural.generation3d@1/*#viewer`

| window kind | owned | emits (measured) | declared |
| --- | --- | --- | --- |
| `procedural-view-preview` | `setShowMode`, `setLodMode`, `setCamera`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity` | all seven | 26 |

The seven are exactly `GENERATION3D_VIEW_TOOL_IDS` (`👁️viewer/🦀️.rs:70`); the remaining 19 are the
framework's own history/clipboard/tutorial/interaction verbs, which no plugin scopes.

### `s.procedural.generation2d@1/*#editor`

| window kind | owned | emits (measured) | declared |
| --- | --- | --- | --- |
| `generation2d-main` | `nodeGraphEdit`, `nodeGraphViewport` | — | 30 |
| `generation2d-preview` | `canvasPointerDown`, `canvasPointerMove`, `canvasPointerUp`, `canvasWheel` | — | 32 |
| `generation2d-generations` | `addGeneration`, `selectGeneration`, `renameGeneration`, `removeGeneration` | `addGeneration`, `selectGeneration`, `renameGeneration`, `removeGeneration` | 32 |
| `generation2d-generate-form` | `updateGenerationValues` | `updateGenerationValues` | 29 |
| `generation2d-generate-preview` | `canvasPointerDown`, `canvasPointerMove`, `canvasPointerUp`, `canvasWheel` | — | 32 |

generation2d declares no `WindowMeasure` chrome at all, so its two scene windows emit nothing the walk
can see; the canvas pointer verbs come from `📐️Canvas2dHost/🟦️.tsx:558`.

### Deliberately left UNOWNED (and therefore on every window)

`setActiveExample` (navbar picker), `addWidget` (catalogue palette + context menu), `patchFlowWidgets`
(inspector), `reorganize`, `removeWidget`, `deleteSelection`, `moveMediaNode`, `setShowMode`/`generate`/
`setEvalOutputs`/`connectMediaPorts` on generation2d, and every framework-injected verb
(`undo`/`redo`/`copy`/`cut`/`paste`/`commitCheckpoint`/`createAlternative`/`switchAlternative`/
`checkoutCheckpoint`/`revertToCommand`/`setHistoryCommandFilter`/`noteShellCommand`/`recordTutorial`/
`startIntroduction`/`startTutorial`/`interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/
`setSelectionMode`/`setInteractionGranularity`). None of them is dispatched by one window's own surface:
`setActiveExample` comes from the navbar, `addWidget`/`patchFlowWidgets` from panel tabs (which are not
window kinds at all), the rest from the app-wide context menu, the palette or the framework. Scoping any
of them to a window would be a lie, and — because `ShellHost:7441`'s keybinding table resolves against
the FOCUSED window kind only — scoping `undo`/`redo` would break `mod+z` in every window that did not
own it.

## 3. Diffs

### 3.1 Window-kind action declarations

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1891-1925`
  — five `.window_kind_action_refs(...)` calls after the existing `.window_kind_utilities(...)`, with the
  block comment that states which surface each list comes from and why the rest stays unowned.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1772-1784`
  — the same five, after `.window_kind_interactions(...)`.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:804-819`
  — one call for `procedural-view-preview`, all seven view verbs.

`WindowKindDefinition::definition()`'s literal `actions: Vec::new()` is left alone in all eleven window
modules: `actions` is the STRUCTURAL list (`.window_kind_actions()` replaces it), `action_refs` is the
reference list, and the builder is where an app's own action ids are in scope. That matches puzzle 5d
(`🧩️puzzle/…/✏️editor/🦀️.rs:8436`), layout, space/home and lowpoly, the four apps that already did this.

### 3.2 A real leak the law test found

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:56-69`

```rust
Some(_) => {
    let gen_fixture = generation_fixture_for(fixture, generation);
    let payload = preview_payload(generation_preview_text.unwrap_or_default(), &gen_fixture, cfg, None, marks);
    gen_fixture.retire_cold();   // ← was missing
    payload
}
```

`generation_fixture_for` (`🧬️schema/🦀️.rs:313`) CLONES the document fixture, so the patched copy owns its
own `layout` ordered-map root. Dropping it bare aborts the plugin actor with `ordered-map root must be
explicitly retired before drop` (`🌱️value/🗂️ordered/🦀️.rs:81`). The arm is reachable only once a
generation is SELECTED, which is why the existing per-example render tests — which all run against the
bundled examples' empty generation roster — never entered it. The new law test seeds a two-entry roster
and hit it on the first run:

```
thread '…::every_emitted_action_is_declared_on_its_window_kind' panicked at 🌱️value/🗂️ordered/🦀️.rs:81:57:
ordered-map root must be explicitly retired before drop
   5: …editor::generation3d::modes::generate::windows::preview::render
```

This is a live abort of the whole generate-mode preview window the moment a generation exists, not a
test artefact.

### 3.3 Stable DOM ids in the React engine

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`

- `:1204-1221` — new `//#region 🪪️StableDomIds`: exported `uiNodeDomId(surface, key, fallbackNodeId)`
  (`` `${surface}/${key}` ``, falling back to `node-${id}` only for a keyless node) and the private
  `nodeDomId(store, record)` that reads the surface off the store's own state.
- `:1234` `treeItemToTreeData` — `id: uiNodeDomId(state.surface, record.key, record.id)` replaces
  `id: String(record.id)`; `:1264` the same for tree SECTIONS; `:1297-1304` `treeStatusSection` now takes
  the store and derives its `-status` id the same way.
- `:1003`, `:1032`, `:1056`, `:1070`, `:1088`, `:1106`, `:1128`, `:1151`, `:1163`, `:1171` — every bound
  view (container/button/input×2/select/toggle/slider/number-stepper/ring/icon-select) takes
  `id={nodeDomId(context.store, record)}` in place of `` id={`node-${record.id}`} ``.
- every `data-ui-node-id={record.id}` site additionally carries `data-ui-node-key={record.key}` — the
  escaping-free selector a coordinator script or an assistive tool should key on
  (`[data-ui-node-key="procedural3d-play-generate.add-generation"]`), since the `id` contains `.` and `/`.

`ImageView`/`TextView` keep the ordinal-derived aria/`img` id: neither carries an `ActionBinding`, so
neither is addressable in the sense this law is about. They do carry `data-ui-node-key`.

### 3.4 Typed, always-visible undeclared-action diagnostic

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:276-311`
— new `WindowKindActionDeclaration`, `UndeclaredActionDiagnostic` and the pure
`undeclaredActionDiagnostic(appId, action, windowKinds, windowKindId?)`. It returns `null` for a declared
or framework-reserved verb and otherwise a record carrying the app id, the action, the DISPATCHING window
kind, every window kind of the app, and a message that names all of them and says which builder call
fixes it.

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5691-5698`
— the gate now reads:

```tsx
const undeclared = undeclaredActionDiagnostic(targetSession.app.id, action.action, targetSession.app.windowKinds, baseDispatchViewState.windowInstances.find((instance) => instance.id === dispatchWindowId)?.windowKindId ?? null);
if (undeclared) {
  console.error(undeclared.message, undeclared);
  return;
}
```

`console.warn("[DEBUG] skipping undeclared action", …)` is gone: `console.error`, no `[DEBUG]` prefix, and
the window kind is named. The gate itself stays app-wide (`.some()` over every window kind) — a context
menu, a palette entry or a keybinding legitimately dispatches a window-owned action from another window,
so scoping the GATE per focused window would have broken exactly the dispatches this ticket is fixing.
`FRAMEWORK_RESERVED_ACTION_IDS` moved out of `ShellHost`'s import list (its only consumer there was this
gate) and is now read inside the helper.

## 4. Tests added

### 4.1 The Rust law, per app

`✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs:220-297` — three `#[cfg(test)]` helpers, shared by
BOTH artifact crates through this file's existing `#[path]` mount (the repo's "if code is repeated it must
be close to each other" rule; each app uses only the half its own law needs, hence the `#[allow(dead_code)]`):

- `emitted_action_ids(projection)` — walks the WHOLE projected tree, not only `BuiltNode.bindings`,
  because a tree row's menu verbs (`Component::TreeItem.row_actions`, i.e. `renameGeneration` /
  `removeGeneration`) live inside the serialized `component` and never reach `bindings`. Any object
  carrying `scope`/`version` is an `ActionId` wherever it sits; `UiText` serializes as a plain string
  (`🎬️action.rs:200`), so `name` is read directly.
- `seed_law_generations(root)` — seeds a two-entry roster with the first selected, IN PLACE via
  `cold_builder_mut()`; `GenerationPlayRoot`'s `Drop` panics on a nonempty unretired root, so a law test
  may never assign a fresh one over an existing one.
- `measure_action_ids(measures)` — the `WindowMeasure` half (`ActionDescriptor.action` is a plain
  `action` string field, so this walk keys on that instead).

`…🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `every_emitted_action_is_declared_on_its_window_kind`:
renders every window body for every one of the eight bundled examples through the SAME
`generation3d_render_body` the live windows go through, with the seeded roster, unions the emitted action
ids per window kind, adds each window's `PluginApp::window_measures` verbs, and then asserts BOTH
directions:

1. `emitted(window) ⊆ declared(window)` — the `declaredAction` gate.
2. for every action in the union of all emitted sets, `declared(window)` contains it ONLY IF
   `emitted(window)` does — the exactness the blanket fallback destroyed. This second half is what fails
   on a lost `window_kind_action_refs` call, and it needs no hardcoded expected list: it is derived
   entirely from the renders.

`…🌀️generation2d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the same test, assertion for assertion, against the
live app fixture (generation2d renders through `PluginApp::render`, so the two generations are DISPATCHED
with `AddGeneration` rather than hand-seeded).

`…🧊️generation3d/…/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` —
`every_emitted_action_is_declared_on_the_preview_window_kind`: one window, so containment plus an equality
against the app's OWN roster (`GENERATION3D_VIEW_TOOL_IDS`) rather than against the declared list, which
also carries the framework's 19 injected verbs. `setCamera` is added explicitly with a comment saying why
the walk cannot see it.

### 4.2 The TypeScript laws

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`

- `//#region 📇️WindowKindActionScoping` — reads the PUBLISHED descriptor
  (`✏️s/🔌️plugins/🌀️procedural/🔣️.json`, the same `readPluginManifest` the contributions law already uses)
  and asserts, for sixteen (app, action) pairs across all three apps, the exact set of window kinds that
  declare it; plus a second law that app-scoped verbs (`setActiveExample`, `addWidget`, `undo`) are on
  EVERY window kind. This fails on a stale `describe` as well as on a lost declaration.
- `//#region 🪪️StableUiNodeDomIds` — four laws on `uiNodeDomId`: derived from surface+key not from the
  ordinal, namespaced per surface, ordinal only as the keyless fallback, and one distinct id per authored
  key of a body.
- `//#region 🚨️UndeclaredActionDiagnostic` — four laws: the message names app+action+window kind and is
  not `[DEBUG]`-prefixed; the gate stays app-wide (an action owned by one window is accepted from any
  other); framework-reserved verbs are never reported; an app with no window kinds still reads.

## 5. Runs (tails)

Private `CARGO_TARGET_DIR` seeded from `target/debug`, `RUSTC_WRAPPER=""`, `RUST_MIN_STACK=134217728`.
Raw logs under `🗑️generated/wka-*.txt`.

### 5.1 The three window-kind action laws — `🗑️generated/wka-law-final.txt`

```
test editor::generation2d::…::every_emitted_action_is_declared_on_its_window_kind ...
[STATS] window-actions kind=generation2d-main declared=30 emitted=0 emits={}
[STATS] window-actions kind=generation2d-preview declared=32 emitted=0 emits={}
[STATS] window-actions kind=generation2d-generations declared=32 emitted=4 emits={"addGeneration", "removeGeneration", "renameGeneration", "selectGeneration"}
[STATS] window-actions kind=generation2d-generate-form declared=29 emitted=1 emits={"updateGenerationValues"}
[STATS] window-actions kind=generation2d-generate-preview declared=32 emitted=0 emits={}
test result: ok. 1 passed; 0 failed; … 230 filtered out

test editor::generation3d::…::every_emitted_action_is_declared_on_its_window_kind ...
[STATS] window-actions kind=procedural-main declared=30 emitted=1 emits={"setLodMode"}
[STATS] window-actions kind=procedural-preview declared=36 emitted=5 emits={"setShowMode", "setSunAzimuth", "setSunElevation", "setSunIntensity", "toggleSun"}
[STATS] window-actions kind=generation3d-generations declared=31 emitted=4 emits={"addGeneration", "removeGeneration", "renameGeneration", "selectGeneration"}
[STATS] window-actions kind=generation3d-generate-form declared=28 emitted=1 emits={"updateGenerationValues"}
[STATS] window-actions kind=generation3d-generate-preview declared=33 emitted=5 emits={"setShowMode", "setSunAzimuth", "setSunElevation", "setSunIntensity", "toggleSun"}
test viewer::generation3d::…::every_emitted_action_is_declared_on_the_preview_window_kind ...
[STATS] window-actions kind=procedural-view-preview declared=26 emitted=7 emits={"setCamera", "setLodMode", "setShowMode", "setSunAzimuth", "setSunElevation", "setSunIntensity", "toggleSun"}
test result: ok. 2 passed; 0 failed; … 336 filtered out
```

### 5.2 `cargo check -p semio-s-plugin-procedural --keep-going` (native) — `🗑️generated/wka-check-native.txt`

```
Checking semio-s-plugin-procedural v0.1.0 (…/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 1m 59s
```
`grep -c "^error"` → **0**.

### 5.3 `cargo check … --target wasm32-wasip2 --profile wasm-dev --keep-going` — `🗑️generated/wka-check-wasm.txt`

`CARGO_PROFILE_WASM_DEV_DEBUG=false`.
```
Checking semio-s-plugin-procedural v0.1.0 (…/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
    Finished `wasm-dev` profile [unoptimized] target(s) in 4m 14s
```
`grep -c "^error"` → **0**.

### 5.4 `bun nx run @semio-tech/procedural-plugin:describe` — `🗑️generated/wka-describe.txt`

```
described procedural (plugin semio:procedural@0.1.0) -> ✏️s/🔌️plugins/🌀️procedural
  (wasm=a66b96114099fed15e7be990d3afa41794a653697da92b66b536867de75b233f
   core=d375ce05c4311fb70c4c73de448779d7c4af745b181c98f7fec2041a58bf6379
   descriptor=22a4384cd013afae0ebd2119f6c841c6d86c6dcde5f78cc7b53f801947ad358a)
@semio-tech/procedural-plugin:describe    14m 46s
```

`✏️s/🔌️plugins/🌀️procedural/🔣️.json` + `🛂️.descriptor.semio` rewritten 12:37, and the emitted manifest reads
back with exactly the counts in §2 and `addGeneration -> generation3d-generations`,
`updateGenerationValues -> generation3d-generate-form`, `setLodMode -> procedural-main`,
`setShowMode -> procedural-preview,generation3d-generate-preview`,
`translateSelection -> procedural-preview`, `setActiveExample`/`undo` -> all five.

Two operational notes for the next lane that runs `describe`:

- **It refuses a `CARGO_TARGET_DIR` outside the repo.** `emitOwnerDescriptorPairV1` rejects with
  `artifact root … resolves outside /Users/ueli/Documents/semio`, so a scratchpad target dir cannot be
  used. A lane-qualified `<repo>/target-wka` works (`target*` is gitignored, `.gitignore:16`) and can be
  APFS-cloned from a scratchpad one with `cp -Rc`; it was deleted again afterwards.
- **The nx daemon times out under peer load**: the first attempt died after 11 min with
  `NX The daemon timed out while processing HASH_TASKS`. `NX_DAEMON=false` gets through.

### 5.5 Engine-contract TS suite — `🗑️generated/wka-engine-contract.txt`

`SEMIO_TEST_LEVEL=long vitest run …🧪️tests/🔬️engine-contract/🟦️.ts`
```
 Test Files  1 passed (1)
      Tests  512 passed (512)
```
(the ten new laws included; run AFTER the descriptor regeneration, so the scoping law read the fresh file).

### 5.6 Full `--lib` suites — `🗑️generated/wka-suite-generation3d.txt`, `wka-suite-generation2d.txt`

`--test-threads=1`. generation3d **334 passed / 4 failed**, generation2d **229 passed / 2 failed**. All six
failures are pre-existing and owned by other lanes, each already named in this ticket's own notes:

| failure | prior record |
| --- | --- |
| `two_instances_converge_disjoint_widget_moves` (both apps) — `module.vcs: remote snapshot merge is fail-closed …` | `📓️runtime-defects-2026-09-09.md` §, `📓️contributions-delivery-2026-09-10.md:222`, `📓️close-ladder-2026-09-10.md:275` |
| `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` (both apps) — `P2 production envelope load did not reach terminal` | `🗑️generated/perf-run13.txt:390`, `🗑️generated/fh-19-generation2d-suite-final.txt:419` |
| `generation_preview_is_one_app_transient_shared_by_two_generation_windows` — `Generation3d preview operation did not finish` | `📓️close-ladder-2026-09-10.md:273`, `📓️hotpath-optimization-2026-09-10.md:315` |
| `refresh_pending_effects_arms_flow_eval_tick_chain` — `typed operation did not retire within 30 seconds` | `📓️close-ladder-2026-09-10.md:274`, `📓️hotpath-optimization-2026-09-10.md` |

### 5.7 `typecheck` — `🗑️generated/wka-typecheck.txt`

`bun ./📜️script.ts typecheck` on `@semio-tech/framework-renderer-react` is broadly red in this tree
(825 `error TS` lines, overwhelmingly `🧪️tests/🧪️docklayoutstore/🟦️.ts` generic-type churn from another
lane). It went 826 → 825 across this lane: the one error introduced here
(`ShellHost:5701 TS18048 'baseDispatchViewState.windowInstances' is possibly 'undefined'`) was fixed with
`(… ?? [])`. The remaining errors in the files this lane touched — `ShellHost:7478/7479`
(`pasteActionWithRetainedFragment` arg typing) and `Interpreter:1447` (`import.meta.dir`) — are on lines
this lane never edited.

## 6. Restage

**restage required: yes.** `✏️s/🔌️plugins/🌀️procedural/🛂️.descriptor.semio` and `🔣️.json` were rewritten
(12:37), so the served copy under
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/`
is stale until the poll-leak lane's next `activate` on the shared target. The engine-contract law reads
the SOURCE descriptor and passes; its sibling `PROCEDURAL_SERVED_DESCRIPTOR` branch is skipped when the
dev tree has no copy, so a stale served copy is silent — a boot after this change must be preceded by a
restage or the shell will still receive the 44-actions-everywhere manifest.

## 7. What this lane did NOT fix

- The inert "Add Generation" row and example picker of boot #11 are NOT explained by `declaredAction`
  (§0): the gate was passing. Whatever drops those dispatches is downstream of it — the retained/typed
  command chain (`Generation3dPreviewCommandWork::step`) or the effect/refresh path — and belongs to the
  work-capacity / hot-path / first-step-deadline lanes. What DID change here is that the next boot will
  say so out loud: the drop, if it happens, now prints
  `semio: app "…" dropped action "…" dispatched from window kind "…": no window kind declares it …` at
  `console.error`, ungated.
- The generate-mode preview leak (§3.2) is fixed, but `generation_command_result`'s own
  `preview_fixture` (`🎮️commands/🧬️generation/🦀️.rs:33`) hands an equally cloned `FlowFixture` to its
  caller; that one is the retained command's to retire and was left alone.
- `[[test]] example-geometry` (`…🧊️generation3d/📦️packages/🦀️rust/Cargo.toml:74`) declares no
  `required-features`, so `cargo check --tests` on that crate fails with three `E0433 cannot find editor`
  before it reaches anything else — the test body reaches through `editor::`, which is behind
  `component-app-assembly`. Pre-existing (committed 6ad7b0e7bc, 2026-09-10 01:31), not this lane's.
