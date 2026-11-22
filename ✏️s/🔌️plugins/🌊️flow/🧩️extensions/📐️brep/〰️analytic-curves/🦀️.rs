//! 〰️ Original geometry production over admitted session and dictionary storage.

use std::mem::{size_of, take};

use neural_engine::{Atom, BudgetedOwnedEntry, Dictionary, EvalError, OperatorJob, OperatorJobStep, OperatorPlanAdmission, OperatorProgress, OperatorWorkFailure, Value};
use semio_framework_3d_brep_engine::{AnalyticCurveParameters, AnalyticPublication, AnalyticTransform, BooleanOperand, BooleanOperation, BooleanPreparation, CertifiedBooleanResult, BoxFinish, Brep, BoxSectionDraft, SectionPolygon, ShellDraft, ConicFacePublication, CuboidSew, HelixWireRecipe, HullDraft, LoftDraft, LoftPlan, PolygonFacePublication, PolygonSectionSet, PolygonWirePublication, PolygonWireRecipe, PrismDraft, PlanarExtrusion, PlanarMeasureJob, WireLengthJob, SECTION_FACE_LIMIT, SectionPlan, SolidPrimitive, SolidPublication, VertexPublication};
use semio_framework_job::StepContext;
use semio_framework_3d_brep_engine::CompoundPublication;
use semio_framework_3d_brep_engine::{SolidPatternPublication, SolidPatternRecipe};
use semio_framework_3d_brep_engine::{MeshDecode, MeshPublication, MeshTextFormat, ObjDecode, StlDecode};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_s_artifact_stdio_step_geometry::geometry::{StepWrite, StepSolid, StepSolidDecode, StepWriteSource};
use semio_s_spatial_kernel_semio_session::{NativeMeshExport,NativeValidationReport,SolidHealing,Session,SessionCapture,classify_number};

#[path = "🧾️topology-output/🦀️.rs"]
mod topology_output;
use topology_output::TopologyOutput as TopologyListOutput;
#[path = "🧊️deconstruct/🦀️.rs"]
mod deconstruct;
use deconstruct::DeconstructOutput;
#[path = "✂️split/🦀️.rs"]
mod split;
#[path = "✂️section-faces/🦀️.rs"]
mod section_faces;
#[path = "🔪️compound-cut/🦀️.rs"]
mod compound_cut;
use compound_cut::CompoundCutDraft;

#[derive(semio_framework_value::RetireOwned)]
enum TopologyOutput {
    Ordered(TopologyListOutput),
    Deconstruct(DeconstructOutput),
}

impl TopologyOutput {
    fn phase(&self) -> &'static str {
        match self { Self::Ordered(job) => job.phase(), Self::Deconstruct(job) => job.phase() }
    }

    fn demand(&self, session: &Session, input: &Dictionary, copy: usize) -> Result<RetirementDemand, ValueError> {
        match self { Self::Ordered(job) => job.demand(session, None, copy), Self::Deconstruct(job) => job.demand(session, input, copy) }
    }

    fn step(&mut self, session: &Session, input: &Dictionary, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<Dictionary>, Progress), ValueError> {
        match self { Self::Ordered(job) => job.step(session, None, effect), Self::Deconstruct(job) => job.step(session, input, effect) }
    }

    fn staged_claim(&self) -> Option<&str> {
        match self { Self::Ordered(job) => job.staged_claim(), Self::Deconstruct(job) => job.staged_claim() }
    }

    fn close_claim(&mut self, session: &Session, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        match self { Self::Ordered(job) => job.close_claim(session, effect), Self::Deconstruct(job) => job.close_claim(session, effect) }
    }
}

use semio_framework_3d_brep_engine::TopologyList;

#[derive(Clone, Copy)]
/// 🧭️ Selects one original geometry operator family.
pub enum GeometryOperation {
    Vertex,
    CurveCurve,
    CurveSurface,
    SurfaceSurface,
    Line,
    Circle,
    Ellipse,
    Arc,
    Helix,
    Interpolate,
    Approximate,
    Plane,
    NurbsGrid,
    Coons,
    Compound,
    Shells,
    Explode,
    Deconstruct,
    Rectangle,
    Polyline,
    RegularPolygon,
    Box,
    Sphere,
    Cylinder,
    ExtrudeWire,
    ExtrudeFace,
    Sweep,
    Pipe,
    Loft,
    Revolve,
    HelicalSweep,
    Thicken,
    OffsetFace,
    OffsetSolid,
    Shell,
    Sew,
    ConvertToNurbs,
    Heal,
    ChamferAll,
    FilletAll,
    ChamferEdges,
    FilletEdges,
    Draft,
    Defeature,
    Section,
    Split,
    ConvexHull,
    BoundingBox,
    ClassifyPoint,
    Distance,
    Fuse,
    Cut,
    CompoundCut,
    Intersect,
    ConicFace,
    PlanarConicFace,
    PlanarFacePoints,
    Translate,
    Rotate,
    RotateAbout,
    Scale,
    Mirror,
    Copy,
    LinearPattern,
    CircularPattern,
    GridPattern,
    ImportObj,
    ImportStl,
    ImportStep,
    ExportStep,
    ExportObj,
    ExportStl,
    Cone,
    Torus,
    Point,
    Tangent,
    Curvature,
    Domain,
    SurfacePoint,
    SurfaceNormal,
    CurveClosestParameter,
    SurfaceClosestUv,
    ClosestPoint,
    Length,
    Volume,
    Area,
    CenterOfMass,
    Label,
    Validate,
}

impl GeometryOperation {
    fn is_solid(self) -> bool {
        matches!(self, Self::Box | Self::Sphere | Self::Cylinder | Self::Cone | Self::Torus | Self::ExtrudeWire | Self::ExtrudeFace | Self::Sweep | Self::Pipe | Self::Loft | Self::Revolve | Self::HelicalSweep | Self::Thicken | Self::OffsetSolid | Self::BoundingBox | Self::Shell | Self::ConvexHull)
    }

    fn initial_phase(self) -> &'static str {
        match self {
            Self::Line | Self::Circle | Self::Ellipse | Self::Arc => "analytic-curve-parameters",
            Self::Vertex => "vertex-point",
            Self::CurveCurve | Self::CurveSurface | Self::SurfaceSurface => "linear-intersection",
            Self::Plane => "analytic-plane-parameters",
            Self::NurbsGrid => "nurbs-grid-source",
            Self::Coons => "coons-boundary-source",
            Self::Compound=>"compound-input-list",
            Self::LinearPattern | Self::CircularPattern | Self::GridPattern => "solid-pattern-source",
            Self::ImportObj => "obj-source",
            Self::ImportStl => "stl-source",
            Self::ImportStep => "step-source",
            Self::ExportStep => "step-export-source",
            Self::ExportObj | Self::ExportStl => "mesh-export-source",
            Self::Validate=>"validation-source",
            Self::Shells => "shells-source",
            Self::Explode => "explode-source",
            Self::Deconstruct => "deconstruct-source",
            Self::Interpolate => "curve-interpolation-source",
            Self::Approximate => "curve-approximation-source",
            Self::Rectangle | Self::Polyline | Self::RegularPolygon | Self::Helix => "polygon-wire-source",
            Self::ExtrudeWire | Self::ExtrudeFace => "extrusion-profile",
            Self::Sweep | Self::Pipe => "straight-sweep-path",
            Self::Loft => "loft-profile-pair",
            Self::Revolve => "revolution-profile",
            Self::HelicalSweep => "closed-helical-profile",
            Self::Thicken => "thickening-profile",
            Self::OffsetFace => "face-offset-source",
            Self::OffsetSolid => "solid-offset-source",
            Self::Shell => "shell-source",
            Self::Sew => "sew-source",
            Self::ConvertToNurbs => "nurbs-cuboid-source",
            Self::Heal => "healing-source",
            Self::ChamferAll | Self::ChamferEdges => "box-chamfer-source",
            Self::FilletAll | Self::FilletEdges => "box-fillet-source",
            Self::Draft => "box-draft-source",
            Self::Defeature => "rectangular-stock-feature-source",
            Self::Section => "section-source",
            Self::Split => "split-source",
            Self::ConvexHull => "convex-hull-source",
            Self::BoundingBox => "bounding-box-source",
            Self::Fuse | Self::Cut | Self::Intersect => "boolean-operands",
            Self::CompoundCut => "compound-cut-target",
            Self::Translate | Self::Rotate | Self::RotateAbout | Self::Scale | Self::Mirror | Self::Copy => "analytic-transform-source",
            Self::ConicFace | Self::PlanarConicFace => "conic-face-profile",
            Self::PlanarFacePoints => "polygon-face-points",
            operation if operation.is_solid() => "solid-primitive-parameters",
            _ => "geometry-query-source",
        }
    }

    fn sweep_guide(self, input: &Dictionary) -> Option<&str> {
        matches!(self, Self::Pipe).then(|| geometry_handle_value(input.get("guide"))).flatten()
    }

    fn output_bytes(self, input: &Dictionary) -> usize {
        match self {
            Self::Rectangle | Self::RegularPolygon => 2 * size_of::<f64>(),
            Self::Polyline | Self::PlanarFacePoints => 2 * size_of::<[f64; 3]>(),
            Self::Line | Self::Plane | Self::ClosestPoint => 6 * size_of::<f64>(),
            Self::NurbsGrid => 3 * size_of::<f64>(),
            Self::Interpolate => size_of::<f64>(),
            Self::Approximate => 2 * size_of::<f64>(),
            Self::Coons => 4 * size_of::<usize>(),
            Self::Compound|Self::Validate|Self::Shells=>size_of::<usize>(),
            Self::LinearPattern | Self::CircularPattern | Self::GridPattern => 64 + size_of::<SolidPrimitive>() + size_of::<SolidPatternRecipe>(),
            Self::ImportStep => size_of::<(usize, usize)>(),
            Self::ExportStep => size_of::<StepWriteSource>(),
            Self::ImportObj | Self::ImportStl => size_of::<(usize, f64)>(),
            Self::ExportObj | Self::ExportStl => size_of::<SolidPrimitive>() + size_of::<f64>(),
            Self::Explode => 64 + size_of::<usize>(),
            Self::Deconstruct => 64 + 64,
            Self::Circle => 7 * size_of::<f64>(),
            Self::Ellipse => 8 * size_of::<f64>(),
            Self::Arc | Self::Helix => 9 * size_of::<f64>(),
            Self::Box | Self::Vertex => 3 * size_of::<f64>(),
            Self::CurveCurve | Self::CurveSurface => 13 * size_of::<f64>(),
            Self::SurfaceSurface => 14 * size_of::<f64>(),
            Self::OffsetSolid => size_of::<SolidPrimitive>() + size_of::<f64>(),
            Self::Shell => 16 * size_of::<f64>(),
            Self::Sew | Self::Heal => size_of::<f64>(),
            Self::ConvertToNurbs => size_of::<SolidPrimitive>(),
            Self::FilletAll => 12 * size_of::<f64>(),
            Self::ChamferAll => 16 * size_of::<f64>(),
            Self::ChamferEdges | Self::FilletEdges => 16 * size_of::<f64>(),
            Self::Draft => 15 * size_of::<f64>(),
            Self::Defeature => size_of::<SolidPrimitive>(),
            Self::Section => 16 * size_of::<f64>(),
            Self::Split => 12 * size_of::<f64>(),
            Self::ClassifyPoint => 4 * size_of::<f64>(),
            Self::Distance => 13 * size_of::<f64>(),
            Self::BoundingBox => 16 * size_of::<f64>(),
            Self::Pipe if self.sweep_guide(input).is_some() => 24 * size_of::<f64>(),
            Self::ExtrudeWire | Self::ExtrudeFace | Self::Sweep | Self::Pipe | Self::Loft => 16 * size_of::<f64>(),
            Self::Revolve => 14 * size_of::<f64>(),
            Self::HelicalSweep => 10 * size_of::<f64>(),
            Self::Thicken => 16 * size_of::<f64>(),
            Self::ConicFace | Self::PlanarConicFace | Self::OffsetFace => 14 * size_of::<f64>(),
            Self::Translate => 3 * size_of::<f64>(),
            Self::Rotate => 4 * size_of::<f64>(),
            Self::RotateAbout => 7 * size_of::<f64>(),
            Self::Scale | Self::Mirror => 6 * size_of::<f64>(),
            Self::Copy | Self::ConvexHull => 0,
            Self::Fuse | Self::Cut | Self::Intersect => BooleanOperand::recipe_copy_bytes(),
            Self::CompoundCut => size_of::<([[f64; 3]; 2], usize)>(),
            Self::Sphere => size_of::<f64>(),
            Self::Cylinder | Self::Cone | Self::Torus => 2 * size_of::<f64>(),
            Self::Point | Self::Tangent => 4 * size_of::<f64>(),
            Self::CurveClosestParameter|Self::SurfaceClosestUv=>9*size_of::<f64>(),
            Self::Curvature => 2 * size_of::<f64>(),
            Self::SurfacePoint | Self::SurfaceNormal => 5 * size_of::<f64>(),
            Self::Domain | Self::Length | Self::Volume | Self::Area | Self::Label => size_of::<f64>(),
            Self::CenterOfMass => 3 * size_of::<f64>(),
        }
    }

    fn output_capacity(self)->usize{
        match self {
            Self::Validate => size_of::<NativeValidationReport>(),
            Self::Heal => size_of::<SolidHealing>(),
            Self::ExportObj | Self::ExportStl => size_of::<NativeMeshExport>(),
            Self::ImportStep => size_of::<StepSolidDecode>(),
            Self::ExportStep => size_of::<StepWrite>(),
            Self::Split => size_of::<[SolidPublication; 2]>(),
            Self::Section => size_of::<PolygonSectionSet>(),
            Self::Shells | Self::Explode | Self::Deconstruct => size_of::<TopologyOutput>(),
            _ => 0,
        }
    }

    fn output(self, session: &Session, input: &Dictionary) -> Result<GeometryOutput, ValueError> {
        let parameters = match self {
            Self::Shells | Self::Explode => {
                let (port, kind) = if matches!(self, Self::Shells) { ("solid", TopologyList::Shells) } else { ("compound", TopologyList::Solids) };
                let plan = session.plan_topology_selection(geometry_handle(input, port)?, kind)?;
                return Ok(GeometryOutput::TopologyList(Box::new(TopologyOutput::Ordered(TopologyListOutput::new(plan)))));
            }
            Self::Deconstruct => {
                let job = DeconstructOutput::new(session, input)?;
                return Ok(GeometryOutput::TopologyList(Box::new(TopologyOutput::Deconstruct(job))));
            }
            Self::Validate=>{
                let handle=geometry_handle(input,"geometry")?;
                return session.with_original_geometry_query(|kernel| NativeValidationReport::new(kernel, handle)).map(|report|GeometryOutput::Validation(Box::new(report)));
            }
            Self::Compound=>return session.plan_compound(compound_members(input)?.len().saturating_sub(1)).map(GeometryOutput::Compound),
            Self::LinearPattern | Self::CircularPattern | Self::GridPattern => {
                return session.plan_solid_pattern(geometry_handle(input, "geometry")?, self.pattern_recipe(input)?).map(GeometryOutput::SolidPattern);
            }
            Self::ImportStep => return Ok(GeometryOutput::StepDecode(Box::new(StepSolidDecode::new(exchange_text(input)?)))),
            Self::ImportObj | Self::ImportStl => {
                let source = exchange_text(input)?;
                let tolerance = number(input, "tolerance", "value")?;
                let decoder = if matches!(self, Self::ImportObj) { MeshDecode::Obj(ObjDecode::new(source, tolerance)?) }
                    else { MeshDecode::Stl(StlDecode::new(source, tolerance)?) };
                return Ok(GeometryOutput::MeshDecode(decoder));
            }
            Self::ExportStep => return step_export_source(session, input, |source| {
                Ok(GeometryOutput::StepExport(Box::new(StepWrite::new(source))))
            }),
            Self::ExportObj | Self::ExportStl => {
                let format = if matches!(self, Self::ExportObj) { MeshTextFormat::Obj } else { MeshTextFormat::Stl };
                let source = geometry_handle(input, "geometry")?;
                let tolerance = number(input, "deflection", "value")?;
                return session.plan_mesh_export(source, tolerance, format).map(|job| GeometryOutput::MeshExport(Box::new(job)));
            }
            Self::Vertex => return session.plan_vertex_publication(point(input, "point")?).map(|vertex| GeometryOutput::Vertex(vertex, "vertex")),
            Self::CurveCurve | Self::CurveSurface => {
                let tolerance = number(input, "tolerance", "value").map_err(|_| {
                    ValueError::literal(ValueRefusalKind::InvalidValue, "tolerance must be positive and finite")
                })?;
                let points = if matches!(self, Self::CurveCurve) {
                    session.original_curve_curve_contacts(geometry_handle(input, "a")?, geometry_handle(input, "b")?, tolerance)?
                } else {
                    session.original_curve_plane_contacts(geometry_handle(input, "curve")?, geometry_handle(input, "surface")?, tolerance)?
                };
                let count = points.iter().flatten().count();
                let points = points.map(|point| point.unwrap_or_default());
                return match count {
                    0 => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "no intersection")),
                    1 => session.plan_vertex_publication(points[0]).map(|vertex| GeometryOutput::Vertex(vertex, "wire")),
                    _ => session.plan_polygon_wire(PolygonWireRecipe::Contacts { points, count }).map(|wire| GeometryOutput::Wire(wire, "wire")),
                };
            }
            Self::SurfaceSurface => {
                let tolerance = number(input, "tolerance", "value")?;
                let curve = session.plan_original_surface_intersection(geometry_handle(input, "a")?, geometry_handle(input, "b")?, tolerance)?;
                return Ok(GeometryOutput::Geometry(curve, "wires"));
            }
            Self::Line => AnalyticCurveParameters::Line { start: point(input, "start")?, end: point(input, "end")? },
            Self::Circle => AnalyticCurveParameters::Circle {
                center: point(input, "center")?, normal: point(input, "normal")?, radius: number(input, "radius", "value")?,
            },
            Self::Ellipse => AnalyticCurveParameters::Ellipse {
                center: point(input, "center")?, normal: point(input, "normal")?,
                major_radius: number(input, "semiMajor", "value")?, minor_radius: number(input, "semiMinor", "value")?,
            },
            Self::Arc => {
                let shape=session.plan_analytic_arc(point(input,"center")?,point(input,"normal")?,number(input,"radius","value")?,number(input,"startAngle","value")?,number(input,"endAngle","value")?)?;
                return Ok(GeometryOutput::Geometry(shape,"curve"));
            }
            Self::Helix => {
                let helix = HelixWireRecipe::new(point(input, "origin")?, point(input, "axis")?,
                    number(input, "radius", "value")?, number(input, "pitch", "value")?, number(input, "turns", "value")?)?;
                return session.plan_polygon_wire(PolygonWireRecipe::Helix(helix)).map(|wire| GeometryOutput::Wire(wire, "curve"));
            }
            Self::Rectangle | Self::Polyline | Self::RegularPolygon => {
                let recipe = match self {
                    Self::Rectangle => PolygonWireRecipe::Rectangle {
                        width: number(input, "width", "value")?, height: number(input, "height", "value")?,
                    },
                    Self::RegularPolygon => {
                        let sides = number(input, "sides", "value")?;
                        if sides.fract() != 0.0 || !(3.0..=65_536.0).contains(&sides) {
                            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "polygon sides must be an integer from 3 through 65536"));
                        }
                        PolygonWireRecipe::Regular { radius: number(input, "radius", "value")?, sides: sides as usize }
                    }
                    Self::Polyline => {
                        let list = point_list(input)?;
                        let count = list.len().saturating_sub(1);
                        if !(2..=65_537).contains(&count) { return Err(invalid_point_list()); }
                        let first = listed_point(list, 0)?;
                        let last = listed_point(list, count - 1)?;
                        let closed = first == last;
                        PolygonWireRecipe::Polyline { vertices: count - usize::from(closed), closed }
                    }
                    _ => unreachable!("selected operation constructs a polygonal wire"),
                };
                return session.plan_polygon_wire(recipe).map(|wire| GeometryOutput::Wire(wire, "wire"));
            }
            Self::Interpolate => {
                let count = point_list(input)?.len().saturating_sub(1);
                let degree = number(input, "degree", "value")? as usize;
                return session.plan_interpolated_curve(count, degree).map(|shape| GeometryOutput::Geometry(shape, "curve"));
            }
            Self::Approximate => {
                let count = point_list(input)?.len().saturating_sub(1);
                let degree = number(input, "degree", "value")? as usize;
                let controls = number(input, "controlPoints", "value")? as usize;
                return session.plan_approximated_curve(count, degree, controls).map(|shape| GeometryOutput::Geometry(shape, "curve"));
            }
            Self::Coons => {
                let source = nested_boundary_list(input)?;
                if source.len() != 5 { return Err(invalid_boundary_list()); }
                let mut sizes = [0; 4];
                for (index, size) in sizes.iter_mut().enumerate() {
                    *size = boundary_point_list(source, index)?.len().saturating_sub(1);
                }
                return session.plan_coons_surface(sizes).map(|shape| GeometryOutput::Geometry(shape, "surface"));
            }
            Self::NurbsGrid => {
                let points = point_list(input)?;
                let size = |channel| number(input, channel, "value");
                let rows = size("rows")?;
                let count = points.len().saturating_sub(1);
                if rows.fract() != 0. || !(2. ..=count as f64).contains(&rows) || count % rows as usize != 0 {
                    return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "NURBS grid requires equal rows of ordered points"));
                }
                let rows = rows as usize;
                return session.plan_interpolated_grid(rows, count / rows, size("degreeU")? as usize, size("degreeV")? as usize)
                    .map(|shape| GeometryOutput::Geometry(shape, "surface"));
            }
            Self::Plane => return session.plan_analytic_plane(point(input, "origin")?, point(input, "normal")?).map(|shape| GeometryOutput::Geometry(shape,"surface")),
            Self::Translate | Self::Rotate | Self::RotateAbout | Self::Scale | Self::Mirror | Self::Copy => {
                let transform = match self {
                    Self::Translate => AnalyticTransform::Translate(point(input,"offset")?),
                    Self::Rotate | Self::RotateAbout => AnalyticTransform::Rotate {
                        origin: if matches!(self,Self::RotateAbout) { point(input,"origin")? } else { [0.0;3] },
                        axis: point(input,"axis")?, angle:number(input,"angle","value")?,
                    },
                    Self::Scale => AnalyticTransform::Scale { center:point(input,"center")?, factors:point(input,"factor")? },
                    Self::Mirror => AnalyticTransform::Mirror { origin:point(input,"origin")?, normal:point(input,"normal")? },
                    Self::Copy => AnalyticTransform::Copy,
                    _ => unreachable!(),
                };
                return Ok(GeometryOutput::Transform(transform));
            }
            Self::CompoundCut => return CompoundCutDraft::new(session, input).map(GeometryOutput::CompoundCut),
            Self::Fuse | Self::Cut | Self::Intersect => {
                let operation = match self {
                    Self::Fuse => BooleanOperation::Unite,
                    Self::Cut => BooleanOperation::Cut,
                    _ => BooleanOperation::Intersect,
                };
                let a = geometry_handle(input, "a")?;
                let b = geometry_handle(input, "b")?;
                return match session.capture_boolean_operand(a) {
                    Ok(first) => Ok(GeometryOutput::Boolean { first, operation }),
                    Err(error) if error.kind == ValueRefusalKind::UnsupportedOwner =>
                        session.plan_boolean_preparation(a,b).map(GeometryOutput::BooleanPreparation),
                    Err(error) => Err(error),
                };
            }
            Self::Loft => {
                number(input, "smooth", "value")?;
                let loft = match session.plan_exact_loft(loft_profiles(input)?)? {
                    LoftPlan::Solid(shape) => GeometryOutput::Solid(shape, "solid"),
                    LoftPlan::Lines(draft) => GeometryOutput::Loft(draft),
                };
                return Ok(loft);
            }
            Self::Sweep | Self::Pipe => {
                let profile = geometry_handle(input, "profile")?;
                let path = geometry_handle(input, "path")?;
                let swept = match session.plan_straight_sweep(profile, path, self.sweep_guide(input))? {
                    PlanarExtrusion::Solid(shape) => GeometryOutput::Solid(shape, "solid"),
                    PlanarExtrusion::Prism(draft) => GeometryOutput::Prism(draft),
                };
                return Ok(swept);
            }
            Self::Thicken => {
                let source = geometry_handle(input, "face")?;
                let thickness = number(input, "thickness", "value")?;
                let solid = match session.plan_planar_thickening(source, thickness)? {
                    PlanarExtrusion::Solid(shape) => GeometryOutput::Solid(shape, "solid"),
                    PlanarExtrusion::Prism(draft) => GeometryOutput::Prism(draft),
                };
                return Ok(solid);
            }
            Self::HelicalSweep => {
                let source=geometry_handle(input,"profile")?;
                let origin=point(input,"axisOrigin")?;
                let direction=point(input,"axisDirection")?;
                let parameters=[number(input,"radius","value")?,number(input,"pitch","value")?,number(input,"turns","value")?];
                return session.plan_closed_helical_sweep(source,origin,direction,parameters)
                    .map(|shape|GeometryOutput::Solid(shape,"solid"));
            }
            Self::Revolve => {
                let source = geometry_handle(input, "face")?;
                let origin = point(input, "axisOrigin")?;
                let direction = point(input, "axisDirection")?;
                let angle = number(input, "angle", "value")?;
                return session.plan_circular_revolution(source, origin, direction, angle)
                    .map(|shape| GeometryOutput::Solid(shape, "solid"));
            }
            Self::ExtrudeWire | Self::ExtrudeFace => {
                let source = self.geometry_source(input)?.ok_or_else(|| ValueError::invariant("extrusion has no source port"))?;
                let vector = point(input, "vector")?;
                if let Some(solid) = session.plan_circular_extrusion(source, vector)? {
                    return Ok(GeometryOutput::Solid(solid, "solid"));
                }
                if let Some(solid) = session.plan_rectangular_extrusion(source, vector)? {
                    return Ok(GeometryOutput::Solid(solid, "solid"));
                }
                if let Some(draft) = session.plan_prism(source, vector)? {
                    return Ok(GeometryOutput::Prism(draft));
                }
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original extrusion requires a closed planar line or circular profile"));
            }
            Self::BoundingBox => {
                let source = geometry_handle(input, "geometry")?;
                return match session.plan_analytic_bounds(source) {
                    Ok(shape) => Ok(GeometryOutput::Solid(shape, "box")),
                    Err(error) if error.kind == ValueRefusalKind::UnsupportedOwner =>
                        session.plan_planar_bounds(source).map(|job| GeometryOutput::PlanarMeasure(job, "box")),
                    Err(error) => Err(error),
                };
            }
            Self::ConvexHull => {
                let draft = HullDraft::new(point_list(input)?.len().saturating_sub(1))?;
                return Ok(GeometryOutput::Hull(draft));
            }
            Self::Split => {
                let source = geometry_handle(input, "solid")?;
                let pair = session.plan_plane_split(source, point(input, "planeOrigin")?, point(input, "planeNormal")?)?;
                return Ok(GeometryOutput::SolidPair(Box::new(pair)));
            }
            Self::Section => {
                let source = geometry_handle(input, "solid")?;
                let plan=session.plan_section(source,point(input,"planeOrigin")?,point(input,"planeNormal")?)?;
                return Ok(match plan{
                    SectionPlan::Box(draft)=>GeometryOutput::Section(draft),
                    SectionPlan::Conic(face)=>GeometryOutput::Face(face,ConicFaceOutput::Section),
                    SectionPlan::Polygon(face,polygon)=>GeometryOutput::PolygonFace(face,FaceOutput::Section(polygon)),
                    SectionPlan::Regions(faces) => GeometryOutput::SectionFaces(Box::new(faces)),
                });
            }
            Self::ChamferAll | Self::FilletAll => {
                let source = geometry_handle(input, "geometry")?;
                let finish = if matches!(self,Self::FilletAll) { BoxFinish::FilletAll(number(input, "radius", "value")?) }
                    else { BoxFinish::ChamferAll(number(input, "distance", "value")?) };
                return session.plan_box_finish(source, &[], finish).map(|solid| GeometryOutput::Solid(solid, "solid"));
            }
            Self::Defeature => {
                let (shape, members, count, _) = defeature_source(input)?;
                return session.plan_defeature(shape, &members[..count]).map(|solid| GeometryOutput::Solid(solid, "solid"));
            }
            Self::ChamferEdges | Self::FilletEdges | Self::Draft => {
                let (shape,members,count)=box_finish_source(input,self)?;
                let finish = if matches!(self,Self::Draft) {
                    BoxFinish::Draft { pull:point(input,"pullDirection")?,neutral:point(input,"neutralPoint")?,angle:number(input,"angle","value")? }
                } else if matches!(self,Self::FilletEdges) {
                    BoxFinish::Fillet(number(input,"radius","value")?)
                } else { BoxFinish::Chamfer(number(input,"distance","value")?) };
                return session.plan_box_finish(shape,&members[..count],finish).map(|solid|GeometryOutput::Solid(solid,"solid"));
            }
            Self::Sew => {
                let list = sew_faces(input)?;
                if list.len() != 7 { return Err(invalid_sew_input()); }
                return CuboidSew::new(number(input, "tolerance", "value")?).map(GeometryOutput::Sew);
            }
            Self::Heal => {
                let source = geometry_handle(input, "geometry")?;
                let tolerance = number(input, "tolerance", "value")?;
                return session.with_original_geometry_query(|kernel| kernel.plan_healing(source, tolerance))
                    .map(|job| GeometryOutput::Healing(Box::new(job)));
            }
            Self::ConvertToNurbs => return session.plan_nurbs_cuboid(geometry_handle(input, "geometry")?)
                .map(|solid| GeometryOutput::Solid(solid, "geometryOut")),
            Self::Shell => {
                let source = geometry_handle(input, "geometry")?;
                shell_open_faces(input)?;
                let draft = session.plan_shell(source, number(input, "thickness", "value")?)?;
                return Ok(GeometryOutput::Shell { draft, next: 0 });
            }
            Self::OffsetSolid => {
                let source = geometry_handle(input, "geometry")?;
                let distance = number(input, "distance", "value")?;
                return session.plan_certified_solid_offset(source, distance)
                    .map(|shape| GeometryOutput::Solid(shape, "solid"));
            }
            Self::OffsetFace => {
                let source = geometry_handle(input, "face")?;
                let distance = number(input, "distance", "value")?;
                return if let Some(face) = session.plan_polygon_face_offset(source, distance)? {
                    Ok(GeometryOutput::PolygonFace(face, FaceOutput::Single("faceOut")))
                } else {
                    session.plan_conic_face_offset(source, distance).map(|shape| GeometryOutput::Face(shape, ConicFaceOutput::Single("faceOut")))
                };
            }
            Self::PlanarFacePoints => {
                let list = point_list(input)?;
                let count = list.len().saturating_sub(1);
                if !(3..=65_537).contains(&count) { return Err(invalid_point_list()); }
                let closed = listed_point(list, 0)? == listed_point(list, count - 1)?;
                return session.plan_polygon_face_points(count - usize::from(closed)).map(|face| GeometryOutput::PolygonFace(face, FaceOutput::Single("face")));
            }
            Self::ConicFace | Self::PlanarConicFace => {
                let source = self.geometry_source(input)?.ok_or_else(|| ValueError::invariant("conic face has no source port"))?;
                let xy_only = matches!(self, Self::PlanarConicFace);
                return if let Some(face) = session.plan_polygon_face(source, xy_only)? {
                    Ok(GeometryOutput::PolygonFace(face, FaceOutput::Single("face")))
                } else {
                    session.plan_conic_face(source, xy_only).map(|shape| GeometryOutput::Face(shape, ConicFaceOutput::Single("face")))
                };
            }
            Self::Box | Self::Sphere | Self::Cylinder | Self::Cone | Self::Torus => {
                let size = |channel| number(input, channel, "value");
                let primitive = match self {
                    Self::Box => SolidPrimitive::axis_box(size("width")?, size("depth")?, size("height")?),
                    Self::Sphere => SolidPrimitive::sphere(size("radius")?),
                    Self::Cylinder => SolidPrimitive::cylinder(size("radius")?, size("height")?),
                    Self::Cone => SolidPrimitive::cone(size("radius")?, size("height")?),
                    Self::Torus => SolidPrimitive::torus(size("major")?, size("minor")?),
                    _ => unreachable!("the selected operation produces a solid"),
                };
                return session.plan_solid_primitive(primitive).map(|shape| GeometryOutput::Solid(shape,"solid"));
            }
            _ => return self.query(session, input),
        };
        session.plan_analytic_curve(parameters).map(|shape| GeometryOutput::Geometry(shape,"curve"))
    }

    fn geometry_source(self, input: &Dictionary) -> Result<Option<&str>, ValueError> {
        let channel = match self {
            Self::Length | Self::Volume | Self::Area | Self::CenterOfMass | Self::Label | Self::ClosestPoint | Self::Validate | Self::Heal | Self::Translate | Self::Rotate | Self::RotateAbout | Self::Scale | Self::Mirror | Self::Copy | Self::OffsetSolid | Self::ChamferAll | Self::FilletAll | Self::BoundingBox | Self::Shell | Self::ConvertToNurbs | Self::LinearPattern | Self::CircularPattern | Self::GridPattern => "geometry",
            Self::ExtrudeWire | Self::ConicFace | Self::PlanarConicFace => "wire",
            Self::ExtrudeFace | Self::Revolve | Self::Thicken | Self::OffsetFace => "face",
            Self::Sweep | Self::Pipe | Self::HelicalSweep => "profile",
            Self::Loft => return loft_profiles(input).map(|[first, _]| Some(first)),
            Self::ClassifyPoint | Self::Shells | Self::Section | Self::Split => "solid",
            Self::Explode => "compound",
            Self::SurfacePoint | Self::SurfaceNormal | Self::SurfaceClosestUv => "surface",
            Self::Point | Self::Tangent | Self::Curvature | Self::Domain | Self::CurveClosestParameter => "curve",
            _ => return Ok(None),
        };
        geometry_handle(input, channel).map(Some)
    }

    fn source_depth(self, session: &Session, input: &Dictionary) -> Result<usize, ValueError> {
        if matches!(self, Self::ExportObj | Self::ExportStl | Self::ExportStep) { return session.certified_solid_depth(geometry_handle(input, "geometry")?); }
        if matches!(self, Self::CompoundCut) {
            return Ok(session.certified_solid_depth(geometry_handle(input, "target")?)?.max(dictionary_depth(input)));
        }
        if matches!(self, Self::ConvexHull | Self::Deconstruct) { return Ok(dictionary_depth(input)); }
        if matches!(self, Self::ChamferAll | Self::FilletAll) { return session.box_finish_depth(geometry_handle(input, "geometry")?, &[]); }
        if matches!(self, Self::Defeature) {
            let (shape, members, count, input_depth) = defeature_source(input)?;
            let native_depth = session.with_original_geometry_query(|kernel| kernel.defeature_depth(shape, &members[..count]))?;
            return Ok(input_depth.max(native_depth));
        }
        if matches!(self, Self::ChamferEdges | Self::FilletEdges | Self::Draft) {
            let (shape,members,count)=box_finish_source(input,self)?;
            return session.box_finish_depth(shape,&members[..count]);
        }
        if matches!(self, Self::CurveSurface) {
            let curve = session.original_intersection_curve_source_depth(&[geometry_handle(input, "curve")?])?;
            let surface = session.original_query_depth_demand(geometry_handle(input, "surface")?)?;
            return Ok(curve.max(surface));
        }
        if matches!(self, Self::CurveCurve) {
            return session.original_intersection_curve_source_depth(&[geometry_handle(input, "a")?, geometry_handle(input, "b")?]);
        }
        if matches!(self, Self::Fuse | Self::Cut | Self::Intersect | Self::SurfaceSurface) {
            let a = session.original_query_depth_demand(geometry_handle(input, "a")?)?;
            let b = session.original_query_depth_demand(geometry_handle(input, "b")?)?;
            return Ok(a.max(b));
        }
        if matches!(self, Self::BoundingBox) {
            return session.analytic_bounds_depth(geometry_handle(input, "geometry")?);
        }
        if matches!(self, Self::OffsetSolid | Self::ClassifyPoint | Self::Shell | Self::ConvertToNurbs | Self::Section | Self::Split | Self::LinearPattern | Self::CircularPattern | Self::GridPattern) {
            let source = self.geometry_source(input)?.ok_or_else(|| ValueError::invariant("solid query has no source port"))?;
            return session.certified_solid_depth(source);
        }
        if matches!(self, Self::Area | Self::Volume | Self::CenterOfMass) {
            return session.original_measure_source_depth(&[geometry_handle(input, "geometry")?]);
        }
        if matches!(self, Self::Distance) {
            return session.original_measure_source_depth(&[geometry_handle(input, "a")?, geometry_handle(input, "b")?]);
        }
        if matches!(self, Self::Loft) {
            return session.loft_depth(loft_profiles(input)?);
        }
        if matches!(self, Self::Sweep | Self::Pipe) {
            return session.straight_sweep_depth(geometry_handle(input, "profile")?, geometry_handle(input, "path")?, self.sweep_guide(input));
        }
        if matches!(self, Self::Translate | Self::Rotate | Self::RotateAbout | Self::Scale | Self::Mirror | Self::Copy) {
            return session.planar_face_boundary_depth(geometry_handle(input, "geometry")?);
        }
        if matches!(self, Self::HelicalSweep) {
            return session.planar_face_boundary_depth(geometry_handle(input, "profile")?);
        }
        if matches!(self, Self::ExtrudeFace | Self::Revolve | Self::Thicken | Self::OffsetFace) {
            return session.planar_face_boundary_depth(geometry_handle(input, "face")?);
        }
        if matches!(self, Self::Length) {
            return session.original_intersection_curve_source_depth(&[geometry_handle(input, "geometry")?]);
        }
        if matches!(self, Self::CurveClosestParameter) {
            return session.original_intersection_curve_source_depth(&[geometry_handle(input, "curve")?]);
        }
        if matches!(self,Self::ClosestPoint){return session.original_closest_point_depth_demand(geometry_handle(input,"geometry")?);}
        self.geometry_source(input)?.map_or(Ok(1), |source| session.original_query_depth_demand(source))
    }

    fn pattern_recipe(self, input: &Dictionary) -> Result<SolidPatternRecipe, ValueError> {
        let scalar = |port| number(input, port, "value");
        let count = |port| {
            let value = scalar(port)?;
            if !(0.0..=4096.0).contains(&value) || value.fract() != 0.0 {
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "pattern count requires a whole number between 0 and 4096"));
            }
            Ok(value as usize)
        };
        match self {
            Self::LinearPattern => SolidPatternRecipe::linear(point(input, "direction")?, scalar("spacing")?, count("count")?),
            Self::CircularPattern => SolidPatternRecipe::circular(point(input, "axis")?, count("count")?),
            Self::GridPattern => SolidPatternRecipe::grid(point(input, "dirX")?, point(input, "dirY")?,
                scalar("spacingX")?, scalar("spacingY")?, count("countX")?, count("countY")?),
            _ => unreachable!("only pattern operators request placements"),
        }
    }

    fn query(self, session: &Session, input: &Dictionary) -> Result<GeometryOutput, ValueError> {
        if matches!(self, Self::Distance) {
            let a = geometry_handle(input, "a")?;
            let b = geometry_handle(input, "b")?;
            if let Some((job, face_at_b)) = session.plan_face_distance(a, b)? {
                let port = if face_at_b { "b" } else { "a" };
                return Ok(GeometryOutput::PlanarClosest(job, port));
            }
            let distance = session.original_measure_distance(a, b)?;
            return Ok(GeometryOutput::Query { channel: "distance", value: QueryOutput::Number(distance) });
        }
        let source = self.geometry_source(input)?.ok_or_else(|| ValueError::invariant("geometry query has no source port"))?;
        let (channel, value) = match self {
            Self::Point | Self::Tangent => {
                let parameter = number(input, "parameter", "value")?;
                let tangent = matches!(self, Self::Tangent);
                let axes = session.original_curve_sample(source, parameter, tangent)?;
                if tangent { ("tangent", QueryOutput::Coordinates("vector", axes)) }
                else { ("point", QueryOutput::Coordinates("point", axes)) }
            }
            Self::Curvature => ("curvature", QueryOutput::Number(session.original_curve_curvature(source, number(input, "parameter", "value")?)?)),
            Self::Domain => ("span", QueryOutput::Number(session.original_curve_span(source)?)),
            Self::ClassifyPoint => {
                let result = session.original_solid_classification(source, point(input, "point")?)?;
                ("classification", QueryOutput::Number(classify_number(result)))
            }
            Self::SurfacePoint | Self::SurfaceNormal => {
                let u = number(input, "u", "value")?;
                let v = number(input, "v", "value")?;
                let normal = matches!(self, Self::SurfaceNormal);
                let axes = session.original_surface_sample(source, u, v, normal)?;
                if normal { ("normal", QueryOutput::Coordinates("vector", axes)) }
                else { ("point", QueryOutput::Coordinates("point", axes)) }
            }
            Self::Area => {
                let Some(area) = session.original_analytic_area(source)? else {
                    return session.plan_planar_measure(source,true).map(|job|GeometryOutput::PlanarMeasure(job,"area"));
                };
                ("area", QueryOutput::Number(area))
            }
            Self::CenterOfMass => match session.original_center_of_mass(source) {
                Ok(center)=>("center",QueryOutput::Coordinates("point",center)),
                Err(error) if error.kind==ValueRefusalKind::UnsupportedOwner=>{
                    return session.plan_planar_center_of_mass(source).map(|job|GeometryOutput::PlanarMeasure(job,"center"));
                },
                Err(error)=>return Err(error),
            },
            Self::Volume => {
                let Some(volume) = session.original_primitive_volume(source)? else {
                    return session.plan_planar_measure(source,false).map(|job|GeometryOutput::PlanarMeasure(job,"volume"));
                };
                ("volume", QueryOutput::Number(volume))
            }
            Self::CurveClosestParameter=>{
                let (u,point,distance)=session.original_curve_closest_parameter(source,point(input,"point")?)?;
                ("parameter",QueryOutput::Projection{u,v:None,point,distance})
            }
            Self::SurfaceClosestUv=>{
                let (u,v,point,distance)=session.original_surface_closest_uv(source,point(input,"point")?)?;
                ("u",QueryOutput::Projection{u,v:Some(v),point,distance})
            }
            Self::ClosestPoint => {
                let target = point(input, "point")?;
                if let Some(job) = session.plan_planar_closest(source, target)? { return Ok(GeometryOutput::PlanarClosest(job, "geometry")); }
                let axes = session.original_geometry_closest_point(source, target)?;
                ("pointOut", QueryOutput::Coordinates("point", axes))
            }
            Self::Length => {
                if let Some(query) = session.plan_wire_length(source)? { return Ok(GeometryOutput::WireLength(query)); }
                ("length", QueryOutput::Number(session.original_geometry_length(source)?))
            }
            Self::Label => ("label", QueryOutput::label(session.original_geometry_label(source)?)),
            _ => unreachable!("only query operations read an original handle"),
        };
        Ok(GeometryOutput::Query { channel, value })
    }
}

enum GeometryOutput {
    Geometry(AnalyticPublication, &'static str),
    Transform(AnalyticTransform),
    Compound(CompoundPublication),
    SolidPattern(SolidPatternPublication),
    MeshDecode(MeshDecode),
    StepDecode(Box<StepSolidDecode>),
    StepExport(Box<StepWrite>),
    MeshPublication(MeshPublication),
    MeshExport(Box<NativeMeshExport>),
    Validation(Box<NativeValidationReport>),
    Healing(Box<SolidHealing>),
    TopologyList(Box<TopologyOutput>),
    Solid(SolidPublication, &'static str),
    Boolean { first: BooleanOperand, operation: BooleanOperation },
    BooleanPreparation(BooleanPreparation),
    CompoundCut(CompoundCutDraft),
    Face(ConicFacePublication, ConicFaceOutput),
    PolygonFace(PolygonFacePublication, FaceOutput),
    Section(BoxSectionDraft),
    SectionFaces(Box<PolygonSectionSet>),
    SolidPair(Box<[SolidPublication; 2]>),
    Prism(PrismDraft),
    Loft(LoftDraft),
    Shell { draft: ShellDraft, next: usize },
    Sew(CuboidSew),
    Hull(HullDraft),
    Wire(PolygonWirePublication, &'static str),
    Vertex(VertexPublication, &'static str),
    WireLength(WireLengthJob),
    PlanarClosest(semio_framework_3d_brep_engine::PlanarClosestJob, &'static str),
    PlanarMeasure(PlanarMeasureJob, &'static str),
    Query { channel: &'static str, value: QueryOutput },
}

fn transform_source_demand(session: &Session, source: &str) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { depth: session.planar_face_boundary_depth(source)?.max(session.certified_solid_depth(source)?), copy_bytes: 14 * size_of::<f64>(), ..Default::default() })
}

enum ConicFaceOutput {Single(&'static str),Section}

enum FaceOutput {
    Single(&'static str),
    Section(SectionPolygon),
}

enum QueryOutput {
    Coordinates(&'static str, [f64; 3]),
    Number(f64),
    Projection {u:f64,v:Option<f64>,point:[f64;3],distance:f64},
    Text,
    Label { digits: [u8; 20], start: usize },
}

impl QueryOutput {
    fn label(mut label: u64) -> Self {
        let mut digits = [b'0'; 20];
        let mut start = digits.len() - 1;
        while label > 9 {
            digits[start] = b'0' + (label % 10) as u8;
            label /= 10;
            start -= 1;
        }
        digits[start] = b'0' + label as u8;
        Self::Label { digits, start }
    }

    fn field(&self, channel: &'static str, index: usize) -> Option<(&'static str, Field<'_>)> {
        if let Self::Projection{u,v,point,distance}=self {
            if index<3{return scalar_field(channel,*u,index);}
            let mut index=index-3;
            if let Some(v)=v{
                if index<3{return scalar_field("v",*v,index);}
                index-=3;
            }
            if index<5{return coordinate_field("pointOut","point",*point,index);}
            return scalar_field("distance",*distance,index-5);
        }
        match (self, index) {
            (Self::Coordinates(schema,axes),index)=>coordinate_field(channel,*schema,*axes,index),
            (Self::Number(value),index)=>scalar_field(channel,*value,index),
            (Self::Label { .. }|Self::Text, 0) => Some(("$schema", Field::Text("text"))),
            (Self::Text,1)=>Some(("value",Field::OwnedText)),
            (Self::Label { digits, start }, 1) => Some(("value", Field::Text(std::str::from_utf8(&digits[*start..]).expect("decimal label contains only ASCII digits")))),
            (_, 2) => Some((channel, Field::Payload)),
            _ => None,
        }
    }
}

fn scalar_field(channel:&'static str,value:f64,index:usize)->Option<(&'static str,Field<'static>)>{
    match index{
        0=>Some(("$schema",Field::Text("number"))),
        1=>Some(("value",Field::Decimal(value))),
        2=>Some((channel,Field::Payload)),
        _=>None,
    }
}

fn coordinate_field(channel:&'static str,schema:&'static str,point:[f64;3],index:usize)->Option<(&'static str,Field<'static>)>{
    match index{
        0=>Some(("$schema",Field::Text(schema))),
        1..=3=>Some((["x","y","z"][index-1],Field::Decimal(point[index-1]))),
        4=>Some((channel,Field::Payload)),
        _=>None,
    }
}

#[derive(Clone, Copy)]
enum Field<'a> {
    Text(&'a str),
    OwnedText,
    Decimal(f64),
    Wrap,
    Payload,
    SectionMember,
    SectionSchema,
    SectionPayload,
}

impl<'a> Field<'a> {
    fn text(self) -> Option<&'a str> {
        match self {
            Self::Text(text) => Some(text),
            Self::SectionSchema => Some("list"),
            _ => None,
        }
    }
}

impl GeometryOutput {
    fn handle(&self) -> Option<&str> {
        match self {
            Self::Boolean { .. } | Self::BooleanPreparation(_) | Self::CompoundCut(_) | Self::Transform(_) => None,
            Self::Geometry(curve, _) => Some(curve.handle()),
            Self::Compound(shape)=>Some(shape.handle()),
            Self::SolidPattern(shape) => Some(shape.handle()),
            Self::MeshDecode(_) | Self::MeshExport(_) | Self::StepDecode(_) | Self::StepExport(_) => None,
            Self::MeshPublication(shape) => shape.handle(),
            Self::Solid(shape, _) => Some(shape.handle()),
            Self::Face(shape, _) => Some(shape.handle()),
            Self::PolygonFace(shape, _) => Some(shape.handle()),
            Self::Wire(shape, _) => Some(shape.handle()),
            Self::Vertex(shape, _) => Some(shape.handle()),
            Self::Query { .. } | Self::Validation(_) | Self::Healing(_) | Self::TopologyList(_) | Self::PlanarClosest(..) | Self::PlanarMeasure(..) | Self::Prism(_) | Self::Loft(_) | Self::Shell { .. } | Self::Sew(_) | Self::Hull(_) | Self::Section(_) | Self::SectionFaces(_) | Self::SolidPair(_) | Self::WireLength(_) => None,
        }
    }

    fn field<'a>(&'a self, index: usize, input: &'a Dictionary) -> Option<(&'static str, Field<'a>)> {
        match (self, index) {
            (Self::Healing(_), 0) => Some(("$schema", Field::Text("geometry"))),
            (Self::Healing(_), 1) => geometry_handle(input, "geometry").ok().map(|handle| ("handle", Field::Text(handle))),
            (Self::Healing(_), 2) => Some(("kind", Field::Text("solid"))),
            (Self::Healing(_), 3) => Some(("solid", Field::Payload)),
            (Self::MeshPublication(_), 0) => Some(("$schema", Field::Text("geometry"))),
            (Self::MeshPublication(shape), 1) => shape.handle().map(|key| ("handle", Field::Text(key))),
            (Self::MeshPublication(shape), 2) => Some(("kind", Field::Text(shape.kind()))),
            (Self::MeshPublication(_), 3) => Some(("geometry", Field::Payload)),
            (Self::Geometry(..), 0) => Some(("$schema", Field::Text("geometry"))),
            (Self::Geometry(shape, _), 1) => Some(("handle", Field::Text(shape.handle()))),
            (Self::Geometry(shape, _), 2) => Some(("kind", Field::Text(shape.kind()))),
            (Self::Geometry(_, "wires"), 3) => Some(("0", Field::Wrap)),
            (Self::Geometry(_, "wires"), 4) => Some(("$schema", Field::Text("list"))),
            (Self::Geometry(_, "wires"), 5) => Some(("wires", Field::Payload)),
            (Self::Geometry(_, channel), 3) => Some((*channel, Field::Payload)),
            (Self::Compound(_) | Self::SolidPattern(_),0)=>Some(("$schema",Field::Text("geometry"))),
            (Self::Compound(shape),1)=>Some(("handle",Field::Text(shape.handle()))),
            (Self::SolidPattern(shape),1)=>Some(("handle",Field::Text(shape.handle()))),
            (Self::Compound(_) | Self::SolidPattern(_),2)=>Some(("kind",Field::Text("compound"))),
            (Self::Compound(_),3)=>Some(("compound",Field::Payload)),
            (Self::SolidPattern(shape),3)=>Some((shape.channel(),Field::Payload)),
            (Self::Solid(..), 0) => Some(("$schema", Field::Text("geometry"))),
            (Self::Solid(shape, _), 1) => Some(("handle", Field::Text(shape.handle()))),
            (Self::Solid(..), 2) => Some(("kind", Field::Text("solid"))),
            (Self::Solid(_, channel), 3) => Some((*channel, Field::Payload)),
            (Self::SolidPair(pair), 0..=7) => split::field(pair, index),
            (Self::SectionFaces(faces), index) => section_faces::field(faces, index),
            (Self::Vertex(..), 0) => Some(("$schema", Field::Text("geometry"))),
            (Self::Vertex(shape, _), 1) => Some(("handle", Field::Text(shape.handle()))),
            (Self::Vertex(..), 2) => Some(("kind", Field::Text("vertex"))),
            (Self::Vertex(_, channel), 3) => Some((*channel, Field::Payload)),
            (Self::Wire(..), 0) => Some(("$schema", Field::Text("geometry"))),
            (Self::Wire(shape, _), 1) => Some(("handle", Field::Text(shape.handle()))),
            (Self::Wire(..), 2) => Some(("kind", Field::Text("wire"))),
            (Self::Wire(_, output), 3) => Some((*output, Field::Payload)),
            (Self::Face(..), 0) => Some(("$schema", Field::Text("geometry"))),
            (Self::Face(shape, _), 1) => Some(("handle", Field::Text(shape.handle()))),
            (Self::Face(..), 2) => Some(("kind", Field::Text("face"))),
            (Self::Face(_, ConicFaceOutput::Single(channel)), 3) => Some((*channel, Field::Payload)),
            (Self::Face(_, ConicFaceOutput::Section), 3) => Some(("0", Field::Wrap)),
            (Self::Face(_, ConicFaceOutput::Section), 4) => Some(("$schema", Field::Text("list"))),
            (Self::Face(_, ConicFaceOutput::Section), 5) => Some(("faces", Field::Payload)),
            (Self::PolygonFace(..), 0) => Some(("$schema", Field::Text("geometry"))),
            (Self::PolygonFace(shape, _), 1) => Some(("handle", Field::Text(shape.handle()))),
            (Self::PolygonFace(..), 2) => Some(("kind", Field::Text("face"))),
            (Self::PolygonFace(_, FaceOutput::Single(output)), 3) => Some((*output, Field::Payload)),
            (Self::PolygonFace(_, FaceOutput::Section(_)), 3) => Some(("0", Field::Wrap)),
            (Self::PolygonFace(_, FaceOutput::Section(_)), 4) => Some(("$schema", Field::Text("list"))),
            (Self::PolygonFace(_, FaceOutput::Section(_)), 5) => Some(("faces", Field::Payload)),
            (Self::Query { channel, value }, index) => value.field(channel, index),
            _ => None,
        }
    }
}

#[derive(semio_framework_value::RetireOwned)]
struct GeometryOutputData {
    input: Dictionary,
    geometry: Option<Dictionary>,
    output: Option<Dictionary>,
    entry: Option<BudgetedOwnedEntry>,
    key: String,
    text: String,
    live_key: String,
    claim_key: String,
    split_live: [String; 2],
    split_claim: [String; 2],
    section_list: Option<Dictionary>,
    section_live: [String; SECTION_FACE_LIMIT],
    section_claim: [String; SECTION_FACE_LIMIT],
    retired_analytic: Option<AnalyticPublication>,
    retired_compound:Option<CompoundPublication>,
    retired_validation:Option<Box<NativeValidationReport>>,
    retired_healing:Option<Box<SolidHealing>>,
    retired_topology_list: Option<Box<TopologyOutput>>,
    owned_text:Option<String>,
    retired_mesh_export:Option<Box<NativeMeshExport>>,
}

pub struct OriginalGeometryJob {
    session: Option<SessionCapture>,
    kind: GeometryOperation,
    result: Option<GeometryOutput>,
    data: Option<GeometryOutputData>,
    retiring: Option<ControlledRetirement<GeometryOutputData>>,
    progress: OperatorProgress,
    receipt: Progress,
    field: usize,
    closing_entry: bool,
    claim_staged: bool,
    split_staged: [bool; 2],
    section_staged: [bool; SECTION_FACE_LIMIT],
    cancelled: bool,
    done: bool,
}

impl OriginalGeometryJob {
    /// 🎟️ Opens this operator over its original input and admitted job storage.
    pub fn admit(kind: GeometryOperation, session: &Session, input: Dictionary, grant: Grant) -> Result<(OperatorPlanAdmission, Progress), (EvalError, Dictionary)> {
        let receipt = Progress { copied_items: 1, copied_bytes: size_of::<Session>(), retained_capacity_bytes: size_of::<Self>(), ..Progress::default() };
        if grant.maximum_depth == 0 || !receipt.fits(grant) {
            return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "original geometry producer frame is not funded").into(), input));
        }
        let data = GeometryOutputData {
            input,
            geometry: None,
            output: None,
            entry: None,
            key: String::new(),
            text: String::new(),
            live_key: String::new(),
            claim_key: String::new(),
            split_live: Default::default(),
            split_claim: Default::default(),
            section_list: None,
            section_live: Default::default(),
            section_claim: Default::default(),
            retired_analytic:None,
            retired_compound:None,
            retired_validation:None,
            retired_healing:None,
            retired_topology_list: None,
            owned_text:None,
            retired_mesh_export:None,
        };
        Ok((OperatorPlanAdmission::Job(Box::new(Self {
            session: Some(session.capture()),
            kind,
            result: None,
            data: Some(data),
            retiring: None,
            progress: OperatorProgress { units_done: 0, units_total: 1, phase: kind.initial_phase() },
            receipt: Progress::default(),
            field: 0,
            closing_entry: false,
            claim_staged: false,
            split_staged: [false; 2],
            section_staged: [false; SECTION_FACE_LIMIT],
            cancelled: false,
            done: false,
        })), receipt))
    }

    fn advance(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<Dictionary>, Progress), ValueError> {
        let grant = effect.grant();
        let empty = Progress::default();
        let data = self.data.as_mut().expect("active geometry producer has its input owner");
        let session = self.session.as_ref().expect("active geometry producer has a session");
        if self.result.is_none() {
            let bytes = self.kind.output_bytes(&data.input);
            let capacity=self.kind.output_capacity();
            if grant.maximum_capacity_bytes<capacity||grant.maximum_copy_bytes < bytes || grant.maximum_depth < self.kind.source_depth(session, &data.input)? || !effect.performed(1)? { return Ok((None, empty)); }
            self.result = Some(self.kind.output(session, &data.input)?);
            match self.result.as_ref().unwrap() {
                GeometryOutput::Boolean { .. } => {},
                GeometryOutput::BooleanPreparation(_) => self.progress.phase = "boolean-acquire-faces",
                GeometryOutput::CompoundCut(_) => self.progress.phase = "compound-cut-tools",
                GeometryOutput::Geometry(..) => self.progress.phase = "analytic-geometry-output",
                GeometryOutput::Transform(_) => self.progress.phase = "affine-image-source",
                GeometryOutput::Compound(_)=>self.progress.phase="compound-members",
                GeometryOutput::SolidPattern(_) => self.progress.phase = "solid-pattern-preparation",
                GeometryOutput::MeshDecode(_) => self.progress.phase = "mesh-decoding",
                GeometryOutput::StepDecode(_) => self.progress.phase = "step-solid-certification",
                GeometryOutput::StepExport(export) => self.progress.phase = export.phase(),
                GeometryOutput::MeshPublication(_) => self.progress.phase = "mesh-topology",
                GeometryOutput::MeshExport(export) => self.progress.phase = export.phase(),
                GeometryOutput::Validation(report)=>self.progress.phase=report.phase(),
                GeometryOutput::Healing(job) => self.progress.phase = job.phase(),
                GeometryOutput::TopologyList(output) => self.progress.phase = output.phase(),
                GeometryOutput::Query { .. } => self.progress.phase = "geometry-query-output",
                GeometryOutput::PlanarClosest(..) => self.progress.phase = "planar-closest-point",
                GeometryOutput::PlanarMeasure(_, channel) => self.progress.phase = match *channel {
                    "area" => "geometry-area-integrating",
                    "box" => "geometry-bounds-integrating",
                    _ => "geometry-volume-integrating",
                },
                GeometryOutput::Loft(_) => self.progress.phase = "loft-corresponding-edges",
                GeometryOutput::Prism(_) => self.progress.phase = match self.kind {
                    GeometryOperation::Thicken => "thickening-profile-validation",
                    GeometryOperation::Sweep | GeometryOperation::Pipe => "straight-sweep-profile-validation",
                    _ => "extrusion-profile-validation",
                },
                GeometryOutput::Shell { .. } => self.progress.phase = "shell-open-faces",
                GeometryOutput::Sew(_) => self.progress.phase = "sew-cuboid-faces",
                GeometryOutput::Hull(_) => self.progress.phase = "convex-hull-read-points",
                GeometryOutput::Section(_) => self.progress.phase = "box-section-edges",
                GeometryOutput::SectionFaces(_) => self.progress.phase = "section-face-topology",
                GeometryOutput::SolidPair(_) => self.progress.phase = "split-topology",
                GeometryOutput::WireLength(_) => self.progress.phase = "bounded-length-measuring",
                GeometryOutput::Solid(..) | GeometryOutput::Face(..) | GeometryOutput::PolygonFace(..) | GeometryOutput::Wire(..) | GeometryOutput::Vertex(..) => {}
            }
            let retained_capacity_bytes = if matches!(self.kind, GeometryOperation::Section) && !matches!(&self.result, Some(GeometryOutput::SectionFaces(_))) { 0 } else { capacity };
            return Ok((None, Progress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes, ..empty }));
        }
        let result = self.result.as_mut().unwrap();
        if let GeometryOutput::Transform(transform) = result {
            let source = geometry_handle(&data.input, "geometry")?;
            let demand = transform_source_demand(session, source)?;
            if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
                return Ok((None, empty));
            }
            let shape = if let Some(shape) = session.plan_certified_solid_transform(source, *transform)? {
                GeometryOutput::Solid(shape, "geometryOut")
            } else if let Some(shape) = session.plan_polygon_face_transform(source, *transform)? {
                GeometryOutput::PolygonFace(shape, FaceOutput::Single("geometryOut"))
            } else {
                GeometryOutput::Geometry(session.plan_analytic_transform(source, *transform)?, "geometryOut")
            };
            self.result = Some(shape);
            return Ok((None, Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let GeometryOutput::TopologyList(output) = result {
            let result = output.step(session, &data.input, effect)?;
            self.progress.phase = output.phase();
            return Ok(result);
        }
        if let GeometryOutput::Validation(report)=result{
            let handle=geometry_handle(&data.input,"geometry")?;
            let (text,receipt)=session.with_original_geometry_query(|kernel| report.step(kernel,handle,effect))?;
            self.progress.phase=report.phase();
            if let Some(text)=text{
                let replacement=GeometryOutput::Query{channel:"report",value:QueryOutput::Text};
                let GeometryOutput::Validation(report)=std::mem::replace(result,replacement) else{unreachable!()};
                data.retired_validation=Some(report);
                data.owned_text=Some(text);
            }
            return Ok((None,receipt));
        }
        if let GeometryOutput::Healing(job) = result {
            if !job.ready() {
                let receipt = session.with_original_geometry_query(|kernel| job.step(kernel.representation(), kernel.healing_revision(), effect))?;
                self.progress.phase = job.phase();
                return Ok((None, receipt));
            }
        }
        if let GeometryOutput::Compound(shape)=result{
            if !shape.complete(){
                let source=shape.next_member().map(|index|compound_member(&data.input,index)).transpose()?;
                return session.step_compound_preparation(shape,source,effect).map(|receipt|(None,receipt));
            }
            if self.progress.phase=="compound-members"{self.progress.phase="compound-output";}
        }
        if let GeometryOutput::SolidPattern(shape) = result {
            if !shape.ready() { return session.step_solid_pattern_preparation(shape, effect).map(|receipt| (None, receipt)); }
        }
        if let GeometryOutput::StepExport(export) = result {
            self.progress.phase = export.phase();
            let (output, receipt) = step_export_source(session, &data.input, |source| {
                if !export.ready() { return export.step(source, effect).map(|receipt| (None, receipt)); }
                let demand = step_export_demand(export, source)?;
                if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((None, empty)); }
                let text = export.take_output(source).map_err(|error| error.with_retained_progress(item()))?;
                Ok((Some(text), Progress { copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..item() }))
            })?;
            if let Some(text) = output {
                data.owned_text = Some(text);
                *result = GeometryOutput::Query { channel: "step", value: QueryOutput::Text };
                self.progress.phase = "geometry-query-output";
            }
            return Ok((None, receipt));
        }
        if let GeometryOutput::StepDecode(decoder) = result {
            if !decoder.is_ready() { return decoder.step(exchange_text(&data.input)?, effect).map(|receipt| (None, receipt)); }
            let demand = step_decode_demand(decoder);
            if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((None, empty)); }
            let receipt = Progress { copied_bytes: demand.copy_bytes, ..item() };
            let proof = decoder.ready().unwrap();
            let solid = if let Some(primitive) = proof.nurbs_cuboid() { session.plan_nurbs_cuboid_primitive(primitive) }
            else if let Some(primitive) = proof.analytic_primitive() { session.plan_solid_primitive(primitive) }
            else if let Some(prism) = proof.prism_boundary() { session.plan_exchange_boundary(semio_framework_3d_brep_engine::SolidExchangeBoundary::Prism(prism)) }
            else if let Some(shape) = proof.rounded_box() { session.plan_exchange_boundary(semio_framework_3d_brep_engine::SolidExchangeBoundary::RoundedBox(shape)) }
            else if let StepSolid::Orthogonal { axes, cells } = proof { session.plan_orthogonal_cells(axes, cells) }
            else { Err(ValueError::invariant("STEP certification did not produce a solid recipe")) }
                .map_err(|error| error.with_retained_progress(receipt))?;
            self.result = Some(GeometryOutput::Solid(solid, "geometry"));
            return Ok((None, Progress { released_bytes: demand.release_bytes, ..receipt }));
        }
        if let GeometryOutput::MeshDecode(decoder) = result {
            if !decoder.ready() { return decoder.step(exchange_text(&data.input)?, effect).map(|receipt| (None, receipt)); }
            let demand = decoder.demand();
            if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((None, empty)); }
            self.result = Some(GeometryOutput::MeshPublication(session.plan_mesh_publication(decoder)?));
            self.progress.phase = "mesh-topology";
            return Ok((None, Progress { copied_bytes: demand.copy_bytes, ..item() }));
        }
        if let GeometryOutput::MeshPublication(shape) = result {
            if shape.handle().is_none() { return session.step_mesh_preparation(shape, effect).map(|receipt| (None, receipt)); }
        }
        if let GeometryOutput::MeshExport(export) = result {
            let source = geometry_handle(&data.input, "geometry")?;
            self.progress.phase = export.phase();
            if !export.ready() { return session.step_mesh_export(source, export, effect).map(|receipt| (None, receipt)); }
            let quote = session.mesh_export_demand(source, export)?;
            if !retained_clone_grant_admits(grant, quote) || !effect.performed(1)? { return Ok((None, empty)); }
            let replacement = GeometryOutput::Query { channel: export.channel(), value: QueryOutput::Text };
            data.owned_text = export.take_output();
            let GeometryOutput::MeshExport(export) = std::mem::replace(result, replacement) else { unreachable!() };
            data.retired_mesh_export = Some(export);
            self.progress.phase = "geometry-query-output";
            return Ok((None, Progress { copied_bytes: quote.copy_bytes, ..item() }));
        }
        if let GeometryOutput::BooleanPreparation(preparation) = result {
            let a = geometry_handle(&data.input, "a")?;
            let b = geometry_handle(&data.input, "b")?;
            let (ready, receipt) = session.step_boolean_preparation(a,b,preparation,effect)?;
            if ready {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,
                    "original Boolean face acquisition completed; general face imprinting is not funded").with_retained_progress(receipt));
            }
            return Ok((None,receipt));
        }
        if let GeometryOutput::Boolean { first, operation } = result {
            let a = geometry_handle(&data.input, "a")?;
            let b = geometry_handle(&data.input, "b")?;
            let bytes = BooleanOperand::recipe_copy_bytes();
            if grant.maximum_copy_bytes < bytes || grant.maximum_depth < self.kind.source_depth(session, &data.input)? || !effect.performed(1)? {
                return Ok((None, empty));
            }
            let receipt = Progress { copied_items: 1, copied_bytes: bytes, ..empty };
            self.result = Some(match session.plan_certified_boolean(a, b, first, *operation) {
                Ok(CertifiedBooleanResult::Solid(solid)) => GeometryOutput::Solid(solid,"solid"),
                Ok(CertifiedBooleanResult::Group(shapes)) => GeometryOutput::SolidPattern(shapes),
                Err(error) if error.kind == ValueRefusalKind::UnsupportedOwner => {
                    self.progress.phase = "boolean-acquire-faces";
                    GeometryOutput::BooleanPreparation(session.plan_boolean_preparation(a,b)
                        .map_err(|error| error.with_retained_progress(receipt))?)
                }
                Err(error) => return Err(error.with_retained_progress(receipt)),
            });
            return Ok((None, receipt));
        }
        if let GeometryOutput::Geometry(shape,_) = result {
            if let Some(demand) = shape.construction_demand() {
                self.progress.phase = match self.kind {
                    GeometryOperation::NurbsGrid => "nurbs-grid-fitting",
                    GeometryOperation::Coons => "coons-surface-fitting",
                    GeometryOperation::Interpolate => "curve-interpolating",
                    GeometryOperation::Approximate => "curve-approximating",
                    _ => "analytic-geometry-construction",
                };
                let sample = if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_depth < demand.depth {
                    None
                } else if let Some(index) = shape.next_flat_sample() {
                    Some(listed_point(point_list(&data.input)?, index)?)
                } else if let Some((boundary, index)) = shape.next_coons_point() {
                    Some(listed_point(boundary_point_list(nested_boundary_list(&data.input)?, boundary)?, index)?)
                } else { None };
                return shape.step_construction(sample, effect).map(|receipt| (None, receipt));
            }
        }
        if matches!(self.progress.phase, "analytic-geometry-construction" | "nurbs-grid-fitting" | "coons-surface-fitting" | "curve-interpolating" | "curve-approximating") {
            self.progress.phase = "analytic-geometry-output";
        }
        if let GeometryOutput::PlanarMeasure(job, channel) = result {
            let source = self.kind.geometry_source(&data.input)?.ok_or_else(|| ValueError::invariant("measurement has no source port"))?;
            let (number, receipt) = session.step_planar_measure(source, job, effect)?;
            if let Some(number) = number {
                if *channel == "box" {
                    let [origin, end] = job.axis_bounds().ok_or_else(|| ValueError::invariant("planar bounds were not integrated"))?;
                    let [width, depth, height] = (end - origin).to_array();
                    let shape = session.plan_solid_primitive(SolidPrimitive::Box { origin, width, depth, height })?;
                    self.result = Some(GeometryOutput::Solid(shape, "box"));
                    self.progress.phase = "solid-primitive-topology";
                    return Ok((None, receipt));
                }
                let value=if *channel=="center" {
                    QueryOutput::Coordinates("point",job.center_of_mass().ok_or_else(||ValueError::invariant("planar center was not integrated"))?)
                } else {QueryOutput::Number(number)};
                self.result = Some(GeometryOutput::Query { channel: *channel, value });
                self.progress.phase = "geometry-query-output";
            }
            return Ok((None, receipt));
        }
        if let GeometryOutput::PlanarClosest(job, port) = result {
            let (measurement, receipt) = session.step_planar_closest(geometry_handle(&data.input, port)?, job, effect)?;
            if let Some(measurement) = measurement {
                use semio_framework_3d_brep_engine::PlanarClosestOutput::{Distance, Point};
                let (channel, value) = match measurement {
                    Point(point) => ("pointOut", QueryOutput::Coordinates("point", point.to_array())),
                    Distance(distance) => ("distance", QueryOutput::Number(distance)),
                };
                self.result = Some(GeometryOutput::Query { channel, value });
                self.progress.phase = "geometry-query-output";
            }
            return Ok((None, receipt));
        }
        if let GeometryOutput::CompoundCut(draft) = result {
            let (solid, receipt) = draft.advance(session, &data.input, effect)?;
            if let Some(solid) = solid {
                self.result = Some(GeometryOutput::Solid(solid, "solid"));
                self.progress.phase = "solid-primitive-topology";
            }
            return Ok((None, receipt));
        }
        if let GeometryOutput::SolidPair(pair) = result {
            if let Some(solid) = split::unfinished(pair) {
                return session.step_solid_primitive_topology(solid, effect).map(|receipt| (None, receipt));
            }
        }
        if let GeometryOutput::Solid(solid, _) = result {
            if !solid.topology_ready() {
                self.progress.phase = "solid-primitive-topology";
                return session.step_solid_primitive_topology(solid, effect).map(|progress| (None, progress));
            }
        }
        if let GeometryOutput::Vertex(vertex, _) = result {
            if !vertex.topology_ready() {
                self.progress.phase = "vertex-topology";
                return session.step_vertex_topology(vertex, effect).map(|progress| (None, progress));
            }
            self.progress.phase = "vertex-output";
        }
        if let GeometryOutput::Wire(wire, _) = result {
            if !wire.topology_ready() {
                self.progress.phase = "polygon-wire-topology";
                let demand = session.polygon_wire_demand(wire, self.claim_staged, grant.maximum_copy_bytes)?;
                let point = wire.next_point_index().filter(|_| demand.capacity_bytes == 0
                    && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_depth >= demand.depth)
                    .map(|index| listed_point(point_list(&data.input)?, index)).transpose()?;
                return session.step_polygon_wire_topology(wire, point, effect).map(|progress| (None, progress));
            }
        }
        if let GeometryOutput::WireLength(query) = result {
            let source = self.kind.geometry_source(&data.input)?.ok_or_else(|| ValueError::invariant("bounded length has no source port"))?;
            let (length, receipt) = session.step_wire_length(source, query, effect)?;
            if let Some(length) = length {
                self.result = Some(GeometryOutput::Query { channel: "length", value: QueryOutput::Number(length) });
                self.progress.phase = "geometry-query-output";
            }
            return Ok((None, receipt));
        }
        if let GeometryOutput::SectionFaces(faces) = result {
            if let Some(index) = section_faces::unfinished(faces) {
                return section_faces::step_topology(session, faces, index, effect).map(|receipt| (None, receipt));
            }
            self.progress.phase = "section-face-list";
        }
        if let GeometryOutput::Section(draft) = result {
            let demand = section_demand(draft);
            if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((None, empty)); }
            let invalid = |reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason);
            if draft.has_edge() {
                draft.advance_edge().map_err(invalid)?;
            } else {
                let polygon = draft.finish().map_err(invalid)?;
                let face = session.plan_polygon_face_points(polygon.count())?;
                self.result = Some(GeometryOutput::PolygonFace(face, FaceOutput::Section(polygon)));
                self.progress.phase = "box-section-face";
            }
            return Ok((None, Progress { copied_bytes: demand.copy_bytes, ..item() }));
        }
        if let GeometryOutput::Hull(draft) = result {
            let demand = point_hull_demand(&data.input, draft)?;
            if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((None, empty)); }
            if let Some(rank) = draft.next_point() {
                let source = point_list(&data.input)?;
                let (key, _) = source.entry_at_rank(rank + 1).ok_or_else(invalid_point_list)?;
                if key.len() > 20 || key.parse::<usize>().is_err() { return Err(invalid_point_list()); }
                draft.add_point(point(source, key)?)?;
                if draft.next_point().is_none() { self.progress.phase = "convex-hull-support-planes"; }
            } else if draft.has_candidate() {
                draft.scan_step()?;
            } else {
                self.result = Some(GeometryOutput::Solid(session.finish_point_hull(draft)?, "solid"));
                self.progress.phase = "solid-primitive-topology";
            }
            return Ok((None, Progress { copied_bytes: demand.copy_bytes, ..item() }));
        }
        if let GeometryOutput::Sew(draft) = result {
            let source = sew_face(&data.input, draft.next_face().unwrap_or(5))?;
            let demand = session.cuboid_sew_demand(draft, source)?;
            if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((None, empty)); }
            if let Some(solid) = session.step_cuboid_sew(draft, source)? {
                self.result = Some(GeometryOutput::Solid(solid, "solid"));
                self.progress.phase = "solid-primitive-topology";
            }
            return Ok((None, Progress { copied_bytes: demand.copy_bytes, ..item() }));
        }
        if let GeometryOutput::Shell { draft, next } = result {
            let (selection, demand) = shell_selection(session, &data.input, draft, *next)?;
            if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((None, empty)); }
            if let Some(face) = selection {
                if let Some(face) = face { session.add_shell_face(draft, face)?; }
                *next += 1;
            } else {
                let source = geometry_handle(&data.input, "geometry")?;
                self.result = Some(GeometryOutput::Solid(session.finish_shell(draft, source)?, "solid"));
                self.progress.phase = "solid-primitive-topology";
            }
            return Ok((None, Progress { copied_bytes: demand.copy_bytes, ..item() }));
        }
        if let GeometryOutput::Loft(draft) = result {
            let (prism, receipt) = session.step_loft_draft(loft_profiles(&data.input)?, draft, effect)?;
            if let Some(prism) = prism {
                self.result = Some(GeometryOutput::Prism(prism));
                self.progress.phase = "loft-prism-validation";
            }
            return Ok((None, receipt));
        }
        if let GeometryOutput::Prism(draft) = result {
            let source = self.kind.geometry_source(&data.input)?.ok_or_else(|| ValueError::invariant("prism has no source port"))?;
            let (solid, receipt) = session.step_prism_draft(source, draft, effect)?;
            if let Some(solid) = solid {
                self.result = Some(GeometryOutput::Solid(solid, "solid"));
                self.progress.phase = "solid-primitive-topology";
            }
            return Ok((None, receipt));
        }
        if let GeometryOutput::PolygonFace(face, output) = result {
            if !face.topology_ready() {
                self.progress.phase = "polygon-face-topology";
                let source = match output {
                    FaceOutput::Single(_) => self.kind.geometry_source(&data.input)?,
                    FaceOutput::Section(_) => None,
                };
                let demand = session.polygon_face_demand(face, source, self.claim_staged, grant.maximum_copy_bytes)?;
                let point = face.next_point_index().filter(|_| demand.capacity_bytes == 0
                    && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_depth >= demand.depth)
                    .map(|index| match output {
                        FaceOutput::Single(_) => listed_point(point_list(&data.input)?, index),
                        FaceOutput::Section(polygon) => polygon.point(index).ok_or_else(|| ValueError::invariant("original section vertex ordinal changed")),
                    }).transpose()?;
                return session.step_polygon_face_topology(face, source, point, effect).map(|progress| (None, progress));
            }
        }
        if let GeometryOutput::Face(face, _) = result {
            if !face.topology_ready() {
                self.progress.phase = "conic-face-topology";
                return session.step_conic_face_topology(face, effect).map(|progress| (None, progress));
            }
        }
        if self.progress.phase == "polygon-face-topology" { self.progress.phase = "polygon-face-output"; }
        if self.progress.phase == "polygon-wire-topology" { self.progress.phase = "polygon-wire-output"; }
        if self.progress.phase == "conic-face-topology" { self.progress.phase = "conic-face-output"; }
        if self.progress.phase == "solid-primitive-topology" { self.progress.phase = "solid-primitive-output"; }
        if let Some(entry) = data.entry.as_mut() {
            if self.closing_entry {
                return match entry.close_step(grant.maximum_copy_bytes, effect) {
                    RetainedWorkPoll::Pending { .. } => Ok((None, entry.step_progress())),
                    RetainedWorkPoll::Complete(outcome) => {
                        let step = outcome.result.map_err(|error| error.into_value_error())?;
                        if entry.terminal_is_empty() {
                            data.entry = None;
                            self.closing_entry = false;
                            self.field += 1;
                        }
                        Ok((None, step.progress()))
                    }
                };
            }
            let poll = entry.step(grant.maximum_copy_bytes, effect);
            let receipt = entry.step_progress();
            if let RetainedWorkPoll::Complete(outcome) = poll {
                if let Some(dictionary) = outcome.result.map_err(|error| error.into_value_error())? {
                    match result.field(self.field, &data.input) {
                        Some((_, Field::Payload | Field::SectionPayload)) => data.output = Some(dictionary),
                        Some((_, Field::SectionMember | Field::SectionSchema)) => data.section_list = Some(dictionary),
                        _ => data.geometry = Some(dictionary),
                    }
                    self.closing_entry = true;
                }
            }
            return Ok((None, receipt));
        }
        if let Some((key, field)) = result.field(self.field, &data.input) {
            if data.key.len() < key.len() {
                return write_text(&mut data.key, key, effect).map(|progress| (None, progress));
            }
            if let Some(text) = field.text() {
                if data.text.len() < text.len() {
                    return write_text(&mut data.text, text, effect).map(|progress| (None, progress));
                }
            }
            let copied_bytes = if matches!(field, Field::Decimal(_)) { size_of::<f64>() } else { 0 };
            if grant.maximum_copy_bytes < copied_bytes || !effect.performed(1)? { return Ok((None, empty)); }
            let base = data.geometry.take().unwrap_or_default();
            let (base, value) = match field {
                Field::Wrap => (Dictionary::new(), Value::Dictionary(base)),
                Field::Payload => (data.output.take().unwrap_or_default(), Value::Dictionary(base)),
                Field::SectionMember => (data.section_list.take().unwrap_or_default(), Value::Dictionary(base)),
                Field::SectionSchema => (data.section_list.take().unwrap_or_default(), Value::Atom(Atom::String(take(&mut data.text)))),
                Field::SectionPayload => (data.output.take().unwrap_or_default(), Value::Dictionary(data.section_list.take().expect("original section list"))),
                Field::Text(_) => (base, Value::Atom(Atom::String(take(&mut data.text)))),
                Field::OwnedText=>(base,Value::Atom(Atom::String(data.owned_text.take().ok_or_else(||ValueError::invariant("original query text disappeared"))?))),
                Field::Decimal(number) => (base, Value::Atom(Atom::Decimal(number))),
            };
            data.entry = Some(BudgetedOwnedEntry::new(base, take(&mut data.key), value));
            return Ok((None, Progress { copied_bytes, ..item() }));
        }
        if let GeometryOutput::SectionFaces(faces) = result {
            self.progress.phase = "section-faces-publication";
            let (ready, receipt) = section_faces::step(session, faces, data, &mut self.section_staged, effect)?;
            if self.section_staged.iter().any(|&staged| staged) { self.progress.phase = "section-staged-claim"; }
            return Ok((if ready { data.output.take() } else { None }, receipt));
        }
        if let GeometryOutput::SolidPair(pair) = result {
            self.progress.phase = "split-publication";
            let (ready, receipt) = split::step(session, pair, data, &mut self.split_staged, effect)?;
            return Ok((if ready { data.output.take() } else { None }, receipt));
        }
        if let GeometryOutput::Healing(job) = result {
            let (ready, receipt) = session.commit_healing(geometry_handle(&data.input, "geometry")?, job, effect)?;
            return Ok((if ready { data.output.take() } else { None }, receipt));
        }
        let Some(handle) = result.handle() else {
            if !effect.performed(1)? { return Ok((None, empty)); }
            return Ok((data.output.take(), item()));
        };
        self.progress.phase = match result {
            GeometryOutput::Solid(..) => "solid-primitive-publication",
            GeometryOutput::Face(..) => "conic-face-publication",
            GeometryOutput::Wire(..) => "polygon-wire-publication",
            GeometryOutput::Vertex(..) => "vertex-publication",
            GeometryOutput::Compound(_)=>"compound-publication",
            GeometryOutput::SolidPattern(_) => "solid-pattern-publication",
            GeometryOutput::MeshPublication(_) => "mesh-publication",
            GeometryOutput::PolygonFace(..) => "polygon-face-publication",
            _ => "analytic-geometry-publication",
        };
        if data.live_key.len() < handle.len() {
            return write_text(&mut data.live_key, handle, effect).map(|progress| (None, progress));
        }
        if !self.claim_staged && data.claim_key.len() < handle.len() {
            return write_text(&mut data.claim_key, handle, effect).map(|progress| (None, progress));
        }
        let (ready, progress) = match result {
            GeometryOutput::Geometry(curve, _) => session.step_analytic_publication(curve, &mut data.live_key, &mut data.claim_key, &mut self.claim_staged, effect)?,
            GeometryOutput::Compound(shape)=>session.step_compound_publication(shape,&mut data.live_key,&mut data.claim_key,&mut self.claim_staged,effect)?,
            GeometryOutput::SolidPattern(shape) => session.step_solid_pattern_publication(shape, &mut data.live_key, &mut data.claim_key, &mut self.claim_staged, effect)?,
            GeometryOutput::MeshPublication(shape) => session.step_mesh_publication(shape, &mut data.live_key, &mut data.claim_key, &mut self.claim_staged, effect)?,
            GeometryOutput::MeshDecode(_) | GeometryOutput::MeshExport(_) | GeometryOutput::StepDecode(_) | GeometryOutput::StepExport(_) => unreachable!("exchange stage owns no live address here"),
            GeometryOutput::Solid(solid, _) => session.step_solid_primitive(solid, &mut data.live_key, &mut data.claim_key, &mut self.claim_staged, effect)?,
            GeometryOutput::Face(face, _) => session.step_conic_face_publication(face, &mut data.live_key, &mut data.claim_key, &mut self.claim_staged, effect)?,
            GeometryOutput::PolygonFace(face, _) => session.step_polygon_face_publication(face, &mut data.live_key, &mut data.claim_key, &mut self.claim_staged, effect)?,
            GeometryOutput::Vertex(vertex, _) => session.step_vertex_publication(vertex, &mut data.live_key, &mut data.claim_key, &mut self.claim_staged, effect)?,
            GeometryOutput::Wire(wire, _) => session.step_polygon_wire_publication(wire, &mut data.live_key, &mut data.claim_key, &mut self.claim_staged, effect)?,
            GeometryOutput::Query { .. } | GeometryOutput::Validation(_) | GeometryOutput::Healing(_) | GeometryOutput::TopologyList(_) | GeometryOutput::PlanarClosest(..) | GeometryOutput::PlanarMeasure(..) | GeometryOutput::Boolean { .. } | GeometryOutput::BooleanPreparation(_) | GeometryOutput::CompoundCut(_) | GeometryOutput::Prism(_) | GeometryOutput::Loft(_) | GeometryOutput::Shell { .. } | GeometryOutput::Sew(_) | GeometryOutput::Hull(_) | GeometryOutput::Section(_) | GeometryOutput::SectionFaces(_) | GeometryOutput::SolidPair(_) | GeometryOutput::WireLength(_) | GeometryOutput::Transform(_) => unreachable!("only geometry handles need a session claim"),
        };
        Ok((if ready { data.output.take() } else { None }, progress))
    }

    fn step_demand(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        let empty = RetirementDemand { depth: 1, ..Default::default() };
        if self.cancelled || self.done { return Ok(empty); }
        let data = self.data.as_ref().expect("active geometry producer has its input owner");
        let session = self.session.as_ref().unwrap();
        let Some(result) = self.result.as_ref() else {
            return Ok(RetirementDemand {copy_bytes:self.kind.output_bytes(&data.input),capacity_bytes:self.kind.output_capacity(),depth:self.kind.source_depth(session,&data.input)?,..empty});
        };
        if matches!(result, GeometryOutput::Transform(_)) {
            return transform_source_demand(session, geometry_handle(&data.input, "geometry")?);
        }
        if let GeometryOutput::TopologyList(output) = result {
            return output.demand(session, &data.input, copy);
        }
        if let GeometryOutput::Validation(report)=result{
            let source = geometry_handle(&data.input, "geometry")?;
            return session.with_original_geometry_query(|kernel| report.demand(kernel, source, copy));
        }
        if let GeometryOutput::Geometry(shape,_) = result {
            if let Some(demand)=shape.construction_demand(){return Ok(demand);}
        }
        if let GeometryOutput::Healing(job) = result {
            if !job.ready() { return session.with_original_geometry_query(|kernel| job.demand(kernel.representation(), kernel.healing_revision(), copy)); }
        }
        if let GeometryOutput::Compound(shape)=result{
            if !shape.complete(){
                let source=shape.next_member().map(|index|compound_member(&data.input,index)).transpose()?;
                return session.compound_preparation_demand(shape,source);
            }
        }
        if let GeometryOutput::SolidPattern(shape) = result {
            if !shape.ready() { return session.solid_pattern_preparation_demand(shape); }
        }
        if let GeometryOutput::MeshDecode(decoder) = result { return Ok(decoder.demand()); }
        if let GeometryOutput::StepDecode(decoder) = result { return Ok(step_decode_demand(decoder)); }
        if let GeometryOutput::StepExport(export) = result {
            return step_export_source(session, &data.input, |source| step_export_demand(export, source));
        }
        if let GeometryOutput::MeshPublication(shape) = result {
            if shape.handle().is_none() { return session.mesh_preparation_demand(shape); }
        }
        if let GeometryOutput::MeshExport(export) = result {
            return session.mesh_export_demand(geometry_handle(&data.input, "geometry")?, export);
        }
        if let GeometryOutput::BooleanPreparation(preparation) = result {
            return session.boolean_preparation_demand(geometry_handle(&data.input, "a")?, geometry_handle(&data.input, "b")?, preparation);
        }
        if matches!(result, GeometryOutput::Boolean { .. }) {
            return Ok(RetirementDemand { copy_bytes: BooleanOperand::recipe_copy_bytes(), depth: self.kind.source_depth(session, &data.input)?, ..empty });
        }
        if let GeometryOutput::CompoundCut(draft) = result {
            return draft.demand(session, &data.input);
        }
        if let GeometryOutput::SolidPair(pair) = result {
            if let Some(solid) = pair.iter().find(|solid| !solid.topology_ready()) {
                return session.solid_primitive_demand(solid, false, copy);
            }
        }
        if let GeometryOutput::Solid(solid, _) = result {
            if !solid.topology_ready() { return session.solid_primitive_demand(solid, self.claim_staged, copy); }
        }
        if let GeometryOutput::Vertex(vertex, _) = result {
            if !vertex.topology_ready() { return session.vertex_publication_demand(vertex, self.claim_staged, copy); }
        }
        if let GeometryOutput::Wire(wire, _) = result {
            if !wire.topology_ready() { return session.polygon_wire_demand(wire, self.claim_staged, copy); }
        }
        if let GeometryOutput::WireLength(query) = result {
            let source = self.kind.geometry_source(&data.input)?.ok_or_else(|| ValueError::invariant("bounded length has no source port"))?;
            return session.wire_length_demand(source, query);
        }
        if let GeometryOutput::SectionFaces(faces) = result {
            if let Some(index) = section_faces::unfinished(faces) {
                return session.polygon_face_demand(&faces.face(index).0, None, false, copy);
            }
        }
        if let GeometryOutput::Section(draft) = result {
            return Ok(section_demand(draft));
        }
        if let GeometryOutput::Hull(draft) = result {
            return point_hull_demand(&data.input, draft);
        }
        if let GeometryOutput::Sew(draft) = result {
            return session.cuboid_sew_demand(draft, sew_face(&data.input, draft.next_face().unwrap_or(5))?);
        }
        if let GeometryOutput::Shell { draft, next } = result {
            return shell_selection(session, &data.input, draft, *next).map(|(_, demand)| demand);
        }
        if let GeometryOutput::Loft(draft) = result {
            return session.loft_draft_demand(loft_profiles(&data.input)?, draft);
        }
        if let GeometryOutput::Prism(draft) = result {
            let source = self.kind.geometry_source(&data.input)?.ok_or_else(|| ValueError::invariant("prism has no source port"))?;
            return session.prism_draft_demand(source, draft);
        }
        if let GeometryOutput::PolygonFace(face, output) = result {
            if !face.topology_ready() {
                let source = match output {
                    FaceOutput::Single(_) => self.kind.geometry_source(&data.input)?,
                    FaceOutput::Section(_) => None,
                };
                return session.polygon_face_demand(face, source, self.claim_staged, copy);
            }
        }
        if let GeometryOutput::Face(face, _) = result {
            if !face.topology_ready() { return session.conic_face_demand(face, self.claim_staged, copy); }
        }
        if let GeometryOutput::PlanarMeasure(job, _) = result {
            let source = self.kind.geometry_source(&data.input)?.ok_or_else(|| ValueError::invariant("measurement has no source port"))?;
            return session.planar_measure_demand(source, job);
        }
        if let GeometryOutput::PlanarClosest(job, port) = result {
            return session.planar_closest_demand(geometry_handle(&data.input, port)?, job);
        }
        if let Some(entry) = &data.entry {
            return if self.closing_entry { entry.next_close_demands(copy) } else { entry.next_demands(copy) };
        }
        if let Some((key, field)) = result.field(self.field, &data.input) {
            if data.key.len() < key.len() { return Ok(text_demand(&data.key, key)); }
            if let Some(text) = field.text().filter(|text| data.text.len() < text.len()) { return Ok(text_demand(&data.text, text)); }
            return Ok(match field {
                Field::Decimal(_) => RetirementDemand { copy_bytes: size_of::<f64>(), ..empty },
                _ => empty,
            });
        }
        if let GeometryOutput::SectionFaces(faces) = result {
            return section_faces::demand(session, faces, data, self.section_staged, copy);
        }
        if let GeometryOutput::SolidPair(pair) = result {
            return split::demand(session, pair, data, self.split_staged, copy);
        }
        if let GeometryOutput::Healing(job) = result {
            return session.with_original_geometry_query(|kernel| job.demand(kernel.representation(), kernel.healing_revision(), copy));
        }
        let Some(handle) = result.handle() else { return Ok(empty); };
        if data.live_key.len() < handle.len() { return Ok(text_demand(&data.live_key, handle)); }
        if !self.claim_staged && data.claim_key.len() < handle.len() { return Ok(text_demand(&data.claim_key, handle)); }
        match result {
            GeometryOutput::Geometry(curve, _) => session.analytic_publication_demand(curve, self.claim_staged, copy),
            GeometryOutput::Compound(shape)=>session.compound_publication_demand(shape,self.claim_staged,copy),
            GeometryOutput::SolidPattern(shape) => session.solid_pattern_publication_demand(shape, self.claim_staged, copy),
            GeometryOutput::MeshPublication(shape) => session.mesh_publication_demand(shape, self.claim_staged, copy),
            GeometryOutput::MeshDecode(_) | GeometryOutput::MeshExport(_) | GeometryOutput::StepDecode(_) | GeometryOutput::StepExport(_) => unreachable!("exchange stage owns no live address here"),
            GeometryOutput::Solid(solid, _) => session.solid_primitive_demand(solid, self.claim_staged, copy),
            GeometryOutput::Face(face, _) => session.conic_face_demand(face, self.claim_staged, copy),
            GeometryOutput::PolygonFace(face, _) => {
                let source = self.kind.geometry_source(&data.input)?;
                session.polygon_face_demand(face, source, self.claim_staged, copy)
            }
            GeometryOutput::Wire(wire, _) => session.polygon_wire_demand(wire, self.claim_staged, copy),
            GeometryOutput::Vertex(vertex, _) => session.vertex_publication_demand(vertex, self.claim_staged, copy),
            GeometryOutput::Query { .. } | GeometryOutput::Validation(_) | GeometryOutput::Healing(_) | GeometryOutput::TopologyList(_) | GeometryOutput::PlanarClosest(..) | GeometryOutput::PlanarMeasure(..) | GeometryOutput::Boolean { .. } | GeometryOutput::BooleanPreparation(_) | GeometryOutput::CompoundCut(_) | GeometryOutput::Prism(_) | GeometryOutput::Loft(_) | GeometryOutput::Shell { .. } | GeometryOutput::Sew(_) | GeometryOutput::Hull(_) | GeometryOutput::Section(_) | GeometryOutput::SectionFaces(_) | GeometryOutput::SolidPair(_) | GeometryOutput::WireLength(_) | GeometryOutput::Transform(_) => unreachable!("only geometry handles need a session claim"),
        }
    }

    fn close_demand(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(GeometryOutput::SectionFaces(faces)) = &self.result {
            return section_faces::close_demand(self.session.as_ref().unwrap(), faces, self.section_staged);
        }
        if let Some(GeometryOutput::SolidPair(pair)) = &self.result {
            return split::close_demand(self.session.as_ref().unwrap(), pair, self.split_staged);
        }
        if let Some(GeometryOutput::TopologyList(output)) = &self.result {
            if let Some(key) = output.staged_claim() {
                return self.session.as_ref().unwrap().staged_geometry_claim_close_demand(key);
            }
        }
        if self.claim_staged {
            let key = &self.data.as_ref().unwrap().live_key;
            return self.session.as_ref().unwrap().staged_geometry_claim_close_demand(key);
        }
        if let Some(GeometryOutput::Solid(solid, _)) = &self.result {
            if !solid.terminal_is_empty() { return self.session.as_ref().unwrap().solid_primitive_close_demand(solid); }
        }
        if let Some(GeometryOutput::SolidPattern(shape)) = &self.result {
            if !shape.terminal_is_empty() { return self.session.as_ref().unwrap().solid_pattern_close_demand(shape); }
        }
        if let Some(GeometryOutput::StepExport(export)) = &self.result {
            return Ok(RetirementDemand { release_bytes: size_of::<StepWrite>() + export.output_capacity(), depth: 1, ..Default::default() });
        }
        if let Some(GeometryOutput::StepDecode(decoder)) = &self.result {
            if !decoder.terminal_is_empty() { return Ok(decoder.close_demand()); }
            return Ok(RetirementDemand { release_bytes: size_of::<StepSolidDecode>(), depth: 1, ..Default::default() });
        }
        if let Some(GeometryOutput::MeshDecode(decoder)) = &self.result {
            if !decoder.terminal_is_empty() { return Ok(decoder.close_demand()); }
        }
        if let Some(GeometryOutput::MeshPublication(shape)) = &self.result {
            if !shape.terminal_is_empty() { return self.session.as_ref().unwrap().mesh_publication_close_demand(shape, copy); }
        }
        if let Some(GeometryOutput::Vertex(vertex, _)) = &self.result {
            if !vertex.terminal_is_empty() { return self.session.as_ref().unwrap().vertex_close_demand(vertex); }
        }
        if let Some(GeometryOutput::Wire(wire, _)) = &self.result {
            if !wire.terminal_is_empty() { return self.session.as_ref().unwrap().polygon_wire_close_demand(wire); }
        }
        if let Some(GeometryOutput::PolygonFace(face, _)) = &self.result {
            if !face.terminal_is_empty() { return self.session.as_ref().unwrap().polygon_face_close_demand(face); }
        }
        if let Some(GeometryOutput::Face(face, _)) = &self.result {
            if !face.terminal_is_empty() { return self.session.as_ref().unwrap().conic_face_close_demand(face); }
        }
        if let Some(owner) = &self.retiring { return owner.selected_demand(copy); }
        if self.data.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        let Some(session) = &self.session else { return Ok(RetirementDemand::default()); };
        Ok(RetirementDemand {
            copy_bytes: session.next_close_copy_byte_demand()?,
            capacity_bytes: session.next_close_capacity_byte_demand(copy)?,
            release_bytes: session.next_close_release_byte_demand()?,
            depth: session.next_close_depth_demand()?,
        })
    }

    fn close_owner(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        let grant = effect.grant();
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, self.close_demand(grant.maximum_copy_bytes)?) {
            return Ok(Step::Progress(Progress::default()));
        }
        if let Some(GeometryOutput::SectionFaces(faces)) = &mut self.result {
            if let Some(receipt) = section_faces::close(self.session.as_ref().unwrap(), faces, &mut self.section_staged, effect)? {
                return Ok(Step::Progress(receipt));
            }
            if !effect.performed(1)? { return Ok(Step::Progress(Progress::default())); }
            drop(self.result.take());
            return Ok(Step::Progress(Progress { released_bytes: size_of::<PolygonSectionSet>(), ..item() }));
        }
        if let Some(GeometryOutput::SolidPair(pair)) = &mut self.result {
            if let Some(receipt) = split::close(self.session.as_ref().unwrap(), pair, &mut self.split_staged, effect)? {
                return Ok(Step::Progress(receipt));
            }
            if !effect.performed(1)? { return Ok(Step::Progress(Progress::default())); }
            drop(self.result.take());
            return Ok(Step::Progress(Progress { released_bytes: size_of::<[SolidPublication; 2]>(), ..item() }));
        }
        if let Some(GeometryOutput::TopologyList(output)) = &mut self.result {
            if output.staged_claim().is_some() {
                return output.close_claim(self.session.as_ref().unwrap(), effect).map(Step::Progress);
            }
        }
        if self.claim_staged {
            let key = &self.data.as_ref().unwrap().live_key;
            let progress = self.session.as_ref().unwrap().close_staged_geometry_claim(key, effect)?;
            if progress.copied_items > 0 { self.claim_staged = false; }
            return Ok(Step::Progress(progress));
        }
        if let Some(GeometryOutput::Solid(solid, _)) = &mut self.result {
            if !solid.terminal_is_empty() {
                let step = self.session.as_ref().unwrap().close_solid_primitive(solid, effect)?;
                return Ok(Step::Progress(step.progress()));
            }
        }
        if let Some(GeometryOutput::Vertex(vertex, _)) = &mut self.result {
            if !vertex.terminal_is_empty() {
                let step = self.session.as_ref().unwrap().close_vertex(vertex, effect)?;
                return Ok(Step::Progress(step.progress()));
            }
        }
        if let Some(GeometryOutput::SolidPattern(shape)) = &mut self.result {
            if !shape.terminal_is_empty() {
                let step = self.session.as_ref().unwrap().close_solid_pattern(shape, effect)?;
                return Ok(Step::Progress(step.progress()));
            }
        }
        if let Some(GeometryOutput::StepExport(export)) = &self.result {
            let released_bytes = size_of::<StepWrite>() + export.output_capacity();
            if !effect.performed(1)? { return Ok(Step::Progress(Progress::default())); }
            drop(self.result.take());
            return Ok(Step::Progress(Progress { released_bytes, ..item() }));
        }
        if let Some(GeometryOutput::StepDecode(decoder)) = &mut self.result {
            if !decoder.terminal_is_empty() { return decoder.close(effect).map(Step::Progress); }
            if !effect.performed(1)? { return Ok(Step::Progress(Progress::default())); }
            drop(self.result.take());
            return Ok(Step::Progress(Progress { released_bytes: size_of::<StepSolidDecode>(), ..item() }));
        }
        if let Some(GeometryOutput::MeshDecode(decoder)) = &mut self.result {
            if !decoder.terminal_is_empty() { return decoder.close(effect); }
        }
        if let Some(GeometryOutput::MeshPublication(shape)) = &mut self.result {
            if !shape.terminal_is_empty() { return self.session.as_ref().unwrap().close_mesh_publication(shape, effect); }
        }
        if let Some(GeometryOutput::Wire(wire, _)) = &mut self.result {
            if !wire.terminal_is_empty() {
                let step = self.session.as_ref().unwrap().close_polygon_wire(wire, effect)?;
                return Ok(Step::Progress(step.progress()));
            }
        }
        if let Some(GeometryOutput::PolygonFace(face, _)) = &mut self.result {
            if !face.terminal_is_empty() {
                let step = self.session.as_ref().unwrap().close_polygon_face(face, effect)?;
                return Ok(Step::Progress(step.progress()));
            }
        }
        if let Some(GeometryOutput::Face(face, _)) = &mut self.result {
            if !face.terminal_is_empty() {
                let step = self.session.as_ref().unwrap().close_conic_face(face, effect)?;
                return Ok(Step::Progress(step.progress()));
            }
        }
        if let Some(owner) = &mut self.retiring {
            let outcome = owner.step(grant.maximum_copy_bytes, effect);
            let RetainedWorkPoll::Complete(outcome) = outcome else { return Ok(Step::Progress(Progress::default())); };
            let step = outcome.result.map_err(|error| error.into_value_error())?;
            if owner.terminal_is_empty() { self.retiring = None; }
            return Ok(Step::Progress(step.progress()));
        }
        if self.data.is_some() {
            if !effect.performed(1)? { return Ok(Step::Progress(Progress::default())); }
            let mut data=self.data.take().unwrap();
            match self.result.take(){
                Some(GeometryOutput::Geometry(shape,_))=>data.retired_analytic=Some(shape),
                Some(GeometryOutput::Compound(shape))=>data.retired_compound=Some(shape),
                Some(GeometryOutput::Validation(report))=>data.retired_validation=Some(report),
                Some(GeometryOutput::Healing(job)) => data.retired_healing = Some(job),
                Some(GeometryOutput::MeshExport(export))=>data.retired_mesh_export=Some(export),
                Some(GeometryOutput::TopologyList(output)) => data.retired_topology_list = Some(output),
                _=>{}
            }
            match ControlledRetirement::new(data) {
                Ok(owner) => self.retiring = Some(owner),
                Err((error, data)) => { self.data = Some(data); return Err(error); }
            }
            return Ok(Step::Progress(item()));
        }
        if let Some(session) = &mut self.session {
            let step = session.close_step(effect)?;
            if session.terminal_is_empty() { self.session = None; }
            return Ok(step);
        }
        Ok(Step::Complete(Progress::default()))
    }
}

fn step_export_source<T>(session: &Session, input: &Dictionary, read: impl FnOnce(StepWriteSource<'_>) -> Result<T, ValueError>) -> Result<T, ValueError> {
    let handle = geometry_handle(input, "geometry")?;
    session.with_original_certified_solid(handle, |body, solid, revision, depth| {
        read(StepWriteSource { body, solid, revision, handle, depth })
    })
}

fn step_export_demand(export: &StepWrite, source: StepWriteSource<'_>) -> Result<RetirementDemand, ValueError> {
    let demand = export.demand(source)?;
    if export.ready() {
        return Ok(RetirementDemand { copy_bytes: size_of::<String>(), release_bytes: size_of::<StepWrite>(), ..demand });
    }
    Ok(demand)
}

fn step_decode_demand(decoder: &StepSolidDecode) -> RetirementDemand {
    let quote = decoder.demand();
    if decoder.is_ready() { RetirementDemand { release_bytes: size_of::<StepSolidDecode>(), ..quote } }
    else { quote }
}

fn text_demand(target: &String, source: &str) -> RetirementDemand {
    let (copy_bytes, capacity_bytes) = if target.capacity() == 0 { (0, source.len()) } else { (1, 0) };
    RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
}

fn item() -> Progress { Progress { copied_items: 1, ..Progress::default() } }

fn geometry_handle<'a>(input: &'a Dictionary, channel: &str) -> Result<&'a str, ValueError> {
    geometry_atom_handle(input.get(channel))
}

fn exchange_text(input: &Dictionary) -> Result<&str, ValueError> {
    input.get("data").and_then(Value::as_dictionary).and_then(|value| value.get("value"))
        .and_then(Value::as_atom).and_then(Atom::as_str)
        .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "original exchange import requires a text data value"))
}

fn geometry_atom_handle(value: Option<&Value>) -> Result<&str, ValueError> {
    geometry_handle_value(value).filter(|source| source.len() == 64)
        .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "geometry operation requires a canonical handle"))
}

fn geometry_handle_value(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_dictionary).and_then(|value| value.get("handle")).and_then(Value::as_atom).and_then(Atom::as_str)
}

fn loft_profiles(input: &Dictionary) -> Result<[&str; 2], ValueError> {
    let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "original Loft requires two profiles in a geometry list");
    let list = input.get("profiles").and_then(Value::as_dictionary)
        .filter(|list| list.schema() == Some("list") && list.len() == 3).ok_or_else(invalid)?;
    let read = |rank| {
        let (key, value) = list.entry_at_rank(rank + 1).ok_or_else(invalid)?;
        let index = key.parse::<usize>().map_err(|_| invalid())?;
        Ok::<_, ValueError>((index, geometry_atom_handle(Some(value))?))
    };
    let (first_index, first) = read(0)?;
    let (last_index, last) = read(1)?;
    if first_index == last_index { return Err(invalid()); }
    Ok(if first_index < last_index { [first, last] } else { [last, first] })
}

fn shell_open_faces(input: &Dictionary) -> Result<Option<&Dictionary>, ValueError> {
    input.get("openFaces").map(|value| value.as_dictionary().filter(|list| list.schema() == Some("list"))
        .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "shell openings require a geometry list"))).transpose()
}

fn dictionary_depth(dictionary: &Dictionary) -> usize {
    dictionary.len().checked_ilog2().map_or(1, |log| 2 * (log as usize + 1))
}

fn section_demand(draft: &BoxSectionDraft) -> RetirementDemand {
    RetirementDemand { copy_bytes: draft.copied_coordinates() * size_of::<f64>(), depth: 1, ..Default::default() }
}

fn point_hull_demand(input: &Dictionary, draft: &HullDraft) -> Result<RetirementDemand, ValueError> {
    let (copy_bytes, depth) = if let Some(rank) = draft.next_point() {
        let list = point_list(input)?;
        let (_, value) = list.entry_at_rank(rank + 1).ok_or_else(invalid_point_list)?;
        let point = value.as_dictionary().ok_or_else(invalid_point_list)?;
        (size_of::<[f64; 3]>(), dictionary_depth(input).max(dictionary_depth(list)).max(dictionary_depth(point)))
    } else if draft.has_candidate() {
        (12 * size_of::<f64>(), 1)
    } else {
        (0, 1)
    };
    Ok(RetirementDemand { copy_bytes, depth, ..Default::default() })
}

fn shell_selection<'a>(
    session: &Session, input: &'a Dictionary, draft: &ShellDraft, next: usize,
) -> Result<(Option<Option<&'a str>>, RetirementDemand), ValueError> {
    if let Some(list) = shell_open_faces(input)? {
        if let Some((key, value)) = list.entry_at_rank(next) {
            let rank_depth = dictionary_depth(input).max(dictionary_depth(list));
            let face = (key.len() <= 20 && key.parse::<usize>().is_ok()).then(|| geometry_atom_handle(Some(value))).transpose()?;
            let depth = face.map_or(Ok(rank_depth), |handle| session.shell_face_depth(draft, handle).map(|depth| depth.max(rank_depth)))?;
            return Ok((Some(face), RetirementDemand { copy_bytes: usize::from(face.is_some()), depth, ..Default::default() }));
        }
    }
    let source = geometry_handle(input, "geometry")?;
    Ok((None, RetirementDemand { copy_bytes: 16 * size_of::<f64>(), depth: session.certified_solid_depth(source)?, ..Default::default() }))
}

fn number(input: &Dictionary, port: &str, field: &str) -> Result<f64, ValueError> {
    input.get(port).and_then(Value::as_dictionary).and_then(|value| value.get(field)).and_then(Value::as_atom)
        .and_then(Atom::as_f64).filter(|value| value.is_finite())
        .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "geometry input requires a finite numeric component"))
}

fn point(input: &Dictionary, port: &str) -> Result<[f64; 3], ValueError> {
    Ok([number(input, port, "x")?, number(input, port, "y")?, number(input, port, "z")?])
}

fn invalid_point_list() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "geometry requires a dense list of finite ordered points")
}

fn point_list(input: &Dictionary) -> Result<&Dictionary, ValueError> {
    input.get("points").and_then(Value::as_dictionary).filter(|list| list.schema() == Some("list"))
        .ok_or_else(invalid_point_list)
}

fn invalid_boundary_list() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Coons patch requires four ordered lists of boundary points")
}

fn nested_boundary_list(input: &Dictionary) -> Result<&Dictionary, ValueError> {
    input.get("curves").and_then(Value::as_dictionary).filter(|list| list.schema() == Some("list"))
        .ok_or_else(invalid_boundary_list)
}

fn boundary_point_list(source: &Dictionary, index: usize) -> Result<&Dictionary, ValueError> {
    let names = ["0", "1", "2", "3"];
    source.get(names[index]).and_then(Value::as_dictionary).filter(|list| list.schema() == Some("list"))
        .ok_or_else(invalid_boundary_list)
}

fn list_key(mut index:usize,digits:&mut [u8;20])->&str{
    let mut start=digits.len();
    loop {
        start-=1;
        digits[start]=b'0'+(index%10)as u8;
        index/=10;
        if index==0{break;}
    }
    std::str::from_utf8(&digits[start..]).expect("decimal index is ASCII")
}

fn listed_point(list:&Dictionary,index:usize)->Result<[f64;3],ValueError>{
    point(list,list_key(index,&mut [0;20]))
}

fn box_finish_source(input:&Dictionary,operation:GeometryOperation)->Result<(&str,[&str;4],usize),ValueError>{
    let (port,message,limit)=match operation{
        GeometryOperation::Draft=>("faces","retained draft requires exactly one selected side",1),
        GeometryOperation::FilletEdges=>("edges","retained fillet requires one to four selected parallel edges",4),
        _=>("edges","retained chamfer requires exactly one selected edge",1),
    };
    let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,message);
    let list=input.get(port).and_then(Value::as_dictionary)
        .filter(|list|list.schema()==Some("list")&&(2..=limit+1).contains(&list.len())).ok_or_else(invalid)?;
    let count=list.len()-1;
    let mut members=["";4];
    for (i,member) in members[..count].iter_mut().enumerate(){
        let (key,value)=list.entry_at_rank(i+1).ok_or_else(invalid)?;
        key.parse::<usize>().map_err(|_|invalid())?;
        *member=geometry_atom_handle(Some(value))?;
    }
    Ok((geometry_handle(input,"geometry")?,members,count))
}

fn defeature_source(input: &Dictionary) -> Result<(&str, [&str; Brep::DEFEATURE_FACE_COUNT], usize, usize), ValueError> {
    let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "defeature requires the bore wall and any floor or all twenty distinct fillet faces");
    let list = input.get("faces").and_then(Value::as_dictionary)
        .filter(|list| list.schema() == Some("list")).ok_or_else(invalid)?;
    let count = list.len().saturating_sub(1);
    if !matches!(count, 1 | 2 | Brep::DEFEATURE_FACE_COUNT) { return Err(invalid()); }
    let mut depth = dictionary_depth(input).max(dictionary_depth(list));
    let shape = input.get("geometry");
    if let Some(dictionary) = shape.and_then(Value::as_dictionary) { depth = depth.max(dictionary_depth(dictionary)); }
    let mut members = [""; Brep::DEFEATURE_FACE_COUNT];
    for (index, member) in members[..count].iter_mut().enumerate() {
        let value = list.get(list_key(index, &mut [0; 20]));
        if let Some(dictionary) = value.and_then(Value::as_dictionary) { depth = depth.max(dictionary_depth(dictionary)); }
        *member = geometry_atom_handle(value).map_err(|_| invalid())?;
    }
    Ok((geometry_atom_handle(shape)?, members, count, depth))
}

fn invalid_sew_input() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "sewing requires an ordered list of six original faces")
}

fn sew_faces(input: &Dictionary) -> Result<&Dictionary, ValueError> {
    input.get("faces").and_then(Value::as_dictionary).filter(|list| list.schema() == Some("list")).ok_or_else(invalid_sew_input)
}

fn sew_face(input: &Dictionary, index: usize) -> Result<&str, ValueError> {
    geometry_handle(sew_faces(input)?, list_key(index, &mut [0; 20]))
}

fn compound_members(input:&Dictionary)->Result<&Dictionary,ValueError>{
    input.get("solids").and_then(Value::as_dictionary).filter(|list|list.schema()==Some("list"))
        .ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"compound requires an ordered list of original solids"))
}

fn compound_member(input:&Dictionary,index:usize)->Result<&str,ValueError>{
    geometry_handle(compound_members(input)?,list_key(index,&mut [0;20]))
}

fn write_text(target: &mut String, source: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
    let grant = effect.grant();
    if target.capacity() == 0 {
        if grant.maximum_capacity_bytes < source.len() || !effect.performed(1)? { return Ok(Progress::default()); }
        target.try_reserve_exact(source.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "geometry output text allocation failed"))?;
        return Ok(Progress { retained_capacity_bytes: target.capacity(), ..item() });
    }
    let count = (source.len() - target.len()).min(grant.maximum_copy_bytes);
    if count == 0 || !effect.performed(1)? { return Ok(Progress::default()); }
    target.push_str(&source[target.len()..target.len() + count]);
    Ok(Progress { copied_bytes: count, ..item() })
}

impl OperatorJob for OriginalGeometryJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> Result<(OperatorJobStep, Progress), OperatorWorkFailure> {
        self.receipt = Progress::default();
        if cx.is_cancelled() || self.cancelled { return Ok((OperatorJobStep::Cancelled(self.progress), self.receipt)); }
        let grant = cx.retained_grant();
        if self.done || cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok((OperatorJobStep::Working(self.progress), self.receipt));
        }
        let Some((result, work)) = cx.with_original_effect(grant, |_, effect| self.advance(effect))? else {
            return Ok((OperatorJobStep::Working(self.progress), self.receipt));
        };
        cx.consume_work(work);
        let (output, progress) = match result {
            Ok(result) => result,
            Err(error) => {
                self.receipt = error.retained_progress();
                cx.consume_retained(self.receipt)?;
                return Err(error.into());
            }
        };
        self.receipt = progress;
        cx.consume_retained(progress)?;
        self.progress.units_done += usize::from(work > 0);
        self.progress.units_total = self.progress.units_done + usize::from(output.is_none());
        if let Some(output) = output {
            self.done = true;
            self.progress.phase = "complete";
            return Ok((OperatorJobStep::Done(output), progress));
        }
        Ok((OperatorJobStep::Working(self.progress), progress))
    }

    fn normal_step_progress(&self) -> Progress { self.receipt }
    fn next_step_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.step_demand(0)?.copy_bytes) }
    fn next_step_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.step_demand(copy)?.capacity_bytes) }
    fn next_step_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.step_demand(0)?.release_bytes) }
    fn next_step_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.step_demand(0)?.depth) }
    fn progress(&self) -> OperatorProgress { self.progress }
    fn cancel(&mut self) { self.cancelled = !self.done; }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.close_demand(copy)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.depth) }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let result = self.close_owner(effect);
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result: result.map_err(Into::into), work: effect.performed_work() })
    }

    fn terminal_is_empty(&self) -> bool { self.data.is_none() && self.retiring.is_none() && self.session.is_none() }
}
