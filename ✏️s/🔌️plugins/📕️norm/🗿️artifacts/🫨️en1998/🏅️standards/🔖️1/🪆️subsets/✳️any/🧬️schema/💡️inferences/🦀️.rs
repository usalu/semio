//! 💡️ En1998 inference schema + full seismic evaluate with remedies.

use crate::En1998Snapshot;
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::standards::v1::subsets::any::schema::{na_de, part_1, part_2, part_3, part_4, part_5, part_6, AnnexParams};
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1998.inference")]
pub struct En1998Inference {
    #[derived]
    pub outline: En1998Outline,
}

impl Default for En1998Inference {
    fn default() -> Self {
        use protocol::Inference;
        Self::infer(&En1998Snapshot::default())
    }
}

impl protocol::Inference<En1998Snapshot> for En1998Inference {
    fn infer(snapshot: &En1998Snapshot) -> Self {
        Self { outline: En1998Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1998Snapshot> for En1998Inference {
    fn inference_schema_id() -> &'static str { "s.norm.en1998.inference" }
    fn schema_version() -> u32 { 1 }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.en1998.inference.outline",
            reads: &["annex", "site", "buildings", "bridges", "assessments", "silos", "tanks", "foundations", "retainingWalls", "towers"],
        }]
    }
}
//#endregion 🔖️Inference

impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1998Builder {
    type Snapshot = En1998Snapshot;
    type Inference = En1998Inference;
}

pub fn en1998_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1998.inference",
        inference: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}

fn q_force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }
fn q_len(m: f64) -> Quantity { Quantity::length_m(m) }
fn q_acc(v: f64) -> Quantity { Quantity::acceleration_m_s2(v) }
fn q_dim(v: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, v) }
fn q_press(pa: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, pa) }
fn q_mom(nm: f64) -> Quantity { Quantity::new(QuantityKind::Moment, nm) }

fn lc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }


fn id_sel(collection: &str, id: &str) -> String {
    format!("{collection}[id={id}]")
}

fn resolve_annex(document: &En1998Snapshot) -> Result<AnnexParams, CheckResult> {
    let annex = match document.annex.to_ascii_lowercase().as_str() {
        "en" => AnnexChoice::En,
        _ => AnnexChoice::De,
    };
    match annex {
        AnnexChoice::De => {
            let zone = na_de::SeismicZone::from(document.site.seismic_zone);
            let combo = na_de::GroundCombo::from(document.site.de_ground_combo);
            Ok(AnnexParams::De { zone, combo })
        }
        AnnexChoice::En => {
            let g = document.site.en_ground_type.chars().next().unwrap_or('B').to_ascii_uppercase();
            let type1 = !document.site.en_spectrum_type.to_ascii_lowercase().contains("2");
            Ok(AnnexParams::En { a_gr: document.site.a_gr, ground: g, spectrum_type1: type1 })
        }
    }
}

/// 📋️ Full EN 1998 evaluate across claimed parts (scoped; empty lists → NotApplicable).
pub fn check_full_seismic(document: &En1998Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    let annex_params = match resolve_annex(document) {
        Ok(p) => p,
        Err(fail) => {
            report.push(fail);
            return report;
        }
    };
    let annex = annex_params.choice();
    let (a_g, s, tb, tc, td) = annex_params.ground_params();
    let gamma_i = part_1::gamma_i(&document.site.importance_class);

    if matches!(annex_params, AnnexParams::En { .. }) {
        let annex = AnnexChoice::En;
        let zone = na_de::SeismicZone::from(document.site.seismic_zone);
        let combo = na_de::GroundCombo::from(document.site.de_ground_combo);
        let zone_agr = zone.a_gr();
        let zone_ok = (zone_agr - document.site.a_gr).abs() <= 1e-6 || document.site.a_gr <= 0.0;
        let en_g = document.site.en_ground_type.chars().next().unwrap_or('?').to_ascii_uppercase();
        let expected_combo = match en_g {
            'A' => na_de::GroundCombo::AR,
            'B' => na_de::GroundCombo::BR,
            'C' => na_de::GroundCombo::CR,
            'D' | 'E' => na_de::GroundCombo::CS,
            _ => na_de::GroundCombo::BR,
        };
        let combo_ok = combo == expected_combo || document.site.en_ground_type.is_empty();
        let mut zc = CheckResult::assess(
            "en1998.site.deNaConsistencyUnderEn",
            "EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "3.2.1"),
            SubjectRef::new("", "site.seismicZone", lc("Seismic zone", "Erdbebenzone")),
            lc("DE NA zone/combo consistent with EN site", "DE-NA Zone/Kombination konsistent mit EN-Standort"),
        )
        .utilization(q_dim(if zone_ok && combo_ok { 0.5 } else { 1.5 }), q_dim(1.0))
        .annex(annex)
        .explanation(lc(
            &format!("zone a_gR={zone_agr:.3} vs site a_gR={:.3}; combo={combo:?} expected={expected_combo:?} for enGround={en_g}", document.site.a_gr),
            &format!("Zone a_gR={zone_agr:.3} gegen Standort a_gR={:.3}; Kombi={combo:?} erwartet={expected_combo:?} für enGround={en_g}", document.site.a_gr),
        ));
        if !zone_ok {
            zc = zc.remedy(Remedy::exactly(
                SubjectRef::new("", "site.seismicZone", lc("Seismic zone", "Erdbebenzone")),
                q_dim(0.0),
                q_dim(1.0),
                lc("Align seismicZone with site a_gR under EN annex.", "seismicZone unter EN-Anhang an Standort-a_gR anpassen."),
            ));
        }
        if !combo_ok {
            zc = zc.remedy(Remedy::exactly(
                SubjectRef::new("", "site.deGroundCombo", lc("DE ground combo", "DE-Baugrundkombination")),
                q_dim(0.0),
                q_dim(1.0),
                lc("Align deGroundCombo letter with enGroundType under EN annex.", "deGroundCombo-Buchstaben unter EN-Anhang an enGroundType anpassen."),
            ));
        }
        report.push(zc.build());
    }

    if matches!(annex, AnnexChoice::De) {
        let derived = document.site.seismic_zone.a_gr();
        if (document.site.a_gr - derived).abs() > 1e-9 {
            report.push(
                CheckResult::assess(
                    "en1998.site.aGr.derived",
                    "DIN EN 1998-1/NA",
                    ClauseId::new("EN 1998-1", "1", "3.2.1"),
                    SubjectRef::new("", "site.aGr", lc("Reference PGA a_gR", "Bezugsspitzenwert a_gR")),
                    lc("DE a_gR derived from seismic zone", "DE a_gR aus Erdbebenzone"),
                )
                .status(crate::document::CheckStatus::Warning)
                .annex(annex)
                .explanation(lc(
                    &format!("DE annex derives a_gR={derived} m/s² from zone; site.aGr={:.3} is ignored for spectrum.", document.site.a_gr),
                    &format!("DE-NA leitet a_gR={derived} m/s² aus der Zone ab; site.aGr={:.3} wird für das Spektrum ignoriert.", document.site.a_gr),
                ))
                .remedy(Remedy::exactly(
                    SubjectRef::new("", "site.aGr", lc("Reference PGA a_gR", "Bezugsspitzenwert a_gR")),
                    q_acc(document.site.a_gr),
                    q_acc(derived),
                    lc("Set a_gR to the zone Table NA.1 value (or switch annex to EN).", "a_gR auf Tab. NA.1-Zonenwert setzen (oder Anhang EN wählen)."),
                ))
                .build(),
            );
        }
    }

    push_referential_integrity(&mut report, document, annex);

    let low_seismicity = matches!(annex_params, AnnexParams::De { zone: na_de::SeismicZone::Zone0, .. }) || a_g <= 0.0;

    if document.buildings.is_empty() {
        report.push(
            CheckResult::assess("en1998.1.buildings.na", "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "4"), SubjectRef::whole(lc("Buildings", "Hochbauten")), lc("Building seismic checks", "Hochbau-Erdbebennachweise"))
                .not_applicable(lc("No buildings in the subject.", "Keine Hochbauten im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }


    for building in &document.buildings {
        let bid = &building.id;
        let bpath = id_sel("buildings", bid);
        let blabel = lc(&building.name, &building.name);

        if low_seismicity {
            report.push(
                CheckResult::assess(format!("en1998.1.{bid}.zone0"), "DIN EN 1998-1/NA", ClauseId::new("EN 1998-1", "1", "3.2.1"), SubjectRef::new(bid, format!("{bpath}.id"), blabel.clone()), lc("Very low seismicity exemption", "Befreiung bei sehr geringer Seismizität"))
                    .not_applicable(lc("DE zone 0 / a_gR = 0: EN 1998 provisions need not be applied.", "DE-Zone 0 / a_gR = 0: EN-1998-Regeln brauchen nicht angewendet zu werden."))
                    .annex(annex)
                    .build(),
            );
            continue;
        }

        let total_height: f64 = building.storeys.iter().map(|s| s.height_m).sum();
        let n_storeys = building.storeys.len();
        let masses: Vec<f64> = building.storeys.iter().enumerate().map(|(i, s)| {
            part_1::storey_seismic_mass_kg(s, i + 1 == n_storeys)
        }).collect();
        let heights: Vec<f64> = building.storeys.iter().map(|s| s.height_m).collect();
        let total_mass: f64 = masses.iter().sum();

        let t1 = match building.t1_method.to_ascii_lowercase().as_str() {
            "given" => building.t1_given_s,
            "rayleigh" => {
                let mut z = 0.0;
                let mut disp = Vec::new();
                for h in &heights {
                    z += h;
                    disp.push(z / total_height.max(1e-9));
                }
                part_1::t1_rayleigh(&masses, &disp)
            }
            _ => part_1::t1_from_ct(building.ct, total_height),
        };


        {
            let using_given = building.t1_method.to_ascii_lowercase() == "given";
            let mut tg = CheckResult::assess(
                format!("en1998.1.{bid}.t1Given"),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.3.3.2.2"),
                SubjectRef::new(bid, format!("{bpath}.t1GivenS"), blabel.clone()),
                lc("Given fundamental period T₁", "Vorgegebene Fundamentalschwingzeit T₁"),
            )
            .utilization(q_dim(building.t1_given_s), q_dim(if using_given { t1.max(0.05) } else { 0.05 }))
            .annex(annex)
            .explanation(lc(
                &format!("t1Method={}, t1Given={:.3} s, T1={t1:.3} s", building.t1_method, building.t1_given_s),
                &format!("t1Verfahren={}, t1Given={:.3} s, T1={t1:.3} s", building.t1_method, building.t1_given_s),
            ));
            if !using_given && building.t1_given_s > 0.05 {
                tg = tg.status(crate::document::CheckStatus::Fail).remedy(Remedy::at_most(
                    SubjectRef::new(bid, format!("{bpath}.t1GivenS"), lc("Given T1", "Vorgegebenes T1")),
                    q_len(building.t1_given_s),
                    q_len(0.0),
                    lc("Clear t1GivenS when t1Method is not given.", "t1GivenS leeren, wenn t1Method nicht given ist."),
                ));
            }
            report.push(tg.build());
        }

        // §4.2.3 claim flags (LFM applicability) — remedies flip Pass for claimed regularity.
        report.push({
            let ok = building.plan_regular && building.elevation_regular;
            let mut b = CheckResult::assess(
                format!("en1998.1.{bid}.regularity.claim"),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.2.3"),
                SubjectRef::new(bid, format!("{bpath}.planRegular"), blabel.clone()),
                lc("Claimed regularity in plan and elevation", "Beanspruchte Regelmässigkeit im Grund- und Aufriss"),
            )
            .utilization(q_dim(if ok { 0.5 } else { 1.5 }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("planRegular={}, elevationRegular={}", building.plan_regular, building.elevation_regular),
                &format!("Grundrissregelmässigkeit={}, Aufrissregelmässigkeit={}", building.plan_regular, building.elevation_regular),
            ));
            if !building.plan_regular {
                b = b.remedy(Remedy::exactly(
                    SubjectRef::new(bid, format!("{bpath}.planRegular"), lc("Plan regularity", "Regelmässigkeit im Grundriss")),
                    q_dim(0.0),
                    q_dim(1.0),
                    lc("Restore plan regularity (or use modal analysis).", "Regelmässigkeit im Grundriss herstellen (oder Modale Analyse verwenden)."),
                ));
            }
            if !building.elevation_regular {
                b = b.remedy(Remedy::exactly(
                    SubjectRef::new(bid, format!("{bpath}.elevationRegular"), lc("Elevation regularity", "Regelmässigkeit im Aufriss")),
                    q_dim(0.0),
                    q_dim(1.0),
                    lc("Restore elevation regularity (or use modal analysis).", "Regelmässigkeit im Aufriss herstellen (oder Modale Analyse verwenden)."),
                ));
            }
            b.build()
        });

        // §4.2.3.2 plan regularity from CM/CS eccentricity and torsional radius.
        if building.plan_width_m > 0.0 && building.plan_length_m > 0.0 {
            for storey in &building.storeys {
                let (e0x, e0y, rx, ry, ls) = part_1::plan_regularity_metrics(
                    storey.centre_of_mass_x_m,
                    storey.centre_of_mass_y_m,
                    storey.centre_of_stiffness_x_m,
                    storey.centre_of_stiffness_y_m,
                    storey.stiffness_x,
                    storey.stiffness_y,
                    building.plan_width_m,
                    building.plan_length_m,
                );
                let e0 = e0x.max(e0y);
                let r_min = rx.min(ry);
                let limit_e = 0.30 * r_min;
                let stpath = format!("{}.storeys[id={}]", bpath, storey.id);
                let mut ch = CheckResult::assess(
                    format!("en1998.1.{bid}.storey.{}.planRegularity", storey.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "4.2.3.2"),
                    SubjectRef::new(bid, format!("{stpath}.centreOfMassXM"), lc(&storey.id, &storey.id)),
                    lc("Plan regularity e₀ ≤ 0.30 r and r ≥ ℓₛ", "Grundrissregelmässigkeit e₀ ≤ 0.30 r und r ≥ ℓₛ"),
                )
                .utilization(q_len(e0), q_len(limit_e.max(1e-6)))
                .annex(annex)
                .explanation(lc(
                    &format!("e0x={e0x:.3} m, e0y={e0y:.3} m, rx={rx:.3} m, ry={ry:.3} m, ls={ls:.3} m, kx={:.0}, ky={:.0}", storey.stiffness_x, storey.stiffness_y),
                    &format!("e0x={e0x:.3} m, e0y={e0y:.3} m, rx={rx:.3} m, ry={ry:.3} m, ℓₛ={ls:.3} m, kx={:.0}, ky={:.0}", storey.stiffness_x, storey.stiffness_y),
                ));
                if e0 > limit_e || ls > r_min {
                    ch = ch.remedy(Remedy::at_most(
                        SubjectRef::new(bid, format!("{stpath}.centreOfMassXM"), lc("Centre of mass x", "Massenschwerpunkt x")),
                        q_len(storey.centre_of_mass_x_m),
                        q_len(storey.centre_of_stiffness_x_m),
                        lc("Reduce structural eccentricity e₀ (align CM and CS) or increase torsional stiffness.", "Strukturelle Exzentrizität e₀ verringern (SM und SS ausrichten) oder Torsionssteifigkeit erhöhen."),
                    ));
                }
                report.push(ch.build());
            }

            // §4.2.3.3 elevation regularity — stiffness/mass variation between adjacent storeys.
            let n = building.storeys.len();
            if n >= 2 {
                for i in 0..n - 1 {
                    let lo = &building.storeys[i];
                    let hi = &building.storeys[i + 1];
                    let k_lo = lo.stiffness_x + lo.stiffness_y;
                    let k_hi = hi.stiffness_x + hi.stiffness_y;
                    let m_lo = part_1::storey_seismic_mass_kg(lo, false);
                    let m_hi = part_1::storey_seismic_mass_kg(hi, i + 2 == n);
                    let k_ratio = if k_lo > 0.0 { k_hi / k_lo } else { 0.0 };
                    let m_ratio = if m_lo > 0.0 { m_hi / m_lo } else { 0.0 };
                    // Soft-storey / abrupt stiffness drop: k_hi/k_lo ≥ 0.70; mass jump m_hi/m_lo ≤ 1.50.
                    let util = (0.70 / k_ratio.max(1e-9)).max(m_ratio / 1.50);
                    let mut ch = CheckResult::assess(
                        format!("en1998.1.{bid}.elevationRegularity.{}-{}", lo.id, hi.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "4.2.3.3"),
                        SubjectRef::new(bid, format!("{}.storeys[id={}].stiffnessX", bpath, hi.id), lc(&hi.id, &hi.id)),
                        lc("Elevation regularity (stiffness/mass variation)", "Aufrissregelmässigkeit (Steifigkeits-/Masseänderung)"),
                    )
                    .utilization(q_dim(util), q_dim(1.0))
                    .annex(annex)
                    .explanation(lc(
                        &format!("k_ratio={k_ratio:.3} (≥0.70), m_ratio={m_ratio:.3} (≤1.50), correlated_lo={}, correlated_hi={}", lo.correlated_occupancy, hi.correlated_occupancy),
                        &format!("k-Verhältnis={k_ratio:.3} (≥0.70), m-Verhältnis={m_ratio:.3} (≤1.50), korreliert_unten={}, korreliert_oben={}", lo.correlated_occupancy, hi.correlated_occupancy),
                    ));
                    if util > 1.0 {
                        ch = ch.remedy(Remedy::at_least(
                            SubjectRef::new(bid, format!("{}.storeys[id={}].stiffnessX", bpath, hi.id), lc("Storey stiffness X", "Geschosssteifigkeit X")),
                            q_dim(hi.stiffness_x),
                            q_dim(0.70 * lo.stiffness_x),
                            lc("Increase upper-storey stiffness (avoid soft storey) or smooth mass variation.", "Obere Geschosssteifigkeit erhöhen (weiches Geschoss vermeiden) oder Masseübergang glätten."),
                        ));
                    }
                    report.push(ch.build());
                }
            }
        }

        // Plan dimensions required for accidental eccentricity (§4.3.2) — no hardcoded fallback.
        {
            let ok = building.plan_width_m > 0.0 && building.plan_length_m > 0.0;
            let mut c = CheckResult::assess(
                format!("en1998.1.{bid}.planDims"),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.3.2"),
                SubjectRef::new(bid, format!("{bpath}.planWidthM"), blabel.clone()),
                lc("Plan dimensions Lx, Ly", "Grundrissabmessungen Lx, Ly"),
            )
            .utilization(q_len(building.plan_width_m), q_len(building.plan_length_m.max(0.01)))
            .annex(annex)
            .status(if ok { crate::document::CheckStatus::Pass } else { crate::document::CheckStatus::Fail })
            .explanation(lc(
                &format!("plan width Lx={:.2} m, plan length Ly={:.2} m", building.plan_width_m, building.plan_length_m),
                &format!("Grundrissbreite Lx={:.2} m, Grundrisslänge Ly={:.2} m", building.plan_width_m, building.plan_length_m),
            ));
            if building.plan_width_m <= 0.0 {
                c = c.remedy(Remedy::at_least(
                    SubjectRef::new(bid, format!("{bpath}.planWidthM"), lc("Plan width Lx", "Grundrissbreite Lx")),
                    q_len(building.plan_width_m),
                    q_len(1.0),
                    lc("Provide a positive plan width Lx.", "Positive Grundrissbreite Lx angeben."),
                ));
            }
            if building.plan_length_m <= 0.0 {
                c = c.remedy(Remedy::at_least(
                    SubjectRef::new(bid, format!("{bpath}.planLengthM"), lc("Plan length Ly", "Grundrisslänge Ly")),
                    q_len(building.plan_length_m),
                    q_len(1.0),
                    lc("Provide a positive plan length Ly.", "Positive Grundrisslänge Ly angeben."),
                ));
            }
            report.push(c.build());
        }

        // Multiple resisting systems — §5.2.2.1 dual/wall-equivalent + §4.2.3.1 torsional flexibility.
        {
            let types: Vec<String> = building.systems.iter().map(|s| s.system_type.to_ascii_lowercase()).collect();
            let has_frame = types.iter().any(|t| t == "frame" || t == "dual");
            let has_wall = types.iter().any(|t| t == "wall" || t == "dual" || t.contains("wall"));
            let dual_ok = has_frame && has_wall;
            let tor_ok = building.plan_regular;
            let claimed = building.multiple_resisting_systems;
            let ok = if claimed { dual_ok && tor_ok } else { true };
            let mut c = CheckResult::assess(
                format!("en1998.1.{bid}.multipleSystems"),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "5.2.2.1"),
                SubjectRef::new(bid, format!("{bpath}.multipleResistingSystems"), blabel.clone()),
                lc("Multiple resisting systems / dual classification", "Mehrere Aussteifungssysteme / Dualklassifikation"),
            )
            .utilization(q_dim(if ok { 0.5 } else { 1.5 }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("multipleResistingSystems={claimed}, frame={has_frame}, wall={has_wall}, planRegular={}", building.plan_regular),
                &format!("mehrere Aussteifungssysteme={claimed}, Rahmen={has_frame}, Wand={has_wall}, Grundrissregelmässigkeit={}", building.plan_regular),
            ));
            if claimed && !dual_ok {
                c = c.remedy(Remedy::exactly(
                    SubjectRef::new(bid, format!("{bpath}.multipleResistingSystems"), lc("Multiple resisting systems", "Mehrere Aussteifungssysteme")),
                    q_dim(1.0),
                    q_dim(0.0),
                    lc("Clear multipleResistingSystems or provide frame+wall (dual) systems per §5.2.2.1.", "multipleResistingSystems deaktivieren oder Rahmen+Wand (Dual) nach §5.2.2.1 vorsehen."),
                ));
            }
            if claimed && !tor_ok {
                c = c.remedy(Remedy::exactly(
                    SubjectRef::new(bid, format!("{bpath}.planRegular"), lc("Plan regularity", "Regelmässigkeit im Grundriss")),
                    q_dim(0.0),
                    q_dim(1.0),
                    lc("Restore plan regularity for multi-system torsional flexibility (§4.2.3.1).", "Grundrissregelmässigkeit für Torsion bei Mehrfachsystemen herstellen (§4.2.3.1)."),
                ));
            }
            report.push(c.build());
        }

        let lfm_ok = part_1::lateral_force_method_applicable(t1, tc, building.elevation_regular);
        report.push(
            CheckResult::assess(
                format!("en1998.1.{bid}.lfm"),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.3.3.2"),
                SubjectRef::new(bid, format!("{bpath}.elevationRegular"), blabel.clone()),
                lc("Lateral force method applicability", "Anwendbarkeit des Kraftgrössenverfahrens"),
            )
            .utilization(q_dim(if lfm_ok { 0.5 } else { 0.95 }), q_dim(1.0))
            .status(if lfm_ok { crate::document::CheckStatus::Pass } else { crate::document::CheckStatus::Warning })
            .annex(annex)
            .explanation(lc(
                &format!("T1={t1:.3}s (method={}, t1Given={:.3}s), TC={tc:.3}s, elevationRegular={}", building.t1_method, building.t1_given_s, building.elevation_regular),
                &format!("T1={t1:.3}s (Verfahren={}, t1Given={:.3}s), TC={tc:.3}s, Aufrissregelmässigkeit={}", building.t1_method, building.t1_given_s, building.elevation_regular),
            ))
            .build(),
        );

        let systems = if building.systems.is_empty() {
            report.push(
                CheckResult::assess(format!("en1998.1.{bid}.system.na"), "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "5"), SubjectRef::new(bid, format!("{bpath}.systems"), blabel.clone()), lc("Structural system", "Aussteifungssystem"))
                    .not_applicable(lc("No structural systems defined.", "Keine Aussteifungssysteme definiert."))
                    .annex(annex)
                    .build(),
            );
            continue;
        } else {
            &building.systems
        };

        for system in systems {
            {
                let dup = building
                    .systems
                    .iter()
                    .filter(|s| s.direction.to_ascii_lowercase() == system.direction.to_ascii_lowercase())
                    .count()
                    > 1;
                let known = matches!(system.direction.to_ascii_lowercase().as_str(), "x" | "y");
                let mut dch = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.direction", system.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "4.3.3.2.1"),
                    SubjectRef::new(bid, format!("{}.systems[id={}].direction", bpath, system.id), lc(&system.id, &system.id)),
                    lc("Horizontal analysis direction", "Horizontale Analyserichtung"),
                )
                .utilization(q_dim(if !known || dup { 1.5 } else { 0.5 }), q_dim(1.0))
                .annex(annex)
                .explanation(lc(
                    &format!("direction={}, known={}, duplicate={}", system.direction, known, dup),
                    &format!("Richtung={}, gültig={}, doppelt={}", system.direction, known, dup),
                ));
                if !known || dup {
                    dch = dch.remedy(Remedy::exactly(
                        SubjectRef::new(bid, format!("{}.systems[id={}].direction", bpath, system.id), lc("Direction", "Richtung")),
                        q_dim(0.0),
                        q_dim(1.0),
                        lc(
                            "Assign unique orthogonal directions x and y to resisting systems.",
                            "Den Aussteifungssystemen eindeutige orthogonale Richtungen x und y zuweisen.",
                        ),
                    ));
                }
                report.push(dch.build());
            }

            let q = part_1::behaviour_factor(system.q0, system.alpha_u_over_alpha_1, system.k_w);
            let q_max = part_1::q_max_for_system(&system.system_type, &system.material, &system.ductility_class);
            let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t1);
            let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q.min(q_max), a_g);
            let lambda = part_1::lambda_factor(t1, tc, building.storeys.len());
            let f_b = part_1::base_shear_n(s_d, total_mass, lambda);
            let spath = format!("{}.systems[id={}]", bpath, system.id);

            // q limit vs tabulated max
            {
                let ok = q <= q_max + 1e-9;
                let mut c = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.qLimit", system.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5.2.2.2"),
                    SubjectRef::new(bid, format!("{spath}.q0"), lc(
                &format!("System {}", system.id),
                &format!("Nachweis: System {}", system.id),
            )),
                    lc("Behaviour factor q within tabulated limit", "Verhaltensbeiwert q innerhalb Tabellengrenze"),
                )
                .utilization(q_dim(if ok { q.min(q_max) } else { q }), q_dim(q_max))
                .annex(annex)
                .explanation(lc(
                    &format!("q={q:.2} from q0={:.2}·αu/α1·kw; q_max={q_max:.2} for {}/{}/{}", system.q0, system.system_type, system.material, system.ductility_class),
                    &format!("q={q:.2} aus q0={:.2}·αu/α1·kw; q_max={q_max:.2} für {}/{}/{}", system.q0, system.system_type, system.material, system.ductility_class),
                ));
                if !ok {
                    let req_q0 = (q_max / (system.alpha_u_over_alpha_1 * system.k_w).max(1e-9)).max(1.0);
                    c = c.remedy(Remedy::at_most(
                        SubjectRef::new(bid, format!("{spath}.q0"), lc("q₀", "q₀")),
                        q_dim(system.q0),
                        q_dim(req_q0),
                        lc(&format!("Reduce q0 to at most {req_q0:.2} so q ≤ {q_max:.2}.",), &format!("q0 auf höchstens {req_q0:.2} senken, damit q ≤ {q_max:.2}.")),
                    ));
                }
                report.push(c.build());
            }

            let mut shear = CheckResult::assess(
                format!("en1998.1.{bid}.{}.baseShear", system.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.3.3.2.2"),
                SubjectRef::new(bid, format!("{spath}.baseShearResistanceN"), lc(
                &format!("System {}", system.id),
                &format!("Nachweis: System {}", system.id),
            )),
                lc("Seismic base shear F_b ≤ V_Rd", "Erdbeben-Querkraft F_b ≤ V_Rd"),
            )
            .utilization(q_force(f_b), q_force(system.base_shear_resistance_n))
            .annex(annex)
            .explanation(lc(
                &format!("dir={}, Sd(T1={t1:.3}s)={s_d:.3} m/s², λ={lambda:.2}, Fb={:.0} N, VRd={:.0} N, q={q:.2}", system.direction, f_b, system.base_shear_resistance_n),
                &format!("Richtung={}, Sd(T1={t1:.3}s)={s_d:.3} m/s², λ={lambda:.2}, F_b={:.0} N, V_Rd={:.0} N, q={q:.2}", system.direction, f_b, system.base_shear_resistance_n),
            ));
            if f_b > system.base_shear_resistance_n {
                let req_v = f_b;
                let req_mass = (system.base_shear_resistance_n / (s_d * lambda)).max(0.0);
                shear = shear
                    .remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{spath}.baseShearResistanceN"), lc("Base shear resistance", "Querkrafttragfähigkeit")),
                        q_force(system.base_shear_resistance_n),
                        q_force(req_v),
                        lc(&format!("Increase V_Rd from {:.0} N to at least {:.0} N (or add walls).", system.base_shear_resistance_n, req_v), &format!("V_Rd von {:.0} N auf mindestens {:.0} N erhöhen (oder Wände ergänzen).", system.base_shear_resistance_n, req_v)),
                    ))
                    .remedy(Remedy::at_most(
                        SubjectRef::new(bid, format!("{bpath}.storeys[id={}].permanentGkN", building.storeys.first().map(|s| s.id.as_str()).unwrap_or("s1")), lc("Permanent G_k", "Ständige Einwirkung G_k")),
                        q_force(building.storeys.first().map(|s| s.permanent_gk_n).unwrap_or(0.0)),
                        q_force((req_mass * 9.81 * 0.85).max(0.0)),
                        lc(&format!("Reduce permanent G_k so seismic mass approaches {req_mass:.0} kg.",), &format!("Ständige G_k senken, damit seismische Masse Richtung {req_mass:.0} kg geht.")),
                    ));
            }
            report.push(shear.build());

            // Accidental torsion §4.3.3.2.4: e_ai = ±0.05 L_i; δ = 1 + 0.6 x/L_e amplifies storey forces.
            let plan_w = if system.direction == "y" { building.plan_length_m } else { building.plan_width_m };
            let e_ai = 0.05 * plan_w.max(0.0);
            let (delta_torsion, e_tot) = if building.plan_width_m > 0.0 && building.plan_length_m > 0.0 {
                let st = building.storeys.first();
                let e0 = st.map(|s| {
                    let cm = if system.direction == "y" { s.centre_of_mass_x_m } else { s.centre_of_mass_y_m };
                    let cs = if system.direction == "y" { s.centre_of_stiffness_x_m } else { s.centre_of_stiffness_y_m };
                    (cm - cs).abs()
                }).unwrap_or(0.0);
                // Design eccentricity includes accidental; ratio field scales e_ai (normative 0.05 → ratio=1).
                let e_ai_eff = e_ai * (building.accidental_eccentricity_ratio / 0.05).max(0.0);
                let e_tot = e0 + e_ai_eff;
                let l_e = plan_w.max(1e-9);
                // δ = 1 + 0.6·(e_tot)/(L_e/2) = 1 + 1.2·e_tot/L_e (EN 1998-1 §4.3.3.2.4).
                let delta = (1.0 + 1.2 * e_tot / l_e).min(3.0);
                let x = e_tot;
                if building.plan_width_m > 0.0 && building.plan_length_m > 0.0 {
                    let mut tor = CheckResult::assess(
                        format!("en1998.1.{bid}.{}.torsion", system.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "4.3.3.2.4"),
                        SubjectRef::new(bid, format!("{bpath}.accidentalEccentricityRatio"), blabel.clone()),
                        lc("Accidental torsion amplification δ", "Zufällige Torsion δ"),
                    )
                    .utilization(q_dim(delta), q_dim(1.5))
                    .annex(annex)
                    .explanation(lc(
                        &format!("e_ai={e_ai_eff:.3} m (0.05·L·ratio), e0={e0:.3} m, e_tot={e_tot:.3} m, x={x:.3} m, L_e={l_e:.1} m, δ={delta:.3}"),
                        &format!("e_ai={e_ai_eff:.3} m (0.05·L·Verhältnis), e0={e0:.3} m, e_tot={e_tot:.3} m, x={x:.3} m, L_e={l_e:.1} m, δ={delta:.3}"),
                    ));
                    if delta > 1.5 {
                        tor = tor.remedy(Remedy::at_most(
                            SubjectRef::new(bid, format!("{bpath}.accidentalEccentricityRatio"), lc("Accidental eccentricity ratio", "Zufällige Exzentrizität")),
                            q_dim(building.accidental_eccentricity_ratio),
                            q_dim(0.05),
                            lc("Reduce accidental eccentricity ratio toward 0.05 or reduce structural eccentricity e₀.", "Zufällige Exzentrizität Richtung 0.05 senken oder strukturelle Exzentrizität e₀ verringern."),
                        ));
                    }
                    report.push(tor.build());
                }
                (delta, e_tot)
            } else {
                (1.0, 0.0)
            };
            // Storey shear demand vs resistance — F_i and V_i amplified by δ (§4.3.3.2.4 / §4.3.3.3).
            let forces_raw = part_1::storey_forces_n(f_b, &masses, &heights);
            let forces: Vec<f64> = forces_raw.iter().map(|f| f * delta_torsion).collect();
            let mut remaining_v = f_b * delta_torsion;
            for (sti, storey) in building.storeys.iter().enumerate() {
                let is_roof = sti + 1 == building.storeys.len();
                if is_roof {
                    let mut ph = CheckResult::assess(
                        format!("en1998.1.{bid}.storey.{}.roofPhi", storey.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "4.2.4"),
                        SubjectRef::new(bid, format!("{}.storeys[id={}].correlatedOccupancy", bpath, storey.id), lc(&storey.id, &storey.id)),
                        lc("Roof φ = 1.0 (Table 4.2)", "Dach φ = 1.0 (Tabelle 4.2)"),
                    )
                    .utilization(q_dim(if storey.correlated_occupancy { 0.5 } else { 1.5 }), q_dim(1.0))
                    .annex(annex)
                    .explanation(lc(
                        &format!("correlatedOccupancy={} (roof requires true so φ=1.0)", storey.correlated_occupancy),
                        &format!("korrelierteNutzung={} (Dach erfordert true, damit φ=1.0)", storey.correlated_occupancy),
                    ));
                    if !storey.correlated_occupancy {
                        ph = ph.remedy(Remedy::exactly(
                            SubjectRef::new(bid, format!("{}.storeys[id={}].correlatedOccupancy", bpath, storey.id), lc("Correlated occupancy", "Korrelierte Nutzung")),
                            q_dim(0.0),
                            q_dim(1.0),
                            lc("Set correlatedOccupancy true for the roof storey (Table 4.2 φ=1.0).", "Korrelierte Nutzung für das Dachgeschoss auf true setzen (Tab. 4.2 φ=1.0)."),
                        ));
                    }
                    report.push(ph.build());
                }

                let fi = forces.get(sti).copied().unwrap_or(0.0);
                // storey shear demand = sum of forces at and above this storey
                let v_i = remaining_v;
                remaining_v = (remaining_v - fi).max(0.0);
                let v_rd = if storey.shear_resistance_n > 0.0 { storey.shear_resistance_n } else { system.base_shear_resistance_n };
                let stpath = format!("{}.storeys[id={}]", bpath, storey.id);
                let mut sc = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.storeyForces.{}", system.id, storey.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "4.3.3.2.3"),
                    SubjectRef::new(bid, format!("{stpath}.shearResistanceN"), lc(&storey.id, &storey.id)),
                    lc("Storey shear V_i ≤ V_Rd,i", "Geschossquerkraft V_i ≤ V_Rd,i"),
                )
                .utilization(q_force(v_i), q_force(v_rd))
                .annex(annex)
                .explanation(lc(
                &format!("Fi={fi:.0} N, Vi={v_i:.0} N, VRd,i={v_rd:.0} N",),
                &format!("Fi={fi:.0} N, Vi={v_i:.0} N, V_Rd,i={v_rd:.0} N",),
            ));
                if v_i > v_rd {
                    sc = sc.remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{stpath}.shearResistanceN"), lc("Storey shear resistance", "Geschoss-Querkrafttragfähigkeit")),
                        q_force(v_rd),
                        q_force(v_i),
                        lc(&format!("Increase storey shear resistance to at least {v_i:.0} N.",), &format!("Geschoss-Querkrafttragfähigkeit auf mindestens {v_i:.0} N erhöhen.")),
                    ));
                }
                report.push(sc.build());
            }

            // Drift + P-Δ
            let theta_lim = part_1::drift_theta(&building.drift_limit_class);
            let mut remaining_weight: f64 = building.storeys.iter().enumerate().map(|(i, st)| {
                part_1::storey_seismic_action_n(st, i + 1 == building.storeys.len())
            }).sum();
            let mut shear_above = f_b;
            for storey in &building.storeys {
                let drift = if system.direction == "y" { storey.drift_y_m } else { storey.drift_x_m };
                let limit = part_1::interstorey_drift_limit_m(storey.height_m, building.nu, theta_lim);
                let dfield = if system.direction == "y" { "driftYM" } else { "driftXM" };
                let dpath = format!("{}.storeys[id={}].{}", bpath, storey.id, dfield);
                let mut drift_check = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.drift.{}", system.id, storey.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "4.4.3.2"),
                    SubjectRef::new(bid, dpath.clone(), lc(&storey.id, &storey.id)),
                    lc("Interstorey drift", "Stockwerksverschiebung"),
                )
                .utilization(q_len(drift), q_len(limit))
                .annex(annex)
                .explanation(lc(&format!("interstorey drift d={drift:.4} m ≤ limit {limit:.4} m"), &format!("Stockwerksverschiebung d={drift:.4} m ≤ Grenzwert {limit:.4} m")));
                if drift > limit {
                    drift_check = drift_check.remedy(Remedy::at_most(
                        SubjectRef::new(bid, dpath, lc("Drift", "Verschiebung")),
                        q_len(drift),
                        q_len(limit),
                        lc("Reduce interstorey drift (stiffen or lower q).", "Stockwerksverschiebung reduzieren (aussteifen oder q senken)."),
                    ));
                }
                report.push(drift_check.build());

                let sti = building.storeys.iter().position(|s| s.id == storey.id).unwrap_or(0);
                let weight = part_1::storey_seismic_action_n(storey, sti + 1 == building.storeys.len());
                let k = if system.direction == "y" { storey.stiffness_y } else { storey.stiffness_x };
                let theta = if shear_above * storey.height_m > 1e-9 { remaining_weight * drift / (shear_above * storey.height_m) } else { 0.0 };
                let kpath = format!("{}.storeys[id={}].{}", bpath, storey.id, if system.direction == "y" { "stiffnessY" } else { "stiffnessX" });
                let mut pdelta = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.pdelta.{}", system.id, storey.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "4.4.2.2"),
                    SubjectRef::new(bid, kpath.clone(), lc(&storey.id, &storey.id)),
                    lc("P-Δ sensitivity θ", "P-Δ-Empfindlichkeit θ"),
                )
                .utilization(q_dim(theta), q_dim(0.3))
                .annex(annex)
                .explanation(lc(
                    &format!("P-Δ sensitivity θ={theta:.3}, k={k:.0}, kx={:.0}, ky={:.0}", storey.stiffness_x, storey.stiffness_y),
                    &format!("P-Δ-Empfindlichkeit θ={theta:.3}, Steifigkeit k={k:.0}, kx={:.0}, ky={:.0}", storey.stiffness_x, storey.stiffness_y),
                ));
                if theta > 0.3 {
                    let req_k = remaining_weight * drift / (0.3 * storey.height_m).max(1e-9);
                    pdelta = pdelta.remedy(Remedy::at_least(
                        SubjectRef::new(bid, kpath, lc("Stiffness", "Steifigkeit")),
                        Quantity::new(QuantityKind::Dimensionless, k),
                        Quantity::new(QuantityKind::Dimensionless, req_k),
                        lc(&format!("Increase storey stiffness to at least {req_k:.0}.",), &format!("Geschosssteifigkeit auf mindestens {req_k:.0} erhöhen.")),
                    ));
                }
                report.push(pdelta.build());
                remaining_weight = (remaining_weight - weight).max(0.0);
                let fi = forces.get(building.storeys.iter().position(|s| s.id == storey.id).unwrap_or(0)).copied().unwrap_or(0.0);
                shear_above = (shear_above - fi).max(0.0);
            }

            // Material-specific ductility detailing
            let gov_dc = &system.ductility_class;
            for member in &building.members {
                let mpath = format!("{}.members[id={}]", bpath, member.id);
                let mat = member.material.to_ascii_lowercase();
                if mat == "rc" || mat == "concrete" {
                    let min_b = part_1::rc_column_min_b_m(gov_dc);
                    if member.role.to_ascii_lowercase() == "column" {
                        let ok = member.min_dimension_m + 1e-12 >= min_b;
                        let mut c = CheckResult::assess(
                            format!("en1998.1.{bid}.member.{}.rc.minB", member.id),
                            "DIN EN 1998-1",
                            ClauseId::new("EN 1998-1", "1", "5.4.3.2.1"),
                            SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc(&member.id, &member.id)),
                            lc("RC column minimum dimension", "Stahlbetonstütze Mindestabmessung"),
                        )
                        .minimum(q_len(member.min_dimension_m), q_len(min_b))
                        .annex(annex)
                        .explanation(lc(&format!("b={:.3} m, required≥{min_b:.3} m", member.min_dimension_m), &format!("b={:.3} m, erforderlich≥{min_b:.3} m", member.min_dimension_m)));
                        if !ok {
                            c = c.remedy(Remedy::at_least(
                                SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc("Min dimension", "Mindestabmessung")),
                                q_len(member.min_dimension_m),
                                q_len(min_b),
                                lc(&format!("Increase column min dimension to {min_b:.3} m.",), &format!("Stützenmindestabmessung auf {min_b:.3} m erhöhen.")),
                            ));
                        }
                        report.push(c.build());
                    }
                    
                    {
                        let mut c = CheckResult::assess(
                            format!("en1998.1.{bid}.member.{}.rc.rhoPrimeAbs", member.id),
                            "DIN EN 1998-1",
                            ClauseId::new("EN 1998-1", "1", "5.4.3"),
                            SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc(&member.id, &member.id)),
                            lc("RC reinforcement ρ′", "Stahlbetonbewehrung ρ′"),
                        )
                        .utilization(q_dim(member.rho_prime), q_dim(member.rho.max(1e-6)))
                        .annex(annex)
                        .explanation(lc(
                            &format!("ρ′={:.4}, ρ={:.4}", member.rho_prime, member.rho),
                            &format!("ρ′={:.4}, ρ={:.4} (Bewehrung)", member.rho_prime, member.rho),
                        ));
                        if member.rho_prime > member.rho + 1e-12 {
                            c = c.remedy(Remedy::at_most(
                                SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc("ρ′", "ρ′")),
                                q_dim(member.rho_prime),
                                q_dim(member.rho),
                                lc("Reduce ρ′ to at most ρ.", "ρ′ auf höchstens ρ senken."),
                            ));
                        }
                        report.push(c.build());
                    }
if member.role.to_ascii_lowercase() == "beam" {
                        let min_b = 0.20; // EN 1998-1 §5.4.1.2.1 / §5.5.1.2.1 order-of-magnitude beam width floor for DCH detailing path
                        let ok = member.min_dimension_m + 1e-12 >= min_b;
                        let mut c = CheckResult::assess(
                            format!("en1998.1.{bid}.member.{}.rc.beamMinB", member.id),
                            "DIN EN 1998-1",
                            ClauseId::new("EN 1998-1", "1", "5.4.1.2.1"),
                            SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc(&member.id, &member.id)),
                            lc("RC beam minimum width", "Stahlbetonbalken Mindestbreite"),
                        )
                        .minimum(q_len(member.min_dimension_m), q_len(min_b))
                        .annex(annex)
                        .explanation(lc(
                            &format!("b={:.3} m, required≥{min_b:.3} m", member.min_dimension_m),
                            &format!("b={:.3} m, erforderlich≥{min_b:.3} m", member.min_dimension_m),
                        ));
                        if !ok {
                            c = c.remedy(Remedy::at_least(
                                SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc("Min dimension", "Mindestabmessung")),
                                q_len(member.min_dimension_m),
                                q_len(min_b),
                                lc(&format!("Increase beam min width to {min_b:.3} m."), &format!("Balkenmindestbreite auf {min_b:.3} m erhöhen.")),
                            ));
                        }
                        report.push(c.build());
                    }
                    let (rho_min, rho_max) = part_1::rc_rho_limits(&member.role, gov_dc);
                    let ok_rho = member.rho + 1e-12 >= rho_min && member.rho <= rho_max + 1e-12;
                    let mut c = CheckResult::assess(
                        format!("en1998.1.{bid}.member.{}.rc.rho", member.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "5.4.3"),
                        SubjectRef::new(bid, format!("{mpath}.rho"), lc(&member.id, &member.id)),
                        lc("RC longitudinal reinforcement ratio ρ", "Stahlbeton-Längsbewehrungsgrad ρ"),
                    )
                    .utilization(q_dim(member.rho), q_dim(if member.rho < rho_min { rho_min } else { rho_max }))
                    .annex(annex)
                    .explanation(lc(&format!("ρ={:.4}, limits [{rho_min:.4}, {rho_max:.4}]", member.rho), &format!("ρ={:.4}, Grenzen [{rho_min:.4}, {rho_max:.4}]", member.rho)));
                    if !ok_rho {
                        c = c.status(crate::document::CheckStatus::Fail);
                    }
                    if member.rho + 1e-12 < rho_min {
                        c = c.remedy(Remedy::at_least(
                            SubjectRef::new(bid, format!("{mpath}.rho"), lc("ρ", "ρ")),
                            q_dim(member.rho),
                            q_dim(rho_min),
                            lc(&format!("Increase ρ to at least {rho_min:.4}.",), &format!("ρ auf mindestens {rho_min:.4} erhöhen.")),
                        ));
                    } else if member.rho > rho_max + 1e-12 {
                        c = c.remedy(Remedy::at_most(
                            SubjectRef::new(bid, format!("{mpath}.rho"), lc("ρ", "ρ")),
                            q_dim(member.rho),
                            q_dim(rho_max),
                            lc(&format!("Reduce ρ to at most {rho_max:.4}.",), &format!("ρ auf höchstens {rho_max:.4} senken.")),
                        ));
                    }
                    report.push(c.build());


                    {
                        let mut sc = CheckResult::assess(
                            format!("en1998.1.{bid}.member.{}.rc.noSteelClass", member.id),
                            "DIN EN 1998-1",
                            ClauseId::new("EN 1998-1", "1", "5"),
                            SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc(&member.id, &member.id)),
                            lc("RC member has no steel section class", "Stahlbetonbauteil ohne Stahl-Querschnittsklasse"),
                        )
                        .utilization(q_dim(member.steel_section_class as f64), q_dim(0.5))
                        .annex(annex)
                        .explanation(lc(
                            &format!("steelSectionClass={} (must be 0 for RC)", member.steel_section_class),
                            &format!("Stahl-Querschnittsklasse={} (muss 0 für Stahlbeton sein)", member.steel_section_class),
                        ));
                        if member.steel_section_class != 0 {
                            sc = sc.status(crate::document::CheckStatus::Fail).remedy(Remedy::exactly(
                                SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc("Steel section class", "Stahl-Querschnittsklasse")),
                                q_dim(member.steel_section_class as f64),
                                q_dim(0.0),
                                lc("Set steelSectionClass to 0 for RC members.", "Stahl-Querschnittsklasse für Stahlbeton auf 0 setzen."),
                            ));
                        }
                        report.push(sc.build());
                    }

                    let w_min = part_1::rc_omega_wd_min(gov_dc);
                    if w_min > 0.0 && member.role.to_ascii_lowercase() == "column" {
                        let ok = member.omega_wd + 1e-12 >= w_min;
                        let mut c = CheckResult::assess(
                            format!("en1998.1.{bid}.member.{}.rc.omegaWd", member.id),
                            "DIN EN 1998-1",
                            ClauseId::new("EN 1998-1", "1", "5.4.3.2.2"),
                            SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc(&member.id, &member.id)),
                            lc("RC confinement ω_wd", "Stahlbeton-Umschnürung ω_wd"),
                        )
                        .minimum(q_dim(member.omega_wd), q_dim(w_min))
                        .annex(annex)
                        .explanation(lc(&format!("ω_wd={:.3}, required≥{w_min:.3}", member.omega_wd), &format!("ω_wd={:.3}, erforderlich≥{w_min:.3}", member.omega_wd)));
                        if !ok {
                            c = c.remedy(Remedy::at_least(
                                SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc("ω_wd", "ω_wd")),
                                q_dim(member.omega_wd),
                                q_dim(w_min),
                                lc(&format!("Increase ω_wd to at least {w_min:.3}.",), &format!("ω_wd auf mindestens {w_min:.3} erhöhen.")),
                            ));
                        }
                        report.push(c.build());
                    }
                    if member.role.to_ascii_lowercase() == "beam" {
                        let ratio = if member.rho > 1e-12 { member.rho_prime / member.rho } else { 0.0 };
                        let ok = ratio + 1e-12 >= 0.5;
                        let mut c = CheckResult::assess(
                            format!("en1998.1.{bid}.member.{}.rc.rhoPrime", member.id),
                            "DIN EN 1998-1",
                            ClauseId::new("EN 1998-1", "1", "5.4.3.1.2"),
                            SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc(&member.id, &member.id)),
                            lc("Beam ρ′/ρ ≥ 0.5", "Träger ρ′/ρ ≥ 0.5"),
                        )
                        .minimum(q_dim(ratio), q_dim(0.5))
                        .annex(annex)
                        .explanation(lc(&format!("beam reinforcement ratio ρ′/ρ={ratio:.3}"), &format!("Träger-Bewehrungsverhältnis ρ′/ρ={ratio:.3}")));
                        if !ok {
                            let req = 0.5 * member.rho;
                            c = c.remedy(Remedy::at_least(
                                SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc("ρ′", "ρ′")),
                                q_dim(member.rho_prime),
                                q_dim(req),
                                lc(&format!("Increase ρ′ to at least {req:.4}.",), &format!("ρ′ auf mindestens {req:.4} erhöhen.")),
                            ));
                        }
                        report.push(c.build());
                    }
                    if member.role.to_ascii_lowercase() != "column" {
                        let mut c = CheckResult::assess(
                            format!("en1998.1.{bid}.member.{}.rc.omegaWdBeam", member.id),
                            "DIN EN 1998-1",
                            ClauseId::new("EN 1998-1", "1", "5.4.3.2.2"),
                            SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc(&member.id, &member.id)),
                            lc("RC beam confinement ω_wd (recorded)", "Stahlbetonbalken-Umschnürung ω_wd (dokumentiert)"),
                        )
                        .utilization(q_dim(member.omega_wd), q_dim(0.30))
                        .annex(annex)
                        .explanation(lc(
                            &format!("ω_wd={:.3} (beam; limit 0.30 for excessive confinement steel)", member.omega_wd),
                            &format!("ω_wd={:.3} (Balken; Grenze 0.30 für übermässige Umschnürung)", member.omega_wd),
                        ));
                        if member.omega_wd > 0.30 {
                            c = c.remedy(Remedy::at_most(
                                SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc("ω_wd", "ω_wd")),
                                q_dim(member.omega_wd),
                                q_dim(0.30),
                                lc("Reduce beam ω_wd to at most 0.30.", "Balken-ω_wd auf höchstens 0.30 senken."),
                            ));
                        }
                        report.push(c.build());
                    }
                }
                if mat == "steel" {
                    let max_class = part_1::steel_max_section_class(q);
                    let ok = member.steel_section_class > 0 && member.steel_section_class <= max_class;
                    let mut c = CheckResult::assess(
                        format!("en1998.1.{bid}.member.{}.steel.class", member.id),
                        "DIN EN 1998-1",
                        ClauseId::new("EN 1998-1", "1", "6.5"),
                        SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc(&member.id, &member.id)),
                        lc("Steel section class vs q", "Stahl-Querschnittsklasse vs q"),
                    )
                    .utilization(q_dim(if ok { (member.steel_section_class as f64).min(max_class as f64) } else { member.steel_section_class as f64 }), q_dim(max_class as f64))
                    .annex(annex)
                    .explanation(lc(&format!("class={}, max for q={q:.2} is {max_class}", member.steel_section_class), &format!("Klasse={}, max für q={q:.2} ist {max_class}", member.steel_section_class)));
                    if !ok {
                        c = c.remedy(Remedy::at_most(
                            SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc("Section class", "Querschnittsklasse")),
                            q_dim(member.steel_section_class as f64),
                            q_dim(max_class as f64),
                            lc(&format!("Use section class ≤ {max_class} (or reduce q0).",), &format!("Querschnittsklasse ≤ {max_class} verwenden (oder q0 senken).")),
                        ));
                    }
                    report.push(c.build());
                }

                // Legacy flag still reported (summary of detailing claim)
                let ok = member.detailing_compatible_with_q;
                let mut mcheck = CheckResult::assess(
                    format!("en1998.1.{bid}.member.{}.qDetail", member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5.2"),
                    SubjectRef::new(bid, format!("{mpath}.detailingCompatibleWithQ"), lc(&member.id, &member.id)),
                    lc("q-compatible detailing declared", "q-gerechte Durchbildung erklärt"),
                )
                .utilization(q_dim(if ok { 0.5 } else { 2.0 }), q_dim(1.0))
                .annex(annex)
                .explanation(lc(
                    &format!("Member {} detailing_compatible_with_q={ok} for q={q:.2}", member.id),
                    &format!("Bauteil {} detailing_compatible_with_q={ok} für q={q:.2}", member.id),
                ));
                if !ok {
                    mcheck = mcheck.remedy(Remedy::exactly(
                        SubjectRef::new(bid, format!("{mpath}.detailingCompatibleWithQ"), lc("Detailing flag", "Durchbildungsflag")),
                        q_dim(0.0),
                        q_dim(1.0),
                        lc("Provide q-compatible detailing (or reduce q0).", "q-gerechte Durchbildung herstellen (oder q0 reduzieren)."),
                    ));
                }
                report.push(mcheck.build());
            }

            {
                let zone = document.site.seismic_zone.as_u8();
                let limit = part_1::masonry_wall_ratio_limit(zone, &document.site.importance_class);
                let claiming = building.claims_simple_masonry || system.system_type == "masonry";
                let mut m = CheckResult::assess(
                    format!("en1998.1.{bid}.masonry.simple"),
                    "DIN EN 1998-1/NA",
                    ClauseId::new("EN 1998-1", "1", "9.7.2"),
                    SubjectRef::new(bid, format!("{bpath}.masonryWallAreaRatio"), blabel.clone()),
                    lc("Simple masonry building wall ratio (DE Table NA.12)", "Einfache Mauerwerksbauten Wandanteil (DE Tab. NA.12)"),
                )
                .utilization(q_dim(building.masonry_wall_area_ratio), q_dim(1.0))
                .annex(annex)
                .explanation(lc(
                    &format!("claims={}, wall ratio={:.3}, required≥{:.3}", claiming, building.masonry_wall_area_ratio, limit),
                    &format!("Beansprucht={}, Wandanteil={:.3}, erforderlich≥{:.3}", claiming, building.masonry_wall_area_ratio, limit),
                ));
                if claiming && building.masonry_wall_area_ratio + 1e-12 < limit {
                    m = m.status(crate::document::CheckStatus::Fail).remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{bpath}.masonryWallAreaRatio"), lc("Wall area ratio", "Wandflächenanteil")),
                        q_dim(building.masonry_wall_area_ratio),
                        q_dim(limit),
                        lc(&format!("Increase masonry wall area ratio to at least {limit:.3}."), &format!("Mauerwerks-Wandanteil auf mindestens {limit:.3} erhöhen.")),
                    ));
                } else {
                    m = m.status(crate::document::CheckStatus::Pass);
                }
                report.push(m.build());
            }
        }
    }

    evaluate_bridges(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td);
    evaluate_assessments(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td);
    evaluate_silos_tanks(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td);
    evaluate_foundations_walls(&mut report, document, &annex_params, a_g, s, tb, tc, td, gamma_i);
    evaluate_towers(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td);
    report
}


fn push_duplicate_ids(report: &mut CheckReport, annex: AnnexChoice, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, lc(&format!("Duplicate {table} id"), &format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let free = if options.is_empty() { vec![format!("{id}-unique")] } else { options };
        report.push(
            CheckResult::assess(
                format!("en1998.integrity.duplicate.{table}.{id}"),
                "EN 1998 integrity",
                ClauseId::new("EN 1998", "§2", "id"),
                subject.clone(),
                lc(&format!("Unique {table} id"), &format!("Eindeutige {table}-Id")),
            )
            .annex(annex)
            .explanation(lc(
                &format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
                &format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
            ))
            .status(crate::document::CheckStatus::Fail)
            .remedy(Remedy::one_of(
                subject,
                free,
                lc(
                    &format!("Rename the duplicated '{id}' entry to a free id."),
                    &format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
                ),
            ))
            .build(),
        );
    }
}

fn push_referential_integrity(report: &mut CheckReport, document: &En1998Snapshot, annex: AnnexChoice) {
    push_duplicate_ids(report, annex, "buildings", &document.buildings.iter().map(|b| b.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={id}].id"));
    push_duplicate_ids(report, annex, "bridges", &document.bridges.iter().map(|b| b.id.clone()).collect::<Vec<_>>(), &|id| format!("bridges[id={id}].id"));
    push_duplicate_ids(report, annex, "assessments", &document.assessments.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), &|id| format!("assessments[id={id}].id"));
    push_duplicate_ids(report, annex, "silos", &document.silos.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("silos[id={id}].id"));
    push_duplicate_ids(report, annex, "tanks", &document.tanks.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), &|id| format!("tanks[id={id}].id"));
    push_duplicate_ids(report, annex, "foundations", &document.foundations.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), &|id| format!("foundations[id={id}].id"));
    push_duplicate_ids(report, annex, "retainingWalls", &document.retaining_walls.iter().map(|w| w.id.clone()).collect::<Vec<_>>(), &|id| format!("retainingWalls[id={id}].id"));
    push_duplicate_ids(report, annex, "towers", &document.towers.iter().map(|t| t.id.clone()).collect::<Vec<_>>(), &|id| format!("towers[id={id}].id"));

    for building in &document.buildings {
        let bid = &building.id;
        push_duplicate_ids(report, annex, "systems", &building.systems.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].systems[id={id}].id"));
        push_duplicate_ids(report, annex, "storeys", &building.storeys.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].storeys[id={id}].id"));
        push_duplicate_ids(report, annex, "members", &building.members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].members[id={id}].id"));
        for storey in &building.storeys {
            let sid = &storey.id;
            push_duplicate_ids(report, annex, "variables", &storey.variables.iter().map(|v| v.id.clone()).collect::<Vec<_>>(), &|id| format!("buildings[id={bid}].storeys[id={sid}].variables[id={id}].id"));
        }
    }
    for bridge in &document.bridges {
        let brid = &bridge.id;
        push_duplicate_ids(report, annex, "variables", &bridge.variables.iter().map(|v| v.id.clone()).collect::<Vec<_>>(), &|id| format!("bridges[id={brid}].variables[id={id}].id"));
    }
    for tower in &document.towers {
        let tid = &tower.id;
        push_duplicate_ids(report, annex, "variables", &tower.variables.iter().map(|v| v.id.clone()).collect::<Vec<_>>(), &|id| format!("towers[id={tid}].variables[id={id}].id"));
    }
}

fn evaluate_bridges(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) {
    let annex = annex_params.choice();
    if document.bridges.is_empty() {
        report.push(CheckResult::assess("en1998.2.na", "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "5"), SubjectRef::whole(lc("Bridges", "Brücken")), lc("Bridge seismic checks", "Brücken-Erdbebennachweise")).not_applicable(lc("No bridges in the subject.", "Keine Brücken im Gegenstand.")).annex(annex).build());
        return;
    }
    for bridge in &document.bridges {
        let t1 = bridge.fundamental_period_s.max(0.05);
        let q_isol = part_2::isolation_reduction_factor(bridge.period_ratio);
        {
            let mut pr = CheckResult::assess(
                format!("en1998.2.{}.periodRatio", bridge.id),
                "DIN EN 1998-2",
                ClauseId::new("EN 1998-2", "2", "7.1"),
                SubjectRef::new(&bridge.id, format!("{}.periodRatio", id_sel("bridges", &bridge.id)), lc(&bridge.id, &bridge.id)),
                lc("Isolation period ratio T_isol/T_fixed", "Isolations-Periodenverhältnis T_isol/T_fixed"),
            )
            .utilization(q_dim(bridge.period_ratio), q_dim(3.0))
            .annex(annex)
            .explanation(lc(
                &format!("periodRatio={:.3}, q_isol={q_isol:.3}", bridge.period_ratio),
                &format!("Periodenverhältnis={:.3}, q_isol={q_isol:.3}", bridge.period_ratio),
            ));
            if bridge.period_ratio < 1.0 {
                pr = pr.status(crate::document::CheckStatus::Fail).remedy(Remedy::at_least(
                    SubjectRef::new(&bridge.id, format!("{}.periodRatio", id_sel("bridges", &bridge.id)), lc("Period ratio", "Periodenverhältnis")),
                    q_dim(bridge.period_ratio),
                    q_dim(1.0),
                    lc("Increase isolation period ratio to at least 1.0.", "Isolations-Periodenverhältnis auf mindestens 1.0 erhöhen."),
                ));
            }
            report.push(pr.build());
        }
        let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t1);
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q_isol, a_g);
        let mass = part_1::entity_seismic_mass_kg(bridge.permanent_gk_n, &bridge.variables, false, bridge.correlated_occupancy);
        let v = part_1::base_shear_n(s_d, mass, 1.0);
        let d_ed = part_2::bearing_displacement_m(s_d, t1);
        let path = id_sel("bridges", &bridge.id);
        let mut c = CheckResult::assess(format!("en1998.2.{}.shear", bridge.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "5.3"), SubjectRef::new(&bridge.id, format!("{path}.vRdN"), lc(&bridge.id, &bridge.id)), lc("Bridge base shear", "Brücken-Querkraft")).utilization(q_force(v), q_force(bridge.v_rd_n)).annex(annex).explanation(lc(
                &format!("T={t1:.3}s, periodRatio={:.2}, q_isol={q_isol:.2}, mass={mass:.0} kg, Ved={v:.0} N, Vrd={:.0} N", bridge.period_ratio, bridge.v_rd_n),
                &format!("T={t1:.3}s, Periodenverhältnis={:.2}, q_isol={q_isol:.2}, Masse={mass:.0} kg, V_Ed={v:.0} N, V_Rd={:.0} N", bridge.period_ratio, bridge.v_rd_n),
            ));
        if v > bridge.v_rd_n {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&bridge.id, format!("{path}.vRdN"), lc("V_Rd", "V_Rd")), q_force(bridge.v_rd_n), q_force(v), lc(&format!("Increase bridge V_Rd to at least {v:.0} N."), &format!("Brücken-V_Rd auf mindestens {v:.0} N erhöhen."))));
        }
        report.push(c.build());
        let mut b = CheckResult::assess(format!("en1998.2.{}.bearing", bridge.id), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "7.5"), SubjectRef::new(&bridge.id, format!("{path}.bearingDRdM"), lc(&bridge.id, &bridge.id)), lc("Bearing displacement", "Lagerverschiebung")).utilization(q_len(d_ed), q_len(bridge.bearing_d_rd_m)).annex(annex).explanation(lc(&format!("ded={d_ed:.4} m from Sd(T), drd={:.4} m", bridge.bearing_d_rd_m), &format!("ded={d_ed:.4} m aus Sd(T), drd={:.4} m", bridge.bearing_d_rd_m)));
        if d_ed > bridge.bearing_d_rd_m {
            b = b.remedy(Remedy::at_least(SubjectRef::new(&bridge.id, format!("{path}.bearingDRdM"), lc("d_Rd", "d_Rd")), q_len(bridge.bearing_d_rd_m), q_len(d_ed), lc("Increase bearing displacement capacity.", "Lagerverschiebungskapazität erhöhen.")));
        }
        report.push(b.build());
    }
}

fn evaluate_assessments(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) {
    let annex = annex_params.choice();
    if document.assessments.is_empty() {
        report.push(CheckResult::assess("en1998.3.na", "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "2.3"), SubjectRef::whole(lc("Assessments", "Bestandsbewertung")), lc("Retrofit checks", "Ertüchtigungsnachweise")).not_applicable(lc("No retrofit assessments in the subject.", "Keine Bestandsbewertungen im Gegenstand.")).annex(annex).build());
        return;
    }
    for a in &document.assessments {
        let path = id_sel("assessments", &a.id);
        let building_ok = document.buildings.iter().any(|b| b.id == a.supported_building_id);
        {
            let mut href = CheckResult::assess(
                format!("en1998.3.{}.supportedBuilding", a.id),
                "DIN EN 1998-3",
                ClauseId::new("EN 1998-3", "3", "2"),
                SubjectRef::new(&a.id, format!("{path}.supportedBuildingId"), lc(&a.id, &a.id)),
                lc("Assessment supported building reference", "Bewertung — Bezug Gebäude"),
            )
            .utilization(q_dim(if building_ok { 0.5 } else { 1.5 }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("supportedBuildingId={}{}", a.supported_building_id, if building_ok { " (resolved)" } else { " (dangling)" }),
                &format!("supportedBuildingId={}{}", a.supported_building_id, if building_ok { " (aufgelöst)" } else { " (hängend)" }),
            ));
            if !building_ok {
                let building_ids: Vec<String> = document.buildings.iter().map(|b| b.id.clone()).collect();
                let opts = if building_ids.is_empty() { vec![format!("{}-missing", a.supported_building_id)] } else { building_ids };
                href = href
                    .status(crate::document::CheckStatus::Fail)
                    .remedy(Remedy::one_of(
                        SubjectRef::new(&a.id, format!("{path}.supportedBuildingId"), lc("Supported building id", "Gebäude-ID")),
                        opts,
                        lc("Set supportedBuildingId to one of the existing building ids.", "supportedBuildingId auf eine der vorhandenen Gebäude-Ids setzen."),
                    ));
            }
            report.push(href.build());
        }

        let ls = a.limit_state.to_ascii_lowercase();
        let ag_fac = part_3::limit_state_ag_factor(&ls);
        let cf_raw = part_3::confidence_factor(&a.knowledge_level);
        let cf = if part_3::limit_state_applies_cf(&ls) { cf_raw } else { 1.0 };
        let r_d = part_3::design_capacity_n(a.r_k_n, cf, a.gamma_el);
        let support_id = if a.supported_building_id.is_empty() {
            document.buildings.first().map(|b| b.id.clone()).unwrap_or_default()
        } else {
            a.supported_building_id.clone()
        };
        let e_d = building_base_shear_n(document, &support_id, annex_params, gamma_i, a_g * ag_fac, s, tb, tc, td);
        let path = id_sel("assessments", &a.id);
        let mut c = CheckResult::assess(format!("en1998.3.{}.capacity", a.id), "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "2.1"), SubjectRef::new(&a.id, format!("{path}.rKN"), lc(&a.id, &a.id)), lc("Limit-state capacity check", "Grenzzustands-Tragfähigkeitsnachweis")).utilization(q_force(e_d), q_force(r_d)).annex(annex).explanation(lc(&format!("LS={}, ag×{ag_fac:.2}, CF={cf:.2}, Ed={e_d:.0} N from building {support_id}, Rd={r_d:.0} N", a.limit_state), &format!("GZ={}, ag×{ag_fac:.2}, CF={cf:.2}, Ed={e_d:.0} N aus Gebäude {support_id}, Rd={r_d:.0} N", a.limit_state)));
        if e_d > r_d {
            let req = e_d * cf * a.gamma_el;
            c = c.remedy(Remedy::at_least(SubjectRef::new(&a.id, format!("{path}.rKN"), lc("R_k", "R_k")), q_force(a.r_k_n), q_force(req), lc(&format!("Increase R_k to at least {req:.0} N (or lower limit-state demand)."), &format!("R_k auf mindestens {req:.0} N erhöhen (oder Grenzzustandslast senken)."))));
        }
        // Explicit limit-state selection check (reads limitState).
        let ls_ok = matches!(ls.as_str(), "nc" | "near_collapse" | "near-collapse" | "sd" | "significant_damage" | "significant-damage" | "dl" | "damage_limitation" | "damage-limitation");
        let mut ls_c = CheckResult::assess(format!("en1998.3.{}.limitState", a.id), "DIN EN 1998-3", ClauseId::new("EN 1998-3", "3", "2.1"), SubjectRef::new(&a.id, format!("{path}.limitState"), lc(&a.id, &a.id)), lc("EN 1998-3 limit state (NC/SD/DL)", "EN-1998-3-Grenzzustand (NC/SD/DL)")).utilization(q_dim(if ls_ok { 0.5 } else { 2.0 }), q_dim(1.0)).annex(annex).explanation(lc(
                &format!("limitState={}", a.limit_state),
                &format!("Grenzzustand={}", a.limit_state),
            ));
        if !ls_ok {
            ls_c = ls_c.remedy(Remedy::one_of(SubjectRef::new(&a.id, format!("{path}.limitState"), lc("Limit state", "Grenzzustand")), vec!["sd".into(), "nc".into(), "dl".into()], lc("Set limitState to nc, sd, or dl (Table 2.1).", "limitState auf nc, sd oder dl setzen (Tab. 2.1).")));
        }
        report.push(ls_c.build());
        report.push(c.build());
    }
}

fn evaluate_silos_tanks(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) {
    let annex = annex_params.choice();
    if document.silos.is_empty() {
        report.push(CheckResult::assess("en1998.4.silo.na", "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "3"), SubjectRef::whole(lc("Silos", "Silos")), lc("Silo checks", "Silo-Nachweise")).not_applicable(lc("No silos in the subject.", "Keine Silos im Gegenstand.")).annex(annex).build());
    }
    for silo in &document.silos {
        let hr = silo.height_m / silo.radius_m.max(1e-9);
        let q = part_4::silo_behaviour_factor(silo.q_nominal);
        let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, 0.3);
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q, a_g);
        let mass = part_1::filled_content_seismic_mass_kg(silo.permanent_gk_n, silo.content_qk_n, &silo.content_category, silo.filling_ratio);
        let v_i = part_1::base_shear_n(s_d, mass * part_4::impulsive_mass_ratio(hr), 1.0);
        let t_c = part_4::convective_period_s(silo.radius_m);
        let s_e_c = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t_c);
        let s_d_c = part_1::design_spectrum_sd(s_e_c, gamma_i, q, a_g);
        let v_c = part_1::base_shear_n(s_d_c, mass * part_4::convective_mass_ratio(hr), 1.0);
        let v = part_4::srss(v_i, v_c);
        let path = id_sel("silos", &silo.id);
        let mut c = CheckResult::assess(format!("en1998.4.{}.wall", silo.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "3.4"), SubjectRef::new(&silo.id, format!("{path}.nRdN"), lc(&silo.id, &silo.id)), lc("Silo wall / base shear", "Silowand / Fußquerkraft")).utilization(q_force(v), q_force(silo.n_rd_n)).annex(annex).explanation(lc(
                &format!("q={:.2}, V={v:.0} N, Nrd={:.0} N", silo.q_nominal, silo.n_rd_n),
                &format!("q={:.2}, V={v:.0} N, N_Rd={:.0} N", silo.q_nominal, silo.n_rd_n),
            ));
        if v > silo.n_rd_n {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&silo.id, format!("{path}.nRdN"), lc("N_Rd", "N_Rd")), q_force(silo.n_rd_n), q_force(v), lc("Increase silo wall resistance.", "Silowandtragfähigkeit erhöhen.")));
        }
        report.push(c.build());
        let mut a = CheckResult::assess(format!("en1998.4.{}.anchor", silo.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "3.5"), SubjectRef::new(&silo.id, format!("{path}.vRdN"), lc(&silo.id, &silo.id)), lc("Silo anchorage", "Silo-Verankerung")).utilization(q_force(v), q_force(silo.v_rd_n)).annex(annex).explanation(lc(&format!("Ved={v:.0} from impulsive/convective, Vrd={:.0}", silo.v_rd_n), &format!("Ved={v:.0} aus Impuls/Konvektion, Vrd={:.0}", silo.v_rd_n)));
        if v > silo.v_rd_n {
            a = a.remedy(Remedy::at_least(SubjectRef::new(&silo.id, format!("{path}.vRdN"), lc("V_Rd", "V_Rd")), q_force(silo.v_rd_n), q_force(v), lc("Increase anchorage resistance.", "Verankerungstragfähigkeit erhöhen.")));
        }
        report.push(a.build());
    }
    if document.tanks.is_empty() {
        report.push(CheckResult::assess("en1998.4.tank.na", "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "4"), SubjectRef::whole(lc("Tanks", "Behälter")), lc("Tank checks", "Tank-Nachweise")).not_applicable(lc("No tanks in the subject.", "Keine Tanks im Gegenstand.")).annex(annex).build());
    }
    for tank in &document.tanks {
        let hr = tank.height_m / tank.radius_m.max(1e-9);
        let t_i = part_4::impulsive_period_s(tank.height_m, tank.radius_m);
        let t_c = part_4::convective_period_s(tank.radius_m);
        let s_e_i = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t_i);
        let s_e_c = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t_c);
        let mass = part_1::filled_content_seismic_mass_kg(tank.permanent_gk_n, tank.content_qk_n, &tank.content_category, tank.filling_ratio);
        let v = mass * part_4::impulsive_mass_ratio(hr) * s_e_i + mass * part_4::convective_mass_ratio(hr) * s_e_c;
        let path = id_sel("tanks", &tank.id);
        let mut c = CheckResult::assess(format!("en1998.4.{}.shear", tank.id), "DIN EN 1998-4", ClauseId::new("EN 1998-4", "4", "4.3"), SubjectRef::new(&tank.id, format!("{path}.vRdN"), lc(&tank.id, &tank.id)), lc("Tank base shear", "Tank-Fußquerkraft")).utilization(q_force(v), q_force(tank.v_rd_n)).annex(annex).explanation(lc(&format!("tank base shear V_Ed={v:.0} N"), &format!("Tank-Fußquerkraft V_Ed={v:.0} N")));
        if v > tank.v_rd_n {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&tank.id, format!("{path}.vRdN"), lc("V_Rd", "V_Rd")), q_force(tank.v_rd_n), q_force(v), lc("Increase tank shear resistance.", "Tank-Querkrafttragfähigkeit erhöhen.")));
        }
        report.push(c.build());
    }
}

fn building_base_shear_n(document: &En1998Snapshot, building_id: &str, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) -> f64 {
    let Some(building) = document.buildings.iter().find(|b| b.id == building_id) else { return 0.0 };
    let total_height: f64 = building.storeys.iter().map(|st| st.height_m).sum();
    let n = building.storeys.len();
    let total_mass: f64 = building.storeys.iter().enumerate().map(|(i, st)| part_1::storey_seismic_mass_kg(st, i + 1 == n)).sum();
    let t1 = match building.t1_method.to_ascii_lowercase().as_str() {
        "given" => building.t1_given_s,
        _ => part_1::t1_from_ct(building.ct, total_height),
    };
    let system = building.systems.first();
    let q = system.map(|sys| part_1::behaviour_factor(sys.q0, sys.alpha_u_over_alpha_1, sys.k_w)).unwrap_or(1.5);
    let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t1);
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q, a_g);
    let lambda = part_1::lambda_factor(t1, tc, building.storeys.len());
    part_1::base_shear_n(s_d, total_mass, lambda)
}

fn evaluate_foundations_walls(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, a_g: f64, s: f64, tb: f64, tc: f64, td: f64, gamma_i: f64) {
    let annex = annex_params.choice();
    if document.foundations.is_empty() {
        report.push(CheckResult::assess("en1998.5.foundation.na", "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "7"), SubjectRef::whole(lc("Foundations", "Gründungen")), lc("Foundation checks", "Gründungsnachweise")).not_applicable(lc("No foundations in the subject.", "Keine Gründungen im Gegenstand.")).annex(annex).build());
    }
    for f in &document.foundations {
        let path = id_sel("foundations", &f.id);
        let building_ok = document.buildings.iter().any(|b| b.id == f.supported_building_id);
        {
            let mut href = CheckResult::assess(
                format!("en1998.5.{}.supportedBuilding", f.id),
                "DIN EN 1998-5",
                ClauseId::new("EN 1998-5", "5", "5.3"),
                SubjectRef::new(&f.id, format!("{path}.supportedBuildingId"), lc(&f.id, &f.id)),
                lc("Foundation supported building reference", "Gründung — Bezug Gebäude"),
            )
            .utilization(q_dim(if building_ok { 0.5 } else { 1.5 }), q_dim(1.0))
            .annex(annex)
            .explanation(lc(
                &format!("supportedBuildingId={}{}", f.supported_building_id, if building_ok { " (resolved)" } else { " (dangling)" }),
                &format!("supportedBuildingId={}{}", f.supported_building_id, if building_ok { " (aufgelöst)" } else { " (hängend)" }),
            ));
            if !building_ok {
                let building_ids: Vec<String> = document.buildings.iter().map(|b| b.id.clone()).collect();
                let opts = if building_ids.is_empty() { vec![format!("{}-missing", f.supported_building_id)] } else { building_ids };
                href = href
                    .status(crate::document::CheckStatus::Fail)
                    .remedy(Remedy::one_of(
                        SubjectRef::new(&f.id, format!("{path}.supportedBuildingId"), lc("Supported building id", "Gebäude-ID")),
                        opts,
                        lc("Set supportedBuildingId to one of the existing building ids.", "supportedBuildingId auf eine der vorhandenen Gebäude-Ids setzen."),
                    ));
            }
            report.push(href.build());
        }

        let support_id = if f.supported_building_id.is_empty() {
            document.buildings.first().map(|b| b.id.clone()).unwrap_or_default()
        } else {
            f.supported_building_id.clone()
        };
        let v_b = building_base_shear_n(document, &support_id, annex_params, gamma_i, a_g, s, tb, tc, td);
        let red = part_5::bearing_reduction_factor(a_g);
        let p_rd = f.p_rd_pa * red;
        let p_ed = part_5::seismic_bearing_pressure_pa(v_b, f.area_m2);
        let mut c = CheckResult::assess(format!("en1998.5.{}.bearing", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "7.3"), SubjectRef::new(&f.id, format!("{path}.pRdPa"), lc(&f.id, &f.id)), lc("Seismic bearing pressure", "Seismische Sohldruckspannung")).utilization(q_press(p_ed), q_press(p_rd)).annex(annex).explanation(lc(&format!("building={support_id}, Vb={v_b:.0} N, ped={p_ed:.0} Pa, prd_red={p_rd:.0} Pa",), &format!("Gebäude={support_id}, Vb={v_b:.0} N, ped={p_ed:.0} Pa, prd_red={p_rd:.0} Pa")));
        if p_ed > p_rd {
            let req_area = v_b / p_rd.max(1.0);
            c = c.remedy(Remedy::at_least(SubjectRef::new(&f.id, format!("{path}.areaM2"), lc("Area", "Fläche")), Quantity::area_m2(f.area_m2), Quantity::area_m2(req_area), lc(&format!("Increase foundation area to at least {req_area:.2} m²."), &format!("Gründungsfläche auf mindestens {req_area:.2} m² erhöhen."))));
        }
        report.push(c.build());
        let h_ed = v_b; // EN 1998-5 §5.3 — horizontal demand from supported building base shear
        let mut sld = CheckResult::assess(format!("en1998.5.{}.sliding", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.3"), SubjectRef::new(&f.id, format!("{path}.hRdN"), lc(&f.id, &f.id)), lc("Foundation sliding", "Gleiten der Gründung")).utilization(q_force(h_ed), q_force(f.h_rd_n)).annex(annex).explanation(lc(&format!("Hed=Vb={h_ed:.0} N from {support_id}, Hrd={:.0}", f.h_rd_n), &format!("Hed=Vb={h_ed:.0} N aus {support_id}, Hrd={:.0}", f.h_rd_n)));
        if h_ed > f.h_rd_n {
            sld = sld.remedy(Remedy::at_least(SubjectRef::new(&f.id, format!("{path}.hRdN"), lc("H_Rd", "H_Rd")), q_force(f.h_rd_n), q_force(h_ed), lc("Increase sliding resistance.", "Gleitwiderstand erhöhen.")));
        }
        report.push(sld.build());
        let xi = part_5::radiation_damping(part_5::stiffness_ratio(f.k_foundation, f.k_soil));
        report.push(
            CheckResult::assess(
                format!("en1998.5.{}.radiationDamping", f.id),
                "DIN EN 1998-5",
                ClauseId::new("EN 1998-5", "5", "5.3"),
                SubjectRef::new(&f.id, format!("{path}.kFoundation"), lc(&f.id, &f.id)),
                lc("Foundation radiation damping ξ", "Abstrahlungsdämpfung ξ der Gründung"),
            )
            .status(crate::document::CheckStatus::Pass)
            .utilization(q_dim(xi), q_dim(1.0))
            .annex(annex)
            .explanation(lc(&format!("ξ={xi:.3} from k_f/k_s",), &format!("ξ={xi:.3} aus k_f/k_s")))
            .build(),
        );
    }
    if document.retaining_walls.is_empty() {
        report.push(CheckResult::assess("en1998.5.wall.na", "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "7.3.2"), SubjectRef::whole(lc("Retaining walls", "Stützwände")), lc("Retaining wall checks", "Stützwandnachweise")).not_applicable(lc("No retaining walls in the subject.", "Keine Stützwände im Gegenstand.")).annex(annex).build());
    }
    for w in &document.retaining_walls {
        let k_h = part_5::horizontal_seismic_coefficient(a_g, s, w.r);
        let k_ae = part_5::mononobe_okabe_k_ae(w.phi_deg, k_h);
        let h_ed = part_5::retaining_wall_thrust_n_per_m(w.soil_gamma, w.height_m, k_ae);
        let path = id_sel("retainingWalls", &w.id);
        let mut c = CheckResult::assess(format!("en1998.5.{}.thrust", w.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "E.2"), SubjectRef::new(&w.id, format!("{path}.hRdNPerM"), lc(&w.id, &w.id)), lc("Retaining wall thrust", "Stützwandschub")).utilization(q_force(h_ed), q_force(w.h_rd_n_per_m)).annex(annex).explanation(lc(
                &format!("Hed={h_ed:.0} N/m, Hrd={:.0} N/m", w.h_rd_n_per_m),
                &format!("H_Ed={h_ed:.0} N/m, H_Rd={:.0} N/m", w.h_rd_n_per_m),
            ));
        if h_ed > w.h_rd_n_per_m {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&w.id, format!("{path}.hRdNPerM"), lc("H_Rd", "H_Rd")), q_force(w.h_rd_n_per_m), q_force(h_ed), lc("Increase wall sliding resistance.", "Wand-Gleitwiderstand erhöhen.")));
        }
        report.push(c.build());
    }
}

fn evaluate_towers(report: &mut CheckReport, document: &En1998Snapshot, annex_params: &AnnexParams, gamma_i: f64, a_g: f64, s: f64, tb: f64, tc: f64, td: f64) {
    let annex = annex_params.choice();
    if document.towers.is_empty() {
        report.push(CheckResult::assess("en1998.6.na", "DIN EN 1998-6", ClauseId::new("EN 1998-6", "6", "4.3"), SubjectRef::whole(lc("Towers", "Türme")), lc("Tower checks", "Turmnachweise")).not_applicable(lc("No towers in the subject.", "Keine Türme im Gegenstand.")).annex(annex).build());
        return;
    }
    for t in &document.towers {
        let q = part_6::tower_behaviour_factor(t.q_nominal, t.is_chimney);
        let s_e = part_1::elastic_response_spectrum(a_g, s, tb, tc, td, 0.5);
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q, a_g);
        let mass = part_1::entity_seismic_mass_kg(t.permanent_gk_n, &t.variables, false, t.correlated_occupancy);
        let v = part_6::tower_base_shear_n(part_6::cantilever_modal_participation_factor(), s_d, mass);
        let m_ed = part_6::tower_overturning_moment_nm(v, t.height_m);
        let path = id_sel("towers", &t.id);
        let mut c = CheckResult::assess(format!("en1998.6.{}.overturning", t.id), "DIN EN 1998-6", ClauseId::new("EN 1998-6", "6", "4.3.2"), SubjectRef::new(&t.id, format!("{path}.mRdNm"), lc(&t.id, &t.id)), lc("Tower overturning moment", "Turm-Kippmoment")).utilization(q_mom(m_ed), q_mom(t.m_rd_nm)).annex(annex).explanation(lc(
                &format!("V={v:.0} N, Med={m_ed:.0} Nm, Mrd={:.0} Nm", t.m_rd_nm),
                &format!("V={v:.0} N, M_Ed={m_ed:.0} Nm, M_Rd={:.0} Nm", t.m_rd_nm),
            ));
        if m_ed > t.m_rd_nm {
            c = c.remedy(Remedy::at_least(SubjectRef::new(&t.id, format!("{path}.mRdNm"), lc("M_Rd", "M_Rd")), q_mom(t.m_rd_nm), q_mom(m_ed), lc("Increase overturning resistance.", "Kippwiderstand erhöhen.")));
        }
        report.push(c.build());
    }
}

pub fn evaluate(document: &En1998Snapshot) -> CheckReport {
    check_full_seismic(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;

pub use super::outline::En1998Outline;
