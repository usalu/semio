# Native135 Retained Engine Hit Provenance Fixture Repair

## Failure classification

Native135 failed `retained_engine_hit_provenance_reaches_each_dedicated_pointer_and_wheel_route` because the law fabricated a `ScenePointerTarget` from a temporary arena, used the authored wire surface id as its component host id, and pushed it directly into Shell staging. Production correctly refused that owner: it did not exist in the accepted retained UI tree, so its node, document id, component generation, host id, and window generation could not pass `scene_pointer_target_is_live`.

The language-neutral `SurfaceMirror` law remains the state-retention oracle. It still validates creation, no-repaint retention, refreshed geometry/controller projection, close removal, and cross-kind parity from the shared JSON fixture.

## Corrected retained route

The dedicated pointer/wheel law now selects the shared fixture's cross-kind case and constructs real typed NodeGraph, TiledMap, and Board2d scene documents. It runs each through:

1. `panel_ui_records` and `publish_surface_records`;
2. `render_ui_document_step` via the existing Shell input fixture helper;
3. retained body hit registration;
4. input candidate seal and presenter acknowledgement;
5. `sync_engine_surface_states`, consuming the production engine registration under the mounted component host id.

The assertions resolve the exact accepted `ScenePointerTarget`, require a dedicated engine state map keyed by its generated `host_id`, and then exercise Shell's physical pointer-owner, wheel-owner, and generic-scroll-yield routes. Replacing only the accepted hit's window owner with an unrelated window must still refuse routing as Chrome. No liveness check or capacity was relaxed.

The existing `paint_component_pointer_documents` helper is now `pub(super)` so this sibling Shell law uses the same accepted document pipeline as the established World laws.

## Verification boundary

The Find fixture compile blockers found by Native136 were also corrected: `UiComponentSceneNode` is fully qualified, and the typed NodeGraph fixtures deserialize the canonical viewport wire rather than naming a non-exported `Viewport2d` type.

All touched Rust sources parse with `rustfmt --edition 2021 --emit stdout`. Root owns the native build lane. Required filters:

- `test(retained_engine_hit_provenance_reaches_each_dedicated_pointer_and_wheel_route)`
- `test(find_publishes_filters_and_activates_with_a_physical_row_click)`

No native pass is claimed before root's next execution.
