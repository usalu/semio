# Microsecond Step Budget Repair

## Confirmed Defect

The actual mounted typed-command helper constructs `BatchDriveConfig.step_budget_ms` by dividing the exact admitted `max_step_micros` by1000. Legal positive contracts below1000 therefore receive zero duration. Job StepBudget/StepContext compare a millisecond clock against an absolute millisecond deadline using `>=`; these grants cannot run useful work. Rounding up or rejecting sub-millisecond contracts would change the admitted domain and is not an acceptable repair.

## Proposed Boundary

Use one canonical microsecond deadline and clock throughout the interactive job layer: StepBudget, StepContext, BatchDriveConfig, BatchJobParams, retained worker execution and drive_step. Transfer the exact admitted contract duration without division. Keep actor/backbone millisecond scheduling as a distinct domain with explicit conversions where it feeds interactive work. The clock's microsecond and coarse scheduler readings must share one monotonic epoch. Do not silently reinterpret existing millisecond callbacks or duration literals.

Initial read-only census found48 Rust files referencing budget/session APIs; many are tests. Migrate exact consumers coherently, preserving durations where appropriate and leaving unrelated Presence TTL, wire scheduler and actor budgets in milliseconds. No runtime dependency, generic compatibility alias or second interactive deadline authority is planned.

## Test-First Requirements

Add a strict language-neutral fixture for grants1,499,500,999,1000 and7500 microseconds, exact expiration equality, zero fuel/grant and checked overflow rejection. Independent Node BigInt arithmetic and Ajv validate expected values; native serde-backed fake-time tests execute the real StepContext/retained worker. Add a real registered500-microsecond factory test through dispatch and worker publication, not merely a direct budget constructor. Existing cancellation, ACK and close laws remain unchanged. Only after native evidence may the stale full-census deadline predicate be updated, retaining hostile division/round-up/wrong-clock cases.

## Coordination

CAD4 and real constructor are green. Plugin local-interaction19 is green (the nineteenth case proves cold transaction receipts and encoded route denial, not interactive transaction completion). DAG owns the independent runtime close witness; this executor owns job timing plus mounted command parameter construction. The sole fleet compiler lease remains with this executor.

## Native Checkpoints

- `🧪️microsecond-driver-red-r1-native-2026-08-27.txt`: wrong project name; no Rust execution. Correct task is `@semio-tech/framework-job-rs:test`.
- `🧪️microsecond-driver-red-r2-native-2026-08-27.txt`: actual driver law RED0/1, expired grant entered the job.
- `🧪️microsecond-fixture-source-r1-2026-08-27.txt`: canonical strict fixture/source suite PASS940, before later clock/schema additions.
- `🧪️microsecond-driver-green-r1-native-2026-08-27.txt`: four native laws PASS4/0,16skipped,.105snextest. Real driver zero/expired entry exclusion;9 language-neutral deadline cases; private retained-worker500us/missing/overflow cases; actual native clock and500us worker entry. This is not yet registered plugin500us or Wasm execution evidence.

## Current Source Boundary

Canonical job fields use microseconds; optional real clock readings reject missing host authority. Worker deadline overflow uses its preadmitted fault owner, never an infinite fallback. Trace and job now share one clock installation and epoch; bare-Wasm synthetic ticks were removed. Coarse scheduler milliseconds are an optional projection of that same clock, avoiding bare-Wasm `Instant` panics. Exact caller unit migration is underway; direct bounded contexts use checked deadlines and deny-zero grants when no valid deadline exists.

Plugin lifetime diagnostic rerun `🧪️member-instance-lifetime-close-r2-native-2026-08-27.txt` compiled the migrated dependency graph in3m26s, then failed0/2 without SIGABRT. Recorded outer callbacks took9,147us and38,260us; this does not prove which phase exceeded8ms. DAG owns the phase-attributed diagnosis. His subsequent deterministic terminal-state tests first failed0/2 (`🧪️member-instance-lifetime-terminal-red-r1-native-2026-08-27.txt`), then passed3/0 after owner-retention and watchdog-verdict publication corrections (`🧪️member-instance-lifetime-terminal-green-r1-native-2026-08-27.txt`,452filtered,.02s). Actual application close timing remains open.

The coherent shared-clock job rerun passed5/0,16skipped,.077s (`🧪️microsecond-driver-green-r2-native-2026-08-27.txt`), including fractional browser conversion and job/trace epoch equality. Strict source/schema tests passed958 (`🧪️microsecond-clock-source-r2-2026-08-27.txt`). These are not fresh Wasm or all-app evidence.

Live owned-WASI host now samples real Instant nanoseconds with checked unsigned range; its native helper law and the actual registered500us fake-clock dispatch are compiling in `🧪️microsecond-plugin-registered-host-r1-native-2026-08-27.txt`. Renderer bare startup installation is authored, uncompiled. Raw Flow binding is not complete: its family crate is both a renderer rlib dependency and a wasm-pack cdylib, while the bridge loader also supports direct raw instantiation. A custom import cannot be injected indiscriminately into generated consumers; copied browser package assets must retain their actual clock module dependency. This requires an exact owner-entry binding, not a synthetic fallback.

## Exact Registered And Cold-Close Follow-Ups

- Registered fake500us first reached publication/ACK, then failed because KeyedTestApp had not forwarded the newer exact local Presence retirement factory. The fixture now forwards both exact local Presence/Transient factories; production guards are unchanged. Corrected run `🧪️microsecond-plugin-registered-r2-native-2026-08-27.txt`:1PASS/0FAIL,454filtered,.04s. Real registered dispatcher covers same-target supersession, different-target independence, rebase, document count97, ACK, UTF-8 rejection retirement and close.
- Separate actual-clock500us run `🧪️microsecond-plugin-registered-real-r1-native-2026-08-27.txt`:1PASS/0FAIL,454filtered,.25s. This is distinct from the fake-clock law and not a general latency certificate.
- Combined close5 run `🧪️member-instance-lifetime-close-r3-native-2026-08-27.txt`:5PASS/0FAIL,450filtered,.32s. Original2 alone in a fresh process then failed (`🧪️member-instance-lifetime-close-cold-r4-native-2026-08-27.txt`):155839us and26938us outer callbacks, phase prefixes `[0,0,0,155826,...]` and `[0,0,0,26922,...]`; secondary strict StoreDrop caused SIGABRT. Combined-suite warming must not be treated as cold-close completion. DAG owns the expanded15-phase diagnosis.
- Native UI retirement7 passed7/0,91skipped,.451s (`🧪️member-ui-value-retirement-r1-native-2026-08-27.txt`). Exact mutex-contention law then failed0/1,98skipped,.247s (`🧪️member-ui-value-contention-red-r2-native-2026-08-27.txt`); DAG is repairing this without weakening ownership.

## Clock Authority And Entry Binding

The strict clock fixture now includes same-owner reinstallation, foreign-owner denial, and watchdog7999/8000/8001us boundaries. Reinstallation first failed0/1; the equality boundary separately failed0/1. `🧪️microsecond-trace-authority-green-r1-native-2026-08-27.txt` passed2/0,13skipped,.123s after exact function-authority admission and the shared `>=8000` violation predicate. This is not a full timed Watchdog test.

Bare browser installation now belongs to the existing async package, behind its existing js-sys/wasm-bindgen dependencies and an owned public interface. A single private generated startup caches the Performance receiver/function, samples it afresh, and installs the same authority used by job deadlines and tracing. Exact repeated installation is permitted; a different authority is denied. No extra custom Wasm import namespace or renderer-only duplicate clock remains. `🧪️microsecond-browser-clock-wasm-check-r1-2026-08-27.txt` passed the actual wasm32-unknown-unknown metadata check in7m31s; executing a freshly built consumed Wasm module remains pending.

Flow raw ABI admission now checks that real clock before allocation/send/poll/close. The actual existing Flow binary imports `./flow_core_bg.js`, proving the former default raw instantiate with empty imports was invalid. The default loader now uses its generated initializer; explicit custom loaders retain exact caller imports. The first native-JavaScript loader run failed on the missing generated import, then passed against the existing compiled module. A four-case strict fixture covers preinitialized exports, generated initialization, foreign-import denial, and exact custom import ownership. The package build additionally relocates its one exact generated import to the emitted sibling and preserves the host sibling; its separate emitted-bundle proof is still being iterated. This loader result is not a fresh-clock Wasm result.

Puzzle3D's private zero-valued bare clock was removed. Precompute now uses optional checked shared microsecond deadlines and checks before its first task. Existing standalone brush query duration16ms is preserved and remains a separate interaction-bound obligation; no all-app or whole-precompute bounded claim follows.

The root source verifier now binds the exact mounted helper, admitted microsecond duration, real clock, checked overflow, expired-entry exclusion, and existing decoded/work/output limits. Nine hostile variants reject wrong helpers, division, rounding, coarse clocks, overflow fallback, expired entry, synthetic clock and missing output limit. Source983 was followed by979 after removing four unused custom-import-factory checks when the actual generated Flow binding superseded that design. Full census/runtime completion is not implied.

## Latest Retained Gate Results

- Corrected original-two-only fresh process `🧪️member-instance-lifetime-close-cold-r6-native-2026-08-27.txt` selected exactly2 after25.57s warm compilation. Both raised strict outer-close faults, followed by secondary strict StoreDrop SIGABRT; no passing test footer. Generation1 elapsed8519us had phases `[75,0,0,0,0,87,8515,8516,0,0,0,0,0,0,0]`; the other generation1 elapsed19965us had phases `[0,0,0,19951,19962,0,19964,19964,0,0,0,0,0,0,0]`. Missing inner stamps distinguish this from r5's later-generation traces. DAG owns source-level attribution; no cold-close completion is claimed. Additional heavyweight launches are held for coordinator disk coordination; caches and logs are preserved.
- Invalid-scope r5 completed1PASS/2FAIL,453filtered,.38s,19.79scompile. Its optional-clock law ran alongside the original two, so it is not original-two-only evidence. The two faults were22004us and23483us with the15-phase arrays retained in the full log.

- `🧪️member-ui-value-retirement-green-r3-native-2026-08-27.txt`:8PASS/0FAIL,91skipped,.650s. `🧪️member-ui-value-full-r4-native-2026-08-27.txt`:99PASS/0FAIL,0skipped,.301s. The exact contention repair is included.
- `🧪️member-instance-lifetime-terminal-green-r2-native-2026-08-27.txt`:4PASS/0FAIL,452filtered,.02s, strict8000 predicate and optional missing/backward clock law included. A fresh original-two-only15-phase gate remains required. The first r5 launch misspelled the fourth exclusion and therefore is not valid two-only evidence; it will be repeated with the exact function name.
- `🧪️microsecond-flow-loader-green-r3-2026-08-27.txt`:four strict startup cases plus actual existing Flow catalogue/close and emitted browser-package sibling binding PASS. `🧪️microsecond-consumed-flow-clock-red-r1-2026-08-27.txt`:the separate consumed-clock gate correctly fails the existing old binary because it has no startup clock samples. Parent coordinates a fresh engine/core package build with the independent demonstrator publisher; this executor does not write shared OS-dev plugin outputs.
- `📊️microsecond-full-command-census-2026-08-27.json`:actual canonical full census remains RED:773command rows,350source-admitted,315batch-only,2forbidden,270remaining,12failures,979selftests. The obsolete worker-parameter failure is gone with exact native500us and hostile-source evidence; all other guards remain. CAD's real setContributions command disposition is not recognized by the source action-only classifier and will be repaired in the exact app-command catalog packet, never by restoring a no-op action setter.
