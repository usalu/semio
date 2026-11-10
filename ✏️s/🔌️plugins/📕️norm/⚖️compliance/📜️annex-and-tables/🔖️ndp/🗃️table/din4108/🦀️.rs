//! 🗃️ Static NDP rows of din4108 generated from `🔣️.json` by the ticket codegen; sorted by id. Edit the JSON, not this file.

use semio_s_plugin_norm_ndp_model::NdpValue::*;
use semio_s_plugin_norm_ndp_model::{c, d, NdpEntry};

const S0: &str = "DIN 4108-3";
const S1: &str = "DIN 4108-3:2018-10";
const S2: &str = "2018-10";
const S3: &str = "DIN 4108-2";
const S4: &str = "degC";

/// 📊️ Registered NDPs and table cells of din4108.
pub const ROWS: &[NdpEntry] = &[
    c("ndp.din4108-2.mould.f_rsi_min", S3, "6.2", "f_Rsi minimum", Number(0.7), "", S3),
    c("ndp.din4108-2.mould.r_si", S3, "6.2", "R_si mould check", Number(0.25), "m2K/W", S3),
    c("ndp.din4108-2.mould.theta_e", S3, "6.2", "theta_e mould check", Number(-5.0), S4, S3),
    c("ndp.din4108-2.mould.theta_i", S3, "6.2", "theta_i mould check", Number(20.0), S4, S3),
    c("ndp.din4108-3.annex-a.condensation.duration", S0, "Annex A", "t condensation period", Number(2160.0), "h", S1).ed(S2),
    c("ndp.din4108-3.annex-a.condensation.phi_e", S0, "Annex A", "phi_e condensation period", Number(0.8), "", S1).ed(S2),
    c("ndp.din4108-3.annex-a.condensation.phi_i", S0, "Annex A", "phi_i condensation period", Number(0.5), "", S1).ed(S2),
    c("ndp.din4108-3.annex-a.condensation.theta_e", S0, "Annex A", "theta_e condensation period", Number(-5.0), S4, S1).ed(S2),
    c("ndp.din4108-3.annex-a.condensation.theta_i", S0, "Annex A", "theta_i condensation period", Number(20.0), S4, S1).ed(S2),
    c("ndp.din4108-3.annex-a.evaporation.duration", S0, "Annex A", "t evaporation period", Number(2160.0), "h", S1).ed(S2),
    c("ndp.din4108-3.annex-a.evaporation.phi", S0, "Annex A", "phi evaporation period", Number(0.7), "", S1).ed(S2),
    c("ndp.din4108-3.annex-a.evaporation.theta", S0, "Annex A", "theta evaporation period", Number(12.0), S4, S1).ed(S2),
    c("ndp.din4108-3.delta0", S0, "Annex A", "delta_0", Number(2e-10), "kg/(m s Pa)", S1).ed(S2),
    c("ndp.din4108-3.mass-limit.general", S0, "4.2", "m_W,T limit", Number(1.0), "kg/m2", S1).ed(S2),
    c("ndp.din4108-3.mass-limit.wood-light", S0, "4.2", "m_W,T limit wood and light elements", Number(0.5), "kg/m2", S1).ed(S2),
    c("ndp.din4108-3.saturation.ice.a", S0, "Annex A", "a over ice", Number(21.875), "", S1).ed(S2),
    c("ndp.din4108-3.saturation.ice.b", S0, "Annex A", "b over ice", Number(265.5), S4, S1).ed(S2),
    c("ndp.din4108-3.saturation.ice.p0", S0, "Annex A", "p_0 over ice", Number(610.5), "Pa", S1).ed(S2),
    c("ndp.din4108-3.saturation.water.a", S0, "Annex A", "a over water", Number(17.269), "", S1).ed(S2),
    c("ndp.din4108-3.saturation.water.b", S0, "Annex A", "b over water", Number(237.3), S4, S1).ed(S2),
    c("ndp.din4108-3.saturation.water.p0", S0, "Annex A", "p_0 over water", Number(610.5), "Pa", S1).ed(S2),
    c("ndp.en-iso-6946.combined.max_ratio", "EN ISO 6946", "6.7.2.4", "R_upper/R_lower validity limit", Number(1.5), "", "EN ISO 6946"),
];
