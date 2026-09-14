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

<!-- RUNTIME TABLE -->

## 3. What changed, file by file

### Product

| file | change |
|---|---|
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs` | `PREVIEW_MESH_ROLES` + `preview_mesh_role(inline, data)` — the role a published mesh declares |
| `…/✳️any/✏️editor/🦀️.rs` | `preview_payload` stamps `"role"` into every `meshes_json` entry; the role helpers re-exported beside the rest of `preview_eval` |
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

<!-- BATTERY -->

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
