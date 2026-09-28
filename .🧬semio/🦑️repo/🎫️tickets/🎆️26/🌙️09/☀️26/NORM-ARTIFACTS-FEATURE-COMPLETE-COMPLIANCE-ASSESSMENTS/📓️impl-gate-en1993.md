# Impl Gate — EN 1993

## Blocking Fail

`en1993.1-8.4.5.directional.joint-w1`: remedy[0] lowered utilization `1.1506→1.0000` but status stayed Fail because exact `a_req = a·(F_Ed/F_Rd)` left a float residual (`u = 1 + ε`), and the check compared shear force to `f_w,d·A` instead of the directional demand `σ_eq·A`.

## Fix

In `🗿️artifacts/🔩️en1993/…/🧬️schema/🦀️.rs` fillet-weld directional assess:

1. Utilization uses directional demand `(σ_eq·A, f_w,d·A)` — EN 1993-1-8 §4.5.3.2 ≤ clause.
2. Throat remedy scales `weldThroat` until `σ_eq·A ≤ f_w,d·A` after float (`next_up` loop), so re-eval clears Fail (equal capacity Passes; residual cannot leave Fail at printed `u=1.0000`).
3. Simplified weld remedy similarly pads with `next_up` until `F_w,Rd ≥ F_w,Ed`.
4. Member interaction / M+N / compression Fails always carry a real section or buckling/area remedy (unblocked family tests).

## nx

```
NX_DAEMON=false bun nx run @semio-tech/norm-en1993-rs:test --skip-nx-cache
```

Summary [   0.868s] 159 tests run: 159 passed, 0 skipped
