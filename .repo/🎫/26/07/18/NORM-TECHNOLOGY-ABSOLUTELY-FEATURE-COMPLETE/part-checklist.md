# Norm Feature Complete — Per-Part Gate Checklist

Gate: real formulas, clause tables, DE NA divergence, numeric worked-example test, evaluate() coverage.

Verified: `CARGO_TARGET_DIR=/tmp/semio-norm-target-fc2 cargo test` — **130/130 passed** across 14 crates.

## norm_core
- [x] Annex params (gamma_m, gamma_r, xi, combination selectors)
- [x] NormHost session tests

## norm_din_4108
- [x] part_1 scope — evaluate
- [x] part_2 U-limit — evaluate + numeric test
- [x] part_3 Glaser + f_Rsi (rh_int wired) — evaluate + numeric test
- [x] part_4 material λ — evaluate + numeric test
- [x] part_5 summer heat (clause table) — evaluate + numeric test
- [x] part_6 U + bridges — evaluate + numeric test
- [x] part_7 airtightness class — evaluate + numeric test
- [x] part_8 catalog — evaluate + numeric test

## norm_din_en_16798
- [x] parts 1–17 + na_de — evaluate all + ISO 7730 PMV/PPD

## norm_din_v_18599
- [x] parts 1–12 in balance_annual — distinct part_12 tabular method

## norm_en_1990
- [x] combinations 6.10/6.10a/6.10b + accidental/seismic — evaluate

## norm_en_1991
- [x] parts 1_1–1_7, 2–4 — evaluate all + numeric e2e

## norm_en_1992
- [x] flexure/shear/punching/torsion/crack/FEM — evaluate + prestress basics

## norm_en_1993
- [x] fix part numbering — all parts distinct — evaluate + FEM

## norm_en_1994
- [x] composite + connectors + fire — evaluate all parts

## norm_en_1995
- [x] k_mod + LTB + connections — evaluate all parts

## norm_en_1996
- [x] flexure/shear/sliding/retaining — evaluate all parts

## norm_en_1997
- [x] bearing + piles shaft — evaluate all parts

## norm_en_1998
- [x] s_d base shear + parts 2–6 — evaluate all

## norm_en_1999
- [x] alloys + joints — evaluate all parts
