# ⏱️ Wave R2 — store replacement step budget (measured, native, opt-level 0)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Date 2026-09-09.

## Verdict, up front

**The store replacement unit is not the boot offender, and stage 14 is a mis-attribution.**

Measured natively at opt-level 0, on the real paths:

| unit | worst single unit | budget | ceiling |
| --- | --- | --- | --- |
| puzzle3d stage 14 `drive_store_replacement_jobs`, Concrete Forest | **2–4 µs** | 2 000 µs | 8 000 µs |
| puzzle3d stage 14 `drive_store_replacement_jobs`, Nakagin | **2 µs** | 2 000 µs | 8 000 µs |
| puzzle3d worst unit of ALL 21 stages, Concrete Forest | **76 µs** (stage 10) | 2 000 µs | 8 000 µs |
| puzzle3d worst unit of ALL 21 stages, Nakagin | **152 µs** (stage 0) | 2 000 µs | 8 000 µs |
| framework store replacement `Initializing` (real initializer worker step) | **372 µs** over 42 units | 2 000 µs | 8 000 µs |
| framework store replacement `CandidateReady` (disposer + validate + atomic swap) | **1 µs**, 1 unit | 2 000 µs | 8 000 µs |
| framework store replacement `RetiringCommittedStore` (bounded displaced close) | **36 µs** over 412 units | 2 000 µs | 8 000 µs |

Nothing in the store replacement path is anywhere near the ceiling, so **there is nothing to slice**
(task 2). Slicing an in-budget unit on a hot framework path would be speculative churn against the
measurement, so it was not done. The measurement is now a permanent law instead (task 3).

Two structural findings explain the browser trace, below.

## 1. `drive_store_replacement_jobs` never runs at all for puzzle3d

`Puzzle3dPlayApp` is an `ArtifactEditor`, run as `VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>`
(`✏️s/🔌️plugins/🧩️puzzle/🦀️.rs:14`). It does **not** override
`ArtifactEditor::build_document_store_initialization_job`, so the default at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24298` returns `Err(envelope)`:

- `ArtifactStoreReplacementAdmissionTarget::try_adopt_completed`
  (`🔌️plugin/🦀️.rs:16263`) calls `A::build_document_store_initialization_job(...)?` and therefore
  **never inserts a replacement job** for puzzle3d;
- `drive_store_replacement_jobs` (`🔌️plugin/🦀️.rs:17475`) hits
  `self.store_replacement_jobs.next_id_from(*cursor) → None` and returns `Complete` immediately —
  the measured 2–4 µs;
- consequently the puzzle3d Storybook wasm bridge's envelope-load loop
  (`✏️editor/🌉️wasm/🦀️.rs:95` `pollEnvelopeLoad`) can never reach `Ready`. Separate defect, not
  fixed here (out of R2's scope), but it means **no whole-document store replacement runs anywhere
  in the puzzle3d boot** — the OS React boot does not call `begin_artifact_envelope_ingress` at all
  (the only callers in the tree are the four per-plugin wasm bridges: writer, jack, raster,
  puzzle3d).

## 2. `stage=14` in the browser trace is a stale cursor, not the stage that ran

`VcsArtifactApp::maintenance_step` (`🔌️plugin/🦀️.rs:21980`) stores `LAST_MAINTENANCE_STAGE` at
entry, **then** takes a large idle early return (`🔌️plugin/🦀️.rs:21984-22011`, the
`tool_operations.is_empty() && … && store_replacement_jobs.is_empty() && …` chain) *before*
`let stage = self.maintenance_stage;` and *before* the `% MAINTENANCE_STAGES` advance.

So whenever the app is idle — which is exactly the steady state after boot — the round-robin cursor
freezes and `LAST_MAINTENANCE_STAGE` keeps reporting whatever stage it stopped at, forever. The
coordinator's `[DEBUG] maintenance stage=14 elapsed_us=8401` (`🔌️plugin/🦀️.rs:27008`) reads that
frozen value, so it names stage 14 for work that stage 14 did not do.

**Action for the coordinator (the trace is yours, I did not touch it):** either move the
`LAST_MAINTENANCE_STAGE.store` to just after `let stage = self.maintenance_stage;`, or store a
distinct sentinel on the idle early-return branch. Until then every over-ceiling trace from an idle
instance will keep accusing stage 14.

## 3. The ceiling verdict measures WALL time, and fires on descheduling

`RuntimeLiveCleanupJob::step` brackets `maintenance_step` with `default_now_us()` and feeds the
delta to `semio_framework_trace::interactive_step_contract_violated`. That is wall time on a
contended thread, not the unit's own work.

Reproduced natively, by accident: with the new law running inside the **full 584-test puzzle3d
suite** (dozens of threads on this loaded machine), one Concrete Forest unit measured
**14 571 µs** — for work that measures 1–19 µs when the same test runs alone. Same code, same
document, 700× the reading.

The browser boots ~20 WASM plugins concurrently in a hidden 0×0 pane
(cf. `📓️…hidden-browser-pane-throttles-plugin-boot`), so 8 401 / 9 500 / 10 201 µs readings for a
unit that costs single-digit microseconds natively at opt-level 0 are consistent with descheduling,
not with real work. **A release-wasm unit cannot plausibly be 400× slower than the same unit in an
unoptimised native debug build.**

This is why the new laws fold the per-stage worst over three independent rounds
(`keep_best_round`): real over-budget work costs the same every round, a hiccup does not.

## 4. What was measured, and where

### puzzle3d: every maintenance unit, both flagship documents

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:66-125`
— `every_maintenance_unit_stays_inside_the_interactive_step_budget`. Drives the REAL app (registry-
backed `VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>` bound to instance 1), the REAL typed
`setActiveExample` command for `concrete-forest` and `nakagin-capsule-tower`, real host turns
(`measured_host_turn`: `maintenance_step(1, 4096)` → `advance_typed_operation_publication` →
present/ACK → drain), then eight full round-robin sweeps, timing every unit with
`semio_framework_job::default_now_us` and attributing it to the stage read **before** the call.

Measured (isolated run, `keep_best_round` over 3 rounds):

```
concrete-forest      worst_stage=10 worst_us=76
  0=58 1=3 2=2 3=2 4=4 5=1 6=2 7=4 8=18 9=21 10=76 11=2 12=5 13=55 14=4 15=31 16=5 17=3 18=7 19=6 20=4   (µs, 69 units each)
nakagin-capsule-tower worst_stage=0 worst_us=152
  0=152 1=3 2=1 3=2 4=2 5=1 6=2 7=2 8=3 9=3 10=16 11=2 12=3 13=4 14=2 15=2 16=4 17=2 18=10 19=21 20=2   (µs, ~108 units each)
```

Stage 14 (`drive_store_replacement_jobs`) is 4 µs / 2 µs. The most expensive stages are 10
(`drive_envelope_ingress`) and 0 (typed-operation worker/retirement).

### framework: the store replacement unit's three phases

`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-laws/🦀️.rs:173-223`
— `every_store_replacement_phase_unit_fits_the_interactive_step_budget`. process3d is the domain
that actually implements `build_document_store_initialization_job`, so this is the real path:
`Process3dStoreInitializationAuthority::step` (`Initializing`), then
`ArtifactDocumentStoreDisposer::new()` + `process3d_validate_atomic_lease` +
`publish_document_store_candidate_if_authoritative` (`CandidateReady`), then the bounded one-item
displaced-store close (`RetiringCommittedStore`).

```
worst_phase=Initializing worst_us=372
  Initializing=372us/42u  CandidateReady=1us/1u  RetiringCommittedStore=36us/412u
```

So, against the suspect list in the brief:

- `publish_document_store_candidate_if_authoritative` — **1 µs**. It is a `validate()` plus one
  `std::mem::replace`; `ArtifactStore` is a struct of heap handles, not an inline arena, so the swap
  moves a few hundred bytes. Not a cost centre.
- `build_document_store_disposer` — inside that same 1 µs (a zero-sized `PhantomData` disposer).
- `ArtifactStoreInitializationOwnerCatalog::try_new`'s six `try_reserve_exact(64)` — inside
  `Initializing`, whose 42 units peak at 372 µs total. Not isolated further because the whole phase
  is already 5× under budget.
- W-B's batch machinery, W-P2's spatial index, the envelope decode page loop — none of them appear:
  their stages (0, 10, 11) measure 152/76/2 µs on the real documents.

## 5. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21417` — new
  `pub const MAINTENANCE_STAGES: u8 = 21`, the length of the cooperative round robin.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22019` — `% 21` → `% MAINTENANCE_STAGES`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:17192` — new
  `VcsArtifactApp::next_maintenance_stage()`, so a budget law can name the stage a unit ran
  (reading `LAST_MAINTENANCE_STAGE` afterwards names a stale one — finding 2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31393` — re-export `MAINTENANCE_STAGES`.
- `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs:7-49` — `RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP`
  and `MaintenanceStageBudget` (fixed `[u64; 21]` / `[u32; 21]`, `worst`, `report`,
  `keep_best_round`).
- `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs:59-92` — `Puzzle3dApp` is now a named struct
  carrying its `maintenance` budget, plus `measure_maintenance_step`.
- `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` — `settle` drives
  `measure_maintenance_step`, so every existing fixture accumulates the budget for free.
- `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:51-125` — the new law plus
  `measured_host_turn`.
- `✏️s/…/🧊️process3d/…/💾️binary/🧪️tests/🔬️retained-laws/🦀️.rs:33-97` —
  `REPLACEMENT_PHASES`, `ReplacementPhaseBudget`, `measure_phase`, `owned_store_measured`.
- `✏️s/…/🧊️process3d/…/💾️binary/🧪️tests/🔬️retained-laws/🦀️.rs:173-223` — the new law plus
  `measure_one_store_replacement`.

Nothing in `⚛️reactor/🔄️turn/🦀️.rs`, `RuntimeLiveCleanupJob`, `🕹️interaction/**` or the
coordinator's `[DEBUG]` traces was touched. No wasm was rebuilt and no server was started.

## 6. Other defects found while measuring (not R2's, reported for routing)

1. **`fillBuildTick` really does breach the ceiling.** The existing law
   `fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin`
   (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:2868`) fails with
   `fillBuildTick turn 763 took 25.593042ms, at or over the framework's interactive step ceiling 8ms`.
   That is a genuine >8 ms interactive unit — the only one this wave found anywhere. If the browser
   fault is real work rather than descheduling, **this** is the shape to look at, not stage 14.
2. **`setActiveExample nakagin-capsule-tower` faults** its retained operation with
   `job-session.terminal-fault`, so the flagship example cannot be loaded through its real typed
   command. Same family as the 53 other in-flight failures below.
3. **puzzle3d's envelope-load bridge is dead** — see finding 1.

## 7. Commands run, and what is NOT verified

All foreground, `RUSTC_WRAPPER=""`,
`CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d`,
`RUST_MIN_STACK=134217728`.

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly \
  every_maintenance_unit_stays_inside_the_interactive_step_budget -- --nocapture
→ test result: ok. 1 passed; 0 failed; … 582 filtered out; finished in 4.40s

cargo test -p semio-s-artifact-process-process3d retained_laws -- --nocapture
→ test result: ok. 7 passed; 0 failed; … 325 filtered out; finished in 0.14s
```

Concurrency notes: the workspace was broken by peers four separate times during this wave
(`semio-framework-replication` mid-refactor of `🌱️value/🧬️clone`, `semio-framework-os-kernel`
mid de-async sweep, `semio-framework-plugin` mid `Arc<Snapshot>` view migration, the
`🧫️fixtures/⚖️scale` → `🧪️testkit/⚖️scale` crate move, and a missing
`🏛️architectural.dwg` asset). Each was waited out, none was caused by this wave.

**Not verified:**

- `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly` (whole suite):
  **531 passed / 53 failed**, none of them this wave's. The failure families are
  17× `interactive-job.missing-owned-reducer` (`setActiveTool` / `setActiveUtility` have only a
  generic proof — cf. `📓️…bare-bounded-factory-means-every-action-dead`), 7×
  `job-session.terminal-fault`, 3× `first object id`, 1× outliner label, 1× the real
  `fillBuildTick` ceiling breach above. All pre-existing / peer in-flight.
- `cargo test -p semio-s-artifact-process-process3d` (whole suite): aborts with SIGABRT in
  `editor::process3d::component::tests::arg_form_set_stock_emits_ops_reading_kind_arg` —
  "panic in a destructor during cleanup" inside that app's own `testkit::app`. Pre-existing and
  unrelated; the `retained_laws` module passes in full.
- `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly`: see §8.
- `cargo check --target wasm32-wasip2 -p semio-framework-plugin`: see §8.
