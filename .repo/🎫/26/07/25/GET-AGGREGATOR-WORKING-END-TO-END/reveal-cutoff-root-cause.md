# Aggregator / Puzzle 3D — object meshes invisible, vortices still hoverable

## Symptom

In the mit-bestand Aggregator (`framework/product/os/dev` + `ENTWERFEN_MIT_BESTAND_BRAND`, puzzle3d app,
`concrete-forest` / "Abbau Aufbau" example) the seeded object renders no mesh. The reference floorplan
plane and the grid draw normally. Hovering where the object should be still reveals its vortices.

## Runtime evidence (live browser, instrumented `framework/renderer/react/index.tsx`)

`World3dHost` scene payload:

    instanceCount 1
    instances[0]  { id: "seed-left-001",
                    meshId: "mesh:hexagonal-cut-concrete-forest-left",
                    position: [0,0,0],
                    revealIndex: null }          <-- null, not omitted
    meshes[2]     { id: "mesh:hexagonal-cut-concrete-forest-left",
                    url: "/mesh/hexagonal-cut-concrete-forest-left.glb",
                    hasData: false }

`WorldInstanceNode`: branch `glb`, scale `[1,1,1]`, styleKind `neutral`, opacity `1`, meshColor `#1d2b2f`,
disabled `false`.

`GlbInstanceMesh`: 57 meshes, `visible: true`, world AABB `[0,0,0] -> [10.81, 4.68, 3.00]` — i.e. the GLB
subtree itself is loaded, posed and unhidden.

Asset path ruled out: `/mesh/hexagonal-cut-concrete-forest-left.glb` serves `model/gltf-binary`, 86112 B,
sha256 identical to `asset/abbau-aufbau/hexagonal-cut-concrete-forest-left.glb` (not the placeholder), and
loading it through the app's own `GLTFLoader` in-page yields 57 meshes / 958 triangles with bounds
`[0,0,-4.68] -> [10.8,3,0]` — correct once `GLB_MESH_FRAME_ROTATION_X` (+90 deg X) is applied. No required
glTF extensions.

## Root cause

Three facts combine:

1. `puzzle/plugin/rs/lib.rs` `world_instances_json` built each instance with the `json!` macro and wrote
   `"revealIndex": object.reveal_index`. `json!` serializes `Option::None` as **`null`** — it does not omit
   the key. So every ordinary (non-fill-plan) object shipped `revealIndex: null`.
2. `puzzle/plugin/rs/lib.rs` `world3d_interaction_json` always emits
   `"revealCutoffs": { "puzzle3d-fill": runtime.fill_count }`, which is **0** on a fresh boot.
3. `framework/renderer/react/index.tsx` `applyRevealCutoff` skipped only `=== undefined`:

       if (instance.revealIndex === undefined) continue;          // null passes through
       const visible = cutoff === undefined || instance.revealIndex < cutoff;
       //              cutoff = 0, null < 0  ->  0 < 0  ->  false
       root.visible = visible;                                    // instance root group hidden

   `null` is not `undefined`, so the guard let it through, and `null < 0` coerces to `0 < 0` = `false`.
   Every ordinary object's instance root group got `visible = false`.

`isRevealCutoffHidden` (same file) had the identical `=== undefined` guard, so marquee hit-testing treated
untagged instances as hidden too (`null >= 0` is `true`).

The vortex markers render in `WorldVortexMarkers`, a **sibling** layer of the instances group, so they were
never hidden — which is exactly why the objects were invisible but their vortices still lit up on hover.

## Fix

- `puzzle/plugin/rs/lib.rs` `world_instances_json`: build the instance object, then insert `revealIndex`
  only when `object.reveal_index` is `Some`. Untagged objects now omit the key entirely, matching the
  host's declared `readonly revealIndex?: number` contract.
- `framework/renderer/react/index.tsx`: both guards (`applyRevealCutoff`, `isRevealCutoffHidden`) use a
  nullish check so a JSON `null` can never coerce to `0` again.

## Tests

- `puzzle/plugin/rs/lib.rs` — new `d3::tests::seeded_objects_omit_reveal_index_so_the_boot_cutoff_cannot_hide_them`
  asserts seeded instances carry no `revealIndex` key and that the boot cutoff really is `0`.
- `puzzle/plugin/rs/lib.rs` — `fill_render_reveals_the_full_available_plan_tagged_with_reveal_index`
  tightened from "no non-null u64" to "no key at all"; its comment previously *documented* the `null`
  behaviour that caused this bug.
- `framework/renderer/react/index.test.ts` — new case: a `null` `revealIndex` is untagged even at cutoff 0.
  Verified it fails (`expected false, got true`) with the old `=== undefined` guard restored.

## Verification status

- `cargo test -p puzzle-plugin`: 105 passed, 0 failed.
- `vitest framework/renderer/react/index.test.ts -t isRevealCutoffHidden`: 2 passed.
- Full `index.test.ts` run: 233 passed / 28 failed — all 28 fail with
  `TypeError: readStoredUiChromeCompact is not a function` from a concurrent session's in-flight
  `ui/js/react/index.tsx` refactor (`Expertise`, `readStoredUiChromeCompact` currently neither defined nor
  exported). Unrelated to this change; the same breakage stops the dev server from booting the shell, so
  the final live browser confirmation of the fix is still outstanding.
