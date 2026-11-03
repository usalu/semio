# 🚦️ Doc oracles (wave 2): check-outcome for EN 1992, 1993 and 1994

Nothing compiled or run on the Rust side (no cargo); the Python side is dry-run green for all 30 rows.

## What exists
| artifact | oracle (`🔮️oracles/🚦️check-outcome/🐍️.py`, `python -I`, `--json` default, `--self-check`) | case (`🧪️tests/🚦️check-outcome-enXXXX-1`: `🥒️.feature`, `🐍️.py`, `🦀️.rs`) | rows |
|---|---|---|---|
| en1992 | flexure `6.1.flexure` (+ `.acc`), shear `6.2.shear`, crack `7.3.crack`, `7.4.ld`, deflection `7.4.deflection` per member | yes | 5 pass, 2 fail |
| en1993 | `6.2.4.n`, `6.2.3.nt`, `6.2.5.my`, `6.2.6.vz/vy`, `6.3.1.nb`, `6.3.2.mb`, `7.sls` per member (worst row over every ultimate combination, as the step layer's prefix rule reads it), joints `1-8.3.6.shear/bearing`, `1-8.3.9.slip`, `1-8.4.5.simplified` | yes | 7 pass, 6 fail |
| en1994 | simply supported beams: `5.4.1.2.beff`, `6.2.1.3.mrd`, `6.2.1.construction`, `6.2.2.vpl`, `6.6.3.1.prd`, `6.6.1.2.etamin`, `7.3.1.deflection`, `7.3.1.deflection-variable` | yes | 7 pass, 3 fail |

Protocol as the other artifacts: `{"checks":[{"id","utilization"}]}` on stdout; status is `pass_or_fail(utilization)` in the shared step layer; the Python adapter is the en1999 one with the oracle path replaced; the Rust adapter decodes the committed snapshot with `semio_s_artifact_norm_enXXXX_io::text::snapshot::decode_enXXXX_snapshot_json`, evaluates with `semio_s_artifact_norm_enXXXX_inferences::evaluate` and picks the row with `pick_outcome`. Tolerance 1e-2; rows are away from the near-limit band (0.95 to 1.0). Manifest `🔮️oracles/🔣️.json` of the three artifacts: host packages `…-inferences` (+ `semio-s-plugin-norm-compliance` for en1993 and en1994) added, rationale of `enXXXX-1-compliance-independent` extended. The capabilities `enXXXX-1-evaluate` and oracle ids `enXXXX-1-compliance-independent` are the existing ones.

## Independence and third-party validation
No line is derived from the Rust. En1992: strain compatibility with horizontal shapely strips (parabola-rectangle, hardening steel, pivots A/B/C), cracked state II by fibre equilibrium, EN 1990 6.10 rows, Annex B creep, NA-DE and EN NDP values written from the NA (not read through the Rust). En1993: Tables 5.2, 6.1, 6.2, 6.4, 3.4, ENV 1993-1-1 C1 table, closed-form M_cr. En1994: plate strips with the catalogue area kept in the web, stress block, Table 6.2 `k_t`.
`--self-check` (all green): en1992 V_Rd,c and creep phi against `structuralcodes` (0 and 1.6e-7), M_Rd of the C35/45 tank wall section (209.358 kNm) against `structuralcodes` `GenericSection` (1.5e-7); en1993 mid-span deflection against PyNite; en1994 plastic modulus of the strip plates against `sectionproperties` (0).

## Dry run (Python side)
`🗑️generated/doc-oracles/dry-run.py <case dir>` loads each case's `🐍️.py`, resolves the `shared://` fixtures and compares status and utilisation per row: en1992 7/7, en1993 13/13, en1994 10/10 ok. Rows with values: see each `🥒️.feature`.

## Fixture notes
All en1992 mutation snapshots are one tank wall; 92 of the 147 distinct documents carry a `bars` row that moves the tension steel above the centroid (details, with numbers, in the request below). The en1992 rows use the 50 clean documents plus `remove-bar-layer` (no steel, `flexure = 210000`, the Rust floor `max(M_Rd, 1 N m)` of an unbalanced section). The en1993 and en1994 fixtures are one HEB beam-column (S460, buckling 12 m: `nb` 4.15) and one HEB300 composite beam; failing documents come from `change-material-e-modulus`, `insert-member-action`, `insert-joint-action` (en1993) and `change-beam-sheeting-height-m`, `change-beam-stud-count-per-rib`, `change-steel-fy-pa` (en1994, shear connection below `eta_min`). There is no fixture with a failing en1994 bending or deflection check.

## Not covered (stated gaps)
En1992: prestressed members with a resolved tendon (skipped), torsion, punching, stresses, fire; polygon sections are supported but not covered by a row. En1993: class 4 sections (bending, LTB and interaction skipped; N uses the effective area), interactions 6.3.3, special elements. En1994: continuous spans, columns, slabs, fire, bridges, hogging, LTB, stresses, crack.

## Mismatches found by reading (owners in `📌️important/requests.md`, section "from: doc-oracles")
1. en1992: 92 of 147 snapshots with a single `bars` row above the centroid: flexure 68.9 instead of 1.0031, crack infinite instead of 0.7489.
2. en1992: `shear_v_rd_c_n` sigma_cp uses `b d / 0.9` instead of `A_c` (0.7 % on V_Rd,c at d = 0.495 m).
3. en1992: `member_section` does not centre polygon sections: M_Rd about the frame origin, shift `N_Ed z_c` (15.75 kNm of 209 kNm for z_c = 0.25 m).
4. en1994: `section_class` omits the root radius (HEB300 flange class 2 against 1).
5. en1993/en1994: governing row chosen by a unit-mixed sum.
No numeric disagreement of a check value is known: the Rust values could not be run. If a row fails after the rebuild, the oracle number is in the feature and the oracle can be run on the fixture with `python -I 🔮️oracles/🚦️check-outcome/🐍️.py <snapshot>`.

## Files
- `🏛️en1992|🔩️en1993|🧩️en1994/…/✳️any/🔮️oracles/🚦️check-outcome/🐍️.py` (three new), `…/🔮️oracles/🔣️.json` (three edited)
- `…/✳️any/🧪️tests/🚦️check-outcome-en1992-1|en1993-1|en1994-1/{🥒️.feature,🐍️.py,🦀️.rs}` (nine new)
- `🗑️generated/doc-oracles/{rows.py,gen-cases.py,dry-run.py,rows19*.txt}` (scripts and row inputs; no tool output kept)
- `📌️important/requests.md`, `🗑️generated/build-en1992|1993|1994/latest.md` (appended)
