# 🪶 EN 1999 Norm-Completion Report

Artifact: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999`. `SUB` = `ART/🏅️standards/🔖️1/🪆️subsets/✳️any`. Spec: `📐️specs/en1994-en1999-target.md` (A-01..A-81). Inventory: `🔍️audits/en1999-inventory.md`.

**Status.** All code, tests, the independent oracle and the vectors are written, but NOTHING HAS BEEN COMPILED OR RUN IN RUST YET: the host capacity queue (27 requests ahead, critical memory pressure) held the build for more than 35 minutes. Only the Python side ran (oracle on both examples, 16 shared vectors: all pass). See section 7 for what the next agent must run. Items that are blocked by the snapshot schema or by unverifiable table values are in sections 5 and 6; they are not hidden behind placeholder utilisations.

## 1. Design

- `SUB/🧬️schema/💡️inferences/⚖️checks/📏️parts` is now split into pure clause modules. `🔩️alloys-and-sections` is `part_1_1`, `🔥️fire` is `part_1_2`, `🔄️fatigue` is `part_1_3`, `🧩️joints` is `joints` and `🫙️thin-wall` is `part_1_4_1_5`. `🦀️.rs` holds `na_de` (annex parameters and the NDP registry) and `part_en1990` (combinations).
- Every check set is evaluated for every EN 1990 combination (6.10a and 6.10b for each leading variable action, serviceability combinations, fire). `worst_by_id` keeps the highest utilisation per check id. The old heuristic "largest sum of absolute components" is gone.
- No snapshot schema field was added. Schema twins, SQLite files and the 18 committed mutation vectors are therefore untouched. Reasons are in section 6.
- The check ids and their clauses are listed in section 2.

## 2. Implemented catalogue ids

| Spec id | Check ids (suffix `.<member|connection|scenario|detail|sheet|shell>`) | Clause |
|---|---|---|
| A-01 | `alloy` bands in `ALLOY_BANDS` (6082-T6 has two thickness bands; `resolve_alloy_for_thickness`); unknown alloy gives `en1999.3.2.alloy.*` Fail with `OneOf` | 3.2 |
| A-02, A-03 | HAZ strips per welded element (`haz_width`: 20/30/35/40 mm; strip `[w_pos − b, w_pos + b]` clipped to the element); ρo,haz in compression/bending, ρu,haz in tension and welds; `en1999.8.6.haz` for the connected plate | 6.1.6, 8.6 |
| A-04 | `en1999.6.1.4.class.<m>.<el>` per element; Table 6.2 limits by buckling class and weld state; stress-gradient η from the real N/M stress state; round tubes by β = 3√(D/t). A class 4 element is a Warning (effective section applies), not a Fail | 6.1.4 |
| A-05 | ρc with Table 6.3 constants C1/C2 per class/weld/outstand (continuity at β3 is a unit test); per-element effective thickness; centroid shift e_N for class 4 | 6.1.5 |
| A-10, A-11, A-12 | `en1999.6.2.3.n` tension (yield on the HAZ section, ultimate 0.9·A·fu/γM2 or ρu,haz) and compression (`A_eff`) | 6.2.3 |
| A-14 | `en1999.6.2.5.m`, `.mz` with α (class 1/2 `Wpl/Wel` capped, class 3 interpolation, class 4 `Weff`), HAZ via the strip model | 6.2.5 |
| A-15 | `en1999.6.2.6.v`, `.vy` (flange area for V_y), `en1999.6.2.8.mv` (bending with shear, ρ_V) | 6.2.6, 6.2.8 |
| A-17 | `en1999.6.2.9.nm` (replaces `6.2.8.nm`): class 1/2 exponent form with a data-driven exponent table, class 3/4 linear form with eccentricity ΔM = N·e_N | 6.2.9 |
| A-20, A-21 | `en1999.6.3.1.fb`, `.fb.z` (λ̄ from `A_eff`, curve α/λ̄0 per buckling class, κ for longitudinal welds, ω_x function `omega_x`), `en1999.6.3.1.tb` (N_cr,T with warping, α of the class, λ̄0 = 0.2) | 6.3.1 |
| A-22 | `en1999.6.3.2.ltb` (own curve: class 1/2 α = 0.10, λ̄0 = 0.6; class 3/4 α = 0.20, λ̄0 = 0.4; corrected I_w) | 6.3.2 |
| A-23 | `en1999.6.3.3.int` Eq. 6.59/6.60 with η_c, γ_c, ξ_c | 6.3.3 |
| A-25 | `en1999.6.7.4.vb` web shear buckling ρ_v (λ̄w = 0.35·hw/tw·√(fo/E)), unstiffened over the member length | 6.7.4 |
| A-30, A-31, A-33 (long joints) | `en1999.8.5.bolt` (shear and bearing, bolts and rivets), `.tension`, `.vt`, `.punch`, `.pitch`, `.edge`, `.gauge`, `.emax`, `.pmax`, `.block`, `en1999.8.5.grade` Fail for unknown grades, β_Lf | 8.5 |
| A-32 | `en1999.8.6.weld` (weld metal and HAZ fusion boundary), `.length`, `.haz`; unknown connection kind is now `en1999.8.kind` Fail (was a silent not-applicable) | 8.6 |
| A-80 | `en1999.7.2.defl` (quasi-permanent, load-based `k·M·L²/(E·I_eff)`, limit from the NDP), `.stress`, `.sls-freq`, `en1999.8.sls-freq` | 7.2 |
| A-40, A-41 | `en1999.1-3.fat` (Palmgren–Miner D ≤ 1 drives the verdict), `.fat.range` (γFf·Δσ ≤ Δσ_R(N)/γMf); S–N corner stresses; unknown categories Fail | 1-3 |
| A-50, A-51, A-52 | `en1999.1-2.fire` (cross-section at k0.2proof,θ with γM,fi), `.fire.buckling` (λ̄θ = λ̄·√(k0.2/kE)), `.fire.ltb`, `.fire.theta-cr` (critical temperature from μ0) ; member temperature by time stepping (c_a = 0.41θ + 903, αc = 25, εres = 0.3, Δt ≤ 5 s) | 1-2 |
| A-60 (flat element), A-61 (bending, shear) | `en1999.1-4.local`, `.axial`, `.bend` (W = b·t²/6, HAZ), `.shear`, `.nm` | 1-4 |
| A-70 | `en1999.1-5.buckle`, `.ring`, `.shear`, `.int` (σx,Rcr with C_x for short/medium/long cylinders, σθ,Rcr = 0.92·E·(t/r)/ω, τRcr = 0.75·E·(t/r)/√ω, imperfection α_x from fabrication class B, interaction exponents) | 1-5 |

## 3. Fixed defects (inventory numbers)

1 interaction formulas (6.2.9, 6.3.3 now exponent forms). 2 per-element ρc with effective thickness and HAZ strips (`per_element_reduction_is_not_a_global_minimum`). 3 classification checks all three limits and reports the class. 4 λ̄ uses `A_eff`. 5 own LTB, torsional and flexural curves. 6 V_y uses the flange shear area. 7 weak axis uses `W_eff,z`. 8 shear buckling by 6.7.4. 9 α_v by grade. 10 bolt resistance is the minimum of the modes (no pitch factor). 11 the lever 0.5 m is replaced by bolt-group geometry (Σz², Σr). 12 weld `·10` and the clamps are gone. 13 deflection from the load model. 14 SLS limits come from `AnnexParams` (NDP registry). 15 fire: tabulated k, γM,fi, time stepping, section with local buckling/HAZ, buckling in fire. 16, 17, 28 fatigue verdict by Miner sum, no moment scaling, no 0.9/0.1 blend, no `(1 + 0.02·m2/m1)`. 18, 19, 20 cold-formed modulus, ρ and shear area. 21 shell constants are named functions with the formulas of the shell rules. 22 N_cr,T includes warping. 24 web slenderness uses the clear height. 26, 27 the `lcr` and `lambda.*` checks were removed (they could not fail or used a project limit). 29 edge distance is a check. 30 unknown kind fails. 31 the oracle test no longer skips unmatched ids. 32 γG, γQ, ξ are NDPs. 33 `HAZ_ZONE_M` deleted. 34 dead snow branch removed. 35 `is_non_tube_od` removed. 36 no hard-coded section ids in remedies; remedies change fields that exist. 37 the `hazExtent → 0` remedy was removed. 39 grade list is `joints::FASTENER_GRADES`. 40 6082-T6 follows E2-01. 41 unknown fatigue category fails. 46 the oracle test uses the shared interpreter resolver (`semio_s_plugin_norm_compliance::python_oracle`, `SEMIO_PYTHON`, `-I`); nothing calls `python3` any more. Also fixed: `I_w` of the I section was 4× too small; the channel `I_w` was the I formula; box `I_t` used the wrong denominator; G is now 27 000 MPa (was E/2.66).

## 4. Tests

- Unit tests with the spec examples E2-01..E2-12: `SUB/🧬️schema/🧪️tests/⚖️compliance/🔬️resistance-worked-examples/🦀️.rs`.
- Language-agnostic vectors: `SUB/🔮️oracles/⚖️compliance/📐️worked-examples/🔣️.json` (16 vectors). Rust reads them in `📐️worked-example-vectors/🦀️.rs`; the Python oracle evaluates the same file (`--vectors`). The Python result was run: no failures.
- Independent oracle `SUB/🔮️oracles/⚖️compliance/🐍️.py` (stdlib only): slice integration for section properties, Ayrton–Perry root finding for χ, endurance by segment search. The test fails on any oracle id without a Rust counterpart and on any Rust id of the families `6.`, `7.`, `8.`, `1-` that the oracle does not cover. Coverage: classification, N, M, V, M+V, N+M, flexural/torsional/LTB buckling, 6.3.3, 6.7.4, SLS, bolts, welds, HAZ, fire (4 checks), fatigue (2), sheets (5), shells (4).
- Gate parity list updated to the ids that fail in the non-compliant example (23 ids).
- Leaf-influence test: `weldPosition` is perturbed to the opposite element edge; hollow-section torsion/LTB leaves and the second S–N slope above the knee are documented as physically inactive.
- No `.feature` was added: the repo feature platform needs a subject and an oracle adapter pair, which cannot be validated without running it; the shared JSON vectors play that role and run inside the Rust test target.

## 5. Unverified normative values

All of these are in code as data or named functions and are visible in the NDP registry (`na_de::NDP_REGISTRY`, `confirmed: false`) or here.

- Table 3.2: every row except 6082-T6 is the repo value (6060/6061/6063/5083 unverified, 6061 and 6063 differ from the spec seed). 6082-T6 follows the spec (E2-01), which replaces the old repo ρ = 0.64/0.73. np, ductility and product-form columns are not modelled.
- Table 6.2 limits with welds, Table 6.3 C1/C2 (internal consistency verified: ρc = 1 at β3 for all 8 rows), round-tube rule β = 3√(D/t) with the internal limits.
- Class 1/2 shape factor cap α_max = 1.25.
- κ for longitudinal welds (κ0 = A_eff,haz/A_eff, linear to 1 at λ̄ = 1.2), ω_x exponent c, torsional curve (λ̄0 = 0.2), η0 = γ0 = ξ0 = 0.8 with the lower bound 1 (conservative), section interaction exponents (1, 1, 1).
- Shear buckling ρ_v rule; stress-gradient treatment; MIG HAZ widths; TIG and T-junction factors are not modelled.
- Bolts: α_v = 0.5 for 10.9, stress areas, d0 = d + 2/3 mm, punching d_m = 1.5·d, rivet f_ur (only `rivet-6082` = 160 MPa), stainless 700 MPa; weld filler strengths 190/240/270 MPa (repo values), γMw = 1.25.
- Fire: k0.2proof,θ and kE,θ rows (6xxx-T6 seed), γM,fi = 1.0, εres = 0.3, αc = 25.
- Fatigue: Annex J category rows 14..125, γMf (DE 1.35, EN 1.0), γFf = 1.0, `-weld` suffix alias.
- Shells: fabrication class B (Q = 25), hoop/shear imperfection parameters 0.65/0.60/0.40, interaction exponents (taken from the shell rules of EN 1993-1-6), C_x with clamped-edge assumption.
- Deflection limit L/200, stress limit 0.8·f_o (project value, not in §7.2), both NDPs.
- The compliant example had to change to stay compliant: fire start temperature 20 °C and duration 30 s (the old 180 °C/600 s is not survivable for an unprotected profile), shell hoop stress 12 MPa, bolt gauge 18 mm, sheet point load 0. Assets are regenerated with `REGEN_EN1999_ASSETS=1`.

## 6. Remaining gaps and why

- **Schema-blocked** (a new field would change 5 schema twins, the SQLite files, the diff/mutation vocabulary and about 70 committed mutation vectors that must be produced by production dispatch): A-06 stiffened elements, A-16 torsion (no T_Ed), A-19/A-27 local loads (no load-introduction length), A-28 stiffeners and tension field (no stiffener spacing: webs are treated as unstiffened over the member length), A-21 localized-weld ω_x wiring (no weld location on the member; the function `omega_x` is implemented and unit tested), A-72 welded shells (no weld flag), A-62 sheeting from tests (Annex A data), A-71 stiffened shells, TIG/T-junction HAZ factors (no process flag), shell fabrication class.
- **Table-blocked** (values must be transcribed from the standard, the spec forbids guessing): A-23 Annex H method B, A-61 web crippling and its combination, exact η0/γ0/ξ0 and ψ/η/ξ exponent tables, α_max table, κ table, full Table 3.2 with all product forms.
- A-33 adhesives and pin joints are not modelled. A-81 is out of scope (creep negligible).
- A `.feature` adapter pair was not written (see section 4).

## 6b. Follow-up rounds (coordinator requests)

**Schema extension (done, uncompiled).** Twelve fields were added to the snapshot and carried through every artifact layer by hand and by script:

| Entity | New fields |
|---|---|
| `PlateElement` | `weldProcess` (`mig`/`tig`), `weldTJunction`, `weldDirection` (`longitudinal`/`transverse`), `edgeStiffenerDepth`, `edgeStiffenerThickness` |
| `MemberAction` | `torsionK`, `localLoadK` |
| `AluminiumMember` | `stiffenerSpacing`, `bearingLength`, `transverseWeld`, `transverseWeldOffset` |
| `AluminiumShell` | `welded`, `fabricationClass` (`A`/`B`/`C`) |

Layers touched: Rust model and constructors; snapshot and root JSON Schema (properties, required, `x-semio-ui`), GraphQL, proto and TS twins; SQLite `.sql`, Rust codec, TS codec, borrowed native admission (slot ids), SQLite tests (word oracle 64 → 77 float cells, `maxColumns` 28 → 36, semantic contract widths and schema bytes 10086 → 11807, fixture JSON); field-meta (EN and DE); 54 committed mutation fixtures plus the SQLite fixture; the Python oracle and the shared vectors (19 now). Existing whole-collection mutations carry the new fields; no new mutation kinds were added. The diff crate got explicit `by Key` list markers (the E0117 fix announced in build-loop.md).

The checks now run from the document: HAZ width from process and T-junction, transverse welds put the whole element into the HAZ, a complete edge stiffener turns an outstand into an internal element, shear buckling uses the stiffener spacing, new checks `6.2.7.t`, `6.2.7.vt` (torsion), `6.7.5.patch` (patch load with bearing length), ω_x from the transverse weld distance, shell HAZ and fabrication class Q, and validation checks `en1999.ref.element.process/direction/stiffener.*` and `en1999.ref.shell.class.*`.

**Review fixes (review-en1998-en1999.md).** Per-element η (webs only; an RHS flange keeps its own class), transverse-weld HAZ plate check with full width (longitudinal strips only when `hazExtent` > 0), alloy rows 6061/6063 reset to the spec seeds, alloy band by the weakest wall thickness of the section (and by plate thickness for connections), characteristic deflection plus a quasi-permanent one, SLS stress limit demoted to a Warning, fatigue refuses m1 ≤ 0, L_cr taken as given (k only when derived), plastic-theory exponents for 6.2.9, public `member_resistances` and `check_units`. Regression tests: `rhs_flange_in_bending_keeps_its_own_class`, `check_units_group_a_report_by_entity`, `torsion_and_patch_loads_come_from_the_document` and others in `🔬️resistance-worked-examples`.

**Placement, free section polygon, explicit weld runs, product form/temper pair**: not added (visual-side requests in `requests.md`); they need new entities (`placements`, `outline`/`holes`, `weldRuns`), mutation kinds and fixtures and are the next vertical.

**Still table-blocked**: Annex H method B, web crippling for cold-formed sheets, exact η0/γ0/ξ0, α_max and κ tables, full Table 3.2 (all registered as unverified).

## 7. Builds and results

Nothing is compiled yet. Builds 1 to 7 (`🗑️generated/build-en1999/*.txt`) ended without rustc diagnostics: ingress/`check` (cancelled), source boundary in `semio-s-plugin-norm-app-surface` (peer, fixed later), `Admitted Cargo artifact plan Source changed` (peer saved a file), resource loan deadline, Nx graph publication contention, `Insufficient physical compiler initial headroom`, graph-plan SIGKILL (out of memory). Build 7 was cancelled when the central runner took over; the request `ready for rebuild` is in `build-en1999/latest.md` and waits for the runner (it was working on EN 1990).

Python side (run): 19 shared vectors pass; the oracle evaluates both examples after the constructor changes with no utilization above 0.93 for the compliant one and 46 failing checks for the non-compliant one.

Next steps once the runner reports: fix rustc errors; `:test` with `REGEN_EN1999_ASSETS=1` once (assets and packs of both examples), then without; update `🧪️tests/🧫️fixtures/🎛️semantic.json` `valueBytes` from the first failing semantic-limits test (rows 17 and 1 are unchanged); delete `🗑️generated/en1999/`.
