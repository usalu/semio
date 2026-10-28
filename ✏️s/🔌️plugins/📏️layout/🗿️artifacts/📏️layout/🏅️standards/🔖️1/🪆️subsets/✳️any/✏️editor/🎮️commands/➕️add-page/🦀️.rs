//! ✏️ ✏️ Layout play app commands command — `add-page`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::modes::edit::windows::blueprint::config::current;
use semio_s_artifact_layout_layout_mutations_content::create_page::CreatePage;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_model::{PageColumns, PageMargins};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-page")]
pub struct AddPage {}

pub fn handle(_payload: &AddPage, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = current(cfg);
    let template = document.pages.iter().find(|page| page.id == config.active_page_id).or_else(|| document.pages.first());
    let (width, height, spread_id, parent_page_id, margins, columns) = template.map_or((595.0, 842.0, "spread-1".into(), None, PageMargins { top: 48.0, right: 36.0, bottom: 48.0, left: 36.0 }, PageColumns { count: 1, gutter: 0.0 }), |page| {
        (page.width, page.height, page.spread_id.clone(), page.parent_page_id.clone(), page.margins.clone(), page.columns.clone())
    });
    let page_id = format!("page-{}", document.pages.len() + 1);
    let layer_id = format!("layer-{page_id}");
    let index = document.pages.len();
    let page = semio_s_artifact_layout_layout_model::Page {background_drawing:None,
        id: page_id.clone(),
        name: format!("Page {}", document.pages.len() + 1),
        spread_id,
        parent_page_id,
        width,
        height,
        margins,
        columns,
        guides: Vec::new(),
        layer_ids: vec![layer_id.clone()],
        layers: vec![semio_s_artifact_layout_layout_model::Layer { id: layer_id, name: "Content".into(), visible: true, locked: false, object_ids: Vec::new() }],
        frames: Vec::new(),
        overrides: Vec::new(),
    };
    Ok(Emit {
        artifact_mutations: vec![LayoutMutation::CreatePage(CreatePage { page, index: Some(index) })],
        ..Default::default()
    })
}
