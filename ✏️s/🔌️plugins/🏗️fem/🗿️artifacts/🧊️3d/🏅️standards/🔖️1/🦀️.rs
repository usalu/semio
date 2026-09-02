//! 🏅️ Standard root — `standard() -> StandardDeclaration` (`terra-fleet-trinity-recipe` recipe,
//! `📓️terra-fleet-trinity-recipe-report.md`), mounts subset `any`. `mimes`/`extensions`: no real MIME
//! registration exists for `s.fem3d` outside the old `ArtifactDefinition`'s capability rows
//! (`definition()`, kept — debt D1) — that channel only ever claimed a codec id (`fem.3d:fem3d`) and
//! an extension (`fem3d`), never a mime type. `mimes: ["application/vnd.semio.fem3d+json"]` is a
//! documented synthesis (matches `🗒️note`/`🖍️draw`/`🔱️trinity`'s identical documented deviation);
//! `extensions: ["fem3d"]` is the real, carried-over value from `definition()`'s
//! `s.fem3d.codec.document.v1` row.

use crate::artifacts::fem3d::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard() -> StandardDeclaration<crate::FemApps> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.fem3d+json"], extensions: &["fem3d"] }, subsets: vec![subsets::any::subset()] }
}
