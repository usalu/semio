//! 🏛️ The section and the elevation of a view: what the solids of the building show behind the plane of the view, drawn in `(u, z)`.
//!
//! Every solid the view sees is clipped to the slab of the view and projected: its silhouette is a filled region (painted farthest first), its visible feature edges are stroked, and in a
//! fine view its hidden edges are dashed. A *section* additionally cuts the solids the plane passes through: the cut is a filled region in the cut style, and the faces that close it hide
//! what lies behind them. An *elevation* cuts nothing. Storey levels are datum lines across the plane.

use super::clip::{clip_ring, clip_segment, Rect};
use super::cut::{cut_regions, Cut};
use super::filters::{category_of_family, phase_of};
use super::frame::Frame;
use super::hidden_lines::{bodies, faces, feature_edges, mean_depth, pieces, Facing, Tri};
use super::ViewLinework;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanKind, PlanLinework, PlanStyle, PlanVertex, Sheet};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::{DetailLevel, ModelSnapshot, View, ViewKind};
use semio_framework_geometry::triangulation::triangulate;
use semio_framework_geometry::Point;
use std::collections::BTreeMap;

/// 🧾️ The values a vertical view is drawn from: the levels of the storeys of its building and the solids of that building, by element id.
pub struct Inputs<'a> {
    pub levels: &'a BTreeMap<String, StoreyLevel>,
    pub solids: &'a BTreeMap<&'a str, &'a ElementSolid>,
}

fn vertices(ring: &[[f64; 2]]) -> Vec<PlanVertex> {
    ring.iter().map(|p| PlanVertex { x: p[0], y: p[1], bulge: 0.0 }).collect()
}

fn caps(cuts: &[Cut], frame: &Frame) -> Vec<Tri> {
    let toward = [-frame.look[0], -frame.look[1], 0.0];
    cuts.iter()
        .flat_map(|cut| {
            let points = |ring: &[[f64; 2]]| ring.iter().map(|p| Point::new(p[0], p[1])).collect::<Vec<_>>();
            let holes: Vec<Vec<Point>> = cut.holes.iter().map(|hole| points(hole)).collect();
            let mesh = triangulate(&points(&cut.outer), &holes);
            mesh.triangles
                .iter()
                .map(|t| {
                    let corner = |index: u32| {
                        let p = mesh.vertices[index as usize];
                        [p.x, p.y, 0.0]
                    };
                    let drawn = [corner(t[0]), corner(t[1]), corner(t[2])];
                    Tri { world: drawn, drawn, normal: toward, facing: Facing::Front }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn datums(sheet: &mut Sheet, snapshot: &ModelSnapshot, view: &View, frame: &Frame, levels: &BTreeMap<String, StoreyLevel>, crop: Option<&Rect>) {
    let mut rows: Vec<(&String, &str, f64, String)> = snapshot.storeys.iter().filter(|(_, storey)| storey.building == view.building).filter_map(|(id, storey)| levels.get(id).map(|level| (id, storey.name.as_str(), level.elevation, String::new()))).collect();
    rows.sort_by(|a, b| a.2.total_cmp(&b.2).then(a.0.cmp(b.0)));
    let top = snapshot.storeys.iter().filter(|(_, storey)| storey.building == view.building).filter_map(|(id, storey)| levels.get(id).map(|level| (id, storey.name.as_str(), level.top_elevation))).max_by(|a, b| a.2.total_cmp(&b.2));
    if let Some((id, name, elevation)) = top {
        rows.push((id, name, elevation, " top".to_string()));
    }
    for (storey, name, z, suffix) in rows {
        let segment = clip_segment([0.0, z], [frame.length, z], &crop.copied().unwrap_or(Rect { x0: f64::NEG_INFINITY, y0: f64::NEG_INFINITY, x1: f64::INFINITY, y1: f64::INFINITY }));
        let Some((a, b)) = segment else { continue };
        sheet.polyline(storey, PlanKind::Datum, PlanStyle::Annotation, false, vertices(&[a, b]));
        if crop.is_none_or(|rect| rect.contains([frame.length, z])) {
            sheet.text(storey, PlanKind::DatumLabel, PlanStyle::Annotation, Point::new(frame.length + 0.15, z), 0.0, &format!("{name}{suffix}"), &format!("{z:+.2}"), None);
        }
    }
}

fn region(sheet: &mut Sheet, element: &str, kind: PlanKind, style: PlanStyle, cut: &Cut, crop: Option<&Rect>) {
    let (outer, holes) = match crop {
        Some(rect) => (clip_ring(&cut.outer, rect), cut.holes.iter().map(|hole| clip_ring(hole, rect)).filter(|hole| !hole.is_empty()).collect::<Vec<_>>()),
        None => (cut.outer.clone(), cut.holes.clone()),
    };
    if outer.len() >= 3 {
        sheet.region(element, kind, style, vertices(&outer), holes.iter().map(|hole| vertices(hole)).collect());
    }
}

fn stroke(sheet: &mut Sheet, element: &str, style: PlanStyle, a: [f64; 2], b: [f64; 2], crop: Option<&Rect>) {
    let segment = match crop {
        Some(rect) => clip_segment(a, b, rect),
        None => Some((a, b)),
    };
    if let Some((a, b)) = segment {
        sheet.polyline(element, PlanKind::Edge, style, false, vertices(&[a, b]));
    }
}

/// 🏛️ The linework of a section or elevation; an empty drawing when the plane has no length.
pub fn vertical_view(snapshot: &ModelSnapshot, id: &str, view: &View, inputs: &Inputs<'_>) -> ViewLinework {
    let Some(frame) = view.plane.as_ref().and_then(Frame::of) else {
        return ViewLinework::empty(id, view);
    };
    let crop = view.crop.map(|crop| Rect { x0: crop.min.x, y0: crop.min.y, x1: crop.max.x, y1: crop.max.y });
    let crop = crop.as_ref();
    let shown: Vec<(&str, &ElementSolid)> = inputs
        .solids
        .iter()
        .filter(|(element, solid)| !category_of_family(solid.family).is_some_and(|category| view.hides(category)) && !(view.phase.is_some() && phase_of(snapshot, element).is_some_and(|phase| !view.shows_phase(phase))))
        .map(|(element, solid)| (*element, *solid))
        .collect();
    let seen = bodies(&frame, view.depth, shown.iter().copied());
    let section = view.kind == ViewKind::Section;
    let cuts: Vec<(&str, Vec<Cut>)> = if section { shown.iter().map(|(element, solid)| (*element, cut_regions(solid, &frame))).filter(|(_, cuts)| !cuts.is_empty()).collect() } else { Vec::new() };
    let closing = caps(&cuts.iter().flat_map(|(_, cuts)| cuts.iter().cloned()).collect::<Vec<_>>(), &frame);
    let mut sheet = Sheet::default();
    datums(&mut sheet, snapshot, view, &frame, inputs.levels, crop);
    let mut order: Vec<&_> = seen.iter().collect();
    order.sort_by(|a, b| mean_depth(b).total_cmp(&mean_depth(a)).then(a.element.cmp(&b.element)));
    for body in &order {
        for cut in faces(body) {
            region(&mut sheet, &body.element, PlanKind::Silhouette, PlanStyle::Projection, &cut, crop);
            if view.detail == DetailLevel::Coarse {
                let ring = &cut.outer;
                for index in 0..ring.len() {
                    stroke(&mut sheet, &body.element, PlanStyle::Projection, ring[index], ring[(index + 1) % ring.len()], crop);
                }
            }
        }
    }
    for (element, cuts) in &cuts {
        for cut in cuts {
            region(&mut sheet, element, PlanKind::SectionCut, PlanStyle::Cut, cut, crop);
        }
    }
    if view.detail != DetailLevel::Coarse {
        let edges: Vec<_> = seen.iter().flat_map(|body| feature_edges(body, &frame, view.depth)).collect();
        for piece in pieces(&edges, &seen, &closing) {
            if piece.visible {
                stroke(&mut sheet, &piece.element, PlanStyle::Projection, piece.a, piece.b, crop);
            } else if view.detail == DetailLevel::Fine {
                stroke(&mut sheet, &piece.element, PlanStyle::Hidden, piece.a, piece.b, crop);
            }
        }
    }
    let lines = sheet.finish("", 0.0);
    let lines = match crop {
        Some(rect) => PlanLinework { bounds: crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanBounds { min_x: rect.x0, min_y: rect.y0, max_x: rect.x1, max_y: rect.y1 }, ..lines },
        None => lines,
    };
    ViewLinework { view: id.to_string(), kind: view.kind, scale: view.scale, detail: view.detail, lines }
}
