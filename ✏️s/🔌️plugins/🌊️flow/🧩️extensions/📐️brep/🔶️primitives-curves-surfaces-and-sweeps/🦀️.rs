//! 🔶️ Primitives, curves, surfaces and sweeps: the primitive, curve, surface and sweep operators of the brep extension. See [`super`].

use super::*;

// #region 🔖️Primitives
geo_operation!(BoxPrim, "solid", |k, i| k.box_prim(read_channel_number(i, "width")?, read_channel_number(i, "depth")?, read_channel_number(i, "height")?));

geo_operation!(SpherePrim, "solid", |k, i| k.sphere_prim(read_channel_number(i, "radius")?));

geo_operation!(CylinderPrim, "solid", |k, i| k.cylinder_prim(read_channel_number(i, "radius")?, read_channel_number(i, "height")?));

geo_operation!(ConePrim, "solid", |k, i| k.cone_prim(read_channel_number(i, "radius")?, read_channel_number(i, "height")?));

geo_operation!(TorusPrim, "solid", |k, i| k.torus_prim(read_channel_number(i, "major")?, read_channel_number(i, "minor")?));

pub(super) struct ConvexHullPrim(pub(super) SessionCapture);

impl Operator for ConvexHullPrim {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
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
geo_operation!(LineCurve, "curve", |k, i| k.line_curve(read_xyz(i, "start")?, read_xyz(i, "end")?));

geo_operation!(CircleCurve, "curve", |k, i| k.circle_curve(read_xyz(i, "center")?, read_xyz(i, "normal")?, read_channel_number(i, "radius")?));

geo_operation!(ArcCurve, "curve", |k, i| k.arc_curve(read_xyz(i, "center")?, read_xyz(i, "normal")?, read_channel_number(i, "radius")?, read_channel_number(i, "startAngle")?, read_channel_number(i, "endAngle")?,));

geo_operation!(EllipseCurve, "curve", |k, i| k.ellipse_curve(read_xyz(i, "center")?, read_xyz(i, "normal")?, read_channel_number(i, "semiMajor")?, read_channel_number(i, "semiMinor")?,));

pub(super) struct PolylineWire(pub(super) SessionCapture);

impl Operator for PolylineWire {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let handle = kernel.polyline_wire(&points).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(RectangleWire, "wire", |k, i| k.rectangle_wire(read_channel_number(i, "width")?, read_channel_number(i, "height")?));

geo_operation!(RegularPolygonWire, "wire", |k, i| k.regular_polygon_wire(read_channel_number(i, "radius")?, read_channel_number(i, "sides")? as usize));

pub(super) struct InterpolateCurve(pub(super) SessionCapture);

impl Operator for InterpolateCurve {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
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
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
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

geo_operation!(HelixCurve, "curve", |k, i| k.helix_curve(read_xyz(i, "origin")?, read_xyz(i, "axis")?, read_channel_number(i, "radius")?, read_channel_number(i, "pitch")?, read_channel_number(i, "turns")?,));
// #endregion 🔖️Curves

// #region 🔖️Surfaces
geo_operation!(PlaneSurface, "surface", |k, i| k.plane_surface(read_xyz(i, "origin")?, read_xyz(i, "normal")?));

pub(super) struct PlanarFacePoints(pub(super) SessionCapture);

impl Operator for PlanarFacePoints {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let handle = kernel.planar_face_from_points(&points).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("face", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(PlanarFaceWire, "face", |k, i| k.planar_face_from_wire(&read_geometry(i, "wire")?));

pub(super) struct NurbsGridSurface(pub(super) SessionCapture);

impl Operator for NurbsGridSurface {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
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
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let curves = read_nested_point_lists(input, "curves")?;
            let handle = kernel.coons_patch(&curves).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("surface", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(OffsetFace, "faceOut", |k, i| k.offset_face(&read_geometry(i, "face")?, read_channel_number(i, "distance")?));

geo_operation!(ThickenFace, "solid", |k, i| k.thicken_face(&read_geometry(i, "face")?, read_channel_number(i, "thickness")?));
// #endregion 🔖️Surfaces

// #region 🔖️Sweeps
pub(super) struct ExtrudeCurve(pub(super) SessionCapture);

impl Operator for ExtrudeCurve {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
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
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
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

geo_operation!(Revolve, "solid", |k, i| k.revolve(&read_geometry(i, "face")?, read_xyz(i, "axisOrigin")?, read_xyz(i, "axisDirection")?, read_channel_number(i, "angle")?,));

geo_operation!(Sweep, "solid", |k, i| k.sweep(&read_geometry(i, "profile")?, &read_geometry(i, "path")?));

pub(super) struct Loft(pub(super) SessionCapture);

impl Operator for Loft {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
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
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
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
));
// #endregion 🔖️Sweeps
