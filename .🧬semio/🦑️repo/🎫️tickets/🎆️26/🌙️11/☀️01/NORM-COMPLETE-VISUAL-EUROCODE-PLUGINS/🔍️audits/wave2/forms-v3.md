# FORMS-V3 (wave 2)

Status: UNCOMPILED. No cargo, rustc or build was run. Python only: `geometry_gen.py` write and `--check` green for all six codes, `source-boundary-scan.py` (no crate reported), the coverage audit below. `- ready for rebuild: forms v3` is appended to `integrate` and `build-en1990/1991/1992/1995/1996/1997` `latest.md`; entry in `📌️important/requests.md`. No git writes. CRATE-BUDGET G had not designated a home for new form code (no entry in `requests.md`), so the new code sits in its own folder `forms/🕹️grips`, which moves with the forms.

## 1. Mechanism (shared forms, `projection-kit/🗂️design-case/🧊️forms`)
- `GripKind` (new `🕹️grips/🦀️.rs`, re-exported from `forms`): `Scalar`, `Face`, `Spacing` (linear drag on a length input), `Count { min, max }` (stepper, identity map, `Quantise::Int`), `Gauge { k, step, limits, follow }` (`stored = value + k * s` on any numeric input, optional `Follow { key, base, k, step }` inputs written in the same release, e.g. area and inertia). `Grip.face: bool` became `Grip.kind: GripKind`; `Plan.face` likewise; `profiles::apply` maps the extra grips; `handles_with` sends count and gauge plans to `grips::quantity` (written by `design_case::input_path`, so every drag is one commit of `change-case-number` writes). A grip whose input has no leaf is dropped (as before for sizes); a follower without leaf is skipped.
- Generic form grips that never existed and now do (all six codes share them): Fastener `count`/`rows` steppers, Pile `rows`/`columns` steppers, soil `layers` thickness grips (Footing, Pile, SoilBlock; one per declared layer, series key), Slope and GroundAnchor `angle` (angular, zero-direction convention checked by reading `angular_s`), Slope `ratio` gauge (toe = h x ratio).

## 2. EN 1995 restored (`🧩️cases/🦀️.rs`, `case_extras`)
| Entry | Grip tag (kind) | Input(s) written |
|---|---|---|
| T-90 | `a_i` (gauge, width of part i) | `a_i` and `i_i` together (A = w h, I = w h^3/12 in the same release) |
| T-90 | `s_i` (spacing) | `s_i` |
| T-93 | `n_parts` (count 1..4), `area` (gauge, post side) | `n_parts`, `area` (tangent law, quadratic in the side) |
| T-94 | `s` (spacing), `n_panels` (count 1..12), `h` (wall height) | same keys |
| T-121 | `n_k` per block (gauge, tower height = damage), `years` (gauge, scales all towers) | `n_k`, `years` |
| T-122 | `bar_spacing` (spacing), `plate_a` (gauge, anchor plate side) | same keys |
| T-120 | `crowd_density` (gauge, 0.5 persons/m2 per metre, max 2.5) | `crowd_density` |
| T-101/102/106 | `t_req` (gauge on the char front of the first exposed face, minutes per metre from the kernel char rate), `face_left/right/top/bottom` (steppers 0..1) | same keys |
| T-107 | `t_req` (gauge on the CLT char front), `n_layers` (count 1..9) | same keys |
| T-95, T-96 | `span`, `height`, `bays`, chord and web sizes; `n_braced`, `spacing`, `member_b/h` | only when those detail inputs exist (see 5) |

Already present and kept: `h_i` and `l1` and `lef`, `deck_depth`, size grips of the roles, `taper-angle`, `apex-angle`, `apex-radius`, notch grips, `ply-n`.

## 3. Tests (all written, none run)
- Forms unit (`🧪️tests/🔬️unit/🦀️.rs`, toy catalogue): `the_fastener_count_and_row_steppers_write_the_integer_count_of_their_input`, `the_pile_group_steppers_write_rows_and_columns`, `the_slope_and_anchor_angle_grips_write_degrees_about_their_zero_direction` (also asserts that the anchor stands at the current angle), `the_slope_ratio_grip_moves_the_toe_by_the_height_times_the_ratio`, `extra_count_spacing_and_gauge_grips_bind_their_input_and_followers` (kinds, maps, follower, no leaf no grip, body carries the grip ids). Toy SL entry gained the `ratio` role. The `Grip` literal of `assembled()` uses `GripKind::Scalar`.
- EN 1995 scene unit (`🎬️scene/🧪️tests/🔬️unit/🦀️.rs`): restored `the_width_handle_of_a_jointed_part_writes_area_and_inertia_consistently` (new id `jb:a1`, drag delta 0.01 m gives 5600 mm2 and 140 x 40^3/12 mm4, no drag no change), `every_restored_count_spacing_and_gauge_grip_binds_its_own_input` (one assertion per restored grip: first write path is the input leaf, handle laws, steppers are integer identity, spacings widen outwards, density/years/cycles/t_req scale from their value). Helper document `catalogue_grips()` adds T-94 and T-101 cases to `special_document()`.
- Language-agnostic/third-party: grips have no new numeric kernel; the area/inertia law is the closed form A = w h, I = w h^3/12 (checked by hand in the test, same as the removed test). The forms oracle (`🔮️oracle`) was not extended.

## 4. Audit of the six codes (script `🧰️scripts/forms-v3/coverage.py`, `audit.py`, `show.py`)
Declared grip roles = non-empty width/length/height/load/aux keys of the geometry rows (appearance roles `grain`, `ply_dir` excluded). "Gap" = a declared role whose form offers no grip.

| Code | Entries | Declared roles | With grip before v3 | With grip after v3 | Gaps after |
|---|---|---|---|---|---|
| EN 1990 | 30 | 26 | 26 | 26 | none |
| EN 1991 | 97 | 139 | 139 | 139 | none |
| EN 1992 | 174 | 338 | 338 | 338 | none |
| EN 1995 | 70 | 181 | 178 | 181 | none (T-75/T-76/T-123 `count` closed) |
| EN 1996 | 40 | 97 | 97 | 97 | none |
| EN 1997 | 71 | 168 | 154 | 168 | none (7 `layers`, 6 `angle`, PILE-06 rows/columns, EMB-01 ratio closed) |

FORMS-V2 changed only the EN 1995 pipeline; for EN 1990/1991/1992/1996/1997 the grips before and after FORMS-V2 are identical (they use `draw`/`handles` without extras), so none has fewer grips than before. Note: the pre-change geometry declarations are not in git HEAD (the declarations were created in this ticket and are untracked), so the comparison is against the declared roles themselves and the FORMS-V2 report; HEAD was not usable as a baseline.
EN 1995 geometry declaration added: T-16 and T-17 `width: b`, `height: h` (they declared the section inputs but drew a nominal member); `🦀️.rs` regenerated.
Remaining geometric inputs (lengths, angles, minutes) that no role draws, hence no grip because there is nothing in the picture to anchor to (per code: EN 1990 17 inputs in 10 entries, EN 1991 46/28, EN 1992 193/90, EN 1995 49/29, EN 1996 34/19, EN 1997 103/38; full lists by running `coverage.py`): EN 1995 examples are T-74 end and edge distances `a3t/a3c/a4t/a4c`, T-70/75/76/80 `alpha_deg` (load-to-grain angle), T-79 `d0/e1/e2`, T-78 `he`, T-13 `l1`, T-15 `a_support`, T-20..23 `lef_y/lef_z`, T-31 `t_lam`, T-33 `r_in` (no apex angle input, so no profile), T-46 `h_ef` (no notch length input), T-60..62 `h_a/h_b/w_c/h1..3/s1/s3`, T-91/T-92 flange sizes (a T-section is no box form), T-107 `protect_mm`, T-93 `i_tot/i1`. They need a new form role (e.g. an end-distance dimension line or a T-section form) before a grip is meaningful; none was invented here. Non-geometric design parameters (strengths, factors, stiffnesses `ei_l/ei_b`, masses, `zeta`, stress ranges `ds_k`) have no anchor and no gauge.

## 5. Open items and risks
- T-95 and T-96: the catalogue entries declare only `n_d`, `l_m`, `product`, `q_rd`/`f_rd`, `k_br`; the truss/bracing details (`span`, `bays`, `n_braced`, ...) exist only on the typed entity rows (which keep their entity-engine grips). The case extras for them (and their new grips) are dead for catalogue cases until the inputs are declared; "T-95 bays, T-96 count/spacing" were therefore never lost for catalogue cases. No test asserts them.
- `T-122 contact_mm` has no input in the catalogue and is not drawn: no grip.
- Gauge grips with a quadratic law (T-93 `area`, T-122 `plate_a`) use the tangent at the current size; the pointer and the redrawn edge differ by the second-order term until the next redraw.
- Angular grips follow the world zero direction (+X projected), as the existing taper grip does; a yawed case (non-zero `yaw`) shifts the zero. Unchanged convention, noted.
- The `Spacing` kind is a scalar drag in the stored delta convention (`stored = value + factor * per_metre * s`), not the absolute `DimensionLine` of the old entity engine, because the form handles are delta based (the forms tests fix `map.base = value`).
- Expected first-run compile points: `Plan.kind` replacing `face` in `handles_with` (the `match plan.kind` on a non-Copy enum with unit patterns), `use super::*` reaching the private imports (`HandleSpec`, `handle_kit`) from `grips`, closure coercion `&clt_depth(c)` to `&dyn Fn(f64) -> f64`, two-phase borrows of `draft.footprint.0` in `Draft::count(.., [draft.footprint.0, ..])`, `scene::{HandleKind, Quantise, Affine}` paths in the new scene test.
- Goldens: the case scene goldens of all codes change again (new handle ids on the bodies, T-16/T-17 sizes): `SEMIO_UPDATE_GOLDEN=1` after the rebuild.

## 6. Files
Shared: `projection-kit/🗂️design-case/🧊️forms/{🦀️.rs,🕹️grips/🦀️.rs (new),🪵️profiles/🦀️.rs,🌍️ground/🦀️.rs,🔩️joints/🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`.
EN 1995: `🗿️artifacts/🪵️en1995/…/✳️any/🎬️scene/🧱️entities/🧩️cases/🦀️.rs`, `🎬️scene/🧪️tests/🔬️unit/🦀️.rs`, `🧬️schema/⚖️kernel/📇️catalogue/🗂️cases/📐️geometry/{🔣️.json,🦀️.rs}`.
Ticket: `🧰️scripts/forms-v3/{audit,coverage,keys,show}.py`, `📌️important/requests.md`, `🗑️generated/{integrate,build-en1990,build-en1991,build-en1992,build-en1995,build-en1996,build-en1997}/latest.md`, this report.
