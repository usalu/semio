import pathlib, re

path = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️wp-wg8.md")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:90])
    text = text.replace(old, new)


replace(
    "| 1 | B1 — native kernel turn never returns after a real guest boots | **FIXED, law green (measured 05:39)**: `a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub ... ok` — open 14.6 s (debug interpreter), every surface admitted, backbone bind receipt accepted, `addHandleKind` once in 3.6 s. **Undo still red**: native reserved-tool jobs are never stepped (root cause §1.4) |",
    "| 1 | B1 — native kernel turn never returns after a real guest boots | **FIXED, law green (measured 05:39)**: `a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub ... ok` — open 14.6 s (debug interpreter), every surface admitted, backbone bind receipt accepted, `addHandleKind` once in 3.6 s |\n"
    "| 1b | §1.4 — native reserved-tool jobs (undo, redo, selection, clipboard) spawned but never stepped | **FIXED, one mechanism with React + guest (07:3x)**: 4/4 native journey laws green in 5 consecutive runs (edit, undo, redo, select-all + copy + paste; `generated/journey-{17..21}.raw.txt`), new shard law, kernel fixture laws (Rust 7/7, TS) — §1.4 |",
)

replace(
    "Law updated and ready (creates its document through the door, step 4a; reads the shared fixture); runner `wp-wg8/run-collab-live.sh` (§6). Step 11 (undo) stays red until §1.4 lands |",
    "Law updated and ready (creates its document through the door, step 4a; reads the shared fixture, step 11 uses the fixture's undo); runner `wp-wg8/run-collab-live.sh` (§6). Step 11 (undo) is expected green now that §1.4 landed. W2 at 07:40: release 4/34 (`architect`), ~30 min per package |",
)

replace(
    "`a_native_guest_undoes_its_authored_edit_without_a_hub` (red, §1.4) (`🐚️Shell/🧪️tests/🔗️hub-projection-workspace`),",
    "`a_native_guest_undoes_its_authored_edit_without_a_hub` (green since §1.4) (`🐚️Shell/🧪️tests/🔗️hub-projection-workspace`),",
)

start = text.index("### 1.4 Still open — native reserved-tool jobs")
end = text.index("### 1.5 Earlier notes (resolved)")
text = text[:start] + """### 1.4 Native reserved-tool jobs — root cause and fix (landed 07:3x)

Two faults, both measured on the native journey (block2d release, no hub):

1. **Nothing stepped the job** (runs 11–14): the undo page answers `commandComplete` with the admission `Invocation`
   plus `SpawnJob { kind: "framework.reserved.tool" }`. The shard admitted the spawn as a replay seed and started it,
   but steps a job only on a host `Payload::JobStep { turn }` carrying the exact shard-minted `JobTurn`, which the host
   never learned; the renderer sent `JobStep` only for product-replay entries. `JobCompleted` never reached the guest.
2. **The seed closed silently** (runs 15–16, once the host stepped): `ShardLoop::pump: job 71 has no independently
   admitted operation authority (retained shard failure: plugin: ShardLoop::replay: checkpoint page admission refused
   for actor 16384)`. Every spawn's seed checkpoints the whole guest before it starts the job (so a host can replay
   it on another worker); block2d's checkpoint outgrows the fixed checkpoint pages, the seed closed, and the executor
   retained the failure where no host ever read it.

Fix (one mechanism; the guest half is C8's `admit_reserved_spawned_job` + one commit unit per turn, unchanged):

- **Kernel contract** `semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND` (next to `SPAWNED_JOB_STEP_CEILING`), TS twin
  in `🎠️kernel/🟦️.ts`, both asserted from the fixture `🧫️fixtures/🧵️spawned-job-drive` (`reservedKind`). The plugin's
  `app::FRAMEWORK_RESERVED_JOB_KIND` is a re-export; React's `PluginRuntime` uses the TS constant (3 literals gone).
  Pure additions — no ABI, pack-schema or codec-hash change (soft freeze).
- **Shard** (`🔌️plugin/🖥️host/🧵️shard`): `ShardOutcome::Turn { jobs }` reports each admitted spawn's exact `JobTurn`;
  `ShardOutcome::actor()`; a reserved seed is live-only (`MountedReplaySeed.replayable == false`: capture kind and input,
  then start — no guest checkpoint) and `validate_replay_request` refuses it. Law
  `a_framework_reserved_spawn_starts_live_hands_its_turn_to_the_host_and_refuses_replay` (69/69 `shard::`).
- **Renderer kernel thread** (`kernel_runtime`): a fixed `ReservedToolJob` registry (64, like every job table there),
  filled by `admit_reserved_jobs` from each settled turn's reserved spawns and reported turns. `settle_reserved_jobs`
  runs after `exchange` and after `exchange_commands` completes (React: `driveReservedToolJob` serialized after the
  command's ingress): `run_reserved_job_once` grants one `Payload::JobStep` per turn, the step publication yields the next
  turn or ends the job, and the shard's deferred `JobCompleted` turn — an outcome no grant asked for — is counted as owed
  and awaited by `dispatch_turn` (per-outcome grant/owed tally), then settled by the same `settle_turn` rules (typed-page
  ACKs, `MoreWork`). Reserved spawns are host work, never `ExchangeOutcome.effects` (so never a product replay); every
  turn result of one dispatch is applied (a second used to overwrite the first). `ParallelRuntime::take_shard_failure`
  names a retained shard failure in the fault (that is how fault 2 was found).

Measured (runs 17–21, `generated/journey-{17..21}.raw.txt`): undo `Ok`, ledger `addHandleKind` → `false`, 3.8–4.3 s
(debug interpreter; the verb's own exchange 0.43 s, the rest is the shell's refresh exchanges); redo `Ok`, → `true`;
`selectAll`/`copy`/`paste` `Ok`, ledger unchanged (block2d has no clipboard producer, so the framework clipboard routes
answer empty — the fixture declares it). The reserved drive takes 2 steps per job (`Yield`, `Complete`). Two latency
outliers of ~30 s (run 18 undo, run 19 `selectAll`) did not recur in runs 20–21 under `[DEBUG]` timing (no kernel
request > 1.6 s, no outcome wait > 0.5 s, no foreign grant); the machine was running W2's release builds. Watch item.

""" + text[end:]

replace(
    "| 05:42 | live door law on hub 7800 | **1 passed** (`generated/door-live-1.txt`) |",
    """| 05:42 | live door law on hub 7800 | **1 passed** (`generated/door-live-1.txt`) |
| 06:5x | `cargo check -p semio-framework-os-renderer-wgpu -p semio-framework-plugin-host --lib --tests` (§1.4 host drive) | green (only pre-existing warnings) |
| 07:0x | native journey runs 15–16 | undo red: first no step, then fault 2 of §1.4 named by the new retained-failure report |
| 07:1x | `cargo check -p semio-framework -p semio-framework-plugin -p semio-framework-plugin-host -p semio-framework-os-renderer-wgpu --lib --tests` (kernel constant, live-only seed) | green |
| 07:1x | `cargo test -p semio-framework --lib -- spawned_job` + TS `testSpawnedJobDriveContract` (bun) | 7/7 + TS pass (`reservedKind` asserted on both) |
| 07:1x–07:3x | native journey runs 17–21 (edit, undo, redo, select-all/copy/paste) | **4/4 green, 5 runs in a row** |
| 07:3x | `cargo test -p semio-framework-plugin-host --lib -- shard::` | 69/69 (`generated/laws-plugin-host-3.txt`) |
| 07:3x | `cargo check -p semio-framework-os-run -p semio-framework-os-renderer-wgpu --lib --tests` | green |
| 07:3x | renderer `kernel_runtime program_bridge typed_result` laws | parallel: 8 product-replay laws abort on a poisoned process-wide registry (pre-existing: shared registries); serially `kernel_runtime::semantic_document_tests` **34/34** (`laws-renderer-{3,4}.txt`); program-bridge + typed-page laws green |
| 07:3x | React package `typecheck` (PluginRuntime uses the kernel constant) | exit 0 |""",
)

replace(
    "- `🔌️plugin/🖥️host/🧵️shard/🦀️.rs` — `ShardOutcome::Preempted` (+ codec round-trip law row in `🧪️tests/🔬️unit`).",
    "- `🔌️plugin/🖥️host/🧵️shard/🦀️.rs` — `ShardOutcome::Preempted` (+ codec round-trip law row in `🧪️tests/🔬️unit`);\n"
    "  §1.4: `ShardOutcome::Turn { jobs }`, `ShardOutcome::actor()`, live-only reserved seeds (`replayable`), replay refusal;\n"
    "  law `a_framework_reserved_spawn_starts_live_hands_its_turn_to_the_host_and_refuses_replay` in `🧪️tests/🔬️unit`.\n"
    "- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/{🦀️.rs,🟦️.ts}` — `FRAMEWORK_RESERVED_JOB_KIND` (Rust + TS twin);\n"
    "  fixture `🧫️fixtures/🧵️spawned-job-drive/🔣️.json` (`reservedKind`) + both runners in `🧪️tests/🧵️spawned-job-drive`.\n"
    "- `🔌️plugin/🦀️.rs` — `app::FRAMEWORK_RESERVED_JOB_KIND` re-exports the kernel constant.\n"
    "- `📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — the React reserved drive reads the kernel constant.\n"
    "- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs` — `ParallelRuntime::take_shard_failure`.",
)

replace(
    "  `carries_nothing`, `absorb` keeps non-idle ingress, typed-operation pages on `ExchangeOutcome::typed_results` + in-settle\n  ACKs (dead exchange/request/ACK plumbing deleted), `create_app(.., artifact_schema)` + component codec registration;\n  G7w's `[DEBUG] g7w` logging removed.",
    "  `carries_nothing`, `absorb` keeps non-idle ingress, typed-operation pages on `ExchangeOutcome::typed_results` + in-settle\n  ACKs (dead exchange/request/ACK plumbing deleted), `create_app(.., artifact_schema)` + component codec registration;\n  G7w's `[DEBUG] g7w` logging removed; §1.4: `ReservedToolJob`, `settle_turn`, `settle_reserved_jobs`, `admit_reserved_jobs`,\n  `run_reserved_job_once`, `dispatch_turn` (grant/owed tally, every turn result applied), reserved spawns out of `effects`.",
)

replace(
    "- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` — native journey laws (edit, undo), live\n  door law, two-user law creates through the door.",
    "- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` — native journey laws (edit, undo, redo,\n  select-all/copy/paste), live door law, two-user law creates through the door and undoes with the fixture's verb.",
)

replace(
    "- `📺️renderer/🧑‍🎨engine/🧫️fixtures/⏯️native-guest-journey/🔣️.json` (new).",
    "- `📺️renderer/🧑‍🎨engine/🧫️fixtures/⏯️native-guest-journey/🔣️.json` (new; `redo`, `clipboard`, expected ledgers added in §1.4).",
)

replace(
    "- Ticket-local: `wp-wg8/{run-native-journey.sh,run-door-live.sh,block-release.sh,wasm-checks.sh,wasm-checks-2.sh,edit-*.py}`.",
    "- `native-guest-journey-check` (renderer package `📜️script.ts`) runs the four journey laws.\n"
    "- Ticket-local: `wp-wg8/{run-native-journey.sh,run-door-live.sh,block-release.sh,wasm-checks.sh,wasm-checks-2.sh,edit-*.py}`\n"
    "  (`edit-debug-*.py` apply/revert the temporary `[DEBUG] wg8` timing; reverted, 0 lines left).",
)

replace(
    "- `44189`, `73318`, `9716` wasm32 checks (all exited green). `2629`, `3335`, `6859`, `9609` journey runs 11–14 (exited).",
    "- `44189`, `73318`, `9716` wasm32 checks (all exited green). `2629`, `3335`, `6859`, `9609` journey runs 11–14 (exited).\n"
    "- Journey runs 15–21 (06:5x–07:3x, `nohup`, each exited by itself).",
)

replace(
    "2. **Native reserved-tool jobs (§1.4)** want their own slice: the shard must publish a live seed's `JobTurn` and the\n   kernel thread must step it to completion inside the verb's dispatch (React: `driveReservedToolJob`). It blocks\n   undo/redo/checkpoint/copy/paste and every reserved interaction verb (`interactionSelect`, `clearSelection`, …) on\n   native wgpu.",
    "2. **§1.4 landed.** Left open by design: product (non-reserved) spawns of a guest whose checkpoint outgrows the fixed\n   replay checkpoint pages still close their seed (block2d would, gis likely too) — the product-replay owners should\n   size or page that checkpoint; the failure is now named instead of lost. Pre-existing: the renderer's product-replay\n   laws share process-wide registries and abort in parallel (serially green); an unmounted stray copy of the shard unit\n   tests lives at `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs` (compiled by no crate).",
)

replace(
    "   `SEMIO_HUB_LIVE_ORIGIN=http://127.0.0.1:8090`). Expected: steps 1–10 and 12 measurable; 11 red (§1.4).",
    "   `SEMIO_HUB_LIVE_ORIGIN=http://127.0.0.1:8090`). Expected: steps 1–12 measurable, 11 green since §1.4.",
)

replace(
    "  exceeds the 64 MiB execution-target bound); WG8 republishes it from W2's `component-release` once W2's final pass lands it.",
    "  exceeds the 64 MiB execution-target bound); WG8 republishes it from W2's `component-release` once W2's final pass lands it.\n"
    "- 06:4x–07:4x §1.4: shard reports admitted job turns, renderer drives reserved jobs (runs 15–16 red: seed checkpoint\n"
    "  overflow found through the new retained-failure report), live-only reserved seeds + kernel constant, redo and\n"
    "  select-all/copy/paste laws; runs 17–21 all green; 2 latency outliers timed, not recurring.",
)
path.write_text(text)
print("ok")
