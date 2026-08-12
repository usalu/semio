# Proposed (NOT applied) consumer-side follow-up for `♾️infinite`

Non-blocking sketch for whoever eventually threads `origin_lon`/`origin_lat`/`exaggeration` as
parameters instead of leaving `TerrainSessionCore::set_project_origin`/`set_exaggeration` as
setters. **Not required to land W2** — the wave 2 exemplar edit to
`🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` kept the public API unchanged specifically
so this file (owned by whoever picks up `♾️infinite`) does not need to change yet.

Two files must be edited TOGETHER — they are byte-identical duplicates today (see report,
"Duplicate consumer file" finding), same commits in `git log`, no `#[path]` link found between them:

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs`

## Sketch

In `apply_terrain_style_if_changed_state` (both files, ~line 898-900):

```diff
-        state.terrain_session.set_project_origin(style.project_origin_lon, style.project_origin_lat);
-        state.terrain_session.set_exaggeration(style.exaggeration);
+        // origin/exaggeration now threaded as call-site parameters, not stored on the session —
+        // see 🏔️terrain/🦀️component.rs's VisibleTileQuery region.
```

In `sync_terrain_state` (both files, ~line 934, ~956):

```diff
-        let visible_json = state.terrain_session.visible_terrain_tiles_json(&camera_json);
+        let visible_json = state.terrain_session.visible_terrain_tiles_json(
+            &camera_json, style.project_origin_lon, style.project_origin_lat,
+        );
...
-            let mesh_json = state.terrain_session.terrain_tile_mesh_json(z, x, y);
+            let mesh_json = state.terrain_session.terrain_tile_mesh_json(
+                z, x, y, style.project_origin_lon, style.project_origin_lat, style.exaggeration,
+            );
```

This requires `TerrainSessionCore` to drop `origin_lon`/`origin_lat`/`exaggeration` as fields
entirely (keeping only `elevation`), and `set_project_origin`/`set_exaggeration` to be deleted.

## Open question this does NOT resolve

Whether `elevation`/`upload_elevation_tile`/`terrain_tile_mesh_json`'s decode+mesh-build should
move into the host `EngineCache` (`💻️os/🔨️modules/⚙️engine`, W1-owned/frozen) as registered `Engine`
impls, or stay a plain per-consumer cache field (like `World3dState.meshes`). `EngineCache`'s
docstring scopes it to "the wasm guest↔host boundary" — whether `semio-framework-os-infinite`
(the crate that owns this consumer) crosses that boundary was not established in this wave. Whoever
picks this up should confirm that first; guessing wrong here would misplace the cache for every
sibling surface lane that copies this recipe.
