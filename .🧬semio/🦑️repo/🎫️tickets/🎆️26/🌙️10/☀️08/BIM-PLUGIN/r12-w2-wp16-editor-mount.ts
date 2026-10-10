#!/usr/bin/env bun
/**
 * 🪢️ Wires the coordination IO, entities, panels, commands, world window state and projections of WP-16 into the crate in ONE transactional pass (idempotent): every file is read, every anchored edit applied in
 * memory and all files are written at the end, so the crate is never left half converted. An anchor that is missing or ambiguous stops the script before anything is written. Run `r12-w2-wp16-labels.ts` first
 * (the new code uses the labels it adds).
 */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, child, schema, subset } from "./r3-f1-paths.ts";

const pending = new Map<string, string>();
const notes: string[] = [];
const crlf = new Set<string>();
const load = (path: string) => {
  const held = pending.get(path);
  if (held !== undefined) return held;
  const text = readFileSync(path, "utf8");
  if (!text.includes("\r\n")) return text;
  crlf.add(path);
  return text.replaceAll("\r\n", "\n");
};
const find = (dir: string, suffix: string) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix))!);
const rs = (dir: string) => find(dir, ".rs");
const io = child(subset, "io");
const editor = child(subset, "editor");

type Edit = { marker: string; anchor: string | RegExp; insert?: string; before?: boolean; replace?: string };

const patch = (path: string, edits: Edit[]) => {
  let text = load(path);
  for (const edit of edits) {
    if (text.includes(edit.marker)) {
      notes.push(`already: ${edit.marker.trim().slice(0, 60)}`);
      continue;
    }
    const where = typeof edit.anchor === "string" ? text.indexOf(edit.anchor) : text.search(edit.anchor);
    if (where < 0) throw new Error(`anchor not found in ${path}: ${edit.anchor}`);
    const matched = typeof edit.anchor === "string" ? edit.anchor : text.slice(where).match(edit.anchor)![0];
    if (typeof edit.anchor === "string" && text.indexOf(edit.anchor, where + 1) >= 0) throw new Error(`ambiguous anchor in ${path}: ${edit.anchor.slice(0, 80)}`);
    if (edit.replace !== undefined) text = text.slice(0, where) + edit.replace + text.slice(where + matched.length);
    else {
      const cut = edit.before ? where : where + matched.length;
      text = text.slice(0, cut) + edit.insert + text.slice(cut);
    }
    notes.push(`edited: ${edit.marker.trim().slice(0, 60)}`);
  }
  pending.set(path, text);
};

const mount = (indent: string, name: string, path: string) => `${indent}#[path = "."]\n${indent}pub mod ${name} {\n${indent}    #[path = "${path}"]\n${indent}    mod component;\n${indent}    pub use component::*;\n${indent}}\n`;
const STD = "🏅️standards/🔖️1/🪆️subsets/✳️any/";

//#region 🔖️Cargo
patch(join(artifact, "Cargo.toml"), [{ marker: "[workspace.dependencies.semio-s-artifact-stdio-bcf]", anchor: "[workspace.dependencies.semio-s-artifact-stdio-csv]\n", before: true, insert: '[workspace.dependencies.semio-s-artifact-stdio-bcf]\npath = "../../../🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🦀️rust"\n\n' }]);
patch(join(artifact, "📦️packages", "🦀️rust", "Cargo.toml"), [{ marker: "semio-s-artifact-stdio-bcf", anchor: "semio-s-artifact-stdio-csv = { workspace = true }\n", before: true, insert: "semio-s-artifact-stdio-bcf = { workspace = true }\n" }]);
//#endregion 🔖️Cargo

//#region 🔖️Io
patch(rs(io), [
  { marker: "pub mod bcf;", anchor: '#[path = "📥️import/🦀️.rs"]\npub mod import;\n', insert: '\n#[path = "💬️bcf/🦀️.rs"]\npub mod bcf;\n' },
  { marker: "bcf::ModelIntoBcf", anchor: "                    serializer_entry::<ModelSnapshot, export::sheets::ModelIntoSheetsPdf>(BIM_MODEL_DIALECT),\n", insert: "                    serializer_entry::<ModelSnapshot, bcf::ModelIntoBcf>(BIM_MODEL_DIALECT),\n                    deserializer_entry::<ModelSnapshot, bcf::BcfIntoModel>(BIM_MODEL_DIALECT),\n" },
]);
//#endregion 🔖️Io

//#region 🔖️Projections
patch(rs(find(child(io, "text"), "snapshot")), [
  {
    marker: '"clashes" =>',
    anchor: '        "sheets" => Some(',
    before: true,
    insert:
      '        "clashes" => Some(crate::standards::v1::subsets::any::schema::inferences::clash_sets::table_json(&inferred.clash_sets)),\n        "clash-meshes" => Some(crate::standards::v1::subsets::any::schema::inferences::clash_sets::meshes_json(snapshot, &inferred.element_solids)),\n        "rules" => Some(crate::standards::v1::subsets::any::schema::inferences::rule_results::table_json(&inferred.rule_results)),\n        "rule-measures" => Some(crate::standards::v1::subsets::any::schema::inferences::rule_results::measures_json(snapshot, inferred)),\n        "bcf" => Some(crate::standards::v1::subsets::any::io::bcf::table_json(snapshot)),\n',
  },
]);
const inferences = child(schema, "inferences");
const drafts = join(import.meta.dir, "🗑️generated", "w2-wp16-coordination", "drafts");
for (const [folder, draft] of [["clash-sets", "clash-tables.rs"], ["rule-results", "rule-tables.rs"]] as const) {
  patch(rs(join(inferences, readdirSync(inferences).find((name) => name.endsWith(folder))!)), [{ marker: "//#region 🔖️Tables", anchor: '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;', before: true, insert: readFileSync(join(drafts, draft), "utf8") }]);
}
//#endregion 🔖️Projections

//#region 🔖️Entities
patch(rs(child(editor, "entities")), [
  { marker: "pub mod coordination;", anchor: '#[path = "📄️sheets/🦀️.rs"]\npub mod sheets;\n', insert: '\n#[path = "🤝️coordination/🦀️.rs"]\npub mod coordination;\n' },
  { marker: "coordination::CLASH_SET,", anchor: "    sheets::REVISION,\n", insert: "    coordination::CLASH_SET,\n    coordination::RULE,\n    coordination::ISSUE,\n    coordination::ISSUE_COMMENT,\n" },
  { marker: '"sheet-revision" | "clash-set"', anchor: '"viewport" | "sheet-revision" => None,', replace: '"viewport" | "sheet-revision" | "clash-set" | "rule" | "issue" | "issue-comment" => None,' },
]);
//#endregion 🔖️Entities

//#region 🔖️World
const world = child(child(child(child(editor, "modes"), "edit"), "windows"), "world");
patch(rs(child(world, "config")), [{ marker: "isolated_elements: Vec<String>", anchor: "    hidden_storeys: Vec<String> = Vec::new();\n", insert: "    isolated_elements: Vec<String> = Vec::new();\n    section_box: Vec<f64> = Vec::new();\n" }]);
patch(rs(world), [
  {
    marker: "fn in_box(",
    anchor: "/// 👁️ Whether the solid of element `id` is drawn:",
    before: true,
    insert:
      "/// 🔭️ Whether the world box of the solid meets the section box of the window (six values: the corner with the smallest coordinates, then the largest; none means no box).\nfn in_box(solid: &ElementSolid, config: &BimWorldWindowConfig) -> bool {\n    let [low_x, low_y, low_z, high_x, high_y, high_z] = config.section_box[..] else { return true };\n    world_bounds([solid]).is_none_or(|(min, max)| min[0] <= high_x && max[0] >= low_x && min[1] <= high_y && max[1] >= low_y && min[2] <= high_z && max[2] >= low_z)\n}\n\n",
  },
  { marker: "config.isolated_elements.is_empty()", anchor: "    (config.isolated_storey.is_empty() || solid.storey == config.isolated_storey)\n", replace: "    (config.isolated_elements.is_empty() || config.isolated_elements.iter().any(|isolated| isolated == id))\n        && in_box(solid, config)\n        && (config.isolated_storey.is_empty() || solid.storey == config.isolated_storey)\n" },
]);
//#endregion 🔖️World

//#region 🔖️Commands
const commands = child(editor, "commands");
patch(rs(child(commands, "create-entity")), [{ marker: '"clash-set" | "rule" | "issue" => None', anchor: '| "window-type" | "door-type" => None,', replace: '| "window-type" | "door-type" | "clash-set" | "rule" | "issue" => None,\n        "issue-comment" => Some("issue"),' }]);
patch(rs(child(commands, "export-model")), [
  { marker: '"glb", "svg", "csv", "bcf"]', anchor: '"glb", "svg", "csv"]', replace: '"glb", "svg", "csv", "bcf"]' },
  { marker: "use crate::standards::v1::subsets::any::io::bcf;", anchor: "use crate::standards::v1::subsets::any::io::export::{csv, gltf, ifc, svg};\n", insert: "use crate::standards::v1::subsets::any::io::bcf;\n" },
  { marker: '"bcf" => {', anchor: '        "csv" => ("csv", "text/csv", csv::report_csv(model, inferred), None),\n', insert: '        "bcf" => {\n            let bytes = bcf::export_bcf(model).map_err(|error| failed(format, error))?;\n            ("bcf", "application/zip", semio_framework_io_base64::base64_standard_encode(&bytes), Some(MEDIA_EXPORT_BASE64_ENCODING))\n        }\n' },
]);
patch(rs(child(editor, "chrome")), [
  { marker: '("bcf", labels.export_bcf.as_str())', anchor: '("csv", labels.export_csv.as_str())]', replace: '("csv", labels.export_csv.as_str()), ("bcf", labels.export_bcf.as_str())]' },
  {
    marker: "bim.measure.world.show-all",
    anchor: '        WindowMeasure::Toggle { id: "bim.measure.world.analyse".into()',
    before: true,
    insert:
      '        WindowMeasure::Toggle { id: "bim.measure.world.show-all".into(), icon_id: "eye".into(), label: Some(labels.clash_show_all.as_str().to_string()), pressed: !(config.isolated_elements.is_empty() && config.section_box.is_empty()), text: None, on_change: bim_window_action("viewClash", Some(DslValue::object([("first".to_string(), DslValue::String(String::new())), ("second".to_string(), DslValue::String(String::new())), ("mode".to_string(), DslValue::String("clear".to_string()))]))) },\n',
  },
]);
//#endregion 🔖️Commands

//#region 🔖️EditorRoot
const root = rs(editor);
patch(root, [
  { marker: "select_findings, coordinate, set_camera,", anchor: "select_findings, set_camera,", replace: "select_findings, coordinate, set_camera," },
  { marker: "coordination as coordination_panel", anchor: "diagnostics as diagnostics_panel,", replace: "diagnostics as diagnostics_panel, coordination as coordination_panel," },
  {
    marker: '"viewClash" as "view-clash"',
    anchor: /            "selectFindings" as "select-findings"[^\n]*\n/,
    insert:
      '            "viewClash" as "view-clash" => coordinate::ViewClash, [WindowConfig]; View, cmd_view_clash, cmd_view_clash_describe;\n            "raiseIssue" as "raise-issue" => coordinate::RaiseIssue, [Artifact]; Mutation, cmd_raise_issue, cmd_raise_issue_describe;\n            "captureViewpoint" as "capture-viewpoint" => coordinate::CaptureViewpoint, [Artifact]; Mutation, cmd_capture_viewpoint, cmd_capture_viewpoint_describe;\n            "restoreViewpoint" as "restore-viewpoint" => coordinate::RestoreViewpoint, [WindowConfig]; View, cmd_restore_viewpoint, cmd_restore_viewpoint_describe;\n',
  },
  {
    marker: '"viewClash" => BimCommand::ViewClash',
    anchor: /            "selectFindings" => BimCommand::SelectFindings[^\n]*\n/,
    insert:
      '            "viewClash" => BimCommand::ViewClash(decode(action, only(fold(args, &[], &[("first", text("")), ("second", text("")), ("mode", text("zoom"))]), &["first", "second", "mode"]))?),\n            "raiseIssue" => BimCommand::RaiseIssue(decode(action, only(fold(args, &[], &[("set", text("")), ("first", text("")), ("second", text(""))]), &["set", "first", "second"]))?),\n            "captureViewpoint" => BimCommand::CaptureViewpoint(decode(action, only(fold(args, &[], &[("issue", text(""))]), &["issue"]))?),\n            "restoreViewpoint" => BimCommand::RestoreViewpoint(decode(action, only(fold(args, &[], &[("issue", text(""))]), &["issue"]))?),\n',
  },
  {
    marker: '"viewClash" => vec![',
    anchor: '        "selectFindings" => vec![ids()],\n',
    insert:
      '        "viewClash" => vec![text("first", |labels| labels.arg_first).required(), text("second", |labels| labels.arg_second).required(), text("mode", |labels| labels.arg_clash_mode)],\n        "raiseIssue" => vec![text("set", |labels| labels.arg_clash_set), text("first", |labels| labels.arg_first).required(), text("second", |labels| labels.arg_second).required()],\n        "captureViewpoint" | "restoreViewpoint" => vec![text("issue", |labels| labels.arg_issue).required()],\n',
  },
  {
    marker: "coordination_panel::CLASHES_KEY =>",
    anchor: /        diagnostics_panel::BODY_KEY => diagnostics_panel::render\([^\n]*\n/,
    insert:
      "        coordination_panel::CLASHES_KEY => coordination_panel::render_clashes(snapshot, inference, labels, &windows()),\n        coordination_panel::RULES_KEY => coordination_panel::render_rules(snapshot, inference, labels, &windows()),\n        coordination_panel::ISSUES_KEY => coordination_panel::render_issues(snapshot, labels, &windows()),\n",
  },
  {
    marker: "(labels.panel_clashes, Some(labels.clash_surface_describe))",
    anchor: /        diagnostics_panel::BODY_KEY => \(labels\.panel_diagnostics[^\n]*\n/,
    insert:
      "        coordination_panel::CLASHES_KEY => (labels.panel_clashes, Some(labels.clash_surface_describe)),\n        coordination_panel::RULES_KEY => (labels.panel_rules, Some(labels.rules_surface_describe)),\n        coordination_panel::ISSUES_KEY => (labels.panel_issues, Some(labels.issues_surface_describe)),\n",
  },
  { marker: "coordination_panel::clashes_definition()", anchor: "        .panel_tab_def(diagnostics_panel::definition())\n", insert: "        .panel_tab_def(coordination_panel::clashes_definition())\n        .panel_tab_def(coordination_panel::rules_definition())\n        .panel_tab_def(coordination_panel::issues_definition())\n" },
  {
    marker: '"bim.clash.window-required"',
    anchor: '    "bim.create.wall-type-missing" => fault_create_wall_type_missing;\n',
    insert:
      '    "bim.clash.window-required" => fault_clash_window_required;\n    "bim.clash.world-required" => fault_clash_world_required;\n    "bim.clash.mode-unknown" => fault_clash_mode_unknown;\n    "bim.clash.missing" => fault_clash_missing;\n    "bim.clash.inference" => fault_clash_inference;\n    "bim.issue.missing" => fault_issue_missing;\n    "bim.issue.viewpoint-missing" => fault_issue_viewpoint_missing;\n    "bim.create.issue-missing" => fault_create_issue_missing;\n',
  },
]);
//#endregion 🔖️EditorRoot

//#region 🔖️Checks
patch(rs(child(child(subset, "examples"), "checks")), [
  {
    marker: "crate::clash_set_problem",
    anchor: "    let present = |id: &str| owned(&id.to_string())",
    before: true,
    insert:
      '    for (id, row) in &model.clash_sets {\n        need(format!("clash_sets/{id}/definition"), crate::clash_set_problem(model, id, row).is_none());\n    }\n    for (id, row) in &model.rules {\n        need(format!("rules/{id}/definition"), crate::rule_problem(model, id, row).is_none());\n    }\n    for (id, row) in &model.issues {\n        need(format!("issues/{id}/definition"), crate::issue_problem(model, id, row).is_none());\n    }\n    for (id, row) in &model.issue_comments {\n        need(format!("issue_comments/{id}/definition"), crate::comment_problem(model, id, row).is_none());\n    }\n',
  },
  {
    marker: '("create-clash-set"',
    anchor: '    ("create-schedule", "createSchedule", "schedule", schedules),\n',
    before: true,
    insert: '    ("create-clash-set", "createClashSet", "clash_set", clash_sets),\n    ("create-rule", "createRule", "rule", rules),\n    ("create-issue", "createIssue", "issue", issues),\n    ("create-issue-comment", "createIssueComment", "issue_comment", issue_comments),\n',
  },
]);
//#endregion 🔖️Checks

//#region 🔖️CrateRoot
const crateRoot = join(artifact, "🦀️.rs");
patch(crateRoot, [
  {
    marker: "🎮️commands/🤝️coordinate/",
    anchor: mount("                ", "select_findings", `${STD}✏️editor/🎮️commands/🎯️select-findings/🦀️.rs`),
    insert: mount("                ", "coordinate", `${STD}✏️editor/🎮️commands/🤝️coordinate/🦀️.rs`),
  },
  {
    marker: "📌️panels/🤝️coordination/",
    anchor: mount("                ", "diagnostics", `${STD}✏️editor/📌️panels/🚨️diagnostics/🦀️.rs`),
    insert: mount("                ", "coordination", `${STD}✏️editor/📌️panels/🤝️coordination/🦀️.rs`),
  },
]);
//#endregion 🔖️CrateRoot

if (!process.env.DRY) for (const [path, content] of pending) writeFileSync(path, crlf.has(path) ? content.replaceAll("\n", "\r\n") : content);
console.log(notes.join("\n"));
