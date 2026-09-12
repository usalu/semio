# 🧱️ One shared `boxed_fixed_slots`, the two remaining megabyte frames, and a tree-wide slot-table audit (2026-09-12)

Closes the three follow-ups `📓️wgpu-world3d-stack-frames-2026-09-12.md` §7 left open. Same methodology
as that packet: reproduce at the platform default stack, read `sub sp` prologue reservations out of the
linked binary with LLVM `objdump -d` (third-party oracle, outside this codebase), fix structurally,
re-measure. No `RUST_MIN_STACK` bump, no `stacker`, no bigger thread anywhere, shared cargo build dir
throughout (no `CARGO_TARGET_DIR`), every run in the foreground.

Raw logs: `🗑️generated/boxed-slots/`. Instruments: `🐍️boxed-slot-sites.py` (new, tree-wide site
inventory joined to measured frames), `🐍️boxed-slots-baseline-toggle.py` (new, three-lane fix on/off
toggle), `🐍️wgpu-frame-sizes.py` and `🐍️wgpu-stack-baseline-toggle.py` (the prior packet's, the latter
repaired for the moved helper).

## 0. TL;DR

| | before | after |
|---|---|---|
| **the helper** | 3 private copies of the same idea (`🌉️mcp/🚚️transport`, `🌉️mcp/🧵️bridge`, `🎞️Scenes/…/🧊️wgpu`) | one `semio_framework_async::{boxed_slots, boxed_fixed_slots}`; **zero** private copies remain |
| **`tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam`** | aborted at the default 2 MiB stack (`OpaqueSceneRetirementRegistry::default`, 962 640 B) | **ok** at the default stack |
| **the 13 `interpreter::ui_command_wiring_tests::*`** | 13 SIGABRTs (`UiSurfaceRegistry`'s 11 929 152 B frame, six stacked ≈ 88 MiB) | **all 19 run, all pass** at the default stack — 12 SIGABRTs fixed, the 13th's newly-unmasked assertion fixed too (§4) |
| **largest construction frame in the renderer test binary** | `core::array::try_from_fn::<Option<RetainedSurfaceSlot>, 64>` = **4 447 824 B**, over twice the 2 MiB a pool worker owns | **50 560 B** (−98.9 %) |
| **sites ≥ 64 KiB in the whole tree** | 25 | **10**, all in crates this packet deliberately did not open (§6.3) |
| **guard** | none beyond the prior packet's one law | a committed budget fixture + **6** per-crate Rust guards on explicit 1 MiB threads + a Rust-free TypeScript twin |

---

## 1. The shared home: `semio-framework-async`

`boxed_fixed_slots` now lives once, at `🧰️framework/🔨️modules/⏳️async/🦀️.rs` region `🧱️BoxedSlots`:

```rust
pub fn boxed_slots<T>(len: usize, fill: impl FnMut() -> T) -> Box<[T]>
pub fn boxed_fixed_slots<T, const N: usize>(fill: impl FnMut() -> T) -> Box<[T; N]>
```

`boxed_fixed_slots` delegates to `boxed_slots` and seals the result with `TryFrom` (a pointer cast), so
there is exactly one `Vec::with_capacity` + `resize_with` in the tree. `Vec`/`Box`/`TryFrom` only — no
external dependency, target-neutral (checked on `wasm32-unknown-unknown` and `wasm32-wasip2`).

### 1.1 Why `⏳️async` and not a new module

The brief asked for "the lowest common dependency-free framework module". The candidates that every
consumer already depends on are `semio-framework-trace` (diagnostics/clocks) and
`semio-framework-async` (which depends only on trace). `⏳️async` wins on domain, not just on depth: it
is the crate whose own docstring calls it "the process-wide CPU substrate", it owns `WorkerPool` — the
thing that spawns the 2 MiB OS threads these tables were overflowing (`native_pool` calls
`thread::Builder::new()…spawn` with **no** `.stack_size()`) — and it is therefore the one module where
"a fixed slot table must not be built in the caller's frame" is a statement about its own subject
matter. A brand-new `🧱️memory` module would have been a third option; it was rejected because it buys
no isolation (`⏳️async` pulls in nothing but `⏱️trace`) while costing a workspace member, a
`🔒️dependencies.json` row, a `🔣️taxonomy.json` row, a `📋️project.json`/`📜️script.ts` pair and launch
entries — churn in exactly the shared files other lanes are editing right now.

Two crates needed a new **first-party** edge for this (`semio-framework-os-infinite`,
`semio-framework-actor`); in both, `semio-framework-async` was already in the build graph transitively
via `semio-framework-job`, and in both it was already a direct dependency under
`cfg(all(target_arch = "wasm32", not(target_env = "p2")))` — the change makes it unconditional and the
manifests say why. No crate gained a new external dependency.

### 1.2 The copies that are gone

| copy | call sites | now |
|---|---|---|
| `🌉️mcp/🚚️transport/🦀️.rs:439` `boxed_slot_ring` | 2 | `semio_framework_async::boxed_slots(N, \|\| None)` |
| `🌉️mcp/🧵️bridge/🦀️.rs:1610` `boxed_slot_ring` (byte-for-byte identical to the above) | 3 | same |
| `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:92` `pub(crate) boxed_fixed_slots` | 4 (incl. 2 in `⚙️EngineCanvas`) | `semio_framework_async::boxed_fixed_slots` |

`grep -rn "boxed_slot_ring" --include="*.rs"` now returns nothing.

---

## 2. `OpaqueSceneRetirementRegistry::default` — follow-up §7.1

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs:658` built
`[OpaqueSceneRetirementSlot; OPAQUE_SCENE_RETIREMENT_CAPACITY]` (1024 × 312 B = **319 488 B**) through
`Box::new(std::array::from_fn(…))`; measured frame **962 640 B**, reached through
`reserve_opaque_scene_retirement` ← `BoardHost::quarantine_world_content_step`. Now
`semio_framework_async::boxed_fixed_slots(…)`; the owner is **16 B**.

```
$ env -u RUST_MIN_STACK cargo test -p semio-framework-os-renderer-wgpu --lib -- \
      engine_surface_attach_tests ui_command_wiring_tests heap_first --test-threads=1
running 27 tests
test engine_canvas::engine_surface_attach_tests::tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam ... ok
…
test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 474 filtered out; finished in 0.05s
```
`🗑️generated/boxed-slots/28-final-default-stack.txt` (`RUST_MIN_STACK` verified unset in the shell).

---

## 3. `UiSurfaceRegistry` — follow-up §7.2, and the 13 SIGABRTs

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/⚙️engine.rs:209` held its slot table
**inline**:

```rust
struct UiSurfaceRegistry {
    slots: [Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS],   // 64 × 186 344 B = 11 926 016 B
    generations: [u64; UI_LAYOUT_SURFACE_SLOTS],
}
```

That is a strictly worse shape than the `Box::new([…])` sites the prior packet fixed: the array is not
only materialised by `array::from_fn` in the constructing frame, it **is** the struct, so every
by-value move of the owning `Ui` copies all 11.9 MB again — which is why the
`WorkerCell<Ui>::state` → `OnceLock::get_or_init` → `Mutex::new` → `Ui::new` chain stacked six such
frames (~88 MiB) and took the whole `interpreter::ui_command_wiring_tests` module down even under
`runCargoTestBudgeted`'s 128 MiB floor.

Fix: `slots: Box<[Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS]>` built by `boxed_fixed_slots`. All
indexing/iteration is unchanged (`Box<[T; N]>` derefs to the array). Measured: the registry is now
**520 B** (one pointer + the 512 B generation array), against 11 926 528 B before.

```
running 24 tests
test interpreter::ui_command_wiring_tests::apply_drop_committed_is_a_no_op_without_a_decodable_semio_payload ... ok
… all 19 ui_command_wiring tests ok …
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 474 filtered out; finished in 0.14s
```

`UiSurfaceRegistry::try_admit` still reserves 563 840 B, because it builds one `UiSurfaceSlot`
(`UiWindow::new` alone is 190 016 B) on the stack before moving it into the boxed slot. That is one
slot, not 64, and it fits the 2 MiB budget with room; it is recorded in §6.3 rather than chased here.

---

## 4. The 13th test: a defect the SIGABRT was hiding

With the stack fixed, `window_action_context_retained_commands_preserve_the_clicked_window` ran for the
first time — and failed on `Number(7.0)` vs `Number(7)`. Not a stack problem and not this packet's
change; the cause is that `ui_wgpu::wgpu::action`'s bounded action ring flattened **every** number to
`f64`:

```rust
enum FlatValue { …, Number(f64), … }                     // 🧾️action.rs:50
DslValue::Number(value) => self.number(key, value.as_f64())   // copy_value, :284
FlatValue::Number(value) => Ok(DslValue::float(value))        // materialize_node, :108
```

`DslValue`↔`serde_json`↔`pack::json` all preserve `UInt`/`Int`/`Float` variant-for-variant (their own
doc comments say so explicitly); the action ring was the one lossy hop. `FlatValue::Number` now carries
`dsl::Number`, the public `BoundedActionBuilder::number(f64)` keeps its `f64` signature (it constructs
`Number::Float`), and `copy_value` pushes the exact variant. `@semio-tech/ui-rs:test` stayed green
(237/237) across that change.

---

## 5. The guard

### 5.1 One committed budget

`🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` — language-agnostic, exported to
Rust as `semio_framework_async::BOXED_FIXED_SLOTS_FIXTURE` so no guard carries a `../../..` path:

```json
{ "boundedThreadStackBytes": 1048576, "workerPoolStackBytes": 2097152, "conversionThresholdBytes": 65536,
  "tables": [ { "guard": "…", "crate": "…", "owner": "…", "capacityConstant": "…", "capacity": 64,
                "elementType": "…", "elementSizeBytes": 186344, "ownerSizeBytes": 520 }, … ] }
```

| guard | owner | capacity × slot | inline would be | owner is |
|---|---|---|---|---|
| `renderer::scenes` | `scenes::AdmittedSurfaceMap<World3dState>` | 256 × 21 952 | 5 619 712 | 47 008 |
| `renderer::engine_canvas` | `engine_canvas::EngineSurfaceRegistry` | 256 × 67 528 | 17 287 168 | 16 |
| `renderer::engine_canvas` | `engine_canvas::StagedEngineScenes` | 256 × 352 | 90 112 | 24 |
| `renderer::engine_canvas` | `engine_canvas::EngineCanvasBuildContext` | 256 × 384 | 98 304 | 72 |
| `renderer::kernel_runtime` | `kernel_runtime::RetainedSurfaceRegistry` | 64 × 23 144 | 1 481 216 | 32 |
| `renderer::kernel_runtime` | `kernel_runtime::CommandDocumentRetirementRegistry` | 512 × 584 | 299 008 | 4 112 |
| `renderer::kernel_runtime` | `kernel_runtime::MountedTypedOperationResultExchange` | 64 × 4 160 | 266 240 | 24 |
| `ui::wgpu_engine` | `wgpu::engine::UiSurfaceRegistry` | 64 × 186 344 | 11 926 016 | 520 |
| `infinite::canvas` | `canvas::renderer::OpaqueSceneRetirementRegistry` | 1024 × 312 | 319 488 | 16 |
| `infinite::world` | `world::WorldInteractionObjectRegistry` | 1024 × 400 | 409 600 | 40 |

### 5.2 The Rust half — six guards, one law

`semio_framework_async::assert_fixed_slot_tables(guard, stack_bytes, threshold_bytes, declared, measured, build)`
owns every assertion, so each crate-side guard is four lines of JSON extraction plus its `size_of`
rows. It asserts, per table:

1. the measured `(capacity, size_of::<Slot>(), size_of::<Owner>())` equals the committed row;
2. `capacity * elementSizeBytes > conversionThresholdBytes` — a row that no longer earns the helper must
   be deleted, not carried;
3. **`ownerSizeBytes < capacity * elementSizeBytes`** — the structural proof the table is on the heap,
   because an inline `[T; N]` field can never be smaller than its own array;

then runs every owner's `Default`/`new` on `std::thread::Builder::new().stack_size(1 MiB)`.
`Builder::stack_size` **overrides** `RUST_MIN_STACK`, which is the point: `runCargoTestBudgeted`'s
blanket 128 MiB floor cannot hide a re-inflated frame on this route.

The six: `scenes::admitted_surface_map_tests::…`, `engine_canvas::engine_surface_attach_tests::…`,
`kernel_runtime::…`, `ui wgpu::engine::tests::…`, `infinite canvas::renderer::…`,
`infinite world::tests::…` — all named `…_heap_first_and_fit…_bounded_thread_stack`.

### 5.3 It bites — two independent demonstrations

`🐍️boxed-slots-baseline-toggle.py off ui`, rebuild, run (`🗑️generated/boxed-slots/11-guard-baseline-off.txt`):

```
assertion `left == right` failed: 🧱️ guard 'ui::wgpu_engine': measured slot tables differ from the committed budget
  left:  [FixedSlotTableBudget { owner: "wgpu::engine::UiSurfaceRegistry", capacity: 64, element_bytes: 186344, owner_bytes: 11926528 }]
 right:  [FixedSlotTableBudget { owner: "wgpu::engine::UiSurfaceRegistry", capacity: 64, element_bytes: 186344, owner_bytes: 520 }]
```

`🐍️boxed-slots-baseline-toggle.py off infinite`, rebuild, run — the bounded lane does not just report,
it dies (`🗑️generated/boxed-slots/18-infinite-baseline-off.txt`, first attempt):

```
thread 'infinite::canvas' (15897790) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

Both toggled back and re-verified green.

### 5.4 The twin — TypeScript, `@semio-tech/framework-async`

`🧰️framework/🔨️modules/⏳️async/🟦️.ts` gains `FixedSlotTableBudget`/`BoxedFixedSlotsBudget`,
`inlineSlotTableBytes`, and `fixedSlotTableFaults` — the Rust-free re-check of the same file's
`capacity × size_of` arithmetic, owner-uniqueness, threshold and "owner smaller than its table"
invariants — plus an in-source `import.meta.vitest` suite over the committed JSON. New
`vitest.config.ts`, a `test` command on the package router, a `test` nx target, and one launch entry
(`⚖️gate⏳️async🧱️boxed-fixed-slots-twin`, order 407.35, mirrored into `🧩️launch.seed.jsonc`).

```
$ bun nx run @semio-tech/framework-async:test
 Test Files  1 passed (1)
      Tests  4 passed (4)
```

The twin asserts what a language without per-frame stack reservation *can* assert: the record's
arithmetic. The dynamic half — "this descent fits a 1 MiB OS thread" — is inexpressible in JS and stays
Rust-side; the independent oracle for that half is the LLVM toolchain's own `sub sp` prologue, read by
`objdump` (§6), which agrees with every row of §0 and §5.1.

---

## 6. The tree-wide audit

`🐍️boxed-slot-sites.py <repo-root> [frames.tsv]` walks `🧰️framework`, `✏️s` and `🌎️hub`, inventories
every fixed-size slot-table construction, and joins each site's owning type to its measured frame by
parsing the Rust v0 mangled symbols' length-prefixed identifiers. **421 sites** before this packet
(82 boxed + 339 inline, `🗑️generated/boxed-slots/25-sites.tsv`), **406** after (72 + 334,
`30-sites-final.tsv`) — 15 construction lines disappeared because several registries collapsed three
field literals into one helper call.

Two shapes, and the audit found the brief's assumption about which one matters to be the wrong way
round:

* **`boxed`** — `Box::new([…; N])` / `Box::new(array::from_fn(…))`: **82 sites**. A heap destination
  with a stack source, so the array is still built in the caller's frame at `opt-level = 0`.
* **`inline`** — `array::from_fn(…)` into an array **field**: **339 sites**. Strictly worse: the frame
  cost is paid on construction *and* on every by-value move of the owner. Every one of the five largest
  remaining frames in the renderer test binary was this shape, not the boxed one.

### 6.1 Threshold and what was converted

Threshold: **64 KiB** of `capacity × size_of::<Slot>()`, stated in the fixture as
`conversionThresholdBytes`. Where a type's exact slot size was not nameable without opening a crate,
the measured constructor frame was used as the triage signal (it is an upper bound on the array plus
the frame's other temporaries).

Every site at or above the threshold in the six crates this packet opened was converted — 25 of them.
Measured `Default`/`new` construction frames, same binary, same instrument:

| owner | before | after |
|---|---|---|
| `kernel_runtime::RetainedSurfaceRegistry` | 4 447 824 | **50 560** |
| `kernel_runtime::CommandDocumentRetirementRegistry` | 901 200 | **4 160** |
| `shell::ShellDocumentRetirementRegistry` | 888 912 | **4 160** |
| `kernel_runtime::MountedTypedOperationResultExchange` | 802 896 | **8 496** |
| `actor::JobReplayLog` | 704 592 | **5 136** |
| `interpreter::SceneIntentQueue` | 538 704 | **1 552** |
| `world::WorldInteractionObjectRegistry` | 455 344 | **960** |
| `kernel_runtime::PendingSurfaceRejectionRegistry` | 435 792 | **4 672** |
| `kernel_runtime::MountedReplayRecoveryRegistry` | 325 200 | **3 504** |
| `board_host::BoardEventQueue` | 213 072 | **704** |
| `kernel_runtime::MountedProductReplayRecoveryRegistry` | 210 000 | **2 304** |
| `engine_canvas::EngineCanvasBuildContext` | 200 896 | **928** |
| `world::World3dBuildContext` | 129 712 | **640** |
| `kernel_runtime::KernelRequestQueue` | 121 088 | **1 392** |
| `FrameEnginePackets` | 102 480 | **928** |
| `GlbSchemaOutput` | 99 152 | **400** |
| `world::WorldOpaqueQuarantine<1024>` | 94 304 | **336** |
| `engine_canvas::StagedEngineScenes` | 94 304 | **864** |
| `GlbInstancePlanCursor` | 86 416 | **336** |
| `world::WorldInteractionMeshRegistry` | 84 448 | **768** |
| `world::WorldAssetIoAuthority` | 82 544 | **2 592** |
| `directed::IconPaintRegistry` | 77 904 | **352** |
| `kernel_runtime::MountedProductReplayRefusalRegistry` | 68 688 | **848** |

plus `canvas::renderer::OpaqueSceneRetirementRegistry` (962 640 → boxed, §2) and
`wgpu::engine::UiSurfaceRegistry` (11 929 152 → boxed, §3). A named
`MOUNTED_TYPED_OPERATION_RESULT_PAGES = 64` replaced the bare literal in the exchange's array type so
the fixture can pin a constant rather than a magic number.

### 6.2 What stays below the threshold, and why the byte pages are fine

The great majority of the 82 boxed sites are **byte pages**, not slot tables: `Box::new([0; N])` where
`N` is a 4 KiB or 16 KiB page constant (`JOB_PAYLOAD_PAGE_BYTES`, `BRIDGE_OUTBOX_PAGE_BYTES`,
`WORLD_INTERACTION_BYTE_CAPACITY`, `PREPARED_ATLAS_PAGE_BYTES`, `LAYOUT_ATLAS_PAGE_BYTES`,
`ACTION_ITEM_BYTE_CAPACITY`, `BOARD_POINTER_BYTE_CAPACITY`, `FILL_ENVELOPE_PAGE_BYTES`,
`INK_INTERACTION_DOCUMENT_BYTE_CAPACITY` and friends — all 16 384 B; `SNAPSHOT_CHUNK_BACKING_BYTES`,
`CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY`, `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` — 4 096 B). One 16 KiB
transient in a 2 MiB frame is 0.8 %; converting them would trade a measurable nothing for a heap
allocation on a hot path. Measured examples that confirm the arithmetic:
`AttachedSurfaceRegistry` 30 800, `World3dSnapshot*` 33 152, `ShellFind*` 41 056, `SnapshotReadLease`
45 904, `InkInteraction*` 25 520, `PreparedAtlas*` 320.

### 6.3 Still ≥ 64 KiB after this packet — 10 sites, 5 owners, none in a crate this packet opened

| measured | shape | owner | site |
|---|---|---|---|
| 362 720 | inline | `ui_contract::limits::UiPatchApplyArena` | `🖱️ui/🧬️contract/…/🛡️limits.rs:505` |
| 217 072 | inline | `job::WorkerJobAuthority` | `🧵️job/🦀️.rs:1924` |
| 188 496 | boxed | `ui_scene::math::Mesh3dAuthority` | `🖱️ui/🎬️scene/…/📐️math.rs:550` |
| 105 616 ×2 | inline | `async::deferred_wake::WorkerDeferredWakeRegistry` | `⏳️async/🔔️deferred-wake/🦀️.rs:85,96` |
| 105 248 ×4 | inline | `ui_contract::limits::UiPatchApplyProducer` | `🖱️ui/🧬️contract/…/🛡️limits.rs:643,960,971,990` |
| 78 528 | inline | `async::WorkerPool` | `⏳️async/🦀️.rs:2423` (over-attributed: the `from_fn` there is `[usize; 3]`; the frame is `WorkerPool::new`'s own) |

`semio-framework-ui-contract` declares no semio dependency at all and `semio-framework-ui-scene`'s
manifest states a "wasm-safe, minimal deps" boundary that another ticket
(`26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`) set deliberately; giving either
a dependency on `⏳️async` to save 188 KiB of a 2 MiB budget is a boundary decision that belongs to those
owners, not to this packet. `🔔️deferred-wake` is inside `⏳️async` itself (the helper is one `use` away)
but sits on the worker pool's own wake path, which another lane is actively working. All are one-line
conversions for whoever takes them.

**Not audited by frame measurement:** crates absent from the renderer test binary's dependency graph —
`🗄️stdio` artifact plugins, `🏗️fem`, `🔱️trinity`, `🌎️hub`. Their sites appear in
`🗑️generated/boxed-slots/30-sites-final.tsv` with the shape and source line but a `0` frame; by
constant they are byte pages (4 KiB/16 KiB) or `[None; N]` tables with `Option<u32>`-class elements.

---

## 7. Gates

| gate | result |
|---|---|
| `env -u RUST_MIN_STACK cargo test -p semio-framework-os-renderer-wgpu --lib -- engine_surface_attach_tests ui_command_wiring_tests heap_first --test-threads=1` | **27 passed, 0 failed** (`28-final-default-stack.txt`) |
| `bun nx run @semio-tech/framework-renderer-wgpu:test-native --no-fail-fast` | 501 run, 481 passed, 20 failed — **13 distinct names, identical to the baseline set** plus one load flake (§7.1) |
| `cargo test -p semio-framework-os-infinite --lib --no-fail-fast -- --skip heap_first` | 307 passed, 16 failed — failure set **byte-identical** to the toggled-off baseline (`diff` empty); the 2 guards pass |
| `bun nx run @semio-tech/ui-rs:test` | **237 passed, 0 failed** |
| `bun nx run @semio-tech/ui-rs:test-wgpu-engine` | 136 passed, 1 failed — pre-existing, §7.1 |
| `bun nx run @semio-tech/framework-actor-rs:test` | **21 passed, 0 failed** |
| `bun nx run @semio-tech/framework-async-rs:test` | 68 passed, 1 failed — pre-existing, §7.1 |
| `bun nx run @semio-tech/framework-async:test` (the TS twin) | **4 passed** |
| all six Rust guards together, `RUST_MIN_STACK` unset | **6 passed, 0 failed** (`31-all-guards.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going` | 0 errors, **24 warnings** (`13-wasm-check.txt`) |
| `cargo check -p semio-framework-async --lib --target wasm32-unknown-unknown` / `--target wasm32-wasip2` | 0 errors, both |

Warning counts are quoted because a zero error count proves nothing if expansion aborted; 24 warnings on
the browser target and 23 on native (`07-check-renderer2.txt`) mean those were real type-checks, and
neither count moved.

### 7.1 Attribution of every red result

* **Renderer, 13 distinct failures.** Measured against a baseline: `🐍️boxed-slots-baseline-toggle.py off`
  reverted every renderer conversion by exact-string replacement (never a file snapshot, so peers' edits
  in the same files survive) and the identical command was re-run. `21-nx-renderer-baseline.txt` vs
  `27-nx-renderer-round2.txt`: **regressions — none; newly fixed — none in this comparison**, because
  the 13 `ui_command_wiring` SIGABRTs were already gone before the baseline was taken. The prior
  packet's run reported 33 failures on the same crate; this one reports 13 distinct. The residue is
  other lanes' (`frame_job` deadlines, `shell::panel_anchor_model_tests` prefs store, `scenes::text_editor_tests`,
  `runtime_publication_tests`, `shell::command_registry_tests`, two source-text contract tests).
* **`kernel_seam::tests::a_fake_seam_receives_submitted_intents_and_outcomes_reach_the_host_on_wake`** —
  appeared once under the full parallel nextest run with
  `WorkerPool: mandatory submission failed closed: Contended`, a pool lock-contention flake with nothing
  to do with slot tables. Re-run in isolation **3/3 ok**.
* **Infinite, 16 failures** — `diff` of the failure name sets, conversions off vs on, is empty.
* **`semio-framework-ui` `wgpu-engine`, 1 failure** —
  `retained_document_hostile_fixtures::max_plus_one_…` panics `hostile builder: (ArenaFull, …)` on the
  **first** `UiDocumentBuilder::try_new` of a fresh process, inside `semio-framework-ui-contract`, a
  crate this packet never opened. Reproduces in isolation.
* **`semio-framework-async`, 1 failure** —
  `wasm_pool::cooperative_tests::worker_pool_use_cooperative_busy_keeps_executor_running_until_final_release`,
  a cooperative wasm-pool scheduling assertion untouched by anything here.
* **`semio-framework-os-mcp` does not compile** — 4 pre-existing errors, all in `🏠️workspace/`
  (`no field expires_at_ms on DirectorySessionAuthorityV1`, `ArtifactStore: ArtifactCompositionFields`
  unsatisfied). `🚚️transport` and `🧵️bridge`, the two files this packet edited, type-check clean.
  `24-check-mcp.txt`.

---

## 8. Files changed

| path | change |
|---|---|
| `🧰️framework/🔨️modules/⏳️async/🦀️.rs` | **new** region `🧱️BoxedSlots`: `boxed_slots`, `boxed_fixed_slots`, `BOXED_FIXED_SLOTS_FIXTURE`, `FixedSlotTableBudget`, `assert_fixed_slot_tables`, `on_bounded_stack` |
| `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` | **new** — the committed budget |
| `🧰️framework/🔨️modules/⏳️async/🟦️.ts` | **new** region `🧱️BoxedFixedSlots` + in-source vitest twin |
| `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/{vitest.config.ts,📜️script.ts,📋️project.json}` | **new** config; `test` command and nx target |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | **new** `⚖️gate⏳️async🧱️boxed-fixed-slots-twin` (407.35) |
| `🌉️mcp/🚚️transport/🦀️.rs`, `🌉️mcp/🧵️bridge/🦀️.rs` | private `boxed_slot_ring` copies removed; 5 call sites on the shared helper |
| `📺️renderer/…/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | private `boxed_fixed_slots` removed; 2 call sites re-pointed |
| `📺️renderer/…/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | 2 call sites re-pointed; `StagedEngineScenes`, `EngineCanvasBuildContext` converted |
| `📺️renderer/…/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` | `SceneIntentQueue::default` |
| `📺️renderer/…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | `RetainedSurfaceRegistry`, `CommandDocumentRetirementRegistry`, `MountedTypedOperationResultExchange` (+ named capacity), `MountedReplayRecoveryRegistry`, `MountedProductReplayRecoveryRegistry`, `MountedProductReplayRefusalRegistry`, `PendingSurfaceRejectionRegistry`, `KernelRequestQueueState`, `FrameEnginePackets`, `GlbSchemaOutput`, `GlbInstancePlanCursor` |
| `📺️renderer/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | `ShellDocumentRetirementRegistry` |
| `🖱️ui/…/🎯️targets/🧊️wgpu/⚙️engine.rs` | `UiSurfaceRegistry.slots` boxed |
| `🖱️ui/…/🎯️targets/🧊️wgpu/🧾️action.rs` | `FlatValue::Number` carries `dsl::Number` (§4) |
| `♾️infinite/🖼️canvas/🦀️.rs` | `OpaqueSceneRetirementRegistry::default` |
| `♾️infinite/🌍️world/🦀️.rs` | `World3dBuildContext` (×5), `WorldOpaqueQuarantine`, `WorldInteractionMeshRegistry`, `WorldInteractionObjectRegistry`, `WorldAssetIoAuthority` (×2), dynamic-draw registries |
| `♾️infinite/🎲️board/🔌️ports/➡️directed{,/➕️normal}/🦀️.rs` | `IconPaintRegistry`, `BoardEventQueue` |
| `♾️infinite/📦️packages/🦀️rust/Cargo.toml`, `🎭️actor/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-async` promoted from a wasm-only to an unconditional dependency, with the reason |
| `🎭️actor/🦀️.rs` | `JobReplayLog::new` |
| 6 test files (`🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map`, `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces`, `🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document`, `🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit`, `♾️infinite/🌍️world/🧪️tests/🔬️unit`, `♾️infinite/🖼️canvas/🧪️tests/🔬️renderer-opaque-scene-retirement`) | **new** bounded-stack guard each |
| `🎫️tickets/…/PROCEDURAL-3D-END-TO-END/🐍️boxed-slot-sites.py`, `🐍️boxed-slots-baseline-toggle.py` | **new** instruments |
| `🎫️tickets/…/PROCEDURAL-3D-END-TO-END/🐍️wgpu-stack-baseline-toggle.py` | repaired for the moved helper |

No `RUST_MIN_STACK`, `.cargo/config.toml`, thread-stack setting or `CARGO_TARGET_DIR` was touched
anywhere; the browser's 16 MiB `-zstack-size` and `runCargoTestBudgeted`'s 128 MiB floor are unchanged,
and every guard overrides them by construction.
