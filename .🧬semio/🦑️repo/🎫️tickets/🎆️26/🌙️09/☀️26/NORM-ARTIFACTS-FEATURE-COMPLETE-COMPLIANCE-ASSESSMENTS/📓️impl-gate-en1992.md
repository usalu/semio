# Impl Gate — EN 1992 Remedy Bounds

**Family:** `en1992` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992`)  
**Project:** `@semio-tech/norm-en1992-rs`

## Blocking checks (before)

| Check | Symptom |
|-------|---------|
| `en1992.6.1.flexure.acc.beam-B1` | remedy[0] only lowered 3.6432→3.0250 |
| `en1992.7.2.sigma-s.beam-B1` | remedy[0] only lowered 5.9600→1.0592 |
| `en1992.7.2.sigma-c.beam-B1` | remedy[0] left failing u=3.3364 |
| `en1992.7.2.creep.beam-B1` | remedy[0] only lowered 2.0286→1.7490 |
| `en1992-4.cone.anc-1` | remedy[0] landed at u=1.0000 and stayed Fail |
| `en1992-4.splitting.anc-1` | remedy[0] left failing u=5.8676 |
| `en1992-4.interaction.anc-1` | remedy[0] left failing u=15.1169 |

## Root cause

Remedy `required` bounds were too shallow (fixed 1.1–1.25× multipliers), targeted a field the check does not read (`height` for σ_c which uses `effectiveDepth`), or hit floating-point equality on cone (`N_Rd ∝ h_ef^1.5`). Splitting was already c1-capped (`c1 ≥ 1.5 h_ef`), so raising `c1` alone did nothing. Interaction’s `aS`-only remedy did not move concrete-governed `N_Rd` / edge `V_Rd`.

## Fix

File: `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`

- **flexure.acc** — `effectiveDepth` ≥ `d · (|M_Ed|/M_Rd) · 1.05`
- **sigma-s** — bar `diameter` · √(A_need/A_s) · **1.2** (cracked-section nonlinearity margin)
- **sigma-c** — retarget **`effectiveDepth`** ≥ `d · (σ_c/limit) · 1.05` (was inert `height`)
- **creep** — `effectiveDepth` ≥ `d · (σ_c/0.45 f_ck) · 1.05`
- **cone** — `hEf` · (N_Ed/N_Rd)^(2/3) · **1.05** (strictly past equality)
- **splitting** — retarget **`hEf`** with capped/uncapped capacity formula (c1 already at cap)
- **interaction** — sequential applicables: `hEf`, `c1`, `aS` scaled from utilization so concrete + steel + edge all clear together

Added regression: `gate_blocking_remedies_clear_fail_status` (mirrors gate apply_remedy_edit / sequential clear).

## Runner

```text
NX_DAEMON=false bun nx run @semio-tech/norm-en1992-rs:test --skip-nx-cache
Summary [   0.890s] 98 tests run: 98 passed, 0 skipped
```
