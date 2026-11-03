# Feature rows per check family (wave 2, FEATURE-ROWS)

No cargo/rustc. Python only, run with `nice -n 19 ionice -c3`. Scripts: `🧰️scripts/feature-rows/{gen1992,gen1993,gen1994,append_rows,crosscheck1992}.py`, row inputs `rows19*.tsv`.

## Scope and method
Families = spec-trace `check` prefixes (all ten codes, count below) and the ids the Python check-outcome oracles emit. Coverage = a family has a pass and a fail row in any feature of the subset. A coarse scan over all ten codes (substring match of the spec-trace prefix against feature rows): families with pass+fail / total: EN1990 18/33, EN1991 11/25, EN1992 6/56, EN1993 7/97, EN1994 7/48, EN1995 1/25, EN1996 9/33, EN1997 9/37, EN1998 8/50, EN1999 5/41. Most spec-trace prefixes are scope, data or detailing rows that no independent oracle models yet; the oracle-emitted families are the actionable set.

## Oracle-emitted families, before -> after (pass/fail rows)
| code | family | before | after |
|---|---|---|---|
| EN1992 | 6.2.shear | 2/0 | 3/1 |
| EN1992 | 6.1.flexure.acc | 0/0 | 1/1 |
| EN1992 | 7.4.deflection | 0/0 | 1/2 |
| EN1992 | 7.4.ld | 0/0 | 1/2 |
| EN1992 | 9.minas (new in oracle) | 0/0 | 1/1 |
| EN1992 | 9.maxas (new in oracle) | 0/0 | 1/1 |
| EN1992 | 7.3.crack / 6.1.flexure | 2/1, 1/4 | 3/1, 2/5 |
| EN1994 | 6.2.2.vpl | 5/0 | 5/1 |
| EN1994 | 6.2.1.construction | 3/0 | 3/1 |
| EN1994 | 6.6.3.1.prd | 0/0 | 1/1 |
| EN1994 | 6.6.1.2.etamin | 1/4 | 2/4 |
| EN1994 | 7.3.1.deflection-variable | 1/0 | 1/1 |
| EN1994 | 6.7.3.npl | 4/0 | 4/1 |
| EN1994 | 9.7.5.vrd, 9.7.2.mrd | 1/0, 1/1 | 1/1, 1/2 |
| EN1993 | 6.2.3.nt | 0/0 | 1/1 |
| EN1993 | 6.2.6.vz | 1/0 | 1/1 |
| EN1993 | 1-8.3.6.shear | 1/0 | 1/1 |
Totals: EN1992 21 -> 38 rows in check-outcome, EN1994 43 -> 52, EN1993 26 -> 30. Families still without a pass/fail pair: EN1992 torsion, punching, anchorage, lap, cover, fire, scope, creep (no oracle formulation); EN1994 5.4.1.2.beff fail (utilisation beff/spacing cannot exceed 1 by construction), 6.2.1.2.hogging ok, slab/column rest ok; EN1993 6.2.4.n fail, 1-8.4.5.simplified pass (fixture weld group always 2.21), 1-8.3.9.slip (no fixture), 6.2.6.vy, class 4 and special elements. EN1998/EN1999 not started (time and host load), EN1990/1991/1995-1997 catalogue families were covered by CATALOGUE-ORACLES.

## What was added
- EN1992: 8 derived-load beam fixtures `🧪️tests/🧫️fixtures/🚦️check-outcome/beam-*` (sls-pass, ld-fail, deflection-fail, shear-fail, minas-fail, maxas-fail, accidental-pass/fail), 17 rows in `🚦️check-outcome-en1992-1`. Oracle extended: `reinforcement_limits` (9.2.1.1 Eq. 9.1N and 0.04 A_c).
- EN1994: 6 oracle-documents (beam-shear-fail, beam-construction-fail, beam-connection-pass/fail, beam-variable-deflection-fail, column-resistance-fail, slab-shear-resistance-fail), 9 rows.
- EN1993: 4 oracle-documents (tension-member, tension-member-fail, shear-member-fail, joint-bolt-fail), 4 rows.
- Third-party reproduction (`crosscheck1992.py`): beam M_Rd utilisation vs structuralcodes GenericSection (rel 1.3e-7); unit-load mid-span deflection of the elastic line vs anastruct (16.875 vs 16.8749983); f_ctm vs structuralcodes (A_s,min ratio 0.12712 vs 0.12752, 0.3 %, from the table 3.2 against the formula 3.2077). Existing `--self-check`s cover V_Rd,c, creep, cracked stresses (structuralcodes, concreteproperties), EN1993 deflection (PyNite), EN1994 plastic modulus (sectionproperties). EN1993/EN1994 new rows are closed-form clause values on existing committed oracle formulations; no new third-party hook was added for them.

## Oracle defect fixed (Python, not Rust)
EN1992 oracle compared the action kind with lowercase `"permanent"/"prestress"` (lines for `characteristic` and the deflection `factor`) while the schema enum is `Permanent/Prestress`: derived (Udl/Point) loads used q instead of g. Fixed to `("Permanent","Prestress")`. Existing rows used external loads and are unchanged (the 21 old rows stay ok).

## Dry run
`🗑️generated/doc-oracles/dry-run.py` per case: EN1992 38/38 ok, EN1994 52/52 ok, EN1993 30/30 ok. These compare the oracle with itself through the step layer (fixtures resolve, statuses consistent); the Rust side is unrun.

## Oracle vs Rust (by reading, no numbers from Rust available)
No numeric disagreement identified for the new families: Rust `7.4.ld` is `l/d / max(limit,1)`, `9.minas` is required/provided, `9.maxas` provided/(0.04 A_c), same as the oracle. Unverified assumption: Rust `Geo.a_s`/`b_t` equal the tension-bar area and the tension-zone width used by the oracle (both rectangle-exact here). Request filed in requests.md only for the `Geo` assumption.

- ready for rebuild: feature rows (en1992, en1993, en1994)
