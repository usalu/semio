# Surface Renderer Fidelity Execution

## Ownership

Existing `World3dHost` inline buffer/material residency and GLB clone styling; Infinite `WorldMeshBuffers`/placeholder publication; native UI scene authored appearance, raster ownership, and draw shaders/pipelines; native GLB schema/materialization. No additional evaluator, cache, document, renderer subsystem, or parallel mesh model. Native WorldMeshBuffers now aliases canonical MeshData. Shared MeshData retirement follows the existing typed value retirement owner.

## Schema and Fixture

`renderer/engine/schema/world3d-inline-surface` and the matching neutral fixture declare indexed Corner normals/UVs, Face RGBA/materials, shared sample aliases, five owned texture roles, opaque/blended culling, and original triangle order. Installed Three BufferGeometry/Material is the differential oracle. The existing GLB fixture now declares all five roles and two source image identities, with installed GLTFLoader verifying both color spaces. A loader source texture reused in color and numeric roles aliases its colorSpace; the owned neutral GLB clone now separates role texture objects while retaining loader-owned images and preserving source resource ownership.

## Executed Receipts

- React inline source fixture: 6/6 passed initially.
- React surface/residency: 25/25 passed after replacing invalid toy geometry in the existing residency fixture with real triangles and preserving distinct slider payloads.
- React package typecheck: passed before the final GLB role clone edit; fresh replay pending.
- Actual native GPU context creates every pipeline: 1/1 passed, 770 skipped, Nextest 0.654s; new five-map shader and all four authored mapped alpha/culling pipelines compiled and were created on the actual adapter.
- Native inline fixture law passed1/1,509filtered after peer test call repairs; actual runtime witness drawVertices6/materialGroups2/fiveMaps5/roleLeases2/work854 and cancellation across all observed publication phases. This preceded the latest image preparation/digest refinements, which require one fresh replay.
- The first expanded GLTFLoader oracle failed: numeric maps reused the same texture object as the color map and therefore inherited sRGB. The fixture separates numeric/color image identities; production role cloning independently corrects same-source aliasing. The corrected installed GLTFLoader/React suite passed8/8; the later stale-callback and invalid-coefficient additions passed10/10 (react-surface-lifecycle.txt).

Logs remain under `generated/surface-renderer-2026-10-03` while the ticket remains active.

## Native Contract

SceneAuthoredMaterial3d carries baseColor, metallicRoughness, normal, occlusion, and emissive texture references/samplers. Numeric raster textures upload as Rgba8Unorm; color/emissive upload as Rgba8UnormSrgb. Raster keep and byte measurement walk every role. The raster pool slot roster expands to16 while preserving the256MiB aggregate byte ceiling and64MiB per-item ceiling. GPU material shader uses packed G/B roughness/metallic channels, derivative tangent basis for normal maps, indirect-light occlusion, sampled emissive, post-map alpha masking, opaque alpha override, and front/back authored normal orientation.

Inline publication retains the same private mesh writer, copies Vertex/Corner/Face shading samples and original IDs/edge buffers, prepares appearance before atomic publication, and retires canonical source metadata incrementally before handing off the lease. Cancellation uses the existing source retirement and mesh authority close ladders.

## Remaining Work

Fresh React typecheck/oracles, native inline and GLB runtime logs, actual shaded five-map pixel comparison, decoder/source preparation budget audit, and broad lifecycle disposal verification remain outstanding. Native GLB explicitly refuses nonzero texture UV selection and nondefault normal scale/occlusion strength until those fields are represented by the native appearance contract. Inline native texture preparation currently supports PNG; JPEG preparation remains to be routed through the existing owned image worker boundary. These limitations must be resolved or reported honestly; full native parity is not yet claimed.

## Latest Preparation and Verification

Inline and native GLB image preparation now copy/hash encoded bytes in bounded blocks, advance the existing first-party PNG job in bounded decode slices, and route JPEG through the existing renderer/process worker queue using the same exact-pixel image backend. Cancellation waits for an already-submitted codec result and retires its pixels instead of publishing a stale image. Shared canonical source metadata retires incrementally; inline residency identity now hashes each geometry item and streams metadata through the existing MeshMetadataCursor without cloning a shadow buffer set. The parse phase skips redundant flat-normal computation when authored Normal attributes are present.

The native GLB replay reached the broad renderer test package but found36 existing JSON admission/namespace errors. The owned package now admits the existing pack-json dependency; the nine compiler-reached dsl::json calls in seven existing renderer consumers/tests were narrowly moved to that owner. The next two launches stopped at missing Nx cached graph; root confirmed disk exhaustion, not an owned Rust error. Root cache cleanup recovered space; one fresh native GLB replay is running.

The registered `scene-shading-pixel-check -- surface` command now validates eight neutral pixel cases (five maps, normal/AO effects, alpha masking/blending and front/back culling) against installed Three using the actual production React material constructor and native Rust-emitted WGSL. Its native shader export proof passed again1/1 with actual pipeline creation. Initial browser preparation hit unsupported IIFE top-level await, then an existing Vite worker-url import; the oracle now executes the two actual constructor declarations extracted by TypeScript AST from the production owner, preserving their source and avoiding unrelated application bootstrapping. Actual pixel results remain pending, so no full parity claim is made.

Root authorized completion of arbitrary selected UV sets plus nondefault normalScale/occlusionStrength within the same native mesh authority/appearance/instance owner. Proposed canonical material extras: textureCoordinates keyed by the existing five owned texture-reference field names, plus finite normalScale and [0,1] occlusionStrength. Native role UV streams will select from canonical UV attributes without creating another mesh model. The native shared contract and matching fixtures still require implementation and execution.

## Sampler, Mip, Tangent and Selected-UV Execution

The canonical material extras admit per-role `textureSamplers`, arbitrary selected texture UV sets, finite normal scale and bounded occlusion strength. Native authored mesh publication uses the existing Mesh3dSchema role streams and TANGENT field; the same SceneAuthoredMaterial3d retains five independent sampler descriptors. The production draw has fifteen vertex attributes, packing its five role UV sets into the existing GPU vertex owner. GLB authored TANGENT uses normalScale `[s,s]`; derivative fallback uses `[s,-s]`, matching installed GLTFLoader.

The existing RasterTextureUploadCursor now generates the full resident mesh texture mip chain without a second image or CPU pixel copy. Its same texture receives one render-pass slice of at most eight output rows per unit, advancing mip and row scalars. Numeric and sRGB targets use their respective existing GPU formats. RasterTextureTable prepares and reserves the full chain byte count before texture allocation, while engine Canvas callers explicitly reserve one level. SceneRasterDescriptor.gpu_byte_len makes the existing scene pool GPU ACK, replacement and retirement account for exactly the same full chain.

Executed native actual adapter pipeline law passed1/1,770skipped, Nextest0.193s. Production WGSL export includes both the authored surface shader and the actual mip-generation shader. Actual Chromium production shader versus installed Three/production React material constructor passed14/14 with identical RGBA8, including five maps, normal/AO coefficients, selected normal UV1, front/back culling, alpha modes, authored tangent/front/back and filtered mip sampling. The prior authoredMipLinear witness was red without generated mips (`147,187,79,255` versus `181,157,121,255`); current production mip owner emits exactly `181,157,121,255`.

The same-owner native GPU runtime law `authored_mip_upload_bounds_each_slice_and_cancellation_retires_the_same_owner` passed1/1,771skipped, Nextest0.093s. A32×32numeric raster reserves5460bytes across six levels. Mid-mip cancellation executes the existing upload-close owner then aborts the candidate without publishing; complete upload commits its GPU witness, releases CPU leases, and retires all5460bytes on close. DEBUG witness: canceled generation17slices; complete generation23slices; both terminal with zero resident bytes. The first replay exposed the test omitting the required existing upload-close phase before abort; correcting the harness to execute the actual owner sequence passed. No production cancellation bypass was added.

Receipt paths while active: `🗑️generated/surface-renderer-2026-10-03/mip-native.txt`, `mip-pixels.txt`, `mip-green/authored-surface-pixels/report.md`, and `mip-runtime-corrected.txt`. React sampler/status suite and fresh native GLB tangent/selected-UV publication replay are in progress. Root reported a concurrent MP3 oracle manifest admission blocker; no unchanged native retries will be made until root clears it.

Fresh React retained sampler/tangent/status/GLTFLoader suites passed14/14 across2files, Vitest12.38s (`react-sampler-long.txt`). The package's quick policy includes only its quick smoke suite; two initial attempted narrow quick invocations matched no owned tests and are not counted as assertions. The current neutral inline material fixture additionally distinguishes metallic-roughness trilinear minification from the other four role samplers; React and native owner assertions follow this distinct descriptor. A fresh verification of that final fixture update is pending.

Fresh native GLB actual publication passed1/1,1655skipped, Nextest0.048s (`glb-tangent-sampler.txt`). Its law checks mixed authored/absent tangent streams, selected UV1 andUV4role buffers, GLTFLoader-compatible normalScale signs, five material samplers, mask/blend primitive ranges, request-owned Icon mesh/raster packet transfer, and exact asset retirement.

Final distinct-sampler React fixtures passed14/14 again (`react-sampler-final.txt`). Package typecheck found one owned scalar normalScale tuple inferred as unknown elements. Routing both scalar and tuple values through the existing typed surfaceTuple fixed it; fresh package typecheck passed14.6s (`react-typecheck-corrected.txt`) and actual React suites passed14/14 again,8.43s (`react-sampler-typed.txt`).

The first final native inline replay accidentally selected the registered target's appended world/terrain union through positional filters; it stopped53passed/2failed before reaching the surface law. Unrelated current failures: pointer gesture interaction carrier retirement bound at pointer-gestures law140; scene-input-residency snap-boolean relocation expected `[1.4,-2.6,0]`, actual `[0.0326416,-0.0606201,0]`. These files were not changed by this lane. A precise Nextest `-E test(authored_inline_surface_preserves_corner_face_channels_five_maps_and_cancellation)` replay is running.

## Arbitrary Selected UV Set Proof

Native GLB schema owns `uv_sets:[Option<u16>;64]`; textureCoordinates selects0..63 per material role and rejects a missing selected accessor before publication. Native inline uv_sample dynamically resolves `uv{set}` for each primitive's selected role and refuses absent nonzero sets. Five GPU UV streams represent five material roles; they do not restrict authored source set numbers to0..4. React aliases arbitrary selected GLB TEXCOORD_n attributes and declares high-numbered inline attributes in the existing material compilation callback.

The precise inline runtime replay exposed a stale law expectation: all six normal/AO role vertices were compared against UV1 even though the blend material deliberately has no selector and uses UV0. The law now checks authored material selection on the first triangle and UV0 on the second. The shared neutral fixture now selects a distinct UV63 normal map while retaining UV1 occlusion; its actual pixel corpus includes authored-normal-uv63. React callback/cancellation expectations use the four resulting owned role/sampler/UV texture resources. Fresh portable/native/browser proof is pending.

Current execution turn changed the existing raster ownership source (`ui/🎯️targets/🧊️wgpu/🖼️raster-ownership/🦀️.rs`), existing raster residency laws (`ui/🧪️tests/🖼️raster-residency/🦀️.rs`), existing World3dHost typed material constructor, neutral inline surface fixture, corresponding React laws, and Infinite World native inline law. It also updated this retained report and created the sibling provider proof report. Existing mip shader/upload/GPU reserve/caller source changes were already present at handoff and were verified here. No permanent script, subsystem, package dependency, git mutation or worktree was added.

The final UV63 React fixture passed14/14 (`react-uv63.txt`); the selected normal channel63 and occlusion channel1 produce four distinct existing appearance texture resources, and asynchronous cancellation/disposal assertions were updated accordingly. Native UV63 execution remains queued behind shared native build ownership; no runtime success is asserted for that final witness yet.

The first final native UV63 replay reached actual assertions after shared build ownership. It passed the authored UV63/UV1 triangle comparisons, then the new test helper incorrectly looked up absent key `uv` on the blend primitive; the neutral fixture names that corner channel `uv0`. The helper was corrected to the actual fixture key. Production selection code remains unchanged, and a fresh precise replay is active (`inline-uv63-corrected.txt`). This partial red receipt is not counted as a passing native law.

## Native UV63 Publication Receipt

The corrected focused replay of `authored_inline_surface_preserves_corner_face_channels_five_maps_and_cancellation` passed1/1,516skipped,0.043s Nextest, exit0. The registered Nx command spent15m36s including shared Cargo ownership and compilation. The fixture now selects canonical vertex channel UV63 for the opaque normal map, UV1 for its occlusion map, and default corner UV0 for the second material. All six draw vertices, material groups, five texture maps, shared role leases, authored tangent signs and every publication cancellation phase are asserted. Actual runtime output reports `[DEBUG] authored inline: drawVertices=6 materialGroups=2 fiveMaps=5 roleLeases=2 work=1302; all publication phases canceled`.

The initial UV63 replay reached and passed the first triangle's UV63 assertions but failed because the new test helper used nonexistent channel key `uv` for the second triangle. Correcting that helper to the fixture's actual `uv0` key yielded the receipt above; no production UV fallback was changed. The draw owner packs five selected role streams and admits source sets0..63; those streams are not limited to source sets0..4. Captured output: `🗑️generated/surface-renderer-2026-10-03/inline-uv63-corrected.txt`.

The15case production WGSL/Three pixel replay including authored-normal-uv63 is now running sequentially, with output under `🗑️generated/surface-renderer-2026-10-03/surface-pixels-uv63.txt`. No result is asserted before completion.

## Final UV63 Pixel Receipt

The fresh registered surface pixel target exited0 after3m26s. Its current production native GPU shader/pipeline export law passed1/1 with771skipped,0.447s Nextest. Actual Chromium GPU rendering then matched the installed Three implementation and production React material constructor in all15cases, with zero difference in every RGBA8 channel. UV63 normal sampling produced `[202,186,154,255]`; filtered mip sampling produced `[181,157,121,255]`. This receipt supersedes the earlier14case snapshot and proves the current typed material source and UV63 fixture at runtime. Output remains under `🗑️generated/surface-renderer-2026-10-03/surface-pixels-uv63.txt` while the ticket is active.

The generated pixel table is retained below so future generated-output cleanup will not remove the actual comparison evidence.

# Authored Surface Pixels

Actual production native WGSL and production React material constructor against installed Three.

| Case | Native RGBA8 | Three RGBA8 | Delta |
| --- | --- | --- | --- |
| authored-unmapped | 203,188,157,255 | 203,188,157,255 | 0,0,0,0 |
| authored-five-maps | 135,36,83,255 | 135,36,83,255 | 0,0,0,0 |
| authored-normal | 195,177,143,255 | 195,177,143,255 | 0,0,0,0 |
| authored-occlusion | 188,167,131,255 | 188,167,131,255 | 0,0,0,0 |
| authored-mask-discard | 0,0,0,0 | 0,0,0,0 | 0,0,0,0 |
| authored-blend | 70,45,52,101 | 70,45,52,101 | 0,0,0,0 |
| authored-back-culled | 0,0,0,0 | 0,0,0,0 | 0,0,0,0 |
| authored-double-back | 186,165,138,255 | 186,165,138,255 | 0,0,0,0 |
| authored-scaled-normal | 200,184,152,255 | 200,184,152,255 | 0,0,0,0 |
| authored-occlusion-strength | 200,183,150,255 | 200,183,150,255 | 0,0,0,0 |
| authored-normal-uv1 | 202,186,154,255 | 202,186,154,255 | 0,0,0,0 |
| authoredTangent | 218,209,184,255 | 218,209,184,255 | 0,0,0,0 |
| authoredTangentDoubleBack | 195,178,154,255 | 195,178,154,255 | 0,0,0,0 |
| authoredMipLinear | 181,157,121,255 | 181,157,121,255 | 0,0,0,0 |
| authored-normal-uv63 | 202,186,154,255 | 202,186,154,255 | 0,0,0,0 |

## JPEG Texture Admission Audit And Authored Runtime Law

Root asked whether canonical JPEG texture assets match the current renderer owner. Native inline texture_step (world/🦀️.rs:9596) explicitly admits image/jpeg and submits the existing Maintenance worker. decode_mesh_surface_image_bytes (:16454) uses the already-installed image decoder behind the first-party RasterImage interface, validates source pixel count, applies EXIF orientation and preserves intrinsic size. Its decoded pixels enter the same SceneRasterPool prepare_moved/seal_prepared_moved path and existing GPU mip owner. PNG follows first-party incremental PngDecodeJob. React TextureLoader delegates JPEG decoding to the browser. Older Canvas paint PNG-only comments describe a separate existing paint path and do not constrain authored 3D texture admission.

Added jpegPublication to the same neutral inline-surface fixture, a Pillow-produced 2x1 RGB JPEG; the new existing native law authored_inline_jpeg_surface_publishes_owned_role_rasters drives the actual surface cursor, checks both sRGB/numeric role raster ownership and pixel equality against installed image::load_from_memory_with_format, then retires the same owners to terminal-empty. This law is authored but not yet executed. UI owner received exact source flow; no fresh JPEG runtime success is claimed.

JPEG oracle strengthened before native execution: independent Pillow decoded the existing neutral633-byte JPEG to two identical RGBA8 pixels [255,128,65,255], while authored pre-JPEG source RGB was [255,128,64]. Neutral fixture/schema now retain decodedRgba8 and the same owned-publication law checks the independent palette before leased-row comparisons. Pillow actual run succeeded; native publication/cancellation law remains unrun for this change and is delegated to root next Infinite gate. No decoder/runtime dependency added.

Current inline-surface React replay after independent JPEG schema widening: actual13/13 GREEN,exit0/Nx12.9s (`jpeg-independent-react-schema.txt`). First16pixel invocation stopped immediately on missing caller SEMIO_TEST_ARTIFACT_DIR, before native/browser work; corrected caller environment and restarted once (`alpha-cutoff-16-pixel-artifacts.txt`). Native shader preparation active; no16pixel result claimed yet.

### Current sixteen-case pixel runtime

Actual native authored GPU pipeline1/1,771skipped and ChromiumproductionWGSL versus installed Three16/16 exactRGBA8,exit0/Nx2m42s (`alpha-cutoff-16-pixel-artifacts.txt`). MASK cutoff1.5 is exact transparent [0,0,0,0], UV63, authoredTANGENT and mip sampling preserve exact prior pixels. Full retained actual table follows before generated cleanup.

# Authored Surface Pixels

Actual production native WGSL and production React material constructor against installed Three.

| Case | Native RGBA8 | Three RGBA8 | Delta |
| --- | --- | --- | --- |
| authored-unmapped | 203,188,157,255 | 203,188,157,255 | 0,0,0,0 |
| authored-five-maps | 135,36,83,255 | 135,36,83,255 | 0,0,0,0 |
| authored-normal | 195,177,143,255 | 195,177,143,255 | 0,0,0,0 |
| authored-occlusion | 188,167,131,255 | 188,167,131,255 | 0,0,0,0 |
| authored-mask-discard | 0,0,0,0 | 0,0,0,0 | 0,0,0,0 |
| authored-blend | 70,45,52,101 | 70,45,52,101 | 0,0,0,0 |
| authored-back-culled | 0,0,0,0 | 0,0,0,0 | 0,0,0,0 |
| authored-double-back | 186,165,138,255 | 186,165,138,255 | 0,0,0,0 |
| authored-scaled-normal | 200,184,152,255 | 200,184,152,255 | 0,0,0,0 |
| authored-occlusion-strength | 200,183,150,255 | 200,183,150,255 | 0,0,0,0 |
| authored-normal-uv1 | 202,186,154,255 | 202,186,154,255 | 0,0,0,0 |
| authoredTangent | 218,209,184,255 | 218,209,184,255 | 0,0,0,0 |
| authoredTangentDoubleBack | 195,178,154,255 | 195,178,154,255 | 0,0,0,0 |
| authoredMipLinear | 181,157,121,255 | 181,157,121,255 | 0,0,0,0 |
| authored-normal-uv63 | 202,186,154,255 | 202,186,154,255 | 0,0,0,0 |
| authored-mask-cutoff-above-one | 0,0,0,0 | 0,0,0,0 | 0,0,0,0 |

