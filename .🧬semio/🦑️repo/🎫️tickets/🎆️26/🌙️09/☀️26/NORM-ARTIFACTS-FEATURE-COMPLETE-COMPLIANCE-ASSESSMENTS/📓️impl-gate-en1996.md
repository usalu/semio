# Impl Gate — EN 1996 Remedy Bounds

**Family:** `en1996` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996`)  
**Blocking checks:**
- `en1996.3.1.material.wall-weak`
- `en1996.6.3.flexure.wall-weak.uls-bad`
- `en1996.6.1.3.concentrated.wall-weak.uls-bad.beam-A`

## Root cause

1. **Material** — `material_conformance_ok` failed on Group2 aspect **and** GP bed-joint thickness (`0.004` m ∉ 6–15 mm). Remedies only touched mortar / `f_b` / unit height, so sequential still left `u=1.5`.
2. **Flexure** — thickness remedy used a fixed `×1.2` while `M_Rd ∝ t²` (`u≈58`). Adding `As,h` without `f_yd` forced the `as_h>0 ∧ f_yd≤0` Fail branch (`u=1.5`).
3. **Concentrated** — lengthening the bearing pad lowers `β`, so `bearingLengthM × u` only dropped utilization to ~1.64 and stayed Fail.

## Fix

File: `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/🦀️.rs`

- Material: add real remedies for `unitWidthM` (aspect) and `bedJointThicknessM` (GP 10 mm / thin-layer 2 mm) so sequential applicables clear.
- Flexure: thickness bound `t·√(M_Ed/M_Rd)·1.01`; always emit `f_yd` remedy with `As,h` so one thickness apply (or sequential) leaves status ≠ Fail.
- Concentrated: primary `bearingAreaM2` bound `A·u·1.01` at fixed `β` (plus length/thickness support remedies).

Regression: `gate_blocking_fails_clear_via_remedy_bounds` mirrors the cross-family gate flip/sequential rule for the three ids.

## Runner

```text
NX_DAEMON=false bun nx run @semio-tech/norm-en1996-rs:test --skip-nx-cache
Summary [   0.507s] 145 tests run: 145 passed, 0 skipped
```
