# wgpu Mesh Oracle — 2026-09-14

Lane `wgpu-mesh-oracle`. Items 1, 2, 3, 5 and 7 of §4 of `📓️example-oracle-strength-audit-2026-09-14.md`.
Item 6's wheel-zoom belongs to lane `wgpu-wheel-zoom-a11y-live` and was not touched.

Evidence root for this lane: `🗑️generated/wgpu-oracle/` (the battery gained a `SEMIO_BATTERY_ROOT`
override so it no longer clobbers `wgpu-verify/`, which peer lanes publish into).

## 1. What the 2-vs-3-vs-1 mesh gap actually was

The audit's open question was whether the extra wgpu meshes are companion wires (fixture gains a
per-role expectation) or a product duplication (fix the product). **Neither.** The runtime number the
probes compared was never the published mesh count:

| reading | what it counts | where |
|---|---|---|
| `delivery.meshes` (oracle) | entries in the producer's `meshes_json` | `preview_payload`, asserted by `assert_delivery` |
| `state-meshes=N` (what every probe read) | the RENDERER's `mesh3d` store — published meshes **plus** the face overlay, the placeholder and whatever a retirement has not yet dropped | `World3dState::ingest_census`, `♾️infinite/🌍️world/🦀️.rs:1445` |
| `sceneInstances` (`dumpFrameStats`) | instances the frame drew | `build_frame_stats` |

`state-meshes` is a store size, so "2 for a wire example, 3 for a solid one" is the store, not a
duplication and not a companion wire. It is not comparable with `delivery.meshes` at any tolerance,
which is why the audit could find no honest fudge factor — there is none to find.

The fix is a third reading that IS comparable: `dumpMeshStats(windowId)` (new) answers the
producer's own publication, one row per `meshes_json` entry.

## 2. Roles are now stamped by the producer

`preview_eval::preview_mesh_role` (new) stamps every published mesh with `solid` / `wire` / `point` /
`vector`, and `preview_payload` writes it into the payload beside `id` and `data`. A preview
publishes one mesh per geometry-bearing output CHANNEL, so a graph that previews three nodes
publishes the wire it extruded and the vector that drove it beside its solid — which is exactly why
the bare count could never say whether the solid was on screen.

Every fixture gained a hand-written `delivery.meshRoles` (8 of 8, no fudge factor), and the native
lane asserts it exactly (`assert_eq!(run.payload_mesh_roles, delivery.mesh_roles)`), that the roles
add up to `delivery.meshes`, that each role is a declared one, and that the fixture's own
`preview.kind` carries at least one published mesh.

### Example × role — oracle vs runtime

`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry delivery_` — **8 passed, 0 failed** (measured, output in §5).

| example | committed `meshRoles` | native `payloadMeshRoles` | native meshes/instances |
|---|---|---|---|
| rectangle-wire-preview | `{wire: 1}` | `{wire: 1}` | 1 / 1 |
| rectangle-extrude-volume | `{solid: 1}` | `{solid: 1}` | 1 / 1 |
| face-sweep-extrude | `{solid: 1}` | `{solid: 1}` | 1 / 1 |
| hexagonal-mushroom-column | `{solid: 1, vector: 1, wire: 1}` | `{solid: 1, vector: 1, wire: 1}` | 3 / 3 |
| box-shell-preview | `{solid: 1}` | `{solid: 1}` | 1 / 1 |
| box-fillet-preview | `{solid: 1}` | `{solid: 1}` | 1 / 1 |
| sphere-box-fuse | `{solid: 1}` | `{solid: 1}` | 1 / 1 |
| sphere-cut-with-torus | `{solid: 1}` | `{solid: 1}` | 1 / 1 |

Hexagonal-mushroom-column's 3 are now NAMED (`profile@wire`, `extrusion-axis@vector`,
`extrude@solid`) rather than a coincidence that matched 3.

### Example × role — committed oracle vs LIVE wgpu publication

Measured on 6118, renderer wasm of 2026-09-15 00:05, guest restaged 01:04, battery root
`🗑️generated/wgpu-oracle/`. `store` is the renderer's `state-meshes` (the number every earlier probe
compared); `published` is `dumpMeshStats`' count of `meshes_json` entries.

| example | lane | oracle `meshRoles` | published roles | published | store | time-to-mesh |
|---|---|---|---|---|---|---|
| rectangle-wire-preview | edit / viewer | `{wire:1}` | `{wire:1}` / `{wire:1}` | 1 / 1 | 2 / 2 | 6.94 / 4.50 s |
| rectangle-extrude-volume | edit / viewer | `{solid:1}` | `{solid:1}` / `{solid:1}` | 1 / 1 | 3 / 3 | 9.07 / 5.61 s |
| face-sweep-extrude | edit / viewer | `{solid:1}` | `{solid:1}` / `{solid:1}` | 1 / 1 | 3 / 3 | 8.88 / 5.60 s |
| hexagonal-mushroom-column | edit / viewer | `{solid:1, vector:1, wire:1}` | same / same | 3 / 3 | 3 / 3 | 10.80 / 4.43 s |
| box-shell-preview | edit / viewer | `{solid:1}` | `{solid:1}` / `{solid:1}` | 1 / 1 | 3 / 3 | 8.06 / 5.58 s |
| box-fillet-preview | edit / viewer | `{solid:1}` | `{solid:1}` / `{solid:1}` | 1 / 1 | 3 / 3 | 6.77 / 5.56 s |
| sphere-box-fuse | edit / viewer | `{solid:1}` | `{solid:1}` / `{solid:1}` | 1 / 1 | 3 / 3 | 10.28 / 7.89 s |
| sphere-cut-with-torus | edit / viewer | `{solid:1}` | `{solid:1}` / `{solid:1}` | 1 / 1 | 3 / 3 | 10.14 / 7.88 s |

16 of 16 rows match their committed `delivery.meshRoles` exactly, and 16 of 16 preview-role bounding
boxes sit inside the committed `expect.boundingBoxMin/Max` at the stated LOD band. The `store` column
is the old number: constant at 3 for every solid example and 2 for the wire one, whatever the
publication is — a renderer-side store size, never a publication count. The audit's "extra meshes"
were neither companion wires nor a duplication.


## 3. What changed, file by file

### Product

| file | change |
|---|---|
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs` | `PREVIEW_MESH_ROLES` + `preview_mesh_role(inline, data)` — the role a published mesh declares |
| `…/✳️any/✏️editor/🦀️.rs` | `preview_payload` stamps `"role"` into every `meshes_json` entry; the role helpers re-exported beside the rest of `preview_eval` |
| `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` | **defect fixed** — `build_preview_mesh_table` stamps the role too. The viewer caches its own mesh table by signature and never went through the editor's `preview_payload`, so a role stamped on one side alone reached a viewer surface unlabelled: measured on 6118 in the 00:14 battery run as edit `{"solid": 1}` against viewer `{"(unstamped)": 1}` for the SAME example, on all four viewer rows that had finished. That run was stopped at the finding and its per-row evidence replaced by the clean 01:13 one, so the before-state is quoted here and in §5, not kept as a file; the after-state is every viewer row of `🗑️generated/wgpu-oracle/examples/` |
| `…/✳️any/👁️viewer/…/👁️preview/🧪️tests/🔬️unit/🦀️.rs` | the law that convicts it natively (`every_published_mesh_declares_its_role`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` | `dumpMeshStats(windowId)` export + `DumpMeshStats`/`DumpMeshSurface`/`DumpMesh`/`DumpMeshInstance`, `mesh_stats_for_scene`, `mesh_bounds`, `walk_mesh_stats`, `build_mesh_stats` |
| `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | `BrowserFrameIntrospectionProbe` gains `"mesh-stats"` |
| `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` | `RendererBindings.dumpMeshStats`; `answerIntrospection` routes `mesh-stats` to it |
| `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` | `semioWgpuIntrospection.dumpMeshStats` shim |

### Oracle (committed fixtures, hand-edited — all 8, no fudge factor)

| file | change |
|---|---|
| `📚️examples/<8 slugs>/🧫️fixtures/🧩️example/🔣️.json` | `delivery.meshRoles` added to each |
| `📚️examples/🧪️tests/🧩️geometry/🦀️.rs` | `DeliveryExpectation.mesh_roles`, `DeliveryRun.payload_mesh_roles`, exact role assertion + sum/validity/preview-kind checks, role map in the `[DELIVERY]` line |
| `📚️examples/🧪️tests/🧩️geometry/🟦️.ts` | TS twin: `meshRoles` on `ExampleDeliveryExpectation`, `EXAMPLE_PREVIEW_MESH_ROLES`, contract assertions in `assertDeliveryContract` |

### Probes

| file | change |
|---|---|
| `🐍️wgpu-example-matrix-probe.mjs` | reads the committed fixture per example; `publicationVerdict` asserts exact per-role counts and the preview-role bounding box against `expect.boundingBox*`; every row carries `oracle` |
| `🐍️wgpu-world3d-interaction-probe.mjs` | loops `SEMIO_PROBE_EXAMPLES` (one page load, one folder, all nine hops each); reads `dumpMeshStats` per hop; publishes an `oracle` step (expected targets, published roles, boot camera fit) and per-hop `publishedBefore`/`publishedAfter`/`cameraFit`; an inspector witness on h3 |
| `🐍️wgpu-battery.mjs` | `SEMIO_BATTERY_ROOT` override; `examples` lane gains one oracle step per row; `world3d-editor`/`world3d-viewer` run all 8 examples; `world3dVerdict` rewritten to value comparisons (hover/select target ids, selection set growth, camera framing) and the h4 gumball escape hatch removed |


## 4. Laws

| law | command | result |
|---|---|---|
| the producer publishes the committed roles, all 8 examples | `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry delivery_` | **8 passed, 0 failed** |
| the fixture contract itself (roles valid, summing to `meshes`, preview kind present) — TS twin | `bun test $(find . -path '*🧪️tests/🧩️example/🟦️.ts')` in `📚️examples` | **32 passed, 0 failed, 507 expects** |
| `dumpMeshStats` answers the publication, with roles and bounds, and finds nested surfaces at their absolute rect | `cargo test -p semio-framework-os-renderer-wgpu --lib introspection` | **7 passed, 0 failed** (2 of them new) |
| the frame Worker routes the new probe | `bun test ./🧰️framework/…/🧪️tests/📨️browser-frame-transport/🟦️.ts` | **33 passed, 0 failed** |
| the VIEWER's own mesh table stamps roles too (the defect §3 fixed) | `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib every_published_mesh_declares_its_role` | **1 passed, 0 failed** |

Measured `[DELIVERY]` role maps (native, `--nocapture`):

```
rectangle-wire-preview     payloadMeshes=1 payloadMeshRoles={"wire": 1}
rectangle-extrude-volume   payloadMeshes=1 payloadMeshRoles={"solid": 1}
face-sweep-extrude         payloadMeshes=1 payloadMeshRoles={"solid": 1}
hexagonal-mushroom-column  payloadMeshes=3 payloadMeshRoles={"solid": 1, "vector": 1, "wire": 1}
box-shell-preview          payloadMeshes=1 payloadMeshRoles={"solid": 1}
box-fillet-preview         payloadMeshes=1 payloadMeshRoles={"solid": 1}
sphere-box-fuse            payloadMeshes=1 payloadMeshRoles={"solid": 1}
sphere-cut-with-torus      payloadMeshes=1 payloadMeshRoles={"solid": 1}
```


## 5. Battery

`SEMIO_BATTERY_ROOT=wgpu-oracle bun 🐍️wgpu-battery.mjs --only=examples,world3d-editor,world3d-viewer`
against `http://127.0.0.1:6118/?plugin=generation3d`, one build, 3292 s, **0 pageerrors**, scoreboard
`🗑️generated/wgpu-oracle/scoreboard.json`.

| lane | verdict | steps | seconds |
|---|---|---|---|
| examples (8 examples × edit+viewer, each row scored twice: convergence + oracle) | **green** | 32/32 | 1395 |
| world3d-editor (8 examples × 9 hops + oracle + fault rows) | red | 75/115 | 953 |
| world3d-viewer (same) | red | 75/115 | 944 |

The world3d reds are eight examples reporting the SAME three defects, not forty separate ones:

| red predicate | examples | what it means |
|---|---|---|
| `h3 clicking the body selects that target`, `h3 the selection reaches the published document`, `h3 the selection shows in the shell`, `h4 shift-clicking adds to the selection`, `h6 a crossing marquee takes the body` | 7 of 8 (the wire example is exempt by construction) | §5.1 — selection never comes back |
| `the boot camera frames the committed bounding box`, `the camera still frames the body after the camera gestures` | `hexagonal-mushroom-column`, `sphere-cut-with-torus` | §5.2 — the boot camera is a fixed one |
| `h1 hovering the centre reports the example's own target` | `box-fillet-preview` | §5.3 |

Everything else is green in BOTH lanes, per example: the publication matches the fixture role-by-role,
hover reports the example's own topology id (`shell@solid`, `extrude@solid`, `fuse@solid`,
`brep_bool_cut_5@solid`), hovering the empty corner reports nothing, `h7` wheel / `h8` orbit / `h9`
pan each publish `setCamera` and move the camera, and no lane records an authority fault, a panic or a
dropped effect.

### 5.1 Selection never comes back (7 of 8 examples, both lanes) — NOT fixed

Traced hop by hop on `box-shell-preview`, `🗑️generated/wgpu-oracle/world3d-editor/box-shell-preview/`:

1. the pick resolves correctly — the frame publishes
   `interactionSelect args={domainId: "graph", targets: [{granularity: "object", id: "shell@solid"}], merge: "replace"}`,
   with the right topology id;
2. the guest accepts it — `[DEBUG] reserved.tool placement=isolated input=172B steps=2 outcome=ok`;
3. the bridge settles the job and carries frames back — `spawn-job settled … turns=3 status=idle frames=2`;
4. and the published payload never changes: `selection_json.ids` stays `[]` (the selection lane stays
   172 B across the whole run) and the world authority's own census stays `selected=0`.

Ruled out by reading the code, not by guessing: the ids are NOT pruned for granularity
(`validate_state`, `📡️replication/📡️wire/🦀️.rs:2839-2847`, filters by topology MEMBERSHIP only and
merely normalises the granularity field); `shell@solid` IS a declared member (`interaction_topology`
publishes every node's ports, `✏️editor/🦀️.rs:2132-2161`); and the payload side is correct — given
marks containing that id, `preview_payload` marks the instance (`marks_paint_hover_and_selection_from_a_bare_widget_id`,
green). The drop is therefore between the reserved interaction job's store write and
`InteractionView::selection("graph")`, and it is not a mesh-oracle defect: this lane's contribution is
the oracle that convicts it on all 7 examples at once instead of the log-text check that read green.

### 5.2 The boot camera is a fixed camera — owned by another lane

`preview_camera_json` publishes the stored default eye `[4, -4, 3]` → target `[0, 0, 0]`, fov 45,
whatever the example is. Measured: `hexagonal-mushroom-column` needs radius 3.07 and the frustum
covers 2.34; `sphere-cut-with-torus` needs 3.75 and covers 2.65 — so both load with the body partly
outside the viewport, while the six small examples happen to fit. I implemented a fit
(`preview_payload_bounds` + `preview_fit_camera`) and then **removed it again**: lane
`boot-camera-framing-2026-09-15` had landed its own `preview_payload_bounds` in the same file while I
was building, with a richer design (a topology revision so the fit is one-shot per document, and
`delivery.boundingBoxMin/Max` committed beside `expect.boundingBox*`). Their version stands; mine was
reverted rather than merged over it, and this lane contributes the assertion that scores it.

### 5.3 `box-fillet-preview` reports no hover target at the surface centre

Both lanes, every other example on the same build reports its own id. `h2` (empty corner → no target)
is green here, so the ray is being cast; the centre pixel of the fixed boot camera simply does not
land on this body. Very likely the same root cause as §5.2 — this is the one example whose framing
puts its centre off-body — and it should be re-measured once the framing lane lands rather than
chased separately.


## 6. Not claimed

- **Item 6 of the audit (the wheel-zoom hop) is not mine and was not touched.** `h7_wheel_zoom`
  publishes no `setCamera` at all in the runs read here (`actions: []`, camera unchanged, every
  example) — that is lane `wgpu-wheel-zoom-a11y-live`'s. The battery still scores it, so it stays
  visible; it did not become green because of anything in this lane.
- **The React side is untouched.** Items 1 and 4 of the audit also name `journey-probe.mjs` /
  `react-battery.mjs`; this lane's brief was the WGPU renderer, and the React half of item 1 (the
  `EXPECTED_MESHES` sibling) and item 4 (the viewer-role `meshes > 0` floor) are not done here.
- **`(unstamped)` is what every non-generation3d World3d producer publishes.** `dumpMeshStats` reports
  the absence rather than guessing a role from the arrays; CAD, puzzle3d and remodeling previews are
  not labelled, and no fixture of theirs was touched.
- **The role is what the payload CARRIES, not what the channel was named.** In `wireframe` show mode
  `apply_show_mode_mesh` strips triangles and a solid channel publishes role `wire` — correct, since
  that is what is on screen, but it means the committed `meshRoles` describe the default show mode
  the example lanes boot with.
- **No mesh duplication was found and none was fixed**, because there was none: the 2-vs-3-vs-1 gap
  was a store count being read as a publication count (§1). Nothing in the product was changed to
  make a count match.
- **The selection defect (§5.1) is NOT fixed.** It is located to one seam and convicted on 7 of 8
  examples in both lanes, and three candidate causes are ruled out in writing — but no code was
  changed for it, and `world3d-editor`/`world3d-viewer` stay red because of it. The laws were not
  weakened to make them pass.
- **The boot-camera fit (§5.2) is NOT fixed by this lane** — `boot-camera-framing-2026-09-15` owns it
  and had already landed the shared half; my own implementation was removed so as not to overwrite
  theirs. What this lane leaves behind is the assertion (`cameraFit`) that grades it.
- **`box-fillet-preview`'s missing centre hover (§5.3) is not fixed** and is probably downstream of
  §5.2; it was not chased separately.
- **One peer browser was collateral damage.** Clearing a wedged probe at 00:36 I ran
  `pkill -9 -f chrome-headless-shell`, which also killed a peer's `🐍️viewer-actions-probe.mjs` run
  that had been going ~15 min. A 24-hour-old orphaned `cargo test -p semio-framework-os-renderer-wgpu`
  (pid 97073, ppid 1) plus four deadlocked duplicate `framework-renderer-wgpu:wasm` nx runs were also
  killed — they were at 0 % CPU with no `rustc` child and were starving every build on the machine;
  compilation resumed within seconds of the kill.
