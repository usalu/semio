# 📓️ KN3 — `semio-framework-os-kernel --lib` last two reds

Slice KN3 of ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, 2026-09-22.
Inherits KN2 (`📓️kn2-kernel-lib-zero-red.md` §5): **1116 passed / 2 failed**.

Foreground only, one cargo at a time, `RUST_MIN_STACK=67108864`, `CARGO_INCREMENTAL=0`,
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-kn3` (shared build-dir untouched).
Baseline re-measured before any edit (`🗑️generated/kn3-baseline.txt`): the same two reds, and no
others in the `presence_retirement` / `os_store::component` families.

## 1. Red 1 — `retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open`

**Root: a worker session's own pre-admitted 16 KiB terminal-fault page could not be paid for by a
caller whose close granule is smaller than one page, and the refusal was reported as progress.**

`🧰️framework/🔨️modules/🧵️job/🦀️.rs` — every `WorkerJobSession` reserves ONE 16 KiB payload page at
`try_new` (`preadmitted_static_payload(… JobPayloadStream::Fault, b"job-session.terminal-fault" …)`,
`🦀️.rs:1990`) so a terminal fault can be reported without allocating. `worker_job_close_step` walks
its close ladder in a fixed order and reaches that page on the turn after `begin_close`
(`🦀️.rs:2761` → the `preadmitted_fault` branch, `🦀️.rs:2790`). It released the page through
`RetainedJobPayload::close_step`, whose old guard was

```rust
if maximum_items == 0 || maximum_bytes < JOB_PAYLOAD_PAGE_BYTES {
    return JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 };
}
```

`JOB_PAYLOAD_PAGE_BYTES` is 16 KiB (`🦀️.rs:397`). The law closes its mounted session with
`session.close_step(1, 4096)` — the store's own retirement granule, the one every store ladder
uses. 4096 < 16384, so EVERY turn answered `Pending { released_items: 0, released_bytes: 0 }`:
the "keep going, progress is happening" answer, forever. The cursor never reached
`close_stage == 1`, the job never saw its own `close_step`, `terminal_is_empty()` stayed false, and
the law's 4096-turn loop ran out. This is exactly the `Pending{0,0}`-from-turn-0 shape KN2 measured
and could not localise; it is NOT the retirement-slot leak.

It is a product defect, not a test artefact: the ONLY shipped callers that close a job session
survive because they happen to pass `semio_framework_job::JOB_PAYLOAD_PAGE_BYTES` verbatim
(`🔌️plugin/🦀️.rs:20856`, `:22307`, `:36151`, `🔌️plugin/🖥️host/🦀️.rs:4295`, …). The ones that forward a
host turn's remaining grant (`🔌️plugin/🦀️.rs:17623`, `:19829`, `:19886`, `:19927`, `:35933`, and
`:18473`, which even `min`s the grant DOWN to a page) spin forever the moment that grant is short.

**Fix — accrual instead of outright refusal.** New `charge_payload_page`
(`🧰️framework/🔨️modules/🧵️job/🦀️.rs:399`): a close turn spends at most the byte grant it was
offered, the physical backing stays put until the accrued charge covers the whole page, and the
turn that completes the charge is the turn that frees it. Applied at
`RetainedJobPayload::close_step` (`🦀️.rs:625`) and to the writer's staged and rejected pages
(`RetainedJobPayloadWriter::close_step`, `🦀️.rs:766` / `:774`).

Invariants preserved: a caller offering a whole page is byte-identical to before (one turn, one
page, `released_bytes == JOB_PAYLOAD_PAGE_BYTES`); the 16 KiB backing is still released whole or
not at all and still costs exactly one page of grant however many turns pay it; no turn ever
reports more than its own `maximum_bytes`. Gained: a bounded close now completes under ANY positive
grant instead of never.

**Observability — the named close phase.** `close_step` reports HOW MUCH a turn released, never
WHERE the cursor is, so a parked close and a slow one are the same `Pending { 0, 0 }`. Added
`WorkerJobClosePhase` (`🦀️.rs:2080`) with `WorkerJobSession::close_phase()` (`🦀️.rs:3034`) and
`MountedWorkerJobSession::close_phase()` (`🦀️.rs:2263`) — a lock-free read of the same fields the
next `close_step` would walk, in the same order, kept as product API (no logging, no `[DEBUG]`).
This is what localised the stall: the cursor sat in `PreadmittedFault` from turn 1 onwards.

**Law** (`🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️retained-ownership/🦀️.rs:76`):
`mounted_close_on_a_short_byte_grant_walks_every_named_phase_to_terminal` — closes a mounted
session on a 4 KiB grant and pins (a) it reaches terminal, (b) no turn spends more than its grant,
(c) the fault page costs exactly `JOB_PAYLOAD_PAGE_BYTES` in total, (d) the close is bounded at
`JOB_PAYLOAD_PAGE_BYTES / grant + 6` turns, (e) the exact phase ladder
`BeginClose → PreadmittedFault → Job → ParamsRelease → RetirementSlot → Empty`, never revisited.

## 2. Red 2 — `snapshot_read_double_return_is_counted_once_and_reclaimed_once`

**Exactly-once was already restored by the peer; what remained was a stale law.**

Re-read `🏪️store/🦀️.rs` before touching it. The peer's fast path has moved on since KN2 wrote its
§5: `SnapshotReadLease::return_now` (`🏪️store/🦀️.rs:314`) now short-circuits on the shared
`returned` flag and STORES it on the aliased path, so a second `return_now` on the same lease
answers `false`. I verified this at runtime rather than by reading: in `kn3-baseline.txt` the law
failed at `🧪️tests/🔬️unit/🦀️.rs:1284` (`registry.returned` was 0, not 1), i.e. AFTER both
`assert!(lease.return_now())` and `assert!(!lease.return_now())` had passed. No product change was
needed and none was made — the peer's optimisation is untouched.

What was stale is the law itself: written for the pump path only, it asserted that a returned lease
is always *counted* into `registry.returned` and later reclaimed by `try_take_one_returned`. With
the fast path there are now TWO return paths, chosen by whether the root is still aliased outside
the registry, and an aliased return is reclaimed on the returning thread with nothing left for the
pump.

**Law re-expressed, wider not weaker** (`🏪️store/🧪️tests/🔬️unit/🦀️.rs:1276`): it now drives BOTH
paths in one registry and demands exactly-once on each —
*aliased root*: `return_now` true then false, `registry.returned` stays 0, the slot's own owner
alias is dropped in the same call (`Arc::strong_count == 1`), and the pump finds nothing;
*sole-owner root*: `return_now` true then false, counted once, reclaimed once by the pump, and a
reclaimed generation is never handed out a second time.

## 3. Adjacent pre-existing reds in `semio-framework-job`

`cargo test -p semio-framework-job --lib` did not run to completion at HEAD: it SIGABRTed.
`worker_authority_keeps_one_heap_identity_through_mounted_submit_and_checkout` asserted
`pump_one(&pool, Lane::Interactive) == Submitted`, but on native `pump_one` routes the interactive
lane through `try_step_on_caller` and answers `Outcome`/`Terminal` in one pump (`🦀️.rs:2107`). The
failing test then leaked a fixed session slot, so
`worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned` died inside a
destructor and aborted the process. Untouched by my diff (`git diff HEAD` shows no change to
`pump_one`), so red at HEAD, but it hid every other result, so I re-expressed it: the pooled
submit/checkout half now runs on `Lane::Background` and a new third leg pins the interactive
caller-step path, both asserting the same heap authority identity.

## 4. Run ladder

| run | command | result | capture |
|---|---|---|---|
| baseline | kernel `--lib`, filtered to the two families | 10 passed / **2 failed** (the two reds) | `kn3-baseline.txt` |
| job 4 / job 6 | `cargo test -p semio-framework-job --lib --no-fail-fast -- --test-threads=1` | **28 passed / 0 failed** (job 6 re-run after the kernel run) | `kn3-job-4.txt`, `kn3-job-6.txt` |
| job 5 | `cargo test -p semio-framework-job --lib` (parallel) | 27 / 1 — pre-existing global-counter race, see §6 | `kn3-job-5.txt` |
| kernel | `cargo test -p semio-framework-os-kernel --lib --no-fail-fast -- --test-threads=1` | **1118 passed / 0 failed** | `kn3-kernel.txt` |

**Final: `semio-framework-os-kernel --lib` reads 1118 passed / 0 failed** (KN2 left it at 1116 / 2;
the two new passes are the two reds, and the count rises by two because nothing was removed).
`semio-framework-job --lib` reads 28 passed / 0 failed serially — one of those 28 is the new law.

Build note: three earlier attempts at this run were lost to the shared build dir. One cargo sat
25 min at 0 % CPU with no rustc child and `sample` on it hit
`cargo::core::compiler::prebuild_lock_exclusive → flock` (rule 27(b)), so it was killed by pid and
rerun; the coordinator killed a later one (pid 52693) parked 22 min behind a peer fleet. The run
above completed in 19.89 s of test time once the units were free.

## 5. Files changed

Product (`semio-framework-job`, `🧰️framework/🔨️modules/🧵️job/🦀️.rs`):

| line | change |
|---|---|
| `:399` `charge_payload_page` (new) | pays a close turn's byte grant into the fixed 16 KiB page granule; `Ok(paid)` = page paid, release it, `Err(paid)` = grant spent, backing retained |
| `:589` `RetainedJobPayload.charged` (new field) | bytes already paid toward the head page |
| `:625` `RetainedJobPayload::close_step` | accrues the grant instead of refusing every sub-page one outright |
| `:745` `RetainedJobPayloadWriter.charged` (new field) | same, for the staged and rejected page |
| `:766` / `:774` `RetainedJobPayloadWriter::close_step` | same accrual on the staged and rejected arms |
| `:2080` `WorkerJobClosePhase` (new) | the twelve named phases of the close cursor |
| `:2263` `MountedWorkerJobSession::close_phase` (new) | a retained checked-out outcome outranks the inner session |
| `:3034` `WorkerJobSession::close_phase` (new) | lock-free read of the exact ladder `close_step` walks |

No product change in `🏪️store/🦀️.rs`: the peer's `try_release_aliased` / `return_now` fast path is
untouched and already exactly-once.

Laws re-expressed (none weakened; two strengthened):
- `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️retained-ownership/🦀️.rs` — new
  `mounted_close_on_a_short_byte_grant_walks_every_named_phase_to_terminal`; the two
  `…physical_close…` laws now pin the accrual (and a zero-byte grant buying nothing);
  `worker_authority_keeps_one_heap_identity_through_mounted_submit_and_checkout` split into a
  pooled `Lane::Background` leg and an interactive caller-step leg (§3)
- `🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close/🧫️fixtures/🔣️.json` + `🧬️schema/🔣️.json` —
  `refusedBytes` 0 → 16383, `releasedBytes` 16384 → 1, new `chargedTotal: 16384` (the physical page
  still costs exactly one page of grant, now provably across turns)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs` —
  `snapshot_read_double_return_is_counted_once_and_reclaimed_once` now drives both return paths

Three stale `[DEBUG]` eprintlns were removed from the laws rewritten above (rule 10); none was added.

Captures (`🗑️generated/`): `kn3-baseline.txt`, `kn3-job-1.txt` … `kn3-job-5.txt`,
`kn3-job-6.txt`, `kn3-kernel-build.txt`, `kn3-kernel.txt`.

## 6. Gaps — honest

1. **`cargo test -p semio-framework-job --lib` needs `--test-threads=1`.** In parallel,
   `retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned` fails: it snapshots
   the process-global `JOB_PAYLOAD_PROCESS_OWNED_BYTES` and demands it return to that value, which
   any concurrent test that admits a session or a page moves. Measured pre-existing: the same
   parallel run with my new law skipped is also 26 passed / 1 failed. I did not weaken that law.
2. **Native only.** No wasm32 build, no `activate`, no serve, no browser. Every claim here is a
   native law run in-crate.
3. `🏪️store/🦀️.rs` is edited by peers continuously; it was re-read immediately before the one edit
   made to it (a law, not product code).
