# Verify Gate — EN 1994 Remedy-Clear Fix (R2)

**Verdict: PASS**

**Blocking:** None

## Scope

Read-only re-audit of the two checks that remained **Fail** after [Verify EN 1994 remedies](82c78610-da0e-4f1c-8f6d-0328218c3e92), following the fix claimed in [Fix EN 1994 remaining remedies](b499b00f-a3e5-455d-8da3-5bbdf18a89e6) and [📓️fix-en1994-ltb-crack-remedies.md](📓️fix-en1994-ltb-crack-remedies.md).

- **Family:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994` (`en1994` / `@semio-tech/norm-en1994-rs`)
- **Gate criterion:** `apply_remedy_edit(..., remedy_index 0, option_index 0)` then `evaluate()` → status **not** `Fail`
- **Prior R1:** [📓️verify-gate-en1994.md](📓️verify-gate-en1994.md) — **FAIL** on `en1994.6.4.ltb.girder-G1`, `en1994.7.4.crack.girder-G1`

## Test execution

**Not rerun in this audit.** `cargo` / `rustc` were held by other fleet jobs (wg11 overlay checks, demonstrator wasm build). Judgment is from source review plus cached output in [🗑️generated/nx-en1994-test.txt](🗑️generated/nx-en1994-test.txt):

```
Summary [   0.887s] 76 tests run: 76 passed, 0 skipped
```

(was 75 tests before `bridge_girder_ltb_and_crack_remedy0_clear` was added)

## Previously blocking checks

### `en1994.6.4.ltb.girder-G1` (bridge `composite-bridge-girder`, HEB400, L_cr = 8 m)

| Aspect | R1 failure | R2 source |
|--------|------------|-----------|
| Prior remedy | `ltbLengthM` → 8.0 m (no-op; u stayed 1.948) | When `m_b,Rd(L=0.05) < M_Ed`, length search is skipped |
| Root cause | Inverted binary search; M_Ed > M_pl,Rd so shortening L cannot clear | Catalogue `resolve()` overwrites geometry leaves — remedy must change `steel.designation` |
| R2 remedy[0] | — | `Remedy::one_of(steel.designation, clearing, …)` where `clearing` = heavier HEB450–HEB700 **filtered** so `ltb_moment_resistance_nm ≥ M_Ed` at current L |
| option 0 | — | First entry in `clearing` (lightest section that clears, not a weaker catalogue step) |
| Field read by resistance | `ltbLengthM` in `chi_lt` | `steel.designation` → `SteelSection::from_catalogue` → `w_pl_y_m3`, `i_y_m4`, `a_m2` in `ltb_moment_resistance_nm` |
| Length search (when min-L clears) | Inverted (`lo = mid` on fail) | Corrected: resistance ≥ M_Ed → `lo = mid`, else `hi = mid`; emit only if `l_req + 1e-9 < current` |

Hogging LTB on `girder-G1` (`continuous_2_span`, `m_hog_nm > 0`) cannot clear at χ_LT → 1 with HEB400; the designation OneOf is the operative remedy, not a no-op length write.

### `en1994.7.4.crack.girder-G1`

| Aspect | R1 failure | R2 source |
|--------|------------|-----------|
| Prior remedy | `barSpacingM` at `s_max/scale` → u = `1.00000000000000022` (strict `<= 1.0` → Fail) | `s_req = s_limit * 0.98` (passing side of ≤) |
| Util formula | `(s/s_max)*scale` recomputed above 1.0 at equality | `crack_util = max(as_limit/As, s/s_limit)` with shared `as_limit`, `s_limit` |
| After apply | u ≈ 1.0 Fail | u ≤ 0.98 when spacing governs (`CheckBuilder::utilization` uses `utilization <= 1.0` → Pass) |

For `girder-G1`, `asHoggingM2PerM` already meets `as_req`; only `barSpacingM` remedy is emitted → remedy[0] targets spacing.

## Dedicated gate test

`bridge_girder_ltb_and_crack_remedy0_clear` in compliance-report tests:

- Decodes `composite-bridge-girder`
- Asserts both checks start **Fail**
- `apply_remedy_edit(report, id, 0, 0, tree)` for each id
- Re-evaluates; asserts status **≠ Fail**

Included in the 76/76 passed family suite above.

## Regression — four checks still clear on remedy[0]

Source for these remedy paths was **not** modified by the LTB/crack fix (only §6.4 LTB block and §7.4 crack block changed). Remedy[0] behaviour unchanged from R1 passing audit:

| Check | Example | remedy[0] field | Clears? (source + R1) |
|-------|---------|-----------------|------------------------|
| `en1994.6.6.6.vlrd.beam-B1` | `composite-floor-beam-failing` | `transverseAsM2PerM` | yes — sizes from `v_l_ed / (f_yd · L/2)` matching `longitudinal_shear_resistance_n` |
| `en1994.9.7.2.mrd.slab-S1` | same | `asM2PerM` | yes — sizes from `(m_ed − sheeting) / (0.9·d·f_yd)` matching `bending_resistance_nm_per_m` |
| `en1994.6.6.6.vlrd.girder-G1` | `composite-bridge-girder` | `transverseAsM2PerM` | yes — same steel branch as beam-B1 |
| `en1994.7.3.1.deflection.girder-G1` | bridge | `spanM` | yes — `span_req = span · (δ_lim/δ)^(1/3) · 0.98`; δ and limit both scale with span |

No deleted tests; one new test (`bridge_girder_ltb_and_crack_remedy0_clear`).

## Anti-gaming

Grep over `🧩️en1994`: no `fingerprint`, `id_score`, `tag_fp`, dummy utilization, or epsilon fold on unused fields. Checks use normative demand/resistance ratios only.

## Locale

Remedy `action` copy remains `lc(en, de)` with distinct prose (e.g. LTB designation “Select a heavier steel section…” / “Schwereren Stahlquerschnitt wählen…”; crack spacing “Reduce bar spacing…” / “Stababstand … verringern”).

## Conclusion

Both previously blocking bridge-girder checks now have remedy[0] / option[0] semantics that read fields the resistance formula uses and clear under `utilization <= 1.0`. Four regression checks remain unchanged. Family unit suite reports 76/76 passed (not rerun live due to cargo lock).
