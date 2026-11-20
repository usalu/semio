# Debug Probe Inventory

Read-only inventory of temporary `[DEBUG]` probes and debug helpers for the plugin reactor and command debugging work. Nothing was edited.

Base for relative paths below: `C:\semio\🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\` (abbreviated `P\`).

## Totals

- Plugin tree (`P\`): 93 hits
- Value tree (`C:\semio\🧰️framework\🔨️modules\🌱️value`): 2 hits
- Dev tree (`🧑‍💻dev`): 0 hits
- Actor tree: path `🎭️actor` does not exist under `🛍️products\💻️os\🔨️modules\` (the module list has no actor directory). 0 hits.
- TS handle-turn-drive probe: 1 hit
- Total: 96 hits

Pattern notes: `exchange.unapplied` and `exchange.retire-refused` are present (in `command-ingress-turn`). `return_table_summary`, `return_budget_summary`, `last_claim_reason` and `EXCH_TURNS` (except its own definition and uses below) produced no hits. `debug_failed` is defined but is not matched as a call by the `debug_failed\(` pattern when generic (see definitions).

## Definitions (helpers and statics)

| File (relative to P\) | Line | Item | Notes |
|---|---|---|---|
| `⚛️reactor\🔄️turn\🪜️entry-lifecycle-and-retirement\🦀️.rs` | 8 | `fn debug_phase(phase)` | Helper, used only by `poll-kernel-turn` |
| same | 13 | `fn debug_state(point, detail)` | Helper; its body logs `[DEBUG] gate` at line 20 |
| same | 28 | `fn debug_failed<T>(step, result)` | Helper wrapper, used only by `poll-kernel-turn` lines 65 and 67 |
| `⚛️reactor\🪟️surfaces\native.rs` | 80 | `pub fn debug_state(&self)` on surfaces | Used by probe at native.rs:86 and manifest:238, plus tests |
| `⚛️reactor\🪟️surfaces\native.rs` | 86 | `thread_local! LAST` (inside fn) | Static, used only by the probe log |
| `⚛️reactor\📨️pending\🦀️.rs` | 583 | `pub fn debug_state(&self)` on pending | Used by probes and tests |
| `⚛️reactor\🩹️patches\🏃️tracker-drive-and-work-predicates\🦀️.rs` | 111 | `pub fn debug_state(&self)` on tracker | Used by tests (assert messages) and probes |
| `⚛️reactor\🔄️turn\⌨️command-ingress-turn\🦀️.rs` | 297 | `thread_local! EXCH_TURNS` | Static, used only at lines 168 and 180 |
| `⚛️reactor\🏃️runtime\🎞️command\📥️custody\🛂️manifest\🦀️.rs` | 58 | `thread_local! LAST` (advance_step) | Static, used only by line 60 log |
| same | 99 | `fn debug_summary(&self)` on receiving | Used at line 237 (diagnostic string feeding the probe) |
| same | 239 | `thread_local! LAST` (RefCell<String>) | Static, used only by line 241 log |
| same | 247 | `thread_local! LAST` (Cell<bool>) | Static, used only by line 247 gate log |
| `⚛️reactor\🏃️runtime\🎞️command\📥️custody\🛂️manifest\metadata.rs` | 38 | `pub(super) fn debug_phase(&self)` | Used only by `debug_summary` (line 99) |
| same | 41 | `thread_local! LAST` | Static, used only by line 41 log |
| `🪜️maintenance-ladder\🦀️.rs` | 586 | `thread_local! LAST` | Static, used only by the archive-load log |
| `🧪️testing\🧪️artifact-app-laws\...\🦀️.rs` | 323 | `static MAX: AtomicUsize` (inside test fn) | Static, used only by the stall log |

## Probe hits by file

### Plugin tree (`P\`)

#### Test-only diagnostic messages (`debug_state()` inside assert/panic text, not logs)

These are assertion messages in test files. They are not `[DEBUG]` probes, but they call the debug helpers. Keep them unless the helpers are removed.

| File (relative to P\) | Lines | Description | Type |
|---|---|---|---|
| `⚛️reactor\🧪️tests\🧺️turn-patch-batch\🦀️.rs` | 92 | assert msg calls tracker and pending `debug_state()` | call (test) |
| `⚛️reactor\🩹️patches\🧪️tests\🔬️unit\🪤️panic-hooks-and-tracker-fixtures\🦀️.rs` | 115, 187 | panic msg with tracker `debug_state()` | call (test) |
| `...\🔓️reservation-release-and-registry-idle\🦀️.rs` | 34, 46, 80, 115, 116, 149, 150, 154, 198 | assert/panic msgs; line 116 is `let state = tracker.debug_state()` | call (test) |
| `...\🏋️real-size-surfaces-and-producer-shell-denials\🦀️.rs` | 57, 58, 61, 84, 89 | assert msgs; line 61 is `let state = tracker.debug_state()` | call (test) |
| `...\♾️generation-exhaustion-and-abandoned-jobs\🦀️.rs` | 102, 124, 153, 159, 160 | assert msgs with tracker `debug_state()` | call (test) |
| `⚛️reactor\🧪️tests\🚚️more-work-drive\🦀️.rs` | 621 | assert msg with tracker and pending `debug_state()` | call (test) |
| `⚛️reactor\🧪️tests\🔬️reconcile-spin\🦀️.rs` | 72, 176, 223, 226, 227, 298 | assert msgs with `debug_state()` | call (test) |
| `⚛️reactor\🧪️tests\🔬️reconcile-budget\🦀️.rs` | 279, 288 | assert msgs with tracker `debug_state()` | call (test) |

#### Probe logs and call sites in production code

| File (relative to P\) | Line | Description | Type |
|---|---|---|---|
| `⚛️reactor\🪟️surfaces\read.rs` | 64 | `[DEBUG] surface capture taken by reader` log inside `unit.step` | call site |
| same | 115 | `[DEBUG] surface capture released by reader close` log | call site |
| `⚛️reactor\🪟️surfaces\native.rs` | 86 | `[DEBUG] displaced: ...` log guarded by `LAST` static | call site (uses static L86 and fn L80) |
| `⚛️reactor\🔄️turn\🪜️entry-lifecycle-and-retirement\🦀️.rs` | 20 | `[DEBUG] gate {point}` log inside `debug_state` | call site (inside helper) |
| same | 31 | `[DEBUG] turn step {step} failed` log inside `debug_failed` path | call site |
| same | 46 | `[DEBUG] turn entry mismatch` log | call site |
| `⚛️reactor\🔄️turn\🩹️reconcile-drive-and-publication-receipts\🦀️.rs` | 101 | `debug_state("reconcile loop", ...)` | call site |
| `⚛️reactor\🔄️turn\🔦️more-work-sources-and-tracing\🦀️.rs` | 291 | `with_pending_patches(... debug_state())` inside a trace | call site |
| `⚛️reactor\🔄️turn\📨️event-dispatch\🦀️.rs` | 48 | `[DEBUG] lifecycle admit refused` log | call site |
| `⚛️reactor\🔄️turn\🌀️poll-kernel-turn\🦀️.rs` | 32, 34, 38, 40, 42, 45, 49, 51, 53, 56, 64, 66, 68, 72, 74, 76, 79, 81, 84 | `debug_phase("...")` markers (phase labels) | call sites (19 lines) |
| same | 65 | `debug_failed("pending-effects", ...)` wrapping `plugin_drive_pending_effects` | call site |
| same | 67 | `debug_failed("displaced-surfaces", ...)` wrapping `plugin_drive_displaced_surfaces` | call site |
| same | 82 | `debug_state("more-work inputs", ...)` | call site |
| `⚛️reactor\🔄️turn\⌨️command-ingress-turn\🦀️.rs` | 63 | `turn.exchange_generic(...)` call (not a probe; the function has probes) | call site (non-probe) |
| same | 156 | `async fn exchange_generic` definition (not a probe) | definition (non-probe) |
| same | 163 | `[DEBUG] exchange_generic: seq FAILED` log | call site |
| same | 164 | `debug_state("failed-decoded state", ...)` | call site |
| same | 166 | `debug_state("exchange.retire-refused", ...)` | call site |
| same | 168 | `EXCH_TURNS` update + `debug_state("exchange_generic", ...)` | call site (uses EXCH_TURNS) |
| same | 170 | `debug_state("exchange.produced", ...)` and `debug_state("exchange.unapplied", ...)` | call site |
| same | 173 | `[DEBUG] exchange_generic: command seq refused` log | call site |
| same | 180 | `[DEBUG] exchange_generic: input retired, routing the output` log using `EXCH_TURNS` | call site |
| same | 287 | `self.exchange_generic(...)` call | call site (non-probe) |
| same | 297 | `thread_local! EXCH_TURNS` definition | definition |
| `⚛️reactor\📨️pending\🦀️.rs` | 583 | `debug_state` method definition | definition |
| `⚛️reactor\🩹️patches\🏃️tracker-drive-and-work-predicates\🦀️.rs` | 111 | `debug_state` method definition | definition |
| `⚛️reactor\🏃️runtime\🛡️wire-limits\🦀️.rs` | 264 | `[DEBUG] self-wake spin` log | call site |
| `⚛️reactor\🏃️runtime\📦️instances\🔁️cleanup-turns\🦀️.rs` | 83, 275 | `[DEBUG] cleanup job body unreachable` eprintln (two copies) | call site |
| `⚛️reactor\🏃️runtime\🎞️command\📥️custody\🛂️manifest\🦀️.rs` | 54 | non-probe match on `pending()` (matched only due to the word) | ignore |
| same | 58 | `thread_local! LAST` for advance_step | definition (static) |
| same | 60 | `[DEBUG] manifest advance_step: phase ...` log | call site |
| same | 99 | `fn debug_summary(&self)` on receiving | definition |
| same | 237 | `receiving.debug_summary()` used to build the diagnostic line | call site |
| same | 238 | `instance.surface_contexts.debug_state()` inside the diagnostic string | call site |
| same | 239 | `thread_local! LAST` (RefCell<String>) for the pending probe | definition (static) |
| same | 241 | `[DEBUG] manifest pending: {line}` log | call site (the only "pending" probe in a manifest module) |
| same | 247 | `thread_local! LAST` (Cell<bool>) and `[DEBUG] gate manifest.admit` log | definition + call site |
| `...\🛂️manifest\metadata.rs` | 38 | `pub(super) fn debug_phase(&self)` | definition |
| same | 41 | `thread_local! LAST` and `[DEBUG] metadata advance` log | definition + call site |
| `⚛️reactor\🎟️context\🌉️call\🎭️exports\🦀️.rs` | 50 | `[DEBUG] input admission refused` log | call site |
| same | 91 | three logs: `[DEBUG] poll authority failure`, `[DEBUG] poll currency` (uses `entry.debug_currency`), `[DEBUG] poll semantic failure` | call site |
| `🪜️maintenance-ladder\🦀️.rs` | 586 | `[DEBUG] archive load deferred` log with `LAST` static | call site |
| `🧪️testing\🧪️artifact-app-laws\🏗️registered-app-construction-and-typed-operation-settlement\🦀️.rs` | 323 | `[DEBUG] stall5d laws idle=...` eprintln with `MAX` static | call site |
| same | 324 | `[DEBUG] stall5d laws {typed_operation_diagnostic}` eprintln | call site |
| `🪟️window\🎚️config\🧬️preparation\✏️apply\🧪️tests\🦀️.rs` | 264 | `[DEBUG] pagedbg body=...` eprintln (test) | call site (test) |

### Value tree (`C:\semio\🧰️framework\🔨️modules\🌱️value`)

| File | Line | Description | Type |
|---|---|---|---|
| `🎟️control\🧬️contract\🎟️work-action\⚙️operation\🦀️.rs` | 140 | `[DEBUG] no caller loan` eprintln with `Backtrace::force_capture()` | call site |
| `♻️retirement\🦀️.rs` | 80 | `[DEBUG] birth mismatch` eprintln | call site |

### Dev tree (`🧑‍💻dev`)

No hits.

### Actor tree

Directory `🎭️actor` not found under `C:\semio\🧰️framework\🛍️products\💻️os\🔨️modules\`. No hits.

### TS handle-turn-drive probe

| File | Line | Description | Type |
|---|---|---|---|
| `C:\semio\🧰️framework\🔨️modules\📺️renderer\🧑‍🎨engine\🧱️elements\🔌️PluginRuntime\🚚️handle-turn-drive\🟦️.ts` | 390 | `console.warn("[DEBUG] command ingress ...")` sampled every 200 continuations | call site |

## Helpers that become unused once call sites are removed

- `debug_phase` (`entry-lifecycle-and-retirement\🦀️.rs:8`): only used by `poll-kernel-turn` markers. Unused after those are removed.
- `debug_state` helper (`entry-lifecycle-and-retirement\🦀️.rs:13`): used by `poll-kernel-turn:82`, `command-ingress-turn:164/166/168/170`, `reconcile-drive:101`. Unused after those are removed.
- `debug_failed` (`entry-lifecycle-and-retirement\🦀️.rs:28`): used only at `poll-kernel-turn:65` and `:67`. Unused once those call the underlying functions directly.
- `EXCH_TURNS` (`command-ingress-turn\🦀️.rs:297`): used only by the probes at lines 168 and 180. Unused after both are removed.
- `LAST` statics used only by logs: `native.rs:86`, `manifest\🦀️.rs:58`, `manifest\🦀️.rs:239`, `manifest\🦀️.rs:247`, `manifest\metadata.rs:41`, `maintenance-ladder\🦀️.rs:586`, and `artifact-app-laws` `MAX` at its line 323.
- `debug_summary` (`manifest\🦀️.rs:99`) and `metadata.debug_phase` (`metadata.rs:38`): used only to build the line feeding the `manifest pending` probe at line 241. Likely unused after the probe is removed (verify no other caller).
- `surface_contexts.debug_state()` (`native.rs:80`) and `entry.debug_currency(...)` (`exports\🦀️.rs:91`): used only in probe or diagnostic strings. Likely unused after removal (verify `debug_currency` has no other caller).
- Not unused: tracker and pending `debug_state` methods (`tracker-drive...:111`, `pending\🦀️.rs:583`). They are used by many test assert messages and stay alive.

## Unknown or not checked

- The `debug_failed` definition was found via a broader grep, not the call-pattern grep.
- `debug_summary` and `debug_currency` callers beyond the lines above were not checked in full.

## 2026-11-20 WIT poll semantic cause

- `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`, `poll_wit`. The temporary `inspect_err` logs `[DEBUG] FEM WIT poll semantic failure` once per Wasm instance, with the primary message, epoch and related receipt/context refusals before the WIT semantic tag discards the detail. Remove the complete `inspect_err` after diagnosing sequence 7.
