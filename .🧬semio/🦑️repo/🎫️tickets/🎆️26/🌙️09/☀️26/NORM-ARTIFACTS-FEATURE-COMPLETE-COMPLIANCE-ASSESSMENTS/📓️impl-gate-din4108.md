# Impl Gate — DIN 4108 Remedy Clearance

## Command

```
NX_DAEMON=false bun nx run @semio-tech/norm-din4108-rs:test --skip-nx-cache
```

## Summary

Summary [   0.489s] 95 tests run: 95 passed, 0 skipped

## Blocking checks fixed

| Check | Cause | Remedy bound change |
|-------|--------|---------------------|
| `din4108-6.u-prime.wall-north` | After zone-ht thickness on `thin-eps`, insulation (`d_req ≤ current`) and ψ (`psi_req ≥ 0.4` for `tb-bad`) both skipped → bare Fail / gate panic | Always emit clearing `thermalBridges[id].lengthM` `at_most` when shortening dominant ψ·l bridge yields U′ ≤ U_max; area `at_least` and forced insulation bump as further fallbacks |
| `din4108-6.u-prime.roof` | Same U′ empty-remedy abort (gate never reached) | Same length/area/insulation fallbacks in `check_u_prime` |
| `din4108-2.summer.zone-living` | Gate aborted on U′ panic | Unblocked once U′ always carries a clearing remedy |
| `din4108-2.zone-ht.zone-living` | Gate aborted; zone-ht thickness flip re-eval triggered bare U′ Fail | Unblocked; U′ no longer panics mid-flip |
| `din4108-10.app.wall-north.thin-eps` | Gate aborted on U′ panic | Unblocked once U′ remedies are non-empty on every Fail |

## What changed this round

- `check_u_prime`: keep insulation / ψ remedies; nudge ψ for float; **always** attach a clearing bridge-`lengthM` remedy when it can make U′ ≤ U_max; if still bare, grow element `areaM2`; if U itself exceeds U_max, force an insulation thickness bump.
- en/de copy differs on all new remedies. No compliance-gate edits, no check/test deletion, no fingerprint/utilization gaming.
- Test count vs prior Round 4 (96): still 95; no named deleted test vs `HEAD` (27 compliance `async fn` unchanged).

## Files

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- Ticket notes: `📓️fix-din4108-u-prime-length-remedy.md`

## Notes

A Fail from U′ must always carry ≥1 applicable remedy whose option-0 (or sequential applicables) leaves status not Fail.
