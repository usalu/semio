# 📓️ terra — react-w4 — report

Packet `react-w4` (executor "terra", coordinator "sol"). Scope: TypeScript only —
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/**`, the plugin window-kit TS
(`🔌️plugin/🪟️window-kits/**`), and the platform/kernel TS barrels carrying fallout from the
UiNode → UiNodeRecord/UiPatchOp contract migration. Continues `📋️SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`'s
`react-renderer` and `react-tests` packets (read first, both already migrated `Interpreter`,
`UiDocumentStore`, `PluginRuntime`'s scoped region, the react barrel, and `🧪️index.test.ts`).

## how I actually type-check

No `typecheck` nx target exists for any package in my scope (see **missing-gate finding** below), and
a bare `bunx tsc -p tsconfig.json --noEmit` from repo root floods with TS5097
(`allowImportingTsExtensions` missing on the root config) — a known, already-diagnosed trap on this
ticket. I built a scratch tsconfig exactly like `react-renderer`/`react-tests` did:

```json
{
  "extends": "/Users/ueli/Documents/semio/tsconfig.json",
  "compilerOptions": { "allowImportingTsExtensions": true },
  "exclude": [
    "**/node_modules/**", "js/temp", "temp", "reports", "log",
    "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts",
    "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📜️script.ts"
  ]
}
```

Run: `bunx tsc -p <scratch>/scratch-tsconfig.json --noEmit`. Both excluded files are **live, unowned,
mid-edit Rust-adjacent `script.ts` files with a parse error** (`runCargoTestBudgeted([], packageRoot,
["...rest]);` — a spread `...rest` corrupted into a string literal), one flagged already by
`react-renderer`, the second a NEW instance of the same shape found this session (different crate,
`🖱️ui/🎬️scene`) — confirms the "scene crate... doesn't exist yet" gap `sdk-helpers`'s report already
named is now actively being built by a concurrent session. Neither is mine (no Rust, not in path
scope); a real parse error aborts the whole tsc run if left in, so exclusion (not a fix) is required
to see anything at all downstream.

## ground truth vs the inherited "62 across 16"

That figure was `manifest-typegen`'s inventory, taken **before** `react-renderer`/`react-tests` ran —
both packets already zeroed most of it. I re-measured from scratch rather than trust it; my own first
full run (repo-wide) showed **8039 diagnostics, exit 2** — within the ticket's own documented
externally-red baseline range (~8800–10100).

**Mid-task event**: the coordinator regenerated the stale ts-rs mirror
(`🛂️manifest/🤖️generated/🟦️ui-contract.ts`, previously missing entirely — confirmed missing myself
before the regen, exactly as `react-renderer`/`react-tests` both flagged) partway through my session.
This changed real shapes under me: `SurfaceKind` went kebab-case (`"virtualFileSystem"` →
`"virtual-file-system"`), `SurfaceProps` dropped `surfaceId`/`controllerId`/`paneId`/`bindingId`/`menu`
in favour of `{ kind, docSchema, doc, bindings }`, `Option<T>` fields render as `T | null` (not
`| undefined`) throughout, `EdgeSpace`'s `symmetric`/`each` variants nest their fields one level deeper
than the pre-regen shape, and `UiRevision`/`seq`/`layoutEpoch` types split (`UiRevision`/`revision` =
`number`, `layoutEpoch`/`UiIntent.seq` = `bigint`). Repo-wide count after regen: **6759 → 6685** by the
time I finished (net −74 repo-wide from my fixes; the regen itself moved the baseline, so a bare
before/after delta across that boundary isn't a fair "did I fix things" number — the per-file table
below is measured entirely on the POST-regen mirror, my actual real ground truth).

## per-file count, post-regen mirror (before my fixes → after)

| file | before | after |
|---|---:|---:|
| `🛂️manifest/🟦️component.ts` | 5 | **0** |
| `🖥️platform/🟦️component.ts` | 21 | **0** |
| `🔺️mesh/🟦️component.ts` | 1 | **0** |
| `🪟️window-kits/🌳️tree/🟦️component.ts` | 3 | **0** |
| `🪟️window-kits/🎬️media/🟦️component.ts` | 3 | **0** |
| `🪟️window-kits/📄️document/🟦️component.ts` | 3 | **0** |
| `🪟️window-kits/📊️table/🟦️component.ts` | 1 | **0** |
| `🪟️window-kits/📝️text/🟦️component.ts` | 1 | **0** |
| `🪟️window-kits/🖼️image/🟦️component.ts` | 2 | **0** |
| `🪟️window-kits/🧊️mesh/🟦️component.ts` | 1 | **0** |
| `Interpreter/🟦️component.tsx` | 51 | **19** (all `TS2820`, pre-existing i18n gap — see below) |
| `UiDocumentStore/🟦️component.tsx` | 15 | **0** |
| `PluginRuntime/🟦️component.tsx` (scoped edit) | 2 | **1** (`ArtifactPresencePeer`, unrelated) |
| `IconRenderHost/🟦️component.tsx` | 1 | **0** |
| `Shell/🟦️component.tsx` | 9 | **8** (only the `UiNode` import was mine; rest pre-existing) |
| `⚛️react/🧪️index.test.ts` | 95 | **93** (2 `layoutEpoch: number→bigint` fixes; rest pre-existing/unrelated) |
| `⚛️react/📦️index.tsx` | 6 | 6 (pre-existing/unrelated — see below) |

Every other file with errors in `📺️renderer/**` (`ChromePanels` 33, `NodeGraph` 31, `UtilityTree` 19,
`World3dHost` 17, `Table` 9, `Board2dHost` 6, `WasmSessionLoader` 5, `TaskManager` 4, `TiledMapHost` 3,
`ShellSync` 3, `Paint2dHost` 3, `InkCanvasHost` 3, `Canvas2dHost` 3, `TextEditor` 2, and six 1-error
files) is **pre-existing and unrelated to the migration** — verified by grepping every one of their
diagnostics for `UiNode`/`UiControlNode`/`UiTreeNode`/`UiStackNode`/`UiTextNode`/`UiImageNode`/
`UiKeyValueNode`/`Component`/`UiNodeRecord`/`UiSnapshot`/`declarativeTreeDragController`/
`renderUiControl`/`uiTreeNodeToTreePanelConfig`/`IconRenderRequest`: **zero hits** in any of them.
These are `id`-prop-now-required on `ElementProps`/`InputProps`, `icon`-now-required on the button-ish
control, an xyflow `Node` type-package split, `ThreeEvent` missing from `three`, a `shellScope`/
`sampleBezierSegments` undefined-name pair — a different, concurrent, unrelated churn in this same
live tree. Not touched, not claimed fixed. `✏️s/🔌️plugins/🎞️animate/**` (133) and
`✏️s/🔌️plugins/📐️cad/**` (87) are outside my path scope entirely (fleet plugin UI, blocked on the SDK
per `📓️luna-exclusion-map.md`) — not measured further, not touched.

## what I actually fixed (real bugs, no suppressions)

1. **`🛂️manifest/🟦️component.ts`**: added the missing internal imports (`ContextMenuItemSpec` from
   `../🔺️mesh/🟦️component.ts`, `Effect` from `../🎠️kernel/🟦️component.ts` — both real, live,
   hand-written types this file's own re-exports (via `🟦️glue.ts`'s barrel aggregation) never made
   visible to the file's OWN internal references); renamed a stale `UiMenuRef` reference to the real
   `MenuRef` (tsc's own suggestion, confirmed correct — `UiMenuRef` is a *different*, unrelated,
   `🔺️mesh`-local type, not a rename target); added the missing `case "settingsDefaultApps":` arm to
   `panelTabKindId`'s switch (a new `PanelTabKind` variant added upstream mid-ticket left the switch
   non-exhaustive) — `"framework.settings.defaultApps"`, consistent with its siblings' naming.
2. **`🔺️mesh/🟦️component.ts`**: added the missing `PluginContextMenuRequest` import (same "self
   reference not covered by the aggregating barrel" class as #1).
3. **`🖥️platform/🟦️component.ts`**: added missing imports for real, live, unrelated-to-migration types
   (`UiPresence`/`UiStatus`/`WindowLayoutWindowNode` — all still exist, this file just never asked for
   them from the barrel). Migrated `pendingWindowUiNode`/`pendingPanelUiNode` (real callers:
   `ShellHelpers` line ~1511, `ShellHost` — 6 call sites) from the deleted `UiStackNode`/`UiTreeNode`
   to the new `BuiltNode` (the contract's own "author a node, ids get minted at reconciliation" type),
   with `activity: "loading"` replacing the old inline `presence` field per the contract's own design
   (presence is a document-external channel now, never baked into a node). **Removed** (not migrated)
   `uiInspectorStepperField`/`uiInspectorToggleField`/`uiInspectorVec3Group`/`uiInspectorGroupsToTree`/
   `uiDeclarativeSectionsToTree`/`uiDeclarativeChildToTreeItem`/`isUiControlNode`/
   `UI_CONTROL_NODE_TYPES` — see **found, not fixed** below; a doc comment in the file explains why and
   names the two independent blockers.
4. **7 window-kit files**: `table`/`text`/`mesh` build a `Component::Surface`-adjacent
   `UiComponentSceneNode` (unchanged, still live) and just needed their return-type annotation
   narrowed from the deleted `UiNode` to the real type they already construct. `image`/`media`/
   `document`/`tree` were migrated to the new contract (`BuiltNode`) — real value, since these 4 are
   exactly the ones the Rust `WindowKit` trait's own doc comment (`🔌️plugin/🦀️component.rs` `#region
   🔖️WindowKits`, `sdk-helpers`'s report) says COULD individually convert today; the Rust side is
   deliberately holding all seven back together for the other three's sake, so **this creates a
   temporary TS/Rust twin mismatch, flagged in each file's own header comment** — re-verify parity
   once the Rust `WindowKit` trait migrates. Nothing in the repo currently calls any of the four
   (confirmed by grep), so there is no behavioral risk today.
5. **`Interpreter/🟦️component.tsx`** — the coordinator's two direct asks:
   - **`Component::Surface`/scene-host unknown-`docSchema` behaviour**: added `isWellFormedDocSchema`
     (shape-only check — `"<name>@<number>"`, since this file has no per-kind version registry to
     validate against) and wrapped the `decodePackValue` call in try/catch.
     `surfacePropsToComponentSceneNode` now returns `null` (never throws) on either failure, logged via
     `console.error` with `{nodeId, kind, docSchema}`/`{..., error}`; `renderComponentSceneHost` renders
     a labelled placeholder (`data-unknown-surface-schema`) instead of crashing or silently dropping
     the surrounding patch.
   - **Registrar-owned mirror-internal errors**: none found in the regenerated `ui-contract.ts` itself
     (0 diagnostics inside it) — the 42 diagnostics in the OTHER generated file,
     `🛂️manifest/🤖️generated/🟦️manifest.ts` (`Cannot find name 'Label'/'StyleSpec'/'UiMenuRef'/
     'ConfigSpec'/'CommandGrammar'/'ArtifactPresentation'/'FileTypeContribution'/'TopicContribution'/
     'IoEntryDescriptor'/'ComposerEntryDescriptor'`), are a **separate, pre-existing generation gap in a
     file I did not touch and cannot touch** (registrar-only, `bun nx run @semio-tech/framework:generate`'s
     own output) — reporting verbatim per your instruction, not mine to fix or paper over.
   - Also fixed independent of the ask: `SurfaceProps`'s dropped fields (`surfaceId`/`controllerId`
     bridge substituted with `String(record.id)` — the record's own stable identity, since the 14
     unowned scene-host elements still require *some* string there; `paneId`/`bindingId` simply omitted,
     both optional on the target type); `virtualFileSystem` → `virtual-file-system` kebab-case fix;
     `EdgeSpace`'s new one-level-deeper `symmetric`/`each` nesting; ~15 `T | null → T | undefined`
     `?? undefined` conversions at the `@semio-tech/ui-react` prop boundary (the file's OWN
     already-established convention, just missing at these specific new-from-regen sites);
     `dragData`'s `Record<string, string|undefined> → Record<string, string>` filter (drops
     `undefined`-valued entries — `TreeDataItem.dragData` requires definite string values); a
     `MenuRef.args` now-required field on a literal that was missing it; `RowAction.title` needed
     `wireLabel()` branding, not a bare string; the file's own inline tests updated for the same
     `null`/`bigint`/full-`Component`-literal shape changes (see next item).
6. **`UiDocumentStore/🟦️component.tsx`**: `SetMenu`'s handler was doing `op.menu ?? undefined` —
   **backwards**, since the target field wants `MenuRef | null` and `op.menu` IS `MenuRef | null`
   already; the `?? undefined` was silently converting a valid `null` into a value the field doesn't
   accept. Fixed to pass `op.menu` straight through. `buildIntent`'s `args`/`input` fields fixed the
   same class (`UiValue | null`, not `| undefined`). `private seq` promoted `number → bigint`
   (`UiIntent.seq: bigint`, confirmed via the generated binding) — genuinely was the wrong type before
   the regen surfaced it, not a regression I introduced. Inline tests: full `StyleSpec`/
   `AccessibilitySpec`/`Component` literals (were `{}`/bare `{type: "..."}`), `layoutEpoch: bigint`.
7. **`PluginRuntime/🟦️component.tsx`** (the `🔖️RetainedUiPatch` region, my only permitted scope there):
   `retainedSurfaceToSnapshot`'s `layoutEpoch: 0 → 0n`.
8. **`Shell/🟦️component.tsx`**: `windowUiByWindowId`/`panelUiByKey`/`spawnedWindowUi` and their reducer
   action payload types were still `UiNode` (deleted) — migrated to `BuiltNode`, matching #3's
   `pendingWindowUiNode`/`pendingPanelUiNode` return type exactly (Shell holds these as reducer state
   that ShellHost reads). This does **not** unblock `ShellHost` (registrar-only, independently broken
   on ~15 other old-shape usages — see below), but it is the correct type for the one file I'm allowed
   to touch.
9. **`IconRenderHost/🟦️component.tsx`**: `IconRenderRequest` was imported from `@semio-tech/framework`,
   where it has never lived — it's defined in `@semio-tech/ui-styling`, re-exported by
   `@semio-tech/ui-react` (which this file already imports `iconRenderPort`/`IconShotFrame` from).
   Moved the import to the correct source. Unrelated to the migration; a pre-existing wrong-package
   import, one-line fix.
10. **`⚛️react/🧪️index.test.ts`**: 2 `layoutEpoch: number → bigint` fixes (same regen-driven class as
    #6/#7) in the file's own `buildContractSnapshot`-adjacent local fixture builders.

## found, not fixed — the biggest residue

**`ShellHelpers/🟦️component.tsx`** imports 3 names Interpreter no longer exports
(`declarativeTreeDragController`/`renderUiControl`/`uiTreeNodeToTreePanelConfig`, lines 169/172/173)
and 3 deleted types (`UiControlNode`/`UiNode`/`UiTreeNode`, lines 90/92/93) feeding a whole local
tree-panel-config subsystem (`uiNodeToTreePanelConfig`/`declarativeUiNodeToTreePanelConfig`/
`declarativeUiChildToTreeItems`, ~lines 1621–1760). **Not fixed**, for the same two reasons I removed
platform's inspector-tree subsystem rather than migrate it:

1. **No rendering path exists for it.** `Interpreter`'s current `TreeView`/`treeItemToTreeData`
   (`#region Tree`) reads only `label`/`description`/`icon`/`defaultOpen`/`dimmed`/`draggable`/
   `dragData`/`rowActions`/`bindings` off a `treeItem` record — it never recurses into a non-`treeItem`
   child as an embedded row control. A type-correct `BuiltNode` tree built to feed this subsystem would
   still render nothing extra; the feature this code implements (an editable control inside a tree row)
   has no landing spot in the new Interpreter yet.
2. **`ActionDescriptor → ActionBinding` needs a real product decision.** Every field here that used to
   carry a plain `ActionDescriptor` (`{controllerId, action, args?}`) now needs a versioned
   `ActionBinding.action: ActionId` (`{scope, name, version}`) — and there is no version to put there;
   inventing one (`version: 1`?) is a real semantic choice about action-registry versioning, not a
   mechanical rename, and not mine to make unilaterally for a whole subsystem.

Both were **already independently confirmed** by `react-renderer`'s and `react-tests`'s own reports as
a genuine runtime crash (`TypeError: uiTreeNodeToTreePanelConfig is not a function`, blocking 3 named
tests) and flagged as needing "the natural next step... a `shell-host` migration packet." I did not
attempt a third, differently-shaped fix under time pressure; I confirmed their finding still holds and
narrowed exactly why (`TreeView`'s own switch, read line-by-line) rather than re-guessing. This is a
`shell-host`-scale packet, not a residual cleanup — flagging for the coordinator to scope separately.

The other 90 `ShellHelpers` diagnostics (`947`→`3825` by line number) are **pre-existing, unrelated**:
`LocalizedLabel`/`UiRibbonParentCategory`/`CommandDefinition.semantics`/`IconName`/`ActionArgDef`/
`ToggleGroupItemProps`/`TutorialUiSnapshot`/`ElementProps.id` mismatches — none reference
`UiNode`/`Component`/`UiNodeRecord`/the tree-panel subsystem, confirmed by grep across every one.

**`ShellHost/🟦️component.tsx`** — **registrar-only** (explicit in `📌️important.md`'s registrar-file
list: `ShellHost/🟦️component.tsx`). 81 diagnostics, dominated by `as UiNode` casts (5 sites) and
`InterpretedUiNode`'s prop shape (`{node, onAction}` in ShellHost's usage vs the real, current
`{store, onAction, onIntent, requestContextMenu?}`). Not touched, not counted as mine to fix — this is
squarely the "next step" packet both prior reports named.

## suppressions

**None added by me.** Audited every file I touched for `any`/`as unknown as`/`@ts-ignore`/
`@ts-expect-error` after finishing: every hit predates my edits (verified by reading each site in
context — e.g. `Interpreter:276`'s `node as unknown as UiComponentSceneNode` is the pre-existing
scene-bridge cast neither `react-renderer` nor I touched; `🧪️index.test.ts`'s ~20 casts are all in
`describe` blocks neither this packet nor `react-tests` rewrote). One genuine "type cannot be satisfied
as written" finding, not suppressed: `SeparatorProps = Record<string, never>` (a truly-empty Rust struct)
intersected into `Component`'s union makes a bare `{ type: "separator" }` object LITERAL fail
TypeScript's structural check (`Property 'type' is incompatible with index signature: Type
'"separator"' is not assignable to type 'never'`) — reproduced standalone in isolation, confirmed not
this file's bug. Worked around in `UiDocumentStore`'s own test fixture by using a `container` literal
instead (real fields, no empty-`Record` intersection) rather than casting past it — same underlying
defect would hit ANY code authoring a bare separator literal against `Component` repo-wide; flagging
for whoever owns the contract's TS mirror generation (ts-rs's `Record<string, never>` rendering for a
genuinely-empty Rust struct is the root cause, not something this packet can fix in a hand-written
barrel).

## missing-typecheck-target finding

Confirmed: **no `typecheck` nx target exists anywhere in my scope**, and one precedent exists elsewhere
in the repo (`@semio-tech/ui-react`, `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📋️project.json`)
to mirror:
```json
"typecheck": {
  "executor": "nx:run-commands",
  "options": { "cwd": "<package dir>", "command": "bun ./📜️script.ts typecheck", "forwardAllArgs": true }
}
```
with a `TypecheckScript` class in the package's own `📜️script.ts` running
`runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root, ...)`.

- **`@semio-tech/framework-renderer-react`** (`📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/`):
  has `📋️project.json` and `📜️script.ts` already (test/lint targets exist) but **no `tsconfig.json` and
  no `typecheck` target** — add both, mirroring the pattern above. This package's tests run and pass
  green while its own type errors (95 in `🧪️index.test.ts`, 6 in `📦️index.tsx`, plus everything upstream
  it imports) are invisible to every gate.
- **`@semio-tech/plugin-window-kits`** (`🔌️plugin/🪟️window-kits/`): has **no `project.json` at all** (not
  just no `typecheck` target — no nx project registration whatsoever; only a bare `package.json` with 7
  `exports` entries and no `scripts`). Its own `import.meta.vitest` inline tests (one per file, 7 total)
  are in no gate either — confirmed no vitest config anywhere under this directory. Needs a
  `📋️project.json` + `📜️script.ts` (test + typecheck targets, same shape as every other package) created
  from scratch — a whole missing package registration, not a one-line target addition.

## registrar-requests

1. Add the `typecheck` targets above (2 packages, one needs a new `project.json`+`script.ts` entirely).
2. `ShellHost/🟦️component.tsx` migration (registrar-only, ~15 old-shape usages, 81 tsc errors) — needs
   its own packet; `Shell`/`ShellHelpers` sides of the same boundary are now correctly typed (`Shell`
   fully, `ShellHelpers`'s import surface pending the tree-panel redesign above) and waiting on it.
3. `ShellHelpers`'s tree-panel-config subsystem — needs a design decision (ActionId versioning +
   TreeView row-control rendering) before it can be migrated; see **found, not fixed**.
4. `🛂️manifest/🤖️generated/🟦️manifest.ts`'s 42 internal diagnostics (missing `Label`/`StyleSpec`/
   `UiMenuRef`/etc. imports inside the generated file itself) — registrar/generator-owned, reported
   verbatim per your instruction, not touched.

## test results

`bun x vitest run --config 🧪️vitest.config.ts --reporter=verbose` (real run, exit 1, expected):
**436 tests, 4 files — 423 passed, 13 failed.** Exact 1:1 match, by name, with `react-tests`'s own
already-recorded 423/13 baseline (its report's "Named fail set (13)") — **zero regression, zero new
passes** from my TS-only fixes, as expected (I touched no runtime logic `react-tests` didn't already
cover; my changes were type-only except the `docSchema` guard, which no existing test exercises with a
malformed schema). Classification (unchanged from `react-tests`'s own, reconfirmed by re-running):

- **(a) pre-existing, unrelated** (9): `s workflow flow routing > isolates render faults in
  ShellFaultBoundary`; `window action panel — staging and single dispatch (P1/P2)` ×3; `registry-derived
  utilities and activation (P5) > resolveWindowActions surfaces only panel-eligible definitions owned by
  the window`; `resolveCommands / commandCategories... > commandCategories orders and dedupes categories
  by first appearance`; `shell option locks (SEMIO_LOCKED_*)` ×2; `buildCommandCategoryTree /
  buildCommandCategoryTabs... > buildCommandCategoryTabs builds one namespaced PanelTabLeaf per
  category...`.
- **(c) caused by the UI migration, still open** (4): `framework renderer hosts > interprets virtual
  file system component scenes` (blocked on `📁️VirtualFileSystem`'s own unrelated `Table` story-vs-real
  import bug, not mine to fix); `s workflow flow routing > attaches a drag-and-drop controller to tree
  panels whose items carry drag data`, `s workflow flow routing > omits the drag-and-drop controller for
  tree panels without drag data`, `registry-derived utilities and activation (P5) >
  panelTabDefinitionToNode maps the framework-injected History panel tab through its rendered body` (all
  3 blocked by `ShellHelpers`'s broken import of the 3 deleted Interpreter exports — see **found, not
  fixed**).
- **(b) caused by the migration and fixed by this packet**: none in the vitest suite — every
  migration-caused runtime failure was already fixed by `react-renderer`/`react-tests`; this packet's
  own work was type-only (no behavior change reachable by an existing test) except the `docSchema`
  guard, which has no test yet (flagging: a test asserting placeholder-not-throw on a malformed
  `docSchema` would be genuinely new coverage — did not add it under time pressure, noting honestly
  rather than skipping silently).

Two `Unhandled Rejection` "postMessage requires 2 arguments" errors from `🟦️backbone-worker.ts` inside
jsdom — pre-existing (both `react-renderer` and `react-tests` reports already named this exact file/
error as unrelated).

## files touched

- Edited: `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`, `🧰️framework/🔨️modules/🖥️platform/🟦️component.ts`,
  `🧰️framework/🔨️modules/🔺️mesh/🟦️component.ts`
- Edited (migrated to `BuiltNode`): `🔌️plugin/🪟️window-kits/{🖼️image,🎬️media,📄️document,🌳️tree}/🟦️component.ts`
- Edited (return-type narrowed to `UiComponentSceneNode`): `🔌️plugin/🪟️window-kits/{📊️table,📝️text,🧊️mesh}/🟦️component.ts`
- Edited: `📺️renderer/🧑️‍🎨️engine/🧱️elements/{Interpreter,UiDocumentStore,Shell,IconRenderHost}/🟦️component.tsx`
- Edited (scoped `🔖️RetainedUiPatch` region only): `📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
- Edited (2 lines): `📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- Not touched (forbidden/registrar-only): `ShellHost/🟦️component.tsx`, `Shell/🧊️component.rs`,
  `🤖️generated/**`, all Rust, `project.json`/`package.json` anywhere, the plugin fleet
  (`✏️s/🔌️plugins/**`)
- Scratch (this session, not committed): `/private/tmp/claude-501/.../scratchpad/scratch-tsconfig.json`,
  `tsc-run{1..13}.txt`, `vitest-run1.txt`
