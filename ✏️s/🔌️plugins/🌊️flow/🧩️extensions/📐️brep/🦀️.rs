//! 🔷️ Flow brep extension — geometry operators packaged as a runtime-installable unit.

use semio_s_spatial_kernel_semio_session::*;
use flow_extension_sdk::mesh::*;
use flow_extension_sdk::build_manifest_json;
use neural_engine::{channel_output, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};
use semio_framework_3d::brep::operations::boolean::BooleanOp;
use semio_framework_3d::brep::engine::{BrepBooleanAdmission, BrepBooleanJob, BrepBooleanStep, BrepKernel};

/// 🎯️ Appends a node's live [`OpQuality`] (looked up by the `BrepKernel` method it wraps) to a
/// human-readable summary, so both `register()`'s catalogue and the packaged `🔣️.json` descriptor
/// carry the same fidelity the kernel contract declares — audit §13.1/§13.2: no node may imply
/// exactness it does not have. [`operation_quality_tags_match_the_kernel_contract`] pins this
/// against [`NODE_KERNEL_METHOD`].
fn q(method: &str, summary: &str) -> String {
    format!("{summary} [quality:{:?}]", operation_quality(method))
}

/// 📇️ Every registered flow node's operator id mapped to the `BrepKernel` method it wraps — the
/// single source of truth both metadata tests below check against. Hand-maintained, not
/// reflectively derived (mirrors W1-A's `OPERATION_QUALITY` table's own documented rationale):
/// whoever adds a node must add its row here, or `operation_quality_tags_match_the_kernel_contract`
/// fails.
const NODE_KERNEL_METHOD: &[(&str, &str)] = &[
    ("brep.brep", "deconstruct"),
    ("brep.prim3d.box", "box_prim"),
    ("brep.prim3d.sphere", "sphere_prim"),
    ("brep.prim3d.cylinder", "cylinder_prim"),
    ("brep.prim3d.cone", "cone_prim"),
    ("brep.prim3d.torus", "torus_prim"),
    ("brep.prim3d.convexHull", "convex_hull"),
    ("brep.curve.line", "line_curve"),
    ("brep.curve.circle", "circle_curve"),
    ("brep.curve.arc", "arc_curve"),
    ("brep.curve.ellipse", "ellipse_curve"),
    ("brep.curve.polyline", "polyline_wire"),
    ("brep.curve.rectangle", "rectangle_wire"),
    ("brep.curve.polygon", "regular_polygon_wire"),
    ("brep.curve.interpolate", "interpolate_curve"),
    ("brep.curve.approximate", "approximate_curve"),
    ("brep.curve.helix", "helix_curve"),
    ("brep.surf.plane", "plane_surface"),
    ("brep.surf.planarFace", "planar_face_from_points"),
    ("brep.surf.planarFaceWire", "planar_face_from_wire"),
    ("brep.surf.nurbsGrid", "nurbs_surface_from_grid"),
    ("brep.surf.coons", "coons_patch"),
    ("brep.surf.offset", "offset_face"),
    ("brep.surf.thicken", "thicken_face"),
    ("brep.solid.extrude", "extrude_wire"),
    ("brep.sweep.extrude", "extrude"),
    ("brep.sweep.revolve", "revolve"),
    ("brep.sweep.loft", "loft"),
    ("brep.sweep.sweep", "sweep"),
    ("brep.sweep.pipe", "pipe"),
    ("brep.sweep.helical", "helical_sweep"),
    ("brep.bool.fuse", "fuse"),
    ("brep.bool.cut", "cut"),
    ("brep.bool.intersect", "intersect"),
    ("brep.bool.compoundCut", "compound_cut"),
    ("brep.xform.translate", "translate"),
    ("brep.xform.rotate", "rotate"),
    ("brep.xform.rotateAbout", "rotate_about"),
    ("brep.xform.scale", "scale_axes"),
    ("brep.xform.mirror", "mirror"),
    ("brep.xform.copy", "copy_shape"),
    ("brep.xform.linearPattern", "linear_pattern"),
    ("brep.xform.circularPattern", "circular_pattern"),
    ("brep.xform.gridPattern", "grid_pattern"),
    ("brep.solid.fillet", "fillet"),
    ("brep.solid.filletVariable", "fillet_variable"),
    ("brep.solid.chamfer", "chamfer"),
    ("brep.solid.chamferAsymmetric", "chamfer_asymmetric"),
    ("brep.solid.filletEdges", "fillet_edges"),
    ("brep.solid.chamferEdges", "chamfer_edges"),
    ("brep.solid.shell", "shell"),
    ("brep.solid.draft", "draft"),
    ("brep.solid.offsetSolid", "offset_solid"),
    ("brep.solid.defeature", "defeature"),
    ("brep.intersect.section", "section"),
    ("brep.intersect.split", "split"),
    ("brep.intersect.curveCurve", "curve_curve_intersect"),
    ("brep.intersect.curveSurface", "curve_surface_intersect"),
    ("brep.intersect.surfaceSurface", "surface_surface_intersect"),
    ("brep.eval.curvePoint", "curve_point"),
    ("brep.eval.curveTangent", "curve_tangent"),
    ("brep.eval.curveDomain", "curve_domain"),
    ("brep.eval.curveCurvature", "curve_curvature"),
    ("brep.eval.surfPoint", "surface_point"),
    ("brep.eval.surfNormal", "surface_normal"),
    ("brep.eval.curveClosestParameter", "curve_closest_parameter"),
    ("brep.eval.surfaceClosestUv", "surface_closest_uv"),
    ("brep.measure.volume", "volume"),
    ("brep.measure.area", "area"),
    ("brep.measure.length", "length"),
    ("brep.measure.centerOfMass", "center_of_mass"),
    ("brep.measure.boundingBox", "bounding_box"),
    ("brep.measure.distance", "distance"),
    ("brep.measure.closestPoint", "closest_point"),
    ("brep.measure.classify", "classify_point"),
    ("brep.measure.validate", "validate"),
    ("brep.util.vertex", "vertex"),
    ("brep.util.faceFromWire", "face_from_wire"),
    ("brep.util.sew", "sew_faces"),
    ("brep.util.heal", "heal_solid"),
    ("brep.util.convertToNurbs", "convert_to_nurbs"),
    ("brep.topology.shells", "solid_shells"),
    ("brep.topology.compound", "compound"),
    ("brep.topology.explode", "explode"),
    ("brep.topology.label", "label"),
    ("brep.io.exportStep", "export_step"),
    ("brep.io.exportStl", "export_stl"),
    ("brep.io.exportObj", "export_obj"),
    ("brep.io.importStep", "import_step"),
    ("brep.io.importStl", "import_stl"),
    ("brep.io.importObj", "import_obj"),
    ("brep.io.exportDwg", "export_mesh"),
    ("brep.io.importDwg", "import_mesh"),
];

/// 🙈️ `BrepKernel` methods this extension deliberately exposes NO node for, with why — checked by
/// `every_kernel_operation_is_either_a_node_or_explicitly_unexposed` against
/// [`BREP_KERNEL_OPERATIONS`] so a newly added trait method can never silently fall through both
/// lists unnoticed.
#[cfg(test)]
const INTENTIONALLY_UNEXPOSED: &[(&str, &str)] = &[
    ("scale", "Uniform scaling is represented by equal factors in the scale_axes widget"),
    ("kind", "internal handle-kind lookup behind geometry_dict, not a graph operation"),
    ("tessellate", "internal preview/export bridge (tessellate_geometry), not a graph node"),
    ("dispose", "internal GC primitive (dispose_geometry), not a graph node"),
    ("retain", "internal GC primitive (retain_geometry_handles), not a graph node"),
    ("registry_len", "internal diagnostic counter, not a graph node"),
    ("export_gltf", "glTF/GLB leaves the extension only via the tessellation mesh bridge (export_solid_json \"glb\"), never this trait method directly"),
];

macro_rules! retire_geometry_capture {
    ($field:tt) => {
        fn retirement_is_empty(&self) -> bool { self.$field.terminal_is_empty() }
        fn retire_step(&mut self, grant:semio_framework_value::retained_clone::RetainedCloneGrant, _: &mut neural_engine::ValueRetirement) -> Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError> { self.$field.close_step(grant) }
        fn next_retire_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {self.$field.next_close_copy_byte_demand()}
        fn next_retire_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError> {self.$field.next_close_capacity_byte_demand(copy)}
        fn next_retire_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {self.$field.next_close_release_byte_demand()}
        fn next_retire_depth_demand(&self)->Result<usize,semio_framework_value::ValueError> {self.$field.next_close_depth_demand()}
        fn retire_cold(mut self:Box<Self>) { self.$field.retire_cold(); }
    };
}

macro_rules! geo_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name(SessionCapture);
        impl Operator for $name {
            fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
            fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                self.0.with_kernel(|$k| {
                    let $i = input;
                    let handle = $expr.map_err(|error| map_kernel_error(&error))?;
                    Ok(channel_output($channel, geometry_dict($k, &handle)?))
                })
            }
        }
    };
}

// 🔓️ `num_operation!`/`point_operation!`/`vec_operation!`/`text_operation!` back exclusively `&self` `BrepKernel` trait
// methods (volume/area/length/center_of_mass/distance/curve_point/curve_tangent/curve_domain/
// curve_curvature/surface_point/surface_normal/validate) — safe to route through the read lock.
macro_rules! num_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name(SessionCapture);
        impl Operator for $name {
            fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
            fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                self.0.with_kernel_read(|$k| {
                    let $i = input;
                    let value = $expr.map_err(|error| map_kernel_error(&error))?;
                    Ok(channel_output($channel, number_dictionary(value)))
                })
            }
        }
    };
}

macro_rules! point_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name(SessionCapture);
        impl Operator for $name {
            fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
            fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                self.0.with_kernel_read(|$k| {
                    let $i = input;
                    let value = $expr.map_err(|error| map_kernel_error(&error))?;
                    Ok(channel_output($channel, point_dictionary(value)))
                })
            }
        }
    };
}

macro_rules! vec_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name(SessionCapture);
        impl Operator for $name {
            fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
            fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                self.0.with_kernel_read(|$k| {
                    let $i = input;
                    let value = $expr.map_err(|error| map_kernel_error(&error))?;
                    Ok(channel_output($channel, vector_dictionary(value)))
                })
            }
        }
    };
}

macro_rules! text_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name(SessionCapture);
        impl Operator for $name {
            fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
            fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
            fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                self.0.with_kernel_read(|$k| {
                    let $i = input;
                    let value = $expr.map_err(|error| map_kernel_error(&error))?;
                    Ok(channel_output($channel, text_dictionary(value)))
                })
            }
        }
    };
}

// #region 🔖️Primitives
geo_operation!(BoxPrim, "solid", |k, i| k.box_prim(read_channel_number(i, "width")?, read_channel_number(i, "depth")?, read_channel_number(i, "height")?));
geo_operation!(SpherePrim, "solid", |k, i| k.sphere_prim(read_channel_number(i, "radius")?));
geo_operation!(CylinderPrim, "solid", |k, i| k.cylinder_prim(read_channel_number(i, "radius")?, read_channel_number(i, "height")?));
geo_operation!(ConePrim, "solid", |k, i| k.cone_prim(read_channel_number(i, "radius")?, read_channel_number(i, "height")?));
geo_operation!(TorusPrim, "solid", |k, i| k.torus_prim(read_channel_number(i, "major")?, read_channel_number(i, "minor")?));

struct ConvexHullPrim(SessionCapture);
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

struct PolylineWire(SessionCapture);
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

struct InterpolateCurve(SessionCapture);
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

struct ApproximateCurve(SessionCapture);
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

struct PlanarFacePoints(SessionCapture);
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

struct NurbsGridSurface(SessionCapture);
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

struct CoonsPatch(SessionCapture);
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
struct ExtrudeCurve(SessionCapture);
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

struct ExtrudeFace(SessionCapture);
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

struct Loft(SessionCapture);
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

struct Pipe(SessionCapture);
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

// #region 🔖️Booleans
/// ⏱️ BRep set operations retain kernel plans across evaluator turns; mesh modeling jobs use the same domain-neutral scheduler.
macro_rules! boolean_operation {
    ($name:ident, $op:expr) => {
        struct $name(SessionCapture);
        impl Operator for $name {
            retire_geometry_capture!(0);
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                self.0.with_kernel(|kernel| {
                    let handle = kernel.boolean_sync(&read_geometry(input, "a")?, &read_geometry(input, "b")?, $op).map_err(|error| map_kernel_error(&error))?;
                    Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
                })
            }

            fn step_plan(&self, input: Dictionary, grant: semio_framework_value::RetainedCloneGrant) -> Result<(neural_engine::OperatorPlanAdmission, semio_framework_value::RetainedCloneProgress), (EvalError, Dictionary)> {
                BrepBooleanOperatorJob::admit(&self.0, input, grant, $op)
            }
            fn next_plan_copy_byte_demand(&self, _input: &Dictionary) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
            fn next_plan_capacity_byte_demand(&self, _input: &Dictionary, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(BrepBooleanOperatorJob::plan_capacity()) }
            fn next_plan_release_byte_demand(&self, _input: &Dictionary) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
            fn next_plan_depth_demand(&self, _input: &Dictionary) -> Result<usize, semio_framework_value::ValueError> { Ok(1) }
        }
    };
}

boolean_operation!(Fuse, BooleanOp::Unite);
boolean_operation!(Cut, BooleanOp::Cut);
boolean_operation!(Intersect, BooleanOp::Intersect);

/// ⏱️ One set operation as a budgeted, resumable [`neural_engine::OperatorJob`]. Admission runs the
/// kernel's microsecond-cheap fast paths eagerly (they mutate the body and produce the result in
/// place, so they can never be re-run later) and retains their answer; anything else becomes a
/// retained [`BrepBooleanJob`] the steps advance.
///
/// 🔒️ Every step re-acquires the process-global kernel lock through [`with_kernel`] and hands the
/// job the body again, which is what makes the job retainable across host turns: it borrows
/// nothing.
struct BrepBooleanOperatorJob {
    session: Session,
    job: Option<BrepBooleanJob>,
    answered: Option<GeometryHandle>,
    value_retirement: neural_engine::ValueRetirement,
    receipt: semio_framework_value::RetainedCloneProgress,
    cancelled: bool,
    progress: neural_engine::OperatorProgress,
}

impl BrepBooleanOperatorJob {
    fn plan_capacity() -> usize {
        std::mem::size_of::<BrepBooleanOperatorJob>().saturating_add(neural_engine::ValueRetirement::domain_frame_birth_bytes())
    }

    /// 🔀️ Admits one set operation. Always answers a job: even a fast-path result comes back as a
    /// job, because admission has already mutated the body and an immediate plan would make the
    /// caller evaluate the whole boolean a second time. The original input moves into the job's
    /// own value retirement.
    fn admit(session: &Session, input: Dictionary, grant: semio_framework_value::RetainedCloneGrant, op: BooleanOp) -> Result<(neural_engine::OperatorPlanAdmission, semio_framework_value::RetainedCloneProgress), (EvalError, Dictionary)> {
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_capacity_bytes < Self::plan_capacity() {
            return Err((EvalError::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "boolean plan admission requires one item and its job frame")), input));
        }
        let (a, b) = match (read_geometry(&input, "a"), read_geometry(&input, "b")) {
            (Ok(a), Ok(b)) => (a, b),
            (Err(error), _) | (_, Err(error)) => return Err((error, input)),
        };
        let admitted = session.with_kernel(|kernel| {
            let admission = kernel.boolean_job_sync(&a, &b, op).map_err(|error| map_kernel_error(&error))?;
            Ok(match admission {
                BrepBooleanAdmission::Answered(handle) => BrepBooleanOperatorJob { session: session.clone(), job: None, answered: Some(handle), value_retirement: Default::default(), receipt: Default::default(), cancelled: false, progress: neural_engine::OperatorProgress { units_done: 1, units_total: 1, phase: "complete" } },
                BrepBooleanAdmission::Job(job) => {
                    let progress = job.progress();
                    BrepBooleanOperatorJob { session: session.clone(), job: Some(job), answered: None, value_retirement: Default::default(), receipt: Default::default(), cancelled: false, progress: neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase.tag() } }
                }
            })
        });
        let mut job = match admitted {
            Ok(job) => job,
            Err(error) => return Err((error, input)),
        };
        let child = semio_framework_value::RetainedCloneGrant { maximum_items: 1, ..grant };
        match job.value_retirement.push_dictionary(input, child) {
            Ok(progress) => Ok((neural_engine::OperatorPlanAdmission::Job(Box::new(job)), semio_framework_value::RetainedCloneProgress { retained_capacity_bytes: progress.retained_capacity_bytes.saturating_add(std::mem::size_of::<BrepBooleanOperatorJob>()), ..progress })),
            Err((error, input)) => {
                neural_engine::OperatorJob::cancel(&mut job);
                job.job = None;
                job.answered = None;
                Err((EvalError::from(error), input))
            }
        }
    }

    /// 📦️ The out dictionary a finished boolean produces — identical to the one-shot operator's.
    fn output(session: &Session, handle: &GeometryHandle) -> Result<Dictionary, EvalError> {
        session.with_kernel(|kernel| Ok(channel_output("solid", geometry_dict(kernel, handle)?)))
    }

    fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.answered.is_none() && self.value_retirement.terminal_is_empty()
    }
}

impl neural_engine::OperatorJob for BrepBooleanOperatorJob {
    fn step(&mut self, budget: usize, _grant: semio_framework_value::RetainedCloneGrant) -> Result<(neural_engine::OperatorJobStep, semio_framework_value::RetainedCloneProgress), EvalError> {
        self.receipt = Default::default();
        let step = self.advance(budget)?;
        Ok((step, self.receipt))
    }

    fn normal_step_progress(&self) -> semio_framework_value::RetainedCloneProgress {
        self.receipt
    }

    fn next_step_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_step_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_step_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_step_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(1) }

    fn progress(&self) -> neural_engine::OperatorProgress {
        self.progress
    }

    fn cancel(&mut self) {
        if self.answered.is_some() {
            return;
        }
        self.cancelled = true;
        if let Some(job) = self.job.as_mut() {
            job.cancel();
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if self.job.is_some() || self.answered.is_some() { return Ok(0); }
        self.value_retirement.next_copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        if self.job.is_some() || self.answered.is_some() { return Ok(0); }
        self.value_retirement.next_capacity_byte_demand(maximum_copy_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if self.job.is_some() || self.answered.is_some() { return Ok(0); }
        self.value_retirement.next_release_byte_demand()
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if self.job.is_some() || self.answered.is_some() { return Ok(1); }
        self.value_retirement.next_depth_demand()
    }

    fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.job.take().is_some() || self.answered.take().is_some() {
            self.cancelled = true;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let step = self.value_retirement.close_step(semio_framework_value::RetainedCloneGrant { maximum_items: 1, ..grant })?;
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    fn terminal_is_empty(&self) -> bool {
        BrepBooleanOperatorJob::terminal_is_empty(self)
    }
}

impl BrepBooleanOperatorJob {
    fn advance(&mut self, budget: usize) -> Result<neural_engine::OperatorJobStep, EvalError> {
        if self.cancelled {
            return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress));
        }
        if let Some(handle) = self.answered.clone() {
            return Ok(neural_engine::OperatorJobStep::Done(Self::output(&self.session, &handle)?));
        }
        let Some(job) = self.job.as_mut() else {
            return Err(EvalError::InvalidInput("boolean job carries neither an answer nor a plan".to_string()));
        };
        let step = self.session.with_kernel(|kernel| kernel.step_boolean_job_sync(job, budget.max(1)).map_err(|error| map_kernel_error(&error)))?;
        match step {
            BrepBooleanStep::Working(progress) => {
                self.progress = neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase.tag() };
                Ok(neural_engine::OperatorJobStep::Working(self.progress))
            }
            BrepBooleanStep::Cancelled(progress) => {
                self.cancelled = true;
                self.progress = neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase.tag() };
                Ok(neural_engine::OperatorJobStep::Cancelled(self.progress))
            }
            BrepBooleanStep::Ready(handle) => {
                self.job = None;
                self.answered = Some(handle.clone());
                self.progress = neural_engine::OperatorProgress { units_done: self.progress.units_total.max(self.progress.units_done), units_total: self.progress.units_total.max(self.progress.units_done), phase: "complete" };
                Ok(neural_engine::OperatorJobStep::Done(Self::output(&self.session, &handle)?))
            }
        }
    }
}

struct CompoundCut(SessionCapture);
impl Operator for CompoundCut {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let target = read_geometry(input, "target")?;
            let tools = read_geometry_list(input, "tools")?;
            let handle = kernel.compound_cut(&target, &tools).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️Booleans

// #region 🔖️Transforms
geo_operation!(Translate, "geometryOut", |k, i| k.translate(&read_geometry(i, "geometry")?, read_xyz(i, "offset")?));
geo_operation!(Rotate, "geometryOut", |k, i| k.rotate(&read_geometry(i, "geometry")?, read_xyz(i, "axis")?, read_channel_number(i, "angle")?));
geo_operation!(RotateAbout, "geometryOut", |k, i| k.rotate_about(&read_geometry(i, "geometry")?, read_xyz(i, "origin")?, read_xyz(i, "axis")?, read_channel_number(i, "angle")?));
geo_operation!(Scale, "geometryOut", |k, i| k.scale_axes(&read_geometry(i, "geometry")?, read_xyz(i, "factor")?, read_xyz(i, "center")?));
geo_operation!(Mirror, "geometryOut", |k, i| k.mirror(&read_geometry(i, "geometry")?, read_xyz(i, "origin")?, read_xyz(i, "normal")?));
geo_operation!(CopyShape, "geometryOut", |k, i| k.copy_shape(&read_geometry(i, "geometry")?));
geo_operation!(LinearPattern, "compound", |k, i| k.linear_pattern(&read_geometry(i, "geometry")?, read_xyz(i, "direction")?, read_channel_number(i, "spacing")?, read_channel_number(i, "count")? as usize,));
geo_operation!(CircularPattern, "compound", |k, i| k.circular_pattern(&read_geometry(i, "geometry")?, read_xyz(i, "axis")?, read_channel_number(i, "count")? as usize,));
geo_operation!(GridPattern, "compound", |k, i| k.grid_pattern(
    &read_geometry(i, "geometry")?,
    read_xyz(i, "dirX")?,
    read_xyz(i, "dirY")?,
    read_channel_number(i, "spacingX")?,
    read_channel_number(i, "spacingY")?,
    read_channel_number(i, "countX")? as usize,
    read_channel_number(i, "countY")? as usize,
));
// #endregion 🔖️Transforms

// #region 🔖️Features
geo_operation!(Fillet, "solid", |k, i| k.fillet(&read_geometry(i, "geometry")?, read_channel_number(i, "radius")?));
geo_operation!(FilletVariable, "solid", |k, i| k.fillet_variable(&read_geometry(i, "geometry")?, read_channel_number(i, "radiusStart")?, read_channel_number(i, "radiusEnd")?,));
geo_operation!(Chamfer, "solid", |k, i| k.chamfer(&read_geometry(i, "geometry")?, read_channel_number(i, "distance")?));
geo_operation!(ChamferAsymmetric, "solid", |k, i| k.chamfer_asymmetric(&read_geometry(i, "geometry")?, read_channel_number(i, "d1")?, read_channel_number(i, "d2")?,));

// 🎯️ Selective-edge variants: fillet/chamfer only the given edges instead of the whole solid —
// avoids the full-solid edge-count cost when a user selects just one or a few edges.
struct FilletEdges(SessionCapture);
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

struct ChamferEdges(SessionCapture);
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

struct ShellMutation(SessionCapture);
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

struct Draft(SessionCapture);
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

struct Defeature(SessionCapture);
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
struct Section(SessionCapture);
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
struct Split(SessionCapture);
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

struct CurveCurveIntersect(SessionCapture);
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

struct CurveSurfaceIntersect(SessionCapture);
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
struct SurfaceSurfaceIntersect(SessionCapture);
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

struct CurveDomain(SessionCapture);
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
struct CurveClosestParameter(SessionCapture);
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
struct SurfaceClosestUv(SessionCapture);
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
use semio_framework_3d::brep::engine::{Brep, GeometryHandle};

/// 📇️ A `geometry`-schema list, each entry carrying its own live [`GeometryKind`] (via
/// `geometry_dict`) — unlike [`topology_list`], which hardcodes one fixed schema/kind for the
/// vertex/edge/face lists it was built for, this never mislabels a shell as a `"solid"`.
fn geometry_list(kernel: &Brep, handles: Vec<GeometryHandle>) -> Result<Dictionary, EvalError> {
    handles.into_iter().enumerate().try_fold(Dictionary::with_schema("list"), |list, (index, handle)| Ok(list.insert(index.to_string(), Value::Dictionary(geometry_dict(kernel, &handle)?))))
}

/// 🐚️ The solid's shells as independent geometry handles — `solid_shells` never silently fuses
/// or drops inner voids/cavities, one output entry per shell.
struct SolidShells(SessionCapture);
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

struct CompoundOf(SessionCapture);
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
struct Explode(SessionCapture);
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
struct GeometryLabel(SessionCapture);
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

// #region 🔖️Measure
num_operation!(Volume, "volume", |k, i| k.volume(&read_geometry(i, "geometry")?));
num_operation!(Area, "area", |k, i| k.area(&read_geometry(i, "geometry")?));
num_operation!(Length, "length", |k, i| k.length(&read_geometry(i, "geometry")?));
point_operation!(CenterOfMass, "center", |k, i| k.center_of_mass(&read_geometry(i, "geometry")?));
geo_operation!(BoundingBox, "box", |k, i| k.bounding_box(&read_geometry(i, "geometry")?));
num_operation!(Distance, "distance", |k, i| k.distance(&read_geometry(i, "a")?, &read_geometry(i, "b")?));

struct ClosestPoint(SessionCapture);
impl Operator for ClosestPoint {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let result = kernel.closest_point(&read_geometry(input, "geometry")?, read_xyz(input, "point")?).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("pointOut", point_dictionary(result.point)))
        })
    }
}

struct ClassifyPoint(SessionCapture);
impl Operator for ClassifyPoint {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let classification = kernel.classify_point(&read_geometry(input, "solid")?, read_xyz(input, "point")?).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("classification", number_dictionary(classify_number(classification))))
        })
    }
}

text_operation!(Validate, "report", |k, i| k.validate(&read_geometry(i, "geometry")?));
// #endregion 🔖️Measure

// #region 🔖️Utilities
geo_operation!(Vertex, "vertex", |k, i| k.vertex(read_xyz(i, "point")?));
geo_operation!(FaceFromWire, "face", |k, i| k.face_from_wire(&read_geometry(i, "wire")?));

struct SewFaces(SessionCapture);
impl Operator for SewFaces {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let faces = read_geometry_list(input, "faces")?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.sew_faces(&faces, tolerance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(HealSolid, "solid", |k, i| k.heal_solid(&read_geometry(i, "geometry")?, read_channel_number(i, "tolerance")?));
geo_operation!(ConvertToNurbs, "geometryOut", |k, i| k.convert_to_nurbs(&read_geometry(i, "geometry")?));
// #endregion 🔖️Utilities

// #region 🔖️IO
struct ExportStep(SessionCapture);
impl Operator for ExportStep {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let value = semio_s_artifact_stdio_step::geometry::export_step(kernel, &[geometry]).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("step", text_dictionary(value)))
        })
    }
}

struct ExportStl(SessionCapture);
impl Operator for ExportStl {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let data = kernel.export_stl(&[geometry], deflection).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("stl", text_dictionary(encode_base64(&data))))
        })
    }
}

struct ExportObj(SessionCapture);
impl Operator for ExportObj {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let value = kernel.export_obj(&[geometry], deflection).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("obj", text_dictionary(value)))
        })
    }
}

struct ImportStep(SessionCapture);
impl Operator for ImportStep {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let data = read_text(input, "data")?;
            let shapes = semio_s_artifact_stdio_step::geometry::import_step(kernel, &data).map_err(|error| map_kernel_error(&error))?;
            let handle = shapes.into_iter().next().ok_or_else(|| EvalError::InvalidInput("step import produced no solids".into()))?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ImportStl(SessionCapture);
impl Operator for ImportStl {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let data = decode_base64(&read_text(input, "data")?)?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.import_stl(&data, tolerance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ImportObj(SessionCapture);
impl Operator for ImportObj {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let data = read_text(input, "data")?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.import_obj(&data, tolerance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ExportDwg(SessionCapture);
impl Operator for ExportDwg {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let data = semio_s_artifact_stdio_semio::standards::v1::subsets::brep::io::dwg::export(kernel, &[geometry], deflection).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("dwg", text_dictionary(encode_base64(&data))))
        })
    }
}

struct ImportDwg(SessionCapture);
impl Operator for ImportDwg {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let data = decode_base64(&read_text(input, "data")?)?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = semio_s_artifact_stdio_semio::standards::v1::subsets::brep::io::dwg::import(kernel, &data, tolerance).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️IO

#[path = "🥽️mesh/🦀️.rs"]
mod mesh;

/// 📦️ Registers brep geometry schema and operators.
pub fn register(registry: &mut Registry, session: &Session) {
    registry.register_schema(geometry_schema());
    registry.register_schema(topology_element_schema("vertex", "Vertex", "emoji:📍️"));
    registry.register_schema(topology_element_schema("edge", "Edge", "emoji:〰"));
    registry.register_schema(topology_element_schema("face", "Face", "emoji:⬜️"));
    registry.register_schema(topology_element_schema("shell", "Shell", "emoji:🐚️"));
    registry.register_schema(brep_schema());
    registry.register_schema(text_schema());
    registry.register_operator(
        OperatorInfo {
            id: "brep.brep".into(),
            extension: "brep".into(),
            name: "Brep".into(),
            abbreviation: "Brep".into(),
            icon: "emoji:🧊️".into(),
            summary: q("deconstruct", "Deconstructs B-Rep geometry into vertices, edges, faces, and shells"),
            inputs: vec![geometry_channel("brep", "brep.brep").with_value_types(&["geometry", "list"]), ChannelSpec::text_default("edgeLabels", "[]", &["brep.brep"]), ChannelSpec::text_default("faceLabels", "[]", &["brep.brep"]), ChannelSpec::text_default("sourceHandle", "", &["brep.brep"])],
            outputs: vec![
                ChannelSpec::named("B", "Brep", neural_engine::produced_channel_id("brep"), "BrepGeometry").with_operators(vec!["brep.brep".into()]).with_value_types(&["geometry"]),
                topology_output("V", "Vtx", "vertex", "vertex"),
                topology_output("E", "Edg", "edge", "edge"),
                topology_output("F", "Fce", "face", "face"),
                topology_output("S", "Shl", "shell", "shell"),
                topology_output("SE", "SelE", "selectedEdges", "edge"),
                topology_output("SF", "SelF", "selectedFaces", "face"),
                ChannelSpec::named("SI", "SrcI", "sourceIndex", "SourceIndex").with_value_types(&["number"]),
                ChannelSpec::list_output("errors", vec![]),
            ],
            group: vec!["Schemas".into()],
            ..Default::default()
        },
        ["geometry", "list"].into_iter().map(|schema| OperatorImpl { schemas: vec![schema.into(), "text".into(), "text".into(), "text".into()], operator: Box::new(BrepDeconstruct(session.capture())) as Box<dyn Operator> }).collect(),
        &["geometry", "list"],
    );

    reg_geo(
        registry,
        "brep.prim3d.box",
        "Box",
        "Box",
        "emoji:📦️",
        &q("box_prim", "Axis-aligned box solid"),
        vec![number_channel("width", "brep.prim3d.box", 1.0), number_channel("depth", "brep.prim3d.box", 1.0), number_channel("height", "brep.prim3d.box", 1.0)],
        out_solid("BoxSolid"),
        &["Primitives 3D"],
        Box::new(BoxPrim(session.capture())),
    );
    reg_geo(registry, "brep.prim3d.sphere", "Sphere", "Sphere", "emoji:⚪️", &q("sphere_prim", "Sphere solid"), vec![number_channel("radius", "brep.prim3d.sphere", 1.0)], out_solid("SphereSolid"), &["Primitives 3D"], Box::new(SpherePrim(session.capture())));
    reg_geo(
        registry,
        "brep.prim3d.cylinder",
        "Cylinder",
        "Cylinder",
        "emoji:🛢️",
        &q("cylinder_prim", "Cylinder solid"),
        vec![number_channel("radius", "brep.prim3d.cylinder", 1.0), number_channel("height", "brep.prim3d.cylinder", 1.0)],
        out_solid("CylinderSolid"),
        &["Primitives 3D"],
        Box::new(CylinderPrim(session.capture())),
    );
    reg_geo(
        registry,
        "brep.prim3d.cone",
        "Cone",
        "Cone",
        "emoji:🛢️",
        &q("cone_prim", "Cone solid"),
        vec![number_channel("radius", "brep.prim3d.cone", 1.0), number_channel("height", "brep.prim3d.cone", 1.0)],
        out_solid("ConeSolid"),
        &["Primitives 3D"],
        Box::new(ConePrim(session.capture())),
    );
    reg_geo(
        registry,
        "brep.prim3d.torus",
        "Torus",
        "Torus",
        "emoji:🛢️",
        &q("torus_prim", "Torus solid"),
        vec![number_channel("major", "brep.prim3d.torus", 2.0), number_channel("minor", "brep.prim3d.torus", 0.5)],
        out_solid("TorusSolid"),
        &["Primitives 3D"],
        Box::new(TorusPrim(session.capture())),
    );
    reg_geo(
        registry,
        "brep.prim3d.convexHull",
        "Convex Hull",
        "Hull",
        "emoji:📦️",
        &q("convex_hull", "Convex hull from points"),
        vec![list_channel("points", "brep.prim3d.convexHull").with_item_types(&["point", "vector"])],
        out_solid("ConvexHullSolid"),
        &["Primitives 3D"],
        Box::new(ConvexHullPrim(session.capture())),
    );

    reg_geo(registry, "brep.curve.line", "Line", "Line", "emoji:📏️", &q("line_curve", "Line curve"), vec![point_channel("start", "brep.curve.line"), point_channel("end", "brep.curve.line")], out_curve("LineCurve"), &["Curves"], Box::new(LineCurve(session.capture())));
    reg_geo(
        registry,
        "brep.curve.circle",
        "Circle",
        "Circle",
        "emoji:⭕️",
        &q("circle_curve", "Circle curve"),
        vec![point_channel("center", "brep.curve.circle"), point_channel("normal", "brep.curve.circle"), number_channel("radius", "brep.curve.circle", 1.0)],
        out_curve("CircleCurve"),
        &["Curves"],
        Box::new(CircleCurve(session.capture())),
    );
    reg_geo(
        registry,
        "brep.curve.arc",
        "Arc",
        "Arc",
        "emoji:⭕️",
        &q("arc_curve", "Arc curve"),
        vec![
            point_channel("center", "brep.curve.arc"),
            point_channel("normal", "brep.curve.arc"),
            number_channel("radius", "brep.curve.arc", 1.0),
            number_channel("startAngle", "brep.curve.arc", 0.0),
            number_channel("endAngle", "brep.curve.arc", std::f64::consts::FRAC_PI_2),
        ],
        out_curve("ArcCurve"),
        &["Curves"],
        Box::new(ArcCurve(session.capture())),
    );
    reg_geo(
        registry,
        "brep.curve.ellipse",
        "Ellipse",
        "Ellipse",
        "emoji:⭕️",
        &q("ellipse_curve", "Ellipse curve"),
        vec![point_channel("center", "brep.curve.ellipse"), point_channel("normal", "brep.curve.ellipse"), number_channel("semiMajor", "brep.curve.ellipse", 2.0), number_channel("semiMinor", "brep.curve.ellipse", 1.0)],
        out_curve("EllipseCurve"),
        &["Curves"],
        Box::new(EllipseCurve(session.capture())),
    );
    reg_geo(registry, "brep.curve.polyline", "Polyline", "Poly", "emoji:📏️", &q("polyline_wire", "Polyline wire"), vec![list_channel("points", "brep.curve.polyline").with_item_types(&["point", "vector"])], out_wire("PolylineWire"), &["Curves"], Box::new(PolylineWire(session.capture())));
    reg_geo(
        registry,
        "brep.curve.rectangle",
        "Rectangle",
        "Rect",
        "emoji:⬜️",
        &q("rectangle_wire", "Rectangle wire"),
        vec![number_channel("width", "brep.curve.rectangle", 1.0), number_channel("height", "brep.curve.rectangle", 1.0)],
        out_wire("RectangleWire"),
        &["Curves"],
        Box::new(RectangleWire(session.capture())),
    );
    reg_geo(
        registry,
        "brep.curve.polygon",
        "Polygon",
        "Poly",
        "emoji:⬡️",
        &q("regular_polygon_wire", "Regular polygon wire"),
        vec![number_channel("radius", "brep.curve.polygon", 1.0), number_channel("sides", "brep.curve.polygon", 6.0)],
        out_wire("RegularPolygonWire"),
        &["Curves"],
        Box::new(RegularPolygonWire(session.capture())),
    );
    reg_geo(
        registry,
        "brep.curve.interpolate",
        "Interpolate",
        "Intp",
        "emoji:〰",
        &q("interpolate_curve", "Interpolated curve"),
        vec![list_channel("points", "brep.curve.interpolate").with_item_types(&["point", "vector"]), number_channel("degree", "brep.curve.interpolate", 3.0)],
        out_curve("InterpolatedCurve"),
        &["Curves"],
        Box::new(InterpolateCurve(session.capture())),
    );
    reg_geo(
        registry,
        "brep.curve.approximate",
        "Approximate",
        "Appr",
        "emoji:〰",
        &q("approximate_curve", "Approximated curve"),
        vec![list_channel("points", "brep.curve.approximate").with_item_types(&["point", "vector"]), number_channel("degree", "brep.curve.approximate", 3.0), number_channel("controlPoints", "brep.curve.approximate", 4.0)],
        out_curve("ApproximatedCurve"),
        &["Curves"],
        Box::new(ApproximateCurve(session.capture())),
    );
    reg_geo(
        registry,
        "brep.curve.helix",
        "Helix",
        "Helix",
        "emoji:🌀️",
        &q("helix_curve", "Helix curve"),
        vec![
            point_channel("origin", "brep.curve.helix"),
            point_channel("axis", "brep.curve.helix"),
            number_channel("radius", "brep.curve.helix", 1.0),
            number_channel("pitch", "brep.curve.helix", 1.0),
            number_channel("turns", "brep.curve.helix", 1.0),
        ],
        out_curve("HelixCurve"),
        &["Curves"],
        Box::new(HelixCurve(session.capture())),
    );

    reg_geo(
        registry,
        "brep.surf.plane",
        "Plane",
        "Plane",
        "emoji:⬜️",
        &q("plane_surface", "Plane surface"),
        vec![point_channel("origin", "brep.surf.plane"), point_channel("normal", "brep.surf.plane")],
        out_surface("PlaneSurface"),
        &["Surfaces"],
        Box::new(PlaneSurface(session.capture())),
    );
    reg_geo(
        registry,
        "brep.surf.planarFace",
        "Planar Face",
        "PFace",
        "emoji:⬜️",
        &q("planar_face_from_points", "Planar face from points"),
        vec![list_channel("points", "brep.surf.planarFace").with_item_types(&["point", "vector"])],
        out_face("PlanarFace"),
        &["Surfaces"],
        Box::new(PlanarFacePoints(session.capture())),
    );
    reg_geo(
        registry,
        "brep.surf.planarFaceWire",
        "Planar Face Wire",
        "PFW",
        "emoji:⬜️",
        &q("planar_face_from_wire", "Planar face from wire"),
        vec![geometry_channel("wire", "brep.surf.planarFaceWire")],
        out_face("PlanarFaceWire"),
        &["Surfaces"],
        Box::new(PlanarFaceWire(session.capture())),
    );
    reg_geo(
        registry,
        "brep.surf.nurbsGrid",
        "Nurbs Grid",
        "Grid",
        "emoji:🧮️",
        &q("nurbs_surface_from_grid", "Nurbs surface from point grid"),
        vec![list_channel("points", "brep.surf.nurbsGrid").with_item_types(&["point", "vector"]), number_channel("rows", "brep.surf.nurbsGrid", 2.0), number_channel("degreeU", "brep.surf.nurbsGrid", 3.0), number_channel("degreeV", "brep.surf.nurbsGrid", 3.0)],
        out_surface("NurbsSurface"),
        &["Surfaces"],
        Box::new(NurbsGridSurface(session.capture())),
    );
    reg_geo(registry, "brep.surf.coons", "Coons Patch", "Coons", "emoji:🧩️", &q("coons_patch", "Coons patch from boundary curves"), vec![list_channel("curves", "brep.surf.coons")], out_surface("CoonsPatch"), &["Surfaces"], Box::new(CoonsPatch(session.capture())));
    reg_geo(
        registry,
        "brep.surf.offset",
        "Offset Face",
        "Offset",
        "emoji:↔",
        &q("offset_face", "Offset face"),
        vec![geometry_channel("face", "brep.surf.offset"), number_channel("distance", "brep.surf.offset", 0.1)],
        out_face_result("OffsetFace"),
        &["Surfaces"],
        Box::new(OffsetFace(session.capture())),
    );
    reg_geo(
        registry,
        "brep.surf.thicken",
        "Thicken",
        "Thick",
        "emoji:🧱️",
        &q("thicken_face", "Thicken face to solid"),
        vec![geometry_channel("face", "brep.surf.thicken"), number_channel("thickness", "brep.surf.thicken", 0.1)],
        out_solid("ThickenedSolid"),
        &["Surfaces"],
        Box::new(ThickenFace(session.capture())),
    );

    reg_geo(
        registry,
        "brep.solid.extrude",
        "Extrude Curve",
        "ExtC",
        "emoji:🧱️",
        &q("extrude_wire", "Extrude closed wire along vector magnitude"),
        vec![geometry_channel("wire", "brep.solid.extrude"), vector_channel("vector", "brep.solid.extrude", [0.0, 0.0, 5.0])],
        out_solid("ExtrudedSolid"),
        &["Solids"],
        Box::new(ExtrudeCurve(session.capture())),
    );
    reg_geo(
        registry,
        "brep.sweep.extrude",
        "Extrude",
        "Extr",
        "emoji:⬆️",
        &q("extrude", "Extrude face along vector magnitude"),
        vec![geometry_channel("face", "brep.sweep.extrude"), vector_channel("vector", "brep.sweep.extrude", [0.0, 0.0, 1.0])],
        out_solid("ExtrudedSolid"),
        &["Sweeps"],
        Box::new(ExtrudeFace(session.capture())),
    );
    reg_geo(
        registry,
        "brep.sweep.revolve",
        "Revolve",
        "Rev",
        "emoji:🔄️",
        &q("revolve", "Revolve face"),
        vec![geometry_channel("face", "brep.sweep.revolve"), point_channel("axisOrigin", "brep.sweep.revolve"), point_channel("axisDirection", "brep.sweep.revolve"), number_channel("angle", "brep.sweep.revolve", std::f64::consts::TAU)],
        out_solid("RevolvedSolid"),
        &["Sweeps"],
        Box::new(Revolve(session.capture())),
    );
    reg_geo(
        registry,
        "brep.sweep.loft",
        "Loft",
        "Loft",
        "emoji:🌉️",
        &q("loft", "Loft profiles"),
        vec![list_channel("profiles", "brep.sweep.loft"), number_channel("smooth", "brep.sweep.loft", 0.0)],
        out_solid("LoftedSolid"),
        &["Sweeps"],
        Box::new(Loft(session.capture())),
    );
    reg_geo(
        registry,
        "brep.sweep.sweep",
        "Sweep",
        "Sweep",
        "emoji:🛤️",
        &q("sweep", "Sweep profile along path"),
        vec![geometry_channel("profile", "brep.sweep.sweep"), geometry_channel("path", "brep.sweep.sweep")],
        out_solid("SweptSolid"),
        &["Sweeps"],
        Box::new(Sweep(session.capture())),
    );
    reg_geo(
        registry,
        "brep.sweep.pipe",
        "Pipe",
        "Pipe",
        "emoji:🛤️",
        &q("pipe", "Pipe profile along path"),
        vec![geometry_channel("profile", "brep.sweep.pipe"), geometry_channel("path", "brep.sweep.pipe"), geometry_channel("guide", "brep.sweep.pipe")],
        out_solid("PipeSolid"),
        &["Sweeps"],
        Box::new(Pipe(session.capture())),
    );
    reg_geo(
        registry,
        "brep.sweep.helical",
        "Helical Sweep",
        "HelSw",
        "emoji:🌀️",
        &q("helical_sweep", "Helical sweep"),
        vec![
            geometry_channel("profile", "brep.sweep.helical"),
            point_channel("axisOrigin", "brep.sweep.helical"),
            point_channel("axisDirection", "brep.sweep.helical"),
            number_channel("radius", "brep.sweep.helical", 1.0),
            number_channel("pitch", "brep.sweep.helical", 1.0),
            number_channel("turns", "brep.sweep.helical", 1.0),
        ],
        out_solid("HelicalSolid"),
        &["Sweeps"],
        Box::new(HelicalSweep(session.capture())),
    );

    reg_geo(registry, "brep.bool.fuse", "Fuse", "Fuse", "emoji:🔗️", &q("fuse", "Boolean union"), vec![geometry_channel("a", "brep.bool.fuse"), geometry_channel("b", "brep.bool.fuse")], out_solid("FusedSolid"), &["Booleans"], Box::new(Fuse(session.capture())));
    reg_geo(registry, "brep.bool.cut", "Cut", "Cut", "emoji:🔗️", &q("cut", "Boolean difference"), vec![geometry_channel("a", "brep.bool.cut"), geometry_channel("b", "brep.bool.cut")], out_solid("CutSolid"), &["Booleans"], Box::new(Cut(session.capture())));
    reg_geo(
        registry,
        "brep.bool.intersect",
        "Intersect",
        "Int",
        "emoji:🔗️",
        &q("intersect", "Boolean intersection"),
        vec![geometry_channel("a", "brep.bool.intersect"), geometry_channel("b", "brep.bool.intersect")],
        out_solid("IntersectedSolid"),
        &["Booleans"],
        Box::new(Intersect(session.capture())),
    );
    reg_geo(
        registry,
        "brep.bool.compoundCut",
        "Compound Cut",
        "CCut",
        "emoji:🔗️",
        &q("compound_cut", "Compound boolean cut"),
        vec![geometry_channel("target", "brep.bool.compoundCut"), list_channel("tools", "brep.bool.compoundCut")],
        out_solid("CompoundCutSolid"),
        &["Booleans"],
        Box::new(CompoundCut(session.capture())),
    );

    reg_geo(
        registry,
        "brep.xform.translate",
        "Translate",
        "Trans",
        "emoji:🔁️",
        &q("translate", "Translate geometry"),
        vec![geometry_channel("geometry", "brep.xform.translate"), point_channel("offset", "math.move")],
        out_geometry_result("TranslatedGeometry"),
        &["Transforms"],
        Box::new(Translate(session.capture())),
    );
    reg_geo(
        registry,
        "brep.xform.rotate",
        "Rotate",
        "Rot",
        "emoji:🔁️",
        &q("rotate", "Rotate geometry"),
        vec![geometry_channel("geometry", "brep.xform.rotate"), number_channel("angle", "brep.xform.rotate", std::f64::consts::FRAC_PI_4), point_channel("axis", "brep.xform.rotate")],
        out_geometry_result("RotatedGeometry"),
        &["Transforms"],
        Box::new(Rotate(session.capture())),
    );
    reg_geo(
        registry,
        "brep.xform.rotateAbout",
        "Rotate About",
        "RotA",
        "emoji:🔁️",
        &q("rotate_about", "Rotate geometry about an explicit origin"),
        vec![
            geometry_channel("geometry", "brep.xform.rotateAbout"),
            point_channel("origin", "brep.xform.rotateAbout"),
            point_channel("axis", "brep.xform.rotateAbout"),
            number_channel("angle", "brep.xform.rotateAbout", std::f64::consts::FRAC_PI_4),
        ],
        out_geometry_result("RotatedGeometry"),
        &["Transforms"],
        Box::new(RotateAbout(session.capture())),
    );
    reg_geo(
        registry,
        "brep.xform.scale",
        "Scale",
        "Scale",
        "emoji:🔁️",
        &q("scale_axes", "Scale geometry independently on each axis about a center"),
        vec![geometry_channel("geometry", "brep.xform.scale"), vector_channel("factor", "brep.xform.scale", [1.0, 1.0, 1.0]), point_channel("center", "brep.xform.scale")],
        out_geometry_result("ScaledGeometry"),
        &["Transforms"],
        Box::new(Scale(session.capture())),
    );
    reg_geo(
        registry,
        "brep.xform.mirror",
        "Mirror",
        "Mir",
        "emoji:🔁️",
        &q("mirror", "Mirror geometry"),
        vec![geometry_channel("geometry", "brep.xform.mirror"), point_channel("origin", "brep.xform.mirror"), point_channel("normal", "brep.xform.mirror")],
        out_geometry_result("MirroredGeometry"),
        &["Transforms"],
        Box::new(Mirror(session.capture())),
    );
    reg_geo(registry, "brep.xform.copy", "Copy", "Copy", "emoji:📋️", &q("copy_shape", "Copy geometry"), vec![geometry_channel("geometry", "brep.xform.copy")], out_geometry_result("CopiedGeometry"), &["Transforms"], Box::new(CopyShape(session.capture())));
    reg_geo(
        registry,
        "brep.xform.linearPattern",
        "Linear Pattern",
        "LinP",
        "emoji:📐️",
        &q("linear_pattern", "Linear pattern"),
        vec![geometry_channel("geometry", "brep.xform.linearPattern"), point_channel("direction", "brep.xform.linearPattern"), number_channel("spacing", "brep.xform.linearPattern", 1.0), number_channel("count", "brep.xform.linearPattern", 3.0)],
        out_compound("LinearPattern"),
        &["Transforms"],
        Box::new(LinearPattern(session.capture())),
    );
    reg_geo(
        registry,
        "brep.xform.circularPattern",
        "Circular Pattern",
        "CircP",
        "emoji:📐️",
        &q("circular_pattern", "Circular pattern"),
        vec![geometry_channel("geometry", "brep.xform.circularPattern"), point_channel("axis", "brep.xform.circularPattern"), number_channel("count", "brep.xform.circularPattern", 4.0)],
        out_compound("CircularPattern"),
        &["Transforms"],
        Box::new(CircularPattern(session.capture())),
    );
    reg_geo(
        registry,
        "brep.xform.gridPattern",
        "Grid Pattern",
        "GridP",
        "emoji:📐️",
        &q("grid_pattern", "Grid pattern"),
        vec![
            geometry_channel("geometry", "brep.xform.gridPattern"),
            point_channel("dirX", "brep.xform.gridPattern"),
            point_channel("dirY", "brep.xform.gridPattern"),
            number_channel("spacingX", "brep.xform.gridPattern", 1.0),
            number_channel("spacingY", "brep.xform.gridPattern", 1.0),
            number_channel("countX", "brep.xform.gridPattern", 2.0),
            number_channel("countY", "brep.xform.gridPattern", 2.0),
        ],
        out_compound("GridPattern"),
        &["Transforms"],
        Box::new(GridPattern(session.capture())),
    );

    reg_geo(
        registry,
        "brep.solid.fillet",
        "Fillet",
        "Fil",
        "emoji:🧱️",
        &q("fillet", "Fillet all solid edges"),
        vec![geometry_channel("geometry", "brep.solid.fillet"), number_channel("radius", "brep.solid.fillet", 0.1)],
        out_solid("FilletedSolid"),
        &["Features"],
        Box::new(Fillet(session.capture())),
    );
    reg_geo(
        registry,
        "brep.solid.filletVariable",
        "Variable Fillet",
        "VFil",
        "emoji:🧱️",
        &q("fillet_variable", "Variable fillet"),
        vec![geometry_channel("geometry", "brep.solid.filletVariable"), number_channel("radiusStart", "brep.solid.filletVariable", 0.1), number_channel("radiusEnd", "brep.solid.filletVariable", 0.2)],
        out_solid("VariableFilletedSolid"),
        &["Features"],
        Box::new(FilletVariable(session.capture())),
    );
    reg_geo(
        registry,
        "brep.solid.chamfer",
        "Chamfer",
        "Chm",
        "emoji:🧱️",
        &q("chamfer", "Chamfer all solid edges"),
        vec![geometry_channel("geometry", "brep.solid.chamfer"), number_channel("distance", "brep.solid.chamfer", 0.1)],
        out_solid("ChamferedSolid"),
        &["Features"],
        Box::new(Chamfer(session.capture())),
    );
    reg_geo(
        registry,
        "brep.solid.chamferAsymmetric",
        "Asymmetric Chamfer",
        "AChm",
        "emoji:🧱️",
        &q("chamfer_asymmetric", "Asymmetric chamfer"),
        vec![geometry_channel("geometry", "brep.solid.chamferAsymmetric"), number_channel("d1", "brep.solid.chamferAsymmetric", 0.1), number_channel("d2", "brep.solid.chamferAsymmetric", 0.1)],
        out_solid("AsymmetricChamferedSolid"),
        &["Features"],
        Box::new(ChamferAsymmetric(session.capture())),
    );
    reg_geo(
        registry,
        "brep.solid.filletEdges",
        "Fillet Edges",
        "FilE",
        "emoji:🧱️",
        &q("fillet_edges", "Fillet only the given edges"),
        vec![geometry_channel("geometry", "brep.solid.filletEdges"), list_channel("edges", "brep.solid.filletEdges"), number_channel("radius", "brep.solid.filletEdges", 0.1)],
        out_solid("FilletedEdgesSolid"),
        &["Features"],
        Box::new(FilletEdges(session.capture())),
    );
    reg_geo(
        registry,
        "brep.solid.chamferEdges",
        "Chamfer Edges",
        "ChmE",
        "emoji:🧱️",
        &q("chamfer_edges", "Chamfer only the given edges"),
        vec![geometry_channel("geometry", "brep.solid.chamferEdges"), list_channel("edges", "brep.solid.chamferEdges"), number_channel("distance", "brep.solid.chamferEdges", 0.1)],
        out_solid("ChamferedEdgesSolid"),
        &["Features"],
        Box::new(ChamferEdges(session.capture())),
    );
    reg_geo(
        registry,
        "brep.solid.shell",
        "Shell",
        "Shell",
        "emoji:🧱️",
        &q("shell", "Shell solid"),
        vec![geometry_channel("geometry", "brep.solid.shell"), number_channel("thickness", "brep.solid.shell", 0.1), list_channel("openFaces", "brep.solid.shell")],
        out_solid("ShelledSolid"),
        &["Features"],
        Box::new(ShellMutation(session.capture())),
    );
    reg_geo(
        registry,
        "brep.solid.draft",
        "Draft",
        "Draft",
        "emoji:🧱️",
        &q("draft", "Draft faces"),
        vec![
            geometry_channel("geometry", "brep.solid.draft"),
            list_channel("faces", "brep.solid.draft"),
            point_channel("pullDirection", "brep.solid.draft"),
            point_channel("neutralPoint", "brep.solid.draft"),
            number_channel("angle", "brep.solid.draft", 0.1),
        ],
        out_solid("DraftedSolid"),
        &["Features"],
        Box::new(Draft(session.capture())),
    );
    reg_geo(
        registry,
        "brep.solid.offsetSolid",
        "Offset Solid",
        "OffS",
        "emoji:🧱️",
        &q("offset_solid", "Offset solid"),
        vec![geometry_channel("geometry", "brep.solid.offsetSolid"), number_channel("distance", "brep.solid.offsetSolid", 0.1)],
        out_solid("OffsetSolid"),
        &["Features"],
        Box::new(OffsetSolid(session.capture())),
    );
    reg_geo(
        registry,
        "brep.solid.defeature",
        "Defeature",
        "Def",
        "emoji:🧱️",
        &q("defeature", "Remove faces"),
        vec![geometry_channel("geometry", "brep.solid.defeature"), list_channel("faces", "brep.solid.defeature")],
        out_solid("DefeaturedSolid"),
        &["Features"],
        Box::new(Defeature(session.capture())),
    );

    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.intersect.section",
            "Section",
            "Sect",
            "emoji:✂️",
            &q("section", "Section solid with plane — every resulting face"),
            vec![geometry_channel("solid", "brep.intersect.section"), point_channel("planeOrigin", "brep.intersect.section"), point_channel("planeNormal", "brep.intersect.section")],
            vec![topology_output("F", "Fces", "faces", "geometry")],
            &["Intersect"],
        ),
        Box::new(Section(session.capture())),
        &["geometry", "list"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.intersect.split",
            "Split",
            "Split",
            "emoji:✂️",
            &q("split", "Split solid with plane — both halves"),
            vec![geometry_channel("solid", "brep.intersect.split"), point_channel("planeOrigin", "brep.intersect.split"), point_channel("planeNormal", "brep.intersect.split")],
            vec![ChannelSpec::named("P", "Pos", "positive", "PositiveSolid").with_value_types(&["geometry"]), ChannelSpec::named("N", "Neg", "negative", "NegativeSolid").with_value_types(&["geometry"])],
            &["Intersect"],
        ),
        Box::new(Split(session.capture())),
        &["geometry"],
    );
    reg_geo(
        registry,
        "brep.intersect.curveCurve",
        "Curve Curve",
        "CC",
        "emoji:✂️",
        &q("curve_curve_intersect", "Curve-curve intersection"),
        vec![geometry_channel("a", "brep.intersect.curveCurve"), geometry_channel("b", "brep.intersect.curveCurve"), number_channel("tolerance", "brep.intersect.curveCurve", 0.001)],
        out_wire("CurveCurveIntersection"),
        &["Intersect"],
        Box::new(CurveCurveIntersect(session.capture())),
    );
    reg_geo(
        registry,
        "brep.intersect.curveSurface",
        "Curve Surface",
        "CS",
        "emoji:✂️",
        &q("curve_surface_intersect", "Curve-surface intersection"),
        vec![geometry_channel("curve", "brep.intersect.curveSurface"), geometry_channel("surface", "brep.intersect.curveSurface"), number_channel("tolerance", "brep.intersect.curveSurface", 0.001)],
        out_wire("CurveSurfaceIntersection"),
        &["Intersect"],
        Box::new(CurveSurfaceIntersect(session.capture())),
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.intersect.surfaceSurface",
            "Surface Surface",
            "SS",
            "emoji:✂️",
            &q("surface_surface_intersect", "Surface-surface intersection — every resulting wire"),
            vec![geometry_channel("a", "brep.intersect.surfaceSurface"), geometry_channel("b", "brep.intersect.surfaceSurface"), number_channel("tolerance", "brep.intersect.surfaceSurface", 0.001)],
            vec![topology_output("W", "Wres", "wires", "geometry")],
            &["Intersect"],
        ),
        Box::new(SurfaceSurfaceIntersect(session.capture())),
        &["geometry", "list"],
    );

    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.eval.curvePoint",
            "Curve Point",
            "Cpt",
            "emoji:📍️",
            &q("curve_point", "Evaluate curve point"),
            vec![geometry_channel("curve", "brep.eval.curvePoint"), number_channel("parameter", "brep.eval.curvePoint", 0.0)],
            vec![out_point("CurvePoint")],
            &["Evaluate"],
        ),
        Box::new(CurvePoint(session.capture())),
        &["point"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.eval.curveTangent",
            "Curve Tangent",
            "Ctn",
            "emoji:➡️",
            &q("curve_tangent", "Evaluate curve tangent"),
            vec![geometry_channel("curve", "brep.eval.curveTangent"), number_channel("parameter", "brep.eval.curveTangent", 0.0)],
            vec![ChannelSpec::named("T", "Tan", "tangent", "CurveTangent").with_value_types(&["vector"])],
            &["Evaluate"],
        ),
        Box::new(CurveTangent(session.capture())),
        &["vector"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.eval.curveDomain", "Curve Domain", "Cdm", "emoji:📏️", &q("curve_domain", "Curve domain span"), vec![geometry_channel("curve", "brep.eval.curveDomain")], vec![out_span()], &["Evaluate"]),
        Box::new(CurveDomain(session.capture())),
        &["number"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.eval.curveCurvature",
            "Curve Curvature",
            "Ccv",
            "emoji:〰",
            &q("curve_curvature", "Curve curvature"),
            vec![geometry_channel("curve", "brep.eval.curveCurvature"), number_channel("parameter", "brep.eval.curveCurvature", 0.0)],
            vec![out_curvature()],
            &["Evaluate"],
        ),
        Box::new(CurveCurvature(session.capture())),
        &["number"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.eval.surfPoint",
            "Surface Point",
            "Spt",
            "emoji:📍️",
            &q("surface_point", "Evaluate surface point"),
            vec![geometry_channel("surface", "brep.eval.surfPoint"), number_channel("u", "brep.eval.surfPoint", 0.0), number_channel("v", "brep.eval.surfPoint", 0.0)],
            vec![out_point("SurfacePoint")],
            &["Evaluate"],
        ),
        Box::new(SurfacePoint(session.capture())),
        &["point"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.eval.surfNormal",
            "Surface Normal",
            "Sn",
            "emoji:➡️",
            &q("surface_normal", "Evaluate surface normal"),
            vec![geometry_channel("surface", "brep.eval.surfNormal"), number_channel("u", "brep.eval.surfNormal", 0.0), number_channel("v", "brep.eval.surfNormal", 0.0)],
            vec![out_normal("SurfaceNormal")],
            &["Evaluate"],
        ),
        Box::new(SurfaceNormal(session.capture())),
        &["vector"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.eval.curveClosestParameter",
            "Curve Closest Parameter",
            "CCp",
            "emoji:🎯️",
            &q("curve_closest_parameter", "Certified closest parameter, point, and achieved distance on a curve"),
            vec![geometry_channel("curve", "brep.eval.curveClosestParameter"), point_channel("point", "brep.eval.curveClosestParameter")],
            vec![ChannelSpec::named("T", "Prm", "parameter", "ClosestParameter").with_value_types(&["number"]), out_point_result("ClosestPoint"), ChannelSpec::named("D", "Dst", "distance", "AchievedDistance").with_value_types(&["number"])],
            &["Evaluate"],
        ),
        Box::new(CurveClosestParameter(session.capture())),
        &["number", "point"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.eval.surfaceClosestUv",
            "Surface Closest Uv",
            "SCuv",
            "emoji:🎯️",
            &q("surface_closest_uv", "Certified closest (u, v), point, and achieved distance on a surface"),
            vec![geometry_channel("surface", "brep.eval.surfaceClosestUv"), point_channel("point", "brep.eval.surfaceClosestUv")],
            vec![ChannelSpec::named("U", "U", "u", "ClosestU").with_value_types(&["number"]), ChannelSpec::named("V", "V", "v", "ClosestV").with_value_types(&["number"]), out_point_result("ClosestPoint"), ChannelSpec::named("D", "Dst", "distance", "AchievedDistance").with_value_types(&["number"])],
            &["Evaluate"],
        ),
        Box::new(SurfaceClosestUv(session.capture())),
        &["number", "point"],
    );

    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.volume", "Volume", "Vol", "emoji:📐️", &q("volume", "Volume of distinct solids"), vec![geometry_channel("geometry", "brep.measure.volume")], vec![out_volume()], &["Measure"]),
        Box::new(Volume(session.capture())),
        &["number"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.area", "Area", "Area", "emoji:📐️", &q("area", "Area of distinct topological faces"), vec![geometry_channel("geometry", "brep.measure.area")], vec![out_area()], &["Measure"]),
        Box::new(Area(session.capture())),
        &["number"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.length", "Length", "Len", "emoji:📐️", &q("length", "Length of a bounded curve or distinct boundary edges"), vec![geometry_channel("geometry", "brep.measure.length")], vec![out_length()], &["Measure"]),
        Box::new(Length(session.capture())),
        &["number"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.centerOfMass", "Center Of Mass", "CoM", "emoji:📐️", &q("center_of_mass", "Center of mass"), vec![geometry_channel("geometry", "brep.measure.centerOfMass")], vec![out_center()], &["Measure"]),
        Box::new(CenterOfMass(session.capture())),
        &["point"],
    );
    reg_geo(registry, "brep.measure.boundingBox", "Bounding Box", "BBox", "emoji:📐️", &q("bounding_box", "Axis-aligned bounding box"), vec![geometry_channel("geometry", "brep.measure.boundingBox")], out_box(), &["Measure"], Box::new(BoundingBox(session.capture())));
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.measure.distance",
            "Distance",
            "Dist",
            "emoji:📐️",
            &q("distance", "Minimum distance"),
            vec![geometry_channel("a", "brep.measure.distance"), geometry_channel("b", "brep.measure.distance")],
            vec![out_distance()],
            &["Measure"],
        ),
        Box::new(Distance(session.capture())),
        &["number"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.measure.closestPoint",
            "Closest Point",
            "ClPt",
            "emoji:📐️",
            &q("closest_point", "Closest point on geometry"),
            vec![geometry_channel("geometry", "brep.measure.closestPoint"), point_channel("point", "brep.measure.closestPoint")],
            vec![out_point_result("ClosestPoint")],
            &["Measure"],
        ),
        Box::new(ClosestPoint(session.capture())),
        &["point"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.measure.classify",
            "Classify",
            "Cls",
            "emoji:📐️",
            &q("classify_point", "Classify point relative to solid"),
            vec![geometry_channel("solid", "brep.measure.classify"), point_channel("point", "brep.measure.classify")],
            vec![out_classification()],
            &["Measure"],
        ),
        Box::new(ClassifyPoint(session.capture())),
        &["number"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.validate", "Validate", "Val", "emoji:📐️", &q("validate", "Validate geometry"), vec![geometry_channel("geometry", "brep.measure.validate")], vec![out_report()], &["Measure"]),
        Box::new(Validate(session.capture())),
        &["text"],
    );

    reg_geo(registry, "brep.util.vertex", "Vertex", "Vtx", "emoji:📍️", &q("vertex", "Create vertex"), vec![point_channel("point", "brep.util.vertex")], out_vertex(), &["Utilities"], Box::new(Vertex(session.capture())));
    reg_geo(
        registry,
        "brep.util.faceFromWire",
        "Face From Wire",
        "FFW",
        "emoji:⬜️",
        &q("face_from_wire", "Face from closed wire"),
        vec![geometry_channel("wire", "brep.util.faceFromWire")],
        out_face("FaceFromWire"),
        &["Utilities"],
        Box::new(FaceFromWire(session.capture())),
    );
    reg_geo(
        registry,
        "brep.util.sew",
        "Sew",
        "Sew",
        "emoji:🧵️",
        &q("sew_faces", "Sew faces"),
        vec![list_channel("faces", "brep.util.sew"), number_channel("tolerance", "brep.util.sew", 0.001)],
        out_solid("SewnSolid"),
        &["Utilities"],
        Box::new(SewFaces(session.capture())),
    );
    reg_geo(
        registry,
        "brep.util.heal",
        "Heal",
        "Heal",
        "emoji:🩹️",
        &q("heal_solid", "Heal solid"),
        vec![geometry_channel("geometry", "brep.util.heal"), number_channel("tolerance", "brep.util.heal", 0.001)],
        out_solid("HealedSolid"),
        &["Utilities"],
        Box::new(HealSolid(session.capture())),
    );
    reg_geo(
        registry,
        "brep.util.convertToNurbs",
        "Convert To Nurbs",
        "Nrb",
        "emoji:〰",
        &q("convert_to_nurbs", "Convert to NURBS"),
        vec![geometry_channel("geometry", "brep.util.convertToNurbs")],
        out_geometry_result("NurbsGeometry"),
        &["Utilities"],
        Box::new(ConvertToNurbs(session.capture())),
    );

    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.topology.shells",
            "Shells",
            "Shls",
            "emoji:🐚️",
            &q("solid_shells", "Solid's shells as independent geometry"),
            vec![geometry_channel("solid", "brep.topology.shells")],
            vec![topology_output("S", "Shls", "shells", "geometry")],
            &["Topology"],
        ),
        Box::new(SolidShells(session.capture())),
        &["geometry", "list"],
    );
    reg_geo(registry, "brep.topology.compound", "Compound", "Cmpd", "emoji:🗃️", &q("compound", "Combine solids into a compound"), vec![list_channel("solids", "brep.topology.compound")], out_compound("Compound"), &["Topology"], Box::new(CompoundOf(session.capture())));
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.topology.explode",
            "Explode",
            "Xpld",
            "emoji:💥️",
            &q("explode", "Split a compound into its member solids"),
            vec![geometry_channel("compound", "brep.topology.explode")],
            vec![topology_output("S", "Slds", "solids", "geometry")],
            &["Topology"],
        ),
        Box::new(Explode(session.capture())),
        &["geometry", "list"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.topology.label",
            "Label",
            "Lbl",
            "emoji:🏷️",
            &q("label", "Handle's persistent label"),
            vec![geometry_channel("geometry", "brep.topology.label")],
            vec![ChannelSpec::named("L", "Lbl", "label", "PersistentLabel").with_value_types(&["text"])],
            &["Topology"],
        ),
        Box::new(GeometryLabel(session.capture())),
        &["text"],
    );

    register_typed(
        registry,
        operator_info_with_outputs("brep.io.exportStep", "Export Step", "Stp", "emoji:💾️", &q("export_step", "Export STEP"), vec![geometry_channel("geometry", "brep.io.exportStep")], vec![out_step()], &["IO"]),
        Box::new(ExportStep(session.capture())),
        &["text"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.io.exportStl",
            "Export Stl",
            "Stl",
            "emoji:💾️",
            &q("export_stl", "Export STL as base64"),
            vec![geometry_channel("geometry", "brep.io.exportStl"), number_channel("deflection", "brep.io.exportStl", 0.1)],
            vec![out_stl()],
            &["IO"],
        ),
        Box::new(ExportStl(session.capture())),
        &["text"],
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.io.exportObj",
            "Export Obj",
            "Obj",
            "emoji:💾️",
            &q("export_obj", "Export OBJ"),
            vec![geometry_channel("geometry", "brep.io.exportObj"), number_channel("deflection", "brep.io.exportObj", 0.1)],
            vec![out_obj()],
            &["IO"],
        ),
        Box::new(ExportObj(session.capture())),
        &["text"],
    );
    reg_geo(registry, "brep.io.importStep", "Import Step", "IStp", "emoji:📂️", &q("import_step", "Import STEP"), vec![ChannelSpec::requires("data", &["brep.io.importStep"]).with_value_types(&["text"])], out_geometry("ImportedGeometry"), &["IO"], Box::new(ImportStep(session.capture())));
    reg_geo(
        registry,
        "brep.io.importStl",
        "Import Stl",
        "IStl",
        "emoji:📂️",
        &q("import_stl", "Import STL from base64"),
        vec![ChannelSpec::requires("data", &["brep.io.importStl"]).with_value_types(&["text"]), number_channel("tolerance", "brep.io.importStl", 0.1)],
        out_geometry("ImportedGeometry"),
        &["IO"],
        Box::new(ImportStl(session.capture())),
    );
    reg_geo(
        registry,
        "brep.io.importObj",
        "Import Obj",
        "IObj",
        "emoji:📂️",
        &q("import_obj", "Import OBJ"),
        vec![ChannelSpec::requires("data", &["brep.io.importObj"]).with_value_types(&["text"]), number_channel("tolerance", "brep.io.importObj", 0.1)],
        out_geometry("ImportedGeometry"),
        &["IO"],
        Box::new(ImportObj(session.capture())),
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.io.exportDwg",
            "Export Dwg",
            "Dwg",
            "emoji:💾️",
            &q("export_mesh", "Export DWG as base64"),
            vec![geometry_channel("geometry", "brep.io.exportDwg"), number_channel("deflection", "brep.io.exportDwg", 0.1)],
            vec![ChannelSpec::named("D", "Dwg", "dwg", "DwgExport").with_value_types(&["text"])],
            &["IO"],
        ),
        Box::new(ExportDwg(session.capture())),
        &["text"],
    );
    reg_geo(
        registry,
        "brep.io.importDwg",
        "Import Dwg",
        "IDwg",
        "emoji:📂️",
        &q("import_mesh", "Import DWG from base64"),
        vec![ChannelSpec::requires("data", &["brep.io.importDwg"]).with_value_types(&["text"]), number_channel("tolerance", "brep.io.importDwg", 0.1)],
        out_geometry("ImportedGeometry"),
        &["IO"],
        Box::new(ImportDwg(session.capture())),
    );

    mesh::register_mesh(registry, session);
    registry.finalize();
}

/// 🛂️ Manifest JSON for host contribution install (tests + packaging metadata).
pub async fn extension_manifest_json() -> String {
    let session = semio_s_artifact_stdio_step::geometry::session::geometry_session();
    let registry = neural_engine::ColdOwner::new(module_registry(&session));
    let manifest = build_manifest_json("brep", "Brep", env!("CARGO_PKG_VERSION"), &registry, vec!["onStartup".into()], vec![], vec![], vec![]);
    drop(registry);
    session.close();
    manifest
}

pub fn module_registry(session: &Session) -> Registry {
    let mut registry = Registry::new();
    register(&mut registry, session);
    registry
}

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests

#[cfg(feature = "component-guest")]
#[path = "💡️inferences/📐️geometry/🦀️.rs"]
pub mod geometry_inference;

// #region 🔖️ExtensionGuest
#[cfg(feature = "component-guest")]
mod extension_guest {
    use flow_extension_sdk::flow_extension_topic_contribution;
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle, ExtensionResourceOwner, PluginLifecycleStep};

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "brep";
    const EXTENSION_LABEL: &str = "Brep";
    /// ⏱️ Face/edge units one `tessellate` STEP spends before the round trip re-checks its wall
    /// deadline — the granularity at which a cancel can land, never the round trip's own bound.
    /// The bound is `wallMicros` (default `brep_geometry::TESSELLATE_STEP_WALL_MICROS`): units are
    /// not time, so a unit budget alone spends a whole interactive round trip on 29 µs of work
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    const TESSELLATE_STEP_BUDGET: usize = 24;

    pub(crate) fn bundle() -> ExtensionBundle {
        owned_bundle(super::geometry_inference::GeometryInferenceContext::new(super::Session::new().capture()))
    }

    fn owned_bundle(context:super::geometry_inference::GeometryInferenceContext)->ExtensionBundle {
        let manifest_json = ::semio_framework_async::poll::resolve_ready(super::extension_manifest_json());
        let flow_topic = flow_extension_topic_contribution(FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "brep", &manifest_json);
        let procedural3d_topic = flow_extension_topic_contribution(PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "brep", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-brep", "Brep", env!("CARGO_PKG_VERSION")).extends("flow").depends_on("flow", semio_framework::tree_pin!());
        let bundle = bundle.mode(ExecutionMode::Linked);
        let bundle = ::semio_framework_async::poll::resolve_ready(async {
            bundle.contributes(semio_framework_plugin::app::ArtifactContribution::builder(super::geometry_inference::GEOMETRY_ARTIFACT_KIND).await
                .inference_service(super::geometry_inference::geometry_inference_service()).await).await
        });
        let bundle = bundle.contributes_topic(flow_topic.topic, flow_topic.payload);
        let bundle = bundle.contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload);
        bundle.resource_owner(BrepExtensionResources { context }).owned_handler("evaluate").owned_handler("tessellate").owned_handler("evaluateCancel").owned_handler("tessellateCancel")
    }

    #[cfg(test)]
    fn scoped_bundle()->(ExtensionBundle,neural_engine::SharedRegistry) {let context=super::geometry_inference::GeometryInferenceContext::new(super::Session::new().capture());let scope=context.registry().clone();(owned_bundle(context),scope)}

    struct BrepExtensionResources { context:super::geometry_inference::GeometryInferenceContext }

    impl BrepExtensionResources {
        fn invoke_bytes(&self, capability: &str, req: &[u8]) -> Result<Vec<u8>, Fault> {
            match capability {
                "tessellate" => {
                    let request = semio_framework_pack_json::parse_bytes(req, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.tessellate.bad-request"), err.to_string()))?;
                    let handle = request.get("handle").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.tessellate.bad-request"), "missing field `handle`".to_string()))?;
                    let tolerance = request.get("tolerance").and_then(semio_framework_pack_json::Value::as_f64).unwrap_or(0.05);
                    let budget = request.get("budget").and_then(semio_framework_pack_json::Value::as_f64).map_or(TESSELLATE_STEP_BUDGET, |value| (value as usize).max(1));
                    let wall_micros = request.get("wallMicros").and_then(semio_framework_pack_json::Value::as_f64).map_or(flow_extension_sdk::mesh::TESSELLATE_STEP_WALL_MICROS, |value| value.max(0.0) as u64);
                    let chunk = request.get("chunk").and_then(semio_framework_pack_json::Value::as_f64).map_or(0, |value| value.max(0.0) as usize);
                    Ok(self.context.session().tessellate_step_envelope_json(handle, tolerance, budget, wall_micros, chunk).into_bytes())
                },
                "evaluateCancel" => {
                    let request = semio_framework_pack_json::parse_bytes(req, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate-cancel.bad-request"), err.to_string()))?;
                    let retired = match (request.get("operatorId").and_then(semio_framework_pack_json::Value::as_str), request.get("nodeHash").and_then(semio_framework_pack_json::Value::as_f64)) {
                        (Some(operator_id), Some(node_hash)) => usize::from(flow_extension_sdk::cancel_evaluation(self.context.registry(),operator_id, node_hash.max(0.0) as u64)),
                        _ => flow_extension_sdk::cancel_all_evaluations(self.context.registry()),
                    };
                    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("ok".to_string(), semio_framework_pack_json::Value::Bool(true)), ("retired".to_string(), semio_framework_pack_json::Value::from(retired as u64)), ("pending".into(),semio_framework_pack_json::Value::Bool(flow_extension_sdk::evaluation_retirement_pending(self.context.registry())))])).into_bytes())
                },
                "tessellateCancel" => {
                    let request = semio_framework_pack_json::parse_bytes(req, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.tessellate-cancel.bad-request"), err.to_string()))?;
                    let retired = match (request.get("handle").and_then(semio_framework_pack_json::Value::as_str), request.get("tolerance").and_then(semio_framework_pack_json::Value::as_f64)) {
                        (Some(handle), Some(tolerance)) => usize::from(self.context.session().cancel_tessellation(handle, tolerance)),
                        _ => self.context.session().cancel_all_tessellations(),
                    };
                    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("ok".to_string(), semio_framework_pack_json::Value::Bool(true)), ("retired".to_string(), semio_framework_pack_json::Value::from(retired as u64))])).into_bytes())
                },
                _ => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.unknown-capability"), "unknown BREP capability")),
            }
        }
    }

    impl ExtensionResourceOwner for BrepExtensionResources {
        fn invoke(&self, capability: &str, request: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::ExtensionInvokeStep, Fault> {
            match capability {
                "evaluate" => self.context.evaluate_raw(request, cx),
                _ => self.invoke_bytes(capability, request).map(|payload| semio_framework_plugin::ExtensionInvokeStep { payload: Some(payload), retained_progress: Default::default(), refusal: None }),
            }
        }
        fn inference_context(&self) -> Option<&dyn std::any::Any> { Some(&self.context) }
        fn cancel_inference(&self, cancellation_id: &str) -> bool { flow_extension_sdk::cancel_inference_evaluation(self.context.registry(),cancellation_id) }
        fn begin_close(&mut self) { self.context.begin_close(); }
        fn close_step(&mut self, grant:semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {self.context.close_step(grant)}
        fn terminal_is_empty(&self) -> bool { self.context.terminal_is_empty() }
        fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {self.context.retirement_demands(copy)}
        fn cancel_close(&mut self) { if !self.context.session().terminal_is_empty() { self.context.session().cancel_close(); } }
        fn resume_close(&mut self) { if !self.context.session().terminal_is_empty() { self.context.session().resume_close(); } }
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️extension-guest-standalone/🦀️.rs");

    semio_framework_plugin::extension_exports!({ let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }, bundle);
}
// #endregion 🔖️ExtensionGuest

#[cfg(test)]
use semio_framework_3d::brep::engine::BREP_KERNEL_OPERATIONS;

fn out_step() -> ChannelSpec { ChannelSpec::named("S","Stp","step","StepExport").with_value_types(&["text"]) }

fn operation_quality(method:&str) -> semio_framework_3d::brep::engine::OpQuality {
    if semio_s_artifact_stdio_step::geometry::STEP_GEOMETRY_OPERATIONS.contains(&method) { semio_s_artifact_stdio_step::geometry::operation_quality(method) } else { semio_framework_3d::brep::engine::operation_quality(method) }
}
