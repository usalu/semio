//! 🧬️ En1993 snapshot schema — artifact-lane fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1993", layout = "lines")]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[dsl(unit = "mm2")]
    #[state(artifact)]
    pub a_mm2: f64,
    #[dsl(unit = "mm2")]
    #[state(artifact)]
    pub a_v_mm2: f64,
    #[state(artifact)]
    pub w_pl_mm3: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub f_y_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub f_u_mpa: f64,
    #[state(artifact)]
    pub chi: f64,
    #[dsl(unit = "mm2")]
    #[state(artifact)]
    pub a_net_mm2: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub tension_n_ed_kn: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub fire_thickness_mm: f64,
    #[state(artifact)]
    pub fire_rating: String,
    #[state(artifact)]
    pub fire_massivity: f64,
    #[state(artifact)]
    pub fire_mu_0: f64,
    #[state(artifact)]
    pub fire_design_temperature_c: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub cf_b_bar_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub cf_t_mm: f64,
    #[state(artifact)]
    pub cf_k_sigma: f64,
    #[state(artifact)]
    pub cf_psi: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub cf_n_ed_kn: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub cf_gross_resistance_kn: f64,
    #[state(artifact)]
    pub stainless_m_ed_knm: f64,
    #[state(artifact)]
    pub stainless_w_pl_mm3: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub stainless_f_y_mpa: f64,
    #[state(artifact)]
    pub plated_lambda_p: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub plated_sigma_ed_mpa: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub silo_t_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub silo_r_mm: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub shell_sigma_x_ed_mpa: f64,
    #[state(artifact)]
    pub silo_k: f64,
    #[state(artifact)]
    pub silo_gamma_kn_m3: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub silo_depth_m: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub bolt_f_ed_kn: f64,
    #[state(artifact)]
    pub bolt_n_bolts: u32,
    #[dsl(unit = "mm2")]
    #[state(artifact)]
    pub bolt_a_s_mm2: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub bolt_e1_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub bolt_e2_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub bolt_d0_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub bolt_d_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub bolt_t_mm: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub bolt_f_u_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub bolt_f_ub_mpa: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub weld_a_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub weld_l_mm: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub weld_f_u_mpa: f64,
    #[state(artifact)]
    pub weld_steel_grade: String,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub weld_f_ed_kn: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub delta_sigma_mpa: f64,
    #[state(artifact)]
    pub fatigue_category: u8,
    #[state(artifact)]
    pub fatigue_method: String,
    #[state(artifact)]
    pub t10_steel_subgrade: String,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub t10_actual_thickness_mm: f64,
    #[state(artifact)]
    pub t10_t_ed_c: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub tension_component_f_uk_kn: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub tension_component_f_k_kn: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub tension_component_n_ed_kn: f64,
    #[state(artifact)]
    pub hss_w_el_mm3: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub hss_f_y_mpa: f64,
    #[state(artifact)]
    pub hss_section_class: u8,
    #[state(artifact)]
    pub hss_m_ed_knm: f64,
    #[state(artifact)]
    pub bridge_lambda: f64,
    #[state(artifact)]
    pub bridge_phi_2: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub bridge_delta_sigma_p_mpa: f64,
    #[state(artifact)]
    pub tower_wind_factor: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub tower_n_ed_kn: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub pile_sigma_mpa: f64,
    #[state(artifact)]
    pub pile_k_red: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub pile_n_ed_kn: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub crane_f_z_ed_kn: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub crane_wheel_contact_length_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub crane_dispersion_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub crane_t_w_mm: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1993Snapshot, extension = "en1993", envelope_id = "norm.en1993");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1993Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            n_ed_kn: 500.0,
            m_ed_knm: 150.0,
            v_ed_kn: 80.0,
            a_mm2: 5000.0,
            a_v_mm2: 2500.0,
            w_pl_mm3: 500_000.0,
            f_y_mpa: 355.0,
            f_u_mpa: 510.0,
            chi: 0.75,
            a_net_mm2: 4250.0,
            tension_n_ed_kn: 400.0,
            fire_thickness_mm: 20.0,
            fire_rating: "r60".into(),
            fire_massivity: 150.0,
            fire_mu_0: 0.5,
            fire_design_temperature_c: 550.0,
            cf_b_bar_mm: 90.0,
            cf_t_mm: 2.0,
            cf_k_sigma: 4.0,
            cf_psi: 1.0,
            cf_n_ed_kn: 20.0,
            cf_gross_resistance_kn: 50.0,
            stainless_m_ed_knm: 40.0,
            stainless_w_pl_mm3: 300_000.0,
            stainless_f_y_mpa: 220.0,
            plated_lambda_p: 0.8,
            plated_sigma_ed_mpa: 200.0,
            silo_t_mm: 8.0,
            silo_r_mm: 3000.0,
            shell_sigma_x_ed_mpa: 150.0,
            silo_k: 0.4,
            silo_gamma_kn_m3: 18.0,
            silo_depth_m: 5.0,
            bolt_f_ed_kn: 120.0,
            bolt_n_bolts: 2,
            bolt_a_s_mm2: 245.0,
            bolt_e1_mm: 40.0,
            bolt_e2_mm: 40.0,
            bolt_d0_mm: 22.0,
            bolt_d_mm: 20.0,
            bolt_t_mm: 10.0,
            bolt_f_u_mpa: 510.0,
            bolt_f_ub_mpa: 800.0,
            weld_a_mm: 5.0,
            weld_l_mm: 100.0,
            weld_f_u_mpa: 510.0,
            weld_steel_grade: "S355".into(),
            weld_f_ed_kn: 80.0,
            delta_sigma_mpa: 50.0,
            fatigue_category: 71,
            fatigue_method: "damage_tolerant".into(),
            t10_steel_subgrade: "J2".into(),
            t10_actual_thickness_mm: 25.0,
            t10_t_ed_c: 0.0,
            tension_component_f_uk_kn: 500.0,
            tension_component_f_k_kn: 350.0,
            tension_component_n_ed_kn: 250.0,
            hss_w_el_mm3: 400_000.0,
            hss_f_y_mpa: 460.0,
            hss_section_class: 2,
            hss_m_ed_knm: 100.0,
            bridge_lambda: 1.0,
            bridge_phi_2: 1.0,
            bridge_delta_sigma_p_mpa: 30.0,
            tower_wind_factor: 1.1,
            tower_n_ed_kn: 300.0,
            pile_sigma_mpa: 280.0,
            pile_k_red: 0.85,
            pile_n_ed_kn: 400.0,
            crane_f_z_ed_kn: 50.0,
            crane_wheel_contact_length_mm: 100.0,
            crane_dispersion_mm: 50.0,
            crane_t_w_mm: 10.0,
        }
    }
}
//#endregion 🔖️Snapshot


//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`En1993Snapshot`] — the surface
/// `../../../../../🧪️tests/🔩️mutate-en1993-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1993_snapshot_json(snapshot: &En1993Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_en1993_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`En1993Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1993_snapshot_json(text: &str) -> Result<En1993Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`En1993Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1993_dsl(text: &str) -> Result<En1993Snapshot, String> {
    <En1993Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`En1993Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1993_dsl(snapshot: &En1993Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`En1993Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1993_pack(bytes: &[u8]) -> Result<En1993Snapshot, String> {
    <En1993Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`En1993Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1993_pack(snapshot: &En1993Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
