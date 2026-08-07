    semio_framework_os::register_mesh_dwg_export_handler("3d.procedural", "procedural", procedural3d_mesh_from_document);
    semio_framework_os::register_mesh_importer("3d.procedural", procedural3d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.procedural", procedural3d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.procedural", procedural3d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.procedural", procedural3d_document_from_mesh);
    // 📦️ Registers `Procedural3dDocument`'s pack<->dsl codec so `framework/sync`'s `FolderEndpoint`
    // can print/parse `.procedural3d` packs without depending on this crate's concrete types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::procedural3d::Procedural3dPlayApp>(crate::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA);
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️TestSupport
/// 🧵️ `flow_extension_brep::tessellate_geometry` (and the flow-eval neuron kernel cache it sits behind)
/// is a process-wide cache shared by every test in this ONE merged crate — before the crate
/// consolidation, the artifact/app constitutional crates each ran in their own `cargo test` process, so
/// a `TEST_SERIAL` local to one of them never had to coordinate with the other's. Now that every
/// taxonomy node's tests share one test binary, ANY test that evaluates a flow fixture and/or tessellates
/// BRep geometry (directly here, or indirectly via the app's preview-window `render()`) must acquire
/// THIS single crate-wide lock — see `crate::apps::procedural3d::modes::edit::windows::preview`'s test
/// for the app-side half of this.
#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::{Mutex, MutexGuard};

    
    pub fn lock() -> MutexGuard<'static, ()> {
        TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}
//#endregion 🧪️TestSupport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_3d::scene::{aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Mesh3d, Vec3};
    use std::sync::MutexGuard;

    fn test_serial() -> MutexGuard<'static, ()> {
        test_support::lock()