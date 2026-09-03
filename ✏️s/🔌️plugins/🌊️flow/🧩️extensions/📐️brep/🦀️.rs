//! 🔷️ Flow brep extension — geometry operators packaged as a runtime-installable unit.

use flow_extension_sdk::brep_geometry::*;
use flow_extension_sdk::build_manifest_json;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{operation_quality, BrepKernel, BREP_KERNEL_OPERATIONS};
use neural_engine::{channel_output, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};

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
    ("brep.xform.scale", "scale"),
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
    ("brep.io.exportDwg", "export_dwg"),
    ("brep.io.importDwg", "import_dwg"),
];

/// 🙈️ `BrepKernel` methods this extension deliberately exposes NO node for, with why — checked by
/// `every_kernel_operation_is_either_a_node_or_explicitly_unexposed` against
/// [`BREP_KERNEL_OPERATIONS`] so a newly added trait method can never silently fall through both
/// lists unnoticed.
const INTENTIONALLY_UNEXPOSED: &[(&str, &str)] = &[
    ("kind", "internal handle-kind lookup behind geometry_dict, not a graph operation"),
    ("tessellate", "internal preview/export bridge (tessellate_geometry), not a graph node"),
    ("dispose", "internal GC primitive (dispose_geometry), not a graph node"),
    ("retain", "internal GC primitive (retain_geometry_handles), not a graph node"),
    ("registry_len", "internal diagnostic counter, not a graph node"),
    ("export_gltf", "glTF/GLB leaves the extension only via the tessellation mesh bridge (export_solid_json \"glb\"), never this trait method directly"),
];

macro_rules! geo_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name;
        impl Operator for $name {
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                with_kernel(|$k| {
                    let $i = input;
                    let handle = $expr.map_err(map_kernel_error)?;
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
        struct $name;
        impl Operator for $name {
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                with_kernel_read(|$k| {
                    let $i = input;
                    let value = $expr.map_err(map_kernel_error)?;
                    Ok(channel_output($channel, number_dictionary(value)))
                })
            }
        }
    };
}

macro_rules! point_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name;
        impl Operator for $name {
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                with_kernel_read(|$k| {
                    let $i = input;
                    let value = $expr.map_err(map_kernel_error)?;
                    Ok(channel_output($channel, point_dictionary(value)))
                })
            }
        }
    };
}

macro_rules! vec_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name;
        impl Operator for $name {
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                with_kernel_read(|$k| {
                    let $i = input;
                    let value = $expr.map_err(map_kernel_error)?;
                    Ok(channel_output($channel, vector_dictionary(value)))
                })
            }
        }
    };
}

macro_rules! text_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name;
        impl Operator for $name {
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                with_kernel_read(|$k| {
                    let $i = input;
                    let value = $expr.map_err(map_kernel_error)?;
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

struct ConvexHullPrim;
impl Operator for ConvexHullPrim {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let handle = kernel.convex_hull(&points).map_err(map_kernel_error)?;
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

struct PolylineWire;
impl Operator for PolylineWire {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let handle = kernel.polyline_wire(&points).map_err(map_kernel_error)?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(RectangleWire, "wire", |k, i| k.rectangle_wire(read_channel_number(i, "width")?, read_channel_number(i, "height")?));
geo_operation!(RegularPolygonWire, "wire", |k, i| k.regular_polygon_wire(read_channel_number(i, "radius")?, read_channel_number(i, "sides")? as usize));

struct InterpolateCurve;
impl Operator for InterpolateCurve {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let degree = read_channel_number(input, "degree")? as usize;
            let handle = kernel.interpolate_curve(&points, degree).map_err(map_kernel_error)?;
            Ok(channel_output("curve", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ApproximateCurve;
impl Operator for ApproximateCurve {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let degree = read_channel_number(input, "degree")? as usize;
            let control_points = read_channel_number(input, "controlPoints")? as usize;
            let handle = kernel.approximate_curve(&points, degree, control_points).map_err(map_kernel_error)?;
            Ok(channel_output("curve", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(HelixCurve, "curve", |k, i| k.helix_curve(read_xyz(i, "origin")?, read_xyz(i, "axis")?, read_channel_number(i, "radius")?, read_channel_number(i, "pitch")?, read_channel_number(i, "turns")?,));
// #endregion 🔖️Curves

// #region 🔖️Surfaces
geo_operation!(PlaneSurface, "surface", |k, i| k.plane_surface(read_xyz(i, "origin")?, read_xyz(i, "normal")?));

struct PlanarFacePoints;
impl Operator for PlanarFacePoints {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let handle = kernel.planar_face_from_points(&points).map_err(map_kernel_error)?;
            Ok(channel_output("face", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(PlanarFaceWire, "face", |k, i| k.planar_face_from_wire(&read_geometry(i, "wire")?));

struct NurbsGridSurface;
impl Operator for NurbsGridSurface {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = read_point_list(input, "points")?;
            let rows = read_channel_number(input, "rows")? as usize;
            let grid = points_to_grid(&points, rows)?;
            let degree_u = read_channel_number(input, "degreeU")? as usize;
            let degree_v = read_channel_number(input, "degreeV")? as usize;
            let handle = kernel.nurbs_surface_from_grid(&grid, degree_u, degree_v).map_err(map_kernel_error)?;
            Ok(channel_output("surface", geometry_dict(kernel, &handle)?))
        })
    }
}

struct CoonsPatch;
impl Operator for CoonsPatch {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let curves = read_nested_point_lists(input, "curves")?;
            let handle = kernel.coons_patch(&curves).map_err(map_kernel_error)?;
            Ok(channel_output("surface", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(OffsetFace, "face", |k, i| k.offset_face(&read_geometry(i, "face")?, read_channel_number(i, "distance")?));
geo_operation!(ThickenFace, "solid", |k, i| k.thicken_face(&read_geometry(i, "face")?, read_channel_number(i, "thickness")?));
// #endregion 🔖️Surfaces

// #region 🔖️Sweeps
struct ExtrudeCurve;
impl Operator for ExtrudeCurve {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let wire = read_geometry(input, "wire")?;
            let vector = read_xyz(input, "vector")?;
            let handle = kernel.extrude_wire(&wire, vector).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ExtrudeFace;
impl Operator for ExtrudeFace {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let face = read_geometry(input, "face")?;
            let vector = read_xyz(input, "vector")?;
            let distance = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
            if distance < 1e-12 {
                return Err(EvalError::InvalidInput("extrusion vector magnitude must be positive".into()));
            }
            let direction = [vector[0] / distance, vector[1] / distance, vector[2] / distance];
            let handle = kernel.extrude(&face, direction, distance).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
geo_operation!(Revolve, "solid", |k, i| k.revolve(&read_geometry(i, "face")?, read_xyz(i, "axisOrigin")?, read_xyz(i, "axisDirection")?, read_channel_number(i, "angle")?,));
geo_operation!(Sweep, "solid", |k, i| k.sweep(&read_geometry(i, "profile")?, &read_geometry(i, "path")?));

struct Loft;
impl Operator for Loft {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let profiles = read_geometry_list(input, "profiles")?;
            let smooth = read_channel_number(input, "smooth")? >= 0.5;
            let handle = kernel.loft(&profiles, smooth).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

struct Pipe;
impl Operator for Pipe {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let profile = read_geometry(input, "profile")?;
            let path = read_geometry(input, "path")?;
            let guide_handle = read_optional_geometry(input, "guide");
            let guide = guide_handle.as_ref();
            let handle = kernel.pipe(&profile, &path, guide).map_err(map_kernel_error)?;
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
geo_operation!(Fuse, "solid", |k, i| k.fuse(&read_geometry(i, "a")?, &read_geometry(i, "b")?));
geo_operation!(Cut, "solid", |k, i| k.cut(&read_geometry(i, "a")?, &read_geometry(i, "b")?));
geo_operation!(Intersect, "solid", |k, i| k.intersect(&read_geometry(i, "a")?, &read_geometry(i, "b")?));

struct CompoundCut;
impl Operator for CompoundCut {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let target = read_geometry(input, "target")?;
            let tools = read_geometry_list(input, "tools")?;
            let handle = kernel.compound_cut(&target, &tools).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️Booleans

// #region 🔖️Transforms
geo_operation!(Translate, "geometry", |k, i| k.translate(&read_geometry(i, "geometry")?, read_xyz(i, "offset")?));
geo_operation!(Rotate, "geometry", |k, i| k.rotate(&read_geometry(i, "geometry")?, read_xyz(i, "axis")?, read_channel_number(i, "angle")?));
geo_operation!(RotateAbout, "geometry", |k, i| k.rotate_about(&read_geometry(i, "geometry")?, read_xyz(i, "origin")?, read_xyz(i, "axis")?, read_channel_number(i, "angle")?));
geo_operation!(Scale, "geometry", |k, i| k.scale(&read_geometry(i, "geometry")?, read_channel_number(i, "factor")?, read_xyz(i, "center")?));
geo_operation!(Mirror, "geometry", |k, i| k.mirror(&read_geometry(i, "geometry")?, read_xyz(i, "origin")?, read_xyz(i, "normal")?));
geo_operation!(CopyShape, "geometry", |k, i| k.copy_shape(&read_geometry(i, "geometry")?));
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
struct FilletEdges;
impl Operator for FilletEdges {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let edges = read_geometry_list(input, "edges")?;
            let radius = read_channel_number(input, "radius")?;
            let handle = kernel.fillet_edges(&geometry, &edges, radius).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ChamferEdges;
impl Operator for ChamferEdges {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let edges = read_geometry_list(input, "edges")?;
            let distance = read_channel_number(input, "distance")?;
            let handle = kernel.chamfer_edges(&geometry, &edges, distance).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ShellMutation;
impl Operator for ShellMutation {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let thickness = read_channel_number(input, "thickness")?;
            let open_faces = read_geometry_list_or_empty(input, "openFaces")?;
            let handle = kernel.shell(&geometry, thickness, &open_faces).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

struct Draft;
impl Operator for Draft {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let faces = read_geometry_list(input, "faces")?;
            let handle = kernel.draft(&geometry, &faces, read_xyz(input, "pullDirection")?, read_xyz(input, "neutralPoint")?, read_channel_number(input, "angle")?).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(OffsetSolid, "solid", |k, i| k.offset_solid(&read_geometry(i, "geometry")?, read_channel_number(i, "distance")?));

struct Defeature;
impl Operator for Defeature {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let faces = read_geometry_list(input, "faces")?;
            let handle = kernel.defeature(&geometry, &faces).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️Features

// #region 🔖️Intersect
/// 🍰️ Emits EVERY section face the plane produced, not just the first — a solid with multiple
/// disjoint cross-sections must not lose the rest silently (audit §13.2).
struct Section;
impl Operator for Section {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let faces = kernel.section(&read_geometry(input, "solid")?, read_xyz(input, "planeOrigin")?, read_xyz(input, "planeNormal")?).map_err(map_kernel_error)?;
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
struct Split;
impl Operator for Split {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let (positive, negative) = kernel.split(&read_geometry(input, "solid")?, read_xyz(input, "planeOrigin")?, read_xyz(input, "planeNormal")?).map_err(map_kernel_error)?;
            Ok(Dictionary::new().insert("positive", Value::Dictionary(geometry_dict(kernel, &positive)?)).insert("negative", Value::Dictionary(geometry_dict(kernel, &negative)?)))
        })
    }
}

struct CurveCurveIntersect;
impl Operator for CurveCurveIntersect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = kernel.curve_curve_intersect(&read_geometry(input, "a")?, &read_geometry(input, "b")?, read_channel_number(input, "tolerance")?).map_err(map_kernel_error)?;
            let handle = wire_from_points(kernel, &points)?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

struct CurveSurfaceIntersect;
impl Operator for CurveSurfaceIntersect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = kernel.curve_surface_intersect(&read_geometry(input, "curve")?, &read_geometry(input, "surface")?, read_channel_number(input, "tolerance")?).map_err(map_kernel_error)?;
            let handle = wire_from_points(kernel, &points)?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

/// 〰️ Emits EVERY intersection wire (two surfaces can meet along several disjoint curves), not
/// just the first (audit §13.2).
struct SurfaceSurfaceIntersect;
impl Operator for SurfaceSurfaceIntersect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let wires = kernel.surface_surface_intersect(&read_geometry(input, "a")?, &read_geometry(input, "b")?, read_channel_number(input, "tolerance")?).map_err(map_kernel_error)?;
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

struct CurveDomain;
impl Operator for CurveDomain {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let domain = kernel.curve_domain(&read_geometry(input, "curve")?).map_err(map_kernel_error)?;
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
struct CurveClosestParameter;
impl Operator for CurveClosestParameter {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let (parameter, point, distance) = kernel.curve_closest_parameter(&read_geometry(input, "curve")?, read_xyz(input, "point")?).map_err(map_kernel_error)?;
            Ok(Dictionary::new()
                .insert("parameter", Value::Dictionary(number_dictionary(parameter)))
                .insert("point", Value::Dictionary(point_dictionary(point)))
                .insert("distance", Value::Dictionary(number_dictionary(distance))))
        })
    }
}

/// 🎯️ Certified nearest `(u, v)` on a surface — see [`CurveClosestParameter`]'s docstring.
struct SurfaceClosestUv;
impl Operator for SurfaceClosestUv {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let (u, v, point, distance) = kernel.surface_closest_uv(&read_geometry(input, "surface")?, read_xyz(input, "point")?).map_err(map_kernel_error)?;
            Ok(Dictionary::new()
                .insert("u", Value::Dictionary(number_dictionary(u)))
                .insert("v", Value::Dictionary(number_dictionary(v)))
                .insert("point", Value::Dictionary(point_dictionary(point)))
                .insert("distance", Value::Dictionary(number_dictionary(distance))))
        })
    }
}
// #endregion 🔖️Evaluate

// #region 🔖️Topology
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{Brep, GeometryHandle};

/// 📇️ A `geometry`-schema list, each entry carrying its own live [`GeometryKind`] (via
/// `geometry_dict`) — unlike [`topology_list`], which hardcodes one fixed schema/kind for the
/// vertex/edge/face lists it was built for, this never mislabels a shell as a `"solid"`.
fn geometry_list(kernel: &Brep, handles: Vec<GeometryHandle>) -> Result<Dictionary, EvalError> {
    handles.into_iter().enumerate().try_fold(Dictionary::with_schema("list"), |list, (index, handle)| Ok(list.insert(index.to_string(), Value::Dictionary(geometry_dict(kernel, &handle)?))))
}

/// 🐚️ The solid's shells as independent geometry handles — `solid_shells` never silently fuses
/// or drops inner voids/cavities, one output entry per shell.
struct SolidShells;
impl Operator for SolidShells {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let shells = kernel.solid_shells(&read_geometry(input, "solid")?).map_err(map_kernel_error)?;
            let list = geometry_list(kernel, shells)?;
            Ok(Dictionary::new().insert("shells", Value::Dictionary(list)))
        })
    }
}

struct CompoundOf;
impl Operator for CompoundOf {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let solids = read_geometry_list(input, "solids")?;
            let handle = kernel.compound(&solids).map_err(map_kernel_error)?;
            Ok(channel_output("compound", geometry_dict(kernel, &handle)?))
        })
    }
}

/// 💥️ Inverse of [`CompoundOf`] — every member solid as its own handle, none silently merged.
struct Explode;
impl Operator for Explode {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let solids = kernel.explode(&read_geometry(input, "compound")?).map_err(map_kernel_error)?;
            let list = geometry_list(kernel, solids)?;
            Ok(Dictionary::new().insert("solids", Value::Dictionary(list)))
        })
    }
}

/// 🏷️ The handle's persistent label (stable across deconstruct/reconstruct) as a diagnostic
/// number — explicit `EvalError`, never a silent `0`/`-1` placeholder, when the handle carries none.
struct GeometryLabel;
impl Operator for GeometryLabel {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let handle = read_geometry(input, "geometry")?;
            let label = kernel.label(&handle).ok_or_else(|| EvalError::InvalidInput(format!("geometry {} carries no persistent label", handle.as_str())))?;
            Ok(channel_output("label", number_dictionary(label as f64)))
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

struct ClosestPoint;
impl Operator for ClosestPoint {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let result = kernel.closest_point(&read_geometry(input, "geometry")?, read_xyz(input, "point")?).map_err(map_kernel_error)?;
            Ok(channel_output("point", point_dictionary(result.point)))
        })
    }
}

struct ClassifyPoint;
impl Operator for ClassifyPoint {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let classification = kernel.classify_point(&read_geometry(input, "solid")?, read_xyz(input, "point")?).map_err(map_kernel_error)?;
            Ok(channel_output("classification", number_dictionary(classify_number(classification))))
        })
    }
}

text_operation!(Validate, "report", |k, i| k.validate(&read_geometry(i, "geometry")?));
// #endregion 🔖️Measure

// #region 🔖️Utilities
geo_operation!(Vertex, "vertex", |k, i| k.vertex(read_xyz(i, "point")?));
geo_operation!(FaceFromWire, "face", |k, i| k.face_from_wire(&read_geometry(i, "wire")?));

struct SewFaces;
impl Operator for SewFaces {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let faces = read_geometry_list(input, "faces")?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.sew_faces(&faces, tolerance).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}

geo_operation!(HealSolid, "solid", |k, i| k.heal_solid(&read_geometry(i, "geometry")?, read_channel_number(i, "tolerance")?));
geo_operation!(ConvertToNurbs, "geometry", |k, i| k.convert_to_nurbs(&read_geometry(i, "geometry")?));
// #endregion 🔖️Utilities

// #region 🔖️IO
struct ExportStep;
impl Operator for ExportStep {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let value = kernel.export_step(&[geometry]).map_err(map_kernel_error)?;
            Ok(channel_output("step", text_dictionary(value)))
        })
    }
}

struct ExportStl;
impl Operator for ExportStl {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let data = kernel.export_stl(&[geometry], deflection).map_err(map_kernel_error)?;
            Ok(channel_output("stl", text_dictionary(encode_base64(&data))))
        })
    }
}

struct ExportObj;
impl Operator for ExportObj {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let value = kernel.export_obj(&[geometry], deflection).map_err(map_kernel_error)?;
            Ok(channel_output("obj", text_dictionary(value)))
        })
    }
}

struct ImportStep;
impl Operator for ImportStep {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let data = read_text(input, "data")?;
            let shapes = kernel.import_step(&data).map_err(map_kernel_error)?;
            let handle = shapes.into_iter().next().ok_or_else(|| EvalError::InvalidInput("step import produced no solids".into()))?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ImportStl;
impl Operator for ImportStl {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let data = decode_base64(&read_text(input, "data")?)?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.import_stl(&data, tolerance).map_err(map_kernel_error)?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ImportObj;
impl Operator for ImportObj {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let data = read_text(input, "data")?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.import_obj(&data, tolerance).map_err(map_kernel_error)?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}

struct ExportDwg;
impl Operator for ExportDwg {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let data = kernel.export_dwg(&[geometry], deflection).map_err(map_kernel_error)?;
            Ok(channel_output("dwg", text_dictionary(encode_base64(&data))))
        })
    }
}

struct ImportDwg;
impl Operator for ImportDwg {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let data = decode_base64(&read_text(input, "data")?)?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = kernel.import_dwg(&data, tolerance).map_err(map_kernel_error)?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️IO

/// 📦️ Registers brep geometry schema and operators.
pub async fn register(registry: &mut Registry) {
    registry.register_schema(geometry_schema());
    registry.register_schema(topology_element_schema("vertex", "Vertex", "emoji:📍️"));
    registry.register_schema(topology_element_schema("edge", "Edge", "emoji:〰"));
    registry.register_schema(topology_element_schema("face", "Face", "emoji:⬜️"));
    registry.register_schema(brep_schema());
    registry.register_schema(text_schema());
    registry.register_operator(
        OperatorInfo {
            id: "brep.brep".into(),
            extension: "brep".into(),
            name: "Brep".into(),
            abbreviation: "Brep".into(),
            icon: "emoji:🧊️".into(),
            summary: q("deconstruct", "Deconstructs B-Rep geometry into vertices, edges, and faces"),
            inputs: vec![geometry_channel("brep", "brep.brep")],
            outputs: vec![
                ChannelSpec::named("B", "Brep", "brep", "BrepGeometry").with_operators(vec!["brep.brep".into()]),
                topology_output("V", "Vtx", "vertex", "vertex"),
                topology_output("E", "Edg", "edge", "edge"),
                topology_output("F", "Fce", "face", "face"),
                ChannelSpec::list_output("errors", vec![]),
            ],
            group: vec!["Schemas".into()],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec!["geometry".into()], operator: Box::new(BrepDeconstruct) }],
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
        Box::new(BoxPrim),
    );
    reg_geo(registry, "brep.prim3d.sphere", "Sphere", "Sphere", "emoji:⚪️", &q("sphere_prim", "Sphere solid"), vec![number_channel("radius", "brep.prim3d.sphere", 1.0)], out_solid("SphereSolid"), &["Primitives 3D"], Box::new(SpherePrim));
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
        Box::new(CylinderPrim),
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
        Box::new(ConePrim),
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
        Box::new(TorusPrim),
    );
    reg_geo(registry, "brep.prim3d.convexHull", "Convex Hull", "Hull", "emoji:📦️", &q("convex_hull", "Convex hull from points"), vec![list_channel("points", "brep.prim3d.convexHull")], out_solid("ConvexHullSolid"), &["Primitives 3D"], Box::new(ConvexHullPrim));

    reg_geo(registry, "brep.curve.line", "Line", "Line", "emoji:📏️", &q("line_curve", "Line curve"), vec![point_channel("start", "brep.curve.line"), point_channel("end", "brep.curve.line")], out_curve("LineCurve"), &["Curves"], Box::new(LineCurve));
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
        Box::new(CircleCurve),
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
        Box::new(ArcCurve),
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
        Box::new(EllipseCurve),
    );
    reg_geo(registry, "brep.curve.polyline", "Polyline", "Poly", "emoji:📏️", &q("polyline_wire", "Polyline wire"), vec![list_channel("points", "brep.curve.polyline")], out_wire("PolylineWire"), &["Curves"], Box::new(PolylineWire));
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
        Box::new(RectangleWire),
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
        Box::new(RegularPolygonWire),
    );
    reg_geo(
        registry,
        "brep.curve.interpolate",
        "Interpolate",
        "Intp",
        "emoji:〰",
        &q("interpolate_curve", "Interpolated curve"),
        vec![list_channel("points", "brep.curve.interpolate"), number_channel("degree", "brep.curve.interpolate", 3.0)],
        out_curve("InterpolatedCurve"),
        &["Curves"],
        Box::new(InterpolateCurve),
    );
    reg_geo(
        registry,
        "brep.curve.approximate",
        "Approximate",
        "Appr",
        "emoji:〰",
        &q("approximate_curve", "Approximated curve"),
        vec![list_channel("points", "brep.curve.approximate"), number_channel("degree", "brep.curve.approximate", 3.0), number_channel("controlPoints", "brep.curve.approximate", 4.0)],
        out_curve("ApproximatedCurve"),
        &["Curves"],
        Box::new(ApproximateCurve),
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
        Box::new(HelixCurve),
    );

    reg_geo(registry, "brep.surf.plane", "Plane", "Plane", "emoji:⬜️", &q("plane_surface", "Plane surface"), vec![point_channel("origin", "brep.surf.plane"), point_channel("normal", "brep.surf.plane")], out_surface("PlaneSurface"), &["Surfaces"], Box::new(PlaneSurface));
    reg_geo(registry, "brep.surf.planarFace", "Planar Face", "PFace", "emoji:⬜️", &q("planar_face_from_points", "Planar face from points"), vec![list_channel("points", "brep.surf.planarFace")], out_face("PlanarFace"), &["Surfaces"], Box::new(PlanarFacePoints));
    reg_geo(registry, "brep.surf.planarFaceWire", "Planar Face Wire", "PFW", "emoji:⬜️", &q("planar_face_from_wire", "Planar face from wire"), vec![geometry_channel("wire", "brep.surf.planarFaceWire")], out_face("PlanarFaceWire"), &["Surfaces"], Box::new(PlanarFaceWire));
    reg_geo(
        registry,
        "brep.surf.nurbsGrid",
        "Nurbs Grid",
        "Grid",
        "emoji:🧮️",
        &q("nurbs_surface_from_grid", "Nurbs surface from point grid"),
        vec![list_channel("points", "brep.surf.nurbsGrid"), number_channel("rows", "brep.surf.nurbsGrid", 2.0), number_channel("degreeU", "brep.surf.nurbsGrid", 3.0), number_channel("degreeV", "brep.surf.nurbsGrid", 3.0)],
        out_surface("NurbsSurface"),
        &["Surfaces"],
        Box::new(NurbsGridSurface),
    );
    reg_geo(registry, "brep.surf.coons", "Coons Patch", "Coons", "emoji:🧩️", &q("coons_patch", "Coons patch from boundary curves"), vec![list_channel("curves", "brep.surf.coons")], out_surface("CoonsPatch"), &["Surfaces"], Box::new(CoonsPatch));
    reg_geo(
        registry,
        "brep.surf.offset",
        "Offset Face",
        "Offset",
        "emoji:↔",
        &q("offset_face", "Offset face"),
        vec![geometry_channel("face", "brep.surf.offset"), number_channel("distance", "brep.surf.offset", 0.1)],
        out_face("OffsetFace"),
        &["Surfaces"],
        Box::new(OffsetFace),
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
        Box::new(ThickenFace),
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
        Box::new(ExtrudeCurve),
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
        Box::new(ExtrudeFace),
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
        Box::new(Revolve),
    );
    reg_geo(registry, "brep.sweep.loft", "Loft", "Loft", "emoji:🌉️", &q("loft", "Loft profiles"), vec![list_channel("profiles", "brep.sweep.loft"), number_channel("smooth", "brep.sweep.loft", 0.0)], out_solid("LoftedSolid"), &["Sweeps"], Box::new(Loft));
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
        Box::new(Sweep),
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
        Box::new(Pipe),
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
        Box::new(HelicalSweep),
    );

    reg_geo(registry, "brep.bool.fuse", "Fuse", "Fuse", "emoji:🔗️", &q("fuse", "Boolean union"), vec![geometry_channel("a", "brep.bool.fuse"), geometry_channel("b", "brep.bool.fuse")], out_solid("FusedSolid"), &["Booleans"], Box::new(Fuse));
    reg_geo(registry, "brep.bool.cut", "Cut", "Cut", "emoji:🔗️", &q("cut", "Boolean difference"), vec![geometry_channel("a", "brep.bool.cut"), geometry_channel("b", "brep.bool.cut")], out_solid("CutSolid"), &["Booleans"], Box::new(Cut));
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
        Box::new(Intersect),
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
        Box::new(CompoundCut),
    );

    reg_geo(
        registry,
        "brep.xform.translate",
        "Translate",
        "Trans",
        "emoji:🔁️",
        &q("translate", "Translate geometry"),
        vec![geometry_channel("geometry", "brep.xform.translate"), ChannelSpec::requires("offset", &["math.move"])],
        out_geometry("TranslatedGeometry"),
        &["Transforms"],
        Box::new(Translate),
    );
    reg_geo(
        registry,
        "brep.xform.rotate",
        "Rotate",
        "Rot",
        "emoji:🔁️",
        &q("rotate", "Rotate geometry"),
        vec![geometry_channel("geometry", "brep.xform.rotate"), number_channel("angle", "brep.xform.rotate", std::f64::consts::FRAC_PI_4), ChannelSpec::requires("axis", &["brep.xform.rotate"])],
        out_geometry("RotatedGeometry"),
        &["Transforms"],
        Box::new(Rotate),
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
            ChannelSpec::requires("axis", &["brep.xform.rotateAbout"]),
            number_channel("angle", "brep.xform.rotateAbout", std::f64::consts::FRAC_PI_4),
        ],
        out_geometry("RotatedGeometry"),
        &["Transforms"],
        Box::new(RotateAbout),
    );
    reg_geo(
        registry,
        "brep.xform.scale",
        "Scale",
        "Scale",
        "emoji:🔁️",
        &q("scale", "Scale geometry"),
        vec![geometry_channel("geometry", "brep.xform.scale"), number_channel("factor", "brep.xform.scale", 2.0), ChannelSpec::requires("center", &["brep.xform.scale"])],
        out_geometry("ScaledGeometry"),
        &["Transforms"],
        Box::new(Scale),
    );
    reg_geo(
        registry,
        "brep.xform.mirror",
        "Mirror",
        "Mir",
        "emoji:🔁️",
        &q("mirror", "Mirror geometry"),
        vec![geometry_channel("geometry", "brep.xform.mirror"), ChannelSpec::requires("origin", &["brep.xform.mirror"]), ChannelSpec::requires("normal", &["brep.xform.mirror"])],
        out_geometry("MirroredGeometry"),
        &["Transforms"],
        Box::new(Mirror),
    );
    reg_geo(registry, "brep.xform.copy", "Copy", "Copy", "emoji:📋️", &q("copy_shape", "Copy geometry"), vec![geometry_channel("geometry", "brep.xform.copy")], out_geometry("CopiedGeometry"), &["Transforms"], Box::new(CopyShape));
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
        Box::new(LinearPattern),
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
        Box::new(CircularPattern),
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
        Box::new(GridPattern),
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
        Box::new(Fillet),
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
        Box::new(FilletVariable),
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
        Box::new(Chamfer),
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
        Box::new(ChamferAsymmetric),
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
        Box::new(FilletEdges),
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
        Box::new(ChamferEdges),
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
        Box::new(ShellMutation),
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
        Box::new(Draft),
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
        Box::new(OffsetSolid),
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
        Box::new(Defeature),
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
        Box::new(Section),
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
            vec![ChannelSpec::named("P", "Pos", "positive", "PositiveSolid"), ChannelSpec::named("N", "Neg", "negative", "NegativeSolid")],
            &["Intersect"],
        ),
        Box::new(Split),
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
        Box::new(CurveCurveIntersect),
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
        Box::new(CurveSurfaceIntersect),
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
        Box::new(SurfaceSurfaceIntersect),
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
        Box::new(CurvePoint),
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
            vec![ChannelSpec::named("T", "Tan", "tangent", "CurveTangent")],
            &["Evaluate"],
        ),
        Box::new(CurveTangent),
        &["vector"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.eval.curveDomain", "Curve Domain", "Cdm", "emoji:📏️", &q("curve_domain", "Curve domain span"), vec![geometry_channel("curve", "brep.eval.curveDomain")], vec![out_span()], &["Evaluate"]),
        Box::new(CurveDomain),
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
        Box::new(CurveCurvature),
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
        Box::new(SurfacePoint),
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
        Box::new(SurfaceNormal),
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
            vec![ChannelSpec::named("T", "Prm", "parameter", "ClosestParameter"), out_point("ClosestPoint"), ChannelSpec::named("D", "Dst", "distance", "AchievedDistance")],
            &["Evaluate"],
        ),
        Box::new(CurveClosestParameter),
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
            vec![
                ChannelSpec::named("U", "U", "u", "ClosestU"),
                ChannelSpec::named("V", "V", "v", "ClosestV"),
                out_point("ClosestPoint"),
                ChannelSpec::named("D", "Dst", "distance", "AchievedDistance"),
            ],
            &["Evaluate"],
        ),
        Box::new(SurfaceClosestUv),
        &["number", "point"],
    );

    register_typed(registry, operator_info_with_outputs("brep.measure.volume", "Volume", "Vol", "emoji:📐️", &q("volume", "Solid volume"), vec![geometry_channel("geometry", "brep.measure.volume")], vec![out_volume()], &["Measure"]), Box::new(Volume), &["number"]);
    register_typed(registry, operator_info_with_outputs("brep.measure.area", "Area", "Area", "emoji:📐️", &q("area", "Surface area"), vec![geometry_channel("geometry", "brep.measure.area")], vec![out_area()], &["Measure"]), Box::new(Area), &["number"]);
    register_typed(registry, operator_info_with_outputs("brep.measure.length", "Length", "Len", "emoji:📐️", &q("length", "Curve length"), vec![geometry_channel("geometry", "brep.measure.length")], vec![out_length()], &["Measure"]), Box::new(Length), &["number"]);
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.centerOfMass", "Center Of Mass", "CoM", "emoji:📐️", &q("center_of_mass", "Center of mass"), vec![geometry_channel("geometry", "brep.measure.centerOfMass")], vec![out_center()], &["Measure"]),
        Box::new(CenterOfMass),
        &["point"],
    );
    reg_geo(registry, "brep.measure.boundingBox", "Bounding Box", "BBox", "emoji:📐️", &q("bounding_box", "Axis-aligned bounding box"), vec![geometry_channel("geometry", "brep.measure.boundingBox")], out_box(), &["Measure"], Box::new(BoundingBox));
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.distance", "Distance", "Dist", "emoji:📐️", &q("distance", "Minimum distance"), vec![geometry_channel("a", "brep.measure.distance"), geometry_channel("b", "brep.measure.distance")], vec![out_distance()], &["Measure"]),
        Box::new(Distance),
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
            vec![out_point("ClosestPoint")],
            &["Measure"],
        ),
        Box::new(ClosestPoint),
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
        Box::new(ClassifyPoint),
        &["number"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.validate", "Validate", "Val", "emoji:📐️", &q("validate", "Validate geometry"), vec![geometry_channel("geometry", "brep.measure.validate")], vec![out_report()], &["Measure"]),
        Box::new(Validate),
        &["text"],
    );

    reg_geo(registry, "brep.util.vertex", "Vertex", "Vtx", "emoji:📍️", &q("vertex", "Create vertex"), vec![point_channel("point", "brep.util.vertex")], out_vertex(), &["Utilities"], Box::new(Vertex));
    reg_geo(registry, "brep.util.faceFromWire", "Face From Wire", "FFW", "emoji:⬜️", &q("face_from_wire", "Face from closed wire"), vec![geometry_channel("wire", "brep.util.faceFromWire")], out_face("FaceFromWire"), &["Utilities"], Box::new(FaceFromWire));
    reg_geo(registry, "brep.util.sew", "Sew", "Sew", "emoji:🧵️", &q("sew_faces", "Sew faces"), vec![list_channel("faces", "brep.util.sew"), number_channel("tolerance", "brep.util.sew", 0.001)], out_solid("SewnSolid"), &["Utilities"], Box::new(SewFaces));
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
        Box::new(HealSolid),
    );
    reg_geo(registry, "brep.util.convertToNurbs", "Convert To Nurbs", "Nrb", "emoji:〰", &q("convert_to_nurbs", "Convert to NURBS"), vec![geometry_channel("geometry", "brep.util.convertToNurbs")], out_geometry("NurbsGeometry"), &["Utilities"], Box::new(ConvertToNurbs));

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
        Box::new(SolidShells),
        &["geometry", "list"],
    );
    reg_geo(registry, "brep.topology.compound", "Compound", "Cmpd", "emoji:🗃️", &q("compound", "Combine solids into a compound"), vec![list_channel("solids", "brep.topology.compound")], out_compound("Compound"), &["Topology"], Box::new(CompoundOf));
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
        Box::new(Explode),
        &["geometry", "list"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.topology.label", "Label", "Lbl", "emoji:🏷️", &q("label", "Handle's persistent label"), vec![geometry_channel("geometry", "brep.topology.label")], vec![ChannelSpec::named("L", "Lbl", "label", "PersistentLabel")], &["Topology"]),
        Box::new(GeometryLabel),
        &["number"],
    );

    register_typed(registry, operator_info_with_outputs("brep.io.exportStep", "Export Step", "Stp", "emoji:💾️", &q("export_step", "Export STEP"), vec![geometry_channel("geometry", "brep.io.exportStep")], vec![out_step()], &["IO"]), Box::new(ExportStep), &["text"]);
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
        Box::new(ExportStl),
        &["text"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.io.exportObj", "Export Obj", "Obj", "emoji:💾️", &q("export_obj", "Export OBJ"), vec![geometry_channel("geometry", "brep.io.exportObj"), number_channel("deflection", "brep.io.exportObj", 0.1)], vec![out_obj()], &["IO"]),
        Box::new(ExportObj),
        &["text"],
    );
    reg_geo(registry, "brep.io.importStep", "Import Step", "IStp", "emoji:📂️", &q("import_step", "Import STEP"), vec![ChannelSpec::requires("data", &["brep.io.importStep"])], out_geometry("ImportedGeometry"), &["IO"], Box::new(ImportStep));
    reg_geo(
        registry,
        "brep.io.importStl",
        "Import Stl",
        "IStl",
        "emoji:📂️",
        &q("import_stl", "Import STL from base64"),
        vec![ChannelSpec::requires("data", &["brep.io.importStl"]), number_channel("tolerance", "brep.io.importStl", 0.1)],
        out_geometry("ImportedGeometry"),
        &["IO"],
        Box::new(ImportStl),
    );
    reg_geo(
        registry,
        "brep.io.importObj",
        "Import Obj",
        "IObj",
        "emoji:📂️",
        &q("import_obj", "Import OBJ"),
        vec![ChannelSpec::requires("data", &["brep.io.importObj"]), number_channel("tolerance", "brep.io.importObj", 0.1)],
        out_geometry("ImportedGeometry"),
        &["IO"],
        Box::new(ImportObj),
    );
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.io.exportDwg",
            "Export Dwg",
            "Dwg",
            "emoji:💾️",
            &q("export_dwg", "Export DWG as base64"),
            vec![geometry_channel("geometry", "brep.io.exportDwg"), number_channel("deflection", "brep.io.exportDwg", 0.1)],
            vec![out_dwg()],
            &["IO"],
        ),
        Box::new(ExportDwg),
        &["text"],
    );
    reg_geo(
        registry,
        "brep.io.importDwg",
        "Import Dwg",
        "IDwg",
        "emoji:📂️",
        &q("import_dwg", "Import DWG from base64"),
        vec![ChannelSpec::requires("data", &["brep.io.importDwg"]), number_channel("tolerance", "brep.io.importDwg", 0.1)],
        out_geometry("ImportedGeometry"),
        &["IO"],
        Box::new(ImportDwg),
    );

    registry.finalize();
}

/// 🛂️ Manifest JSON for host contribution install (tests + packaging metadata).
pub async fn extension_manifest_json() -> String {
    build_manifest_json("brep", "Brep", "0.3.0", &neural_engine::ColdOwner::new(module_registry().await), vec!["onStartup".into()], vec![], vec![], vec![])
}

pub async fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry).await;
    registry
}

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::{Atom, Value};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::Brep;
    use std::sync::{Mutex, OnceLock};

    async fn point(x: f64, y: f64, z: f64) -> Dictionary {
        Dictionary::with_schema("point").insert("x", Value::Atom(Atom::Decimal(x))).insert("y", Value::Atom(Atom::Decimal(y))).insert("z", Value::Atom(Atom::Decimal(z)))
    }

    async fn vector(x: f64, y: f64, z: f64) -> Dictionary {
        Dictionary::with_schema("vector").insert("x", Value::Atom(Atom::Decimal(x))).insert("y", Value::Atom(Atom::Decimal(y))).insert("z", Value::Atom(Atom::Decimal(z)))
    }

    /// 🔒️ Serialises the tests that share the process-wide brep kernel. Recovers from a poisoned
    /// lock so that one failing test reports its own assertion instead of cascading `PoisonError`s
    /// through every sibling.
    async fn test_serial() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        let lock = LOCK.get_or_init(|| Mutex::new(()));
        lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// 🔗️ Brep handles are content-addressed: `Brep::mint` hashes the geometry with blake3 and uses
    /// the hex digest verbatim, so a handle carries no kind prefix — `kind` is the separate field.
    async fn is_geometry_handle(geometry: &Dictionary) -> bool {
        let Some(handle) = geometry.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) else {
            return false;
        };
        handle.len() == 64 && handle.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    }

    async fn channel_payload(out: &Dictionary, channel: &str) -> Dictionary {
        out.get(channel).and_then(|v| v.as_dictionary()).cloned().expect("channel payload")
    }

    async fn reset_test_kernel() {
        if let Ok(mut guard) = kernel().write() {
            *guard = Box::new(Brep::new());
        }
        if let Ok(mut cache) = mesh_cache().lock() {
            cache.clear();
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn box_emits_geometry_handle() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0)));
        let out = reg.dispatch("brep.prim3d.box", &input).unwrap();
        let solid = channel_payload(&out, "solid");
        assert_eq!(solid.schema(), Some("geometry"));
        assert!(is_geometry_handle(&solid));
        assert_eq!(solid.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("solid"));
    }

    #[semio_framework_async_macros::async_test]
    async fn line_curve_emits_curve_handle() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let out = reg.dispatch("brep.curve.line", &Dictionary::new().insert("start", Value::Dictionary(point(0.0, 0.0, 0.0))).insert("end", Value::Dictionary(point(1.0, 0.0, 0.0)))).unwrap();
        let curve = channel_payload(&out, "curve");
        assert_eq!(curve.schema(), Some("geometry"));
        assert!(is_geometry_handle(&curve));
        assert_eq!(curve.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("curve"));
    }

    #[semio_framework_async_macros::async_test]
    async fn dwg_export_import_round_trips_a_box() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let solid = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0))))
                .unwrap(),
            "solid",
        );
        let dwg = channel_payload(&reg.dispatch("brep.io.exportDwg", &Dictionary::new().insert("geometry", Value::Dictionary(solid)).insert("deflection", Value::Dictionary(number_dictionary(0.1)))).unwrap(), "dwg");
        let data = dwg.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("dwg base64").to_string();
        assert!(!data.is_empty());

        let imported = channel_payload(&reg.dispatch("brep.io.importDwg", &Dictionary::new().insert("data", Value::Dictionary(text_dictionary(data))).insert("tolerance", Value::Dictionary(number_dictionary(0.1)))).unwrap(), "geometry");
        assert_eq!(imported.schema(), Some("geometry"));
        assert!(imported.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).is_some());
    }

    async fn box_handle(reg: &mut Registry) -> String {
        let solid = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0))))
                .unwrap(),
            "solid",
        );
        solid.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).expect("box handle").to_string()
    }

    #[semio_framework_async_macros::async_test]
    async fn step_export_import_round_trips_a_box() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported = pack::json::parse(&export_solid_json(&[handle], "step", 0.1)).unwrap();
        assert!(exported.get("error").is_none(), "{exported:?}");
        assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(false));
        let data = exported.get("data").and_then(|value| value.as_str()).expect("step text").to_string();
        assert!(!data.is_empty());
        let imported = pack::json::parse(&import_solid_json("step", &data, 0.1)).unwrap();
        assert!(imported.get("error").is_none(), "{imported:?}");
        assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn obj_export_import_round_trips_a_box() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported = pack::json::parse(&export_solid_json(&[handle], "obj", 0.1)).unwrap();
        assert!(exported.get("error").is_none(), "{exported:?}");
        let data = exported.get("data").and_then(|value| value.as_str()).expect("obj text").to_string();
        assert!(data.contains('v'));
        let imported = pack::json::parse(&import_solid_json("obj", &data, 0.1)).unwrap();
        assert!(imported.get("error").is_none(), "{imported:?}");
        assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn stl_export_import_round_trips_a_box() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported = pack::json::parse(&export_solid_json(&[handle], "stl", 0.1)).unwrap();
        assert!(exported.get("error").is_none(), "{exported:?}");
        assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(true));
        let data = exported.get("data").and_then(|value| value.as_str()).expect("stl base64").to_string();
        assert!(!data.is_empty());
        let imported = pack::json::parse(&import_solid_json("stl", &data, 0.1)).unwrap();
        assert!(imported.get("error").is_none(), "{imported:?}");
        assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn glb_export_import_round_trips_a_box_through_the_mesh_bridge() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported = pack::json::parse(&export_solid_json(&[handle], "glb", 0.1)).unwrap();
        assert!(exported.get("error").is_none(), "{exported:?}");
        assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(true));
        let data = exported.get("data").and_then(|value| value.as_str()).expect("glb base64").to_string();
        assert!(!data.is_empty());
        let imported = pack::json::parse(&import_solid_json("glb", &data, 0.1)).unwrap();
        assert!(imported.get("error").is_none(), "{imported:?}");
        assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn export_solid_json_rejects_unsupported_format() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported = pack::json::parse(&export_solid_json(&[handle], "fbx", 0.1)).unwrap();
        assert!(exported.get("error").is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn extrude_and_area() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let wire = channel_payload(&reg.dispatch("brep.curve.rectangle", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(), "wire");
        let face = channel_payload(&reg.dispatch("brep.surf.planarFaceWire", &Dictionary::new().insert("wire", Value::Dictionary(wire))).unwrap(), "face");
        let solid = channel_payload(&reg.dispatch("brep.sweep.extrude", &Dictionary::new().insert("face", Value::Dictionary(face)).insert("vector", Value::Dictionary(vector(0.0, 0.0, 3.0)))).unwrap(), "solid");
        assert_eq!(solid.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("solid"));
        let area = channel_payload(&reg.dispatch("brep.measure.area", &Dictionary::new().insert("geometry", Value::Dictionary(solid))).unwrap(), "area");
        assert_eq!(area.schema(), Some("number"));
        let value = area.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
        assert!(value > 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn extrude_curve_wire_uses_vector_magnitude() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let wire = channel_payload(&reg.dispatch("brep.curve.rectangle", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0)))).unwrap(), "wire");
        let solid = channel_payload(&reg.dispatch("brep.solid.extrude", &Dictionary::new().insert("wire", Value::Dictionary(wire)).insert("vector", Value::Dictionary(vector(0.0, 0.0, 4.0)))).unwrap(), "solid");
        assert_eq!(solid.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("solid"));
        let volume = channel_payload(&reg.dispatch("brep.measure.volume", &Dictionary::new().insert("geometry", Value::Dictionary(solid))).unwrap(), "volume");
        let value = volume.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
        assert!((value - 16.0).abs() < 1e-3);
    }

    #[semio_framework_async_macros::async_test]
    async fn fillet_translate_chain() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let box_out = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0))))
                .unwrap(),
            "solid",
        );
        let fillet_out = channel_payload(&reg.dispatch("brep.solid.fillet", &Dictionary::new().insert("geometry", Value::Dictionary(box_out)).insert("radius", Value::Dictionary(number_dictionary(0.1)))).unwrap(), "solid");
        let moved = channel_payload(&reg.dispatch("brep.xform.translate", &Dictionary::new().insert("geometry", Value::Dictionary(fillet_out)).insert("offset", Value::Dictionary(vector(1.0, 0.0, 0.0)))).unwrap(), "geometry");
        assert_eq!(moved.schema(), Some("geometry"));
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_lists_brep_operators() {
        let _serial = test_serial();
        reset_test_kernel();
        let json = build_manifest_json("brep", "Brep", "0.3.0", &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![], vec![]);
        assert!(json.contains("brep.prim3d.box"));
        assert!(json.contains("brep.curve.line"));
        assert!(json.contains("brep.solid.extrude"));
        assert!(json.contains("brep.sweep.extrude"));
        assert!(json.contains("brep.measure.area"));
        assert!(json.contains("\"operators\""));
        assert!(json.contains("brep.xform.translate"));
        assert!(json.contains("brep.geometry"));
        assert!(json.contains("brep.brep"));
        assert!(json.contains("\"Schemas\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn evaluate_json_box() {
        let _serial = test_serial();
        reset_test_kernel();
        let reg = module_registry();
        let json_number = |value: f64| pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(value))]);
        let input_json = pack::json::to_string(&pack::json::object([("width".to_string(), json_number(1.0)), ("depth".to_string(), json_number(1.0)), ("height".to_string(), json_number(1.0))]));
        let out_json = evaluate_json(&reg, "brep.prim3d.box", &input_json);
        let out = pack::json::parse(&out_json).unwrap();
        assert_eq!(out.get("solid").and_then(|value| value.get("$schema")).and_then(pack::json::Value::as_str), Some("geometry"));
    }

    #[semio_framework_async_macros::async_test]
    async fn retain_geometry_handles_sweeps_orphaned_shapes() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let box_out = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0))))
                .unwrap(),
            "solid",
        );
        let orphan = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(2.0))).insert("height", Value::Dictionary(number_dictionary(2.0))))
                .unwrap(),
            "solid",
        );
        let live_handle = box_out.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap().to_string();
        let orphan_handle = orphan.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap().to_string();
        retain_geometry_handles(std::slice::from_ref(&live_handle));
        let live_mesh = tessellate_geometry(&live_handle, 0.1).expect("live tessellation");
        assert!(!live_mesh.positions.is_empty());
        assert!(tessellate_geometry(&orphan_handle, 0.1).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn tessellate_geometry_is_memoized() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let box_out = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0))))
                .unwrap(),
            "solid",
        );
        let handle = box_out.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap();
        let first = tessellate_geometry(handle, 0.1).expect("mesh");
        let second = tessellate_geometry(handle, 0.1).expect("mesh");
        assert_eq!(first.positions, second.positions);
        assert!(!first.positions.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn brep_component_deconstructs_solid_topology() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let solid = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0))))
                .unwrap(),
            "solid",
        );
        let deconstructed = reg.dispatch("brep.brep", &Dictionary::new().insert("brep", Value::Dictionary(solid))).unwrap();
        let vertices = deconstructed.get("vertex").and_then(Value::as_dictionary).expect("vertex list");
        let edges = deconstructed.get("edge").and_then(Value::as_dictionary).expect("edge list");
        let faces = deconstructed.get("face").and_then(Value::as_dictionary).expect("face list");
        assert_eq!(list_indices(vertices).len(), 8);
        assert_eq!(list_indices(edges).len(), 12);
        assert_eq!(list_indices(faces).len(), 6);
    }

    #[semio_framework_async_macros::async_test]
    async fn schema_component_deconstructs_geometry() {
        let mut reg = Registry::new();
        register(&mut reg);
        let geometry = Dictionary::with_schema("geometry").insert("handle", Value::Atom(Atom::String("solid-1".into()))).insert("kind", Value::Atom(Atom::String("solid".into())));
        let out = reg.dispatch("brep.geometry", &Dictionary::new().insert("geometry", Value::Dictionary(geometry.clone()))).unwrap();
        assert_eq!(out.get("handle").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid-1"));
        assert_eq!(out.get("kind").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid"));
    }

    #[semio_framework_async_macros::async_test]
    async fn extension_bundle_extends_flow_and_evaluates_box() {
        use semio_framework_plugin::{extension_activate, extension_invoke, extension_manifest, install_extension_bundle, ExtensionBundle};

        let _serial = test_serial();
        reset_test_kernel();
        let manifest_json = extension_manifest_json();
        let flow_topic = flow_extension_sdk::flow_extension_topic_contribution("flow-play", "brep", "Brep", "brep", &manifest_json);
        let procedural3d_topic = flow_extension_sdk::flow_extension_topic_contribution("procedural3d-play", "brep", "Brep", "brep", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-brep", "Brep", "0.3.0")
            .extends("flow")
            .contributes_topic(flow_topic.topic, flow_topic.payload)
            .contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload)
            .handler("evaluate", |req| Ok(flow_extension_sdk::evaluate_invoke_json(&neural_engine::ColdOwner::new(module_registry()), req).unwrap()));
        install_extension_bundle(bundle);
        extension_activate().unwrap();
        assert_eq!(extension_manifest().extension_id, "flow-extension-brep");
        let json_number = |value: f64| pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(value))]);
        let input_json = pack::json::to_string(&pack::json::object([("width".to_string(), json_number(1.0)), ("depth".to_string(), json_number(1.0)), ("height".to_string(), json_number(1.0))]));
        let req = pack::json::to_string(&pack::json::object([("operatorId".to_string(), pack::json::Value::from("brep.prim3d.box")), ("inputJson".to_string(), pack::json::Value::from(input_json)), ("nodeHash".to_string(), pack::json::Value::from(1_i64))]));
        let out = pack::json::parse_bytes(&extension_invoke("evaluate", req.as_bytes()).unwrap()).unwrap();
        assert_eq!(out.get("solid").and_then(|value| value.get("$schema")).and_then(pack::json::Value::as_str), Some("geometry"));
    }

    fn number_value(dict: &Dictionary) -> f64 {
        dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).expect("number channel value")
    }

    async fn box_of(reg: &mut Registry, size: f64) -> Dictionary {
        channel_payload(&reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(size))).insert("depth", Value::Dictionary(number_dictionary(size))).insert("height", Value::Dictionary(number_dictionary(size)))).unwrap(), "solid")
    }

    /// 🏄️ Surfaces family: every `(u, v)` on a plane must stay in-plane regardless of the
    /// surface's own (arbitrary) in-plane basis — a basis-independent invariant to check against.
    #[semio_framework_async_macros::async_test]
    async fn surface_family_plane_point_stays_in_plane_and_normal_matches() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let surface = channel_payload(&reg.dispatch("brep.surf.plane", &Dictionary::new().insert("origin", Value::Dictionary(point(0.0, 0.0, 0.0))).insert("normal", Value::Dictionary(vector(0.0, 0.0, 1.0)))).unwrap(), "surface");
        let evaluated = channel_payload(&reg.dispatch("brep.eval.surfPoint", &Dictionary::new().insert("surface", Value::Dictionary(surface.clone())).insert("u", Value::Dictionary(number_dictionary(1.0))).insert("v", Value::Dictionary(number_dictionary(-2.0)))).unwrap(), "point");
        let z = evaluated.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
        assert!(z.abs() < 1e-9, "a point on the z=0 plane must have z=0 regardless of (u, v), got z={z}");
        let normal = channel_payload(&reg.dispatch("brep.eval.surfNormal", &Dictionary::new().insert("surface", Value::Dictionary(surface)).insert("u", Value::Dictionary(number_dictionary(0.0))).insert("v", Value::Dictionary(number_dictionary(0.0)))).unwrap(), "normal");
        let normal_z = normal.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
        assert!(normal_z.abs() > 0.999, "plane normal must be parallel to the world Z axis, got z={normal_z}");
    }

    /// 🔗️ Booleans family: overlapping unit-ish boxes, checked by plausible volume bounds only —
    /// `fuse`/`cut`/`intersect` are `MeshDerivedBRep` today (not exact), so exact numerics would
    /// be over-claiming precision the kernel does not yet provide.
    #[semio_framework_async_macros::async_test]
    async fn boolean_family_fuse_cut_intersect_report_plausible_volumes() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let a = box_of(&mut reg, 2.0);
        let b_raw = box_of(&mut reg, 2.0);
        let b = channel_payload(&reg.dispatch("brep.xform.translate", &Dictionary::new().insert("geometry", Value::Dictionary(b_raw)).insert("offset", Value::Dictionary(vector(1.0, 0.0, 0.0)))).unwrap(), "geometry");

        let fused = channel_payload(&reg.dispatch("brep.bool.fuse", &Dictionary::new().insert("a", Value::Dictionary(a.clone())).insert("b", Value::Dictionary(b.clone()))).unwrap(), "solid");
        let fused_volume = number_value(&channel_payload(&reg.dispatch("brep.measure.volume", &Dictionary::new().insert("geometry", Value::Dictionary(fused))).unwrap(), "volume"));
        assert!((8.0..16.0).contains(&fused_volume), "fused volume {fused_volume} should exceed either box's own 8.0 but stay below the disjoint sum 16.0");

        let cut = channel_payload(&reg.dispatch("brep.bool.cut", &Dictionary::new().insert("a", Value::Dictionary(a.clone())).insert("b", Value::Dictionary(b.clone()))).unwrap(), "solid");
        let cut_volume = number_value(&channel_payload(&reg.dispatch("brep.measure.volume", &Dictionary::new().insert("geometry", Value::Dictionary(cut))).unwrap(), "volume"));
        assert!((0.0..8.0).contains(&cut_volume), "cut volume {cut_volume} should be less than the untouched box's 8.0");

        let intersected = channel_payload(&reg.dispatch("brep.bool.intersect", &Dictionary::new().insert("a", Value::Dictionary(a)).insert("b", Value::Dictionary(b))).unwrap(), "solid");
        let intersect_volume = number_value(&channel_payload(&reg.dispatch("brep.measure.volume", &Dictionary::new().insert("geometry", Value::Dictionary(intersected))).unwrap(), "volume"));
        assert!((0.0..8.0).contains(&intersect_volume), "intersection volume {intersect_volume} should be less than either box's own 8.0");
    }

    /// 🔁️ Transforms family, `rotate_about` specifically — distinguishes it from `rotate` (world
    /// origin only): rotating 180° about an explicit off-origin point must move the geometry far
    /// from where a world-origin rotation would leave it (audit §6.2's bounding-box-center bug).
    #[semio_framework_async_macros::async_test]
    async fn rotate_about_rotates_around_the_given_origin_not_the_world_origin() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let solid = box_of(&mut reg, 1.0);
        let rotated = channel_payload(
            &reg.dispatch(
                "brep.xform.rotateAbout",
                &Dictionary::new().insert("geometry", Value::Dictionary(solid)).insert("origin", Value::Dictionary(point(5.0, 0.0, 0.0))).insert("axis", Value::Dictionary(vector(0.0, 0.0, 1.0))).insert("angle", Value::Dictionary(number_dictionary(std::f64::consts::PI))),
            )
            .unwrap(),
            "geometry",
        );
        let center = channel_payload(&reg.dispatch("brep.measure.centerOfMass", &Dictionary::new().insert("geometry", Value::Dictionary(rotated))).unwrap(), "center");
        let x = center.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap();
        assert!((x - 9.5).abs() < 1e-6, "180° about origin (5,0,0) should move the unit box's center from x=0.5 to x=9.5, got x={x}");
    }

    /// 🎯️ Evaluation family, closest-parameter/UV specifically — both are exact closed forms for
    /// a line and a plane, so the achieved distance is checked exactly, not just bounded.
    #[semio_framework_async_macros::async_test]
    async fn evaluation_family_closest_parameter_and_closest_uv_report_certified_distance() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let curve = channel_payload(&reg.dispatch("brep.curve.line", &Dictionary::new().insert("start", Value::Dictionary(point(0.0, 0.0, 0.0))).insert("end", Value::Dictionary(point(10.0, 0.0, 0.0)))).unwrap(), "curve");
        let out = reg.dispatch("brep.eval.curveClosestParameter", &Dictionary::new().insert("curve", Value::Dictionary(curve)).insert("point", Value::Dictionary(point(4.0, 3.0, 0.0)))).unwrap();
        let distance = number_value(&channel_payload(&out, "distance"));
        assert!((distance - 3.0).abs() < 1e-9, "closest distance from (4,3,0) to the segment along the x axis should be 3, got {distance}");
        let closest = channel_payload(&out, "point");
        assert!((closest.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap() - 4.0).abs() < 1e-9);

        let surface = channel_payload(&reg.dispatch("brep.surf.plane", &Dictionary::new().insert("origin", Value::Dictionary(point(0.0, 0.0, 0.0))).insert("normal", Value::Dictionary(vector(0.0, 0.0, 1.0)))).unwrap(), "surface");
        let uv_out = reg.dispatch("brep.eval.surfaceClosestUv", &Dictionary::new().insert("surface", Value::Dictionary(surface)).insert("point", Value::Dictionary(point(1.0, 1.0, 5.0)))).unwrap();
        let uv_distance = number_value(&channel_payload(&uv_out, "distance"));
        assert!((uv_distance - 5.0).abs() < 1e-6, "closest distance from (1,1,5) to the z=0 plane should be 5, got {uv_distance}");
    }

    /// 🐚️ Topology family — the new wave-1 handle capabilities: shells, compound/explode, and
    /// the persistent label round trip.
    #[semio_framework_async_macros::async_test]
    async fn topology_family_shells_compound_explode_and_label() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let box_a = box_of(&mut reg, 1.0);
        let box_b = box_of(&mut reg, 2.0);
        let handle_a = box_a.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("box a handle").to_string();
        let handle_b = box_b.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("box b handle").to_string();

        let shells_out = reg.dispatch("brep.topology.shells", &Dictionary::new().insert("solid", Value::Dictionary(box_a.clone()))).unwrap();
        let shells = shells_out.get("shells").and_then(Value::as_dictionary).expect("shells list");
        assert_eq!(list_indices(shells).len(), 1, "a simple box has exactly one outer shell");

        let solids_in = topology_list("geometry", vec![GeometryHandle(handle_a), GeometryHandle(handle_b)]);
        let compound_out = reg.dispatch("brep.topology.compound", &Dictionary::new().insert("solids", Value::Dictionary(solids_in))).unwrap();
        let compound = channel_payload(&compound_out, "compound");
        assert_eq!(compound.get("kind").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("compound"));

        let exploded_out = reg.dispatch("brep.topology.explode", &Dictionary::new().insert("compound", Value::Dictionary(compound))).unwrap();
        let solids_out = exploded_out.get("solids").and_then(Value::as_dictionary).expect("solids list");
        assert_eq!(list_indices(solids_out).len(), 2, "exploding must recover both original solids");

        let label_out = reg.dispatch("brep.topology.label", &Dictionary::new().insert("geometry", Value::Dictionary(box_a))).unwrap();
        let label = number_value(&channel_payload(&label_out, "label"));
        assert!(label >= 0.0);
    }

    /// 🎯️ `q`'s tag round-trips through the live registry — the contract's `operation_quality`
    /// stays the single source of truth: nothing here hardcodes an `OpQuality` a second time.
    #[semio_framework_async_macros::async_test]
    async fn operation_quality_tags_match_the_kernel_contract() {
        let reg = module_registry();
        for (id, method) in NODE_KERNEL_METHOD.iter().copied() {
            let info = reg.operator_info(id).unwrap_or_else(|| panic!("node {id:?} is registered in NODE_KERNEL_METHOD but not in the live Registry"));
            let expected = format!("[quality:{:?}]", operation_quality(method));
            assert!(info.summary.contains(expected.as_str()), "node {id:?}'s summary {:?} does not carry {expected:?} for its wrapped method {method:?}", info.summary);
        }
    }

    /// 📇️ Every `BrepKernel` trait method is either wrapped by exactly one node, or explicitly
    /// listed as unexposed — nothing falls through both lists, and nothing in either list names a
    /// method the trait does not actually have.
    #[semio_framework_async_macros::async_test]
    async fn every_kernel_operation_is_either_a_node_or_explicitly_unexposed() {
        let mut seen = std::collections::HashSet::new();
        for (id, method) in NODE_KERNEL_METHOD {
            assert!(seen.insert(*method), "BrepKernel method {method:?} (node {id:?}) is wrapped by more than one flow node");
        }
        for (method, _reason) in INTENTIONALLY_UNEXPOSED {
            assert!(seen.insert(*method), "{method:?} is listed in both NODE_KERNEL_METHOD and INTENTIONALLY_UNEXPOSED");
        }
        let known: std::collections::HashSet<&str> = BREP_KERNEL_OPERATIONS.iter().copied().collect();
        for method in &seen {
            assert!(known.contains(method), "{method:?} in NODE_KERNEL_METHOD/INTENTIONALLY_UNEXPOSED is not a real BrepKernel method");
        }
        for operation in BREP_KERNEL_OPERATIONS {
            assert!(seen.contains(operation), "BrepKernel method {operation:?} has neither a flow node nor an INTENTIONALLY_UNEXPOSED entry");
        }
        assert_eq!(seen.len(), BREP_KERNEL_OPERATIONS.len());
    }
}

// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::module_registry;
    use flow_extension_sdk::{evaluate_invoke_json, flow_extension_topic_contribution};
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle};

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "brep";
    const EXTENSION_LABEL: &str = "Brep";

    fn bundle() -> ExtensionBundle {
        let manifest_json = semio_framework::io::resolve_ready(super::extension_manifest_json());
        let flow_topic = flow_extension_topic_contribution(FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "brep", &manifest_json);
        let procedural3d_topic = flow_extension_topic_contribution(PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "brep", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-brep", "Brep", "0.3.0").extends("flow");
        let bundle = semio_framework::io::resolve_ready(bundle.mode(ExecutionMode::Linked));
        let bundle = semio_framework::io::resolve_ready(bundle.contributes_topic(flow_topic.topic, flow_topic.payload));
        let bundle = semio_framework::io::resolve_ready(bundle.contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload));
        let bundle = semio_framework::io::resolve_ready(bundle.handler("evaluate", |req| {
                evaluate_invoke_json(&neural_engine::ColdOwner::new(semio_framework::io::resolve_ready(module_registry())), req).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err))
            }));
        semio_framework::io::resolve_ready(bundle.handler("tessellate", |req| {
                let request = pack::json::parse_bytes(req).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.tessellate.bad-request"), err.to_string()))?;
                let handle = request.get("handle").and_then(pack::json::Value::as_str).ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.tessellate.bad-request"), "missing field `handle`".to_string()))?;
                let tolerance = request.get("tolerance").and_then(pack::json::Value::as_f64).unwrap_or(0.05);
                Ok(flow_extension_sdk::brep_geometry::tessellate_geometry_json_for_wasm(handle, tolerance).into_bytes())
            }))
    }

    #[test]
    fn bundle_identity_matches_catalogue_fixture() {
        let fixture = pack::json::parse(include_str!("../🧪️fixtures/🔣️.json")).unwrap();
        let bundle = bundle();
        assert_eq!(Some(bundle.manifest.extension_id.as_str()), fixture.get("brep").and_then(|entry| entry.get("pluginId")).and_then(pack::json::Value::as_str));
        assert_eq!(bundle.manifest.topic_contributions.len(), 2);
        for contribution in &bundle.manifest.topic_contributions {
            assert_eq!(contribution.payload.get("extensionId").and_then(|value| value.as_str()), fixture.get("brep").and_then(|entry| entry.get("flowId")).and_then(pack::json::Value::as_str));
        }
    }

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
