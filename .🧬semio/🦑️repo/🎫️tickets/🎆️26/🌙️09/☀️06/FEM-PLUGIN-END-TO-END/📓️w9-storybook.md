# W9 — Storybook stories for the generated `fem` scope

Ticket `26/09/06/FEM-PLUGIN-END-TO-END`. Precedent: `26/09/05/BLOCK-PLUGIN-END-TO-END/📓️w5-storybook.md`
and the raster ticket's `Paint2dHost.stories.tsx` component-level host fixture.

Before this work, `fem`'s ONLY Storybook coverage was the generic whole-shell matrix entry
`.storybook/stories/framework/os/plugins.stories.tsx:37` (`export const Fem: Story = { args: { plugin:
"fem" } }`), which every registered plugin crate gets automatically. There was no `fem` scope and no
`.storybook/stories/fem/` directory.

---

## 1. `surfaceKind` mapping — the question the exploration left open

`📓️explore-dev-boot-ts-descriptor.md` §5 flagged that nobody had yet established which
`ComponentSceneHost` fem's window kinds resolve to. Settled here by reading each window's own
`render`/`render_with_progress` and following the `crate::app_surface` encode helper it calls:

| app | window kind id | body key | `crate::app_surface` call | `SurfaceKind` | `componentKind` | host |
|---|---|---|---|---|---|---|
| fem2d editor | `fem2d-model` | `fem2d.play.model` | `canvas_2d_surface` | `Canvas2d` | `canvas-2d` | `📐️Canvas2dHost` |
| fem2d editor | `fem2d-results` | `fem2d.play.results` | `canvas_2d_surface` | `Canvas2d` | `canvas-2d` | `📐️Canvas2dHost` |
| fem2d viewer | `fem2d-view-model` | `fem2d.view.model` | `canvas_2d_surface` | `Canvas2d` | `canvas-2d` | `📐️Canvas2dHost` |
| fem3d editor | `fem3d-model` | `fem3d.play.model` | `world_3d_surface` | `World3d` | `world-3d` | `🌐️World3dHost` |
| fem3d editor | `fem3d-results` | `fem3d.play.results` | `world_3d_surface` | `World3d` | `world-3d` | `🌐️World3dHost` |
| fem3d viewer | `fem3d-view-model` | `fem3d.view.model` | `world_3d_surface` | `World3d` | `world-3d` | `🌐️World3dHost` |

Six windows total. Neither viewer has a results window (confirmed in
`📓️explore-fem2d-editor.md` §1 / `📓️explore-fem3d-editor.md` §1 and re-checked against the
`#[path]` mounts in `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs`), and neither app declares an
inspector/panel window kind — so the story matrix is 4 apps × their real windows = 6, not 8.

Encode sites (file:line):
- `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs:24` `canvas_2d_surface` → `SurfaceKind::Canvas2d`
- `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs:35` `world_3d_surface` → `SurfaceKind::World3d`

The shell's dispatch from `componentKind` to host component is the same one
`.storybook/stories/framework/hosts/Canvas2dHost.stories.tsx` / `World3dHost.stories.tsx` mount directly.

## 2. Loader status — fem needs NO `WASM_LOADERS` entry (unlike block)

`.storybook/preview.tsx:373-380` registers exactly six loaders: `node-graph`, `editor`, `paint-2d`,
`tiled-map`, `terrain`, `flow`. Neither `canvas-2d` nor `world-3d` is among them — but, unlike block's
`board-2d`, **neither one needs to be**:

- **`Canvas2dHost` owns no wasm engine.** `🧱️elements/📐️Canvas2dHost/🟦️.tsx:654-656` builds its own
  `JsonLayersCanvasSession` (`:385`), a pure `CanvasRenderingContext2D` implementation, unconditionally —
  there is no `SessionFactoryContext` lookup and no null-session path. Its `renderFrame` (`:462-540`)
  handles exactly the four layer shapes fem2d emits: `kind:"circle"` with bounds (`drawBoundsLayer`),
  `kind:"line"` with `x0/y0/x1/y1`, `kind:"polyline"` with a flat `points` pair list, and
  `segments`+`fill`/`text` via `drawSceneNode`. **So every fem2d story paints real pixels with no plugin
  wasm and no dev server.**
- **`World3dHost` owns no wasm engine either**, except when `scene.terrainJson` is present (it then mounts
  `🗺️WorldTerrainLayer`, the only reason `World3dHost.stories.tsx`'s `TerrainViewport` sets
  `parameters.wasm: ["terrain"]`). fem3d's scenes carry no `terrainJson`. `parseMeshes`
  (`🌐️World3dHost/🟦️.tsx:1039`) accepts `{ id, data: { positions, normals, indices } }` entries, which is
  exactly the shape `world3d_meshes_json_from_kinds(&["box"])` emits.

**Consequence: block's W5 blank-canvas caveat does NOT apply to fem.** No story here sets
`parameters.wasm`, so `withWasm` (`preview.tsx:407`) passes them straight through.

### 2.1 What IS unavailable, and how it is handled (block's W5 fallback discipline)

Four halves of the Rust renders are solver/mesher output with no browser build. Each is **omitted, never
faked**, and reported as a counted entry in the story's own `data-testid`'d debug panel
(`femStoryOmissions`, `.storybook/stories/fem/scene.ts`):

| omission | source | affected stories |
|---|---|---|
| `mesh-edge-*` overlay | `fem2d_region_triangles` → `crate::fem2d_engine::mesh_preview::fem2d_mesh_preview` | every fem2d story |
| `deformed-*` / `reaction-*` / moment / von-Mises contour layers | `crate::fem2d_engine::fem2d_solve_all` | `🏗️fem◻️2d/Results` |
| `solid-*` boundary meshes | `fem3d_solid_mesh_entries` → `fem3d_mesh_preview` | every fem3d story |
| displacement offsets + von-Mises vertex colors | `crate::fem3d_engine::fem3d_solve_all` | `🏗️fem🧊️3d/Results` |

Two guard branches ARE reproduced faithfully rather than omitted: `render_static`'s
`built_text_node(Label::data("No load case defined"))` path returns a bare TEXT node, not a surface, so the
`NoLoadCase` stories mount no host at all and render that text — matching the node shape the shell
would receive.

## 3. Files created

| Path | Role |
|---|---|
| `.storybook/stories/fem/dsl.ts` | Story-local reader for `fem.fem2d.dsl` / `fem.fem3d.dsl` + typed `Fem2dSnapshot`/`Fem3dSnapshot` projections. Reader only, never an emitter. |
| `.storybook/stories/fem/scene.ts` | `?raw` fixtures, en/de labels, `canvas-2d`/`world-3d` scene projections (ports of the Rust builders), story-local command emulators, window summaries, omission registry. |
| `.storybook/stories/fem/2d/Model.stories.tsx` | `Canvas2dHost` for `fem2d-model` — 3 stories. |
| `.storybook/stories/fem/2d/Results.stories.tsx` | `Canvas2dHost` for `fem2d-results` — 3 stories. |
| `.storybook/stories/fem/2d/Viewer.stories.tsx` | `Canvas2dHost` for `fem2d-view-model` — 3 stories. |
| `.storybook/stories/fem/3d/Model.stories.tsx` | `World3dHost` for `fem3d-model` — 3 stories. |
| `.storybook/stories/fem/3d/Results.stories.tsx` | `World3dHost` for `fem3d-results` — 3 stories. |
| `.storybook/stories/fem/3d/Viewer.stories.tsx` | `World3dHost` for `fem3d-view-model` — 3 stories. |
| `.🧬semio/…/FEM-PLUGIN-END-TO-END/🔍️w9-storybook-probe.ts` | The verification probe of §6.2–6.4 (kept as a ticket input file). |

Files modified: `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml` — append-only, §4.

`dsl.ts`/`scene.ts` are **not** `*.stories.*`, so the scope glob does not index them — the same deliberate
split block uses (Storybook's CSF indexer treats every named export of a `*.stories.*` module as a story).

## 4. Opt-in — the ONLY registration fem needed

`.storybook/scopes.ts` has zero literal `fem` rows and needs none: the scope is generated from the
plugin's own manifest opt-in. Added to `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml`, immediately
after `[package.metadata.semio] role = "plugin"` and before the first `[[…playground]]` array-of-tables
entry (`tomlTableBody` reads a table body up to the next `[`, so the placement matters):

```toml
[package.metadata.semio.storybook]
id = "fem"
titlePrefix = "🏗️fem"
sourceRoots = ["."]
```

No `.storybook/main.ts`, `preview.tsx`, `playwright.config.ts` or `scopes.ts` edit is needed — grepping
those three for `block` returns zero hits, confirming the generated-scope mechanism is the whole story
index. No `storyGlobs` is declared, so `buildScopeStoryGlobs` falls back to the default
`./stories/fem/**` derivation, which is where the files live.

## 5. Story list — 18 stories over 6 titles

| Title | Stories | Window |
|---|---|---|
| `🏗️fem◻️2d/Model` | `DemoEnglish`, `DemoGerman`, `ClearedDocument` | `fem2d-model` |
| `🏗️fem◻️2d/Results` | `StaticEnglish`, `ModalGerman`, `NoLoadCase` | `fem2d-results` |
| `🏗️fem◻️2d/Viewer` | `DemoEnglish`, `DemoGerman`, `EmptyDocument` | `fem2d-view-model` |
| `🏗️fem🧊️3d/Model` | `DefaultExampleEnglish`, `DefaultExampleGerman`, `ShippedExampleId` | `fem3d-model` |
| `🏗️fem🧊️3d/Results` | `StaticEnglish`, `BucklingGerman`, `NoLoadCase` | `fem3d-results` |
| `🏗️fem🧊️3d/Viewer` | `DefaultExampleEnglish`, `DefaultExampleGerman`, `EmptyDocument` | `fem3d-view-model` |

Ids are unique: the six titles are distinct (`fem2d-model`, `fem2d-results`, `fem2d-viewer`,
`fem3d-model`, `fem3d-results`, `fem3d-viewer` after Storybook's emoji-stripping slugification, the same
one block's `🧱️block◻️2d` / `🧱️block🧊️3d` titles already survive), and export names are unique per file.
`tags: ["autodocs"]` adds one docs entry per title, so the scoped `index.json` should carry **18 stories +
6 docs = 24 entries**.

### 5.1 Example switch and dispatched actions

Both editor Model stories carry a `data-testid`'d toolbar dispatching real `ActionDescriptor`s
(`{ controllerId, action, args }`) through the same `onAction` the host uses, folded by a story-local
reducer mirroring the app's `bounded_first_step_tool_proofs!` roster:

- fem2d (`reduceFem2dStoryAction`): `setActiveExample`, `addNode`, `setCamera`, `setResultDisplay`,
  `setLocale`. Testids `fem2d-load-example`, `fem2d-clear-example`, `fem2d-add-node`,
  `fem2d-set-locale-en-US`, `fem2d-set-locale-de-DE`, `fem2d-result-mode-{static,modal,buckling}`,
  `fem2d-result-case-{dead,live}`.
- fem3d (`reduceFem3dStoryAction`): `setActiveExample`, `addNode`, `setCamera`, `setResultDisplay`.
  Testids `fem3d-load-default`, `fem3d-load-shipped-id`, `fem3d-add-node`,
  `fem3d-result-mode-{static,modal,buckling}`, `fem3d-result-case-{dead,live}`.
- Panels: `fem2d-model-window` / `-debug`, `fem2d-results-caption` / `-window` / `-debug` /
  `-placeholder`, `fem2d-viewer-window` / `-debug`, and the `fem3d-…` equivalents; canvas/world wrappers
  are `fem2d-model-canvas`, `fem2d-results-canvas`, `fem2d-viewer-canvas`, `fem3d-model-world`,
  `fem3d-results-world`, `fem3d-viewer-world`.

`addNode` mints its id through a port of `crate::app_surface::next_id`, so on the 12-node fem2d fixture it
produces `n12` — verified in §6.3.

### 5.2 en + de labels

Every story sets `locale` explicitly (`"en-US"` or `"de-DE"`) — there is no default language, per
CLAUDE.md. `femStoryLabel` resolves 21 keys in both tags; `meta.argTypes.locale` exposes an inline-radio
so a reader can flip either way. fem2d's switch dispatches the app's **real** `setLocale` tool
(`✏️editor/🎮️commands/🗣️set-locale/🦀️.rs`, writing `Fem2dConfig::locale`); fem3d's roster has **no**
`setLocale`, so its 3D stories carry the locale as a story argument only and say so in their header.
The preview's global `locale` toolbar (`en`/`de`, `preview.tsx:124`) drives react-i18next chrome
translation, a different axis from the plugin-owned `Fem2dConfig::locale` — deliberately not conflated.

## 6. Verification

No cargo / nx / storybook / vite / dev-server run (coordinator constraint). Everything below is either
source-verified or produced by the dependency-free `bun` probe in the ticket folder.

### 6.1 Import-path resolution — PASS

Scripted (python, `os.walk` + regex over every `from "…"` in the eight new files): all 22 specifiers
resolve to a real file on disk — the react barrel at
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`
(via block's own extensionless `…/⚛️react/🟦️` form), `./scene`, `./dsl`, and the two `?raw` example
assets. The only bare specifier is `@storybook/react-vite`, which every existing story imports.

A second scripted pass found **zero unused named imports** across all eight files.

### 6.2 Scope resolution — PASS

```
$ bun -e 'const m = await import("./.storybook/scopes.ts");
          const a = m.resolveActiveScopes("fem");
          console.log("active:", JSON.stringify(a));
          console.log("globs:", JSON.stringify(m.buildScopeStoryGlobs(a)));'
active: [{"id":"fem","titlePrefix":"🏗️fem","sourceRoots":["✏️s/🔌️plugins/🏗️fem"]}]
globs: ["./stories/fem/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"]
```

The manifest opt-in is discovered, the owner-relative `"."` resolves to the plugin root, the glob matches
where the stories live, and `STORY_SCOPES`' id-collision guard did not throw (so `fem` collides with no
hand-curated scope).

### 6.3 Reader + projections against both real documents — PASS

`bun .🧬semio/…/FEM-PLUGIN-END-TO-END/🔍️w9-storybook-probe.ts`, verbatim output:

```
2d parsed counts {"nodes":12,"elements":9,"supports":4,"regions":1,"loadCases":2}
2d quantity units stripped {"firstNodeY":-4,"steelE":210000000000,"steelRho":7850,"chs76Area":0.001}
2d nested loads:BLOCK ["dead:area/l5:self=true","live:nodal/l6+area/l7:self=false"]
2d layer ids [node-n1 … node-rc3, el-e3 … el-e11, support-s1 … support-s4, load-l5, load-l6, load-l7]
2d scene node {"componentKind":"canvas-2d","layerCount":28}
2d addNode {"id":"n12","x":4,"y":9}
2d next id n12
2d setActiveExample(unknown) → nodes 0
2d de labels Knoten: 12 | Stäbe: 9 | Auflager: 4 | Flächen: 1 | Materialien: 3 |
             Querschnitte: 4 | Lastfälle: dead, live | Kombinationen: uls | Verformungsmaßstab: 300
3d parsed counts {"nodes":16,"elements":16,"supports":8,"solids":1,"loadCases":2}
3d instance count 32
3d vertical member (identity quat)      {"id":"el-e1","meshId":"box","position":[0,0,1.4],
                                         "rotation":[0,0,0,1],"scale":[0.05,0.05,2.8],"label":"e1"}
3d horizontal member (+X, 90° about Y)  {"id":"el-fb1_0","meshId":"box","position":[4,0,2.8],
                                         "rotation":[0,0.7071067811865475,0,0.7071067811865476],
                                         "scale":[0.05,0.05,8],"label":"fb1_0"}
3d box mesh buffers [["box",108,108,36]]
3d setActiveExample(demo) → nodes 0
3d de labels Knoten: 16 | Stäbe: 16 | Auflager: 8 | Volumen: 1 | Materialien: 2 |
             Querschnitte: 1 | Lastfälle: dead, live | Kombinationen: uls | Verformungsmaßstab: 300
```

What that proves, item by item:
- Every table, statement block and nested cell of both shipped documents parses — including the
  multi-line `loads:BLOCK` column and fem3d's `terms:MAP` column, neither of which block's line-based
  reader handles (fem's rows are read from a brace-aware token stream chunked by column count).
- Quantity suffixes are stripped exactly where the Rust column type says they may be (`-4m` → `-4`,
  `210000000000Pa` → `2.1e11`, `7850kg/m3` → `7850`, `0.001m2` → `0.001`) and NOT on `TEXT` columns, so
  ids like `chs76` / `hea200` / `post140` stay strings.
- `28 = 12 nodes + 9 members + 4 supports + 3 loads` is exactly `fem2d_structure_layers`' output cardinality
  and id vocabulary. Spot-checked against `screen_2d`: `node-n1` (model `0,-4`) lands at
  `{x:36,y:116,width:8,height:8}` — i.e. `(0·20+40, 4·20+40)` minus the Rust's own 4px half-extent.
  `load-l6` (nodal `Ty`, value `-12000`, on node `p8_l1` at `8,2.8`) is
  `points:[[200,-16],[200,2]]` — the Rust's `origin.1 - vector[1]` with `vector = [0, sign(v)·18]`.
- `32 = 16 node boxes + 16 member prisms` is `fem3d_structural_instances`' cardinality. The two quaternion
  branches are both exercised: a +Z member yields `quat_z_to`'s identity fast path, a +X member yields the
  90°-about-Y shortest arc.
- The `"box"` mesh is `mesh_box(1,1,1)` + `compute_normals`: 12 triangles × 3 vertices × 3 floats = 108
  positions / 108 normals / 36 indices, non-indexed so normals come out flat per face.
- Both `setActiveExample` fallback branches empty the document, matching each handler's `else` arm.
- Both locales resolve for every summary key.

### 6.4 Host contract — source-verified, not run

`Canvas2dScene` = `{ cameraX, cameraY, zoom, layersJson }` and `World3dScene` =
`{ cameraJson, meshesJson, instancesJson, selectionJson, … }` read off
`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:10` / `:258`; `UiComponentSceneNode` (`:615`) and
`ActionDescriptor` = `{ controllerId, action, args? }`
(`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest.ts:63`). The camera/selection/environment JSON
literals are byte-for-byte the Rust helpers' output: `world3d_camera_json([4,-4,3],[0,0,0],45)` adds
`"up":[0,0,1]`, `world3d_selection_json("rectangle",&[],None)` is
`{method,mode:"replace",ids,hoveredId}`, `world3d_environment_json(&WorldSunConfig::default())` is
`{"sun":{"enabled":false,"azimuth":45,"elevation":35,"intensity":0.85,"color":"#ffffff"}}`.

**No TypeScript compiler was run** (host constraint) and **no story has been seen rendering** — no
screenshot, no `iframe.html` load. §6.1/§6.4 are read-verified; §6.2/§6.3 are `bun`-verified against
dependency-free modules. Nothing here claims a rendered pixel.

## 7. What the coordinator must run to verify

From the repo root, in this order.

1. **Scoped Storybook build + story-id list** (the command block's W5 report converged on; direct
   output to a path OUTSIDE the repo so a peer's `storybook-static` is never clobbered):

   ```
   STORYBOOK_SCOPE=fem bunx storybook build -c .storybook --output-dir <scratch>/storybook-static-fem
   jq '.entries | keys' <scratch>/storybook-static-fem/index.json
   ```

   Expect 24 entries (18 stories + 6 autodocs). Block's W5 fixed a repo-wide
   `ERR_IMPORT_ATTRIBUTE_MISSING` blocker in `main.ts`'s JSON import chain; if that regressed, the build
   fails before any fem file is touched.

2. **Interactive** — `parseStorybookSegments` validates the scope through `resolveActiveScopes`, which
   now knows `fem`, so this works today with no nx/package.json registration:

   ```
   bun ./📜️script.ts dev storybook fem
   ```

   then open `http://127.0.0.1:6010`. Confirm the fem2d canvases actually paint (they should — §2) and
   that `fem2d-add-node` appends `n12` to the `fem2d-model-debug` readout.

3. **Repo-wide storybook gate** (unchanged command, now covering 18 more stories):
   `bun nx run workspace:test-storybook`.

4. Nothing else. `bunx tsc --noEmit` over these files needs a `vite/client` types shim for the `?raw`
   imports (root `tsconfig.json` has no `types` entry) — block's W5 kept a ticket-local
   `tsconfig.w5-block-stories.json` for exactly this; the same trick works here if a typecheck is wanted.

## 8. Findings for the coordinator (outside W9's file scope — NOT fixed here)

1. **fem3d's example id is unreachable from the shell's switcher — a real bug.** The subset registers a
   single `ExampleSource` whose id is `"demo"`
   (`🗿️artifacts/🧊️3d/…/📚️examples/🎬️demo/🦀️.rs:5`, wired at `🌐️any/🦀️.rs:31`), but
   `set_active_example`'s handler branches on the literal `"default"`
   (`✏️editor/🎮️commands/📚️set-active-example/🦀️.rs`) and resets to `Fem3dSnapshot::default()` for
   anything else. The shell's navbar switcher dispatches ids straight from `PluginManifest.examples`,
   i.e. `"demo"` — so picking fem3d's own example in the running app **blanks the document**. fem2d has no
   such drift: its handler compares against `crate::artifacts::fem2d::examples::demo::ID`. The
   `🏗️fem🧊️3d/Model → ShippedExampleId` story renders this branch on purpose.
2. **fem2d's `setActiveExample` resets the session locale to `en-US`.** Its handler emits
   `Fem2dConfigMutation::Snapshot { config: Fem2dConfig::default() }`, and `Fem2dConfig::default()`
   carries `locale: "en-US"` — so switching example drops a German session back to English. fem3d avoids
   this only because its config factory refuses a whole-config `Snapshot` and it emits two granular
   mutations instead. Reproduced faithfully in `reduceFem2dStoryAction` rather than hidden.
3. **fem3d has no `setLocale` tool** while fem2d does — an asymmetry in the two rosters, and the reason
   the 3D stories' language selector dispatches nothing.
4. **No `dev:storybook:fem` launch registration.** CLAUDE.md requires every executable command in
   `launch.json`; the per-scope pattern is a `📋️project.json` `dev-storybook-<scope>` nx target + a
   `package.json` `dev:storybook:<scope>` script + `.vscode/launch.json` and `🧩️launch.seed.jsonc`
   entries (see `dev-storybook-cad`, `📋️project.json:163`). W9's file scope excludes all four, so they
   are NOT added here. Note this is a scope-wide gap, not fem-specific: `block` and `remodel` are equally
   unregistered. Suggested triple, mirroring cad exactly:
   `📋️project.json` `"dev-storybook-fem": { … "command": "bun ./📜️script.ts dev storybook fem" … }`,
   `package.json` `"dev:storybook:fem": "bun nx run workspace:dev-storybook-fem"`, and a
   `🛠️dev📖️storybook🏗️fem` launch entry in both `.vscode` files.
5. **No Playwright spec added.** The existing `.storybook/*.spec.ts` files need a running Storybook,
   which the box could not spare. The stories were written for one: the testid vocabulary in §5.1 matches
   `framework-hosts-no-wasm.spec.ts`'s `readDebug` / `expectStoryLoads` pattern, and because fem needs no
   wasm loader the whole scope belongs in the **no-wasm** spec, not the wasm one.
