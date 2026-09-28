# Verify Gate — EN 1994 Remedy-Clear Fix

**Verdict: FAIL**

**Blocking:** `en1994.6.4.ltb.girder-G1`, `en1994.7.4.crack.girder-G1`

## Scope

Read-only audit of the EN 1994 remedy-clear fix claimed in [📓️impl-gate-en1994.md](📓️impl-gate-en1994.md). Family: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994`. Gate criterion: `apply_remedy_edit(..., remedy_index 0, option_index 0)` then `evaluate()` must leave status **not** `Fail`.

**Tests run:** `@semio-tech/norm-en1994-rs:test` — `75 passed, 0 skipped` (1.884s summary). Full cross-family compliance gate **not** rerun.

**Targeted remedy probe:** ticket `🗑️generated/verify_remedy_clear` binary against `composite_floor_beam_failing` and `composite_bridge_girder` DSL examples (same scopes as cross-family gate).

## Per-check audit

| Check | Example | remedy[0] field | Reads same field? | remedy[0] clears Fail? | en/de distinct? |
|-------|---------|-----------------|-------------------|------------------------|-----------------|
| `en1994.6.6.6.vlrd.beam-B1` | failing floor | `transverseAsM2PerM` → 0.001662 | yes (`longitudinal_shear_resistance_n` steel term) | **yes** u 5.031→1.000 Pass | yes |
| `en1994.9.7.2.mrd.slab-S1` | failing floor | `asM2PerM` → 0.000564 | yes (`bending_resistance_nm_per_m`) | **yes** u 1.927→1.000 Pass | yes |
| `en1994.6.6.6.vlrd.girder-G1` | bridge girder | `transverseAsM2PerM` → 0.001617 | yes | **yes** u 3.154→1.000 Pass | yes |
| `en1994.6.4.ltb.girder-G1` | bridge girder | `ltbLengthM` → 8.000 | yes (`ltb_moment_resistance_nm`) | **no** u 1.948→1.948 Fail | yes |
| `en1994.7.3.1.deflection.girder-G1` | bridge girder | `spanM` → 16.635 | yes (δ∝L⁴, limit∝L) | **yes** u 1.636→0.941 Pass | yes |
| `en1994.7.4.crack.girder-G1` | bridge girder | `barSpacingM` → 0.106889 | yes (`crack_demand` spacing term) | **no** u 1.403→1.00000000000000022 Fail | yes |

### Passing checks (4/6)

**§6.6.6 V_L,Rd (beam-B1, girder-G1).** Remedy sizes `transverseAsM2PerM` from `v_l_ed / (f_yd · L/2)` (with linear fallback), matching `longitudinal_shear_resistance_n` steel branch. One apply brings utilization to 1.0 (Pass).

**§9.7.2 m_Rd (slab-S1).** Remedy sizes `asM2PerM` from `(m_ed − sheeting share) / (0.9·d·f_yd)`, matching `bending_resistance_nm_per_m` decomposition. One apply → Pass.

**§7.3.1 deflection (girder-G1).** Remedy[0] scales `spanM` by `(δ_lim/δ)^(1/3)·0.98`; δ and limit both scale with span for area loading. One apply → u≈0.94 Pass. Secondary `qAreaPa` remedy exists but is not needed for remedy[0].

### Blocking checks (2/6)

**§6.4 LTB (girder-G1).** Hogging LTB applies (`continuous_2_span`, `m_hog_nm > 0`). Binary search in `💡️inferences/🦀️.rs` returns `l_req = hi = 8.0 m`, equal to current `ltbLengthM`. `apply_remedy_edit` is a **no-op**; utilization unchanged at **1.948**. Root cause: search interval `[0.05, 8]` never finds `M_b,Rd ≥ M_Ed` (when resistance is low at `mid`, `lo = mid` walks toward longer L, which further reduces `χ_LT`); emitted remedy cannot shorten below the already-failed length.

**§7.4 crack (girder-G1).** Only remedy offered is `barSpacingM` (hogging `asHoggingM2PerM` already ≥ `as_req`). After apply, computed/limit = `1.00000000000000022 / 1.0`; `CheckBuilder::utilization` uses strict `utilization <= 1.0`, so status stays **Fail** despite displayed u≈1. Spacing-only sizing hits a floating-point equality cliff; does not satisfy gate “equality at limit must Pass”.

## Anti-gaming

Grep over `🧩️en1994` family: **no** `fingerprint`, `epsilon fold`, `id_score`, `tag_fp`, or dummy utilization patterns. Checks use normative demand/resistance ratios only.

## Locale

For all six checks, `title`, `explanation`, and remedy copy use `lc(en, de)` with distinct prose (e.g. vlrd de explanation adds “(DE-NA)”; crack remedy “Reduce bar spacing…” vs “Stababstand … verringern”; LTB “LTB (construction stage…)” vs “Biegedrillknicken (Herstellung…)”).

## Conclusion

The fix clears **4 of 6** previously blocking Fails on remedy[0]. Family unit suite passes, but that suite does **not** assert cross-family remedy-flip parity. Bridge-girder **LTB** and **crack** checks remain blocking for compliance-gate remedy[0] semantics.
