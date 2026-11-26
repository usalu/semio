# 🔎️ Inference Payload Contracts: Fix

## Finding

The 69 services without a contract (67 stdio glTF leaves, `s.gis.gismap.inference`, cad `aec-building`) do not take a JSON body at all. Each guest decodes `request.canonical_payload` as the artifact's raw pack (`ArtifactPack::decode_pack`). The gateway never produced that: with no binding it sent `{}` (or the caller's JSON), so these services could not run even with a contract of the existing JSON kind. The glTF guests also returned a custom binary frame and GIS/AEC returned pack wire values, all of which the gateway reported as `payload: null`.

No service genuinely takes no payload; all three families need the named artifact. Their caller-visible payload is empty and the contract says so (`maxProperties: 0`).

## Design

- New binding encoding `artifact-pack-bytes` (`semio_framework::INFERENCE_ARTIFACT_PACK_BYTES`): the artifact's canonical pack bytes are the whole request body, binding `field` is empty, `required: true`.
- `ArtifactInferencePayloadContract::pack_bound(payload_schema_id, output_schema, progress_unit)` builds that contract with the shared schema `📜️pack-bound-input.json` (empty-object input).
- Gateway: `bind_inference_document` writes the pack for this encoding; `validate_inference_request` refuses a caller body and a missing `artifactId` by field name; `inference_run_result_value` projects non-JSON results that are pack wire values to JSON.
- glTF leaves now return canonical JSON (`canonical_json_bytes`, now `pub`) instead of the binary frame, so the result is readable.

## Contracts declared

| Service | payloadSchemaId | Output schema | Unit |
|---|---|---|---|
| 67 glTF leaves | `s.stdio.gltf.inference.leaf.payload.v1` (one shared `GLTF_LEAF_INFERENCE_CONTRACT`) | `🗿️artifacts/🧊️gltf/📤️leaf-result.json` | leaf |
| `s.gis.gismap.inference` | `s.gis.gismap.inference.payload.v1` | `gismap/🧠️inference-service/📤️result.json` | feature |
| cad `aec-building` summary | `s.cad-extension-aec-building.structure-summary.payload.v1` | `🧬️schema/📤️building-structure-summary.json` | document |

The glTF output schema types the envelope exactly and the leaf `value` as a `GltfMeasure`; the measure's own `value` type differs per leaf (number, integer, vec3, bounds, statistics, ...) and is not narrowed per leaf.

## Conformance

`every_declared_inference_publishes_a_payload_contract` (mcp workspace quick tests) walks every committed descriptor and fails by name for any declared inference without a parseable contract. `a_pack_bytes_binding_writes_the_artifact_pack_as_the_whole_body` covers the binding. glTF unit test `every_leaf_publishes_the_pack_bound_contract` covers the 67 leaves.

## Caveats

- Hub-site GIS execution is unchanged (raw pack through `infer_gis_map_controlled`).
- The bound pack is the `pack` half of the canonical pair; `spr` is not replayed by these guests (pre-existing behaviour of the guests).
- The stale glTF unit test signature (`service.infer(&request)`) was left as found apart from the result decode.
