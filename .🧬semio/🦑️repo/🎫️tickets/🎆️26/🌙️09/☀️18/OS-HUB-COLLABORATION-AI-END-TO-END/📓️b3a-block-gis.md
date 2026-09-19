# B3a — 🧱️block + 🌍️gis (middle-tier plugins)

Worker: Opus 5, fleet 3 relaunch (2026-09-19). Slice order: (1) gis compile unblock for the hub — **H1 is waiting on §1**, (2) gis boot + interaction, (3) block E0053 + descriptor + boot.

Captures: `🗑️generated/b3a-*.txt`. Scripts: `📜️b3a-activate.sh`, `📜️b3a-serve.sh`, `🐍️b3a-interaction-probe.mjs`.

---

## 1. URGENT — does `semio-s-artifact-gis-gismap` still block `semio-hub`?

**Status: RESOLVED — gis no longer blocks the hub. `cargo check -p semio-hub` is green (§1.3).**

### 1.1 What the `MutationLeaf` source-authority check compares

`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`:
- `expand_mutation_leaf` (:549) takes the **on-disk path of the source file** (`input.ident.span().unwrap().local_file()`, :552) and calls `mutation_source_authority(&source, &compiler_cwd)` (:557). Any `Err` becomes the compile error `MutationLeaf source authority failed: {error}`.
- `mutation_source_authority` (:88) derives the owner **from the real directory layout**: source file must be the taxonomy's canonical mutation primary filename; the owner directory is the source's parent (or its grandparent when the parent is the mutation payload facet); the mutation root must be the taxonomy's mutation collection directory, either flat or one hop below a **root explicitly registered in `📋️project.json → metadata.semio.taxonomy` → `🔣️taxonomy.json:mutationDomainOwners`**.
- It then reads the sibling descriptor `🔣️.json` and runs `parse_mutation_leaf_descriptor` (:113) — which is where the descriptor's own `"owner"` **path literal** is compared against the `owner` just computed from the on-disk location.

So there are **two** literal-vs-disk comparisons a rename can break:
1. `taxonomy.json.mutationDomainOwners` root/operation path literals vs. the real directory path (error text: *"domain-operation root is not explicitly registered"* / *"source owner is not an exact registered domain operation"*).
2. Each mutation descriptor `🔣️.json`'s `"owner"` string vs. the computed owner (surfaced through the same `MutationLeaf source authority failed:` prefix because `parse_mutation_leaf_descriptor` is called from inside `mutation_source_authority`).

### 1.2 State found on disk (2026-09-19, fleet 3)

The 18 `MutationLeaf source authority failed` errors were comparison (2) above: six gis mutation-descriptor `"owner"` literals still carried the pre-rename `⚙️config` segment while their directory had already become `🎚️config`. **A peer repaired those six descriptors in the worktree before this slice started.** Evidence: `git status --short -- "✏️s/🔌️plugins/🌍️gis"` shows the gisterrain set-camera descriptor as `RM` (renamed **and** content-modified), and the gismap descriptors now read e.g.

```
"owner": "✏️s/🔌️plugins/🌍️gis/…/🪟️windows/🗺️map/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera"
```

`grep -rl '⚙️config' "✏️s/🔌️plugins/🌍️gis"` now returns exactly **one** file — the TS barrel, which is not a Rust compilation input. So nothing was left for me to fix on the Rust side; I verified rather than repaired.

### 1.3 Result for H1 — **gis no longer blocks the hub**

Measured, not inferred (captures in `🗑️generated/`):

| command | result | capture |
|---|---|---|
| `cargo check -p semio-s-artifact-gis-gismap` | **green**, 2 warnings (`unnecessary qualification`, `🦀️.rs:128,132`) | — |
| `cargo check -p semio-s-artifact-gis-gismap --features component-app-assembly` | **green**, same 2 warnings | — |
| `cargo check -p semio-s-artifact-gis-gisterrain --features component-app-assembly` | **green**, 0 errors | `b3a-gisterrain-check.txt` |
| `cargo check -p semio-hub` | **green, exit 0, 0 errors**, 7 warnings in `semio-hub` itself | `b3a-hub-check.txt` |

**`cargo check -p semio-hub` is fully green — zero errors anywhere, not just in gis.** There is no remaining failure list to attribute to other owners. The only hub-crate diagnostics are dead-code/never-read warnings (e.g. `🌎️hub/📦️packages/🦀️rust/../../🏗️bootstrap/🦀️.rs:6088` `directory_message_visible` never used; `🧰️framework/…/🛢️db/…/🗿️artifact/🦀️.rs:468` field `last_error` never read).

Caveat for H1: this is the **default** feature set of `semio-hub`. `--all-features` was not exercised by this slice.

### 1.4 The one break G3 item 1 flagged, now fixed

`…/🪟️windows/🗺️map/🟦️.ts` still re-exported through the pre-rename directory. Fixed (see §4).

---

## 2. 🌍️gis boot + interaction (variant `gis2d`, app `s.gis.gismap@1/*#editor`, react :6040)

Recipe: `📜️b3a-activate.sh gis2d 6040` once (4 m 45 s cold, `@semio-tech/gis-plugin:component-dev` is the
3 m critical path), then `nohup 📜️b3a-serve.sh gis2d 6040 & disown`, then `🐍️b3a-gis2d-probe.mjs`
(headless chromium, 1600×1000 visible viewport, `--use-angle=metal`).

### 2.1 Where it stands

| bar | before | after | evidence |
|---|---|---|---|
| window publishes / app ready | ❌ nothing rendered, then ❌ `data-semio-os-error=gis2d` | ✅ `ready=gis2d`, window `gis2d-main`, 1 canvas pane 1587×907, tab "Map", 27 actions | `🗑️generated/b3a-gis2d/report.json` |
| default example opens | ❌ refused every boot | ✅ no refusal; document parses | `b3a-gis2d-console.txt` |
| a mutating action changes the document | ❌ | ❌ **still not cleared** — see §2.4 | same |
| undo reverts it | ❌ | ❌ (nothing to revert) | same |
| no console errors / guest traps | ❌ 3 fault lines | ⚠️ 2 fault lines, both the same `shell.panelTab` refusal (§2.5) | same |

**gis2d boots and renders; it does not yet clear the full interaction bar.**

### 2.2 Root fault A — the default example could not parse (fixed)

Boot console: `setActiveExample refused: dispatch-failed … gis map example 'demo' does not parse:
TextError { message: "0.unknown field ‹lon›" }`. Reproduced natively — `cargo test -p
semio-s-artifact-gis-gismap --lib inference_determinism_law` panicked on `demo fixture parses`
(capture `b3a-gis-example-test-before.txt`).

Cause: the `demo` asset's `positions=`/`routes=` lines are hex-encoded JSON arrays of **flat** feature
objects (`{id, lon, lat, label, …}`), the shape `MapFeature` carried before its payload moved behind
the opaque `data` escape hatch. Today `MapFeature` is `{ id, data: DslValue }` with
`#[value(deny_unknown_fields)]` (`…/🧬️schema/📍️feature/🦀️.rs:14-19`), so every flat key faults.
The asset was the outlier, not the schema: `…/🧬️schema/🦀️.rs:250` `gis_map_document_from_descriptor_json`
documents `{id: item.id, data: item}` as canonical and 8+ tests build `MapFeature { id, data }`.

Fixed by migrating the asset with exactly that transform (`🐍️b3a-migrate-gis-demo-asset.py`, 152
positions + 149 routes). Keeping the whole original object as the payload also keeps
`gis_map_descriptor_json` projecting the renderer's original flat array unchanged. A repo-wide scan
of every `*.dsl.semio` under `🌍️gis` confirms all three assets are now on the new shape.

### 2.3 Root fault B — the map surface exceeded the fixed UI capacity (fixed)

With the document finally non-empty, the boot changed failure mode to a **guest-side admission fault**:

```
PluginRuntime: actor gis#1 stopped without publishing requested UI surfaces (missing=["1:gis2d-main"],
faults=["plugin.internal: ui.fixed-capacity: fixed UI admission failed at scene-surface.encode:
surface payload exceeds fixed capacity with 59667 bytes"])
```

`SurfaceProps.doc` is `ui_contract::UiFixedBytes`, a hard `UI_FIXED_BYTES = 32 * 1_024`
(`🧬️contract/🎬️action/🦀️.rs:22`) that **cannot page**, and `TiledMapScene` declared no lanes at all —
its `map_fixture_json` is the whole `{positions, routes, regions}` descriptor, so it scales with the
document. `SceneDoc::split_lanes` (`🎬️scene/🎬️scenes/🦀️.rs:60`) is the framework's existing answer,
already used by `World3dScene`/`Canvas2dScene`/`Board2dScene`/`Paint2dScene` for exactly this ("a
57 281-byte world-3d scene refused admission outright"). `TiledMapScene` had simply never been given
one, so **every** map big enough to matter silently failed to publish its window.

Fixed by giving tiled-map its lane, following the board-2d precedent end to end (see §4 for lines):
producer (`scene_surface` splits and emits a `paged_text_carrier` per lane — already generic),
language-neutral contract fixture, Rust lane enum + constants, TS mirror, and the React reassembly
selector in `🗣️Interpreter` (`PagedSurfaceView` merged lanes for `world-3d`/`canvas-2d`/`board-2d`
only; `tiled-map` now goes through it too).

Verified: `cargo test -p semio-framework-ui-scene tiledmap` 2/2 green, including a law that a scene
whose fixture is `2 × UI_FIXED_BYTES` still packs a spine under the ceiling. Live: after re-activating,
the window publishes, the fixed-capacity fault is gone, and the fault count dropped 3 → 2.

### 2.4 Why the mutation half of the bar is still open (honest gap)

Every gis verb the Actions rail can stage with an argument is an `ActionKind::View` by the manifest's
own contract — `…/✏️editor/🦀️.rs:1153-1162` groups `toggleLayerVisibility`, `fitWorld`, `setCamera`,
`setRenderMode`, `setVectorStyle`, `setLodMode`, `focusFeature`, `setLayerStrokeScale` under
"👁️ View actions — mutate ephemeral config state …, never the document". Measured: `setVectorStyle`
staged `value=figureGround` and submitted `ok`, and the ledger/edit count did not move — correct
behaviour, not a defect.

The `ActionKind::Mutation` verbs are not reachable from the rail either:
- `setActiveExample` — the catalogue has exactly ONE example (`example_catalogue()` returns only
  `demo`), so the staged default re-applies the document that is already open and diffs to zero ops.
- `patchPositions` / `patchRoutes` / `patchRoute` — declared with no `action_args`, so the palette
  cannot stage their payloads.

I then tried the framework clipboard route (`selectAll` → `cut`, `🐍️b3a-gis2d-probe.mjs`): `selectAll`
dispatches and journals ("Select All" reaches the ledger), but `cut` clicks `ok` and produces no
ledger row and no edit. **That is the open defect for whoever takes gis next** — plus a second
observation worth chasing with it: the "Select All" ledger row did not appear in the History panel at
the read taken ~28 s after its dispatch, only in the read after the following `undo`, which looks like
the panel refreshing on the next action rather than on the journal write.

Two probe improvements landed while measuring this and help every B3 slice:
`🐍️b3a-interaction-probe.mjs` now fills a staged **combobox** argument (the shell renders a select arg
as `BUTTON[role=combobox]` carrying the arg id, so the old `select`/`input` selectors always reported
`absent` and silently measured an argument-less dispatch), and supports a `setup: [actionIds]` list
dispatched before the witness is taken.

### 2.5 Remaining console fault (not fixed, framework-level)

```
app "s.gis.gismap@1/*#editor" dropped action "shell.panelTab" dispatched from window kind
"gis2d-main": no window kind declares it (window kinds: gis2d-main)
```
`shell.panelTab` is framework shell chrome: `🏛️ShellHost/🟦️.tsx:10202` notes it through
`noteShellCommand`, which routes it via `NOTE_SHELL_COMMAND_ACTION_ID` "so the plugin intercepts it
before the app sees it". It is accepted early in the run (ledger entry 2 is "Switch Panel Tab") and
refused later in the same session as window-scoped, which matches the known "panel actions dispatch in
the active window" hazard. Left for a framework owner: a plugin should not have to declare a shell
verb, so the fix belongs in the dispatch scoping, not in the gis manifest.

---

## 3. 🧱️block (block2d / block3d / block5d)

### 3.1 The briefed E0053 is already gone

All four block crates compile clean today (`--features component-app-assembly` where the crate has it;
`semio-s-plugin-block` has no such feature):

| crate | result | capture |
|---|---|---|
| `semio-s-artifact-block-2d` | green, 0 errors | `b3a-block2d-check.txt` |
| `semio-s-artifact-block-3d` | green, 0 errors | `b3a-semio-s-artifact-block-3d-check.txt` |
| `semio-s-artifact-block-5d` | green, 0 errors | `b3a-semio-s-artifact-block-5d-check.txt` |
| `semio-s-plugin-block` | green, 0 errors | `b3a-plugin-block-check.txt` |

The `async fn print_dsl` vs sync `ArtifactDsl` mismatch is not present: `print_dsl` is sync at every
block definition and call site. Treat that briefing line as stale.

### 3.2 Root fault — the block plugin could not assemble, so no descriptor could be generated (fixed)

The missing `✏️s/🔌️plugins/🧱️block/🔣️.json` is a **symptom**, not the defect. There is a generator
(`bun nx run @semio-tech/block-plugin:describe` → `describePluginComponent`, identical to the one every
other plugin uses, already registered in `📋️project.json`), and it refused:

```
refusing to write a placeholder descriptor for …/semio_s_plugin_block.wasm:
plugin assembly failed — artifact kind "kit.catalog" has conflicting descriptors
```

`PluginBuilder` (`🏗️builder/🦀️.rs:641-653`) collects every app's `artifact_kinds` into one map and
refuses the plugin when two contributions of one id are unequal. All three block apps declared
`kit.catalog` by hand, and the three copies had drifted: `dimension` was `"2d"` / `"3d"` / `"5d"`.
The plugin therefore never assembled at all — which is also why `🧱️block` is one of only three plugin
roots with no committed descriptor (the others are `📖️playbook` and `🗄️stdio`).

Fixed at the root: one canonical declaration, no duplication. `semio-s-artifact-block-2d` is the base
crate both other block crates already depend on, so `KIT_CATALOG_ARTIFACT_ID` +
`kit_catalog_artifact_kind()` now live there and all three apps contribute that single value. This also
matches the ownership note already written in `🧩️puzzle/🗿️artifacts/🧊️3d/🦀️.rs:720-726` ("Ownership
resolved to `🧱️block`, the only plugin that actually PRODUCES the kind"). The kind describes the
catalog (meshes), not the board that emitted it, so one `dimension` is correct.

Pinned by a new per-crate test, `every_app_contributes_the_one_kit_catalog_spelling`
(`✏️s/🔌️plugins/🧱️block/🧪️tests/🔬️surface/🦀️.rs`), which asserts each app declares the kind exactly
once with the identical spec **and** that `plugin()` assembles. Green:
`cargo test -p semio-s-plugin-block --lib kit_catalog` → 1 passed (`b3a-block-kitcatalog-test.txt`).
The three per-app `artifact_kinds` assertions that already existed still pass, because every app keeps
declaring the kind.

### 3.3 Descriptor generation — blocked by a peer's in-flight edit

With assembly fixed, `describe` got past the plugin-assembly gate and now dies earlier, building its
own tool against a shared crate a peer is mid-edit in:

```
error[E0425]: cannot find value `PURE_COMMAND_FIELDS` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1719 and :1872
error: could not compile `semio-framework-os-kernel`
```

Unrelated to block and outside this slice; per the preamble I did not wait on it. **`🔣️.json` and
`🛂️.descriptor.semio` are still absent** — re-run `bun nx run @semio-tech/block-plugin:describe`
(or `bun ./📜️script.ts describe` in `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust`) once
`semio-framework-os-kernel` builds again; nothing else should stand in the way.

### 3.4 Boot + interaction — not reached

block2d/3d/5d were never activated or probed in this slice (variants `block2d`/`block3d`/`block5d`,
react ports 6024/6025/6026, app ids `s.block.block{2,3,5}d@1/*#editor`, examples
`➡️hexagonal-cut-concrete-forest-right`, `🌲️hexagonal-cut-concrete-forest-left`, `🏢️nakagin-capsule`;
block3d additionally mounts the `/mesh` asset route). The probes
`🐍️b3a-block{2,3,5}d-probe.mjs` exist and now inherit the combobox/`setup` improvements from §2.4, but
their `action: "addHandleKind"` target is the dead predecessor's guess and is unverified. No wasm
`--target wasm32-wasip2` build and no launch.json rows were added for block.

---

## 4. Files changed

**🌍️gis**
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🟦️.ts:6,23-27` — G3 item 1: five `./🎚️options/…` import specifiers retargeted to the on-disk `./☑️options/…`; stale `⚙️config` doc reference on line 6 updated to `🎚️config`.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio:2-3` — `positions` (152) and `routes` (149) migrated from the flat feature shape to `{id, data}` (§2.2).
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs` — new `default_example_document_carries_addressable_features` (gated on `component-app-assembly`): the default example resolves to a non-empty document AND its descriptor projection still carries `lon`/`lat` for the renderer.

**Framework — tiled-map scene lane (§2.3)**
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` — `TiledMapScene.lanes` field; `SceneDoc::split_lanes`/`merge_lane`; new `TiledMapSceneLane` + `TILEDMAP_SCENE_LANE_KEY_PREFIX`/`_NAMES`/`_FIELDS`/`_BODY_KEYS`/`_OPTIONAL`; `TiledMapScene::base` initialises `lanes`; `ToValue`/`FromValue` carry `lanes` exactly as the other laned scenes do.
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️tiledmap-scene-lanes/🔣️.json` — **new**, the language-neutral lane declaration both languages are pinned against (same shape as `🚚️board2d-scene-lanes`).
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️scenes-unit/🦀️.rs` — new `//#region 🚚️TiledMapSceneLanes` with two tests: the declaration mirror, and the split/merge round trip plus a `2 × UI_FIXED_BYTES` fixture whose spine still packs under the ceiling.
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` — TS mirror: `TILEDMAP_SCENE_LANE_KEY_PREFIX`, `TILEDMAP_SCENE_LANES`, `tiledMapSceneLaneForBodyKey`, `tiledMapSceneFromLanes`, and `TiledMapScene.lanes`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` — import `TILEDMAP_SCENE_LANES`; `PagedSurfaceView`'s lane table resolves `"tiled-map"`; the `PagedSurfaceView` branch now covers `tiled-map` (it was `world-3d`/`canvas-2d`/`board-2d` only, so a tiled-map surface never reassembled its carriers).

**🧱️block**
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🦀️.rs` — new `KIT_CATALOG_ARTIFACT_ID` + `kit_catalog_artifact_kind()` in the base crate: the ONE `kit.catalog` spelling (§3.2).
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — each app's hand-written `ArtifactKindSpec` literal replaced by `.artifact_kind(kit_catalog_artifact_kind())`, and the per-file `const KIT_CATALOG_ARTIFACT_ID` replaced by a `use` of the shared one.
- `✏️s/🔌️plugins/🧱️block/🧪️tests/🔬️surface/🦀️.rs` — new `every_app_contributes_the_one_kit_catalog_spelling`: all three apps declare the kind exactly once with the identical spec, and `plugin()` assembles.
- `✏️s/🔌️plugins/🧱️block/🔣️.json` + `✏️s/🔌️plugins/🧱️block/🛂️.descriptor.semio` — **new, generated** by `bun ./📜️script.ts describe` (never hand-written): `packageId semio:block`, `pluginId block`, 6 apps, `wasm_sha256=ca5b2393…`. `🧱️block` is no longer one of the descriptor-less plugin roots (`📖️playbook` and `🗄️stdio` still are).

**Ticket scratch (not repo code)**
- `🐍️b3a-gis2d-probe.mjs`, `🐍️b3a-gis2d-diagnose.mjs`, `🐍️b3a-migrate-gis-demo-asset.py` — new.
- `🐍️b3a-interaction-probe.mjs` — combobox argument filling + `setup` pre-actions (§2.4); benefits every B3 probe that shares it.

No `.vscode/launch.json` row was added: `🛠️dev🌍️gis🗺️gismap⚛️react` and `🛠️dev🧱️block{◻️2d,🧊️3d,🖐️5d}⚛️react` already exist, and no plugin has a `describe` row, so adding one only for block would break the file's own consistency.

---

## 5. Honest gaps and handoffs

1. **gis2d does not clear the interaction bar.** Boot, render and undo-of-a-shell-verb work; no document mutation is reachable from the Actions rail (§2.4). The concrete next defect is `cut` over a `selectAll` selection producing no ledger row and no edit. Not fixed.
2. **`shell.panelTab` is refused as an undeclared action mid-session** (§2.5). Framework dispatch scoping, not a gis manifest problem. Not fixed.
3. **Lane arrival in React is inferred, not directly witnessed.** The fixed-capacity fault disappearing and the window publishing prove the producer half; I did not read the merged `mapFixtureJson` back out of the live React scene (the map paints to a canvas with no data attributes, and headless has no tile network, so `tileImgs: 0` / `markers: 0` are expected either way). A follow-up should assert the merged fixture through the host rather than through absence-of-fault.
4. **wgpu was not touched.** No production Rust caller of `SceneDoc::merge_lane` exists, so the wgpu target presumably does not reassemble lanes for *any* laned scene (world-3d/canvas-2d/board-2d/paint-2d included). Adding tiled-map to the laned set makes gis behave like those on wgpu — a pre-existing, uniform gap, not a regression I introduced, but worth an owner.
5. **Three `semio-framework-ui-scene` tests fail, and they are NOT mine** — `board2d_scene_lanes_mirror_the_language_neutral_declaration`, `board2d_scene_splits_into_the_declared_lanes_and_merges_back`, `pack::typed_scene_neutral_catalog_matches_native_serde_contracts` (capture `b3a-scene-full-test.txt`, 138 passed / 3 failed). Cause: `Board2dScene` gained `gridVisible`, `selectableNodes`, `selectableEdges`, `selectableHandles` without its fixtures being updated. Verified pre-existing — `git show HEAD:…/🎬️scenes/🦀️.rs` already has the fields while `git show HEAD:…/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json` has no `gridVisible`. **Fix = add the four fields to `spineFields` and to `roundTrip.assembled`/`roundTrip.spine` in `🚚️board2d-scene-lanes/🔣️.json`, and to the `board-2d` case in `🧬️contract/🧵️retained/🧫️fixtures/🧾️typed-scene/🔣️.json`.** The tiled-map case (index 7) passes; board-2d (index 8) is where the catalog test stops.
6. **G3 item 2 (the empty-facet-authority oracle) is a different defect than the audit describes, and I did not change it.** Measured: `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts` → 2 pass / 3 fail (`b3a-empty-facet-test.txt`). The `☑️options` fixture + its own `☑️options.json` oracle already agree with the live implementation (that test passes). What fails is the **golden 19-case fixture**: the live `semanticArtifactEmptyFacetProjectionAuthority` now returns `unclaimed` for the retired `🎚️options` path while the golden still expects `ownerForm: "artifact-window"` — so editing only the `windowFacet` enum at `🔣️.json:22`, as the audit recommends, would break the schema-match assertion at line 74 instead of fixing anything. A correct fix has to move the golden case and re-derive the test's pinned `goldenBytes.length` / sha256 (already stale: asserts 3154, file is 3155) and its `form !== null` count of 8. The third failure is an unrelated missing `🧹clean🧩️taxonomy🫙️artifact-empty-facet-authority` launch.json row (K1's territory). Left for V1/K1 — a coordinated fixture regeneration, not the mechanical one-liner the audit describes.
7. **🧱️block was never booted or probed** (§3.4). No wasm `wasm32-wasip2` dev build beyond the one `describe` produced, no activation, no interaction probe, no per-crate tests beyond the assembly one.
8. **`cargo check -p semio-hub --all-features` was not run** — §1.3's green is the default feature set.
9. A peer's in-flight break in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` (`PURE_COMMAND_FIELDS` undefined, E0425 ×2) blocked `describe` for ~30 min mid-slice; it resolved on its own and the descriptor generated on retry.

---

# B3a2 — continuation (2026-09-19, fleet 3 relaunch)

Worker: Opus 5, slice **B3a2**. Captures: `🗑️generated/b3a2-*.txt`. Order: (5.5) ui-scene fixtures, (5.6)
empty-facet golden, (1) gis real mutations, (2) block boot + bar.

## 6. Per-plugin status table (current)

| plugin / variant | compiles | assembles + descriptor | boots + renders | mutation reaches document | undo / redo | console clean |
|---|---|---|---|---|---|---|
| 🌍️gis `gis2d` (`s.gis.gismap@1`) | ✅ | ✅ | ✅ | — | — | — |
| 🌍️gis `gisterrain` (`s.gis.gisterrain@1`) | ✅ | ✅ | — | — | — | — |
| 🧱️block `block2d` | ✅ | ✅ | — | — | — | — |
| 🧱️block `block3d` | ✅ | ✅ | — | — | — | — |
| 🧱️block `block5d` | ✅ | ✅ | — | — | — | — |

Legend: ✅ measured green, ❌ measured red, — not reached yet in this slice. Updated as sections land.

## 7. §5.5 — the three `semio-framework-ui-scene` failures (FIXED, 141/141 green)

Root cause confirmed by reading the code, not guessed: `Board2dScene` carries
`#[serde(default = "board2d_default_true")]` on the four new fields (`grid_visible`, `selectable_nodes`,
`selectable_edges`, `selectable_handles`, `🎬️scenes/🦀️.rs:2174-2187`), but the **pack wire** struct
`Board2dScenePack` (`🎬️scenes/🦀️.rs:2225`, built by `scene_pack_wire!`) repeats **no** serde defaults —
only `lanes` carries `#[serde(default)]`. That wire is what `SceneDoc::decode_pack` actually decodes, so
every non-`Option` field is *required* on the wire and each fixture must list it. That is the established
convention for this macro (`glyphCatalogsJson` has a scene-level default too and is listed in both
fixtures), so the fix is in the fixtures, not in the wire.

Changed:
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json` — `spineFields` gained
  `gridVisible` (after `selectionMethod`) and `selectableNodes`/`selectableEdges`/`selectableHandles`
  (after `gridFactor`), in the Rust `ToValue` emission order; the same four added to
  `roundTrip.assembled` and `roundTrip.spine` so the language-neutral declaration stays explicit on
  both halves (the TS mirror in `🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx:343-350` compares
  `board2dSceneFromLanes(spine, …)` against `{...assembled, lanes: spine.lanes}`, so they must move together).
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/🧾️typed-scene/🔣️.json:12` — the `board-2d`
  case value gained the same four booleans, so `check::<Board2dScene>` admits the packed map again.

Verified: `cargo test -p semio-framework-ui-scene --lib` → **141 passed / 0 failed**
(`🗑️generated/b3a2-scene-test.txt`). Was 138/3 before.

## 8. §5.6 — the empty-facet-authority golden defect (FIXED at the root, 5/5 green)

Re-diagnosed and fixed rather than patched. The live taxonomy's `windowFacet` segment
(`📚️library/🔣️taxonomy.json:16886-16902`) lists names `🎚️config ☑️options 🎬️actions 👥️presence 🪛️utilities
🫧️transient` — `🎚️options` is retired. Two artefacts still carried the retired spelling and they must move
together, which is why the audit's "edit only the enum" recipe could not work:
- the **golden** `window-facet` case's owner path ended `…/🪟️windows/🪟️main/🎚️options`, so the live
  `semanticArtifactEmptyFacetProjectionAuthority` answered `unclaimed` while the golden expected
  `ownerForm: "artifact-window"` (test line 77);
- the **independent oracle** `🧬️schema/🫙️artifact-empty-facet-authority/🔣️.json` `definitions.windowFacet`
  enum still listed `🎚️options`, so changing only one of the two breaks the schema-match assertion at line 74.

Changed both to `☑️options`. The retired path keeps its negative coverage — the sibling
`☑️options.json` fixture already pins it as `old-generic-options → ownerForm: null` — so the golden stays
at exactly 19 cases with 8 non-null forms, as its own test asserts.

Byte pins: the golden is now **3154 bytes**, which is exactly the length the test already pinned — the
retired `🎚️options` (U+1F39A U+FE0F, 7 bytes) had made the file 3155. The sha256 pin did not match any
version in git history, so it was re-derived from the corrected file:
`d03f52fd…` → `4af3be2c7b9c6f344e3baad500bfb3b4cd7ba697c876f204b1e8039c04354d72`
(`🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts:42`).

Verified: `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts`
→ **5 pass / 0 fail, 169 expect() calls** (`🗑️generated/b3a2-empty-facet-after.txt`; before:
`b3a2-empty-facet-before.txt`, 3 pass / 2 fail). The launch-row failure B3a §5.6 reported as the third
failure was already fixed by K1 — the registration test passes untouched.

## 9. 🌍️gis real document mutations (§2.4 closed on the native side)

### 9.1 Inventory first — the mutation leaves already existed; the ACTION surface did not

`🗺️gismap`'s document is `{positions[], routes[], regions[], drawing, image, value}` where every feature
is `MapFeature { id, data: DslValue }` (`🧬️schema/🔣️.json`, `📍️feature/🦀️.rs`). Its
`🧬️schema/🧬️mutations/` already carries the **complete** twelve-leaf editing algebra, each with an
authored `🔺️diff` and `↩️inverse`:

| collection | create | delete | replace data | reorder |
|---|---|---|---|---|
| positions | `🆕create-position` | `🗑️delete-position` | `🔁replace-position-data` | `🔀reorder-positions` |
| routes | `🛣️create-route` | `✂️delete-route` | `♻️replace-route-data` | `🧭reorder-routes` |
| regions | `🌐create-region` | `🧹delete-region` | `🔄replace-region-data` | `🔃reorder-regions` |

Window config carries its own five-leaf mutation set (`…/🪟️windows/🗺️map/🎚️config/🧬️schema/🧬️mutations/`:
`🎥️set-camera`, `🎨️set-vector-style`, `👁️set-layer-visibility`, `📏️set-layer-stroke-scale`,
`🔽️set-lod-mode`) — real, undoable, addressed per window instance.

So the schema-first chain was already complete and B3a §2.4's diagnosis needed narrowing: the defect was
**not** a missing mutation vocabulary, it was that **no Actions-rail-stageable verb reached any of the
twelve document leaves**. `patchPositions`/`patchRoutes`/`patchRoute` are `ActionKind::Mutation` but
declare no `action_args`, so the palette cannot stage a payload; `toggleLayerVisibility` and
`setLayerStrokeScale` declare a `layerId` the palette could not stage either, so they dispatched with an
empty layer id and no-oped. Everything else is `ActionKind::View` by design.

### 9.2 What landed — four per-feature verbs, fully staged

`✏️editor/🎮️commands/🗺️features/🦀️.rs` gained a `🔖️FeatureCollections` region and four command modules.
Every verb rebuilds the addressed collection and hands the before/after pair to that collection's
existing `positions_operations`/`routes_operations`/`regions_operations` diff — the SAME granular path
`patchPositions` and `setActiveExample` take — so each verb inherits the leaves' authored inverses and is
undoable by construction rather than by a bespoke history.

| action | kind | staged args (all defaulted) | leaf it reaches |
|---|---|---|---|
| `addFeature` | Mutation | `collection`, `label`, `lon`, `lat`, `span` | `create-{position,route,region}` |
| `moveFeature` | Mutation | `collection`, `featureId`, `lon`, `lat` | `replace-{position,route,region}-data` |
| `renameFeature` | Mutation | `collection`, `featureId`, `label` | `replace-{position,route,region}-data` |
| `deleteFeature` | Mutation | `collection`, `featureId` | `delete-{position,route,region}` |

Design decisions worth their own line:
- **The id is minted, not typed.** `minted_feature_id` takes the lowest free `{position,route,region}-N`,
  so `addFeature` runs one-click from the rail with every default in place.
- **An empty `featureId` addresses the collection's newest entry.** Deterministic and payload-local: a
  retained command is journaled and replayed, so a selection-dependent target would not replay. This also
  makes `add → move → rename → delete` a four-click live sequence with nothing typed.
- **A move translates, it does not collapse.** A position's `lon`/`lat` are re-seated; a route polyline or
  a region ring is translated whole by the anchor delta, so its shape survives.
- **`addFeature` mints geometry each collection can draw** — a point, a real two-vertex segment, a closed
  quad — from `(lon, lat)` and `span` (`GIS2D_DEFAULT_FEATURE_SPAN = 0.01°`).
- **An unknown collection is a no-op, not a fault** — the rail can only stage the three declared ones.

Layer visibility and style are now rail-reachable too: `toggleLayerVisibility` and `setLayerStrokeScale`
gained `action_args` staging the eleven-entry `GIS_MAP_LAYER_IDS` stack (plus a 0.25–4.0 slider). They stay
window-config mutations — that is where the app's own contract puts them and they are already undoable on
that lane — but they are no longer dead verbs that dispatched an empty `layerId`.

### 9.3 The live-only chain this had to satisfy

`cargo check` was green while **every mounted test in the crate failed** with
`interactive-job.catalog-incomplete: a migrated generated command lacks its exact owner-local bounded
reducer proof`. A new command must be added in FIVE places, not two:
1. the `app_commands!` enum (appended — row order is the binary variant ordinal),
2. `GIS2D_RETAINED_TOOL_IDS`,
3. `GIS2D_RETAINED_PUBLICATION_CONTRACTS` (lane `Artifact` for the document verbs),
4. `command_from_action`,
5. **`bounded_first_step_tool_proofs!`** (`✏️editor/🦀️.rs:803`) — the one that is silent at compile time.

`the_retained_tool_surface_proofs_and_publication_contracts_cover_every_command` now pins 2/3/5 together
so the next command cannot repeat this.

### 9.4 Verified

`cargo test -p semio-s-artifact-gis-gismap --features component-app-assembly --lib` →
**261 passed / 1 failed** (`🗑️generated/b3a2-gismap-lib-tests.txt`), up from 258 before this section's
tests. Eleven new tests, all green:

- `add_feature_appends_a_position_and_undo_redo_round_trips` — mutate → undo → redo through the framework's
  own `assert_undo_redo_round_trip` (dispatch, settle, probe, `"undo"`, probe, `"redo"`, probe).
- `delete_feature_undo_redo_round_trips` — the same round trip for the destructive verb.
- `add_feature_mints_drawable_geometry_for_routes_and_regions` — 2-vertex route, 4-vertex region ring.
- `move_feature_translates_a_route_polyline_whole` — anchor lands on the staged coordinate, inter-vertex
  span preserved to 1e-9.
- `rename_feature_retitles_the_addressed_position_and_leaves_geometry_alone` (and keeps `name` in step).
- `rename_feature_with_an_empty_label_emits_nothing`,
  `delete_feature_addresses_the_newest_entry_when_no_id_is_staged`,
  `an_unknown_collection_is_a_no_op_rather_than_a_fault`.
- `every_feature_editing_verb_is_a_fully_defaulted_stageable_mutation`,
  `layer_verbs_stage_every_declared_map_layer`,
  `the_retained_tool_surface_proofs_and_publication_contracts_cover_every_command`.

**The one failure is not mine and not new:** `gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed`
dies in `maintenance_step` with `plugin.internal: candidate parent child projection is invalid` — the
artifact-envelope (archive replacement) path, which is exactly slice **F1**'s live blocker. The test file
is untouched by this slice (`git status --short` on it is empty) and no feature verb participates in
envelope ingress.
