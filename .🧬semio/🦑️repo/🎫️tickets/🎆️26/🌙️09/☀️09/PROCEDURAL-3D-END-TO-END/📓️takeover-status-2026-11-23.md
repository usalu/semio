# Takeover status (2026-11-23 ~01:45 CET)

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END` stayed open. Ports `6018` and `6118` were not listening. Result notes for the React serve, the WGPU serve, and the procedural/retained tests were absent, so those lanes were checked by process and log mtime before any restart.

## Lanes already running (not restarted)

| Lane | Evidence |
| --- | --- |
| Graph-slice verification | `repo run …:dev-generation3d-react-prototype` pid 89189, log `🗑️generated/react-v15.txt`. Owner checks moved 587 → 757 while admitted owners stayed 535 (about 1600 s). Still on the inventory step. Native serve stays the path until this seals. |
| Procedural component via `repo run` | `🗑️generated/native-component-procedural-3.txt` reached `graph: plan answered and verified`, then held a shared lease on `native-fixed-image:…/.cache/b/d`. |
| Native component driver | pid 58486, 45 min, no cargo child, no `[native-build]` line. Heartbeat only. It holds the `b/l` image lease sqlite. The `b/d` lease sqlite is locked by bun pid 17436, a child of `repo tools setup all` (pid 65332, elapsed ~3 h 45 m). |
| WGPU wasm | First `wgpu-native-wasm-build.sh` exited 101. One error: `LocalFileRow` not in scope in `📎️local-folders/🦀️.rs` (`use super::*` does not see the parent's private import). Import added: `use semio_renderer_wgpu_shell_history::LocalFileRow`. The script was started again (pid 27300) and is compiling. |
| io-text tests | `cargo test -p semio-s-artifact-procedural-3d-io-text` still compiling (no `error[E`). |
| job-context | `jc-run8.txt` compiling `semio-framework-value-native-owned` (no `error[E` in that log). |
| P5a cargo check | `error[E0433]` in crates.io `wasmtime-wasi-io-47.0.4` (`bindings::wasi`). Separate from the nine Rust findings, whose law test was already recorded green. `named_scene_surface` has no remaining Rust hits. |

Host at the check: load ~17, system-wide free memory 34–39 %, three cargo processes. No further cargo was started.

## Source fixes landed while those lanes ran

- `📎️local-folders/🦀️.rs` now imports `LocalFileRow`. The wasm rebuild (pid 27300) reached `semio-renderer-wgpu-shell-prefs` with warnings only at the last read.
- `render_widget` in `🧮️scene-tables/🦀️.rs` was private, so the table and block-list modules (`use super::*`) could not call it. It is `pub(crate)` now.

Graph inventory at the same read: 950 owner checks, 535 owners admitted, observations 178532. Component drivers were still on the `b/d` image lease with no `[native-build]` line.
