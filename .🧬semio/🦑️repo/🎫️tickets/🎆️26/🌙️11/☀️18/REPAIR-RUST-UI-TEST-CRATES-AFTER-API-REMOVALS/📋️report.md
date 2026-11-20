# Repair Rust UI Test Crates After API Removals

Follows `26/11/17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI` (`round4-rust-arena.md` lists the starting state).
Every command: `one.sh <tag> <manifest> [args]` / `run.sh <tag> <crates.tsv>` in this folder
(`repo cargo --store=isolated --configuration=prototype -- test --offline --no-fail-fast --manifest-path ...`).

## Root causes found

### 1. Layout of a `Ui` could never settle in a law (production engine bug + harness gap)

- Harness gap: every law drove `Ui::step_layouts` with a bare `StepContext::new(...)`. Since the worker
  refactor the layout worker session is born through `WorkerJobSessionAdmission`, which validates the
  original Source parent, so a bare context is refused at `Layout.Admission` and the window never lays out.
  Laws "passed" their settle loop on an early `Idle` and then painted an unlaid tree (slider ticks 0, empty
  tree rows, zero non-empty layers).
  Fix: `UiFixture` (`semio_ui_wgpu_engine::testkit`) now owns the only way a law drives layout:
  `ui.step_layouts(&pool, atlas)` runs one opportunity inside one funded turn of the fixture's own original
  caller (the Source the arena was admitted under), `ui.settle_layout(window, &pool, atlas)` requests layout
  while dirty and loops until `Idle` and clean, failing the law on `Refused`.
- Engine bug (`⚙️engine/📐️layout-stepping`): the session birth takes `layout_job` and its `BatchJobParams`
  at its authority stage, but `step_layouts` only advanced the birth while `layout_job` was `Some`. After the
  hand-over it built a second `MountedLayoutJob` mid-birth, and after the session closed it kept calling the
  finished (stage 9) birth, which answers `false` forever, so every relayout stalled at `Layout.Mount`.
  Fix: a handed-over birth is driven to its session regardless of `layout_job` (`advance_layout_birth`), a new
  admission starts on a fresh birth (`UiWindow::renew_layout_birth`), and an in-flight birth is closed
  wherever the layout admission is closed, including surface close (`UiWindow::close_layout_birth_step`;
  `LayoutWorkerAdmission::close_step` cannot reach the birth, which closes on the Source context), which
  previously could leave a surface close pending forever.

### 2. Arena scope leaking into the next law

The leak was a consequence of (1): a law that failed mid-layout could not pay its surface close, so the arena
close failed and the process-wide registry stayed owned ("UI arena already has an actual lifetime owner" for
every later law). With the birth closable, the panic path of `UiFixture` pays the close. Pinned by the new law
`a_law_panicking_mid_layout_returns_its_arena_scope_to_the_next_law` (conformance corpus): a law panics while
its relayout is in flight, the next law mounts and paints in a fresh scope.

### 3. Drop targets without content were not hittable (production bug)

`is_plain_stack_container` (`⚡️events/🧭️event-model-and-focus`) exempted `DRAG_SOURCE` stacks but not
`DROP_TARGET` stacks, so an empty row flagged as a drop target was skipped by the hit test and a drag over it
had no hover marker target (`semio-ui-wgpu-events-laws`).

### 4. Test scope expired on real time

`UiArenaFixture` admitted its original caller with a deadline of `10_000_000` µs on the real monotonic
Source clock: 10 s per test scope. On a loaded host, or in a long law (the 1 025-node layout law), the clock
expired mid-law, the Source declined new construction (`WorkLimit "original Source clock declined new
construction"`), layout stalled at `Layout.CollectNodes`, the surface close failed and the scope leaked into
every later law. Now a named finite ceiling `SCOPE_DEADLINE_MICROS = 600 s`. This also explains the
intermittent `the_reserved_media_transport...` arena close failure.

### 5. Stale laws against current contracts

- `same_size_moved_viewport_discards_the_old_origin_candidate_before_atomic_publication`: written before the
  bounded, rollback-able retained append (2026-11-07) that publishes into the caller's draw list across
  several `Pending` steps (pinned by its neighbour law). The intent (no discarded-origin geometry ever reaches
  the target) is now asserted on every `Pending` step instead of "target untouched until `Ready`".
- `large_layout_and_shaping_job_admits_one_work_unit_per_slice`: the one-node/one-glyph bound now applies to
  the six layout-job stages; the worker-session stages (`Layout.ReceiveOriginalReceipt`, ...) report retained
  receipt items in the same field.

### 6. Repo gates and build-shape defects

- `semio-ui-contract-values` exceeded its 262 144-byte source boundary (≈267.6 KB, already over at HEAD), so
  its tests could not start. Its two white-box arena law files now mount from the companion crate
  `semio-ui-contract-values-laws` (`🔬️laws/🏟️arena/🦀️.rs`), files unchanged in place; the crate is ≈254.7 KB.
  `UiArenaBacking::pointer`, `UiArenaOpening::phase_demand`, `release_vec_demand` became public quotes and
  the test serial is exported as `testkit::ui_arena_serial`.
- `semio-ui-render`'s `backend-testing` hooks on `GraphicsBackend` had no default bodies, so any build that
  enabled `semio-ui-render/backend-testing` without each backend's own feature (feature unification) could
  not compile the Metal/Vulkan/D3D12/WebGPU backends. They now default to a loud panic naming the backend.
- `semio-ui-wgpu-engine` and `semio-ui-wgpu-hosting` lib tests used testkit-gated items of their dependencies
  without enabling them (they only passed with `--features`); dev-dependencies now enable `testkit`.
- `semio-ui-contract-typegen` compares against the gitignored `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`;
  `repo run @semio-tech/ui-contract-rs:generate` failed with a repo-internal error ("Closed pinned dependency
  recipe required"), so the file was produced the way that target does (export test with `SEMIO_TYPEGEN_OUT`).
- `semio-ui-contract-values` law `utf8_text_chunk_transport_matches_shared_vectors` now opens its arena scope.

### 7. Someone else's `[DEBUG]` probe removed

A probe in `🌱️value/♻️retirement/🎮️controlled/🦀️.rs` (`ControlledCore::step_original`, written 2026-11-18 22:55 by
another fleet agent, already committed to HEAD) built a `Vec` and called `eprintln!` inside whichever
retirement turn was process-wide call 100000. That made the turn's heap disagree with its receipt and failed
`semio-ui-locale` (label close) and `semio-ui-contract` (graph law). With the approval of the coordinating
session, only that probe line was removed. Its `debug_type_probe` helper (`♻️retirement/🦀️.rs`,
`🧬️contract/🦀️.rs`) stays, because other active `[DEBUG]` probes in `🛍️products/💻️os/🔨️modules/🏪️store` still
call it; those and the birth-mismatch print at `♻️retirement/🦀️.rs:80` belong to their owner.

### 8. Missing arena scopes and test-only dependencies (sweep)

- 22 laws in `semio-ui-render-interaction`, `-runtime-reconcile`, `-contract`, `-contract-builder`,
  `-contract-operations`, `-contract-validation`, `-runtime`, `-runtime-reconcile-primitives` built UI values
  without a scope; each now opens one `UiArenaFixture` as its first statement (`arena-insert.py`). The four
  document laws in `🧬️contract/🧪️conformance/🗂️records` also got a scope and bounded close loops: an unbounded
  `while !close_step() {}` without a scope hung the conformance crate whenever another law held the arena.
- Test-only dev-dependencies were missing for `semio-ui-runtime-reconcile-cursor`,
  `-runtime-reconcile-primitives`, `-render-interaction` and `-runtime-reconcile` (contract-values testkit).
  `semio-ui-declaration`'s wire-format law named the scene crate by package name instead of its `ui_scene`
  alias. `dep-scan.py` checks all 108 crates for this.

## Delegated parts (reports in this folder)

| Report | Crates | Root causes (short) |
|---|---|---|
| `a1-locale-text-splice.md` | locale, text-splice-native, contract, contract-presentation, values-laws (bounded receiving), value admission fixture | text-splice composer never reported its allocation; the retirement pump refused owner-less retirement after the deadline; `NativeReceivingOriginal` counted lifetime instead of per-turn progress; nested metadata item count; the derive refused empty flattened payloads; UI receivers lacked terminal destructors; the decoder session ceiling now follows the funded stock instead of a fixed 16 MiB; the locale law's Source now answers schema keys |
| `a2-engine-laws.md` | wgpu-engine-laws, engine-scene-laws | ports to the return-claim/close APIs, golden laws to shared borders, `UiFixture` everywhere |
| `a3-layout-raster.md` | wgpu-layout-laws, wgpu-raster | ports; `layout_tree_flowing_now`; raster `testkit` feature forwarding |
| `a4-conformance-fixtures.md` | contract-conformance (+ TS twin) | size fixtures re-measured (`reservation_epoch`; arena moved out of static memory) |
| `a5-clipboard-ingress.md` | wgpu-laws clipboard, engine-scene-laws clipboard, prepared-laws, host-tests | origin refresh in close receipts; cancelled outcomes stay unclaimable per the core contract; stale `debug_probe`s; `PreparedRenderJob::close_step` returned `Pending{0}` instead of `Blocked`/`Complete`; stale `DrawList` size formula; process-global test ordering |
| `a6-tui-shared-borders.md` | tui-core, tui-engine (+ TS twin) | docking vectors moved to the shared-splitter model; the drag preview now paints the projected committed layout |
| `a7-arena-laws.md` | contract-values-laws | `std::sync::Mutex` on macOS lazily allocates 64 unreceipted bytes, so the arena registry lock is now allocation-free; return table admitted before measuring; fixed-list owner words read from the shared fixture |
| `a8-scene-crates.md` | scene-mesh, -modelling, -picking, -plane | screen select swallowed `Busy` mesh reads; mesh close stepped on a widened effect; fixture float `0.0` |
| `a9-mixed-ui-crates.md` | backend-metal, -webgpu, render-shader-contract, runtime-reconcile, native_resource_custody | Metal ABI fixture/schema; WORLD3D shader contract now byte-identical to wgpu (webgpu packed shadow opacity into the SVG flat-lighting switch); `SurfaceReconciler` boxes its native resource loans; resource fixture grant |

## Result

All 108 crates under `🔨️modules/🖱️ui` compile their tests, and 107 pass in the final sweep; the per-crate table
is `📊️before-after.md`. The exception is `semio-framework-text-splice-native`: it passed 4/4 after its repair, but
in the final sweep it fails because another session is actively adding `[DEBUG]` probes that allocate inside
measured turns (`🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🎛️driver/🦀️.rs` and `🌱️value/🛬️decode/🦀️.rs`,
written 2026-11-19 04:47). They were left alone because they are in active use.

## Open, outside this ticket

- `semio-framework-job-worker` and other job crates: 74 failing tests exist both with and without this ticket's
  job edits (`a1` A/B run). They come from concurrent work elsewhere (e.g. the new `clock_begin` check).
- Non-UI crates using the shared `receiver-driver.rs`/`driven.rs` (kernel events, value native-owned receivers,
  graph manifest, os store child, board ports) were not re-run after the decoder ceiling change. It only widens a
  fixed 16 MiB ceiling to the funded stock, and none of them asserts that refusal.
- Remaining `[DEBUG]` code from others: `debug_type_probe` users in `🛍️products/💻️os/🔨️modules/🏪️store`, the
  birth-mismatch print in `🌱️value/♻️retirement/🦀️.rs:80`, the decode-driver probes above.
- Two UI laws stay `#[ignore]` by design (run through `runExactCargoLaws`); this ticket did not change them.
- `repo run @semio-tech/ui-contract-rs:generate` fails with "Closed pinned dependency recipe required".
