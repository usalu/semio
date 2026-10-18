//! 🧾️ Register operators: the registration of the brep schemas and operators, the extension manifest json and the module registry. See [`super`].

use super::*;

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
    let session = semio_s_artifact_stdio_step_geometry::geometry::session::geometry_session();
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
