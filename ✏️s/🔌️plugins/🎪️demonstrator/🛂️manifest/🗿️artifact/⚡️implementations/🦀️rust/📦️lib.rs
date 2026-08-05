//! 🎪️ Entwerfen mit Bestand demonstrator — the six demonstrator panes' apps (procedural3d, cad,
//! puzzle3d, sourcing-curate, process3d, gis2d) bundled as ONE hot-swappable WASM plugin instead of
//! six separate ones, so the demonstrator's six panes share one framework/kernel linkage and one
//! plugin worker/module (see `acquirePluginModule`'s lease pool in framework core `📦️index.ts`)
//! instead of statically duplicating the SDK six times over. Manual `PluginBundle` builder (not
//! `semio_plugin!` — this crate owns no `🎛️apps/` of its own; every app it registers is a
//! transitively-shared dependency of its own source plugin crate, same pattern as `s`'s bundle, see
//! `✏️s/🔌️plugins/🪐️space/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs`). Each source
//! plugin (procedural/cad/puzzle/sourcing/process/gis) keeps building its own standalone playground
//! variants unaffected — only the demonstrator's own six rows moved off those crates' `Cargo.toml`s
//! and onto this one.

use semio_framework_plugin::MeshData;
use serde_json::Value;

//#region 🔖️CadExports
fn cad_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    cad::artifacts::cad::engine::cad_mesh_from_document(doc)
}

fn cad_document_from_dwg(drawing: &semio_framework_core::DwgDrawing) -> Result<Value, String> {
    cad::artifacts::cad::engine::cad_document_from_dwg(drawing)
}

fn cad_document_from_mesh(mesh: &MeshData) -> Result<Value, String> {
    cad::artifacts::cad::engine::cad_document_from_mesh(mesh)
}

fn register_cad_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<cad::apps::cad::CadPlayApp>(cad::artifacts::cad::CAD_DOCUMENT_SCHEMA);
    semio_framework_os::register_solid_exporter("3d.cad", Box::new(kernel_3d_brepkit::ObjSolidExporter));
    semio_framework_os::register_solid_exporter("3d.cad", Box::new(kernel_3d_brepkit::StlSolidExporter));
    semio_framework_os::register_solid_exporter("3d.cad", Box::new(kernel_3d_brepkit::StepSolidExporter));
    semio_framework_os::register_solid_importer("3d.cad", Box::new(kernel_3d_brepkit::ObjSolidImporter));
    semio_framework_os::register_solid_importer("3d.cad", Box::new(kernel_3d_brepkit::StlSolidImporter));
    semio_framework_os::register_solid_importer("3d.cad", Box::new(kernel_3d_brepkit::StepSolidImporter));
    semio_framework_os::register_mesh_exporter("3d.cad", "cad", cad_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_importer("3d.cad", cad_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.cad", "cad", cad_mesh_from_document);
    semio_framework_os::register_dwg_import_handler("3d.cad", cad_document_from_dwg);
}
//#endregion 🔖️CadExports

//#region 🔖️ProceduralExports
/// 🪶️ Only the 3d half of procedural's setup — the demonstrator's `generator` pane boots
/// `procedural3d-play` exclusively; `procedural_2d_ui` isn't a dependency of this bundle at all.
fn register_procedural_exports() {
    semio_framework_os::register_mesh_exporter("3d.procedural", "procedural", procedural::artifacts::procedural3d::engine::procedural3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.procedural", "procedural", procedural::artifacts::procedural3d::engine::procedural3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.procedural", "procedural", procedural::artifacts::procedural3d::engine::procedural3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.procedural", "procedural", procedural::artifacts::procedural3d::engine::procedural3d_mesh_from_document);
    semio_framework_os::register_mesh_importer("3d.procedural", procedural::artifacts::procedural3d::engine::procedural3d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.procedural", procedural::artifacts::procedural3d::engine::procedural3d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.procedural", procedural::artifacts::procedural3d::engine::procedural3d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.procedural", procedural::artifacts::procedural3d::engine::procedural3d_document_from_mesh);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<procedural::apps::procedural3d::Procedural3dPlayApp>(procedural::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA);
}
//#endregion 🔖️ProceduralExports

//#region 🔖️ProcessExports
fn process3d_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let document: process_3d::Process3dDocument = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    process_3d_engine::processed_mesh(&document).ok_or_else(|| "process3d: kernel replay failed".to_string())
}

fn process3d_document_from_mesh(_mesh: &MeshData) -> Result<Value, String> {
    Err("process3d: mesh import not supported".into())
}

fn register_process3d_exports() {
    semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.process", "process", process3d_mesh_from_document);
    semio_framework_os::register_mesh_dwg_import_handler("3d.process", process3d_document_from_mesh);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<process_3d_ui::Process3dPlayApp>(process_3d::PROCESS_3D_SCHEMA);
}
//#endregion 🔖️ProcessExports

//#region 🔖️GisExports
/// 🪶️ Only the 2d half of gis's setup — the demonstrator's `verfolgen` pane boots `gis2d-play`
/// exclusively; `gis3d_ui` isn't a dependency of this bundle at all.
fn register_gis_exports() {
    semio_framework_os::register_2d_export_handlers("2d.map", "gis2d", gis2d_engine::gis2d_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.map", gis2d_engine::gis2d_document_json_from_dwg);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<gis2d_ui::Gis2dPlayApp>(gis2d::GIS_MAP_SCHEMA);
}
//#endregion 🔖️GisExports

//#region 🔖️SourcingExports
fn register_sourcing_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<sourcing::apps::curate::SourcingCurateApp>(sourcing::artifacts::curate::SOURCING_CURATE_SCHEMA);
}
//#endregion 🔖️SourcingExports

//#region 🔖️Manifest
fn register_demonstrator_exports() {
    register_procedural_exports();
    register_cad_exports();
    puzzle_3d_ui::register_puzzle3d_exports();
    register_sourcing_exports();
    register_process3d_exports();
    register_gis_exports();
}

fn bundle() -> semio_framework_plugin::PluginBundle {
    register_demonstrator_exports();
    semio_framework_plugin::PluginBundle::new("demonstrator", "Entwerfen mit Bestand", "0.1.0")
        .register_document_app(procedural::apps::procedural3d::create_procedural3d_app(), || procedural::apps::procedural3d::Procedural3dPlayApp)
        .register_document_app(cad::apps::cad::create_cad_app(), || <cad::apps::cad::CadPlayApp as ::std::default::Default>::default())
        .register_document_app(puzzle_3d_ui::create_puzzle3d_app(), || <puzzle_3d_ui::Puzzle3dPlayApp as ::std::default::Default>::default())
        .register_document_app(sourcing::apps::curate::create_sourcing_curate_app(), || sourcing::apps::curate::SourcingCurateApp)
        .register_document_app(process_3d_ui::create_process3d_app(), || process_3d_ui::Process3dPlayApp)
        .register_document_app(gis2d_ui::create_gis2d_app(), || gis2d_ui::Gis2dPlayApp)
}
semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖️Manifest
