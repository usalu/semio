//! 🔷️ Flow brep extension — geometry operators packaged as a runtime-installable unit.

use flow_extension_sdk::brep_geometry::*;
use flow_extension_sdk::build_manifest_json;
use neural_engine::{channel_output, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::block_on;

macro_rules! geo_operation {
    ($name:ident, $channel:literal, |$k:ident, $i:ident| $expr:expr) => {
        struct $name;
        impl Operator for $name {
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                with_kernel(|$k| {
                    let $i = input;
                    let handle = block_on($expr).map_err(map_kernel_error)?;
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
                    let value = block_on($expr).map_err(map_kernel_error)?;
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
                    let value = block_on($expr).map_err(map_kernel_error)?;
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
                    let value = block_on($expr).map_err(map_kernel_error)?;
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
                    let value = block_on($expr).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.convex_hull(&points)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.polyline_wire(&points)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.interpolate_curve(&points, degree)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.approximate_curve(&points, degree, control_points)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.planar_face_from_points(&points)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.nurbs_surface_from_grid(&grid, degree_u, degree_v)).map_err(map_kernel_error)?;
            Ok(channel_output("surface", geometry_dict(kernel, &handle)?))
        })
    }
}

struct CoonsPatch;
impl Operator for CoonsPatch {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let curves = read_nested_point_lists(input, "curves")?;
            let handle = block_on(kernel.coons_patch(&curves)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.extrude_wire(&wire, vector)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.extrude(&face, direction, distance)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.loft(&profiles, smooth)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.pipe(&profile, &path, guide)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.compound_cut(&target, &tools)).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️Booleans

// #region 🔖️Transforms
geo_operation!(Translate, "geometry", |k, i| k.translate(&read_geometry(i, "geometry")?, read_xyz(i, "offset")?));
geo_operation!(Rotate, "geometry", |k, i| k.rotate(&read_geometry(i, "geometry")?, read_xyz(i, "axis")?, read_channel_number(i, "angle")?));
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
            let handle = block_on(kernel.fillet_edges(&geometry, &edges, radius)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.chamfer_edges(&geometry, &edges, distance)).map_err(map_kernel_error)?;
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
            let open_faces = read_geometry_list(input, "openFaces").unwrap_or_default();
            let handle = block_on(kernel.shell(&geometry, thickness, &open_faces)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.draft(&geometry, &faces, read_xyz(input, "pullDirection")?, read_xyz(input, "neutralPoint")?, read_channel_number(input, "angle")?)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.defeature(&geometry, &faces)).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️Features

// #region 🔖️Intersect
struct Section;
impl Operator for Section {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let faces = block_on(kernel.section(&read_geometry(input, "solid")?, read_xyz(input, "planeOrigin")?, read_xyz(input, "planeNormal")?)).map_err(map_kernel_error)?;
            let handle = faces.into_iter().next().ok_or_else(|| EvalError::InvalidInput("section produced no faces".into()))?;
            Ok(channel_output("face", geometry_dict(kernel, &handle)?))
        })
    }
}

struct Split;
impl Operator for Split {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let (positive, _negative) = block_on(kernel.split(&read_geometry(input, "solid")?, read_xyz(input, "planeOrigin")?, read_xyz(input, "planeNormal")?)).map_err(map_kernel_error)?;
            Ok(channel_output("solid", geometry_dict(kernel, &positive)?))
        })
    }
}

struct CurveCurveIntersect;
impl Operator for CurveCurveIntersect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = block_on(kernel.curve_curve_intersect(&read_geometry(input, "a")?, &read_geometry(input, "b")?, read_channel_number(input, "tolerance")?)).map_err(map_kernel_error)?;
            let handle = wire_from_points(kernel, &points)?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

struct CurveSurfaceIntersect;
impl Operator for CurveSurfaceIntersect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let points = block_on(kernel.curve_surface_intersect(&read_geometry(input, "curve")?, &read_geometry(input, "surface")?, read_channel_number(input, "tolerance")?)).map_err(map_kernel_error)?;
            let handle = wire_from_points(kernel, &points)?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
        })
    }
}

struct SurfaceSurfaceIntersect;
impl Operator for SurfaceSurfaceIntersect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let wires = block_on(kernel.surface_surface_intersect(&read_geometry(input, "a")?, &read_geometry(input, "b")?, read_channel_number(input, "tolerance")?)).map_err(map_kernel_error)?;
            let handle = wires.into_iter().next().ok_or_else(|| EvalError::InvalidInput("no intersection wire".into()))?;
            Ok(channel_output("wire", geometry_dict(kernel, &handle)?))
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
            let domain = block_on(kernel.curve_domain(&read_geometry(input, "curve")?)).map_err(map_kernel_error)?;
            Ok(channel_output("span", number_dictionary(domain_span(domain))))
        })
    }
}

num_operation!(CurveCurvature, "curvature", |k, i| k.curve_curvature(&read_geometry(i, "curve")?, read_channel_number(i, "parameter")?));
point_operation!(SurfacePoint, "point", |k, i| k.surface_point(&read_geometry(i, "surface")?, read_channel_number(i, "u")?, read_channel_number(i, "v")?));
vec_operation!(SurfaceNormal, "normal", |k, i| k.surface_normal(&read_geometry(i, "surface")?, read_channel_number(i, "u")?, read_channel_number(i, "v")?));
// #endregion 🔖️Evaluate

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
            let result = block_on(kernel.closest_point(&read_geometry(input, "geometry")?, read_xyz(input, "point")?)).map_err(map_kernel_error)?;
            Ok(channel_output("point", point_dictionary(result.point)))
        })
    }
}

struct ClassifyPoint;
impl Operator for ClassifyPoint {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel_read(|kernel| {
            let classification = block_on(kernel.classify_point(&read_geometry(input, "solid")?, read_xyz(input, "point")?)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.sew_faces(&faces, tolerance)).map_err(map_kernel_error)?;
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
            let value = block_on(kernel.export_step(&[geometry])).map_err(map_kernel_error)?;
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
            let data = block_on(kernel.export_stl(&[geometry], deflection)).map_err(map_kernel_error)?;
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
            let value = block_on(kernel.export_obj(&[geometry], deflection)).map_err(map_kernel_error)?;
            Ok(channel_output("obj", text_dictionary(value)))
        })
    }
}

struct ImportStep;
impl Operator for ImportStep {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let data = read_text(input, "data")?;
            let shapes = block_on(kernel.import_step(&data)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.import_stl(&data, tolerance)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.import_obj(&data, tolerance)).map_err(map_kernel_error)?;
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
            let data = block_on(kernel.export_dwg(&[geometry], deflection)).map_err(map_kernel_error)?;
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
            let handle = block_on(kernel.import_dwg(&data, tolerance)).map_err(map_kernel_error)?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️IO

/// 📦️ Registers brep geometry schema and operators.
pub fn register(registry: &mut Registry) {
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
            summary: "Deconstructs B-Rep geometry into vertices, edges, and faces".into(),
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
        "Axis-aligned box solid",
        vec![number_channel("width", "brep.prim3d.box", 1.0), number_channel("depth", "brep.prim3d.box", 1.0), number_channel("height", "brep.prim3d.box", 1.0)],
        out_solid("BoxSolid"),
        &["Primitives 3D"],
        Box::new(BoxPrim),
    );
    reg_geo(registry, "brep.prim3d.sphere", "Sphere", "Sphere", "emoji:⚪️", "Sphere solid", vec![number_channel("radius", "brep.prim3d.sphere", 1.0)], out_solid("SphereSolid"), &["Primitives 3D"], Box::new(SpherePrim));
    reg_geo(
        registry,
        "brep.prim3d.cylinder",
        "Cylinder",
        "Cylinder",
        "emoji:🛢️",
        "Cylinder solid",
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
        "Cone solid",
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
        "Torus solid",
        vec![number_channel("major", "brep.prim3d.torus", 2.0), number_channel("minor", "brep.prim3d.torus", 0.5)],
        out_solid("TorusSolid"),
        &["Primitives 3D"],
        Box::new(TorusPrim),
    );
    reg_geo(registry, "brep.prim3d.convexHull", "Convex Hull", "Hull", "emoji:📦️", "Convex hull from points", vec![list_channel("points", "brep.prim3d.convexHull")], out_solid("ConvexHullSolid"), &["Primitives 3D"], Box::new(ConvexHullPrim));

    reg_geo(registry, "brep.curve.line", "Line", "Line", "emoji:📏️", "Line curve", vec![point_channel("start", "brep.curve.line"), point_channel("end", "brep.curve.line")], out_curve("LineCurve"), &["Curves"], Box::new(LineCurve));
    reg_geo(
        registry,
        "brep.curve.circle",
        "Circle",
        "Circle",
        "emoji:⭕️",
        "Circle curve",
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
        "Arc curve",
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
        "Ellipse curve",
        vec![point_channel("center", "brep.curve.ellipse"), point_channel("normal", "brep.curve.ellipse"), number_channel("semiMajor", "brep.curve.ellipse", 2.0), number_channel("semiMinor", "brep.curve.ellipse", 1.0)],
        out_curve("EllipseCurve"),
        &["Curves"],
        Box::new(EllipseCurve),
    );
    reg_geo(registry, "brep.curve.polyline", "Polyline", "Poly", "emoji:📏️", "Polyline wire", vec![list_channel("points", "brep.curve.polyline")], out_wire("PolylineWire"), &["Curves"], Box::new(PolylineWire));
    reg_geo(
        registry,
        "brep.curve.rectangle",
        "Rectangle",
        "Rect",
        "emoji:⬜️",
        "Rectangle wire",
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
        "Regular polygon wire",
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
        "Interpolated curve",
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
        "Approximated curve",
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
        "Helix curve",
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

    reg_geo(registry, "brep.surf.plane", "Plane", "Plane", "emoji:⬜️", "Plane surface", vec![point_channel("origin", "brep.surf.plane"), point_channel("normal", "brep.surf.plane")], out_surface("PlaneSurface"), &["Surfaces"], Box::new(PlaneSurface));
    reg_geo(registry, "brep.surf.planarFace", "Planar Face", "PFace", "emoji:⬜️", "Planar face from points", vec![list_channel("points", "brep.surf.planarFace")], out_face("PlanarFace"), &["Surfaces"], Box::new(PlanarFacePoints));
    reg_geo(registry, "brep.surf.planarFaceWire", "Planar Face Wire", "PFW", "emoji:⬜️", "Planar face from wire", vec![geometry_channel("wire", "brep.surf.planarFaceWire")], out_face("PlanarFaceWire"), &["Surfaces"], Box::new(PlanarFaceWire));
    reg_geo(
        registry,
        "brep.surf.nurbsGrid",
        "Nurbs Grid",
        "Grid",
        "emoji:🧮️",
        "Nurbs surface from point grid",
        vec![list_channel("points", "brep.surf.nurbsGrid"), number_channel("rows", "brep.surf.nurbsGrid", 2.0), number_channel("degreeU", "brep.surf.nurbsGrid", 3.0), number_channel("degreeV", "brep.surf.nurbsGrid", 3.0)],
        out_surface("NurbsSurface"),
        &["Surfaces"],
        Box::new(NurbsGridSurface),
    );
    reg_geo(registry, "brep.surf.coons", "Coons Patch", "Coons", "emoji:🧩️", "Coons patch from boundary curves", vec![list_channel("curves", "brep.surf.coons")], out_surface("CoonsPatch"), &["Surfaces"], Box::new(CoonsPatch));
    reg_geo(
        registry,
        "brep.surf.offset",
        "Offset Face",
        "Offset",
        "emoji:↔",
        "Offset face",
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
        "Thicken face to solid",
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
        "Extrude closed wire along vector magnitude",
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
        "Extrude face along vector magnitude",
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
        "Revolve face",
        vec![geometry_channel("face", "brep.sweep.revolve"), point_channel("axisOrigin", "brep.sweep.revolve"), point_channel("axisDirection", "brep.sweep.revolve"), number_channel("angle", "brep.sweep.revolve", std::f64::consts::TAU)],
        out_solid("RevolvedSolid"),
        &["Sweeps"],
        Box::new(Revolve),
    );
    reg_geo(registry, "brep.sweep.loft", "Loft", "Loft", "emoji:🌉️", "Loft profiles", vec![list_channel("profiles", "brep.sweep.loft"), number_channel("smooth", "brep.sweep.loft", 0.0)], out_solid("LoftedSolid"), &["Sweeps"], Box::new(Loft));
    reg_geo(
        registry,
        "brep.sweep.sweep",
        "Sweep",
        "Sweep",
        "emoji:🛤️",
        "Sweep profile along path",
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
        "Pipe profile along path",
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
        "Helical sweep",
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

    reg_geo(registry, "brep.bool.fuse", "Fuse", "Fuse", "emoji:🔗️", "Boolean union", vec![geometry_channel("a", "brep.bool.fuse"), geometry_channel("b", "brep.bool.fuse")], out_solid("FusedSolid"), &["Booleans"], Box::new(Fuse));
    reg_geo(registry, "brep.bool.cut", "Cut", "Cut", "emoji:🔗️", "Boolean difference", vec![geometry_channel("a", "brep.bool.cut"), geometry_channel("b", "brep.bool.cut")], out_solid("CutSolid"), &["Booleans"], Box::new(Cut));
    reg_geo(
        registry,
        "brep.bool.intersect",
        "Intersect",
        "Int",
        "emoji:🔗️",
        "Boolean intersection",
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
        "Compound boolean cut",
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
        "Translate geometry",
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
        "Rotate geometry",
        vec![geometry_channel("geometry", "brep.xform.rotate"), number_channel("angle", "brep.xform.rotate", std::f64::consts::FRAC_PI_4), ChannelSpec::requires("axis", &["brep.xform.rotate"])],
        out_geometry("RotatedGeometry"),
        &["Transforms"],
        Box::new(Rotate),
    );
    reg_geo(
        registry,
        "brep.xform.scale",
        "Scale",
        "Scale",
        "emoji:🔁️",
        "Scale geometry",
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
        "Mirror geometry",
        vec![geometry_channel("geometry", "brep.xform.mirror"), ChannelSpec::requires("origin", &["brep.xform.mirror"]), ChannelSpec::requires("normal", &["brep.xform.mirror"])],
        out_geometry("MirroredGeometry"),
        &["Transforms"],
        Box::new(Mirror),
    );
    reg_geo(registry, "brep.xform.copy", "Copy", "Copy", "emoji:📋️", "Copy geometry", vec![geometry_channel("geometry", "brep.xform.copy")], out_geometry("CopiedGeometry"), &["Transforms"], Box::new(CopyShape));
    reg_geo(
        registry,
        "brep.xform.linearPattern",
        "Linear Pattern",
        "LinP",
        "emoji:📐️",
        "Linear pattern",
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
        "Circular pattern",
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
        "Grid pattern",
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
        "Fillet all solid edges",
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
        "Variable fillet",
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
        "Chamfer all solid edges",
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
        "Asymmetric chamfer",
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
        "Fillet only the given edges",
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
        "Chamfer only the given edges",
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
        "Shell solid",
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
        "Draft faces",
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
        "Offset solid",
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
        "Remove faces",
        vec![geometry_channel("geometry", "brep.solid.defeature"), list_channel("faces", "brep.solid.defeature")],
        out_solid("DefeaturedSolid"),
        &["Features"],
        Box::new(Defeature),
    );

    reg_geo(
        registry,
        "brep.intersect.section",
        "Section",
        "Sect",
        "emoji:✂️",
        "Section solid with plane",
        vec![geometry_channel("solid", "brep.intersect.section"), point_channel("planeOrigin", "brep.intersect.section"), point_channel("planeNormal", "brep.intersect.section")],
        out_face("SectionFace"),
        &["Intersect"],
        Box::new(Section),
    );
    reg_geo(
        registry,
        "brep.intersect.split",
        "Split",
        "Split",
        "emoji:✂️",
        "Split solid with plane",
        vec![geometry_channel("solid", "brep.intersect.split"), point_channel("planeOrigin", "brep.intersect.split"), point_channel("planeNormal", "brep.intersect.split")],
        out_solid("SplitSolid"),
        &["Intersect"],
        Box::new(Split),
    );
    reg_geo(
        registry,
        "brep.intersect.curveCurve",
        "Curve Curve",
        "CC",
        "emoji:✂️",
        "Curve-curve intersection",
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
        "Curve-surface intersection",
        vec![geometry_channel("curve", "brep.intersect.curveSurface"), geometry_channel("surface", "brep.intersect.curveSurface"), number_channel("tolerance", "brep.intersect.curveSurface", 0.001)],
        out_wire("CurveSurfaceIntersection"),
        &["Intersect"],
        Box::new(CurveSurfaceIntersect),
    );
    reg_geo(
        registry,
        "brep.intersect.surfaceSurface",
        "Surface Surface",
        "SS",
        "emoji:✂️",
        "Surface-surface intersection",
        vec![geometry_channel("a", "brep.intersect.surfaceSurface"), geometry_channel("b", "brep.intersect.surfaceSurface"), number_channel("tolerance", "brep.intersect.surfaceSurface", 0.001)],
        out_wire("SurfaceSurfaceIntersection"),
        &["Intersect"],
        Box::new(SurfaceSurfaceIntersect),
    );

    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.eval.curvePoint",
            "Curve Point",
            "Cpt",
            "emoji:📍️",
            "Evaluate curve point",
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
            "Evaluate curve tangent",
            vec![geometry_channel("curve", "brep.eval.curveTangent"), number_channel("parameter", "brep.eval.curveTangent", 0.0)],
            vec![ChannelSpec::named("T", "Tan", "tangent", "CurveTangent")],
            &["Evaluate"],
        ),
        Box::new(CurveTangent),
        &["vector"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.eval.curveDomain", "Curve Domain", "Cdm", "emoji:📏️", "Curve domain span", vec![geometry_channel("curve", "brep.eval.curveDomain")], vec![out_span()], &["Evaluate"]),
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
            "Curve curvature",
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
            "Evaluate surface point",
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
            "Evaluate surface normal",
            vec![geometry_channel("surface", "brep.eval.surfNormal"), number_channel("u", "brep.eval.surfNormal", 0.0), number_channel("v", "brep.eval.surfNormal", 0.0)],
            vec![out_normal("SurfaceNormal")],
            &["Evaluate"],
        ),
        Box::new(SurfaceNormal),
        &["vector"],
    );

    register_typed(registry, operator_info_with_outputs("brep.measure.volume", "Volume", "Vol", "emoji:📐️", "Solid volume", vec![geometry_channel("geometry", "brep.measure.volume")], vec![out_volume()], &["Measure"]), Box::new(Volume), &["number"]);
    register_typed(registry, operator_info_with_outputs("brep.measure.area", "Area", "Area", "emoji:📐️", "Surface area", vec![geometry_channel("geometry", "brep.measure.area")], vec![out_area()], &["Measure"]), Box::new(Area), &["number"]);
    register_typed(registry, operator_info_with_outputs("brep.measure.length", "Length", "Len", "emoji:📐️", "Curve length", vec![geometry_channel("geometry", "brep.measure.length")], vec![out_length()], &["Measure"]), Box::new(Length), &["number"]);
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.centerOfMass", "Center Of Mass", "CoM", "emoji:📐️", "Center of mass", vec![geometry_channel("geometry", "brep.measure.centerOfMass")], vec![out_center()], &["Measure"]),
        Box::new(CenterOfMass),
        &["point"],
    );
    reg_geo(registry, "brep.measure.boundingBox", "Bounding Box", "BBox", "emoji:📐️", "Axis-aligned bounding box", vec![geometry_channel("geometry", "brep.measure.boundingBox")], out_box(), &["Measure"], Box::new(BoundingBox));
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.distance", "Distance", "Dist", "emoji:📐️", "Minimum distance", vec![geometry_channel("a", "brep.measure.distance"), geometry_channel("b", "brep.measure.distance")], vec![out_distance()], &["Measure"]),
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
            "Closest point on geometry",
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
            "Classify point relative to solid",
            vec![geometry_channel("solid", "brep.measure.classify"), point_channel("point", "brep.measure.classify")],
            vec![out_classification()],
            &["Measure"],
        ),
        Box::new(ClassifyPoint),
        &["number"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.measure.validate", "Validate", "Val", "emoji:📐️", "Validate geometry", vec![geometry_channel("geometry", "brep.measure.validate")], vec![out_report()], &["Measure"]),
        Box::new(Validate),
        &["text"],
    );

    reg_geo(registry, "brep.util.vertex", "Vertex", "Vtx", "emoji:📍️", "Create vertex", vec![point_channel("point", "brep.util.vertex")], out_vertex(), &["Utilities"], Box::new(Vertex));
    reg_geo(registry, "brep.util.faceFromWire", "Face From Wire", "FFW", "emoji:⬜️", "Face from closed wire", vec![geometry_channel("wire", "brep.util.faceFromWire")], out_face("FaceFromWire"), &["Utilities"], Box::new(FaceFromWire));
    reg_geo(registry, "brep.util.sew", "Sew", "Sew", "emoji:🧵️", "Sew faces", vec![list_channel("faces", "brep.util.sew"), number_channel("tolerance", "brep.util.sew", 0.001)], out_solid("SewnSolid"), &["Utilities"], Box::new(SewFaces));
    reg_geo(
        registry,
        "brep.util.heal",
        "Heal",
        "Heal",
        "emoji:🩹️",
        "Heal solid",
        vec![geometry_channel("geometry", "brep.util.heal"), number_channel("tolerance", "brep.util.heal", 0.001)],
        out_solid("HealedSolid"),
        &["Utilities"],
        Box::new(HealSolid),
    );
    reg_geo(registry, "brep.util.convertToNurbs", "Convert To Nurbs", "Nrb", "emoji:〰", "Convert to NURBS", vec![geometry_channel("geometry", "brep.util.convertToNurbs")], out_geometry("NurbsGeometry"), &["Utilities"], Box::new(ConvertToNurbs));

    register_typed(registry, operator_info_with_outputs("brep.io.exportStep", "Export Step", "Stp", "emoji:💾️", "Export STEP", vec![geometry_channel("geometry", "brep.io.exportStep")], vec![out_step()], &["IO"]), Box::new(ExportStep), &["text"]);
    register_typed(
        registry,
        operator_info_with_outputs(
            "brep.io.exportStl",
            "Export Stl",
            "Stl",
            "emoji:💾️",
            "Export STL as base64",
            vec![geometry_channel("geometry", "brep.io.exportStl"), number_channel("deflection", "brep.io.exportStl", 0.1)],
            vec![out_stl()],
            &["IO"],
        ),
        Box::new(ExportStl),
        &["text"],
    );
    register_typed(
        registry,
        operator_info_with_outputs("brep.io.exportObj", "Export Obj", "Obj", "emoji:💾️", "Export OBJ", vec![geometry_channel("geometry", "brep.io.exportObj"), number_channel("deflection", "brep.io.exportObj", 0.1)], vec![out_obj()], &["IO"]),
        Box::new(ExportObj),
        &["text"],
    );
    reg_geo(registry, "brep.io.importStep", "Import Step", "IStp", "emoji:📂️", "Import STEP", vec![ChannelSpec::requires("data", &["brep.io.importStep"])], out_geometry("ImportedGeometry"), &["IO"], Box::new(ImportStep));
    reg_geo(
        registry,
        "brep.io.importStl",
        "Import Stl",
        "IStl",
        "emoji:📂️",
        "Import STL from base64",
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
        "Import OBJ",
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
            "Export DWG as base64",
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
        "Import DWG from base64",
        vec![ChannelSpec::requires("data", &["brep.io.importDwg"]), number_channel("tolerance", "brep.io.importDwg", 0.1)],
        out_geometry("ImportedGeometry"),
        &["IO"],
        Box::new(ImportDwg),
    );

    registry.finalize();
}


/// 🛂️ Manifest JSON for host contribution install (tests + packaging metadata).
pub fn extension_manifest_json() -> String {
    build_manifest_json("brep", "Brep", "0.3.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])
}

pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::{Atom, Value};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::Brep;
    use std::sync::{Mutex, OnceLock};

    fn point(x: f64, y: f64, z: f64) -> Dictionary {
        Dictionary::with_schema("point").insert("x", Value::Atom(Atom::Decimal(x))).insert("y", Value::Atom(Atom::Decimal(y))).insert("z", Value::Atom(Atom::Decimal(z)))
    }

    fn vector(x: f64, y: f64, z: f64) -> Dictionary {
        Dictionary::with_schema("vector").insert("x", Value::Atom(Atom::Decimal(x))).insert("y", Value::Atom(Atom::Decimal(y))).insert("z", Value::Atom(Atom::Decimal(z)))
    }

    /// 🔒️ Serialises the tests that share the process-wide brep kernel. Recovers from a poisoned
    /// lock so that one failing test reports its own assertion instead of cascading `PoisonError`s
    /// through every sibling.
    fn test_serial() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        let lock = LOCK.get_or_init(|| Mutex::new(()));
        lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// 🔗️ Brep handles are content-addressed: `Brep::mint` hashes the geometry with blake3 and uses
    /// the hex digest verbatim, so a handle carries no kind prefix — `kind` is the separate field.
    fn is_geometry_handle(geometry: &Dictionary) -> bool {
        let Some(handle) = geometry.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) else {
            return false;
        };
        handle.len() == 64 && handle.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    }

    fn channel_payload(out: &Dictionary, channel: &str) -> Dictionary {
        out.get(channel).and_then(|v| v.as_dictionary()).cloned().expect("channel payload")
    }

    fn reset_test_kernel() {
        if let Ok(mut guard) = kernel().write() {
            *guard = Box::new(Brep::new());
        }
        if let Ok(mut cache) = mesh_cache().lock() {
            cache.clear();
        }
    }

    #[test]
    fn box_emits_geometry_handle() {
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

    #[test]
    fn line_curve_emits_curve_handle() {
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

    #[test]
    fn dwg_export_import_round_trips_a_box() {
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

    fn box_handle(reg: &mut Registry) -> String {
        let solid = channel_payload(
            &reg.dispatch("brep.prim3d.box", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(2.0))).insert("depth", Value::Dictionary(number_dictionary(3.0))).insert("height", Value::Dictionary(number_dictionary(4.0))))
                .unwrap(),
            "solid",
        );
        solid.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).expect("box handle").to_string()
    }

    #[test]
    fn step_export_import_round_trips_a_box() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported: serde_json::Value = serde_json::from_str(&export_solid_json(&[handle], "step", 0.1)).unwrap();
        assert!(exported.get("error").is_none(), "{exported:?}");
        assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(false));
        let data = exported.get("data").and_then(|value| value.as_str()).expect("step text").to_string();
        assert!(!data.is_empty());
        let imported: serde_json::Value = serde_json::from_str(&import_solid_json("step", &data, 0.1)).unwrap();
        assert!(imported.get("error").is_none(), "{imported:?}");
        assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
    }

    #[test]
    fn obj_export_import_round_trips_a_box() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported: serde_json::Value = serde_json::from_str(&export_solid_json(&[handle], "obj", 0.1)).unwrap();
        assert!(exported.get("error").is_none(), "{exported:?}");
        let data = exported.get("data").and_then(|value| value.as_str()).expect("obj text").to_string();
        assert!(data.contains('v'));
        let imported: serde_json::Value = serde_json::from_str(&import_solid_json("obj", &data, 0.1)).unwrap();
        assert!(imported.get("error").is_none(), "{imported:?}");
        assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
    }

    #[test]
    fn stl_export_import_round_trips_a_box() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported: serde_json::Value = serde_json::from_str(&export_solid_json(&[handle], "stl", 0.1)).unwrap();
        assert!(exported.get("error").is_none(), "{exported:?}");
        assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(true));
        let data = exported.get("data").and_then(|value| value.as_str()).expect("stl base64").to_string();
        assert!(!data.is_empty());
        let imported: serde_json::Value = serde_json::from_str(&import_solid_json("stl", &data, 0.1)).unwrap();
        assert!(imported.get("error").is_none(), "{imported:?}");
        assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
    }

    #[test]
    fn glb_export_import_round_trips_a_box_through_the_mesh_bridge() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported: serde_json::Value = serde_json::from_str(&export_solid_json(&[handle], "glb", 0.1)).unwrap();
        assert!(exported.get("error").is_none(), "{exported:?}");
        assert_eq!(exported.get("binary").and_then(|value| value.as_bool()), Some(true));
        let data = exported.get("data").and_then(|value| value.as_str()).expect("glb base64").to_string();
        assert!(!data.is_empty());
        let imported: serde_json::Value = serde_json::from_str(&import_solid_json("glb", &data, 0.1)).unwrap();
        assert!(imported.get("error").is_none(), "{imported:?}");
        assert_eq!(imported.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
    }

    #[test]
    fn export_solid_json_rejects_unsupported_format() {
        let _serial = test_serial();
        reset_test_kernel();
        let mut reg = Registry::new();
        register(&mut reg);
        let handle = box_handle(&mut reg);
        let exported: serde_json::Value = serde_json::from_str(&export_solid_json(&[handle], "fbx", 0.1)).unwrap();
        assert!(exported.get("error").is_some());
    }

    #[test]
    fn extrude_and_area() {
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

    #[test]
    fn extrude_curve_wire_uses_vector_magnitude() {
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

    #[test]
    fn fillet_translate_chain() {
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

    #[test]
    fn manifest_lists_brep_operators() {
        let _serial = test_serial();
        reset_test_kernel();
        let json = build_manifest_json("brep", "Brep", "0.3.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
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

    #[test]
    fn evaluate_json_box() {
        let _serial = test_serial();
        reset_test_kernel();
        let reg = module_registry();
        let input = Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)));
        let out_json = evaluate_json(&reg, "brep.prim3d.box", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(channel_payload(&out, "solid").schema(), Some("geometry"));
    }

    #[test]
    fn retain_geometry_handles_sweeps_orphaned_shapes() {
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

    #[test]
    fn tessellate_geometry_is_memoized() {
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

    #[test]
    fn brep_component_deconstructs_solid_topology() {
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

    #[test]
    fn schema_component_deconstructs_geometry() {
        let mut reg = Registry::new();
        register(&mut reg);
        let geometry = Dictionary::with_schema("geometry").insert("handle", Value::Atom(Atom::String("solid-1".into()))).insert("kind", Value::Atom(Atom::String("solid".into())));
        let out = reg.dispatch("brep.geometry", &Dictionary::new().insert("geometry", Value::Dictionary(geometry.clone()))).unwrap();
        assert_eq!(out.get("handle").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid-1"));
        assert_eq!(out.get("kind").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("solid"));
    }

    #[test]
    fn extension_bundle_extends_flow_and_evaluates_box() {
        use flow_extension_sdk::evaluate_json;
        use semio_framework_plugin::{extension_activate, extension_invoke, extension_manifest, install_extension_bundle, ExtensionBundle};

        let _serial = test_serial();
        reset_test_kernel();
        let manifest_json = extension_manifest_json();
        let bundle = ExtensionBundle::new("brep", "Brep", "0.3.0")
            .extends("flow")
            .contributes_topic(
                "flow.extension",
                serde_json::json!({
                    "appId": "flow-play",
                    "extensionId": "brep",
                    "label": "Brep",
                    "iconId": "brep",
                    "manifestJson": &manifest_json,
                }),
            )
            .contributes_topic(
                "flow.extension",
                serde_json::json!({
                    "appId": "procedural3d-play",
                    "extensionId": "brep",
                    "label": "Brep",
                    "iconId": "brep",
                    "manifestJson": &manifest_json,
                }),
            )
            .handler("evaluate", |req| {
                #[derive(serde::Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct EvaluateRequest {
                    operator_id: String,
                    input_json: String,
                }
                let request: EvaluateRequest = serde_json::from_slice(req).unwrap();
                Ok(evaluate_json(&module_registry(), &request.operator_id, &request.input_json).into_bytes())
            });
        install_extension_bundle(bundle);
        extension_activate().unwrap();
        assert_eq!(extension_manifest().extension_id, "brep");
        let input = Dictionary::new()
            .insert("width", Value::Dictionary(number_dictionary(1.0)))
            .insert("depth", Value::Dictionary(number_dictionary(1.0)))
            .insert("height", Value::Dictionary(number_dictionary(1.0)));
        let req = serde_json::json!({
            "operatorId": "brep.prim3d.box",
            "inputJson": serde_json::to_string(&input).unwrap(),
            "nodeHash": 1,
        });
        let out: Dictionary = serde_json::from_slice(&extension_invoke("evaluate", req.to_string().as_bytes()).unwrap()).unwrap();
        assert_eq!(channel_payload(&out, "solid").schema(), Some("geometry"));
    }
}

// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::module_registry;
    use flow_extension_sdk::evaluate_json;
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::ExtensionBundle;
    use serde::Deserialize;

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct EvaluateRequest {
        operator_id: String,
        input_json: String,
    }

    fn flow_extension_contribution(app_id: &str, manifest_json: String) -> serde_json::Value {
        let extension_id = "brep";
        let label = "Brep";
        let icon_id = "brep";
        let topic_payload = serde_json::json!({
            "appId": app_id,
            "extensionId": extension_id,
            "label": label,
            "iconId": icon_id,
            "manifestJson": &manifest_json,
        });
        topic_payload
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct TessellateRequest {
        handle: String,
        #[serde(default = "default_tessellate_tolerance")]
        tolerance: f64,
    }

    fn default_tessellate_tolerance() -> f64 {
        0.05
    }

    fn bundle() -> ExtensionBundle {
        let manifest_json = super::extension_manifest_json();
        let flow_topic_payload = flow_extension_contribution(FLOW_APP_ID, manifest_json.clone());
        let procedural3d_topic_payload = flow_extension_contribution(PROCEDURAL3D_APP_ID, manifest_json);
        ExtensionBundle::new("brep", "Brep", "0.3.0")
            .extends("flow")
            .contributes_topic("flow.extension", flow_topic_payload)
            .contributes_topic("flow.extension", procedural3d_topic_payload)
            .handler("evaluate", |req| {
                let request: EvaluateRequest = serde_json::from_slice(req).map_err(|err| {
                    Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err.to_string())
                })?;
                Ok(evaluate_json(&module_registry(), &request.operator_id, &request.input_json).into_bytes())
            })
            .handler("tessellate", |req| {
                let request: TessellateRequest = serde_json::from_slice(req).map_err(|err| {
                    Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.tessellate.bad-request"), err.to_string())
                })?;
                match flow_extension_sdk::brep_geometry::tessellate_geometry(&request.handle, request.tolerance) {
                    Ok(mesh) => Ok(serde_json::to_vec(&mesh).map_err(|err| {
                        Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.tessellate.encode"), err.to_string())
                    })?),
                    Err(err) => Ok(serde_json::json!({ "error": err }).to_string().into_bytes()),
                }
            })
    }

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
