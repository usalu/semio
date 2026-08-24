# P2a1 Universal Retained-job Ownership Partial Implementation and Blockers — 2026-08-24

## Status

**RED partial implementation; not source-audit-ready and not an acceptance claim.** The universal
job component now contains the retained payload, fixed child registry, and single-opportunity worker
session foundations described below. The required mounted caller migration and universal producer /
consumer conversion are not complete. No P2a or Phase 2 acceptance is claimed.

## Source Changed

- `🧰️framework/🔨️modules/🧵️job/🦀️component.rs`

No Puzzle 3D P4e source and no P1q kernel/channel/storage region was edited.

## Implemented Foundation

- Non-`Clone` 16 KiB page sources, retained payload writers, exact rejected-source handback,
  operation item/byte counters, per-stream counters, a process byte counter, and one-page close.
- `Checkpoint`, `CommitCandidate`, `JobFault`, preview payloads, and payload-bearing progress events
  now transfer `RetainedJobPayload` rather than cloneable `Vec<u8>`.
- Preview and step sequence advances use checked exhaustion.
- `JobScope` now uses 64 fixed generation-qualified child slots. Admission is fallible, `u64::MAX`
  permanently exhausts a slot, stale/duplicate completion is typed, and release builds reject parent
  completion while a child remains live.
- The public production definitions of `run_to_completion`, `run_on_worker`, and
  `run_on_worker_async` were removed. `BatchJobSession` advances exactly one externally requested
  opportunity.
- `WorkerJobSession` now transfers one exact job authority into one pool closure; pool rejection
  recovers the returned closure and publishes an exact rejected owner. Its public vocabulary covers
  typed contention, ticketed take, rejected take, terminal take, Drop handback, resume, incremental
  close, terminal-empty, and quiet wake registration/recheck.
- Focused hostile fixtures were added for page max/+1 pointer identity, zero close grant, separate
  state/output close, child max/+1/stale/duplicate/exhaustion, overlapping submission, exact terminal
  Drop handback, pool-shutdown rejection, panic, quiet wake, and batch one-opportunity behavior.

## Exact Blocking Boundaries

### 1. Mounted session lifetime has no universal host registry

The native renderer I/O path at
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
currently calls the deleted `run_on_worker` and then discards its returned delivery authority. The
native clipboard path at
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs`
currently depends on `run_to_completion` inside a detached worker callback. A correct replacement
cannot be an inline loop or self-requeue. A mounted renderer-owned registry/pump must retain each
generation-qualified session, request one worker opportunity, adopt one outcome, invoke the exact
completion owner, and advance at most one close unit per host opportunity. That host authority does
not exist yet.

`NativeIoCompletion` is an independent completion owner. Merely submitting the new session once and
dropping its handle would deliver the side-channel result for a one-step job while abandoning the
job/outcome/close authority. It would therefore disguise rather than repair the defect.

### 2. Universal payload conversion exposes production `Vec<u8>` producers and consumers

Approximately two dozen non-Puzzle Rust files construct or consume universal byte outcomes. The
important mounted families include action-bus erased jobs, native/WGPU clipboard, prepared render,
frame build, native I/O, plugin guest relay/effects, reserved tool jobs, store decode authorities,
and the plugin-mounted session controllers. These sources still build `Vec<u8>` candidates/faults
or expect a whole byte vector from a terminal candidate.

Adding `From<Vec<u8>>`, a whole-buffer getter, or a compatibility `WorkerJobSession::step().await`
would compile those callers but directly violate the repair contract: allocation would precede
admission, whole output would again be public, and mounted code could recreate a terminal drain.
Each producer needs a retained page cursor in its own job state; each consumer needs page-wise
adoption/close. Multi-page outputs require multiple external opportunities.

### 3. Cancellation with a partially filled job-owned writer still needs job close vocabulary

The retained output writer can correctly live across low-fuel turns, but `InteractiveJob` currently
has only `step`. If cancellation, panic, or host loss occurs while a concrete job owns a partially
filled writer, the universal session cannot advance that nested writer one page at a time without a
job-owned close method. Dropping the generic job shell would either recursively drop or intentionally
strand retained pages. The clean repair requires a mandatory retained-job close contract implemented
by every job producer, not a default deep-drop adapter.

### 4. Permanent verifier is intentionally not claimed

`📜️script.ts` was not extended with a P2a1 acceptance predicate because a predicate that accepted
the current source would be false evidence. The existing verifier also contains predicates for
older mounted callers that look for `session.step(&pool, ...)`; those predicates and their faithful
mutations must be updated together with the caller cutover.

## Scoped Checks Run

- `rustfmt --edition 2024` on the universal job component.
- `rustfmt --edition 2024 --check` on the same component: clean.
- `git diff --check -- <job component>`: clean.
- Static production-definition census: no public production definition of
  `run_to_completion`, `run_on_worker`, or `run_on_worker_async` remains in the universal job
  component. The old names that remain there are inside the disabled historical test module and
  documentation; mounted external callers remain as described above.

No Cargo, Nx, Wasm, browser, runtime, network, or broad build/test command was run. Type or runtime
success is not claimed.

## Required Continuation

1. Add the mandatory job-owned retained close cursor and migrate every non-Puzzle `InteractiveJob`
   producer before relying on non-Clone universal outcomes.
2. Add mounted renderer/plugin/native/Wasm registries that own one session generation through
   take/resume/terminal/close/terminal-empty and advance only one opportunity per host turn.
3. Migrate all payload consumers to page-wise adoption; remove indirect whole-vector expectations.
4. After P4e and P1q quiesce, migrate their deferred callers without overwriting accepted peer work.
5. Add the complete hostile cap/+1/zero-fuel/cancel/panic/stale/ABA/drop/lost-wake suite and faithful
   verifier mutations, then run only the explicitly allowed scoped gates.
