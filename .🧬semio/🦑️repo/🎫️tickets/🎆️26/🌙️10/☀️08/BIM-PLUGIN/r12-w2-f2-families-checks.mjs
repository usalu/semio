#!/usr/bin/env bun
/**
 * 🧰️ Wave W2 `w2-f2-families`: teaches the example checks kit (`📚️examples/🧰️checks`) the families: the replay creates `families` and `family_solids` through `create-family` and `create-family-solid` and
 * every parameter through `set-family-parameter` (dependencies first, so each formula only names parameters that exist), and the dangling check proves that parameters and solids name a family, that every
 * formula names a parameter of its family and that a `Profile::Family` names a family of category `Profile`. Idempotent; surgical anchored insertions only.
 */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = join(import.meta.dir, "../../../../../../..");
const sub = (dir, suffix) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix)));
const plugins = sub(join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))), "plugins");
const model = sub(sub(sub(plugins, "bim"), "artifacts"), "model");
const subsets = sub(sub(sub(model, "standards"), "1"), "subsets");
const S = join(subsets, readdirSync(subsets).find((n) => n.endsWith("any")));
const path = join(sub(sub(S, "examples"), "checks"), "🦀️.rs");

let source = readFileSync(path, "utf8");
const crlf = source.includes("\r\n");
if (crlf) source = source.replaceAll("\r\n", "\n");
let applied = 0;
const edit = (marker, from, to) => {
  if (source.includes(marker)) return;
  if (!source.includes(from)) throw new Error(`anchor missing: ${from.slice(0, 80)}`);
  source = source.replace(from, to);
  applied += 1;
};

edit(
  '("create-family", "createFamily", "family", families)',
  '    ("create-leader", "createLeader", "leader", leaders),\n',
  '    ("create-leader", "createLeader", "leader", leaders),\n    ("create-family", "createFamily", "family", families),\n    ("create-family-solid", "createFamilySolid", "solid", family_solids),\n',
);
edit(
  "parameters_in_order(model, family)",
  "            state = apply_json(&state, &format!(\"{{\\\"mutation\\\":\\\"{variant}\\\",\\\"id\\\":{id},\\\"{field}\\\":{record}}}\"));\n            applied += 1;\n        }\n",
  "            state = apply_json(&state, &format!(\"{{\\\"mutation\\\":\\\"{variant}\\\",\\\"id\\\":{id},\\\"{field}\\\":{record}}}\"));\n            applied += 1;\n        }\n        if kind == \"create-family\" && KINDS.contains(&\"set-family-parameter\") {\n            for family in model.families.keys() {\n                for row in crate::mutations::family_rules::parameters_in_order(model, family) {\n                    let quoted = |text: &String| semio_framework_pack_json::to_json_string(text);\n                    state = apply_json(&state, &format!(\"{{\\\"mutation\\\":\\\"setFamilyParameter\\\",\\\"family\\\":{},\\\"name\\\":{},\\\"kind\\\":{},\\\"value\\\":{}}}\", quoted(&row.family), quoted(&row.name), semio_framework_pack_json::to_json_string(&row.kind), quoted(&row.value)));\n                    applied += 1;\n                }\n            }\n        }\n",
);
edit(
  'need(format!("family_solids/{id}/family")',
  "    for (id, schedule) in &model.schedules {\n        for storey in &schedule.storeys {",
  "    for (key, row) in &model.family_parameters {\n        need(format!(\"family_parameters/{key}/family\"), model.families.contains_key(&row.family));\n        need(format!(\"family_parameters/{key}/key\"), key == &crate::standards::v1::subsets::any::schema::inferences::families::formula::parameter_id(&row.family, &row.name));\n        need(format!(\"family_parameters/{key}/value\"), crate::standards::v1::subsets::any::schema::inferences::families::formula::references(&row.value).iter().all(|name| model.family_parameters.contains_key(&crate::standards::v1::subsets::any::schema::inferences::families::formula::parameter_id(&row.family, name))));\n    }\n    for (id, row) in &model.family_solids {\n        need(format!(\"family_solids/{id}/family\"), model.families.contains_key(&row.family));\n        need(format!(\"family_solids/{id}/formulas\"), crate::standards::v1::subsets::any::schema::inferences::families::formula::solid_slots(row).iter().all(|(_, text)| crate::standards::v1::subsets::any::schema::inferences::families::formula::references(text).iter().all(|name| model.family_parameters.contains_key(&crate::standards::v1::subsets::any::schema::inferences::families::formula::parameter_id(&row.family, name)))));\n    }\n    let profile_ok = |profile: &crate::Profile| crate::standards::v1::subsets::any::schema::inferences::families::family_of_profile(profile).is_none_or(|family| model.families.get(family).is_some_and(|row| row.category == crate::FamilyCategory::Profile));\n    for (id, kind) in &model.column_types {\n        need(format!(\"column_types/{id}/profile\"), profile_ok(&kind.profile));\n    }\n    for (id, kind) in &model.beam_types {\n        need(format!(\"beam_types/{id}/profile\"), profile_ok(&kind.profile));\n    }\n    for (id, schedule) in &model.schedules {\n        for storey in &schedule.storeys {",
);

writeFileSync(path, crlf ? source.replaceAll("\n", "\r\n") : source);
console.log(`families checks: ${applied} edits applied`);
