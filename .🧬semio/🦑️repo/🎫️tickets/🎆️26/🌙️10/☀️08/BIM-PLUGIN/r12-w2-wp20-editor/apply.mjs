import { readFileSync, writeFileSync, copyFileSync, existsSync, mkdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = process.argv[2];
const here = dirname(fileURLToPath(import.meta.url));
const S = join(root, "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any");
const E = join(S, "✏️editor");
const W = join(E, "🎭️modes/✏️edit/🪟️windows/🧊️world");
const MODEL = join(root, "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🦀️.rs");

const edit = (file, swaps, marker) => {
  let text = readFileSync(file, "utf8");
  if (text.includes(marker)) return console.log("kept  ", file.slice(-48));
  const crlf = text.includes("\r\n");
  text = text.replaceAll("\r\n", "\n");
  for (const [from, to] of swaps) {
    if (!text.includes(from)) throw new Error(file.slice(-48) + " missing: " + from.slice(0, 80));
    text = text.replace(from, () => to);
  }
  const tmp = file + ".new.tmp";
  writeFileSync(tmp, crlf ? text.replaceAll("\n", "\r\n") : text);
  copyFileSync(tmp, file);
  console.log("edited", file.slice(-48));
};

const install = (from, to) => {
  const target = join(to);
  if (existsSync(target)) return console.log("exists", target.slice(-48));
  mkdirSync(dirname(target), { recursive: true });
  copyFileSync(join(here, from), target);
  console.log("wrote ", target.slice(-48));
};

const read = (name) => readFileSync(join(here, name), "utf8").replaceAll("\r\n", "\n");

install("overlay.rs", join(S, "🖌️render/🌡️envelope/🦀️.rs"));
install("overlay-tests.rs", join(S, "🖌️render/🌡️envelope/🧪️tests/🔬️unit/🦀️.rs"));
install("entities-energy.rs", join(E, "🧩️entities/🌡️energy/🦀️.rs"));
install("entities-energy-tests.rs", join(E, "🧩️entities/🌡️energy/🧪️tests/🔬️unit/🦀️.rs"));
install("commands-conditions.rs", join(E, "🎮️commands/🌡️conditions/🦀️.rs"));
install("commands-conditions-tests.rs", join(E, "🎮️commands/🌡️conditions/🧪️tests/🔬️unit/🦀️.rs"));
install("world-envelope.rs", join(W, "🌡️envelope/🦀️.rs"));
install("world-envelope-tests.rs", join(W, "🌡️envelope/🧪️tests/🔬️unit/🦀️.rs"));

edit(join(S, "🖌️render/🦀️.rs"), [['#[path = "🗺️plan/🦀️.rs"]\npub mod plan;', '#[path = "🗺️plan/🦀️.rs"]\npub mod plan;\n#[path = "🌡️envelope/🦀️.rs"]\npub mod envelope;']], 'pub mod envelope;');

edit(MODEL, [[
  '                pub mod apply_template {\n                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️apply-template/🦀️.rs"]\n                    mod component;\n                    pub use component::*;\n                }\n',
  '                pub mod apply_template {\n                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️apply-template/🦀️.rs"]\n                    mod component;\n                    pub use component::*;\n                }\n                #[path = "."]\n                pub mod conditions {\n                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌡️conditions/🦀️.rs"]\n                    mod component;\n                    pub use component::*;\n                }\n',
]], 'pub mod conditions {');

const rowsSpace = `    field!("occupancy", field_occupancy_type, Text, |s, id| energy::condition(s, id, |c| c.occupancy.clone()), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "occupancy", &Assigned::new(energy::stated(value)))),
    field!("occupancy_density", field_occupancy_density, Number, |s, id| energy::condition(s, id, |c| c.occupancy_density.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "occupancy_density", &Assigned::new(parse_optional_number(value)?))),
    field!("heating_setpoint", field_heating_setpoint, Number, |s, id| energy::condition(s, id, |c| c.heating_setpoint.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "heating_setpoint", &Assigned::new(parse_optional_number(value)?))),
    field!("cooling_setpoint", field_cooling_setpoint, Number, |s, id| energy::condition(s, id, |c| c.cooling_setpoint.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "cooling_setpoint", &Assigned::new(parse_optional_number(value)?))),
    field!("ventilation_rate", field_ventilation_rate, Number, |s, id| energy::condition(s, id, |c| c.ventilation_rate.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "ventilation_rate", &Assigned::new(parse_optional_number(value)?))),
    field!("lighting_power_density", field_lighting_power_density, Number, |s, id| energy::condition(s, id, |c| c.lighting_power_density.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "lighting_power_density", &Assigned::new(parse_optional_number(value)?))),
    field!("equipment_power_density", field_equipment_power_density, Number, |s, id| energy::condition(s, id, |c| c.equipment_power_density.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "equipment_power_density", &Assigned::new(parse_optional_number(value)?))),
    field!("schedule", field_schedule_profile, Text, |s, id| energy::condition(s, id, |c| c.schedule.clone()), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "schedule", &Assigned::new(energy::stated(value)))),
`;
const thermal = (collection, key, label, field) => `    field!("${key}", ${label}, Number, |s, id| s.${collection}.get(id).map(|row| row.${field}.map_or_else(String::new, number)), |_, id, value| set!(set_type_thermal_data::SetTypeThermalData, id, "${key}", &Assigned::new(parse_optional_number(value)?))),\n`;
const rowsInferred = `    inferred!("envelope_surfaces", field_envelope_surfaces, |_, inference, id| energy::envelope(inference, id, |_, facts| facts.surfaces.to_string())),
    inferred!("envelope_area", field_envelope_area, |_, inference, id| energy::envelope(inference, id, |_, facts| energy::fixed(facts.envelope_area))),
    inferred!("glazing_area", field_glazing_area, |_, inference, id| energy::envelope(inference, id, |_, facts| energy::fixed(facts.glazing_area))),
    inferred!("glazing_ratio", field_glazing_ratio, |_, inference, id| energy::envelope(inference, id, |_, facts| energy::fixed(facts.glazing_ratio))),
    inferred!("open_length", field_open_length, |_, inference, id| energy::envelope(inference, id, |space, _| energy::fixed(space.open_length)).filter(|length| length != "0.00")),
    inferred!("envelope_findings", field_envelope_findings, |_, inference, id| energy::envelope(inference, id, |space, _| space.issues.len().to_string()).filter(|count| count != "0")),
    inferred!("zone_h_t_prime", field_zone_h_t_prime, |s, inference, id| energy::zone_of_space(s, inference, id, |totals| totals.h_t_prime)),
    inferred!("zone_a_over_v", field_zone_a_over_v, |s, inference, id| energy::zone_of_space(s, inference, id, |totals| totals.a_over_v)),
    inferred!("zone_envelope_area", field_zone_envelope_area, |s, inference, id| energy::zone_of_space(s, inference, id, |totals| totals.envelope_area)),
    inferred!("zone_glazing_ratio", field_zone_glazing_ratio, |s, inference, id| energy::zone_of_space(s, inference, id, |totals| totals.glazing_ratio)),
`;

edit(join(E, "🧩️entities/🦀️.rs"), [
  ['#[path = "🌀️mep/🦀️.rs"]\npub mod mep;\n', '#[path = "🌀️mep/🦀️.rs"]\npub mod mep;\n\n#[path = "🌡️energy/🦀️.rs"]\npub mod energy;\n'],
  ['choices: zoning::material_choices, write: zoning::write_ceiling_finish),\n];', 'choices: zoning::material_choices, write: zoning::write_ceiling_finish),\n' + rowsSpace + '];'],
  ['s.window_types.get(id).map(|row| row.material.clone()), parse_text => set_window_type::SetWindowType),\n];', 's.window_types.get(id).map(|row| row.material.clone()), parse_text => set_window_type::SetWindowType),\n' + thermal("window_types", "u_value", "field_u_value", "u_value") + thermal("window_types", "g_value", "field_g_value", "g_value") + thermal("window_types", "frame_fraction", "field_frame_fraction", "frame_fraction") + '];'],
  ['s.door_types.get(id).map(|row| row.material.clone()), parse_text => set_door_type::SetDoorType),\n];', 's.door_types.get(id).map(|row| row.material.clone()), parse_text => set_door_type::SetDoorType),\n' + thermal("door_types", "u_value", "field_u_value", "u_value") + '];'],
  ['zoning::finish_area(inference, id, zoning::FinishSurface::Ceiling)),\n];', 'zoning::finish_area(inference, id, zoning::FinishSurface::Ceiling)),\n' + rowsInferred + '];'],
  ['fields: BUILDING_FIELDS, inferred: &[]),', 'fields: BUILDING_FIELDS, inferred: energy::BUILDING_INFERRED),'],
], 'pub mod energy;');

edit(join(E, "🧩️entities/🏘️zoning/🦀️.rs"), [[
  'inference.zone_totals.get(id).map(|row| number(row.ceiling_finish_area))),\n];',
  `inference.zone_totals.get(id).map(|row| number(row.ceiling_finish_area))),
    inferred!("envelope_area", field_envelope_area, |s, inference, id| super::energy::zone_total(s, inference, id, |totals| totals.envelope_area)),
    inferred!("h_t_prime", field_h_t_prime, |s, inference, id| super::energy::zone_total(s, inference, id, |totals| totals.h_t_prime)),
    inferred!("a_over_v", field_a_over_v, |s, inference, id| super::energy::zone_total(s, inference, id, |totals| totals.a_over_v)),
    inferred!("glazing_ratio", field_glazing_ratio, |s, inference, id| super::energy::zone_total(s, inference, id, |totals| totals.glazing_ratio)),
];`,
]], 'super::energy::zone_total');

edit(join(E, "🗣️terminology/🦀️.rs"), [['}\n//#endregion 🔖️Labels', read("labels.txt") + '}\n//#endregion 🔖️Labels']], 'field_occupancy_type:');

edit(join(E, "🦀️.rs"), [
  ['apply_template, edit_classification,', 'apply_template, conditions, edit_classification,'],
  ['cmd_engagement_submit, cmd_engagement_submit_describe;\n', 'cmd_engagement_submit, cmd_engagement_submit_describe;\n            "applyConditions" as "apply-conditions" => conditions::ApplyConditions, [Artifact]; Mutation, cmd_apply_conditions, cmd_apply_conditions_describe;\n            "clearConditions" as "clear-conditions" => conditions::ClearConditions, [Artifact]; Mutation, cmd_clear_conditions, cmd_clear_conditions_describe;\n'],
  ['            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported")', '            "applyConditions" => BimCommand::ApplyConditions(decode(action, ids_as_list(fold(args, &[("id", "ids"), ("value", "source")], &[("ids", DslValue::Array(Vec::new())), ("source", text(""))])))?),\n            "clearConditions" => BimCommand::ClearConditions(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),\n            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported")'],
  ['"deleteSelection" | "flipWalls" | "storeyUp" | "storeyDown" => vec![ids()],', '"deleteSelection" | "flipWalls" | "storeyUp" | "storeyDown" | "clearConditions" => vec![ids()],\n        "applyConditions" => vec![ids(), text("source", |labels| labels.arg_source_space)],'],
  ['("alt+arrowup", "storeyUp"), ("alt+arrowdown", "storeyDown")];', '("alt+arrowup", "storeyUp"), ("alt+arrowdown", "storeyDown"), ("shift+h", "applyConditions"), ("alt+h", "clearConditions")];'],
  ['    "bim.template.not-applicable" => fault_template_not_applicable;\n', '    "bim.template.not-applicable" => fault_template_not_applicable;\n    "bim.conditions.source-missing" => fault_conditions_source_missing;\n    "bim.conditions.target-missing" => fault_conditions_target_missing;\n'],
], '"applyConditions" as');

edit(join(E, "🧪️tests/🔬️unit/🦀️.rs"), [[
  '        BimCommand::ApplyTemplate(apply_template::ApplyTemplate { ids: vec!["w-south".into()], template: "pt-wall".into() }),\n',
  '        BimCommand::ApplyTemplate(apply_template::ApplyTemplate { ids: vec!["w-south".into()], template: "pt-wall".into() }),\n        BimCommand::ApplyConditions(conditions::ApplyConditions { source: "sp-a".into(), ids: vec!["sp-b".into()] }),\n        BimCommand::ClearConditions(conditions::ClearConditions { ids: vec!["sp-b".into()] }),\n',
]], 'conditions::ApplyConditions');

edit(join(E, "📌️panels/🔍️properties/🦀️.rs"), [
  ['//#endregion 🔖️Actions', `/// 🌡️ The rows of a selection of spaces that state conditions: remove them, and with several spaces selected copy the first one's conditions to the others.
fn conditions_rows(snapshot: &ModelSnapshot, ids: &[&str], labels: &BimLabels) -> Vec<UiAssemblyResult<BuiltNode>> {
    if ids.is_empty() || !ids.iter().all(|id| snapshot.spaces.contains_key(*id)) || !ids.iter().any(|id| snapshot.space_conditions.contains_key(*id)) {
        return Vec::new();
    }
    let mut rows = vec![ids_args(ids).and_then(|args| tree_item_with_icon(format!("{ROOT}.action.clear-conditions"), Label::data(labels.action_clear_conditions.as_str().to_string()), "trash", bim_action("clearConditions", Some(args))))];
    if ids.len() > 1 {
        rows.push(ids_args(ids).and_then(|args| tree_item_with_icon(format!("{ROOT}.action.copy-conditions"), Label::data(labels.action_copy_conditions.as_str().to_string()), "copy", bim_action("applyConditions", Some(args)))));
    }
    rows
}
//#endregion 🔖️Actions`],
  ['        actions.extend(place_rows(snapshot, &ids, labels));\n', '        actions.extend(place_rows(snapshot, &ids, labels));\n        actions.extend(conditions_rows(snapshot, &ids, labels));\n'],
], 'fn conditions_rows');

edit(join(E, "📌️panels/🌳️outliner/🦀️.rs"), [[
  '    if hosted.is_empty() {\n        return leaf(builder);\n    }\n',
  `    if let (true, Some(conditions)) = (row.kind == "space", snapshot.space_conditions.get(id)) {
        let summary = crate::editor::bim::entities::energy::summary(conditions);
        return semio_framework_plugin::tree_window_indexed_item(windows, builder, id, false, 1, |_| leaf(item(&format!("{id}::conditions"), labels.env_conditions_row.as_str(), "thermometer", None, Some(&summary))?));
    }
    if hosted.is_empty() {
        return leaf(builder);
    }
`,
]], 'env_conditions_row');

edit(join(W, "🎚️config/🦀️.rs"), [['    framed: bool = false;\n}', '    framed: bool = false;\n    energy_overlay: bool = false;\n    energy_mode: String = "u_value".to_string();\n}']], 'energy_overlay');
const C = join(W, "🎚️config/🧬️schema");
edit(join(C, "🔗️.graphql"), [['  framed: Boolean!\n}', '  framed: Boolean!\n  energyOverlay: Boolean!\n  energyMode: String!\n}']], 'energyOverlay');
edit(join(C, "🟦️.ts"), [
  ['  framed: boolean;\n}', '  framed: boolean;\n  energyOverlay: boolean;\n  energyMode: string;\n}'],
  ['"sectionOffset", "framed"]);', '"sectionOffset", "framed", "energyOverlay", "energyMode"]);'],
  ['    framed: flag(row.framed, "$.framed"),\n', '    framed: flag(row.framed, "$.framed"),\n    energyOverlay: flag(row.energyOverlay, "$.energyOverlay"),\n    energyMode: text(row.energyMode, "$.energyMode"),\n'],
], 'energyOverlay');
edit(join(C, "🛰️.proto"), [['  bool framed = 9;\n}', '  bool framed = 9;\n  bool energy_overlay = 10;\n  string energy_mode = 11;\n}']], 'energy_overlay');
edit(join(C, "🔣️.json"), [
  ['    "sectionOffset",\n    "framed"\n  ],', '    "sectionOffset",\n    "framed",\n    "energyOverlay",\n    "energyMode"\n  ],'],
  ['    "framed": {\n      "type": "boolean"\n    }\n  },', '    "framed": {\n      "type": "boolean"\n    },\n    "energyOverlay": {\n      "type": "boolean"\n    },\n    "energyMode": {\n      "type": "string"\n    }\n  },'],
], 'energyOverlay');
edit(join(W, "🎚️config/🧪️tests/🔬️unit/🦀️.rs"), [['        framed: true,\n        ..BimWorldWindowConfig::default()', '        framed: true,\n        energy_overlay: true,\n        energy_mode: "boundary".into(),\n        ..BimWorldWindowConfig::default()']], 'energy_overlay');

edit(join(W, "🦀️.rs"), [
  ['#[path = "🎚️config/🦀️.rs"]\npub mod config;\n', '#[path = "🎚️config/🦀️.rs"]\npub mod config;\n\n#[path = "🌡️envelope/🦀️.rs"]\npub mod envelope;\n'],
  ['    let meshes: Vec<DslValue> = solids.iter().map(|(id, solid)| mesh_entry(id, snapshot, solid)).collect();\n    let instances: Vec<DslValue> = solids.iter().map(|(id, solid)| instance_entry(id, snapshot, solid, selection.contains(id), hover.contains(id))).collect();\n',
   '    let mut meshes: Vec<DslValue> = solids.iter().map(|(id, solid)| mesh_entry(id, snapshot, solid)).collect();\n    let mut instances: Vec<DslValue> = solids.iter().map(|(id, solid)| instance_entry(id, snapshot, solid, selection.contains(id), hover.contains(id))).collect();\n    let overlay = envelope::overlay(snapshot, inference, config);\n    if let Some(overlay) = &overlay {\n        meshes.extend(overlay.meshes.iter().cloned());\n        instances.extend(overlay.instances.iter().cloned());\n    }\n'],
  ['    scene.modelling_options = section_options(config);\n', '    scene.modelling_options = section_options(config);\n    if let Some(overlay) = overlay {\n        scene.annotations = overlay.annotations;\n        scene.scalar_field = overlay.scalar_field;\n    }\n'],
], 'envelope::overlay');

edit(join(E, "🎮️commands/🪟️set-view/🦀️.rs"), [['        "section_offset" => config.section_offset = number(payload)?,\n', '        "section_offset" => config.section_offset = number(payload)?,\n        "energy_overlay" => config.energy_overlay = flag(payload)?,\n        "energy_mode" => {\n            let mode = crate::render::envelope::Mode::parse(&payload.value).ok_or_else(|| fault("bim.view.value-invalid", format!("\'{}\' is not an envelope overlay mode", payload.value)))?;\n            config.energy_mode = mode.key().to_string();\n        }\n']], '"energy_overlay"');

edit(join(E, "🎛️chrome/🦀️.rs"), [
  ['        number("bim.measure.world.section-offset", labels.measure_section_offset.as_str(), config.section_offset, None, 0.1, "section_offset"),\n',
   `        number("bim.measure.world.section-offset", labels.measure_section_offset.as_str(), config.section_offset, None, 0.1, "section_offset"),
        WindowMeasure::Toggle { id: "bim.measure.world.energy".into(), icon_id: "thermometer".into(), label: Some(labels.measure_env_overlay.as_str().to_string()), pressed: config.energy_overlay, text: Some(world::envelope::legend_text(labels, labels.locale().as_str().starts_with("de"), world::envelope::mode(config))), on_change: view_action("energy_overlay") },
        select("bim.measure.world.energy-mode", labels.measure_env_mode.as_str(), world::envelope::mode(config).key(), energy_modes, "energy_mode"),
`],
  ['    let axes = vec![("x".to_string(), labels.axis_x.as_str().to_string())', '    let energy_modes = vec![("u_value".to_string(), labels.env_mode_u_value.as_str().to_string()), ("boundary".to_string(), labels.env_mode_boundary.as_str().to_string())];\n    let axes = vec![("x".to_string(), labels.axis_x.as_str().to_string())'],
], 'bim.measure.world.energy');
