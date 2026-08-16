# GLTF Material, Texture, Skin, Animation, and Camera Mutation Census

## Authority And Scope

The authoritative command matrix is [w1-gltf-authoritative-mutation-matrix.md](./w1-gltf-authoritative-mutation-matrix.md). This writer owns exactly the following canonical `s.stdio.gltf.mutation.<slug>.v1` command families:

| Family | Required leaves |
| --- | ---: |
| material, texture, image, sampler | 43 |
| skin, animation, camera | 40 |
| total | 83 |

Each command is a physical directory in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/`, with command-owned `🦠️mutation`, `🔺️diff`, and `↩️inverse` facets. Every facet has Rust, TypeScript, JSON Schema, GraphQL, and Proto representations.

## Boundary

This writer does not alter mutation root dispatch, glue, transports, artifact-definition rows, or inference. Command leaves use typed fields only; no payload JSON, opaque options, whole-record replacement, aliases, generic `Set*` operations, or legacy tags are introduced. Each mutation validates its addressed index/reference before a sparse update; a diff and inverse are derived directly from the changed field and its prior value.

## Initial Repository Census

At the start of this lane, there were no physical material, texture, image, sampler, skin, animation, or camera command directories. The snapshot types establish the typed command field vocabulary: `GltfMaterial`, `GltfTexture`, `GltfImage`, `GltfSampler`, `GltfSkin`, `GltfAnimation`, and `GltfCamera` in `🧬️schema/📸️snapshot/🦀️component.rs`.

Existing top-level command leaves use a `base.clone()` return shape. That shape is not reused by this lane: the new command functions mutate the addressed sparse field through a typed, validated target and derive their `GltfDiff`/inverse without using a whole-snapshot intermediate.

## Exact Assigned Leaf Inventory

Material (24): `enable-material-pbr-metallic-roughness`, `disable-material-pbr-metallic-roughness`, `change-material-pbr-factors`, `change-material-pbr-extension-data`, `change-material-pbr-extra-data`, `bind-material-base-color-texture`, `unbind-material-base-color-texture`, `bind-material-metallic-roughness-texture`, `unbind-material-metallic-roughness-texture`, `bind-material-normal-texture`, `unbind-material-normal-texture`, `change-material-normal-texture-scale`, `bind-material-occlusion-texture`, `unbind-material-occlusion-texture`, `change-material-occlusion-texture-strength`, `bind-material-emissive-texture`, `unbind-material-emissive-texture`, `change-material-emissive-factor`, `change-material-alpha-mode`, `change-material-alpha-cutoff`, `change-material-double-sided`, `change-material-name`, `change-material-extension-data`, `change-material-extra-data`.

Texture/image/sampler (19): `bind-texture-sampler`, `unbind-texture-sampler`, `bind-texture-image`, `unbind-texture-image`, `change-texture-name`, `change-texture-extension-data`, `change-texture-extra-data`, `bind-image-uri`, `bind-image-buffer-view`, `unbind-image-source`, `change-image-mime-type`, `change-image-name`, `change-image-extension-data`, `change-image-extra-data`, `change-sampler-filters`, `change-sampler-wrapping`, `change-sampler-name`, `change-sampler-extension-data`, `change-sampler-extra-data`.

Skins (11): `bind-skin-inverse-bind-matrices`, `unbind-skin-inverse-bind-matrices`, `bind-skin-skeleton`, `unbind-skin-skeleton`, `bind-skin-joint`, `unbind-skin-joint`, `move-skin-joint`, `reorder-skin-joints`, `change-skin-name`, `change-skin-extension-data`, `change-skin-extra-data`.

Animations (23): `create-animation-sampler`, `delete-animation-sampler`, `move-animation-sampler`, `reorder-animation-samplers`, `change-animation-sampler-input`, `change-animation-sampler-output`, `change-animation-sampler-interpolation`, `change-animation-sampler-extension-data`, `change-animation-sampler-extra-data`, `create-animation-channel`, `delete-animation-channel`, `move-animation-channel`, `reorder-animation-channels`, `bind-animation-channel-sampler`, `bind-animation-channel-target-node`, `unbind-animation-channel-target-node`, `change-animation-channel-target-path`, `change-animation-channel-target-extension-data`, `change-animation-channel-target-extra-data`, `change-animation-channel-extension-data`, `change-animation-channel-extra-data`, `change-animation-name`, `change-animation-extension-data`, `change-animation-extra-data`.

The literal domain coverage is authoritative. Animation has 24 leaves, so skin/animation/camera has 40 and this writer's total is 83. The parent writer is correcting the matrix arithmetic from 221 to 222 command leaves. Camera adds five leaves: `change-camera-perspective-projection`, `change-camera-orthographic-projection`, `change-camera-name`, `change-camera-extension-data`, and `change-camera-extra-data`.

## Verified First Leaf

`change-material-alpha-mode` is the first fully faceted command in this shard. Its mutation, diff, and inverse facets each model the exact `material: usize` / `alphaMode: GltfAlphaMode` shape across Rust, TypeScript, JSON Schema, GraphQL, and Proto. It validates the material index, rejects no-op input, mutates only `document/materials/{material}/alphaMode`, derives a typed sparse diff without `between`, and reconstructs/applies the prior enum value. Its Rust tests cover the identity rejection and forward/inverse law.

Static verification completed:

- all fifteen required facet files exist;
- all three JSON schemas parse with Bun;
- no `base.clone`, `payload_json`, generic `Set*`, or `between` dependency occurs in the leaf.

The leaf also owns `🧪️contract/🔣️component.json`: an `OPAQUE → MASK` canonical vector with the exact forward diff, inverse, and concrete `document/materials/0/alphaMode` touched path. Rust decodes that same vector in its contract test; TypeScript imports the same JSON vector as its typed diff/inverse contract. Bun verified the vector's identity, typed enum values, and path directly. The diff facet has an explicit direct-forward/apply law in addition to the mutation identity and inverse laws. The descriptor alone carries the `{material}` pattern; executable mutation/diff/inverse surfaces derive a concrete addressed path.

`change-material-double-sided` now meets the same acceptance boundary for its concrete `document/materials/0/doubleSided` path. Accepted physical leaves: 2/83. No incomplete or descriptor-only directories are counted.
