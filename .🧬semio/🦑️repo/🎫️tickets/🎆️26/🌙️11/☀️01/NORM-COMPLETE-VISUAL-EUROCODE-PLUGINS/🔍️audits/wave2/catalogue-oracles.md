# Catalogue oracles (wave 2, CATALOGUE-ORACLES)

No cargo, rustc or build started; Rust files were read, a few were edited. Python: `/home/deck/semio/.venv/bin/python -I`.

## 1. Oracle formulations added (independent, from the clause text)
Modules: `🔮️oracles/⚖️compliance/🐍️.py` of EN 1991 (`ev_more`), EN 1995 (`cat_t01 … cat_t123`, `ORACLE_CASES_MORE`) and EN 1996 (six `catalogue` branches). Every function carries its clause in the docstring.
- EN 1991 (41 entries): E11.03/05/06/07/08 (6.3.1.2 Table 6.1DE, 6.4, 6.3.3), E13.04/09/11 (5.3.4, 5.4, Annex E), E12.08/10 (Annex B, 3.3.3), E14.04/09/15 (4.4, 7.2.9, Annex F), E15.02/07/08 (6.1.4, A.1, 5), E16.02/06 (4.11, A.2), E17.01/02/05/07/10, E2.02-2.06/09-11/13-15, E3.04/05/07, E4.01/04/05/08. Third-party: scipy (`constants.sigma`, `constants.g`, `integrate.solve_ivp` Janssen ODE, `integrate.quad`), numpy interp.
- EN 1996 (6 of 8; M-21/M-42 already had formulations, only the vector spelling was missing): M-05 (3.6.4), M-07 (2.4.3), M-16 (6.1.2.2 with wind eccentricity), M-33 (EN 1991-1-4 5.2 + earth pressure by `quad`), M-52, M-60 (EN 1996-2).
- EN 1995 (46 entries): T-01/02/04/05/06 (3.1-3.3), T-10/12/15/16/17/18/50 (6.1-6.6; torsion by the exact Saint-Venant series, not the table), T-21/23/24 (6.3, 5.4.4), T-32/33/46 (6.4.3, 6.5.2), T-60/61 (7.2: anastruct for the bending part, scipy `quad` unit-load integrals for shear and the tapered beam), T-62 (Annex B), T-64/65/66/124 (7.3, EN 1995-2 7.3), T-72/73/75-82 (8.2-8.10, EN 1993-1-8 Table 3.4), T-91/92 (9.1: sectionproperties composite section, mesh), T-94-96/98 (9.2, EN 1998-1), T-100/102/103/105 (EN 1995-1-2; `brentq` for the fire time), T-123. T-70 (demo case without t1) handled by a fastener-only branch.

## 2. Rows and dry runs
`rows_gen.py en1991|en1996 --write`, `rows_gen_fixture.py en1995 --write`: every entry of EN 1990/1991/1995/1996/1997 has a row (`coverage_rows.py`: empty lists). New: EN 1991 +41 entries (+3 vector entries), EN 1996 +8, EN 1995 +46 fixture cases (indices 28-73). Vector inputs: step grouping `name.i=` in the two Python adapters and the two Rust catalogue adapters.
Dry runs (platform fake host): EN 1991 catalogue 159 ok / 1 failed (E17.10 row written with the first oracle, corrected to the Mills value afterwards, equal to 1e-11), EN 1996 catalogue 71 ok / 0, EN 1995 catalogue 74 ok / 0, oracle-comparison of all ten codes 4+5+4+4+4+4+2+5+3+2 ok / 0 failed (the earlier EN 1992/EN 1997 failures were the doubled fixture path, fixed upstream).
Disagreements oracle vs Rust (read): see `📌️important/requests.md` ("from: CATALOGUE-ORACLES"): EN 1995 T-95 `f_d` (666.7 vs 400 N, DE), annex-independent bracing divisor, EN 1991 E14.09 interpolation, E4.01 class boundary, E17.10 fit, E2.13 band; EN 1996 pass-through limits. These rows will fail the Rust run until the owner decides.

## 3. EN 1995 vocabulary (9 wrong labels) and registry
`🧰️scripts/demo-values/vocab_patch_en1995.py`: `f_c90` (stress -> force F_c,90,d, N), `i_tot` and `i1`@T-93 (second moments -> radii of gyration, mm), `load_g`, `load_q`, `p_d` (N/m -> N/mm), `z1`, `z3` (centroid offsets nobody reads, removed), `members`@T-50 (choice -> `n_members`, number). EN and DE labels updated in JSON and generated Rust. Registry/evaluator fixes: `t13` key, T-62 `h1..h3`, `kmod` for T-76/80/81/82, fixture case 13 `m_d`. EN 1996: `exposure` choices mx1..mx5. `demo_gen.py en1995`: 70 entries, 480 value rows, 0 unresolved; `validate.py` 0 issues for all five codes; `static_inputs_check.py` leaves only branch-dependent reads.

## 4. Dry runs left running by DEMO-VALUES
EN 1996, EN 1995 and oracle-comparison finished green (numbers above). Demo defects found and fixed on the way: EN 1996 M-60 `exposure` choice, EN 1995 T-50/T-62/T-13/T-76/T-80-82 inputs, EN 1991 E13.09 flag, load units.

- ready for rebuild: catalogue oracles
