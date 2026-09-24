//! 🧭️ Blueprint gumball — incremental move, rotate, and scale of the selected frames.

use crate::editor::layout::canvas::active_page;
use crate::editor::layout::modes::edit::windows::blueprint::config::current;
use crate::mutations::move_frame::MoveFrame;
use crate::mutations::resize_frame::ResizeFrame;
use crate::mutations::rotate_frame::RotateFrame;
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

fn page_id_for<'a>(doc: &'a LayoutSnapshot, cfg: &ConfigView<'_, NoConfig>) -> Option<&'a crate::Page> {
    active_page(doc, &current(cfg))
}

fn frame_on_page<'a>(page: &'a crate::Page, id: &str) -> Option<&'a crate::Frame> {
    page.frames.iter().find(|frame| frame.id() == id)
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "translate-selection")]
pub struct TranslateSelection {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let Some(page) = page_id_for(doc.snapshot, cfg) else { return Ok(Emit::default()) };
    let mutations: Vec<LayoutMutation> = payload
        .ids
        .iter()
        .filter_map(|id| {
            let bounds = frame_on_page(page, id)?.bounds();
            Some(LayoutMutation::MoveFrame(MoveFrame { page_id: page.id.clone(), frame_id: id.clone(), new_x: bounds.x + payload.dx, new_y: bounds.y + payload.dy }))
        })
        .collect();
    if mutations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::amend(mutations, "gumball-translate"))
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "rotate-selection")]
pub struct RotateSelection {
    pub ids: Vec<String>,
    pub angle: f64,
}

pub fn rotate(payload: &RotateSelection, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let Some(page) = page_id_for(doc.snapshot, cfg) else { return Ok(Emit::default()) };
    let mutations: Vec<LayoutMutation> = payload
        .ids
        .iter()
        .filter_map(|id| {
            let bounds = frame_on_page(page, id)?.bounds();
            Some(LayoutMutation::RotateFrame(RotateFrame { page_id: page.id.clone(), frame_id: id.clone(), new_rotation: bounds.rotation + payload.angle }))
        })
        .collect();
    if mutations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::amend(mutations, "gumball-rotate"))
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "scale-selection")]
pub struct ScaleSelection {
    pub ids: Vec<String>,
    pub sx: f64,
    pub sy: f64,
}

pub fn scale(payload: &ScaleSelection, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let Some(page) = page_id_for(doc.snapshot, cfg) else { return Ok(Emit::default()) };
    let mutations: Vec<LayoutMutation> = payload.ids.iter().filter_map(|id| frame_on_page(page, id).map(|frame| (id.clone(), frame.bounds().clone()))).flat_map(|(id, bounds)| {
        let new_width = (bounds.width * payload.sx).max(1.0);
        let new_height = (bounds.height * payload.sy).max(1.0);
        let new_x = bounds.x + (bounds.width - new_width) * 0.5;
        let new_y = bounds.y + (bounds.height - new_height) * 0.5;
        [
            LayoutMutation::MoveFrame(MoveFrame { page_id: page.id.clone(), frame_id: id.clone(), new_x, new_y }),
            LayoutMutation::ResizeFrame(ResizeFrame { page_id: page.id.clone(), frame_id: id, new_width, new_height }),
        ]
    }).collect();
    if mutations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::amend(mutations, "gumball-scale"))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
