//! 🪡️ Surfaces and sweeps: the surface constructors and the extrude, revolve, loft, sweep, pipe and helical sweep operators. See [`super`].

use super::*;

/// 🛹 Registers the plane, planar face, nurbs grid, coons, offset and thicken surface operators.
pub(super) fn register_surfaces(registry: &mut Registry, session: &Session) {
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
}

/// 🌀 Registers the solid extrude and the extrude, revolve, loft, sweep, pipe and helical sweep operators.
pub(super) fn register_sweeps(registry: &mut Registry, session: &Session) {
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
}
