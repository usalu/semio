# R6 — D3D12 De-Async Repair + Vulkan/D3D12 Platform-Gate Fix

Packet R6 of Phase 1.5. Ownership boundary: `semio-framework-ui-backend-d3d12` (Tasks 1–2) plus a
targeted platform-gating fix in `semio-framework-ui-backend-vulkan` (Task 3).

## 1. The "332 errors" figure is entirely Task 2's bug, not Task 1's

Baseline reproduction on this (macOS) host:

```
cargo check -p semio-framework-ui-backend-d3d12 --all-targets   # 332 (lib) + 332 (lib test)
```

Full-log analysis of that baseline (`/private/tmp/.../scratchpad/d3d12-macos-baseline.txt`):

| Error kind | Count |
|---|---|
| `E0425` (cannot find function/value) | 241 |
| `E0422` (cannot find struct/type) | 55 |
| `E0433` (cannot find crate/module) | 33 |
| `E0432` (unresolved import) | 1 |
| `Future`/`await`-shaped errors (`impl Future`, unary `!`, `?`-on-non-Try) | **0** |

Every one of the 334 total error lines traces back to `🦀️types.rs`'s `use windows::Win32::…` (and every
other file's `use windows::…`) failing to resolve, because `📦️glue.rs` declared its `mod` statements
**without** a `target_os = "windows"` gate (unlike the Vulkan backend's `📦️glue.rs`, which *does* gate
every `mod`). `Cargo.toml` correctly puts the `windows` crate behind
`[target.'cfg(target_os = "windows")'.dependencies]`, so on macOS `windows` genuinely isn't in the
dependency graph — the moment `types.rs` is even parsed, every D3D12/DXGI type name inside it cascades
into "not found in this scope". This is Task 2's incomplete platform gate, not Task 1's async bug class.

**Async fn census for this crate**: `🔧️async-census.json` filtered to this crate's path lists exactly
**2** `async fn` in the whole tree — `D3d12Backend::new` and `D3d12Backend::new_headless`
(`🦀️backend.rs:128,146`) — both classified `B` (non-suspending body). Grepping the crate directly
confirms it: 2 `async fn`, 0 real `.await` calls (the one `.await` grep hit is inside a doc comment, not
code). Every other function in the crate (`from_parts`, `update_globals`, `render`, `resize`,
`encode_scene_pass`, `run_blur_chain`, all of `pipelines.rs`/`resources.rs`/`types.rs`/`world3d.rs`/
`scene_target.rs`/`frame_buffers.rs`/`hlsl.rs`) is already plain sync `fn`, each carrying the marker
comment `// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md`. This
crate had **already been authored directly against the U1 convention** (same situation R5 found for
`semio-framework-ui-backend-webgpu`) — there is no "functions converted" tally to report for Task 1: 0
`async fn` removed, 0 `.await` added, because none were needed.

## 2. GPU futures correctly kept async

`D3d12Backend::new`/`new_headless` stay `pub async fn`, matching every other hand-written backend
(Metal, Vulkan) and the repo-wide "ONE permitted async fn per the `GraphicsBackend` docstring" (U1)
convention — construction is the one call every backend's own doc comment documents as the deliberate
async surface, for uniformity with `webgpu`'s *genuinely* suspending `wgpu::Instance::request_adapter`/
`request_device` round-trips, even though D3D12/DXGI device+swapchain creation is itself synchronous
(confirmed: the bodies of both functions call only sync helpers — `create_device_and_queue`,
`create_swapchain_for_hwnd`/`create_swapchain_for_composition`, `Self::from_parts` — none of which are
`async`, and neither function contains an `.await`). The crate's own `#[cfg(test)]` module drives them
with a hand-rolled no-op-waker `block_on` (identical technique to the Metal backend's test module) rather
than pulling in an executor crate, with an explicit `panic!` if the future is ever `Poll::Pending` —
i.e. the test suite itself asserts non-suspension. This matches the task brief's "GRAPHICS CAVEAT"
exactly: not de-asyncing a function that legitimately mirrors a genuinely-async GPU acquisition path
elsewhere in the same trait family. **No changes made here** — already correct.

## 3. Task 2 — the incomplete platform gate, fixed

`📦️glue.rs`'s `mod` declarations were ungated; every other backend crate (Vulkan, Metal) gates its `mod`
statements on the owning `target_os`. Fixed by gating every `mod`/`pub use` in D3D12's `📦️glue.rs` behind
`#[cfg(target_os = "windows")]`, mirroring Vulkan's existing pattern exactly (see Task 3 below for why
the top-level `compile_error!` itself also changed).

Before → after, this host (macOS), `cargo check -p semio-framework-ui-backend-d3d12 --all-targets`:
**332 (lib) + 332 (lib test) → 0**.

## 4. Task 3 — Vulkan's (and now D3D12's) hard `compile_error!` doesn't block `--workspace` anymore

### The problem

Both `semio-framework-ui-backend-vulkan` and (after §3's fix) `semio-framework-ui-backend-d3d12` are
direct workspace members (`Cargo.toml:31-34`) with a top-level
`#[cfg(not(target_os = "…"))] compile_error!(...)`. That banner is a deliberate, already-reviewed design
(R3's packet confirmed it for Vulkan: "not a bug, needs no code change") — but a `compile_error!` fires
unconditionally whenever the crate itself is compiled, and `cargo check --workspace` always visits every
workspace member directly, independent of whether any other crate on this host actually depends on it.
So on macOS both crates hard-fail `--workspace`, which blocks this refactor's exit gate ("the workspace
builds clean") even though nothing on macOS ever legitimately needs either backend.

### Why the existing target-gated dependency wiring doesn't already fix this

`🖥️host/📦️packages/🦀️rust/Cargo.toml` already only pulls each backend in under
`[target.'cfg(target_os = "…")'.dependencies]` — `backend_vulkan` under `target_os = "linux"`,
`backend_d3d12` under `target_os = "windows"`, `backend_metal` under `target_os = "macos"`,
`backend_webgpu` under `target_arch = "wasm32"`. That wiring is exactly the "target-cfg'd consumer
dependency" approach the packet brief offers as option 1 — and it already fully prevents any *use* of the
wrong backend on the wrong platform: the dependency edge itself doesn't exist, so no consumer code can
even name `VulkanBackend`/`D3d12Backend` on the wrong host. But it does nothing about `cargo check
--workspace` reaching the crate *directly* as a member — Cargo has no `cfg`-conditional workspace
membership, and `--workspace` explicitly overrides `default-members` exclusions. So option 1 alone cannot
close this gate.

### The fix chosen: cfg-gated empty lib (option 2)

Removed the top-level `compile_error!` from both crates' `📦️glue.rs`. On the "wrong" platform the crate
now compiles to an empty, zero-item lib (every `mod`/`pub use` was already, or is now, gated on the
owning `target_os`, so nothing is left ungated to even try compiling). On the crate's real platform,
behavior is completely unchanged — same real modules, same public API.

This was the right choice among the brief's three options, not the other two:
- **Target-gated consumer dependency** — already in place (see above) and insufficient by itself, since
  it doesn't touch direct workspace-member compilation.
- **Optional feature not on by default** — would regress the *real* platform too: `cargo check
  --workspace` uses default features, so gating all code behind an off-by-default feature would make the
  crate compile empty even on Linux/Windows unless every invocation remembered to pass
  `--features vulkan-enabled`/similar, breaking the already-established `cargo check -p ... --target
  x86_64-unknown-linux-gnu` verification flow (no special feature flags) R3 used successfully.
- **cfg-gated empty lib** — matches the crate's own existing `target_os`-gated `mod` structure exactly,
  requires no `Cargo.toml`/feature surface changes, and needs zero coordination with any consumer.

**Intent preservation** ("using the wrong backend on the wrong platform is an error"): still holds, one
layer up. `ui-host`'s target-gated dependency means a consumer on the wrong platform never sees this
crate's dependency edge at all — trying to reference `VulkanBackend`/`D3d12Backend` from such a consumer
still fails to compile (an "unresolved import"/"no such item" error), just raised at the actual misuse
site instead of unconditionally at this crate's own root. That is arguably a *more* useful diagnostic
than the banner was (it points at the real mistake), and it no longer taxes every unrelated crate's
`--workspace` check for a mistake nobody made.

Both crates' `📦️glue.rs` headers were rewritten to document this reasoning in place of the old
"why the banner exists" text, and each now points at the other's identical treatment.

### Consistency across all four backends — what was and wasn't touched

- **Vulkan, D3D12**: fixed (this packet, my ownership).
- **Metal**: currently the *host* platform here (macOS), so its own `compile_error!` never fires on this
  host and isn't blocking anything right now — **not touched**, out of this packet's assigned scope,
  but it carries the identical `compile_error!` pattern and would need the identical fix to keep
  `cargo check --workspace` green on a Linux/Windows host. Flagging for the coordinator.
- **WebGPU**: **not touched** (explicitly a concurrent sibling packet's crate, per the brief). R5's own
  report (`📓️r5-webgpu-backend.md`) confirms they left its `compile_error!` in place and did not solve
  this same workspace-blocking issue — `cargo check -p semio-framework-ui-backend-webgpu --all-targets`
  (native, no `--target`) still hard-fails with 452/478 errors on macOS, for the identical structural
  reason (workspace member + unconditional `compile_error!`). **Coordination note for the coordinator /
  the webgpu packet**: the identical minimal fix — delete the top-level `compile_error!` in
  `🧊️webgpu/📦️packages/🦀️rust/📦️glue.rs`, whose `mod`s are already ungated but only ever contain
  `wgpu`/`web-sys` types that are themselves `wasm32`-gated in `Cargo.toml`, so the same cascade would
  need the same `#[cfg(target_arch = "wasm32")]` gating added to every `mod`/`pub use` there (currently
  absent — webgpu's `mod` statements are not target-gated at all, unlike Vulkan's) before removing its
  `compile_error!` — would make webgpu disappear from `--workspace`'s `could not compile` list too. I did
  **not** make this edit myself (out of my ownership boundary and explicitly the sibling's territory);
  reporting it precisely per the brief's instruction instead.
- No shared/cross-cutting file (`ui-host`'s `Cargo.toml`, the root workspace `Cargo.toml`) needed any
  change — the existing target-gated dependency wiring there was already correct and is exactly what
  makes the "wrong platform" empty-lib case safe.

## 5. Verification actually run

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui-backend-d3d12 --all-targets` (macOS) | 332+332 → **0** |
| `cargo check -p semio-framework-ui-backend-vulkan --all-targets` (macOS) | 1 → **0** |
| `cargo check -p semio-framework-ui-backend-d3d12 --all-targets --target x86_64-pc-windows-msvc` | **0** errors (only pre-existing style/dead-code warnings: unnecessary `std::mem::` qualifications, a few never-read fields/never-used fns behind `backend-testing` off by default) |
| same, `--features backend-testing` | **0** errors |
| `cargo check -p semio-framework-ui-backend-vulkan --all-targets --target x86_64-unknown-linux-gnu` | **0** errors (pre-existing dead-code warnings only, unchanged from R3) |
| `cargo clippy -p semio-framework-ui-backend-d3d12 --all-targets` (macOS) | 0 errors |
| `cargo clippy -p semio-framework-ui-backend-vulkan --all-targets` (macOS) | 0 errors |
| `cargo clippy -p semio-framework-ui-backend-d3d12 --all-targets --target x86_64-pc-windows-msvc` | 0 errors, pre-existing style warnings only (`field_reassign_with_default`, `manual_div_ceil`, dead-code) |
| `cargo clippy -p semio-framework-ui-backend-vulkan --all-targets --target x86_64-unknown-linux-gnu` | 0 errors, pre-existing dead-code warnings only |
| `cargo test -p semio-framework-ui-backend-d3d12 --target x86_64-pc-windows-msvc --no-run` | type-checks clean (0 compile errors); fails at the final **link** step (`link.exe` not found — no MSVC linker/Visual Studio toolchain on this macOS host). Same category of macOS-host cross-linker gap R3 hit with Vulkan's `mold` on the Linux target — not a code defect; type checking (the signal this bug class needs) already succeeded. |
| `cargo check -p semio-framework-ui-backend-metal --all-targets` (macOS, unaffected sanity check) | 0 errors, unchanged |
| `bun ./📜️script.ts verify dependencies` | clean, 238 = 238, no new deps |
| `cargo check --workspace --all-targets --exclude semio-compose-rs` (run twice, non-deterministic per the brief's caveat) | Neither `semio-framework-ui-backend-vulkan` nor `semio-framework-ui-backend-d3d12` appears in `could not compile` in either run. Both runs still show (unrelated, out-of-scope): `semio-framework-machine` (needs its own packet per R3 §"Cross-boundary findings"), `semio-hub`, `semio-framework-ui-backend-webgpu` (sibling packet's territory, confirmed unfixed per §4), `semio-framework-ui` lib-test (pre-existing `Label` gate, per R1), and `semio-compose-rs` (explicitly out of scope, and `--exclude` does not seem to stop it being checked as some other crate's path dependency — not investigated, out of scope). |

`rustfmt --check --config-path ./rustfmt.toml` on the two edited `📦️glue.rs` files shows pre-existing
formatting drift unrelated to my edits (confirmed by running the same check against the pre-edit `HEAD`
version of Vulkan's `📦️glue.rs` — identical diff, e.g. `mod` reordering) — consistent with R1's/R3's
documented precedent of not taking a wholesale-reformat diff on a live, concurrently-edited tree. Left
unformatted; only the literal edits described above were made.

## 6. Category-C / long-running CPU work — Phase 5 seed notes

D3D12's own async census contribution is nil (2 fns, both construction, already correctly async), but the
crate is full of exactly the kind of run-to-completion CPU work Phase 5 targets for worker-job slicing
under an 8 ms/step ceiling — every function already carries the `🚫️async: U1 run-to-completion frame
transaction` marker, which doubles as a Phase 5 seed list. Largest/most relevant, all in `🦀️backend.rs`:

- `render` (~40 lines) — the per-frame top-level transaction: fence wait, command-list reset, two encode
  passes, present. The natural top-level cursor/resumption boundary for Phase 3/5's enqueue-only model.
- `run_blur_chain` (~50 lines, nested `for` loops over `SCENE_MIP_LEVELS`) — 5-level mip blur chain, one
  `CopyTextureRegion` + one draw per mip; classic chunkable CPU/GPU-encode loop.
- `encode_scene_pass`/`encode_composite_pass` (~45/~25 lines) — the two per-frame device passes this
  backend's whole render is split into (see `🦀️backend.rs`'s header for why the split exists).
- `encode_2d_batches` (~40 lines, `for` over `DrawBatch`es with a nested `match`) — draw-call replay/
  sorting loop, same "GPU upload-packet preparation" shape R5 flagged for webgpu's `replay_batches`.
- `capture_readback` (~25 lines, `backend-testing` only) — CPU-side pixel copy/readback prep.

None of these currently suspend or block (confirmed: 0 real `.await` in the crate) — plain synchronous
CPU/encode passes over already-resolved `RenderPacket` state, exactly the shape Phase 5 needs to slice
into resumable steps.

## 7. Files touched

- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/📦️glue.rs` — gated every
  `mod`/`pub use` behind `#[cfg(target_os = "windows")]` (Task 2); removed the top-level
  `compile_error!`, replaced with an updated header documenting the cfg-gated-empty-lib approach
  (Task 3).
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/📦️glue.rs` — removed the
  top-level `compile_error!`, updated header, identical reasoning to D3D12's (Task 3). `mod` gating was
  already correct here (R3), unchanged.
- No other file in either crate was touched. No async fn was converted (none needed it — see §1).

## 8. Cross-boundary findings for the coordinator

- **`semio-framework-ui-backend-webgpu`** still blocks `cargo check --workspace` on macOS via the
  identical structural issue (workspace member + unconditional `compile_error!` + ungated `mod`s) —
  see §4's coordination note for the exact minimal fix, not applied here (sibling's territory).
- **`semio-framework-ui-backend-metal`** carries the same `compile_error!` pattern and would need the
  identical fix to stay green on a non-macOS host — not currently blocking anything on this (macOS) host,
  not touched, flagged for whoever picks up cross-platform workspace-green work next.
- **`semio-framework-machine`**, **`semio-hub`**, **`semio-framework-ui` (lib test)** all still appear in
  `could not compile` — all previously flagged by R3/R1 as out-of-scope, unrelated bug classes (see those
  reports); reconfirmed still present, unrelated to this packet's edits.
