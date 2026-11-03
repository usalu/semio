//! 📏️ Measure tool: picked world points become distance, per-axis delta and, from three points, the included angle.

use super::pose::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeasureState {
    pub points: Vec<V3>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Measurement {
    pub distance: f64,
    pub delta: V3,
    pub horizontal: f64,
    pub angle_deg: Option<f64>,
}

impl MeasureState {
    /// ➕️ Adds a picked point; a fourth pick starts a new measurement.
    pub fn push(&self, point: V3) -> Self {
        let mut points = if self.points.len() >= 3 { Vec::new() } else { self.points.clone() };
        points.push(point);
        Self { points }
    }

    pub fn clear(&self) -> Self {
        Self::default()
    }

    pub fn measurement(&self) -> Option<Measurement> {
        let [a, b, ..] = self.points[..] else { return None };
        let delta = sub(b, a);
        let angle_deg = self.points.get(2).map(|c| {
            let (u, v) = (unit(sub(a, b)), unit(sub(*c, b)));
            dot(u, v).clamp(-1.0, 1.0).acos().to_degrees()
        });
        Some(Measurement { distance: len(delta), delta, horizontal: delta[0].hypot(delta[1]), angle_deg })
    }
}
