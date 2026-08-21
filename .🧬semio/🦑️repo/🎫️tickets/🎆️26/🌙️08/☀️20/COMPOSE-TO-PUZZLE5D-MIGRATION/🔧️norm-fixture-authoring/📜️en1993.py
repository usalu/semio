"""📘️ EN 1993 — 17 hand-authored mutation fixture cases over the flat design-check input sheet."""
import copy
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from importlib import import_module

common = import_module("📜️common")
REPO = common.REPO

ROOT = os.path.join(REPO, "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")


def camel(name):
    """🐫️ serde's `RenameRule::CamelCase`: PascalCase, then lowercase the first character."""
    pascal = "".join(part[:1].upper() + part[1:] for part in name.split("_"))
    return pascal[:1].lower() + pascal[1:]


# 🧱️ The committed before-snapshot: one S355 rolled member sheet with every EN 1993 part's inputs
# populated, so each of the 17 field groups has something real to move.
BASE = {
    "annex": "De",
    "n_ed_kn": 500.0,
    "m_ed_knm": 150.0,
    "v_ed_kn": 80.0,
    "a_mm2": 5000.0,
    "a_v_mm2": 2500.0,
    "w_pl_mm3": 500000.0,
    "f_y_mpa": 355.0,
    "f_u_mpa": 510.0,
    "chi": 0.75,
    "a_net_mm2": 4250.0,
    "tension_n_ed_kn": 400.0,
    "fire_thickness_mm": 20.0,
    "fire_rating": "r60",
    "fire_massivity": 150.0,
    "fire_mu_0": 0.5,
    "fire_design_temperature_c": 550.0,
    "cf_b_bar_mm": 90.0,
    "cf_t_mm": 2.0,
    "cf_k_sigma": 4.0,
    "cf_psi": 1.0,
    "cf_n_ed_kn": 20.0,
    "cf_gross_resistance_kn": 50.0,
    "stainless_m_ed_knm": 40.0,
    "stainless_w_pl_mm3": 300000.0,
    "stainless_f_y_mpa": 220.0,
    "plated_lambda_p": 0.75,
    "plated_sigma_ed_mpa": 200.0,
    "silo_t_mm": 8.0,
    "silo_r_mm": 3000.0,
    "shell_sigma_x_ed_mpa": 150.0,
    "silo_k": 0.5,
    "silo_gamma_kn_m3": 18.0,
    "silo_depth_m": 5.0,
    "bolt_f_ed_kn": 120.0,
    "bolt_n_bolts": 2,
    "bolt_a_s_mm2": 245.0,
    "bolt_e1_mm": 40.0,
    "bolt_e2_mm": 40.0,
    "bolt_d0_mm": 22.0,
    "bolt_d_mm": 20.0,
    "bolt_t_mm": 10.0,
    "bolt_f_u_mpa": 510.0,
    "bolt_f_ub_mpa": 800.0,
    "weld_a_mm": 5.0,
    "weld_l_mm": 100.0,
    "weld_f_u_mpa": 510.0,
    "weld_steel_grade": "S355",
    "weld_f_ed_kn": 80.0,
    "delta_sigma_mpa": 50.0,
    "fatigue_category": 71,
    "fatigue_method": "damage_tolerant",
    "t10_steel_subgrade": "J2",
    "t10_actual_thickness_mm": 25.0,
    "t10_t_ed_c": 0.0,
    "tension_component_f_uk_kn": 500.0,
    "tension_component_f_k_kn": 350.0,
    "tension_component_n_ed_kn": 250.0,
    "hss_w_el_mm3": 400000.0,
    "hss_f_y_mpa": 460.0,
    "hss_section_class": 2,
    "hss_m_ed_knm": 100.0,
    "bridge_lambda": 1.0,
    "bridge_phi_2": 1.0,
    "bridge_delta_sigma_p_mpa": 30.0,
    "tower_wind_factor": 1.25,
    "tower_n_ed_kn": 300.0,
    "pile_sigma_mpa": 280.0,
    "pile_k_red": 0.875,
    "pile_n_ed_kn": 400.0,
    "crane_f_z_ed_kn": 50.0,
    "crane_wheel_contact_length_mm": 100.0,
    "crane_dispersion_mm": 50.0,
    "crane_t_w_mm": 10.0,
}
assert len(BASE) == 74, len(BASE)

BEFORE = {camel(key): value for key, value in BASE.items()}

DIFF_FIELDS = ["artifact"] + list(BASE.keys()) + ["selected_check_index"]
DIFF_NULL = {camel(field): None for field in DIFF_FIELDS}

APPLIED = {"status": "applied"}

CASES = []


def case(leaf, variant, name, summary, changes, extra_applied, extra_diff):
    after = copy.deepcopy(BEFORE)
    diff = copy.deepcopy(DIFF_NULL)
    payload = {}
    for field, value in changes.items():
        assert field in BASE, field
        assert BASE[field] != value, field
        after[camel(field)] = value
        diff[camel(field)] = value
        payload["new_" + field] = value
    CASES.append(
        dict(leaf=leaf, case=name, summary=summary, after=after, mutation={variant: payload}, diff=diff, extra_applied=extra_applied, extra_diff=extra_diff)
    )


KIND = {
    "🎢change-annex": ("change-annex", "change_annex"),
    "🧊update-member-properties": ("update-member-properties", "update_member_properties"),
    "🔆update-fire-inputs": ("update-fire-inputs", "update_fire_inputs"),
    "🔥update-cold-formed-inputs": ("update-cold-formed-inputs", "update_cold_formed_inputs"),
    "🎚️update-stainless-inputs": ("update-stainless-inputs", "update_stainless_inputs"),
    "🧭update-plated-inputs": ("update-plated-inputs", "update_plated_inputs"),
    "🪟update-silo-shell-inputs": ("update-silo-shell-inputs", "update_silo_shell_inputs"),
    "🌞update-bolt-inputs": ("update-bolt-inputs", "update_bolt_inputs"),
    "🧱update-weld-inputs": ("update-weld-inputs", "update_weld_inputs"),
    "❄️update-fatigue-inputs": ("update-fatigue-inputs", "update_fatigue_inputs"),
    "🌬️update-through-thickness-inputs": ("update-through-thickness-inputs", "update_through_thickness_inputs"),
    "⚡update-tension-component-inputs": ("update-tension-component-inputs", "update_tension_component_inputs"),
    "💧update-hss-inputs": ("update-hss-inputs", "update_hss_inputs"),
    "🏗️update-bridge-inputs": ("update-bridge-inputs", "update_bridge_inputs"),
    "🌗update-tower-inputs": ("update-tower-inputs", "update_tower_inputs"),
    "🗺️update-pile-inputs": ("update-pile-inputs", "update_pile_inputs"),
    "🌡️update-crane-inputs": ("update-crane-inputs", "update_crane_inputs"),
}

# ── 1. change-annex ───────────────────────────────────────────────────────────
case(
    "🎢change-annex",
    "ChangeAnnex",
    "switches-the-national-annex-from-de-to-en",
    "The document's lone identity scalar flips from the German national annex to plain EN. This is "
    "the only `change-<field>` in EN 1993's vocabulary — every other mutation updates a whole "
    "part-scoped input group. The builder's sole guard is the `mutation.no-op` equality check; "
    "`De != En`, so a one-field diff is published.",
    {"annex": "En"},
    '''    assert_eq!(snapshot.annex, crate::document::AnnexChoice::En, "change-annex/switches-the-national-annex-from-de-to-en: the annex must resolve to EN");
    assert_eq!(snapshot.f_y_mpa, before().f_y_mpa, "change-annex/switches-the-national-annex-from-de-to-en: switching annex must not silently restate any design input");''',
    '''    assert_eq!(raised_diff.annex, Some(crate::document::AnnexChoice::En), "change-annex/switches-the-national-annex-from-de-to-en: the diff must publish annex = En");
    assert!(raised_diff.n_ed_kn.is_none(), "change-annex/switches-the-national-annex-from-de-to-en: the annex choice is not a member force");''',
)

# ── 2. update-member-properties ───────────────────────────────────────────────
case(
    "🧊update-member-properties",
    "UpdateMemberProperties",
    "re-grades-the-base-member-to-s460-under-a-heavier-load",
    "All eleven base-member fields move as one: the design forces rise (N 500→650 kN, M 150→200 kNm, "
    "V 80→110 kN), the section grows (A 5000→6000 mm2, Av 2500→3000 mm2, Wpl 500000→640000 mm3, "
    "Anet 4250→5100 mm2), the grade rises to S460 (fy 355→460 MPa, fu 510→540 MPa), buckling "
    "reduction chi 0.75→0.875 and the tension force 400→520 kN. This is EN 1993's widest diff: "
    "eleven `Some`s among 76 fields.",
    {
        "n_ed_kn": 650.0,
        "m_ed_knm": 200.0,
        "v_ed_kn": 110.0,
        "a_mm2": 6000.0,
        "a_v_mm2": 3000.0,
        "w_pl_mm3": 640000.0,
        "f_y_mpa": 460.0,
        "f_u_mpa": 540.0,
        "chi": 0.875,
        "a_net_mm2": 5100.0,
        "tension_n_ed_kn": 520.0,
    },
    '''    assert_eq!(snapshot.f_y_mpa, 460.0, "update-member-properties/re-grades-the-base-member-to-s460-under-a-heavier-load: the yield strength must be S460's 460 MPa");
    assert_eq!(snapshot.w_pl_mm3, 640000.0, "update-member-properties/re-grades-the-base-member-to-s460-under-a-heavier-load: the plastic section modulus must be 640000 mm3");
    assert_eq!(snapshot.chi, 0.875, "update-member-properties/re-grades-the-base-member-to-s460-under-a-heavier-load: the buckling reduction factor must be 0.875");
    assert_eq!(snapshot.hss_f_y_mpa, before().hss_f_y_mpa, "update-member-properties/re-grades-the-base-member-to-s460-under-a-heavier-load: the hollow-section part keeps its own independent fy");''',
    '''    assert_eq!(raised_diff.n_ed_kn, Some(650.0), "update-member-properties/re-grades-the-base-member-to-s460-under-a-heavier-load: the diff must publish nEdKn = 650");
    assert_eq!(raised_diff.tension_n_ed_kn, Some(520.0), "update-member-properties/re-grades-the-base-member-to-s460-under-a-heavier-load: tensionNEdKn belongs to this group and must be published too");
    assert!(raised_diff.stainless_f_y_mpa.is_none(), "update-member-properties/re-grades-the-base-member-to-s460-under-a-heavier-load: the stainless part's fy is a different group and must stay untouched");''',
)

# ── 3. update-fire-inputs ─────────────────────────────────────────────────────
case(
    "🔆update-fire-inputs",
    "UpdateFireInputs",
    "raises-the-fire-protection-to-r90",
    "EN 1993-1-2's whole fire facet moves together: the required rating goes r60→r90, the protection "
    "thickness 20→32 mm, section-factor massivity 150→175 1/m, the load level mu0 0.5→0.625 and the "
    "design temperature 550→620 degC. `fire_rating` is one of only four `Option<String>` fields in "
    "this diff.",
    {"fire_thickness_mm": 32.0, "fire_rating": "r90", "fire_massivity": 175.0, "fire_mu_0": 0.625, "fire_design_temperature_c": 620.0},
    '''    assert_eq!(snapshot.fire_rating, "r90", "update-fire-inputs/raises-the-fire-protection-to-r90: the required fire rating must be r90");
    assert_eq!(snapshot.fire_thickness_mm, 32.0, "update-fire-inputs/raises-the-fire-protection-to-r90: the protection thickness must be 32 mm");
    assert_eq!(snapshot.fire_design_temperature_c, 620.0, "update-fire-inputs/raises-the-fire-protection-to-r90: the design temperature must be 620 degC");''',
    '''    assert_eq!(raised_diff.fire_rating.as_deref(), Some("r90"), "update-fire-inputs/raises-the-fire-protection-to-r90: the diff must publish fireRating as the string r90");
    assert_eq!(raised_diff.fire_mu_0, Some(0.625), "update-fire-inputs/raises-the-fire-protection-to-r90: the diff must publish the load level fireMu0 = 0.625");
    assert!(raised_diff.t10_t_ed_c.is_none(), "update-fire-inputs/raises-the-fire-protection-to-r90: the through-thickness reference temperature is a different part's input");''',
)

# ── 4. update-cold-formed-inputs ──────────────────────────────────────────────
case(
    "🔥update-cold-formed-inputs",
    "UpdateColdFormedInputs",
    "thickens-the-cold-formed-flange-and-reverses-its-stress-gradient",
    "EN 1993-1-3's effective-width inputs move as one: the flat width 90→120 mm, sheet thickness "
    "2.0→2.5 mm, buckling factor kSigma 4.0→0.5 (an outstand rather than an internal element), the "
    "stress ratio psi 1.0→-1.0 (pure bending), the design force 20→28 kN and the gross resistance "
    "50→64 kN. `cf_psi` going negative is the one field here that legitimately takes a negative "
    "value.",
    {"cf_b_bar_mm": 120.0, "cf_t_mm": 2.5, "cf_k_sigma": 0.5, "cf_psi": -1.0, "cf_n_ed_kn": 28.0, "cf_gross_resistance_kn": 64.0},
    '''    assert_eq!(snapshot.cf_b_bar_mm, 120.0, "update-cold-formed-inputs/thickens-the-cold-formed-flange-and-reverses-its-stress-gradient: the flat width must be 120 mm");
    assert_eq!(snapshot.cf_psi, -1.0, "update-cold-formed-inputs/thickens-the-cold-formed-flange-and-reverses-its-stress-gradient: the stress ratio must be -1.0 for pure bending");
    assert_eq!(snapshot.cf_k_sigma, 0.5, "update-cold-formed-inputs/thickens-the-cold-formed-flange-and-reverses-its-stress-gradient: the buckling factor must be the outstand value 0.5");''',
    '''    assert_eq!(raised_diff.cf_t_mm, Some(2.5), "update-cold-formed-inputs/thickens-the-cold-formed-flange-and-reverses-its-stress-gradient: the diff must publish cfTMm = 2.5");
    assert_eq!(raised_diff.cf_psi, Some(-1.0), "update-cold-formed-inputs/thickens-the-cold-formed-flange-and-reverses-its-stress-gradient: the diff must carry the negative stress ratio verbatim, not clamp it");
    assert!(raised_diff.plated_lambda_p.is_none(), "update-cold-formed-inputs/thickens-the-cold-formed-flange-and-reverses-its-stress-gradient: plate slenderness is EN 1993-1-5's own group");''',
)

# ── 5. update-stainless-inputs ────────────────────────────────────────────────
case(
    "🎚️update-stainless-inputs",
    "UpdateStainlessInputs",
    "upsizes-the-stainless-section-to-a-duplex-grade",
    "EN 1993-1-4's three stainless fields move together: the design moment 40→56 kNm, the plastic "
    "modulus 300000→384000 mm3 and the proof strength 220→460 MPa (austenitic 1.4301 → duplex "
    "1.4462). Nothing outside the stainless group is republished.",
    {"stainless_m_ed_knm": 56.0, "stainless_w_pl_mm3": 384000.0, "stainless_f_y_mpa": 460.0},
    '''    assert_eq!(snapshot.stainless_f_y_mpa, 460.0, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the stainless proof strength must be the duplex 460 MPa");
    assert_eq!(snapshot.stainless_w_pl_mm3, 384000.0, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the stainless plastic modulus must be 384000 mm3");
    assert_eq!(snapshot.f_y_mpa, before().f_y_mpa, "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the carbon-steel member's own fy must be untouched");''',
    '''    assert_eq!(raised_diff.stainless_m_ed_knm, Some(56.0), "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the diff must publish stainlessMEdKnm = 56");
    assert!(raised_diff.f_y_mpa.is_none(), "update-stainless-inputs/upsizes-the-stainless-section-to-a-duplex-grade: the base member's fy must NOT ride along in this diff");''',
)

# ── 6. update-plated-inputs ───────────────────────────────────────────────────
case(
    "🧭update-plated-inputs",
    "UpdatePlatedInputs",
    "makes-the-plate-panel-more-slender-and-more-stressed",
    "EN 1993-1-5's two-field group: plate slenderness lambda_p 0.75→1.25 (now past the plateau, so "
    "reduction bites) and the applied direct stress 200→275 MPa. The narrowest `update-*-inputs` "
    "diff in this artifact — exactly two `Some`s.",
    {"plated_lambda_p": 1.25, "plated_sigma_ed_mpa": 275.0},
    '''    assert_eq!(snapshot.plated_lambda_p, 1.25, "update-plated-inputs/makes-the-plate-panel-more-slender-and-more-stressed: the plate slenderness must be 1.25");
    assert_eq!(snapshot.plated_sigma_ed_mpa, 275.0, "update-plated-inputs/makes-the-plate-panel-more-slender-and-more-stressed: the applied direct stress must be 275 MPa");''',
    '''    assert_eq!(raised_diff.plated_lambda_p, Some(1.25), "update-plated-inputs/makes-the-plate-panel-more-slender-and-more-stressed: the diff must publish platedLambdaP = 1.25");
    assert!(raised_diff.shell_sigma_x_ed_mpa.is_none(), "update-plated-inputs/makes-the-plate-panel-more-slender-and-more-stressed: the shell meridional stress belongs to the silo/shell group");''',
)

# ── 7. update-silo-shell-inputs ───────────────────────────────────────────────
case(
    "🪟update-silo-shell-inputs",
    "UpdateSiloShellInputs",
    "deepens-the-silo-and-thickens-its-shell",
    "The six fields shared by EN 1993-1-6's shell-buckling check and EN 1993-4-1's silo-wall check "
    "move together, because they describe ONE physical silo: wall thickness 8→12 mm, radius "
    "3000→3500 mm, meridional stress 150→190 MPa, lateral pressure ratio k 0.5→0.625, stored-solid "
    "unit weight 18→22 kN/m3 and depth 5→8 m.",
    {"silo_t_mm": 12.0, "silo_r_mm": 3500.0, "shell_sigma_x_ed_mpa": 190.0, "silo_k": 0.625, "silo_gamma_kn_m3": 22.0, "silo_depth_m": 8.0},
    '''    assert_eq!(snapshot.silo_t_mm, 12.0, "update-silo-shell-inputs/deepens-the-silo-and-thickens-its-shell: the shell thickness must be 12 mm");
    assert_eq!(snapshot.silo_depth_m, 8.0, "update-silo-shell-inputs/deepens-the-silo-and-thickens-its-shell: the stored-solid depth must be 8 m");
    assert_eq!(snapshot.shell_sigma_x_ed_mpa, 190.0, "update-silo-shell-inputs/deepens-the-silo-and-thickens-its-shell: the meridional design stress must be 190 MPa");''',
    '''    assert_eq!(raised_diff.silo_r_mm, Some(3500.0), "update-silo-shell-inputs/deepens-the-silo-and-thickens-its-shell: the diff must publish siloRMm = 3500");
    assert_eq!(raised_diff.silo_gamma_kn_m3, Some(22.0), "update-silo-shell-inputs/deepens-the-silo-and-thickens-its-shell: the diff must publish siloGammaKnM3 = 22");
    assert!(raised_diff.tower_wind_factor.is_none(), "update-silo-shell-inputs/deepens-the-silo-and-thickens-its-shell: EN 1993-3's tower inputs are a separate group");''',
)

# ── 8. update-bolt-inputs ─────────────────────────────────────────────────────
case(
    "🌞update-bolt-inputs",
    "UpdateBoltInputs",
    "moves-the-connection-to-four-m24-grade-10-9-bolts",
    "All ten EN 1993-1-8 bolted-connection fields move as one — the group exists precisely because "
    "`bolt_e1_mm` alone is meaningless: shear force 120→200 kN over 4 rather than 2 bolts, M20→M24 "
    "(As 245→353 mm2, d 20→24 mm, d0 22→26 mm), end/edge distances 40→48 mm, ply thickness 10→16 mm, "
    "plate fu 510→540 MPa and bolt fub 800→1000 MPa. `bolt_n_bolts` is the group's only integer.",
    {
        "bolt_f_ed_kn": 200.0,
        "bolt_n_bolts": 4,
        "bolt_a_s_mm2": 353.0,
        "bolt_e1_mm": 48.0,
        "bolt_e2_mm": 48.0,
        "bolt_d0_mm": 26.0,
        "bolt_d_mm": 24.0,
        "bolt_t_mm": 16.0,
        "bolt_f_u_mpa": 540.0,
        "bolt_f_ub_mpa": 1000.0,
    },
    '''    assert_eq!(snapshot.bolt_n_bolts, 4, "update-bolt-inputs/moves-the-connection-to-four-m24-grade-10-9-bolts: the connection must carry four bolts");
    assert_eq!(snapshot.bolt_d_mm, 24.0, "update-bolt-inputs/moves-the-connection-to-four-m24-grade-10-9-bolts: the bolt shank diameter must be 24 mm");
    assert_eq!(snapshot.bolt_d0_mm, 26.0, "update-bolt-inputs/moves-the-connection-to-four-m24-grade-10-9-bolts: the hole diameter must be 26 mm");
    assert_eq!(snapshot.bolt_f_ub_mpa, 1000.0, "update-bolt-inputs/moves-the-connection-to-four-m24-grade-10-9-bolts: grade 10.9 means fub 1000 MPa");''',
    '''    assert_eq!(raised_diff.bolt_n_bolts, Some(4), "update-bolt-inputs/moves-the-connection-to-four-m24-grade-10-9-bolts: the diff must publish boltNBolts as the integer 4");
    assert_eq!(raised_diff.bolt_e2_mm, Some(48.0), "update-bolt-inputs/moves-the-connection-to-four-m24-grade-10-9-bolts: every one of the ten bolt fields is published, edge distance included");
    assert!(raised_diff.weld_a_mm.is_none(), "update-bolt-inputs/moves-the-connection-to-four-m24-grade-10-9-bolts: the welded-connection group is separate even though both live in EN 1993-1-8");''',
)

# ── 9. update-weld-inputs ─────────────────────────────────────────────────────
case(
    "🧱update-weld-inputs",
    "UpdateWeldInputs",
    "lengthens-the-fillet-weld-and-re-grades-it-to-s460",
    "EN 1993-1-8's fillet-weld group: throat 5→7 mm, effective length 100→160 mm, parent fu "
    "510→540 MPa, the grade string S355→S460 (which is what selects beta_w) and the design force "
    "80→140 kN. `weld_steel_grade` is an `Option<String>` in the diff, so it must appear as a JSON "
    "string, never as a number.",
    {"weld_a_mm": 7.0, "weld_l_mm": 160.0, "weld_f_u_mpa": 540.0, "weld_steel_grade": "S460", "weld_f_ed_kn": 140.0},
    '''    assert_eq!(snapshot.weld_steel_grade, "S460", "update-weld-inputs/lengthens-the-fillet-weld-and-re-grades-it-to-s460: the parent grade string must be S460");
    assert_eq!(snapshot.weld_a_mm, 7.0, "update-weld-inputs/lengthens-the-fillet-weld-and-re-grades-it-to-s460: the weld throat must be 7 mm");
    assert_eq!(snapshot.weld_l_mm, 160.0, "update-weld-inputs/lengthens-the-fillet-weld-and-re-grades-it-to-s460: the effective weld length must be 160 mm");''',
    '''    assert_eq!(raised_diff.weld_steel_grade.as_deref(), Some("S460"), "update-weld-inputs/lengthens-the-fillet-weld-and-re-grades-it-to-s460: the diff must publish weldSteelGrade as the string S460");
    assert_eq!(raised_diff.weld_f_ed_kn, Some(140.0), "update-weld-inputs/lengthens-the-fillet-weld-and-re-grades-it-to-s460: the diff must publish weldFEdKn = 140");
    assert!(raised_diff.bolt_f_ed_kn.is_none(), "update-weld-inputs/lengthens-the-fillet-weld-and-re-grades-it-to-s460: the bolted-connection force must not be republished");''',
)

# ── 10. update-fatigue-inputs ─────────────────────────────────────────────────
case(
    "❄️update-fatigue-inputs",
    "UpdateFatigueInputs",
    "drops-to-detail-category-56-under-a-safe-life-assessment",
    "EN 1993-1-9's three fields: the stress range 50→90 MPa, the detail category 71→56 (a `u8`, so "
    "the diff must carry a JSON integer) and the assessment method string `damage_tolerant`→"
    "`safe_life`, which is what selects gamma_Mf.",
    {"delta_sigma_mpa": 90.0, "fatigue_category": 56, "fatigue_method": "safe_life"},
    '''    assert_eq!(snapshot.fatigue_category, 56, "update-fatigue-inputs/drops-to-detail-category-56-under-a-safe-life-assessment: the detail category must be 56");
    assert_eq!(snapshot.fatigue_method, "safe_life", "update-fatigue-inputs/drops-to-detail-category-56-under-a-safe-life-assessment: the assessment method must be safe_life");
    assert_eq!(snapshot.delta_sigma_mpa, 90.0, "update-fatigue-inputs/drops-to-detail-category-56-under-a-safe-life-assessment: the applied stress range must be 90 MPa");''',
    '''    assert_eq!(raised_diff.fatigue_category, Some(56u8), "update-fatigue-inputs/drops-to-detail-category-56-under-a-safe-life-assessment: the diff must publish fatigueCategory as the u8 56");
    assert_eq!(raised_diff.fatigue_method.as_deref(), Some("safe_life"), "update-fatigue-inputs/drops-to-detail-category-56-under-a-safe-life-assessment: the diff must publish fatigueMethod as a string");
    assert!(raised_diff.bridge_delta_sigma_p_mpa.is_none(), "update-fatigue-inputs/drops-to-detail-category-56-under-a-safe-life-assessment: the bridge part carries its own stress range and is not touched");''',
)

# ── 11. update-through-thickness-inputs ───────────────────────────────────────
case(
    "🌬️update-through-thickness-inputs",
    "UpdateThroughThicknessInputs",
    "upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c",
    "EN 1993-1-10's brittle-fracture inputs: the subgrade string J2→K2, the actual plate thickness "
    "25→40 mm and the reference temperature 0→-20 degC. `t10_t_ed_c` is the only field in EN 1993's "
    "whole sheet whose before-value is 0.0, so this case also pins that a 0.0→-20.0 move is a real "
    "change and not read as a no-op.",
    {"t10_steel_subgrade": "K2", "t10_actual_thickness_mm": 40.0, "t10_t_ed_c": -20.0},
    '''    assert_eq!(snapshot.t10_steel_subgrade, "K2", "update-through-thickness-inputs/upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c: the subgrade must be K2");
    assert_eq!(snapshot.t10_actual_thickness_mm, 40.0, "update-through-thickness-inputs/upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c: the plate thickness must be 40 mm");
    assert_eq!(snapshot.t10_t_ed_c, -20.0, "update-through-thickness-inputs/upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c: the reference temperature must be -20 degC");
    assert_eq!(before().t10_t_ed_c, 0.0, "update-through-thickness-inputs/upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c: the committed before-snapshot must start at 0 degC");''',
    '''    assert_eq!(raised_diff.t10_t_ed_c, Some(-20.0), "update-through-thickness-inputs/upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c: the diff must publish t10TEdC = -20");
    assert_eq!(raised_diff.t10_steel_subgrade.as_deref(), Some("K2"), "update-through-thickness-inputs/upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c: the diff must publish t10SteelSubgrade as a string");
    assert!(raised_diff.fire_design_temperature_c.is_none(), "update-through-thickness-inputs/upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c: the fire design temperature is a different part's temperature");''',
)

# ── 12. update-tension-component-inputs ───────────────────────────────────────
case(
    "⚡update-tension-component-inputs",
    "UpdateTensionComponentInputs",
    "derates-the-tension-rod-to-a-400-kn-characteristic-strength",
    "EN 1993-1-11's tension-component triple: the ultimate characteristic strength 500→400 kN, the "
    "characteristic resistance 350→280 kN and the design force 250→190 kN. All three guards in this "
    "builder are finiteness checks; every value here is finite, so no `mutation.invariant` fires.",
    {"tension_component_f_uk_kn": 400.0, "tension_component_f_k_kn": 280.0, "tension_component_n_ed_kn": 190.0},
    '''    assert_eq!(snapshot.tension_component_f_uk_kn, 400.0, "update-tension-component-inputs/derates-the-tension-rod-to-a-400-kn-characteristic-strength: Fuk must be 400 kN");
    assert_eq!(snapshot.tension_component_n_ed_kn, 190.0, "update-tension-component-inputs/derates-the-tension-rod-to-a-400-kn-characteristic-strength: the design force must be 190 kN");
    assert_eq!(snapshot.tension_n_ed_kn, before().tension_n_ed_kn, "update-tension-component-inputs/derates-the-tension-rod-to-a-400-kn-characteristic-strength: the base member's own tension force is a DIFFERENT field and must not move");''',
    '''    assert_eq!(raised_diff.tension_component_f_k_kn, Some(280.0), "update-tension-component-inputs/derates-the-tension-rod-to-a-400-kn-characteristic-strength: the diff must publish tensionComponentFKKn = 280");
    assert!(raised_diff.tension_n_ed_kn.is_none(), "update-tension-component-inputs/derates-the-tension-rod-to-a-400-kn-characteristic-strength: the similarly-named member-properties field must stay null");''',
)

# ── 13. update-hss-inputs ─────────────────────────────────────────────────────
case(
    "💧update-hss-inputs",
    "UpdateHssInputs",
    "reclassifies-the-hollow-section-to-class-3-in-s355",
    "EN 1993-1-12's hollow-section group: elastic modulus 400000→320000 mm3, grade 460→355 MPa, "
    "section class 2→3 (a `u8`) and design moment 100→75 kNm. Dropping to class 3 is what forces the "
    "elastic rather than plastic modulus into the check.",
    {"hss_w_el_mm3": 320000.0, "hss_f_y_mpa": 355.0, "hss_section_class": 3, "hss_m_ed_knm": 75.0},
    '''    assert_eq!(snapshot.hss_section_class, 3, "update-hss-inputs/reclassifies-the-hollow-section-to-class-3-in-s355: the section class must be 3");
    assert_eq!(snapshot.hss_f_y_mpa, 355.0, "update-hss-inputs/reclassifies-the-hollow-section-to-class-3-in-s355: the hollow-section grade must be S355");
    assert_eq!(snapshot.hss_w_el_mm3, 320000.0, "update-hss-inputs/reclassifies-the-hollow-section-to-class-3-in-s355: the elastic section modulus must be 320000 mm3");''',
    '''    assert_eq!(raised_diff.hss_section_class, Some(3u8), "update-hss-inputs/reclassifies-the-hollow-section-to-class-3-in-s355: the diff must publish hssSectionClass as the u8 3");
    assert_eq!(raised_diff.hss_m_ed_knm, Some(75.0), "update-hss-inputs/reclassifies-the-hollow-section-to-class-3-in-s355: the diff must publish hssMEdKnm = 75");
    assert!(raised_diff.m_ed_knm.is_none(), "update-hss-inputs/reclassifies-the-hollow-section-to-class-3-in-s355: the base member's design moment is a different field and must stay null");''',
)

# ── 14. update-bridge-inputs ──────────────────────────────────────────────────
case(
    "🏗️update-bridge-inputs",
    "UpdateBridgeInputs",
    "raises-the-bridge-damage-equivalence-and-dynamic-factors",
    "EN 1993-2's three fatigue-of-bridges factors: damage-equivalence lambda 1.0→1.5, dynamic "
    "factor phi2 1.0→1.25 and the reference stress range 30→45 MPa. Only these three of the sheet's "
    "74 fields are republished.",
    {"bridge_lambda": 1.5, "bridge_phi_2": 1.25, "bridge_delta_sigma_p_mpa": 45.0},
    '''    assert_eq!(snapshot.bridge_lambda, 1.5, "update-bridge-inputs/raises-the-bridge-damage-equivalence-and-dynamic-factors: the damage-equivalence factor must be 1.5");
    assert_eq!(snapshot.bridge_phi_2, 1.25, "update-bridge-inputs/raises-the-bridge-damage-equivalence-and-dynamic-factors: the dynamic factor must be 1.25");
    assert_eq!(snapshot.bridge_delta_sigma_p_mpa, 45.0, "update-bridge-inputs/raises-the-bridge-damage-equivalence-and-dynamic-factors: the reference stress range must be 45 MPa");''',
    '''    assert_eq!(raised_diff.bridge_phi_2, Some(1.25), "update-bridge-inputs/raises-the-bridge-damage-equivalence-and-dynamic-factors: the diff must publish bridgePhi2 = 1.25");
    assert!(raised_diff.delta_sigma_mpa.is_none(), "update-bridge-inputs/raises-the-bridge-damage-equivalence-and-dynamic-factors: EN 1993-1-9's own stress range must stay null");''',
)

# ── 15. update-tower-inputs ───────────────────────────────────────────────────
case(
    "🌗update-tower-inputs",
    "UpdateTowerInputs",
    "raises-the-tower-wind-factor-and-leg-force",
    "EN 1993-3's two-field lattice-tower group: the wind amplification factor 1.25→1.75 and the leg "
    "design force 300→480 kN.",
    {"tower_wind_factor": 1.75, "tower_n_ed_kn": 480.0},
    '''    assert_eq!(snapshot.tower_wind_factor, 1.75, "update-tower-inputs/raises-the-tower-wind-factor-and-leg-force: the wind amplification factor must be 1.75");
    assert_eq!(snapshot.tower_n_ed_kn, 480.0, "update-tower-inputs/raises-the-tower-wind-factor-and-leg-force: the leg design force must be 480 kN");''',
    '''    assert_eq!(raised_diff.tower_n_ed_kn, Some(480.0), "update-tower-inputs/raises-the-tower-wind-factor-and-leg-force: the diff must publish towerNEdKn = 480");
    assert!(raised_diff.pile_n_ed_kn.is_none(), "update-tower-inputs/raises-the-tower-wind-factor-and-leg-force: the piling part's design force is a separate group");''',
)

# ── 16. update-pile-inputs ────────────────────────────────────────────────────
case(
    "🗺️update-pile-inputs",
    "UpdatePileInputs",
    "derates-the-driven-pile-for-hard-driving",
    "EN 1993-5's three piling fields: the driving stress 280→320 MPa, the section reduction factor "
    "kRed 0.875→0.75 (hard driving into dense ground) and the design force 400→340 kN.",
    {"pile_sigma_mpa": 320.0, "pile_k_red": 0.75, "pile_n_ed_kn": 340.0},
    '''    assert_eq!(snapshot.pile_sigma_mpa, 320.0, "update-pile-inputs/derates-the-driven-pile-for-hard-driving: the driving stress must be 320 MPa");
    assert_eq!(snapshot.pile_k_red, 0.75, "update-pile-inputs/derates-the-driven-pile-for-hard-driving: the section reduction factor must be 0.75");
    assert_eq!(snapshot.pile_n_ed_kn, 340.0, "update-pile-inputs/derates-the-driven-pile-for-hard-driving: the pile design force must be 340 kN");''',
    '''    assert_eq!(raised_diff.pile_k_red, Some(0.75), "update-pile-inputs/derates-the-driven-pile-for-hard-driving: the diff must publish pileKRed = 0.75");
    assert!(raised_diff.chi.is_none(), "update-pile-inputs/derates-the-driven-pile-for-hard-driving: the member buckling reduction chi is a different reduction factor entirely");''',
)

# ── 17. update-crane-inputs ───────────────────────────────────────────────────
case(
    "🌡️update-crane-inputs",
    "UpdateCraneInputs",
    "widens-the-crane-wheel-contact-patch-under-a-heavier-wheel",
    "EN 1993-6's four crane-runway fields: the wheel load 50→75 kN, the wheel contact length "
    "100→125 mm, the rail load-dispersion length 50→62.5 mm and the web thickness 10→12 mm.",
    {"crane_f_z_ed_kn": 75.0, "crane_wheel_contact_length_mm": 125.0, "crane_dispersion_mm": 62.5, "crane_t_w_mm": 12.0},
    '''    assert_eq!(snapshot.crane_f_z_ed_kn, 75.0, "update-crane-inputs/widens-the-crane-wheel-contact-patch-under-a-heavier-wheel: the wheel load must be 75 kN");
    assert_eq!(snapshot.crane_dispersion_mm, 62.5, "update-crane-inputs/widens-the-crane-wheel-contact-patch-under-a-heavier-wheel: the load-dispersion length must be 62.5 mm");
    assert_eq!(snapshot.crane_t_w_mm, 12.0, "update-crane-inputs/widens-the-crane-wheel-contact-patch-under-a-heavier-wheel: the runway-beam web thickness must be 12 mm");''',
    '''    assert_eq!(raised_diff.crane_wheel_contact_length_mm, Some(125.0), "update-crane-inputs/widens-the-crane-wheel-contact-patch-under-a-heavier-wheel: the diff must publish craneWheelContactLengthMm = 125");
    assert!(raised_diff.v_ed_kn.is_none(), "update-crane-inputs/widens-the-crane-wheel-contact-patch-under-a-heavier-wheel: the base member's shear force is not a crane input");''',
)

# ── emit ──────────────────────────────────────────────────────────────────────
assert len(CASES) == 17, len(CASES)

for entry in CASES:
    kind, _ = KIND[entry["leaf"]]
    rust = common.test_source(
        artifact="en1993",
        snapshot_ty="En1993Snapshot",
        diff_ty="En1993Diff",
        mutation_ty="En1993Mutation",
        kind=kind,
        case=entry["case"],
        summary=entry["summary"],
        extra_applied=entry["extra_applied"],
        extra_diff=entry["extra_diff"],
    )
    common.emit_case(ROOT, entry["leaf"], entry["case"], BEFORE, entry["after"], entry["mutation"], entry["diff"], APPLIED, rust)

lines = []
for entry in CASES:
    _, module = KIND[entry["leaf"]]
    lines.append('    #[path = "{}/🧪️tests/{}/🦀️component.rs"]'.format(entry["leaf"], entry["case"]))
    lines.append("    mod tests_{}_{};".format(module, entry["case"].replace("-", "_")))
print("\n".join(lines))
