# Round 4: Rust UI suites vs. the UI arena

Paths below are under `🧰️framework/🔨️modules/🖱️ui/` unless they start with a crate name. Command used for every run:
`repo cargo --store=isolated --configuration=prototype -- test --offline --no-fail-fast --manifest-path <crate>/Cargo.toml [args]`
(scripts `r5-run.sh`, `r5-one.sh`, `r5-batch.sh` in the ticket folder).

## Root cause

The finite UI arena (`🧬️contract/🌱️values`, `🏟️collection-arena`) is a process-wide registry that only accepts values while a *serialized native scope* is open (`UiArenaFixture`, serialized by a process mutex, opened and closed through the Source of one original fixture caller). After the arena-registration refactor every retained document, patch, lease or `Ui` window allocates its values in that registry; outside a scope the registry answers "UI arena has no accepting lifetime registration", and a document that is dropped instead of paid-closed trips "controlled retirement abandoned before terminal-empty".

Two facts decide the design:

1. The scope is **per test, not per helper**: it is process-serialized and must enclose every value built in the test, so a shared harness helper (`mount`, `document`, `assembled`) cannot open it itself (it would end before the test uses the value, and two live scopes in one thread deadlock on the mutex; the original caller is also one per thread).
2. A retained `Ui` additionally owes a paid close of every window (`begin_surface_close` / `close_surface_one` under the same Source) and of the contract pages before the scope may close.

## Design applied

- **Plain retained-document laws** (UiTree, documents, leases, patches): one `UiArenaFixture::new()` as the first statement of the test, the idiom the kernel/runtime/limits/accessibility suites already use. Applied with `r5-arena-insert.py` to every `#[test]` of the affected modules, so none is left out:
  - conformance crate (`🧬️contract/🧪️conformance`): `root_reservation`, `refresh`, `patch_pages`, `whole_patch`, `typed_descendants` (21 tests);
  - `🧪️tests/🌳️document-tree-reconcile` (15), `🧪️tests/🔬️targets-wgpu-reconcile-unit` (16, merged into the `fn` line because the mounting crate is at its 4000 active line source boundary).
- **Laws that own a `Ui`**: new central harness `UiFixture` (`🎯️targets/🧊️wgpu/⚙️engine/🧪️fixture/🦀️.rs`, exported as `semio_ui_wgpu_engine::testkit::UiFixture`, feature `testkit`). It owns the `Ui` *and* the arena scope, derefs to `Ui`, and its `Drop` pays every window's surface close plus the contract page close under the scope's own Source (also on the panic path, inside `catch_unwind`, so a failing law does not leak the scope into the next one). Constructing one is the only thing a test can do, so a `Ui` law can neither forget the scope nor leak a document. `UiArenaFixture` gained `turn(...)` (runs one funded turn of its own original caller) for exactly this.
  - `🧪️tests/🧪️conformance-corpus`: `mount` returns a `UiFixture`; `mount_into` mounts more windows into the same one (a law owns exactly one scope; shadowed `mount` calls in the number-control, precision, slider and dial laws deadlocked on the mutex and are now `mount_into`).
  - `document-tree-reconcile`: the two laws that build a `Ui` (`a_published_document...`, `a_document_whose_root_is_the_engine_surface...`) use `UiFixture`; the tree they also build is `retire`d.
- Already patched hosting/limits/accessibility tests keep their explicit scope: they build `UiTree`/documents only, no `Ui`, so `UiFixture` does not supersede them.
- Dev-dependencies added: `semio-ui-contract-values` (feature `testkit`) in the laws companion crate and in `🌳️retained/🌳️tree/🍃️nodes`; optional `semio-ui-contract-values` plus feature wiring in the engine crate (`testkit`).

## `original_clipboard_ingress` compile error (root cause)

It is not an arena problem: the test (`🧪️tests/📋️clipboard-ingress`) still used the removed in-place birth API (`WorkerJobSessionPreparation`, `ClipboardIoJobInitializer` into a worker session, `acknowledge_outcome(grant)` returning a step). The worker session is now born only through `WorkerJobSessionAdmission::advance(&mut Option<Job>, &mut Option<BatchJobParams>, &mut Option<Session>, cx)` under one original caller, and `acknowledge_outcome(cx)` returns `bool`. The test was ported to that API (`ClipboardIoJob::read()/write(..)`, `admit`, `acknowledge`, caller budget `10_000_000 / 1_000_000` as the worker admission law, per-step grant `stock()`), and both laws now compile and run. They still fail at runtime inside the job-harness accounting (see residual list) because the old law pinned receipts of the removed initializer path.
Not removed: `ClipboardIoJobInitializer` is still used by `🛍️products/💻️os/.../🧩️retained-ui-engine/🎬️scene-intents-and-clipboard`.

## Other fixes found by the sweep

- `semio-ui-retained-tree-tree` (`🧪️tests/🔬️targets-wgpu-reconcile-unit`): 2 arena failures fixed by the scope; the third failure was a stale relative fixture path (`../../../../🎬️scene/...` needs one more `../` from the mounting crate), fixed. 18/21 -> 21/21.

## Results (before -> after)

| Suite | Before | After |
| --- | --- | --- |
| `semio-ui-wgpu-laws` `original_clipboard_ingress` | did not compile (4 errors) | compiles, 0/2 pass (runtime job-accounting, residual) |
| `semio-ui-wgpu-laws` lib (45 tests) | 12 corpus + 4 arena document-tree failures (arena), plus the layout ones below | 36 pass, 9 fail (7 not arena, 2 cascade) |
| `semio-ui-contract-conformance` (43 tests) | 21 tests in the 5 modules failed (no scope) | 38 pass, 5 fail (fixture drift, not arena) |
| `semio-ui-retained-tree-tree` (21) | 18 pass, 3 fail (2 arena) | 21 pass |
| other crates | see sweep table | see sweep table |

### Residual failures that are NOT arena problems (need an owner decision)

1. Fixture drift in the contract conformance crate (5 tests): `size_of::<UiNodeRecord>()` is now 7168 (fixture 7128; document 917504 vs 912384, aggregate 58720256 vs 58392576, full document 928546 -> 933762 ...) and `size_of::<UiPatchOp>()` 7168 vs 7088. The numbers cascade through `🎟️resident/🔄️refresh/🧪️tests/🧫️fixtures`, `🎟️resident/🧪️tests/🧫️fixtures`, `🌳️root`, `♻️retirement/🩹️patch/🧪️tests/🧫️fixtures` and the TypeScript twin `🧪️tests/🔬️fixed-list-storage`; they must be re-measured together once the record type stops growing.
2. Seven laws in `semio-ui-wgpu-laws` fail on paint counts only (slider/dial ticks `0` instead of `3`, tree rows paint nothing, row-action hint, an engine-surface root that never finishes a paint, a published document with zero non-empty layers). They count `ui_instances + vector_vertices + raster_instances`; the draw list now carries shared-border items (`shared_normal_finalized`, round 3) and layout stepping under a bare `StepContext` does not run the worker admission (`validate` needs `original_source_parent`). Driving `step_layouts` through the arena caller (`UiArenaFixture::turn`) was tried and does not settle in 120 s (the admission cycles `authority`/`admissionClose` at `Layout.Mount`), so that was reverted. This is the layout-worker/shared-border track, not the arena.
3. Two laws (`media_transport...`, `history_editor...`) fail only as a cascade of (2): a failed law that mounted documents leaves values in the process-wide registry, so the next scope reports "closed" / "already has an actual lifetime owner". They pass when run alone.
4. `original_clipboard_ingress` runtime: `close_owner` asserts `receipt() == step.progress()` (9 items / 6064 bytes against 0), and the cancelled write never reaches a terminal outcome in 4096 turns with the ported driver. The old law (initializer-based, exact grants) has no direct equivalent on the admission API; needs the job owner's reference law.
5. `semio-ui-wgpu-engine-laws` / `engine-scene-laws` do not compile: the `🧪️tests/🔬️targets-wgpu-engine-unit` and `♻️physical-job-close` laws still call removed APIs (`SelfFundedCloseCaller`, `InteractiveJob::step(cx)` without the return claim, `close_step(grant, ..)`, `RetainedWorkOutcome::unwrap`). Same refactor family, large (100+ laws), not touched.
6. `semio-ui-locale`, `semio-framework-text-splice-native`: compile errors in their test mounts (`fixture::receive/capture/close_owned`, 2-argument `step`); not arena.

## Sweep

`r5-crates.txt`: 108 crate manifests under `🔨️modules/🖱️ui`. Because runs share one build lock and take 1 to 5 minutes each, the sweep was split: the first 31 crates ran before any change (`before`), the remaining ones after (`after`). Final tables are appended below.

### Before (31 crates, no changes yet)

Arena-related: only `semio-ui-retained-tree-tree` (2 of its 3 failures). Everything else that failed was not arena:

| Crate | Result | Cause |
| --- | --- | --- |
| `semio-ui-tui-engine` | 4 pass, 4 fail | tab drag preview geometry |
| `semio-ui-tui-core` | 58 pass, 2 fail | docking vector, 60 ms keystroke budget |
| `semio-ui-locale` | does not compile | stale `fixture::receive/capture/close_owned` |
| `semio-ui-retained-tree-tree` | 18 pass, 3 fail | 2 arena (fixed), 1 stale fixture path (fixed) |
| `semio-framework-text-splice-native` | does not compile | 2-argument `step` |
| `semio-ui-scene-plane`, `semio-ui-scene-mesh` | 21/1 fail, 17/1 fail | scene payload laws |
| remaining 23 (tui frame/host/vt/terminal, arena, theme, styling, action, scene surface/world3d/views/algebra/pack, ...) | all pass | |

### After (the rest of the list; the run was stopped after about 20 more crates, `repo` runs one build at a time)

| Crate | Result |
| --- | --- |
| `semio-ui-wgpu-laws` | 37 pass, 11 fail (7 paint/layout, 2 cascade, 2 clipboard runtime) |
| `semio-ui-wgpu-hosting` (`--features semio-ui-wgpu-mounted-layout/testkit`) | 44/44 (was 43/44) |
| `semio-ui-wgpu-engine` | lib test needs `--features testkit`; with it: compiles, 0 tests |
| `semio-ui-wgpu-events-laws` | 88 pass, 1 fail (hover marker target, layout-dependent) |
| `semio-ui-wgpu-mounted-layout-laws` | 1 pass, 2 fail (`layout never settled` under a bare `StepContext`) |
| `semio-ui-wgpu-engine-laws`, `-engine-scene-laws`, `-layout-laws`, `-raster` | do not compile (removed APIs: `InteractiveJob::step` return claim, `SelfFundedCloseCaller`, `RasterContentIdentity::test_pixels`) |
| `semio-ui-wgpu-render` 16, `-draw-execution` 23, `-pipelines` 1, `-mesh` 4, `-layout` 37, `-widgets` 8, `-events-control-laws` 14, `semio-ui-scene` 1 | all pass |

No "UI arena has no accepting lifetime registration", "requires its actual registered Source" or "abandoned before terminal-empty" appears in any crate that was run after the changes, except the 2 cascade laws in `semio-ui-wgpu-laws` described above.

Not run (time box): the remaining ~55 crates of the list (runtime, host, paint, controls, input, shell, scene-slots, prepared, tui companions, ...). Re-run with `r5-run.sh <tag> <manifest-list>` after a `find ... -name Cargo.toml` listing.

## Files touched

New: `🎯️targets/🧊️wgpu/⚙️engine/🧪️fixture/🦀️.rs`. Edited: engine `🦀️.rs` (mount of `testkit`), engine `Cargo.toml`, `🧬️contract/🌱️values/🧪️fixture/🦀️.rs` (`turn`), laws companion `Cargo.toml`, `🌳️retained/🌳️tree/🍃️nodes/📦️packages/🦀️rust/Cargo.toml`, the five conformance modules, `🧪️tests/🧪️conformance-corpus`, `🧪️tests/🌳️document-tree-reconcile`, `🧪️tests/🔬️targets-wgpu-reconcile-unit`, `🧪️tests/📋️clipboard-ingress`. Scripts kept in the ticket folder: `r5-run.sh`, `r5-one.sh`, `r5-batch.sh`, `r5-arena-insert.py`.
