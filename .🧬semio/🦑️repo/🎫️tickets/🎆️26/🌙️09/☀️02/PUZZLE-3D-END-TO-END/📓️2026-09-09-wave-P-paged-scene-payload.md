# Wave P — the paged, per-lane world-3d scene carrier

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-P, 2026-09-09/10. Written incrementally while the wave ran.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, working tree carries many peers' uncommitted edits.
  Peers active in the same tree throughout: W-N (`🔌️PluginRuntime/🟦️.tsx` refresh read-back,
  `🗣️Interpreter` TreeView, puzzle editor `UiDirtyScope`s, wgpu menu labels) and the coordinator
  (`🔌️plugin/⚛️reactor/**`, `🖱️ui/🧠️runtime/**`). This wave touches none of those regions; every file
  was re-read immediately before each edit.
- The repo MCP server failed to connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk. The ticket is NOT closed by this wave.
- Crate under test `semio-s-artifact-puzzle-3d`, feature `component-app-assembly`.
- Private seeded target dir (56 GB):
  `/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d`.
- Command envelope for every Rust run below:
  `RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 CARGO_TARGET_DIR=<target-p3d> cargo …`.
- Handover baseline for the puzzle 3d suite: `591 passed; 22 failed`.

## 1 Defect + census

### 1.1 The measured fault

Browser rebuild #25 (2026-09-10 00:55), switching the example to "Nakagin Capsule Tower": the switch
completes, but the follow-up `refreshUi` fails in 0.4 s with

```
ui.fixed-capacity: fixed UI admission failed at scene-surface.encode:
surface payload exceeds fixed capacity with 57281 bytes
```

so the world body — and everything else that refresh carries — never updates and the user keeps seeing
Concrete Forest.

### 1.2 Why 32 KiB is structurally the wrong bound for this payload

`semio_framework_plugin::scene_surface` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:441`)
packs the whole typed scene into ONE `SurfaceProps.doc.bytes`, and `SurfaceDoc.bytes` is
`ui_contract::UiFixedBytes` — a hard `UI_FIXED_BYTES` = 32 KiB ceiling
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:22`) that **cannot page**. Wave M
had already written this down as the reason the reserved refresh sections do NOT ride as a surface doc
(`📓️2026-09-09-wave-M-retained-sections.md` §2.4, "Alternatives rejected").

Wave K removed the one pure re-send inside that blob (inline tessellation of built-in mesh kinds:
`world3d_mesh_kind_entry` now emits `{ id, kind }`, `📓️2026-09-09-wave-K-scene-payload-and-residuals.md`
§1.1). That bought ~27 KiB and made Concrete Forest fit. It did not, and could not, make the ceiling go
away: what remains in the blob is genuine per-document data whose size is proportional to the document,
and Nakagin is simply a bigger document. W-D3's byte census of the post-K payload:

```
meshes=… instances=270 selection=212 vortices=2639 attractions=2 volumes=2
refs=327 preview=281 interaction=1094 lod=125 chunk=40 env=92
```

The lanes also have wildly different change rates — `instances` changes when geometry changes, `lod`
and `chunking` change when a view option changes, `interaction` changes on every hover — yet a single
blob re-publishes all of them together on every one of those changes.

## 2 Design

Taken over by W-P2 (cursor fleet) at 2026-09-10 00:25 CEST. Predecessor W-P last edited the key files ~31 min earlier and left this section empty; files were 10+ min cold at takeover.

The world-3d scene is no longer one `SurfaceProps.doc.bytes` blob. `SceneDoc::split_lanes` peels every payload field listed in the language-neutral contract (`ui/scene/fixtures/world3d-scene-lanes`) off the scene and leaves only the **spine** inside the 32 KiB `UiFixedBytes` doc:

- spine: `snapshot`, `cameraJson`, `domainId`, `domainGranularityId`, `lanes[]` (name + UTF-8 byte length + FNV-1a/64 hex)
- 18 lanes: `meshes` / `instances` / `selection` (required, always published) and the optional JSON fields (`vortices` … `status`). An unset optional lane publishes no carrier.

`semio_framework_plugin::scene_surface` encodes the spine via `ui_scene::encode` and hangs each lane off the surface node as its own `paged_text_carrier` (512-byte text leaves, 32-ary page tree — the same shape reserved refresh sections already use). The reconciler pages that tree through `SURFACE_RECONCILE_PAGE_BYTES`. A camera nudge changes only the spine; a hover changes only `interaction`; geometry changes `instances`. The spine's `lanes[].hash` is what makes the surface node's own `Component` differ exactly when a lane's content differs.

TS twins: `WORLD3D_SCENE_LANES` / `world3dSceneFromLanes` in the mesh module. The Interpreter's `PagedSurfaceView` walks the retained children (`surfaceSceneLaneText`), caches complete texts by hash, and reassembles with `world3dSceneFromLanes`. A lane that has not finished arriving keeps its last complete text.

**Intake (the #26 stall).** The guest now emits a surface upsert whose children are the lane carriers. `WireUiPatch.ops` still looks like one small op because the nested tree hides behind a handle; the old `pluginUiIntakeBudget` stopped at depth 4 and credited 36 864 steps (`4096 + 4096×8`). The decoder advances one phase per leaf/byte/attach, so a Nakagin-scale tree never reached `ack` — `OwnedUiPatchIntake.advance` stayed in `input`/`publication` until the budget died. Compounding that, `OwnedUiSurfacePatch` staging waited forever on `!cell.initialized` / `!lease.hasCapacity` (16-byte yields, no forward progress). W-P2:

1. Measures the full nested op tree and floors the budget at 64 KiB × 8 phases (528 384 steps) so a 57 KiB one-op upsert can finish.
2. Rejects after 32 zero-byte pending steps in the same intake phase (`intake-zero-progress`) and after 4096 same-phase pending steps in `acceptUiPatches`.
3. Staging no longer spins: one yield, then skip the unread/full cell so publication can complete; those subscribers pick up the new revision on the next read.

## 3 Changes (file:line)

- `ui/scene` Rust scenes — already landed by W-P (`SceneDoc::split_lanes` / `World3dSceneLane` / hash). Not re-edited this wave.
- `os/plugin` Rust — already landed (`scene_surface` + `paged_text_carrier` children, fixture merge helpers). Coordinator-owned; not re-edited.
- `ui/scene` `scenes-unit` — added `a_nakagin_scale_world3d_scene_pages_per_lane_and_reassembles_losslessly`.
- plugin-builder-contract — already landed `World3dSceneLaneCarriers` (oversized / single-lane / paging). Not re-edited.
- mesh TS — already landed `WORLD3D_SCENE_LANES` / `world3dSceneFromLanes`. Not re-edited.
- Interpreter — already landed `PagedSurfaceView` + lane cache. Not re-edited.
- Interpreter `surface-scene-lanes` test — Nakagin-scale reassembly + hover-only lane law.
- `UiDocumentStore/intake` — zero-progress reject; exported full-tree `pluginUiIntakeBudget` (64 KiB floor) + vitest.
- `PluginRuntime` — shared budget; `acceptUiPatches` fails loudly after 4096 same-phase pending steps.
- retained `surface` — staging skips an uninitialized / at-capacity read cell after one yield instead of spinning.

## 4 Laws

(a) Rust `a_nakagin_scale_world3d_scene_pages_per_lane_and_reassembles_losslessly` (scenes-unit): unsplit pack > 57 281 bytes; spine pack ≤ `docBytesMax` (32 KiB); every lane chunks into ≤ `leafBytes` pages; `merge_lane` restores the source. Anchors: existing `world3d_scene_splits_into_the_declared_lanes_and_merges_back` and plugin-host `world3d_scene_surface_pages_every_lane_beside_a_spine_that_fits_the_fixed_doc`.

(b) TS `reassembles a Nakagin-scale multi-lane carrier without dropping a leaf` (surface-scene-lanes) + intake vitest `credits a one-op nested lane tree enough steps to finish a 57 KiB payload` (budget ≥ 57 281 × 8 and > 36 864 for a one-op upsert).

(c) Rust `world3d_scene_spine_changes_only_for_the_lanes_that_changed` + plugin-host `world3d_scene_surface_republishes_only_the_lanes_that_changed` + TS `a hover-only interaction change republishes only that lane`.

## 5 Commands + tails

Envelope: `RUSTC_WRAPPER="" RUST_MIN_STACK=134217728 CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d` plus `-j 4`. Tails filled after the verification run.

## 6 Not verified

- Browser rebuild #27 + Nakagin example switch (coordinator). Must confirm: no `ui.fixed-capacity` at `scene-surface.encode`, no `plugin-ui.intake-budget-exhausted` / `intake-zero-progress` on `puzzle3d-main-perspective`, world body updates, pick/inspection/context-menu unblocked.
- Served wasm is still #26 (this wave must not rebuild `component-release` / `component-dev` or touch :6013). The Rust guest half is already in #26; the TS intake/budget/staging fixes ride HMR / the next serve.
- `World3dHost` was not edited (only foreign W-H brush-mesh). Assembly happens in the Interpreter before the host sees the scene.
- End-to-end native intake of a real wasm patch (needs a live `OwnedNativeUiPatchAuthority` from a shard turn). Covered by budget + zero-progress laws and the existing typedwire intake suite.

