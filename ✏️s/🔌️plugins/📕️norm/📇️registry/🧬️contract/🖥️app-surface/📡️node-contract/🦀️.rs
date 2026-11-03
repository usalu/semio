//! 📡️ Node contract: the law every artifact's default document meets for the screen reader, so each artifact test is one call. Every check row, scene entity, handle and diagram has an accessible node with a label, a WAI-ARIA role and its value or position in English and German.
//!
//! 🔗️ Builders: [`check_window_nodes`](crate::check_window_nodes), [`viewport_nodes`](crate::accessibility::published_nodes::viewport_nodes), [`diagram_window_nodes`](crate::accessibility::published_nodes::diagram_window_nodes).

use crate::accessibility::published_nodes::{diagram_window_nodes, viewport_nodes};
use crate::diagrams2d::marker_grip_id;
use crate::viewport3d::config::Lang;
use crate::viewport3d::description::pick;
use crate::*;
use semio_s_plugin_norm_structural_scene::{Diagram2d, Handle, StructuralScene};

/// 🔢️ How many nodes of each kind a document published.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NodeCounts {
    pub checks: usize,
    pub entities: usize,
    pub handles: usize,
    pub diagrams: usize,
    pub markers: usize,
}

/// ♿️ Panics unless the report, scene, handles and diagrams of a document publish a complete accessibility tree in both languages: a table and a named, positioned row per check (`aria-posinset` of `aria-setsize`), the scene list with a selectable option per entity, a slider with value text and range per handle named by its accessible name, and per diagram an image described by its table twin. Returns the counts so a caller can state its document is not empty.
pub fn assert_default_document_nodes(report: &CheckReport, scene: &StructuralScene, handles: &[Handle], diagrams: &[Diagram2d]) -> NodeCounts {
    for (lang, locale) in [(Lang::En, Locale::En), (Lang::De, Locale::De)] {
        let rows = check_window_nodes(report, &ChecksViewState::default(), locale);
        assert_eq!(rows.len(), 1 + report.checks.len(), "a table and a row per check ({lang:?})");
        assert_eq!(rows[0].node.role, "table");
        assert!(rows[1..].iter().all(|n| n.node.role == "row"), "{lang:?}");
        for check in &report.checks {
            let node = rows.iter().find(|n| n.key == format!("check:{}", check.id)).unwrap_or_else(|| panic!("no row node for check {} ({lang:?})", check.id));
            let title = if lang == Lang::De { &check.title.de } else { &check.title.en };
            assert!(!title.is_empty() && node.node.name.contains(title.as_str()), "row name {:?} speaks the title {title:?}", node.node.name);
            assert!(node.node.position.is_some_and(|(at, of)| at >= 1 && at <= of && of == report.checks.len()), "{}", check.id);
            assert!(node.node.missing_required().is_empty());
        }
        let scene_nodes = viewport_nodes(scene, &[], handles, lang);
        let options: Vec<_> = scene_nodes.iter().filter(|n| n.node.role == "option").collect();
        let sliders: Vec<_> = scene_nodes.iter().filter(|n| n.node.role == "slider").collect();
        assert_eq!((scene_nodes.iter().filter(|n| n.node.role == "listbox").count(), options.len(), sliders.len()), (1, scene.entities.len(), handles.len()), "{lang:?}");
        assert!(options.iter().all(|n| !n.node.name.is_empty() && n.node.selected == Some(false) && n.node.missing_required().is_empty()));
        for (handle, node) in handles.iter().zip(&sliders) {
            assert_eq!(node.node.name, pick(&handle.accessible_name, lang), "slider of handle {}", handle.id);
            assert!(!node.node.name.is_empty() && node.node.range.is_some() && node.node.value_text.is_some() && node.node.missing_required().is_empty(), "{}", handle.id);
        }
        let diagram_nodes = diagram_window_nodes(diagrams, lang);
        let bound: Vec<_> = diagrams.iter().flat_map(|diagram| diagram.markers.iter().filter(|marker| marker.binding.is_some()).map(move |marker| (diagram, marker))).collect();
        assert_eq!(diagram_nodes.len(), 2 * diagrams.len() + bound.len());
        for (diagram, marker) in &bound {
            let key = marker_grip_id(diagram, marker);
            let slider = diagram_nodes.iter().find(|n| n.key == key).unwrap_or_else(|| panic!("no slider for marker {key} ({lang:?})"));
            assert_eq!(slider.node.role, "slider");
            assert_eq!(slider.node.name, pick(&marker.label, lang), "{key}");
            assert!(!slider.node.name.is_empty() && slider.node.value_text.is_some() && slider.node.range.is_some() && slider.node.missing_required().is_empty(), "{key}");
            assert!(marker.utilisation.is_none() || slider.node.value_text.as_deref().is_some_and(|text| text.contains('%')), "the slider of {key} speaks the live utilisation of its check");
        }
        for (diagram, pair) in diagrams.iter().zip(diagram_nodes.chunks(2)).take(diagrams.len()) {
            let (image, table) = (&pair[0], &pair[1]);
            assert_eq!((image.node.role, table.node.role), ("img", "table"));
            assert_eq!(image.node.name, pick(&diagram.title, lang));
            assert!(!image.node.name.is_empty() && image.node.description.is_some() && image.node.described_by.as_deref() == Some(table.key.as_str()) && table.node.description.is_some());
        }
    }
    NodeCounts { checks: report.checks.len(), entities: scene.entities.len(), handles: handles.len(), diagrams: diagrams.len(), markers: diagrams.iter().map(|d| d.markers.iter().filter(|m| m.binding.is_some()).count()).sum() }
}
