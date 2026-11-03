//! 🧱️ Checks of the row collections added by the new-collection template: LM1 per bridge lane (EN 1991-2, 4.2.3 and Table 4.2), the reduced return periods of the construction stages
//! (EN 1991-1-6, 3.3 and Table 3.1, unverified) and the roof shape coefficients per roof surface (EN 1991-1-3, 5.3 and EN 1991-1-4, 7.2.5 to 7.2.8).

use super::*;
use semio_s_artifact_norm_en1991_model::{BridgeLane, ConstructionStage, RoofSurface, StructureKind, WindFace};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};

const LANE_WIDTH_MAX_M: f64 = 3.0;
const EXTENT_TOLERANCE_M: f64 = 0.05;
const NDP_STAGE_RETURN: &str = "ndp.en1991.e6-01.1";
const ROOF_SHAPES: [&str; 4] = ["flat", "monopitch", "duopitch", "hipped"];

fn length(metres: f64) -> Quantity { Quantity::new(QuantityKind::Length, metres) }
fn number(value: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, value) }
fn is_roof_zone(face: &WindFace) -> bool { part_1_4::is_hipped_zone(&face.zone) }

/// 🧱️ The row checks of the three collections, in the order of the units of [`En1991Units`].
pub(super) fn check_collection_rows(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, lane) in document.bridge_lanes.iter().enumerate().filter(|(index, _)| scope.admits("bridgeLanes", *index)) {
        check_lane(document, annex, report, lane);
    }
    for (_, stage) in document.construction_stages.iter().enumerate().filter(|(index, _)| scope.admits("constructionStages", *index)) {
        check_stage(annex, report, stage);
    }
    for (_, hoist) in document.crane_hoists.iter().enumerate().filter(|(index, _)| scope.admits("craneHoists", *index)) {
        super::crane_silo_checks::check_crane_hoist(hoist, annex, report);
    }
    for (_, bridge) in document.footbridges.iter().enumerate().filter(|(index, _)| scope.admits("footbridges", *index)) {
        super::bridge_effects_checks::check_footbridge(bridge, annex, report);
    }
    for (_, span) in document.rail_dynamics.iter().enumerate().filter(|(index, _)| scope.admits("railDynamics", *index)) {
        super::bridge_effects_checks::check_rail_dynamic(span, annex, report);
    }
    for (_, row) in document.load_dispersals.iter().enumerate().filter(|(index, _)| scope.admits("loadDispersals", *index)) {
        super::bridge_effects_checks::check_load_dispersal(row, annex, report);
    }
    for (_, solid) in document.bulk_solids.iter().enumerate().filter(|(index, _)| scope.admits("bulkSolids", *index)) {
        super::crane_silo_checks::check_bulk_solid(solid, annex, report);
    }
    for (_, machine) in document.rotating_machines.iter().enumerate().filter(|(index, _)| scope.admits("rotatingMachines", *index)) {
        super::crane_silo_checks::check_rotating_machine(machine, annex, report);
    }
    for (index, surface) in document.roof_surfaces.iter().enumerate().filter(|(index, _)| scope.admits("roofSurfaces", *index)) {
        check_roof_surface(document, annex, report, surface);
        check_roof_drift(document, annex, report, index);
    }
}

/// 🛣️ LM1 of one bridge lane: the lane number is known, the lane is at most 3 m wide and inside the carriageway, and the assumed tandem and uniformly distributed load cover the Q_ik and q_ik of its number.
fn check_lane(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, lane: &BridgeLane) {
    let base = format!("bridgeLanes[id={}]", lane.id);
    let subject = |leaf: &str, en: &str, de: &str| SubjectRef::new(&lane.id, &format!("{base}.{leaf}"), copy(en, de));
    if document.structure_kind != StructureKind::Bridge {
        report.push(CheckResult::assess(format!("en1991.2.lane.{}.none", lane.id), "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.2.3", "4.2.3"), subject("id", &format!("Lane {}", lane.id), &format!("Fahrstreifen {}", lane.id)), copy("Bridge lane", "Brückenfahrstreifen")).not_applicable(copy("Structure kind is building — bridge lanes not applicable.", "Tragwerksart ist Gebäude — Brückenfahrstreifen nicht anwendbar.")).annex(annex).build());
        return;
    }
    if lane.lm1_lane == 0 {
        invalid(report, annex, format!("en1991.input.bridgeLanes.{}.lm1Lane", lane.id), "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.2", "4.2.3"), &lane.id, format!("{base}.lm1Lane"), ("LM1 lane number", "LM1-Fahrstreifennummer"), "0", vec!["1".into(), "2".into(), "3".into()]);
        return;
    }
    let width_remedy = (lane.width > LANE_WIDTH_MAX_M + 1e-9).then(|| Remedy::exactly(subject("width", "Lane width", "Fahrstreifenbreite"), length(lane.width), length(LANE_WIDTH_MAX_M), copy(&format!("Set the lane width to {LANE_WIDTH_MAX_M:.1} m, the notional lane width of Table 4.1."), &format!("Fahrstreifenbreite auf {LANE_WIDTH_MAX_M:.1} m setzen, die Breite des ideellen Fahrstreifens nach Tabelle 4.1."))));
    report.push(assess_covers(&format!("en1991.2.lane-width.{}", lane.id), "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.1", "4.2.3"), subject("width", &format!("Lane {}", lane.id), &format!("Fahrstreifen {}", lane.id)), copy("Lane width against the notional lane width", "Fahrstreifenbreite gegen die Breite des ideellen Fahrstreifens"), length(LANE_WIDTH_MAX_M), length(lane.width), copy(&format!("Lane width {:.2} m against at most {LANE_WIDTH_MAX_M:.1} m.", lane.width), &format!("Fahrstreifenbreite {:.2} m gegen höchstens {LANE_WIDTH_MAX_M:.1} m.", lane.width)), annex, width_remedy));
    let half_carriageway = document.bridge_lane_width.max(0.0) / 2.0;
    let reach = lane.offset.abs() + lane.width / 2.0;
    let inside_remedy = (reach > half_carriageway + 1e-9).then(|| {
        let target = (half_carriageway - lane.width / 2.0).max(0.0).copysign(lane.offset);
        Remedy::exactly(subject("offset", "Lane offset", "Fahrstreifenversatz"), length(lane.offset), length(target), copy(&format!("Move the lane axis to {target:.2} m so that the lane lies inside the carriageway of {:.2} m.", document.bridge_lane_width), &format!("Fahrstreifenachse auf {target:.2} m verschieben, damit der Streifen in der Fahrbahn von {:.2} m liegt.", document.bridge_lane_width)))
    });
    report.push(assess_covers(&format!("en1991.2.lane-inside.{}", lane.id), "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.2.3", "4.2.3"), subject("offset", &format!("Lane {}", lane.id), &format!("Fahrstreifen {}", lane.id)), copy("Lane lies inside the carriageway", "Fahrstreifen liegt in der Fahrbahn"), length(half_carriageway), length(reach), copy(&format!("Lane reaches {reach:.2} m from the carriageway axis; the carriageway extends to {half_carriageway:.2} m."), &format!("Der Fahrstreifen reicht {reach:.2} m von der Fahrbahnachse; die Fahrbahn reicht bis {half_carriageway:.2} m.")), annex, inside_remedy));
    let (required_q, required_u) = (part_2::lm1_tandem_n(annex, lane.lm1_lane), part_2::lm1_udl_pa(annex, lane.lm1_lane));
    let tandem_remedy = (document.assumed_bridge_tandem + 1e-9 < required_q).then(|| raise_force_remedy("assumedBridgeTandem", "", "LM1 tandem axle load", "LM1-Tandemachslast", document.assumed_bridge_tandem, required_q));
    report.push(assess_covers(&format!("en1991.2.lm1-lane.{}", lane.id), "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.2", "4.3.2"), subject("lm1Lane", &format!("Lane {}", lane.id), &format!("Fahrstreifen {}", lane.id)), copy("LM1 tandem axle load of the lane number", "LM1-Tandemachslast der Fahrstreifennummer"), force(document.assumed_bridge_tandem), force(required_q), copy(&format!("Lane number {}: Q_ik = {:.0} kN required; assumed {:.0} kN.", lane.lm1_lane, required_q / 1000.0, document.assumed_bridge_tandem / 1000.0), &format!("Fahrstreifennummer {}: Q_ik = {:.0} kN erforderlich; angesetzt {:.0} kN.", lane.lm1_lane, required_q / 1000.0, document.assumed_bridge_tandem / 1000.0)), annex, tandem_remedy));
    let udl_remedy = (document.assumed_bridge_udl + 1e-9 < required_u).then(|| raise_pressure_remedy("assumedBridgeUdl", "", "LM1 uniformly distributed load", "LM1-Gleichlast", document.assumed_bridge_udl, required_u));
    report.push(assess_covers(&format!("en1991.2.lm1-lane-udl.{}", lane.id), "DIN EN 1991-2", ClauseId::new("EN 1991-2", "Table 4.2", "4.3.2"), subject("lm1Lane", &format!("Lane {}", lane.id), &format!("Fahrstreifen {}", lane.id)), copy("LM1 uniformly distributed load of the lane number", "LM1-Gleichlast der Fahrstreifennummer"), pressure(document.assumed_bridge_udl), pressure(required_u), copy(&format!("Lane number {}: q_ik = {:.2} kN/m² required; assumed {:.2} kN/m².", lane.lm1_lane, required_u / 1000.0, document.assumed_bridge_udl / 1000.0), &format!("Fahrstreifennummer {}: q_ik = {:.2} kN/m² erforderlich; angesetzt {:.2} kN/m².", lane.lm1_lane, required_u / 1000.0, document.assumed_bridge_udl / 1000.0)), annex, udl_remedy));
}

/// 🏗️ One construction stage: the activity load, the return period of the climatic actions for the duration (EN 1991-1-6 Table 3.1, unverified) and the reduced wind pressure it implies.
fn check_stage(annex: AnnexChoice, report: &mut CheckReport, stage: &ConstructionStage) {
    let base = format!("constructionStages[id={}]", stage.id);
    let subject = |leaf: &str| SubjectRef::new(&stage.id, &format!("{base}.{leaf}"), copy(&format!("Stage {}", stage.id), &format!("Bauzustand {}", stage.id)));
    let clause = |locator: &str| ClauseId::new("EN 1991-1-6", locator, "3");
    let Some(required_load) = part_1_6::construction_qk_pa(&stage.activity) else {
        invalid(report, annex, format!("en1991.input.constructionStages.{}.activity", stage.id), "DIN EN 1991-1-6", ClauseId::new("EN 1991-1-6", "§4.11", "4.11"), &stage.id, format!("{base}.activity"), ("construction activity", "Bautätigkeit"), &stage.activity, part_1_6::ACTIVITIES.iter().map(|a| a.to_string()).collect());
        return;
    };
    if stage.duration_days <= 0.0 {
        invalid(report, annex, format!("en1991.input.constructionStages.{}.durationDays", stage.id), "DIN EN 1991-1-6", clause("§3.1"), &stage.id, format!("{base}.durationDays"), ("stage duration in days (greater than zero)", "Dauer des Bauzustands in Tagen (größer null)"), &format!("{}", stage.duration_days), vec!["3".into(), "90".into(), "365".into()]);
        return;
    }
    let load_remedy = (stage.assumed_qk + 1e-9 < required_load).then(|| raise_pressure_remedy(&format!("{base}.assumedQk"), &stage.id, "stage load q_ca", "Bauzustandslast q_ca", stage.assumed_qk, required_load));
    report.push(assess_covers(&format!("en1991.1-6.stage-load.{}", stage.id), "DIN EN 1991-1-6", ClauseId::new("EN 1991-1-6", "§4.11 / Table 4.1", "4.11"), subject("assumedQk"), copy("Construction load of the stage", "Bauzustandslast des Bauzustands"), pressure(stage.assumed_qk), pressure(required_load), copy(&format!("Required q_ca = {:.2} kN/m² for {}; assumed {:.2} kN/m².", required_load / 1000.0, stage.activity, stage.assumed_qk / 1000.0), &format!("Erforderliches q_ca = {:.2} kN/m² für {}; angesetzt {:.2} kN/m².", required_load / 1000.0, stage.activity, stage.assumed_qk / 1000.0)), annex, load_remedy));
    let return_ndp = part_1_6::stage_return_period_ndp_id(stage.duration_days);
    if let Some(required_years) = part_1_6::stage_return_period_years(annex, stage.duration_days) {
        let years_remedy = (stage.return_years + 1e-9 < required_years).then(|| Remedy::exactly(subject("returnYears"), number(stage.return_years), number(required_years), copy(&format!("Use the return period {required_years:.0} years for a stage of {:.0} days (Table 3.1, unverified).", stage.duration_days), &format!("Wiederkehrperiode {required_years:.0} Jahre für einen Bauzustand von {:.0} Tagen ansetzen (Tabelle 3.1, ungeprüft).", stage.duration_days))));
        report.push(assess_covers_on(&format!("en1991.1-6.stage-return-period.{}", stage.id), "DIN EN 1991-1-6", clause("Table 3.1"), subject("returnYears"), copy("Return period of the climatic actions", "Wiederkehrperiode der klimatischen Einwirkungen"), number(stage.return_years), number(required_years), copy(&format!("Stage of {:.0} days requires T ≥ {required_years:.0} years; assumed {:.1} years.", stage.duration_days, stage.return_years), &format!("Bauzustand von {:.0} Tagen erfordert T ≥ {required_years:.0} Jahre; angesetzt {:.1} Jahre.", stage.duration_days, stage.return_years)), annex, years_remedy, &[return_ndp]));
    } else {
        report.push(CheckResult::assess(format!("en1991.1-6.stage-return-period.{}", stage.id), "DIN EN 1991-1-6", clause("Table 3.1"), subject("returnYears"), copy("Return period of the climatic actions", "Wiederkehrperiode der klimatischen Einwirkungen"))
            .annex(annex)
            .status(CheckStatus::UnverifiedBasis)
            .explanation(copy(&format!("Awaiting values: the return period for a stage of {:.0} days is read from the national annex table 3.1 of EN 1991-1-6, which is not keyed; the registry row {return_ndp} holds no value for this annex. Assumed {:.1} years, not compared.", stage.duration_days, stage.return_years), &format!("Werte ausstehend: die Wiederkehrperiode für einen Bauzustand von {:.0} Tagen stammt aus der Tabelle 3.1 des nationalen Anhangs zu DIN EN 1991-1-6, die nicht erfasst ist; die Registerzeile {return_ndp} enthält für diesen Anhang keinen Wert. Angesetzt {:.1} Jahre, nicht verglichen.", stage.duration_days, stage.return_years)))
            .ndp(return_ndp)
            .ndp_reads()
            .build());
    }
    let ratio = part_1_6::wind_pressure_ratio(stage.return_years);
    report.push(CheckResult::assess(format!("en1991.1-6.stage-wind.{}", stage.id), "DIN EN 1991-1-6", clause("§3.3"), subject("returnYears"), copy("Reduced wind pressure of the stage", "Reduzierter Winddruck des Bauzustands"))
        .minimum(number(ratio), number(semio_s_artifact_norm_en1991_checks::catalogue::info_floor(ratio)))
        .annex(annex)
        .explanation(copy(&format!("q_p(T = {:.1} a) / q_p(50 a) = c_prob² = {ratio:.3} (EN 1991-1-4 Eq. 4.2, K = 0.2, n = 0.5).", stage.return_years), &format!("q_p(T = {:.1} a) / q_p(50 a) = c_prob² = {ratio:.3} (EN 1991-1-4 Gl. 4.2, K = 0,2, n = 0,5).", stage.return_years)))
        .ndp(NDP_STAGE_RETURN)
        .ndp_reads()
        .build());
}

/// 🏠 One roof surface: shape and extent are admissible, the pressure coefficient of its wind face matches the shape table for its pitch, and the snow shape coefficient it implies is reported.
fn check_roof_surface(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, surface: &RoofSurface) {
    let base = format!("roofSurfaces[id={}]", surface.id);
    let subject = |leaf: &str| SubjectRef::new(&surface.id, &format!("{base}.{leaf}"), copy(&format!("Roof surface {}", surface.id), &format!("Dachfläche {}", surface.id)));
    let Some(face) = document.wind_faces.iter().find(|f| f.id == surface.face) else { return };
    let shape = surface.shape.trim().to_ascii_lowercase();
    if !ROOF_SHAPES.contains(&shape.as_str()) {
        invalid(report, annex, format!("en1991.input.roofSurfaces.{}.shape", surface.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Tables 7.3 to 7.5", "7.2"), &surface.id, format!("{base}.shape"), ("roof shape", "Dachform"), &surface.shape, ROOF_SHAPES.iter().map(|s| s.to_string()).collect());
        return;
    }
    if !is_roof_zone(face) {
        invalid(report, annex, format!("en1991.input.roofSurfaces.{}.face", surface.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Tables 7.3 to 7.5", "7.2"), &surface.id, format!("{base}.face"), ("wind face with a roof zone F to J (hipped roofs F to N)", "Windfläche mit Dachzone F bis J (Walmdach F bis N)"), &surface.face, document.wind_faces.iter().filter(|f| is_roof_zone(f)).map(|f| f.id.clone()).collect());
        return;
    }
    let extent_remedy = (surface.extent > document.depth + EXTENT_TOLERANCE_M).then(|| Remedy::exactly(subject("extent"), length(surface.extent), length(document.depth), copy(&format!("Reduce the extent to the building depth {:.2} m.", document.depth), &format!("Ausdehnung auf die Gebäudetiefe {:.2} m verringern.", document.depth))));
    report.push(assess_covers(&format!("en1991.1-4.roof-surface-extent.{}", surface.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "§7.2.5", "7.2"), subject("extent"), copy("Roof extent inside the building depth", "Dachausdehnung innerhalb der Gebäudetiefe"), length(document.depth), length(surface.extent), copy(&format!("Extent {:.2} m against the building depth {:.2} m.", surface.extent, document.depth), &format!("Ausdehnung {:.2} m gegen die Gebäudetiefe {:.2} m.", surface.extent, document.depth)), annex, extent_remedy));
    if shape == "hipped" && part_1_4::is_hipped_zone(&face.zone) {
        report.push(super::hipped_checks::surface_cpe(annex, surface, face));
        push_roof_surface_mu(annex, report, surface, &shape, subject("pitchDeg"));
        return;
    }
    let roof = document.roofs.iter().find(|r| face.id.starts_with(&r.id)).or(document.roofs.first());
    let hp_over_h = roof.filter(|r| r.has_parapet).map_or(0.0, |r| r.parapet_height / document.height.max(0.1));
    let h_over_d = document.height / document.depth.max(0.1);
    let Some(candidates) = part_1_4::face_cpe_candidates(&face.zone, h_over_d, Some(shape.as_str()), surface.pitch_deg, hp_over_h) else {
        invalid(report, annex, format!("en1991.input.roofSurfaces.{}.pitchDeg", surface.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Tables 7.3 to 7.5", "7.2"), &surface.id, format!("{base}.pitchDeg"), ("pitch with a tabulated coefficient for the zone", "Neigung mit tabelliertem Beiwert für die Zone"), &format!("{}", surface.pitch_deg), vec!["0".into(), "5".into(), "15".into(), "30".into(), "45".into()]);
        return;
    };
    let nearest = candidates.iter().map(|p| p.0).min_by(|a, b| (a - face.c_pe10).abs().partial_cmp(&(b - face.c_pe10).abs()).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(face.c_pe10);
    let deviation = (face.c_pe10 - nearest).abs();
    let face_subject = SubjectRef::new(&face.id, &format!("windFaces[id={}].cPe10", face.id), copy(&format!("Face {}", face.id), &format!("Fläche {}", face.id)));
    let mut cpe = CheckResult::assess(format!("en1991.1-4.roof-surface-cpe.{}", surface.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Tables 7.3 to 7.5", "7.2"), subject("pitchDeg"), copy("Pressure coefficient of the roof shape", "Druckbeiwert der Dachform"))
        .utilization(number(deviation), number(CPE_TOLERANCE))
        .warn_near_limit()
        .annex(annex)
        .explanation(copy(&format!("{} roof of {:.1}° in zone {}: tabulated c_pe,10 {nearest:.3}, face {} has {:.3} (deviation {deviation:.3}).", shape, surface.pitch_deg, face.zone, face.id, face.c_pe10), &format!("Dach {} mit {:.1}° in Zone {}: Tabellenwert c_pe,10 {nearest:.3}, Fläche {} hat {:.3} (Abweichung {deviation:.3}).", shape, surface.pitch_deg, face.zone, face.id, face.c_pe10)));
    if deviation > CPE_TOLERANCE {
        cpe = cpe.remedy(Remedy::exactly(face_subject, number(face.c_pe10), number(nearest), copy(&format!("Set c_pe,10 of {} to {nearest:.3} for the {shape} roof surface {} (one wind face per differing pitch).", face.id, surface.id), &format!("c_pe,10 von {} auf {nearest:.3} für die Dachfläche {} ({shape}) setzen (je abweichender Neigung eine Windfläche).", face.id, surface.id))));
    }
    report.push(cpe.ndp_reads().build());
    push_roof_surface_mu(annex, report, surface, &shape, subject("pitchDeg"));
}

/// ❄️ The snow shape coefficient of a roof surface for its shape and pitch (EN 1991-1-3, 5.3 / Table 5.2).
fn push_roof_surface_mu(annex: AnnexChoice, report: &mut CheckReport, surface: &RoofSurface, shape: &str, subject: SubjectRef) {
    let mu = part_1_3::shape_mu(shape, surface.pitch_deg, false, 0.0, 0.0, false, 0.0, annex);
    report.push(CheckResult::assess(format!("en1991.1-3.roof-surface-mu.{}", surface.id), "DIN EN 1991-1-3", ClauseId::new("EN 1991-1-3", "§5.3 / Table 5.2", "5.3"), subject, copy("Snow shape coefficient of the roof surface", "Schneelast-Formbeiwert der Dachfläche"))
        .minimum(number(mu), number(semio_s_artifact_norm_en1991_checks::catalogue::info_floor(mu)))
        .annex(annex)
        .explanation(copy(&format!("μ₁({:.1}°) = {mu:.3} for the {shape} roof surface {}.", surface.pitch_deg, surface.id), &format!("μ₁({:.1}°) = {mu:.3} für die Dachfläche {} ({shape}).", surface.pitch_deg, surface.id)))
        .ndp_reads()
        .build());
}

/// 🪜 Snow drift of a roof surface (EN 1991-1-3, 5.3.6 and 6.2): on the lower successor of the same wind face behind the step it leaves (sliding μ_s plus wind-induced μ_w over the drift length l_s = 2h) and behind the parapet at the end of the face.
fn check_roof_drift(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, index: usize) {
    let surface = &document.roof_surfaces[index];
    let Some(sk) = part_1_3::design_ground_snow_pa(annex, &document.snow_zone, document.altitude, document.en_sk) else { return };
    let clause = ClauseId::new("EN 1991-1-3", "§5.3.6", "5.3.6");
    let subject = |id: &str, leaf: &str| SubjectRef::new(id, &format!("roofSurfaces[id={id}].{leaf}"), copy(&format!("Roof surface {id}"), &format!("Dachfläche {id}")));
    let shape = surface.shape.trim().to_ascii_lowercase();
    let height = part_1_3::roof_step_height_m(&shape, surface.pitch_deg, surface.extent);
    if let Some(next) = document.roof_surfaces[index + 1..].iter().find(|o| o.face == surface.face).filter(|_| height > 1e-6) {
        let (mu_s, mu_w) = (part_1_3::sliding_mu_s(surface.pitch_deg), part_1_3::drift_mu_w(height, surface.extent, next.extent, sk, annex));
        let drift_length = (2.0 * height).clamp(5.0, 15.0);
        let mu = (mu_s + mu_w).max(part_1_3::mu_1(next.pitch_deg));
        let reads = semio_s_plugin_norm_compliance::drain_ndp_reads();
        report.push(CheckResult::assess(format!("en1991.1-3.roof-step-drift.{}", next.id), "DIN EN 1991-1-3", clause.clone(), subject(&next.id, "pitchDeg"), copy("Snow drift behind the roof step", "Schneeverwehung hinter dem Dachsprung"))
            .minimum(number(mu), number(semio_s_artifact_norm_en1991_checks::catalogue::info_floor(mu)))
            .annex(annex)
            .explanation(copy(&format!("Step of h = {height:.2} m from roof surface {} to {} (b₁ = {:.2} m, b₂ = {:.2} m): μ_s = {mu_s:.3}, μ_w = {mu_w:.3}, μ = {mu:.3} over l_s = {drift_length:.1} m for s_k = {:.3} kN/m².", surface.id, next.id, surface.extent, next.extent, sk / 1000.0), &format!("Sprung von h = {height:.2} m von Dachfläche {} zu {} (b₁ = {:.2} m, b₂ = {:.2} m): μ_s = {mu_s:.3}, μ_w = {mu_w:.3}, μ = {mu:.3} über l_s = {drift_length:.1} m für s_k = {:.3} kN/m².", surface.id, next.id, surface.extent, next.extent, sk / 1000.0)))
            .ndps(&reads)
            .build());
    }
    let last_of_face = document.roof_surfaces[index + 1..].iter().all(|o| o.face != surface.face);
    let roof = document.roofs.iter().find(|r| surface.face.starts_with(&r.id)).or(document.roofs.first());
    if let Some(roof) = roof.filter(|r| last_of_face && r.has_parapet && r.parapet_height > 0.0) {
        let (raw, mu, drift_length) = part_1_3::obstruction_drift(roof.parapet_height, sk, annex);
        let reads = semio_s_plugin_norm_compliance::drain_ndp_reads();
        report.push(CheckResult::assess(format!("en1991.1-3.roof-parapet-drift.{}", surface.id), "DIN EN 1991-1-3", ClauseId::new("EN 1991-1-3", "§6.2 / Annex B", "6.2"), subject(&surface.id, "extent"), copy("Snow drift behind the parapet", "Schneeverwehung hinter der Attika"))
            .minimum(number(mu), number(semio_s_artifact_norm_en1991_checks::catalogue::info_floor(mu)))
            .annex(annex)
            .explanation(copy(&format!("Parapet of h = {:.2} m behind roof surface {}: γh/s_k = {raw:.3}, μ = {mu:.3} over l_s = {drift_length:.1} m.", roof.parapet_height, surface.id), &format!("Attika von h = {:.2} m hinter Dachfläche {}: γh/s_k = {raw:.3}, μ = {mu:.3} über l_s = {drift_length:.1} m.", roof.parapet_height, surface.id)))
            .ndps(&reads)
            .build());
    }
}

/// 🔗 Document-wide consistency of the three collections: lane numbers and overlaps, lane widths against the carriageway, dangling wind faces of roof surfaces and the extents per face.
pub(super) fn push_collection_integrity(report: &mut CheckReport, document: &En1991Snapshot) {
    let fail = |id: String, subject: SubjectRef, title: LocalizedCopy, explanation: LocalizedCopy, remedy: Remedy| {
        CheckResult::assess(id, "EN 1991 integrity", ClauseId::new("EN 1991", "§2", "collections"), subject, title).annex(document.annex).explanation(explanation).status(CheckStatus::Fail).remedy(remedy).ndp_reads().build()
    };
    let lanes = &document.bridge_lanes;
    for (index, lane) in lanes.iter().enumerate() {
        if lanes[..index].iter().any(|other| other.lm1_lane == lane.lm1_lane) {
            let free: Vec<String> = (1..=lanes.len() as u32).filter(|n| lanes.iter().all(|l| u32::from(l.lm1_lane) != *n)).map(|n| n.to_string()).collect();
            let subject = SubjectRef::new(&lane.id, &format!("bridgeLanes[id={}].lm1Lane", lane.id), copy("LM1 lane number", "LM1-Fahrstreifennummer"));
            report.push(fail(format!("en1991.integrity.bridgeLanes.number.{}", lane.id), subject.clone(), copy("Unique LM1 lane number", "Eindeutige LM1-Fahrstreifennummer"), copy(&format!("Lane number {} is used twice; each lane carries its own Table 4.2 loads.", lane.lm1_lane), &format!("Fahrstreifennummer {} ist doppelt vergeben; jeder Streifen trägt seine eigenen Lasten nach Tabelle 4.2.", lane.lm1_lane)), Remedy::one_of(subject, if free.is_empty() { vec![(lanes.len() + 1).to_string()] } else { free }, copy("Give the lane a free LM1 lane number.", "Dem Streifen eine freie LM1-Fahrstreifennummer geben."))));
        }
        if let Some(other) = lanes[..index].iter().find(|o| (o.offset - lane.offset).abs() + 1e-9 < (o.width + lane.width) / 2.0) {
            let target = other.offset + (other.width + lane.width) / 2.0;
            let subject = SubjectRef::new(&lane.id, &format!("bridgeLanes[id={}].offset", lane.id), copy("Lane offset", "Fahrstreifenversatz"));
            report.push(fail(format!("en1991.integrity.bridgeLanes.overlap.{}", lane.id), subject.clone(), copy("Lanes do not overlap", "Fahrstreifen überlappen nicht"), copy(&format!("Lane {} overlaps lane {} (axes {:.2} m and {:.2} m apart less than the half-width sum).", lane.id, other.id, lane.offset, other.offset), &format!("Fahrstreifen {} überlappt Fahrstreifen {} (Achsen {:.2} m und {:.2} m, Abstand kleiner als die halbe Breitensumme).", lane.id, other.id, lane.offset, other.offset)), Remedy::exactly(subject, length(lane.offset), length(target), copy(&format!("Move lane {} to the offset {target:.2} m next to lane {}.", lane.id, other.id), &format!("Fahrstreifen {} auf den Versatz {target:.2} m neben Fahrstreifen {} verschieben.", lane.id, other.id)))));
        }
    }
    let total: f64 = lanes.iter().map(|l| l.width).sum();
    if document.structure_kind == StructureKind::Bridge && !lanes.is_empty() && total > document.bridge_lane_width + EXTENT_TOLERANCE_M {
        let subject = SubjectRef::new("", "bridgeLaneWidth", copy("Carriageway width", "Fahrbahnbreite"));
        report.push(fail("en1991.integrity.bridgeLanes.width-sum".into(), subject.clone(), copy("Lanes fit the carriageway", "Fahrstreifen passen in die Fahrbahn"), copy(&format!("The lane widths add up to {total:.2} m, the carriageway is {:.2} m wide.", document.bridge_lane_width), &format!("Die Fahrstreifenbreiten ergeben {total:.2} m, die Fahrbahn ist {:.2} m breit.", document.bridge_lane_width)), Remedy::exactly(subject, length(document.bridge_lane_width), length(total), copy(&format!("Widen the carriageway to {total:.2} m or narrow the lanes."), &format!("Fahrbahn auf {total:.2} m verbreitern oder Fahrstreifen verschmälern.")))));
    }
    for surface in &document.roof_surfaces {
        if document.wind_faces.iter().all(|f| f.id != surface.face) {
            push_dangling_ref(report, document, format!("en1991.integrity.roofSurfaces.{}.face", surface.id), format!("roofSurfaces[id={}].face", surface.id), &surface.id, "Roof surface face", "Dachflächen-Seite", &surface.face, document.wind_faces.iter().map(|f| f.id.clone()).collect());
        }
    }
    let mut faces: Vec<&str> = document.roof_surfaces.iter().map(|s| s.face.as_str()).collect();
    faces.sort_unstable();
    faces.dedup();
    for face in faces {
        let rows: Vec<&RoofSurface> = document.roof_surfaces.iter().filter(|s| s.face == face).collect();
        let sum: f64 = rows.iter().map(|s| s.extent).sum();
        if document.depth > 0.0 && sum > document.depth + EXTENT_TOLERANCE_M {
            let last = rows[rows.len() - 1];
            let target = (last.extent - (sum - document.depth)).max(0.0);
            let subject = SubjectRef::new(&last.id, &format!("roofSurfaces[id={}].extent", last.id), copy("Roof extent", "Dachausdehnung"));
            report.push(fail(format!("en1991.integrity.roofSurfaces.extent.{face}"), subject.clone(), copy("Roof surfaces of a face fit the building depth", "Dachflächen einer Seite passen in die Gebäudetiefe"), copy(&format!("The roof surfaces of face {face} add up to {sum:.2} m, the building is {:.2} m deep.", document.depth), &format!("Die Dachflächen der Seite {face} ergeben {sum:.2} m, das Gebäude ist {:.2} m tief.", document.depth)), Remedy::exactly(subject, length(last.extent), length(target), copy(&format!("Shorten {} to {target:.2} m.", last.id), &format!("{} auf {target:.2} m kürzen.", last.id)))));
        }
    }
}
