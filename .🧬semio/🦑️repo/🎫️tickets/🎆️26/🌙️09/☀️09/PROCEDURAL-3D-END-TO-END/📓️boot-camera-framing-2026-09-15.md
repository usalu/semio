# 🎯 Boot camera framing — generation3d, 2026-09-15 (lane `boot-camera-framing`, React :6028)

Owns `📓️coordinator-walk-2026-09-14.md` rows **20** ("the boot camera is the seed pose; the fillet box is
cut off at the top-right edge, only `Frame visible` frames it") and **21** ("on a freshly recycled serve
`Frame visible` leaves `data-viewport-camera-json` at the seed pose and the box stays cut off").

---

## 0. The one-line version

The preview now frames the example it just delivered, once per document, in both roles and on both
renderers, because the **guest publishes the extent it delivered** and the world hosts frame *that* —
and three separate defects had to fall for it to hold: the hosts were measuring a scene graph that
answers an empty box for inline-buffer payloads, the fit distance was `radius × 1.8` where the geometry
needs `radius / sin(halfFov)` ≈ `radius × 2.6`, and every completed camera gesture **remounted the orbit
controls at the world origin** the moment the guest echoed the pose back. Runtime: **56/56** rows of
`🐍️boot-camera-frame-probe.mjs` green on :6028 (8 examples × {boot, user-camera, frame-button} × {edit,
viewer} + 8 picker switches), 0 page errors; battery `journey` **25/25**, `interact` `fit` **8/8**.

---

## 1. Root cause — three faults, one missing layer

### 1.1 The producer never said what it delivered

`preview_payload` built meshes and instances and nothing else. Both hosts therefore had to *discover*
the extent to frame:

- React measured `Box3.setFromObject(instancesGroupRef)`. The generation3d payload rides **inline
  `data` buffers**, not mesh URLs, and that box came back empty — so `handleFrameVisibleInstances` fell
  through to `world3dFrameCameraFromInstances(instances, …)`, which frames the *instance transforms*.
  Every generation3d instance sits at `position [0,0,0]` (the geometry is baked into the mesh), so the
  fallback framed a single point at the origin and produced a pose indistinguishable from the seed.
  **That is coordinator row 21 exactly**: the button fired, and framed nothing.
- wgpu had no scene graph to measure at all, and no `fit` handling of any kind — `World3dScene.fit_json`
  was decoded nowhere in `♾️infinite/🌍️world/🦀️.rs`.

### 1.2 The fit distance was short by a factor of 2.6

`fitCameraFromBounds` stood the eye `max(radius × padding, 2)` from the centre, with `radius = max(size)/2`.
Two errors compound: the bounding-sphere radius of a box is **half its diagonal**, not half its longest
edge (√3 × more for a cube), and the distance a sphere of radius `r` needs to project inside a `fovY`
frustum is `r / sin(halfAngle)` — **2.61 r** at the default 45° field, not `1.8 r`. A box "framed" by the
old rule reaches ~1.4 of the viewport half-width: cut off at the corner, which is what row 20 describes.

### 1.3 A self-echoed `setCamera` remounted the orbit controls at the origin

Measured with `🐍️camera-authority-recon.mjs` (`🗑️generated/boot-frame/authority/trace.json`), sampling both
DOM lanes once a second:

```
converged     viewport=[6.3711,-4.3711,5.0283]→[1,1,1]   scene=[4,-4,3]→[0,0,0]
orbit-end     viewport=[0.1467,-4.6881,7.3908]→[1,1,1]   scene=[4,-4,3]→[0,0,0]
after-orbit+3s viewport=…→[1,1,1]                        scene=[0.1467,-4.6881,7.3908]→[1,1,1]   ← echo lands
after-wheel+1s viewport=[0.1338,-4.2746,6.739]→[0,0,0]   scene=[0.1338,-4.2746,6.739]→[0,0,0]    ← target lost
```

The orbit round-trips correctly. The **next** gesture reports `target [0,0,0]`. Chain: the debounced
`setCamera` reaches the guest → the guest republishes the pose on the window-config lane →
`cameraSeedKey = world3dViewportCameraSeedKey(sceneCameraJson, detachEpoch)` changes → `WorldProjectionRig`
mounts a **fresh camera element** → `WorldOrbitControlsBridge`'s effect (keyed on `camera`) constructs a
**new `ThreeOrbitControls`, whose `target` starts at `(0,0,0)`** → the next `reportCamera` publishes that
origin as the user's target. `shouldReattachWorldViewportCamera` already knew this change was a self-echo
and suppressed the *reattach*; the seed key was re-deciding from the raw text and did not ask it.

### 1.4 …and a reattach silently disarmed the one-shot framing

Found by the first full probe run (`🗑️generated/boot-frame/full-run.txt`, 55/56): `edit:Rectangle Extrude
Volume` reported the seed pose while its **orbit controls target was `[1,1,1.5]`** — the box centre. The
framing had run and been thrown away: the guest republished its seed camera after the example load, the
reattach effect nulled `viewportCamera`, and `WorldAutoFit` never fired again because its key had already
been consumed. A reattach discards whatever the viewport held; it must therefore re-arm the framing.

---

## 2. The owning layer

One statement — *"frame the delivered bounds once per document"* — expressed once per side.

| # | File | What it now owns |
|---|---|---|
| 1 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs` | `frame_distance_for_radius(radius, fovY, aspect, margin)` — the exact standoff, binding on the *smaller* of the vertical and horizontal half-angles; `frame_orbit_to_bounds(orbit, min, max, aspect, margin)`; `WORLD_FRAME_BOUNDS_MARGIN = 1.12` |
| 2 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` | `world3d_fit_json(revision, padding, bounds)` — the `fit` lane now carries `boundsMin`/`boundsMax` |
| 3 | `✏️s/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs` | `preview_payload_bounds(meshes_json)` (extent over `positions` **and** `edgePositions`, so a wire example has one), `preview_fit_revision(fixture)` (FNV over graph **topology** — widget ids, kinds, wiring — never parameter values), `preview_fit_json`, `PREVIEW_FIT_PADDING` |
| 4 | `✏️s/…/✏️edit/🪟️windows/👁️preview/🦀️.rs`, `…/🧬️generate/…/👁️preview/🦀️.rs`, `…/👁️viewer/…/👁️preview/🦀️.rs` | all three preview windows publish `fit_json` |
| 5 | `🧰️framework/…/🌐️World3dHost/🟦️.tsx` | `world3dFrameDistanceForRadius` / `world3dBoundsRadius` (TS twins of #1), `world3dAutoFitOwed`, `WorldAutoFit` frames the **published** bounds (scene `Box3` only as fallback) at the live canvas aspect, `handleFrameVisibleInstances` frames the same bounds, the user-moved latch, the attach-keyed rig |
| 6 | `🧰️framework/…/♾️infinite/🌍️world/🦀️.rs` | `World3dSceneFitRecord` + `sync_world3d_scene_fit` — the wgpu twin of #5, with `fit_framed_revision` / `fit_seen_revision` / `camera_user_moved` on `World3dState` |

**The rule, stated once:** a framing is owed when the producer's `fit.revision` has moved since the last
one applied, **or** nothing has been framed for this document yet **and** the user has not moved the
camera. A new revision clears the latch, because a new document is a framing the user has not refused
yet. Every re-evaluation of one example answers the same revision — which is what makes the framing
one-shot at all — because the revision is hashed over topology, not values.

Two consequences worth naming:

- **`Frame visible` no longer defers to the fit lane.** `world3dFrameVisibleOverlayOffered` used to
  return `false` whenever a producer enabled auto-fit. Boot framing stands down for good once the user
  moves the camera, so without the button a user who has orbited away has no way back to the geometry.
  They are two affordances, not two spellings of one.
- **The rig is keyed on the attach verdict, not the camera text.** `sceneCameraAttachJson` advances only
  inside the reattach effect, so a self-echo can no longer remount the controls (§1.3), and the fit key
  carries it so a genuine reattach re-arms the framing (§1.4).

---

## 3. Laws

### 3.1 Rust fixture law — `--test example-geometry` (18/18 green)

`🗑️generated/boot-frame/geom-2.txt`. Two new assertions per example, both reached from `assert_delivery`:

- `assert_delivery_bounds` — the **committed** `delivery.boundingBoxMin/Max/Tolerance`, hand-written into
  all 8 `🧫️fixtures/🧩️example/🔣️.json` from `run_delivery`'s own `preview_payload`, must equal what the
  editor payload **and** the viewer payload deliver, and must lie inside `expect.boundingBox*` (a coarse
  tessellation inscribes a fine one; it never exceeds it).
- `assert_delivered_bounds_frame_inside_the_viewport` — the camera `frame_orbit_to_bounds` derives from
  those bounds puts all **8 corners** in front of the eye and inside NDC, at aspects 1.7 / 1.0 / 0.7.

This is the clean end `📓️react-oracle-hardening-2026-09-14.md` §1.1 named and could not reach: `delivery`
now commits an extent, so the browser oracle grades a **number** and the 2 % shortfall envelope is gone.

| example | committed `delivery.boundingBoxMin → Max` (tol 0.0005) |
|---|---|
| hexagonal-mushroom-column | `[-0.5, -0.4330127, 0] → [0.5, 0.4330127, 6]` |
| rectangle-extrude-volume | `[0,0,0] → [2, 2, 3]` |
| face-sweep-extrude | `[0,0,0] → [2, 1.5, 4]` |
| box-shell-preview | `[0,0,0] → [2, 2, 2]` |
| box-fillet-preview | `[0,0,0] → [2, 2, 2]` |
| rectangle-wire-preview | `[0,0,0] → [2, 1.5, 0]` (from `edgePositions`) |
| sphere-box-fuse | `[-1.1862875, -1.1817694, -1.2] → [1.5, 1.5, 1.5]` |
| sphere-cut-with-torus | `[-2.1005719, -2.1357358, -2.2] → [2.1475, 2.1357358, 2.2]` |

### 3.2 wgpu law — `semio-framework-os-infinite`, 2/2 green

`a_world_surface_frames_the_producers_delivered_bounds_on_the_first_delivery_of_a_document` (a fit lane
without an extent frames nothing; with one, the target is the box centre and all 8 corners project
inside a 1600×900 viewport) and
`a_re_delivery_of_the_same_document_never_takes_back_a_camera_the_user_moved` (after `orbit` + `zoom`, a
second identical `fit_json` leaves yaw and distance untouched; bumping the revision frames again **at the
fitting distance while keeping the look direction the user chose**).

**Not claimed:** that `-p semio-framework-os-infinite --lib world` is green as a whole. 138 pass, **8 fail
and were already failing** — `sync_terrain_state_queues_fetch…`, `world_component_marquee_*`,
`world_object_registry_enforces_capacity_revision_and_aba`, `world_saturation_owner_*`,
`prepared_world_resources_*`, `live_renderer_retains_generation_wake…`,
`world_authority_retains_front_plan…`. None of them touch a camera, a fit lane or a scene bridge; they
fail identically under `--test-threads=1`.

### 3.3 TS twin — `🔬️engine-contract`, 7/7 green

`world3dFrameDistanceForRadius` (the standoff satisfies `asin(r/d) < min(halfV, halfH)` at five
fov/aspect pairs and is never more than 25 % beyond it — a rule that merely retreats is not a framing),
`world3dBoundsRadius` (half-diagonal, never half the longest edge), `world3dAutoFitOwed` (first delivery
frames; same key does not; a user-moved camera is never taken back; a new revision frames again), and a
per-example projection law over all 8 committed boxes × 3 aspects. `world3dFrameVisibleOverlayOffered`'s
own law was inverted to match §2.

### 3.4 Runtime — `🐍️boot-camera-frame-probe.mjs`, 56/56 on :6028

`🗑️generated/boot-frame/full2/results.json`, screenshots beside it. Every verdict is a **projection** of the
committed delivery box through the pose the pane published (`gradeCameraFrames`, now in the shared
`🐍️example-oracle.mjs`): 8 corners inside NDC, in front of the eye, and filling ≥ 12 % of the viewport so a
camera parked in the next county cannot pass. Nothing reproduces the host's arithmetic, so a change to
that arithmetic cannot make the probe agree with it — which is the flaw in the grader it replaces
(`gradeCameraFit` re-derived `max(radius × 1.8, 2)` from `fitCameraFromBounds` and would have passed the
clipped camera row 20 reported). `🐍️interaction-matrix-probe.mjs`'s `fit` hop reads the same new grader.

---

## 4. Per-example camera, before → after

Before is identical for every row and is the defect: `position [4,-4,3]`, `target [0,0,0]`, fov 45 — the
seed pose, whatever the example's extent. `worst X/Y` is the fraction of the viewport half-width /
half-height the box's outermost corner reaches (≤ 1.0 is on screen; the old rule reached ≈ 1.4).

| example | role | framed position → target | worst X / Y |
|---|---|---|---|
| hexagonal-mushroom-column | edit | `[9.5264, -9.5264, 10.1448] → [0, 0, 3]` | 0.206 / 0.495 |
| | viewer | `[5.6166, -5.6166, 7.2125] → [0, 0, 3]` | 0.119 / 0.875 |
| rectangle-extrude-volume | edit | `[7.3929, -5.3929, 6.2946] → [1, 1, 1.5]` | 0.641 / 0.495 |
| | viewer | `[4.7691, -2.7691, 4.3268] → [1, 1, 1.5]` | 0.362 / 0.875 |
| face-sweep-extrude | edit | `[8.3137, -6.5637, 7.4853] → [1, 0.75, 2]` | 0.503 / 0.490 |
| | viewer | `[5.312, -3.562, 5.234] → [1, 0.75, 2]` | 0.291 / 0.840 |
| box-shell-preview | edit | `[6.3711, -4.3711, 5.0283] → [1, 1, 1]` | 0.751 / 0.477 |
| | viewer | `[4.1667, -2.1667, 3.375] → [1, 1, 1]` | 0.420 / 0.870 |
| box-fillet-preview | edit | `[6.3711, -4.3711, 5.0283] → [1, 1, 1]` | 0.751 / 0.477 |
| | viewer | `[4.1667, -2.1667, 3.375] → [1, 1, 1]` | 0.420 / 0.870 |
| rectangle-wire-preview | edit | `[4.8762, -3.1262, 2.9072] → [1, 0.75, 0]` | 0.883 / 0.274 |
| | viewer | `[3.2854, -1.5354, 1.714] → [1, 0.75, 0]` | 0.482 / 0.546 |
| sphere-box-fuse | edit | `[7.3793, -7.0633, 5.5668] → [0.1569, 0.1591, 0.15]` | 0.750 / 0.478 |
| | viewer | `[4.4151, -4.0991, 3.3437] → [0.1569, 0.1591, 0.15]` | 0.419 / 0.871 |
| sphere-cut-with-torus | edit | `[11.5902, -11.5667, 8.675] → [0.0235, 0, 0]` | 0.744 / 0.480 |
| | viewer | `[6.843, -6.8195, 5.1147] → [0.0235, 0, 0]` | 0.416 / 0.872 |

The **picker-switch** pass reproduces the edit column byte for byte on all 8 (the `switch-frame` rows) —
opening at `?example=` and switching in place are the same framing. The two roles differ only by
viewport aspect (edit 0.56 with the panels open, viewer 1.77 full width), which is the aspect term of
§1.2 doing its job rather than a role-specific rule.

The wgpu column is the **native** law of §3.2 rather than a browser reading; see §6.

---

## 5. Batteries

- `SEMIO_BATTERY_URL=http://127.0.0.1:6028/?plugin=generation3d SEMIO_BATTERY_ROOT=react-boot-frame bun 🐍️react-battery.mjs --only=journey,interact`
  — `journey` **25/25 green, 0 page errors, 0 shell faults**. `interact` **55/58**, and its `fit` hop is
  **8/8** on the new projection grader (`🗑️generated/react-boot-frame/`).
- The three `interact` reds are **not this lane's and not new**: `Rectangle Wire Preview · select` (the
  pane publishes a `hoverTarget` of `rect@wire` and the click on that same point leaves `selectedIds`
  empty) with `· inspector` following from it, plus one `Hexagonal Mushroom Column · inspector` in the
  re-run where the Inspection panel had not opened. `Rectangle Wire Preview · select` was already red in
  `🗑️generated/react-safe-area/interact/results.json` (2026-09-14 19:43, before this lane existed) and
  green in two runs either side of it — a standing flake on the wire-only pick path, owned by
  `selection-prune-interact`. Nothing in this lane touches picking.

---

## 6. Not claimed

- **The wgpu battery did not run.** `bun 🐍️wgpu-battery.mjs --only=world3d-editor,world3d-viewer,examples`
  on :6118 is gated behind `until [ -z "$(pgrep -f 'wgpu-batter[y]|wgpu-.*-pro[b]e')" ]`, and lane
  `reconcile-spin` held :6118 with a live `wgpu-battery.mjs` + `wgpu-example-matrix-probe.mjs` for the
  whole window (still running at 09:57); three other lanes were queued on the same gate. The
  `activate-generation3d-wgpu-dev` restage therefore never started either — it must be run before the
  wgpu battery, and both are the first thing to pick up here. What IS proven on the wgpu side is §3.2
  (native, 2/2) plus `cargo check -p semio-framework-os-infinite` clean **for the renderer's own target**
  (`--target wasm32-unknown-unknown`, 22 warnings = the expansion really ran;
  `🗑️generated/boot-frame/check-infinite-wasm.txt`) — a native-only check would not have compiled the
  wgpu-gated code at all. What is **not** proven is a wgpu browser reading of a framed camera.
- **`WorldOrbitGated.reportCamera`'s origin fallback is still there.** `controls?.target ?? targetScratch.set(0,0,0)`
  (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`) invents the world origin when `useThree().controls` is momentarily null.
  §1.3 was proven to be the rig remount, not this, so it was left alone rather than changed on a
  suspicion — but a camera report that invents a target is the same class of defect and is worth a lane.
- **The 8 pre-existing `semio-framework-os-infinite` unit failures are not this lane's** (§3.2) and were
  not investigated.
- **A stale vite serve will hide all of this.** :6028 served host TS old enough to still cap a turn's
  patch batch at one, which threw `actor-ui-patch.pairing` against the current guest and wedged the shell
  at `meshes 0` — while :6027 on the same wasm booted clean. Recycling the serve
  (`📜️serve-generation3d-react-6028.sh`, its log now under `🗑️generated/boot-frame/`) fixed it. Every
  measurement above is from after that restart.
- **Nothing here says the camera is right for a document the user is EDITING into existence.** The
  revision is the graph's topology, so adding or deleting a node is a new document and frames again. That
  is deliberate for an example-driven playground and untested for a long authoring session.

---

## 7. Files

Product:
`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`,
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs`,
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`,
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`,
`…/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`,
`…/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`,
`…/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs`,
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️edit/🪟️windows/🧊️main/🦀️.rs` (the `world3d_fit_json` call site).

Laws and fixtures:
`✏️s/…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs`, `…/🧩️geometry/🟦️.ts`,
`✏️s/…/📚️examples/*/🧫️fixtures/🧩️example/🔣️.json` (all 8),
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`.

Probes (ticket folder): `🐍️boot-camera-frame-probe.mjs` (new), `🐍️camera-authority-recon.mjs` (new),
`🐍️example-oracle.mjs` (`gradeCameraFrames` replaces `gradeCameraFit`; `delivery.boundingBox*` exposed),
`🐍️interaction-matrix-probe.mjs` (`fit` hop reads the new grader),
`📜️serve-generation3d-react-6028.sh` (log path).
Evidence: `🗑️generated/boot-frame/` and `🗑️generated/react-boot-frame/`.
