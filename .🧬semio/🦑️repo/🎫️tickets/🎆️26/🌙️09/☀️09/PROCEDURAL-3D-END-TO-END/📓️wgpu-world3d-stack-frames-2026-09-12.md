# 🧵️ wgpu `World3d` engine-surface tests overflow the default thread stack — root cause, fix, gates (2026-09-12)

Closes `📓️audit-hover-selection-2026-09-12.md` §6 / §8 gap #2. Methodology is the one
`📓️selection-overflow-2026-09-09.md` established: reproduce at the platform default stack, read the
`sub sp` prologue reservations out of the linked binary with LLVM `objdump -d` (a third-party
instrument, outside this codebase), fix the dominant frame structurally, then re-measure.

No `RUST_MIN_STACK` bump, no `stacker`, no bigger thread anywhere. Shared cargo build dir throughout
(no `CARGO_TARGET_DIR`), every run in the foreground.

## 0. TL;DR

| | before | after |
|---|---|---|
| **the failure** | all four `engine_canvas::engine_surface_attach_tests` abort with `fatal runtime error: stack overflow` on a 2 MiB thread; green in every gate only because `runCargoTestBudgeted` floors `RUST_MIN_STACK` at 128 MiB | the three `world3d_*` laws pass at the **default** stack |
| **the cause** | **not recursion, not a generator.** `Box::new([const { None }; N])` materialises the whole array in the CALLER's frame at `opt-level = 0`, then moves it into the allocation | the allocation is grown first and sealed into the fixed array — a pointer cast |
| **the dominant frame** | `<scenes::AdmittedSurfaceMap<World3dState> as Default>::default` reserved **5 670 928 bytes** in one frame — 2.7× the whole 2 MiB stack, hit on the very first statement of `attach_and_settle_world3d` | **51 168** (−99.1 %) |
| **the second frame** | `<engine_canvas::EngineSurfaceRegistry as Default>::default` reserved **16 779 328 bytes** (`Box::new(std::array::from_fn(…))`, 256 × `EngineSurfaceSlot`) | **71 680** (−99.6 %) |
| **measured requirement** | > 2 048 KiB (aborted) | between **512 KiB** (aborts) and **1 024 KiB** (ok) for all three laws together |
| **guard** | none | `world3d_engine_surface_laws_fit_a_bounded_thread_stack` — runs the three laws on a `std::thread::Builder` lane with an explicit **2 MiB** stack, which overrides the 128 MiB floor |

Raw logs and the two instruments: `🗑️generated/wgpu-stack/` (the instruments are also kept outside the
generated folder as `🐍️wgpu-frame-sizes.py` and `🐍️wgpu-stack-baseline-toggle.py`).

---

## 1. Reproduction

```
$ env -u RUST_MIN_STACK cargo test -p semio-framework-os-renderer-wgpu --lib -- \
      world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload --test-threads=1
running 1 test
test engine_canvas::engine_surface_attach_tests::world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload ...
thread '…world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload' (14914668) has overflowed its stack
fatal runtime error: stack overflow, aborting
… (signal: 6, SIGABRT: process abort signal)
```
`🗑️generated/wgpu-stack/01-repro-orbit.txt`. The other two named tests and the module's fourth test
(`tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam`) abort identically.

### 1.1 The macOS crash report names the thread, not (usefully) the chain

`🗑️generated/wgpu-stack/02-crash.ips`, faulting thread 1:

```
exception   EXC_BAD_ACCESS / KERN_PROTECTION_FAILURE at 0x16ba9fb20
vmRegionInfo  Stack Guard 16ba9c000-16baa0000 [16K] --- ; Stack 16baa0000-16bca8000 [2080K]
frame 7     …engine_surface_attach_tests::attach_and_settle_world3d + 104
frame 9     …world3d_orbit_and_wheel_emit_the_generation3d_set_camera + 240
```

Two facts from that: libtest's assertion thread owns exactly **2 MiB** (`2080K` region, 16 KiB guard),
and the fault address lies **1 248 bytes before the stack base** — the descent consumed the whole
region. The frame-pointer walk is otherwise useless (it lists only three user frames for 2 MiB of
stack), because the interrupted frame is mid-prologue-probe. `attach_and_settle_world3d + 104` is the
*return address* of the first `bl` in that function, which is what actually located the culprit.

### 1.2 The prologue says which call it was

`objdump -d --disassemble-symbols=…attach_and_settle_world3d`:

```
10044ff38: stp  x28, x27, [sp, #-0x20]!
10044ff44: sub  x9, sp, #0x2e, lsl #12    ; = 0x2e000        ← 188 416 bytes, its own frame
10044ff48: sub  sp, sp, #0x1, lsl #12                        ← probe loop
10044ff58: sub  sp, sp, #0x130
10044ff9c: bl   <…scenes::AdmittedSurfaceMap<World3dState> as Default>::default>
10044ffa0: …                                                 ← +104, the crash-report frame
```

and the callee:

```
0000000100324198 <…AdmittedSurfaceMap<…World3dState> as …Default>::default>:
1003241a4: sub  x9, sp, #0x567, lsl #12   ; = 0x567000       ← 5 664 768 bytes
1003241b8: sub  sp, sp, #0x7f0                               ← + 2 032
```

**5 670 928 bytes in one frame**, entered when ~340 KiB of the 2 MiB was already spent. `5 664 768 =
256 × 22 128` — exactly one `[Option<AdmittedSurfaceEntry<World3dState>>; SCENE_SURFACE_CAPACITY]`
laid out on the stack. Not a sum of many temporaries (that was the 2026-09-09 packet's shape), and
not a generator: these tests are plain synchronous functions.

Full per-symbol reservation table for the binary (148 797 symbols):
`🗑️generated/wgpu-stack/03-frames-before.tsv`, produced by `🐍️wgpu-frame-sizes.py`.

---

## 2. Root cause

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:99`
(pre-fix):

```rust
impl<T> Default for AdmittedSurfaceMap<T> {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; SCENE_SURFACE_CAPACITY]), … }
    }
}
```

`Box::new(<array literal>)` / `Box::new(std::array::from_fn(…))` is a heap **destination** with a
stack **source**. The array expression is a value: it is built in the calling frame and *then* moved
into the allocation. At `opt-level = 0` — `dev`, `test` and `wasm-dev`, where LLVM neither colours
nor reuses stack slots — that costs `N * size_of::<T>()` of stack, however big. With
`SCENE_SURFACE_CAPACITY = 256` (`:33`) and `World3dState` at ~22 KiB per slot, that is 5.4 MiB.

The same shape, same file/module, three more times:

| site | array | reserved (aarch64 debug) |
|---|---|---|
| `🎞️Scenes/…/🦀️.rs:99` `AdmittedSurfaceMap::<World3dState>::default` | `[Option<AdmittedSurfaceEntry<World3dState>>; 256]` | **5 670 928** |
| `⚙️EngineCanvas/…/🦀️.rs:160` `EngineSurfaceRegistry::default` | `[EngineSurfaceSlot; 256]` | **16 779 328** |
| `🎞️Scenes/…/🦀️.rs:408` `PendingRasterQueue::default` | `[Option<PreparedRasterProducer>; 16]` | 4 096 (via `AdmittedSurfaceMap<PendingRasterSurface>`, 221 968) |
| `⚙️EngineCanvas/…/🦀️.rs:1203` `EngineCanvasPresenter::default` | `[EngineGpuSlot; 256]` | not emitted in the lib-test binary (GPU-gated), changed for the same reason |

`EngineSurfaceRegistry::default` is the one that still aborted the fourth test after the first fix
(`🗑️generated/wgpu-stack/06-module-after.txt` + its crash report): 16 MiB in one frame, on a stack of 2.

---

## 3. The fix

One helper, at the module that owns the fixed-slot-map vocabulary
(`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:92`, `pub(crate)` so `engine_canvas` reaches it):

```rust
pub(crate) fn boxed_fixed_slots<T, const N: usize>(fill: impl FnMut() -> T) -> Box<[T; N]> {
    let mut slots: Vec<T> = Vec::with_capacity(N);
    slots.resize_with(N, fill);
    slots.into_boxed_slice().try_into().ok().expect("fixed slot credits seal into their own array")
}
```

`Vec::with_capacity` allocates once on the heap and `resize_with` writes each slot straight into it,
so the deepest temporary is **one** slot; `Box<[T]> → Box<[T; N]>` is `TryFrom`, a pointer cast with
no copy. The field types, the `SCENE_SURFACE_CAPACITY`/`ENGINE_SURFACE_CAPACITY` credit invariants and
every byte of the initial state are unchanged — the four `Default` bodies produce exactly the value
they produced before. No external dependency; `Vec`/`Box`/`TryFrom` only, target-neutral.

Four call sites now use it: `AdmittedSurfaceMap::default`, `PendingRasterQueue::default`,
`EngineSurfaceRegistry::default`, `EngineCanvasPresenter::default`.

**Measured after** (`🗑️generated/wgpu-stack/08-frames-after.tsv`, same instrument, same binary):

| frame | before | after |
|---|---|---|
| `AdmittedSurfaceMap::<World3dState>::default` | 5 670 928 | **51 168** |
| `EngineSurfaceRegistry::default` | 16 779 328 | **71 680** |
| `AdmittedSurfaceMap::<PendingRasterSurface>::default` | 221 968 | **4 832** |
| `PendingRasterQueue::default` | 4 096 | **208** |

The `resize_with` fill loops that replace them are 139 328 B (`EngineSurfaceSlot`) and 48 176 B
(`AdmittedSurfaceEntry<World3dState>`) — one or two slots' worth, transient, and only while filling.

### 3.1 Prior art in this repo — and why the helper is a fifth copy, not a first

`🌉️mcp/🚚️transport/🦀️.rs:439` and `🌉️mcp/🧵️bridge/🦀️.rs:1610` already carry an identical `boxed_slot_ring`
helper each, with doc comments naming this exact defect ("*proven: this is what `bridge::long`/
`transport::quick`'s stack overflow traced back to, not any recursive call*"). Those two are byte-for-byte
duplicates of each other. There is no shared home: `semio-framework` (the root crate) depends on
`semio-framework-ui`, so `ui_wgpu` cannot reach it, and `semio-framework-os-infinite` sits below the
renderer. **One `boxed_fixed_slots` in a crate every one of them depends on is the clean long-term
answer, and it is not in this packet's scope** — that is a repo-wide refactor across crates other
lanes are editing right now. Recorded in §7 as the follow-up.

---

## 4. Where production runs this, and whether the fixed frames were reachable there

Asked explicitly by the brief. Answered from source, statically — no dev server or native run was
started.

### 4.1 Native runner — the World3d interaction/frame loop is on a **2 MiB pool worker**, not the main thread

- `⌨️native-entrypoint/🦀️.rs:114` → `run_native` (`🧊️renderer/🦀️.rs:14062`) →
  `event_loop.run_app(&mut app)`: winit's loop owns the **process main thread** (8 MiB on macOS and
  Linux; 1 MiB on Windows unless the linker says otherwise). `ShellState::new` — the one production
  caller of `AdmittedSurfaceMap::default`, four times, at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2851-2857` —
  runs there, during boot.
- The **frame loop does not**. `FrameBuildHandle::poll_runtime_and_resubmit`
  (`🧵️frame-job/🦀️.rs:474`, `cfg(not(target_arch = "wasm32"))`) does
  `session.try_submit_step(&crate::renderer_worker_pool(), Lane::Interactive)`, handing
  `ActiveFrameBuild` to another thread. `ActiveFramePhase::Build` (`🧵️frame-job/🦀️.rs:300`) steps
  `crate::FrameTransaction`, whose `AppFrameTransactionPhase::World3dSnapshot`
  (`🧊️renderer/🦀️.rs:11395`) calls `step_world3d_interaction` (`🧊️renderer/🦀️.rs:11474`) — the exact
  authority the three tests drive.
- Those threads: `WorkerPool` spawns them at `🧰️framework/🔨️modules/⏳️async/🦀️.rs:1780` with
  `thread::Builder::new().name(format!("semio-pool-worker-{index}")).spawn(…)` — **no
  `.stack_size()`**, so Rust's default 2 MiB. `RUST_MIN_STACK` is not set for the run either:
  `nativeRunnerEnvironment` (`…🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:66`) only strips credential keys.
  **The production frame loop gets exactly the 2 MiB budget the failing tests had.**
- Reachability of the 16 MiB frame there: `ENGINE_SURFACES` is
  `static … : WorkerCell<EngineSurfaceRegistry>` (`⚙️EngineCanvas/…/🦀️.rs:1590`), and `WorkerCell` is a
  process-wide `OnceLock<Mutex<T>>` whose own doc says it is for "renderer state that may resume on
  any shared-pool worker" (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:27-43`). It is initialised by whichever
  thread first calls `.with(…)` — and the scene paint that does (`sync_engine_scene`, reached from
  `render_component_scene_step` via `🗣️Interpreter/…/🦀️.rs:1067`) runs inside the frame job, i.e. on a
  pool worker. So `EngineSurfaceRegistry::default`'s 16 779 328-byte frame was reachable on a 2 MiB
  thread in the **native dev** profile: 8× over budget. This is a static reachability argument plus a
  measured frame, not an observed crash — I did not launch the renderer. `native-build-release` builds
  `--release`, where LLVM may build the array in place, so the claim is scoped to the unoptimized
  native profile.
- `AdmittedSurfaceMap::<World3dState>::default`'s 5.4 MiB, by contrast, only ran on the boot/main
  thread in production (8 MiB on macOS/Linux — under budget; **over** the 1 MiB Windows main-thread
  default). It is the test thread's 2 MiB that it killed.

### 4.2 wasm32 browser — 16 MiB shadow stack, and the stack-bump workaround is already in the tree

- The browser frame loop does **not** go through a pool: `poll_and_resubmit`'s wasm half
  (`🧵️frame-job/🦀️.rs:557-570`) calls `session.try_step_on_caller()` inside the dedicated
  `semio-frame-worker` isolate, so the World3d authority runs on that module's wasm shadow stack.
- That stack is **16 777 216 bytes**, set in `.cargo/config.toml`:
  `[target.wasm32-unknown-unknown] rustflags = [… '-C', 'link-arg=-zstack-size=16777216']`. Its
  comment documents this very defect class and its own workaround: *"LLVM's wasm-ld default is 1 MiB,
  and a debug build of the wgpu renderer overruns it during ordinary construction —
  `<UiSurfaceRegistry as Default>::default` alone materialises `[Option<UiSurfaceSlot>; 64]` on the
  stack … Measured: every `semioWgpuWorkerBootstrap` died there until this flag was set."* That is the
  stack-bump fix this packet is forbidden to repeat, already in the tree, for a sibling site.
  (Plugin components are a separate 8 MiB budget, `PLUGIN_WASM_STACK_BYTES`.)
- Were the fixed frames reachable there? Yes, by the same paint path — but I am **not** claiming they
  trapped: every byte figure above is aarch64, and wasm32's 32-bit pointers make the same arrays
  materially smaller. What is fair to say is that `EngineSurfaceRegistry::default` at 16 779 328 B
  measured *above* the entire 16 MiB shadow-stack budget on native, so the margin was at best nil,
  and the fix removes it either way.

---

## 5. The guard

`⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:371`
`world3d_engine_surface_laws_fit_a_bounded_thread_stack`.

The three laws were refactored into surface-id-keyed plain functions (`world3d_preview_window_law`,
`world3d_pointer_down_law`, `world3d_orbit_and_wheel_law`) so they can run twice in one binary without
sharing a surface; each `#[test]` is now a one-line call, and the guard calls all three on

```rust
std::thread::Builder::new().name("world3d-bounded-stack".to_string()).stack_size(2 * 1024 * 1024).spawn(…)
```

`Builder::stack_size` **overrides** `RUST_MIN_STACK`, which is the whole point: the repo runner's
blanket 128 MiB floor (`runCargoTestBudgeted`, `🦑️repo/…/📚️library/📦️packages/🟦️typescript/🟦️.ts:1663`,
whose own doc says *"because app-fixture laws routinely exceed libtest's 2 MiB default stack"*) cannot
hide a re-inflated frame on this route again. It is fixture-driven (the committed `🔣️.json`
engine-surface fixture pinned against React's `World3dHost.tsx`) and behavioural — it asserts nothing
about source shape, so any future regrowth fails it.

**It bites.** Threshold sweep on the same binary, the guard's `stack_size` temporarily varied and
restored:

| `stack_size` | result | log |
|---|---|---|
| 512 KiB | `thread 'world3d-bounded-stack' has overflowed its stack` | `🗑️generated/wgpu-stack/11-sweep-512kib.txt` |
| 1 024 KiB | ok | `🗑️generated/wgpu-stack/10-sweep-1mib.txt` |
| 2 048 KiB (shipped) | ok | `🗑️generated/wgpu-stack/09-guard.txt` |

So the three laws now need under 1 MiB where they needed over 2 MiB, and the guard runs with ~1 MiB of
headroom. It also failed, as a `SIGABRT`, on the reverted tree in §6.2 — under the 128 MiB floor —
which is the property being bought.

### 5.1 TypeScript twin — there is none, deliberately

The law is "this descent fits a fixed OS-thread stack". JavaScript has no per-frame stack reservation,
no `-zstack-size`, and no `Thread::stack_size`; React's `World3dHost.tsx` cannot express or violate it.
The *fixture* is genuinely language-agnostic and already carries its provenance, but a TS twin over it
would assert **payload shape** (`interactionSelect` / `setCamera` args) — which is `📓️audit-hover-selection-2026-09-12.md`
§8 gap #3, a different, still-open item owned elsewhere — not this law. The independent oracle for
*this* law is not a second language but the LLVM toolchain's own machine code: the `sub sp` prologue
read out of the linked binary with `objdump` statically measures the same number the guard exercises
dynamically, and the two agree on every row of §0 and §3.

---

## 6. Gates

### 6.1 The three named tests, default stack, `RUST_MIN_STACK` unset

```
$ env -u RUST_MIN_STACK cargo test -p semio-framework-os-renderer-wgpu --lib -- --exact \
    …world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload \
    …world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches \
    …world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid \
    …world3d_engine_surface_laws_fit_a_bounded_thread_stack --test-threads=1

running 4 tests
test engine_canvas::engine_surface_attach_tests::world3d_engine_surface_laws_fit_a_bounded_thread_stack ... ok
test engine_canvas::engine_surface_attach_tests::world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload ... ok
test engine_canvas::engine_surface_attach_tests::world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches ... ok
test engine_canvas::engine_surface_attach_tests::world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 494 filtered out; finished in 0.02s
```
`🗑️generated/wgpu-stack/18-final-default-stack.txt` (`RUST_MIN_STACK` verified unset in the shell).

### 6.2 Whole crate through its own Nx target — no regression, proven against a baseline

`📋️project.json`'s `test-native` → `bun ./📜️script.ts test-native` → `runCargoTestBudgeted`
(cargo-nextest, 128 MiB floor, the repo's real gate):

```
$ bun nx run @semio-tech/framework-renderer-wgpu:test-native --no-fail-fast
Summary  498 tests run: 465 passed, 33 failed, 0 skipped
```

The crate is broadly red from concurrent peer churn, so the 33 were measured against a **baseline**:
the four call sites were toggled back to their pre-fix form with `🐍️wgpu-stack-baseline-toggle.py`
(exact-string replacement, never a file snapshot, so a peer's unrelated edits in the same files
survive), the identical command re-run, then toggled back:

```
baseline (fix off) : 498 tests run: 464 passed, 34 failed
fixed              : 498 tests run: 465 passed, 33 failed

only in baseline (i.e. fixed by this change):
  SIGABRT engine_canvas::engine_surface_attach_tests::world3d_engine_surface_laws_fit_a_bounded_thread_stack
only in fixed (i.e. regressions):
  <none>
```

`🗑️generated/wgpu-stack/14-nx-nofailfast.txt`, `15-failing-tests.txt`, `17-baseline-off.txt`.
**Zero regressions**, and the guard is the only test the fix flips. Note the three `world3d_*` laws
and the module's fourth test are green in *both* columns under the 128 MiB floor — exactly the masking
the audit reported.

The 33 pre-existing failures are other lanes': 13 `interpreter::ui_command_wiring_tests::*` SIGABRTs
(see §7), and assertion failures in `deadlines` (`caret_blink…`: `Some(0.5)` vs `Some(1.0)`), `dock`,
`frame_job`, `shell::*`, `scenes::text_editor_tests`, `kernel_runtime`, `runtime_publication_tests`,
plus four source-text contract tests over files this packet never opened —
`async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied` (its contract takes
`LIBRARY/PREPARED/GPU/DRAW/OS_HOST/WINIT_APP` sources; `ENGINE_CANVAS_SOURCE` is not even an
argument), `raster_upload_cache_…` (panics slicing `✍️draw.rs` at `🔬️wgpu-renderer-async-boundary/🦀️.rs:77`
— "byte range starts at 178536 but ends at 68475", a peer reordering in `semio-framework-ui`),
`native_binary_owns_exactly_one_entrypoint_driver` (reads `⌨️native-entrypoint/🦀️.rs`), and
`renderer_asset_probe_…` (aborts in `os-infinite`'s `WorldAssetFetchOwner::drop`).
`🗑️generated/wgpu-stack/13-preexisting-failures.txt`.

### 6.3 Browser target

```
$ cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going
warning: `semio-framework-os-renderer-wgpu` (lib) generated 24 warnings
    Finished `dev` profile [unoptimized] target(s) in 1m 39s
```
0 errors. Warnings are reported because a zero error count proves nothing if expansion aborted — 24
warnings means this was a real type-check. `🗑️generated/wgpu-stack/19-wasm-check.txt`. None of the 24 is
this packet's (all pre-existing dead-code notices in `os-host`, `render-snapshot`, `🐚️Shell`).

---

## 7. Still red / follow-ups — attribution, not this packet's

1. **`tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam` still aborts at the
   default stack** (it is green in the gate, under the floor). After this packet's two fixes its next
   frame is `<os_infinite::canvas::renderer::OpaqueSceneRetirementRegistry as Default>::default` —
   `♾️infinite/🖼️canvas/🦀️.rs:658`, `Box::new(std::array::from_fn(…))` over
   `OPAQUE_SCENE_RETIREMENT_CAPACITY = 1024`, measured **962 640 bytes** — reached through
   `reserve_opaque_scene_retirement` ← `BoardHost::quarantine_world_content_step`. Different crate
   (`semio-framework-os-infinite`), one line, same `boxed_fixed_slots` shape; not one of the three laws
   this packet owns, and chasing it means a fifth private copy of the helper. Left for the shared-home
   refactor below.
2. **`semio-framework-ui`'s `UiSurfaceRegistry` is the biggest remaining instance and is overflowing
   even the 128 MiB floor.** `<ui_wgpu::wgpu::engine::UiSurfaceRegistry as Default>::default` reserves
   **11 929 152** bytes and its `core::array::try_from_fn::<…UiSurfaceSlot, 64>` **16 780 368**; the
   `WorkerCell<Ui>::state` → `OnceLock::get_or_init` → `Mutex::new` → `Ui::new` chain stacks six such
   frames back to back (~88 MiB). That is almost certainly what the 13
   `interpreter::ui_command_wiring_tests::*` SIGABRTs in §6.2 are, and it is the site
   `.cargo/config.toml`'s 16 MiB `-zstack-size` comment already names by hand. Worth its own packet.
3. **One shared `boxed_fixed_slots`.** Five private copies of this idea now exist (`🌉️mcp/🚚️transport`,
   `🌉️mcp/🧵️bridge`, this packet's one, and the two sites §7.1/§7.2 still need). CLAUDE.md's "if code is
   repeated it MUST be close to each other" wants one home in a crate below `semio-framework-ui`,
   `semio-framework-os-infinite` and the renderer — and every `Box::new([…; N])` / `Box::new(array::from_fn(…))`
   in the tree audited against `N * size_of::<T>()`. ~45 further sites exist; most are harmless (small
   element types), which is exactly why an audit rather than a sweep is the right shape.

---

## 8. Files changed

| path | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | **new** `pub(crate) boxed_fixed_slots<T, const N>` (`:92`); `AdmittedSurfaceMap::default` (`:100`) and `PendingRasterQueue::default` (`:408`) build their slot arrays through it |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | `EngineSurfaceRegistry::default` (`:160`) and `EngineCanvasPresenter::default` (`:1203`) likewise |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs` | three laws extracted into surface-id-keyed `…_law` fns, their `#[test]`s reduced to one-line calls; **new** `world3d_engine_surface_laws_fit_a_bounded_thread_stack` (`:371`) |
| `.🧬semio/🦑️repo/🎫️tickets/…/PROCEDURAL-3D-END-TO-END/🐍️wgpu-frame-sizes.py` | **new** instrument — per-symbol `sub sp` prologue reader over `objdump -d` |
| `.🧬semio/🦑️repo/🎫️tickets/…/PROCEDURAL-3D-END-TO-END/🐍️wgpu-stack-baseline-toggle.py` | **new** instrument — exact-string fix on/off toggle for the §6.2 baseline |

No generation3d, `infinite`, `ui` or plugin source was changed. No `RUST_MIN_STACK`, `launch.json`,
`project.json`, `.cargo/config.toml` or thread-stack setting was touched anywhere.
