# 📓️ Lane C — `fault-discriminators`: "window body renders empty" is now diagnosable end to end

Ticket `26/09/05/S-END-TO-END` · lane C (Opus) · spec: `📓️explore-empty-window-fault-taxonomy.md`.

## 0. What was wrong

Every one of the 13 `RUNTIME_MAINTENANCE_FAULT` store sites and the whole `RUNTIME_CLOSE_FAULT`
family collapsed into one opaque wire fault at their single decode site —
`{origin: "plugin", code: "plugin.internal", message: "runtime live cleanup faulted for instance N"}`
— and the React shell then threw `fault.code`/`fault.origin` away at all three catch sites, reading
only `.message`. A stale-ABI trap, an 8 ms interactive-ceiling overrun and a dead clock were
byte-identical from the outside, and a spawned window whose refresh threw just went `null` (empty
body, console line only).

## 1. Rust — one cause taxonomy, two status enums, discriminating codes

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (39 084 → 39 334 lines).

### 1a. New types (replacing the bare `u8` constants at the old `:28719-28730`)

| symbol | line | what |
| --- | --- | --- |
| `enum RuntimeCleanupFault` (19 variants) | `:28730` | the single cause taxonomy shared by BOTH cleanup loops |
| `struct RuntimeCleanupFaultVector { slug, code, detail }` | `:28754` | wire projection of one cause |
| `const RUNTIME_CLEANUP_FAULTS: [RuntimeCleanupFault; 19]` | `:28760` | declaration order = fixture order |
| `const RUNTIME_CLEANUP_FAULT_VECTORS: [_; 19]` | `:28782` | the literal slug/code/detail table |
| `enum RuntimeMaintenanceStatus { Ready, Queued, Running, Fault(RuntimeCleanupFault) }` | `:28818` | replaces `RUNTIME_MAINTENANCE_{READY,QUEUED,RUNNING,FAULT}`; `repr()` keeps `0..2`, faults are `3 + cause` |
| `enum RuntimeCloseStatus { Ready, Queued, Running, Complete, ExternalWait, Fault(_) }` | `:28857` | replaces `RUNTIME_CLOSE_*`; faults are `5 + cause` |
| `fn runtime_cleanup_fault(scope, cause, instance_id, elapsed_us) -> Fault` | `:28902` | the ONE fault constructor for both loops |
| `const RUNTIME_CLEANUP_UNMEASURED_US: u64 = u64::MAX` | `:28914` | the existing clock-failure sentinel, rendered as `unmeasured` instead of printing `18446744073709551615us` |

`Fault{origin, code, severity, message}` wire shape is unchanged — `FaultCode` was already a
free-form `String`, so nothing on the wire or in `decodeFaultFromWire` had to move. The message shape
is now uniform and carries the instance id AND the measured microseconds:

```
runtime {scope} cleanup faulted for instance {id}: {detail} [{variant}] (elapsed {n}us, ceiling 8000us)
```

### 1b. Every store site now names its variant

Live maintenance loop (was 13 sites, all storing `RUNTIME_MAINTENANCE_FAULT`):

| old line | new line | site | variant → code |
| --- | --- | --- | --- |
| `:29654` | `:29840` | `runtime_live_cleanup_nonterminal_status` stall credit exhausted | `zero-progress` → `plugin.internal.zero-progress` |
| `:29666` | `:29867` | clock begin failed | `clock` / `clock-regression` |
| `:29672` | `:29849-29858` (`runtime_live_cleanup_publish_turn`) | **ceiling overrun split from clock error** | `interactive-ceiling` → `plugin.internal.interactive-ceiling`, `clock`/`clock-regression` → `plugin.internal.clock[-regression]` |
| `:29678` | `:29879` | a close is already in flight | `already-closing` |
| `:29684` | `:29885` | rejected admission drained | `admission-rejected` |
| `:29699` | `:29900` | `session.resume()` failed | `resume` |
| `:29713` | `:29914` | prior `StepOutcome::Fault`/`Cancelled` | `prior-outcome` |
| `:29734` | `:29935` | `pump.session` was `None` | `missing-session` |
| `:29736` | `:29937-29939` | **`session.step()` `Err` — captured, no longer `.is_err()`-discarded** | `abi-mismatch` → `plugin.internal.abi-mismatch` |
| `:29738` | `:29940` | `checkout_outcome()` false | `checkout` |
| `:29739` | `:29941` | `checked_out_job_mut()` `None` | `checked-out-job` |
| `:29745` | `:29947` | `take_outcome()` `None` | `take-outcome` |
| `:30308` (wasm32-gated) | `:30559` | `default_now_ms()` `None` in `pump_runtime_live_cooperative_turn` | `clock-cooperative` → `plugin.internal.clock-cooperative` |

The 8 ms branch was one boolean `elapsed.is_err() || elapsed.is_ok_and(interactive_step_contract_violated)`.
It is now `runtime_live_cleanup_publish_turn` (`:29849`), which stores the measured µs into the new
`RuntimeAppCell::maintenance_fault_us: AtomicU64` (`:28690`) alongside the status, so the decode site
can print the actual overrun.

Close loop (`RUNTIME_CLOSE_FAULT` family) — `runtime_close_fault(state, origin: u8)` took an
untyped origin byte that was only ever stored under `#[cfg(test)]`; it now takes a
`RuntimeCleanupFault` and returns a `RuntimeCloseStatus::Fault(cause)` (`:30167`). Sites: ceiling `:30430`, clock `:30393-30401`
(`runtime_callback_clock_begin`/`_elapsed` now return `Result<_, RuntimeCleanupFault>` instead of
`Result<_, u8>`), owner/instance/maintenance poisoning and not-drained `:30143-30158`,
admission-rejected `:30191`, resume `:30214`, missing-session `:30218`/`:30256`, prior-outcome
`:30227`, abi-mismatch `:30261`, checkout `:30264`, checked-out-job `:30265`, zero-progress `:30108`,
take-outcome `:30270`.

### 1c. The two decode sites

- `plugin_step_close_cleanup` `:30531` — `RuntimeCloseStatus::Fault(cause) => Err(runtime_cleanup_fault("close", cause, instance_id, state.last_callback_elapsed_us.load(…)))`
- `plugin_step_live_cleanup` `:30573` — `RuntimeMaintenanceStatus::Fault(cause) => Err(runtime_cleanup_fault("live", cause, cell.id, cell.maintenance_fault_us.load(…)))`

The old catch-all `_ => Err(plugin_internal_fault("runtime live cleanup has an invalid state …"))` arm
is gone: the match is now exhaustive over a real enum, so an unhandled state is a compile error
rather than a runtime string.

## 2. Language-neutral fixture + oracles

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🩺️runtime-fault-vectors.json` — 19 vectors
  (`variant`, `code`, `class`, `detail`), the `messageTemplate`, `ceilingUs: 8000`, the two `scopes`,
  and 7 `classifications` cases (supervisor precedence, bare `plugin.internal`, non-plugin codes,
  absent code).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧯️runtime-fault-vectors.schema.json` — strict
  draft-07 schema, `additionalProperties: false` everywhere, `ceilingUs` and `messageTemplate` as
  `const`, `code` pattern-locked to `^plugin\.internal\.[a-z][a-z-]*[a-z]$`.
  (Same `📐️…schema.json` + `🔬️….json` pair convention as the existing
  `🔬️tool-factory-proof.json` fixture in the same directory, which is likewise `include_str!`-ed
  from `🦀️.rs` and Ajv-validated on the TS side.)
- Rust test: `mod runtime_cleanup_fault_vector_tests` at `🔌️plugin/🦀️.rs:30344`, two cases —
  `every_declared_variant_decodes_to_its_language_neutral_wire_vector` (round-trips every variant
  through BOTH status atoms, asserts origin/severity/code/message against the fixture, plus the
  `unmeasured` rendering) and `non_fault_status_reprs_never_collide_with_a_fault_variant`
  (non-fault reprs round-trip, `cause() == None`, and all 19 codes are distinct).
- TypeScript test: `…/🎯️targets/⚛️react/🩺️window-fault.test.ts`, 7 cases, reads the SAME fixture and
  validates it with **Ajv** (`strict: true, allErrors: true`) as the independent oracle, including 5
  adversarial mutations that must be rejected. No new runtime dependency — `ajv` is already a
  devDependency used by `🚪️opening.test.ts` in the same package and by `📜️script.ts`.

## 3. React `ShellHost` — the code is no longer discarded

- New pure module `…/🧱️elements/🏛️ShellHost/🩺️fault/🟦️.ts`:
  `type WindowFaultClass = "abi-mismatch" | "interactive-ceiling" | "clock" | "plugin-internal" | "install-failed" | "unknown"`,
  `WINDOW_FAULT_ATTRIBUTE = "data-semio-window-fault"`, `classifyWindowFault(code, supervisor)`,
  `windowFaultFromError(error, supervisor) -> { class, code, origin, message }`.
  A `crashed`/`quarantined` supervisor outranks every code (that is the install-failed axis §3(d) of
  the taxonomy).
- Shell store (`…/🧱️elements/🐚️Shell/🟦️.tsx`): `PluginRuntimeState.sessionFault` (`:425`),
  `PluginRuntimeState.instanceFault` (`:429`), `SpawnedWindowState.spawnedWindowFault` (`:448`).
  No new action for the first two paths — `SET_ERROR` (`:641`) and `SET_SPAWNED_WINDOW_UI` (`:648`)
  each gained an optional `fault`, so a banner/blank body and its discriminator move in lockstep and
  cannot drift; `SET_INSTANCE_FAULT` (`:642`) is the one new action, for the background
  `readConflicts` path.
- Catch sites in `…/🏛️ShellHost/🟦️.tsx`:
  - `:3346-3349` session `refreshUi` → classifies, logs `[DEBUG] render failed [<class>] <code>`,
    dispatches `SET_ERROR` with the fault. The **existing fatal banner is kept** and now carries
    `data-semio-window-fault` + `data-semio-window-fault-code` (`:7316`).
  - `:3367-3370` spawned `refreshUi` → `SET_SPAWNED_WINDOW_UI {value: null, fault}`. The spawned
    window branch guard changed from `spawnedWindowUi &&` to `(spawnedWindowUi || spawnedWindowFault) &&`
    (`:7098`) so the window is still built, and its body renders `<WindowFaultStatus/>` instead of
    nothing (`:7124`).
  - `:5261-5267` `readConflicts` → classifies, logs class/code/origin, dispatches
    `SET_INSTANCE_FAULT` (cleared on the next successful read). Session window bodies render the
    status above their UI (`:7158`, `:7204`).
- `WindowFaultStatus` (`:690`, label table `:677`): `role="status" aria-live="polite" data-semio-window-fault={fault.class}`
  plus `-code`/`-origin`, a bilingual title + per-class sentence via `shellLabel`, and the raw
  `code: message` in monospace.
- i18n, EN and DE, no default language: `ui.windowFault.{title,abiMismatch,interactiveCeiling,clock,pluginInternal,installFailed,unknown}`
  added to `UiTranslationSchema` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx:310`) and to
  BOTH bundles in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`
  (`de` `:2658`, `en` `:3484`). Every entry has `normal` and `beginner` copy.

A catalog smoke can now do `document.querySelector("[data-semio-window-fault]")?.getAttribute("data-semio-window-fault")`
and get one of the six classes.

## 4. Two call sites the enum flushed out (silent bugs before this lane)

`RUNTIME_CLOSE_FAULT`/`RUNTIME_CLOSE_COMPLETE` were bare `const u8`, so a `match` arm naming one is a
**binding pattern** when the constant is not in scope, not a comparison. Two files matched on them
across a module boundary:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️.rs:29-33`
  (`PluginInstanceCloseLease::is_retired`) — ported to
  `match RuntimeCloseStatus::from_repr(state.status.load(…))`, and its `Fault` arm now returns the
  DISCRIMINATED `runtime_cleanup_fault("close", cause, …)` instead of the generic
  `plugin_internal_fault("captured app close faulted before terminal ownership")`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/🦀️.rs`
  (the exact-close-lease law suite) — ported to the enums, and the assertions got STRONGER rather
  than merely compiling again: the `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🚨️fault.fixture.json`
  rows now pin WHICH variant each turn publishes, not just "some fault":
  - `:49-53` a `callbacks` row over the 8 000 µs limit must publish `Fault(InteractiveCeiling)`,
    under it the candidate's own variant.
  - `:66` a `terminalPump` `"fault"` row must be `Fault(PriorOutcome)`.
  - `:7-13` + `:80` a new `expected_clock_cause(samples)` helper derives `Clock` vs
    `ClockRegression` vs `InteractiveCeiling` from the fixture's own clock samples, so each
    `clocks` row pins its exact cause.
  - `:98` the non-drained retire preflight must be `Fault(InstanceNotDrained)`.
  That shared fixture and its Ajv schema (`🧯️fault.schema.json`, consumed by
  `🎭️actor/🚪️lifetime/🟦️.ts:109`) were NOT edited — the new discrimination is derived from the rows
  it already carries.

## 5. Commands and real outputs

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e-c \
    cargo check -p semio-framework-plugin --tests --keep-going --message-format=short
EXIT=0
    Finished `dev` profile [unoptimized] target(s) in 5m 33s
```
(620 pre-existing dead-code warnings, zero errors. An earlier `--tests` run, before the two call
sites above were ported, was RED with exactly 17 `E0425`/`E0277` errors, all in
`📨️dispatch/🧪️tests/🦀️.rs`.)

```
$ cd …/🎯️targets/⚛️react && bun ./📜️script.ts test long 🩺️window-fault.test.ts --reporter=verbose
 RUN  v4.1.10 …/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react

 ✓ 🩺️window-fault.test.ts > window fault discriminators > accepts the shared fixture under a strict independent schema oracle and rejects adversarial shapes 474ms
 ✓ 🩺️window-fault.test.ts > window fault discriminators > classifies every declared runtime fault code exactly as the fixture says, for both cleanup scopes 209ms
 ✓ 🩺️window-fault.test.ts > window fault discriminators > resolves the three named discriminators the empty-window taxonomy asked for 40ms
 ✓ 🩺️window-fault.test.ts > window fault discriminators > applies the fixture's supervisor and non-plugin classification cases 115ms
 ✓ 🩺️window-fault.test.ts > window fault discriminators > recovers the code from a nested fault, an Error, and an unstructured rejection 22ms
 ✓ 🩺️window-fault.test.ts > window fault discriminators > keeps the DOM contract a catalog smoke reads wired into the live shell 8ms
 ✓ 🩺️window-fault.test.ts > window fault discriminators > carries a distinct English and German label for every class, with no default language 15ms

 Test Files  1 passed (1)
      Tests  7 passed (7)
   Duration  16.57s
```

```
$ cd …/🎯️targets/⚛️react && bun ./📜️script.ts lint
framework-renderer-react: region/host-contract lint passed
```

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e-c \
    cargo test -p semio-framework-plugin --lib runtime_cleanup_fault_vector_tests -- --nocapture
    Finished `test` profile [unoptimized] target(s) in 19m 45s
     Running unittests 🦀️.rs (target-s-e2e-c/debug/deps/semio_framework_plugin-efc9de6203d0f79c)

running 2 tests
test component::plugin_runtime::runtime_cleanup_fault_vector_tests::every_declared_variant_decodes_to_its_language_neutral_wire_vector ... ok
test component::plugin_runtime::runtime_cleanup_fault_vector_tests::non_fault_status_reprs_never_collide_with_a_fault_variant ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 543 filtered out; finished in 0.01s
```

The ported close-lease law suite (`--lib instance_lifetime_close -- --test-threads=1`, machine load
average 54):

```
running 9 tests
… instance_lifetime_close_construction_failure_preserves_original_live_root ... FAILED
… instance_lifetime_close_constructs_worker_shell_before_exact_live_detachment ... FAILED
… instance_lifetime_close_contended_pump_keeps_exact_outcome_source ... ok
… instance_lifetime_close_does_not_publish_terminal_before_watchdog ... ok
… instance_lifetime_close_fault_outcome_dominates_complete_progress ... ok
… instance_lifetime_close_optional_monotonic_clock_rejects_missing_and_backward_authority ... ok
… instance_lifetime_close_preflight_and_shared_restore_preserve_exact_owner ... ok
… instance_lifetime_close_rejects_foreign_root_and_exhaustion_before_detach ... FAILED
```

**All five tests whose assertions this lane rewrote are green** (watchdog/publish precedence, terminal
pump, clock authority, retire preflight, contended pump) — i.e. the per-variant expectations added in
§4 hold exactly.

The three reds are the ones that drive a REAL close through `plugin_step_close_cleanup` against the
8 000 µs wall-clock ceiling on a box under load 54-305, and this lane's own output is what says so:

```
[DEBUG] exact close Fault { origin: Plugin, code: FaultCode("plugin.internal.interactive-ceiling"),
  severity: Error, message: "runtime close cleanup faulted for instance 7: the turn overran the
  interactive step ceiling [interactive-ceiling] (elapsed 8905us, ceiling 8000us)", … }:
  generation=1 origin=2 elapsed=8905 phases=[0,0,0,6544,8902,0,8905,8905,…] stalled=0
  terminal=false complete=false blocked=false faulted=false pending=Ready detail=
```
and, in the single-test rerun, `… (elapsed 21810us, ceiling 8000us)`.

This is a load-induced environment failure, not a regression: the elapsed measurement
(`runtime_callback_clock_elapsed`) and the verdict (`semio_framework_trace::
interactive_step_contract_violated`) are byte-for-byte unchanged by this lane; only the code and
message the same verdict carries are richer. Before this lane the identical run would have aborted
with `plugin.internal: captured app close faulted before terminal ownership` — no elapsed, no cause,
indistinguishable from a stale-ABI trap. That is precisely the ambiguity the ticket asked to remove,
and the first thing it bought us was a one-line diagnosis of a flake we would otherwise have had to
bisect. (Not independently re-run on a pre-change tree — `git stash`/`checkout` are forbidden here —
so this is a causal argument from unchanged code plus the printed µs, not an A/B measurement.)

The package `typecheck` (`bun ./📜️script.ts typecheck`) reports ~30 errors across the renderer
import graph — every one of them pre-existing and owned by other lanes (`Uint8Array<ArrayBufferLike>`
variance in `📡️replication`, `TutorialUiSnapshot.interactionSelection`, missing
`DirectoryCommandResultSlotV1`/`retainDirectoryCommandResult` in `🏛️ShellHost`, `selectionJson` in
`🛠️ShellHelpers`, a missing puzzle fixture `.schema.json`). **Zero** of them names `windowFault`,
`sessionFault`, `instanceFault`, `spawnedWindowFault`, `WindowFaultStatus` or `🩺️fault`.

## 6. Remaining blockers / notes for the coordinator

- Nothing in this lane is blocked. The kernel `E0432` the brief warned about was gone by the time
  this lane compiled (`cargo check -p semio-framework-plugin --tests` reached `Finished`, EXIT=0).
- The three red `instance_lifetime_close_*` tests above need a quieter machine, not a code fix. If
  they stay red on an idle box, the cause is now printed with its variant and measured µs — start
  there rather than at the plugin.
- The repo-wide `typecheck` for `@semio-tech/framework-renderer-react` is RED for reasons outside
  this lane (list above) — a lane owning `📡️replication`/`🏛️ShellHost` directory bootstrap should
  pick those up.
- The `🩺️runtime-fault-vectors.json` fixture is currently validated by Ajv only from the TS test.
  If the repo wants it in `📜️script.ts`'s self-test roster too (next to
  `toolJobFactoryProofJoinSelfTests`), that is a one-function addition — deliberately not done here
  to avoid touching the shared repo script while other lanes are editing it.
- `data-semio-window-fault` is the DOM contract lane A's `verify catalog` smoke should read; it is
  present on both the per-window status (`role="status"`) and the session fatal banner
  (`role="alert"`), with `data-semio-window-fault-code` / `-origin` alongside.
