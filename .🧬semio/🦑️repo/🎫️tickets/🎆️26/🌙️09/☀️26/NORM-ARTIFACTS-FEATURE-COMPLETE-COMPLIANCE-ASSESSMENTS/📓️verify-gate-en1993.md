# Verify Gate — EN 1993 directional weld remedy

**PASS**

## Scope

Read-only source audit of fixer claim for `en1993.1-8.4.5.directional.joint-w1` (high-strength-connection example, welded joint `joint-w1`). Canonical family root located via `os.walk`: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993`. Full compliance gate and family `nx` test were **not** rerun (cargo/rustc active elsewhere).

## Blocking

None

## Criterion 1 — Remedy path and Ed ≤ Rd after apply

**Pass.**

Directional fail branch (`schema/🦀️.rs` ~2390–2407):

- Fail when `f_ed_dir > fw_d` (directional demand vs capacity), not raw shear alone.
- Remedy targets `joints[id={joint.id}].weldThroat` via `Remedy::at_least` with `length_m(a_req)`.
- `a_req` starts as `joint.weld_throat * (f_ed_dir / fw_d)` then loops `fillet_weld_directional_n(a_req, …)` until `ed <= rd`, bumping `a_req = a_req.next_up()` each iteration.
- `apply_remedy_edit` writes `remedy.required.value` (the padded throat) into the snapshot path; re-eval recomputes the same `(ed, rd)` pair.

For `joint-w1` inputs (`weldThroat=0.003 m`, `weldLength=0.08 m`, `shear=100 kN`, `weldFu=550 MPa`, `weldGrade=S460`, `β_w=0.9`, `γ_M2=1.25`): initial `u≈1.205`; scaled throat `a_req≈0.003616` yields `ed≤rd` and `u=1.0`.

## Criterion 2 — Real stress check, not dummy utilization

**Pass.**

`part_1_8::fillet_weld_directional_n` (~981–991) derives `σ⊥`, `τ⊥`, `τ∥` (equal-share assumption), computes equivalent stress `σ_eq`, and returns `(σ_eq·A, f_w,d·A)`.

Directional assess (~2385):

```rust
.utilization(force_n(f_ed_dir), force_n(fw_d))
```

`computed`/`limit` are directional force equivalents, not a cosmetic ratio on unrelated shear-vs-capacity terms. (The simplified sibling check at §4.5.3.3 still correctly uses `F_w,Ed` vs `F_w,Rd`.)

## Criterion 3 — en/de copy not identical

**Pass.**

| Field | en | de |
|-------|----|----|
| title | Fillet weld directional method | Kehlnaht gerichtetes Verfahren |
| explanation | Directional σ_eq·A=… kN (gov. …), f_w,d·A=… kN. | Gerichtetes Verfahren σ_eq·A=… kN (maßgebend …), f_w,d·A=… kN. |
| remedy | Increase weld throat a to at least … mm. | Nahtdicke a auf mindestens … mm erhöhen. |

Compliance gate `assert_localized_copy` would reject identical en/de strings.

## Criterion 4 — next_up is not epsilon gaming

**Pass.**

`next_up` appears only on the weld-throat scalar inside a re-evaluation loop against `fillet_weld_directional_n` (simplified weld uses the same pattern ~2362–2363). No `1e-9 * field`, `field_fingerprint`, `id_score`, or `tag_fp` in the family. This is float-safe stepping on the field the check actually reads.

## Status boundary

`CheckBuilder::utilization` (`⚖️compliance/🦀️.rs` ~302) sets `Pass` when `utilization <= 1.0`. The `next_up` pad lands on the passing side of `ed ≤ rd`, so equality at the limit is `Pass`, not residual `Fail` at printed `u=1.0000`.

## Prior failure (confirmed root cause)

Pre-fix code compared `shear_ed` to `f_w,d·A` in utilization while directional demand is `σ_eq·A = √2·F_Ed` under the equal-share model — so remedy could print `u=1.0000` yet `σ_eq·A > f_w,d·A` remained. Fix aligns assess, fail guard, and remedy scaling on the same pair.

## Test evidence

Fixer cites `NX_DAEMON=false bun nx run @semio-tech/norm-en1993-rs:test` → 159 passed. **Not independently rerun** (cargo lock). No dedicated `directional.joint-w1` unit test; coverage is via cross-family `compliance_gate` on `high_strength_connection` example.

## Note — hub overlay copy

`.🧬semio/🌐hub/s14-st2-overlay/…/🔩️en1993` still has the pre-fix directional assess (`shear_ed` vs `fw_d`, no `next_up`). Canonical build path is `✏️s/…/🔩️en1993`; audit verdict applies to that tree.
