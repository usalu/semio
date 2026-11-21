//! 🌉️ DIN 4108 Beiblatt 2 of the compliance checks: the thermal bridge categories and the equivalence checks. See [`super`].

//#region 🔖️ComplianceHelpers
// #region 🔖️BB2
use crate::*;

pub const DELTA_U_WB_CONFORM: f64 = 0.05;
pub const DELTA_U_WB_FLAT: f64 = 0.10;

/// 🏷️ Beiblatt 2 equivalence category wire values.
pub const BB2_CATEGORY_A: &str = "categoryA";
pub const BB2_CATEGORY_B: &str = "categoryB";
pub const BB2_DETAILED: &str = "detailed";

/// 📐️ Flat surcharge for a building whose connections meet at least category A.
pub const DELTA_U_WB_CATEGORY_A: f64 = 0.05;

/// 🧊 Flat surcharge for a building whose connections all meet category B.
pub const DELTA_U_WB_CATEGORY_B: f64 = 0.03;

/// 🏷️ Declared thermal-bridge category in the project model.
pub fn bb2_category(bb2_type: &str) -> &'static str {
    match bb2_type {
        "categoryA" | "A" | "W01" | "D01" | "conform" => BB2_CATEGORY_A,
        "categoryB" | "B" => BB2_CATEGORY_B,
        _ => BB2_DETAILED,
    }
}

/// 📐️ Flat surcharge supported by the documented equivalence, in W/(m²K).
/// See DIN V 18599-2:2018-09 §6.2.5 and DIN 4108 Beiblatt 2:2019-06.
pub fn delta_u_wb_limit(bridges: &[ThermalBridge], details_conform: bool) -> f64 {
    if !details_conform || bridges.is_empty() {
        return DELTA_U_WB_FLAT;
    }
    let mut categories = bridges.iter().map(|bridge| bb2_category(&bridge.bb2_type));
    if categories.clone().any(|category| category == BB2_DETAILED) {
        DELTA_U_WB_FLAT
    } else if categories.all(|category| category == BB2_CATEGORY_B) {
        DELTA_U_WB_CATEGORY_B
    } else {
        DELTA_U_WB_CATEGORY_A
    }
}

/// 📐️ ΔU_WB = Σ(ψ·l)/A [W/(m²K)].
pub fn delta_u_wb(psi_l: f64, area_m2: f64) -> f64 {
    if area_m2 <= 0.0 {
        f64::INFINITY
    } else {
        psi_l / area_m2
    }
}

/// ✅️ Compare the detailed thermal-bridge contribution with the applicable flat-rate reference.
pub fn check_equivalence(bridges: &[ThermalBridge], area_m2: f64, details_conform: bool) -> CheckResult {
    let subject = SubjectRef::new("", "thermalBridges", loc("Thermal bridges", "Wärmebrücken"));
    if area_m2 <= 0.0 || bridges.is_empty() {
        return CheckResult::assess(
            "din4108-bb2.delta-u",
            "DIN 4108 Bbl.2",
            ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
            subject,
            loc("Calculated thermal-bridge surcharge against the flat rate", "Berechneter Wärmebrückenzuschlag im Vergleich zum Pauschalwert"),
        )
        .not_applicable(loc("The comparison requires a positive envelope area and documented junctions.", "Der Vergleich erfordert eine positive Hüllfläche und dokumentierte Anschlüsse."))
        .annex(AnnexChoice::De)
        .build();
    }
    let psi_l = psi_l_sum(bridges);
    let actual = delta_u_wb(psi_l, area_m2);
    let limit = delta_u_wb_limit(bridges, details_conform);
    let mut builder = CheckResult::assess(
        "din4108-bb2.delta-u",
        "DIN 4108 Bbl.2",
        ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
        subject,
        loc("Calculated thermal-bridge surcharge against the flat rate", "Berechneter Wärmebrückenzuschlag im Vergleich zum Pauschalwert"),
    )
    .utilization(Quantity::u_value_w_m2k(actual), Quantity::u_value_w_m2k(limit))
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!(
            "ΔU_WB = {actual:.3} (limit {limit:.3}) from Σψ·l = {psi_l:.3} over A = {area_m2:.1} (conform={details_conform}, cats={:?}).",
            bridges.iter().map(|b| bb2_category(&b.bb2_type)).collect::<Vec<_>>()
        ),
        &format!(
            "ΔU_WB = {actual:.3} (Grenze {limit:.3}) aus Σψ·l = {psi_l:.3} über A = {area_m2:.1} (konform={details_conform}, Kat={:?}).",
            bridges.iter().map(|b| bb2_category(&b.bb2_type)).collect::<Vec<_>>()
        ),
    ));
    if actual > limit {
        let psi_l_max = limit * area_m2;
        if let Some((bi, bridge)) = bridges.iter().enumerate().max_by(|a, b| {
            (a.1.psi * a.1.length_m)
                .partial_cmp(&(b.1.psi * b.1.length_m))
                .unwrap_or(std::cmp::Ordering::Equal)
        }) {
            let other: f64 = bridges
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != bi)
                .map(|(_, b)| b.psi * b.length_m)
                .sum();
            let psi_req = if bridge.length_m > 0.0 {
                (psi_l_max - other) / bridge.length_m
            } else {
                0.0
            };
            builder = builder.remedy(Remedy::at_most(
                SubjectRef::new(
                    &bridge.id,
                    bridge_field_path(&bridge.id, "psi"),
                    loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
                ),
                Quantity::new(QuantityKind::HeatTransferCoefficient, bridge.psi),
                Quantity::new(QuantityKind::HeatTransferCoefficient, psi_req),
                loc(
                    &format!("Reduce ψ of '{}' from {:.3} to ≤ {:.3} W/(m·K) using an assessed junction detail.", bridge.id, bridge.psi, psi_req),
                    &format!("ψ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) mit einem bewerteten Anschlussdetail senken.", bridge.id, bridge.psi, psi_req),
                ),
            ));
        }
    }
    builder.build()
}

/// ✅️ Per-bridge Beiblatt 2 equivalence category (DIN 4108 Bbl.2 §5).
pub fn check_bridge_category(bridge: &ThermalBridge) -> CheckResult {
    let subject = SubjectRef::new(
        &bridge.id,
        bridge_field_path(&bridge.id, "bb2Type"),
        loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
    );
    let category = bb2_category(&bridge.bb2_type);
    let builder = CheckResult::assess(
        format!("din4108-bb2.bridge.{}", bridge.id),
        "DIN 4108 Bbl.2",
        ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
        subject,
        loc("Declared thermal-bridge category", "Angegebene Wärmebrückenkategorie"),
    ).annex(AnnexChoice::De);
    if category == BB2_DETAILED {
        builder.not_applicable(loc(
            "This junction requires its own calculation or evidence of equivalence to a published detail.",
            "Dieser Anschluss erfordert eine eigene Berechnung oder einen Gleichwertigkeitsnachweis zu einem veröffentlichten Detail.",
        )).build()
    } else {
        builder.status(semio_s_plugin_norm_compliance::CheckStatus::Info).explanation(loc(
            &format!("Declared category {category} for junction '{}' with ψ = {:.3} W/(m·K).", bridge.id, bridge.psi),
            &format!("Angegebene Kategorie {category} für Anschluss '{}' mit ψ = {:.3} W/(m·K).", bridge.id, bridge.psi),
        )).build()
    }
}
// #endregion 🔖️BB2
//#endregion 🔖️ComplianceHelpers
