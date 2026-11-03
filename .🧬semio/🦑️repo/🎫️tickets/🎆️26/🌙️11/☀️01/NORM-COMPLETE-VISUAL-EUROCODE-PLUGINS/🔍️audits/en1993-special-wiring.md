# EN 1993 Special-Part Wiring (special-wiring agent)

`A = ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any`, `C = A/🧬️schema/💡️inferences/⚖️checks`. Status: **saved, not built** (the primary EN 1993 agent is the sole builder; "ready for rebuild: special-parts wiring" is appended to `🗑️generated/build-en1993/latest.md`). Every edited Rust file was parsed with `rustc +nightly-2026-07-20 -Zunpretty=normal` (syntax only); nothing was type-checked. Expect first-compile fixes in `C/🔗️special-element-checks/🔭️special-parts-checks`.

## Design decisions

- No new collections and no new mutation kinds: the inputs are **188 appended leaves of ten existing entities**. The whole-entity `update-*-inputs` kinds already exist for all ten, and `mutations::resolve` turns every `collection[id=…].leaf` set-field into exactly that update. So there is one set-field path per editable leaf without a new `EDIT_RULES` row. The test `every_special_part_leaf_resolves_to_its_whole_entity_update` (generated list of all 188 paths) enforces it.
- Schema-first: the field spec is `🧰️scripts/en1993-special-spec.py` (name, type, default, showcase value, unit, EN/DE label, choices); `🧰️scripts/en1993-special-gen.py` (+ `en1993-special-showcase.py`) appends it to every site. No migration is left behind; the generator is idempotent and the fixtures are fixed in the same change.
- Per-entity check units are unchanged (`En1993Units`): the new checks run inside the existing per-row loops, so `evaluate_units == check_full_steel_structure` (the fold law) and the read audit still hold. Default subjects (all new fields at their defaults) emit no new check, so existing reports and the compliant example stay green.

## Appended snapshot fields (all SI: m, N, Pa, N·m, deg, K, degC, a)

| Entity (collection) | New leaves | Clause functions fed |
|---|---|---|
| `SteelMember` (members) 8 | `stainlessForming`, `designTemperature`, `toughnessStressRatio`, `radiationShift`, `stressShift`, `marginShift`, `strainRateShift`, `coldFormingRadius` | `stainless::*`, `toughness::*`, `misc::reference_temperature/cold_forming_shift` |
| `ColdFormedMember` 17 | `flangeWidth`, `lipLength`, `webHeight`, `webSlope`, `oppositeFlangeWidth`, `springFactor`, `cornerRadius`, `flangeStress`, `shearForce`, `supportReaction`, `supportLength`, `cripplingCategory`, `stiffenedSupport`, `bendingMoment`, `momentResistance`, `sheetingStiffness`, `requiredRestraintStiffness` | `cold_formed::*` |
| `PlatedPanel` 32 | stiffener (`stiffenerInertia/Area/Eccentricity/Alpha`), `compressionStress`, shear lag (`halfWidth`, `equivalentSpan`, `hogging`), patch loading (`patchType`, `patchForce`, `patchLength`, `patchOffset`, `flangeThickness/Width/Fy`, `longitudinalOffset/Inertia`, `patchMoment`, `patchMomentResistance`), M-V (`bendingMoment`, `plasticMoment`, `flangeMoment`, `shearForce`), transverse stiffener (`transverseStiffenerInertia/Torsion/Polar`, `flangeInducedK`, `flangeArea`), reduced stress (`stressX/Z`, `shearStress`, `alphaCr`) | `plated_ext::*`, `resistance::*` |
| `SiloShell` 17 | `length`, `fabricationClass`, `meridionalStress`, `hoopStress`, `shellShearStress`, `boundaryFactorAxial/Hoop/Shear`, silo (`wallFriction`, `planArea`, `planPerimeter`), tank (`liquidDensity`, `liquidHeight`, `courseElevation`), pipeline (`internalPressure`, `externalPressure`, `temperatureChange`) | `shell_stability::*`, `containment::*` |
| `SteelJoint` 38 | end plate/column web/beam (`plateFy`, `plateWidth`, `boltToWeb`, `extensionToFlange/Edge`, `alphaFactor`, `boltStressArea`, `boltGripLength`, `column*` x8, `beamFlangeThickness`, `compressionLever`, `momentDemand`, `beam/columnMomentResistance`, `beamInertia`, `beamSpan`, `bracedFrame`, `stiffnessRatio`, `transformationParameter`, `assumedClass`), base plate (`basePlateLength`, `concreteFcd/Modulus/AreaA0/AreaA1`, `jointCoefficient`, `anchorArea/Length`, `baseNormalForce`, `baseMoment`, `baseLeverLeft/Right`); joint kinds `endPlate`, `basePlate` | `joint_components::*`, `joints::bolt_fub`, `parts_2_6::base_plate_*` |
| `TensionComponent` 18 | cable (`partialFactor`, `cableLength/Weight/HorizontalForce/Modulus/Stress/UnitWeight`), rod (`threadFactor`, `rodStressArea`, `rodFu`), pin (`pinDiameter`, `pinPlateThickness`, `pinGap`, `pinOuterThickness`, `pinInnerThickness`, `pinFup/Fyp/PlateFy`) | `tension_components::*` |
| `BridgeFatigue` 10 | `heavyVehicleWeight`, `referenceWeight`, `observedTraffic`, `designLife`, `secondLaneTraffic/Weight`, `lambda1`, `lambdaMax`, `detailId`, `plateThickness` | `bridge_fatigue::*` |
| `TowerLeg` 21 | `latticeKind`, `slenderness`, mast (`diameter`, `height`, `bendingStiffness`, `massPerLength`, `strouhal`, `logDecrement`, `airDensity`, `meanWindSpeed`, `crossingRate`, `designLife`, `kFactor`, `kwFactor`, `lateralForceCoefficient`, `permittedAmplitudeRatio`, `ringMode`, `shellThickness`), guy (`guyArea/Angle/Length`) | `towers_masts::*` |
| `SteelPile` 10 | `environment`, `designLife`, `wallThickness`, `interlockBendingFactor`, `interlockStiffnessFactor`, `sectionModulus`, `shearArea`, `designMoment`, `designShear`, `interlockShear` | `sheet_piling::*` |
| `CraneRunway` 17 | `maxWheelLoad`, `loadFactor`, `railInertia`, `railFixed`, `flangeEffectiveWidth/Thickness`, `wheelEccentricity`, `torsionConstant`, `longitudinalStress`, `railRadius`, `railContactWidth`, `craneNu`, `craneKq`, `damageExponent`, `stressRange`, `detailCategoryStress`, `fatiguePhi` | `crane_runway::*` |

Sites updated per leaf (scripted, key order = struct order): snapshot Rust model + compliant-frame literals; schema twins (`snapshot` and root `🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `🟦️.ts`; JSON Schema carries EN/DE label, unit, display unit, step); diff patches (`🔺️diff/🦀️.rs` and the four twins); SQLite (`🗄️.sql` columns incl. IEEE-754 bit/class columns, Rust and TypeScript codecs, `FloatColumn`/`Ieee754Column` lists, column counts, string admission, exact-word test now `all_272_fields`); binary and text IO twins (`💾️binary 🟦️.ts`, `📝️text 🔗️.graphql`); field-meta EN/DE rows (188) + choice tables (stainless kind, patch type, fabrication class, sheet-pile environment, lattice kind, assumed class, joint kinds `endPlate`/`basePlate`); 215 JSON fixtures/assets (mutation vectors, sqlite fixture, two `snapshot.json`); the two `🗣️.dsl.semio` examples; the mutation unit test literals.

## Wired checks (ids, clause, remedies)

Every check is a `CheckResult` with utilisation, EN+DE explanation (with the intermediate quantities) and, when it fails, at least one remedy (`emit` guarantees a fallback). NDPs listed are cited with `.ndp(..)`, so a passing verdict becomes `UnverifiedBasis` until the table row is verified.

| Entity row (scope tag) | Check id | Clause | Remedies |
|---|---|---|---|
| cold-formed (`coldFormedMembers`) | `en1993.1-3.5.5.dist.<id>` | 1-3 5.5.3.2 (iterated χ_d) | lip length at least, thickness at least |
| | `en1993.1-3.6.1.5.<id>` / `6.1.7.<id>` / `6.1.11.<id>` | 6.1.5 shear, 6.1.7 crippling, 6.1.11 M+F | thickness at least |
| | `en1993.1-3.10.1.5.<id>` | 10.1.5 sheeting restraint (C_D series) | thickness, sheeting stiffness at least |
| member (`memberProperties`) | `en1993.1-10.t.<id>` | 1-10 2.1 Table 2.1 with T_Ed = T_md + shifts + ΔT_εcf (replaces the fixed 0 degC and the legacy table) | one-of sub-grades; `NotApplicable` if the S/sub-grade column is not encoded |
| | `en1993.1-4.5.5.<id>` | 1-4 5.5.4 (ρ, χ, k_y/k_LT, interaction, secant factor) | one-of sections |
| fire (`fireExposures`) | `en1993.1-4.fire.<id>` | 1-2 C.2 (k_0.2p,θ, grade 1.4301) | μ0 at most |
| plated (`platedPanels`) | `en1993.1-5.4.5.<id>`, `.3.sl.`, `.6.a|b|c.`, `.7.2.`, `.7.1.`, `.9.3.`, `.9.2.`, `.8.`, `.10.` | 1-5 4.5.4, 3.3, 6.2-6.5, 7.2, 7.1, 9.3.3, 9.2.1, 8, 10 | thickness / stiffener inertia / half width / flange area / α_cr at least or at most |
| shells (`siloShells`) | `en1993.1-6.8.5.x|theta|tau|int.<id>`, `.1-6.8.2.vm.<id>` | 1-6 8.5.2-8.5.5, 8.2 (class from `fabricationClass`) | thickness at least |
| | `en1993.4-1.5.2.ph|nx.<id>` | 4-1 Janssen p_h, n_x | thickness at least |
| | `en1993.4-2.6.<id>` | 4-2 tank course | thickness at least |
| | `en1993.4-3.5.t|vm|pcr.<id>` | 4-3 wall, restrained von Mises, collapse | thickness at least |
| joints (`joints`) | `en1993.1-8.6.3.mj.<id>` | 1-8 6.2.7 M_j,Rd (T-stubs, column web, row distribution) | plate thickness at least, or M_j,Ed at most |
| | `en1993.1-8.5.2.sj.<id>` | 1-8 5.2.2 S_j,ini class vs assumption | one-of class, plate thickness at least |
| | `en1993.1-8.6.2.base.<id>` | 1-8 6.2.8 base plate M_Rd (Table 6.7 equilibrium) | plate thickness at least |
| tension (`tensionComponents`) | `en1993.1-11.6.2.cable.<id>`, `en1993.1-8.3.6.rod.<id>`, `en1993.1-8.3.13.pin.<id>`, `.pinb.<id>` | 1-11 6.2, 1-8 3.6.1 / 3.13 | F_uk, stress area, pin diameter, plate thickness at least |
| bridge (`bridgeFatigue`) | `en1993.2.9.5.<id>` | 2 9.5.2 (λ1..λ4, ladder, size effect) | category at least |
| tower (`towerLegs`) | `en1993.3-1.5.lambda.<id>`, `en1993.3-2.vortex.<id>`, `.ovalling.<id>` | 3-1 5.4, 3-2 5.2/5.3 | slenderness at most, damping at least, shell thickness at least |
| pile (`piles`) | `en1993.5.5.2.m|v|il.<id>` | 5 5.2.2 (corrosion, β_B/β_D, shear reduction) | thickness at least |
| crane (`craneRunways`) | `en1993.6.5.7.vm.<id>`, `en1993.6.9.fat.<id>` | 6 5.7 (σ_oz, τ, torsion, von Mises, Hertz in the text), 6 9 | web thickness, category at least |

Fixed duplicates: `joints::k5_end_plate` (0.85) deleted, the test uses `joint_components::k5_end_plate_bending` (0.9, Table 6.11; expected k5 9.9668, k_eq 3.9464, S_j,ini 51.80e9 recomputed). `plates_shells::patch_loading` deleted, the test uses `plated_ext::patch_resistance(PatchType::Through, ..)`; the check picks type a/b/c from `patchType`. `parts::part_1_10` (legacy table) deleted.

## Tests, oracle and features

- `C/🧪️tests/🔭️special-wiring/🦀️.rs` (mounted in `C/🦀️.rs`): all 45 ids present once for the showcase; finite utilisation, bilingual text and a remedy for every failure; unverified basis blocks `complies()`; default subjects emit no special-part id; hand-formula values (S355 J2 at -20 degC 17/55, tank course, web shear, sheet-pile interlock); fold == full evaluation and read audit for the showcase.
- `C/🧮️clauses/🧪️tests/🔭️special-parts/🦀️.rs` `sp08_wired_checks_match_independent_oracle` and `A/🔮️oracles/🧮️special-parts/🐍️.py` `wired()`: six utilisations from numpy/scipy formulations (interp tables, quad Janssen integral), tolerance 1e-9; the oracle runs with `/home/deck/semio/.venv/bin/python -I` and returns `w_tank 0.062617…`, `w_brittle 0.309091…`, `w_pile_il 0.271056…`, `w_cf_shear 0.065681…`, `w_silo_ph 0.029694…`, `w_crane_vm 0.303772…`.
- `A/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` `every_special_part_leaf_resolves_to_its_whole_entity_update` (188 paths).
- `A/🧪️tests/🔭️special-parts/🥒️.feature`: language-agnostic vectors for the 92 clause quantities (`sp07` keys with tolerances), the six wired check utilisations and the presence/remedy/unverified contract.
- Third-party validation: all 164 snapshot JSON fixtures/assets validate against the updated `snapshot/🔣️.json` with `jsonschema` (venv). Nothing was compiled or run in Rust.

## Unverified normative values (in addition to `en1993-special-parts.md`)

- Lattice slenderness limits 120/180/250, ovalling criterion `1.25 v_m <= v_crit,i` (3-1/3-2), gamma_Ff = 1.0 for bridge/crane fatigue, tank courses as plain hydrostatic membrane, sheet-pile section loss scaling `W_net = W_el t_net/t`, `kt`-free pin double shear `F_v,Ed = F/2`, crane `tau_oxz = 0.2 sigma_oz` plus St-Venant torsion, stainless effective area by simplified I-section outstand/internal elements with k_σ 0.43/4.0 and class-3 `W_el` resistances, base plate effective width `t_fc + 2c` with the user footprint as upper bound of A_c0, end plate rows without group-limit reduction (`group = inf`), `cap = min(F_c,wc, V_wp/β, M_b/z)`.
- 1-10: the default inputs (T_md 0 degC, σ_Ed/f_y 0.75, no shifts) deliberately keep the pre-existing uncited verdict, otherwise the compliant example would turn into `UnverifiedBasis`; any explicit toughness input cites `ndp.en1993.l402.1`.
- 30 cited NDP ids are not in `NDP_TABLE` (listed in `requests.md`).

## Remaining gaps

- Not compiled or run (build owner: primary agent); `.feature` runner binding for the new feature file is not written (the scenarios are executed by the Rust tests above).
- Not modelled (no snapshot field yet): hollow-section joints, weld group torsion, chart-based `alpha(λ1,λ2)`, 1-5 Annex A/D, 1-6 external-pressure/ring stiffened cases, 4-1 hoppers, stainless grades other than 1.4301 in the fire table, 1-10 stress levels other than 0.75 and grades above S355 (reported `NotApplicable`, not passed).
- Row distribution of end plates models all rows with equal geometry (first row extended, further rows "first below"); individual row geometries need a child table.
