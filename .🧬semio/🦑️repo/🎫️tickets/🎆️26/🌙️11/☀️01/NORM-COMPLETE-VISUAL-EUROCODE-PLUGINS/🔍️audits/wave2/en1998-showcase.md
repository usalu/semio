# EN1998-SHOWCASE (wave 2)

No cargo/rustc/`repo run`, no git write. `- ready for rebuild: en1998 showcase` appended to `🗑️generated/build-en1998/latest.md`.

## Builder `🧰️scripts/showcase/en1998.py` (`--check` exit 0, all three outputs unchanged)
- Site `importanceClass` "ii" (was invalid "iI"), `annex` "De" (snapshot schema spelling). The old showcase was hand-patched and its stale values won the additive merge, so it was removed (backup `🗑️generated/en1998-showcase/old-showcase.json`) and regenerated from the builder alone.
- `piers[]` helper with all fields: reinforcementRatio, coverM 0.08, C30/500, axialLoadN, stiffnessBasis; pier-1 declared (rho 0.010, N 7 MN), pier-2 computed (rho 0.008, N 8 MN). Bridge: importanceClass 2, verticalPeriodS 0.3, pierCrackedInertiaRatio 0.5 (used by the declared pier).
- Second footing `fd-2` (steel hall, soil layers l5, l6) so `tieBeams[]` has `tb-1` fd-1 to fd-2 (12 m, 0.4 x 0.6, 12 cm2 at 434.8 MPa, displacement 0.01 of 0.03). `slopes[]` `sl-1` (layer l1, 8 m at 30 deg, phi 32, c 5 kPa, S_T 1.2, circle (4, 12) r 13).
- Wall water on rw-1 (1.5 m, saturated 20 kN/m3, permeability 1e-6 m/s); foundation fd-1 baseFrictionAngleDeg 25, basePorePressurePa 10 kPa (fd-2 identical).
- Assessments: as-1 KL2/SD, as-2 KL3/NC (with shearForceDemandN), new as-3 KL1/DL on the setback block (CF 1.35 path).
- Irregular in elevation: building `bldg-setback` (4 storeys, upper two lighter, elevationRegular false, setbackRatio 0.15 of the 0.20 limit, storeyStiffnessBasis declared 0.5).
- Also corrected the steel hall so the showcase is compliant apart from the deliberate placeholder building (q0 3.5 x 1.0, column class 1, given period 0.5 s).
- Python oracle (`🔮️oracles/🚦️check-outcome/🐍️.py`) on the showcase: only `bldg-pending.planRegularity` exceeds 1 (the not-yet-modelled annex, by design). tb-1.tie 0.7245, sl-1.slope 0.8384, rw-1.water 0.0441, setback 0.75.

## Outputs
- `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, `🚦️check-outcome/seismic-showcase-pass` (same document) and `-fail` (office base shear resistance 1e5, setback 0.35, tie steel 4 cm2 and displacement 0.04, slope phi 24 c 0, wall resistance 25 kN/m with 4 m water, footing delta 5 deg with 60 kPa pore pressure and little passive). jsonschema Draft 2020-12 on the snapshot schema: 0 errors for all three.
- 21 new rows `pass-78..90`, `fail-91..98` in `🧪️tests/🚦️check-outcome-en1998-1/🥒️.feature` (tolerance 1e-6, values from the Python oracle; each asserted <= 1 for pass rows and > 1 for fail rows while generating).
- DSL: `🛠️asset-laws/patch.py en1998 --apply` run (reads the Rust `#[dsl(key)]` spellings): the four assets gained the typed ENUM headers, new scalar and column cells, and the empty collections (soil-layers, diaphragms, accelerograms, piers, bearings, joints, tie-beams, slopes). Idempotent afterwards. No showcase DSL asset was emitted: en1998 buildings use the nested block form for storeys that `dslfmt.emit` does not print and a hand-made one cannot be round-trip verified without cargo; it also needs a palette example (Rust).

## Gates
`--check` rc 0; fixture checker (`kind-convention-v4/verify_fixtures.py en1998`, via heavy.sh): 335 scenarios, 327 inverted, 0 failures; feature dry run of `check-outcome-en1998-1` (heavy.sh): 98 ok, 0 failed (77 old rows + 21 new).

## Open
- Palette example and showcase DSL asset for the showcase (Rust, next build); the DSL packs need regeneration post-build.
- Goldens of tests pinned to the old showcase (periods, check counts) may need re-baselining; pier-2 stays computed.
