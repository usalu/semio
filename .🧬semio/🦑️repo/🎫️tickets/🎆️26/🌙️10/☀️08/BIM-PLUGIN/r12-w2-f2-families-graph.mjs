#!/usr/bin/env bun
/**
 * 🕸️ Wave W2 `w2-f2-families`: joins the family inference to the model graph with small anchored insertions into the shared files (idempotent: an edit whose marker is present is skipped, nothing is
 * rewritten): the `Family` node kind, key, data and mask, its plan step (no parents), the family parents of the solids of columns, beams, curtain walls and railings and of the model diagnostics, the compute
 * arms and findings, the projection into `ModelInference.families`, the dirty rule, the diagnostic codes with their texts, the solid entry points that take family outlines, the inference aggregate field and
 * the `families` table of the text projection. Usage: `bun r12-w2-f2-families-graph.mjs`.
 */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = join(import.meta.dir, "../../../../../../..");
const sub = (dir, suffix) => join(dir, readdirSync(dir).find((name) => name.endsWith(suffix)));
const plugins = sub(join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))), "plugins");
const model = sub(sub(sub(plugins, "bim"), "artifacts"), "model");
const subsets = sub(sub(sub(model, "standards"), "1"), "subsets");
const S = join(subsets, readdirSync(subsets).find((n) => n.endsWith("any")));
const I = sub(sub(S, "schema"), "inferences");
const G = sub(I, "model-graph");
const RS = "🦀️.rs";
const solids = sub(I, "element-solids");

let applied = 0;
function patch(path, edits) {
  let source = readFileSync(path, "utf8");
  const crlf = source.includes("\r\n");
  if (crlf) source = source.replaceAll("\r\n", "\n");
  for (const { marker, from, regex, to } of edits) {
    if (source.includes(marker)) continue;
    if (regex) {
      if (!regex.test(source)) throw new Error(`${path.slice(-70)}: pattern ${regex} not found`);
      source = source.replace(regex, to);
    } else {
      if (!source.includes(from)) throw new Error(`${path.slice(-70)}: anchor missing: ${from.slice(0, 80)}`);
      source = source.replace(from, to);
    }
    applied += 1;
  }
  writeFileSync(path, crlf ? source.replaceAll("\n", "\r\n") : source);
}
const fileOf = (dir, suffix) => join(sub(dir, suffix), RS);

//#region 🔖️Graph
patch(join(G, RS), [
  { marker: "families::FamilyValue", from: "use super::super::element_solids::{SolidEntry, SolidKey};\n", to: "use super::super::element_solids::{SolidEntry, SolidKey};\nuse super::super::families::FamilyValue;\n" },
  { marker: "    Properties,\n    Family,\n}", from: "    Surface,\n    Properties,\n}\n\nimpl NodeKind {", to: "    Surface,\n    Properties,\n    Family,\n}\n\nimpl NodeKind {" },
  { marker: "        Self::Family,\n    ];", regex: /pub const ALL: \[NodeKind; (\d+)\] = \[([\s\S]*?)\n    \];/, to: (_, count, rows) => `pub const ALL: [NodeKind; ${Number(count) + 1}] = [${rows}\n        Self::Family,\n    ];` },
  { marker: 'Self::Family => "family"', from: '            Self::Properties => "properties",\n', to: '            Self::Properties => "properties",\n            Self::Family => "family",\n' },
  { marker: "| Family => &[]", from: "Storey | Band | Cut | PhaseVisibility => &[],", to: "Storey | Band | Cut | PhaseVisibility | Family => &[]," },
  { marker: "RampRun, Family],", regex: /(            Solid => &\[[^\]\n]*?RampRun)(\],)/, to: "$1, Family$2" },
  { marker: "Solid, Room, Family],", regex: /(            Quantity => &\[[^\]\n]*?Solid, Room)(\],)/, to: "$1, Family$2" },
  { marker: "Properties, Family],", regex: /(            Diagnostics => &\[[^\]\n]*?Properties)(\],)/, to: "$1, Family$2" },
  { marker: "pub const FAMILIES", from: "    pub const PROPERTIES: u32 = mask(NodeKind::Properties);\n", to: "    pub const PROPERTIES: u32 = mask(NodeKind::Properties);\n    pub const FAMILIES: u32 = mask(NodeKind::Family);\n" },
  { marker: "    Properties(String),\n    Family(String),\n}", from: "    Surface(String),\n    Properties(String),\n}", to: "    Surface(String),\n    Properties(String),\n    Family(String),\n}" },
  { marker: "Self::Family(_) => NodeKind::Family", from: "            Self::Properties(_) => NodeKind::Properties,\n", to: "            Self::Properties(_) => NodeKind::Properties,\n            Self::Family(_) => NodeKind::Family,\n" },
  { marker: "    Family(Arc<FamilyValue>),", from: "    Properties(Arc<EffectiveProperties>),\n}", to: "    Properties(Arc<EffectiveProperties>),\n    Family(Arc<FamilyValue>),\n}" },
  { marker: "(Data::Family(a), Data::Family(b))", from: "            (Data::Properties(a), Data::Properties(b)) => Arc::ptr_eq(a, b),\n", to: "            (Data::Properties(a), Data::Properties(b)) => Arc::ptr_eq(a, b),\n            (Data::Family(a), Data::Family(b)) => Arc::ptr_eq(a, b),\n" },
  { marker: "Data::Family(family) =>", from: "            Data::Phases(phases) =>", to: "            Data::Family(family) => family.solids.values().map(|solid| 160 + 8 * (solid.positions.len() + solid.normals.len()) + 4 * solid.indices.len()).sum::<usize>() + family.parameters.len() * 128 + family.issues.len() * 192 + family.outline.len() * 32,\n            Data::Phases(phases) =>" },
  { marker: '"families", "family_parameters", "family_solids"', from: "pub const READS: &[&str] = &[\n    ", to: 'pub const READS: &[&str] = &[\n    "families", "family_parameters", "family_solids", ' },
]);
//#endregion 🔖️Graph

//#region 🔖️Plan
patch(fileOf(G, "plan"), [
  { marker: "use super::super::super::families;", from: "use super::super::super::effective_properties;\n", to: "use super::super::super::effective_properties;\nuse super::super::super::families;\n" },
  { marker: "if has(NodeKind::Family)", from: "    if has(NodeKind::Solid) {\n        for (id, wall) in &walls {", to: "    if has(NodeKind::Family) {\n        steps.extend(snapshot.families.keys().map(|id| step(ModelNode::Family(id.clone()), Vec::new())));\n    }\n    if has(NodeKind::Solid) {\n        for (id, wall) in &walls {" },
  { marker: "family_parents(snapshot, snapshot.column_types", from: "steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Column, id)), storeys_of(snapshot, &column.storey, Some(&column.top))));", to: "steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Column, id)), [storeys_of(snapshot, &column.storey, Some(&column.top)), family_parents(snapshot, snapshot.column_types.get(&column.column_type).map(|kind| vec![&kind.profile]).unwrap_or_default())].concat()));" },
  { marker: "family_parents(snapshot, snapshot.beam_types", from: "steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Beam, id)), parents));", to: "parents.extend(family_parents(snapshot, snapshot.beam_types.get(&beam.beam_type).map(|kind| vec![&kind.profile]).unwrap_or_default()));\n            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Beam, id)), parents));" },
  { marker: "family_parents(snapshot, snapshot.curtain_wall_types", from: "steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::CurtainWall, id)), parents));", to: "parents.extend(family_parents(snapshot, snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).map(|kind| vec![&kind.interior_mullion, &kind.border_mullion]).unwrap_or_default()));\n            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::CurtainWall, id)), parents));" },
  { marker: "family_parents(snapshot, railing_profiles(railing))", from: "steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Railing, id)), railing_parents(snapshot, railing)));", to: "steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Railing, id)), [railing_parents(snapshot, railing), family_parents(snapshot, railing_profiles(railing))].concat()));" },
  { marker: "DiagnosticScope::Model), snapshot.families", from: "steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Model), Vec::new()));", to: "steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Model), snapshot.families.keys().map(|id| ModelNode::Family(id.clone())).collect()));" },
  { marker: "pub fn family_parents", from: "/// 🪝️ The nodes the solid of a railing is computed from", to: "/// 🧩️ The family nodes of the profile families the given profiles name (only families that exist).\npub fn family_parents(snapshot: &ModelSnapshot, profiles: Vec<&crate::Profile>) -> Vec<ModelNode> {\n    let named: BTreeSet<&str> = profiles.into_iter().filter_map(families::family_of_profile).filter(|id| snapshot.families.contains_key(*id)).collect();\n    named.into_iter().map(|id| ModelNode::Family(id.to_string())).collect()\n}\n\n/// 🛤️ The rail, post and baluster sections of a railing.\npub fn railing_profiles(railing: &crate::Railing) -> Vec<&crate::Profile> {\n    [Some(&railing.profile), Some(&railing.post_profile), railing.baluster.as_ref().map(|row| &row.profile)].into_iter().flatten().collect()\n}\n\n/// 🪝️ The nodes the solid of a railing is computed from" },
]);
//#endregion 🔖️Plan

//#region 🔖️Compute
patch(fileOf(G, "compute"), [
  { marker: "families::{self, FamilyProfiles", from: "use super::super::super::phase_visibility;\n", to: "use super::super::super::families::{self, FamilyProfiles, FamilyValue};\nuse super::super::super::phase_visibility;\n" },
  { marker: "pub families: BTreeMap", from: "    pub solids: BTreeMap<&'a str, &'a SolidEntry>,\n", to: "    pub solids: BTreeMap<&'a str, &'a SolidEntry>,\n    pub families: BTreeMap<&'a str, &'a FamilyValue>,\n" },
  { marker: "(ModelNode::Family(id), Data::Family(value))", from: "                (ModelNode::Solid(key), Data::Solid(entry)) => {", to: "                (ModelNode::Family(id), Data::Family(value)) => {\n                    index.families.insert(id, value);\n                }\n                (ModelNode::Solid(key), Data::Solid(entry)) => {" },
  { marker: "pub fn profiles(&self)", from: "    fn level(&self, storey: &str) -> StoreyLevel {", to: "    /// ▭️ The outlines of the profile families among the parents.\n    pub fn profiles(&self) -> FamilyProfiles<'_> {\n        self.families.iter().fold(FamilyProfiles::new(), |profiles, (id, family)| profiles.with(id, &family.outline))\n    }\n\n    fn level(&self, storey: &str) -> StoreyLevel {" },
  { marker: "ModelNode::Family(id) => families::dependency", from: "        ModelNode::Properties(id) => effective_properties::dependency(snapshot, id),\n", to: "        ModelNode::Properties(id) => effective_properties::dependency(snapshot, id),\n        ModelNode::Family(id) => families::dependency(snapshot, id),\n" },
  { marker: "families::findings::reference_dependency", from: "ModelNode::Diagnostics(DiagnosticScope::Model) => diagnostics::model_dependency(snapshot),", to: "ModelNode::Diagnostics(DiagnosticScope::Model) => dep_object([(\"model\", diagnostics::model_dependency(snapshot)), (\"profiles\", families::findings::reference_dependency(snapshot))])," },
  { marker: "Data::Family(Arc::new(families::family_of", from: "        ModelNode::PhaseVisibility(storey) => Data::Phases(", to: "        ModelNode::Family(id) => Data::Family(Arc::new(families::family_of(snapshot, id, &BTreeMap::new()))),\n        ModelNode::PhaseVisibility(storey) => Data::Phases(" },
  { marker: "let profiles = index.profiles();", from: "    let target = |top: &TopConstraint| index.target(&storey, top);\n    let built: Option<SolidEntry> = match solid.family {", to: "    let target = |top: &TopConstraint| index.target(&storey, top);\n    let profiles = index.profiles();\n    let built: Option<SolidEntry> = match solid.family {" },
  { marker: "columns::column_solid_in(", from: "columns::column_solid(snapshot, column, &own, target(&column.top).as_ref())", to: "columns::column_solid_in(snapshot, column, &own, target(&column.top).as_ref(), &profiles)" },
  { marker: "beams::beam_solid_in(", from: "beams::beam_solid(snapshot, beam, &own, &joiners)", to: "beams::beam_solid_in(snapshot, beam, &own, &joiners, &profiles)" },
  { marker: "curtain_walls::curtain_solid_in(", from: "curtain_walls::curtain_solid(snapshot, curtain, layout, &cuts)", to: "curtain_walls::curtain_solid_in(snapshot, curtain, layout, &cuts, &profiles)" },
  { marker: "profiles.resolve_railing(railing)", from: "snapshot.railings.get(id).map(|railing| SolidEntry { solid: if railing.host.is_some() { rail_hosts::hosted_solid(railing, host_of(snapshot, railing, index).as_ref()) } else { railings::railing_solid(snapshot, railing, &own) }, fallback: None }),", to: "snapshot.railings.get(id).map(|railing| profiles.resolve_railing(railing)).map(|railing| SolidEntry { solid: if railing.host.is_some() { rail_hosts::hosted_solid(&railing, host_of(snapshot, &railing, index).as_ref()) } else { railings::railing_solid(snapshot, &railing, &own) }, fallback: None })," },
  { marker: "family_findings(snapshot, index)", from: "        DiagnosticScope::Model => diagnostics::model_findings(&diagnostics::references::ReferenceView::of(snapshot)),", to: "        DiagnosticScope::Model => {\n            let mut found = diagnostics::model_findings(&diagnostics::references::ReferenceView::of(snapshot));\n            found.extend(family_findings(snapshot, index));\n            found\n        }" },
  { marker: "fn family_findings(", from: "//#endregion 🔖️Value\n", to: "/// 🧬️ The findings of the families: their issues and the dangling profile references.\nfn family_findings(snapshot: &ModelSnapshot, index: &Index<'_>) -> Vec<diagnostics::Diagnostic> {\n    use diagnostics::DiagnosticCode as Code;\n    use families::findings::FindingCode;\n    use families::FamilyIssueCode as Issue;\n    let issues = families::findings::issue_findings(&index.families);\n    let references = families::findings::reference_findings(snapshot, &index.families);\n    issues\n        .into_iter()\n        .chain(references)\n        .map(|finding| {\n            let code = match finding.code {\n                FindingCode::ProfileReference => Code::RefProfileFamily,\n                FindingCode::Issue(Issue::Syntax) => Code::FamilySyntax,\n                FindingCode::Issue(Issue::Kind) => Code::FamilyKind,\n                FindingCode::Issue(Issue::Cycle) => Code::FamilyCycle,\n                FindingCode::Issue(Issue::Unknown) => Code::FamilyUnknown,\n                FindingCode::Issue(Issue::DivisionByZero) => Code::FamilyDivisionByZero,\n                FindingCode::Issue(Issue::Negative) => Code::FamilyNegative,\n                FindingCode::Issue(Issue::Dependency) => Code::FamilyDependency,\n                FindingCode::Issue(Issue::Domain) => Code::FamilyDomain,\n                FindingCode::Issue(Issue::Outline) => Code::FamilyOutline,\n            };\n            let elements: Vec<&str> = finding.elements.iter().map(String::as_str).collect();\n            finding.missing.iter().fold(diagnostics::Diagnostic::new(code, &elements), |found, missing| found.lacking(missing))\n        })\n        .collect()\n}\n//#endregion 🔖️Value\n" },
]);
//#endregion 🔖️Compute

//#region 🔖️Projection
patch(fileOf(G, "projection"), [
  { marker: "inference.families.insert(id", from: "        (ModelNode::PhaseVisibility(storey), Data::Phases(visibility)) => {", to: "        (ModelNode::Family(id), Data::Family(family)) => {\n            inference.families.insert(id, Arc::unwrap_or_clone(family));\n        }\n        (ModelNode::PhaseVisibility(storey), Data::Phases(visibility)) => {" },
  { marker: "inference.families.remove(id)", from: "        (ModelNode::PhaseVisibility(storey), _) => {", to: "        (ModelNode::Family(id), _) => {\n            inference.families.remove(id);\n        }\n        (ModelNode::PhaseVisibility(storey), _) => {" },
]);
patch(fileOf(G, "dirty"), [
  { marker: "ModelNode::Family(id) =>", from: "        ModelNode::Totals(_) | ModelNode::DiagnosticIndex => false,", to: "        ModelNode::Family(id) => regions.row(\"families\", id) || regions.collection(\"family_parameters\") || regions.collection(\"family_solids\") || regions.collection(\"materials\"),\n        ModelNode::Totals(_) | ModelNode::DiagnosticIndex => false," },
]);
//#endregion 🔖️Projection

//#region 🔖️Aggregate
patch(join(I, RS), [
  { marker: "use super::families::FamilyValue;", from: "use super::annotation_layout::StoreyAnnotations;\n", to: "use super::annotation_layout::StoreyAnnotations;\nuse super::families::FamilyValue;\n" },
  { marker: "pub families: BTreeMap<String, FamilyValue>", from: "    #[derived]\n    pub ramp_runs: BTreeMap<String, RampRun>,\n", to: "    #[derived]\n    pub ramp_runs: BTreeMap<String, RampRun>,\n    #[derived]\n    pub families: BTreeMap<String, FamilyValue>,\n" },
  { marker: "inference.families\"", regex: /(            protocol::InferenceFieldSpec \{ id: "s\.bim\.model\.inference\.ramp-runs"[^\n]*\n)/, to: '$1            protocol::InferenceFieldSpec { id: "s.bim.model.inference.families", reads: super::families::READS },\n' },
]);
//#endregion 🔖️Aggregate

//#region 🔖️Diagnostics
const D = sub(I, "diagnostics");
const codes = ["FamilySyntax", "FamilyKind", "FamilyCycle", "FamilyUnknown", "FamilyDivisionByZero", "FamilyNegative", "FamilyDependency", "FamilyDomain", "FamilyOutline", "RefProfileFamily"];
patch(join(D, RS), [
  { marker: "    FamilySyntax,\n", regex: /(pub enum DiagnosticCode \{[\s\S]*?)(\n\}\n)/, to: `$1\n${codes.map((code) => `    ${code},`).join("\n")}$2` },
  { marker: "Self::FamilySyntax,", regex: /(    pub const ALL: &'static \[DiagnosticCode\] = &\[[\s\S]*?)(\n    \];)/, to: `$1\n        ${codes.map((code) => `Self::${code}`).join(", ")},$2` },
]);
const row = (code, slug, level, en, de) => `        ${code} => row("${slug}", ${level}, ${JSON.stringify(en)}, ${JSON.stringify(de)}),\n`;
const rows = [
  row("FamilySyntax", "family.syntax", "Error", "A formula of family {elements} does not parse.", "Eine Formel der Familie {elements} lässt sich nicht lesen."),
  row("FamilyKind", "family.kind", "Error", "A formula of family {elements} mixes kinds or computes the wrong kind.", "Eine Formel der Familie {elements} mischt Größenarten oder liefert die falsche Art."),
  row("FamilyCycle", "family.cycle", "Error", "Parameters of family {elements} depend on each other in a circle: {missing}.", "Parameter der Familie {elements} hängen im Kreis voneinander ab: {missing}."),
  row("FamilyUnknown", "family.unknown", "Error", "A formula of family {elements} uses the unknown name {missing}.", "Eine Formel der Familie {elements} verwendet den unbekannten Namen {missing}."),
  row("FamilyDivisionByZero", "family.division-by-zero", "Error", "A formula of family {elements} divides by zero.", "Eine Formel der Familie {elements} teilt durch null."),
  row("FamilyNegative", "family.negative", "Error", "A dimension of family {elements} is not positive.", "Eine Abmessung der Familie {elements} ist nicht positiv."),
  row("FamilyDependency", "family.dependency", "Warning", "A formula of family {elements} depends on {missing}, which has no value.", "Eine Formel der Familie {elements} hängt von {missing} ab, das keinen Wert hat."),
  row("FamilyDomain", "family.domain", "Error", "A formula of family {elements} has no finite value.", "Eine Formel der Familie {elements} hat keinen endlichen Wert."),
  row("FamilyOutline", "family.outline", "Warning", "The outline or section of family {elements} is not usable.", "Die Kontur oder der Schnitt der Familie {elements} ist nicht brauchbar."),
  row("RefProfileFamily", "reference.profile-family", "Error", "{elements} uses the profile family {missing}, which does not exist or is no profile family.", "{elements} verwendet die Profilfamilie {missing}, die nicht existiert oder keine Profilfamilie ist."),
].join("");
patch(fileOf(D, "messages"), [{ marker: "FamilySyntax => row(", from: "        ClashWallWall => row(", to: `${rows}        ClashWallWall => row(` }]);
//#endregion 🔖️Diagnostics

//#region 🔖️Solids
patch(fileOf(solids, "columns"), [
  { marker: "pub fn column_solid_in(", from: "pub fn column_solid(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>) -> ElementSolid {\n    let mut builder = SolidBuilder::new(SolidFamily::Column);\n    if let Some(kind) = snapshot.column_types.get(&column.column_type) {\n        builder.add(parts::BODY, &kind.material, 0, &column_geometry(column, &kind.profile, own, target).mesh);\n    }\n    builder.build()\n}", to: "pub fn column_solid(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>) -> ElementSolid {\n    column_solid_in(snapshot, column, own, target, &FamilyProfiles::new())\n}\n\n/// 🏛️ [`column_solid`] where a type that names a profile family gets that family's outline from `profiles`.\npub fn column_solid_in(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>, profiles: &FamilyProfiles<'_>) -> ElementSolid {\n    let mut builder = SolidBuilder::new(SolidFamily::Column);\n    if let Some(kind) = snapshot.column_types.get(&column.column_type) {\n        builder.add(parts::BODY, &kind.material, 0, &column_geometry(column, &profiles.resolve(&kind.profile), own, target).mesh);\n    }\n    builder.build()\n}" },
  { marker: "families::FamilyProfiles", from: "use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{", to: "use crate::standards::v1::subsets::any::schema::inferences::families::FamilyProfiles;\nuse crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::{" },
]);
patch(fileOf(solids, "beams"), [
  { marker: "pub fn beam_solid_in(", from: "pub fn beam_solid(snapshot: &ModelSnapshot, beam: &Beam, own: &StoreyLevel, joiners: &[Joiner<'_>]) -> ElementSolid {\n    let mut builder = SolidBuilder::new(SolidFamily::Beam);\n    if let Some(kind) = snapshot.beam_types.get(&beam.beam_type) {\n        builder.add(parts::BODY, &kind.material, 0, &beam_geometry(beam, &kind.profile, own, joiners).mesh);\n    }\n    builder.build()\n}", to: "pub fn beam_solid(snapshot: &ModelSnapshot, beam: &Beam, own: &StoreyLevel, joiners: &[Joiner<'_>]) -> ElementSolid {\n    beam_solid_in(snapshot, beam, own, joiners, &FamilyProfiles::new())\n}\n\n/// ➖️ [`beam_solid`] where a type that names a profile family gets that family's outline from `profiles`.\npub fn beam_solid_in(snapshot: &ModelSnapshot, beam: &Beam, own: &StoreyLevel, joiners: &[Joiner<'_>], profiles: &FamilyProfiles<'_>) -> ElementSolid {\n    let mut builder = SolidBuilder::new(SolidFamily::Beam);\n    if let Some(kind) = snapshot.beam_types.get(&beam.beam_type) {\n        builder.add(parts::BODY, &kind.material, 0, &beam_geometry(beam, &profiles.resolve(&kind.profile), own, joiners).mesh);\n    }\n    builder.build()\n}" },
  { marker: "families::FamilyProfiles", from: "use crate::standards::v1::subsets::any::schema::inferences::element_solids::columns::{", to: "use crate::standards::v1::subsets::any::schema::inferences::families::FamilyProfiles;\nuse crate::standards::v1::subsets::any::schema::inferences::element_solids::columns::{" },
]);
patch(fileOf(solids, "curtain-walls"), [
  { marker: "pub fn curtain_solid_in(", from: "pub fn curtain_solid(snapshot: &ModelSnapshot, curtain: &CurtainWall, layout: &CurtainLayout, cuts: &[OpeningCut]) -> ElementSolid {\n", to: "pub fn curtain_solid(snapshot: &ModelSnapshot, curtain: &CurtainWall, layout: &CurtainLayout, cuts: &[OpeningCut]) -> ElementSolid {\n    curtain_solid_in(snapshot, curtain, layout, cuts, &FamilyProfiles::new())\n}\n\n/// 🧊️ [`curtain_solid`] where a type whose mullions name a profile family gets that family's outline from `profiles`.\npub fn curtain_solid_in(snapshot: &ModelSnapshot, curtain: &CurtainWall, layout: &CurtainLayout, cuts: &[OpeningCut], profiles: &FamilyProfiles<'_>) -> ElementSolid {\n" },
  { marker: "profile_polygon(&profiles.resolve(&kind.border_mullion))", from: "let (border, interior) = (profile_polygon(&kind.border_mullion), profile_polygon(&kind.interior_mullion));", to: "let (border, interior) = (profile_polygon(&profiles.resolve(&kind.border_mullion)), profile_polygon(&profiles.resolve(&kind.interior_mullion)));" },
  { marker: "families::FamilyProfiles", from: "use super::super::{dep_object, dep_types, dep_value, parts, profile_extents, profile_polygon, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};\n", to: "use super::super::{dep_object, dep_types, dep_value, parts, profile_extents, profile_polygon, ElementSolid, SolidBuilder, SolidFamily, CHORD_TOLERANCE};\nuse crate::standards::v1::subsets::any::schema::inferences::families::FamilyProfiles;\n" },
]);
//#endregion 🔖️Solids

//#region 🔖️Quantities
patch(join(G, RS), [{ marker: "Solid, Room, Family],", regex: /(            Quantity => &\[[^\]\n]*?Solid, Room)(\],)/, to: "$1, Family$2" }]);
patch(fileOf(G, "plan"), [
  { marker: "quantities.push((id.clone(), [storeys_of(snapshot, &column.storey, Some(&column.top)), family_parents(", from: "quantities.push((id.clone(), storeys_of(snapshot, &column.storey, Some(&column.top))));", to: "quantities.push((id.clone(), [storeys_of(snapshot, &column.storey, Some(&column.top)), family_parents(snapshot, snapshot.column_types.get(&column.column_type).map(|kind| vec![&kind.profile]).unwrap_or_default())].concat()));" },
  { marker: "[vec![ModelNode::Solid(SolidKey::of(SolidFamily::Beam, id))], family_parents(", from: "quantities.push((id.clone(), if on_storey(&beam.storey) { vec![ModelNode::Solid(SolidKey::of(SolidFamily::Beam, id))] } else { Vec::new() }));", to: "quantities.push((id.clone(), if on_storey(&beam.storey) { [vec![ModelNode::Solid(SolidKey::of(SolidFamily::Beam, id))], family_parents(snapshot, snapshot.beam_types.get(&beam.beam_type).map(|kind| vec![&kind.profile]).unwrap_or_default())].concat() } else { Vec::new() }));" },
]);
patch(fileOf(G, "compute"), [
  { marker: "quantities::column_quantity_in(", from: "quantities::column_quantity(snapshot, column, &index.level(&column.storey), index.target(&column.storey, &column.top).as_ref())", to: "quantities::column_quantity_in(snapshot, column, &index.level(&column.storey), index.target(&column.storey, &column.top).as_ref(), &index.profiles())" },
  { marker: "quantities::beam_quantity_in(", from: "quantities::beam_quantity(snapshot, beam, solid(SolidFamily::Beam))", to: "quantities::beam_quantity_in(snapshot, beam, solid(SolidFamily::Beam), &index.profiles())" },
]);
patch(fileOf(I, "quantities"), [
  { marker: "families::FamilyProfiles", from: "use super::super::finishes::{self, FinishQuantity};\n", to: "use super::super::families::FamilyProfiles;\nuse super::super::finishes::{self, FinishQuantity};\n" },
  { marker: "pub fn column_quantity_in(", from: "pub fn column_quantity(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>) -> Option<ElementQuantity> {\n    let kind = snapshot.column_types.get(&column.column_type)?;\n    let (base_z, top_z) = vertical_of(column.base_offset, &column.top, own, target);\n    let outline = profile_loop(&kind.profile);", to: "pub fn column_quantity(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>) -> Option<ElementQuantity> {\n    column_quantity_in(snapshot, column, own, target, &FamilyProfiles::new())\n}\n\n/// 🏛️ [`column_quantity`] where a type that names a profile family gets that family's outline from `profiles`.\npub fn column_quantity_in(snapshot: &ModelSnapshot, column: &Column, own: &StoreyLevel, target: Option<&StoreyLevel>, profiles: &FamilyProfiles<'_>) -> Option<ElementQuantity> {\n    let kind = snapshot.column_types.get(&column.column_type)?;\n    let (base_z, top_z) = vertical_of(column.base_offset, &column.top, own, target);\n    let outline = profile_loop(&profiles.resolve(&kind.profile));" },
  { marker: "pub fn beam_quantity_in(", from: "pub fn beam_quantity(snapshot: &ModelSnapshot, beam: &Beam, solid: Option<&ElementSolid>) -> Option<ElementQuantity> {\n    let kind = snapshot.beam_types.get(&beam.beam_type)?;\n    let outline = profile_loop(&kind.profile);", to: "pub fn beam_quantity(snapshot: &ModelSnapshot, beam: &Beam, solid: Option<&ElementSolid>) -> Option<ElementQuantity> {\n    beam_quantity_in(snapshot, beam, solid, &FamilyProfiles::new())\n}\n\n/// ➖️ [`beam_quantity`] where a type that names a profile family gets that family's outline from `profiles`.\npub fn beam_quantity_in(snapshot: &ModelSnapshot, beam: &Beam, solid: Option<&ElementSolid>, profiles: &FamilyProfiles<'_>) -> Option<ElementQuantity> {\n    let kind = snapshot.beam_types.get(&beam.beam_type)?;\n    let outline = profile_loop(&profiles.resolve(&kind.profile));" },
]);
//#endregion 🔖️Quantities

//#region 🔖️Projection json
const text = join(sub(sub(S, "io"), "text"), readdirSync(sub(sub(S, "io"), "text")).find((n) => n.endsWith("snapshot")), RS);
patch(text, [
  { marker: '"families" =>', from: '        "annotations" =>', to: '        "families" => Some(crate::standards::v1::subsets::any::schema::inferences::families::metrics::table_json(&inferred.families)),\n        "annotations" =>' },
]);
//#endregion 🔖️Projection json

console.log(`w2-f2-families graph: ${applied} edits applied`);
