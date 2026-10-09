#!/usr/bin/env bun
/** 🎛️ The chrome of the plan and section windows for authored views: a view select replaces the storey select, the cut height and the section depth (idempotent). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, RS, subset } from "./r3-f1-paths.ts";

const file = join(child(child(subset, "editor"), "chrome"), RS);
let text = readFileSync(file, "utf8");
const swap = (from: string, to: string) => {
  if (!text.includes(from)) return;
  text = text.replace(from, to);
};
swap("the plan's storey and cut height, the world's projection, storey isolation\n//! and section plane, the section's depth)", "the plan's and the section's view, the world's projection, storey isolation\n//! and section plane)");
swap('.filter(|row| !row.library && !matches!(row.kind, "site" | "building" | "storey" | "grid"))', '.filter(|row| !row.library && !matches!(row.kind, "site" | "building" | "storey" | "grid" | "view"))');
swap(`fn plan_measures(snapshot: &ModelSnapshot, config: &plan::config::BimPlanWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    let storey = plan::active_storey(snapshot, config).unwrap_or_default();
    vec![select("bim.measure.plan.storey", labels.measure_storey.as_str(), &storey, storey_items(snapshot), "storey"), number("bim.measure.plan.cut-height", labels.measure_cut_height.as_str(), config.cut_height, Some(0.0), 0.1, "cut_height")]
}`, `fn view_items(snapshot: &ModelSnapshot, ids: Vec<String>) -> Vec<(String, String)> {
    ids.into_iter().map(|id| (snapshot.views.get(&id).map_or(id.clone(), |row| row.name.clone()), id)).map(|(name, id)| (id, name)).collect()
}

fn plan_measures(snapshot: &ModelSnapshot, config: &plan::config::BimPlanWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    let view = plan::active_view(snapshot, config).unwrap_or_default();
    vec![select("bim.measure.plan.view", labels.measure_view.as_str(), &view, view_items(snapshot, plan::plan_views(snapshot)), "view")]
}`);
swap(`fn section_measures(config: &section::config::BimSectionWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    vec![number("bim.measure.section.depth", labels.measure_section_depth.as_str(), config.depth, Some(0.0), 0.5, "depth")]
}`, `fn section_measures(snapshot: &ModelSnapshot, config: &section::config::BimSectionWindowConfig, labels: &BimLabels) -> Vec<WindowMeasure> {
    let view = section::active_view(snapshot, config).unwrap_or_default();
    vec![select("bim.measure.section.view", labels.measure_view.as_str(), &view, view_items(snapshot, section::vertical_views(snapshot)), "view")]
}`);
swap("section::WINDOW_KIND_ID => section_measures(&section::config::current(cfg), labels),", "section::WINDOW_KIND_ID => section_measures(snapshot, &section::config::current(cfg), labels),");
writeFileSync(file, text);
console.log("chrome updated");
