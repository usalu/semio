# 📓️ terra — react-tests — report

Packet `react-tests` (wave W4): migrate the React renderer package's main `🧪️index.test.ts`
(~336 tests) onto the semantic contract. `react-renderer` (this same wave) already migrated the
production files (`Interpreter`/`UiDocumentStore`/`PluginRuntime`/barrel) and left this file as
explicit out-of-scope follow-up — see `📓️terra-react-renderer-report.md`'s "remaining work inside
OWNS not completed".

## done

Migrated every test in `🧪️index.test.ts` that exercised the old `UiNode` shape onto the new
contract (`UiDocumentStore` + `UiSnapshot`/`UiNodeRecord`/`Component`), following the exact
`leaf`/`snapshot` fixture convention `UiDocumentStore`'s and `Interpreter`'s own inline test suites
already use — no parallel fixture format invented.

1. **`describe("framework plugin runtime" > "applyUiPatchToRetained")`** (4 tests) — `PatchOp::Replace`'s
   payload is now a whole `UiSnapshot`, not a recursive `UiNode`; a successful `applyUiPatchToRetained`
   result is a `UiDocumentState` (`{ revision, root, nodes: ReadonlyMap }`), not `{ revision, node }`.
   Rewrote the local `leaf`/`snapshot` fixture builders and every assertion to read
   `result.surface?.nodes.get(id)` instead of `result.surface.node`.
2. **`describe("framework external slots")`** — `renders external slot fallback text when unresolved`:
   `Component::Extension` (`ExtensionProps`) replaces the old `externalSlot` UiNode; `pluginId`/`appId`/
   `bodyKey` collapse into one `extension` address string.
3. **`describe("declarative forms parity")`** (11 of its original tests, plus 1 deleted — see
   **deleted tests**) — every `interpretUiNode({...oldNodeShape}, {onAction})` call site rewritten to
   build a real `UiSnapshot` via a new local `buildContractSnapshot`/`renderContractTree` helper (added
   right after `noopAction`, `#region 🧪️Contract test fixtures`) and go through the real
   `UiDocumentStore.loadSnapshot` → `interpretUiNode(store, context)` path — never a hand-rolled shadow
   renderer. Field-by-field mapping used throughout (all confirmed against the real ts-rs bindings in
   `🧬️contract/📦️packages/🦀️rust/bindings/*.ts` and `Interpreter/🟦️component.tsx`'s own View
   components, not assumed):
   - old `field`/`group` UiNode → `Component::Container` with `role: "field"` / `role: "group"`
     (`ContainerProps`); the old `child: Box<UiNode>` singular is just `children[0]` on the record.
   - old `stack` → `Component::Container` (`role: "plain"`) + the record's own `LayoutSpec` (`kind:
     "stack"`), never a node-level `direction`/`gap` field.
   - `selected`/`activate` (old node-level fields) → `UiPresenceOverlayContext` (selection, fed via a
     new `renderContractTree(root, presenceByKey)` overload) + the record's own `bindings` (activation,
     `Trigger::Activate`) — presence is deliberately NOT a document field per the contract's own design
     (`PresenceOverlay` region's doc).
   - `button`/`input`/`slider`/`numberStepper`/`image` → their matching `Component` variant + props
     (`ButtonProps`/`InputProps`/`SliderProps`/`NumberStepperProps`/`ImageProps`); `disabled`/`action`
     moved off the component onto the record (`record.disabled`, `record.bindings`).
   - **rewritten, not just ported**: `"tokenizes stack node gap/padding instead of hardcoded rem inline
     styles..."` → renamed `"resolves stack gap/padding through the closed SpaceToken scale as inline
     CSS, and keeps separators off raw border-border"`. The new architecture DELIBERATELY resolves
     `LayoutSpec` gap/padding to inline CSS via `spaceTokenRem` (`Interpreter`'s own
     `layoutSpecStyle`/`LayoutAndStyle` region) — the old assertion `not.toContain("style=")` is now
     false BY DESIGN, not a regression (`react-renderer`'s own decisions doc already flagged this). The
     real requirement behind the old test — "never a raw/arbitrary rem value, always a closed token" —
     is still real and still testable, so I kept it, asserting the gap resolves to exactly the
     `SpaceToken` "xs" → `0.2rem` lookup value (`SPACE_TOKEN_MULTIPLIER`/`uiSpacingRem`), not an
     arbitrary number.
4. **`describe("framework renderer hosts")`** — `interprets virtual file system component scenes`:
   rewrote to build a `Component::Surface` (`SurfaceProps`, `kind: "virtualFileSystem"`,
   `doc.bytes: Array.from(encodePackValue({schemaJson, rowsJson}))`) instead of the old
   `componentScene`/`virtualFileSystem` node fields — matches `surfacePropsToComponentSceneNode`'s
   (`Interpreter/🟦️component.tsx`) real decode path exactly, the same bridge every other still-passing
   `Component::Surface` corpus case already goes through. **This test still fails** — not from the
   migration, see **production bugs found, NOT fixed** #4.
5. **3 trivial "accepts ... component scene nodes" tests** (`framework renderer types` ×2,
   `world 3d scene fields` ×1) — these never called any renderer function; they only type-annotated a
   plain literal as `: UiNode` and read a property off it (no `interpretUiNode` call), so they were
   passing at RUNTIME the whole time (the type-only annotation is erased by `esbuild`/vitest) — but
   `UiNode` no longer exists as an exported type (`🛂️manifest/🟦️component.ts`'s own comment: "the
   hand-written `UiNode` recursive-union mirror this file used to carry" — deleted), so the annotation
   was a real (if silent) `tsc` error. Since the fixture data itself is still 100% valid (it feeds the
   OLD, UNCHANGED `UiComponentSceneNode` shape the 14 unowned scene-host elements still consume via
   `Component::Surface`'s bridge — not something the new contract touches), this is not "removed
   behaviour": I just dropped the stale `: UiNode` annotation (`const node: UiNode = {` → `const node =
   {`), letting TS infer the literal's own shape. No assertion changed.
6. **`import { type UiNode, ... }`** removed from the `@semio-tech/framework` import block; added
   `type Component`, `type UiNodeRecord`, `type UiSnapshot`, `type ActionBinding`, `type LayoutSpec`
   (all real ts-rs-generated contract types, same source `Interpreter`/`UiDocumentStore` themselves
   import from). Added `UiDocumentStore`, `type UiInterpreterContext`, `UiPresenceOverlayContext`,
   `type UiPresenceOverlayEntry` to the existing `./📦️index.tsx` barrel import block.
7. **New local test infra** (`#region 🧪️Contract test fixtures`, right after `noopAction`):
   `ContractNodeSpec` (a small nested-tree DSL: `key`/`component`/`layout?`/`disabled?`/`bindings?`/
   `children?`), `buildContractSnapshot` (walks it into a flat `UiSnapshot`, auto-assigning ids
   depth-first so root is always id `0`), and `renderContractTree` (real `UiDocumentStore` +
   `interpretUiNode`, optionally wrapped in a `UiPresenceOverlayContext.Provider`). This is
   test-fixture-building infra, not a parallel implementation of anything production owns — every
   assertion still runs through the real `interpretUiNode`/`UiDocumentStore` production code.

## acceptance

**`bun ./📜️script.ts test`** (`= bun nx run @semio-tech/framework-renderer-react:test`, the package's
own declared `test` target in `📋️project.json` — not invented) and the equivalent direct
`bun x vitest run --config 🧪️vitest.config.ts --reporter=verbose` (used for the named breakdown below,
same numbers both ways) — both real runs, exit 1 both times (expected — 13 named failures remain, none
of them mine to fix, see below).

- **436 unique tests, 4 files** (`🧪️index.test.ts` + the 3 `import.meta.vitest` in-source suites in
  `UiDocumentStore`/`Interpreter`/`PluginRuntime`). **423 passed, 13 failed.**
- **Vitest-config double-count check (explicitly required by this packet)**: inspected
  `🧪️vitest.config.ts` before touching anything — its `include` is left at vitest's DEFAULT glob
  (which is what discovers `🧪️index.test.ts`) and `includeSource` lists the 3 element files
  ADDITIVELY, never repeating a name across both keys (the file's own comment already says so, and I
  verified it directly: `bun x vitest run --reporter=verbose` shows exactly 4 distinct file paths, no
  file appearing twice, `Test Files 1 failed | 3 passed (4)` matching). I did not edit this config —
  the double-counting/silently-not-running trap `react-renderer` already fixed in this same wave.
  **Verbose-run name confirmation**: every rewritten test appears BY NAME in the `--reporter=verbose`
  output with a `✓`, e.g. `🧪️index.test.ts > declarative forms parity > resolves stack gap/padding
  through the closed SpaceToken scale as inline CSS, and keeps separators off raw border-border`,
  `... > framework plugin runtime > applyUiPatchToRetained > a root Replace on a fresh surface (no
  previous body) is applied`, `... > framework external slots > renders external slot fallback text
  when unresolved` — not just present in the summary count.
- **Named pass set (the 16 tests this packet's migration made pass)**, all previously red with either
  `store.getState is not a function` / `Cannot read properties of undefined (reading 'nodes')` /
  `declarativeTreeDragController is not a function`, all now green:
  - `framework plugin runtime > applyUiPatchToRetained > a root Replace on a fresh surface (no previous body) is applied`
  - `framework plugin runtime > applyUiPatchToRetained > a root Replace with a matching baseRevision advances the retained body`
  - `framework external slots > renders external slot fallback text when unresolved`
  - `declarative forms parity > renders declarative text with appearance-aware foreground`
  - `declarative forms parity > renders field description, required marker and inline error`
  - `declarative forms parity > renders slider unit readout`
  - `declarative forms parity > renders numberStepper as a single-border Stepper control, not hand-rolled double-bordered buttons`
  - `declarative forms parity > shows the mixed-values placeholder on a non-uniform numberStepper`
  - `declarative forms parity > renders a group node as a labeled section nesting its child controls (Origin > X/Y/Z steppers)`
  - `declarative forms parity > resolves stack gap/padding through the closed SpaceToken scale as inline CSS, and keeps separators off raw border-border` (renamed, see **done** #3)
  - `declarative forms parity > passes number bounds and file accept to inputs`
  - `declarative forms parity > disables gated wizard buttons`
  - `declarative forms parity > renders selectable builder cards with selection ring`
  - `declarative forms parity > renders image nodes from url sources`
  - (the 3 trivial `: UiNode`-annotation fixes in `framework renderer types`/world-3d were already
    green at runtime before my edit — fixed for `tsc`, not counted here as a vitest flip)
- **Named fail set (13)** — every one read and classified, none silently accepted:
  - **1 still-red, migration-adjacent, blocked by an unrelated production bug** (not the 16 above,
    not fixable here): `framework renderer hosts > interprets virtual file system component scenes`
    — see **production bugs found** #4.
  - **3 blocked by a confirmed ShellHelpers production bug**, unowned/forbidden (see **production
    bugs found** #1): `s workflow flow routing > attaches a drag-and-drop controller to tree panels
    whose items carry drag data`, `s workflow flow routing > omits the drag-and-drop controller for
    tree panels without drag data`, `registry-derived utilities and activation (P5) >
    panelTabDefinitionToNode maps the framework-injected History panel tab through its rendered body`.
  - **9 pre-existing, unrelated to `UiNode`/`Component`/this packet at all** (verified by inspection —
    none touch the contract): `s workflow flow routing > isolates render faults in ShellFaultBoundary`
    (chai `toHaveTextContent` matcher not registered), `window action panel — staging and single
    dispatch (P1/P2)` ×3 (a staged-args dispatch count, a missing DOM element for a `change` event, a
    null `input()` lookup), `registry-derived utilities and activation (P5) > resolveWindowActions
    surfaces only panel-eligible definitions owned by the window` (a set-membership mismatch),
    `resolveCommands / commandCategories... > commandCategories orders and dedupes categories by first
    appearance` (a category label text mismatch, `"Document"` vs `"Artifact"`), `shell option locks
    (SEMIO_LOCKED_*)` ×2 (two footer-logo path regexes), `buildCommandCategoryTree /
    buildCommandCategoryTabs... > buildCommandCategoryTabs builds one namespaced PanelTabLeaf per
    category...` (a call-argument shape mismatch). These read as concurrent, in-flight edits elsewhere
    in this live tree (U2: "the working tree is the baseline, never HEAD") — identical to what
    `react-renderer`'s own report already found in this exact file before I touched it. Confirmed still
    present, still not mine to fix (none reference `UiNode`/`Component`/`interpretUiNode`).

- **`bun x tsc --noEmit`**: could not run `-p tsconfig.json` directly — `🧰️framework/🔨️modules/🖱️ui/
  🧠️runtime/📦️packages/🦀️rust/📜️script.ts` is STILL mid-edit (unterminated string literal,
  confirmed unowned/unrelated, same file `react-renderer`'s report already flagged). Used a scratch
  tsconfig (`extends` the root, one absolute-path `exclude` for that file — real repo-relative
  `exclude` entries don't apply across an `extends` boundary from a scratchpad-located config, so the
  path had to be absolute) at
  `/private/tmp/claude-501/.../scratchpad/scratch-tsconfig.json`. Real run, exit 2, saved at
  `/private/tmp/claude-501/.../scratchpad/tsc-run4.txt`.
  - **10103 diagnostic lines repo-wide** — within U5's named externally-red baseline range
    (~8800–10100), not a regression against it.
  - **`🧪️index.test.ts`: 84 diagnostics.** Read every one and classified:
    - **4 are the repo-wide `TS5097 allowImportingTsExtensions` cascade** (my scratch tsconfig lacks
      the per-package setting every file in this repo relies on) — not real bugs, same class
      `react-renderer`'s report already excluded for its own files.
    - **1 genuine bug of my own, found and fixed**: my first-draft `renderContractTree` cast
      `{onAction, onIntent} as UiInterpreterContext` (missing the `store` field) — `tsc` correctly
      flagged "neither type sufficiently overlaps" (this is real: the missing generated
      `🤖️ui-contract.ts` collapses MOST cross-file contract types to `any`, but `UiInterpreterContext`
      itself is declared and exported directly by `Interpreter/🟦️component.tsx`, a real local file
      that DOES exist, so it stayed a real, checked type). Fixed by building a real, fully-typed
      `context: UiInterpreterContext = { store, onAction, onIntent }` object instead of a same-shape
      cast. Verified the fix: 85 → 84.
    - **The remaining 79 are pre-existing, in completely different `describe` blocks this packet never
      touched** (`WindowKindDefinition`/`UtilityNode`/`IconName`/`ActionDefinition`/
      `CommandDefinition`/`WindowMeasure`/`TutorialUiSnapshot`/`applyMutations`/`PluginManifest`
      mismatches, an unrelated `parseSpacePanelState` import, `OrthographicCamera`/`Camera` in the
      world-3d host tests, etc. — spot-checked several by line/context; none reference `UiNode`,
      `Component`, `UiNodeRecord`, or `interpretUiNode`). These are the SAME class of concurrent,
      in-flight peer-packet fallout the vitest run's own 9 unrelated failures show, just visible to
      `tsc` and not to vitest (a type mismatch a test never actually exercises at runtime). Not caused
      by, and not fixable within, this packet.

## deleted tests

**1 deletion**, reason recorded inline in the file as a comment at the deletion site
(`declarative forms parity`, end of block) as well as here:

- **`"dispatches the tree drop action with payload, target and position"`** — called
  `declarativeTreeDragController` (`./📦️index.tsx` re-export), a standalone pure function taking a
  whole tree `UiNode` + a dispatch callback and returning a `TreeDragAndDropController`. This function
  was deliberately not ported forward by `react-renderer` (its own decisions doc; the barrel's in-file
  migration comment says so too) — drag/drop for a `tree` component is now wired INSIDE
  `Interpreter`'s own `TreeView` (built from the record's own `drop` `ActionBinding`), not a
  separately-importable factory this packet's OWNS can call in isolation. There is no equivalent unit
  boundary left to test this way. The deletion surfaced a real production-bug flag, see next section.

Total test count moved 437 → 436 (exactly this one deletion; every other count change is a rename/fix
in place, not an add or remove).

## production bugs found, NOT fixed (routed to the coordinator)

All four are in files outside this packet's OWNS (`Interpreter`/`UiDocumentStore` production
bodies are FORBIDDEN to me even though I own their tests; `ShellHelpers`/`📁️VirtualFileSystem` are
different packets' files entirely) — reported, not touched.

1. **`🧱️elements/ShellHelpers/🟦️component.tsx` lines 169/172/173** import `declarativeTreeDragController`,
   `renderUiControl`, `uiTreeNodeToTreePanelConfig` from `../Interpreter/🟦️component.tsx` — none of
   these three names exist in `Interpreter` anymore (`react-renderer`'s own decisions doc: deliberately
   not ported forward). This breaks `uiNodeToTreePanelConfig` (line 1625, calls
   `uiTreeNodeToTreePanelConfig` at line 1629) and, transitively, `panelTabDefinitionToNode` (line 1484,
   calls `uiNodeToTreePanelConfig` at line 1511) at RUNTIME: `TypeError: uiTreeNodeToTreePanelConfig is
   not a function`. Blocks 3 named tests (listed above). Already flagged by `react-renderer`'s own
   report as "3 errors, unowned" and as the reason a `shell-host` migration packet is the natural next
   step — this confirms it is a real runtime crash, not just a type error.
2. **`🧱️elements/Interpreter/🟦️component.tsx` line 856**, `TreeView`'s internal drag controller:
   `return { handleDrop: () => dispatchTrigger(context, record, "drop") };` — calls `dispatchTrigger`
   with NO `input` argument at all, discarding the drop event's target/payload/drop-position entirely.
   The OLD `declarativeTreeDragController` this replaces dispatched a rich `{ kind, targetId,
   dropPosition }` args object built from the real drop event. Not something I can fix (forbidden
   file) or test around (the deleted test above had no successor assertion to make). Surfaced only by
   attempting to migrate that deleted test — flagging so whoever migrates drag/drop next knows the new
   `TreeView` path is a functional narrowing, not just a rename.
3. **`🧱️elements/UiDocumentStore/🟦️component.tsx` lines 512/516/520** — `useUiNode`, `useUiDocumentRoot`,
   `useUiDocumentRevision` all call `useSyncExternalStore(subscribe, getSnapshot)` with only 2
   arguments, omitting the `getServerSnapshot` 3rd argument React requires for SSR. Any consumer that
   renders an `interpretUiNode(store, ...)` tree through `renderToStaticMarkup`/`renderToString`
   (React's real SSR entry points — not hypothetical, this repo's own `🧪️index.test.ts` used exactly
   that call for the ENTIRE `declarative forms parity`/`framework external slots` blocks before this
   migration) throws `Error: Missing getServerSnapshot, which is required for server-rendered content`.
   I worked around this test-side by switching `renderContractTree` to client-side `render()` +
   `container.innerHTML` (a test-only choice, not a production fix) — but the underlying gap is real:
   this renderer cannot be server-rendered today, silently, until someone adds the 3rd argument.
4. **`🧱️elements/📁️VirtualFileSystem/🟦️component.tsx` line 19**:
   `import { Table } from "../🦴️Skeletons/🧪️story.tsx";` — imports the runtime `Table` VALUE from a
   Storybook story file, where it is `export const Table: Story = {...}` (a CSF story object, not a
   React component) — NOT from `../📊️Table/🟦️component.tsx` (the real `Table` component, which this
   same file correctly imports on the very next line, but ONLY for its TYPES:
   `type TableColumn`/`TableProps`/etc.). Every render of `<VirtualFileSystem>` that reaches its
   internal `<Table columns={columns} rows={rows} .../>` JSX throws `Error: Element type is invalid:
   expected a string ... but got: object. Check the render method of 'VirtualFileSystem'.` — confirmed
   with an isolated repro (added a temporary test to this package, removed after) using BOTH the
   `📁️VirtualFileSystem` package's own passing-elsewhere `VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA` fixture and
   a minimal one; both fail identically, proving this is unconditional and unrelated to fixture shape.
   This is the sole reason `framework renderer hosts > interprets virtual file system component scenes`
   still fails — the test itself (rewritten per **done** #4) is correct against the new contract.
   `📁️VirtualFileSystem` is owned by a different packet entirely (the generic `@semio-tech/ui-react`
   component library), not `react-tests`/`react-renderer`.

## vitest config findings

`🧪️vitest.config.ts` (in this package) was ALREADY fixed by `react-renderer` this same wave — see
**acceptance**'s double-count check above. No change needed or made by this packet. I inspected it
both before touching any test and after finishing, per this packet's explicit instruction; unchanged
both times, and the additive `include`/`includeSource` split (never the same file named in both) is
still correct.

## decisions

- **Prefer real `UiDocumentStore`/`interpretUiNode` production calls over hand-rolled assertions
  everywhere** — `renderContractTree` never bypasses the store; every markup assertion in this file now
  runs through the exact same `validateUiDocumentCore`/`applyOp`/`interpretUiNode` code path the
  conformance corpus and the app itself use.
- **Did not port the 3 trivial `: UiNode`-typed "accepts ... scene nodes" tests to the shared
  conformance corpus** — they test the OLD `UiComponentSceneNode` sub-field shapes the 14 unowned
  scene-host elements still consume (unchanged by this migration, bridged via
  `surfacePropsToComponentSceneNode`), not anything the corpus's `Component`/`UiNodeRecord`
  document-level fixtures describe. Fixing the stale type annotation was the minimal correct move;
  inventing a corpus fixture for content the corpus was never meant to cover would be new scope.
- **Client-side `render()` instead of `renderToStaticMarkup` in `renderContractTree`** — a test-only
  workaround for production bug #3 above (`useSyncExternalStore` missing `getServerSnapshot`), not a
  design choice about how the real renderer should work. Documented inline in the helper's own doc
  comment so the workaround doesn't read as an accidental drift from the rest of the file's SSR-based
  tests.
- **Kept, did not delete, the 3 ShellHelpers-blocked tests** (production bug #1) — they encode a real,
  still-valid requirement (tree-panel drag/drop wiring), the underlying function they call still HAS
  the right shape of intent (it's just crashing on an unrelated broken import), and per this packet's
  own rules a test blocked by a genuine bug is reported, not deleted or weakened.

## deviations

None from the packet brief. `bun x tsc --noEmit` needed a scratch config (documented above) rather than
running bare, consistent with `react-renderer`'s own precedent for the same broken peer file.

## files touched

- Edited: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
  (the only file in this packet's OWNS; 282 insertions / 275 deletions, net −1 test)
- Not touched: `🧪️vitest.config.ts` (already correct, see **vitest config findings**), `📦️index.tsx`
  (already correct, edited by `react-renderer` this same wave), all production files (`Interpreter`,
  `UiDocumentStore`, `PluginRuntime`, `ShellHelpers`, `📁️VirtualFileSystem` — all forbidden or unowned)
- Scratch (this session, not committed): `/private/tmp/claude-501/.../scratchpad/scratch-tsconfig.json`,
  `tsc-run{1,2,3,4}.txt`, `baseline-verbose.txt`, `run{2,3,4,5}-*.txt`, `nx-test-run.txt`,
  `my-file-errors.txt` — a temporary `🧪️zzz-repro.test.ts` was added to and removed from the real
  package directory during the `VirtualFileSystem` bug isolation (production bug #4); confirmed removed
  before finishing (`git status` on the package dir shows only `🧪️index.test.ts` modified).
