# Procedural3D Surface Renderer Audit

Read-only source inspection on 2026-10-03. No source edits, Git mutations, runtime executions, or test pass claims. Root owns ticket lifecycle. Applicable root, `✏️s`, and framework products AGENTS instructions were read.

## React Existing Owner

All paths below are relative to `/Users/ueli/Documents/semio`.

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` owns the complete inline mesh path:

- Lines 152–166 `WorldMeshData` only declares flat positions/normals/indices/RGB3/UV2 and picking fields, plus one PNG `paintTextureBase64`. It lacks canonical attributes/materials/textures declarations. Its native-color unsupported comment is stale: the current native upload reads RGBA4.
- Lines 1369–1470 parse/residency uses JSON.parse and preserves unknown runtime object fields; it does not reconstruct or strip metadata. Widen the existing contract rather than introducing a parser/cache. Unchanged complete wire element text reuses the same record object, including metadata. Metadata changes therefore already invalidate the existing record boundary.
- Line 2104 `geometryFromMesh` consumes only flat channels, binds `color` with itemSize 3, computes normals when flat normal count differs, and has no canonical domain/index sampling or material groups.
- Lines 2140–2165 `MeshVisuals`/`buildMeshVisuals`/`disposeMeshVisuals` own geometry, border, vertex-pick and edge buffers. Extend this owner for authored materials/textures, if necessary. Do not add a second residency subsystem.
- Line 2268 `PaintTexturedMesh` creates one palette material, one loaded paint PNG, unconditional DoubleSide, metalness 0, roughness 1; transparency follows only palette opacity. Neutral and disabled preserve vertex color via white material; selected/hover styles intentionally use palette override. This is the inline material consumption point.
- Line 2442 `GlbInstanceMesh` clones source scene and replaces every primitive material for every revision, including neutral; source material arrays/textures/alpha/emissive/double-sided are lost. Preserve/clone authored primitive materials for neutral/disabled and apply palette override for active styles. Dispose clone-owned replacement and border materials on scene retirement; currently this function has no disposal effect. Never dispose loader-owned shared geometries/textures.
- Lines 3550–3590 retain per-ID visuals by record identity and dispose retired buffers after commit and on unmount. Preserve this law for new assets. Existing texture useLoader ownership cannot simply be mixed with individually disposed textures.

The direct `PaintTexturedMesh` call at line 3190 receives geometry/style/paint texture only; `WorldInstanceMesh` props at 3073/3110 and its call around 3938 must carry the existing MeshVisuals/authored material data. Preserve index-to-face picking correspondence while adding draw groups.

## Canonical Contract and Minimal Consumption

`🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs` owns `MeshAttribute` (line 44), `MeshTexture` (line 64), polygon source schema and validation, and `MeshData` (line 304). Metadata is retained in JSON leaf conversion at 366–370 and FromValue at 420–421. `🧰️framework/🛍️products/💻️os/🟦️.ts` MeshPack declares attributes/materials/textures at 2451–2453 and decodes a metadata body at 2538–2539. Do not flatten away these fields in an intermediate visual normalization.

`validate_mesh_surface_assets` line 123 validates baseColor RGBA, metallic/roughness/alphaCutoff, emissive RGB, alphaMode OPAQUE/MASK/BLEND, doubleSided and owned texture references. It permits arbitrary other names and interprets any name ending Texture as an owned ID; renderer must explicitly consume supported names, avoiding external-URL fallbacks. Domain material is Face only; normal/UV/color values are independently indexed. Materials are owned DslValue objects rather than a distinct external runtime API.

`🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs` tessellation cursor at 1533 onward emits flat shaded samples and retained metadata. `advance_attributes` at 1655 remaps canonical indices into transferred domains and moves materials/textures. Renderer-side sampling must use the transferred domain indices, not original topology vertex IDs. Corner and Face channels can require corner-expanded GPU positions; duplicate only the draw vertices needed to represent discontinuities, retain original pick IDs, and preserve triangle ordering. Sample `indices[domainIndex]` when indices exist; otherwise sample the domain index. Preserve all custom metadata untouched. Existing tessellation cursor owns progress/cancellation; no separate evaluation worker is needed.

Use BufferAttribute itemSize 4 for canonical RGBA. Map authored material baseColor, metallic, roughness, emissive, alphaMode/alphaCutoff/doubleSided onto existing MeshStandardMaterial. Assign each triangle a group from its transferred Face material ID. Five texture refs need explicit mapping: baseColorTexture → map, metallicRoughnessTexture → metalnessMap/roughnessMap, normalTexture → normalMap, occlusionTexture → aoMap, emissiveTexture → emissiveMap. Resolve only mesh-owned texture IDs and MIME bytes. Account for base/emissive sRGB versus linear numeric maps and glTF UV orientation. AO requires the expected UV channel; confirm installed Three behavior in tests before choosing uv1 duplication. Do not infer texture alpha as BLEND when alphaMode says OPAQUE/MASK.

## Native Surface Expectations

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs` line 659 `ensure_mesh_step` owns incremental GPU upload and reads Positions, Normals, Colors vec4 and Uvs vec2 into `World3dVertex` (around 688). It guards key/version/lease identity, uploading one vertex/index per step. This is already the native RGBA/UV owner.

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs` line 1360 accounts for lease channels including colors*16 and UV*8. Generic metadata/owned texture payload costs must join whichever existing preparation owner carries them; they are absent from this byte calculation.

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` has existing GLB schema/material/texture cursors, color emission at 2847, texture lease collection at 2589/2721, and material draw construction around 2999. Extend the same authored appearance owner for inline MeshData. Native draw consumption inspected at 4565/4618 currently mentions base_color_texture; all-five-map support is not established by this audit. Do not claim parity from mesh metadata retention alone.

`mesh_content_version` at `🖍️draw/🏷️types/🦀️.rs:1378` hashes only positions/normals/indices. Before reusing it for authored inline appearance, include all rendered channel/material/texture content through the existing version owner; otherwise appearance-only changes could keep stale GPU assets.

## Verification to Run

Existing registered targets: `bun nx run @semio-tech/framework-renderer-react:test`, `:typecheck`, `:world3d-interaction-check`, `:scene-shading-pixel-check`, and `:surface-retention-check`. They are declared in the React package `📋️project.json` and routed through its `📜️script.ts`. Inspect script-supported filtering before running a narrow unit subset.

Existing Procedural3D checks in `.vscode/launch.json`: `bun nx run @semio-tech/procedural-generation3d:test --skip-nx-cache`, `bun nx run @semio-tech/procedural-generation3d-rs:test-snapshot-sqlite-native --skip-nx-cache`, semantic-wire-check (source and native), document IO and preview-window-transient verification. Registered native app is `bun nx run @semio-tech/framework-renderer-wgpu:native -- generation3d`; React/WGPU browser launchers set `SEMIO_APP=s.procedural.generation3d@1/*#editor`.

Existing test oracles: mesh-engine Cargo dev dependencies include gltf 1.4.1, serde and serde_json; existing `gltf-oracle-differential`, `mesh-data-json-oracle`, and `mesh-data-from-value-round-trip` tests are adjacent. Existing Three GLTFLoader and BufferGeometry in React are suitable third-party oracles without adding runtime deps. Playwright is already declared. Add one language-agnostic fixture for seams/indexed Vertex-Corner-Face channels, RGBA alpha, two Face materials, owned five-map references, and alpha/emissive/culling properties. Compare decoded/expanded output to installed Three GLTFLoader plus existing gltf oracle. Verify unchanged record identity, metadata-only replacement, disposal once on replacement/removal/unmount, cancellation/stale decode disposal, and no changing unaffected mesh identity.

Browser acceptance must render the existing host and use visible pixels plus explicit temporary `[DEBUG]` lifecycle/material diagnostics. Test seam normals/UV, different face materials, alpha MASK/BLEND/OPAQUE, emissive and double-sided culling, selection/deselection restoring authored appearance, shared texture ownership, and source updates/cancellation. This audit did not execute those checks and provides no runtime evidence.
