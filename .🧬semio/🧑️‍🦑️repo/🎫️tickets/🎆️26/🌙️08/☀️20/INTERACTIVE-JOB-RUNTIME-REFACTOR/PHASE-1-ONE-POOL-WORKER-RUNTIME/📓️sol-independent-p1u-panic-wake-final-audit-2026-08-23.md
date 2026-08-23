# Sol Independent P1u Panic-wake Final Audit — 2026-08-23

## Verdict

**REJECT (source evidence).** The production panic transition now retains the exact backend work and
publishes cancellation, fault progress, terminal error, and `RetainWork` while `polling` is still
true. Pending and Ready also republish their exact work/result owners before the release boundary,
and the public post-Ready cancel/stale result checkout preserves pointer identity. The permanent
controlled-wake fixture does not execute the production scheduling state, however, so it cannot
prove the requested one-successor/no-repoll property.

This was an independent read-only audit. No production, fixture, or verifier source was edited. No
Cargo, Nx, Wasm, browser, runtime, or network command ran.

## Direct source reasoning

### Repaired production transition

- `poll_backend_once` moves the exact `DatabaseCapabilityOpenWork` back to `poll_work` in all three
  Pending, Ready, and panic branches before `polling` becomes false.
- Ready additionally installs the exact `DatabaseCapabilityOpenResult` in `staged_result` and sets
  `RetainWork` before release. Cancel/stale injected after `Future::poll` is revalidated before the
  release store.
- Panic calls `publish_poll_terminal` before `release_terminal_poll`. That helper sets cancellation,
  fault progress, the retained terminal error, and `RetainWork`; only then does the release helper
  clear `polling` and the coalesced wake and schedule cleanup. A later backend waker sees cancellation
  and cannot schedule `Poll` again.
- Normal Pending publishes `Pending` before release and uses one `wake_requested.swap(false)` to
  coalesce a wake into one scheduled successor. Normal Ready leaves `Poll` permanently before wake
  re-enable, so any later successor advances `RetainWork`, not the completed future.
- `publish_staged` revalidates cancellation and admission freshness before public completion. The
  terminal-result ticket remains registry-owned while checked out; Drop clears only the shallow
  checkout bit, and take/resume moves the exact retained result.
- Retry contention performs one `compare_exchange` opportunity and returns through a
  generation-observed callback. The capability region has no spin loop.

### Blocking evidence defect

`database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary` does use real
controlled Pending, Ready, and panic futures, and each future calls its real waker during `poll`.
Immediately before directly invoking `poll_backend_once`, however, the fixture executes:

```rust
state.scheduled.store(true, Ordering::Release);
state.poll_backend_once(state.generation);
```

Production `drive_one` does the opposite: it clears `scheduled` before entering the phase switch.
Because the fixture leaves `scheduled == true`, both the during-poll coalesced wake and the explicit
post-release wake are rejected by `schedule`'s compare-exchange. The assertion that the poll count
remains one therefore passes even if successor admission is broken. It also never asserts that
exactly one successor was retained/submitted.

`ControlledCapabilityFuture` contains a Condvar boundary, but both live calls pass `None`; no test
fires a waker from a barrier at the release boundary. The 22-mutation ledger checks textual owner,
phase, and fault ordering, but has no mutation that makes the controlled fixture use production's
`scheduled == false` entry state or removes exactly-one successor admission. The fixture-name check
cannot discriminate this defect.

The post-Ready cancel/stale fixture is otherwise meaningful: its hook runs after real Ready polling,
it drives both public cancellation and admission-generation staleness, moves the staged exact result
into the terminal registry, exercises checked-out Drop handback and resume, and checks the storage
pointer.

## Required repair packet

1. Drive Pending, Ready, and panic through the real scheduled callback or explicitly reproduce
   production entry with `scheduled == false`; do not mask wake admission with a pre-set flag.
2. Use the existing controlled boundary (or an equivalent deterministic hook) to fire a cloned real
   waker during polling, immediately after all owner/scalar publication, and immediately after the
   release store.
3. Assert exactly one retained/submitted successor, never merely `polls == 1`: Pending's successor
   may poll only on its next governed opportunity; Ready and panic successors must enter cleanup and
   must never repoll the completed/panicked future.
4. Add faithful mutations that pre-set `scheduled`, delete the successor assertion, bypass the
   boundary, and route a Ready/panic wake back to `Poll`; each must be rejected.

## Gates and census

- Edition-2021 Rustfmt check on DB engine and facade: **PASS**.
- Interactivity self-test: **PASS**; all 22 declared P1u mutations reject and DENY is clean.
- Plain interactivity verifier: **PASS**; DENY is clean.
- Production census: exactly five `db_actor::block_on` groups before the test region and exactly one
  `storage.capabilities().await` in retained backend work.
- Capability-region scan: no loop/spin, private pool/thread construction, blocking bridge, or live
  rejected-owner `.into_parts().0` discard.
- Scoped and whole working/staged/HEAD diff checks: **PASS**.

## Residual status

The single backend capability call remains the explicit Phase 9 latency residual. Five DB-engine
wait groups and Phase 1 remain RED. The production panic/wake ordering is materially repaired, but
P1u should not be accepted until the deterministic successor fixture and mutations exercise the
same scheduled state as production.
