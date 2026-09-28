# Impl Gate — EN 1994

## Family
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994` (`en1994` / `@semio-tech/norm-en1994-rs`)

## Blocking Fails (pre-fix)
Verifier [Verify EN 1994 remedies] left Fail after `apply_remedy_edit(..., remedy_index, option_index 0)`:

| Check | Symptom |
|-------|---------|
| `en1994.6.4.ltb.girder-G1` | `ltbLengthM` binary search inverted → `l_req` stayed 8.0 m (no-op); even correct min-L cannot clear because M_Ed > M_pl,Rd. `steel.wPlYM3` alone is also a no-op under catalogue `resolve()`. |
| `en1994.7.4.crack.girder-G1` | `barSpacingM` at `s_max/scale` recomputed util as `(s/s_max)*scale` → `1.000…022` Fail under `utilization <= 1.0`. |

Already clearing (must stay): `en1994.6.6.6.vlrd.beam-B1`, `en1994.9.7.2.mrd.slab-S1`, `en1994.6.6.6.vlrd.girder-G1`, `en1994.7.3.1.deflection.girder-G1`.

## Fix
1. **LTB** — correct max-L search when min L clears; otherwise OneOf `steel.designation` to heavier HEB (catalogue extended HEB450–HEB700), filtered so option 0 clears M_b,Rd ≥ M_Ed at current L; load `qAreaPa` fallback if none. Honor L down to 0.05 m in `chi_lt`.
2. **Crack** — util = `max(as_limit/As, s/s_limit)` with shared limits; remedy spacing to `0.98 * s_limit` (passing side of ≤).
3. **Test** — `bridge_girder_ltb_and_crack_remedy0_clear` asserts both clears via `apply_remedy_edit` option 0.

## Runner
```
NX_DAEMON=false bun nx run @semio-tech/norm-en1994-rs:test --skip-nx-cache
```

```
Summary [   0.887s] 76 tests run: 76 passed, 0 skipped
```
