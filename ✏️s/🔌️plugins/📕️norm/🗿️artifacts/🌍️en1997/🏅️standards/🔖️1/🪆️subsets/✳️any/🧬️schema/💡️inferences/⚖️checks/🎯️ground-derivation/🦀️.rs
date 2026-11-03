//! 🎯️ Ground investigation derivation (EN 1997-2 Annex D, GI-03, GI-06, GI-07, GI-13): characteristic c_u and φ′ of a layer from the CPT and SPT readings inside it, and the typed basis (declared or derived) that decides which value the checks use. The samples of the statistics are the independent values per investigation point (the mean of the readings of one sounding inside the layer, spatial averaging), never the autocorrelated readings themselves; the rule is the registry row `ndp.en1997.2.characteristic-samples-per-sounding`, unverified.

use super::*;
use semio_s_artifact_norm_en1997_model::{CharacteristicParameter, Investigation, InvestigationKind, ParameterBasis};
use std::borrow::Cow;

/// 🔢️ Number of independent samples per layer from which the statistical derivation with the Student t factor applies (spec GI: n ≥ 3 per layer recommended); fewer samples give the cautious estimate, the lowest reading inside the layer.
pub const MIN_SAMPLES: usize = 3;

/// 🔖️ Registry row: independent samples one investigation point contributes to a layer (1 = the mean of its readings in the layer; k > 1 = k depth segments of one sounding, the scale-of-fluctuation reading), unverified.
pub const SAMPLES_PER_SOUNDING_ID: &str = "ndp.en1997.2.characteristic-samples-per-sounding";

/// 🧮️ How a characteristic value was reached.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Statistical,
    Cautious,
}

/// 📈️ One independent sample of an investigation point: the mean value of its readings inside the layer (or of one depth segment of them) at their mean depth.
#[derive(Clone, Debug, PartialEq)]
pub struct Sample {
    pub investigation: String,
    pub depth: f64,
    pub value: f64,
}

/// 🎯️ Characteristic value of one parameter of one layer with the samples it rests on.
#[derive(Clone, Debug, PartialEq)]
pub struct Derived {
    pub samples: Vec<Sample>,
    pub readings: usize,
    pub rejected: usize,
    pub method: Method,
    pub mean: f64,
    pub variation: f64,
    pub kn: f64,
    pub characteristic: f64,
}

/// 🧭️ The outcome of a typed basis row: what the layer declares and what the investigations give.
#[derive(Clone, Debug, PartialEq)]
pub struct Resolution {
    pub layer_id: String,
    pub parameter: CharacteristicParameter,
    pub declared: f64,
    pub derived: Option<Derived>,
}

impl Resolution {
    pub fn value(&self) -> f64 {
        self.derived.as_ref().map_or(0.0, |d| d.characteristic)
    }
}

/// 💧️ Unit weight of water [N/m³] of the annex (9.81 kN/m³ EN, 10 kN/m³ DE).
pub fn gamma_w_n_m3(annex: AnnexChoice) -> f64 {
    if annex == AnnexChoice::De { 10_000.0 } else { 9_810.0 }
}

/// 🧱️ Effective vertical stress σ′v0 [Pa] at `depth` from the layer unit weights γ above and γ′ below the groundwater level.
pub fn effective_stress_pa(layers: &[SoilLayer], gwl: f64, depth: f64) -> f64 {
    layers
        .iter()
        .map(|l| {
            let (top, bottom) = (l.depth_top, l.depth_bottom.min(depth));
            let thickness = (bottom - top).max(0.0);
            let dry = (bottom.min(gwl) - top).clamp(0.0, thickness);
            l.gamma * dry + l.gamma_prime * (thickness - dry)
        })
        .sum()
}

/// 🧱️ Total vertical stress σv0 [Pa] at `depth`: the effective stress plus the pore pressure below the groundwater level.
pub fn total_stress_pa(layers: &[SoilLayer], gwl: f64, annex: AnnexChoice, depth: f64) -> f64 {
    effective_stress_pa(layers, gwl, depth) + gamma_w_n_m3(annex) * (depth - gwl).max(0.0)
}

/// 🔩️ Cone factor N_kt of a layer: its own value, or the registry row `ndp.en1997.2.cpt-nkt` when the layer names none.
pub fn cone_factor(layer: &SoilLayer, annex: AnnexChoice) -> f64 {
    if layer.cpt_nkt > 0.0 { layer.cpt_nkt } else { semio_s_plugin_norm_compliance::ndp_number(catalogue::investigation::CPT_NKT_ID, annex) }
}

fn layer_containing(layers: &[SoilLayer], depth: f64) -> Option<&SoilLayer> {
    layers.iter().find(|l| depth >= l.depth_top && depth < l.depth_bottom)
}

/// 📈️ c_u [Pa] at every CPT reading of an investigation: (q_t − σv0)/N_kt with σv0 from the layer unit weights and N_kt of the layer at the depth; readings outside the layers or with q_t ≤ σv0 are left out of the profile.
pub fn cu_profile(doc: &En1997Snapshot, investigation: &Investigation) -> Vec<(f64, f64)> {
    if investigation.kind != InvestigationKind::Cpt {
        return Vec::new();
    }
    investigation
        .readings
        .iter()
        .filter_map(|r| {
            let layer = layer_containing(&doc.layers, r.depth)?;
            let net = r.measurement - total_stress_pa(&doc.layers, doc.groundwater_level, doc.annex, r.depth);
            (net > 0.0).then(|| (r.depth, net / cone_factor(layer, doc.annex)))
        })
        .collect()
}

/// 📈️ (N1)60 at every SPT reading of an investigation.
pub fn n1_60_profile(doc: &En1997Snapshot, investigation: &Investigation) -> Vec<(f64, f64)> {
    if investigation.kind != InvestigationKind::Spt {
        return Vec::new();
    }
    investigation.readings.iter().map(|r| (r.depth, part_2::n1_60(r.measurement, effective_stress_pa(&doc.layers, doc.groundwater_level, r.depth)))).collect()
}

/// 📈️ φ′ [°] at every CPT (Kulhawy-Mayne) and SPT (Hatanaka-Uchida from (N1)60) reading of an investigation.
pub fn phi_profile(doc: &En1997Snapshot, investigation: &Investigation) -> Vec<(f64, f64)> {
    investigation
        .readings
        .iter()
        .filter_map(|r| {
            let sigma = effective_stress_pa(&doc.layers, doc.groundwater_level, r.depth);
            match investigation.kind {
                InvestigationKind::Cpt => (r.measurement > 0.0 && sigma > 0.0).then(|| (r.depth, part_2::phi_from_cpt_deg(r.measurement, sigma))),
                InvestigationKind::Spt => (r.measurement > 0.0 && sigma > 0.0).then(|| (r.depth, part_2::phi_from_spt_deg(r.measurement, sigma))),
                InvestigationKind::Borehole => None,
            }
        })
        .collect()
}

/// 🎯️ Characteristic value of the independent samples: the statistical value X_m(1 − k_n·V_x) with the Student t factor of `n − 1` degrees of freedom from `n ≥ MIN_SAMPLES` samples, below that the lowest of the `readings` inside the layer (cautious estimate); `None` without samples.
pub fn characteristic(samples: Vec<Sample>, lowest: f64, readings: usize, rejected: usize) -> Option<Derived> {
    if samples.is_empty() {
        return None;
    }
    let values: Vec<f64> = samples.iter().map(|s| s.value).collect();
    if values.len() >= MIN_SAMPLES {
        let (mean, _, variation, kn, xk) = catalogue::investigation::characteristic_value(&values, false, None);
        return Some(Derived { samples, readings, rejected, method: Method::Statistical, mean, variation, kn, characteristic: xk.min(mean) });
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Some(Derived { samples, readings, rejected, method: Method::Cautious, mean, variation: 0.0, kn: 0.0, characteristic: lowest })
}

/// 📐️ The independent samples of one investigation point from its `(depth, value)` readings inside a layer: `per_sounding` depth segments with the same reading count (at most one per reading), each averaged to its mean depth and mean value; one segment is the spatial average over the layer thickness.
pub fn independent_samples(investigation: &str, mut readings: Vec<(f64, f64)>, per_sounding: usize) -> Vec<Sample> {
    readings.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (n, k) = (readings.len(), per_sounding.clamp(1, readings.len().max(1)));
    (0..k)
        .filter_map(|i| {
            let segment = &readings[i * n / k..(i + 1) * n / k];
            let count = segment.len() as f64;
            (!segment.is_empty()).then(|| Sample { investigation: investigation.to_owned(), depth: segment.iter().map(|r| r.0).sum::<f64>() / count, value: segment.iter().map(|r| r.1).sum::<f64>() / count })
        })
        .collect()
}

/// 🎯️ Characteristic value of a layer parameter from the investigations: c_u from the CPT readings inside the layer, φ′ from its CPT and SPT readings; the statistics run over the independent samples per investigation point (`SAMPLES_PER_SOUNDING_ID`), not over the readings.
pub fn derive_layer(doc: &En1997Snapshot, layer: &SoilLayer, parameter: CharacteristicParameter) -> Option<Derived> {
    let per_sounding = semio_s_plugin_norm_compliance::ndp_number(SAMPLES_PER_SOUNDING_ID, doc.annex).max(1.0) as usize;
    let (mut samples, mut rejected, mut readings_inside, mut lowest) = (Vec::new(), 0, 0, f64::INFINITY);
    let used = |kind: InvestigationKind| match parameter {
        CharacteristicParameter::UndrainedStrength => kind == InvestigationKind::Cpt,
        CharacteristicParameter::FrictionAngle => kind != InvestigationKind::Borehole,
    };
    for investigation in &doc.investigations {
        let profile = match parameter {
            CharacteristicParameter::UndrainedStrength => cu_profile(doc, investigation),
            CharacteristicParameter::FrictionAngle => phi_profile(doc, investigation),
        };
        let readings = investigation.readings.iter().filter(|r| r.depth >= layer.depth_top && r.depth < layer.depth_bottom && used(investigation.kind)).count();
        let inside: Vec<(f64, f64)> = profile.into_iter().filter(|(z, _)| *z >= layer.depth_top && *z < layer.depth_bottom).collect();
        rejected += readings.saturating_sub(inside.len());
        readings_inside += inside.len();
        lowest = inside.iter().fold(lowest, |least, (_, value)| least.min(*value));
        samples.extend(independent_samples(&investigation.id, inside, per_sounding));
    }
    characteristic(samples, lowest, readings_inside, rejected)
}

/// 🧭️ The resolution of every typed basis row that asks for the derived value; rows naming an unknown layer are left to the integrity check.
pub fn resolutions(doc: &En1997Snapshot) -> Vec<Resolution> {
    doc.characteristic_bases
        .iter()
        .filter(|b| b.basis == ParameterBasis::Derived)
        .filter_map(|b| {
            let layer = doc.layers.iter().find(|l| l.id == b.layer_id)?;
            let declared = match b.parameter {
                CharacteristicParameter::UndrainedStrength => layer.cohesion_undrained,
                CharacteristicParameter::FrictionAngle => layer.phi_prime_deg,
            };
            Some(Resolution { layer_id: layer.id.clone(), parameter: b.parameter, declared, derived: derive_layer(doc, layer, b.parameter) })
        })
        .collect()
}

/// 🧱️ The document the checks run on: every layer parameter whose basis is `derived` carries the characteristic value from the investigations, or 0 (unknown, flagged by its own check) when nothing can be derived; declared parameters are never touched.
pub fn resolved_document(doc: &En1997Snapshot) -> Cow<'_, En1997Snapshot> {
    let resolved = resolutions(doc);
    if resolved.is_empty() {
        return Cow::Borrowed(doc);
    }
    let mut out = doc.clone();
    for r in &resolved {
        if let Some(layer) = out.layers.iter_mut().find(|l| l.id == r.layer_id) {
            match r.parameter {
                CharacteristicParameter::UndrainedStrength => layer.cohesion_undrained = r.value(),
                CharacteristicParameter::FrictionAngle => layer.phi_prime_deg = r.value(),
            }
        }
    }
    Cow::Owned(out)
}

/// 🔎️ Depth [m] below the ground reached by the deepest investigation point; the declared `investigationDepth` of the document only counts when no investigation point is recorded.
pub fn reached_depth(doc: &En1997Snapshot) -> f64 {
    if doc.investigations.is_empty() { doc.investigation_depth } else { doc.investigations.iter().map(|i| i.depth).fold(0.0, f64::max) }
}

fn parameter_path(layer: &SoilLayer, parameter: CharacteristicParameter) -> String {
    format!("layers[id={}].{}", layer.id, if parameter == CharacteristicParameter::UndrainedStrength { "cohesionUndrained" } else { "phiPrimeDeg" })
}

/// 🧮️ The check of one derived characteristic value: the number of independent samples (investigation points) against the statistical minimum, with the method, mean, V_x, k_n and the value used by the checks; nothing to derive fails with a remedy to add readings (no silent fallback to the declared value).
pub fn characteristic_check(layer: &SoilLayer, resolution: &Resolution, annex: AnnexChoice) -> CheckResult {
    let (name_en, name_de, unit, scale) = match resolution.parameter {
        CharacteristicParameter::UndrainedStrength => ("c_u", "c_u", "kPa", 1000.0),
        CharacteristicParameter::FrictionAngle => ("φ′", "φ′", "°", 1.0),
    };
    let key = if resolution.parameter == CharacteristicParameter::UndrainedStrength { "cu" } else { "phi" };
    let subject = subject_entity(&layer.id, &parameter_path(layer, resolution.parameter), &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id));
    let n = resolution.derived.as_ref().map_or(0, |d| d.samples.len());
    let (en, de) = match &resolution.derived {
        None => (format!("No CPT/SPT reading inside {:.1}…{:.1} m: no characteristic {name_en} can be derived and the checks use 0.", layer.depth_top, layer.depth_bottom), format!("Keine CPT/SPT-Messung in {:.1}…{:.1} m: kein charakteristischer Wert {name_de} ableitbar, die Nachweise rechnen mit 0.", layer.depth_top, layer.depth_bottom)),
        Some(d) => {
            let how_en = if d.method == Method::Statistical { format!("statistical X_m(1 − k_n V_x): X_m={:.2}, V_x={:.3}, k_n={:.3}", d.mean / scale, d.variation, d.kn) } else { format!("cautious estimate (lowest of {} reading(s), fewer than {MIN_SAMPLES} independent samples for statistics)", d.readings) };
            let how_de = if d.method == Method::Statistical { format!("statistisch X_m(1 − k_n V_x): X_m={:.2}, V_x={:.3}, k_n={:.3}", d.mean / scale, d.variation, d.kn) } else { format!("vorsichtiger Schätzwert (kleinster von {} Wert(en), weniger als {MIN_SAMPLES} unabhängige Stichproben für Statistik)", d.readings) };
            (
                format!("{name_en},k = {:.2} {unit} from {n} independent sample(s) (one per investigation point: the mean of its {} reading(s) inside the layer), {how_en}; declared {:.2} {unit}; {} reading(s) rejected; the sample rule is an unverified registry row.", d.characteristic / scale, d.readings, resolution.declared / scale, d.rejected),
                format!("{name_de},k = {:.2} {unit} aus {n} unabhängigen Stichprobe(n) (eine je Aufschlusspunkt: das Mittel seiner {} Messwert(e) in der Schicht), {how_de}; angegeben {:.2} {unit}; {} Messwert(e) verworfen; die Stichprobenregel ist eine ungeprüfte Registerzeile.", d.characteristic / scale, d.readings, resolution.declared / scale, d.rejected),
            )
        }
    };
    let mut check = CheckResult::assess(
        format!("en1997.2.characteristic.{key}.{}", layer.id),
        "DIN EN 1997-2",
        ClauseId::new("EN 1997-2", "2", "2.4.5.2"),
        subject.clone(),
        loc(&format!("Characteristic {name_en} from investigations"), &format!("Charakteristischer Wert {name_de} aus Erkundungen")),
    )
    .minimum(Quantity::new(QuantityKind::Dimensionless, n as f64), Quantity::new(QuantityKind::Dimensionless, MIN_SAMPLES as f64))
    .annex(annex)
    .ndp(SAMPLES_PER_SOUNDING_ID)
    .explanation(loc(&en, &de));
    if n < MIN_SAMPLES {
        check = check.remedy(Remedy::at_least(
            subject,
            Quantity::new(QuantityKind::Dimensionless, n as f64),
            Quantity::new(QuantityKind::Dimensionless, MIN_SAMPLES as f64),
            loc(&format!("Add CPT/SPT investigation points that reach layer {} or declare the parameter on the layer.", layer.id), &format!("CPT/SPT-Aufschlusspunkte in Schicht {} ergänzen oder den Parameter auf der Schicht angeben.", layer.id)),
        ));
    }
    check.build()
}

/// 🧮️ The derivation checks of one layer for every typed basis row that asks for the derived value.
pub fn layer_checks(declared: &En1997Snapshot, layer: &SoilLayer) -> Vec<CheckResult> {
    resolutions(declared).iter().filter(|r| r.layer_id == layer.id).map(|r| characteristic_check(layer, r, declared.annex)).collect()
}

/// 🧱️ Integrity of the typed bases: a row names an existing layer and each (layer, parameter) pair has one row, so no basis is ambiguous.
pub fn push_integrity(report: &mut CheckReport, doc: &En1997Snapshot) {
    for (index, basis) in doc.characteristic_bases.iter().enumerate() {
        let unknown = !doc.layers.iter().any(|l| l.id == basis.layer_id);
        let repeated = doc.characteristic_bases.iter().take(index).any(|b| b.layer_id == basis.layer_id && b.parameter == basis.parameter);
        if !unknown && !repeated {
            continue;
        }
        let subject = subject_entity(&basis.id, &format!("characteristicBases[id={}].layerId", basis.id), "Characteristic value basis", "Grundlage des charakteristischen Werts");
        let (en, de) = if unknown { (format!("Layer '{}' does not exist.", basis.layer_id), format!("Schicht '{}' existiert nicht.", basis.layer_id)) } else { (format!("Layer '{}' has a second basis for the same parameter.", basis.layer_id), format!("Schicht '{}' hat eine zweite Grundlage für denselben Parameter.", basis.layer_id)) };
        report.push(
            CheckResult::assess(format!("en1997.integrity.basis.{}", basis.id), "EN 1997 integrity", ClauseId::new("EN 1997-1", "1", "1.5"), subject.clone(), loc("Unambiguous characteristic value basis", "Eindeutige Grundlage des charakteristischen Werts"))
                .annex(doc.annex)
                .explanation(loc(&en, &de))
                .status(CheckStatus::Fail)
                .remedy(Remedy::one_of(subject, doc.layers.iter().map(|l| l.id.clone()).collect(), loc("Point the basis at an existing layer and keep one row per parameter.", "Die Grundlage auf eine vorhandene Schicht richten und je Parameter eine Zeile behalten.")))
                .build(),
        );
    }
}
