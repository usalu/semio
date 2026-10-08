//! 🔌️ Input resolution: for every catalogue input port of a widget, the connected output, else the stored literal, else the
//! catalogue default — type-checked, range-checked and localized, before any compute runs.

use super::value::{shape_kind, GeometryValue, PlaneValue, SelectionKind, SelectionValue, WidgetEvaluation, WidgetFault};
use crate::standards::v1::subsets::any::schema::catalogue::{Kind, Port, PortType, SelectionComponent};
use semio_framework_artifact_flow_flow::neural::Dictionary;
use semio_framework_artifact_flow_flow::SynapseSpec;
use semio_framework_3d::brep::engine::ShapeValue;
use semio_framework_3d::mesh::HalfedgeMesh;
use semio_framework_value::{DslValue, ToValue};
use std::collections::BTreeMap;
use std::sync::Arc;

//#region 🔖️Codes
pub const INPUT_MISSING: &str = "generation3d.geometry.input-missing";
pub const INPUT_TYPE: &str = "generation3d.geometry.input-type";
pub const INPUT_RANGE: &str = "generation3d.geometry.input-range";
pub const INPUT_LITERAL: &str = "generation3d.geometry.input-literal";
pub const INPUT_MULTIPLE: &str = "generation3d.geometry.input-multiple";
pub const INPUT_ITEMS: &str = "generation3d.geometry.input-items";
pub const INPUT_OPTION: &str = "generation3d.geometry.input-option";
pub const INPUT_UNKNOWN: &str = "generation3d.geometry.input-unknown";
pub const SHAPE_KIND: &str = "generation3d.geometry.shape-kind";
pub const UPSTREAM: &str = "generation3d.geometry.upstream";
//#endregion 🔖️Codes

//#region 🔖️Inputs
/// 🔌️ The resolved inputs of one widget, by catalogue input port name; an optional port without a value is absent.
#[derive(Clone, Debug)]
pub struct WidgetInputs {
    widget: String,
    labels: BTreeMap<String, crate::standards::v1::subsets::any::schema::catalogue::Localized>,
    values: BTreeMap<String, GeometryValue>,
}

fn port_label(kind: &Kind, port: &str) -> (String, String) {
    kind.input(port).map_or_else(|| (port.to_string(), port.to_string()), |declared| (declared.label.en.clone(), declared.label.de.clone()))
}

impl WidgetInputs {
    pub fn new(widget: impl Into<String>, kind: &Kind, values: BTreeMap<String, GeometryValue>) -> Self {
        Self { widget: widget.into(), labels: kind.inputs.iter().map(|port| (port.name.clone(), port.label.clone())).collect(), values }
    }

    pub fn widget(&self) -> &str {
        &self.widget
    }

    fn label(&self, port: &str) -> (String, String) {
        self.labels.get(port).map_or_else(|| (port.to_string(), port.to_string()), |label| (label.en.clone(), label.de.clone()))
    }

    /// 🔎️ The value of a port, absent for an optional port nothing feeds.
    pub fn get(&self, port: &str) -> Option<&GeometryValue> {
        self.values.get(port)
    }

    fn require(&self, port: &str) -> Result<&GeometryValue, WidgetFault> {
        let (en, de) = self.label(port);
        self.values.get(port).ok_or_else(|| WidgetFault::new(INPUT_MISSING, format!("Input \u{201c}{en}\u{201d} has no value."), format!("Eingang \u{201c}{de}\u{201d} hat keinen Wert.")).at(port))
    }

    fn mismatch(&self, port: &str, expected: (&str, &str)) -> WidgetFault {
        let (en, de) = self.label(port);
        WidgetFault::new(INPUT_TYPE, format!("Input \u{201c}{en}\u{201d} is not a {}.", expected.0), format!("Eingang \u{201c}{de}\u{201d} ist kein(e) {}.", expected.1)).at(port)
    }

    /// 🔢️ A `Number` or `Integer` value as a real number.
    pub fn number(&self, port: &str) -> Result<f64, WidgetFault> {
        match self.require(port)? {
            GeometryValue::Number(value) => Ok(*value),
            GeometryValue::Integer(value) => Ok(*value as f64),
            _ => Err(self.mismatch(port, ("number", "Zahl"))),
        }
    }

    pub fn integer(&self, port: &str) -> Result<i64, WidgetFault> {
        match self.require(port)? {
            GeometryValue::Integer(value) => Ok(*value),
            _ => Err(self.mismatch(port, ("whole number", "Ganzzahl"))),
        }
    }

    pub fn boolean(&self, port: &str) -> Result<bool, WidgetFault> {
        match self.require(port)? {
            GeometryValue::Boolean(value) => Ok(*value),
            _ => Err(self.mismatch(port, ("boolean", "Wahrheitswert"))),
        }
    }

    pub fn text(&self, port: &str) -> Result<&str, WidgetFault> {
        match self.require(port)? {
            GeometryValue::Text(value) => Ok(value),
            _ => Err(self.mismatch(port, ("text", "Text"))),
        }
    }

    pub fn vector(&self, port: &str) -> Result<[f64; 3], WidgetFault> {
        match self.require(port)? {
            GeometryValue::Vector(value) => Ok(*value),
            _ => Err(self.mismatch(port, ("vector", "Vektor"))),
        }
    }

    pub fn point(&self, port: &str) -> Result<[f64; 3], WidgetFault> {
        match self.require(port)? {
            GeometryValue::Point(value) => Ok(*value),
            _ => Err(self.mismatch(port, ("point", "Punkt"))),
        }
    }

    pub fn plane(&self, port: &str) -> Result<PlaneValue, WidgetFault> {
        match self.require(port)? {
            GeometryValue::Plane(value) => Ok(*value),
            _ => Err(self.mismatch(port, ("plane", "Ebene"))),
        }
    }

    pub fn shape(&self, port: &str) -> Result<&Arc<ShapeValue>, WidgetFault> {
        match self.require(port)? {
            GeometryValue::Shape(value) => Ok(value),
            _ => Err(self.mismatch(port, ("shape", "Form"))),
        }
    }

    /// 🧱️ The shapes of a `shapes` port or a list-of-shape port.
    pub fn shapes(&self, port: &str) -> Result<Vec<&Arc<ShapeValue>>, WidgetFault> {
        self.list(port)?.iter().map(|item| match item {
            GeometryValue::Shape(value) => Ok(value),
            _ => Err(self.mismatch(port, ("list of shapes", "Liste von Formen"))),
        }).collect()
    }

    pub fn mesh(&self, port: &str) -> Result<&Arc<HalfedgeMesh>, WidgetFault> {
        match self.require(port)? {
            GeometryValue::Mesh(value) => Ok(value),
            _ => Err(self.mismatch(port, ("mesh", "Netz"))),
        }
    }

    pub fn selection(&self, port: &str) -> Result<&SelectionValue, WidgetFault> {
        match self.require(port)? {
            GeometryValue::Selection(value) => Ok(value),
            _ => Err(self.mismatch(port, ("selection", "Auswahl"))),
        }
    }

    pub fn list(&self, port: &str) -> Result<&[GeometryValue], WidgetFault> {
        match self.require(port)? {
            GeometryValue::List(items) => Ok(items),
            _ => Err(self.mismatch(port, ("list", "Liste"))),
        }
    }

    /// 🔢️ The items of a list port as real numbers.
    pub fn numbers(&self, port: &str) -> Result<Vec<f64>, WidgetFault> {
        self.list(port)?.iter().map(|item| match item {
            GeometryValue::Number(value) => Ok(*value),
            GeometryValue::Integer(value) => Ok(*value as f64),
            _ => Err(self.mismatch(port, ("list of numbers", "Liste von Zahlen"))),
        }).collect()
    }
}
//#endregion 🔖️Inputs

//#region 🔖️Decode
fn axes_of_default(value: &DslValue) -> Option<[f64; 3]> {
    let items = value.as_array()?;
    (items.len() == 3).then_some(())?;
    Some([items[0].as_f64()?, items[1].as_f64()?, items[2].as_f64()?])
}

fn axes_of_literal(literal: &DslValue, schema: &str) -> Option<[f64; 3]> {
    (literal.get("$schema")?.as_str()? == schema).then_some(())?;
    Some([literal.get("x")?.as_f64()?, literal.get("y")?.as_f64()?, literal.get("z")?.as_f64()?])
}

fn id_of(value: &DslValue) -> Option<u64> {
    match value.as_str() {
        Some(text) => text.parse().ok(),
        None => value.as_f64().filter(|number| *number >= 0.0 && number.fract() == 0.0 && *number <= u64::MAX as f64).map(|number| number as u64),
    }
}

fn element_default(port_type: PortType, value: &DslValue) -> Option<GeometryValue> {
    Some(match port_type {
        PortType::Number | PortType::Length | PortType::Angle => GeometryValue::Number(value.as_f64()?),
        PortType::Integer => {
            let number = value.as_f64().filter(|number| number.fract() == 0.0)?;
            GeometryValue::Integer(number as i64)
        }
        PortType::Boolean => GeometryValue::Boolean(value.as_bool()?),
        PortType::Text | PortType::Enum => GeometryValue::Text(value.as_str()?.to_string()),
        PortType::Vector => GeometryValue::Vector(axes_of_default(value)?),
        PortType::Point => GeometryValue::Point(axes_of_default(value)?),
        PortType::Plane => GeometryValue::Plane(PlaneValue { origin: axes_of_default(value.get("origin")?)?, normal: axes_of_default(value.get("normal")?)? }),
        PortType::Any => match value {
            DslValue::Bool(flag) => GeometryValue::Boolean(*flag),
            DslValue::String(text) => GeometryValue::Text(text.clone()),
            _ => GeometryValue::Number(value.as_f64()?),
        },
        PortType::Shape | PortType::Shapes | PortType::Mesh | PortType::Selection => return None,
    })
}

fn element_literal(port_type: PortType, literal: &DslValue) -> Option<GeometryValue> {
    let schema = literal.get("$schema")?.as_str()?;
    let value = literal.get("value");
    Some(match (port_type, schema) {
        (PortType::Number | PortType::Length | PortType::Angle, "number") => GeometryValue::Number(value?.as_f64()?),
        (PortType::Integer, "number") => {
            let number = value?.as_f64().filter(|number| number.fract() == 0.0)?;
            GeometryValue::Integer(number as i64)
        }
        (PortType::Boolean, "boolean") => GeometryValue::Boolean(value?.as_bool()?),
        (PortType::Text | PortType::Enum, "text") => GeometryValue::Text(value?.as_str()?.to_string()),
        (PortType::Vector, "vector") => GeometryValue::Vector(axes_of_literal(literal, "vector")?),
        (PortType::Point, "point") => GeometryValue::Point(axes_of_literal(literal, "point")?),
        (PortType::Plane, "plane") => GeometryValue::Plane(PlaneValue { origin: axes_of_literal(literal.get("origin")?, "point")?, normal: axes_of_literal(literal.get("normal")?, "vector")? }),
        (PortType::Any, "number") => GeometryValue::Number(value?.as_f64()?),
        (PortType::Any, "boolean") => GeometryValue::Boolean(value?.as_bool()?),
        (PortType::Any, "text") => GeometryValue::Text(value?.as_str()?.to_string()),
        (PortType::Any, "vector") => GeometryValue::Vector(axes_of_literal(literal, "vector")?),
        (PortType::Any, "point") => GeometryValue::Point(axes_of_literal(literal, "point")?),
        _ => return None,
    })
}

fn literal_items(literal: &DslValue) -> Option<Vec<&DslValue>> {
    (literal.get("$schema")?.as_str()? == "list").then_some(())?;
    let mut items = Vec::new();
    while let Some(item) = literal.get(&items.len().to_string()) {
        items.push(item);
    }
    Some(items)
}

fn counted(port: &Port) -> bool {
    port.list || port.port_type == PortType::Shapes
}

fn element_type(port: &Port) -> PortType {
    if port.port_type == PortType::Shapes { PortType::Shape } else { port.port_type }
}

fn decode_default(port: &Port, default: &DslValue, selection: Option<SelectionKind>) -> Option<GeometryValue> {
    if let Some(component) = selection {
        let ids = default.as_array()?.iter().map(id_of).collect::<Option<Vec<_>>>()?;
        return Some(GeometryValue::Selection(SelectionValue { component, ids }));
    }
    if counted(port) {
        let items = default.as_array()?.iter().map(|item| element_default(element_type(port), item)).collect::<Option<Vec<_>>>()?;
        return Some(GeometryValue::List(items));
    }
    element_default(port.port_type, default)
}

fn decode_literal(port: &Port, literal: &DslValue, selection: Option<SelectionKind>) -> Option<GeometryValue> {
    if let Some(component) = selection {
        let ids = literal_items(literal)?.into_iter().map(|item| if item.get("$schema")?.as_str()? == "number" { item.get("value").and_then(id_of) } else { None }).collect::<Option<Vec<_>>>()?;
        return Some(GeometryValue::Selection(SelectionValue { component, ids }));
    }
    if counted(port) {
        let items = literal_items(literal)?.into_iter().map(|item| element_literal(element_type(port), item)).collect::<Option<Vec<_>>>()?;
        return Some(GeometryValue::List(items));
    }
    element_literal(port.port_type, literal)
}
//#endregion 🔖️Decode

//#region 🔖️Check
fn type_names(port_type: PortType) -> (&'static str, &'static str) {
    match port_type {
        PortType::Number => ("number", "Zahl"),
        PortType::Integer => ("whole number", "Ganzzahl"),
        PortType::Angle => ("angle", "Winkel"),
        PortType::Length => ("length", "Länge"),
        PortType::Boolean => ("boolean", "Wahrheitswert"),
        PortType::Text => ("text", "Text"),
        PortType::Enum => ("choice", "Auswahlwert"),
        PortType::Vector => ("vector", "Vektor"),
        PortType::Point => ("point", "Punkt"),
        PortType::Plane => ("plane", "Ebene"),
        PortType::Shape => ("shape", "Form"),
        PortType::Shapes => ("list of shapes", "Liste von Formen"),
        PortType::Mesh => ("mesh", "Netz"),
        PortType::Selection => ("selection", "Auswahl"),
        PortType::Any => ("value", "Wert"),
    }
}

fn fault(code: &str, port: &Port, en: impl FnOnce(&str) -> String, de: impl FnOnce(&str) -> String) -> WidgetFault {
    WidgetFault::new(code, en(&port.label.en), de(&port.label.de)).at(&port.name)
}

fn mismatch(port: &Port, found: &GeometryValue) -> WidgetFault {
    let (expected_en, expected_de) = type_names(port.port_type);
    fault(INPUT_TYPE, port, |label| format!("Input \u{201c}{label}\u{201d} needs a {expected_en}, not a {}.", found.kind_name()), |label| format!("Eingang \u{201c}{label}\u{201d} braucht ein(e) {expected_de}, kein(e) {}.", found.kind_name()))
}

fn range_checked(port: &Port, number: f64) -> Result<(), WidgetFault> {
    let above = match port.min {
        Some(min) if port.exclusive_min => number > min,
        Some(min) => number >= min,
        None => true,
    };
    if above && port.max.is_none_or(|max| number <= max) && number.is_finite() {
        return Ok(());
    }
    let bound_en = match (port.min, port.max) {
        (Some(min), Some(max)) => format!("between {min}{} and {max}", if port.exclusive_min { " (exclusive)" } else { "" }),
        (Some(min), None) => format!("{} {min}", if port.exclusive_min { "above" } else { "at least" }),
        (None, Some(max)) => format!("at most {max}"),
        (None, None) => "finite".to_string(),
    };
    let bound_de = match (port.min, port.max) {
        (Some(min), Some(max)) => format!("zwischen {min}{} und {max}", if port.exclusive_min { " (exklusiv)" } else { "" }),
        (Some(min), None) => format!("{} {min}", if port.exclusive_min { "über" } else { "mindestens" }),
        (None, Some(max)) => format!("höchstens {max}"),
        (None, None) => "endlich".to_string(),
    };
    Err(fault(INPUT_RANGE, port, |label| format!("Input \u{201c}{label}\u{201d} is {number}, but must be {bound_en}."), |label| format!("Eingang \u{201c}{label}\u{201d} ist {number}, muss aber {bound_de} sein.")))
}

fn checked_element(port: &Port, value: GeometryValue, selection: Option<SelectionKind>) -> Result<GeometryValue, WidgetFault> {
    match (element_type(port), value) {
        (PortType::Number | PortType::Length | PortType::Angle, GeometryValue::Number(number)) => range_checked(port, number).map(|()| GeometryValue::Number(number)),
        (PortType::Number | PortType::Length | PortType::Angle, GeometryValue::Integer(integer)) => range_checked(port, integer as f64).map(|()| GeometryValue::Number(integer as f64)),
        (PortType::Integer, GeometryValue::Integer(integer)) => range_checked(port, integer as f64).map(|()| GeometryValue::Integer(integer)),
        (PortType::Boolean, value @ GeometryValue::Boolean(_)) | (PortType::Text, value @ GeometryValue::Text(_)) | (PortType::Vector, value @ GeometryValue::Vector(_)) | (PortType::Point, value @ GeometryValue::Point(_)) | (PortType::Plane, value @ GeometryValue::Plane(_)) | (PortType::Mesh, value @ GeometryValue::Mesh(_)) | (PortType::Any, value) => Ok(value),
        (PortType::Enum, GeometryValue::Text(text)) => {
            if port.options.as_deref().unwrap_or_default().iter().any(|option| option.value == text) {
                Ok(GeometryValue::Text(text))
            } else {
                let allowed = port.options.as_deref().unwrap_or_default().iter().map(|option| option.value.as_str()).collect::<Vec<_>>().join(", ");
                Err(fault(INPUT_OPTION, port, |label| format!("Input \u{201c}{label}\u{201d} is \u{201c}{text}\u{201d}, but must be one of {allowed}."), |label| format!("Eingang \u{201c}{label}\u{201d} ist \u{201c}{text}\u{201d}, muss aber einer von {allowed} sein.")))
            }
        }
        (PortType::Shape, GeometryValue::Shape(shape)) => match port.shape_kinds.as_deref() {
            Some(kinds) if !kinds.contains(&shape_kind(shape.kind())) => {
                let found = format!("{:?}", shape.kind()).to_lowercase();
                let allowed = kinds.iter().map(|kind| format!("{kind:?}").to_lowercase()).collect::<Vec<_>>().join(", ");
                Err(fault(SHAPE_KIND, port, |label| format!("Input \u{201c}{label}\u{201d} is a {found}, but must be a {allowed}."), |label| format!("Eingang \u{201c}{label}\u{201d} ist ein(e) {found}, muss aber ein(e) {allowed} sein.")))
            }
            _ => Ok(GeometryValue::Shape(shape)),
        },
        (PortType::Selection, GeometryValue::Selection(found)) => match selection {
            Some(wanted) if wanted != found.component => Err(fault(INPUT_TYPE, port, |label| format!("Input \u{201c}{label}\u{201d} needs a {} selection, not a {} selection.", wanted.name(), found.component.name()), |label| format!("Eingang \u{201c}{label}\u{201d} braucht eine {}-Auswahl, keine {}-Auswahl.", wanted.name(), found.component.name()))),
            _ => Ok(GeometryValue::Selection(found)),
        },
        (_, found) => Err(mismatch(port, &found)),
    }
}

fn items_checked(port: &Port, count: usize) -> Result<(), WidgetFault> {
    let bound = |bound: Option<u32>| bound.map(|bound| bound as usize);
    if bound(port.min_items).is_some_and(|min| count < min) || bound(port.max_items).is_some_and(|max| count > max) {
        let (min, max) = (port.min_items, port.max_items);
        let words_en = match (min, max) {
            (Some(min), Some(max)) if min == max => format!("exactly {min}"),
            (Some(min), Some(max)) => format!("between {min} and {max}"),
            (Some(min), None) => format!("at least {min}"),
            (None, Some(max)) => format!("at most {max}"),
            (None, None) => "a valid number of".to_string(),
        };
        let words_de = match (min, max) {
            (Some(min), Some(max)) if min == max => format!("genau {min}"),
            (Some(min), Some(max)) => format!("zwischen {min} und {max}"),
            (Some(min), None) => format!("mindestens {min}"),
            (None, Some(max)) => format!("höchstens {max}"),
            (None, None) => "eine gültige Anzahl".to_string(),
        };
        return Err(fault(INPUT_ITEMS, port, |label| format!("Input \u{201c}{label}\u{201d} has {count} items, but needs {words_en}."), |label| format!("Eingang \u{201c}{label}\u{201d} hat {count} Elemente, braucht aber {words_de}.")));
    }
    Ok(())
}

fn checked(port: &Port, value: GeometryValue, selection: Option<SelectionKind>) -> Result<GeometryValue, WidgetFault> {
    match value {
        GeometryValue::List(items) if counted(port) => {
            items_checked(port, items.len())?;
            items.into_iter().map(|item| checked_element(port, item, None)).collect::<Result<Vec<_>, _>>().map(GeometryValue::List)
        }
        GeometryValue::Selection(found) => {
            items_checked(port, found.ids.len())?;
            checked_element(port, GeometryValue::Selection(found), selection)
        }
        value if counted(port) => Err(mismatch(port, &value)),
        value => checked_element(port, value, selection),
    }
}
//#endregion 🔖️Check

//#region 🔖️Resolve
fn literal_fault(port: &Port) -> WidgetFault {
    let (expected_en, expected_de) = type_names(port.port_type);
    fault(INPUT_LITERAL, port, |label| format!("The value stored for input \u{201c}{label}\u{201d} is not a {expected_en}."), |label| format!("Der für Eingang \u{201c}{label}\u{201d} gespeicherte Wert ist kein(e) {expected_de}."))
}

fn source_value(port: &Port, wire: &SynapseSpec, parent: &dyn Fn(&str) -> Option<Arc<WidgetEvaluation>>) -> Result<GeometryValue, WidgetFault> {
    let Some(evaluation) = parent(&wire.from) else {
        let from = wire.from.clone();
        return Err(fault(INPUT_MISSING, port, |label| format!("Input \u{201c}{label}\u{201d} is wired to widget \u{201c}{from}\u{201d}, which does not exist."), |label| format!("Eingang \u{201c}{label}\u{201d} ist mit dem Widget \u{201c}{from}\u{201d} verbunden, das nicht existiert.")));
    };
    if let Some(source) = &evaluation.fault {
        let from = wire.from.clone();
        let (en, de) = (source.message.en.clone(), source.message.de.clone());
        return Err(fault(UPSTREAM, port, |label| format!("Input \u{201c}{label}\u{201d} is unavailable because widget \u{201c}{from}\u{201d} failed: {en}"), |label| format!("Eingang \u{201c}{label}\u{201d} ist nicht verfügbar, weil Widget \u{201c}{from}\u{201d} fehlgeschlagen ist: {de}")));
    }
    let output = if wire.from_port.is_empty() {
        if evaluation.outputs.len() == 1 { evaluation.outputs.values().next() } else { None }
    } else {
        evaluation.outputs.get(&wire.from_port)
    };
    output.cloned().ok_or_else(|| {
        let (from, from_port) = (wire.from.clone(), wire.from_port.clone());
        fault(INPUT_MISSING, port, |label| format!("Input \u{201c}{label}\u{201d} is wired to \u{201c}{from}\u{201d}, which has no output \u{201c}{from_port}\u{201d}."), |label| format!("Eingang \u{201c}{label}\u{201d} ist mit \u{201c}{from}\u{201d} verbunden, das keinen Ausgang \u{201c}{from_port}\u{201d} hat."))
    })
}

fn connected_value(port: &Port, wires: &[&SynapseSpec], parent: &dyn Fn(&str) -> Option<Arc<WidgetEvaluation>>) -> Result<GeometryValue, WidgetFault> {
    if counted(port) {
        let mut items = Vec::new();
        for wire in wires {
            match source_value(port, wire, parent)? {
                GeometryValue::List(spliced) => items.extend(spliced),
                single => items.push(single),
            }
        }
        return Ok(GeometryValue::List(items));
    }
    if wires.len() > 1 {
        return Err(fault(INPUT_MULTIPLE, port, |label| format!("Input \u{201c}{label}\u{201d} accepts one connection, but {} are wired.", wires.len()), |label| format!("Eingang \u{201c}{label}\u{201d} akzeptiert eine Verbindung, aber {} sind verbunden.", wires.len())));
    }
    source_value(port, wires[0], parent)
}

fn selection_component(port: &Port, resolved: &BTreeMap<String, GeometryValue>) -> Result<Option<SelectionKind>, WidgetFault> {
    let Some(selection) = &port.selection else { return Ok(None) };
    Ok(Some(match selection.component {
        SelectionComponent::Face => SelectionKind::Face,
        SelectionComponent::Edge => SelectionKind::Edge,
        SelectionComponent::Vertex => SelectionKind::Vertex,
        SelectionComponent::Mode => {
            let mode = selection.mode_from.as_deref().and_then(|from| resolved.get(from));
            match mode {
                Some(GeometryValue::Text(name)) => SelectionKind::parse(name).ok_or_else(|| literal_fault(port))?,
                _ => return Err(fault(INPUT_MISSING, port, |label| format!("Input \u{201c}{label}\u{201d} needs its element mode first."), |label| format!("Eingang \u{201c}{label}\u{201d} braucht zuerst seinen Elementmodus."))),
            }
        }
    }))
}

/// 🔌️ Resolves every input port of `kind` for widget `widget`: wiring first (`wires` are the synapses ending at the widget), then the stored literal in `params`, then the catalogue default.
/// `parent` answers the evaluation of a source widget. The first refusal is returned as a localized fault naming the port.
pub fn resolve_inputs(kind: &Kind, widget: &str, params: &Dictionary, wires: &[&SynapseSpec], parent: &dyn Fn(&str) -> Option<Arc<WidgetEvaluation>>) -> Result<WidgetInputs, WidgetFault> {
    if let Some(unknown) = wires.iter().find(|wire| kind.input(&wire.to_port).is_none()) {
        let name = unknown.to_port.clone();
        return Err(WidgetFault::new(INPUT_UNKNOWN, format!("A wire ends at \u{201c}{name}\u{201d}, which is not an input of {}.", kind.label.en), format!("Eine Verbindung endet bei \u{201c}{name}\u{201d}, das kein Eingang von {} ist.", kind.label.de)).at(name));
    }
    let mut resolved: BTreeMap<String, GeometryValue> = BTreeMap::new();
    for selecting in [false, true] {
        for port in kind.inputs.iter().filter(|port| (port.port_type == PortType::Selection) == selecting) {
            let component = selection_component(port, &resolved)?;
            let incoming: Vec<&SynapseSpec> = wires.iter().copied().filter(|wire| wire.to_port == port.name).collect();
            let value = if !incoming.is_empty() {
                Some(connected_value(port, &incoming, parent)?)
            } else if let Some(stored) = params.get(&port.name) {
                Some(decode_literal(port, &stored.to_value(), component).ok_or_else(|| literal_fault(port))?)
            } else if let Some(default) = &port.default {
                Some(decode_default(port, default, component).ok_or_else(|| literal_fault(port))?)
            } else {
                None
            };
            match value {
                Some(value) => {
                    resolved.insert(port.name.clone(), checked(port, value, component)?);
                }
                None if port.optional => {}
                None => return Err(fault(INPUT_MISSING, port, |label| format!("Input \u{201c}{label}\u{201d} has no value: connect a widget or enter one."), |label| format!("Eingang \u{201c}{label}\u{201d} hat keinen Wert: Widget verbinden oder Wert eingeben."))),
            }
        }
    }
    Ok(WidgetInputs::new(widget, kind, resolved))
}
//#endregion 🔖️Resolve
