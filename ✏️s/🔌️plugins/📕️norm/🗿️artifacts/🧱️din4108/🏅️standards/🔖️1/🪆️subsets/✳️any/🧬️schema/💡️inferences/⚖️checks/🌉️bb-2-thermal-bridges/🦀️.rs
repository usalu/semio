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

/// 📐️ DIN 4108 Beiblatt 2 §5 / GEG: Kategorie A → ΔU_WB = 0.05 W/(m²K).
pub const DELTA_U_WB_CATEGORY_A: f64 = 0.05;
/// 📐️ DIN 4108 Beiblatt 2 §5: Kategorie B / non-conforming catalogue → ΔU_WB = 0.10 W/(m²K).
pub const DELTA_U_WB_CATEGORY_B: f64 = 0.10;
/// 📐️ High-performance equivalent details (GEG reduced surcharge) → 0.03 W/(m²K) when all bridges are Kategorie A and details_conform.
pub const DELTA_U_WB_CATEGORY_A_STRICT: f64 = 0.03;

/// 🏷️ Parse bridge bb2Type into equivalence category.
pub fn bb2_category(bb2_type: &str) -> &'static str {
    match bb2_type {
        "categoryA" | "A" | "W01" | "D01" | "conform" => BB2_CATEGORY_A,
        "categoryB" | "B" => BB2_CATEGORY_B,
        _ => BB2_DETAILED,
    }
}

/// 📐️ Admissible flat ΔU_WB from bridge categories + building conform flag (DIN 4108 Bbl.2 §5).
pub fn delta_u_wb_limit(bridges: &[ThermalBridge], details_conform: bool) -> f64 {
    if bridges.is_empty() {
        return if details_conform { DELTA_U_WB_CATEGORY_A } else { DELTA_U_WB_CATEGORY_B };
    }
    let cats: Vec<&str> = bridges.iter().map(|b| bb2_category(&b.bb2_type)).collect();
    if cats.iter().any(|c| *c == BB2_DETAILED) {
        // Detailed ψ·l method: still compare Σψ·l/A against the catalogue flat for the remaining class.
        if cats.iter().all(|c| *c == BB2_DETAILED || *c == BB2_CATEGORY_A) && details_conform {
            DELTA_U_WB_CATEGORY_A
        } else {
            DELTA_U_WB_CATEGORY_B
        }
    } else if cats.iter().all(|c| *c == BB2_CATEGORY_A) {
        if details_conform { DELTA_U_WB_CATEGORY_A_STRICT } else { DELTA_U_WB_CATEGORY_A }
    } else {
        DELTA_U_WB_CATEGORY_B
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

/// ✅️ Beiblatt 2 thermal-bridge equivalence.
pub fn check_equivalence(bridges: &[ThermalBridge], area_m2: f64, details_conform: bool) -> CheckResult {
    let subject = SubjectRef::new("", "thermalBridges", loc("Thermal bridges", "Wärmebrücken"));
    if area_m2 <= 0.0 {
        return CheckResult::assess(
            "din4108-bb2.delta-u",
            "DIN 4108 Bbl.2",
            ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
            subject,
            loc("Thermal bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB"),
        )
        .not_applicable(loc("Envelope area is zero.", "Hüllfläche ist null."))
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
        loc("Thermal bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB"),
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
                ((psi_l_max - other) / bridge.length_m).max(0.0)
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
                    &format!("Reduce ψ of '{}' from {:.3} to ≤ {:.3} W/(m·K), or set bb2DetailsConform with redesigned details.", bridge.id, bridge.psi, psi_req),
                    &format!("ψ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) senken oder bb2DetailsConform mit geänderten Details setzen.", bridge.id, bridge.psi, psi_req),
                ),
            ));
        }
        if !details_conform {
            builder = builder.remedy(Remedy::exactly(
                SubjectRef::new("", "bb2DetailsConform", loc("Beiblatt 2 conformity", "Beiblatt-2-Konformität")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                loc(
                    "Redesign thermal-bridge details to Beiblatt 2 conformity (allows ΔU_WB ≤ 0.05 W/(m²K)).",
                    "Wärmebrückendetails nach Beiblatt 2 ausführen (dann ΔU_WB ≤ 0,05 W/(m²K)).",
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
    let cat = bb2_category(&bridge.bb2_type);
    let detailed = cat == BB2_DETAILED;
    let mut builder = CheckResult::assess(
        format!("din4108-bb2.bridge.{}", bridge.id),
        "DIN 4108 Bbl.2",
        ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
        subject.clone(),
        loc("Thermal-bridge equivalence category", "Wärmebrücken-Äquivalenzkategorie"),
    )
    .utilization(
        Quantity::new(QuantityKind::Dimensionless, if detailed { 1.1 } else { 1.0 }),
        Quantity::new(QuantityKind::Dimensionless, 1.0),
    )
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!("Bridge '{}' bb2Type='{}' → category {cat} (ψ={:.3}).", bridge.id, bridge.bb2_type, bridge.psi),
        &format!("Wärmebrücke '{}' bb2Type='{}' → Kategorie {cat} (ψ={:.3}).", bridge.id, bridge.bb2_type, bridge.psi),
    ));
    if detailed {
        builder = builder.remedy(Remedy::one_of(
            subject.clone(),
            vec![BB2_CATEGORY_A.to_string(), BB2_CATEGORY_B.to_string()],
            loc(
                "Switch bb2Type to an equivalent Beiblatt 2 catalogue category (A or B), or keep detailed ψ with full documentation.",
                "bb2Type auf eine äquivalente Beiblatt-2-Kategorie (A oder B) umstellen, oder detailliertes ψ mit vollständiger Dokumentation belassen.",
            ),
        ));
        if bridge.psi > 0.15 {
            builder = builder.remedy(Remedy::at_most(
                SubjectRef::new(
                    &bridge.id,
                    bridge_field_path(&bridge.id, "psi"),
                    loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
                ),
                Quantity::new(QuantityKind::HeatTransferCoefficient, bridge.psi),
                Quantity::new(QuantityKind::HeatTransferCoefficient, 0.15),
                loc(
                    &format!("Reduce ψ of '{}' from {:.3} to ≤ 0.15 W/(m·K) for detailed proof.", bridge.id, bridge.psi),
                    &format!("ψ von '{}' von {:.3} auf ≤ 0,15 W/(m·K) für den detaillierten Nachweis senken.", bridge.id, bridge.psi),
                ),
            ));
        }
    }
    builder.build()
}
// #endregion 🔖️BB2
//#endregion 🔖️ComplianceHelpers
