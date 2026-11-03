//! 🔭️ Investigation extent of the project (GI-01, EN 1997-2 Annex B.3 and 2.4.2) and the cone-factor range of the CPT layers (GI-03, Annex D): the depth below the footing and pile bases, the largest nearest-neighbour spacing of the investigation points against the category limits, the missing-investigation warning and the plausibility of an entered N_kt. See [`super`].

use super::*;
use semio_s_artifact_norm_en1997_model::{Investigation, Pile};

/// 🔖️ Prefix of the GI-01 check ids (`en1997.gi-01.points`, `.spacing`, `.pile.<pile>`).
pub const GI_01: &str = "en1997.gi-01.";

/// 🔖️ Prefix of the GI-03 check ids (`en1997.gi-03.nkt.<layer>`).
pub const GI_03: &str = "en1997.gi-03.";

const PART: &str = "DIN EN 1997-2";

fn extent_clause() -> ClauseId {
    ClauseId::new("EN 1997-2", "2", "2.4.2")
}

fn metres(value: f64) -> Quantity {
    Quantity::length_m(value)
}

/// 📐️ Depth [m] below the ground the investigation must reach for a spread foundation: the embedment plus z_a ≥ max(6 m, 2 B) below the base (Annex B.3).
pub fn footing_required_depth_m(footing: &SpreadFoundation, gk: u8) -> f64 {
    footing.embedment + part_2::min_investigation_depth_for_gk(footing.width.max(footing.length), gk)
}

/// 📐️ Depth [m] below the ground the investigation must reach for a pile: the length plus z_a ≥ max(5 m, 3 D_b) below the base (Annex B.3).
pub fn pile_required_depth_m(pile: &Pile) -> f64 {
    pile.length + (5.0f64).max(3.0 * pile.diameter)
}

/// 📏️ The nearest neighbour `(index, distance [m])` of every investigation point in the plan, `None` for a single point.
pub fn nearest_spacings(points: &[Investigation]) -> Vec<Option<(usize, f64)>> {
    (0..points.len())
        .map(|i| {
            (0..points.len())
                .filter(|&j| j != i)
                .map(|j| (j, (points[i].position_x - points[j].position_x).hypot(points[i].position_y - points[j].position_y)))
                .min_by(|a, b| a.1.total_cmp(&b.1))
        })
        .collect()
}

/// 📏️ The point with the widest gap to its nearest neighbour as `(point, neighbour, distance [m])`: the coverage gap of the site.
pub fn widest_gap(points: &[Investigation]) -> Option<(usize, usize, f64)> {
    nearest_spacings(points).into_iter().enumerate().filter_map(|(i, nearest)| nearest.map(|(j, gap)| (i, j, gap))).max_by(|a, b| a.2.total_cmp(&b.2))
}

fn point_subject(point: &Investigation, leaf: &str) -> SubjectRef {
    subject_entity(&point.id, &format!("investigations[id={}].{leaf}", point.id), &format!("Investigation point {}", point.id), &format!("Erkundungspunkt {}", point.id))
}

fn depth_target(doc: &En1997Snapshot) -> SubjectRef {
    doc.investigations
        .iter()
        .max_by(|a, b| a.depth.total_cmp(&b.depth))
        .map_or_else(|| subject_entity("", "investigationDepth", "Investigation depth", "Erkundungstiefe"), |point| point_subject(point, "depth"))
}

fn push_points(report: &mut CheckReport, doc: &En1997Snapshot) {
    let count = doc.investigations.len() as f64;
    let mut check = CheckResult::assess(
        format!("{GI_01}points"),
        PART,
        extent_clause(),
        subject_entity("", "investigations", "Ground investigation", "Baugrunderkundung"),
        loc("Investigation points recorded", "Erkundungspunkte erfasst"),
    )
    .minimum(Quantity::new(QuantityKind::Dimensionless, count), Quantity::new(QuantityKind::Dimensionless, 1.0))
    .annex(doc.annex);
    check = if doc.investigations.is_empty() {
        check
            .status(CheckStatus::Warning)
            .explanation(loc(
                &format!("No investigation point is recorded; the declared investigation depth {:.1} m is used unverified (GI-01).", doc.investigation_depth),
                &format!("Es ist kein Erkundungspunkt erfasst; die angegebene Erkundungstiefe {:.1} m wird ungeprüft verwendet (GI-01).", doc.investigation_depth),
            ))
            .remedy(Remedy::advisory(
                subject_entity("", "investigations", "Ground investigation", "Baugrunderkundung"),
                loc("Record the boreholes and soundings of the site investigation as investigation points (position and depth).", "Die Bohrungen und Sondierungen der Baugrunderkundung als Erkundungspunkte (Lage und Tiefe) erfassen."),
            ))
    } else {
        check.explanation(loc(&format!("{count:.0} investigation point(s) recorded."), &format!("{count:.0} Erkundungspunkt(e) erfasst.")))
    };
    report.push(check.build());
}

fn push_spacing(report: &mut CheckReport, doc: &En1997Snapshot) {
    let gk = doc.geotechnical_category;
    let (lo, hi) = catalogue::investigation::spacing_limits_m(gk, doc.annex);
    let row = usize::from(gk >= 3);
    let ids = [catalogue::investigation::SPACING_MIN_IDS[row], catalogue::investigation::SPACING_MAX_IDS[row]];
    let title = loc("Investigation point spacing", "Aufschlussabstand");
    let Some((i, j, gap)) = widest_gap(&doc.investigations) else {
        report.push(
            CheckResult::assess(format!("{GI_01}spacing"), PART, extent_clause(), subject_entity("", "investigations", "Ground investigation", "Baugrunderkundung"), title)
                .not_applicable(loc("Fewer than two investigation points: no spacing to assess.", "Weniger als zwei Erkundungspunkte: kein Abstand zu beurteilen."))
                .annex(doc.annex)
                .ndps(&ids)
                .build(),
        );
        return;
    };
    let (point, near) = (&doc.investigations[i], &doc.investigations[j]);
    let closest = nearest_spacings(&doc.investigations).into_iter().flatten().map(|(_, d)| d).fold(f64::INFINITY, f64::min);
    let mut check = CheckResult::assess(format!("{GI_01}spacing"), PART, extent_clause(), point_subject(point, "positionX"), title)
        .utilization(metres(gap), metres(hi))
        .annex(doc.annex)
        .ndps(&ids)
        .explanation(loc(
            &format!("Widest gap {gap:.1} m between {} and {} against the recommended {lo:.0}…{hi:.0} m of GK{gk}; the closest pair is {closest:.1} m apart{}.", point.id, near.id, if closest < lo { " (below the minimum: redundant point, no failure)" } else { "" }),
            &format!("Größter Abstand {gap:.1} m zwischen {} und {} gegenüber empfohlenen {lo:.0}…{hi:.0} m für GK{gk}; das engste Paar liegt {closest:.1} m auseinander{}.", point.id, near.id, if closest < lo { " (unter dem Mindestwert: überzähliger Punkt, kein Versagen)" } else { "" }),
        ));
    if gap > hi {
        let (mx, my) = ((point.position_x + near.position_x) / 2.0, (point.position_y + near.position_y) / 2.0);
        check = check.remedy(Remedy::advisory(
            point_subject(point, "positionX"),
            loc(&format!("Add an investigation point between {} and {} (about x = {mx:.1} m, y = {my:.1} m) so that no gap exceeds {hi:.0} m.", point.id, near.id), &format!("Einen Erkundungspunkt zwischen {} und {} ergänzen (etwa x = {mx:.1} m, y = {my:.1} m), damit kein Abstand {hi:.0} m überschreitet.", point.id, near.id)),
        ));
    }
    report.push(check.build());
}

fn push_pile_depths(report: &mut CheckReport, doc: &En1997Snapshot) {
    let reached = ground_derivation::reached_depth(doc);
    for pile in &doc.piles {
        let required = pile_required_depth_m(pile);
        let subject = subject_entity(&pile.id, &format!("piles[id={}].length", pile.id), &format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id));
        let mut check = CheckResult::assess(format!("{GI_01}pile.{}", pile.id), PART, extent_clause(), subject, loc("Investigation depth below the pile base", "Aufschlusstiefe unter dem Pfahlfuß"))
            .minimum(metres(reached), metres(required))
            .annex(doc.annex)
            .explanation(loc(
                &format!("Reached {reached:.2} m below the ground; pile length {:.2} m + z_a = max(5 m, 3 D_b) = {:.2} m gives {required:.2} m.", pile.length, (5.0f64).max(3.0 * pile.diameter)),
                &format!("Erreicht {reached:.2} m unter Gelände; Pfahllänge {:.2} m + z_a = max(5 m, 3 D_b) = {:.2} m ergibt {required:.2} m.", pile.length, (5.0f64).max(3.0 * pile.diameter)),
            ));
        if reached < required {
            check = check.remedy(Remedy::at_least(
                depth_target(doc),
                metres(reached),
                metres(required),
                loc(&format!("Deepen the investigation from {reached:.2} m to at least {required:.2} m."), &format!("Aufschlusstiefe von {reached:.2} m auf mindestens {required:.2} m erhöhen.")),
            ));
        }
        report.push(check.build());
    }
}

/// ✅️ The document-level GI-01 checks: the recorded points, their spacing and the depth below every pile base; the footing depth sits with the footing row (`en1997.2.4.2.investigation.<footing>`).
pub(super) fn push_investigation_extent(report: &mut CheckReport, doc: &En1997Snapshot) {
    push_points(report, doc);
    push_spacing(report, doc);
    push_pile_depths(report, doc);
}

/// 🔩️ The GI-03 check of an entered cone factor: N_kt of the layer against the plausible range of the registry; a value outside the range is a warning (a project calibration may lie outside), never changed silently. `None` for a layer without an entered value.
pub fn cone_factor_check(layer: &SoilLayer, annex: AnnexChoice) -> Option<CheckResult> {
    if layer.cpt_nkt <= 0.0 {
        return None;
    }
    let (lo, hi) = catalogue::investigation::cpt_nkt_range(annex);
    let subject = subject_entity(&layer.id, &format!("layers[id={}].cptNkt", layer.id), &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id));
    let value = Quantity::new(QuantityKind::Dimensionless, layer.cpt_nkt);
    let mut check = CheckResult::assess(format!("{GI_03}nkt.{}", layer.id), "DIN EN 1997-2", ClauseId::new("EN 1997-2", "D", "D"), subject.clone(), loc("Cone factor N_kt of the layer", "Kegelfaktor N_kt der Schicht"))
        .informative(value)
        .annex(annex)
        .ndps(&catalogue::investigation::CPT_NKT_RANGE_IDS)
        .explanation(loc(
            &format!("N_kt = {:.1} of layer {} against the plausible range {lo:.0}…{hi:.0} of c_u = (q_t − σ_v0)/N_kt.", layer.cpt_nkt, layer.id),
            &format!("N_kt = {:.1} der Schicht {} gegenüber dem plausiblen Bereich {lo:.0}…{hi:.0} von c_u = (q_t − σ_v0)/N_kt.", layer.cpt_nkt, layer.id),
        ));
    if layer.cpt_nkt < lo || layer.cpt_nkt > hi {
        let nearest = layer.cpt_nkt.clamp(lo, hi);
        check = check.status(CheckStatus::Warning).remedy(Remedy::exactly(
            subject,
            value,
            Quantity::new(QuantityKind::Dimensionless, nearest),
            loc(&format!("Set N_kt to {nearest:.1} (end of the plausible range) or document the project calibration outside {lo:.0}…{hi:.0}."), &format!("N_kt auf {nearest:.1} setzen (Ende des plausiblen Bereichs) oder die projektbezogene Kalibrierung außerhalb {lo:.0}…{hi:.0} belegen.")),
        ));
    }
    Some(check.build())
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
