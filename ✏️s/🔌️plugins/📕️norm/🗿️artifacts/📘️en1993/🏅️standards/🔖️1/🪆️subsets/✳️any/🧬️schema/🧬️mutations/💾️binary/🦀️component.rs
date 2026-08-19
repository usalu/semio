//! ⚖️ En1993 artifact — hand-rolled `OpBinary` for `En1993Mutation`, sibling to `../📝️text`'s
//! `OpText`. Every variant's binary form is `format-byte | tag u8 | json-string-per-field` — the
//! same uniform per-field JSON strategy as the text codec, so every one of the 74 fields' 5 Rust
//! types (`f64`/`u32`/`u8`/`String`/`AnnexChoice`) shares one write/read helper pair.

pub use crate::artifacts::en1993::schema::mutations::En1993Mutation;
use crate::artifacts::en1993::schema::mutations::{
    change_annex::mutation::ChangeAnnex, update_bolt_inputs::mutation::UpdateBoltInputs, update_bridge_inputs::mutation::UpdateBridgeInputs, update_cold_formed_inputs::mutation::UpdateColdFormedInputs,
    update_crane_inputs::mutation::UpdateCraneInputs, update_fatigue_inputs::mutation::UpdateFatigueInputs, update_fire_inputs::mutation::UpdateFireInputs, update_hss_inputs::mutation::UpdateHssInputs,
    update_member_properties::mutation::UpdateMemberProperties, update_pile_inputs::mutation::UpdatePileInputs, update_plated_inputs::mutation::UpdatePlatedInputs, update_silo_shell_inputs::mutation::UpdateSiloShellInputs,
    update_stainless_inputs::mutation::UpdateStainlessInputs, update_tension_component_inputs::mutation::UpdateTensionComponentInputs, update_through_thickness_inputs::mutation::UpdateThroughThicknessInputs,
    update_tower_inputs::mutation::UpdateTowerInputs, update_weld_inputs::mutation::UpdateWeldInputs,
};

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

//#region 🔖️OpBinaryCodec
async fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
async fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}
/// 🧬️ Every payload field (even plain scalars) round-trips through JSON — see `../📝️text`'s
/// `🔖️ScalarCodec` doc for why this stays uniform across all 5 field types in this facet.
async fn write_json_bin<T: serde::Serialize>(out: &mut Vec<u8>, value: &T) {
    write_str_bin(out, &serde_json::to_string(value).expect("en1993 mutation payload field always serializes"));
}
async fn read_json_bin<T: serde::de::DeserializeOwned>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    serde_json::from_str(&read_str_bin(reader)?).map_err(|e| e.to_string())
}

impl protocol::OpBinary for En1993Mutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            En1993Mutation::ChangeAnnex(_) => 0,
            En1993Mutation::UpdateMemberProperties(_) => 1,
            En1993Mutation::UpdateFireInputs(_) => 2,
            En1993Mutation::UpdateColdFormedInputs(_) => 3,
            En1993Mutation::UpdateStainlessInputs(_) => 4,
            En1993Mutation::UpdatePlatedInputs(_) => 5,
            En1993Mutation::UpdateSiloShellInputs(_) => 6,
            En1993Mutation::UpdateBoltInputs(_) => 7,
            En1993Mutation::UpdateWeldInputs(_) => 8,
            En1993Mutation::UpdateFatigueInputs(_) => 9,
            En1993Mutation::UpdateThroughThicknessInputs(_) => 10,
            En1993Mutation::UpdateTensionComponentInputs(_) => 11,
            En1993Mutation::UpdateHssInputs(_) => 12,
            En1993Mutation::UpdateBridgeInputs(_) => 13,
            En1993Mutation::UpdateTowerInputs(_) => 14,
            En1993Mutation::UpdatePileInputs(_) => 15,
            En1993Mutation::UpdateCraneInputs(_) => 16,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            En1993Mutation::ChangeAnnex(p) => {
                write_json_bin(&mut out, &p.new_annex);
            }
            En1993Mutation::UpdateMemberProperties(p) => {
                write_json_bin(&mut out, &p.new_n_ed_kn);
                write_json_bin(&mut out, &p.new_m_ed_knm);
                write_json_bin(&mut out, &p.new_v_ed_kn);
                write_json_bin(&mut out, &p.new_a_mm2);
                write_json_bin(&mut out, &p.new_a_v_mm2);
                write_json_bin(&mut out, &p.new_w_pl_mm3);
                write_json_bin(&mut out, &p.new_f_y_mpa);
                write_json_bin(&mut out, &p.new_f_u_mpa);
                write_json_bin(&mut out, &p.new_chi);
                write_json_bin(&mut out, &p.new_a_net_mm2);
                write_json_bin(&mut out, &p.new_tension_n_ed_kn);
            }
            En1993Mutation::UpdateFireInputs(p) => {
                write_json_bin(&mut out, &p.new_fire_thickness_mm);
                write_json_bin(&mut out, &p.new_fire_rating);
                write_json_bin(&mut out, &p.new_fire_massivity);
                write_json_bin(&mut out, &p.new_fire_mu_0);
                write_json_bin(&mut out, &p.new_fire_design_temperature_c);
            }
            En1993Mutation::UpdateColdFormedInputs(p) => {
                write_json_bin(&mut out, &p.new_cf_b_bar_mm);
                write_json_bin(&mut out, &p.new_cf_t_mm);
                write_json_bin(&mut out, &p.new_cf_k_sigma);
                write_json_bin(&mut out, &p.new_cf_psi);
                write_json_bin(&mut out, &p.new_cf_n_ed_kn);
                write_json_bin(&mut out, &p.new_cf_gross_resistance_kn);
            }
            En1993Mutation::UpdateStainlessInputs(p) => {
                write_json_bin(&mut out, &p.new_stainless_m_ed_knm);
                write_json_bin(&mut out, &p.new_stainless_w_pl_mm3);
                write_json_bin(&mut out, &p.new_stainless_f_y_mpa);
            }
            En1993Mutation::UpdatePlatedInputs(p) => {
                write_json_bin(&mut out, &p.new_plated_lambda_p);
                write_json_bin(&mut out, &p.new_plated_sigma_ed_mpa);
            }
            En1993Mutation::UpdateSiloShellInputs(p) => {
                write_json_bin(&mut out, &p.new_silo_t_mm);
                write_json_bin(&mut out, &p.new_silo_r_mm);
                write_json_bin(&mut out, &p.new_shell_sigma_x_ed_mpa);
                write_json_bin(&mut out, &p.new_silo_k);
                write_json_bin(&mut out, &p.new_silo_gamma_kn_m3);
                write_json_bin(&mut out, &p.new_silo_depth_m);
            }
            En1993Mutation::UpdateBoltInputs(p) => {
                write_json_bin(&mut out, &p.new_bolt_f_ed_kn);
                write_json_bin(&mut out, &p.new_bolt_n_bolts);
                write_json_bin(&mut out, &p.new_bolt_a_s_mm2);
                write_json_bin(&mut out, &p.new_bolt_e1_mm);
                write_json_bin(&mut out, &p.new_bolt_e2_mm);
                write_json_bin(&mut out, &p.new_bolt_d0_mm);
                write_json_bin(&mut out, &p.new_bolt_d_mm);
                write_json_bin(&mut out, &p.new_bolt_t_mm);
                write_json_bin(&mut out, &p.new_bolt_f_u_mpa);
                write_json_bin(&mut out, &p.new_bolt_f_ub_mpa);
            }
            En1993Mutation::UpdateWeldInputs(p) => {
                write_json_bin(&mut out, &p.new_weld_a_mm);
                write_json_bin(&mut out, &p.new_weld_l_mm);
                write_json_bin(&mut out, &p.new_weld_f_u_mpa);
                write_json_bin(&mut out, &p.new_weld_steel_grade);
                write_json_bin(&mut out, &p.new_weld_f_ed_kn);
            }
            En1993Mutation::UpdateFatigueInputs(p) => {
                write_json_bin(&mut out, &p.new_delta_sigma_mpa);
                write_json_bin(&mut out, &p.new_fatigue_category);
                write_json_bin(&mut out, &p.new_fatigue_method);
            }
            En1993Mutation::UpdateThroughThicknessInputs(p) => {
                write_json_bin(&mut out, &p.new_t10_steel_subgrade);
                write_json_bin(&mut out, &p.new_t10_actual_thickness_mm);
                write_json_bin(&mut out, &p.new_t10_t_ed_c);
            }
            En1993Mutation::UpdateTensionComponentInputs(p) => {
                write_json_bin(&mut out, &p.new_tension_component_f_uk_kn);
                write_json_bin(&mut out, &p.new_tension_component_f_k_kn);
                write_json_bin(&mut out, &p.new_tension_component_n_ed_kn);
            }
            En1993Mutation::UpdateHssInputs(p) => {
                write_json_bin(&mut out, &p.new_hss_w_el_mm3);
                write_json_bin(&mut out, &p.new_hss_f_y_mpa);
                write_json_bin(&mut out, &p.new_hss_section_class);
                write_json_bin(&mut out, &p.new_hss_m_ed_knm);
            }
            En1993Mutation::UpdateBridgeInputs(p) => {
                write_json_bin(&mut out, &p.new_bridge_lambda);
                write_json_bin(&mut out, &p.new_bridge_phi_2);
                write_json_bin(&mut out, &p.new_bridge_delta_sigma_p_mpa);
            }
            En1993Mutation::UpdateTowerInputs(p) => {
                write_json_bin(&mut out, &p.new_tower_wind_factor);
                write_json_bin(&mut out, &p.new_tower_n_ed_kn);
            }
            En1993Mutation::UpdatePileInputs(p) => {
                write_json_bin(&mut out, &p.new_pile_sigma_mpa);
                write_json_bin(&mut out, &p.new_pile_k_red);
                write_json_bin(&mut out, &p.new_pile_n_ed_kn);
            }
            En1993Mutation::UpdateCraneInputs(p) => {
                write_json_bin(&mut out, &p.new_crane_f_z_ed_kn);
                write_json_bin(&mut out, &p.new_crane_wheel_contact_length_mm);
                write_json_bin(&mut out, &p.new_crane_dispersion_mm);
                write_json_bin(&mut out, &p.new_crane_t_w_mm);
            }
        }
        Ok(out)
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => {
                let new_annex = read_json_bin(&mut reader).map_err(|e| malformed("annex", reader.position(), e))?;
                Ok(En1993Mutation::ChangeAnnex(ChangeAnnex { new_annex }))
            }
            1 => {
                let new_n_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("n_ed_kn", reader.position(), e))?;
                let new_m_ed_knm = read_json_bin(&mut reader).map_err(|e| malformed("m_ed_knm", reader.position(), e))?;
                let new_v_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("v_ed_kn", reader.position(), e))?;
                let new_a_mm2 = read_json_bin(&mut reader).map_err(|e| malformed("a_mm2", reader.position(), e))?;
                let new_a_v_mm2 = read_json_bin(&mut reader).map_err(|e| malformed("a_v_mm2", reader.position(), e))?;
                let new_w_pl_mm3 = read_json_bin(&mut reader).map_err(|e| malformed("w_pl_mm3", reader.position(), e))?;
                let new_f_y_mpa = read_json_bin(&mut reader).map_err(|e| malformed("f_y_mpa", reader.position(), e))?;
                let new_f_u_mpa = read_json_bin(&mut reader).map_err(|e| malformed("f_u_mpa", reader.position(), e))?;
                let new_chi = read_json_bin(&mut reader).map_err(|e| malformed("chi", reader.position(), e))?;
                let new_a_net_mm2 = read_json_bin(&mut reader).map_err(|e| malformed("a_net_mm2", reader.position(), e))?;
                let new_tension_n_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("tension_n_ed_kn", reader.position(), e))?;
                Ok(En1993Mutation::UpdateMemberProperties(UpdateMemberProperties { new_n_ed_kn, new_m_ed_knm, new_v_ed_kn, new_a_mm2, new_a_v_mm2, new_w_pl_mm3, new_f_y_mpa, new_f_u_mpa, new_chi, new_a_net_mm2, new_tension_n_ed_kn }))
            }
            2 => {
                let new_fire_thickness_mm = read_json_bin(&mut reader).map_err(|e| malformed("fire_thickness_mm", reader.position(), e))?;
                let new_fire_rating = read_json_bin(&mut reader).map_err(|e| malformed("fire_rating", reader.position(), e))?;
                let new_fire_massivity = read_json_bin(&mut reader).map_err(|e| malformed("fire_massivity", reader.position(), e))?;
                let new_fire_mu_0 = read_json_bin(&mut reader).map_err(|e| malformed("fire_mu_0", reader.position(), e))?;
                let new_fire_design_temperature_c = read_json_bin(&mut reader).map_err(|e| malformed("fire_design_temperature_c", reader.position(), e))?;
                Ok(En1993Mutation::UpdateFireInputs(UpdateFireInputs { new_fire_thickness_mm, new_fire_rating, new_fire_massivity, new_fire_mu_0, new_fire_design_temperature_c }))
            }
            3 => {
                let new_cf_b_bar_mm = read_json_bin(&mut reader).map_err(|e| malformed("cf_b_bar_mm", reader.position(), e))?;
                let new_cf_t_mm = read_json_bin(&mut reader).map_err(|e| malformed("cf_t_mm", reader.position(), e))?;
                let new_cf_k_sigma = read_json_bin(&mut reader).map_err(|e| malformed("cf_k_sigma", reader.position(), e))?;
                let new_cf_psi = read_json_bin(&mut reader).map_err(|e| malformed("cf_psi", reader.position(), e))?;
                let new_cf_n_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("cf_n_ed_kn", reader.position(), e))?;
                let new_cf_gross_resistance_kn = read_json_bin(&mut reader).map_err(|e| malformed("cf_gross_resistance_kn", reader.position(), e))?;
                Ok(En1993Mutation::UpdateColdFormedInputs(UpdateColdFormedInputs { new_cf_b_bar_mm, new_cf_t_mm, new_cf_k_sigma, new_cf_psi, new_cf_n_ed_kn, new_cf_gross_resistance_kn }))
            }
            4 => {
                let new_stainless_m_ed_knm = read_json_bin(&mut reader).map_err(|e| malformed("stainless_m_ed_knm", reader.position(), e))?;
                let new_stainless_w_pl_mm3 = read_json_bin(&mut reader).map_err(|e| malformed("stainless_w_pl_mm3", reader.position(), e))?;
                let new_stainless_f_y_mpa = read_json_bin(&mut reader).map_err(|e| malformed("stainless_f_y_mpa", reader.position(), e))?;
                Ok(En1993Mutation::UpdateStainlessInputs(UpdateStainlessInputs { new_stainless_m_ed_knm, new_stainless_w_pl_mm3, new_stainless_f_y_mpa }))
            }
            5 => {
                let new_plated_lambda_p = read_json_bin(&mut reader).map_err(|e| malformed("plated_lambda_p", reader.position(), e))?;
                let new_plated_sigma_ed_mpa = read_json_bin(&mut reader).map_err(|e| malformed("plated_sigma_ed_mpa", reader.position(), e))?;
                Ok(En1993Mutation::UpdatePlatedInputs(UpdatePlatedInputs { new_plated_lambda_p, new_plated_sigma_ed_mpa }))
            }
            6 => {
                let new_silo_t_mm = read_json_bin(&mut reader).map_err(|e| malformed("silo_t_mm", reader.position(), e))?;
                let new_silo_r_mm = read_json_bin(&mut reader).map_err(|e| malformed("silo_r_mm", reader.position(), e))?;
                let new_shell_sigma_x_ed_mpa = read_json_bin(&mut reader).map_err(|e| malformed("shell_sigma_x_ed_mpa", reader.position(), e))?;
                let new_silo_k = read_json_bin(&mut reader).map_err(|e| malformed("silo_k", reader.position(), e))?;
                let new_silo_gamma_kn_m3 = read_json_bin(&mut reader).map_err(|e| malformed("silo_gamma_kn_m3", reader.position(), e))?;
                let new_silo_depth_m = read_json_bin(&mut reader).map_err(|e| malformed("silo_depth_m", reader.position(), e))?;
                Ok(En1993Mutation::UpdateSiloShellInputs(UpdateSiloShellInputs { new_silo_t_mm, new_silo_r_mm, new_shell_sigma_x_ed_mpa, new_silo_k, new_silo_gamma_kn_m3, new_silo_depth_m }))
            }
            7 => {
                let new_bolt_f_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("bolt_f_ed_kn", reader.position(), e))?;
                let new_bolt_n_bolts = read_json_bin(&mut reader).map_err(|e| malformed("bolt_n_bolts", reader.position(), e))?;
                let new_bolt_a_s_mm2 = read_json_bin(&mut reader).map_err(|e| malformed("bolt_a_s_mm2", reader.position(), e))?;
                let new_bolt_e1_mm = read_json_bin(&mut reader).map_err(|e| malformed("bolt_e1_mm", reader.position(), e))?;
                let new_bolt_e2_mm = read_json_bin(&mut reader).map_err(|e| malformed("bolt_e2_mm", reader.position(), e))?;
                let new_bolt_d0_mm = read_json_bin(&mut reader).map_err(|e| malformed("bolt_d0_mm", reader.position(), e))?;
                let new_bolt_d_mm = read_json_bin(&mut reader).map_err(|e| malformed("bolt_d_mm", reader.position(), e))?;
                let new_bolt_t_mm = read_json_bin(&mut reader).map_err(|e| malformed("bolt_t_mm", reader.position(), e))?;
                let new_bolt_f_u_mpa = read_json_bin(&mut reader).map_err(|e| malformed("bolt_f_u_mpa", reader.position(), e))?;
                let new_bolt_f_ub_mpa = read_json_bin(&mut reader).map_err(|e| malformed("bolt_f_ub_mpa", reader.position(), e))?;
                Ok(En1993Mutation::UpdateBoltInputs(UpdateBoltInputs { new_bolt_f_ed_kn, new_bolt_n_bolts, new_bolt_a_s_mm2, new_bolt_e1_mm, new_bolt_e2_mm, new_bolt_d0_mm, new_bolt_d_mm, new_bolt_t_mm, new_bolt_f_u_mpa, new_bolt_f_ub_mpa }))
            }
            8 => {
                let new_weld_a_mm = read_json_bin(&mut reader).map_err(|e| malformed("weld_a_mm", reader.position(), e))?;
                let new_weld_l_mm = read_json_bin(&mut reader).map_err(|e| malformed("weld_l_mm", reader.position(), e))?;
                let new_weld_f_u_mpa = read_json_bin(&mut reader).map_err(|e| malformed("weld_f_u_mpa", reader.position(), e))?;
                let new_weld_steel_grade = read_json_bin(&mut reader).map_err(|e| malformed("weld_steel_grade", reader.position(), e))?;
                let new_weld_f_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("weld_f_ed_kn", reader.position(), e))?;
                Ok(En1993Mutation::UpdateWeldInputs(UpdateWeldInputs { new_weld_a_mm, new_weld_l_mm, new_weld_f_u_mpa, new_weld_steel_grade, new_weld_f_ed_kn }))
            }
            9 => {
                let new_delta_sigma_mpa = read_json_bin(&mut reader).map_err(|e| malformed("delta_sigma_mpa", reader.position(), e))?;
                let new_fatigue_category = read_json_bin(&mut reader).map_err(|e| malformed("fatigue_category", reader.position(), e))?;
                let new_fatigue_method = read_json_bin(&mut reader).map_err(|e| malformed("fatigue_method", reader.position(), e))?;
                Ok(En1993Mutation::UpdateFatigueInputs(UpdateFatigueInputs { new_delta_sigma_mpa, new_fatigue_category, new_fatigue_method }))
            }
            10 => {
                let new_t10_steel_subgrade = read_json_bin(&mut reader).map_err(|e| malformed("t10_steel_subgrade", reader.position(), e))?;
                let new_t10_actual_thickness_mm = read_json_bin(&mut reader).map_err(|e| malformed("t10_actual_thickness_mm", reader.position(), e))?;
                let new_t10_t_ed_c = read_json_bin(&mut reader).map_err(|e| malformed("t10_t_ed_c", reader.position(), e))?;
                Ok(En1993Mutation::UpdateThroughThicknessInputs(UpdateThroughThicknessInputs { new_t10_steel_subgrade, new_t10_actual_thickness_mm, new_t10_t_ed_c }))
            }
            11 => {
                let new_tension_component_f_uk_kn = read_json_bin(&mut reader).map_err(|e| malformed("tension_component_f_uk_kn", reader.position(), e))?;
                let new_tension_component_f_k_kn = read_json_bin(&mut reader).map_err(|e| malformed("tension_component_f_k_kn", reader.position(), e))?;
                let new_tension_component_n_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("tension_component_n_ed_kn", reader.position(), e))?;
                Ok(En1993Mutation::UpdateTensionComponentInputs(UpdateTensionComponentInputs { new_tension_component_f_uk_kn, new_tension_component_f_k_kn, new_tension_component_n_ed_kn }))
            }
            12 => {
                let new_hss_w_el_mm3 = read_json_bin(&mut reader).map_err(|e| malformed("hss_w_el_mm3", reader.position(), e))?;
                let new_hss_f_y_mpa = read_json_bin(&mut reader).map_err(|e| malformed("hss_f_y_mpa", reader.position(), e))?;
                let new_hss_section_class = read_json_bin(&mut reader).map_err(|e| malformed("hss_section_class", reader.position(), e))?;
                let new_hss_m_ed_knm = read_json_bin(&mut reader).map_err(|e| malformed("hss_m_ed_knm", reader.position(), e))?;
                Ok(En1993Mutation::UpdateHssInputs(UpdateHssInputs { new_hss_w_el_mm3, new_hss_f_y_mpa, new_hss_section_class, new_hss_m_ed_knm }))
            }
            13 => {
                let new_bridge_lambda = read_json_bin(&mut reader).map_err(|e| malformed("bridge_lambda", reader.position(), e))?;
                let new_bridge_phi_2 = read_json_bin(&mut reader).map_err(|e| malformed("bridge_phi_2", reader.position(), e))?;
                let new_bridge_delta_sigma_p_mpa = read_json_bin(&mut reader).map_err(|e| malformed("bridge_delta_sigma_p_mpa", reader.position(), e))?;
                Ok(En1993Mutation::UpdateBridgeInputs(UpdateBridgeInputs { new_bridge_lambda, new_bridge_phi_2, new_bridge_delta_sigma_p_mpa }))
            }
            14 => {
                let new_tower_wind_factor = read_json_bin(&mut reader).map_err(|e| malformed("tower_wind_factor", reader.position(), e))?;
                let new_tower_n_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("tower_n_ed_kn", reader.position(), e))?;
                Ok(En1993Mutation::UpdateTowerInputs(UpdateTowerInputs { new_tower_wind_factor, new_tower_n_ed_kn }))
            }
            15 => {
                let new_pile_sigma_mpa = read_json_bin(&mut reader).map_err(|e| malformed("pile_sigma_mpa", reader.position(), e))?;
                let new_pile_k_red = read_json_bin(&mut reader).map_err(|e| malformed("pile_k_red", reader.position(), e))?;
                let new_pile_n_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("pile_n_ed_kn", reader.position(), e))?;
                Ok(En1993Mutation::UpdatePileInputs(UpdatePileInputs { new_pile_sigma_mpa, new_pile_k_red, new_pile_n_ed_kn }))
            }
            16 => {
                let new_crane_f_z_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("crane_f_z_ed_kn", reader.position(), e))?;
                let new_crane_wheel_contact_length_mm = read_json_bin(&mut reader).map_err(|e| malformed("crane_wheel_contact_length_mm", reader.position(), e))?;
                let new_crane_dispersion_mm = read_json_bin(&mut reader).map_err(|e| malformed("crane_dispersion_mm", reader.position(), e))?;
                let new_crane_t_w_mm = read_json_bin(&mut reader).map_err(|e| malformed("crane_t_w_mm", reader.position(), e))?;
                Ok(En1993Mutation::UpdateCraneInputs(UpdateCraneInputs { new_crane_f_z_ed_kn, new_crane_wheel_contact_length_mm, new_crane_dispersion_mm, new_crane_t_w_mm }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec
