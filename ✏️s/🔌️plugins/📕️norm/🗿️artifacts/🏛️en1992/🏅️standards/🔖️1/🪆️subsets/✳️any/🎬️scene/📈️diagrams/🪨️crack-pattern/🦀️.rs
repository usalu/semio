//! 🪨️ Crack pattern: the tension-face elevation of a bending member with one line per predicted crack at the crack spacing s_r,max (EN 1992-1-1 7.3.4), each line coded by its width w_k read from the stations of the crack-width check against the limit w_max, with the table twin.

use super::kit::*;
use super::layout::Class;
use super::mechanics;
use super::norm;
use super::model::RcMember;
use super::scene::*;
use super::scene::projection_kit::diagram_parts::{self, column};
use super::SceneInput;

const MAX_LINES: usize = 40;
const SAMPLES: usize = 200;

/// 🪨️ One predicted crack: position along the span [m], width w_k [m] and its width class against w_max.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crack {
    pub x: f64,
    pub wk: f64,
    pub class: WidthClass,
}

/// 🎨️ Width class of a crack: at most half of the limit, within the limit, above the limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WidthClass {
    Fine,
    Near,
    Exceeding,
}

/// 🧮️ The class of a crack width against the limit.
pub fn width_class(wk: f64, w_max: f64) -> WidthClass {
    if wk > w_max {
        WidthClass::Exceeding
    } else if wk > 0.5 * w_max {
        WidthClass::Near
    } else {
        WidthClass::Fine
    }
}

/// 📐️ Crack positions centred inside the cracked zone `[lo, hi]` with spacing `sr_max`, at most `limit` of them.
pub fn crack_positions(lo: f64, hi: f64, sr_max: f64, limit: usize) -> Vec<f64> {
    if sr_max <= 0.0 || hi <= lo {
        return Vec::new();
    }
    let count = (((hi - lo) / sr_max).floor() as usize).min(limit);
    let offset = (hi - lo - sr_max * count as f64) / 2.0;
    (0..count).map(|i| lo + offset + sr_max * (i as f64 + 0.5)).collect()
}

/// 🪨️ The predicted cracks, the spacing s_r,max, the limit w_max and the neutral-axis depth of one member, all from the crack-width check stations.
pub fn pattern(input: &SceneInput, m: &RcMember, sec: &super::checks::Sec) -> Option<(Vec<Crack>, f64, f64, f64)> {
    let id = format!("en1992.7.3.crack.{}", m.id);
    let check = input.report.checks.iter().find(|r| r.id == id && r.status != norm::CheckStatus::NotApplicable && !r.stations.is_empty())?;
    let state = mechanics::crack_state(sec, m, &input.document.action_factors())?;
    let w_max = check.limit.value;
    let params: Vec<f64> = (0..=SAMPLES).map(|i| i as f64 / SAMPLES as f64).collect();
    let widths: Vec<f64> = projection_kit::field_along(check, &params).into_iter().map(|u| u * w_max).collect();
    let zone: Vec<f64> = params.iter().zip(&widths).filter(|(_, w)| **w > 1e-6).map(|(s, _)| *s).collect();
    let (&lo, &hi) = (zone.first()?, zone.last()?);
    let cracks = crack_positions(lo * m.span, hi * m.span, state.sr_max, MAX_LINES)
        .into_iter()
        .map(|x| {
            let wk = widths[((x / m.span) * SAMPLES as f64).round().clamp(0.0, SAMPLES as f64) as usize];
            Crack { x, wk, class: width_class(wk, w_max) }
        })
        .collect();
    Some((cracks, state.sr_max, w_max, state.x))
}

fn style(class: WidthClass) -> Stroke {
    match class {
        WidthClass::Fine => Stroke { width_px: 1.0, dash: Dash::Dotted, tone: Tone::Info },
        WidthClass::Near => Stroke { width_px: 2.0, dash: Dash::Dashed, tone: Tone::Warning },
        WidthClass::Exceeding => Stroke { width_px: 3.0, dash: Dash::Solid, tone: Tone::Error },
    }
}

/// 📈️ The crack pattern diagram of a bending member with a crack-width check; `None` for columns and members without cracks.
pub fn diagram(input: &SceneInput, m: &RcMember, sec: &super::checks::Sec, kind: EntityKind, path: &str, pose_class: Class) -> Option<Diagram2d> {
    if pose_class == Class::Upright {
        return None;
    }
    let (cracks, sr_max, w_max, x_na) = pattern(input, m, sec)?;
    if cracks.is_empty() {
        return None;
    }
    let depth = (m.height - x_na).max(0.05);
    let check_id = format!("en1992.7.3.crack.{}", m.id);
    let mut g = Diagram2d {
        id: format!("crack-pattern:{}", m.id),
        title: loc(&format!("{} - Crack pattern", m.label_en), &format!("{} - Rissbild", m.label_de)),
        description: loc(
            &format!("Tension-face elevation: one line per predicted crack at s_r,max = {} mm up to the neutral axis, coded by w_k against w_max = {:.2} mm (dotted up to half the limit, dashed within, solid above); widths are read from the stations of check {check_id}.", mm(sr_max), w_max * 1000.0),
            &format!("Ansicht der Zugseite: eine Linie je berechnetem Riss im Abstand s_r,max = {} mm bis zur Nulllinie, nach w_k gegen w_max = {:.2} mm codiert (gepunktet bis zur halben Grenze, gestrichelt innerhalb, durchgezogen darüber); die Breiten stammen aus den Prüfstellen der Nachweise {check_id}.", mm(sr_max), w_max * 1000.0),
        ),
        axes: vec![
            diagram_parts::axis("Position along the span", "Lage entlang der Spannweite", QuantityKind::Length, "m", AxisScale::Linear, None),
            diagram_parts::axis("Depth above the tension face", "Höhe über der Zugseite", QuantityKind::Length, "m", AxisScale::Linear, None),
        ],
        series: Vec::new(),
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(entity_ref(kind, &m.id, path)),
        legend: true,
        table_twin: DataTable { columns: Vec::new() },
    };
    g.series.push(Series::Polyline { id: "tension-face".into(), label: loc("Tension face", "Zugseite"), points: vec![Vec2::new(0.0, 0.0), Vec2::new(m.span, 0.0)], style: Stroke { width_px: 2.0, dash: Dash::Solid, tone: Tone::Neutral } });
    for (class, en, de) in [(WidthClass::Fine, "w_k up to half of w_max", "w_k bis zur Hälfte von w_max"), (WidthClass::Near, "w_k within w_max", "w_k innerhalb w_max"), (WidthClass::Exceeding, "w_k above w_max", "w_k über w_max")] {
        let mut first = true;
        for (i, c) in cracks.iter().enumerate().filter(|(_, c)| c.class == class) {
            let label = if first { loc(en, de) } else { loc(&format!("Crack {}", i + 1), &format!("Riss {}", i + 1)) };
            first = false;
            g.series.push(Series::Polyline { id: format!("crack-{i}"), label, points: vec![Vec2::new(c.x, 0.0), Vec2::new(c.x, depth)], style: style(class) });
        }
    }
    g.table_twin = DataTable { columns: vec![
        column("Crack position", "Rissposition", "m", cracks.iter().map(|c| c.x).collect()),
        column("Crack width w_k", "Rissbreite w_k", "mm", cracks.iter().map(|c| c.wk * 1000.0).collect()),
        column("Utilisation w_k / w_max", "Ausnutzung w_k / w_max", "1", cracks.iter().map(|c| c.wk / w_max).collect()),
        column("Crack spacing s_r,max", "Rissabstand s_r,max", "m", cracks.iter().map(|_| sr_max).collect()),
        column("Crack height", "Risshöhe", "m", cracks.iter().map(|_| depth).collect()),
    ] };
    Some(g)
}
