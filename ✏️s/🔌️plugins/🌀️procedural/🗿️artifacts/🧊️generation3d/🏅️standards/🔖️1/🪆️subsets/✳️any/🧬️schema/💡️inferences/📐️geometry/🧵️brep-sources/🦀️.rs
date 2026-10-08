//! 🧵️ Curve and surface sources of shape values: the value-level view `brep.evaluate` and `brep.intersect` read, so edges, wires and faces answer the same questions as bare curves and surfaces without a kernel session.
//!
//! A curve and an edge are parameterised by their own curve parameter; a wire of `n` members is parameterised over `[0, n]`, member `i` covering `[i, i + 1]` in the member's own direction. A face answers through its supporting surface, with the normal turned the way the face points.

use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{ShapeRoot, ShapeValue};
use semio_framework_3d::brep::representation::curve::curve_ops::closest_parameter;
use semio_framework_3d::brep::representation::curve::Curve3;
use semio_framework_3d::brep::representation::surface::surface_ops::closest_uv;
use semio_framework_3d::brep::representation::surface::Surface;
use semio_framework_3d::brep::representation::vector::Pnt3;

fn axes(point: Pnt3) -> [f64; 3] {
    [point.x, point.y, point.z]
}

//#region 🔖️Curve
/// 🧩️ One stretch of a curve source: the source parameter range `s`, the curve parameter range `t` it maps onto linearly (descending when the member runs against its edge) and the curve.
#[derive(Clone, Debug)]
pub struct Piece {
    pub s: (f64, f64),
    pub t: (f64, f64),
    pub identity: bool,
    pub curve: Curve3,
}

impl Piece {
    /// 🔁️ The curve parameter at source parameter `s`.
    pub fn curve_parameter(&self, s: f64) -> f64 {
        if self.identity {
            s
        } else {
            self.t.0 + (s - self.s.0) / (self.s.1 - self.s.0) * (self.t.1 - self.t.0)
        }
    }

    /// 🔁️ The source parameter at curve parameter `t`.
    pub fn source_parameter(&self, t: f64) -> f64 {
        if self.identity {
            t
        } else {
            self.s.0 + (t - self.t.0) / (self.t.1 - self.t.0) * (self.s.1 - self.s.0)
        }
    }

    /// 📏️ The ascending curve parameter interval the piece covers.
    pub fn curve_range(&self) -> (f64, f64) {
        (self.t.0.min(self.t.1), self.t.0.max(self.t.1))
    }

    fn reversed(&self) -> bool {
        !self.identity && self.t.1 < self.t.0
    }
}

/// 〰️ A curve, an edge or a wire as one parameterised curve.
#[derive(Clone, Debug)]
pub struct CurveSource {
    pieces: Vec<Piece>,
}

impl CurveSource {
    /// 🔎️ The curve source of a curve, edge or wire value; `None` for every other kind.
    pub fn of(shape: &ShapeValue) -> Option<Self> {
        let body = &shape.body;
        let pieces = match &shape.root {
            ShapeRoot::Curve { curve, .. } => {
                let domain = curve.domain();
                vec![Piece { s: domain, t: domain, identity: true, curve: curve.clone() }]
            }
            ShapeRoot::Edge(id) => {
                let edge = body.edges.get(*id)?;
                vec![Piece { s: edge.range, t: edge.range, identity: true, curve: body.curves3.get(edge.curve)?.clone() }]
            }
            ShapeRoot::Wire(wire) => {
                let mut pieces = Vec::with_capacity(wire.members.len());
                for (index, (id, forward)) in wire.members.iter().enumerate() {
                    let edge = body.edges.get(*id)?;
                    let t = if *forward { edge.range } else { (edge.range.1, edge.range.0) };
                    pieces.push(Piece { s: (index as f64, index as f64 + 1.0), t, identity: false, curve: body.curves3.get(edge.curve)?.clone() });
                }
                if pieces.is_empty() {
                    return None;
                }
                pieces
            }
            _ => return None,
        };
        Some(Self { pieces })
    }

    /// 🧩️ The pieces in source order.
    pub fn pieces(&self) -> &[Piece] {
        &self.pieces
    }

    /// 📏️ The source parameter range; infinite for an unbounded line.
    pub fn domain(&self) -> (f64, f64) {
        let first = &self.pieces[0];
        let last = &self.pieces[self.pieces.len() - 1];
        (first.s.0, last.s.1)
    }

    /// 🔎️ The piece that answers source parameter `s` (clamped into the domain) and the clamped parameter.
    fn locate(&self, s: f64) -> (&Piece, f64) {
        let (start, end) = self.domain();
        let s = s.clamp(start, end);
        let piece = self.pieces.iter().find(|piece| s <= piece.s.1).unwrap_or(&self.pieces[self.pieces.len() - 1]);
        (piece, s)
    }

    pub fn point(&self, s: f64) -> [f64; 3] {
        let (piece, s) = self.locate(s);
        axes(piece.curve.eval(piece.curve_parameter(s)))
    }

    /// 🧭️ The unit tangent in the direction of growing source parameter, `None` where the curve stalls.
    pub fn tangent(&self, s: f64) -> Option<[f64; 3]> {
        let (piece, s) = self.locate(s);
        let direction = piece.curve.tangent(piece.curve_parameter(s))?;
        let sign = if piece.reversed() { -1.0 } else { 1.0 };
        Some([sign * direction.x, sign * direction.y, sign * direction.z])
    }

    pub fn curvature(&self, s: f64) -> f64 {
        let (piece, s) = self.locate(s);
        piece.curve.curvature(piece.curve_parameter(s))
    }

    /// 🎯️ The source parameter, point and distance of the closest point of the source to `point`; the earliest piece wins a tie.
    pub fn closest(&self, point: [f64; 3]) -> (f64, [f64; 3], f64) {
        let target = Pnt3::new(point[0], point[1], point[2]);
        let mut best: Option<(f64, [f64; 3], f64)> = None;
        for piece in &self.pieces {
            let found = closest_parameter(&piece.curve, piece.curve_range(), target, 1e-9);
            if best.is_none_or(|(_, _, distance)| found.distance < distance) {
                best = Some((piece.source_parameter(found.t), axes(found.point), found.distance));
            }
        }
        best.unwrap_or((self.domain().0, [0.0; 3], f64::INFINITY))
    }
}
//#endregion 🔖️Curve

//#region 🔖️Surface
/// 🏄️ A surface or a face as one parameterised surface.
#[derive(Clone, Debug)]
pub struct SurfaceSource {
    pub surface: Surface,
    pub flipped: bool,
}

impl SurfaceSource {
    /// 🔎️ The surface source of a surface or face value; `None` for every other kind.
    pub fn of(shape: &ShapeValue) -> Option<Self> {
        match &shape.root {
            ShapeRoot::Surface { surface, .. } => Some(Self { surface: surface.clone(), flipped: false }),
            ShapeRoot::Face(id) => {
                let face = shape.body.faces.get(*id)?;
                Some(Self { surface: shape.body.surfaces.get(face.surface)?.clone(), flipped: face.flipped })
            }
            _ => None,
        }
    }

    pub fn point(&self, u: f64, v: f64) -> [f64; 3] {
        axes(self.surface.eval(u, v))
    }

    /// 🧭️ The unit normal, turned the way a face points; `None` at a singular point.
    pub fn normal(&self, u: f64, v: f64) -> Option<[f64; 3]> {
        let normal = self.surface.normal(u, v)?;
        let sign = if self.flipped { -1.0 } else { 1.0 };
        Some([sign * normal.x, sign * normal.y, sign * normal.z])
    }

    /// 🎯️ The parameters, point and distance of the closest point of the surface to `point`.
    pub fn closest(&self, point: [f64; 3]) -> (f64, f64, [f64; 3], f64) {
        let found = closest_uv(&self.surface, self.surface.domain(), Pnt3::new(point[0], point[1], point[2]), 1e-9);
        (found.u, found.v, axes(found.point), found.distance)
    }
}
//#endregion 🔖️Surface

//#region 🔖️Inputs
const SHAPE_KIND: &str = "generation3d.geometry.shape-kind";

fn kind_fault(port: &str, en: &str, de: &str) -> WidgetFault {
    WidgetFault::new(SHAPE_KIND, format!("This input takes {en}."), format!("Dieser Eingang erwartet {de}.")).at(port)
}

/// 〰️ The curve source of a curve, edge or wire input.
pub fn curve_input(inputs: &WidgetInputs, port: &str) -> Result<CurveSource, WidgetFault> {
    CurveSource::of(inputs.shape(port)?).ok_or_else(|| kind_fault(port, "a curve, an edge or a wire", "eine Kurve, eine Kante oder einen Kantenzug"))
}

/// 🏄️ The surface source of a surface or face input.
pub fn surface_input(inputs: &WidgetInputs, port: &str) -> Result<SurfaceSource, WidgetFault> {
    SurfaceSource::of(inputs.shape(port)?).ok_or_else(|| kind_fault(port, "a surface or a face", "eine Fläche oder ein Flächenstück"))
}
//#endregion 🔖️Inputs

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
