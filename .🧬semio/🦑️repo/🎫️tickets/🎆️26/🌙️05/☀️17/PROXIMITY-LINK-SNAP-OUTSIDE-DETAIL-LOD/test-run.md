# Test run

- `cargo test overview_lod` in `elements/client/lib/board/rs`: 2 tests OK (overview proximity/indirect link + resolve_hit).
- Root cause: `nearest_link_snap_handle_world` returned no snap targets outside detail/micro LOD, so at Nakagin default zoom (overview band) pointer-up never got `target_id` for `proximityConnect`; node-body path could still use `indirectConnect` when sole compatible handle exists.
