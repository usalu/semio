# Oracle wiring en1998

## Counts
- Blessed cases: 17 in 3 files (5 distinct checks), all 5 checks wired, 0 gap cases, 0 gap quantities.
- Quantities compared: 53 expected values.
- Known differences registered: 0.

## Adapters
- en1998.3.2.2.5.design-spectrum: `part_1::elastic_response_spectrum` (S_e, eta = 1) and `part_1::design_spectrum_horizontal` (S_d, beta fixed 0.2 in our function, equal to the case input).
- en1998.4.3.shear-building-modal: `part_1::shear_building_modes` (periods ascending in omega, so T_1 is the fundamental) and `part_1::effective_modal_masses`; Gamma_1 = (sum m phi / sum m phi^2) phi_top (invariant under scale and sign), m_eff_1_fraction = M_eff,1 / sum m.
- en1998.3.1.vs30: `part_1::vs30_from_layers` divides by the total thickness of the layers it gets (documented to sum to 30 m); the oracle averages over the top 30 m of profiles that are 35 to 42 m thick, so the adapter clips the profile to 30 m first. The class index is `ground_class_from_vs30` mapped A..D to 0..3. Observation: the clause function itself does not clip; a profile deeper than 30 m would give a wrong v_s,30 (not an open request).
- en1998.4.3.5.2.nonstructural: `part_1::non_structural_force_n` with alpha = a_g / 9.81 (as `check_full_seismic` does); S_a is recovered as F_a q_a / (W_a gamma_a) because no function returns S_a alone.
- mechanics.lateral-frame: `semio_s_plugin_norm_mechanics` (`Analysis::solve_case`), fixed-base single bay in the X-Z plane, forces at the left joints, same model as the en1993 portal wiring; u_i at the left floor nodes, drift_i differences, V_base = |Rx| of both bases, base moments from `force[4]`. No en1998 clause function solves a frame; the kernel is the owner.

## Files changed
- New: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🎯️oracle-wiring/🦀️.rs`
- Edited: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` (mount `#[cfg(test)] mod oracle_wiring;`), `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` (dev-dependency `semio-s-plugin-norm-mechanics`), `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/Cargo.toml` (`[workspace.dependencies.semio-s-plugin-norm-mechanics]`, same path as en1993). `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/Cargo.lock` will gain the mechanics package on the next cargo run.

## Mount choice
The harness test sits in the artifact root crate (manifest above the oracle-cases folder); the registry is mounted in `semio-s-artifact-norm-en1998-schema` (manifest below the leaf, already has `compliance-testing` as dev-dependency).

## Unverified compile points
- Nothing was built. The mechanics `Model`, `Support`, `NodalLoad`, `StaticResult` field names are copied from the en1993 wiring (`displacements[].node/u`, `reactions[].node/force`); the plane and the use of `force[4]` for the base moment follow it too.
- `shear_building_modes` and `effective_modal_masses` are in `part_1`; `m` and `k` are read through `nums`.
- Lateral-frame expectations use 5 storey names at most 6 (arrays `u_1..u_6`).
