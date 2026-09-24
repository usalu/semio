# 🚀️ Build Semio Tech Play For CDN Deployment — Status

## Target

`bun nx run @semio-tech/semio-tech-play:build` → `🏢️semio-tech/🎡️play/dist/site` (static, `base: "./"`, CNAME from
`PLAY_HOST`, GIS tiles bundled via `GIS_MAP_TILE_SERVE_MODE=bundle`). Logs live outside `🗑️generated` (external sweeps)
in `.🧬semio/🦑️repo/⚡️cache/play-fleet/cdn-build/build-<n>.log`.

## Timeline

- 09-23 22:37 build-1: full graph (190 tasks, 0 % cache). 372 min. 8 components failed: six `component-release`
  (mathematical, reasoning, dag, trinity, stdio, animate) on a peer's in-flight rename in `semio-framework-plugin`
  (`complete_reserved_spawned_job_inner` gone, 04:28), two `materialize-release` (puzzle, sourcing) cut by SIGINT
  (nx exit 130) at the end of the run.
- 09-24 04:51 build-2: every hash invalidated by the peer edit → full rebuild again; at 06:25 a second peer edit broke
  `semio-framework-os-kernel` (`crate::mounted_pack_session` inside nested `store` module; peer fixed to `super::` at
  06:30). Killed build-2 (`📜️kill-tree.py`).
- Insight: with peers continuously editing the framework, the full nx graph (~6 h) never converges. The release
  path has no cross-lane receipt merge (`prepare release` only checks each component's `.nx-artifact.json`), so
  each component is self-consistent and only missing components need building.
- 06:31 build-3: `nx run-many -t materialize-release` for the 8 missing components only.

## Next

1. Verify all 60 components complete (`prepare release` logic).
2. `nx run @semio-tech/semio-tech-play:build --excludeTaskDependencies` → `dist/site`.
3. Serve `dist/site` statically (no dev server), boot panes in a browser, check console/network for 404s.
