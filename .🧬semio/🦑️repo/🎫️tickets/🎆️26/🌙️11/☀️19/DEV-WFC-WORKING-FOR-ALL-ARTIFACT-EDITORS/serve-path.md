# WFC dev playground: build and serve path

## Registration (verified)
- All five variants (`bitmap`, `grid2d`, `wfc2d`, `grid3d`, `wfc3d`) are declared as `[[package.metadata.semio.playground]]` rows in
  `🌎️hub/🧩️compositions/🀄️wfc/🧩️component/📦️packages/🦀️rust/Cargo.toml` (not under the plugin). They all use plugin id `wfc`, so ONE
  component build serves all five. React ports 6041..6045, wgpu 6141..6145.
- The checked-in `🧰️framework/.../📇️registry/🤖️generated` catalog is empty by design. The dev catalog is generated into
  `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` (158 playgrounds incl. all five WFC rows) by `@semio-tech/s-dev-composition:generate`.
- `✏️s/🔌️plugins/🀄️wfc/AGENTS.md` line 20 wrongly says the ports live in `📦️packages/🦀️rust/Cargo.toml` of the plugin (no such file). AGENTS.md must
  not be edited: report only. README.md is correct.

## Graph-free driving
`repo run` and every `repo ./📜️script.ts` need an original invocation envelope. While the graph load is blocked, `native-step.sh` mints the
same envelope (`envelope.json`: policy owner `dev-wfc-native`, no deadline, native-caller capabilities) into
`SEMIO_SCRIPT_PROCESS_INVOCATION` and runs the script directly. Rust goes through the repo's own cargo gate inside those scripts.
Steps run (see `native-pipeline.sh`): catalog generate, `support`, wasm-opt `prepare`, `session <variant>` x5, describe build,
native-transpile build, font assets, infinite `fonts`, wasm surface/editor/node-graph, component (wfc), then prepare/activate/serve.

## Status (2026-11-19 ~02:00)
Done natively (all logs in `🗑️generated`): catalog generated, `support`, wasm-opt prepare, `session-<variant>` x5 (staged in
`.cache/b/p/m/browser/prototype/sessions`), describe + native-transpile builds, font assets, infinite fonts, wasm surface/editor/node-graph.
Provider env for scripts: `SEMIO_SCRIPT_PROVIDERS` from `providers-env.ts` (needed by every cargo/wasm script).

Blocked: `native-component.sh` (the one shared `wfc` component, wasm32-wasip2, all five variants link into it). Fails at rustc in
`semio-s-artifact-wfc-grid3d-editor` (lib): `Grid3dTransientDiff: RetainedPatchEmpty` not satisfied and
`Grid3dTransientMutation: ColdMutation<Grid3dTransient>` not satisfied, at
`🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:30` and `.../✏️editor/🦀️.rs:555`
(7 errors total, see `component-errors.txt`). Incremental rebuild takes about 100 s.
Not yet run: prepare / activate / serve for any variant (they need the component).

## Status update (component rerun after grid3d fix, 313 s, 733 units reused)
Full error blocks in `🗑️generated/component-errors.txt`. Remaining compile errors (not mine to fix):
- `semio-s-artifact-wfc-grid3d-editor`, `🧱️grid3d/.../✏️editor/🫧️transient/🧬️schema/🧬️mutations/🦀️.rs:10:145` E0433 `semio_os_command` not found in crate root
  (the new `retained_empty_option_patch!` expansion; the crate lacks a `semio_os_command` dependency or the macro should name it via a re-export).
- same dir `🏁️set-solve/🦀️.rs:47:82` E0425 type `Grid3dTransientMutation` not in scope.
- `✏️editor/🦀️.rs:555:30` E0277 `Grid3dTransientMutation: ColdMutation<Grid3dTransient>` (follows from the two above).
- E0433 `retained_command::terminal_frame_bound` not found at `✏️editor/🦀️.rs:841:33` (grid3d editor root; cargo attributes it to a
  `semio-s-artifact-wfc-grid2d-tests-cuts-pinned-corner` "could not compile" line, so grid2d's test-cut crate that shares the mirrored source also fails).
Other variants' own errors are not visible yet: cargo stops at these crates. prepare/activate/serve not run.

## Serve results (component built by coordinator)
- `prepare` and `activate <variant> react prototype` pass for all five via `native-step.sh` (OS dev package dir, `SEMIO_PLUGIN_CATALOG` = dev catalog).
- Bug fixed: serving failed with `Unknown playground module identity: <variant>`. The WFC playground rows had no `devContribution`, so
  `serve` fell back to the framework default vite config whose inventory is empty (`createFrameworkOsDevConfig({inventory:{...[]}})`).
  Fix: added `✏️s/🧑‍💻dev/🎭️variants/🀄️wfc/{🏗️builder/🌐️vite/🟦️.ts,🚀️entry/🟦️.ts,🧬️schema/🔣️.json}` (copy of the `🏗️fem` contribution, entry accepts the
  five WFC variants) and `devContribution = ".../🎭️variants/🀄️wfc/🧬️schema/🔣️.json"` on the five rows of
  `🌎️hub/🧩️compositions/🀄️wfc/🧩️component/📦️packages/🦀️rust/Cargo.toml`; then regenerated the dev catalog and re-ran prepare/activate.
- 6041 bitmap: HTTP 200, shell mounts ("semio · wfc · bitmap", Editor/Example), editor surface blank with `original-turn.source-not-ready`;
  console: actor `OriginalTurnFailure ... Value(InvalidValue "staged original metadata is absent")`.
- 6042 grid2d: HTTP 200, shell mounts, two empty panes, banner "has not answered Load Example for 18 s".
- 6043..6045 (wfc2d, grid3d, wfc3d): vite not listening yet; the serves wait in the shared `semio.compiler.cpu-picker`/`host-capacity` lease queue
  behind bitmap/grid2d (and an on-demand `vite:lazy-activate:animate` component build).
