//! 🌈️ Gamma bands: the permanent partial factor of every permanent action of a member as a colour band on the side face of its beam (EN 1990 A1.3.1, Table A1.2(B) note 1: γ_G,sup where the action adds to the effect, γ_G,inf where it relieves). The bands follow the permanent actions of the member in table order, equal in width; each carries one scalar field per combination column, so the combination stepper recolours the pattern of the governing arrangement of every column without re-projecting. See [`super::combinations::permanent_factors`].

use super::scene::projection_kit::{loc, unit, along};
use super::combinations::{self, Column};
use super::dynamics::field_id;
use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::SceneInput;

const OFFSET: f64 = 0.012;

/// 🌈️ The surface grid of a member's side face split into `bands` equal bands: two vertex columns per band edge pair so each band is flat-coloured, two rows (bottom, top).
fn grid(frame: &Frame, span: f64, depth: f64, bands: usize) -> (Vec<Vec3>, PointGrid) {
    let side = depth / 4.0 + OFFSET;
    let stations: Vec<f64> = (0..bands).flat_map(|k| [k, k + 1]).map(|edge| span * edge as f64 / bands as f64).collect();
    let points: Vec<Vec3> = [-depth / 2.0, depth / 2.0].into_iter().flat_map(|z| stations.iter().map(move |s| (*s, z))).map(|(s, z)| along(&along(&along(&frame.origin, &frame.x, s), &frame.y, side), &frame.z, z)).collect();
    let grid = PointGrid { rows: 2, columns: stations.len() as u32, points: points.clone() };
    (points, grid)
}

/// 🌈️ Adds the band surface of every member with at least one permanent action acting on it.
pub(crate) fn project(input: &SceneInput, layout: &Layout, columns: &[Column], active: Option<&Column>, out: &mut Out) {
    let d = input.document;
    if columns.is_empty() {
        return;
    }
    let first = field_id(active.unwrap_or(&columns[0]));
    for member in &d.members {
        if !out.wants(&member.id) {
            continue;
        }
        let (Some(pose), true) = (layout.member(&member.id), member.span > 1e-6) else { continue };
        let rows = combinations::permanent_factors(d, &member.id, columns);
        if rows.is_empty() {
            continue;
        }
        let (points, shape) = grid(&pose.frame, member.span, member_depth(member.span), rows.len());
        let fields: Vec<ScalarField> = columns
            .iter()
            .enumerate()
            .map(|(c, column)| ScalarField { id: field_id(column), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values: (0..2).flat_map(|_| rows.iter().flat_map(|(_, factors)| [factors[c], factors[c]])).collect() } })
            .collect();
        let ids = rows.iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>().join(", ");
        let spec = Spec {
            kind: EntityKind::Surface,
            id: format!("{}:gamma-bands", member.id),
            collection: "members",
            geometry: Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: shape } }),
            role: MaterialRole::Neutral,
            binding: None,
            name: loc(&format!("{} permanent factor bands", member.label_en), &format!("{} Bänder der ständigen Teilsicherheitsbeiwerte", member.label_de)),
            detail: loc(&format!("γ_G of the permanent actions {ids} in the active combination, one band per action in table order"), &format!("γ_G der ständigen Einwirkungen {ids} in der aktiven Kombination, ein Band je Einwirkung in Tabellenreihenfolge")),
            layer: LAYER_GAMMA,
            handles: Vec::new(),
            deformation: None,
            ghost: false,
        };
        out.push_fields(spec, &points, first.clone(), fields);
    }
}
