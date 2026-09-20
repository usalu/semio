# Astra Sol Camera Framing

## Fault

Checkpoint 10 delivered identical camera seeds to React and WGPU, but the live cameras diverged. React's `WorldAutoFit` used producer bounds when present and otherwise measured the actual rendered instances with `THREE.Box3.setFromObject`. WGPU returned when `fitJson` omitted bounds, ran projection framing after its fit, and installed the delivered snapshot camera after both. It also keyed fit only by revision. The observed result was React targeting the loaded puzzle geometry near `[5.4054, 2.3406, 1.5015]`, while WGPU retained the reference/instance-origin targets `[7, 0, 0.01]` and `[3.5, 0, 0.005]`.

The current React orthographic sequence matters: `WorldProjectionContentFrame` establishes Top zoom, then `WorldAutoFit` moves the target and eye with the same sphere-fit distance as perspective while preserving that zoom. WGPU's old `frame_orbit_to_bounds` parallel branch recomputed zoom from the geometry and therefore did not implement this path.

## Implementation

`World3dState` now retains:

- the parsed fit request;
- an owner identity of fit revision plus the external scene-camera digest;
- the applied key, with producer bounds rounded to React's key precision when present;
- an allocation-free live-bounds cursor;
- a geometry generation invalidation witness.

`step_world3d_camera_fit` waits for scene-bridge, snapshot-apply, and draw-rebuild ownership to settle. Published bounds then apply directly. Without published bounds, the cursor visits at most one draw transition or one transformed local-AABB corner per frame. It reads only resident mesh leases and actual draw instance matrices, so an empty or unloaded scene does not frame a placeholder origin or consume the fit key. Geometry/draw changes retire and restart an incomplete measurement. A completed empty measurement remains as an exact draw/geometry-generation witness in the cursor's existing tail padding. That generation sleeps without polling; a later draw seal invalidates the witness and re-arms the ordinary World3d retained wake lane without growing `World3dState`.

The fit step runs after projection framing and before deriving the frame camera. Snapshot readiness prevents the delivered seed from landing afterward and overwriting the fit. `frame_orbit_to_bounds` now mirrors current React `WorldAutoFit` for both families: target the bounds centre, use bounding-sphere distance with the current FOV/aspect/padding, preserve look direction, and preserve zoom. Explicit projection-content framing remains in `frame_projection_orbit_to_bounds` and continues to own orthographic zoom.

A same revision and seed never retakes a user orbit. A new revision or external camera seed clears that ownership latch and frames once. Bounds changes participate in the applied key but do not independently clear a user gesture. Mesh roster churn is absent from the key, matching React.

Dynamic World3d retirement clears the value-only cursor before closing registry owners. The change adds no runtime dependency, unbounded traversal, allocation inside measurement, capacity increase, fuel increase, or fault suppression.

## Checkpoint 11 Runtime Follow-up

The first paired runtime after the base laws exposed three integration gaps which the isolated World driver could not show:

- scene paint ran before typed snapshot sealing, measured the empty generation, and the frame transaction never advanced camera fit after it sealed the real draws; Top therefore stayed at its wire pose for the entire 13-step run, while Perspective moved only when a later Settings action happened to wake the shell;
- the dock retained React's encoded `world-projection:{…}` initial template but never handed it to the World surface, so the boot Perspective eye used the authored direction instead of the layout's 3-Point direction;
- live bounds covered mesh triangles but omitted the ordinary `LineSegments` outline React mounts at scale `1.001`, leaving the fitted target and distance slightly short.

The frame transaction's `World3dSnapshot` terminal branch now advances one bounded camera-fit step after snapshot/draw work. The camera cursor remains in `world3d_cursor_work_pending`, so every unfinished corner owns the next event-driven frame. No global frame grant or pump was enlarged. A two-visible-pane law first completes and parks an empty generation for each pane, publishes the real resident geometry, proves both pending predicates re-arm, and pumps one step per pane until both fits apply inside the exact corner-walk ceiling.

Shell paint now decodes the same `world-projection:` payload React's `decodeWorldProjectionTemplateId` accepts, waits for the delivered snapshot lease, and seeds the pane exactly once. Encoded Top resolves to orthographic/cardinal Top with direction `[0,0,1]`; encoded 3-Point resolves to perspective/free with direction `[0.75,-0.75,0.55]`. The flat Projection-pane selection and icon also resolve from that encoded identity. Malformed payloads are refused rather than silently becoming another camera.

The live bounds cursor now includes the outline scale only for the same rows React outlines: edges enabled, non-provisional instances, and meshes without authored edge geometry. The shared fixture builds a real Three `LineSegments(EdgesGeometry)` child at scale `1.001`, so its Box3 oracle independently supplies the expanded transformed bounds and both Top and 3-Point expected poses.

## Shared Fixture and Laws

`🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🎥️world3d-camera-framing/🔣️.json` is the language-neutral `framework.world3d.camera-framing/v1` fixture. It defines:

- one inline loaded box mesh;
- two real instances, including a 90-degree rotation and nonuniform scale;
- perspective and orthographic Top camera seeds;
- viewport, padding, exact transformed bounds, centre, radius, fitted poses and zoom;
- empty, delayed, user-owned, revision and seed ownership laws.

The Rust law enters that fixture through the actual World scene JSON bridge, publishes a real `Mesh3dLease`, applies the snapshot/draw rebuild, and advances the camera cursor through the resident mesh bounds. It covers both camera families, confirms at least one retained step per transformed AABB corner, proves empty geometry remains unapplied, proves delayed geometry applies, proves a geometry-generation change retires partial work, and covers user orbit versus revision/seed ownership.

The follow-up Rust laws additionally cover two concurrently visible fit owners, exact empty-generation sleep/re-awaken behavior, initial-template ordering behind a real snapshot lease, and the retained renderer's explicit post-snapshot handoff. The Shell law decodes the actual Top and 3-Point strings authored in Puzzle3D's default layout.

The React oracle constructs the same geometry and transforms with actual Three `BufferGeometry`, `Mesh`, `Group`, `Quaternion`, and `Box3.setFromObject`, then passes the resulting centre/radius through the production `world3dFrameCameraFromBounds`. It also exercises production fit-key/owed helpers.

## Fixed-Slot Receipt

Native 17 measured the retained state change exactly:

- `Option<AdmittedSurfaceEntry<World3dState>>`: 23,208 → 23,336 bytes, +128;
- `AdmittedSurfaceMap<World3dState>`: 49,528 → 49,784 bytes, +256 for its two admitted slots.

The committed fixed-slot receipt and its explanatory law were updated to those measured values. Capacity and stack conversion thresholds are unchanged.

Native 21 then measured a second accidental increase after the empty-generation wake law: 23,336 → 23,344 bytes per admitted value and 49,784 → 49,800 bytes for the two-slot owner. The cause was one `bool` whose alignment occupied eight bytes. The cursor now uses the otherwise invalid `draw == u16::MAX` state as its empty-generation witness. This removes the field and restores the already committed 23,336/49,784 budget without changing a capacity, stack threshold, or traversal grant. Native 22 is the pending receipt for that restoration.

## Checkpoint 13 Asset Admission Regression

The checkpoint 13 runtime supplied a stronger integration red than the earlier resident-geometry laws. Both Puzzle3D panes applied the URL scene snapshot, then remained at zero draws and zero instances for more than 90 seconds. No GLB request could follow because `offer_missing_mesh_fetches` derives its work exclusively from resident draws.

The retained diagnostics identify the exact transaction boundary:

- `world3d delivery applied ... rebuild=true` appeared for both panes at 13,428 ms;
- the next frame reported `world3d draw rebuild ... step=Stale` for both panes at 14,760 ms;
- the rebuild then closed with `state-draws=0`, so neither URL ever entered the asset queue.

Shell applies a dock's initial projection template after `snapshot_lease` becomes resident and before the document paint advances the sealed draw rebuild. `apply_world3d_initial_projection_seed` changed the camera and advanced `interaction_revision`, but left that rebuild stamped with the prior revision. Top's following projection-content frame advanced it again. The next rebuild step therefore rejected the delivery whose snapshot had just completed. The Top target change proves that both view-state mutations ran; the explicit `Stale` result proves why the models and asset requests did not survive them.

Camera/view revision advances now carry a live rebuild's freshness witness across their camera-only mutation. This includes the initial template, its following orthographic content frame, local navigation, camera reporting, and explicit projection selection. It preserves template-before-auto-fit order and keeps the URL draw that owns the asset request; document deliveries still use their snapshot lease and apply transaction as the independent freshness authority. A new law uses an actual URL-only scene bridge and the production Top-pane ordering: seal the bridge, complete snapshot apply without advancing the rebuild, apply the initial template, run the following document sync and projection-content frame, drain the rebuild, assert the URL draw and instance, admit exactly one GLB request, publish its decoded mesh lease, and complete camera fit from the translated loaded geometry rather than a placeholder origin.

World 14 then exposed the final missing producer lane in that law: the URL draw survived, but the Top content frame remained owed because `world3d_content_bounds` had no points. The retained `instance_positions` map was read by projection framing and chunking but no production path ever populated it. This also corrects the checkpoint 13 interpretation: the Top target `[7, 0, 0.01]` was not the initial-template target applied twice. It was a single projection-content frame over the only bounds WGPU could see, Puzzle3D's reference plane. React includes every raw instance position before its GLB is loaded, so its corresponding content frame sees the URL instance as well. The scene bridge now publishes those same raw positions, defaulting an omitted position to the origin exactly as `worldSceneContentBounds` does, before the snapshot becomes visible. The URL law asserts the pre-asset Top frame targets `[2, 0, 0]`, then independently proves loaded-mesh auto-fit replaces it.

World 17 proved that this producer lane also changes the intended sequential Top zoom. A two-pane wake law first delivered an empty scene, then raw instance positions and loaded geometry. React re-runs `WorldProjectionContentFrame` when its raw-content bounds key changes, then `WorldAutoFit` preserves that newly framed orthographic zoom. The old law compared against the original wire zoom `7.7595`; the actual production sequence yields `88.518518…` for the fixture's `[478,814]` viewport and raw position bounds. The neutral fixture now names this sequential result separately. A mounted-source oracle calls the production `worldSceneContentBounds` and `frameWorldProjectionPose` before the existing actual-Three Box3 fit and passes all four camera laws. No production camera behavior or tolerance changed.

## Verification

Focused React/Three oracle:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache -- --run '../../../../🧪️tests/🎥️world3d-camera-framing/🟦️.ts' --silent=false --reporter=verbose
```

Base result: 1 file passed, 3 tests passed, 4.81 seconds Vitest / 5.6 seconds Nx. The checkpoint-11 follow-up reran the expanded actual Three `EdgesGeometry`/`LineSegments` oracle: 1 file and all 3 tests passed, with 43 ms test time, 16.08 seconds Vitest duration, and 18.3 seconds Nx duration.

All touched Rust sources and tests parse with standalone `rustfmt --edition 2021 --emit stdout`. Per packet ownership, no Cargo or native build was run locally. Native 17 compiled the production camera changes; its only camera-related red result was the exact fixed-slot receipt above. UI 15 then passed 610/610, including the updated shared orbit math.

World 7 supplied the required red receipt: 221/224 passed. Two older fit tests stepped while the real JSON bridge was still staged, and the new ownership test exposed a fixture pump that stopped at the draw swap while its old draws and bridge lease still awaited bounded retirement. World 8 passed the ownership and redelivery laws after the fixture began draining the actual retirement steps, leaving 223/224. World 9 proved the last “empty geometry” helper had inherited a real triangle and then compared against the pre-delivery default orbit instead of the delivered camera seed. After making that fixture genuinely empty and capturing its baseline after bridge ingestion, World 10 passed 224/224 with 243 filtered in 1.589 seconds test time / 20.4 seconds Nx. No ceiling, budget, ready flag, or production fit behavior was changed to make those laws pass.

Checkpoint 11 supplied the integration red receipt described above. The follow-up source and laws parse with standalone `rustfmt --check`, and `git diff --check` is clean. World 13 passed all 228 selected tests with 243 unrelated tests filtered, 3.598 seconds test time and 1 minute 2 seconds Nx duration. Native 21 compiled the camera packet and exposed the fixed-slot padding regression recorded above. Checkpoint 13 supplied the live asset-admission red receipt and the new exact-boundary law. World 14 passed 228/229 selected laws with 243 unrelated tests filtered; its sole red was the exact Top sync assertion, which exposed the unpopulated raw-position lane above. World 17 passed 228/229 selected laws with 243 filtered; the sole red exposed the stale pre-content zoom expectation above. The focused actual React/Three oracle then passed one file and all four tests in 11.8 seconds Nx time. World 18 passed all 229 selected laws with 243 unrelated laws filtered, 2.779 seconds test time and 1 minute 28 seconds Nx duration. Native 23 and the next paired runtime are pending at this report revision. No Cargo/native build or browser activation was run by Sol.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️math-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🎥️world3d-camera-framing/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎥️world3d-camera-framing/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫀️settle-pump/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs`
- `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs`


## Astra Runtime Aspect Correction — Checkpoint 14

The settled live Top camera retained a12.81577 z-distance delta despite matching target/zoom. Root traced the actual React `WorldAutoFit` to `(sceneCamera as { aspect?: number }).aspect ??1`. Three's OrthographicCamera has no aspect property; the existing test oracle had passed viewport width/height unconditionally, accidentally agreeing with the incorrect native rule.

The oracle now constructs actual Three OrthographicCamera/PerspectiveCamera instances and reads their aspect by the same host rule. This exposed two old neutral-fixture failures. The authoritative neutral orthographic fixture eye distance is10.086422076517433 (positionz11.648984576517433), not16.140232308204684. Parallel projection zoom remains unchanged. Rust's shared `frame_orbit_to_bounds` now usesaspect1for parallel families and viewportaspectfor perspective. The native law also checks that swapping portrait/landscape leaves parallel eye distance unchanged.

Validation: actualThreeoracle red2failed/2passed, then4/4green; focusednative red1failed, then entireUIscene141/141green0skipped (1.025stest/25.1sNx). Logs:`camera-aspect-oracle-red.log`,`camera-aspect-oracle-green.log`,`camera-aspect-native-red.log`,`camera-aspect-native-green.log` under Astra runtime. FullWorldstate gate and freshapplicationruntime remain pending.
