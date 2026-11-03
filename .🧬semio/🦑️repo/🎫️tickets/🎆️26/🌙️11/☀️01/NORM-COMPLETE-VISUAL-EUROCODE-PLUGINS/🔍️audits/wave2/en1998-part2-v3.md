# EN 1998-2 bridges v3 (wave 2): computed pier effective stiffness

Nothing compiled or run in Rust (no cargo/rustc/`repo run`). One Python oracle run (concreteproperties).

## 1. Schema (via `evolve.py add-leaf en1998 piers`, one leaf at a time, dry run first)
New pier fields (all in snapshot, diff, SQLite DDL and codecs, field meta EN+DE, edit rules, fixtures, mutation kinds `change-pier-*`): `reinforcementRatio` (A_s/(b h), 0..0.04), `coverM` (bar centre d1), `concreteFckMpa`, `steelFykMpa`, `axialLoadN` (compression positive; no per-pier axial load is derived anywhere, so it is an input), `stiffnessBasis`. No transverse reinforcement: confinement is not modelled by the kernel (no Mander), so it would be an unused input.
- `stiffnessBasis` is the typed enum `PierStiffnessBasis {Declared, Computed}` (wire `declared`/`computed`). The tool refuses enum leaves, so: String leaf through add-leaf (needed a one-line tool fix, `l_leaf.py` evaluated `float(default)` eagerly for every type), then `typed-enums/convert.py --spec specs/en1998-pier-stiffness-basis.json` (new spec) converted it in all twins, codecs, kinds and fixtures. `Declared` keeps the bridge share `pierCrackedInertiaRatio` (BR-05 v2 check unchanged, now only for bridges with a declared pier).
- Units: the tool knows no `MPa` unit; fields carry it in the name like `modulusMpa`.
- Hand fixes: `demo-cases` InsertPier literal, structures test literal, showcase `pier-2` set to Computed (rho 0.008, d1 0.08, C30, 500, N 8 MN).

## 2. Computation
- `part_3::RcMember::pier(..)` (rectangular, A_s split into two layers at the cover) and `effective_stiffness_nm2` = M_y/phi_y from `section_response` (shared `section::fibre` kernel, registry strain laws of v16; yield = first of tension steel f_y/E_s and eps_c2).
- `bridge::{pier_section, pier_gross_ei_nm2, pier_ei_ratio, has_supports}`; `bridge::supports` takes the `AnnexA` and uses the ratio per pier and direction (longitudinal depth h, transverse depth b), capped at the gross section (cap = `cracked_ratio`), 0 when the section has no response (pier without stiffness, check fails, no silent gross fallback).
- Consumers: `bridge-spectrum::analyse` (new `annex` argument, caller in `part-evaluators`) so periods, forces, deck displacement, pier top displacement and the pier-capacity diagram (through `scene-structures::bridge_view`, `Site.section`) all use the same supports. New check `en1998.2.<b>.pier.<p>.stiffness` (EN 1998-2 2.3.6.1) reports EI_eff, EI_eff/EI_g per direction, fails with remedy on `reinforcementRatio`. BR-05 trace updated (check suffixes `.action.stiffness`, `.stiffness`).
- Not normative: f_ck/f_yk are used as entered (characteristic as strength of the laws), the standard's Annex C text is not in the repo; any factor (design vs expected strengths) is the designer's input.

## 3. 3D, grips, diagram
Pier colour field `pier-cracked-share` now carries EI_eff/EI_gross (computed) or the declared share, legend and detail text say so; grip `<p>:rho` on `reinforcementRatio` (0..0.04) replaces `<p>:cracked` for computed piers; diagram `pier-moment-curvature:<pier>` (curve, yield and ultimate markers, secant of slope EI_eff, table twin, EN+DE).

## 4. Tests
- `🧪️tests/🏗️structures`: EI_eff = M_y/phi_y vs the oracle (N = 0 and 1 MN, 1.5 percent), compression stiffens, supports of mixed bases (3 EI_eff/H^3), missing reinforcement fails the stiffness check with a remedy.
- Oracle `🔮️oracles/🌉️pier-stiffness/🐍️.py` (concreteproperties, first steel yield found by bisection on the fixed-step moment-curvature curve through `calculate_service_stress`). Library values (0.8x1.2 m, rho 0.008, d1 0.08, C30, 500): N = 1 MN phi_y 0.0029943 1/m, M_y 2465.27 kNm, EI_eff 8.2333e8 N m2; N = 0 phi_y 0.0027977, M_y 1883.92 kNm, EI_eff 6.7339e8.
- Scene unit test (colour, grip, diagram, check); feature rows: `schema-en1998-1` (oracle row, both bases), `scene-en1998-1`, `mutate-en1998-1` (6 kinds in both tables).

## 5. Gates
spec-trace en1998 68 ok; i18n-mutations ok (3100 leaves; the six new kinds had DE `pier: ... ändern`, now `Pfeiler: ... ändern`); twins strict, schema-sync, handle-paths, cargo authority, build-owners sync: ok after every apply (one transient schema-sync fail on v17 tieBeams order, resolved). Final run after all edits: twins strict, schema-sync, handle-paths, cargo authority, build-owners sync ok; fixture checker en1998 335 scenarios, 327 inverted, 0 failures; binary tags come with the tool's mutation layer (no separate script found).

## Unverified
Compile errors possible in new Rust (bridge, checks, scene, tests). The 1.5 percent tolerance vs the library is a guess (v16 used 0.3 percent for phi_y; the oracle's yield detection interpolates). Showcase pier-2 computed changes showcase periods; tests pinned to the showcase may need re-baselining. `🔮️oracles/🔣️.json` was not extended with the concreteproperties pier oracle entry.

- ready for rebuild: en1998 part 2 v3
