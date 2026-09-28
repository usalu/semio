# Impl Gate — DIN 18599

## Blocking check

`din18599.2.heating-demand` — nine U-value remedies only lowered utilization from 2.4544 toward 2.10–2.43; sequential applicables left failing (`u=1.1936`) because `deltaUWbWM2k` (0.10 vs GEG ref 0.05) was gated behind `if !remedied` and never offered alongside envelope U remedies.

## Fix

In `din18599.2.heating-demand` remedies:

- Keep per-element `AtMost` U remedies to GEG reference U.
- Stack window `gValue` / `fc` remedies when below the GEG reference building (same axes `derive_balance` adjusts on the reference run).
- Always stack `deltaUWbWM2k` `AtMost` → 0.05 when above the reference surcharge (same pattern as `din18599.geg.ht-prime`), instead of exclusive fallback after U remedies.

Sequential application of those distinct inputs brings actual Q_H,nd to (or below) the reference building, so status is no longer Fail.

## nx test

Project: `@semio-tech/norm-din18599-rs`

Command: `NX_DAEMON=false bun nx run @semio-tech/norm-din18599-rs:test --skip-nx-cache`

```
     Summary [   2.483s] 113 tests run: 113 passed, 0 skipped
```
