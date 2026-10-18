//! 📂️ IO operators: the step, stl, obj and dwg import and export operators. See [`super`].

use super::*;

/// 💾 Registers the step, stl, obj and dwg export and import operators.
pub(super) fn register_io(registry: &mut Registry, session: &Session) {
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
}
