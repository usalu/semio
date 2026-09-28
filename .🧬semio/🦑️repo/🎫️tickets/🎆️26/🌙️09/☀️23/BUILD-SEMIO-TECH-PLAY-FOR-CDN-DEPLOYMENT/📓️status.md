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

## 09-28

`prepare release` reported 69 panes and 60 components. `bun ./🔨️modules/📦️site/📜️script.ts build` exited 0 in 117s (vite 51.80s, 2153 runtime files from 63 artifacts). The monolith `dist/site` is removed after packing. Upload each folder under `🏢️semio-tech/🎡️play/dist/pages`:

| folder | host | bytes |
| --- | --- | --- |
| play | play.semio-tech.com | 176474018 |
| map | map.assets.semio-tech.com | 609289505 |
| media | media.assets.semio-tech.com | 203051322 |
| modules | modules.assets.semio-tech.com | 711825652 |

Each page is under the 1_000_000_000 byte budget. `play` has `index.html` and `CNAME`. The three asset pages have `CNAME` and `_headers` with `Access-Control-Allow-Origin: *`.

A first rebuild attempt from the repo root exited 1 (`StopIteration`: the site script is not at the repo root). Rebuilt from `🏢️semio-tech/🎡️play` after CAD release artifacts updated at 15:35–15:39. Exit 0 at 15:58 (vite 4m 49s). `modules.assets.semio-tech.com` is 712457886 bytes and includes `semio_s_plugin_cad_component.core.wasm` from 15:35. All four pages remain under the budget.
