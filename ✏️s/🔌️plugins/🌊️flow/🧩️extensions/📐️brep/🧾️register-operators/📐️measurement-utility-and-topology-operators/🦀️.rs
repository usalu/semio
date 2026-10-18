//! 📐️ Measurement, utility and topology operators: the measure operators, the vertex, sew, heal and nurbs conversion utilities and the shell, compound, explode and label topology operators. See [`super`].

use super::*;

/// 📏 Registers the volume, area, length, center of mass, bounding box, distance, closest point, classify and validate operators.
pub(super) fn register_measurements(registry: &mut Registry, session: &Session) {
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
}

/// 🧰 Registers the vertex, face from wire, sew, heal and convert to nurbs operators.
pub(super) fn register_utilities(registry: &mut Registry, session: &Session) {
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
}

/// 🕸 Registers the shells, compound, explode and label operators.
pub(super) fn register_topology(registry: &mut Registry, session: &Session) {
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
}
