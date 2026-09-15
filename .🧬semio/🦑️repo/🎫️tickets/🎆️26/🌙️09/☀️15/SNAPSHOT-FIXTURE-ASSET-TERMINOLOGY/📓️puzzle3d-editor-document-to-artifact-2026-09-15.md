# Puzzle3d Editor — Document To Artifact Symbols — 2026-09-15

Renamed domain-facing `document_*` identifiers in the puzzle3d editor where they denoted persisted artifact state (outliner memo, session slot keys, artifact-intent actions, retained-work progress copy).

## Renamed (representative)

| Before | After |
| --- | --- |
| `puzzle3d_action_document_intent` | `puzzle3d_action_artifact_intent` |
| `document_tree` / `document_tree_cache` / `document_tree_cached*` | `artifact_tree` / `artifact_tree_cache` / `artifact_tree_cached*` |
| `Puzzle3dDocumentTreeKey` | `Puzzle3dArtifactTreeKey` |
| `PUZZLE3D_DOCUMENT_TREE_BUILDS` | `PUZZLE3D_ARTIFACT_TREE_BUILDS` |
| Session `document_id` | `artifact_id` |
| Progress EN/DE | "Reading the artifact" / "Artefakt wird gelesen" |

## Intentionally unchanged

- Framework fields: `operation.parent_document_id`, `request.parent_document_id`.
- `store::pack_rt::encode_document` pack encode path.
- `puzzle3d_operations_from_host_document_change` and other `host_document` bridge names (separate host-document vocabulary).

## Verification

`cargo test -p semio-s-artifact-puzzle-3d`: **285 passed**, 1 failed (`wire_format_guard::engine_command_rows_keep_their_pre_migration_wire_bytes` — binary wire drift, unrelated to this rename). Compile was blocked by duplicate `artifact_schema` fields in `stdio` gltf inference metadata; removed the stray duplicate lines so the crate graph builds.
