# ⏱️ Wave R2 — store replacement step budget (measured, native, opt-level 0)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Date 2026-09-09.

## Verdict, up front

**The store replacement unit is not the boot offender, and `stage=14` is a mis-attribution.**

Measured natively at opt-level 0, on the real paths, min-folded over three independent rounds:

| unit | typical (median) | peak | typical budget | ceiling |
| --- | --- | --- | --- | --- |
| puzzle3d stage 14 `drive_store_replacement_jobs`, Concrete Forest | **1 µs** | **3 µs** | 2 000 µs | 8 000 µs |
| puzzle3d stage 14 `drive_store_replacement_jobs`, Nakagin | **1 µs** | **1 µs** | 2 000 µs | 8 000 µs |
| puzzle3d worst of ALL 22 stages, Concrete Forest | 3 µs (stage 0) | **43 µs** (stage 0) | 2 000 µs | 8 000 µs |
| puzzle3d worst of ALL 22 stages, Nakagin | 3 µs (stage 0) | **18 µs** (stage 0) | 2 000 µs | 8 000 µs |
| store replacement `Initializing` (real initializer worker step, 42 units) | 1 µs | **368 µs** | 2 000 µs | 8 000 µs |
| store replacement `CandidateReady` (disposer + validate + atomic swap, 1 unit) | 1 µs | **1 µs** | 2 000 µs | 8 000 µs |
| store replacement `RetiringCommittedStore` (bounded displaced close, 412 units) | 0 µs | **30 µs** | 2 000 µs | 8 000 µs |

Nothing in the store replacement path is anywhere near the ceiling, so **there is nothing to slice**
(task 2). Slicing an in-budget unit on a hot framework path would be speculative churn against the
measurement, so it was not done. The measurement is a permanent law instead (task 3).

Three structural findings explain the browser trace.

## 1. `drive_store_replacement_jobs` never runs at all for puzzle3d

`Puzzle3dPlayApp` is an `ArtifactEditor`, run as `VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>`
(`✏️s/🔌️plugins/🧩️puzzle/🦀️.rs:14`). It does **not** override
`ArtifactEditor::build_document_store_initialization_job`, so the trait default
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `ArtifactEditor` block) returns
`Err(envelope)`:

- `ArtifactStoreReplacementAdmissionTarget::try_adopt_completed` calls
  `A::build_document_store_initialization_job(envelope, …)?` and therefore **never inserts a
  replacement job** for puzzle3d;
- `drive_store_replacement_jobs` hits `self.store_replacement_jobs.next_id_from(*cursor) → None`
  and returns `Complete` immediately — the measured 1–3 µs;
- consequently the puzzle3d Storybook wasm bridge's envelope-load loop
  (`✏️s/…/🧊️3d/…/✏️editor/🌉️wasm/🦀️.rs`, `pollEnvelopeLoad`) can never reach `Ready`. Separate
  defect, not fixed here (outside R2), but it means **no whole-document store replacement runs
  anywhere in the puzzle3d boot** — the OS React boot does not call
  `begin_artifact_envelope_ingress` at all (the only callers in the tree are the four per-plugin
  wasm bridges: writer, jack, raster, puzzle3d).

## 2. `stage=14` in the browser trace is a stale cursor, not the stage that ran

`VcsArtifactApp::maintenance_step` stores `LAST_MAINTENANCE_STAGE` at entry, **then** takes a large
idle early return (the `tool_operations.is_empty() && … && store_replacement_jobs.is_empty() && …`
chain) *before* `let stage = self.maintenance_stage;` and *before* the `% MAINTENANCE_STAGES`
advance.

So whenever the app is idle — the steady state after boot — the round-robin cursor freezes and
`LAST_MAINTENANCE_STAGE` keeps reporting whatever stage it stopped at, forever. The coordinator's
`[DEBUG] maintenance stage=14 elapsed_us=8401` reads that frozen value, so it names stage 14 for
work stage 14 did not do.

**Action for the coordinator (the trace is yours, I did not touch it):** either move the
`LAST_MAINTENANCE_STAGE.store` to just after `let stage = self.maintenance_stage;`, or store a
distinct sentinel on the idle early-return branch. Until then every over-ceiling trace from an idle
instance keeps accusing stage 14.

The new `VcsArtifactApp::next_maintenance_stage()` gives any caller the stage a unit is *about* to
run, which is what the new laws attribute by.

## 3. The ceiling verdict measures WALL time, and fires on descheduling

`RuntimeLiveCleanupJob::step` brackets `maintenance_step` with `default_now_us()` and feeds the
delta to `semio_framework_trace::interactive_step_contract_violated`. That is wall time on a
contended thread, not the unit's own work.

Reproduced natively, by accident: with an early version of the new law running inside the **full
584-test puzzle3d suite** (dozens of threads on this loaded machine), one Concrete Forest unit
measured **14 571 µs** — for work that measures 1–43 µs when the same test runs alone. Same code,
same document, ~340× the reading. A second full-suite run showed stage 0 at 2 220 µs and stage 8 at
1 160 µs where the isolated run reads 3 µs and 2 µs.

The browser boots ~20 WASM plugins concurrently in a hidden 0×0 pane
(cf. `📓️…hidden-browser-pane-throttles-plugin-boot`), so 8 401 / 9 500 / 10 201 µs readings for
units that cost single-digit microseconds natively at opt-level 0 are consistent with descheduling,
not with real work. **A release-wasm unit cannot plausibly be 1 000× slower than the same unit in an
unoptimised native debug build.**

This is why both new laws budget two statistics — see §4.

## 4. What was measured, and how the laws are built

Each law runs the scenario `ROUNDS = 3` times and folds per stage/phase by keeping the SMALLEST
median and the SMALLEST maximum any round saw (`keep_best_round`). Two statistics are asserted:

- the per-stage **median** unit against a tight 2 000 µs typical-unit budget — a stage that overruns
  because of its own work overruns unit after unit, and a median ignores the machine;
- the per-stage **maximum** unit against the framework's own 8 000 µs
  `INTERACTIVE_STEP_CEILING_US` — that is literally what the ceiling bounds, and 8 000 µs leaves
  enough headroom that a scheduler hiccup surviving three rounds is not credible.

Median precedent in-repo: `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️fixed-operation-registry/🦀️.rs:211`.

### puzzle3d: every maintenance unit, both flagship documents

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
— `every_maintenance_unit_stays_inside_the_interactive_step_budget`. Drives the REAL app
(registry-backed `VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>` bound to instance 1), the REAL typed
`setActiveExample` command for `concrete-forest` and `nakagin-capsule-tower`, real host turns
(`measured_host_turn`: `maintenance_step(1, 4096)` → `advance_typed_operation_publication` →
present/ACK → drain → local-interaction ACKs), then eight full round-robin sweeps, timing every unit
with `semio_framework_job::default_now_us` and attributing it to the stage read **before** the call.
It deliberately does not assert the operation's own result lane — a domain command that faults is a
different law's subject, and the clock must stay in budget either way.

Measured, isolated (`median/peak/units` per stage):

```
concrete-forest        typical_stage=0 median_us=3 peak_stage=0 peak_us=43
  0=3/43us/95u  1=1/2us/191u 2=1/1us 3=1/1us 4=1/1us 5=0/3us 6=1/1us 7=1/2us 8=1/7us 9=1/4us
  10=1/4us 11=1/3us 12=1/3us 13=1/3us 14=1/3us 15=1/3us 16=1/2us 17=1/2us 18=1/2us 19=1/4us
  20=1/1us 21=1/3us          (95 units per stage)
nakagin-capsule-tower  typical_stage=0 median_us=3 peak_stage=0 peak_us=18
  0=3/18us/266u 1=1/4us 2=1/1us 3=1/1us 4=1/1us 5=1/1us 6=1/1us 7=1/1us 8=1/2us 9=1/3us
  10=1/2us 11=1/1us 12=1/2us 13=1/3us 14=1/1us 15=1/2us 16=1/3us 17=1/2us 18=1/3us 19=1/2us
  20=1/2us 21=1/2us          (266 units per stage)
```

Stage 14 (`drive_store_replacement_jobs`) peaks at 3 µs / 1 µs. The only stage above 10 µs is
stage 0 (typed-operation worker/retirement).

### framework: the store replacement unit's three phases

`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-laws/🦀️.rs`
— `every_store_replacement_phase_unit_fits_the_interactive_step_budget`. process3d is the domain
that actually implements `build_document_store_initialization_job`, so this is the real path:
`Process3dStoreInitializationAuthority::step` (`Initializing`), then
`ArtifactDocumentStoreDisposer::new()` + `process3d_validate_atomic_lease` +
`publish_document_store_candidate_if_authoritative` (`CandidateReady`), then the bounded one-item
displaced-store close (`RetiringCommittedStore`).

```
typical_phase=Initializing median_us=1 peak_phase=Initializing peak_us=368
  Initializing=1/368us/42u  CandidateReady=1/1us/1u  RetiringCommittedStore=0/30us/412u
```

Against the suspect list in the brief:

- `publish_document_store_candidate_if_authoritative` — **1 µs**. It is a `validate()` plus one
  `std::mem::replace`; `ArtifactStore` is a struct of heap handles, not an inline arena, so the swap
  moves a few hundred bytes. Not a cost centre.
- `build_document_store_disposer` — inside that same 1 µs (a zero-sized `PhantomData` disposer).
- `ArtifactStoreInitializationOwnerCatalog::try_new`'s six `try_reserve_exact(64)` — inside
  `Initializing`, whose 42 units peak at 368 µs and typically cost 1 µs. Not isolated further,
  because the whole phase is already 20× under the ceiling.
- W-B's batch machinery, W-P2's spatial index, the envelope decode page loop — none of them appear:
  their stages (0, 10, 11) peak at 43/4/3 µs on the real documents.

## 5. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `pub const MAINTENANCE_STAGES: u8`,
  the length of the cooperative round robin, replacing the `% 21` literal in `maintenance_step`
  (a peer has since added a 22nd stage and updated the constant, which is exactly what it is for);
  re-exported from the crate root.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — new
  `VcsArtifactApp::next_maintenance_stage()`, immediately before `begin_artifact_envelope_ingress`,
  so a budget law can name the stage a unit ran (reading `LAST_MAINTENANCE_STAGE` afterwards names
  a stale one — §2).
- `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` — `RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP`,
  `MAINTENANCE_UNIT_SAMPLES`, `MaintenanceStageBudget` (fixed
  `[[u64; 64]; MAINTENANCE_STAGES]` samples plus per-stage max/count; `median`, `worst`,
  `worst_unit`, `keep_best_round`, `report`); `Puzzle3dApp` is now a named struct carrying its
  `maintenance` budget with `measure_maintenance_step`; `settle` drives
  `measure_maintenance_step`, so every existing fixture accumulates the budget for free.
- `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the new law, its four budget constants and
  `measured_host_turn`.
- `✏️s/…/🧊️process3d/…/💾️binary/🧪️tests/🔬️retained-laws/🦀️.rs` — `REPLACEMENT_PHASES`,
  `REPLACEMENT_UNIT_SAMPLES`, `REPLACEMENT_PHASE_BUDGET_US`, `REPLACEMENT_STEP_CEILING_US`,
  `ReplacementPhaseBudget` (same two statistics), `measure_phase`, `owned_store_measured`, the new
  law and `measure_one_store_replacement`; plus `PUBLICATION_AUTHORITY_LANE` — see below.
- `✏️s/…/🧊️process3d/…/💾️binary/🧪️tests/🔬️retained-laws/🦀️.rs` — **bug fixed while measuring:**
  `process3d_admit_publication_authority` fills ONE process-global fixed authority table and answers
  `process3d-publication.saturated` when two holders overlap, so two laws in the same file that mint
  real publication leases abort the whole test binary under `cargo test`'s own parallelism
  (`saturated` → `expect` panic → leaked store → "artifact store cursor disposer reached Drop
  before terminal-empty ownership" → non-unwinding abort). Both now take
  `publication_authority_lane()` first.

Nothing in `⚛️reactor/🔄️turn/🦀️.rs`, `RuntimeLiveCleanupJob`, `🕹️interaction/**` or the
coordinator's `[DEBUG]` traces was touched. No wasm was rebuilt and no server was started.

## 6. Other defects found while measuring (not R2's, routed on)

1. **`fillBuildTick` really does breach the ceiling.** The existing law
   `fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin` failed one full-suite
   run with `fillBuildTick turn 763 took 25.593042ms, at or over the framework's interactive step
   ceiling 8ms` (it passed a later run, so it is load-sensitive too). That is the only >8 ms
   interactive unit this wave found anywhere. If the browser fault is real work rather than
   descheduling, **this** is the shape to look at, not stage 14.
2. **`setActiveExample nakagin-capsule-tower` faults** its retained operation with
   `job-session.terminal-fault`, so the flagship example cannot be loaded through its real typed
   command. Same family as the other in-flight failures in §7.
3. **puzzle3d's envelope-load bridge is dead** — see §1.

## 7. Commands run, and what is NOT verified

All foreground, `RUSTC_WRAPPER=""`,
`CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad/target-p3d`
(wasm check: `…/target-p3d-wasm`), `RUST_MIN_STACK=134217728`.

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly \
  every_maintenance_unit_stays_inside_the_interactive_step_budget -- --nocapture
→ test result: ok. 1 passed; 0 failed; … 583 filtered out; finished in 4.81s

cargo test -p semio-s-artifact-process-process3d retained_laws -- --nocapture
→ test result: ok. 7 passed; 0 failed; … 325 filtered out; finished in 0.10s

cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly
→ test result: FAILED. 536 passed; 48 failed  (the new law: ok — it survives full parallel load)

cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
→ Finished `dev` profile in 25.71s, 0 errors

cargo check --target wasm32-wasip2 -p semio-framework-plugin
→ Finished `dev` profile in 1m 50s, 0 errors   (wasm32-wasip2 IS installed)
```

**Not verified:**

- `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly` (whole suite):
  **536 passed / 48 failed**, none of them this wave's; the new law passes. The failure families are
  17× `interactive-job.missing-owned-reducer` (`setActiveTool` / `setActiveUtility` carry only a
  generic proof — cf. `📓️…bare-bounded-factory-means-every-action-dead`), 9×
  `job-session.terminal-fault`, 2× `first object id`, 1× outliner label, plus the load-sensitive
  `fillBuildTick` ceiling breach of §6.1. All pre-existing / peer in-flight.
- `cargo test -p semio-s-artifact-process-process3d` (whole suite): aborts with SIGABRT in
  `editor::process3d::component::tests::arg_form_set_stock_emits_ops_reading_kind_arg`
  ("panic in a destructor during cleanup" inside that app's own `testkit::app`). Pre-existing and
  unrelated; the whole `retained_laws` module passes, in parallel and single-threaded.
- `cargo test -p semio-framework-plugin` (the crate whose lib I edited): its **lib test target does
  not compile**, from peer work only — a peer added a `terminal_fault` field to
  `MountedTypedCommandFullOperation` without updating
  `🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` (4× E0063), and
  `🧪️tests/🧩️composition/🦀️.rs:126` `include_str!`s a
  `🏪️store/🧩️composition/🪪️member-dialect/🧪️tests/🔣️.json` that is currently absent. The plugin
  **lib** itself checks clean natively and for `wasm32-wasip2`, as above.
- `cargo test -p semio-framework-os-kernel` (the store crate): aborts in
  `os_pack::value::tests::retained_value_vm_covers_every_wire_tag_and_terminal_empty_close` and
  fails `os_pack::value::tests::large_bytes_field_is_chunked_and_round_trips` — peer pack/value work
  in flight; this wave did not touch that crate.

Concurrency notes: the workspace was broken by peers six separate times during this wave
(`semio-framework-replication` mid-refactor of `🌱️value/🧬️clone`, `semio-framework-os-kernel`
mid de-async sweep, `semio-framework-plugin` twice — the `Arc<Snapshot>` view migration and a
`BTreeMap::range` inference error, the `🧫️fixtures/⚖️scale` → `🧪️testkit/⚖️scale` crate move, and a
missing `🏛️architectural.dwg` asset). Each was waited out; none was caused by this wave.
