# Astra Sol Scene Input and Residency

## Scope

This packet repairs the two confirmed World3d residuals from `📓️astra-terra-scenes-current.md`:

1. the document's `gridSnapEnabled` value now governs WGPU relocate ghosts, relocate commit arguments, catalogue drop ghosts, and catalogue drop commit origins;
2. active-document reference URL replacement/removal now retires old pixels and URL-cancels old asset claims, while the decoder rejects late bytes even when the pane's `World3dState` generation remains live.

The concurrent lighting/material work in the same World3d and renderer sources was left intact. This packet does not change shader, navbar, footer, cap, driver, or window behavior.

## Contract and fixture

The language-neutral contract is:

- schema: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧬️schema/🧲️scene-input-residency/🔣️.json`
- fixture: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🧲️scene-input-residency/🔣️.json`

It fixes the discriminating vectors at:

- input point `[1.4, -2.6, 0]`, grid factor `1`;
- snapping disabled → `[1.4, -2.6, 0]`;
- snapping enabled → `[1, -3, 0]`;
- removed URL A, replacement URL B;
- 300 distinct replacements, with at most one resident reference URL.

The TypeScript oracle validates the fixture with Ajv, obtains the enabled rounding independently from `three.Vector3.round`, and checks the current React source paths for the same boolean-aware relocate/catalogue decisions.

## Production repair

### Snap authority

`WorldLodRecord` now deserializes the existing shared `gridSnapEnabled` wire member and defaults it to `false`, matching the shared WGPU scene record and React's `lod.gridSnapEnabled ?? false`.

The single `snap_world_point_to_grid` decision now accepts both `grid_snap_enabled` and `grid_factor`. These production routes all use it:

- `world3d_relocate_dispatch_args`, reached by both relocate preview and release;
- `world3d_update_catalogue_drop_preview`;
- `world3d_catalogue_drop_origin`.

The existing relocate helper law was updated to spell the boolean explicitly. The new Rust law projects the neutral world point through a real orthographic camera and drives the actual WGPU pointer-coordinate preview/commit functions for both relocate and catalogue drop.

### Active-reference ownership

`sync_world3d_scene_document_lanes` parses the next reference set before installing it and calls `reconcile_world3d_reference_urls`:

- decoded pixel entries absent from the next document are removed from the 256-entry registry through `retire_world_pixels` and transferred to the existing fixed opaque-owner quarantine;
- pending URL markers absent from the next document are removed;
- reference-image asset claims absent from the next document are marked URL-cancelled.

Cancellation preserves the existing bounded ownership lifecycle:

- a queued or sealed claim still owned by `WorldAssetIoAuthority` enters `WorldAssetFetchOwner::begin_close` and is retired by the existing `retire_cancelled_world3d_asset_step`, one response page/string transition per grant;
- an in-flight claim keeps its exact token and reservation, reports cancellation to the host, and is forced into close when returned;
- a completed owner already checked out by the decode probe remains attached to its exact claim until terminal finish.

`world3d_reference_url_is_current` is the active-document witness. The renderer checks it immediately before applying decoded reference bytes, and `apply_reference_image_bytes` checks it again before decode/publication. A late A response therefore cannot publish A after the document switches to B, independent of the still-live pane-generation token.

The replacement law installs a valid one-pixel PNG through the existing `image` oracle, proves late A is refused, proves B is accepted, and repeats 300 distinct active URLs while the pixel registry stays at one entry without setting a dynamic-capacity fault.

## Test history and validation

Tests were added before production changes:

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🧲️scene-input-residency/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧲️scene-input-residency/🟦️.ts`

The first correctly rooted focused TypeScript run observed the intended red result: the shared schema, independent Three oracle, and residency model passed, while the WGPU source contract failed because `grid_snap_enabled` and URL ownership hooks were absent.

Focused post-repair command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache -- '🧪️tests/🧲️scene-input-residency/🟦️.ts' -t 'World scene input'
```

Observed result:

```text
Test Files  1 passed | 8 skipped (9)
Tests       3 passed | 93 skipped (96)
NX Successfully ran target test-browser-worker and 4 dependency tasks
```

Additional static checks:

- `rustfmt --edition 2021 --check` accepted the new Rust law file;
- `git diff --check` reported no whitespace errors in the packet paths.

Per root ownership, this agent did not launch Cargo, native, wasm, activation, or browser-runtime builds. The Rust laws and runtime behavior require the root-owned integrated build/test verdict; no unrun result is claimed here.

## Files owned by this packet

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🧲️scene-input-residency/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧬️schema/🧲️scene-input-residency/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🧲️scene-input-residency/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧲️scene-input-residency/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`

`🗑️generated/astra-scene-input-residency` was reserved for generated artifacts; this packet produced no durable generated artifact or tool log to retain.
