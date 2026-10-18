//! 🧲️ Features, intersect, evaluate and topology: the feature, intersection, curve and surface evaluation and topology operators. See [`super`].

use super::*;

// #region 🔖️Features
geo_operation!(Fillet, "solid", |k, i| k.fillet(&read_geometry(i, "geometry")?, read_channel_number(i, "radius")?));

geo_operation!(FilletVariable, "solid", |k, i| k.fillet_variable(&read_geometry(i, "geometry")?, read_channel_number(i, "radiusStart")?, read_channel_number(i, "radiusEnd")?,));

geo_operation!(Chamfer, "solid", |k, i| k.chamfer(&read_geometry(i, "geometry")?, read_channel_number(i, "distance")?));

geo_operation!(ChamferAsymmetric, "solid", |k, i| k.chamfer_asymmetric(&read_geometry(i, "geometry")?, read_channel_number(i, "d1")?, read_channel_number(i, "d2")?,));

// 🎯️ Selective-edge variants: fillet/chamfer only the given edges instead of the whole solid —
// avoids the full-solid edge-count cost when a user selects just one or a few edges.
pub(super) struct FilletEdges(pub(super) SessionCapture);

impl Operator for FilletEdges {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let edges = read_geometry_list(input, "edges")?;
            let radius = read_channel_number(input, "radius")?;
            let handle = kernel.fillet_edges(&geometry, &edges, radius).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct ChamferEdges(pub(super) SessionCapture);

impl Operator for ChamferEdges {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let edges = read_geometry_list(input, "edges")?;
            let distance = read_channel_number(input, "distance")?;
            let handle = kernel.chamfer_edges(&geometry, &edges, distance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct ShellMutation(pub(super) SessionCapture);

impl Operator for ShellMutation {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let thickness = read_channel_number(input, "thickness")?;
            let open_faces = read_geometry_list_or_empty(input, "openFaces")?;
            let handle = kernel.shell(&geometry, thickness, &open_faces).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct Draft(pub(super) SessionCapture);

impl Operator for Draft {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let faces = read_geometry_list(input, "faces")?;
            let handle = kernel.draft(&geometry, &faces, read_xyz(input, "pullDirection")?, read_xyz(input, "neutralPoint")?, read_channel_number(input, "angle")?).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(OffsetSolid, "solid", |k, i| k.offset_solid(&read_geometry(i, "geometry")?, read_channel_number(i, "distance")?));

pub(super) struct Defeature(pub(super) SessionCapture);

impl Operator for Defeature {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let faces = read_geometry_list(input, "faces")?;
            let handle = kernel.defeature(&geometry, &faces).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️Features

// #region 🔖️Intersect
/// 🍰️ Emits EVERY section face the plane produced, not just the first — a solid with multiple
/// disjoint cross-sections must not lose the rest silently (audit §13.2).
pub(super) struct Section(pub(super) SessionCapture);

impl Operator for Section {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let faces = kernel.section(&read_geometry(input, "solid")?, read_xyz(input, "planeOrigin")?, read_xyz(input, "planeNormal")?).map_err(|error| map_kernel_error(&error))?;
            if faces.is_empty() {
                return Err(EvalError::InvalidInput("section produced no faces".into()));
            }
            let list = geometry_list(kernel, faces)?;
            Ok(Dictionary::new().insert("faces", Value::Dictionary(list)))
        })
    }
}

/// ✂️ Emits BOTH halves the plane produced — the earlier implementation silently discarded the
/// negative half (audit §13.2's exact "continue after failure ... return a copied input" pattern,
/// here a copied-output-minus-half pattern).
pub(super) struct Split(pub(super) SessionCapture);

impl Operator for Split {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let (positive, negative) = kernel.split(&read_geometry(input, "solid")?, read_xyz(input, "planeOrigin")?, read_xyz(input, "planeNormal")?).map_err(|error| map_kernel_error(&error))?;
            Ok(Dictionary::new().insert("positive", Value::Dictionary(geometry_dict(kernel, &positive)?)).insert("negative", Value::Dictionary(geometry_dict(kernel, &negative)?)))
        })
    }
}

pub(super) struct CurveCurveIntersect(pub(super) SessionCapture);

impl Operator for CurveCurveIntersect {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = kernel.curve_curve_intersect(&read_geometry(input, "a")?, &read_geometry(input, "b")?, read_channel_number(input, "tolerance")?).map_err(|error| map_kernel_error(&error))?;
            let handle = wire_from_points(kernel, &points)?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

pub(super) struct CurveSurfaceIntersect(pub(super) SessionCapture);

impl Operator for CurveSurfaceIntersect {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let points = kernel.curve_surface_intersect(&read_geometry(input, "curve")?, &read_geometry(input, "surface")?, read_channel_number(input, "tolerance")?).map_err(|error| map_kernel_error(&error))?;
            let handle = wire_from_points(kernel, &points)?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

/// 〰️ Emits EVERY intersection wire (two surfaces can meet along several disjoint curves), not
/// just the first (audit §13.2).
pub(super) struct SurfaceSurfaceIntersect(pub(super) SessionCapture);

impl Operator for SurfaceSurfaceIntersect {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let wires = kernel.surface_surface_intersect(&read_geometry(input, "a")?, &read_geometry(input, "b")?, read_channel_number(input, "tolerance")?).map_err(|error| map_kernel_error(&error))?;
            if wires.is_empty() {
                return Err(EvalError::InvalidInput("no intersection wire".into()));
            }
            let list = geometry_list(kernel, wires)?;
            Ok(Dictionary::new().insert("wires", Value::Dictionary(list)))
        })
    }
}
// #endregion 🔖️Intersect

// #region 🔖️Evaluate
point_operation!(CurvePoint, "point", |k, i| k.curve_point(&read_geometry(i, "curve")?, read_channel_number(i, "parameter")?));

vec_operation!(CurveTangent, "tangent", |k, i| k.curve_tangent(&read_geometry(i, "curve")?, read_channel_number(i, "parameter")?));

pub(super) struct CurveDomain(pub(super) SessionCapture);

impl Operator for CurveDomain {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let domain = kernel.curve_domain(&read_geometry(input, "curve")?).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("span", number_dictionary(domain_span(domain))))
        })
    }
}

num_operation!(CurveCurvature, "curvature", |k, i| k.curve_curvature(&read_geometry(i, "curve")?, read_channel_number(i, "parameter")?));

point_operation!(SurfacePoint, "point", |k, i| k.surface_point(&read_geometry(i, "surface")?, read_channel_number(i, "u")?, read_channel_number(i, "v")?));

vec_operation!(SurfaceNormal, "normal", |k, i| k.surface_normal(&read_geometry(i, "surface")?, read_channel_number(i, "u")?, read_channel_number(i, "v")?));

/// 🎯️ Certified nearest parameter on a curve — `curve_closest_parameter` exposes the achieved
/// `distance` alongside the point/parameter so callers can tell a converged fit from a coarse one,
/// per audit §13.2 ("achieved tolerance/error" is part of an operation's honest result).
pub(super) struct CurveClosestParameter(pub(super) SessionCapture);

impl Operator for CurveClosestParameter {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let (parameter, point, distance) = kernel.curve_closest_parameter(&read_geometry(input, "curve")?, read_xyz(input, "point")?).map_err(|error| map_kernel_error(&error))?;
            Ok(Dictionary::new().insert("parameter", Value::Dictionary(number_dictionary(parameter))).insert("pointOut", Value::Dictionary(point_dictionary(point))).insert("distance", Value::Dictionary(number_dictionary(distance))))
        })
    }
}

/// 🎯️ Certified nearest `(u, v)` on a surface — see [`CurveClosestParameter`]'s docstring.
pub(super) struct SurfaceClosestUv(pub(super) SessionCapture);

impl Operator for SurfaceClosestUv {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let (u, v, point, distance) = kernel.surface_closest_uv(&read_geometry(input, "surface")?, read_xyz(input, "point")?).map_err(|error| map_kernel_error(&error))?;
            Ok(Dictionary::new()
                .insert("u", Value::Dictionary(number_dictionary(u)))
                .insert("v", Value::Dictionary(number_dictionary(v)))
                .insert("pointOut", Value::Dictionary(point_dictionary(point)))
                .insert("distance", Value::Dictionary(number_dictionary(distance))))
        })
    }
}
// #endregion 🔖️Evaluate

// #region 🔖️Topology
/// 📇️ A `geometry`-schema list, each entry carrying its own live [`GeometryKind`] (via
/// `geometry_dict`) — unlike [`topology_list`], which hardcodes one fixed schema/kind for the
/// vertex/edge/face lists it was built for, this never mislabels a shell as a `"solid"`.
fn geometry_list(kernel: &Brep, handles: Vec<GeometryHandle>) -> Result<Dictionary, EvalError> {
    handles.into_iter().enumerate().try_fold(Dictionary::with_schema("list"), |list, (index, handle)| Ok(list.insert(index.to_string(), Value::Dictionary(geometry_dict(kernel, &handle)?))))
}

/// 🐚️ The solid's shells as independent geometry handles — `solid_shells` never silently fuses
/// or drops inner voids/cavities, one output entry per shell.
pub(super) struct SolidShells(pub(super) SessionCapture);

impl Operator for SolidShells {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let shells = kernel.solid_shells(&read_geometry(input, "solid")?).map_err(|error| map_kernel_error(&error))?;
            let list = geometry_list(kernel, shells)?;
            Ok(Dictionary::new().insert("shells", Value::Dictionary(list)))
        })
    }
}

pub(super) struct CompoundOf(pub(super) SessionCapture);

impl Operator for CompoundOf {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let solids = read_geometry_list(input, "solids")?;
            let handle = kernel.compound(&solids).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("compound", geometry_dict(kernel, &handle)?))
        })
    }
}

/// 💥️ Inverse of [`CompoundOf`] — every member solid as its own handle, none silently merged.
pub(super) struct Explode(pub(super) SessionCapture);

impl Operator for Explode {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let solids = kernel.explode(&read_geometry(input, "compound")?).map_err(|error| map_kernel_error(&error))?;
            let list = geometry_list(kernel, solids)?;
            Ok(Dictionary::new().insert("solids", Value::Dictionary(list)))
        })
    }
}

/// 🏷️ The handle's persistent label as exact decimal text across the native and browser boundary.
pub(super) struct GeometryLabel(pub(super) SessionCapture);

impl Operator for GeometryLabel {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let handle = read_geometry(input, "geometry")?;
            let label = kernel.label(&handle).ok_or_else(|| EvalError::InvalidInput(format!("geometry {} carries no persistent label", handle.as_str())))?;
            Ok(channel_output("label", text_dictionary(label.to_string())))
        })
    }
}
// #endregion 🔖️Topology
