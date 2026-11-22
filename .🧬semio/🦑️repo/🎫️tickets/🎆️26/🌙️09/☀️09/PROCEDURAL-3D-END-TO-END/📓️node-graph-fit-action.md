# Node graph `fitGraph`: one shared declared action (2026-11-22)

Closes the `port-fit` gap of `📓️wgpu-journey-reds-fix.md`: `node_graph_fit_camera` had no caller and `nodeGraph.fitGraph` was a label only.

## Design
- Action id `fitGraph` (flat camelCase like `nodeGraphViewport`; `nodeGraph.fitGraph` stays the i18n label key). View action, chrome audience, palette, category `view`, chord `f`, labels en "Fit graph" / de "Graph einpassen", no arguments (a chord must run it, not stage it).
- Declared once in the framework manifest (`node_graph_fit_action_definition`, Rust; TS twin constants). The plugin builder joins it to every window kind that declares `nodeGraphViewport` (generation3d Flow window `procedural-main`, trinity graph windows) and binds `f` when free; the window-scoped keybinding resolver delivers it to the focused graph window, else raises the mounted owner. Palette and window toolbar read it from the window roster, as for every action. The MCP catalog compiles it as a framework capability.
- Shell-intercepted, never forwarded to the program (listed beside export/import in the plugin's shell-owned refusal and `is_framework_reserved_action_id`).
- Camera law: unchanged `fit_camera` / `dagFitCamera`. Unlike the opening camera, an explicit fit ignores a stored camera.
- wgpu: `node_graph_fit_action` (EngineCanvas node-graph) fits via `node_graph_fit_camera`, records the viewport fingerprint and returns the `nodeGraphViewport` action the shell dispatches (`framework_fit_node_graph` in the dispatch funnel; refusal notice en/de). Persistence is identical to a wheel zoom.
- React: `🕸️NodeGraph/🎯️fit-graph` registry (the flow canvas registers a runner per surface in `useFlowSessionAttach`); the input funnel handler `🔲️fit-graph` finds the graph surface in the target window body, runs it (setCamera + `nodeGraphViewport` dispatch) or refuses with a localized notice.

## Tests (language-agnostic fixture: `🖼️canvas/🧪️tests/🧫️fixtures/📷️camera-fit/🔣️.json`, `fitAction` + `fitRows`)
- Independent oracle (Python, no Rust/TS): `node-graph-fit-oracle.py` — 9/9 rows agree (run, passed).
- Rust: canvas `every_fit_row_installs_the_camera_the_explicit_fit_names`; manifest `node_graph_fit_action_is_declared_as_the_shared_fixture_names_it`; wgpu `fit_graph_action_persists_the_fitted_camera_like_a_wheel_zoom`.
- TS: `renderer-twins` block "node-graph fitGraph action".
- NOT RUN: the repo build tooling was unusable during this lane (project-graph slice workers failing on concurrently edited `build/` sources; `repo cargo` ENOENT on `remote-execution/sidecar`). Nothing compiled; expect first-compile fixes.

## Known limits / follow-ups
- Apps whose `nodeGraphViewport` is app-roster only (flow, dag plugins) get no per-window join; give their graph window an `action_refs` entry.
- React non-flow `WasmGraphSurface` registers no runner (refuses with the notice).
- Descriptor snapshot fixtures of generation3d/trinity (they embed `importArtifactDocument` rows) may need `fitGraph` rows.
- Browser: battery `port-fit` and React probe must click the Fit control / press `f` in the Flow window; wgpu logs `nodeGraphViewport` and the camera changes.

## Verification pass (2026-11-22)
Workspace routing: canvas/manifest crates belong to the framework workspace (`--manifest-path 🧰️framework/Cargo.toml`); the wgpu and plugin crates belong to the root workspace (default manifest). `Cargo.lock` of `🔀️shell-actions/📦️packages/🦀️rust/` does not exist (no per-crate lock; only the workspace locks), nothing to regenerate there.

Compiled and run through `repo cargo`:
- canvas `semio-framework-canvas-camera` (owns `camera-fit` tests): `camera_fit_laws` 7 passed, 0 failed (includes `every_fit_row_installs_the_camera_the_explicit_fit_names`).
- manifest `semio-framework-manifest-app-surface`: `node_graph_fit_action` 1 passed, 0 failed (`node_graph_fit_action_is_declared_as_the_shared_fixture_names_it`); compiles clean, no source fixes needed.

Fix made: `is_framework_reserved_action_id` (`🔌️plugin/👷️job/🏛️reserved/🦀️.rs`) did not list `NODE_GRAPH_FIT_ACTION_ID` although the instance dispatch refusal and `framework_reserved_action_kind` assumed it (the kind lookup would have returned `None`). Added it beside the document-transfer ids and asserted it in `shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity`.

NOT verified (blocked by concurrent peer edits, retried ~20 times over ~1 h):
- wgpu `fit_graph_action_persists_the_fitted_camera_like_a_wheel_zoom` (`semio-renderer-wgpu-engine-attach-tests-2`): blocked first by `semio-ui-wgpu-paint` and `semio-renderer-wgpu-shell-view-chrome` source-boundary overruns (4209/4000 and 4099/4000 active lines, peer edits), then by `semio-ui-theme` not compiling (`chrome_metrics::TRANSITION_DURATION_MS` etc. undefined, peer edit in flight).
- generation3d `repo cargo check`: same `semio-ui-theme` E0425 errors (11), none in generation3d, the plugin builder or fit code.
- plugin law tests (`semio-plugin-tests-11-2`): blocked by `semio-plugin-runtime-ops` source-boundary overrun (263739 of 262144 bytes, peer `raw-media/natural-source.rs`), later by `repo` itself failing to load (`🏗️build/🧷️format-stamps/🟨️.mjs:134` unterminated string literal).
- TS renderer-twins "node-graph fitGraph action": `bun test` cannot run it (needs jsdom + `SEMIO_VITEST_POLICY`); it must go through `repo ./📜️script.ts test` under a `repo` envelope, blocked by the same tooling failures.

Descriptor snapshots: only the hub component descriptors whose window kinds declare `nodeGraphViewport` gain a `fitGraph` row: `🌎️hub/🧩️compositions/🔱️trinity`, `🎪️demonstrator` (apps[2]) and `🌀️procedural` (apps[0], apps[2]). `🔌️plugin/🖥️host/🧪️testing/🧩️component` declares no `nodeGraphViewport` and is unaffected. Each descriptor is a hash-paired `🔣️.json` + `🛂️.descriptor.semio` regenerated by the `component` target (describe stage), so it is NOT hand-edited here; regenerate after the tree builds.
