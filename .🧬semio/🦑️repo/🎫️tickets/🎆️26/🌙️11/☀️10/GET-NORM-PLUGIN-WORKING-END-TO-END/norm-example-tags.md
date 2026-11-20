# Norm example DSL tags audit

## Root cause
The DSL decoder derives enum tags with `to_kebab(variant)` unless `#[dsl(key = "...")]` is set. `PermanentKind::GSup` / `GInf` therefore decode as `g-sup` / `g-inf`. The en1990 assets carried the stale `gsup` / `ginf`, hence `unknown enum tag 'gsup' at 3:9`. The schema and decoder are right; the assets were stale. No aliases or shims were added.

## Fixes (assets only)
- en1990, 9 assets: `gsup` to `g-sup`, `ginf` to `g-inf` (permanents table). Assets: default-office, high-consequence-office, fatigue-compliant, fatigue-failing, road-bridge-compliant, road-bridge-failing, road-bridge-showcase, accidental-seismic-compliant, accidental-seismic-failing.
- en1991, 2 assets (de-office-compliant, multi-fail-noncompliant): `fire-load-basis=occupancyTable` to `occupancy-table`.
- en1991, bridge-showcase: top field `t0` to `t-0` (field `t_0`).
- en1992, 5 assets: `cement-type=N` to `n` (`CementClass::N`).
- en1996, masonry-showcase: `masonry-class=Class1` to `class1`; `design-situation=Persistent` to `persistent`.
- en1997, 3 assets (demo, compliant, noncompliant): `design-situation=bsP` to `persistent` (`SituationChoice` has persistent, transient, accidental).
- din18599, 5 assets: zones columns `internal-gains-wm2` and `lighting-power-wm2` to `internal-gains-w-m2` and `lighting-power-w-m2` (field `internal_gains_w_m2`, schema `internalGainsWM2`).

## Audit method
`audit-norm-example-tags-rs.py` (this folder) parses each family's Rust enums (dsl key, else kebab of the variant) and structs (field name to kebab) plus the shared compliance crates. It checks every `ENUM` column and every top-line field in every `🗣️.dsl.semio` of all 15 families. `audit-norm-example-tags.py` is the first, JSON-schema based attempt (noisier, superseded).

Result after the fixes: 0 enum mismatches and 0 unknown row columns in all 15 families. The only remaining reports are `artifact-kind` and `subset` in din18599, which are nested inline `Dialect` fields and false positives.

## Not covered
- Nested inline records on line 2 (for example `heating=...`, `climate-table=...`) are not checked key by key.
- No decode test was run (builds belong to the main session). I found no existing language-agnostic test parsing these examples.
- Families din4108, din16798, iso16757, vdi3805, en1993, en1994, en1995, en1998, en1999 had no mismatches.
