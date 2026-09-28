# Fix — DIN 4108 U′ bare Fail after remedy flip

## Problem

Gate panic: `Fail checks must carry at least one remedy: id=din4108-6.u-prime.wall-north`.

During remedy-flip re-evaluation (after zone-ht thickness on `thin-eps`), `check_u_prime` could still Fail while both primary remedy branches skipped:

- insulation: `d_req ≤ current thickness`
- ψ: `psi_req + 1e-12 ≥ bridge.psi` for `tb-bad` (ψ = 0.4)

## Fix

In `check_u_prime`, when U′ > U_max:

1. Keep insulation / ψ remedies when they apply (ψ bound nudged for float).
2. Always emit a clearing `thermalBridges[id].lengthM` `at_most` when shortening the dominant ψ·l bridge makes U′ ≤ U_max.
3. Fallback: grow element `areaM2` when length cannot clear but U < U_max.
4. Last resort: force an insulation thickness bump when U itself still exceeds U_max.

en/de copy differs. No gate edits, no check/test deletion, no fingerprint gaming.
