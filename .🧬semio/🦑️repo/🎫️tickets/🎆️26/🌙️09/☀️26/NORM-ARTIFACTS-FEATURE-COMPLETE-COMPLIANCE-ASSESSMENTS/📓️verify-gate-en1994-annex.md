# Verify Gate — EN 1994 Annex Divergence

**Verifier:** read-only source audit (no source edits)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994` (located via `os.walk`, basename `🧩️en1994`)  
**Fixer:** [Fix EN 1994 annex divergence](9e3116df-5c1b-430d-a656-6048bb288d6e) — claimed real NA NDPs (γ_M1, deflection limit) + 78/78 family tests  
**Impl note:** `📓️impl-gate-en1994-annex.md`

---

## VERDICT: PASS

**Blocking list:** None

---

## Method

- Read-only inspection of `🧬️schema/🦀️.rs`, `💡️inferences/🦀️.rs`, compliance unit tests, compliance-report tests, and cross-family gate `assert_annex_divergence`.
- **Did not rerun** compliance gate or family `nx test`: fleet mutex held (`wg11` overlay / `.tmp-ticket/📜️fleet-mutex.sh`); another agent may be compiling DIN 4108 work.
- Fixer claim of `78 passed` family tests taken from `📓️impl-gate-en1994-annex.md` only — not independently executed this round.

---

## 1. Gate divergence on default document

Gate (`✏️s/🔌️plugins/📕️norm/🧪️tests/🚦️compliance-gate/🦀️.rs`) evaluates `En1994Snapshot::default()` with `annex` set to `De` vs `En` and requires at least one shared check id whose `limit` or `computed` differs by > 1e-12 (`annex_values_differ`).

| Item | Source finding |
|------|----------------|
| `ANNEX_IDENTICAL_ALLOWLIST` | `en1994` **not** listed (only `din4108`, `din18599`, `iso16757`, `vdi3805`) — correct |
| Default snapshot | Building, `beam-B1`, span 8 m, `construction=propped` → LTB check **NotApplicable** on default |
| Diverging check on default | `en1994.7.3.1.deflection.beam-B1`: `limit` = `deflection_limit_m(8, annex)` → **EN 0.032 m (L/250)** vs **DE 0.02667 m (L/300)**; `computed` δ identical → gate passes on **limit** delta |
| γ_M1 on default gate path | `gamma_m1` only enters `ltb_moment_resistance_nm` (limit side of LTB util); not exercised on default because §6.4 N/A — **deflection limit alone satisfies gate** |

Prior fleet failure (`📓️verify-gate-din4108-r3.md`): `en1994: FAIL — DE vs EN annex produced identical limit/computed values` — addressed by annex-aware `deflection_limit_m` / `deflection_span_divisor`.

---

## 2. Real NDP values (not fingerprint / epsilon)

| NDP | EN | DE | Wiring |
|-----|----|----|--------|
| γ_M1 (EN 1994-1-1 §2.4.1.2 → EN 1993-1-1) | 1.00 | 1.10 (DIN EN 1993-1-1/NA) | `AnnexParams::en/de`, `ltb_moment_resistance_nm` divides by `p.gamma_m1` |
| Frequent deflection limit (EN 1990 Tab. A1.4 / DIN EN 1990/NA) | L/250 | L/300 | `deflection_limit_m`, `deflection_span_divisor` |

**No gaming:** grep over family `*.rs` shows no `field_fingerprint`, `tag_fp`, `id_score`, or annex-keyed dummy offsets. `GAMING_TOKENS` in gate does not match en1994 evaluate paths.

**Unit tests** (`🧪️tests/⚖️compliance/🦀️.rs`):

- `de_gamma_m1_stricter_than_en` — asserts 1.0 vs 1.1 and `m_en / m_de ≈ 1.1` on placeholder beam.
- `de_deflection_limit_tighter_than_en` — asserts `8/250` vs `8/300` to 1e-12.

---

## 3. Remedies not regressed (source + existing tests)

| Requirement | Evidence |
|-------------|----------|
| `en1994.6.4.ltb.girder-G1` remedy option 0 clears Fail | `bridge_girder_ltb_and_crack_remedy0_clear` applies remedy[0] on `composite-bridge-girder` DSL (`girder-G1`, hogging → LTB active); designation `one_of` filter calls `ltb_moment_resistance_nm(..., annex)` with document annex |
| `en1994.7.4.crack.girder-G1` remedy option 0 clears Fail | Same test; crack remedies (`at_least` hogging A_s, `at_most` bar spacing) unchanged in structure |
| `vlrd` remedy | `en1994.6.6.6.vlrd.*` still `Remedy::at_least` on `transverseAsM2PerM` with `as_need` from shear demand (lines ~295–311) |
| Slab `m_Rd` remedy | `en1994.9.7.2.mrd.*` still `Remedy::at_least` on `asM2PerM` when `m_ed > m_rd` (lines ~612–637) |
| Deflection remedy | Still scales from annex-aware `delta_lim` / `lim_div` (span and imposed-load `at_most` remedies, lines ~425–477) — not a no-op |

Tests not re-executed this round (fleet lock); structure and regression test bodies match fixer claims.

---

## 4. EN / DE localized copy

Deflection check explanation (`💡️inferences/🦀️.rs` ~434–454):

- EN: `"EN 1990 Table A1.4 recommended"` / `"EN 1990 Tab. A1.4 empfohleniert"`
- DE: `"DIN EN 1990/NA variable-actions appearance"` / `"DIN EN 1990/NA Verformung aus veränderlichen Einwirkungen"`
- Remedy copy embeds annex-specific `L/{lim_div}` (250 vs 300).

Gate `assert_localized_copy` requires `explanation.en ≠ explanation.de` per check — deflection check satisfies.

---

## 5. γ_M1 when LTB is active (beyond default gate doc)

On bridge / hogging / unpropped subjects, LTB limit `M_b,Rd` scales as `1/γ_M1` → DE limit ≈ 10% lower than EN (test `m_en / m_de ≈ 1.1`). This is a real resistance change, not a display-only annex tag.

---

## Summary vs fixer claims

| Fixer claim | This audit |
|-------------|------------|
| `evaluate()` applies real NA NDPs (γ_M1 1.1 DE, L/300 vs L/250) | **Confirmed in source** |
| Annex De vs En changes limit/computed on gate default | **Confirmed** — deflection `limit` on `en1994.7.3.1.deflection.beam-B1` |
| Not on `ANNEX_IDENTICAL_ALLOWLIST` | **Confirmed** |
| No epsilon / fingerprint | **Confirmed** |
| LTB + crack girder-G1 remedy[0] clear | **Confirmed in test source** (not rerun) |
| vlrd / slab mrd / deflection remedies intact | **Confirmed in source** |
| 78/78 family tests | **Not rerun** — fleet mutex; impl note only |
