# W5 — Storybook stories for the generated `block` scope

## 1. What the scope needed

`.storybook/scopes.ts` has **zero** literal `block` rows. The scope is generated from the plugin's own
manifest opt-in (`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml`,
`[package.metadata.semio.storybook]` → `id = "block"`, `titlePrefix = "🧱️block"`, `sourceRoots = ["."]`).

Discovery was verified live, not assumed — `bun` against `.storybook/scopes.ts`:

```
$ bun -e 'const m = await import("./.storybook/scopes.ts");
          const active = m.resolveActiveScopes("block");
          console.log("active:", JSON.stringify(active, null, 2));
          console.log("globs:", JSON.stringify(m.buildScopeStoryGlobs(active)));'
active: [
  {
    "id": "block",
    "titlePrefix": "🧱️block",
    "sourceRoots": [
      "✏️s/🔌️plugins/🧱️block"
    ]
  }
]
globs: ["./stories/block/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"]
```

`Cargo.toml` declares no `storyGlobs`, so `buildScopeStoryGlobs` falls back to the default
`./stories/<id>/**` derivation. **Story files therefore must live under `.storybook/stories/block/`** —
that is where they were added. No manifest edit was needed.

## 2. Files added

| Path | Role |
|---|---|
| `.storybook/stories/block/dsl.ts` | Story-local reader for the three block DSL dialects (`block.block2d.dsl` / `block.block3d.dsl` / `block.block5d.dsl`) — banner, `key=value` scalars, `name { … }` blocks, `name [col:TYPE …] { rows }` tables, with `_`/`@x,y,z`/`^x,y,z`/`<n>rad`/`[ … ]` token handling. Reader only, never an emitter. |
| `.storybook/stories/block/scene.ts` | Shared fixtures, `UiComponentSceneNode` projections, window-render text and story-local command emulators for all three apps. |
| `.storybook/stories/block/2d/Board.stories.tsx` | `Board2dHost` for `block2d-board` — 3 stories. |
| `.storybook/stories/block/2d/Fixtures.stories.tsx` | `Board2dHost` fixture coverage over every shipped block2d example — 3 stories. |
| `.storybook/stories/block/3d/World.stories.tsx` | `World3dHost` for `block3d-world` — 2 stories. |
| `.storybook/stories/block/5d/Board.stories.tsx` | `Board2dHost` for `block5d-board` — 3 stories. |
| `.storybook/stories/block/5d/World.stories.tsx` | `World3dHost` for `block5d-world` — 2 stories. |

`dsl.ts`/`scene.ts` are **not** `*.stories.*`, so the scope glob does not index them. That separation is
deliberate and load-bearing: Storybook's CSF indexer treats every named export of a `*.stories.*` module
as a story, so a shared helper exported from a story file would be indexed as a broken story. Puzzle's
own stories avoid the problem by duplicating their reducers per file; block shares one module instead
(CLAUDE.md: "if code is repeated, it MUST be close to each other" — same folder).

### Story titles (all under the scope's `titlePrefix`)

- `🧱️block◻️2d` — `HexagonalCutConcreteForestLeft`, `HexagonalCutConcreteForestRight`, `ReadOnlyViewerBoard`
- `🧱️block◻️2d/Fixtures` — `AllExamples`, `HexagonalCutConcreteForestLeft`, `HexagonalCutConcreteForestRight`
- `🧱️block🧊️3d` — `HexagonalCutConcreteForestLeft`, `NakaginCapsule`
- `🧱️block🖐️5d/Board` — `HexagonalCutConcreteForestLeft`, `NakaginCapsule`, `ReadOnlyBoard`
- `🧱️block🖐️5d/World` — `HexagonalCutConcreteForestLeft`, `NakaginCapsule`

## 3. Fixture provenance — the real example DSL, not hand-authored data

Every fixture is `?raw`-imported from the shipped asset and parsed by `dsl.ts`. The real asset path is one
level deeper than a plain `📚️examples/<name>/🖼️assets/🗣️.dsl.semio`:

```
✏️s/🔌️plugins/🧱️block/🗿️artifacts/<dim>/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/<unit>/🖼️assets/🧪️<unit>/🗣️.dsl.semio
```

Six documents are covered: block2d `🎬️hexagonal-cut-concrete-forest-left` + `➡️hexagonal-cut-concrete-forest-right`,
block3d and block5d `🎬️hexagonal-cut-concrete-forest-left` + `🏢️nakagin-capsule`.

The parser was validated against all six in `bun` before any story was wired. Real output (abridged):

```
=== 2d left ===  nodeKind "Hexagonal Cut Concrete Forest Left", camera2d {x:230.7,y:93.5,zoom:2},
                 6 handleKinds (b-l/b-l-m/b-s/b-s-m/c-b/c-t, each with its hsl(...) colour),
                 11 handles h0..h10 with rad angles (h0 = -1.5707963267948966) and radius 0.36
=== 2d right === same 6 kinds; differs only in the h4/h5/h6 kind assignment
=== 3d left  === objectKind "…Left", camera3d {position:[4,-4,3],target:[0,0,0],zoom:1},
                 1 representation r0 "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb",
                 6 vortexKinds, 11 vortices with position/direction/radius
=== 3d nakagin == objectKind "Capsule J", 2 representations
                 (r0 "/mesh/🧊️capsule_J.glb", r1 "/mesh/capsule_J.1to500.glb"), 1 `door` vortex
=== 5d left  === partKind "…Left", part2d {shape:"circle",radius:20},
                 camera2d {230.7,93.5,2}, camera3d {[30,-30,20],[7,0,3],3},
                 1 representation, 1 gripKind b-l, 1 grip g0 (angle -0.1, radius2d 3, @4.05,4.68,3)
=== 5d nakagin == partKind "Capsule J", 1 representation, 1 gripKind door, 1 grip g0 at -π/2
```

## 4. Hosts mounted, and what each story actually shows

### `Board2dHost` (2d Board, 2d Fixtures, 5d Board)

Mounted directly against a `UiComponentSceneNode` with `componentKind: "board-2d"` — the same contract
`.storybook/stories/puzzle/2d/Board.stories.tsx` uses. The document projects into the fixture as one node
(the node/part kind) carrying one board handle per `Block2dHandleTemplate` / `Block5dGrip` at its own
angle, with the document-shaped kind catalogs translated to the engine-shaped `nodeKinds`/`handleKinds`
(the same narrowing puzzle's `storyBoardKindCatalogsJson` performs).

**Honest limitation, verified in source, not assumed:** `Board2dHost` paints through a board-2d **wasm
session** obtained from `BoardSessionFactoryContext`, and `.storybook/preview.tsx` registers **no**
board-2d entry in its `WASM_LOADERS` map (`node-graph`, `editor`, `paint-2d`, `tiled-map`, `terrain`,
`flow` only). With no factory in context, `sessionRef` stays null and the `<canvas>` renders empty — this
is equally true of the existing puzzle board stories. So each board story also renders the window's own
Rust output as text beside the canvas (`data-testid="block2d-board-window"` /
`"block5d-board-window"`): the exact `ui_text` lines from
`◻️2d/…/👁️viewer/🎭️modes/👁️view/🪟️windows/📋️board/🦀️.rs`'s `render` — node-kind label, every handle kind as
`  ◦ <label> (<id>) — <color>` with a colour swatch, every handle as
`  ◦ <id> — kind <k>, angle <deg>°, radius <r>`. That panel is the assertable, always-visible half.

### `World3dHost` (3d World, 5d World)

Mounted against `componentKind: "world-3d"`, projected the way
`🧊️3d/…/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs` builds it: one mesh + one instance per
representation at the document origin with identity rotation, plus every rim vortex / grip at its document
`position`/`direction`/`radius` coloured from its `vortex-kind-extra` / `grip-kinds` row. `World3dHost` is
plain three.js/r3f — no plugin wasm — so this half renders for real.

### Mesh resolution (the requested catalog path)

`World3dHost` loads every mesh through `meshAssetTransportUrl(url)` →
`resolveMeshAsset(url, MESH_DELIVERY_CATALOG)` (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🟦️.ts`, backed by
`🥽️mesh/📇️catalog.json` plus the `🌱️metabolism/🎨️representation` collection it composes). Verified live:

```
$ bun -e '…resolveMeshAsset / meshAssetTransportUrl…'
OK   /mesh/🧊️hexagonal-cut-concrete-forest-left.glb
     -> ♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/◀️hexagonal-cut-concrete-forest-left.glb
     | transport /mesh/🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.glb | exists true
OK   /mesh/🧊️capsule_J.glb
     -> 🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb
     | transport /mesh/🌱️metabolism/💊️capsules/🪝️j/🧊️capsule_J.glb | exists true
MISS /mesh/capsule_J.1to500.glb -> Unknown mesh asset: /mesh/capsule_J.1to500.glb
```

Those transport urls are served by the `mesh-collection` playground asset plugin
(`meshCollectionVitePlugin`, route `/mesh`), which `.storybook/main.ts` only installs for
`activeScopes.flatMap(s => s.assets)` — and a **generated** scope cannot declare `assets`
(`buildGeneratedScopes` emits `id`/`titlePrefix`/`sourceRoots`/`storyGlobs` only). So the stories resolve
the document's public identity through `resolveMeshAsset` and then hand `World3dHost` the **Vite-emitted
url** of the exact GLB the catalog names (`?url` import in `scene.ts`, keyed by catalog `source` so a
catalog rename breaks loudly at module load). `meshAssetTransportUrl` passes a non-`/mesh/` url straight
through, so the host loads the real mesh instead of 404ing — this is the same override discipline
`stories/puzzle/3d/World.stories.tsx` uses for its reference-plane images, except block's meshes actually
exist so they are real rather than placeholder boxes.

**Dropped representation (as flagged in the task):** the nakagin `Capsule J` document's second
representation, `r1 "1:500"`, names `/mesh/capsule_J.1to500.glb` — an identity **no** mesh catalog in the
repo declares (only `🔭️capsule_J_1to500.3dm`, an unconverted Rhino file, exists on disk at
`💊️capsules/🪝️j/`). It is dropped from the scene rather than rendered mesh-less, and the drop is reported
in the story's own debug readout under `droppedRepresentations` with the reason. block5d's nakagin
document ships only the `Full Detail` representation, so nothing is dropped there.

## 5. Story-local reducers (no dev server, no plugin wasm)

- **block2d** (`reduceBlock2dStoryAction`) — `setActiveExample` (both examples), `patchNodeKind`,
  `addHandleKind`, `removeHandleKind`, `addHandle`, `removeHandle`. Mirrors `command_from_action` →
  `Block2dCommand::dispatch` (`◻️2d/…/✏️editor/🦀️.rs`); an unknown action is ignored exactly as the real
  `command_from_action` returns `None`. Every action is reachable from a `data-testid`'d toolbar button
  that dispatches a real `ActionDescriptor` through the same `onAction` the host uses.
- **block5d** (`reduceBlock5dStoryAction`) — `setActiveExample`, `patchPartKind`, `addGripKind`,
  `removeGripKind`, `addGrip`, `removeGrip`.
- **3d/5d world** — the `worldPick`/`worldSelect`/`worldVortexSelect`/`setHover`/`worldVortexHover`
  subset, with `applyStoryWorldMerge` copied from puzzle's 3d story. One correction over the puzzle copy:
  `World3dHost` dispatches `worldPick` with `id` as the **index into `instancesJson`** (or `null`), never
  an instance id — read off `🌐️World3dHost/🟦️.tsx:4385`, so the story resolves it against the same
  admitted-representation order the scene was built from.

## 6. Verification

### 6.1 Scope resolution — PASS

The `bun` run in §1. Confirms both the scope id/prefix and the story glob the files satisfy.

### 6.2 Parser against all six real documents — PASS

The `bun` run in §3. All six parse; no throw, no empty table.

### 6.3 Mesh catalog resolution — PASS (with one expected MISS)

The `bun` run in §4.

### 6.4 TypeScript — PASS for every added file

Root `tsconfig.json` has no `types` entry, so `?raw`/`?url` imports need `vite/client`; a ticket-local
config supplies it and scopes the program to the new files
(`tsconfig.w5-block-stories.json`, kept in this ticket folder as an input file):

```
$ bun x tsc --noEmit -p .🧬semio/…/BLOCK-PLUGIN-END-TO-END/tsconfig.w5-block-stories.json
EXIT=2
--- errors in .storybook/stories/block/** ---
(none)
--- total errors in the program ---
1172
```

**Zero errors in any added file.** The 1172 remaining errors are all in transitively-imported framework
modules and are pre-existing, not introduced here — top offenders:
`📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx` (296),
`🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` (107),
`🎭️actor/📮️shard-client/🟦️.ts` (55), `…/⚛️react/🟦️.tsx` (31), `🏛️ShellHost/🟦️.tsx` (30).

### 6.5 Storybook build for the `block` scope

`📜️script.ts`'s `build storybook` slice (line ~19842) shells `bunx storybook build -c .storybook
--output-dir storybook-static` and takes the scope from `STORYBOOK_SCOPE` (there is no scope argument;
`assertUiStorybookDiscovery` is gated on `STORYBOOK_SCOPE === "ui"`). The equivalent scoped invocation was
run in the foreground, with the output directed outside the repo so a peer's `storybook-static` is never
clobbered:

```
STORYBOOK_SCOPE=block bunx storybook build -c .storybook --output-dir <scratchpad>/storybook-static-block
```

**Blocker found and fixed first (pre-existing, repo-wide, not caused by W5).** The first run failed before
any story was touched:

```
SB_CORE-SERVER_0007 (MainFileEvaluationError): Storybook couldn't evaluate your .storybook/main.ts file.
TypeError [ERR_IMPORT_ATTRIBUTE_MISSING]: Module ".../🥽️mesh/📇️catalog.json"
needs an import attribute of "type: json"
```

Confirmed scope-independent by reproducing it verbatim with `STORYBOOK_SCOPE=puzzle/3d`, i.e. Storybook's
config load has been broken for **every** scope. Cause: `main.ts` imports
`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts`, which reaches
`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🟦️.ts` → `🔍️resolver/🌐️delivery.ts`; three of their JSON imports
lacked the `with { type: "json" }` attribute Node's ESM loader requires. `with { type: "json" }` is already
the established convention everywhere else in this repo (`🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️.ts`
and every `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/🟦️.ts`), so the three outliers were brought in line:

- `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🟦️.ts` (2 imports)
- `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🌐️delivery.ts` (1)
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts` (1, `./🌐️favicon.json`)

After that, `main.ts` evaluates and the build proceeds into bundling. **Build outcome: see §7 —
the build was still running when this report was written; the scoped build on this repo takes well over
25 minutes.**

## 7. Unverified / open

1. **Storybook build completion for the `block` scope.** The config-load blocker is fixed and the build
   gets past `main.ts` into bundling, but a full green build (and therefore the generated
   `index.json` story-id list) had not completed at the time of writing. It must be re-checked:
   `STORYBOOK_SCOPE=block bunx storybook build -c .storybook --output-dir <dir>`, then
   `jq '.entries | keys' <dir>/index.json` for the 13 story ids.
2. **No story has been seen rendering.** No screenshot, no `iframe.html` load. Everything above is
   source-verified (host contracts, `worldPick` payload shape, mesh catalog, preview's wasm registry) or
   `bun`-verified (scope, parser, mesh resolution) — nothing here claims a rendered pixel.
3. **No Playwright spec added.** The existing `.storybook/*.spec.ts` files need a running Storybook, which
   the box could not spare while the build was occupying it. The stories were written with that spec in
   mind: `data-testid` hooks are `block2d-board-window`, `block2d-board-debug`,
   `block2d-fixture-window-<exampleId>`, `block2d-fixtures-debug`, `block3d-world-debug`,
   `block5d-board-window`, `block5d-board-debug`, `block5d-world-window`, `block5d-world-debug`, plus one
   per toolbar button (`block2d-add-handle`, `block5d-add-grip`, …), matching the
   `framework-hosts-no-wasm.spec.ts` `readDebug`/`expectStoryLoads` pattern exactly.
4. **The board canvas is blank by design** until someone registers a board-2d session factory for
   Storybook (same gap as the puzzle board stories). Closing it would mean a
   `BoardSessionFactoryContext` provider in `preview.tsx` plus a board-2d wasm entry in `WASM_LOADERS`.
5. **Pre-existing stale imports in the puzzle stories** were noticed but deliberately left alone (out of
   W5 scope): `stories/puzzle/2d/Board.stories.tsx`, `2d/Fixtures.stories.tsx` and `5d/Timeline.stories.tsx`
   import `../../../../framework/product/os/module/renderer/js/react/index.tsx`, a path that **no longer
   exists** (the renderer moved to `🧰️framework/🛍️products/💻️os/…/⚛️react/🟦️.tsx`, which only
   `3d/World.stories.tsx` was updated to). `2d/Fixtures.stories.tsx` and `3d/World.stories.tsx` also
   point at `🗿️artifacts/{◻️2d,🧊️3d}/📚️examples/…` and `.🦑️repo/🎫️tickets/…`, neither of which exists
   any more (`🏅️standards/🔖️1/🪆️subsets/✳️any/` was inserted, and the ticket root moved under
   `.🧬semio/`). Those three puzzle stories cannot resolve today; the `framework/hosts` `UiInterpreter`
   story is stale too (it calls `interpretUiNode(node, { onAction })` while the current signature is
   `interpretUiNode(store: UiDocumentStore, context)`). Worth its own ticket.
