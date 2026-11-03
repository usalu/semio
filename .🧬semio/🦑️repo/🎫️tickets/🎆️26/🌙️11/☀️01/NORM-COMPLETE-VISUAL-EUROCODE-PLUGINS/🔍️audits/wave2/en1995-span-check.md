# EN1995 span check (wave 2)

Verdict: no change needed. Nothing run; no code edited.

## What en1995 computes
`action_internals` (`en1995 schema/⚖️timber/🧮️load-combinations`) for `ContinuousTwoSpan` returns ONE design moment, the interior-support hogging `qL²/8 + 3FL/16`, and shear `5qL/8 + 11F/16` (interior-support shear). Combos use gamma_G,sup on the whole member (both spans loaded, the EN-correct hogging/support arrangement). There is no span sagging effect in production, so no sagging value is underestimated.

## Why sagging cannot govern
Per unit length: hogging 0.125 qL² (line), 0.1875 FL (point) vs sagging 0.070 qL² / 0.156 FL (with the unfavourable arrangement, gamma_G,inf on the neighbouring span, incl.). Same section for both, so the support moment always governs: for 4.5+4.5, g 1.5, q 2.5 kN/m: hogging 14 617.97 vs sag 10 376.73 N·m (shared envelope). Interior-support shear 5/8 qL is the max-arrangement shear. The check is therefore conservative-safe.

## Oracle row 10 376.73
The oracle case `two-span-timber-gamma-arrangement-envelope` validates the shared `continuous_beam_envelope`, not a production sagging path. The old value 10 098.70 was the all-gamma_G,sup-incl. mismatch of the oracle, not production.

## Test (already present)
`the_two_span_support_moment_is_the_hogging_of_the_shared_arrangement_envelope` (`🧪️tests/⚖️compliance`): asserts governing ULS m_ed = envelope.hogging = 14 617.97, and every envelope sagging = 10 376.73 and < m_ed. Both values asserted; feature rows unaffected (they use the support moment).

## Residual
Rust not compiled; if a future sagging check (e.g. per-span bending with different section) is added, route it through `continuous_beam_envelope`. No rebuild request appended.
