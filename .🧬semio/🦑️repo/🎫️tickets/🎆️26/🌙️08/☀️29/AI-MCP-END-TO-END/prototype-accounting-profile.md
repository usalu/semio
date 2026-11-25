# Prototype Accounting Profile

Native opt-level 3 rows added for the hot budget/clock accounting crates seen in the 5 s `sample` of the MCP server (scratchpad `s.txt`, top-of-stack section; counts are samples).

## Crates and justifying symbols

| Package | Profile symbol (rustc v0 mangling decoded) | Samples |
| --- | --- | --- |
| `semio-framework-job-payload` | `original_return_escrow::OriginalReturnBudgetSnapshot::{remaining,raw_remaining,advance_work_exact}`, plus `checked_sub/checked_add/try_from` instantiated here | 101+59+21, 73+36+36 |
| `semio-framework-job-return-budget` | `OriginalReturnBudget::{snapshot,remaining,visible_origin}`, `OriginalReturnBudgetReference::{get,get_mut}` | 66+28+28, 42+14 |
| `semio-framework-job-context` | `Option<OriginalReturnSourceInput>::as_ref`, `StepContext::{deadline_exceeded,fuel_remaining}` closures, `StepCancelReference::deref` (generics monomorphised in this crate) | 74, 59, 21 |
| `semio-framework-job-clock` | `component::stride::ClockStride::read`, `Cell<ClockStride>::set` | 37, 13 |
| `semio-framework-value-control-physical` | `RetainedCloneProgress::fits`, `Result<_, ValueError>::branch`, `RetainedCloneGrant` map_or | 51, 68, 48 |
| `semio-framework-value-control-contract` | `Result<_, ValueError>::branch` (contract instantiation), `Ord::min` | 61, 19 |

All six are workspace members with stable package names (verified in each `Cargo.toml` and in `🧰️framework/Cargo.toml` members / root `Cargo.toml` path dependencies).

Deliberately not added (small, not hot): `semio_framework_trace` (atomic_load 23), `semio_plugin_interpreter_runtime` (advance_original_call_step 16), `semio_framework_actor_retained_context` (only inclusive counts, no self-time row), `semio_framework_job_recorded_clock`, `semio_plugin_host_poll`.

## Policy

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/📋️constraints/🔣️.json`.
Rows (`optLevel: "3"`, section `native`, alphabetical) added to groups `repository`, `runtime`, `streaming` and `framework`. Framework is included because `semio-os-mcp` is itself a member of the framework workspace and `semio-framework-hash` already sets the precedent. No wasm rows, no limits, no schema change.
Counts after both rounds: repository, runtime, streaming 30 to 48 native, framework 28 to 46. Policy size 57593 bytes (limit 262144), at most 88 package rows per owner (limit 512).

## Tests and fixtures (under `🧪️tests`)

- `📥️consumption/🔣️.json`: native counts and digests regenerated for all 238 owners from the real policy module (only `native`/`digest` lines changed).
- `📥️consumption/🟦️.ts`: owner inventory histogram (`46/51`, `48/4`x6, `48/38`x135, `48/52`), `Cargo.toml` row count 82 to 100, and the native interpreter test renamed and extended so the descriptor and MCP workspaces assert all eighteen accounting crates plus both interpreter crates render at opt-level 3 in prototype and release (independent Ajv, TOML, Bun.TOML, SQLite checks kept).
- Result: `bun test ./📥️consumption/🟦️.ts` with bundled bun: 26 pass, 0 fail.

## Round 2: late phase (s3.txt, 110 s; s2.txt)

On-CPU excludes the parked wait frames (`read`, `__psynch_cvwait`, `__ulock_wait`, `semaphore_wait_trap`): s3 = 1550 samples, s2 = 2173. Crate = last `semio_*` identifier in the v0 mangling (the instantiating crate). Self = top-of-stack samples of that crate's symbols; memmove = `_platform_memmove` samples whose nearest `semio_*` ancestor frame is the crate (approximate; 19%/16% of memmove has no semio ancestor and stays unattributed). Percentages are of on-CPU, s3 / s2.

| Package | Self % | memmove under it % | Sampled symbol |
| --- | --- | --- | --- |
| `semio-plugin-host-wake` | 7.2 / 9.8 | 22.3 / 23.1 | dispatch_operation_step, borrow_key, begin_scope, GuestLedger::usage |
| `semio-plugin-host-poll` | 2.2 / 2.5 | 4.3 / 4.0 | original_session |
| `semio-framework-job-native-turn` | 2.6 / 2.8 | 3.3 / 2.6 | OriginalJobReturnBirth::advance |
| `semio-framework-actor-retained-context` | 1.5 / 2.1 | 2.9 / 2.5 | borrow_context |
| `semio-framework-actor-retained-turn` | 1.2 / 0.9 | 1.5 / 0.8 | retained turn accounting |
| `semio-framework-job-receiving` | 3.1 / 2.0 | 0.3 / 0.5 | component::budget::retained_delta |
| `semio-framework-job-recorded-clock` | 2.4 / 2.0 | 1.0 / 0.8 | recorded clock reads |
| `semio-framework-value-retirement` | 2.1 / 2.2 | - | Result::branch instantiated here |
| `semio-framework-io-base64` | 4.5 / 3.7 | - | `Ord::min` |
| `semio-framework-2d-compute` | 2.3 / 1.8 | - | self time (not in your list, over 1%) |
| `semio-framework-value-source-turn` | 1.9 / 1.7 | - | self time (not in your list, over 1%) |
| `semio-framework-trace` | 1.0 / 0.3 | - | atomic loads (borderline, 1% in s3 only) |

Already listed and still hot: value-control-physical 7.5 / 7.5, value-control-contract 4.6 / 3.7, job-context 3.1 / 3.6 (+6.1 / 5.0 memmove), job-clock (memmove 0.7 / 0.9).
Not added: `semio-framework-async` (5 samples, 0.2%, s2 only, not in s3), `semio-framework-geometry` (0.6 / 0.5), `semio-plugin-host-post-turn-relay` (0.6 / 0.7), `semio-framework-value-allocation-return`, `semio-os-mcp-plugin-runtime`, `semio-framework-job-failure`.
All twelve are members of the framework workspace (checked in `🧰️framework/Cargo.toml`), so framework gets them too. Wasm rows untouched. Results: `bun test ./📥️consumption/🟦️.ts` 26 pass, 0 fail. The memmove under `wake` is a copy cost, not a codegen cost: O3 will not remove it; fixing it needs avoiding the copies (by-value ledger/key structs) in `semio-plugin-host-wake`.

## Unverified

- `⚙️arguments` and `🦀️profile-precedence` (and `⚙️arguments/🎭️roles`) cannot run under plain `bun test`: they fail before any assertion with "Original parent Script process invocation required" (needs the repo test envelope). They assert baseline settings, not package lists; no edits were needed there.
- No cargo build was run; the speedup is unmeasured.

## Effect

This changes the compile profile, and therefore unit identity, of all eighteen listed crates, so every dependent rebuilds once.
