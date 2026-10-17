//! 📇️ Registration of the mesh schema and every polygon operator in the flow registry.

use semio_s_spatial_kernel_semio_session::*;
use flow_extension_sdk::mesh::*;
use neural_engine::ChannelSpec;
use neural_engine::Dictionary;
use neural_engine::Registry;
use neural_engine::Value;
use neural_engine::FieldSpec;
use neural_engine::Schema;
use semio_framework_value::ValueType;
use super::{MeshOperation, analyze, positive, selection, vector};

/// 🥽️ Declares the representation fidelity of each owned polygon operator.
pub fn operation_quality(operation: &str) -> &'static str {
    match operation { "fromBrep" => "TessellatedMesh", "toBrep" => "MeshDerivedBRep", _ => "PolygonMesh" }
}

pub fn register_mesh(registry: &mut Registry, session: &Session) {
    registry.register_schema(Schema { id: "mesh".into(), module: "brep".into(), name: "Polygon Mesh".into(), icon: "emoji:🥽️".into(), summary: "Indexed polygon vertices and faces".into(), fields: vec![FieldSpec::new("data", ValueType::Text), FieldSpec::new("preview", ValueType::Text)] });
    let definitions: &[(&str, &str, &str, &[(&str, f64)])] = &[
        ("construct", "Construct Mesh", "Mesh Creation", &[]),
        ("box", "Mesh Box", "Mesh Creation", &[("width", 1.0), ("height", 1.0), ("depth", 1.0)]),
        ("plane", "Mesh Plane", "Mesh Creation", &[("width", 1.0), ("depth", 1.0)]),
        ("sphere", "Mesh Sphere", "Mesh Creation", &[("radius", 1.0), ("subdivisions", 2.0)]),
        ("cylinder", "Mesh Cylinder", "Mesh Creation", &[("radius", 1.0), ("height", 1.0), ("segments", 32.0)]),
        ("cone", "Mesh Cone", "Mesh Creation", &[("radius", 1.0), ("height", 1.0), ("segments", 32.0)]),
        ("fromBrep", "Mesh from B-Rep", "Mesh Conversion", &[("deflection", 0.1)]),
        ("toBrep", "Faceted B-Rep from Mesh", "Mesh Conversion", &[("tolerance", 0.001)]),
        ("translate", "Translate Mesh", "Mesh Editing", &[]),
        ("transform", "Transform Mesh", "Mesh Editing", &[]),
        ("rotate", "Rotate Mesh", "Mesh Editing", &[("angle", 0.0)]),
        ("scale", "Scale Mesh", "Mesh Editing", &[]),
        ("moveVertices", "Move Mesh Vertices", "Mesh Editing", &[]),
        ("translateComponents", "Move Mesh Components", "Mesh Editing", &[]),
        ("rotateComponents", "Rotate Mesh Components", "Mesh Editing", &[("angle", 0.0)]),
        ("scaleComponents", "Scale Mesh Components", "Mesh Editing", &[]),
        ("bevel", "Bevel Mesh Edges", "Mesh Editing", &[("amount", 0.1), ("segments", 1.0)]),
        ("dissolveEdges", "Dissolve Mesh Edges", "Mesh Editing", &[]),
        ("dissolveVertices", "Dissolve Mesh Vertices", "Mesh Editing", &[]),
        ("mergeVertices", "Merge Mesh Vertices", "Mesh Editing", &[("tolerance", 0.0001)]),
        ("moveProportional", "Move Mesh Proportionally", "Mesh Editing", &[("radius", 1.0)]),
        ("snapVertices", "Snap Mesh Vertices to Grid", "Mesh Editing", &[("grid", 1.0)]),
        ("mirror", "Mirror Mesh Half", "Mesh Editing", &[("tolerance", 0.0001)]),
        ("decimate", "Simplify Mesh", "Mesh Editing", &[("ratio", 0.5)]),
        ("mergeCoplanar", "Merge Coplanar Mesh Faces", "Mesh Repair", &[]),
        ("loopCut", "Cut Mesh Loops", "Mesh Editing", &[("cuts", 1.0)]),
        ("knifeCut", "Knife Cut Mesh Face", "Mesh Editing", &[("face", 0.0)]),
        ("extrude", "Extrude Mesh Faces", "Mesh Editing", &[("distance", 1.0)]),
        ("inset", "Inset Mesh Faces", "Mesh Editing", &[("amount", 0.1)]),
        ("subdivide", "Subdivide Mesh Faces", "Mesh Editing", &[]),
        ("flip", "Flip Mesh Faces", "Mesh Editing", &[]),
        ("deleteFaces", "Delete Mesh Faces", "Mesh Editing", &[]),
        ("triangulate", "Triangulate Mesh", "Mesh Editing", &[]),
        ("weld", "Weld Mesh Vertices", "Mesh Repair", &[("tolerance", 0.0001)]),
        ("orient", "Orient Mesh Faces", "Mesh Repair", &[]),
        ("fillHoles", "Fill Mesh Holes", "Mesh Repair", &[]),
        ("inspectVertex", "Inspect Mesh Vertex", "Mesh Analysis", &[("index", 0.0)]),
        ("inspectEdge", "Inspect Mesh Edge", "Mesh Analysis", &[("index", 0.0)]),
        ("inspectFace", "Inspect Mesh Face", "Mesh Analysis", &[("index", 0.0)]),
        ("analyze", "Analyze Mesh", "Mesh Analysis", &[]),
        ("exportObj", "Mesh to OBJ", "Mesh Interchange", &[]),
        ("exportJson", "Mesh to JSON", "Mesh Interchange", &[]),
    ];
    for &(operation, name, group, parameters) in definitions {
        let id = format!("brep.mesh.{operation}");
        let mut inputs = Vec::new();
        if !matches!(operation, "construct" | "box" | "plane" | "sphere" | "cylinder" | "cone" | "fromBrep") { inputs.push(ChannelSpec::requires("mesh", &[&id]).with_value_types(&["mesh"])); }
        if operation == "construct" { inputs.push(ChannelSpec::text_default("data", r#"{"vertices":[[0,0,0],[1,0,0],[0,1,0]],"faces":[[0,1,2]]}"#, &[&id])); }
        if operation == "fromBrep" { inputs.push(geometry_channel("geometry", &id)); }
        if matches!(operation, "extrude" | "inset" | "subdivide" | "flip" | "deleteFaces" | "moveVertices" | "loopCut" | "bevel" | "dissolveEdges") {
            inputs.push(ChannelSpec::requires(match operation { "moveVertices" => "vertices", "loopCut" | "bevel" | "dissolveEdges" => "edges", _ => "faces" }, &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("[0]"))));
        }
        if matches!(operation, "moveProportional" | "snapVertices" | "mergeVertices" | "dissolveVertices") {
            inputs.push(ChannelSpec::requires("selection", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary(if operation == "mergeVertices" { "[0,1]" } else { "[0]" }))));
        }
        if operation == "moveProportional" { inputs.push(vector_channel("center", &id, [0.0; 3])); }
        if operation == "mergeVertices" { inputs.push(ChannelSpec::requires("mode", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("center")))); }
        if operation == "mirror" { inputs.push(ChannelSpec::requires("axis", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("x")))); }
        if operation.ends_with("Components") {
            for (key, value) in [("mode", "vertex"), ("selection", "[0]")] { inputs.push(ChannelSpec::requires(key, &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary(value)))); }
            if operation != "translateComponents" {
                inputs.push(ChannelSpec::requires("pivot", &[&id]).with_value_types(&["text"]).with_default(Value::Dictionary(text_dictionary("selection"))));
                inputs.push(ChannelSpec::requires("center", &[&id]).with_value_types(&["point", "vector"]).with_default(Value::Dictionary(point_dictionary([0.0; 3]))));
            }
        }
        if operation == "knifeCut" {
            for (key, point) in [("start", [0.0, -1.0, 0.0]), ("end", [0.0, 1.0, 0.0])] {
                inputs.push(ChannelSpec::requires(key, &[&id]).with_value_types(&["point", "vector"]).with_default(Value::Dictionary(point_dictionary(point))));
            }
        }
        match operation {
            "transform"=>{let identity=[1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,1.0];let list=identity.into_iter().enumerate().fold(Dictionary::with_schema("list"),|list,(index,value)|list.insert(index.to_string(),Value::Dictionary(number_dictionary(value))));let mut channel=ChannelSpec::named("MA","mat","matrix","Matrix").with_operators(vec![id.clone()]).with_value_types(&["list"]).with_item_types(&["number"]).with_default(Value::Dictionary(list));channel.cardinality=neural_engine::Cardinality::Exactly(16);inputs.push(channel);},
            "translate" | "moveVertices" | "translateComponents" | "moveProportional" => inputs.push(vector_channel("offset", &id, [0.0, 0.0, 1.0])),
            "rotate" | "rotateComponents" => inputs.push(vector_channel("axis", &id, [0.0, 0.0, 1.0])),
            "scale" | "scaleComponents" => inputs.push(vector_channel("factor", &id, [1.0, 1.0, 1.0])),
            _ => {}
        }
        inputs.extend(parameters.iter().map(|(key, value)| number_channel(key, &id, *value)));
        let (outputs, produced) = match operation {
            "analyze" => {
                let mut channels: Vec<_> = ["vertices", "faces", "edges", "triangles", "boundaryEdges", "nonManifoldEdges", "inconsistentEdges", "degenerateTriangles", "area", "volume"].iter().map(|&key| ChannelSpec::named(key, key, key, key).with_value_types(&["number"])).collect();
                for channel in &mut channels { if channel.name == "volume" { channel.cardinality = neural_engine::Cardinality::ZeroOrOne; } }
                channels.extend([out_point("Minimum"), out_point("Maximum")].into_iter().zip(["minimum", "maximum"]).map(|(mut channel, name)| { channel.name = name.into(); channel }));
                (channels, vec!["number", "point"])
            }
            "inspectVertex" => (vec![out_point("VertexPosition")], vec!["point"]),
            "inspectEdge" => (vec![ChannelSpec::named("S", "Start", "start", "EdgeStart").with_value_types(&["point"]), ChannelSpec::named("E", "End", "end", "EdgeEnd").with_value_types(&["point"]), out_length()], vec!["point", "number"]),
            "inspectFace" => (vec![ChannelSpec::named("V", "Verts", "vertices", "FaceVertices").with_value_types(&["text"]), out_normal("FaceNormal"), out_center().with_value_types(&["point"])], vec!["text", "vector", "point"]),
            "toBrep" => (vec![out_geometry("FacetedGeometry")], vec!["geometry"]),
            "exportObj" | "exportJson" => (vec![ChannelSpec::named("T", "Text", "text", "MeshText").with_value_types(&["text"])], vec!["text"]),
            _ => (vec![ChannelSpec::named("M", "Mesh", "meshOut", "PolygonMesh").with_value_types(&["mesh"])], vec!["mesh"]),
        };
        let summary = match operation {
            "construct" => "Create a polygon mesh from JSON vertices and zero-based face indices.",
            "box" => "Create a closed box with six quad faces and independent width, height, and depth.",
            "plane" => "Create one open quad in the horizontal plane.",
            "sphere" => "Create a closed triangular sphere; each subdivision increases surface detail.",
            "cylinder" => "Create a closed cylinder with polygon caps and quad sides.",
            "cone" => "Create a closed cone with a polygon base and triangular sides.",
            "fromBrep" => "Tessellate a B-Rep into a triangle mesh; smaller deflection produces more detail.",
            "toBrep" => "Convert mesh polygons to planar B-Rep faces without reconstructing curved surfaces.",
            "translate" => "Move every mesh vertex by the offset vector.",
            "transform" => "Apply a column-major affine matrix with inverse-transpose normals and reflection winding repair.",
            "rotate" => "Rotate the mesh around an axis through the origin; angle is in radians.",
            "scale" => "Scale each axis about the origin; negative factors mirror and preserve outward winding.",
            "moveVertices" => "Move selected zero-based vertex indices by the offset vector.",
            "translateComponents" => "Move selected vertices, edges, or faces; shared vertices move once.",
            "rotateComponents" => "Rotate selected components about their centroid or a chosen point; angle is in radians.",
            "scaleComponents" => "Scale selected components about their centroid or a chosen point, preserving polygon indices.",
            "bevel" => "Round selected edges of a closed convex mesh with 1–64 profile segments; reject widths crossing adjacent vertices.",
            "dissolveEdges" => "Remove selected connecting edges and join their neighboring polygon faces.",
            "dissolveVertices" => "Join a planar connected vertex neighborhood into one polygon without removing its surface.",
            "mergeVertices" => "Merge selected vertices at the first position, their mean, or within the distance tolerance; clean collapsed polygon loops.",
            "moveProportional" => "Move selected vertices fully and surrounding vertices with linear distance falloff from the center within the radius.",
            "snapVertices" => "Snap selected vertices to an origin-aligned grid with positive spacing.",
            "mirror" => "Reflect a one-sided mesh across an origin axis plane; weld only matching seam vertices within the tolerance.",
            "decimate" => "Approximate mesh simplification by shortest-edge collapse with winding and manifold checks; may stop before the target ratio.",
            "mergeCoplanar" => "Join adjacent coplanar faces without changing their surface.",
            "loopCut" => "Cut connected quad strips through selected preview edges; 1–256 cuts share vertices and crossing strips form grids.",
            "knifeCut" => "Split one face along the projected line through two points, sharing new boundary vertices with its neighbors.",
            "extrude" => "Extrude selected faces along their normals and connect the boundary with side faces.",
            "inset" => "Inset each selected face by a positive distance, preserving connected border faces.",
            "subdivide" => "Split selected faces into triangles while preserving their boundary edges.",
            "flip" => "Reverse the winding and normals of selected faces.",
            "deleteFaces" => "Remove selected faces, leaving open boundaries for further editing.",
            "triangulate" => "Triangulate polygon faces, including concave planar polygons.",
            "weld" => "Merge vertices within the tolerance and remove collapsed faces.",
            "orient" => "Make adjacent face winding consistent across connected components.",
            "fillHoles" => "Cap open boundary loops with polygon faces.",
            "inspectVertex" => "Read the position of a zero-based vertex without changing mesh data.",
            "inspectEdge" => "Read endpoints and length of a preview halfedge index without changing mesh data.",
            "inspectFace" => "Read polygon corner indices, unit normal, and the arithmetic mean of corner positions.",
            "analyze" => "Measure surface area, bounds, and topology; volume is available for closed, consistently oriented meshes.",
            "exportObj" => "Serialize mesh vertices and polygon faces as OBJ text.",
            "exportJson" => "Serialize editable indexed vertices and polygon faces as JSON text.",
            _ => name,
        };
        register_untyped(registry, operator_info_with_outputs(&id, name, name, "emoji:🥽️", &format!("{summary} [quality:{}]", operation_quality(operation)), inputs, outputs, &[group]), Box::new(MeshOperation(operation, session.capture())), &produced);
    }
}
