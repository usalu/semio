//! 🗂️ VCS app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

//#region 🔖️Constants
pub const VCS_DEMO_SCHEMA: &str = "vcs.demo";
//#endregion 🔖️Constants

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "vcsdemo")]
pub struct VcsDemoProjection {
    pub schema: String,
    pub title: String,
    pub counter: i64,
    pub notes: String,
    pub status: String,
    pub tags: Vec<String>,
}
//#endregion 🔖️Types
