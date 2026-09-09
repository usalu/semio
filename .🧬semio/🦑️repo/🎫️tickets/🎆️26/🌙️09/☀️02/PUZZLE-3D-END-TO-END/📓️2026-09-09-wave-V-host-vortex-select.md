# Wave V — React host: marker picks on the generic interaction path, `[DEBUG]` sweep

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Scope: React host TypeScript under
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/` only. No Rust touched, no
cargo run, no browser run, no git state change, no ticket transition.

## TL;DR

1. **Vortex / target-volume / reference marker picks and hovers now take the generic
   `interactionSelect` / `interactionHover` path** whenever the scene is domain-bound
   (`scene.domainId`), at a granularity the **scene** decides — never host-side app knowledge. The
   unhandled `worldVortexSelect` / `worldVortexHover` dispatch is now only reached by an **unbound**
   scene, where it is the pre-existing legacy vocabulary (kept — see [§4](#4-legacy-fallbacks-kept-and-why)).
2. **Attractions are not pickable in this host** — `WorldAttractionLines` takes no pointer callbacks,
   so there is nothing to route. A granularity default is nevertheless declared for the layer so the
   day it becomes pickable there is no app-specific decision to make.
3. **Six temporary `[DEBUG]` traces removed** from `World3dHost` (the two gumball ones the brief named
   plus four more), and the dead `gumballDragDebugTickRef` they existed for. Two `[DEBUG]`-prefixed
   **permanent fault diagnostics** (Interpreter, ShellHelpers) kept but un-prefixed. `ShellHost`'s
   ~80 records deliberately untouched — see [§5](#5-debug-sweep).
4. Tests: `🔬️engine-contract` **445 passed** (443 pre-existing + 2 new), Interpreter in-source
   **65 passed**, `long` corpus **17/18 files green** with one pre-existing collection failure.
   `tsc --noEmit` on the renderer React package: **0 errors in any file this wave edited**.

---

## 1. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` | new `🎯️WorldInteractionDomain` exports (`World3dMarkerLayer`, `WORLD3D_DEFAULT_MARKER_GRANULARITY`, `World3dMarkerInteractionFields`, `world3dMarkerInteractionTarget`); the four marker record types gained the two optional scene-JSON interaction fields; `handleVortexSelect`, `dispatchVortexHover`, `handleTargetVolumeSelect`, `handleReferenceSelect`, `handleReferenceHover` gained the domain-bound branch; `vorticesRef`; six `[DEBUG]` traces + `gumballDragDebugTickRef` removed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` | re-exports the four new symbols (the package barrel is the only import surface the engine test suite has) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | two new cases (see [§6](#6-tests-run)) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` | dropped the `[DEBUG] ` prefix from the permanent unknown-component-type diagnostic (1 line) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | dropped the `[DEBUG] ` prefix from the permanent program-load-failure diagnostic (1 line) |

`🏛️ShellHost/🟦️.tsx` — **not edited**. It was written by a peer 2 minutes before this wave reached it
(mtime `01:48`, wave started `01:41`), and the standing rule is to leave a file a peer touched inside
the last ten minutes alone.

## 2. The design: granularity is the scene's statement, not the host's

The gap was that `handleVortexSelect` / `dispatchVortexHover` dispatched `worldVortexSelect` /
`worldVortexHover` **unconditionally**, while instance picks had long since been gated on
`scene.domainId`. Those two verbs are handled by **nothing** — see [§4](#4-legacy-fallbacks-kept-and-why).

A marker hit needs two things the instance path already had: a **target id** and a **granularity**.

* **Target id** — already correct with zero new plumbing. `vorticesJson[].fullId` is written by
  `puzzle3d_vortex_full_id(&object.id, &vortex.id)`, which is *character-for-character* the id
  `interaction_topology` declares for the `vortex` granularity
  (`✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🦀️.rs:6878`). Likewise `targetVolumesJson[].id` = `volume.id`
  and `referencesJson[].id` = `reference.id` versus `:6884` / `:6887`.
* **Granularity** — the scene schema carries exactly one (`World3dScene.domainGranularityId`,
  `🧾️typed/📇️catalog.json:22`), which the instance layer already consumes. There is no per-layer
  field, and adding one to the typed scene contract is a Rust change this wave may not make.

So the host now resolves it in two steps, in this order:

1. **The record's own scene-JSON fields.** `interactionGranularityId` and `interactionId` are read off
   each marker record. `vorticesJson` / `attractionsJson` / `targetVolumesJson` / `referencesJson` are
   `?text` blobs whose record shape lives in the plugin's own JSON builder and in this host's TS types
   — **no typed-scene contract change is needed for a plugin to start emitting them.** This is the
   authoritative channel: a plugin that names its marker rows something else says so here.
2. **The layer's own name**, when the record says nothing:
   `{ vortex: "vortex", attraction: "attraction", targetVolume: "targetVolume", reference: "reference" }`.
   These are `World3dScene`'s **own** field names, not any app's vocabulary — the exact same move
   `WORLD3D_DEFAULT_INTERACTION_GRANULARITY = "handle"` already makes for the instance layer. puzzle3d's
   `PUZZLE3D_GRANULARITY_VORTEX` / `_TARGET_VOLUME` / `_REFERENCE` / `_ATTRACTION` (`…/✏️editor/🦀️.rs:85-88`)
   happen to equal them, so **the marker path works today with no Rust change at all.**

```ts
export function world3dMarkerInteractionTarget(layer: World3dMarkerLayer, id: string, record?: World3dMarkerInteractionFields): { readonly granularity: string; readonly id: string } {
  return { granularity: record?.interactionGranularityId ?? WORLD3D_DEFAULT_MARKER_GRANULARITY[layer], id: record?.interactionId ?? id };
}
```

Nothing puzzle-specific entered the host, and nothing app-specific has to be added for the next
domain-bound world plugin: it either accepts the layer names or overrides them per record.

## 3. Exact dispatch shapes now sent

`dispatch` wraps every payload with `{ surfaceId: node.surfaceId, … }`; the tables below show the args
added on top of that. `merge` is `instanceMergeArg(resolveWorldMergeMode(…))` — the click-modifier /
persistent-mode resolution, unchanged. Every domain-bound row goes through
`world3dSelectionActionArgs` / `world3dHoverActionArgs`, i.e. the identical encoder the instance and
marquee paths use.

### 3.1 Domain-bound scene (puzzle3d: `domainId = "vortex"`, `domainGranularityId = "object"`)

| gesture | action | args |
| --- | --- | --- |
| instance (object) click — *unchanged, wave S* | `interactionSelect` | `{ domainId: "vortex", targets: "[{\"granularity\":\"object\",\"id\":\"seed-left-001\"}]", merge, method: "pick" }` |
| instance hover — *unchanged, wave S* | `interactionHover` | `{ domainId: "vortex", channel: "pointer", targets: "[{\"granularity\":\"object\",\"id\":\"seed-left-001\"}]" }` |
| background click (clear) — *unchanged, wave S* | `interactionSelect` | `{ domainId: "vortex", targets: "[]", merge, method: "pick" }` |
| **vortex marker click** | `interactionSelect` | `{ domainId: "vortex", targets: "[{\"granularity\":\"vortex\",\"id\":\"seed-left-001:v0\"}]", merge, method: "pick" }` |
| **vortex marker hover** | `interactionHover` | `{ domainId: "vortex", channel: "pointer", targets: "[{\"granularity\":\"vortex\",\"id\":\"seed-left-001:v0\"}]" }` |
| **vortex hover leave** | `interactionHover` | `{ domainId: "vortex", channel: "pointer", targets: "[]" }` |
| **target-volume click** | `interactionSelect` | `{ domainId: "vortex", targets: "[{\"granularity\":\"targetVolume\",\"id\":\"volume-1\"}]", merge: "replace", method: "pick" }` |
| **target-volume click, `locked`** | `interactionSelect` | `{ domainId: "vortex", targets: "[]", merge: "replace", method: "pick" }` |
| **reference click** | `interactionSelect` | `{ domainId: "vortex", targets: "[{\"granularity\":\"reference\",\"id\":\"ref-1\"}]", merge: "replace", method: "pick" }` |
| **reference click, `locked`** | `interactionSelect` | `{ domainId: "vortex", targets: "[]", merge: "replace", method: "pick" }` |
| **reference hover / leave** | `interactionHover` | `{ domainId: "vortex", channel: "pointer", targets: "[{\"granularity\":\"reference\",\"id\":\"ref-1\"}]" }` / `"[]"` |

A record carrying `{"interactionGranularityId":"pin","interactionId":"pin-7"}` yields
`[{"granularity":"pin","id":"pin-7"}]` instead — the override is exercised by the new test.

Locked markers deliberately dispatch an **empty-target replace** rather than a no-op: that is exactly
what the already-migrated background-clear path sends, so a locked pick still deselects instead of
silently keeping a stale selection (the legacy branch achieved the same with `worldPick {id: null}`).

### 3.2 Unbound scene (block3d `scene.domain_id = None`, cad, lowpoly, storybook fixtures)

Byte-identical to before this wave — `worldVortexSelect {fullId, merge}`,
`worldVortexHover {fullId}` / `{}`, `setSelection {selection:{…targetVolumeIds:[id]…}}`,
`setReferenceSelection {pane, referenceId}`, `referenceHover {referenceId}` / `{}`, and the
`worldPick {granularity, id: null, merge}` locked-clear.

## 4. Legacy fallbacks kept, and why

The brief allowed deleting the `worldPick` / `worldSelect` / `setHover` / `worldVortexSelect`
fallbacks only if nothing else relies on them. Grep says: **keep them.**

* `grep -rn '"worldPick"|"worldSelect"|"setHover"|"worldHover"|worldVortexSelect|worldVortexHover' --include='🦀️.rs'`
  finds **no plugin handler anywhere**. `✏️s/🔌️plugins/💠️lowpoly/…/🧪️tests/🔬️unit/🦀️.rs:290` actively
  asserts they are deleted, and the puzzle3d testkit docstrings call them "deleted".
* But `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` — the **native/wgpu** world host,
  a Rust file outside this wave — both *emits* (`:6165`, `:6189`, `:6204`, `:6233`, `:9418`, `:9869`,
  `:9935`) and *consumes* (`:9772`, `:9781`, `:9801`, `:9834`) that same verb vocabulary as its own
  internal action language. Deleting the React half would leave the two hosts speaking different
  languages for the unbound case.
* `📐️cad` really does handle two of the marker verbs — `setReferenceSelection` and `referenceHover`
  (`✏️s/🔌️plugins/📐️cad/…/✏️editor/🦀️.rs:898-899`, `:991-992`). cad's world scene never sets
  `domain_id`, so it keeps the legacy branch untouched. This is the concrete reason the new behaviour
  is **gated** on `interactionDomainId` rather than replacing the old path outright.
* `.storybook/stories/puzzle/3d/World.stories.tsx:167`, `block/3d/World.stories.tsx:79`,
  `block/5d/World.stories.tsx:75` reduce `worldVortexSelect` / `worldVortexHover` in story-local
  mocks; none of those fixtures set `domainId`, so they still exercise the legacy branch and stay green.

Only **three** scenes in the repo bind a domain today —
`✏️s/🔌️plugins/🧩️puzzle/…/🪟️windows/🧊️main/🦀️.rs:516` (`"vortex"`), gis-terrain (`"features"`/`"pin"`,
no marker layers), procedural `🧊️generation3d` (no marker layers). So this change is observable only
on puzzle3d, which is the intent.

Also worth recording: **puzzle3d handles neither `setSelection` nor `setReferenceSelection` nor
`referenceHover`** (grep over the whole `🧊️3d` artifact returns nothing). Target-volume and reference
picks in puzzle3d were therefore just as dead as the vortex pick, and this wave fixes all three at once.

## 5. `[DEBUG]` sweep

### Removed from `World3dHost/🟦️.tsx` (6 traces + 1 dead ref)

| was at | trace |
| --- | --- |
| `:1133` | `` console.log(`[DEBUG] hoverSuggestion`, …) `` inside `mapContextMenuSpecs`' `onHover` — the whole conditional collapsed back to `spec.hoverAction ? () => dispatch(…) : undefined` |
| `:3926` | `"[DEBUG] world3d viewport reattached to scene camera"` (effect body; `node.surfaceId` dropped from its deps, now unused there) |
| `:4084` | `` `[DEBUG] brushPreview` `` — an entire `useEffect` that existed only to log |
| `:4112` | `"[DEBUG] world3d viewport detached from shared scene camera"` — the `setViewportOwned((owned) => …)` updater existed only to read the previous value for this log; now `setViewportOwned(true)`, and `node.surfaceId` left `adoptViewportCamera`'s deps |
| `:4717` | `"[DEBUG] gumball drag begin"` (named by the brief) |
| `:4732` | `"[DEBUG] gumball drag end"` (named by the brief) |
| `:4074` | `gumballDragDebugTickRef` — a `useRef(0)` whose only two readers were the two gumball logs |

### `[DEBUG]`-prefixed but permanent — prefix dropped, log kept

Both are fault diagnostics on real failure paths whose own docstrings describe them as the
anti-silent-failure mechanism, so the `[DEBUG]` marker (which by `CLAUDE.md` means "temporary, remove
later") was simply wrong on them:

* `🗣️Interpreter/🟦️.tsx:1128` — `UnknownComponentView`'s `console.error`; its docstring is *"never
  renders nothing (a silent blank is the failure mode that makes a missing renderer look like a broken
  document)"*. `🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx:38` asserts the spy was **called**, not
  its text, so the rename is test-safe (verified by running the suite, [§6](#6-tests-run)).
* `🛠️ShellHelpers/🟦️.tsx:1449` — the program-load timeout/failure `console.error`.

### `🏛️ShellHost/🟦️.tsx` — ~80 records, intentionally left alone

Two independent reasons, both blocking:

1. **Peer-hot file.** mtime `01:48` against a wave start of `01:41`.
2. **They are not "clearly temporary".** Every one is either a `console.error`/`console.warn` on a
   genuine fault path (`"[DEBUG] boot fault text"` `:2934`, `"[DEBUG] render failed"` `:4101`,
   `"[DEBUG] shell uri apply failed"` `:4687`, `"[DEBUG] action failed"` `:5223`, the tutorial
   retirement family, …) or a lifecycle trace the fleet is *currently* using to debug boot for this
   very ticket (`"[DEBUG] hot-swap …"` `:2984`, `"[DEBUG] loadDocument pack/spr for instance"` `:4288`,
   `"[DEBUG] recovery diagnostics"` `:4933`). `:8723` already documents the distinction in prose:
   *"permanent, not a `[DEBUG]` trace"*.

**Recommendation for the coordinator:** a single dedicated pass over `🏛️ShellHost/🟦️.tsx` that drops
the `[DEBUG] ` prefix from the fault diagnostics and deletes the pure success-path `console.log`s
(`:1547` per extension invocation, `:3127`, `:3339`, `:4536`, `:4662`, `:5524`, `:5641`). It wants to
be one atomic edit at a moment the file is not being written, not a side effect of this wave.

## 6. Tests run

Two cases added to `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`, next to the existing
`"encodes world3d interaction dispatch args the same way the node graph does"`:

* `"resolves world3d marker interaction targets from the scene record, not from host knowledge"` —
  pins the four layer defaults, the record override (`{interactionGranularityId:"pin",interactionId:"pin-7"}`),
  and that an empty record still resolves to the layer default.
* `"encodes a vortex marker pick and hover as generic domain interaction args"` — pins the exact wire
  shapes in [§3.1](#31-domain-bound-scene-puzzle3d-domainid--vortex-domaingranularityid--object),
  including the empty-targets hover clear.

All commands run from the repo root, foreground.

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/🎯️targets/⚛️react/vitest.config.ts "🔬️engine-contract"
 Test Files  1 passed (1)
      Tests  445 passed (445)
   Duration  28.36s
```

443 before this wave, 445 after — both new cases run and pass.

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/🎯️targets/⚛️react/vitest.config.ts "Interpreter"
 Test Files  1 passed (1)
      Tests  65 passed (65)
```

(the in-source Interpreter suite, which owns the unknown-component-placeholder case whose
`console.error` this wave renamed)

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/🎯️targets/⚛️react/vitest.config.ts
 FAIL  ../../../../🧪️tests/🧩️package-integration/🟦️.ts [ collection error ]
 Test Files  1 failed | 17 passed (18)
      Tests  685 passed (685)
```

The single failure is a **collection** failure, not an assertion: `ReferenceError: self is not defined`
raised at import time by
`🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🔌️plugin-bridge.ts:159` (a worker-global
reference evaluated under `jsdom`). Pre-existing and untouched by this wave — nothing in it imports
anything this wave changed, and zero tests ran, so zero regressed. Every one of the 685 tests that
did run passed.

Also observed at `SEMIO_TEST_LEVEL=exhaustive` (which adds the `📃️UiDocumentStore` in-source suite):
**16 failures in `🧱️elements/📃️UiDocumentStore/🟦️.tsx`** (TypedWire / OwnedWire / retained-patch
oracles, e.g. `TypeError: Cannot read properties of undefined (reading 'ok')`). That file is a peer's
area, is not imported by anything this wave edited, and the failures reproduce when its suite is run
alone (`16 failed | 195 passed`). Reported, not touched.

### `tsc --noEmit`

```
node_modules/.bin/tsc --noEmit --project 🧰️framework/…/🎯️targets/⚛️react/tsconfig.json
→ exit 2, 1084 `error TS…` lines, 16s
```

It completes well inside the five-minute budget. The count looks alarming and is entirely
**pre-existing and elsewhere**: the project has only 24 root files (`tsc --showConfig` confirms), but
its transitive import graph reaches `📜️script.ts`, `♻️mit-bestand/`, plugin test files and so on, none
of which are typed for Bun. The histogram is dominated by `TS7006` implicit-any (378), `TS2304`
cannot-find-name (148, mostly `Bun`), `TS2345` (130) and `TS2749` (81).

Filtered to the files this wave edited:

| file | errors |
| --- | --- |
| `🧱️elements/🌐️World3dHost/🟦️.tsx` | **0** |
| `📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` (the barrel) | **0** |
| `🧱️elements/🛠️ShellHelpers/🟦️.tsx` | **0** |
| `🧱️elements/🏛️ShellHost/🟦️.tsx` | **0** |
| `🧪️tests/🔬️engine-contract/🟦️.ts` | 1, pre-existing, at `:3377` (a `PluginWasmHandle` mock map) — hundreds of lines above this wave's insertions, and the suite passes at runtime |

`🗣️Interpreter/🟦️.tsx:1225` reports one `TS2339 Property 'dir' does not exist on type 'ImportMeta'` —
a pre-existing `import.meta.dir` under a config without Bun types, on a line this wave did not touch
(its edit is at `:1128`).

## 7. Not verified

* **No browser run.** No `preview_start`, no live puzzle3d boot, no console capture. Everything in
  [§3](#3-exact-dispatch-shapes-now-sent) is proven only at the encoder level (unit tests) and by
  reading the call sites — *not* by observing a real click land on a real reducer.
* **The handler half is another wave's.** That a `interactionSelect` carrying
  `{"granularity":"vortex","id":"…"}` actually mutates puzzle3d selection state is asserted by wave S
  for the document/catalogue-tree path (`puzzle3d_interaction_select`), and the marker path now emits
  the identical shape onto the identical verb — but the marker path itself has no Rust-side test.
* **`🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts` never ran.** It appears in **no** `include` list of
  `…/🎯️targets/⚛️react/vitest.config.ts` and there is no `component.feature` beside it, so the test
  domain's discovery (`**/🧪️tests/*/component.feature`) cannot see it either — vitest exposes no
  `--include` CLI flag to force it. This looks like the taxonomy/discovery drift class of bug rather
  than anything this wave caused; the file tests `📤️SegmentedDownload`, which this wave did not touch.
  **Flagged for the coordinator: this test is currently unreachable by any runner.**
* **`🏛️ShellHost/🟦️.tsx`'s `[DEBUG]` records are still there** — deliberately, see [§5](#5-debug-sweep).
* **Attraction picks are still unreachable**, because `WorldAttractionLines` exposes no pointer
  callbacks. `WORLD3D_DEFAULT_MARKER_GRANULARITY.attraction` is declared and unit-tested, so wiring a
  pick later is a props change with no interaction-design decision left in it. Attraction selection
  from the document tree is unaffected and continues to work.
* **`interactionId` / `interactionGranularityId` are not emitted by any plugin yet.** No plugin needs
  them today (puzzle3d's granularity ids equal the layer names), so this is an available override,
  not a pending dependency. If a future app's marker granularity names diverge, the fix is two extra
  JSON fields in that plugin's marker builder — **no** typed-scene contract change.
