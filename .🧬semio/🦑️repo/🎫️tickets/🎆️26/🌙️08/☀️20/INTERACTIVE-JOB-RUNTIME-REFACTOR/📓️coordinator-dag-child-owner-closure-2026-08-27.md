# DAG Child Owner Closure

## Outcome

The DAG working graph is now retained by the exact `ArtifactChild<SemioGraphSnapshot>` that owns it. `DAG_SCRATCH`, `cache_dag_content`, and the `dag_content_child_handle_and_cache` API were removed without compatibility aliases.

`dag_content_child_with_owner` mints the deterministic wire identity and transfers `DagWorkingScene` into the child-local owner. `dag_working_scene_for_handle` retains only that typed owner and fails soft for a wire-only child awaiting host materialization. Test adapters now attach their scene directly to the snapshot child instead of mutating shared state.

## Language-Neutral Oracle

`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🧪️fixtures/🎯️child-owner-isolation.json` declares the exact observation: the owned child has a scene, a third-party `serde_json` wire roundtrip preserves identity, and the reconstructed wire-only child has no local scene.

## Validation

- Both DAG and Raster owner fixtures parse with Bun: exit 0.
- Old cache symbols and terminology are absent from DAG production sources.
- The official tool-job census reduced process-global payload candidates from 27 to 26 while retaining all four shared publication pipeline gates.
- The repository remains red for the other declared route/global/importer blockers.
- Rust compilation and test execution remain queued behind the exclusive compiler lease; no runtime-green claim is made here.
