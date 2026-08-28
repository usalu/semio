# Runtime Handback Entry — Actual RED R37–R39

Three separately selected canonical native tests executed against unchanged production entry points. Each test recovered and drained the exact queued owner before its final failure assertion.

| Run | Exact selector | Actual result |
| --- | --- | --- |
| R37 | `retained_handback_maintenance_entry_` | 0 passed, 1 failed, 103 skipped; 0.158s |
| R38 | `retained_handback_take_entry_` | 0 passed, 1 failed, 103 skipped; 0.151s |
| R39 | `retained_handback_poison_` | 0 passed, 1 failed, 103 skipped; 0.029s |

All commands were `bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib <selector> -- --nocapture'`, exit 1.

R37/R38 actual assertion:

```text
assertion `left == right` failed
  left: true
 right: false
```

The boolean records whether the externally held registry mutex had already been released by its 100ms timeout when the entry returned. Both old entry points waited. R39 poisoned the actual registry, observed the old recovery path mutate its queued owner's retirement scalar, then failed `assertion failed: unchanged`. Test-only poison clearing happened before exact cleanup, never in production.

Raw logs: `🧪️member-runtime-handback-red-r37-native-2026-08-27.txt`, `🧪️member-runtime-handback-red-r38-native-2026-08-27.txt`, `🧪️member-runtime-handback-red-r39-native-2026-08-27.txt`. The permanent domain fixture/schema are independently validated with Ajv and Node UTF-8 encoding; these tests target entry contention and poison, not every nested retirement operation.
