# Coordinator Independent P1u Controlled-Wake Final Audit — 2026-08-23

## Verdict

**ACCEPT the bounded P1u capability-open source packet.** Phase 1 remains RED for five production
DB-engine waits and all unrun build/runtime/platform evidence.

## Final rejection re-audit

The prior audit found that the controlled wake fixture pre-set `scheduled = true`, masking real
successor admission. The repaired fixture now uses the production scheduling route:

1. `state.schedule()` builds the same `submit_drive_job` closure used in production.
2. A fixed eight-slot test queue intercepts that closure without changing its `drive_one` body.
3. The initial callback clears production scheduling state and polls the controlled backend once.
4. A waker fired during the poll coalesces with exactly one phase successor.
5. The retained real waker fires again after release and cannot admit a duplicate successor.
6. The exact queued successor is executed. Pending reaches a second poll only on that later grant;
   Ready and panic remain at one poll while the successor advances retained cleanup.

Direct source inspection confirms the former `state.scheduled.store(true)`/direct-poll masking is
absent from the fixture. Ready/panic assert retained work; Ready also asserts retained staged result.
The fixture drains controlled successor work and cursor-closes state to terminal.

## Preserved live ownership

- Pending, Ready, and panic publish exact work/result/fault/phase state before polling is released.
- Rejected storage exposes typed take/retry/one-owner close; production has no `.into_parts().0`
  discard path.
- Registry-held terminal results expose take/resume/close and checked-out Drop handback.
- Retry contention advances through one compare-exchange opportunity and timer callback, without a
  production loop/spin.
- The selected capability `block_on` is absent; the exact production DB-engine census is five.

## Gates

- Independent Rust 2021 `rustfmt --check` on the DB engine source: PASS.
- Independent `bun ./📜️script.ts verify interactivity --self-test --format json`: PASS, DENY clean
  with the single recorded allowlisted test-only bridge.
- Permanent verifier inspects the live controlled fixture and rejects restored scheduling-mask and
  dropped-successor mutations; total P1u matrix is 24.
- Cargo, Nx, Wasm, browser, runtime, network, and root lint were not run.

The remaining five wait groups are catalog-root read, initial catalog-root CAS, create-document
catalog CAS, compaction, and network hello.
