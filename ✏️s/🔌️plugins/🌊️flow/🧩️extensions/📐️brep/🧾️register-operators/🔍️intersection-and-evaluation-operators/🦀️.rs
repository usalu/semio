//! 🔍️ Intersection and evaluation operators: the section, split and intersection operators and the curve and surface evaluation operators. See [`super`].

use super::*;

/// ✂ Registers the section, split, curve curve, curve surface and surface surface intersection operators.
pub(super) fn register_intersections(registry: &mut Registry, session: &Session) {
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
}

/// 📍 Registers the curve and surface point, tangent, domain, curvature, normal and closest parameter operators.
pub(super) fn register_evaluations(registry: &mut Registry, session: &Session) {
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
}
