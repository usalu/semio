//! ⚙️ VCS artifact — headless compute (was: constitutional `engine`).

use crate::artifacts::vcs::{VcsDemoProjection, VCS_DEMO_SCHEMA};

//#region 🔖️DocumentHelpers
pub fn empty_vcs_demo_projection() -> VcsDemoProjection {
    VcsDemoProjection { schema: VCS_DEMO_SCHEMA.into(), title: "VCS Demo".into(), counter: 0, notes: String::new(), status: "new".into(), tags: Vec::new() }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers `VcsDemoProjection`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse vcs-play
/// documents without depending on this crate's concrete `Projection`/`Operation` types. Called by
/// `semio_plugin!`'s `setup:` hook — was the old bundle crate's `register_vcs_exports()`.
pub fn register() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::vcs::VcsPlayApp>(VCS_DEMO_SCHEMA);
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_projection_matches_schema() {
        let projection = empty_vcs_demo_projection();
        assert_eq!(projection.schema, VCS_DEMO_SCHEMA);
        assert_eq!(projection.status, "new");
    }
}
//#endregion 🧪️Tests
