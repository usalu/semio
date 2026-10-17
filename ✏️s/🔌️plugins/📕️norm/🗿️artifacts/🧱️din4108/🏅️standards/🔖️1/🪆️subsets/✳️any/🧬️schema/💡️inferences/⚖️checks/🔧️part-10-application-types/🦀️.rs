//! 🔧️ DIN 4108-10 of the compliance checks: the application type codes and the product property rows. See [`super`].

//#region 🔖️ComplianceHelpers
// #region 🔖️Part10
use crate::*;

/// 📋️ DIN 4108-10 Table 1 application-type codes.
pub const APPLICATION_TYPES: &[&str] = &[
    "DAD", "DAA", "DUK", "DZ", "DI", "DEO", "DES", "WAB", "WAA", "WAP", "WZ", "WH", "WI", "WTH", "WTR", "PW", "PB",
];

/// 🏷️ Required DIN 4108-10 application types from element kind / layer position / adjacent.
pub fn required_application_types(element: &EnvelopeElement, layer_index: usize) -> &'static [&'static str] {
    let n = element.layers.len().max(1);
    let exteriorish = layer_index >= n / 2;
    match element.kind.as_str() {
        "roof" => {
            if exteriorish { &["DAD", "DAA", "DUK", "DZ"] } else { &["DI", "DZ"] }
        }
        "floor" => {
            if element.adjacent == "ground" { &["DEO", "DES", "PW"] } else { &["DEO", "DES", "DI"] }
        }
        "wall" | "frameOpaque" | "rollerShutterBox" => {
            if element.adjacent == "unheated" || element.adjacent == "otherHeated" {
                &["WI", "WZ", "WTH"]
            } else if exteriorish {
                &["WAP", "WAB", "WAA", "WH"]
            } else {
                &["WI", "WZ", "WTH", "WTR"]
            }
        }
        _ => APPLICATION_TYPES,
    }
}

fn class_rank_compressive(code: &str) -> i32 {
    match code { "dx" => 5, "dk" => 4, "dm" => 3, "ds" => 2, "dh" => 1, _ => 0 }
}

fn class_rank_water(code: &str) -> i32 {
    match code { "wd" => 3, "wf" => 2, "wk" => 1, _ => 0 }
}

fn class_rank_tensile(code: &str) -> i32 {
    match code { "tf" => 2, "tk" => 1, _ => 0 }
}

fn class_rank_acoustic(code: &str) -> i32 {
    match code { "sg" => 3, "sm" => 2, "sh" => 1, _ => 0 }
}

fn min_compressive_for(app: &str) -> &'static str {
    match app {
        "DEO" | "DES" | "PW" | "PB" => "dk",
        "WAP" | "WAB" | "DAD" | "DAA" => "dm",
        _ => "dh",
    }
}

/// 💧 DIN 4108-10 Table 1 minimum water-absorption class for an application type.
pub fn min_water_for(app: &str) -> &'static str {
    match app {
        "WAP" | "WAB" | "WAA" | "PW" | "PB" | "DUK" => "wf",
        "DEO" | "DES" => "wd",
        _ => "wk",
    }
}

/// 🪢 DIN 4108-10 Table 1 minimum tensile class for an application type.
pub fn min_tensile_for(app: &str) -> &'static str {
    match app {
        "WAP" | "WAB" | "DEO" | "DES" | "PW" | "PB" => "tf",
        _ => "tk",
    }
}

/// 🔊 DIN 4108-10 Table 1 minimum acoustic class for an application type.
pub fn min_acoustic_for(app: &str) -> &'static str {
    match app {
        "DES" | "DEO" => "sm",
        _ => "sh",
    }
}

/// ✅️ DIN 4108-10 application-type + property-class suitability (Table 1).
pub fn check_application(element: &EnvelopeElement, layer_index: usize) -> CheckResult {
    let layer = &element.layers[layer_index];
    let subject = SubjectRef::new(
        &layer.id,
        layer_field_path(&element.id, &layer.id, "applicationType"),
        loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
    );
    let insulation = layer.lambda < 0.1 && layer.thickness_m > 0.0;
    let tagged = !(layer.application_type.is_empty()
        && layer.compressive_class.is_empty()
        && layer.water_class.is_empty()
        && layer.tensile_class.is_empty()
        && layer.acoustic_class.is_empty());
    if !insulation && !tagged {
        return CheckResult::assess(
            format!("din4108-10.app.{}.{}", element.id, layer.id),
            "DIN 4108-10",
            ClauseId::new("DIN 4108-10", "Table 1", "application"),
            subject,
            loc("Insulation application type", "Anwendungsgebiet Dämmstoff"),
        )
        .not_applicable(loc(
            "Layer is not an insulation product under DIN 4108-10.",
            "Schicht ist kein Dämmstoff nach DIN 4108-10.",
        ))
        .annex(AnnexChoice::De)
        .build();
    }
    let required = required_application_types(element, layer_index);
    let ok_type = insulation && required.iter().any(|c| *c == layer.application_type.as_str());
    let ref_app = if ok_type { layer.application_type.as_str() } else { required[0] };
    let min_c = min_compressive_for(ref_app);
    let min_w = min_water_for(ref_app);
    let min_t = min_tensile_for(ref_app);
    let min_a = min_acoustic_for(ref_app);
    let c_have = class_rank_compressive(&layer.compressive_class);
    let w_have = class_rank_water(&layer.water_class);
    let t_have = class_rank_tensile(&layer.tensile_class);
    let a_have = class_rank_acoustic(&layer.acoustic_class);
    let c_need = class_rank_compressive(min_c);
    let w_need = class_rank_water(min_w);
    let t_need = class_rank_tensile(min_t);
    let a_need = class_rank_acoustic(min_a);
    let ok_c = c_have >= c_need;
    let ok_w = w_have >= w_need;
    let ok_t = t_have >= t_need;
    let ok_a = a_have >= a_need;
    let type_idx = APPLICATION_TYPES.iter().position(|t| *t == layer.application_type.as_str()).map(|i| i as i32 + 1).unwrap_or(0);
    let req_idx = APPLICATION_TYPES.iter().position(|t| *t == ref_app).map(|i| i as i32 + 1).unwrap_or(0);
    let score = c_have + w_have + t_have + a_have + type_idx;
    let need = c_need + w_need + t_need + a_need + req_idx;
    let pass = ok_type && ok_c && ok_w && ok_t && ok_a && insulation;
    let mut builder = CheckResult::assess(
        format!("din4108-10.app.{}.{}", element.id, layer.id),
        "DIN 4108-10",
        ClauseId::new("DIN 4108-10", "Table 1", "application"),
        subject.clone(),
        loc("Insulation application type", "Anwendungsgebiet Dämmstoff"),
    )
    .minimum(
        Quantity::new(QuantityKind::Dimensionless, score as f64),
        Quantity::new(QuantityKind::Dimensionless, need as f64),
    )
    .status(if pass { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!(
            "Application '{}' classes C/W/T/A {}/{}/{}/{} vs min {}/{}/{}/{} for {:?}.",
            layer.application_type, layer.compressive_class, layer.water_class, layer.tensile_class, layer.acoustic_class, min_c, min_w, min_t, min_a, required
        ),
        &format!(
            "Anwendung '{}' Klassen D/W/Z/A {}/{}/{}/{} gegenüber Mindest {}/{}/{}/{} für {:?}.",
            layer.application_type, layer.compressive_class, layer.water_class, layer.tensile_class, layer.acoustic_class, min_c, min_w, min_t, min_a, required
        ),
    ));
    if !insulation {
        builder = builder.remedy(Remedy::one_of(
            SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "applicationType"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
            vec![String::new()],
            loc(
                "Clear DIN 4108-10 application tags on non-insulation layers.",
                "DIN-4108-10-Anwendungsmarkierungen bei Nicht-Dämmstoffen leeren.",
            ),
        ));
    }
    if !ok_type {
        builder = builder.remedy(Remedy::one_of(
            SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "applicationType"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
            required.iter().map(|c| (*c).to_string()).collect(),
            loc(&format!("Set application type to one of {:?}.", required), &format!("Anwendungsgebiet auf eines von {:?} setzen.", required)),
        ));
    }
    if !ok_c {
        let compressive = ["dh", "ds", "dm", "dk", "dx"];
        let start = compressive.iter().position(|c| class_rank_compressive(c) >= c_need).unwrap_or(0);
        builder = builder.remedy(Remedy::one_of(
            SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "compressiveClass"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
            compressive[start..].iter().map(|c| (*c).to_string()).collect(),
            loc(&format!("Raise compressive class to at least '{min_c}'."), &format!("Druckfestigkeitsklasse mindestens '{min_c}' wählen.")),
        ));
    }
    if !ok_w {
        let water = ["wk", "wf", "wd"];
        let start = water.iter().position(|c| class_rank_water(c) >= w_need).unwrap_or(0);
        builder = builder.remedy(Remedy::one_of(
            SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "waterClass"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
            water[start..].iter().map(|c| (*c).to_string()).collect(),
            loc(&format!("Raise water class to at least '{min_w}'."), &format!("Wasseraufnahmeklasse mindestens '{min_w}' wählen.")),
        ));
    }
    if !ok_t {
        let tensile = ["tk", "tf"];
        let start = tensile.iter().position(|c| class_rank_tensile(c) >= t_need).unwrap_or(0);
        builder = builder.remedy(Remedy::one_of(
            SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "tensileClass"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
            tensile[start..].iter().map(|c| (*c).to_string()).collect(),
            loc(&format!("Raise tensile class to at least '{min_t}'."), &format!("Zugfestigkeitsklasse mindestens '{min_t}' wählen.")),
        ));
    }
    if !ok_a {
        let acoustic = ["sh", "sm", "sg"];
        let start = acoustic.iter().position(|c| class_rank_acoustic(c) >= a_need).unwrap_or(0);
        builder = builder.remedy(Remedy::one_of(
            SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "acousticClass"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
            acoustic[start..].iter().map(|c| (*c).to_string()).collect(),
            loc(&format!("Raise acoustic class to at least '{min_a}'."), &format!("Schwingungsgruppe mindestens '{min_a}' wählen.")),
        ));
    }
    builder.build()
}

/// 📋 Application types published in the catalogue Table 1 view.
pub const CATALOGUE_APPLICATION_TYPES: &[&str] = &[
    "WAP", "WAB", "WAA", "DAD", "DAA", "DEO", "DES", "WZ", "WI", "PW", "PB",
];

/// 📋 Catalogue row: (application, min compressive, min water, min tensile, min acoustic) from the same helpers evaluate uses.
pub fn application_property_row(app: &'static str) -> (&'static str, &'static str, &'static str, &'static str, &'static str) {
    (app, min_compressive_for(app), min_water_for(app), min_tensile_for(app), min_acoustic_for(app))
}
// #endregion 🔖️Part10
//#endregion 🔖️ComplianceHelpers
