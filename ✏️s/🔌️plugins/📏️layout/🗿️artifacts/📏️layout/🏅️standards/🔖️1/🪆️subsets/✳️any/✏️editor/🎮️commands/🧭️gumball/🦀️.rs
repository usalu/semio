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
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    fn views() -> (LayoutSnapshot, NoConfig) {
        (crate::standards::v1::subsets::any::schema::default_document(), NoConfig::default())
    }

    #[test]
    fn translate_selection_moves_the_frame_by_the_delta() {
        let (document, config) = views();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let emit = handle(&TranslateSelection { ids: vec!["frame-1".into()], dx: 5.0, dy: -2.0 }, &doc, &cfg).expect("translate");
        let LayoutMutation::MoveFrame(moved) = &emit.artifact_mutations[0] else { panic!("move") };
        assert_eq!((moved.new_x, moved.new_y), (15.0, 8.0));
    }

    #[test]
    fn rotate_selection_adds_the_angle() {
        let (document, config) = views();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let emit = rotate(&RotateSelection { ids: vec!["frame-1".into()], angle: 0.5 }, &doc, &cfg).expect("rotate");
        let LayoutMutation::RotateFrame(rotated) = &emit.artifact_mutations[0] else { panic!("rotate") };
        assert_eq!(rotated.new_rotation, 0.5);
    }

    #[test]
    fn scale_selection_resizes_and_keeps_a_minimum() {
        let (document, config) = views();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let emit = scale(&ScaleSelection { ids: vec!["frame-1".into()], sx: 2.0, sy: 0.0 }, &doc, &cfg).expect("scale");
        let LayoutMutation::MoveFrame(moved) = &emit.artifact_mutations[0] else { panic!("move") };
        let LayoutMutation::ResizeFrame(resized) = &emit.artifact_mutations[1] else { panic!("resize") };
        assert_eq!((resized.new_width, resized.new_height), (80.0, 1.0));
        assert_eq!((moved.new_x, moved.new_y), (-10.0, 29.5));
    }
}
//#endregion 🧪️Tests
