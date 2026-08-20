# 📓️ terra — react-renderer — report

Packet `react-renderer` (wave W4): migrate the React DOM renderer onto the semantic contract
(`semio-framework-ui-contract`). Anchor at open: `cb9bcce7a4`.

## done

1. **New element `UiDocumentStore`**
   (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx`)
   — a per-surface retained store, `{ revision, root, nodes: ReadonlyMap<UiNodeId, UiNodeRecord> }`.
   - `validateUiDocumentCore` and `applyOp`/`applyUiPatch` are a line-for-line TypeScript port of
     `🦀️limits.rs`'s `validate_core`/`apply_op`/`apply_patch` — same iterative (non-recursive) preorder
     walk, same violation set (`Cycle`/`OrphanChild`/`DuplicateSiblingKey`/`NodeQuota`/`DepthQuota`/
     `DanglingRoot`/`SectionNested`/`NonFiniteNumber`), same per-op quota checks
     (`children`/`textBytes`/`patchOps`/`patchBytes`), same UTF-8-byte-length accounting (not
     UTF-16 `.length`, which would silently disagree with Rust's `str::len()` on non-ASCII text).
     `DEFAULT_UI_DOCUMENT_LIMITS` is numerically identical to `UiDocumentLimits::default()`.
   - Transactional: `applyUiPatch` builds a draft `Map` cloned from the caller's state up front, applies
     every op to the draft only, validates the draft, and only on success returns a *new*
     `UiDocumentState` — the caller's original object is never mutated. `UiDocumentStore.applyPatch`
     reuses this and, on rejection, returns without touching `this.state` at all (reference-identical,
     not just value-equal — verified by a dedicated test).
   - Per-node subscription: `subscribeNode(id)`/`getNodeSnapshot(id)` via `useUiNode` +
     `useSyncExternalStore`. `notifyDiff` walks the old/new node maps and fires only the listeners of
     ids whose record *reference* changed (every mutating op produces a new object only for the node(s)
     it touches; untouched entries keep their exact prior reference) — this is what makes a
     `SetComponent` on one node re-render exactly that node's component. `subscribeRoot`/
     `subscribeRevision` are separate channels for root-pointer/version-level consumers.
   - `buildIntent`/`emitIntent` build a `UiIntent` carrying the store's own current `revision`, the
     node's `key`, and a per-surface monotonic `seq` — replaces the old `dispatch(controllerId, action,
     args)` plumbing.
   - **Decision, load-bearing**: `loadSnapshot` does **not** validate on ingestion (mirrors
     `crate::UiSnapshotState`'s `From<UiSnapshot>` exactly, which also doesn't validate — the crate's
     one validated entry point is `apply_patch`'s draft-then-validate flow, never a bare snapshot
     conversion). Confirmed against the conformance corpus itself: every `🚫️rejection` fixture loads
     its `.snapshot.json` unchecked and only asserts the *following* `.patch.json` is rejected — an
     earlier draft that validated on load broke `quota-depth` for exactly this reason (the base
     snapshot in that fixture is only 1 edge deep, over `maxDepth: 0`, and is meant to be accepted as
     the pre-patch state).
   - 10 inline (`import.meta.vitest`) tests: transactional apply/reject + reference-identity-on-reject,
     per-node listener isolation (a `setComponent` on one id fires only that id's listeners, never a
     sibling's), intent revision/seq.

2. **`Interpreter` rewrite**
   (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🟦️component.tsx`)
   — switches on `record.component.type` (18 variants: `container`/`text`/`button`/`separator`/
   `input`/`select`/`toggle`/`keyValueList`/`slider`/`numberStepper`/`ring`/`iconSelect`/`tree`/
   `treeSection`/`treeItem`/`image`/`surface`/`extension`) mapping onto the existing
   `@semio-tech/ui-react` components (`Button`/`Field`/`Section`/`Input`/`Textarea`/`Select`/`Toggle`/
   `Slider`/`Stepper`/`Ring`/`IconSelector`/`Tree`), reusing the same components and DOM-affordance
   patterns the pre-migration file used.
   - **`UiNodeView`** is the atomic per-node React unit: it alone calls `useUiNode(store, id)`, and
     `renderComponent` recurses into children by `id` only (`<UiNodeView id={childId} .../>`), never by
     passing a child's own record down — so a child's change can never re-render its parent. Verified
     by a `React.Profiler`-based test asserting a `SetComponent` on one node fires exactly that node's
     `onRender`, not a sibling's.
   - **Layout**: `layoutSpecStyle` resolves all six `LayoutSpec` kinds (leaf/stack/grid/overlay/
     scroll/absolute) to inline flex/grid CSS, every metric traced back through a closed `SpaceToken`/
     `Sizing`/`GridTrack`/`EdgeSpace` enum via `spaceTokenRem` (→ `@semio-tech/ui-styling`'s
     `uiSpacingRem`, itself resolved against the theme's `--ui-spacing` root) — never a raw pixel from
     the wire.
   - **Style**: `StyleSpec`'s five tokens (variant/size/density/tone/emphasis) are exposed as
     `data-variant`/`data-size`/`data-density`/`data-tone`/`data-emphasis` attributes rather than a
     guessed color mapping — flagged as a decision below, since tokens.json does not yet ship a full
     component-size/tone CSS ramp (already flagged by this ticket's own upstream `contract-layout`
     packet report).
   - **Accessibility**: `accessibilityAriaProps` maps `AccessibilitySpec` to real `aria-label`/
     `aria-describedby` (with a rendered visually-hidden description span)/`aria-live`/
     `aria-keyshortcuts`/`aria-hidden` — no `role` guess, since the semantic role is implied by
     `Component` itself, matching `🦀️accessibility.rs`'s own design.
   - **Activity/disabled/transition**: `record.activity`/`record.disabled` drive `aria-busy` and the
     existing loading/waiting border classes; a busy node (`loading`/`waiting`) renders the existing
     skeleton shell before its real component.
   - **Presence** (`UiPresenceOverlayContext`/`usePresenceOverlayEntry`) is a separate React context
     keyed by `UiNodeRecord.key` (not `UiNodeId` — matches `crate::PresenceUpdate`'s own keying, stable
     across a reconciliation that reassigns ids), fed from `PresenceUpdate` wire messages by whichever
     element owns the transport (outside this packet), and **never** read from or written into
     `UiDocumentStore` — presence changes at input frequency and must not touch a document revision.
   - **Intents**: `emitIntent(store, record, trigger, input?)` (from `UiDocumentStore`) looks up the
     node's own `ActionBinding` for `trigger` and mints a `UiIntent`; `dispatchTrigger` is the one call
     site every interactive component (`button`/`input`/`select`/`toggle`/`slider`/`numberStepper`/
     `ring`/`iconSelect`/tree row actions/a `container` with an `activate` binding) goes through.
   - **`Component::Surface` bridge**: `surfacePropsToComponentSceneNode` decodes `SurfaceProps.doc.bytes`
     (`Vec<u8>` → plain `number[]` on the wire) via `decodePackValue`, and reconstructs the OLD
     `UiComponentSceneNode` shape (`🔺️mesh/🟦️component.ts`, untouched, not in this packet's OWNS) the 14
     scene-host elements (`Canvas2dHost`/`World3dHost`/.../`EventFeedHost`) and `VirtualFileSystemHost`
     still expect — so none of those 15 unowned host components needed to change. The context-menu flow
     (`openSurfaceContextMenu`/`surfaceContextMenuTitleKey`/`PluginSurfaceActionsContext`/
     `ShellContextMenuFallbackContext`) is carried over unchanged, since none of it depended on the old
     `UiNode` shape.
   - **`tree`/`treeSection`/`treeItem`**: `TreeView` walks the store's own retained state (via
     `store.getState()`, re-subscribing on `useUiDocumentRevision`) to build `TreeDataSection[]`/
     `TreeDataItem[]` for the existing `Tree` component — row activate/hover come from the item
     record's own `bindings` (`Trigger::Activate`/`HoverPreview`), row actions from `TreeItemProps`'s
     own `rowActions: RowAction[]` (each already carries a full `ActionBinding`).
   - **Unknown component**: `UnknownComponentView` renders a visible `role="alert"` placeholder with
     `data-unknown-component` and calls `console.error` — never nothing. Covered by a dedicated test
     (constructs a record whose `component.type` matches no known tag).

3. **`PluginRuntime`** — scoped edit, exactly the `🔖️RetainedUiPatch` region
   (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`).
   This region bridges the MICROKERNEL kernel/actor WIT boundary's own `PatchOp`/`RetainedSurface`
   (unrelated protocol from `⚛️reactor/🩹️patches/🦀️component.rs`, coincidentally also named "patch") —
   its own header doc already says this boundary is "UNVERIFIED against a real compiled artifact... no
   plugin has migrated onto `world actor` yet," so this is a type-level migration, not a behavior
   change: `PatchOp.Replace`'s payload changes from a recursive `UiNode` (deleted) to a whole
   `UiSnapshot` (the new contract's actual whole-document type); `PatchOp.InsertChild`'s payload changes
   from a single `UiNode` to a single `UiNodeRecord` (a flat row, since there is no more recursive node
   shape to insert). `RetainedSurface` is now literally `UiDocumentState` (`UiDocumentStore`'s own
   state shape) built via the store's own `uiDocumentStateFromSnapshot` — one algorithm, not a second
   copy. `applyUiPatchToRetained`'s desync semantics (only a single root `Replace` accepted this wave;
   anything else is an honest desync) are unchanged.

4. **React barrel**
   (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`)
   — removed the 25 dead re-export lines naming deleted `Ui*Node` types (plus two unused value imports,
   `pendingWindowUiNode`/`pendingPanelUiNode`, confirmed unused *in this file* — their real callers,
   `ShellHost`/`ShellHelpers`, import them directly from `@semio-tech/framework`, bypassing this
   barrel); updated the `UiInterpreter` region's import/re-export block to match `Interpreter`'s actual
   new surface (`interpretUiNode`/`InterpretedUiNode`/`UiNodeView`/`UiPresenceOverlay*`/etc.), and added
   a re-export of `UiDocumentStore` and its hooks/`emitIntent` alongside it. Dropped re-exports for
   `renderUiControl`/`uiTreeNodeToTreePanelConfig`/`declarativeTreeDragController`/
   `declarativeSurfaceStatus` — none of these concepts exist in the new contract (no more `UiControlNode`
   union; tree conversion is internal to `TreeView`; a node's status is just `record.activity`).

5. **Test wiring** — `🧪️vitest.config.ts`
   (same `⚛️react` package dir) gained `includeSource` entries for all three touched element files.
   **Verified empirically this was a real, pre-existing gap, not a hypothetical one**: `vitest list`
   against the unmodified config found exactly one file (`🧪️index.test.ts`); `PluginRuntime`'s own
   26-case inline suite carried a comment saying so explicitly ("This file has no vitest project of its
   own... wiring a real project target later is a config change, not a test rewrite") — confirmed true,
   and now fixed. All 26 of those pre-existing `PluginRuntime` tests pass unmodified, which is reassuring
   independent evidence the scoped `RetainedUiPatch` edit didn't disturb the rest of that file.

## acceptance

Ran directly (packet ruling: "U4: no cargo; you MAY run TypeScript checks" — extended in practice to
the package's own vitest, per ACCEPTANCE's explicit allowance). No cargo command was run.

- **`bunx tsc -p <scratch-tsconfig> --noEmit`** (scratch config = root `tsconfig.json` +
  one `exclude` for the same pre-existing unrelated broken peer file `manifest-typegen`'s report
  already flagged, `🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts` — still mid-edit, still an
  unterminated string literal, confirmed unowned/unrelated). Saved at
  `/private/tmp/claude-501/.../scratchpad/tsc-run3.txt`.
  - **Exit 2, 8809 diagnostic lines** (baseline per U5/manifest-typegen's own run: ~8800–10100,
    externally red from the concurrent MICROKERNEL asyncify program — this run is within that range,
    not a regression).
  - **Started from 62 genuine errors across 16 files** (manifest-typegen's inventory). Of the files in
    this packet's OWNS:
    | file | started at | now |
    |---|---:|---:|
    | `Interpreter/🟦️component.tsx` | 6 | **0** |
    | `PluginRuntime/🟦️component.tsx` | 1 | **0** |
    | `⚛️react/📦️index.tsx` (react-renderer barrel) | 25 | **0** |
    | `⚛️react/🧪️index.test.ts` | 1 | **214** (see "remaining work in OWNS," below — not fixed) |
  - The 0-error files above are genuinely clean, not merely "no `Ui*Node` name found": I read every
    remaining diagnostic in each and classified it as (a) `TS5097`/implicit-`any`/"missing return"
    cascades from the still-ungenerated `🛂️manifest/🤖️generated/🟦️ui-contract.ts` (my scratch
    tsconfig lacks the per-package `allowImportingTsExtensions` setting every file in this repo relies
    on, and the contract types are `any` until `generate` runs — see **registrar-requests**), or (b) a
    genuine bug, which I fixed inline (a `ReactNode`/`ControlIcon` mismatch on the icon prop passed to
    `Button`/`Toggle`; a missing `id` prop `<Select>` requires; a stray `decodeScenePackField<T>()`
    type argument that function doesn't accept; `SurfaceDoc.bytes` decoded through the wrong codec —
    `decodeScenePackField` expects a `"pk:"`-prefixed *string*, but `Vec<u8>` renders as a plain
    `number[]`, so the fix is `decodePackValue(new Uint8Array(bytes))`). One bystander, NOT fixed
    (forbidden/out of scope) and NOT caused by this packet: ~20 `TS2820` errors in `Interpreter`'s
    (copied-verbatim) `contextMenuSurfaceTitleKeys`/`contextMenuTargetTitleKeys` tables — the
    `UiTranslationKey` union currently has no `ui.surfaceContextMenu.*` entries at all, which the
    pre-migration file already had latently (masked there by its own bigger failures never letting
    `tsc` reach these lines); flagging for whoever owns the i18n catalog.
  - **Total repo-wide count moved 8819 → 8809** (net −10) — small because `🧪️index.test.ts`'s count grew
    sharply (its ~15 `interpretUiNode(...)` call sites and object literals now use a fully incompatible
    old shape) while the other three files' counts fell to zero; see "remaining work in OWNS."

- **`bun nx run @semio-tech/framework-renderer-react:test`** (`vitest run --config
  🧪️vitest.config.ts --passWithNoTests`, real run, not UNRUN — under the packet's explicit vitest
  allowance). Full output at `/private/tmp/claude-501/.../scratchpad/vitest-final.txt`.
  - **437 tests, 4 files. `UiDocumentStore` (10/10), `PluginRuntime` (26/26, previously never executed
    at all), `Interpreter` (65/65 — 3 of my own plus all 62 conformance cases) all fully green.**
  - `🧪️index.test.ts`: 336 tests, **28 failed**. All 28 read and classified (none silently ignored):
    - **16 are direct, expected fallout of this migration** (`interpretUiNode`'s signature is no
      longer `(node: UiNode, context) => ReactNode`; `uiTreeNodeToTreePanelConfig`/
      `declarativeTreeDragController` no longer exist) — 12 in `describe("declarative forms parity")`
      /`describe("framework external slots")`/`describe("framework renderer hosts")` calling
      `interpretUiNode` with an old-shape node literal; 4 in `describe("s workflow flow routing")`/
      `describe("registry-derived utilities...")` going through `ShellHelpers`'s (unowned, forbidden)
      `uiNodeToTreePanelConfig`, which itself calls the now-removed `uiTreeNodeToTreePanelConfig`.
    - **12 are pre-existing, unrelated to `UiNode`/this packet at all** — verified by inspection, none
      touch `UiNode`/`Ui*Node`/`interpretUiNode`/`Component`: a chai matcher error
      (`toHaveTextContent` not registered), a `resolveWindowActions` set-membership mismatch, a
      category label text mismatch (`"Document"` vs `"Artifact"`), two footer-logo path regexes, a
      `buildCommandCategoryTabs` call-argument shape mismatch, a staged-args dispatch count, a missing
      DOM element in a change-event test, a null `input()` lookup, and a `postMessage` 2-argument
      TypeError inside `🟦️backbone-worker.ts` (unrelated file) surfacing as an unhandled rejection.
      These read as concurrent, in-flight edits elsewhere in this live tree (U2: "the working tree is
      the baseline, never HEAD") — not something to fix under this packet's OWNS.

## corpus conformance result

**62/62 conformance cases pass**, run for real (not simulated) through `UiDocumentStore.loadSnapshot`/
`applyPatch` — the exact same `validateUiDocumentCore`/`applyUiPatch` production code the store uses,
loaded straight from `🧬️contract/📚️examples/🧪️conformance/` at test time (no fixture copied or
hand-transcribed into the test file). Every `accept` case's retained tree shape (root/nodeCount/
id/key/type/children), every node's accessibility fields (label/description/live/shortcut/hidden, cross-
checked against the ARIA props `Interpreter`'s own `accessibilityAriaProps` produces from them), and the
full set of reachable `ActionId`s (formatted `scope.name@version`, matching `ActionId::Display`
verbatim) all match their `.expect.json`. Every `reject` case is rejected with the exact named
`PatchRejection`, and the store is asserted reference-identical before/after (not just value-equal).

**No case where React legitimately cannot match the GPU renderer was found** — the corpus is entirely
about document-level accept/reject semantics (tree shape, quotas, accessibility, action reachability),
which is renderer-neutral by construction; nothing in it exercises paint/layout-pixel output where the
two renderers' outputs would be expected to diverge.

One thing the corpus does **not** exercise, flagged for whoever owns the corpus next: it never renders
through actual React components (no DOM/ARIA-role assertions against a mounted tree) — my structural
checks assert the *inputs* to ARIA (the `AccessibilitySpec` fields) and the function that derives ARIA
props from them, not a live `getByRole` query against `Component::Surface`'s the 14 scene-host bridge,
tree row rendering, or the layout/style CSS resolution. That live-DOM layer is covered separately by
`Interpreter`'s own 3 non-corpus tests (unknown-component placeholder, per-node render-count granularity)
plus the pre-existing `🧪️index.test.ts` suite (where compatible), not by the corpus itself.

## remaining errors outside OWNS (next packet's work list)

Unchanged from `manifest-typegen`'s inventory except zeroed for this packet's four files (above). Still
broken, not owned, not touched — 29 genuine `Ui*Node`-deletion errors remain:

| package / module | file | errors |
|---|---|---|
| `@semio-tech/framework` barrel — `🔨️modules/🖥️platform/🟦️component.ts` | (same file) | 10 |
| `@semio-tech/plugin-window-kits` | `🌳️tree/🟦️component.ts` | 3 |
| ″ | `📄️document/🟦️component.ts` | 3 |
| ″ | `🖼️image/🟦️component.ts` | 2 |
| ″ | `🎬️media/🟦️component.ts` | 2 |
| ″ | `🧊️mesh/🟦️component.ts` | 1 |
| ″ | `📝️text/🟦️component.ts` | 1 |
| ″ | `📊️table/🟦️component.ts` | 1 |
| renderer-engine `🧱️elements/` (same product, NOT this packet's OWNS) | `ShellHelpers/🟦️component.tsx` | 3 |
| ″ | `ShellHost/🟦️component.tsx` | 1 (registrar-only, U7) |
| ″ | `Shell/🟦️component.tsx` | 1 |

`ShellHost`/`Shell` additionally now show **new runtime** (not just type) failures once exercised
through `🧪️index.test.ts` — `InterpretedUiNode`'s prop shape changed from `{node, onAction}` to
`{store, onAction, onIntent, requestContextMenu?}`, and `ShellHelpers`'s `uiNodeToTreePanelConfig` calls
the now-gone `uiTreeNodeToTreePanelConfig`. This is expected, unavoidable fallout of "no dual-path old
shapes are gone" (both files are explicitly FORBIDDEN to me; `ShellHost` is also registrar-only per U7)
— a `shell-host` migration packet is the natural next step, and should land before/alongside any packet
depending on windows/panels actually rendering again end to end.

## remaining work inside OWNS not completed

**`⚛️react/🧪️index.test.ts`** — NOT rewritten. This 5853-line file's `describe("framework plugin
runtime" > "applyUiPatchToRetained")`, `describe("framework renderer types")`,
`describe("framework external slots")`, `describe("declarative forms parity")`, and
`describe("framework renderer hosts")` blocks (15+ call sites) construct old-shape `UiNode` object
literals and call `interpretUiNode`/`applyUiPatchToRetained` with the old 2-argument signature. Every
one of these needs hand-conversion to a `UiSnapshot`+`UiDocumentStore` fixture preserving its original
test intent — genuinely substantial, self-contained follow-up work (my own estimate: on the order of the
Interpreter rewrite itself, given the literal count and variety of node shapes exercised). I chose not
to attempt a rushed version of this rather than risk silently weakening test coverage; flagging it
honestly here rather than claiming it done. The 214 tsc errors and 28 vitest failures in this file are
the accurate, current measure of that remaining scope (16 of the 28 vitest failures map onto it
directly; the other 12 are pre-existing/unrelated, see **acceptance**).

## decisions

- **`UiDocumentStore.loadSnapshot` does not validate** (see **done**, item 1) — deliberately matches
  `crate::UiSnapshotState`'s own `From<UiSnapshot>`, confirmed against the corpus itself rather than
  assumed.
- **`StyleSpec` tokens render as `data-*` attributes, not resolved CSS** — tokens.json has no
  component-size/tone ramp yet (already flagged upstream); this keeps the renderer honest about what it
  actually knows rather than guessing a color mapping, and is a pure CSS-layer addition later, not a
  wire change.
- **`Component::Surface` bridges to the OLD `UiComponentSceneNode`/`ComponentSceneHostProps`
  (`🔺️mesh/🟦️component.ts`, untouched)** rather than rewriting the 15 unowned scene-host elements —
  keeps this packet inside its OWNS boundary; the bridge is one function
  (`surfacePropsToComponentSceneNode`), not a shim layer threaded through every host.
- **`PluginRuntime`'s edit is type-level only** — the kernel/actor WIT boundary it bridges is a
  different, unrelated "patch" protocol (MICROKERNEL program, its own header doc: unverified, no plugin
  has flipped onto it yet), so there is no real wire behavior to preserve or break here, only types to
  align with the new contract's actual whole-document/single-record shapes.

## registrar-requests

- **`bun nx run @semio-tech/ui-contract-rs:generate` has still not been run** (confirmed again this
  session: `🛂️manifest/🤖️generated/🟦️ui-contract.ts` does not exist on disk, via both a `find` and an
  independent `os.listdir` check per U8.8). Every file in this packet's OWNS imports types from it
  through `@semio-tech/framework`; until it runs, `bunx tsc` on these files shows cascading
  `any`-typed-cascade diagnostics (implicit-`any` callback params, "missing return" on otherwise-
  exhaustive switches) that are not real bugs — I traced and excluded every one, but a real generate run
  followed by a fresh `tsc` pass is the only way to get a genuinely clean diagnostic count for this
  packet's files.

## deviations

- Two bugs found and fixed in my own first-draft tests while debugging against the real
  implementation (not shipped broken, but noting per U8.4's "never claim a check passed without its
  output" spirit — these were caught BY running the suite, which is the point): a stale `baseRevision`
  left over from an earlier test-writing pass, and a granularity test that (before the fix) rendered a
  child node twice — once directly, once nested inside its own parent's real recursive rendering —
  which would have made the test assert a false property rather than the one it's named for.
- `renderUiControl`/`uiTreeNodeToTreePanelConfig`/`declarativeTreeDragController`/
  `declarativeSurfaceStatus` are not ported forward under any name — see **decisions**; their concepts
  don't exist in the new contract. Not a silent removal: called out here and in the barrel's own
  in-file migration comment.

## files touched

- Created: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx`
- Rewritten: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🟦️component.tsx`
- Edited (scoped, `🔖️RetainedUiPatch` region only): `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
- Edited: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- Edited: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- Not touched (forbidden/not owned): `ShellHost/🟦️component.tsx`, `Shell/🟦️component.tsx`,
  `ShellHelpers/🟦️component.tsx`, `🛂️manifest/**`, `🤖️generated/**`, all Rust crates, the plugin fleet.
- Scratch (this session, not committed): `/private/tmp/claude-501/.../scratchpad/scratch-tsconfig.json`,
  `tsc-run{1,2,3}.txt`, `vitest-final.txt`, `debug-store.mjs`.
