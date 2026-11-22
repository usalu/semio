# Wave Coordination: Procedural 3d End To End (2026-11-22 21:11)

Supersedes runtime state in [📓️handoff-2026-11-22-evening.md](./📓️handoff-2026-11-22-evening.md). Ticket `26/09/09/PROCEDURAL-3D-END-TO-END` confirmed `OPEN`.

## 1. Launch & Host Audit (21:05)
- Ports `6018` and `6118` both down (`000`).
- Previous service hosts (`0003b7003034`, `efecd7669476`) exited at `19:59Z` (`shutdown superseded`), and `🗑️generated/wgpu-v13.txt` ended on `SIGTERM`.
- Host memory healthy (`77%` free at check, `68%` free after React launch).
- Relaunched React prototype (`repo run @semio-tech/framework-os-dev:dev-generation3d-react-prototype`) into `🗑️generated/react-v14.txt`; confirmed in graph-slice read (`4127 MB` granted).
- WGPU prototype (`dev-generation3d-wgpu-prototype`) queued immediately behind the `blit.wgsl` offload fix so its snapshot includes the fix and avoids cold-start graph-read contention.

## 2. Root Cause of `wgpu 29.0.4` `util/blit.wgsl` & `host-winit` Budget Check
- `semio-renderer-wgpu-host-winit` closure verified via `reachedFilesV1`: `20` files (`<= 32`), `236,443` bytes (`<= 262,144`), `3,919` active lines (`<= 4,000`). Within budget (keep new code out of `host-winit`).
- `couldn't read .../wgpu-29.0.4/src/util/blit.wgsl`: `INCLUDES` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts` only matched `include(?:_str|_bytes)?!`, missing `wgpu`'s `include_wgsl!("blit.wgsl")`. Because `wgpu-29.0.4` has `< 512` `.rs` files, `planOffloadV1` took `closure` (`reachedFilesV1` only) instead of `walkFilesV1`, omitting `src/util/blit.wgsl` on remote offload workers (and in `unit-cache` source hashing).

## 3. Active Parallel Wave 1 Lanes & 22:33 Checkpoint
- **Offload `blit.wgsl` + `#[derive(Mutations)]` local guard + `🔖️mutation-source-authority` diagnostics (`eb9e0b24...`) — COMPLETED**: offload bun tests `36/36`, unit-cache sources `6/6`, `semio-os-kernel-dsl-derive` exit 0, and new `mutation-source-authority` path/`ErrorKind` tests green (see `📓️offload-wgsl-and-mutation-authority-2026-11-22.md`).
- **WGPU source prep (`0cd9a6aa...`) — COMPLETED**: marquee `advance_paint2d_pixel_selection_into` window-id fix applied, `framework_fit_node_graph` `[DEBUG] shell node-graph fit` `None`-arm JSON trace added, `frame_maintenance_terminal_fault` deadline lane retired (`deadline-maintenance-law.test.ts` 1/1 pass, 12 expects), and audit R10/R12/5–7 updated (`📓️engine-attach-tests-2-port.md`, `📓️wgpu-frame-loop-fix.md` §7, `📓️battery-readiness-audit.md`).
- **Descriptor pairs & navbar preflight (`798c6bda...`) — COMPLETED**: re-encoded `remodel` and 47 other stale `🛂️.descriptor.semio` packs from `🔣️.json` (`semantic-source-pairs` `1/1` pass, `297` expects) and confirmed `P5a` navbar preflight passes (`📓️replica-stale-overwrite.md`, `📓️wgpu-frame-loop-fix.md` §7).
- **React (`6018`) & WGPU (`6118`) prototypes**: both actively advancing through their graph-slice reads (`973` owner checks in `🗑️generated/react-v14.txt`, `837` in `🗑️generated/wgpu-v14.txt`; host `45%` memory free, load `17.04` — keeping <= 2 compiling lanes until `a7b14342...` or `4ab7c327...` completes).
- **Job-context exact control-row conservation + laws (`a7b14342...`)**: running.
- **Procedural crate test verification (`4ab7c327...`)**: running.

## 4. Next Wave (upon Wave 1 completion)
- Compile and test WGPU crates (`engine-attach-tests-2`, `shell-tests-input-3`, `shell-tests-docs-1`/`-2`, `frame-job`, `runtime-mailbox-tests`) and `world_host_unit_3` GPU section test.
- Finish replica stale-overwrite leftovers (`🛂️.descriptor.semio`, `🌎️hub/Cargo.lock` metadata).
- Run `🐍️react-battery.mjs` (6018) and `🐍️wgpu-battery.mjs` (6118) once serving and fix any reds.
