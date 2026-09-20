# FP2 — `semio-framework-plugin --lib`: the B1 packet and the singles

Slice: continue FP1. Inherited state (FP1 round 4, `🗑️generated/fp1-round4-suite.txt`):
700 passed / 112 failed. **FP2 re-measured that same tree at 698 / 114** (two load-flaky laws, §5).

Started 2026-09-20 (session 5b+). All runs are
`cargo test -p semio-framework-plugin --lib --no-fail-fast` under preamble rule 25
(`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp1`, inherited warm from FP1 — one cargo at a
time, foreground, no sub-agents).

## 1. Round table

| round | worked | passed | failed | capture |
|---|---|---|---|---|
| 0 (FP1 round 4, inherited claim) | — | 700 | 112 | `fp1-round4-suite.txt` |
| 0′ (FP2 re-measure, same tree) | — | **698** | **114** | `fp2-round0-suite.txt` |
| 1 | B1 packet lands (factory, modes, migrated registry, self-closing fixture, 70 call sites) | 688 | 124 | `fp2-round1-suite.txt` |
| 2 | publication lanes + store preparation authority + the three other TestApp registries | 721 | 91 | `fp2-round2-suite.txt` |
| 3 | the same lane/wire-page/preparation findings applied to `🧬️mutation-fixtures-dummy` and `-transaction` | 722 | 90 | `fp2-round3-suite.txt` |
| 4 | a settling `dispatch_settled` helper for the six transaction laws | 727 | 85 | `fp2-round4-suite.txt` |
| 5 | `defaultProofs` fixture join + the three `test_retained_*` helper instantiations | **730** | **82** | `fp2-round5-suite.txt` |

**Net: 698 → 730 passed, 114 → 82 failed. 34 laws that were red now execute and hold; 2 went red
(§5). The B1 root is gone: `interactive-job.missing-factory` went 50 → 3, `interactive-job.dispatch`
8 → 0, `interactive-job.catalog-authority` 1 → 1.**

## 2. The B1 packet (FP1 §4.1) — what it actually needed

FP1's §4.1 specified (a) an app-owned bounded factory, (b) `TOOL_JOB_IDS` + fixture JSON, (c) a
migrated registry beside `contract_registry()`, (d) a self-closing instance-bound fixture newtype for
the 73 bare constructions, (e) settle-based assertions. All of that was necessary; four things it did
not predict were also necessary, and each of them is a measured finding.

### 2.1 Three app modes, not two (`TestApp<const RETAINED: bool, const TOOLS: u8>`)

`validate_tool_job_rows` (`🔌️plugin/🦀️.rs:13041`) requires `seen == expected` where
`expected = <A::Command as OpBinary>::TOOL_JOB_IDS ∩ migrated_ids(live registry)` and `seen` is the
set of `bounded_first_step_tool_proofs()` rows — a **static** per-type answer joined against a
**per-instance** registry. One app type therefore cannot satisfy both a registry that declares
`compositeEdit`/`applyCountFromTask` `Migrated` (needs rows) and a registry-less wrapper (needs none).
`TestApp` gained a second const parameter, defaulted so that every existing `TestApp` /
`TestApp<false>` spelling still means the full fixture:

| mode | proofs | factories | who uses it |
|---|---|---|---|
| `TEST_APP_TOOLS_NONE` (0) | none | none | `unproved_command_fails_before_an_overrun_reducer_can_start`, `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations`, `ui_dispatch_backstop_rejects_every_non_migrated_action_and_command` — the three laws `contract_registry()`'s `BatchOnlyPendingRewrite` exists for |
| `TEST_APP_TOOLS_FACTORIES` (1) | none | all verbs | `registry_less_construction_rejects_before_the_reducer` — it must reach `interactive-job.unknown-key`, i.e. past the proof gate and into the registry |
| `TEST_APP_TOOLS_FULL` (2, default) | `TOOL_JOB_IDS` rows | all verbs | every behavioural law |

### 2.2 The owned factory is deliberately NOT `retained_command::ArtifactRetainedCommandJob`

`🧪️tests/⏳️completion/🦀️.rs` gains `TestAppCommandFactory<RETAINED, TOOLS>` +
`TestAppCommandJob<RETAINED, TOOLS>` — the same minimal shape as the neighbouring `TestRestartJob`
(command + five `Arc` roots + completion + one wire-page owner). The generic
`ArtifactRetainedCommandJob` inlines an `Emit`, an `EphemeralEmit`, a 512-byte checkpoint buffer and
two wire owners; see §3.1 for why that mattered.

`TestApp::handle` and the owned route now run **one** reducer body: `handle`'s match was extracted
into a free `test_app_reduce(command, doc, cfg)` (and `command_id`'s into `test_command_id`), so a
migrated dispatch and a direct one cannot diverge. That extraction needed
`AsyncTask::{new,keyed,restartable}` to stop being `async` — they are `#[cfg(test)]`, non-suspending,
and had exactly one production-shaped caller (`project-semio-async-convention-debt`: fix the callee).

### 2.3 Publication lanes are a three-way declaration, and `HostOnly` is not a wildcard

`publish_typed_operation` (`🔌️plugin/🦀️.rs:27519-27529`) refuses any emit that writes a store lane its
factory did not declare. Declaring everything `HostOnly` (which is what
`🧬️mutation-fixtures-dummy`/`-transaction` do today, per FP1 §3 item 9) produced **40 new reds**
in round 1 — `typed-operation emitted a store lane absent from its exact factory publication
contract`. The lane table is now per verb (`Artifact` for the nine document verbs, `Config` for
`select`, `Artifact + Child` for `compositeEdit`, `HostOnly` for the eight effect/event-only ones).

An `Artifact`/`Config` lane is only admissible while the app installs the matching
`build_*_store_one_item_preparation_factory` — otherwise app construction quietly files the tool
under `unsupported_publication_contracts` and dispatch answers
`interactive-job.publication-authority-missing`. `TestApp` had **neither** for the non-retained mode,
so both were added (`bounded_config_store_one_item_preparation_factory` is generic over the
snapshot/mutation pair and serves the document store too).

> This is a live product finding for the two testkit fixtures: `DummyFixtureFactory` and the
> transaction fixture's factory declare `HostOnly` while their jobs emit `artifact_mutations`. That
> was the root of 8 `interactive-job.dispatch` reds; round 3 applied the same three fixes to both, and that bucket is now 0.

### 2.4 Four registries, not one

Every registry paired with a `TOOLS_FULL` `TestApp` must declare the verbs the laws dispatch AND mark
the two `TOOL_JOB_IDS` verbs `Migrated`. A shared `declare_test_app_verbs(builder)` now feeds
`migrated_contract_registry()`, `interaction_app_definition()`, `flat_menu_registry()`,
`synthetic_play_app()` and `🔬️surface-view-state-routing`'s `surface_routing_registry()`; missing it
is `interactive-job.catalog-authority` at **construction** (15 reds in round 1).

`declare_test_app_verbs` is a **sync** fn driving `AppBuilder` through `resolve_ready` per statement —
see §3.1.

### 2.5 The self-closing fixture, and the 70 call sites

`ContractApp` / `ContractComposedApp` wrap `VcsArtifactApp<TestApp[, TestMembers]>` with `Deref` +
`DerefMut` + a `Drop` that runs `close_registered_fixture_app` unless
`std::thread::panicking()` or the app is already terminal-empty (idempotent, so the laws that close
explicitly still work). An inherent `dispatch_typed` shadows the `Deref` target's through the
inherent-before-`Deref` rule and **settles** the operation before returning, folding the settle
receipt's effects/events/ui-scope back into the returned `InvocationResult`; `mutations` stays empty
by construction (mounted app), which is the remaining assertion work in §4.

The 70 bare `VcsArtifactApp::<TestApp[, TestMembers]>::new(TestApp::<false>::default()).await`
constructions were rewritten by script with a **region guard** (the file must start with
`mod plugin_builder_contract_tests {`, the whole file being one `#[cfg(test)]` module) and a
**diffstat ceiling**: the scripted pass was diffed against a pre-edit snapshot and produced exactly
142 changed lines (70 replacements + hunk headers), zero lines outside `let … = …;` constructor
statements. 21 of them were then moved to non-self-closing `*_raw()` twins: a generic bound
(`PluginApp`) never applies a `Deref` coercion, and a law that moves its app into a `PluginRuntime`
cell or a `dyn_enum_close!` arm must keep owning the close.

## 3. Product/infrastructure defects found and fixed along the way

### 3.1 A panic inside the 2 MiB bounded-stack law is reported as a stack overflow

`one_framework_reserved_route_fits_a_bounded_thread_stack` spawns a 2 MiB thread and was already
measured at ~1.75 MiB for construct-select-close. Round 1 aborted the WHOLE test binary with
`fatal runtime error: stack overflow` — which masked every other result. Bisected with a temporary
three-phase probe (reverted):

1. Twelve extra `.await`s in the `AppBuilder` chain (`declare_test_app_verbs`) overflowed it outright —
   an unoptimized build gives every await in one generator its own non-overlapping slot, which is the
   exact defect the law's own docstring records. Cured by making the fn synchronous
   (`resolve_ready` per statement, so each temporary dies at its own semicolon).
2. Nesting the fixture constructor one level deeper (`interaction_app_under_test` →
   `contract_app_with` → `with_registry`) inlined `with_registry_on_bus`'s enormous future into the
   caller's. Cured by `Box::pin`ning the `with_registry` future in all five fixture constructors.
3. With those fixed, a plain `catalog-incomplete` **panic** inside that bounded thread still surfaces
   as `has overflowed its stack` (the panic machinery needs stack the big live frame has already
   taken). Anyone debugging this law should assume a masked panic first.

### 3.2 `A::ephemeral` is never called on the typed path

Repo-wide there is no call to `ArtifactApp::ephemeral` outside the `ArtifactEditor`/`ArtifactViewer`
adapters — the mounted route expects the owned job to hand its `EphemeralEmit` to
`ArtifactToolCompletion::complete`. `TestAppCommandJob` currently passes `EphemeralEmit::default()`,
which is why `a_command_reaches_both_ephemeral_lanes_without_touching_history` is still red (§4).
The job has no presence root to compute it from: neither `ArtifactOwnedToolJobRequest` nor
`ArtifactOwnedToolJobContext` carries `Presence`.

## 4. The 82 still red (round 5)

| count | bucket | note |
|---|---|---|
| 17 | bare `assertion left == right` | the settle-receipt rewrite still owed: `result.mutations` / `result.inverse_group` are empty on a mounted app and must be read from the store, the edit log or the settle receipt instead. **This is the single biggest remaining chunk and the direct continuation of FP1 §4.1(e).** |
| 6 | `interactive-job.output-envelope` | clipboard `copy`/`cut`/`paste` route (FP1 counted 4; the two that previously died earlier now reach the same envelope) |
| 4 | `app.message` (child lane) | the four composed-child laws — `compositeEdit`'s child emit needs more than the `Child` lane declaration |
| 4 | fixture close never reaches terminal-empty | FP1 §4.3 verbatim, untouched |
| 3 | `interactive-job.missing-factory` | `editor_fixture_still_mutates_normally` (see below) + two manifest-command routes |
| 2 | `plugin.internal` / 2 `fixture app has no external owner` / 2 maintenance-step | FP1 §4.2 singles |
| 1 | `interactive-job.catalog-authority` | `new_app_constructs_a_registry_less_wrapper` — FP1 §4.4's known stale law, still untouched |
| 41 | one-off per-law reds | each its own question |

`editor_fixture_still_mutates_normally` is the **surface** fixture, and it is a measured finding of
its own: `SurfaceEditorFixture` is an `ArtifactEditor`, and `ArtifactEditor::command_id`'s default
(`🔌️plugin/🦀️.rs:31974`) answers the literal `"typed-command"` for every variant — the same defect
FP1 fixed for `TxnApp`/`DummyApp` (FP1 §3 item 8), one adapter layer down. It also uses the
registry-less `new_app::<EditorApp<SurfaceEditorFixture>>()`, so it needs the whole §2 treatment
(id table, owned factory, lanes, store preparation, registry) — a small packet of its own, not a
one-line fix, and deliberately left rather than half-done.

## 5. Honest gaps, regressions and load-flakiness

**Two laws went from green to red** (measured test-by-test, round 0 vs round 5):

1. `component::reactor::turn::inbound_request_tests::every_inbound_request_row_is_answered_on_the_turn_it_arrives`
   — one of the three load-flaky wall-clock/turn-budget laws FP1 §4.4 records as moving in and out of
   the failure list independently of any edit. It is red in round 0′ *and* round 5 of some runs and
   green in others; nothing in this slice touches the reactor turn budget.
2. `local_interaction_cold_transaction_receipts_and_encoded_route_rejection` — a **real** regression
   and this slice's honest debt. It asserts that an encoded `TransactionPrepare` with no prepared ops
   "must remain explicitly unadmitted" (`plugin.command-route-state-machine-required`); with the
   local-interaction fixture app now registry-backed and migrated, that cold route is admitted and no
   `AppFrame::Error` comes back. Either the law's premise (an app that cannot route) needs its own
   un-migrated fixture, or the cold route genuinely admits something it should not — that is a
   question for the owner of `🕹️interaction/📡️live/📨️dispatch`, and it was not guessed at here.

Everything else:

- **The FP1 baseline did not reproduce**: FP1 reported 700/112, FP2 re-measured the identical tree at
  698/114. The movers are the same load-flaky class. Any future round should re-measure rather than
  inherit a number.
- **The three load-flaky laws were NOT given a deterministic clock/turn basis.** That was in this
  slice's brief and is the one item it did not reach. `tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks`
  is red in every round measured so far (round 5: 6 of 771 appends over 2 ms, worst 10.2 ms) and is a
  wall-clock law on a machine running a whole agent fleet.
- No law was deleted, `#[ignore]`d or loosened. Every ceiling is byte-identical. The one law whose
  budget was genuinely threatened — the 2 MiB bounded thread stack — was fixed by shrinking the
  fixture (§3.1), never by raising the bound, and it passes.
- `TOOL_JOB_IDS` was left at its two ids; only the fixture JSON's `restartAuthority.defaultProofs`
  moved (0 → 2), which is the non-retained mode's new proof-row count. FP1 §4.1(b)'s "extend
  `TOOL_JOB_IDS`" turned out to be unnecessary — the existing pair is exactly `expected` once the
  migrated registry declares both `Migrated`.
- The crate is compile-green at every round (`fp2-check-1.txt` … `fp2-check-5.txt`, final round-5 run
  compiles clean), so the MCP sibling (WR2) and the other dependents were never left red.
- Everything in this report is measured from a captured run; nothing is claimed that was not executed.

## 6. Files changed

Framework product code (1 file):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `AsyncTask::{new,keyed,restartable}` are no
  longer `async` (they are `#[cfg(test)]`, non-suspending, and had one caller).

Framework test fixtures and laws (7 files):
- `…/🔌️plugin/🧪️tests/⏳️completion/🦀️.rs` — `TestAppCommandFactory`/`TestAppCommandJob`, the three
  `TEST_APP_TOOLS_*` modes, the per-verb publication-lane table, a synchronous `#[inline(never)]`
  job builder, `TOOLS`-threaded restart fixture
- `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — `TestApp<RETAINED, TOOLS>`,
  `test_command_id`/`test_app_reduce`, `declare_test_app_verbs`, `migrated_contract_registry`,
  `ContractApp`/`ContractComposedApp` + six fixture constructors (all `Box::pin`ned), the artifact and
  config store one-item preparation factories, 70 rewritten call sites, migrated `synthetic_play_app`
  and `flat_menu_registry`
- `…/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs` — `Artifact` lane, wire-page ownership, artifact
  store preparation factory
- `…/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs` — the same three, plus the settling
  `dispatch_settled` helper used by the six transaction laws
- `…/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-unit-command-close/🦀️.rs` — job literal
- `…/🔌️plugin/🧪️tests/🔬️surface-view-state-routing/🦀️.rs` — migrated registry
- `…/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs` — `query_app` takes the raw fixture

Fixtures (1 file):
- `…/🔌️plugin/🧫️fixtures/⏳️completion/🔣️.json` — `restartAuthority.defaultProofs` 0 → 2

Captures (all in `🗑️generated/`): `fp2-round0-suite.txt`, `fp2-check-1.txt` … `fp2-check-5.txt`,
`fp2-round1-suite.txt`, `fp2-round1-serial.txt`, `fp2-stack-probe.txt`,
`fp2-round2-suite.txt` … `fp2-round5-suite.txt`.

## 7. Next slice — the shortest path to a trustworthy gate

1. The 17 `assertion left == right` laws: rewrite each to read the store / edit log / settle receipt
   instead of `result.mutations` (`ContractApp::dispatch_typed` already settles, so the data is there).
2. The surface fixture packet (§4) — one more `ArtifactEditor`-shaped repeat of §2.
3. `local_interaction_cold_transaction_receipts_and_encoded_route_rejection` (§5 item 2) with the
   owner of the local-interaction dispatch route.
4. The three load-flaky laws: a deterministic turn/clock basis, which is a different cure from the
   allocator-scope one FP1 applied to the two heap laws.
