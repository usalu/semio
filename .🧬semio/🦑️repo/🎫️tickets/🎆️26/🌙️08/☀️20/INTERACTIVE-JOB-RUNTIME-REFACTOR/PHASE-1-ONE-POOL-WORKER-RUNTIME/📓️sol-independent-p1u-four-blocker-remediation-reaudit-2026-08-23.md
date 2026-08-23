# Sol Independent P1u Four-blocker Remediation Re-audit — 2026-08-23

## Admission and verdict

I independently re-audited the P1u remediation against
`📓️sol-independent-p1u-db-engine-retained-capability-open-audit-2026-08-23.md`,
the updated implementation report, DB engine/facade source, fixtures, permanent verifier, and
working/staged/HEAD diffs. I made no production or verifier edits.

**REJECT — source-only.** Rejected-owner handback, public terminal-result checkout, and bounded
retry contention are materially repaired. Pending and Ready publish their exact owners before
re-enabling scheduling. The panic branch still exposes one live wake race, however, and the new
fixtures/mutation do not exercise that race or real post-Ready cancel/stale transitions. P1u and
Phase 1 therefore remain RED.

## Evidence that now passes

### Exact census and retained boundary

- The production DB engine contains exactly five `db_actor::block_on` groups: catalog read,
  catalog initialization CAS, create-document CAS, compaction, and sync hello.
- The selected `db_actor::block_on(storage.capabilities())` is absent. The capability region has
  exactly one `storage.capabilities().await`, inside the retained backend work future.
- The fixed 64-slot, 8-item/16-KiB operation, 512-item/1-MiB aggregate admission and the eight
  `Handoff → Poll → RetainWork → DrainWork → ReleaseWork → RetainResult → Publish → Terminal`
  phases remain present.

### Three repaired rejection findings

- `DatabaseCapabilityOpenRejected` retains the exact `Arc<DbBackend>` and exposes typed
  `take_storage`, `retry`, `close_step`, terminal witness, and error-after-close. The live
  `open_with` route explicitly closes the one rejected owner and has no production
  `.into_parts().0` discard.
- `terminal_result` remains registry-owned while
  `DatabaseCapabilityOpenTerminalResult` is checked out. Take, resume, close, and checked-out
  Drop handback all preserve that retained slot; the main close path blocks on
  `terminal_result_checked_out`.
- Retry generation performs one `compare_exchange` opportunity. Contention schedules one
  generation-observed timer callback; the capability region contains no `loop` or `while`.

## Blocking finding 1 — panic re-enables polling before terminal state is stable

In `DatabaseCapabilityOpenState::poll_backend_once`, Pending republishes `work` before clearing
`polling`, and Ready republishes `work`, `output`, and `RetainWork` before clearing it. The
panic arm only republishes `work`, then clears `polling`, then calls `stage_terminal`:

```rust
Err(_) => {
    *self.poll_work.lock(...) = Some(work);
    self.polling.store(false, Ordering::Release);
    self.stage_terminal(...);
}
```

A backend-held waker firing after the release store and before `stage_terminal` sees a current,
non-cancelled state in `Poll`, successfully schedules another drive, and may repoll the same work
before the first turn publishes its fault/retention phase. That violates deterministic panic
terminalization and the requested “republish before wake enable” contract even though the owner is
no longer missing.

The panic path must publish the fault scalar and a non-Poll retained phase while `polling` is
still true, then clear `polling`, consume/coalesce the wake, and schedule exactly one cleanup turn.

## Blocking finding 2 — boundary fixtures and mutation are not adversarial

`database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary` does not invoke
`poll_backend_once` or any Pending/Ready/panic future. It manually moves one work owner, manually
sets a Ready result only for boundary 1, clears `polling`, and merely swaps an already-set
`wake_requested` bit. Boundaries 0 and 2 are identical; no wake is injected after polling release,
and the panic terminal phase is never asserted. It cannot reproduce finding 1.

Likewise,
`database_capability_open_terminal_result_take_resume_and_checked_out_drop_handback` manually
inserts `terminal_result`; it does not drive backend Ready followed by cancellation or admission
staleness. It therefore does not prove the specifically requested post-Ready cancel/stale result
handback.

The permanent verifier contains exactly 19 mutations and rejects them all, but mutation 16 only
reverses the first synthetic `Some(work) / polling=false` sequence. It has no predicate for
fault/phase publication before panic wake enable. The mutation corpus is therefore not faithful to
the remaining race.

## Required repair packet

1. Split panic fault staging from scheduling: retain `work`, publish the exact error/progress and
   `RetainWork` phase while `polling == true`, then clear `polling` and admit at most one
   cleanup successor. A wake must never observe a live `Poll` phase after panic.
2. Replace the manual three-boundary fixture with controlled Pending, Ready, and panic futures plus
   a barrier/hook that fires a real cloned waker at every publication/release boundary. Assert one
   successor, no missing/duplicate owner, and deterministic panic fault.
3. Add real Ready→cancel and Ready→stale fixtures that reach `terminal_result`, then exercise
   checked-out Drop handback, exact take/resume, close blocking, pointer identity, and terminal
   emptiness.
4. Keep the exact 19-mutation ledger if required, but make the publication-order mutation remove
   the panic phase/fault-before-release guarantee and make its predicate inspect all three
   branches. The intended mutation must fail for that rule rather than only for Pending owner
   order.

## Permitted gates

| Gate | Result |
| --- | --- |
| edition-2021 `rustfmt --check` on DB engine/facade | **PASS**, exit 0 |
| interactivity self-test DENY | **PASS**, exit 0; all 19 P1u mutations rejected |
| interactivity plain DENY | **PASS**, exit 0 |
| production wait census | **PASS**, exact five |
| capability-region forbidden scan | **PASS**, no loop/while/blocking/private pool; one declared backend await |
| scoped working/staged/HEAD diff checks | **PASS**, exit 0 |
| whole working and combined-HEAD diff checks | **PASS**, exit 0 |
| whole staged diff check | **RED outside P1u**: two pre-existing Phase 2 report blank-EOF findings |
| Cargo/Nx/Wasm/browser/runtime/network | **not run by instruction** |

## Residual status

P1u remains source-rejected on the panic wake race and non-discriminating evidence. The one backend
capability poll remains the declared Phase 9 latency residual; exactly five DB-engine wait groups
remain; Phase 1 remains RED.
