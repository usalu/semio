//! 🔮 Ground truth used only by tests, kept deliberately independent from the kernel's own
//! algorithms (WFC-crate convention: a brute-force oracle catches bugs a self-referential test
//! never could). This module grows alongside the kernel — [`Sdf`] lands in Phase 0 with the
//! primitives it can already describe; mass-property, watertightness and shape-generator oracles
//! land in the phases that need them.

use crate::mat::Trsf;
use crate::vec::Pnt3;

// #region 🔖Sdf

/// 🔮 A closed-form signed distance field: negative inside, zero on the boundary, positive
/// outside. Used to probe classification and Boolean results independently of the kernel's own
/// ray-casting/arrangement code.
#[derive(Clone, Debug, PartialEq)]
pub enum Sdf {
    /// 🔮 Axis-aligned box of the given half-extents, centered at the origin before `placement`.
    Box { half_extents: Pnt3, placement: Trsf },
    /// 🔮 Sphere of the given radius, centered at the origin before `placement`.
    Sphere { radius: f64, placement: Trsf },
    /// 🔮 Cylinder of the given radius and half-height, axis along local `z`, centered at the
    /// origin before `placement`.
    Cylinder { radius: f64, half_height: f64, placement: Trsf },
    /// 🔮 Boolean combination of two fields.
    Union(Box<Sdf>, Box<Sdf>),
    Intersect(Box<Sdf>, Box<Sdf>),
    Difference(Box<Sdf>, Box<Sdf>),
}

impl Sdf {
    /// 🔮 Evaluates the field at a world-space point.
    pub fn eval(&self, p: Pnt3) -> f64 {
        match self {
            Sdf::Box { half_extents, placement } => {
                let local = placement.inverse().apply_point(p);
                let dx = local.x.abs() - half_extents.x;
                let dy = local.y.abs() - half_extents.y;
                let dz = local.z.abs() - half_extents.z;
                let outside = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2) + dz.max(0.0).powi(2)).sqrt();
                let inside = dx.max(dy).max(dz).min(0.0);
                outside + inside
            }
            Sdf::Sphere { radius, placement } => {
                let local = placement.inverse().apply_point(p);
                local.to_vec().norm() - radius
            }
            Sdf::Cylinder { radius, half_height, placement } => {
                let local = placement.inverse().apply_point(p);
                let radial = (local.x * local.x + local.y * local.y).sqrt() - radius;
                let axial = local.z.abs() - half_height;
                let outside = (radial.max(0.0).powi(2) + axial.max(0.0).powi(2)).sqrt();
                let inside = radial.max(axial).min(0.0);
                outside + inside
            }
            Sdf::Union(a, b) => a.eval(p).min(b.eval(p)),
            Sdf::Intersect(a, b) => a.eval(p).max(b.eval(p)),
            Sdf::Difference(a, b) => a.eval(p).max(-b.eval(p)),
        }
    }
    /// 🔮 `true` when `p` is inside (or on, within `tol`) the field's boundary.
    pub fn contains(&self, p: Pnt3, tol: f64) -> bool {
        self.eval(p) <= tol
    }
    pub fn union(self, other: Sdf) -> Sdf {
        Sdf::Union(Box::new(self), Box::new(other))
    }
    pub fn intersect(self, other: Sdf) -> Sdf {
        Sdf::Intersect(Box::new(self), Box::new(other))
    }
    pub fn difference(self, other: Sdf) -> Sdf {
        Sdf::Difference(Box::new(self), Box::new(other))
    }
}

// #endregion 🔖Sdf

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_sdf_is_negative_inside_and_positive_outside() {
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement: Trsf::IDENTITY };
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(5.0, 0.0, 0.0)) > 0.0);
        assert!((b.eval(Pnt3::new(1.0, 0.0, 0.0))).abs() < 1e-9);
    }

    #[test]
    fn sphere_sdf_matches_analytic_distance() {
        let s = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        assert!((s.eval(Pnt3::new(5.0, 0.0, 0.0)) - 3.0).abs() < 1e-9);
        assert!((s.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-2.0)).abs() < 1e-9);
    }

    #[test]
    fn cylinder_sdf_is_correct_on_axis_and_cap() {
        let c = Sdf::Cylinder { radius: 1.0, half_height: 2.0, placement: Trsf::IDENTITY };
        assert!((c.eval(Pnt3::new(0.0, 0.0, 0.0)) - (-1.0)).abs() < 1e-9);
        assert!((c.eval(Pnt3::new(0.0, 0.0, 5.0)) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn union_is_the_min_and_matches_containment_of_either_operand() {
        let a = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::vec::Vec3::new(-1.0, 0.0, 0.0)) };
        let b = Sdf::Sphere { radius: 1.0, placement: Trsf::translation(crate::vec::Vec3::new(1.0, 0.0, 0.0)) };
        let u = a.union(b);
        assert!(u.contains(Pnt3::new(-1.0, 0.0, 0.0), 1e-9));
        assert!(u.contains(Pnt3::new(1.0, 0.0, 0.0), 1e-9));
        assert!(!u.contains(Pnt3::new(5.0, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn difference_removes_the_second_operand() {
        let big = Sdf::Sphere { radius: 2.0, placement: Trsf::IDENTITY };
        let small = Sdf::Sphere { radius: 1.0, placement: Trsf::IDENTITY };
        let d = big.difference(small);
        assert!(!d.contains(Pnt3::new(0.0, 0.0, 0.0), 1e-9));
        assert!(d.contains(Pnt3::new(1.5, 0.0, 0.0), 1e-9));
    }

    #[test]
    fn placed_box_sdf_respects_transform() {
        let placement = Trsf::translation(crate::vec::Vec3::new(10.0, 0.0, 0.0));
        let b = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement };
        assert!(b.eval(Pnt3::new(10.0, 0.0, 0.0)) < 0.0);
        assert!(b.eval(Pnt3::new(0.0, 0.0, 0.0)) > 0.0);
    }
}
// #endregion 🔖Tests
