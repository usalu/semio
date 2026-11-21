//! 🔶️ Primitives, curves, surfaces and sweeps: the primitive, curve, surface and sweep operators of the brep extension. See [`super`].

use super::*;

macro_rules! original_geometry {
    ($name:ident, $kind:ident, $channel:literal, |$kernel:ident, $input:ident| $evaluate:expr) => {
        pub(super) struct $name(pub(super) SessionCapture);

        impl Operator for $name {
            retire_geometry_capture!(0);

            fn evaluate(&self, $input: &Dictionary) -> Result<Dictionary, EvalError> {
                self.0.with_kernel(|$kernel| {
                    let curve = $evaluate.map_err(|error| map_kernel_error(&error))?;
                    Ok(channel_output($channel, geometry_dict($kernel, &curve)?))
                })
            }

            fn step_plan(&self, input: Dictionary, grant: semio_framework_value::RetainedCloneGrant) -> Result<(neural_engine::OperatorPlanAdmission, semio_framework_value::RetainedCloneProgress), (EvalError, Dictionary)> {
                super::original_curves::OriginalGeometryJob::admit(super::original_curves::GeometryOperation::$kind, &self.0, input, grant)
            }

            fn next_plan_copy_byte_demand(&self, _: &Dictionary) -> Result<usize, semio_framework_value::ValueError> { Ok(std::mem::size_of::<Session>()) }
            fn next_plan_capacity_byte_demand(&self, _: &Dictionary, _: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(std::mem::size_of::<super::original_curves::OriginalGeometryJob>()) }
            fn next_plan_release_byte_demand(&self, _: &Dictionary) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
            fn next_plan_depth_demand(&self, _: &Dictionary) -> Result<usize, semio_framework_value::ValueError> { Ok(1) }
        }
    };
}

// #region 🔖️Primitives
original_geometry!(BoxPrim, Box, "solid", |k, i| k.box_prim(read_channel_number(i, "width")?, read_channel_number(i, "depth")?, read_channel_number(i, "height")?));


original_geometry!(SpherePrim, Sphere, "solid", |k, i| k.sphere_prim(read_channel_number(i, "radius")?));

original_geometry!(CylinderPrim, Cylinder, "solid", |k, i| k.cylinder_prim(read_channel_number(i, "radius")?, read_channel_number(i, "height")?));

original_geometry!(ConePrim, Cone, "solid", |k, i| k.cone_prim(read_channel_number(i, "radius")?, read_channel_number(i, "height")?));

original_geometry!(TorusPrim, Torus, "solid", |k, i| k.torus_prim(read_channel_number(i, "major")?, read_channel_number(i, "minor")?));

pub(super) struct ConvexHullPrim(pub(super) SessionCapture);

impl Operator for ConvexHullPrim {
    retire_geometry_capture!(0);
    retained_geometry_job!(ConvexHull);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let handle = kernel.convex_hull(&points).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️Primitives

// #region 🔖️Curves

original_geometry!(LineCurve, Line, "curve", |k, i| k.line_curve(read_xyz(i, "start")?, read_xyz(i, "end")?));
original_geometry!(CircleCurve, Circle, "curve", |k, i| k.circle_curve(read_xyz(i, "center")?, read_xyz(i, "normal")?, read_channel_number(i, "radius")?));
original_geometry!(EllipseCurve, Ellipse, "curve", |k, i| k.ellipse_curve(read_xyz(i, "center")?, read_xyz(i, "normal")?, read_channel_number(i, "semiMajor")?, read_channel_number(i, "semiMinor")?));

geo_operation!(ArcCurve, "curve", |k, i| k.arc_curve(read_xyz(i, "center")?, read_xyz(i, "normal")?, read_channel_number(i, "radius")?, read_channel_number(i, "startAngle")?, read_channel_number(i, "endAngle")?,), Arc);

pub(super) struct PolylineWire(pub(super) SessionCapture);

impl Operator for PolylineWire {
    retire_geometry_capture!(0);
    retained_geometry_job!(Polyline);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let handle = kernel.polyline_wire(&points).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct RectangleWire(pub(super) SessionCapture);

impl Operator for RectangleWire {
    retire_geometry_capture!(0);
    retained_geometry_job!(Rectangle);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let handle = kernel.rectangle_wire(read_channel_number(input, "width")?, read_channel_number(input, "height")?).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct RegularPolygonWire(pub(super) SessionCapture);

impl Operator for RegularPolygonWire {
    retire_geometry_capture!(0);
    retained_geometry_job!(RegularPolygon);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let handle = kernel.regular_polygon_wire(read_channel_number(input, "radius")?, read_channel_number(input, "sides")? as usize).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct InterpolateCurve(pub(super) SessionCapture);

impl Operator for InterpolateCurve {
    retire_geometry_capture!(0);
    retained_geometry_job!(Interpolate);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let degree = read_channel_number(input, "degree")? as usize;
            let handle = kernel.interpolate_curve(&points, degree).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("curve", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct ApproximateCurve(pub(super) SessionCapture);

impl Operator for ApproximateCurve {
    retire_geometry_capture!(0);
    retained_geometry_job!(Approximate);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let degree = read_channel_number(input, "degree")? as usize;
            let control_points = read_channel_number(input, "controlPoints")? as usize;
            let handle = kernel.approximate_curve(&points, degree, control_points).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("curve", geometry_dict(kernel, &handle)?))
        })
    }
}

original_geometry!(HelixCurve, Helix, "curve", |k, i| k.helix_curve(
    read_xyz(i, "origin")?, read_xyz(i, "axis")?, read_channel_number(i, "radius")?,
    read_channel_number(i, "pitch")?, read_channel_number(i, "turns")?,
));
// #endregion 🔖️Curves

// #region 🔖️Surfaces
pub(super) struct PlaneSurface(pub(super) SessionCapture);

impl Operator for PlaneSurface {
    retire_geometry_capture!(0);
    retained_geometry_job!(Plane);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let origin = read_xyz(input, "origin")?;
            let normal = read_xyz(input, "normal")?;
            let surface = kernel.plane_surface(origin, normal).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("surface", geometry_dict(kernel, &surface)?))
        })
    }
}

pub(super) struct PlanarFacePoints(pub(super) SessionCapture);

impl Operator for PlanarFacePoints {
    retire_geometry_capture!(0);
    retained_geometry_job!(PlanarFacePoints);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let handle = kernel.planar_face_from_points(&points).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("face", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct PlanarFaceWire(pub(super) SessionCapture);

impl Operator for PlanarFaceWire {
    retire_geometry_capture!(0);
    retained_geometry_job!(PlanarConicFace);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let profile = read_geometry(input, "wire")?;
            let face = kernel.planar_face_from_wire(&profile).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("face", geometry_dict(kernel, &face)?))
        })
    }
}

pub(super) struct NurbsGridSurface(pub(super) SessionCapture);

impl Operator for NurbsGridSurface {
    retire_geometry_capture!(0);
    retained_geometry_job!(NurbsGrid);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let rows = read_channel_number(input, "rows")? as usize;
            let grid = points_to_grid(&points, rows)?;
            let degree_u = read_channel_number(input, "degreeU")? as usize;
            let degree_v = read_channel_number(input, "degreeV")? as usize;
            let handle = kernel.nurbs_surface_from_grid(&grid, degree_u, degree_v).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("surface", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct CoonsPatch(pub(super) SessionCapture);

impl Operator for CoonsPatch {
    retire_geometry_capture!(0);
    retained_geometry_job!(Coons);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let curves = read_nested_point_lists(input, "curves")?;
            let handle = kernel.coons_patch(&curves).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("surface", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(OffsetFace, "faceOut", |k, i| k.offset_face(&read_geometry(i, "face")?, read_channel_number(i, "distance")?), OffsetFace);

geo_operation!(ThickenFace, "solid", |k, i| k.thicken_face(&read_geometry(i, "face")?, read_channel_number(i, "thickness")?), Thicken);
// #endregion 🔖️Surfaces

// #region 🔖️Sweeps
pub(super) struct ExtrudeCurve(pub(super) SessionCapture);

impl Operator for ExtrudeCurve {
    retire_geometry_capture!(0);
    retained_geometry_job!(ExtrudeWire);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let wire = read_geometry(input, "wire")?;
            let vector = read_xyz(input, "vector")?;
            let handle = kernel.extrude_wire(&wire, vector).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct ExtrudeFace(pub(super) SessionCapture);

impl Operator for ExtrudeFace {
    retire_geometry_capture!(0);
    retained_geometry_job!(ExtrudeFace);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let face = read_geometry(input, "face")?;
            let vector = read_xyz(input, "vector")?;
            let distance = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
            if distance < 1e-12 {
                return Err(EvalError::InvalidInput("extrusion vector magnitude must be positive".into()));
            }
            let direction = [vector[0] / distance, vector[1] / distance, vector[2] / distance];
            let handle = kernel.extrude(&face, direction, distance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(Revolve, "solid", |k, i| k.revolve(&read_geometry(i, "face")?, read_xyz(i, "axisOrigin")?, read_xyz(i, "axisDirection")?, read_channel_number(i, "angle")?,), Revolve);

geo_operation!(Sweep, "solid", |k, i| k.sweep(&read_geometry(i, "profile")?, &read_geometry(i, "path")?), Sweep);

pub(super) struct Loft(pub(super) SessionCapture);

impl Operator for Loft {
    retire_geometry_capture!(0);
    retained_geometry_job!(Loft);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let profiles = read_geometry_list(input, "profiles")?;
            let smooth = read_channel_number(input, "smooth")? >= 0.5;
            let handle = kernel.loft(&profiles, smooth).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct Pipe(pub(super) SessionCapture);

impl Operator for Pipe {
    retire_geometry_capture!(0);
    retained_geometry_job!(Pipe);

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let profile = read_geometry(input, "profile")?;
            let path = read_geometry(input, "path")?;
            let guide_handle = read_optional_geometry(input, "guide");
            let guide = guide_handle.as_ref();
            let handle = kernel.pipe(&profile, &path, guide).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(HelicalSweep, "solid", |k, i| k.helical_sweep(
    &read_geometry(i, "profile")?,
    read_xyz(i, "axisOrigin")?,
    read_xyz(i, "axisDirection")?,
    read_channel_number(i, "radius")?,
    read_channel_number(i, "pitch")?,
    read_channel_number(i, "turns")?,
), HelicalSweep);
// #endregion 🔖️Sweeps
