//! 🏅️ Standard root — `standard() -> StandardDeclaration` (`terra-fleet-trinity-recipe` recipe,
//! `📓️terra-fleet-trinity-recipe-report.md`), mounts subset `any`. `mimes`/`extensions`: no real MIME
//! registration exists for `s.fem2d` outside the old `ArtifactDefinition`'s capability rows
//! (`definition()`, kept — debt D1) — that channel only ever claimed a codec id (`fem.2d:fem2d`) and
//! an extension (`fem2d`), never a mime type. `mimes: ["application/vnd.semio.fem2d+json"]` is a
//! documented synthesis (matches `🗒️note`/`🖍️draw`/`🔱️trinity`'s identical documented deviation);
//! `extensions: ["fem2d"]` is the real, carried-over value from `definition()`'s
//! `s.fem2d.codec.document.v1` row.

use crate::artifacts::fem2d::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.fem2d+json"], extensions: &["fem2d"] }, subsets: vec![subsets::any::subset()] }
}
