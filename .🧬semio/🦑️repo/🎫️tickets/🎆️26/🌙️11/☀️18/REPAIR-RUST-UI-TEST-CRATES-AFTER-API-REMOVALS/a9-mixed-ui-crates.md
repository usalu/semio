# A9 Mixed UI Crates

Paths are relative to `🧰️framework/🔨️modules/🖱️ui/`.

## 1. `semio-ui-backend-metal` › `owned_runtime_preserves_empty_single_max_max_plus_one_and_hostile_contract`

- **Root cause:** commit `0c3e78944afc` (2026-10-14) changed the law to emit a schema-bound fixture (`$schema` = `…/render/schema.json#/$defs/MetalObjectiveCAbiFixture`, compact rows). Nobody updated the fixture, and the schema had no such `$def`.
- **Fix:** added the `MetalObjectiveCAbiFixture` definition to `🖌️render/🧬️schema/🔣️.json`, which is schema-first. It is a closed object that pins the layout, ownership and boundary constants. The fixture `🖌️render/🧪️tests/🧫️fixtures/🍎️metal/🔣️.json` was rewritten to the canonical bytes the law emits. The law itself is unchanged.

## 2. `semio-ui-render-shader-contract` › WGSL drift and shadow consumer

- **Drift root cause:** commit `8f91299152ce` (2026-10-19) added these to the wgpu target's `WORLD3D_SHADER` only:
  - section-plane clipping (`Globals.clip_plane`, `world3d_section_clipped`, `fs_section_stencil`, `fs_section_cap`)
  - authored emissive/alpha cutoff (`@location(10) emissive_cutoff`)
  - opaque-alpha bit `8u`

  This is exactly the W1f failure the law guards against. The contract had not changed since `4dab415ed905` (10-18).
- **Fix (contract is canonical, so the feature was propagated into it):**
  - `✨️shader-contract/🌐️world3d-mesh-family/🦀️.rs`
    - `WORLD3D_SHADER` is now byte-identical to the target copy.
    - The mesh instance buffer is 112 bytes, with `@location(10) Float32x4 @ 96`.
    - The world globals `min_size` is 256.
  - webgpu, the only other backend that compiles the contract WGSL and derives its layouts from the spec, now feeds neutral lanes (`🖌️render/🎯️targets/🧊️webgpu/🧮️gpu-uniforms/🦀️.rs`):
    - `World3dGlobals` adds `clip_plane = 0` (256 B), so it clips nothing.
    - `World3dGpuInstance` adds `emissive_cutoff = [0,0,0,-1]` (112 B): no extra emissive, no cutoff.
    - Its law `🖌️render/🧪️tests/🔬️webgpu-gpu-uniforms-unit/🦀️.rs` is updated to match.
  - d3d12 and metal compile their own HLSL/MSL mirrors with their own layouts, and the core `ui_render` pass/instance has no section or cutoff data, so they are unaffected.
- **Real bug fixed along the way:** webgpu packed `shadow = [enabled, opacity, softness, 0]`. Since `3b2f1181d27c` (09-28) the contract WGSL reads `globals.shadow.y > 0.5` as the SVG flat-lit profile bit, so any shadow opacity above 0.5 silently switched webgpu meshes to SVG face lighting. The core pass has no SVG profile, so webgpu now packs `[enabled, 0, 0, 0]`.
- **Shadow-consumer root cause:** the `48b9d63cf601` (09-20) law forbade `globals.shadow.y` anywhere in `WORLD3D_SHADER`. The 09-28 SVG profile law then requires `globals.shadow.y > 0.5` in the same shader, so the two laws contradicted each other.
- **Shadow-consumer fix:** `🖌️render/🧪️tests/🔬️shader-contract-unit/🦀️.rs` now checks the React-dead/biased tokens (`ndc.z - 0.0005`, `globals.shadow.y`, `globals.shadow.z`, `index < 9u`) inside the `world3d_shadow_visibility` body, which is the consumer the law names. All required tokens are unchanged.

## 3. `semio-ui-runtime-reconcile` › `fixed_runtime_owners_keep_bounded_state_off_the_stack`

- **Measured layout (temporary `[DEBUG]` print, since removed):** shadow = 2304, surface = 514, document = 40, resources = 208, loans = 1368, assembly = 88, handback = 32. The reconciler was 2344.
- **Root cause:** `UiNativeResourceLoans` (added 11-01…11-08) is held inline in `SurfaceShadow`. It inlines a pending `UiNativeMeshLoan` (two 514-byte `UiText`s) plus two failure places.
- **Fix:** `⏭️cursor/🪞️surface-shadow-and-record-read/🦀️.rs` pages it off the stack as `Option<Box<UiNativeResourceLoans>>`.
  - The page is born lazily and paid: `with_native_resource_loans_on_original` charges `retained_capacity_bytes = size_of` against the grant before boxing.
  - It is retired paid: `close_native_resources_on_original` charges `released_bytes` before dropping.
  - `take_all` moves it without allocating.
  - `retire_one` and `terminal_is_empty` require it to be absent.
  - `⏭️cursor/🚶️cursor-step/🦀️.rs` starts at `None`.
- The bound is unchanged. Expected reconciler size is about 984 bytes.

## 4 and 5. `semio-ui-contract-validation` / `semio-ui-contract-builder`

The coordinator reported both cascaded from missing arena scopes. Both crates are green in `a9-run1`.

Two edits were already made before that report; both were kept because they are correct under the current contract:

- **Number-control law** (`🧬️contract/🛡️limits/🧪️tests/🧩️components/🦀️.rs`): the document rows now name their control with `accessibility.label = case`. Rule `4f04c7f33989` (11-04) refuses unnamed controls, and the canonical corpus case `🧲️slider-with-snaps` is labelled too.
- **Builder laws:** every law in the builder crate starts with one `UiArenaFixture` scope. The scope's serial lock also serializes the process-wide `BUILT_CHILD_RETIRE_AUTHORITY`. Its full-capacity laws (all `builtChildRetireSlots`) otherwise race parallel laws for slots. Files:
  - `🧬️contract/🏗️builder/🧪️tests/🔬️unit/🦀️.rs`
  - `🧬️contract/🏗️builder/♻️retirement/🧪️tests/🔬️unit/🦀️.rs`
  - `🧬️contract/🏗️builder/♻️retirement/📋️children/🧪️tests/🔬️unit/🦀️.rs`

## 6. `native_resource_custody` › `native_resource_owner_and_loan_bundle_preserve_original_pages_and_full_close`

- **Root cause:** commit `8fdb3e10b975` (11-10) raised the law file's `TURN.maximum_copy_bytes` from 64 to 4096 but left the fixture's `caller.turn` at 64.
- **Why 4096 is right:** restoring 64 broke two sibling laws in a9-run1 (the receiver label/geometry loop and the message receiver), so 4096 is the current contract.
- **Fix:** `🎬️scene/📐️math/📦️resource/🧪️tests/🧫️fixtures/🔣️.json` now has `caller.turn.maximumCopyBytes = 4096`. This law is its only reader.

## Results

Run `🗑️generated/a9-run2.log` used `repo cargo … -p` with these crates. It exited with code 0 and every target passed:

| Crate | Test result |
|---|---|
| semio-ui-backend-metal | 5 passed |
| semio-ui-backend-webgpu | 14 passed |
| semio-ui-contract-builder | 27 passed |
| semio-ui-contract-validation | 41 passed |
| semio-ui-render | 32 passed |
| semio-ui-render-shader-contract | 11 passed |
| semio-ui-runtime-presentation | 24 passed |
| native_resource_custody | 5 passed |
| semio-ui-runtime-reconcile | 36 passed, 2 ignored |
| semio-ui-runtime-reconcile-cursor | 32 passed |
| Doc and integration targets | all ok |

## Follow-ups (not done here)

- Section clipping and authored cutoff still exist only on wgpu: the core `ui_render` model has no section plane, cutoff or SVG profile. d3d12 and metal still pack the dead opacity/softness lanes into `shadow.y/z`; their HLSL/MSL do not read them.
