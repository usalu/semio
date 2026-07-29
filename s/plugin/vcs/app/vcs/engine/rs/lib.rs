//! ⚙️ VCS app — headless compute (constitutional: engine).

use vcs::{VcsDemoProjection, VCS_DEMO_SCHEMA};

//#region 🔖DocumentHelpers
pub fn empty_vcs_demo_projection() -> VcsDemoProjection {
    VcsDemoProjection {
        schema: VCS_DEMO_SCHEMA.into(),
        title: "VCS Demo".into(),
        counter: 0,
        notes: String::new(),
        status: "new".into(),
        tags: Vec::new(),
    }
}
//#endregion 🔖DocumentHelpers
