# GIS Startup and End-to-End Verification — 2026-09-12

The ticket was reopened through the repository MCP after reading `repo://goals`. Its existing r26-03 association is retained.

## Scope

Run the public `bun nx run workspace:dev -- gis 2d` command successfully, verify the actual GIS application and map interactions in a browser, and fix failures encountered on that path. Earlier ticket measurements used a standalone map harness and explicitly left the GIS development application unable to boot; those measurements do not establish current end-to-end success.

## Initial Evidence

- The reported launch blocks in `NxScript.run` awaiting `watch process waiting...`, with a fixed 120-second deadline.
- Installed Nx registers its watcher only after awaiting its project graph.
- The custom project provider calculates script import closures independently for every target. Its parser cache retains import text but each closure walks, reads, and resolves the shared files again. Many targets reference the same command router.
- A first reproduction failed even earlier on concurrent Print taxonomy edits. A subsequent launch passed that point and is under observation.
- No existing test or old status claim is treated as evidence of current runtime success.

## Validation Plan

1. Freeze repeated-discovery and cross-pass source freshness expectations in a language-neutral fixture.
2. Compare closure results with esbuild and verify bounded file reads within one graph pass.
3. Re-run the public launch and inspect browser console, map painting, zoom, pan, and tile transport.

## Results

- The new regression check failed before implementation: graph discovery had no pass-scoped script input cache.
- Added a per-graph cache for resolved file imports and complete command closures, shared across project targets. Independent calls and subsequent graph passes get a fresh cache.
- The regression check passed through Nx in an isolated ticket-owned test workspace: 40 overlapping closure requests read each file once, match esbuild, agree with native Node execution, and refresh changed transitive imports on the next pass.
- The first full-workspace test attempt spent over a minute in graph construction and then encountered an existing dependency cycle between `@semio-tech/framework-os-kernel` and `@semio-tech/value-derive-rs`. The bounded fixture workspace allows the regression check to execute independently of that workspace graph issue.
- Reproduced the exact watcher timeout with `SEMIO_RENDERER=react bun nx run workspace:dev -- gis 2d`. Without that terminal setting, the same command selects the WGPU path in this environment.
- Replaced the arbitrary readiness deadline with readiness/exit/error handling, progress every 15 seconds, and existing process cancellation. Four language-neutral lifecycle cases pass, including a simulated 150-second cold startup, split readiness output, error exit, and cancellation. The readiness marker is checked against installed Nx's actual watch implementation.
- The live React invocation passed watcher readiness after roughly 60 seconds and started the GIS task graph with 19 dependencies. Flow WASM compilation completed.
- The watcher incorrectly scheduled activation for compiler cache `.lock` writes. Added explicit compiler and map cache roots to `.nxignore`. The `ignore` library and Nx's native watcher both validate cache exclusion while retaining source edit events. The native fixture supplies the equivalent exclusion globs directly with inherited ignore processing disabled, because its required ticket location is itself ignored by the parent repository.
- The ignore configuration update restarted the daemon and interrupted that in-flight build. The public React launch has been restarted with the new configuration.
- Re-enabled `TiledMapHost` in the existing React renderer long suite and corrected its broken relative import. Vitest executed all 11 map tests successfully (13ms test time; ~30s including module transforms).
- Full GIS application boot and browser checks remain pending.
- The command import regression also passes through the full workspace graph with `NX_DAEMON=false bun nx exec --projects=workspace --excludeTaskDependencies --skipNxCache -- bun <ticket>/📜️script.ts imports`. The explicit dependency exclusion makes this diagnostic run once on the selected workspace, avoiding unrelated package-cycle traversal.
- Registered all ticket verification modes (`imports`, `watcher`, `map`, `graph`) in VS Code as **🗺️gis verification**, with matching authored seed and generated launch entries. Both JSONC files parse successfully.
- The live watcher and initial startup raced while publishing browser-support artifacts, exposing an `EEXIST` failure in the synchronous publication lock. A language-neutral publication fixture reproduced the missing wait/cancellation behavior. Publication now uses existing SQLite resource leases, asynchronous copying, ownership validation under the lease, and awaited callers. The focused fixture passes cancellation and contention and compares delivered bytes with Python file reads. All 18 direct publication callers/helpers have valid async boundaries.
- Flow, Surface and Editor WASM builds completed in the independent GIS preparation target. The original development supervisor was stopped after its failed browser-support task; preparation continues before restarting the public launch.
- Resource lease checks pass across Bun, Node and Python: shared readers, exclusive writers, independent resources, cancelled waiters, killed owners, cold initialization, bounded repeated storage, identity guards and ordered multi-resource acquisition.
- The real `@semio-tech/framework-os-dev:prepare-gis2d-react-dev` target succeeded with its 17 prerequisites, including GIS component compilation and browser materialization (18m18s in this shared cold-build run, 5/18 cached). The public development command has been restarted for browser validation.
- Added `NX_PROJECT_NAME` and `NX_FILE_CHANGES` to the authored Nx patch's task-environment exclusions. The existing native Nx fixture failed before this change and passes afterward, while `NX_LOAD_DOT_ENV_FILES` still changes graph identity. This prevents watcher callback bookkeeping from repeatedly invalidating the graph. The complete authored Nx patch matches the installed package in a reverse dry-run check.
- Activation of the already-prepared GIS application succeeded via `NX_DAEMON=false bun nx run @semio-tech/framework-os-dev:activate-gis2d-react-dev --excludeTaskDependencies --output-style=stream`. It validated the component, session, support and fonts and published the runtime receipt. A second actual app server is being launched on port 6041 through the existing serve target, while the public watch invocation continues on its configured port 6040.
- The actual application served on 6041 and rendered a vector map with geographic labels and fixture positions. A wheel gesture produced a completed `setCamera` invocation. Keyboard selection of Image mode exposed `typed-operation failed: validation failed: batched publication requires preinstalled fixed applied and revision capacity`; the optimistic control changed but the map retained vector rendering, so this is not counted as a successful mode change.
- The store source was newer than the compiled GIS component (22:53 versus 22:41 local). The current shared source already contains cursor-capacity preservation and a targeted `retained_latest_wins_cold_rebase_preserves_admitted_cursor_capacity_for_next_publication` regression. Rebuilding the actual GIS preparation target against that source before deciding whether further store changes are needed.
- The shared daemon kept restarting target graph requests for more than 12 minutes while independent Nx requests completed. Watched task processes now compute their own Nx graphs (`NX_DAEMON=false`) while the source watcher retains the daemon. Ordinary commands preserve their daemon selection. Four language-neutral process cases pass against installed Nx's native daemon-enable decision; the existing readiness, ignore and resource-lock suites still pass. Restarted the public launcher with this routing correction.
- The current rebuild encountered a concurrent viewport-schema edit with an invalid derived `Default` bound. The shared source already contains its manual generic `Default` correction; no duplicate viewport edit was made here.
