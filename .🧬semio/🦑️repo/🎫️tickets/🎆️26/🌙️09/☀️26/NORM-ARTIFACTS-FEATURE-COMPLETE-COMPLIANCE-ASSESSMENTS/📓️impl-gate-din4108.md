# Impl Gate — DIN 4108 Remedy Clearance

## Command

```
NX_DAEMON=false bun nx run @semio-tech/norm-din4108-rs:test --skip-nx-cache
```

## Summary

Summary [   1.740s] 97 tests run: 97 passed, 0 skipped

## Root cause (R4)

On `failing_thin_insulation`, `check_u_prime` remedy[0] (thin-eps `thicknessM`) stopped when the **combined-denominator** formula `1/(r_other+d/λ)+bridge_add ≤ U_max` held. Re-eval uses `u_value(total_resistance)` = `1/(r_si+r_se+Σ layer R)`, which is **one ulp hotter**. Gate saw utilization `1.0000000000000002` print as `1.0000` while status stayed Fail (`utilization ≤ 1` false).

## Fix

In `check_u_prime`, when U′ fails:

1. Clearance predicate matches `CheckBuilder::utilization`: `u_prime / u_max ≤ 1` (not a 4-decimal print tie).
2. Insulation candidates use `u_prime_at_insulation_thickness` — same R_T layer-sum as `total_resistance`.
3. ψ / lengthM / area / guarantee branches ulp-nudge with the same utilization predicate.
4. en/de copy still differs. No fingerprint / id_score / tag_fp / dummy utilization. Compliance gate untouched.

## Regression tests

- `failing_thin_u_prime_option0_clears_wall_north` — initial-document remedy[0] option-0 path the gate uses.
- `zone_ht_flip_leaves_u_prime_and_targets_with_clearing_remedies` — post zone-ht option-0 + sequential applicables still clear target Fails.

## Blocking checks

| Check | Status |
|-------|--------|
| `din4108-6.u-prime.wall-north` | Cleared by util≤1 insulation nudge + option0 test |
| `din4108-6.u-prime.roof` | Same `check_u_prime` path |
| `din4108-2.summer.zone-living` | Flip regression asserts clearing |
| `din4108-2.zone-ht.zone-living` | Flip regression asserts clearing |
| `din4108-10.app.wall-north.thin-eps` | Flip regression asserts clearing |

## Files

- din4108 schema `check_u_prime` (+ helpers `u_prime_utilization_clears`, `u_prime_at_insulation_thickness`)
- din4108 compliance tests (`failing_thin_u_prime_option0_clears_wall_north`, existing zone-ht flip)
