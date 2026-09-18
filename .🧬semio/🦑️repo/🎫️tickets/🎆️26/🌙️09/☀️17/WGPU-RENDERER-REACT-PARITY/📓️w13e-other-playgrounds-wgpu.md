# 🎮️ W13e — the other wgpu playgrounds (generation3d, gis2d)

Packet W13e of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Every claim here was produced by a live
boot on this machine; every line number is post-edit. Logs and screenshots live under `🗑️generated/`
with the `w13e-` prefix.

Serves left running for the next packet:

| what | url |
| --- | --- |
| generation3d · wgpu | `http://127.0.0.1:6118/?plugin=generation3d` (log `🗑️generated/w13e-generation3d-wgpu-serve-6118.txt`) |
| generation3d · React twin | `http://127.0.0.1:6018/?plugin=generation3d` (log `🗑️generated/w13e-generation3d-react-serve-6018.txt`) |
| gis2d · wgpu | `http://127.0.0.1:6140/?plugin=gis2d` (log `🗑️generated/w13e-gis2d-wgpu-serve-6140.txt`) |

---

## 1. Headline: the wgpu boot graph was dead for ELEVEN playgrounds, not just these two

`generation3d` on wgpu did not boot at all. The frame Worker faulted with

```
worker-boot-failed: no wasm plugin modules found for variant generation3d
Plugin "flow-extension-draw" needs "flow", which is not installed.
```

### Root cause

The wgpu frame Worker resolves the plugin graph ITSELF at boot
(`🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:545`, `new PlaygroundBootPlanner(PLUGIN_CATALOG, variant)`),
while the React dev entry (`🧑‍💻dev/🟦️.ts:20`) boots from the serve-time
`virtual:semio-playground-session` rows and never runs `orderPluginRegistryEntries` at all. So only
the wgpu target validates the dependency graph — and the graph had a CYCLE:

* `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` declared
  `depends-on = ["flow-extension-bim", …, "flow-extension-text"]` (commit `0b460ed19f`, 2026-09-17
  12:02, which replaced `consumes = ["flow.extension"]` with that list), and
* every one of those extensions declares `depends-on = ["flow"]`.

`resolvePluginLoadOrder` reports `transaction.cycle{flow, flow-extension-bim}`, drops both, and the
retry then reports `transaction.dependency-missing` for `procedural` and for all seven remaining
extensions. The plan comes back EMPTY, `mountPluginHandles` mounts nothing, and the Worker faults.

Reproduced over the whole registry before the fix — **11 of 65 playground variants** resolved to a
broken plan, including `s` itself:

| variant | plugins in plan | first error |
| --- | --- | --- |
| flow, generation2d, generation3d | **0** | `transaction.cycle{flow, flow-extension-bim}` |
| aggregator, aussuchen, bearbeiten, demonstrator, generator, koordinator, verfolgen | 16 (of ~24) | same cycle + `demonstrator`/`procedural` dependency-missing |
| s | 48 | same |

### Fix

`✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml:15` — `depends-on` of its own extensions replaced by
`consumes = ["flow.extension"]`, which is what it carried before 09-17 and what every other
plugin/extension pair uses. Nothing is lost: the nx runtime closure
(`🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs:28`) already follows `consumes` →
contributors, so `activate-flow-*`/`activate-generation3d-*` still materialize all nine extensions,
and `expandPluginRegistry` (`🎠️kernel/🟦️.ts:465`) pulls them into the boot plan the same way.

After the fix: **0 of 65 variants fault**, and `generation3d` resolves
`flow, flow-extension-{bim,brep,dictionary,list,logic,math,primitive,text}, procedural` with `flow`
before `procedural` — which is exactly what the pre-existing law in
`📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts:451` demands. **That law was RED on HEAD
before this packet** (`Expected to contain: "procedural" / Received: []`) and is green now.

### New laws

`🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts`:

* *"resolves a non-empty, dependency-fault-free boot plan for every registered playground variant"* —
  the whole registry, which is what the frame Worker actually resolves.
* *"keeps host plugin and extension edges one-way"* — a plugin naming an extension that names it back
  is refused at its source, so the cycle cannot be re-declared.

Both pass; the two failures left in that file (`59` vs `60` crates) are a peer's crate-count drift,
unrelated, and were being fixed by a peer while this packet ran.

---

## 2. generation3d (procedural3d) — boots, paints, two faults fixed

Port 6118, React twin on 6018, both at 1440×900 dpr 1.

| run | outcome |
| --- | --- |
| `w13e-wgpu-generation3d-1` | `data-semio-os-error=generation3d`, worker dead — §1 |
| `w13e-wgpu-generation3d-3` | `data-semio-os-ready=generation3d`, 0 page errors, chrome paints; Tool-run panel FAULTED, flow graph BLANK |
| `w13e-wgpu-generation3d-4` | Tool-run panel fixed (§2.1) |
| `w13e-wgpu-generation3d-5` | flow graph paints (§2.2) — screenshot pair below |

Screenshot pair: `🗑️generated/w13e-pair-generation3d-react.png` vs
`🗑️generated/w13e-pair-generation3d-wgpu.png`.

### 2.1 P0 — one unmounted tree row killed the whole surface

Live symptom, in the window body where React draws the run's rows:

```
Surface unavailable: framework.panel.toolRun [framework.body.toolRun] — retained document ingress reached its terminal fault
[DEBUG] ui-doc paint fault window=framework.panel.toolRun phase=Some("synchronize-node") sync-line=1665
```

`sync_interactive_state_node_step`'s `TreeApplyScan`/`TreeApplyPrepare` answered
`RetainedInteractiveSyncStep::Fault` when a declared section/row had no retained child to write its
drag flag onto. `reconcile` mounts a row only when the published document carries a child record for
it, so a `Tree` whose producer authors sections/items without a matching record subtree is ORDINARY —
React renders such a tree from the component props alone, and this module's own `cfg(test)` twin
`sync_tree_row_drag_sources` walks past the row with `continue`. The strict arm turned that into a
terminal fault, and the shell paints a faulted surface as an empty body with a red notice.

Fixed in `🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`:

* `:1491` new `retained_sync_tree_skip` — advances to the next record and re-enters `TreeApplyPrepare`.
* `:1503` new `retained_sync_select_skip` — the same rule for an open `Select`'s popup rows, which had
  the identical strict arm.
* `:1665`/`:1681` (`TreeApplyPrepare`'s missing parent, `TreeApplyScan`'s exhausted child scan) and
  `:1596` (`SelectScan`) now skip instead of faulting.

Law: `🖱️ui/🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs` —
*"retained_tree_sync_skips_a_declared_row_the_document_never_mounted"*.

LIVE PROOF (`w13e-wgpu-generation3d-4/final.png`): the Tool runs panel now shows
`Finalized · Tessellating meshes (2/2)`, `Start`/`Dismiss`, `7 nodes evaluated, 3 meshes tessellated`
and the `Tessellated`/`Evaluated` rows — the same content React shows. `grep -c "terminal fault"` on
the console went from 264 to 0.

### 2.2 P0 — every engine surface solved to height ZERO

`semioWgpuIntrospection.dumpStructure` on the live Flow window:

```json
{"path":"stack[0]#procedural-play-main.body","kind":"stack","rect":[0,0,974.8,813.6]}
{"path":"stack[0]#procedural-play-main.body/componentScene[0]#procedural-main","kind":"componentScene","rect":[0,0,974.8,0.0]}
```

The scene was one pixel tall, so the node-graph engine was handed a 974×1 viewport and every node and
port fell outside it — the geometry census reported `visible:false` for 48 of 52 entities and 1×1
rects for the rest, and the window painted nothing at all.

Cause: `UiNode::ComponentScene` fell into `mounted_layout`'s `_ => LayoutNodeKind::Leaf` arm. A leaf
has no intrinsic size, and an AUTHORED container grows no child by itself
(`flex::grows_children` is `!authored && …`), so a scene whose record carries the default terminal
`LayoutSpec::Leaf` — which is what a producer that does not spell `grow` publishes — collapses. React
has no such rule: its surface host fills its parent whatever the record declares.

Fixed by giving the engine surface its own layout kind:

* `🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:63` new `LayoutNodeKind::EngineSurface`.
* `📐️flex/🦀️.rs:290` `flow_for` — clips, and grows into its container when the record neither sizes it
  (`Dim::Auto`) nor declares its own `grow`; a record that DOES size itself is untouched.
* `📌️mounted_layout/🦀️.rs:506` maps `UiNode::ComponentScene(_)` to it.

Laws in `🖱️ui/🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs`: fills the authored container, keeps an authored
height, and solves as the document ROOT (the shape a map playground publishes).

LIVE PROOF (`w13e-wgpu-generation3d-5/final.png`): the Flow window now paints the grid, all seven
nodes with their ports, the wire routing and the bottom-right minimap, in the same arrangement as the
React twin.

### 2.3 Remaining differences vs React (generation3d)

1. **Node/port LABELS are not drawn.** React prints `Polygon`, `ExtrudeCurve`, `! wire`, `? x` …; the
   wgpu nodes are drawn as empty boxes. Node BODIES, ports and wires are correct.
2. **The 3D Preview window paints no mesh.** `[DEBUG] world3d delivery applied surface=procedural-preview
   state-draws=0 state-instances=0 state-meshes=3 bridge=false bridge-lease=true apply=false
   rebuild=true … fault=None` — three meshes are resident, nothing publishes a draw for them. React
   shows the tessellated column. This is the W3d/W11a world-draw family, not the layout one.
3. **The Tool runs panel squeezes the Preview pane** (Preview body ends at x≈1140 where the panel
   starts) instead of floating over it as React's overlay does — the panel-overlay lane W12d/W13b owns.
4. Minor: the wgpu `Start`/`Dismiss` buttons overlap their labels slightly; React's tool-run header
   band is one row taller.

---

## 3. gis2d (TiledMap) — boots ready, presents NOTHING

Port 6140, activated clean (`activate-gis2d-wgpu-dev`, 19 tasks, `Successfully ran`).

`w13e-wgpu-gis2d-1` (70 s), `w13e-wgpu-gis2d-2` (150 s) and `w13e-wgpu-gis2d-3` (**420 s**):
`data-semio-os-ready=gis2d`,
`data-semio-os-error=null`, **zero page errors, zero worker faults, zero failed module requests** — and
the canvas is uniformly `#001117` (the page background). Not one pixel of chrome, navbar or map.

The console is the evidence. Boot completes normally:

```
[DEBUG] wgpu-shell boot: program=gis app=s.gis.gismap@1/*#editor mode=Some("edit")
[DEBUG] wgpu-shell render leave surface=gis2d-main 83 ms resident-roots=1
… artifact / catalogue / inspection / history surfaces render …
[DEBUG] wgpu-shell refresh scope=full rendered=5
[DEBUG] wgpu-worker boot_shell leave 3320 ms
[DEBUG] wgpu-shell dock plan canvas=1434x836 windows=1 gis2d-main@1434x814+3,54
[DEBUG] ui-doc ingress window=gis2d-main generation=1 nodes=1
```

…and then the worker goes **silent for the remaining 146 s**. No `ui-doc paint stalled`, no
`worker-present-failed`, no watchdog line, no fault — the pump simply stops emitting, which means the
worker is inside ONE call that never returns rather than looping over steps.

Facts collected around it:

* The window document is a SINGLE node: `componentScene#gis2d-main`, and with §2.2's fix it is laid
  out at `[0, 0, 1433.6, 813.6]` (`w13e-diag-gis2d/dumps.json`) — the layout half is healthy.
* The map asset lane WORKS: `[DEBUG] asset ready kind=MapTile { surface: gis2d-main … }` arrives
  before the silence. One vector tile aborts (`GET /vt/3/0/1.pbf net::ERR_ABORTED`), which is a miss,
  not a fault.
* Just before the silence the frame step overruns repeatedly:
  `worker-step recorded-overrun site=frame-step executing=48.300ms budget=8ms`, and the actor reports
  `reactor more-work streak=2 sources=["reconcile"]` several times.
* Ruled out: the tile cursors are NOT unbounded — `pick_raster_tile_zoom` lowers `z` until
  `visible_tile_cursor(..).remaining() <= MAX_VISIBLE_TILE_REQUESTS` (256,
  `🗺️surface/🗺️tiled-map/🦀️.rs:104`, `:435`), so `reserve_map_tile_fetches`' two `while` loops are
  capped at ~256 offers per lane per frame.
* Ruled out: §2.2's fill rule does not loop on a root engine surface — law
  *"an_engine_surface_root_solves_against_the_viewport_it_is_given"* passes.

* **It is a HANG, not slowness.** The 420 s probe still ends on a uniformly empty canvas with the
  worker's last line still `ui-doc ingress`, which settles the "the first vello raster of a 1434×814
  map is just slow in `wasm-dev`" hypothesis: seven minutes is not slow, it is stuck.
* The native seam is covered by `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`
  (`tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam`,
  `tiled_map_paint_reserves_the_visible_tiles_react_fetches`) — but that crate's TEST target does not
  currently build: a peer's in-flight agent-bridge change removed `ShellState::agent_chat_messages`
  while `🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:829-836` still reads it (12 errors, all in
  Shell/Dock test files, none in this packet's edits — the crate's `--lib` check is 0 errors). Those
  map laws could not be re-run from here; W13d owns that suite.

**Hand-off (P0).** The next step is not another guess: arm the freeze-dispatch technique from
`feedback-freeze-dispatch-to-read-guest-trap` — install a never-settling hook after
`ui-doc ingress` and read the console, or add one `[DEBUG]` line at the entry and exit of
`scenes::render_component_scene_step`'s `TiledMap` arm (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2585`
`sync_tiled_map_scene`) plus `ensure_engine_surface`, rebuild, and re-probe 6140. Since the 420 s run
rules slowness out, the live suspects are `MapHost::new()`, `host.sync_map_json(map_fixture_json)` and
`host.fit_world_camera()` (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2509`, `:2446`, `:2485`) — all three
run inside the FIRST window paint, all three are non-yielding, and the gis demo's own fixture is the
one input the passing native laws do not use.

No React twin was captured for gis2d: `dist/runtime/react/dev/` has no `gis2d` directory, so a
comparison needs `activate-gis2d-react-dev` first (~9 min under the build lock).

---

## 4. Not reached

`flow`, `note`, `puzzle2d`, `draw`, `layout`, `forms` were not booted. Each needs one
`activate-<variant>-wgpu-dev` under the shared `🗑️generated/wasm-build.lock` (8–13 min each on this
machine, the renderer wasm itself ~7 min of that). All six now RESOLVE a clean boot plan (§1), which
they did not before this packet — `flow` in particular was one of the three variants whose plan was
completely empty.

## 5. Verification run

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | 0 errors |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib` | 0 errors |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib` | **571 / 0** (was 569, +2 … +3 new laws) |
| `cargo test -p semio-framework-os-infinite --lib` | 442 / 4 — the four `board::ports::directed_dag` failures are pre-existing (named in W11a's report) |
| `bun test …/🧩️package-integration/🟦️.ts -t "boot plan"` | 1 / 0 (was 1 FAIL on HEAD) |
| `vitest …/✅️catalog-complete` | 16 / 2 — both failures are a peer's `59` vs `60` crate-count drift, not this packet |
| renderer wasm + `activate-generation3d-wgpu-dev` + `activate-gis2d-wgpu-dev` | rebuilt and activated under the shared lock |

The shared `wasm-build.lock` was taken before every build/activation and released after each one.
