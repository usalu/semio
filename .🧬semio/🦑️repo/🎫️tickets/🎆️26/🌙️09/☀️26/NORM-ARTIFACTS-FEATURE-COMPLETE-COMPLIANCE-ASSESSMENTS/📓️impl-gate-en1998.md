# Impl Gate — EN 1998 P-Δ Remedies

**Family:** `en1998` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998`)  
**Blocking checks:** eight `bldg-weak` P-Δ Fail ids (`sys-x` / `sys-y` × `s1`–`s4`)

## Root cause

`θ = P·d / (V·h)` (EN 1998-1 §4.4.2.2) is evaluated from storey **drift**, gravity action **P**, and storey shear **V**. The Fail remedy targeted `stiffnessX` / `stiffnessY`, which never enter `θ`, so `apply_remedy_edit` left utilization unchanged.

## Fix

File: `🧬️schema/💡️inferences/🦀️.rs` (drift + P-Δ loop)

- Check subject path → `storeys[id=…].driftXM` / `driftYM` (field that enters `θ`)
- Remedy → `Remedy::at_most` on that drift, bound `d ≤ 0.3·V·h/P` so re-evaluate yields `θ≤0.3` (Pass)
- Localized en / de explanation and remedy copy updated to match

## Runner

```text
NX_DAEMON=false bun nx run @semio-tech/norm-en1998-rs:test --skip-nx-cache
Summary [   1.737s] 74 tests run: 74 passed, 0 skipped
```
