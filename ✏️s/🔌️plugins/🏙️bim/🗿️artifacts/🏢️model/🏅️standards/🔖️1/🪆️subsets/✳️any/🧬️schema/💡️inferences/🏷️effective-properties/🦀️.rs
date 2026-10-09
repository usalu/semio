//! 🏷️ `effective-properties`: the properties an element or type really has. Authored data is only what was typed onto an element or onto its type (`properties`, keyed by element or type id) and the property set templates of the library
//! (`property_templates`); what an element has is derived:
//! * a value the element states itself (source [`Source::Own`]) wins; else the value its type states or gets by default (source [`Source::Type`], inheritance is a graph edge: the node of an instance has the node of its type as parent); else
//!   the default of a template that applies to the kind of the element (source [`Source::Default`]);
//! * a template applies to an element when its kind is listed in `applies_to`; a property of such a template without any value is a finding when it is required;
//! * a value the element states itself is checked against the definition it falls under (type, range, enumeration); a type is checked for its own values against the templates of its own kind and of the kind it types, so an inherited value
//!   is reported once, at its type, and not at every instance.
//!
//! The graph has one `Properties` node per holder that has authored properties or is subject to a template; none for a model without both. Nothing here is stored.
//!
//! Related: IFC property set templates, <https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifckernel/lexical/ifcpropertysettemplate.htm>.

use super::diagnostics::{Diagnostic, DiagnosticCode};
use super::element_solids::{dep_object, dep_value};
use crate::{ModelSnapshot, OpeningKind, PropertyTemplate, PropertyValue, TemplateTarget, Violation};
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};

/// 🗺️ The snapshot collections the effective properties read.
pub const READS: &[&str] = &[
    "properties", "property_templates", "sites", "buildings", "storeys", "walls", "curtain_walls", "columns", "beams", "slabs", "ceilings", "roofs", "openings", "stairs", "ramps", "railings", "spaces", "zones", "wall_types", "slab_types", "ceiling_types", "roof_types", "column_types", "beam_types", "window_types", "door_types",
];

//#region 🔖️Values
/// 🧭️ Where an effective value comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum Source {
    Own,
    Type,
    Default,
}

/// 🏷️ One effective value: the value, where it comes from and the template (id) that defines its property, when one does.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct EffectiveValue {
    pub value: PropertyValue,
    pub source: Source,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

/// 🚫️ What is wrong with one property: no value although required, or a value that breaks its definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
pub enum Issue {
    Missing,
    KindMismatch,
    BelowMinimum,
    AboveMaximum,
    NotAllowed,
}

impl From<Violation> for Issue {
    fn from(violation: Violation) -> Self {
        match violation {
            Violation::KindMismatch => Self::KindMismatch,
            Violation::BelowMinimum => Self::BelowMinimum,
            Violation::AboveMaximum => Self::AboveMaximum,
            Violation::NotAllowed => Self::NotAllowed,
        }
    }
}

/// 🚫️ One finding about one property of one holder.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub struct Finding {
    pub template: String,
    pub set: String,
    pub property: String,
    pub issue: Issue,
}

/// 🏷️ The effective properties of one holder: property set → property → value, and the findings against the templates that apply.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct EffectiveProperties {
    #[value(default)]
    pub values: BTreeMap<String, BTreeMap<String, EffectiveValue>>,
    #[value(default)]
    pub findings: Vec<Finding>,
}

impl EffectiveProperties {
    /// 🔎️ The effective value of one property.
    pub fn get(&self, set: &str, property: &str) -> Option<&EffectiveValue> {
        self.values.get(set).and_then(|properties| properties.get(property))
    }

    /// 🔢️ How many properties the holder has.
    pub fn len(&self) -> usize {
        self.values.values().map(BTreeMap::len).sum()
    }

    /// 🕳️ Whether the holder has no property.
    pub fn is_empty(&self) -> bool {
        self.values.values().all(BTreeMap::is_empty)
    }
}
//#endregion 🔖️Values

//#region 🔖️Holders
/// 🎯️ What a record id names for the templates: the element or type kind, `None` for an id that names neither.
pub fn target_of(snapshot: &ModelSnapshot, id: &str) -> Option<TemplateTarget> {
    let found = [
        (snapshot.sites.contains_key(id), TemplateTarget::Site),
        (snapshot.buildings.contains_key(id), TemplateTarget::Building),
        (snapshot.storeys.contains_key(id), TemplateTarget::Storey),
        (snapshot.walls.contains_key(id), TemplateTarget::Wall),
        (snapshot.curtain_walls.contains_key(id), TemplateTarget::CurtainWall),
        (snapshot.columns.contains_key(id), TemplateTarget::Column),
        (snapshot.beams.contains_key(id), TemplateTarget::Beam),
        (snapshot.slabs.contains_key(id), TemplateTarget::Slab),
        (snapshot.ceilings.contains_key(id), TemplateTarget::Ceiling),
        (snapshot.roofs.contains_key(id), TemplateTarget::Roof),
        (snapshot.stairs.contains_key(id), TemplateTarget::Stair),
        (snapshot.ramps.contains_key(id), TemplateTarget::Ramp),
        (snapshot.railings.contains_key(id), TemplateTarget::Railing),
        (snapshot.spaces.contains_key(id), TemplateTarget::Space),
        (snapshot.zones.contains_key(id), TemplateTarget::Zone),
        (snapshot.wall_types.contains_key(id), TemplateTarget::WallType),
        (snapshot.slab_types.contains_key(id), TemplateTarget::SlabType),
        (snapshot.ceiling_types.contains_key(id), TemplateTarget::CeilingType),
        (snapshot.roof_types.contains_key(id), TemplateTarget::RoofType),
        (snapshot.column_types.contains_key(id), TemplateTarget::ColumnType),
        (snapshot.beam_types.contains_key(id), TemplateTarget::BeamType),
        (snapshot.window_types.contains_key(id), TemplateTarget::WindowType),
        (snapshot.door_types.contains_key(id), TemplateTarget::DoorType),
    ]
    .into_iter()
    .find_map(|(present, target)| present.then_some(target));
    found.or_else(|| {
        snapshot.openings.get(id).map(|opening| match opening.kind {
            OpeningKind::Window { .. } => TemplateTarget::Window,
            OpeningKind::Door { .. } => TemplateTarget::Door,
            OpeningKind::Void { .. } => TemplateTarget::Void,
        })
    })
}

/// 🏛️ The id of the type record an instance is an instance of, when it names one that exists.
pub fn type_of(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    let named = snapshot
        .walls
        .get(id)
        .map(|row| (&row.wall_type, snapshot.wall_types.contains_key(&row.wall_type)))
        .or_else(|| snapshot.slabs.get(id).map(|row| (&row.slab_type, snapshot.slab_types.contains_key(&row.slab_type))))
        .or_else(|| snapshot.ceilings.get(id).map(|row| (&row.ceiling_type, snapshot.ceiling_types.contains_key(&row.ceiling_type))))
        .or_else(|| snapshot.roofs.get(id).map(|row| (&row.roof_type, snapshot.roof_types.contains_key(&row.roof_type))))
        .or_else(|| snapshot.columns.get(id).map(|row| (&row.column_type, snapshot.column_types.contains_key(&row.column_type))))
        .or_else(|| snapshot.beams.get(id).map(|row| (&row.beam_type, snapshot.beam_types.contains_key(&row.beam_type))))
        .or_else(|| {
            snapshot.openings.get(id).and_then(|row| match &row.kind {
                OpeningKind::Window { window_type } => Some((window_type, snapshot.window_types.contains_key(window_type))),
                OpeningKind::Door { door_type } => Some((door_type, snapshot.door_types.contains_key(door_type))),
                OpeningKind::Void { .. } => None,
            })
        });
    named.filter(|(_, present)| *present).map(|(type_id, _)| type_id.clone())
}

fn templates_for<'a>(snapshot: &'a ModelSnapshot, target: TemplateTarget) -> impl Iterator<Item = (&'a String, &'a PropertyTemplate)> {
    snapshot.property_templates.iter().filter(move |(_, template)| template.applies_to.contains(&target))
}

fn subject(snapshot: &ModelSnapshot, target: TemplateTarget) -> bool {
    templates_for(snapshot, target).next().is_some()
}

fn records(snapshot: &ModelSnapshot) -> impl Iterator<Item = &String> {
    snapshot
        .sites
        .keys()
        .chain(snapshot.buildings.keys())
        .chain(snapshot.storeys.keys())
        .chain(snapshot.walls.keys())
        .chain(snapshot.curtain_walls.keys())
        .chain(snapshot.columns.keys())
        .chain(snapshot.beams.keys())
        .chain(snapshot.slabs.keys())
        .chain(snapshot.ceilings.keys())
        .chain(snapshot.roofs.keys())
        .chain(snapshot.openings.keys())
        .chain(snapshot.stairs.keys())
        .chain(snapshot.ramps.keys())
        .chain(snapshot.railings.keys())
        .chain(snapshot.spaces.keys())
        .chain(snapshot.zones.keys())
        .chain(snapshot.area_schemes.keys())
        .chain(snapshot.views.keys())
        .chain(snapshot.wall_types.keys())
        .chain(snapshot.slab_types.keys())
        .chain(snapshot.ceiling_types.keys())
        .chain(snapshot.roof_types.keys())
        .chain(snapshot.column_types.keys())
        .chain(snapshot.beam_types.keys())
        .chain(snapshot.window_types.keys())
        .chain(snapshot.door_types.keys())
}

/// 🔎️ Whether `id` names a record that can hold properties: an element or a type of a library (the ids of the model are unique across collections).
pub fn holds(snapshot: &ModelSnapshot, id: &str) -> bool {
    target_of(snapshot, id).is_some() || snapshot.area_schemes.contains_key(id) || snapshot.views.contains_key(id)
}

/// 🧭️ The holders that have a node, each with the type it inherits from (also a holder): every record that states properties or falls under a template, and every instance of a type that is one; types before instances.
pub fn holders(snapshot: &ModelSnapshot) -> Vec<(String, Option<String>)> {
    let mut ids: BTreeSet<String> = snapshot.properties.keys().filter(|id| holds(snapshot, id)).cloned().collect();
    if !snapshot.property_templates.is_empty() {
        ids.extend(records(snapshot).filter(|id| target_of(snapshot, id).is_some_and(|target| subject(snapshot, target))).cloned());
    }
    let (types, instances): (Vec<String>, Vec<String>) = ids.into_iter().partition(|id| target_of(snapshot, id).is_some_and(TemplateTarget::is_type));
    let mut inheriting: BTreeSet<String> = instances.into_iter().collect();
    for type_id in &types {
        inheriting.extend(instances_of(snapshot, type_id));
    }
    let mut rows: Vec<(String, Option<String>)> = types.iter().map(|id| (id.clone(), None)).collect();
    rows.extend(inheriting.into_iter().map(|id| {
        let parent = type_of(snapshot, &id).filter(|type_id| types.contains(type_id));
        (id, parent)
    }));
    rows
}

/// 🧱️ The instances of the type `type_id`.
pub fn instances_of(snapshot: &ModelSnapshot, type_id: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    found.extend(snapshot.walls.iter().filter(|(_, row)| row.wall_type == type_id).map(|(id, _)| id.clone()));
    found.extend(snapshot.slabs.iter().filter(|(_, row)| row.slab_type == type_id).map(|(id, _)| id.clone()));
    found.extend(snapshot.ceilings.iter().filter(|(_, row)| row.ceiling_type == type_id).map(|(id, _)| id.clone()));
    found.extend(snapshot.roofs.iter().filter(|(_, row)| row.roof_type == type_id).map(|(id, _)| id.clone()));
    found.extend(snapshot.columns.iter().filter(|(_, row)| row.column_type == type_id).map(|(id, _)| id.clone()));
    found.extend(snapshot.beams.iter().filter(|(_, row)| row.beam_type == type_id).map(|(id, _)| id.clone()));
    found.extend(snapshot.openings.iter().filter(|(_, row)| matches!(&row.kind, OpeningKind::Window { window_type } if window_type == type_id) || matches!(&row.kind, OpeningKind::Door { door_type } if door_type == type_id)).map(|(id, _)| id.clone()));
    found
}
//#endregion 🔖️Holders

//#region 🔖️Rules
fn finding(template: &str, set: &str, property: &str, issue: Issue) -> Finding {
    Finding { template: template.to_string(), set: set.to_string(), property: property.to_string(), issue }
}

/// 🏷️ The effective properties of `id`, from the snapshot and the effective properties of its type (`inherited`, absent when it has none).
pub fn effective_of(snapshot: &ModelSnapshot, id: &str, inherited: Option<&EffectiveProperties>) -> EffectiveProperties {
    let target = target_of(snapshot, id);
    let mut values: BTreeMap<String, BTreeMap<String, EffectiveValue>> = BTreeMap::new();
    for (set, properties) in inherited.into_iter().flat_map(|inherited| &inherited.values) {
        for (name, effective) in properties {
            values.entry(set.clone()).or_default().insert(name.clone(), EffectiveValue { value: effective.value.clone(), source: Source::Type, template: effective.template.clone() });
        }
    }
    let own = snapshot.properties.get(id);
    for (set, properties) in own.into_iter().flatten() {
        for (name, value) in properties {
            values.entry(set.clone()).or_default().insert(name.clone(), EffectiveValue { value: value.clone(), source: Source::Own, template: None });
        }
    }
    let mut findings = Vec::new();
    for (template_id, template) in target.into_iter().flat_map(|target| templates_for(snapshot, target)) {
        for definition in &template.properties {
            let slot = values.get_mut(&template.name).and_then(|set| set.get_mut(&definition.name));
            match slot {
                Some(effective) => {
                    effective.template.get_or_insert_with(|| template_id.clone());
                    if effective.source == Source::Own {
                        if let Some(violation) = definition.violation(&effective.value) {
                            findings.push(finding(template_id, &template.name, &definition.name, violation.into()));
                        }
                    }
                }
                None => match &definition.default_value {
                    Some(default) => {
                        values.entry(template.name.clone()).or_default().insert(definition.name.clone(), EffectiveValue { value: default.clone(), source: Source::Default, template: Some(template_id.clone()) });
                    }
                    None if definition.required => findings.push(finding(template_id, &template.name, &definition.name, Issue::Missing)),
                    None => {}
                },
            }
        }
    }
    if let Some(served) = target.and_then(TemplateTarget::served) {
        for (template_id, template) in templates_for(snapshot, served) {
            for definition in &template.properties {
                let Some(effective) = values.get_mut(&template.name).and_then(|set| set.get_mut(&definition.name)).filter(|effective| effective.source == Source::Own) else { continue };
                effective.template.get_or_insert_with(|| template_id.clone());
                if let Some(violation) = definition.violation(&effective.value) {
                    findings.push(finding(template_id, &template.name, &definition.name, violation.into()));
                }
            }
        }
    }
    EffectiveProperties { values, findings }
}

/// 🔑️ What `effective_of` reads of the snapshot for `id` besides the effective properties of its type: the kind and the type of the holder, its own properties and the templates of its kind (and of the kind it types).
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let target = target_of(snapshot, id);
    let templates: BTreeMap<&String, &PropertyTemplate> = target.into_iter().flat_map(|target| std::iter::once(target).chain(target.served())).flat_map(|target| templates_for(snapshot, target)).collect();
    dep_object([("target", dep_value(&target.map(|target| target.name().to_string()))), ("type", dep_value(&type_of(snapshot, id))), ("own", dep_value(&snapshot.properties.get(id).cloned())), ("templates", DslValue::object(templates.into_iter().map(|(template, row)| (template.clone(), dep_value(row)))))])
}
//#endregion 🔖️Rules

//#region 🔖️Findings
fn code_of(issue: Issue) -> DiagnosticCode {
    match issue {
        Issue::Missing => DiagnosticCode::PropertyRequiredMissing,
        Issue::KindMismatch => DiagnosticCode::PropertyKindMismatch,
        Issue::BelowMinimum | Issue::AboveMaximum => DiagnosticCode::PropertyOutOfRange,
        Issue::NotAllowed => DiagnosticCode::PropertyNotAllowed,
    }
}

/// ⚠️ The findings of the data of the model: one per property finding of a holder, one per classification that names a missing system or a code the table of its system lacks. `storey_of` names the storey a holder stands on.
pub fn findings(snapshot: &ModelSnapshot, holders: &BTreeMap<&str, &EffectiveProperties>, storey_of: &dyn Fn(&str) -> Option<String>) -> Vec<Diagnostic> {
    let mut found = Vec::new();
    for (id, effective) in holders {
        let id: &str = id;
        for issue in &effective.findings {
            let mut diagnostic = Diagnostic::new(code_of(issue.issue), &[id]).lacking(&format!("{}.{}", issue.set, issue.property));
            if let Some(storey) = storey_of(id) {
                diagnostic = diagnostic.on(&storey);
            }
            found.push(diagnostic);
        }
    }
    for (holder, classified) in &snapshot.classifications {
        if !holds(snapshot, holder) {
            continue;
        }
        for (system, code) in classified {
            let diagnostic = match snapshot.classification_systems.get(system) {
                None => Some(Diagnostic::new(DiagnosticCode::RefClassificationSystem, &[holder.as_str()]).lacking(system)),
                Some(table) if table.entry(code).is_none() => Some(Diagnostic::new(DiagnosticCode::ClassificationUnknownCode, &[holder.as_str()]).lacking(code)),
                Some(_) => None,
            };
            if let Some(diagnostic) = diagnostic {
                found.push(match storey_of(holder) {
                    Some(storey) => diagnostic.on(&storey),
                    None => diagnostic,
                });
            }
        }
    }
    found
}

/// 🔑️ What the findings of the data read besides the effective properties: the classifications of the model and the tables of the systems.
pub fn findings_dependency(snapshot: &ModelSnapshot) -> DslValue {
    dep_object([("classifications", dep_value(&snapshot.classifications)), ("systems", dep_value(&snapshot.classification_systems))])
}
//#endregion 🔖️Findings

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
