# Impl Gate — EN 1994 Annex Divergence

## Family
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994` (`en1994` / `@semio-tech/norm-en1994-rs`)

## Blocker
Compliance gate `assert_annex_divergence` on default document:

> DE vs EN annex produced identical limit/computed values for all shared check ids.

Bridge-only γ_Mf (1.15 EN / 1.35 DE) already differed, but the default snapshot is a **building** (no fatigue actions), so shared check limits stayed identical. Family is not on `ANNEX_IDENTICAL_ALLOWLIST` (must stay off).

## Fix (real NA NDPs — no epsilon / fingerprint / dummy offset)

File: `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` + `💡️inferences/🦀️.rs`

1. **γ_M1** — `AnnexParams::de().gamma_m1 = 1.1` (DIN EN 1993-1-1/NA via EN 1994-1-1 §2.4.1.2); EN stays 1.0. Already wired into `ltb_moment_resistance_nm`.
2. **Deflection limit** — `deflection_limit_m(span, annex)`: EN L/250 (EN 1990 Table A1.4 recommended floor w_max); DE L/300 (DIN EN 1990/NA variable-actions appearance, applied to the §7.3.1 frequent check). Changes **limit** on every default-building deflection check → gate divergence.
3. Remedy / explanation copy uses annex-specific L/N and distinct en vs de prose.

## Must not regress
- `en1994.6.4.ltb.girder-G1` remedy option 0 still clears Fail (designation filter uses document annex γ_M1).
- `en1994.7.4.crack.girder-G1` stays not Fail after remedy 0.
- vlrd / slab mrd / deflection remedies unchanged in structure (deflection still scales from `delta_lim`).

## Tests added
- `de_gamma_m1_stricter_than_en`
- `de_deflection_limit_tighter_than_en`

## Runner
```
NX_DAEMON=false bun nx run @semio-tech/norm-en1994-rs:test --skip-nx-cache
```

```
Summary [   0.753s] 78 tests run: 78 passed, 0 skipped
```
