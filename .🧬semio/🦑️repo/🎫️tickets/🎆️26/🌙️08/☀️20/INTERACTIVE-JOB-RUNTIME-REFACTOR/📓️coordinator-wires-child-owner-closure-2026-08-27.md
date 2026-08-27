# Wires Child Owner Closure

## Outcome

Reasoning/Wires no longer retains live graph payloads in `WIRES_SCRATCH`. `wires_content_child_with_owner` now transfers `WiresWorkingScene` into the exact `ArtifactChild<SemioGraphSnapshot>`, and `wires_working_scene_for_handle` resolves only that typed owner.

All committed mutation fixtures were changed from process-global cache seeding to `materialize_wires_content(&mut snapshot.content, ...)`. The mutable child requirement prevents payload injection through a bare content id. Old cache APIs and names were removed without compatibility aliases.

## Language-Neutral Oracle

`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🧪️fixtures/🎯️child-owner-isolation.json` fixes the ownership observation. A test-only `serde_json` oracle verifies that an owned child has its scene, wire identity round-trips, and a reconstructed matching child does not inherit the local scene.

## Validation

- Raster, DAG, and Wires ownership fixtures parse with Bun: exit 0.
- `WIRES_SCRATCH`, `cache_wires_content`, and `wires_content_child_handle_and_cache` are absent.
- The official census reduced global payload candidates from 26 to 25; all four shared publication pipeline gates remain true.
- Rust compilation and test execution remain pending the exclusive compiler lease. The other 25 global owners and 661 fail-closed routes keep the repository gate red.
