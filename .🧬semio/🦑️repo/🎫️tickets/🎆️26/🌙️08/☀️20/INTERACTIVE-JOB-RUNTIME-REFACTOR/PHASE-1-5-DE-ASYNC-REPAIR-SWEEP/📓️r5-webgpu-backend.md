# 📓️ R5 — WebGPU Render Backend De-Async Repair

## 🎯️ Scope
Crate `semio-framework-ui-backend-webgpu` at
`🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/` (11 source files, 2528 lines).

## 🚨️ Finding: the "452 errors" baseline is a measurement artifact, not the async bug class

`📦️glue.rs` opens with:
```rust
#[cfg(not(target_arch = "wasm32"))]
compile_error!("semio-framework-ui-backend-webgpu is browser-only: wgpu is deliberately confined to wasm32 builds. Native targets use the hand-written metal/d3d12/vulkan backends.");
```
and `Cargo.toml` puts `wgpu`, `web-sys`, `wasm-bindgen` under `[target.'cfg(target_arch = "wasm32")'.dependencies]` only.

Running the literal command from the MEASUREMENT CAVEAT —
`cargo check -p semio-framework-ui-backend-webgpu --all-targets` — with **no `--target`** hits the native
default target, trips the `compile_error!`, and then cascades into `error[E0433]: cannot find crate wgpu /
web_sys` on every line that names those types: **452 lib errors / 478 total (lib+test)**, exact match to
the packet brief's number. None of these are `Future`/`.await`/async-fallout errors (confirmed: 0 hits for
`Future` or `await` in that error log) — they are 100% `E0433` unresolved-crate errors, and no amount of
`async`/`.await` editing can fix them, because the deps genuinely don't exist on that target. This is
exactly the case the packet brief's own "GRAPHICS CAVEAT" and "wasm-gated code never compiles natively"
line anticipated.

The correct measurement is `cargo check -p semio-framework-ui-backend-webgpu --all-targets --target
wasm32-unknown-unknown`, and on that target the crate **already compiles with 0 errors**, both before and
after this session (no edits were made — see below).

## 📈️ Error trajectory
| Measurement | Errors |
|---|---|
| `cargo check --all-targets` (native, no `--target`) — baseline per packet brief | 452 (lib) / 478 (lib+test) — all `E0433` crate-not-found, not the async bug class |
| `cargo check --all-targets --target wasm32-unknown-unknown` (correct target) — before this session | 0 |
| `cargo check --all-targets --target wasm32-unknown-unknown` — after this session | 0 (unchanged, no edits made) |
| `cargo clippy --all-targets --target wasm32-unknown-unknown` | 0 errors, 17 pre-existing style warnings (unrelated to async: unnecessary qualification, too-many-arguments, needless-pass-by-value, dead-code on an unused `slice()` helper) |
| `bun ./📜️script.ts verify dependencies` | clean, 238/238, no new third-party deps |

## 🔍️ async fn / .await audit
Grepped the crate directly and cross-checked against Phase 0's `🔧️async-census.json` (filtered to this
crate's path): **exactly 2 `async fn` in the whole crate**, both already correct, and **0 functions were
converted** in this session because none needed it — the crate was already fully compliant with the U1
owner ruling (`SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY/📌️important.md`, "U1 — new UI crates use LITERAL
`fn`, not `async fn`") before this packet started. Every non-suspending function in the crate already
carries the marker comment `// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20
📌️important.md` (found on ~75 functions across all 11 files) — this crate appears to have been
authored/repaired directly against the U1 convention rather than mechanically converted, so there is no
"functions lost async vs. call sites gained .await" tally to report: **0 / 0**.

## ✅️ GPU futures correctly KEPT async
Only `GpuContext::new` (`🦀️gpu_context.rs:101`) is `async`, called from the crate's other genuinely-async
fn `WebGpuBackend::new` (`🦀️backend.rs:92`), which awaits it. Body of `GpuContext::new`:
- `instance.request_adapter(...).await` (`🦀️gpu_context.rs:106`) — real round-trip to the browser's GPU
  process to negotiate a `wgpu::Adapter`.
- `adapter.request_device(...).await` (`🦀️gpu_context.rs:117`) — real round-trip to obtain
  `(Device, Queue)`.

Both are genuine suspension points (per-call browser IPC, not CPU work), correctly kept `async`/`.await`ed,
and are annotated in-source: `// 🌐️async: genuinely async device/adapter construction — the one exception
U1 itself carves out.` No other function in the crate touches a GPU future; buffer mapping, queue submit,
and surface present are all synchronous/polled paths here (see Phase 5 seed notes below) so nothing else
needed an `.await`.

## 🧵️ Category-C / long-running CPU work — Phase 5 seed notes
These are plain `fn` (correctly non-async) but are exactly the "long-running CPU work" Phase 5 targets for
worker-job offload under an 8 ms/step ceiling. Flagging per the packet brief, no changes made:
- `🦀️frame.rs::replay_batches` (line 147, 10 args) and `draw_silhouette_mask` (line 55, 8 args) — draw-call
  replay/sorting loop over `RenderPacket` batches; this is the "GPU upload-packet preparation" /
  draw-sorting work Phase 5 names explicitly.
- `🦀️resources.rs::upload_atlas` (line 236, 8 args) and `upload_texture` (line 278, 8 args) — CPU-side
  pixel buffer packing before `queue.write_texture`; classic upload-packet-prep candidate.
- `🦀️buffers.rs::GrowBuffer` staging/grow path — buffer packing ahead of GPU upload.
- `🦀️pipelines.rs` — pipeline/shader-module construction; one-time-ish but CPU-bound, worth Phase 5
  triage if it ever runs mid-frame.

None of these currently block the UI thread with an actual suspend (they're synchronous CPU loops, matching
the repo-wide 88.28% non-suspending finding from Phase 0), so no `async`/`.await` changes were warranted
here — this is purely a Phase 5 forward note, not a Phase 1.5 defect.

## 🚧️ Cross-boundary breakage
None observed. No edits were made outside (or inside) this crate's boundary.

## 🧪️ Verification actually run
- `cargo check -p semio-framework-ui-backend-webgpu --all-targets` (native) — 452/478 errors, all E0433,
  confirmed as the compile_error!-gated native-target artifact described above.
- `cargo check -p semio-framework-ui-backend-webgpu --all-targets --target wasm32-unknown-unknown` — 0
  errors, 9–12 warnings (lib/test), unrelated to async.
- `cargo clippy -p semio-framework-ui-backend-webgpu --all-targets --target wasm32-unknown-unknown` — 0
  errors, 14–17 warnings, unrelated to async.
- `cargo test -p semio-framework-ui-backend-webgpu --target wasm32-unknown-unknown` — test *compilation*
  succeeded (0 errors); test *execution* failed with `cannot execute binary file` (exit 126) — this
  environment has no runner configured for `wasm32-unknown-unknown` in `.cargo/config.toml` (no
  `wasm-bindgen-test-runner` / headless-browser harness wired up), and the crate's tests are plain `#[test]`
  (not `wasm_bindgen_test`), so they cannot be executed headlessly here. Reporting this explicitly rather
  than claiming a pass — this is an environment/tooling gap, not a crate defect.
- `cargo test --release` — not attempted; same runner gap would apply, would not change the finding.
- `bun ./📜️script.ts verify dependencies` — clean, 238/238 baseline, no new third-party deps.

## 📝️ Files touched
None. No source edits were made in this crate — it was already correct.
