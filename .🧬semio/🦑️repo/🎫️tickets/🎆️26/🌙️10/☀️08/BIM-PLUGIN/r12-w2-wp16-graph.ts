#!/usr/bin/env bun
/**
 * 🕸️ Wires the `Probe`, `ClashSet` and `Rule` node kinds of WP-16 into the model graph and the two result fields into `ModelInference` (idempotent). Every edit is an anchored insertion that is read, replaced and
 * written in one go, because peers extend the same files; an anchor that is missing or ambiguous stops the script before anything is written to that file.
 */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, schema } from "./r3-f1-paths.ts";

const inferences = child(schema, "inferences");
const graph = join(inferences, readdirSync(inferences).find((name) => name.endsWith("model-graph"))!);
const file = (...parts: string[]) => {
  let dir = graph;
  for (const part of parts.slice(0, -1)) dir = join(dir, readdirSync(dir).find((name) => name.endsWith(part))!);
  return join(dir, readdirSync(dir).find((name) => name.endsWith(parts.at(-1)!))!);
};
const notes: string[] = [];
const pending = new Map<string, string>();
const load = (path: string) => pending.get(path) ?? readFileSync(path, "utf8");

type Edit = { anchor: string | RegExp; insert: (match: string) => string; after?: boolean; marker: string };

const patch = (path: string, edits: Edit[]) => {
  let text = load(path);
  for (const edit of edits) {
    if (text.includes(edit.marker)) {
      notes.push(`already: ${edit.marker.trim().slice(0, 50)}`);
      continue;
    }
    const where = typeof edit.anchor === "string" ? text.indexOf(edit.anchor) : text.search(edit.anchor);
    if (where < 0) throw new Error(`anchor not found in ${path}: ${edit.anchor}`);
    const matched = typeof edit.anchor === "string" ? edit.anchor : text.slice(where).match(edit.anchor)![0];
    const cut = where + matched.length;
    text = edit.after === false ? text.slice(0, where) + edit.insert(matched) + text.slice(where) : text.slice(0, cut) + edit.insert(matched) + text.slice(cut);
    notes.push(`edited: ${edit.marker.trim().slice(0, 50)}`);
  }
  pending.set(path, text);
};

patch(join(graph, readdirSync(graph).find((name) => name.endsWith(".rs") && name.startsWith("🦀"))!), [
  { marker: "use super::super::clash_sets::", anchor: "use super::super::families::FamilyValue;\n", insert: () => "use super::super::clash_sets::{ClashSetResult, SolidProbe};\nuse super::super::rule_results::RuleResult;\n" },
  { marker: "    Probe,\n    ClashSet,\n    Rule,\n", anchor: /pub enum NodeKind \{[\s\S]*?\n    Family,\n/, insert: () => "    Probe,\n    ClashSet,\n    Rule,\n" },
  { marker: "        Self::Probe,\n", anchor: /pub const ALL: \[NodeKind; \d+\] = \[[\s\S]*?\n        Self::Family,\n/, insert: () => "        Self::Probe,\n        Self::ClashSet,\n        Self::Rule,\n" },
  { marker: "pub const ALL: [NodeKind; ", anchor: "pub const ALL: [NodeKind; ", insert: () => "" },
  { marker: 'Self::Probe => "probe"', anchor: '            Self::Family => "family",\n', insert: () => '            Self::Probe => "probe",\n            Self::ClashSet => "clash-set",\n            Self::Rule => "rule",\n' },
  { marker: "Probe => &[Solid],", anchor: /\n            Sheet => &\[View\],\n/, insert: () => "            Probe => &[Solid],\n            ClashSet => &[Probe],\n            Rule => &[Room, StairRun, RampRun, OpeningFrame, Zone],\n" },
  { marker: "pub const CLASHES: u32", anchor: "    pub const FAMILIES: u32 = mask(NodeKind::Family);\n", insert: () => "    pub const CLASHES: u32 = mask(NodeKind::ClashSet);\n    pub const RULES: u32 = mask(NodeKind::Rule);\n" },
  { marker: "    Probe(SolidKey),\n", anchor: /pub enum ModelNode \{[\s\S]*?\n    Family\(String\),\n/, insert: () => "    Probe(SolidKey),\n    ClashSet(String),\n    Rule(String),\n" },
  { marker: "Self::Probe(_) => NodeKind::Probe", anchor: "            Self::Family(_) => NodeKind::Family,\n", insert: () => "            Self::Probe(_) => NodeKind::Probe,\n            Self::ClashSet(_) => NodeKind::ClashSet,\n            Self::Rule(_) => NodeKind::Rule,\n" },
  { marker: "    Probe(Arc<SolidProbe>),\n", anchor: "    Family(Arc<FamilyValue>),\n", insert: () => "    Probe(Arc<SolidProbe>),\n    Clashes(Arc<ClashSetResult>),\n    Rule(Arc<RuleResult>),\n" },
  { marker: "(Data::Probe(a), Data::Probe(b))", anchor: "            (Data::Family(a), Data::Family(b)) => Arc::ptr_eq(a, b),\n", insert: () => "            (Data::Probe(a), Data::Probe(b)) => Arc::ptr_eq(a, b),\n            (Data::Clashes(a), Data::Clashes(b)) => Arc::ptr_eq(a, b),\n            (Data::Rule(a), Data::Rule(b)) => Arc::ptr_eq(a, b),\n" },
  { marker: "Data::Probe(probe) =>", anchor: /            Data::Family\(family\) => [^\n]*\n/, insert: () => "            Data::Probe(probe) => probe.byte_size(),\n            Data::Clashes(found) => 128 + found.clashes.len() * 224 + found.groups.iter().map(|group| 48 + group.members.len() * 4).sum::<usize>(),\n            Data::Rule(found) => 64 + found.violations.len() * 112,\n" },
  { marker: '"clash_sets", "rules",', anchor: '"families", ', insert: () => '"clash_sets", "rules", ' },
]);

const length = (path: string) => {
  let text = load(path);
  const declared = text.match(/pub const ALL: \[NodeKind; (\d+)\]/);
  const body = text.match(/pub const ALL: \[NodeKind; \d+\] = \[([\s\S]*?)\n    \];/);
  if (!declared || !body) throw new Error("NodeKind::ALL not found");
  const count = body[1].split("\n").filter((line) => line.trim().startsWith("Self::")).length;
  if (Number(declared[1]) !== count) {
    text = text.replace(/pub const ALL: \[NodeKind; \d+\]/, `pub const ALL: [NodeKind; ${count}]`);
    pending.set(path, text);
    notes.push(`NodeKind::ALL length ${declared[1]} -> ${count}`);
  }
};
length(join(graph, readdirSync(graph).find((name) => name.endsWith(".rs") && name.startsWith("🦀"))!));

patch(file("🧭️plan", "🦀️.rs"), [
  { marker: "use super::super::super::clash_sets;", anchor: "use super::super::super::annotation_layout;\n", insert: () => "use super::super::super::clash_sets;\nuse super::super::super::rule_results;\n" },
  {
    marker: "if has(NodeKind::ClashSet) {",
    anchor: "    if has(NodeKind::Sheet) {\n        steps.extend(sheet_steps(snapshot, &steps));\n    }\n",
    insert: () => "    if has(NodeKind::ClashSet) {\n        steps.extend(clash_steps(snapshot, &steps));\n    }\n    if has(NodeKind::Rule) {\n        steps.extend(rule_steps(snapshot, &steps));\n    }\n",
  },
  {
    marker: "fn clash_steps(",
    anchor: "/// 🖼️ The `View` nodes:",
    after: false,
    insert: () => `/// 🧨️ The \`Probe\` nodes (the spatial index of every solid some clash set picks) and the \`ClashSet\` nodes (one per set, the probes of the elements it picks as parents).
fn clash_steps(snapshot: &ModelSnapshot, steps: &[InferenceStep<ModelNode>]) -> Vec<InferenceStep<ModelNode>> {
    let solids: Vec<SolidKey> = steps.iter().filter_map(|planned| if let ModelNode::Solid(key) = &planned.key { Some(key.clone()) } else { None }).collect();
    let mut probes: BTreeSet<SolidKey> = BTreeSet::new();
    let mut sets: Vec<InferenceStep<ModelNode>> = Vec::new();
    for (id, set) in &snapshot.clash_sets {
        let picked = clash_sets::needed(snapshot, set, &solids);
        probes.extend(picked.iter().map(|key| (*key).clone()));
        sets.push(step(ModelNode::ClashSet(id.clone()), picked.into_iter().map(|key| ModelNode::Probe(key.clone())).collect()));
    }
    probes.into_iter().map(|key| step(ModelNode::Probe(key.clone()), vec![ModelNode::Solid(key)])).chain(sets).collect()
}

/// ⚖️ The \`Rule\` nodes: one per rule, the planned nodes that hold the measures of its members as parents (stair runs, opening frames, ramp runs, rooms of the storeys of its spaces or zones).
fn rule_steps(snapshot: &ModelSnapshot, steps: &[InferenceStep<ModelNode>]) -> Vec<InferenceStep<ModelNode>> {
    use crate::RuleKind::{MaxCompartmentArea, MaxRampSlope, MaxRiser, MinClearHeight, MinCorridorWidth, MinDoorWidth, MinStairWidth, MinTread};
    let planned: BTreeSet<&ModelNode> = steps.iter().map(|planned| &planned.key).collect();
    snapshot
        .rules
        .iter()
        .map(|(id, rule)| {
            let members = rule_results::members(snapshot, rule);
            let nodes: BTreeSet<ModelNode> = members
                .iter()
                .filter_map(|member| match rule.kind {
                    MaxRiser | MinTread | MinStairWidth => Some(ModelNode::StairRun(member.id.clone())),
                    MinDoorWidth => Some(ModelNode::OpeningFrame(member.id.clone())),
                    MaxRampSlope => Some(ModelNode::RampRun(member.id.clone())),
                    MaxCompartmentArea => Some(ModelNode::Zone(member.id.clone())),
                    MinClearHeight | MinCorridorWidth => member.storey.clone().map(ModelNode::Room),
                })
                .filter(|node| planned.contains(node))
                .collect();
            step(ModelNode::Rule(id.clone()), nodes.into_iter().collect())
        })
        .collect()
}

`,
  },
]);

patch(file("🧮️compute", "🦀️.rs"), [
  { marker: "use super::super::super::clash_sets::", anchor: "use super::super::super::annotation_layout::{self, StoreyAnnotations};\n", insert: () => "use super::super::super::clash_sets::{self, ClashSetResult, SolidProbe};\nuse super::super::super::rule_results::{self, RuleResult};\nuse super::super::super::zones::ZoneTotals;\n" },
  { marker: "pub probes: BTreeMap", anchor: "    pub families: BTreeMap<&'a str, &'a FamilyValue>,\n", insert: () => "    pub probes: BTreeMap<&'a str, &'a SolidProbe>,\n    pub zones: BTreeMap<&'a str, &'a ZoneTotals>,\n" },
  { marker: "(ModelNode::Probe(key), Data::Probe(probe))", anchor: "                (ModelNode::Solid(key), Data::Solid(entry)) => {\n                    index.solids.insert(&key.id, entry);\n                }\n", insert: () => "                (ModelNode::Probe(key), Data::Probe(probe)) => {\n                    index.probes.insert(&key.id, probe);\n                }\n                (ModelNode::Zone(id), Data::Zone(totals)) => {\n                    index.zones.insert(id, totals);\n                }\n" },
  { marker: "ModelNode::ClashSet(id) => clash_sets::dependency", anchor: "        ModelNode::Sheet(id) => sheet_layout::dependency(snapshot, id),\n", insert: () => "        ModelNode::Probe(solid) => solid_dependency(snapshot, solid),\n        ModelNode::ClashSet(id) => clash_sets::dependency(snapshot, id),\n        ModelNode::Rule(id) => rule_results::dependency(snapshot, id),\n" },
  { marker: "ModelNode::ClashSet(id) => Data::Clashes", anchor: "        ModelNode::Sheet(id) => Data::Sheet(Arc::new(sheet_layout::layout_of(snapshot, id, &index.drawings))),\n", insert: () => "        ModelNode::Probe(key) => Data::Probe(Arc::new(probe_value(snapshot, key, &index))),\n        ModelNode::ClashSet(id) => Data::Clashes(Arc::new(clash_set_value(snapshot, id, &index))),\n        ModelNode::Rule(id) => Data::Rule(Arc::new(rule_value(snapshot, id, &index))),\n" },
  {
    marker: "fn clash_set_value(",
    anchor: "fn wall_layout_value(",
    after: false,
    insert: () => `/// 📦️ The spatial index of the solid of a \`Probe\` node: the world mesh, its box and its hierarchy; an element without a solid gets an empty probe.
fn probe_value(snapshot: &ModelSnapshot, key: &element_solids::SolidKey, index: &Index<'_>) -> SolidProbe {
    index.solids.get(key.id.as_str()).map_or_else(
        || SolidProbe { class: None, storey: String::new(), bounds: None, mesh: Default::default(), bvh: Default::default() },
        |entry| clash_sets::probe_of(snapshot, &key.id, &entry.solid),
    )
}

/// 🧨️ The clashes of a clash set among the probes of its parents.
fn clash_set_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> ClashSetResult {
    let Some(set) = snapshot.clash_sets.get(id) else { return ClashSetResult::default() };
    let (a, b) = clash_sets::picks(snapshot, set);
    let side = |ids: &[String]| -> Vec<(&str, &SolidProbe)> { ids.iter().filter_map(|element| index.probes.get(element.as_str()).map(|probe| (element.as_str(), *probe))).collect() };
    clash_sets::clashes_of(set, &side(&a), &side(&b), &clash_sets::hosting(snapshot), &|element| crate::storey_of(snapshot, element).cloned().unwrap_or_default(), &|| false)
}

/// ⚖️ The findings of a rule over the measures of its parents.
fn rule_value(snapshot: &ModelSnapshot, id: &str, index: &Index<'_>) -> RuleResult {
    let Some(rule) = snapshot.rules.get(id) else { return RuleResult::default() };
    let inputs = rule_results::Inputs { rooms: index.rooms.clone(), runs: index.runs.clone(), ramp_runs: index.ramp_runs.clone(), frames: index.frames.clone(), zones: index.zones.clone() };
    rule_results::result_of(rule, &rule_results::members(snapshot, rule), &inputs)
}

`,
  },
]);


patch(file("🪞️projection", "🦀️.rs"), [
  { marker: "(ModelNode::ClashSet(id), Data::Clashes(found))", anchor: "        (ModelNode::Family(id), Data::Family(family)) => {\n", after: false, insert: () => "        (ModelNode::ClashSet(id), Data::Clashes(found)) => {\n            inference.clash_sets.insert(id, Arc::unwrap_or_clone(found));\n        }\n        (ModelNode::Rule(id), Data::Rule(found)) => {\n            inference.rule_results.insert(id, Arc::unwrap_or_clone(found));\n        }\n" },
  { marker: "inference.clash_sets.remove(id)", anchor: "        (ModelNode::Family(id), _) => {\n", after: false, insert: () => "        (ModelNode::ClashSet(id), _) => {\n            inference.clash_sets.remove(id);\n        }\n        (ModelNode::Rule(id), _) => {\n            inference.rule_results.remove(id);\n        }\n" },
]);

patch(file("🎯️dirty", "🦀️.rs"), [
  { marker: "ModelNode::Probe(solid_key)", anchor: '        ModelNode::Sheet(id) => regions.row("sheets", id)', after: false, insert: () => "        ModelNode::Probe(solid_key) => solid(&regions, snapshot, solid_key),\n" },
]);

const root = join(inferences, readdirSync(inferences).find((name) => name.endsWith(".rs") && name.startsWith("🦀"))!);
patch(root, [
  { marker: "use super::clash_sets::", anchor: "use super::sheet_layout::SheetLayout;\n", insert: () => "use super::clash_sets::ClashSetResult;\nuse super::rule_results::RuleResult;\n" },
  { marker: "pub clash_sets: BTreeMap", anchor: "    pub effective_properties: BTreeMap<String, EffectiveProperties>,\n", insert: () => "    #[derived]\n    pub clash_sets: BTreeMap<String, ClashSetResult>,\n    #[derived]\n    pub rule_results: BTreeMap<String, RuleResult>,\n" },
  { marker: "inference.clash-sets", anchor: /            protocol::InferenceFieldSpec \{ id: "s.bim.model.inference.sheet-layout"[^\n]*\n/, insert: () => '            protocol::InferenceFieldSpec { id: "s.bim.model.inference.clash-sets", reads: super::clash_sets::READS },\n            protocol::InferenceFieldSpec { id: "s.bim.model.inference.rule-results", reads: super::rule_results::READS },\n' },
]);

for (const [path, text] of pending) writeFileSync(path, text);
console.log(notes.join("\n"));
