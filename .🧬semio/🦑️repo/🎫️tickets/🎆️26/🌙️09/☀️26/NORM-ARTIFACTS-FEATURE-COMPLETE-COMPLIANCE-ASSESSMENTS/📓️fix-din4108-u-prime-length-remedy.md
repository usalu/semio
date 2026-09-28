# Fix — DIN 4108 U′ remedy[0] printed 1.0000 still Fail (R4)

## Problem

Verifier re-run: `din4108-6.u-prime.wall-north — remedy[0] only lowered utilization 3.2819→1.0000; status still Fail after option-0 apply on failing_thin_insulation`.

## Diagnosis

Insulation ulp-nudge stopped when `1/(r_other + d/λ) + bridge_add ≤ U_max`. That combined-denominator U is **one ulp lower** than the check’s `u_value(total_resistance = r_si+r_se+Σ layer R)`. After apply:

| Quantity | Value |
|----------|-------|
| formula sum | `0.83333333333333337034` (≤ U_max → nudge stops) |
| re-eval U′ | `0.83333333333333348136` |
| utilization | `1.00000000000000022204` → prints `1.0000`, status Fail (`util ≤ 1` false) |

## Change

- Clearance predicate = same as `CheckBuilder::utilization`: `u_prime / u_max ≤ 1`.
- Insulation candidate U′ uses the same R_T layer-sum as `total_resistance` (`u_prime_at_insulation_thickness`).
- ψ / lengthM / area / guarantee branches nudge with the same utilization predicate.
- New test `failing_thin_u_prime_option0_clears_wall_north` covers the gate’s initial-doc remedy[0] path; `zone_ht_flip_leaves_u_prime_and_targets_with_clearing_remedies` kept.

## Result

Summary [   1.740s] 97 tests run: 97 passed, 0 skipped
