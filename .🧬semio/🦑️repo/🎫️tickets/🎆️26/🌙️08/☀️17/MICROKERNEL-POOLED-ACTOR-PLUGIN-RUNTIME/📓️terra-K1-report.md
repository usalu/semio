# K1-suspend-resume-placement — terra's report

## Summary

`ShardLoop::pump` in `🧵️shard/🦀️component.rs` now dispatches `Payload::Suspend`/`Resume`/`Cancel`
instead of faulting them out, and `Effect::SpawnJob`'s `placement` (previously matched and
discarded) is captured and used to route `Exclusive` jobs to the front of each pump's step order.
Three new `ShardOutcome` variants (`Checkpoint`, `Resumed`, `Cancelled`) carry the results back over
the transport. Four new tests added, all passing. One PRE-EXISTING, out-of-`path_scope` bug was
discovered (not introduced by this packet) and is documented below with a lease-request, not faked
around.

## What changed

All changes are in **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`**
(my primary path_scope file). No other file in path_scope was touched — `🎭️actor/🦀️component.rs` and
`🖥️host/🦀️component.rs` (outside `shard/`) are both unmodified by me. (A `+7` diff you may see on
`🎭️actor/🦀️component.rs` — a `Scheduler::lane_of` helper tagged "T1" — belongs to a concurrent
session's unrelated work; confirmed via `git log --date=iso -3` showing no commit of mine touches
it, and I never opened it for editing.)

Region-by-region:

- **imports**: `JobPlacement` moved from a `#[cfg(test)]`-only import to an unconditional one
  (production code now needs it too); added `GuestInstanceState` as a `#[cfg(test)]` import (needed
  by the suspend/resume round-trip test to inspect `GuestInstance.state` directly — visible because
  `shard::tests` is a descendant module of the host crate root that defines it).
- **`ShardOutcome` enum**: added three struct variants — `Checkpoint { actor: u64, state: Vec<u8> }`
  (Suspend's outcome), `Resumed { actor: u64 }` (Resume's success outcome), `Cancelled { actor: u64
  }` (Cancel's outcome). All struct variants per the ticket's serde-hazard note (no `Option<T>` or
  bare `Vec<u8>` newtype anywhere — see the "discovered bug" section below for why the latter matters).
- **`ShardLoop` struct**: added `job_placement: HashMap<(u64, u64), JobPlacement>`, doc-commented,
  populated at `Effect::SpawnJob` admission and purged everywhere a `running_jobs` entry is removed
  (`unregister`, `Effect::CancelJob`, `step_job` Done/Failed/Err, and the new `Payload::Cancel` arm).
- **`ShardLoop::new`**: initializes the new field.
- **`unregister`**: now also purges `job_placement` for the actor (previously only purged
  `running_jobs`/`pending_completions`).
- **`pump`'s drain loop**: replaced the single catch-all `Fault` arm for
  `Suspend|Resume|Cancel(_)` with three real dispatch arms (see "Payload::Cancel semantics" below for
  the reasoning behind Cancel's behavior).
- **`Effect::SpawnJob` handling**: now destructures `placement` (was `..`) and records it into
  `job_placement` alongside the `running_jobs` insert.
- **`to_step` construction**: added a stable `sort_by_key` that moves any `(actor, job)` pair whose
  `job_placement` entry is `JobPlacement::Exclusive` to the front — see "Placement routing" below.

## `Payload::Cancel`'s semantics — what I read and what I implemented

`Payload::Cancel(u64)` (`🎭️actor/🦀️component.rs`, region `✉️Envelope`) carries **no doc comment of
its own** beyond the enum-level one on `Payload` ("The message body an `Envelope` carries"). I
grepped the whole tree for every construction/match site of `Payload::Cancel` before writing
anything — the only one that existed was `ShardLoop::pump`'s own now-replaced fault arm; nothing
else in the repo (Rust, TS, or JS) ever constructs or interprets this variant, so there was no
second call site to infer intent from.

Reasoning I used instead of guessing at the bare `u64`'s meaning:
- It sits in the enum grouped with `Suspend`/`Resume` (actor lifecycle), not next to `JobStep{job:
  u64}` (single-job control) — and `Effect::CancelJob{job}` already exists as the guest-side,
  per-job cancellation path (dispatched separately, unchanged by this packet).
- `Envelope` already has its own generic `cancel_of: Option<u64>` field ("an optional envelope-seq
  this cancels") for cancelling a specific prior envelope by sequence — a mechanism orthogonal to
  `Payload::Cancel` and not something `Payload::Cancel`'s own `u64` needs to duplicate.
- The packet brief itself describes the required behavior in actor-wide terms: "cancel that actor's
  running jobs (plural) ... and drop/unregister the instance."

I implemented `Payload::Cancel` as **actor-level teardown**: every `(actor, job)` pair in
`running_jobs` for that actor is cancelled via `GuestRuntime::cancel_job`, then the actor's instance
is unregistered via the existing `unregister` method (which now also purges `job_placement`). The
bare `u64` payload is not consumed by any logic (no documented meaning to key behavior off) — it is
matched with `Payload::Cancel(_)`. If a future packet's doc comment reveals a narrower per-job
meaning for that `u64`, the dispatch arm's own comment flags it as the place to revisit.

On success: `ShardOutcome::Cancelled { actor }`. If the actor isn't registered on this shard:
`ShardOutcome::Fault` naming it — matches the file's existing "never let an envelope silently vanish"
convention.

## Suspend / Resume dispatch

- `Payload::Suspend { checkpoint: bool }`: if the actor isn't registered → `Fault`. If `checkpoint ==
  true` → calls `GuestRuntime::checkpoint`, returns `ShardOutcome::Checkpoint { actor, state }` on
  success or `Fault` on a `PluginHostError`. If `checkpoint == false` → `ShardOutcome::Checkpoint {
  actor, state: Vec::new() }` (nothing was asked to be snapshotted, so nothing is called). The
  instance stays registered either way — suspend only means "stop scheduling turns to me", which is
  `Kernel`/`Scheduler` bookkeeping outside this file's `path_scope`; a caller on the other end of the
  transport is expected to turn `Checkpoint` into `Kernel::suspend(actor, Some(state))`.
- `Payload::Resume { checkpoint: Option<Vec<u8>> }`: if not registered → `Fault`. If `checkpoint` is
  `Some(bytes)` → calls `GuestRuntime::restore(instance, &bytes)`, `Resumed { actor }` on success,
  `Fault` on error. If `None` → `Resumed { actor }` directly (nothing to restore).

## Placement routing — what I did vs. what needs a lease

**Done, in-scope, tested**: `job_placement` now records every job's `JobPlacement` at admission
(previously matched with `..` and silently discarded — a real, if quiet, bug this packet closes).
`JobPlacement::Exclusive` entries are moved to the front of `to_step` before the per-pump step loop
runs, via a **stable** sort (`Vec::sort_by_key`, which preserves relative order within each priority
group) — so an `Exclusive` job admitted in the same pump as an `Inline`/`Isolated` one always steps
first, proven by `exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump`.

**Honest limit, documented at the `to_step` sort site and in the `job_placement` field's own doc
comment**: this is priority-within-one-shard's-own-pump-loop, not cross-shard/dedicated-thread
placement. I traced why true "dedicated shard" routing is unreachable from inside `ShardLoop`:

- `ShardLoop` has no reference to `Kernel` or `ShardTable` at all — its fields are `runtime`,
  `transport`, `instances`, `running_jobs`/`job_placement`, `pending_completions`. It does not know
  its own `ShardId`, whether it IS the exclusive-reserved shard, or how to reach a sibling
  `ShardLoop` running on a different thread/process.
- `ShardTable::request_exclusive` (`🎭️actor/🦀️component.rs`, region `🧩️ShardTable`) leases a shard to
  an **`ActorId`**, not to a job — there is no job-level entity in the `Kernel`/`ShardTable` model to
  route via that call at all. A spawned job is purely `ShardLoop`-local bookkeeping
  (`(u64, u64)` pairs), never an `ActorId` the `Kernel` can pin/lease a shard to.
- Genuinely honouring `Isolated`/`Exclusive` (a job running in its OWN `GuestInstance`, not the
  spawning actor's) would need `ShardLoop` to call `GuestRuntime::instantiate`, which needs a
  `CompiledHandle`/capabilities/budget `ShardLoop` never stores per-actor today (only already-built
  `GuestInstance`s via `register`).

**lease-request** (against `🎭️actor/🦀️component.rs` / a new cross-shard mechanism, NOT mine to build
under this packet's `path_scope`): real `Exclusive` placement needs (a) a way for a `ShardLoop` to
learn it is (or isn't) an exclusive-reserved shard, (b) a `Kernel`-mediated call analogous to
`request_exclusive` but for jobs, and (c) an envelope/transport path to forward a `SpawnJob`-derived
job to a DIFFERENT shard's `ShardLoop` when this one isn't the exclusive one. None of that exists
yet; I did not fake it. The in-shard priority ordering above is the honest subset achievable today.

## A pre-existing bug this packet's fix unmasked (out of `path_scope`, not touched)

Fixing my own code required first fixing a **compile error** already present in this same test file
(unrelated to me — a leftover from an earlier packet that changed `JobStep::Running` from a newtype
`Running(Option<Vec<u8>>)` to a struct variant `Running { progress: Option<Vec<u8>> }` but never
updated this file's two old-style `matches!` patterns at what were originally lines 446-447). I fixed
those two `assert!(matches!(...))` lines (now struct-variant patterns) since they're inside my own
primary path_scope file and were blocking `cargo check --all-targets` entirely — see
`terra-K1-check1.txt` for the original `E0164` errors.

Fixing that compile error unblocked test execution for the FIRST time in a while, which surfaced a
**second, independent, pre-existing runtime bug**: `JobStep::Done(Vec<u8>)` and `JobStep::Failed
(Vec<u8>)` (both defined in `🖥️host/🦀️component.rs`, NOT my path_scope) are newtype variants under
`#[serde(tag = "kind")]` internal tagging — the EXACT same class of hazard the `Running` variant's own
doc comment already names for `Option<T>` ("cannot serialize tagged newtype variant ... containing an
optional"), except serde also rejects a bare `Vec<u8>` newtype the same way ("cannot serialize tagged
newtype variant JobStep::Done containing a sequence"). This panics `send_outcome`'s
`serde_json::to_vec` any time a job actually reaches `Done`/`Failed` and `ShardLoop::pump` tries to
send that outcome over the transport.

This is what makes the pre-existing test
`spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor`
(J1's own headline test, untouched by me except the `Running` pattern-match syntax fix above) FAIL —
see `terra-K1-test2.txt`. I did not introduce this: the test's own logic and script (idle turn +
three job steps ending in `Done`) is unchanged from before I started; the bug was simply unreachable
until the unrelated compile error was fixed.

**lease-request**: fixing this properly means converting `JobStep::Done`/`Failed` to struct variants
(matching `Running`'s own precedent, e.g. `Done { result: Vec<u8> }`), but `JobStep` is defined in
`🖥️host/🦀️component.rs` and consumed by construction/match sites in THREE other files entirely
outside my `path_scope`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (reactor↔job
conversion) and `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale/🦀️component.rs` (fixture conversion),
plus several more call sites inside `🖥️host/🦀️component.rs` itself. I did not touch any of them. My
OWN new `exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump` test avoids
this bug entirely by scripting `Running { progress: None }` instead of `Done` (documented inline at
the call site) since it only needs to prove step ORDER, not completion — so this bug does not block
any of my four new tests.

## Tests added (all in `🧵️shard/🦀️component.rs`'s existing `#[cfg(test)] mod tests`, region
`🔖️K1SuspendResumePlacement`)

1. `suspend_with_checkpoint_true_surfaces_checkpoint_bytes_in_the_outcome`
2. `suspend_then_resume_round_trips_byte_identical_checkpoint_state` — the bench-budget-#7 "identical
   state hash" property; verified by reaching into the restored `GuestInstance`'s own mock state
   (not just trusting the `Resumed` outcome, since `MockGuestRuntime::restore` returns `Ok(())` for
   any bytes and wouldn't itself catch a mismatch).
3. `cancel_unregisters_the_instance_and_no_further_step_job_happens`
4. `exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump`

Also fixed (pre-existing compile error, same file, in-scope): the two `matches!` patterns in
`spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor`.

## Commands run (every one, foreground, with exit code)

```
CARGO_TARGET_DIR=".../MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target-k1" cargo check -p semio-framework-plugin-host --all-targets
  -> exit 0   (log: terra-K1-check2.txt; terra-K1-check1.txt is the FIRST attempt, which hit the
     pre-existing E0164 compile error described above, before I fixed it)

CARGO_TARGET_DIR=".../MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target-k1" cargo test -p semio-framework-plugin-host --lib
  -> exit 101 (log: terra-K1-test2.txt) — 72 passed, 1 failed.
     The ONE failure is the pre-existing, out-of-path_scope JobStep::Done/Failed serde bug described
     above (spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor),
     NOT anything introduced by this packet. All four of MY new tests pass. terra-K1-test1.txt is the
     FIRST test attempt (before I reworked the exclusive-placement test to avoid the Done bug), which
     showed 2 failures for the same reason.
```

Real crate name confirmed from `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml`:
`semio-framework-plugin-host` (not `semio-framework-os-plugin-host` as the packet brief guessed).

I did not touch `🎭️actor/🦀️component.rs`, so the actor crate's own `cargo check`/`cargo test`/purity
grep are not applicable to this packet and were not run.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs` (only file
  modified — primary path_scope)

## Files NOT touched, with lease-requests written above

- `🎭️actor/🦀️component.rs` — no change needed; `Payload`/`ShardTable`/`Kernel` all already exposed
  what I needed as read-only/reference material.
- `🖥️host/🦀️component.rs` (outside `shard/`) — no change made; the `JobStep::Done`/`Failed` serde
  fix is a lease-request (see above), not implemented.
- `🔌️plugin/🦀️component.rs`, `🧫️fixtures/🔌️scale/🦀️component.rs` — out of path_scope entirely;
  named only because they'd need to change alongside `JobStep` if that lease is ever picked up.

## Remaining gaps (honest, not faked)

1. `JobPlacement::Exclusive`/`Isolated` still run on the SAME `GuestInstance` that spawned them —
   only the STEP ORDER within one shard's pump differs now for `Exclusive`. True dedicated-instance/
   dedicated-shard placement needs the cross-shard/`Kernel` mechanism described above (lease-request).
2. `JobStep::Done`/`Failed` cannot be sent over the wire at all right now (pre-existing, unmasked by
   this packet's unrelated compile fix, lease-request above) — this affects ANY job that completes
   through `ShardLoop::pump`, not just placement-related ones. It is the single pre-existing test
   failure in `terra-K1-test2.txt`.
3. `Payload::Cancel`'s bare `u64` has no confirmed documented meaning anywhere in the tree; I
   implemented the actor-teardown reading the packet brief itself describes and left the field
   unconsumed rather than invent a narrower meaning. Flagged inline at the dispatch arm for whoever
   writes that doc comment next.
