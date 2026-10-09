//! 📊️ Native JSON inference table projection.
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanKind;
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use crate::ViewKind;
use std::collections::BTreeMap;

/// 📏️ The canonical JSON table `view → measure → value` the view oracle compares: the silhouette union, cut area and datum length of every section and elevation (see `VERTICAL_METRICS`). A plan is measured by the `plan-metrics` of its storey, a camera draws nothing.
pub fn metrics_json(views: &BTreeMap<String, ViewLinework>) -> String {
    let table: BTreeMap<String, BTreeMap<String, f64>> = views
        .iter()
        .filter(|(_, view)| matches!(view.kind, ViewKind::Section | ViewKind::Elevation))
        .map(|(id, view)| {
            let row = [("Silhouette.union_area", view.union_area_of(PlanKind::Silhouette)), ("SectionCut.area", view.lines.area_of(PlanKind::SectionCut)), ("Datum.length", view.lines.length_of(PlanKind::Datum))];
            (id.clone(), row.into_iter().map(|(name, value)| (name.to_string(), value)).collect())
        })
        .collect();
    semio_framework_pack_json::to_json_string(&table)
}
