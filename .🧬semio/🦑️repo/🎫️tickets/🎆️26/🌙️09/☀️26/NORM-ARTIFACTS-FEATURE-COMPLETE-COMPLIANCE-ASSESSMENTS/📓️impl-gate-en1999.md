# Impl Gate — EN 1999 Remedy Bounds

## Goal

Clear every listed cross-family compliance-gate Fail for family `en1999` by changing remedy bounds so `apply_remedy_edit(..., option_index 0)` (or the sequential set of applicables on different inputs) leaves the check status not `Fail`.

## Changes (schema remedies)

Family root located by `os.walk` under norm plugin artifacts matching directory name containing `en1999`.

In `🧬️schema/🦀️.rs`:

1. Added `scale_char_for_limit` / `scale_action_leaf_remedies` so characteristic leaves (N_k, M_k, g_k, q_k, …) scale by `limit/design × 0.99` across **all** contributing actions (permanent + variable). Setting only the lead action to Rd left G-driven design effects failing.
2. Fixed auto-fallback in `utilization_check`: buckling/LTB length remedies no longer target `0` (L_cr falls back to `member.length`).
3. Explicit remedies for `fb.z`, `lambda.y`, `lambda.t` (shorten L_cr to a positive length that clears).
4. Weld remedies sized with HAZ factor and `V_Ed+N_Ed` (not V alone); optional `hazExtent→0`.
5. Bolt remedies use `V+N` demand, stronger bolt count, plus action scaling.
6. Connection SLS remedies no longer write force Rd into `qKLine` (that raised utilization); scale real load leaves instead.
7. Fire: `durationS→0`, θ_a for ambient k_θ, plus member action scaling to ambient Rd.
8. Sheet bend/axial/nm and shell ring/shear: scaled action leaves + thickness capacity remedies.

Gate-parity lock: `gate_parity_every_fail_remedy_clears_status` in schema compliance tests covers the 17 listed check ids with the same clear/sequential rules as the fleet gate.

## Runner

```
NX_DAEMON=false bun nx run @semio-tech/norm-en1999-rs:test --skip-nx-cache
```

Summary: `78 tests run: 78 passed, 0 skipped`
