# R15 Integration Execution

Scope: restore current BIM compile and runtime integration without blanket fixture blessing. Read root and BIM AGENTS.md, r15-explore-plugin.md, coordination tail and r13 integration report.

## Current Verification

No current test pass is claimed. Coordinator's `bun nx @semio-tech/bim-model-rs:check --excludeTaskDependencies --skip-nx-cache --outputStyle=stream` remains running. At first it emitted only the bootstrap command; exact processes were bun 49028, bootstrap bun 51612. A guarded cancellation deliberately preserved them when node 67132 appeared. A direct pinned Bun Nx launch (54176, parent 69424) was canceled before children to avoid running duplicate checks. Baseline then reached the native owner-command and dependency-pair preparation (Locking 0 packages); logs are `🗑️generated/r15/baseline-check.log`.

## Bounded Upstream Repair

Historical active `r13-stack/bl-21.txt` reports four current os-infinite PNG caller errors in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`: stale Arc source, integer work budget, non-tuple progress, removed into_result. Source inspection confirmed each. The repair passes owned Vec data to PngDecodeInput, preserves rejected input bytes, grants explicit copy/capacity/depth work budgets, verifies receipts, transfers RasterImage through take_result and drains decoder allocations using ControlledRetirement both after publication and on cancellation. It preserves the prior maximum 4096 decode units per caller turn.

Existing language-neutral inline surface fixture is consumed by its Rust test and independent Three.js lane. The Rust world inline surface law was extended to compare PNG role raster pixels with the existing image test library, observe cancellation in decoder/retirement phases and assert no residual decoder owners after publication/close.

Changed files so far:

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`

## Pending

Compile repair validation, focused inline surface conformance test, current BIM lib/test compile, mounted gestures on ordinary stacks, house inference reproduction, session retirement/progress/undo integration. Current source still has BIM_STACK_MB, 8 MB wrappers and debug_stack_probe; those remain until the actual runtime cause can be reproduced. Ownership callers in editor still use old u32 registry hooks while the instance module exposes framework handles; the law execution agent owns that migration.
