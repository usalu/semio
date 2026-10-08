# Current Native Canonical Six Invocation

The current observation on 2026-10-08 found no `current-native` process and no `🗑️generated/current-native-origin/epoch-2` directory. The authored current-origin inputs and reports remain present. Missing historical output is not reconstructed and does not establish acceptance.

The current General UI owner declares `test-wgpu-engine` in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📋️project.json`. Its package script routes `test wgpu-engine` through the shared owned Cargo test provider, selecting `wgpu-engine` and `--lib`. The existing GUI command is `📦️test🧊️wgpu🖱️ui` in `.vscode/launch.json`.

A fresh actual invocation uses `bun nx run @semio-tech/ui-rs:test-wgpu-engine --skip-nx-cache --excludeTaskDependencies --args=long`, explicitly sets `SEMIO_TEST_LEVEL=long`, and directs test artifacts and Cargo target/build output under ticket-owned `🗑️generated/current-native-canonical-6`. The shell terminal handle is 30639. Its initial log confirms the actual repository Bun bootstrap dispatched the requested Nx target. At this initial observation compilation and assertions have not started; there is no acceptance claim.

This invocation reads the current shared workspace. It does not claim an atomic source snapshot or identity with a removed historical capture. Source changes made concurrently must remain distinguishable from the actual compiler and assertion results.

Repository MCP goals and ticket tools remain unavailable in the current exposed tool inventory. The existing ticket remains the scope of this work.
