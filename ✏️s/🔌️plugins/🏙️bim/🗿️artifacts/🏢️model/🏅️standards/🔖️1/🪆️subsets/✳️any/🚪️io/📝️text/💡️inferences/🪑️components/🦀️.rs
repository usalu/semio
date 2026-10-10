//! 📊️ Native JSON inference table projection of the components and MEP elements: what the third-party oracle reproduces from the snapshot alone. Everything in it follows in closed form from the authored values (the
//! placement of every component with its host fit, footprint, bounds, volume and connector; the section, path, length and closed-form volume of every MEP element; the clashing pairs; the quantities per element and per
//! group; the findings of the component and MEP codes). The tessellated volumes of the solids are left to the three.js solids oracle.

use crate::standards::v1::subsets::any::schema::inferences::components::{ComponentValue, Connector, HostFit};
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::{Diagnostic, DiagnosticCode};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidBounds;
use crate::standards::v1::subsets::any::schema::inferences::mep::{self, MepValue};
use crate::standards::v1::subsets::any::schema::inferences::quantities::{ElementQuantity, QuantityKind};
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::{FamilyCategory, Point2, Point3};
use std::collections::BTreeMap;

/// 🪑️ The placement of one component as the oracle reproduces it: the turn as cosine and sine (an angle has two spellings), the rest as inferred.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct ComponentRow {
    pub storey: String,
    pub family: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<FamilyCategory>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw_cos: f64,
    pub yaw_sin: f64,
    pub mirrored: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<HostFit>,
    pub footprint: Vec<Point2>,
    pub footprint_area: f64,
    pub bounds: SolidBounds,
    pub volume: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub connector: Option<Connector>,
    pub overridden: Vec<String>,
    pub issues: Vec<String>,
}

/// 🌀️ One MEP element as the oracle reproduces it.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct MepRow {
    pub storey: String,
    pub system: String,
    pub colour: String,
    pub kind: String,
    pub label: String,
    pub width: f64,
    pub height: f64,
    pub area: f64,
    pub perimeter: f64,
    pub path: Vec<Point3>,
    pub length: f64,
    pub volume: f64,
    pub surface_area: f64,
    pub bounds: SolidBounds,
    pub issues: Vec<String>,
}

/// 🧮️ The closed-form measures of one element quantity.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct QuantityRow {
    pub kind: String,
    pub type_id: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub perimeter: f64,
    pub gross_area: f64,
    pub net_area: f64,
    pub gross_volume: f64,
    pub groups: Vec<String>,
}

/// ➕️ The sum of the closed-form measures of the elements of one group.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue)]
pub struct GroupRow {
    pub count: u32,
    pub length: f64,
    pub area: f64,
    pub gross_volume: f64,
}

/// 🚦️ One finding of the component and MEP codes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
pub struct FindingRow {
    pub code: String,
    pub elements: Vec<String>,
    pub missing: Vec<String>,
    pub values: BTreeMap<String, f64>,
}

/// 📊️ The whole table.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue)]
pub struct ComponentsTable {
    pub components: BTreeMap<String, ComponentRow>,
    pub mep: BTreeMap<String, MepRow>,
    pub quantities: BTreeMap<String, QuantityRow>,
    pub groups: BTreeMap<String, GroupRow>,
    pub findings: Vec<FindingRow>,
}

fn component_row(value: &ComponentValue) -> ComponentRow {
    let (sin, cos) = value.placement.yaw.sin_cos();
    ComponentRow {
        storey: value.storey.clone(),
        family: value.family.clone(),
        category: value.category,
        x: value.placement.x,
        y: value.placement.y,
        z: value.placement.z,
        yaw_cos: cos,
        yaw_sin: sin,
        mirrored: value.placement.mirrored,
        host: value.placement.host.clone(),
        footprint: value.footprint.clone(),
        footprint_area: value.footprint_area,
        bounds: value.bounds,
        volume: value.volume,
        connector: value.connector.clone(),
        overridden: value.overridden.clone(),
        issues: slugs(value.issues.iter().map(|issue| issue.code.slug())),
    }
}

fn slugs<'a>(codes: impl Iterator<Item = &'a str>) -> Vec<String> {
    let mut rows: Vec<String> = codes.map(str::to_string).collect();
    rows.sort();
    rows
}

fn mep_row(value: &MepValue) -> MepRow {
    MepRow {
        storey: value.storey.clone(),
        system: mep::key(value.system).to_string(),
        colour: value.colour.clone(),
        kind: value.section.kind.key().to_string(),
        label: value.section.label.clone(),
        width: value.section.width,
        height: value.section.height,
        area: value.section.area,
        perimeter: value.section.perimeter,
        path: value.path.clone(),
        length: value.length,
        volume: value.volume,
        surface_area: value.surface_area,
        bounds: value.bounds,
        issues: slugs(value.issues.iter().map(|issue| issue.code.slug())),
    }
}

fn quantity_row(element: &ElementQuantity) -> QuantityRow {
    QuantityRow {
        kind: element.kind.key().to_string(),
        type_id: element.type_id.clone(),
        length: element.length,
        width: element.width,
        height: element.height,
        perimeter: element.perimeter,
        gross_area: element.gross_area,
        net_area: element.net_area,
        gross_volume: element.gross_volume,
        groups: element.groups.clone(),
    }
}

const CODES: [DiagnosticCode; 8] = [
    DiagnosticCode::ComponentOutsideStorey,
    DiagnosticCode::ComponentInWall,
    DiagnosticCode::RefComponentFamily,
    DiagnosticCode::RefComponentHost,
    DiagnosticCode::ComponentOverride,
    DiagnosticCode::MepDegenerate,
    DiagnosticCode::MepClash,
    DiagnosticCode::TerminalUnconnected,
];

fn finding_row(finding: &Diagnostic) -> FindingRow {
    FindingRow { code: finding.code.slug().to_string(), elements: finding.elements.clone(), missing: finding.missing.clone(), values: finding.values.clone() }
}

fn sorted(mut rows: Vec<FindingRow>) -> Vec<FindingRow> {
    rows.sort_by(|a, b| a.code.cmp(&b.code).then_with(|| a.elements.cmp(&b.elements)).then_with(|| a.missing.cmp(&b.missing)));
    rows
}

/// 🧾️ The table of the components and MEP elements of an inference.
pub fn table_of(inferred: &ModelInference) -> ComponentsTable {
    let measured: BTreeMap<&String, &ElementQuantity> = inferred.quantities.elements.iter().filter(|(_, element)| matches!(element.kind, QuantityKind::Component | QuantityKind::Mep)).collect();
    let mut groups: BTreeMap<String, GroupRow> = BTreeMap::new();
    for element in measured.values() {
        for group in &element.groups {
            let row = groups.entry(group.clone()).or_default();
            row.count += element.count;
            row.length += element.length;
            row.area += element.area();
            row.gross_volume += element.gross_volume;
        }
    }
    ComponentsTable {
        components: inferred.components.iter().map(|(id, value)| (id.clone(), component_row(value))).collect(),
        mep: inferred.mep.iter().map(|(id, value)| (id.clone(), mep_row(value))).collect(),
        quantities: measured.iter().map(|(id, element)| ((*id).clone(), quantity_row(element))).collect(),
        groups,
        findings: sorted(inferred.diagnostics.iter().filter(|finding| CODES.contains(&finding.code)).map(finding_row).collect()),
    }
}

/// 🧾️ The table the third-party oracle reproduces, as canonical JSON.
pub fn table_json(inferred: &ModelInference) -> String {
    semio_framework_pack_json::to_json_string(&table_of(inferred))
}
