//! 📜️ EN 1990 nationally determined parameters, ψ tables and reliability tables: the single data source of every factor the checks use. See [`super`].

use super::*;

//#region 🔖️NdpRegistry
/// 🔎️ The value of one NDP (`ndp.en1990.*`) of the shared registry for `annex`, `None` when the id is unknown or not a number.
pub fn ndp_lookup(annex: AnnexChoice, id: &str) -> Option<f64> {
    semio_s_plugin_norm_compliance::ndp_number_opt(id, annex)
}

/// 🕳️ Whether the registry holds the id for `annex` but no value yet (a table cell still awaiting its transcription); an unknown id is not awaiting but missing.
pub fn ndp_awaits_value(annex: AnnexChoice, id: &str) -> bool {
    semio_s_plugin_norm_compliance::ndp(id).is_some_and(|entry| matches!(if annex == AnnexChoice::En { entry.recommended } else { entry.na_de }, semio_s_plugin_norm_compliance::NdpValue::Unset))
}

/// 🔢️ Number of an NDP for ids the artifact itself guarantees; a missing value is logged as an `NdpGap` and the check that rests on it is refused (`semio_s_plugin_norm_compliance::ndp_number`).
pub fn ndp(annex: AnnexChoice, id: &str) -> f64 {
    semio_s_plugin_norm_compliance::ndp_number(id, annex)
}

//#endregion 🔖️NdpRegistry

//#region 🔖️StructureDomain
/// 🏗️ Annex A1 (buildings) or Annex A2 (bridges) selected by the structure kind, shared with every Eurocode artifact.
pub use semio_s_plugin_norm_compliance::action_factors::Domain;

/// 🏷️ The permanent action kinds the artifact accepts.
pub const PERMANENT_KINDS: &[&str] = &["g_sup", "g_inf", "prestress"];

/// 🚦️ The mutually exclusive traffic load group (EN 1990 Tables A2.1-A2.3, EN 1991-2 load groups) a bridge traffic category belongs to; the components of one group share a group id and never act together with another group.
pub fn traffic_group(category: &str) -> Option<&'static str> {
    match category {
        "road_traffic" | "road_udl" | "road_pedestrian" => Some("gr1a"),
        "road_single_axle" => Some("gr1b"),
        "road_horizontal" => Some("gr2"),
        "road_gr3_pedestrian" => Some("gr3"),
        "road_gr4_crowd" => Some("gr4"),
        "road_special" => Some("gr5"),
        "footbridge_crowd" => Some("fb-crowd"),
        "footbridge_service" => Some("fb-service"),
        "rail_traffic" => Some("lm71"),
        "rail_sw2" => Some("sw2"),
        "rail_unloaded" => Some("unloaded"),
        "rail_hslm" => Some("hslm"),
        _ => None,
    }
}

/// 🌬️ The non-simultaneity family of a variable action category (Annex A2.2.2): wind and thermal actions are never combined; `None` for every other category.
pub fn non_simultaneous_family(category: &str) -> Option<&'static str> {
    match category {
        "wind_bridge" | "wind_rail" => Some("wind"),
        "temperature_bridge" => Some("thermal"),
        _ => None,
    }
}

/// 🚧️ Whether a variable action category acts in transient execution situations only (Annex A2: F*_W, Q_Sn,k and Q_c of EN 1991-1-6) and never in the persistent situation.
pub fn is_execution_category(category: &str) -> bool {
    matches!(category, "wind_bridge_execution" | "snow_construction" | "construction_load")
}
//#endregion 🔖️StructureDomain

//#region 🔖️PartialFactorSets
/// ⚖️ Partial-factor sets, γ_G, γ_Q, ξ and γ_P: one registry-backed source shared with every Eurocode artifact.
pub use semio_s_plugin_norm_compliance::action_factors::{gamma_g, gamma_p, gamma_q, traffic_family, xi, FactorSet};

/// 🔢️ K_FI of Table B3 for a reliability class, `None` outside RC1..RC3.
pub fn k_fi_for_rc(reliability_class: u8) -> Option<f64> {
    semio_s_plugin_norm_compliance::ReliabilityClass::from_number(reliability_class).map(|class| class.k_fi(AnnexChoice::En))
}

/// 🔢️ Whether Eq. 6.10a/6.10b are offered: NA-DE offers Eq. 6.10 only, and Annex A2 (bridges) uses Eq. 6.10 only (EN 1990 A2.3.1).
pub fn offers_610ab(annex: AnnexChoice, domain: Domain) -> bool {
    annex != AnnexChoice::De && domain == Domain::Building
}
//#endregion 🔖️PartialFactorSets

//#region 🔖️PsiTables
/// 📊️ ψ factors for one action category, shared with every Eurocode artifact.
pub use semio_s_plugin_norm_compliance::action_factors::PsiRow;

/// 📊️ One ψ table row: category key, labels and source table; the values are the registry entries `ndp.en1990.psi.<key>.<0|1|2>`.
#[derive(Clone, Copy, Debug)]
pub struct PsiEntry {
    pub key: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub table: &'static str,
}

const fn psi_entry(key: &'static str, label_en: &'static str, label_de: &'static str, table: &'static str) -> PsiEntry {
    PsiEntry { key, label_en, label_de, table }
}

impl PsiEntry {
    fn id(&self, index: u8) -> String {
        format!("ndp.en1990.psi.{}.{index}", self.key)
    }

    /// 🔢️ The ψ0, ψ1, ψ2 row of the annex, read from the shared registry.
    pub fn row(&self, annex: AnnexChoice) -> PsiRow {
        semio_s_plugin_norm_compliance::action_factors::psi_row_of_key(annex, self.key).unwrap_or(PsiRow { psi_0: f64::NAN, psi_1: f64::NAN, psi_2: f64::NAN })
    }

    /// 🛡️ Whether all three registry values are verified against the printed standard.
    pub fn verified(&self) -> bool {
        (0..3).all(|index| semio_s_plugin_norm_compliance::ndp_is_verified(&self.id(index)))
    }
}

/// 📚️ The ψ table of Annex A1 and A2.
pub const PSI_TABLE: &[PsiEntry] = &[
    psi_entry("residential", "Imposed A residential", "Nutzlast A Wohnen", "A1.1"),
    psi_entry("office", "Imposed B offices", "Nutzlast B Büro", "A1.1"),
    psi_entry("congregation", "Imposed C congregation", "Nutzlast C Versammlung", "A1.1"),
    psi_entry("retail", "Imposed D shopping", "Nutzlast D Verkauf", "A1.1"),
    psi_entry("storage", "Imposed E storage", "Nutzlast E Lager", "A1.1"),
    psi_entry("traffic_light", "Traffic F (<= 30 kN)", "Verkehr F (<= 30 kN)", "A1.1"),
    psi_entry("traffic_heavy", "Traffic G (30-160 kN)", "Verkehr G (30-160 kN)", "A1.1"),
    psi_entry("roof", "Roofs H", "Dächer H", "A1.1"),
    psi_entry("snow", "Snow (<= 1000 m)", "Schnee (<= 1000 m)", "A1.1"),
    psi_entry("snow_high", "Snow (> 1000 m)", "Schnee (> 1000 m)", "A1.1"),
    psi_entry("wind", "Wind", "Wind", "A1.1"),
    psi_entry("temperature", "Temperature (not fire)", "Temperatur (nicht Brand)", "A1.1"),
    psi_entry("road_traffic", "Road gr1a tandem system", "Straße gr1a Tandemsystem", "A2.1"),
    psi_entry("road_udl", "Road gr1a uniform load", "Straße gr1a Flächenlast", "A2.1"),
    psi_entry("road_pedestrian", "Road gr1a pedestrians and cycles", "Straße gr1a Fußgänger und Radfahrer", "A2.1"),
    psi_entry("road_single_axle", "Road gr1b single axle", "Straße gr1b Einzelachse", "A2.1"),
    psi_entry("road_horizontal", "Road gr2 horizontal forces", "Straße gr2 Horizontalkräfte", "A2.1"),
    psi_entry("road_gr3_pedestrian", "Road gr3 pedestrians", "Straße gr3 Fußgänger", "A2.1"),
    psi_entry("road_gr4_crowd", "Road gr4 crowd loading", "Straße gr4 Menschenansammlung", "A2.1"),
    psi_entry("road_special", "Road gr5 special vehicles", "Straße gr5 Sonderfahrzeuge", "A2.1"),
    psi_entry("wind_bridge", "Wind F_Wk persistent", "Wind F_Wk ständig", "A2.1"),
    psi_entry("wind_bridge_execution", "Wind F*_W execution", "Wind F*_W Bauzustand", "A2.1"),
    psi_entry("temperature_bridge", "Thermal T_k", "Temperatur T_k", "A2.1"),
    psi_entry("snow_construction", "Snow Q_Sn,k construction", "Schnee Q_Sn,k Bauzustand", "A2.1"),
    psi_entry("construction_load", "Construction loads Q_c", "Bauzustandslasten Q_c", "A2.1"),
    psi_entry("footbridge_crowd", "Footbridge crowd Q_fwk", "Fußgängerbrücke Menschengedränge Q_fwk", "A2.2"),
    psi_entry("footbridge_service", "Footbridge service vehicle Q_serv", "Fußgängerbrücke Dienstfahrzeug Q_serv", "A2.2"),
    psi_entry("rail_traffic", "Rail LM71 / SW/0 groups", "Schiene LM71 / SW/0 Gruppen", "A2.3"),
    psi_entry("rail_sw2", "Rail SW/2", "Schiene SW/2", "A2.3"),
    psi_entry("rail_unloaded", "Rail unloaded train", "Schiene Leerzug", "A2.3"),
    psi_entry("rail_hslm", "Rail HSLM", "Schiene HSLM", "A2.3"),
    psi_entry("wind_rail", "Rail wind", "Schiene Wind", "A2.3"),
    psi_entry("settlement", "Settlements", "Setzungen", "NA.A.1.1"),
    psi_entry("variable_other", "Other variable actions", "Sonstige veränderliche Einwirkungen", "NA.A.1.1"),
];

/// 🔤️ The canonical registry key of an imposed-load category such as `B` or `B2`, and the snow altitude switch, shared with every Eurocode artifact.
pub use semio_s_plugin_norm_compliance::action_factors::{canonical_category, resolve_psi_category};

/// 🔎️ The ψ table entry of a category.
pub fn psi_entry_of(category: &str) -> Option<&'static PsiEntry> {
    let key = canonical_category(category);
    PSI_TABLE.iter().find(|entry| entry.key == key)
}

/// 🔎️ The ψ row of a category in an annex, `None` for an unknown category.
pub fn psi_lookup(annex: AnnexChoice, category: &str) -> Option<PsiRow> {
    psi_entry_of(category).map(|entry| entry.row(annex))
}

/// 📊️ The ψ row of a category for display tables; an unknown category shows the most unfavourable row (1, 1, 1).
pub fn psi_for_category<A: NationalAnnex>(annex: &A, category: &str) -> PsiRow {
    psi_lookup(annex.choice(), category).unwrap_or(PsiRow { psi_0: 1.0, psi_1: 1.0, psi_2: 1.0 })
}

/// 📊️ The ψ row of an imposed-load category for display tables.
pub fn psi_for_imposed<A: NationalAnnex>(annex: &A, category: ImposedCategory) -> PsiRow {
    psi_for_category(annex, category.label())
}

/// ✅️ Whether ψ0 ≥ ψ1 ≥ ψ2 ≥ 0 holds (EN 1990 4.1.3 plausibility).
pub fn psi_is_ordered(row: PsiRow) -> bool {
    row.psi_0 >= row.psi_1 && row.psi_1 >= row.psi_2 && row.psi_2 >= 0.0
}

//#endregion 🔖️PsiTables

//#region 🔖️ReliabilityTables
/// 📅 EN 1990 Table 2.1 indicative design working life [years] of a category.
pub fn design_working_life_indicative(category: u8) -> Option<(f64, f64)> {
    match category {
        1 => Some((10.0, 10.0)),
        2 => Some((10.0, 30.0)),
        3 => Some((15.0, 30.0)),
        4 => Some((50.0, 50.0)),
        5 => Some((100.0, 100.0)),
        _ => None,
    }
}

/// 👷 Annex B Tables B4 and B5: the recommended DSL and IL level of a reliability class.
pub fn annex_b_levels(reliability_class: u8) -> Option<(u8, u8)> {
    matches!(reliability_class, 1..=3).then_some((reliability_class, reliability_class))
}

/// 🔢️ Parses `DSL1..DSL3` or `IL1..IL3` strictly; any other spelling is `None`.
pub fn parse_level(text: &str, prefix: &str) -> Option<u8> {
    let digit = text.strip_prefix(prefix)?;
    match digit {
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        _ => None,
    }
}

/// 📐️ Target reliability index β (Table B2): the tabulated 1-year and 50-year columns are anchors; between them β follows the table linearly in ln T (continuous and monotonic), outside them the conversion p_T = 1 − (1 − p_a)^(T / a) starts from the nearest anchor a.
pub fn target_reliability_index_for(reliability_class: u8, reference_period_years: f64) -> Option<f64> {
    if !matches!(reliability_class, 1..=3) || !(reference_period_years > 0.0) {
        return None;
    }
    let b1 = ndp(AnnexChoice::En, &format!("ndp.en1990.beta1.{reliability_class}"));
    let b50 = ndp(AnnexChoice::En, &format!("ndp.en1990.beta50.{reliability_class}"));
    let t = reference_period_years;
    if (1.0..=50.0).contains(&t) {
        return Some(b1 + (b50 - b1) * t.ln() / 50.0_f64.ln());
    }
    let (anchor_years, anchor_beta) = if t < 1.0 { (1.0, b1) } else { (50.0, b50) };
    let p_anchor = phi_cdf(-anchor_beta);
    let p_t = -((t / anchor_years) * (-p_anchor).ln_1p()).exp_m1();
    Some(-phi_inv(p_t))
}
//#endregion 🔖️ReliabilityTables
