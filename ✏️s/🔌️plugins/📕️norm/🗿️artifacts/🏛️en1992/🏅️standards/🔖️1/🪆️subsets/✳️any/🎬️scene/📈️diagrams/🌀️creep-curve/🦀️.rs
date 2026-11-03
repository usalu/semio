//! 🌀️ Creep and shrinkage over time: the creep coefficient φ(t, t0) and the total shrinkage strain ε_cs(t) of EN 1992-1-1 3.1.4 and Annex B (the functions of the age tracks) as one diagram over the logarithmic concrete age.

use super::{axis, base, stroke, titled, xy};
use super::kit::*;
use super::model::{MemberKind, RcMember};
use super::scene::projection_kit::diagram_parts::{self, column};
use super::scene::*;
use super::tracks::{creep_phi, notional_size, shrinkage_strain, DRYING_START_DAYS, LOADING_AGE_DAYS, RELATIVE_HUMIDITY};

const SAMPLES: usize = 40;
const LAST_DAY: f64 = 36_500.0;
const MARK_DAY: f64 = 18_250.0;

/// 📈️ Diagram `creep-shrinkage:<member>` for a concrete of characteristic strength `fck` (MPa) in the section of `m`: φ(t, 28 d) and 1000 ε_cs(t) over the age in days, with markers at 50 years.
pub fn diagram(m: &RcMember, fck: f64, kind: EntityKind, path: &str) -> Diagram2d {
    let width = if m.kind == MemberKind::FlatSlab { m.span } else { m.width };
    let h0 = notional_size(width, m.height);
    let days: Vec<f64> = (0..SAMPLES).map(|i| LAST_DAY.powf(i as f64 / (SAMPLES - 1) as f64)).collect();
    let phi = |t: f64| creep_phi(fck + 8.0, RELATIVE_HUMIDITY, h0, LOADING_AGE_DAYS, t);
    let eps = |t: f64| 1000.0 * shrinkage_strain(fck, RELATIVE_HUMIDITY, h0, t, DRYING_START_DAYS);
    let creep: Vec<[f64; 2]> = days.iter().map(|&t| [t, phi(t)]).collect();
    let shrinkage: Vec<[f64; 2]> = days.iter().map(|&t| [t, eps(t)]).collect();
    let mut g = base(
        m,
        "creep-shrinkage",
        titled(m, "Creep and shrinkage", "Kriechen und Schwinden"),
        loc("Creep coefficient φ(t, 28 d) and total shrinkage strain εcs(t) in permille of EN 1992-1-1 3.1.4 and Annex B for normal-hardening cement, RH 50 % and the notional size h0 of the section; drying starts at 3 days.", "Kriechzahl φ(t, 28 d) und gesamte Schwinddehnung εcs(t) in Promille nach EN 1992-1-1 3.1.4 und Anhang B für normal erhärtenden Zement, RH 50 % und die wirksame Dicke h0 des Querschnitts; das Austrocknen beginnt mit 3 Tagen."),
        vec![
            diagram_parts::axis("Concrete age", "Betonalter", QuantityKind::Time, "d", AxisScale::Log10, None),
            axis("Creep coefficient φ and shrinkage εcs (‰)", "Kriechzahl φ und Schwinddehnung εcs (‰)", QuantityKind::Dimensionless, "1"),
        ],
        kind,
        path,
    );
    g.series.push(Series::Polyline { id: "creep".into(), label: loc("Creep coefficient φ(t, 28 d)", "Kriechzahl φ(t, 28 d)"), points: xy(&creep), style: stroke(2.0, Dash::Solid, Tone::Info) });
    g.series.push(Series::Polyline { id: "shrinkage".into(), label: loc("Shrinkage εcs (‰)", "Schwinden εcs (‰)"), points: xy(&shrinkage), style: stroke(2.0, Dash::Dashed, Tone::Warning) });
    let marker = |id: &str, value: f64, tone: Tone, en: String, de: String| Marker { id: format!("reference:{id}"), at: Vec2::new(MARK_DAY, value), shape: MarkerShape::Diamond, tone, label: loc(&en, &de), utilisation: None, bound_check: None, binding: None };
    g.markers.push(marker("phi-50a", phi(MARK_DAY), Tone::Info, format!("φ(50 a) = {:.2}", phi(MARK_DAY)), format!("φ(50 a) = {:.2}", phi(MARK_DAY))));
    g.markers.push(marker("eps-50a", eps(MARK_DAY), Tone::Warning, format!("εcs(50 a) = {:.3} ‰", eps(MARK_DAY)), format!("εcs(50 a) = {:.3} ‰", eps(MARK_DAY))));
    g.table_twin = DataTable { columns: vec![column("Age", "Alter", "d", days.clone()), column("Creep coefficient", "Kriechzahl", "1", creep.iter().map(|p| p[1]).collect()), column("Shrinkage strain", "Schwinddehnung", "‰", shrinkage.iter().map(|p| p[1]).collect())] };
    g
}
