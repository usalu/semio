//! 🏅️ BIM standard `1` root: mounts subset `any` and exports `standard() -> StandardDeclaration`.
//! The media identity is a synthesis: the artifact's capability rows only ever claimed the codec extension `bim`, never a MIME type.

use crate::standards::v1::subsets;
use semio_framework_artifact_reference::StandardId;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};

//#region 🔖️Standard
/// 🏅️ The complete declaration of standard `1`.
pub fn standard<A: crate::BimApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.bim+json"], extensions: &["bim"] }, subsets: vec![subsets::any::subset()] }
}
//#endregion 🔖️Standard
