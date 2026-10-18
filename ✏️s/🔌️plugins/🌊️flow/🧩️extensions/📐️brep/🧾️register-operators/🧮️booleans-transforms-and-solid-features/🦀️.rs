//! 🧮️ Booleans, transforms and solid features: the boolean, transform, pattern, fillet, chamfer, shell, draft and defeature operators. See [`super`].

use super::*;

/// ➗ Registers the fuse, cut, intersect and compound cut operators.
pub(super) fn register_booleans(registry: &mut Registry, session: &Session) {
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
}

/// 🔄 Registers the translate, rotate, scale, mirror, copy and pattern operators.
pub(super) fn register_transforms_and_patterns(registry: &mut Registry, session: &Session) {
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
}

/// 🛠 Registers the fillet, chamfer, shell, draft, offset solid and defeature operators.
pub(super) fn register_solid_features(registry: &mut Registry, session: &Session) {
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
}
