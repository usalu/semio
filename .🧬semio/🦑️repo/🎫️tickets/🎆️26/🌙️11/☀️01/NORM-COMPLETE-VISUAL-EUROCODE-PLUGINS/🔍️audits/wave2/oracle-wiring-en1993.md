# Oracle wiring EN 1993

New test module: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs` (crate `semio-s-artifact-norm-en1993-checks`), test `evaluators_reproduce_the_blessed_oracle_cases`. The old harness test (`third_party_libraries_reproduce_the_blessed_oracle_cases`) is untouched. Nothing was compiled or run.

## Counts
- Cases: 47 in 4 case files; 36 cases sit on a registered check, 11 on an unregistered check.
- Checks wired: 25 adapters (25 of 34 distinct check ids; 9 ids unregistered).
- Known differences: 0. Eq. 5.2 against eigenvalue is a property of the blessed cases and both our evaluators reproduce their own blessed side (`alpha_cr_storey` = Eq. 5.2 with kernel sway, `alpha_cr_portal_fixed` = eigenvalue); the `[open] oracle -> en1993` note (requests.md ~577) is informational.
- Cases with at least one gap: 18 (11 unregistered + 7 partial).

## Wired checks (evaluator)
- mechanics.portal-sway, mechanics.beam-continuous: `semio_s_plugin_norm_mechanics::Analysis::solve_case` (model built from inputs: X-Z plane, fixed bases / pinned+roller, beam split at midspan).
- en1993.5.2.alpha-cr-storey: `clauses::global::alpha_cr_storey` with kernel sway; en1993.5.2.alpha-cr-eigen: `global::alpha_cr_portal_fixed` (V/2 per column); en1993.5.2.alpha-cr: `alpha_cr_storey`.
- en1993.5.3.2.sway-imperfection: `global::sway_phi`, `sway_equivalent_force` (phi, H_eq).
- en1993.1-8.4.5.weld-group-box: `weld_group::group_properties` + `max_force_per_length` (scale-free, SI in, SI out).
- en1993.section.i-rolled: `rolled_heb_catalogue` row matched by h, b, tf, tw, r; en1993.section.rhs-sharp: `parametric_sections::rhs`; en1993.section.chs: `parametric_sections::chs`.
- 6.7 nu-rd `n_u_rd`; 6.10 nc-rd and 6.2 npl `n_pl_rd`; 6.14 class 3 `part_1_12::elastic_bending_resistance_nm`; 6.2 mc-rd `part_1_1::bending_resistance_nm`; 6.32 `m_n_rd_solid_rect`; 6.39 mnyrd `m_n_rd_rhs`; vpl-rd `v_pl_rd`; av-rolled-i/rhs-depth/chs/channel `av_rolled_i/av_rhs/av_chs/av_rolled_u`; my-v-rd `m_y_v_rd_i` (min with given M_y,c,Rd); my-n-rd `m_n_y_rd_i`; mb-rd `part_1_1::ltb_resistance_nm`.

## Gaps (no evaluator entry point; not faked, test fails by design)
Unregistered checks (11 cases):
- en1993.section.tee (en1993.section.tee-t150x150): A, Iy, Iz, It, Wpl_y, Wpl_z. Suggested owner API: `parametric_sections::tee(h, b, tf, tw, r) -> Properties` with It, Iw, Wpl.
- en1993.section.mono-i (2 cases): A, Iy, Iz, It, Iw, Wpl_y, Wpl_z. Suggested `parametric_sections::mono_symmetric_i(h, b_t, b_b, t_ft, t_fb, tw, r)`.
- en1993.section.ehs (1): A, Iy, Iz, It. Suggested `parametric_sections::ehs(d_x, d_y, t)`.
- en1993.section.rhs (1, rounded corners r_out): A, Iy, Iz, It, Wel_y/z, Wpl_y/z. Suggested `parametric_sections::rhs_rounded(h, b, t, r_out)` (existing `rhs` is sharp-corner only).
- en1993.section.channel (1): A, Iy, Iz, It, Wpl_y/z, Iw, y_sc. Suggested `parametric_sections::channel(h, b, tf, tw, r)`.
- en1993.section.angle (1): A, Iy, Iz, I_major, I_minor, It, Wpl_y. Suggested `parametric_sections::angle(h, b, t, r_root, r_toe)`.
- en1993.6.3.2.mcr-i (2): M_cr, Iz, It, Iw of a rolled I from h, b, tf, tw, r (IPE300 is not in the HEB catalogue; `stability::mcr_closed_form` needs Iz/It/Iw). Suggested `parametric_sections::rolled_i(h, b, tf, tw, r) -> Properties` (with It, Iw) and an `mcr_rolled_i` wrapper.
- en1993.6.39.aw-hollow (1): a_w = min((A - 2 b t)/A, 0.5). Suggested `resistance::a_w_hollow(a, b, t)`; today the web fraction exists only inline in `part_1_1::mn_interaction_eta` for I sections.
- en1993.6.21.tau-i (1): tau_Ed for I/H from V_Ed, A_w, A_f. Suggested `resistance::tau_ed_i(v_ed, a_w, a_f)`.

Partial (checks registered, quantity missing; the driver reports each):
- en1993.5.3.2.sway-imperfection (3 cases): alpha_h. Suggested `global::sway_alpha_h(h)` and `global::sway_alpha_m(m)` split out of `sway_phi`.
- en1993.section.rhs-sharp (2 cases): Wel_z, Wpl_z. Suggested extend `parametric_sections::Properties` with `w_el_z`, `w_pl_z`.
- en1993.section.chs (1): It, Wel_z, Wpl_z. Same `Properties` extension plus `it`.
- en1993.section.i-rolled (en1993.section.ipe300): all quantities, the HEB catalogue has no IPE 300 row. Suggested IPE catalogue rows or `parametric_sections::rolled_i`.

## Files changed
- Created `.../⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs`
- Edited `.../⚖️checks/🦀️.rs` (mount `#[cfg(test)] mod oracle_wiring`)
- Edited `.../⚖️checks/📦️packages/🦀️rust/Cargo.toml` (dev-dependency `semio-s-plugin-norm-mechanics`; the compliance dev-dependency with `compliance-testing` already existed). `Cargo.lock` will gain the edge on the next build.

## Assumptions / unverified compile points
- `mech::Section`, `Member`, `Support`, `LoadCase`, `Model`, `DistributedLoad` are used with `..Default::default()`; `Section` is assumed to derive `Default` (its derive line was read for `Node`; the unit test of the kernel builds `Section` with all fields).
- `crate::clauses`, `crate::part_1_1`, `crate::part_1_12`, `AnnexParams` reachable from the crate root through the glob re-exports (same as the existing compliance tests; `part_1_12` is new to a test).
- `AnnexParams::en()` needs the NDP registry; the existing tests call it too.
- Kernel sign conventions: reactions upward-positive (as in the kernel unit tests); sway taken from `u[0]` of node `b`; moments by absolute value.
- Tolerance risk: HEB catalogue values are rounded table entries compared under numerical-v1 (5e-3); `alpha_cr_portal_fixed` vs the scipy eigenvalue under method-spread-v1 (3e-2).
