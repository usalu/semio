# Runtime Handback Entry — Native GREEN R40

Command: `bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib retained_handback_ -- --nocapture'`.

```text
retained_handback_maintenance_entry_does_not_wait_for_registry ... ok
retained_handback_poison_is_fault_without_mutating_queued_owner ... ok
retained_handback_take_entry_does_not_wait_for_registry ... ok
Summary [0.056s] 3 tests run: 3 passed, 101 skipped
NX Successfully ran target test for project @semio-tech/ui-runtime-rs
```

Exit 0. Raw: `🧪️member-runtime-handback-green-r40-native-2026-08-27.txt`. Both public entry functions now use nonblocking acquisition and typed `Result` faults. Busy close returns `Ok(false)` and busy retrieval returns `Ok(None)`, preserving the exact registry-owned state. Poison is not recovered in production. Resident deferred-return errors are propagated instead of discarded.

Maintenance now borrows the queued state in its original registry slot throughout its step, removing the old take/execute/requeue ownership gap. Only a terminal empty state is removed; a nonterminal state remains in place and its queue position rotates.

The underlying old reconciler/tree retirement and blocking ordinary Drop handback/admission helpers are not certified by these entry tests. They remain separate open lifecycle obligations. R40 precedes the immediately following small preflight/typed-patch error-forwarding hardening; that source is covered by the next runtime regression gate, not retroactively attributed to this run.
