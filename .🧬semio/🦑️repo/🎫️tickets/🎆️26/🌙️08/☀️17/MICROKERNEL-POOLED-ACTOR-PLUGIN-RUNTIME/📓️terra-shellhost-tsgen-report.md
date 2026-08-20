# 📓️ terra — shellhost-tsgen — report

Packet `shellhost-tsgen` (executor "terra", coordinator "sol"). Scope: `ShellHost/🟦️component.tsx` and its
immediate boundary, the framework TypeScript generator, and two typecheck registrations. No Rust source
touched.

## how I type-checked

Same scratch tsconfig `react-w4`/`react-renderer` established (repo root `tsconfig.json` extended,
`allowImportingTsExtensions: true`, the same two live/mid-edit `script.ts` files excluded — both still
present and unrelated to me):

```
bunx tsc -p <scratchpad>/scratch-tsconfig.json --noEmit
```

15 full repo-wide runs over the session (`tsc-run1.txt` … `tsc-final.txt` in this ticket folder's
scratch, plus copies of the two new packages' own `bun nx run <project>:typecheck` output). Baseline
confirmed first: **81** `ShellHost` diagnostics, **42** `manifest.ts` diagnostics — exact match with the
brief's own figures.

## Item 1 — `ShellHost/🟦️component.tsx`: 81 → 0

**Command:** `bunx tsc -p <scratchpad>/scratch-tsconfig.json --noEmit`, filtered to `ShellHost` lines.
**Before → after: 81 → 0** (last full run, `tsc-final.txt`).

### The retained-mode migration (the core ask)

- Import: `type UiNode` (deleted) → `type BuiltNode`. All ~9 usage sites (`resolveExternalSlots` calls,
  `pendingWindowUiNode`/`pendingPanelUiNode` cache reads, the `SET_SPAWNED_WINDOW_UI` dispatch value,
  `patchWorld3dChromeOntoNode`/`patchDocumentTreeSelectedIds` call sites) renamed to `BuiltNode` — every
  cast source was already `unknown`, so no new structural risk.
- **New `//#region 🔖️BuiltNodeReconciler`** (module scope, before `FrameworkOsShell`):
  - `builtNodeToSnapshot(surface, root, revision?, layoutEpoch?): UiSnapshot` — DFS pre-order id-minting
    reconciler turning an authored `BuiltNode` tree into the flat `UiSnapshot` `UiDocumentStore.loadSnapshot`
    needs. Ids are minted fresh per call (documented as intentional: `loadSnapshot`'s own doc already
    treats every call as a whole-body replace, so cross-call id stability was never the store's contract).
  - `reportUnwiredUiIntent(intent)` — see "found, not fixed" below.
  - `withLocalizedWindowKindLabels` — see the `LocalizedLabel`/`unknown` section below.
- **New `builtNodeStoresRef`/`builtNodeStoreFor(key, node)`** (component scope, plain `useRef`-backed
  memoization, NOT a custom hook — the 3 call sites live inside `useMemo`/`.map()` callbacks, where a
  hook call would violate the Rules of Hooks). One stable `UiDocumentStore` per window/panel/spawned key,
  reloaded via `loadSnapshot` only when that key's `BuiltNode` reference actually changed (the reducer's
  own `mergeRecordPreservingIdentity` already keeps an unchanged window's node reference-stable across a
  refresh, so this rarely reloads).
- All 3 `<InterpretedUiNode>` call sites migrated from the old `{node, onAction}` props to the real
  `{store, onAction, onIntent, requestContextMenu?}` shape: `store={builtNodeStoreFor(key, node)}`,
  `onIntent={reportUnwiredUiIntent}`.
- `declarativeSurfaceStatus` (deleted from `Interpreter`, import removed) — its job is now trivial since
  `BuiltNode` carries `activity` directly: both call sites became `windowUiByWindowId[id]?.activity`
  (`Activity = "waiting"|"loading"|"idle"|"finished"`, structurally identical to the `UiStatus` the
  `status?:` prop wants — confirmed by reading both generated unions).

### Presence invariant (explicitly checked, not violated)

Audited every presence-adjacent site I touched. `builtNodeStoreFor`/`UiDocumentStore` never receive
presence data — `usePresenceOverlayEntry`/`UiPresenceOverlayContext` (the render-plane channel) are
untouched, and the render-plane vs. collaboration-roster split (`ArtifactPresencePeer`, the backbone
heartbeat) stays on its own channel, never routed into a document revision. No merge found; nothing to fix.

### `onIntent`: real gap found, reported rather than faked

`InterpretedUiNode`'s `onIntent` is a required prop. There is **no plugin-facing dispatch for `UiIntent`
today** — `PluginWasmHandle` (`PluginRuntime/🟦️component.tsx`) has `handleAction` for the legacy
`ActionDescriptor` channel only, no `UiIntent`-shaped counterpart. Wiring a real one needs an `ActionId`
(`{scope,name,version}`) → wasm-call convention that doesn't exist yet — the SAME versioning gap already
flagged as needing a design decision for `ShellHelpers`' tree-panel-config subsystem. `reportUnwiredUiIntent`
logs the intent loudly (`console.error`) instead of silently dropping it, matching `Interpreter`'s own
"never throw, never silently drop" convention for an unresolved contract gap. **Flagging for the
coordinator; not invented.**

### `ExternalSlotResolverContext`/kernel `PluginWasmHandle`: real adapter, not a cast

`resolveExternalSlots`/`ensureContributorInstance` (kernel/component.ts) take a `PluginWasmHandle` that is
a **genuinely different type** from this file's own (`manifest: () => Promise<Uint8Array>`/`enqueue`/
`outcomes`/`dispose` — an actor/turn handle — vs. this file's `manifest: PluginManifest` value-shaped
handle; `PluginRuntime` itself already renames kernel's own import to `KernelPluginWasmHandle` to keep them
apart). A direct `as` cast fails with TS2352 ("neither type sufficiently overlaps") — genuinely, not from
laziness. Read both real call sites before deciding: `ensureContributorInstance` only calls
`.createApp(appId)`; `resolveExternalSlots` only checks the handle's truthiness — the actual external-slot
render path is an **explicit, already-documented stub** in kernel/component.ts ("the dedicated follow-up
work package", always returns "Extension unavailable"). Built a real adapter object per plugin
(`createApp`/`destroyApp` forward to the real handle; `manifest`/`enqueue`/`outcomes`/`dispose` are honest
no-ops, never invoked by either function today) instead of `as unknown as`.

### `buildContributionsJson`: kernel `PluginManifest` missing a required field

Same root cause class: kernel's own `PluginManifest` (`workflows` required, no `?`) is a stale, narrower
type than this file's real one (`Shell/🟦️component.tsx`'s, which never carries `workflows` at all).
Fixed by adding `workflows: []` at the two call sites (`{ ...entry.manifest, workflows: [] }`) — a real,
faithful "no workflow data" value, not a cast. Confirms kernel/component.ts itself has **5 own pre-existing
diagnostics** (`PluginUiNode` missing entirely, `PluginManifest.contributions`/`ProgramContributionEntry.
contribution` don't exist on their own declared types, 2 implicit-`any` params) — not touched, out of
scope, reported here and in `🎠️kernel` is not in my grant.

### Genuine bug found and fixed: `Shell/🟦️component.tsx`

`SpaceProgramEntry`/`SpawnedAppEntry` each declared **`label` twice** (`readonly label: string;` followed
immediately by `readonly label: readonly string[];` — TS silently keeps only the last declaration, no
error). `ShellHost` was already constructing/reading a `breadcrumb: readonly string[]` field that never
existed under that name; `ShellHelpers`' own `appBreadcrumb(breadcrumb: readonly string[] | undefined)` /
`resolveAppBreadcrumb(): readonly string[]` confirm the intended field name and shape. Renamed the second
`label` to `breadcrumb` in both types — a real bug (obvious copy/rename typo), not a redesign.

### `flattenPanelTabLeaves`: a genuine TypeScript "weak type" limitation

`flattenPanelTabLeaves<T extends { readonly children?: readonly T[] }>` rejects `PanelTabNode` (`PanelTabLeaf
| PanelTabBranch`) with "has no properties in common" — `PanelTabLeaf` carries no `children` key at all, and
a constraint of *only* optional properties is TS's "weak type" detection, which requires the source to
share at least one of those properties. (`PanelTabDefinition`, the OTHER real caller, has `children` as a
*required* field on a non-union type, so it was never affected — confirmed by testing both shapes in
isolation before touching anything.) Rather than loosen the shared constraint (which broke the
`PanelTabDefinition` caller when I tried it) or force an explicit type argument (still rejected — same
weak-type check applies to explicit args too, verified empirically), added a **sibling** helper in
`ShellHost` itself, `flattenPanelTabNodeLeaves`, built on the union-aware `panelTabChildren()` accessor that
already exists in `PanelTabBar/🟦️component.tsx` for exactly this reason. `flattenPanelTabLeaves` itself is
unchanged from its original (pre-my-touch) signature.

### `React.KeyboardEvent`/`React.MouseEvent` name-shadowing (3 sites, real bug)

`ShellHost` imports React's `type KeyboardEvent`/`type MouseEvent` under their bare names, shadowing the
global DOM types file-wide. `useShellKeydown(rootRef, handler: (event: KeyboardEvent) => void, deps)` (from
`ShellScope/🟦️component.tsx`, unshadowed there) needs the **native** type; two local handlers
(`handleAppKeydown`/`handleCommandKeydown`) and a `window.addEventListener("contextmenu", ...)` listener
were declared against the shadowed React type. Fixed all 3 with an explicit `globalThis.KeyboardEvent`/
`globalThis.MouseEvent` annotation (including the nested `matches(event, chord)` closure inside
`handleAppKeydown`, which shared the same shadowed name). `keyboardEventMatchesChord` (ShellHelpers,
exported, same shadowing bug in ITS OWN file) widened to a small structural `KeyboardEventLike` type
(`key`/`ctrlKey`/`metaKey`/`shiftKey`/`altKey` — the only 5 fields it actually reads, confirmed by reading
the whole function body) so it accepts either shape honestly, rather than casting at each of its 2 callers.

### `ArtifactPresencePeer`: `cursor`/`viewport` genuinely gone from the wire

The presence-heartbeat `peer:` literal set `cursor`/`viewport` fields that no longer exist on
`ArtifactPresencePeer` (`@semio-tech/framework-replication`'s own type — confirmed against `ArtifactActorMsg`'s
`presenceHeartbeat` variant, the real wire source of truth). Removed both from the literal (dead fields, not a
suppression); added the newly-required `views: []` (a real, empty, honest "no open windows reported"
default matching that field's own doc comment). **Not fixed, flagged**: `presenceCursorRef` is still
actively tracked (a `pointermove` listener) but now has nowhere to go on the wire — left tracked rather than
torn out, since removing the listener is a product decision about whether cursor-sharing returns, not a
type fix.

### `mutationEnvelopeFromWire`/`mutationEnvelopeToWire`/`MutationEnvelope`: wrong package

Imported from `@semio-tech/framework-os`, which re-exports neither — they live in
`@semio-tech/framework-replication` (confirmed: `🛍️products/💻️os/🟦️component.ts`, `framework-os`'s own root
file, imports them from that exact package for its own internal use). Already symlinked at repo-root
`node_modules` (no `package.json` edit needed). Same "wrong package" defect class `react-w4`'s report
already named for `IconRenderRequest`.

### `LocalizedLabel`/`unknown` boundary (the file's own established pattern, extended)

`WindowKindDefinition.label`/`CommandDefinition.label`/`ExampleDefinition.label` (and siblings) are all
`unknown` — "no ts-rs mirror yet" is the field's own doc comment; a genuine, acknowledged upstream typegen
gap, not something fixable from this file. Followed the file's OWN pre-established convention (already used
for other fields before I touched anything): assert the known real shape (`LocalizedLabel | string`) at the
read site, then run it through `resolveManifestLabel`. Applied at ~24 sites (12 individual `.label` reads,
12 `windowKinds` array arguments via a new `withLocalizedWindowKindLabels` helper — verified in isolation
that a direct array-level `as` cast between the two shapes compiles, since the only differing field
(`label`) is `unknown` on the source, which TS always treats as cast-compatible in either direction).

### `UiLabel` brand (3 sites)

`title`/`label` fields typed `UiLabel` (`string & {brand}`) were being assigned plain `string` (from
`windowTitlesById[id] ?? appWindowLabel(...)`, `resolveAppLabel(...)`). Wrapped with `wireLabel()` — the
file's own existing "mint a `UiLabel` from an already-resolved string" helper (imported from `Interpreter`),
not a new pattern.

### Other one-off fixes
- `TutorialDocumentEventKind` (deleted) — **first attempt was wrong**: renamed to `TutorialEventKind`
  (tsc's own "did you mean" suggestion), which compiled but was semantically incorrect — `slice.document`'s
  entries are edit/load/undo/redo/checkpoint events, not action/command/key events. Caught it because the
  rename introduced 20 NEW diagnostics (`"command"|"action"|"key"` has no overlap with `"edit"`/`"load"`/…)
  on the very next tsc run; the real type is `TutorialArtifactEventKind` (manifest/component.ts). Also
  fixed the two `documentJson`/`previousJson` field names to the real `documentDsl`/`previousDsl`, and cast
  the "edit" variant's opaque `forwards`/`backwards: readonly unknown[]` to `readonly MutationEnvelope[]`
  (the type's own comment: "opaque per-app mutation JSON").
- `TutorialBase.documentJson` → `documentDsl`; `TutorialTracks.document` → `artifact` (matches
  `TutorialArtifactEvent[]`) — 2 sites in the tutorial recorder + 1 tutorial-base read site.
- `TutorialUiSnapshot.selectionJson` → real per-domain diff over `interactionSelection` (`Record<string,
  DomainSelection>`), replacing the deleted opaque string with the documented replacement the type's own
  comment names.
- `Effect::LoadDocument`'s dead JSON-fallback branch removed (`payload.documentJson` doesn't exist on the
  real `{pack, spr}` wire shape — confirmed via kernel/component.ts's own `Effect` union; the branch was
  unreachable dead code from before the pack/spr-only migration, not something to work around).
- `PluginInstallOutcome`/`ActionPaneSlice`/`TutorialUiBridgeContext` — all 3 already correctly defined in
  `ShellHelpers` but declared as **module-private** (no `export`). Exported all 3 (pure visibility change,
  zero semantic change) and imported them into `ShellHost`.
- `example.iconId` (doesn't exist on `PluginManifest.examples`'s real `{id,label,documentJson,appId}`
  shape) — generic `"file"` fallback icon, with a comment naming why.
- `presencePeersJson` on `PluginViewState` — a local, host-side-only field piggybacked onto `viewState`
  (visible at both the write site, `ensureBackboneWorker`'s presence-event branch, and now the two read
  sites), not part of `PluginViewState`'s real wire contract. Read back through the accurate extended shape
  (`ViewModel & { readonly presencePeersJson?: string }`) rather than the declared one.
- `buildFrameworkSyncUtilities(...) as readonly UtilityNode[]` — **removed, not replaced**. The function
  already returns `readonly FrameworkSyncUtilityLeaf[]` (its own declared return type); the cast was simply
  the wrong target type for `SyncAttachCard`'s real prop, a stale workaround with no purpose. This is the
  one place I found and REMOVED an existing (harmless but pointless) type assertion.
- `activeUtilityByWindowId[id]` (`Record<string, string | null>`) passed into params typed
  `string | undefined` — 3 sites, `?? undefined` (the file's own already-established `T|null → T|undefined`
  conversion idiom, per `react-w4`'s report).
- `session?.app.id === hostAppId && session.app.controllerId` — TS doesn't narrow `session` through an
  optional-chained equality; rewritten `session && session.app.id === hostAppId && ...` (2 sites).

## suppressions

**None added by me.** Every `as unknown as`/`@ts-ignore`/`@ts-expect-error` in the 3 files I edited was
audited and found pre-existing, unrelated to anything I touched:
- `ShellHost`: 2 `import.meta.env` access-pattern casts (Vite typing gap, common idiom), 1
  `entry.manifest.apps as unknown as Record<string, unknown>[]` inside `AppRouter.build` (a different call
  site than anything I touched).
- `ShellHelpers`: 3 WebCodecs feature-detection casts (`window as unknown as {VideoDecoder?...}`, a
  `CanvasImageSource` cast) — all unrelated to anything I edited.
- `Shell/🟦️component.tsx`: 1 `import.meta.env` cast, same pattern as ShellHost's.

One genuine "type cannot be satisfied as written" case, not suppressed (see above): `ExternalSlotResolverContext`'s
`PluginWasmHandle` — solved with a real adapter object instead of a cast.

## Item 2 — the framework TypeScript generator

**Confirmed the coordinator's diagnosis for 3 of 14 missing type names, corrected it for the other 11.**

### What's genuinely a cross-mirror import gap (coordinator's diagnosis, confirmed)

`Label`, `StyleSpec`, `WindowStackCorner` (15 of the 42 diagnostics) are real `semio-framework-ui-contract`
types, exported by that crate's OWN typegen (`🖱️ui/🧬️contract/📦️packages/🦀️rust/tests/typegen_export.rs`),
landing in the sibling mirror `🟦️ui-contract.ts` — never in `🟦️manifest.ts` itself. `consolidateBindings`'s
`stripTsRsBoilerplate` strips ALL `import` lines unconditionally, which was correct while every referenced
type lived in the one file being consolidated and is now wrong.

**Fix implemented** in `🧰️framework/📦️packages/🦀️rust/📜️script.ts` (`GenerateScript`/`CheckScript`'s shared
`consolidateBindings`): after flattening the body, scan the sibling `🟦️ui-contract.ts` mirror for every
top-level `export type`/`export interface` name; for each one the manifest body references in actual code
(not inside a `/** */` doc comment — stripped first) but does not itself declare, emit
`import type { ... } from "./🟦️ui-contract.ts";` right after the `@generated` header. Falls back to `""`
(no import line at all) when the sibling mirror is absent, so a from-scratch checkout still generates.

### What ISN'T a cross-mirror gap — a DIFFERENT defect, corrected finding

The other **11 unique type names / 27 diagnostics** (`UiMenuRef`, `ConfigSpec`, `CommandGrammar`,
`ArtifactPresentation`, `FileTypeContribution`, `TopicContribution`, `IoEntryDescriptor`,
`ComposerEntryDescriptor`, `UiTreeActionPlacement`, `Locale`, `Terminology`) do **not** exist in
`🟦️ui-contract.ts` either — confirmed by grep. Traced each to its real source:
- `ConfigSpec`/`CommandGrammar`/`ArtifactPresentation`/`FileTypeContribution`/`TopicContribution`/
  `IoEntryDescriptor`/`ComposerEntryDescriptor` are `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]`-
  decorated types living **in the exact same file** as the framework crate's own typegen test
  (`🛂️manifest/🦀️component.rs`), simply never called via `.export()` in that test's ~200-line manual list
  (`exports_typescript_bindings`, lines 6810–7009).
- `UiMenuRef`/`Locale`/`Terminology`/`UiTreeActionPlacement` are decorated types in `semio-framework-ui`
  (`ui_wgpu` crate — a normal, non-optional dependency the framework crate already reaches via
  `ui_wgpu::wgpu::X` for the types it DOES export, e.g. `UiButtonNode`), also simply never individually
  `.export()`'d.

**No generated mirror anywhere declares these 11 names**, so there is nothing for a generator-side import
fix to import — confirmed empirically: I rebuilt the fixed manifest in isolation (see idempotency below) and
these 27 diagnostics persist exactly as predicted, unchanged by the import fix.

**Root cause and the real long-term fix (found, not applied — beyond my grant):** the framework crate's
typegen test calls `X::export()` per type (writes only that type's own file, no transitive walk), so any
type reachable only as a FIELD of an exported type — never itself individually listed — silently never gets
a binding file. `semio-framework-ui-contract`'s own typegen test already solved this exact problem the right
way: it uses `<T as TS>::export_all_to(dir)` (transitive dependency walk) instead, and its own doc comment
says so explicitly — quoting it: *"the manifest-typegen's registrar-requests ask for `#[ts(export_to = …)]`
there instead; this test is the 'meanwhile' workaround"*. Migrating the framework crate's own test to the
same `export_all_to` pattern (or adding the 11 missing individual `.export()` calls) is Rust source beyond
a single `#[derive]`/attribute — outside my grant, flagged here per the "say so before doing it" instruction
rather than done unilaterally.

### Verification

**Could not run the real `bun nx run @semio-tech/framework-rs:generate`/`:check`** — `cargo test --features
typegen exports_typescript_bindings` fails to COMPILE with **19 pre-existing errors**, entirely unrelated to
the manifest (E0599/E0277 "trait bound `protocol_core::{HierarchyProvider,HoverSpec,SelectionSpec,
SelectionMode,SelectionMethod,MergeMode,DomainSelection}: TS` is not satisfied" — a feature-unification/
typegen-propagation gap in `semio-framework-os-kernel`'s hover/selection module, `📡️replication/📡️wire/
🦀️component.rs`, committed 2026-08-19 15:51, confirmed NOT live via `git log --date=iso` + `git status`
clean on that path). Blocked before I could reach it; not mine to fix (Rust, shared, out of scope).

**Verified the generator's LOGIC in isolation instead** (full script + output saved in this ticket's
scratch, `verify-generator-fix.ts`): fed the real, committed `🟦️manifest.ts`/`🟦️ui-contract.ts` through
the exact new `crossMirrorImportLine`/`declaredTypeNames`/`stripBlockComments` functions (copied verbatim
from the real fix), producing `import type { Label, StyleSpec, WindowStackCorner } from
"./🟦️ui-contract.ts";` — exactly the 3 names the manual analysis predicted, alphabetically ordered. Wrote
the resulting fixed file into an isolated scratch directory alongside a copy of the real `ui-contract.ts`
and a minimal standalone `tsconfig.json`, then ran `tsc` on JUST those two files: **42 → 27 diagnostics**,
and the 27 remaining are byte-for-byte the 11-name residue predicted above (confirmed by diff). **Never
wrote to the real `🤖️generated/🟦️manifest.ts`** at any point — `git status` on that path stays clean
throughout.

**Idempotency**: ran the isolated verification script twice; `diff` on the two outputs is empty
(byte-identical). This proves the NEW LOGIC is deterministic; it does not substitute for the real
`generate`/`check` round-trip, which needs the blocked cargo build.

## Item 3 — two typecheck registrations

### `@semio-tech/framework-renderer-react`

Added `tsconfig.json` (mirrors `@semio-tech/ui-react`'s own, the one existing `typecheck`-target precedent
in the repo) and a `TypecheckScript` class + `"typecheck"` router registration in the existing `📜️script.ts`,
plus the target entry in `📋️project.json` (calls `bun ./📜️script.ts typecheck` only, per house convention).

```
bun nx run @semio-tech/framework-renderer-react:typecheck
```
**Exit 1**, 598 errors. **Zero of them mention `ShellHost`** (confirmed: `grep -c ShellHost` on the full
output → 0). The target is correctly wired and reports real, honest, pre-existing debt across this
package's full transitive dependency graph (`Interpreter`'s 19 i18n-related errors `react-w4` already named,
`ShellHelpers`'s 96 — mostly the tree-panel-config subsystem, explicitly out of scope — plus
`backbone-worker.ts`, `🦑️repo/📚️library`'s discovery module, and more, all pre-existing, none touched by
this packet). **Not green, honestly reported** — making this exact class of previously-invisible defect
visible is the whole point of the gate this packet was asked to add, not something to silently paper over
into a false-green target on day one.

### `@semio-tech/plugin-window-kits`

Had **zero** nx registration (only a bare `package.json`, 7 `exports` entries, no `scripts`). Created from
scratch, all 4 files matching house convention:
- `📋️project.json` — `test`/`typecheck` targets, both `nx:run-commands` calling `bun ./📜️script.ts <cmd>`
  only.
- `📜️script.ts` — `TestScript`/`TypecheckScript`, same shape as every other package's.
- `tsconfig.json`.
- `🧪️vitest.config.ts` — `includeSource` listing all 7 component files (6 carry `import.meta.vitest`
  blocks, 1 — `text` — doesn't, harmless to list), `include: []` (the double-collection trap, rule 18).

```
bun nx run @semio-tech/plugin-window-kits:test -- --reporter=verbose
```
**Exit 0.** 6 test files, 8 tests, all passing, each confirmed **by name** in the verbose output
(`renderDocument > renders one child per page`, `renderTree > expands nested children recursively`,
`renderMesh > carries the JSON blobs into the world3d scene`, `renderImage > builds a base64 data URI from
mime + base64`, `renderTable > serializes columns and rows into the table scene`, `renderTableRows` ×2,
`renderMedia > renders duration, position, and kind as key-value entries`) — genuinely NEW coverage, no
gate saw these before.

```
bun nx run @semio-tech/plugin-window-kits:typecheck
```
**Exit 1**, 66 errors. **Zero from this package's own 7 files** (confirmed by grepping the output for each
of the 7 component paths). All 66 are the SAME transitively-inherited pre-existing debt as above (kernel's 5
own bugs, `manifest.ts`'s 27-residue generator gap from item 2, `📡️replication/🟦️component.ts` missing
`encodePackValue`/`decodePackValue` names, `🖱️ui/🎨️styling`'s own package issues, discovery library) — none
touched, none mine.

**Two bugs found and fixed in my OWN new files before this final run** (both self-inflicted on the FIRST
draft, caught by running the target immediately rather than assuming): `📜️script.ts` needed the same
`declare global { interface ImportMeta { readonly dir: string } }` ambient-type workaround
`@semio-tech/ui-react`'s own `📜️script.ts` already documents (bun-types isn't installed in this
workspace); `🧪️vitest.config.ts`'s `mode: "test"` field doesn't exist on this vitest version's
`InlineConfig` (TS2769) — removed. **Flagging, not fixing** (out of scope, not mine): the exact same
`mode: "test"` line exists in `🧰️framework/📦️packages/🟦️typescript/🧪️vitest.config.ts` (the file I copied
the pattern from) and would fail identically if that package's OWN `typecheck` target were ever added.

## test baseline — confirmed by name

```
bun nx run @semio-tech/framework-renderer-react:test -- --reporter=verbose
```
**436 tests, 423 passed / 13 failed** — exact match with the documented baseline, re-run twice (once
mid-session, once after every ShellHost edit landed). All 13 failing names checked individually against the
brief's own classification:

- **(a) pre-existing, unrelated (9):** `isolates render faults in ShellFaultBoundary`; `window action panel
  — staging and single dispatch (P1/P2)` ×3 (`stages both args locally…`, `gates Execute on required
  args…`, `Reset restores defaults…`); `resolveWindowActions surfaces only panel-eligible definitions owned
  by the window`; `commandCategories orders and dedupes categories by first appearance`; `shell option locks
  (SEMIO_LOCKED_*)` ×2 (`ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND introduction is app-specific…`, `mit-bestand/
  demonstrator footer credits render…`); `buildCommandCategoryTabs builds one namespaced PanelTabLeaf per
  category…`.
- **(c) caused by the UI migration, still open (4):** `interprets virtual file system component scenes`;
  `attaches a drag-and-drop controller to tree panels whose items carry drag data`; `omits the drag-and-drop
  controller for tree panels without drag data`; `panelTabDefinitionToNode maps the framework-injected
  History panel tab through its rendered body` — all 4 blocked by `ShellHelpers`' broken import of 3 deleted
  `Interpreter` exports (the tree-panel-config subsystem), explicitly out of scope for this packet.

**Zero new failures, zero newly-passing tests** — expected, since every fix in this packet was type-only
except the `views: []`/`cursor` removal on the presence heartbeat literal (no existing test exercises that
code path with a real backbone worker) and the `Effect::LoadDocument` dead-branch removal (unreachable
before and after).

## files touched

- Edited: `ShellHost/🟦️component.tsx` (granted)
- Edited (immediate boundary, "keep type-correct" carve-out): `Shell/🟦️component.tsx` (breadcrumb bug),
  `ShellHelpers/🟦️component.tsx` (3 `export` additions, `KeyboardEventLike` type +
  `keyboardEventMatchesChord` signature widened, `flattenPanelTabLeaves` doc comment only — its own
  signature is UNCHANGED from before I touched it)
- Edited: `🧰️framework/📦️packages/🦀️rust/📜️script.ts` (the generator fix, granted)
- New: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/tsconfig.json`
- Edited: that package's `📋️project.json` (added `typecheck` target), `📜️script.ts` (added
  `TypecheckScript`)
- New: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/{📋️project.json,📜️script.ts,
  🧪️vitest.config.ts,tsconfig.json}` (full registration from scratch)
- Not touched: `🎠️kernel/🟦️component.ts` (5 own pre-existing bugs, confirmed and named, not mine — shared
  file, no lease), `Interpreter/🟦️component.tsx`, `UiDocumentStore/🟦️component.tsx`, `PluginRuntime/
  🟦️component.tsx` (already correctly migrated by prior packets, untouched), any Rust, `🤖️generated/**`
  (absolute rule — hand-edit never attempted, verified clean via `git status` throughout), any other
  `project.json`/`package.json`
- Unrelated concurrent change observed in git status, not mine:
  `📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` (a Rust file I never opened)

## registrar-requests / open findings for the coordinator

1. **`🛂️manifest/🤖️generated/🟦️manifest.ts`'s residual 27 diagnostics** (11 type names) need either the
   framework crate's typegen test migrated to `TS::export_all_to` (matching `ui-contract`'s own precedent
   and its own doc comment's stated intent) or 11 individual `.export()` calls added to
   `exports_typescript_bindings` — Rust source in `🛂️manifest/🦀️component.rs`, beyond a single
   `#[derive]`/attribute, not attempted per my scope boundary.
2. **`🎠️kernel/🟦️component.ts` has 5 own pre-existing diagnostics** (missing `PluginUiNode` type,
   `PluginManifest.contributions`/`ProgramContributionEntry.contribution` don't exist on their own declared
   types, 2 implicit-`any` params) blocking BOTH new `typecheck` targets from ever going green — a shared
   file, no lease held, not touched.
3. **`ExternalSlotResolverContext`/kernel's `PluginWasmHandle`** is fundamentally the wrong abstraction for
   what `ShellHost` actually has (an orphaned actor/turn-handle shape vs. this file's real
   `PluginRuntime`-backed handle) — worked around locally with a real adapter; the deeper fix (reconciling
   or retiring kernel's copy) is a design decision, not mine to make.
4. **`onIntent` has no plugin-facing dispatch** — the ActionId-versioning gap already named for
   `ShellHelpers`' tree-panel-config subsystem blocks this too; `reportUnwiredUiIntent` reports it loudly
   rather than silently dropping.
5. **`presenceCursorRef`** tracks pointer position with nowhere to send it (the wire field is gone) — left
   tracked, not torn out; a product call, not a type fix.
6. **`🧰️framework/📦️packages/🟦️typescript/🧪️vitest.config.ts`'s `mode: "test"`** field doesn't exist on the
   installed vitest version's `InlineConfig` — would fail identically if that package ever grows a
   `typecheck` target. Not touched (out of scope), flagged here since I copied its shape and found the bug.
