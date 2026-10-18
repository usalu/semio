//! 🧊️ Schemas, primitives and curves: the brep schemas with the deconstruct operator, the 3d primitive solids and the curve operators. See [`super`].

use super::*;

/// 📦 Registers the geometry, topology, brep and text schemas and the brep deconstruct operator.
pub(super) fn register_schemas_and_deconstruct(registry: &mut Registry, session: &Session) {
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
}

/// 🧱 Registers the box, sphere, cylinder, cone, torus and convex hull operators.
pub(super) fn register_primitive_solids(registry: &mut Registry, session: &Session) {
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
}

/// 〰 Registers the line, circle, arc, ellipse, polyline, rectangle, polygon, interpolate, approximate and helix operators.
pub(super) fn register_curves(registry: &mut Registry, session: &Session) {
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
}
