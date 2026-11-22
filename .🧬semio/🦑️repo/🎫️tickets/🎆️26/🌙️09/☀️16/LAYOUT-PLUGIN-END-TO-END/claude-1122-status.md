# Layout End To End: Session 2026-11-22 (claude-code)

## Target

The accepted target is WGPU on port 6179. The chain is `repo run @semio-tech/framework-os-dev:activate-layout-wgpu-prototype`, then `serve-layout-wgpu-prototype`, then `browser-journey.mjs` (13 steps). Activation depends on `prepare-layout-wgpu-prototype`. Its inputs are the plugin-registry session, plugin-web support, infinite fonts, renderer-wgpu wasm and boot generators, and `semio-hub-layout-component:component`. Serve depends on activate. Source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs` (activate line 263, prepare lines 272–276).

## Result So Far

No activation has reached task planning. Ten attempts since 02:00 all stalled or died inside the shared repo graph read. The genuine closure is about 2,300 owners, roughly 1,000 Rust crates, confirmed by the peer's E33 lane. On a host at load 40–60 with 30/31 GB swap, that read takes 1.5–2 h per attempt.

| Run | Outcome |
| --- | --- |
| 1 | Niced (zsh BG_NICE); restarted with `setopt no_bg_nice` |
| 2 | Inventory restarted 3× (peer graph lease, transient missing `🎚️machine` module) |
| 3 | Planning failed: `@semio-tech/framework-os/modules/renderer/engine/media/browser/tutorial-video` unresolved |
| 4 | Same failure; root cause was a missing package-local re-export, fixed (see below) |
| 5 | `route-undeclared:test-build-test-file` (stale suites JSON import in the resident service; peer fixed in E29) |
| 6 | `route-undeclared:test-build-task-view` (peer's two-step suites edit was mid-flight) |
| 7 | `owner-isolation` import path mid-landing (peer E29) |
| 8 | Killed at 2,359 owner checks to capture the `[graph-closure]` trace for the peer |
| 9 | `BUILD_OWNER_FAILED`: framework-renderer-wgpu (new `🎓️HubFirstRun` wgpu crate without an owner row) and repo (`route-table-invalid`). Both transient; both infer cleanly afterwards |
| 10 | Running; silent inventory reset at ~5,300 s, reported to the peer (E45) |

## Fixes Made

- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/🌐️browser/🎞️tutorial-video/🟦️.ts`: a package-local re-export for the new `./modules/renderer/engine/media/browser/tutorial-video` export. Without it the WGPU browser-frame-transport, frame worker and browser host could not resolve the module. A repo-wide scan found no other missing non-dist export target.
- `browser-journey.mjs`: headless Chromium now gets `--enable-gpu` plus `--use-angle=metal` on macOS (Metal instead of SwiftShader), so WebGPU timing verdicts are real.

## Reports

- `claude-1122-state-audit.md`: prior ticket state. The browser journey has never run against a live server.
- `claude-1122-route-table-fix.md`: the `repo` route table validates. It lists two unrouted test files for the suite owners.
- Native layout crate check: the instance-commands, stdio-pdf authoring and document-media errors all landed fixed by peers during the session. No layout-crate error was reproduced on current sources.

## Next

1. Keep one activation attempt alive until it plans, or until the peer's E45 seals a generation that `current` can adopt.
2. `repo run @semio-tech/framework-os-dev:serve-layout-wgpu-prototype --exclude-dependencies`, detached and un-niced.
3. `repo tools exec bun -- <ticket>/browser-journey.mjs`, then fix journey faults step by step.

The host's memory thrash is dominated by fseventsd (~8 GB) plus concurrent whole-graph reads. Restarting fseventsd needs the user's sudo.
