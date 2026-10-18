//! 🔷️ Flow brep extension — geometry operators packaged as a runtime-installable unit.

use semio_s_spatial_kernel_semio_session::*;
use flow_extension_sdk::mesh::*;
use flow_extension_sdk::build_manifest_json;
use neural_engine::{channel_output, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};
use semio_framework_3d_brep_boolean::BooleanOp;
use semio_s_plugin_flow_brep_mesh::retire_geometry_capture;
use semio_framework_3d_brep_engine::{BrepBooleanAdmission, BrepBooleanJob, BrepBooleanStep, BrepKernel};

#[path="📐️tessellate/🦀️.rs"]
mod tessellation;

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
pub const NODE_KERNEL_METHOD: &[(&str, &str)] = &[
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

macro_rules! geo_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        pub(super) struct $name(pub(super) SessionCapture);
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
        pub(super) struct $name(pub(super) SessionCapture);
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
        pub(super) struct $name(pub(super) SessionCapture);
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
        pub(super) struct $name(pub(super) SessionCapture);
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
        pub(super) struct $name(pub(super) SessionCapture);
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

#[path = "🔶️primitives-curves-surfaces-and-sweeps/🦀️.rs"]
mod primitives_curves_surfaces_and_sweeps;
use primitives_curves_surfaces_and_sweeps::{ArcCurve, BoxPrim, CircleCurve, ConePrim, CylinderPrim, EllipseCurve, HelicalSweep, HelixCurve, LineCurve, OffsetFace, PlanarFaceWire, PlaneSurface, RectangleWire, RegularPolygonWire, Revolve, SpherePrim, Sweep, ThickenFace, TorusPrim};
use primitives_curves_surfaces_and_sweeps::{
    ApproximateCurve, ConvexHullPrim, CoonsPatch, ExtrudeCurve, ExtrudeFace, InterpolateCurve, Loft,
    NurbsGridSurface, Pipe, PlanarFacePoints, PolylineWire,
};

#[path = "🔪️booleans-and-transforms/🦀️.rs"]
mod booleans_and_transforms;
use booleans_and_transforms::{CircularPattern, CopyShape, Cut, Fuse, GridPattern, Intersect, LinearPattern, Mirror, Rotate, RotateAbout, Scale, Translate};
use booleans_and_transforms::CompoundCut;

#[path = "🧲️features-intersect-evaluate-and-topology/🦀️.rs"]
mod features_intersect_evaluate_and_topology;
use features_intersect_evaluate_and_topology::{Chamfer, ChamferAsymmetric, CurveCurvature, CurvePoint, CurveTangent, Fillet, FilletVariable, OffsetSolid, SurfaceNormal, SurfacePoint};
use features_intersect_evaluate_and_topology::{
    ChamferEdges, CompoundOf, CurveClosestParameter, CurveCurveIntersect, CurveDomain,
    CurveSurfaceIntersect, Defeature, Draft, Explode, FilletEdges, GeometryLabel, Section,
    ShellMutation, SolidShells, Split, SurfaceClosestUv, SurfaceSurfaceIntersect,
};

#[path = "📏️measure-utilities-and-io/🦀️.rs"]
mod measure_utilities_and_io;
use measure_utilities_and_io::{Area, BoundingBox, CenterOfMass, ConvertToNurbs, Distance, FaceFromWire, HealSolid, Length, Validate, Vertex, Volume};
use measure_utilities_and_io::{
    ClassifyPoint, ClosestPoint, ExportDwg, ExportObj, ExportStep, ExportStl, ImportDwg, ImportObj,
    ImportStep, ImportStl, SewFaces,
};

#[path = "🧾️register-operators/🦀️.rs"]
mod register_operators;
pub use register_operators::{extension_manifest_json, module_registry, register};

// #endregion 🔖️Primitives

// #region 🔖️Topology
use semio_framework_3d_brep_engine::{Brep, GeometryHandle};

// #endregion 🔖️Topology

use semio_s_plugin_flow_brep_mesh as mesh;

#[path = "💡️inferences/📐️geometry/🦀️.rs"]
pub mod geometry_inference;

// #region 🔖️ExtensionGuest
pub mod extension_guest {
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

    pub fn bundle() -> ExtensionBundle {
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

    pub fn scoped_bundle()->(ExtensionBundle,neural_engine::SharedRegistry) {let context=super::geometry_inference::GeometryInferenceContext::new(super::Session::new().capture());let scope=context.evaluation().registry().expect("original fixture must admit its registry before reader handoff").clone();(owned_bundle(context),scope)}

    struct BrepExtensionResources { context:super::geometry_inference::GeometryInferenceContext }

    impl BrepExtensionResources {
        fn invoke_bytes(&self, capability: &str, req: &[u8]) -> Result<Vec<u8>, Fault> {
            match capability {
                "evaluateCancel" => {
                    let request = semio_framework_pack_json::parse_bytes(req, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate-cancel.bad-request"), err.to_string()))?;
                    let evaluation=self.context.evaluation();let registry=evaluation.registry();
                    let retired = match (request.get("operatorId").and_then(semio_framework_pack_json::Value::as_str), request.get("nodeHash").and_then(semio_framework_pack_json::Value::as_f64)) {
                        (Some(operator_id), Some(node_hash)) => usize::from(registry.is_some_and(|registry|flow_extension_sdk::cancel_evaluation(registry,operator_id,node_hash.max(0.0)as u64))),
                        _ => registry.map_or(0,flow_extension_sdk::cancel_all_evaluations),
                    };
                    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("ok".to_string(), semio_framework_pack_json::Value::Bool(true)), ("retired".to_string(), semio_framework_pack_json::Value::from(retired as u64)), ("pending".into(),semio_framework_pack_json::Value::Bool(registry.is_some_and(flow_extension_sdk::evaluation_retirement_pending)))])).into_bytes())
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
        fn invoke(&mut self, capability: &str, request: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::ExtensionInvokeStep, Fault> {
            match capability {
                "evaluate" => self.context.evaluate_raw(request, cx),
                "tessellate" => self.context.tessellate_raw(request, cx),
                _ => self.invoke_bytes(capability, request).map(|payload| semio_framework_plugin::ExtensionInvokeStep { payload: Some(payload), retained_progress: Default::default(), refusal: None }),
            }
        }
        fn inference_context(&self) -> Option<&dyn std::any::Any> { Some(&self.context) }
        fn cancel_inference(&self, cancellation_id: &str) -> bool { self.context.evaluation().registry().is_some_and(|registry|flow_extension_sdk::cancel_inference_evaluation(registry,cancellation_id)) }
        fn begin_close(&mut self) { self.context.begin_close(); }
        fn close_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure> {self.context.close_step(cx)}
        fn terminal_is_empty(&self) -> bool { self.context.terminal_is_empty() }
        fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {self.context.retirement_demands(copy)}
        fn cancel_close(&mut self) { if !self.context.session().terminal_is_empty() { self.context.session().cancel_close(); } }
        fn resume_close(&mut self) { if !self.context.session().terminal_is_empty() { self.context.session().resume_close(); } }
    }

}
// #endregion 🔖️ExtensionGuest

fn out_step() -> ChannelSpec { ChannelSpec::named("S","Stp","step","StepExport").with_value_types(&["text"]) }

pub fn operation_quality(method:&str) -> semio_framework_3d_brep_representation::contract::OpQuality {
    if semio_s_artifact_stdio_step_geometry::geometry::STEP_GEOMETRY_OPERATIONS.contains(&method) { semio_s_artifact_stdio_step_geometry::geometry::operation_quality(method) } else { semio_framework_3d_brep_representation::contract::operation_quality(method) }
}
