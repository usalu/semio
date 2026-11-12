//! 🥞️ Diaphragm diagrams: the wall force distribution of every classified plan direction (rigid beside flexible assumption) and the classification ratio over the diaphragm stiffness with the rigid limit; the grip and the plate live with the scene.

use kit::{diaphragm_walls, wall_ref};
use layout::EPS;
use model::{En1996Snapshot, MasonryWall};
use norm::CheckReport;
use scene::diagram_kit::curves;
use scene::projection_kit::diagram_parts::{axis, column, stroke};
use scene::projection_kit::{base_id, loc};
use scene::*;
use schema::checks::masonry as m;

const RATIO_SAMPLES: usize = 25;
const STIFFNESS_LADDER: [f64; 28] = [1e3, 2e3, 5e3, 1e4, 2e4, 5e4, 1e5, 2e5, 5e5, 1e6, 2e6, 5e6, 1e7, 2e7, 5e7, 1e8, 2e8, 5e8, 1e9, 2e9, 5e9, 1e10, 2e10, 5e10, 1e11, 2e11, 5e11, 1e12];

fn tone_of(class: m::DiaphragmClass) -> Tone {
    match class {
        m::DiaphragmClass::Rigid => Tone::Info,
        m::DiaphragmClass::Flexible => Tone::Warning,
    }
}

fn bar(id: String, label: LocalizedCopy, x: (f64, f64), kn: f64, active: bool, tone: Tone) -> Series {
    let ring = vec![Vec2::new(x.0, 0.0), Vec2::new(x.1, 0.0), Vec2::new(x.1, kn), Vec2::new(x.0, kn)];
    Series::Polygon { id, label, ring, fill: Fill { tone, opacity: if active { 0.7 } else { 0.25 } }, hatch: if active { None } else { Some(Hatch::Diagonal) } }
}

fn distribution(d: &m::Diaphragm, wall: &MasonryWall, check: Option<String>) -> Diagram2d {
    let axis_key = d.axis.key();
    let span = (d.span_m.1 - d.span_m.0).max(EPS);
    let gap = d.walls.windows(2).map(|w| (w[1].s_m - w[0].s_m).abs()).filter(|g| *g > EPS).fold(span, f64::min);
    let half = (0.15 * gap).min(0.04 * span).max(0.02 * span.min(1.0));
    let kn = |n: f64| n / 1000.0;
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let (mut top, mut table_s) = (0.0_f64, Vec::new());
    for w in &d.walls {
        top = top.max(kn(w.rigid_n)).max(kn(w.tributary_n));
        table_s.push(w.s_m);
        series.push(bar(format!("rigid-{}", w.wall_id), loc(&format!("Rigid assumption, {}", w.wall_id), &format!("Starre Annahme, {}", w.wall_id)), (w.s_m - 1.1 * half, w.s_m), kn(w.rigid_n), d.class == m::DiaphragmClass::Rigid, tone_of(m::DiaphragmClass::Rigid)));
        series.push(bar(format!("flexible-{}", w.wall_id), loc(&format!("Flexible assumption, {}", w.wall_id), &format!("Verformbare Annahme, {}", w.wall_id)), (w.s_m, w.s_m + 1.1 * half), kn(w.tributary_n), d.class == m::DiaphragmClass::Flexible, tone_of(m::DiaphragmClass::Flexible)));
        let share = kn(w.share_n(d.class));
        markers.push(Marker { id: format!("share-{}", w.wall_id), at: Vec2::new(w.s_m, share), shape: MarkerShape::Diamond, tone: tone_of(d.class), label: loc(&format!("{}: {share:.1} kN governing share, rigid {:.1} kN, flexible {:.1} kN", w.wall_id, kn(w.rigid_n), kn(w.tributary_n)), &format!("{}: {share:.1} kN maßgebender Anteil, starr {:.1} kN, verformbar {:.1} kN", w.wall_id, kn(w.rigid_n), kn(w.tributary_n))), utilisation: None, bound_check: check.clone(), binding: None });
    }
    let (class_en, class_de) = match d.class {
        m::DiaphragmClass::Rigid => ("rigid", "starr"),
        m::DiaphragmClass::Flexible => ("flexible", "verformbar"),
    };
    let note = DiagramNote { id: "ratio".into(), at: Vec2::new(d.span_m.0 + 0.5 * span, 1.05 * top.max(1e-9)), text: loc(&format!("Classification ratio {:.2} against {:.2}: {class_en}", d.ratio, m::RIGID_RATIO), &format!("Einstufungsverhältnis {:.2} gegenüber {:.2}: {class_de}", d.ratio, m::RIGID_RATIO)), tone: if d.ratio > m::RIGID_RATIO { Tone::Warning } else { Tone::Info } };
    let table = vec![
        column("Wall position", "Wandlage", "m", table_s),
        column("Stiffness k", "Steifigkeit k", "N/m", d.walls.iter().map(|w| w.stiffness_n_per_m).collect()),
        column("Rigid share", "Starrer Anteil", "kN", d.walls.iter().map(|w| kn(w.rigid_n)).collect()),
        column("Flexible share", "Verformbarer Anteil", "kN", d.walls.iter().map(|w| kn(w.tributary_n)).collect()),
        column("Wall drift", "Wandverschiebung", "mm", d.walls.iter().map(|w| w.drift_m * 1e3).collect()),
    ];
    Diagram2d {
        id: format!("diaphragm:{axis_key}"),
        title: loc(&format!("Wall forces, direction {axis_key}: rigid against flexible diaphragm"), &format!("Wandkräfte, Richtung {axis_key}: starre gegenüber verformbarer Scheibe")),
        description: loc(
            &format!("Share of the storey shear {:.1} kN of every bracing wall at its position across the diaphragm: solid bar = rigid assumption (by stiffness), second bar = flexible assumption (tributary width); the classified rule is drawn solid and its shares are marked (EN 1998-1 4.3.1(4)).", kn(d.force_n)),
            &format!("Anteil der Geschossquerkraft {:.1} kN jeder aussteifenden Wand an ihrer Lage über die Scheibe: erster Balken = starre Annahme (nach Steifigkeit), zweiter Balken = verformbare Annahme (Einflussbreite); die eingestufte Regel ist voll gezeichnet und ihre Anteile sind markiert (EN 1998-1 4.3.1(4)).", kn(d.force_n)),
        ),
        axes: vec![axis("Position across the diaphragm", "Lage quer zur Scheibe", QuantityKind::Length, "m", AxisScale::Linear, Some((d.span_m.0 - 2.0 * half - 0.05 * span, d.span_m.1 + 2.0 * half + 0.05 * span))), axis("Share of storey shear", "Anteil der Geschossquerkraft", QuantityKind::Force, "kN", AxisScale::Linear, Some((0.0, 1.2 * top.max(1e-9))))],
        series,
        markers,
        annotations: vec![note],
        focus: Some(wall_ref(&wall.id)),
        legend: true,
        table_twin: DataTable { columns: table },
    }
}

fn classification(document: &En1996Snapshot, d: &m::Diaphragm, wall: &MasonryWall, check: Option<String>) -> Option<Diagram2d> {
    let weakest = diaphragm_walls(document).into_iter().min_by(|a, b| a.diaphragm_stiffness_n_per_m.total_cmp(&b.diaphragm_stiffness_n_per_m))?;
    let g_t = d.g_t_n_per_m;
    let (lo, hi) = (g_t / 100.0, g_t * 100.0);
    let samples: Vec<(f64, f64)> = curves::log_space(lo, hi, RATIO_SAMPLES).into_iter().map(|g| (g, m::displacement_ratio(d.span_m, g * d.depth_m, &d.springs))).collect();
    let top = samples.iter().map(|s| s.1).fold(m::RIGID_RATIO * 1.2, f64::max).max(d.ratio);
    let axis_key = d.axis.key();
    let points: Vec<Vec2> = samples.iter().map(|s| Vec2::new(s.0, s.1)).collect();
    let series = vec![
        Series::Polyline { id: "ratio".into(), label: loc("Largest displacement over the rigid displacement", "Größte Verschiebung über der starren Verschiebung"), points, style: stroke(Tone::Info, 2.0, Dash::Solid) },
        Series::Limit { id: "limit".into(), label: loc(&format!("Rigid up to {:.2} (EN 1998-1 4.3.1(4))", m::RIGID_RATIO), &format!("Starr bis {:.2} (EN 1998-1 4.3.1(4))", m::RIGID_RATIO)), axis: AxisRef::Y, value: m::RIGID_RATIO, style: stroke(Tone::Warning, 1.5, Dash::Dashed) },
        Series::Limit { id: "declared".into(), label: loc("Declared stiffness G·t", "Angesetzte Steifigkeit G·t"), axis: AxisRef::X, value: g_t, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
    ];
    let marker = Marker {
        id: "stiffness".into(),
        at: Vec2::new(g_t, d.ratio),
        shape: MarkerShape::Circle,
        tone: if d.ratio > m::RIGID_RATIO { Tone::Warning } else { Tone::Info },
        label: loc(&format!("G·t = {g_t:.3e} N/m: ratio {:.2} against {:.2}", d.ratio, m::RIGID_RATIO), &format!("G·t = {g_t:.3e} N/m: Verhältnis {:.2} gegenüber {:.2}", d.ratio, m::RIGID_RATIO)),
        utilisation: Some(d.ratio / m::RIGID_RATIO),
        bound_check: check,
        binding: Some(FieldBinding { path: format!("walls[id={}].diaphragmStiffnessNPerM", weakest.id), map: Affine::IDENTITY, unit: "N/m".into(), quantise: Some(Quantise::Choice { values: STIFFNESS_LADDER.to_vec() }) }),
    };
    let table = vec![column("Stiffness G·t", "Steifigkeit G·t", "N/m", samples.iter().map(|s| s.0).collect()), column("Displacement ratio", "Verschiebungsverhältnis", "1", samples.iter().map(|s| s.1).collect())];
    Some(Diagram2d {
        id: format!("diaphragm-ratio:{axis_key}"),
        title: loc(&format!("Diaphragm classification, direction {axis_key}"), &format!("Einstufung der Scheibe, Richtung {axis_key}")),
        description: loc(
            &format!("Largest displacement of the diaphragm over the rigid-diaphragm displacement F/Σk against its stiffness G·t; the diaphragm is rigid below the limit {:.2}; the marker drags the declared stiffness of the weakest wall.", m::RIGID_RATIO),
            &format!("Größte Verschiebung der Scheibe über der Verschiebung F/Σk der starren Scheibe über ihrer Steifigkeit G·t; die Scheibe ist unterhalb der Grenze {:.2} starr; die Markierung verschiebt die angesetzte Steifigkeit der schwächsten Wand.", m::RIGID_RATIO),
        ),
        axes: vec![axis("Diaphragm stiffness G·t", "Scheibensteifigkeit G·t", QuantityKind::ForcePerLength, "N/m", AxisScale::Log10, Some((lo, hi))), axis("Displacement ratio", "Verschiebungsverhältnis", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.9, top * 1.05)))],
        series,
        markers: vec![marker],
        annotations: Vec::new(),
        focus: Some(wall_ref(&wall.id)),
        legend: true,
        table_twin: DataTable { columns: table },
    })
}

/// 📈️ The two diagrams of every classified direction: the wall force distribution (rigid beside flexible) and the classification ratio over the stiffness with the limit; with `focus` set only when the focused wall braces the direction.
pub fn diagrams(document: &En1996Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let mut out = Vec::new();
    for d in m::diaphragms(document.annex, &document.walls) {
        if !focus.is_none_or(|f| d.walls.iter().any(|w| w.wall_id == base_id(f))) {
            continue;
        }
        let Some(wall) = d.walls.first().and_then(|w| document.walls.iter().find(|x| x.id == w.wall_id)) else { continue };
        let check = report.checks.iter().any(|c| c.id == format!("en1996.9.3.diaphragm.{}", d.axis.key())).then(|| format!("en1996.9.3.diaphragm.{}", d.axis.key()));
        out.push(distribution(&d, wall, check.clone()));
        out.extend(classification(document, &d, wall, check));
    }
    out
}
