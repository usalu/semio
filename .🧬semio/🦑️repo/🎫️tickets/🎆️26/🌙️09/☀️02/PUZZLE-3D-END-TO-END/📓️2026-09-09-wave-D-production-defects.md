# Wave D — puzzle3d production defects P1-P10

Ticket 26/09/02/PUZZLE-3D-END-TO-END. Input: `📓️2026-09-09-wave-X-test-suite.md` §5 (P1-P10) and §7.
Scope: production code in puzzle3d's own files, plus the two test-side ownership obligations the
framework's own destructor contracts make mandatory. No test assertion was weakened; every number below
was **run**.

---

## 1. TL;DR

| | wave X baseline | wave D now |
| --- | --- | --- |
| isolated (`🔍️isolate-puzzle3d-tests.py`, one process per test) | 577 tests, **405 ok / 172 failed** | 579 tests, **515 ok / 64 failed** |
| whole suite, one process (`cargo test … -j 4`) | **SIGABRT, no summary** | **493 passed / 86 failed, 0 aborts** |
| `standards::…::mutations` (the P9 cohort) | 87 failed | **0 failed** |

**The suite can be measured as a suite again.** The abort chain is gone: no destructor panic, no
`panic in a destructor during cleanup`, no `SIGABRT`. Eight of the ten named defects are fixed and
verified. The two that are not (P4, P10) are both **framework-side**, and §4 states exactly why puzzle3d
cannot reach them.

Gates: `publication-authority-audit Puzzle3dPlayApp` green (63 admitted routes); the three puzzle-owned
`verify interactivity` self-test suites PASS; `cargo check --features component-app-assembly --tests`
0 errors / 0 warnings; rustfmt exit 0 on every edited file.

---

## 2. Fixed, with the test that now passes

### P1 puzzle3d can never close — FIXED
`VcsArtifactApp::close_step` walks seven mandatory owned lanes; puzzle3d supplied disposers for two, so
`drive_artifact_owned_disposer` faulted `interactive-job.close-owned-disposer-missing` for `draft-store`
and no instance could ever reach its terminal-empty witness — in a test process OR in a host, where the
framework-installed `ArtifactStoreCursorDisposer` members then panic in `Drop`.

* `✏️editor/🦀️.rs` `build_draft_store_owners` / `build_draft_store_disposer`
  (`bounded_document_store_owners`/`…_disposer::<NoDraft, NoDraftMutation>`),
  `build_transient_store_disposer` (`NoTransientStoreDisposer::new()`),
  `build_presence_local_root_retirement_factory` + `build_presence_peer_retirement_factory` +
  `build_presence_store_disposer`.
* `✏️editor/👥️presence/🦀️.rs` — new `//#region 🧹️Retirement`: `puzzle3d_presence_is_terminal_empty`
  (`active_tool_id.is_none()` — the ONE heap owner a presence root holds; every other field is an inline
  `f64` array), `Puzzle3dPresenceRetirementFactory` (returns the UTF-8 tool id in one bounded turn, the
  inline root in the next), `puzzle3d_presence_store_disposer` over the framework's own
  `PresenceStoreOwnedDisposer`. `PresenceStore::begin_retirement` refuses without the installed
  local-root factory, which is why all three hooks move together.

**Test:** `editor::puzzle3d::component::tests::fixture_app_reaches_its_terminal_empty_close_witness` — ok.

### P2 `setActiveTool` / `setActiveUtility` were hard dead — FIXED
`dispatch_action` routes a framework-injected host-configuration verb through
`A::host_configuration_mutation`; puzzle3d did not implement it, so both verbs fell to
`admit_command_json` and failed `interactive-job.missing-factory`. The fill tool could not be selected at
all. The shell's own `SET_ACTIVE_TOOL_ACTION_ID` / `SET_ACTIVE_UTILITY_ACTION_ID` branches
(`🏛️ShellHost/🟦️.tsx`) own the session state and then **forward the resolved value to the plugin "so it
can clear/prepare scratch"** — that forwarded call is exactly what had no route.

Three coupled changes, because a proof and a mutation are both required:

1. `✏️editor/🦀️.rs` `host_configuration_mutation`: `setActiveTool` →
   `Puzzle3dConfigMutation::SetActiveTool { tool_id }`; `setActiveUtility` →
   `SetWindowEngagementInput { window_id, value: "" }` — puzzle3d's active utility IS host view state
   (`puzzle3d_scene_active_utility` reads `ViewModel.active_utility_by_window_id`), so the only app-owned
   state a utility switch invalidates is that window's engagement scratch, and an existing invertible
   variant already expresses it.
2. `✏️editor/🎚️config/🦀️.rs` new `SetActiveTool { tool_id: Option<String> }` variant + descriptor +
   `diff` (active tool, suggestion menu, brush candidate index, every window's engagement input — exactly
   what `🎮️commands/🧰️set-active` does) + a multi-step `inverse` (the one variant whose forward touches
   more than one field).
3. The proof. `qualified_host_configuration_tool_proof` needs an entry in `app_tool_registrations`,
   `framework_tool_registrations` or `bounded_tool_proofs`, and `validate_tool_job_rows` admits a bounded
   row only if the tool id is in `<Command as OpBinary>::TOOL_JOB_IDS` ∩ the registry's `Migrated`
   declarations. Both verbs are already `interactiveJob: "migrated"` in `🧩️puzzle/🔣️.json`, so:
   `TOOL_JOB_IDS` gained the two ids, and the single `bounded_first_step_tool_proofs!` invocation was split
   into `Puzzle3dRetainedCommandProofs` (unchanged, `factory_type: Puzzle3dRetainedCommandJobFactory`) and
   a new `Puzzle3dHostConfigurationProofs` carrying **generic** proofs (`factory:
   "BoundedFirstStepCommandJobFactory"`, no `factory_type`) for the two verbs — the only shape
   `validate_tool_job_rows` accepts for a tool the retained factory does not claim.
   `PUZZLE3D_RETAINED_TOOL_IDS` is untouched, so
   `retained_command_catalog_excludes_framework_owned_shared_actions` stays true and the host-configuration
   branch's `require_tool_operation_authority` (which a Bounded proof satisfies) is reached instead of
   `require_complete_tool_operation_pipeline` (which it would not).

**Effect:** zero `interactive-job.missing-factory` failures remain; all 17 previously-blocked tests run.

### P3 the Config publication lane admitted exactly one variant — FIXED
`puzzle3d_config_store_mutation_bytes` matched `Snapshot` and returned `None` for everything else;
`Puzzle3dConfigStorePreparation::advance` rejects whatever it returns `None` for and the operation's
publication lease is cancelled behind it. The whole `Puzzle3dScalarConfigWork` family published nothing.

`✏️editor/🦀️.rs`: the byte cost is now derived from the variant's own encoded payload and bounded by the
single `PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES` envelope —
`(to_json_string(mutation).len() <= PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES).then_some(len)`. Every variant is
admissible; only an oversize payload is refused; `Snapshot`'s cost is still the whole-config cost, because
its payload *is* the config.

`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` pinned the old allowlist literal as its
publication-authority anchor AND as its "widened Config mutation envelope" hostile mutation. Both were
repointed at the new authority (`fn puzzle3d_config_store_mutation_bytes(...)` + the bound line), and the
hostile mutation now strips the bound instead of the allowlist — still a real refusal.

**Tests:** every `interactive-job` Config-lane rejection is gone; `set_camera_*`, `setVortexShow`,
`setVortexDirection`, `closeVortexSuggestions`, the kind-weight and engagement routes all publish.

### P5 the inspection panel's flag rows — FIXED (root cause was NOT capacity)
The report read `ui.inspection.fields: puzzle3d inspector action map entry admission failed` as a fixed
capacity being too small. It is not: `UiMapBuilder::push` (`🖱️ui/🧬️contract/…/🎬️action.rs`) admits keys in
**strictly ascending order only** and returns the allocation otherwise. `flag_row` pushed
`entity, ids, field, value` — which regresses at `field` — so every flag row on a live selection failed.
`✏️editor/📌️panels/🔍️inspection/🦀️.rs`: reordered to `entity, field, ids, value`, with the ordering
contract stated at the call site.

**Test:** `selected_object_inspector_renders_that_object_field_group` — ok.

### P6 a standalone `Puzzle3dStore` could not record an edit — FIXED
`Puzzle3dStore` is a bare `ArtifactStore` alias, so `ArtifactStore::new` yields a store with no mutation
retirement factory and its first `Apply` fails
`edit history insertion requires its exact mutation retirement factory`.
`🧬️schema/🧬️mutations/💾️binary/🦀️.rs` now owns the lifecycle:

* `puzzle3d_store(envelope)` — installs exactly the owners `Puzzle3dPlayApp::build_document_store_owners`
  hands the host, so both entry points record edits under one authority.
* `close_puzzle3d_store(&mut store)` — installing owners also installs the cursor disposer whose Drop
  witness the store asserts (`artifact store reached Drop without its exact terminal-empty shallow-shell
  witness`), so a standalone store is no more droppable-on-the-floor than a host-owned one. Bounded by
  `PUZZLE3D_STORE_CLOSE_TURNS`, derived from the ledger capacity × the shell owners per edit.

Four call sites moved onto the pair (the CW7 law in `✏️editor/🧪️tests/🔬️unit`, and the binary/text
snapshot + binary mutation unit tests).

**Tests:** `mutations::binary::tests::puzzle3d_document_vcs_replays_granular_operations`,
`snapshot::binary::tests::command_envelope_round_trip_holds_for_an_applied_operation`,
`snapshot::text::tests::…`, `component::tests::command_envelope_round_trip_holds_for_an_applied_operation`
— all ok.

### P7 `⏳️precompute` dropped job owners without their incremental close — FIXED
`WorkerJobSessionAdmissionRejected::drop` (`🧵️job/🦀️.rs:2440`) asserts exact incremental close, and
`RetainedJobPayload::drop` (`:663`) asserts one-page close. Five owners were released without it, and each
one aborted the whole binary from a destructor during unwinding — which is what erased libtest's summary.

`✏️editor/⏳️precompute/🦀️.rs`:
* `retire_rejected_fill_worker(rejected)` + `REJECTED_FILL_WORKER_CLOSE_TURNS = 6`, derived (one turn for
  `SharedFillWorkerJob`'s single `Arc`, one to observe `Complete`, one each for the job shell, the batch
  params and the fault page, plus one).
* `reserve`'s rejection path (was `begin_close()` + **one** `close_step`).
* `start_fill_preparation`: the replaced and the cleared `fill_rejected_worker` both retire instead of
  being dropped (`= Some(rejected)` / `= None`).
* new `impl Drop for Puzzle3dCollision` — the engine is the owner, so every path that lets one go (session
  Drop, a standalone engine in a test, a replaced engine) retires it in one place instead of each caller
  remembering to. This is what removed the last abort.

`⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`: a `faulted(outcome)` helper closes the fault detail the four
`assert!(matches!(…, StepOutcome::Fault(_)))` sites took ownership of. Not a weakening — the assertion is
unchanged and strengthened (`stale_generation_faults_without_progress` now also pins the fault body
`b"stale-fill-operation"`); every production caller closes that payload and so must a test.

### P8 fill-worker admission poisoned by earlier sessions — FIXED (the process-global part)
Requesting the `Closed` terminal is not enough: only `FillEnvelopeTerminalHandle::close_step` takes the
registry slot back, so a session that merely asked and died left one of `FILL_ENVELOPE_MAX_OPERATIONS = 4`
slots occupied forever. After four such sessions `begin_measurement` finds no candidate and every later
`enqueue_fill_job` returns `None`.

`✏️editor/⏳️precompute/🦀️.rs` `impl Drop for Puzzle3dPrecomputeSession` now drains
`pump_fill_terminal_step` — which also collects envelopes earlier sessions abandoned — bounded by
`FILL_ENVELOPE_SESSION_CLOSE_TURNS = FILL_ENVELOPE_MAX_OPERATIONS * (FILL_ENVELOPE_MAX_ITEMS + 9)`, derived
from the very census ceiling `finish_measurement` admitted those envelopes against.

**Evidence:** the whole suite reaches a summary; the shared-process failure count (86) is now within 22 of
the isolated count (64), where before the process died. **One test still fails in isolation** —
`set_fill_count_step_work_advances_a_real_admitted_fill_plan` — see §4.

### P9 mutation fixtures vs. the canonical JSON writer — FIXED, the fixtures were stale
**The writer is authoritative.** `os_pack::json` states its own contract in
`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs`: number formatting is byte-identical to `serde_json`'s
`zmij`-based writer, and *"A float and an integer of the same magnitude are still never spelled the same
way (`42.0` always keeps its `.0`)"*. An `f64` field therefore MUST encode as `1.0`. The committed
fixtures spelled integral `f64` fields as bare integers, so `committed_json_is_canonical`,
`committed_diff_is_canonical` and `produces_committed_diff` compared `Number(1.0)` to `Number(1)` — and
`committed_json_is_canonical` is itself the law that the committed JSON be a decode→encode fixed point, so
the fixtures were failing their own declared contract.

The Python second implementation is unaffected: it hands back a parsed payload
(`Outcome(payload, raw=json.dumps(...))`) and `@comparison-ordered-json-v1` compares parsed JSON in
TypeScript, where `8` and `8.0` are the same number. Nothing else lives beside the quintets (no derived
`.dsl.semio`/`.pack.semio` encodings in those case directories), so no encoding needed regeneration.

Repair, one shot, kept in the ticket folder as
`🔨️canonicalize-puzzle3d-mutation-fixture-floats.py`: schema-driven (`"type": "number"` → float,
`"type": "integer"` left alone — `index`, `order`, `revealIndex` and the fill-preview counters are genuine
integers), idempotent, byte-stable (all 175 files round-trip through
`json.dumps(indent=2, ensure_ascii=False)` unchanged, so the only textual change is number spelling).
**91 of 175 fixture files changed.**

One thing the script had to supplement, and it is a finding of its own: the committed artifact JSON Schema
(`🧬️schema/🔣️.json`) is INCOMPLETE — `Puzzle3dObject.scale` is `{}` and both `Puzzle3dTargetVolume` and
`Puzzle3dReference` are `additionalProperties: true` stubs carrying only `id`. The script's `SUPPLEMENT`
table cites the Rust records that are the authority for those three (`🗿️artifacts/🧊️3d/🦀️.rs:82`, `:227`,
`:308`, `:347`). **Completing the committed schema is not done here** — its GraphQL/proto/TypeScript
siblings and the design-parity harness move with it. Flagged to the coordinator.

**Tests:** `standards::v1::subsets::any::schema::mutations` — 262 run, 262 ok (was 184/262).

### Task 8 `nx test` never ran a single app test — FIXED
The whole `editor` tree is `#[cfg(feature = "component-app-assembly")]` (crate root `🦀️.rs:14-21`), so a
default-feature `cargo test` compiled 286 of 579 tests and ran none of the `editor::puzzle3d` app tests.
There is no `[package.metadata.semio]` feature mechanism in the repo, and `--features` is already a
required BUILD option in `partitionNextestExecutionFilters`, so the declaration goes where the package
owns it:

* `🧰️framework/…/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts` — `runArtifactRustTests` gained a
  `testFeatures: readonly string[] = []` parameter (feature args go in front of the caller's own, so the
  warm build and the execution pass agree), and `runArtifactRustPackageMain` an
  `options: { testFeatures? }`. Additive and default-empty: no other artifact package changes behaviour.
* `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` —
  `{ testFeatures: ["component-app-assembly"] }`, beside the `RUST_MIN_STACK` wave X added.

No new command appeared, so `launch.json` is untouched (`📋️project.json`'s `test` target already routes
through this script).

---

## 3. Two production defects found in the course of this wave, not in P1-P10

### `Puzzle3dAddObjectKindWork::extent` refused a document with no kind catalogs — FIXED
`extent` returned `None` when `meta.kind_catalogs` is absent, which the preflight reports as
*"puzzle command exceeds fixed semantic work capacity"* — conflating "nothing to do" with "over
capacity". `step`'s own no-catalog branch completes in ONE turn, so `extent` now returns `Some(1)`. Every
`addObjectKind` after `setActiveExample ""` (which clears the catalogs) was faulting; that single defect
accounted for several of wave X's "typed-operation cancelled" reports.

### The "cancelled" message hides the real fault (diagnosis, no fix)
`typed-operation cancelled before its next publication unit` is emitted by
`reject_cancelled_publication`, which fires whenever the lease is cancelled — and
`MountedTypedCommandFullOperation`'s worker pump cancels the lease on ANY terminal
`StepOutcome::Cancelled | Fault(_)` before the fault body is ever surfaced. So a real app fault reaches
the app author as a generic cancellation. Measured (temporary instrumentation, since removed): the bucket
is two distinct causes —
`addObjectKind`'s extent refusal (fixed above) and `job-session.terminal-fault`, which is the framework's
pre-admitted page for a **step-budget quarantine**: `interactive_step_contract_violated` faults a step at
`INTERACTIVE_STEP_CEILING_US`, and `openVortexSuggestions` measured **14 644 µs**, `fillBuildTick`
**11 705 µs**, `acceptSuggestion` **11 796 µs** in one step of an unoptimized build. That is a real
per-step budget overrun in `⏳️precompute`'s hot paths — the same class §7 already lists as
`fill_first_substantive_preview_arrives_below_fifty_ms_and_every_step_below_eight_ms` — and it belongs to
W-P/W-F, not to any of P1-P10. Surfacing the fault body instead of the cancellation is a framework change
(`🔌️plugin/🦀️.rs`) worth its own ticket: it cost wave X and this wave hours each.

---

## 4. Not fixed, and exactly why

### P4 the document one-item publication has no preinstalled capacity — FRAMEWORK CEILING
Measured, not inferred (temporary instrumentation in `🏪️store/🦀️.rs`, since reverted):

```
one-item publication requires preinstalled fixed applied and revision capacity
  applied=64/64 revision=64/64 cursor=Some((64, 64))
```

64 is `os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY`, reserved by
`ArtifactStoreInitializationOwnerCatalog::try_new()` — a framework constant with no per-app knob
(`ArtifactStoreInitializationRuntime::push_applied` refuses at the same number, and the store exposes no
capacity installer: `install_snapshot_retirement_factory`, `install_owned_disposer`,
`install_member_store_owners_exact` and nothing else). A retained tool operation publishes
`emit.artifact_mutations` ONE PER TURN through `store.begin_apply_one`, so `setActiveExample nakagin`
(180 objects + domain + catalogs ⇒ ~182 one-item edits) saturates the ledger at edit 64.
**No puzzle3d-side change can preinstall past it.** The two honest options are both out of this wave's
remit: raise `ARTIFACT_HISTORY_LEDGER_CAPACITY` (fleet-wide memory + a `🏪️store` fixture that pins the
value), or give the document vocabulary a whole-document replacement leaf so an example load is one
edit — which means a new `Puzzle3dMutation` variant, its leaf directory, its JSON schema, its committed
quintet and its Python oracle arm. Coordinator decision.
**Tests still red:** `nakagin_example_loads_via_operations`,
`set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document`.

### P10 `worker-pump` rejects the reserved mounted worker transition — FRAMEWORK, AND FLAKY
Not deterministic: `selected_object_inspector_renders_that_object_field_group` fails under the isolated
sweep and passes when run alone, immediately after. The cause is pool contention.
`run_framework_reserved_job` builds `process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores))`
and treats ANY `pump_one` error as a hard `interactive-job.worker-pump` fault — including
`Submit(Pool(Contended | Saturated))`, which is transient. puzzle3d's own fill pump
(`⏳️precompute/🦀️.rs`) handles exactly those two variants gracefully and retries; the reserved-job pump
does not. puzzle3d cannot move its fill workers to a different pool: `process_worker_pool` is a process
singleton that asserts on a config mismatch, so `fill_worker_pool()` MUST name the same
`ProcessKind::InteractiveNative` — the separation the framework offers is the `Lane`, not the pool.
**The fix is one retry-after-yield in `🔌️plugin/🦀️.rs`'s reserved-job loop.** Coordinator decision (peer-
contended file).

### P8's residual — `set_fill_count_step_work_advances_a_real_admitted_fill_plan`
Still red in isolation on `a fresh session with no live job must admit a fill worker`. The process-global
saturation is fixed (§2); what remains is that `enqueue_fill_job` returns `None` on its FIRST call for a
fresh session because the admission census (`FillBuilderOwnerCensusCursor::step`) is resumable and
answers `Pending`, while the test calls it once. Either the test drives it to completion or
`enqueue_fill_job` completes its own admission — a W-F call, inside `🪣️fill`'s capacity design.

### The 64 remaining isolated failures, by owner

| n | module | owner |
| ---: | --- | --- |
| 20 | `editor::puzzle3d::precompute::{component,fill,geometry}::tests` | **W-F** — the fill-capacity tests `📓️2026-09-08-wave-F-fill-capacity.md` §NOT-verified listed as written-but-never-executed. They execute now. |
| ~14 | `component::tests` on the step-budget quarantine | **W-P** — §3, per-step overrun in `⏳️precompute` |
| 8 | `component::tests` on `interactive-job.worker-pump` | framework, P10 |
| 5 | `component::tests` `main window measures` / `base measures` / `document tree objects section` | **W-G2** — `window_measures_body` and `window_engagements` both bail with an empty map when `puzzle3d_labels(view_state)` is `None`, which it is for `ViewModel::default()`; `🗣️terminology` is W-G2's file and off-limits to this wave |
| 2 | `nakagin_example_loads_via_operations`, `set_active_example_…` | framework, P4 |
| 1 | `retained_command::tests::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle` (`Some(112)` vs `Some(120)`) | **peer in flight** — a concurrent session added `context_identity: u64` to `PuzzleCommandCheckpointState` (+8 bytes) at 04:43 without moving its fixture. This wave added the missing field initializer to that struct literal in `🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs` only because the crate would not compile at all otherwise; the byte-count fixture is theirs. |
| rest | single behavioural assertions in `component::tests` | mixed W-S / W-F |

---

## 5. Audit outputs

```
bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts publication-authority-audit Puzzle3dPlayApp
  validated Puzzle publication authority; owners=Puzzle3dPlayApp; admitted=<63 routes>; schema=Ajv; oracle=independent

bun .🧬semio/…/PUZZLE-3D-END-TO-END/🔍️probe-wave-G2-puzzle-interactivity-clauses.ts .
  PASS interactivityPuzzleFillEnvelopeSelfTests
  PASS interactivityPuzzleFillP4eSelfTests
  PASS interactivityPuzzleFillPreviewJsonSelfTests

CARGO_TARGET_DIR=… RUSTC_WRAPPER="" cargo check -p semio-s-artifact-puzzle-3d \
    --features component-app-assembly --tests -j 4
  0 errors, 0 warnings

rustfmt --unstable-features --skip-children <every edited .rs>   # repo rustfmt.toml, edition 2021
  exit 0, all 12 files
```

`bun ./📜️script.ts verify interactivity` (the CLI) still aborts before printing a verdict, at the
FRAMEWORK-owned `interactivityLiveReconcileSelfTests` `per-surface-credit-cap` mutation — the same stale
literal `🔍️probe-wave-G2-puzzle-interactivity-clauses.ts`'s own docstring records, plus unrelated
`✏️s/🔌️plugins/✒️writer` discovery failures and a `launch.json` fixed-capacity overflow. Nothing in that
output names puzzle; the puzzle-owned suites are the three above, run directly.

---

## 6. Files changed

Production:
* `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — P1 hooks, P2
  hook + proof split + `TOOL_JOB_IDS`, P3 byte function, `addObjectKindWork::extent`
* `…/✏️editor/👥️presence/🦀️.rs` — P1 presence retirement region
* `…/✏️editor/🎚️config/🦀️.rs` — P2 `SetActiveTool` variant, descriptor, diff, inverse
* `…/✏️editor/📌️panels/🔍️inspection/🦀️.rs` — P5 map key order
* `…/✏️editor/⏳️precompute/🦀️.rs` — P7 (4 sites + engine Drop), P8 session Drop drain
* `…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` — P6 `puzzle3d_store` / `close_puzzle3d_store`

Tests (ownership obligations only, no assertion weakened):
* `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `…/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`,
  `…/🧬️schema/📸️snapshot/{💾️binary,📝️text}/🧪️tests/🔬️unit/🦀️.rs` — P6 constructor + close
* `…/✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` — P7 `faulted(...)` helper
* `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs` — one missing field initializer, to
  unblock compilation of a peer's in-flight struct change

Scripts / gates:
* `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` — task 8
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts` — task 8
* `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` — P3 audit anchor + hostile mutation

Fixtures: 91 of 175 files under
`…/🧬️schema/🧬️mutations/*/🧪️tests/**/{📸️snapshot/{⬅️before,➡️after},🦠️mutation,🔺️diff}/🔣️.json`.

Ticket folder: this report + `🔨️canonicalize-puzzle3d-mutation-fixture-floats.py`.

---

## 7. Not verified

1. **No runtime/browser confirmation.** Nothing here was observed in a running app; every verdict is a
   cargo test, a TypeScript gate, or measured instrumentation.
2. Task 8 IS verified through the package script itself —
   `SEMIO_TEST_LEVEL=fundamental bun ./📜️script.ts test` in
   `…/🧊️3d/📦️packages/🦀️rust` reports `Summary 25/579 tests run` under cargo-nextest, i.e. the whole
   `component-app-assembly` suite (579) instead of the default-feature 286. It fails fast on the first
   red tests, which is the level filter working, not the feature declaration failing. `nx test
   @semio-tech/puzzle-3d-rs` itself (the Nx wrapper around that same command) was not invoked.
3. **P4 and P10 are diagnosed, not fixed** — both need a framework change (§4).
4. **The step-budget overruns (§3) are measured, not fixed.** Whether 14 ms is only an unoptimized-build
   cost or also a release-build one was not measured.
5. **Cross-test pollution still exists**: 86 shared-process failures vs 64 isolated. The fill registry is
   still process-global by design, and the interactive worker pool is shared — P10's contention is the
   visible part.
6. **The committed artifact JSON Schema's three incomplete record types** (P9) are supplemented inside the
   repair script, not fixed in the schema.
7. **`Puzzle3dCommand::SetActiveTool` and its `🎮️commands/🧰️set-active` arm are now unreachable** from the
   typed channel (the verb has no app-owned proof, deliberately). Kept, following `🏭️process3d`'s own
   `setContributions` precedent of declaring both a command variant and a `host_configuration_mutation`.
   Deleting the arm, the variant and the leaf is a command-vocabulary change with manifest and
   design-parity gates attached — coordinator decision.

---

## 8. ⚠️ Collision: a concurrent `Puzzle3dConfig` split landed at 06:05, after this wave's last green run

Everything in §1-§5 was measured against a crate that compiled with **0 errors / 0 warnings at 06:03**.
At **06:05** a concurrent session rewrote `✏️editor/🎚️config/🦀️.rs` wholesale (its generated escaped-unicode
owner strings are the signature): the old `Puzzle3dConfig` is now `Puzzle3dRuntime` (all per-window and
session state, `active_tool_id` included) and a NEW four-field `Puzzle3dConfig`
(`fill_count`, `overlap_budget`, `object_kind_weights`, `vortex_kind_weights`) is the app's
`ArtifactApp::Config`. `Puzzle3dConfigMutation` shrank to five variants against it.

State as of this hand-over: **the crate does not compile — 181 errors**, of which **176 are that refactor's
own fallout** (`Puzzle3dRuntime`, `fill_applied_count`, `suggestion_menu`, `window_options`, `camera`,
`engagement_input` call sites across `✏️editor/🦀️.rs`, `⏳️precompute` and the panels) and **5 are this
wave's P2 config-mutation half**, whose `Puzzle3dConfigMutation::SetActiveTool` variant and
`SetWindowEngagementInput` reuse the peer deleted along with the enum.

Everything else this wave landed **survived intact and is unaffected by the split**: P1 (all five
`ArtifactEditor` close hooks + the whole `👥️presence` retirement region), P2's `host_configuration_mutation`
hook, the `Puzzle3dRetainedCommandProofs` / `Puzzle3dHostConfigurationProofs` split and the two
`TOOL_JOB_IDS` entries, P3's `puzzle3d_config_store_mutation_bytes`, P5, P6, P7, P8, P9 and task 8.

**What P2 needs to be re-landed on top of the new shape, and why it was NOT guessed here:**
`host_configuration_mutation` returns a `Self::ConfigMutation`, and under the new split `active_tool_id`
lives on `Puzzle3dRuntime`, not on `Puzzle3dConfig` — so the mode-level active tool is no longer expressible
as a Config mutation at all. Whoever finishes the split decides which lane `Puzzle3dRuntime` becomes
(window-config or window-transient), and P2's two lines follow that decision:

* `✏️editor/🦀️.rs` `host_configuration_mutation` — `SET_ACTIVE_TOOL_ACTION_ID` must emit whatever mutation
  sets `Puzzle3dRuntime::active_tool_id` (plus the scratch a tool change invalidates: `suggestion_menu`,
  `brush_candidate_index`, every window's `engagement_input`), and `SET_ACTIVE_UTILITY_ACTION_ID` whatever
  clears that window's `engagement_input`. If `Puzzle3dRuntime` lands on the window-config lane, the hook
  itself moves with it — `dispatch_action`'s host-configuration branch is Config-lane only.
* The proof half (§2 P2 item 3) is lane-independent and already correct as it stands.

Nothing here was reverted, and no peer line was touched: per CLAUDE.md this wave kept to its own task and
recorded the collision rather than fighting a moving refactor. Re-run the numbers in §1 once the split
compiles.
