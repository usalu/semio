# 🔎️ Inference Payload Contracts: Declaration, Pipeline and Visibility Audit

Read-only audit. No source files were modified and no git command was run.

## Verdict

- The payload contract is declared in source and carried through the whole pipeline. There is no pipeline drop for the services that declare one.
- Of the 76 declared inference services, 7 declare a payload contract in source and 69 do not. Those 69 are declared with `payload: None`, which is a source-side gap, not a loss in transit.
- The two examples from the request, `s.gis.gismap.inference` and `s.stdio.gltf.inference.centroid.v1`, both declare `payload: None` in source. Their rows correctly have no `payload`.
- The one place `payload` is removed on purpose is the roster-wide listing: `declared_page` strips it unless the call is narrowed by `artifactId` or `inferenceSchema` (see section 4).

## 1. Where a plugin declares the contract

The contract is authored in Rust, not in a JSON manifest. `🔣️.json` is the committed output descriptor, not the place where the contract is written.

- Type definition (declaration side): `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🛬️lifted/💡️inference/🦀️.rs`
  - `ArtifactInferenceServiceMetadata.payload: Option<ArtifactInferencePayloadContract>` at line 26 (field doc at lines 20-25).
  - `ArtifactInferencePayloadContract` at lines 30-39.
  - `From<ArtifactInferencePayloadContract> for semio_framework::InferencePayloadContract` at lines 63-73.
- Type definition (manifest side): `🧰️framework/🔨️modules/🛂️manifest/🧩️contribution/🦀️.rs`
  - `ContributedInferenceMetadata.payload` at lines 20-28.
  - `InferencePayloadContract` at lines 31-40 and following.
- Plugin contract constants, one per declaring service:
  - `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry/🦀️.rs` line 116 (`GEOMETRY_INFERENCE_CONTRACT`), used at line 136.
  - `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/📐️geometry-service/🦀️.rs` line 140, used at line 161.
  - `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/💡️inferences/📜️contract-and-request/🦀️.rs` line 38, used at line 58.
  - `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🔨️modules/🏠️host/💡️inferences/🦀️.rs` line 41, used at line 61.
  - `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🔨️modules/🏠️host/💡️inferences/🦀️.rs` line 54, used at line 74.
  - `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🔨️modules/🏠️host/💡️inferences/🦀️.rs` line 45, used at line 65.
  - `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🔨️modules/🏠️host/💡️inferences/🦀️.rs` line 42, used at line 943.
- Services that set `payload: None` explicitly:
  - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs` line 188, in `gltf_inference_leaf_service` (line 177), which builds all leaf services of the gltf plugin.
  - `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧠️inference-service/🦀️.rs` line 19.
  - `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🦀️.rs` line 144.
- Built-in print plugin (not in the 76): `🧰️framework/🛍️products/📓️print/🦀️.rs` line 102 sets a payload.

## 2. Which declared services have a contract (76 total)

The count was taken from the committed descriptors under `🌎️hub/🧩️compositions/*/🧩️component/🔣️.json` (74 rows) and `✏️s/🔌️plugins/*/🧩️extensions/*/🧩️component/🔣️.json` (2 rows).

| Owner | Rows | Payload in source | Payload in committed descriptor |
|---|---|---|---|
| stdio (gltf leaves) | 67 | No (`payload: None`) | No |
| gis (`s.gis.gismap.inference`) | 1 | No | No |
| wfc (bitmap, grid2d, grid3d, wfc2d, wfc3d `.solve`) | 5 | Yes | Yes (`s.wfc.*.inference.request.v1`) |
| procedural (`s.procedural.generation3d.inference`) | 1 | Yes | Yes (`s.procedural.generation3d.geometry.payload`) |
| cad-extension-aec-building (artifact contribution on `s.cad.cad`) | 1 | No | No |
| flow-extension-brep (artifact contribution on `s.flow.flow`) | 1 | Yes | Yes (`s.flow-extension-brep.geometry.payload`) |

Totals: 7 with a contract, 69 without. The 69 without a contract are 67 stdio, 1 gis and 1 cad.

Descriptor locations checked:
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🔣️.json` (67 `inferenceServices` rows, none with `payload`)
- `🌎️hub/🧩️compositions/🌍️gis/🧩️component/🔣️.json` (1 row, no `payload`)
- `🌎️hub/🧩️compositions/🀄️wfc/🧩️component/🔣️.json` (5 rows, all with `payload`)
- `🌎️hub/🧩️compositions/🌀️procedural/🧩️component/🔣️.json` (1 row, with `payload`)
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧩️component/🔣️.json` (1 `artifactContributions` inference, no `payload`)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧩️component/🔣️.json` (1 `artifactContributions` inference, with `payload`)

## 3. Is the contract dropped in the pipeline?

No. Each hop was checked, and every hop that holds a payload copies it.

1. Source constant or `payload` field, see section 1.
2. Descriptor creation: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📇️registration/🦀️.rs` lines 468-484 build `semio_framework::ContributedInferenceMetadata` with `payload: metadata.payload.map(Into::into)` (line 481).
3. Plugin describe path (wire form): `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs` lines 95-115 carry `payload: metadata.payload.map(Into::into)` (line 113). The wire type is `WireArtifactInferenceMetadata` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🔌️program/🛬️lifted/💡️inference/🦀️.rs`, `payload` at lines 47-52 and conversion at lines 115-129).
4. Committed descriptor: `🔣️.json` (`semio_framework::PackageDescriptor`) written next to each plugin. The `payload` key is present for the 7 declaring services and absent for the 69 others.
5. MCP reads the descriptor with `load_package_descriptor` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧭️plugin-paths-and-descriptor/🦀️.rs` lines 253-257). It uses serde, which keeps `payload`.
6. `DeclaredInference::from` copies `payload` at `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🔖️declared-inference/🦀️.rs` line 59.

The generated catalog is not on this path. `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/` (`🦀️catalog.json`, `🔌️plugins.json`, `🗂️sources.json`) contains no inference data at all: no `inference`, `inferenceSchema`, `inferenceServices` or `payloadSchemaId` string. It only holds plugin rows (74 entries in `🦀️catalog.json`, with `cratePath` values pointing at `🌎️hub/🧩️compositions/...` or `✏️s/🔌️plugins/...`). The MCP descriptor loader uses those `cratePath` values to find each `🔣️.json` (see section 4).

Two TypeScript checks reference the contract, and neither is part of the runtime pipeline. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts` line 258 slices the discovery source for a smoke check. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` lines 923-924 (`contractOk`) report "publishing NO payload contract" when a declared service has no contract.

## 4. Code path that loads the workspace roster

Entry points:
- `inference_list` handler: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🛠️tools-and-resources/🦀️.rs` lines 27-56. It calls `declared_inferences_for_workspace` (line 35) or `declared_inferences_for_artifact` (line 34), then `declared_page` (line 42).
- `declared_page`: same file, lines 17-25. Lines 21-23 set `row.payload = None` when `narrowed` is false. `narrowed` is `artifact_id.is_some() || service.is_some()` (line 42). This is the only strip of `payload`, and it is intentional.
- `declared_inferences_for_workspace`: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🔖️declared-inference/🦀️.rs` lines 108-118 (wrapper with roster cache keyed by catalog hash, lines 120-137).
- `compute_declared_inferences_for_workspace`: same file, lines 139-150. It calls `workspace.discovery_descriptors()` (line 140) and `declared_inferences_from_descriptor` (line 147). That function reads `descriptor.contributions.inference_services` and `artifact_contributions[].inferences` (lines 67-69). It does not read any generated catalog.
- `discovery_descriptors` (folder mode): `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🛂️gateway-backend/🏗️open-and-discovery/🦀️.rs` lines 100-117. For each catalog plugin id it calls `load_registry_plugin_descriptor`.
- `load_registry_plugin_descriptor`: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧭️plugin-paths-and-descriptor/🦀️.rs` lines 241-246. It first tries the retained component model (`resolve_plugin_model_root`, lines 213-219, which needs an environment variable). On `PluginUnavailable` it falls back to `load_package_descriptor(&entry.owner_root)`.
- `owner_root` is `cratePath` minus two components (`plugin_registry_entries`, lines 48-62 of the same file), so the descriptor is `<cratePath>/../../🔣️.json`. For `cratePath` `🌎️hub/🧩️compositions/🀄️wfc/🧩️component/📦️packages/🦀️rust` that is `🌎️hub/🧩️compositions/🀄️wfc/🧩️component/🔣️.json`.
- Extension rows: `contributed_inference_descriptors` (`declared-inference/🦀️.rs` lines 80-91) adds installed packages that contribute an inference onto an owned kind. This is how `flow-extension-brep` and `cad-extension-aec-building` appear.

Hub mode does not use these files. `discovery_descriptors` (lines 102-106 of `open-and-discovery/🦀️.rs`) takes `verified_hub_catalog_selections()` descriptors instead. This audit did not inspect the hub selection path.

## 5. Caveats

- The committed `🔣️.json` descriptors are build outputs. This audit did not trace the generator. It only confirmed that the payload-bearing descriptors match the source declarations.
- The gltf rows share one construction site (`gltf_inference_leaf_service`, line 177). If that function was ever made to take a contract, all 67 would change together.
- The retained model path (`resolve_plugin_model_root`) was not exercised at runtime. Only the committed-descriptor fallback was verified statically.

## 6. What is missing, and what would fix it

The gap is in source, not in the pipeline. To make `inference_run` work for the 69 services without a contract, the owner must author an `ArtifactInferencePayloadContract` next to each service (gltf leaves, gis, and the aec-building extension). Then rebuild the descriptors.
