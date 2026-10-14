# Native Inactive Generated Output Inventory

Read-only inventory, 2026-10-14. No files were deleted. `ps` showed no Cargo, rustc, headroom, or wgpu-mode build actor. Disk free space was 167 MiB during the final inventory.

All paths below are relative to the ticket `🗑️generated` directory. Preserve source inputs and reports; the candidate paths contain generated caches only.

| Candidate | Physical KiB | Provenance |
|---|---:|---|
| `g-gpGk0F/cache` | 67,696 | Private Go build cache; adjacent original `main.go` prints `go-mode-runtime-ok`. |
| `g-B3WcXc/cache` | 67,696 | Same private Go fixture cache and preserved adjacent fixture input. |
| `g-7LTebV/cache` | 67,696 | Same private Go fixture cache and preserved adjacent fixture input. |
| `wgpu-wasm-vRcjWU/state` | 68,988 | Private Trunk/Cargo oracle state; adjacent Cargo package `trunk-output-fixture`, wasm-bindgen 0.2.126, script and project remain outside candidate. |
| `wgpu-wasm-59AT4D/state` | 68,988 | Same root-local renderer oracle state. |
| `wgpu-wasm-uV0OlX/state` | 68,988 | Same root-local renderer oracle state. |

Combined candidate size: 410,052 KiB, approximately 400 MiB. Current canonical wasm-output test owner is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️wasm-outputs/🟦️.ts`; its independent oracle explicitly directs Cargo to a fixture-local `state/oracle-target`. Older retained roots use the prior `wgpu-wasm-` prefix; package names and local inputs confirm the fixture identity. Go roots are identified by their original input contents and private cache layout, not a currently preserved source producer path.

The larger top-level `nv2` directory contains additional fixtures, so its entire 81,376 KiB is not proposed for deletion without a child inventory. `isolated` currently contains the new native-window-source project graph and other active acceptance inputs; its whole directory is excluded. Shared Cargo debug, prototype, release and target profiles are excluded.
