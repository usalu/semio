//! 📊️ DIN 4108-4 of the compliance checks: the design thermal conductivity tables and the catalogue checks. See [`super`].

//#region 🔖️ComplianceHelpers
// #region 🔖️Part4
use crate::*;

/// 📊️ DIN 4108-4 Table 1 design thermal conductivity λ [W/(m·K)] — shared with catalogue.
pub const DESIGN_LAMBDA_ROWS: &[(&str, f64)] = &[
    ("mineral_wool", 0.035),
    ("glass_wool", 0.035),
    ("rock_wool", 0.035),
    ("eps", 0.035),
    ("eps_w", 0.035),
    ("xps", 0.035),
    ("pu", 0.028),
    ("pir", 0.028),
    ("brick", 0.81),
    ("masonry", 0.81),
    ("concrete", 2.1),
    ("gypsum_plaster", 0.70),
    ("gypsum", 0.70),
    ("mineral_render", 0.70),
    ("screed", 1.4),
    ("timber", 0.13),
    ("wood", 0.13),
    ("softwood", 0.13),
    ("osb", 0.13),
    ("aerated_concrete", 0.16),
    ("lime_sandstone", 0.99),
];

/// 📊️ DIN 4108-4 design λ for a catalogue material id.
pub fn design_lambda(material_id: &str) -> Option<f64> {
    DESIGN_LAMBDA_ROWS.iter().find(|(id, _)| *id == material_id).map(|(_, v)| *v)
}

/// 📊️ Catalogue material ids for referential integrity remedies.
pub fn design_lambda_material_ids() -> Vec<String> {
    DESIGN_LAMBDA_ROWS.iter().map(|(id, _)| (*id).to_string()).collect()
}

/// ✅️ Declared λ of each layer must not exceed DIN 4108-4 design value.
pub fn check_design_lambda(element: &EnvelopeElement, layer_index: usize) -> CheckResult {
    let layer = &element.layers[layer_index];
    let subject = SubjectRef::new(
        &element.id,
        layer_field_path(&element.id, &layer.id, "lambda"),
        loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
    );
    let Some(limit) = design_lambda(&layer.material_id) else {
        return CheckResult::assess(
            format!("din4108-4.lambda.{}.{}", element.id, layer.id),
            "DIN 4108-4",
            ClauseId::new("DIN 4108-4", "Table 1", "λ"),
            subject,
            loc("Design thermal conductivity", "Bemessungswärmeleitfähigkeit"),
        )
        .not_applicable(loc(
            &format!("Material '{}' has no DIN 4108-4 row in this catalogue.", layer.material_id),
            &format!("Werkstoff '{}' ist in diesem Katalog nicht tabelliert.", layer.material_id),
        ))
        .annex(AnnexChoice::De)
        .build();
    };
    let mut builder = CheckResult::assess(
        format!("din4108-4.lambda.{}.{}", element.id, layer.id),
        "DIN 4108-4",
        ClauseId::new("DIN 4108-4", "Table 1", "λ"),
        subject.clone(),
        loc("Design thermal conductivity", "Bemessungswärmeleitfähigkeit"),
    )
    .utilization(
        Quantity::new(QuantityKind::ThermalConductivity, layer.lambda),
        Quantity::new(QuantityKind::ThermalConductivity, limit),
    )
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!("λ = {:.3} W/(m·K) vs λ_design = {:.3} for '{}'.", layer.lambda, limit, layer.material_id),
        &format!("λ = {:.3} W/(m·K) gegenüber λ_Bemessung = {:.3} für '{}'.", layer.lambda, limit, layer.material_id),
    ));
    if layer.lambda > limit {
        builder = builder.remedy(Remedy::at_most(
            subject,
            Quantity::new(QuantityKind::ThermalConductivity, layer.lambda),
            Quantity::new(QuantityKind::ThermalConductivity, limit),
            loc(
                &format!("Reduce declared λ of '{}' from {:.3} to ≤ {:.3} W/(m·K) or change material_id.", layer.id, layer.lambda, limit),
                &format!("Deklariertes λ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) senken oder materialId wechseln.", layer.id, layer.lambda, limit),
            ),
        ));
    }
    builder.build()
}

/// ✅️ DIN 4108-4 λ for one inhomogeneous segment (ISO 6946 §6.7 / DIN 4108-4 Table 1).
pub fn check_segment_design_lambda(element: &EnvelopeElement, layer_index: usize, segment_index: usize) -> CheckResult {
    let layer = &element.layers[layer_index];
    let seg = &layer.segments[segment_index];
    let subject = SubjectRef::new(
        &seg.id,
        format!("elements[id={}].layers[id={}].segments[id={}].lambda", element.id, layer.id, seg.id),
        loc(&format!("Segment {}", seg.id), &format!("Abschnitt {}", seg.id)),
    );
    let Some(limit) = design_lambda(&seg.material_id) else {
        return CheckResult::assess(
            format!("din4108-4.lambda.{}.{}.{}", element.id, layer.id, seg.id),
            "DIN 4108-4",
            ClauseId::new("DIN 4108-4", "Table 1", "λ"),
            subject,
            loc("Design thermal conductivity", "Bemessungswärmeleitfähigkeit"),
        )
        .not_applicable(loc(
            &format!("Segment material '{}' has no DIN 4108-4 row.", seg.material_id),
            &format!("Abschnittswerkstoff '{}' ist nicht tabelliert.", seg.material_id),
        ))
        .annex(AnnexChoice::De)
        .build();
    };
    let mut builder = CheckResult::assess(
        format!("din4108-4.lambda.{}.{}.{}", element.id, layer.id, seg.id),
        "DIN 4108-4",
        ClauseId::new("DIN 4108-4", "Table 1", "λ"),
        subject.clone(),
        loc("Design thermal conductivity", "Bemessungswärmeleitfähigkeit"),
    )
    .utilization(
        Quantity::new(QuantityKind::ThermalConductivity, seg.lambda),
        Quantity::new(QuantityKind::ThermalConductivity, limit),
    )
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!("Segment λ = {:.3} vs λ_design = {:.3} for '{}' (ρ={:.0}, μ={:.1}).", seg.lambda, limit, seg.material_id, seg.density, seg.mu),
        &format!("Abschnitt λ = {:.3} gegenüber λ_design = {:.3} für '{}' (ρ={:.0}, μ={:.1}).", seg.lambda, limit, seg.material_id, seg.density, seg.mu),
    ));
    if seg.lambda > limit {
        builder = builder.remedy(Remedy::at_most(
            subject,
            Quantity::new(QuantityKind::ThermalConductivity, seg.lambda),
            Quantity::new(QuantityKind::ThermalConductivity, limit),
            loc(
                &format!("Reduce segment λ of '{}' from {:.3} to ≤ {:.3} W/(m·K).", seg.id, seg.lambda, limit),
                &format!("Abschnitts-λ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) senken.", seg.id, seg.lambda, limit),
            ),
        ));
    }
    builder.build()
}
// #endregion 🔖️Part4
//#endregion 🔖️ComplianceHelpers
