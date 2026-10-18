//! 🧮️ Values and dictionaries of the typed snapshot owner: the JSON and DSL value assignment, the number and boolean scalars, the dictionary containers and the record start. See [`super`].

use super::mounted_owner_state::{
    Generation2dMountedContainerOwner, Generation2dMountedDictionaryDestination, Generation2dMountedDictionaryEntryOwner, Generation2dMountedDslFrame, Generation2dMountedJsonFrame, Generation2dMountedNeuralOwner, Generation2dMountedRecordOwner,
    Generation2dMountedTypedSnapshotOwner, Generation2dMountedWidgetOwner,
};
use store::mounted_pack_rt as mounted;

//#region 🔖️MountedCanonicalPackSession
impl Generation2dMountedTypedSnapshotOwner {
    pub(super) fn assign_json(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.json_stack.last_mut() {
            Some(Generation2dMountedJsonFrame::Array(values)) => values.push(value),
            Some(Generation2dMountedJsonFrame::Object { values, key }) => {
                values.push((key.take().ok_or("generation2d-mounted.json-value-key")?, value));
            }
            None => {
                let (table, row) = self.json_destination.take().ok_or("generation2d-mounted.json-destination")?;
                let values = match value {
                    semio_framework_value::DslValue::Object(values) => values,
                    _ => return Err("generation2d-mounted.generation-values-shape"),
                };
                match self.stack.get_mut(table) {
                    Some(Generation2dMountedContainerOwner::Generations { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mounted.generation-row")?.values = values,
                    _ => return Err("generation2d-mounted.generation-values-table"),
                }
            }
        }
        Ok(())
    }

    pub(super) fn assign_dsl(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.dsl_stack.last_mut() {
            Some(Generation2dMountedDslFrame::Array(values)) => values.push(value),
            Some(Generation2dMountedDslFrame::Object { values, key }) => values.push((key.take().ok_or("generation2d-mounted.dsl-value-key")?, value)),
            None => {
                let (parent, slot) = self.dsl_destination.take().ok_or("generation2d-mounted.dsl-destination")?;
                match self.stack.get_mut(parent) {
                    Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field, .. }) if *field == Some((slot + 2) as u16) => {
                        widget.dynamic[slot] = Some(value);
                        *field = None;
                    }
                    _ => return Err("generation2d-mounted.dsl-widget-owner"),
                }
            }
        }
        Ok(())
    }

    pub(super) fn end_dsl(&mut self, kind: mounted::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.dsl_destination.is_none() {
            return Ok(false);
        }
        let value = match self.dsl_stack.pop().ok_or("generation2d-mounted.dsl-end")? {
            Generation2dMountedDslFrame::Array(values) if kind == mounted::RetainedValueContainer::List => semio_framework_value::DslValue::Array(values),
            Generation2dMountedDslFrame::Object { values, key: None } if kind == mounted::RetainedValueContainer::Map => semio_framework_value::DslValue::Object(values),
            _ => return Err("generation2d-mounted.dsl-container-mismatch"),
        };
        self.assign_dsl(value)?;
        Ok(true)
    }

    pub(super) fn begin_dsl(&mut self) -> bool {
        if self.dsl_destination.is_some() {
            return true;
        }
        let Some(parent) = self.stack.len().checked_sub(1) else { return false };
        let field = match self.stack.get(parent) {
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field: Some(field @ (2 | 3)), .. }) if widget.keyword == "cluster" => *field,
            _ => return false,
        };
        self.dsl_destination = Some((parent, usize::from(field - 2)));
        true
    }

    pub(super) fn assign_f64(&mut self, value: f64) -> Result<(), &'static str> {
        match self.stack.last_mut() {
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if *field == Some(3) && target.is_none() => {
                *target = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(value)));
                *field = None;
            }
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Camera(camera), field, .. }) => match field.take() {
                Some(0) => camera.x = value,
                Some(1) => camera.y = value,
                Some(2) => camera.zoom = value,
                _ => return Err("generation2d-mounted.camera-field"),
            },
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field, .. }) => {
                let field = field.take().ok_or("generation2d-mounted.widget-number-owner")?;
                *widget.numbers.get_mut(field.checked_sub(2).ok_or("generation2d-mounted.widget-number-field")? as usize).ok_or("generation2d-mounted.widget-number-field")? = value;
            }
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Layout { value: layout, .. }, field, .. }) => match field.take() {
                Some(0) => layout.x = value,
                Some(1) => layout.y = value,
                _ => return Err("generation2d-mounted.layout-field"),
            },
            _ => return Err("generation2d-mounted.number-owner"),
        }
        Ok(())
    }

    pub(super) fn assign_bool(&mut self, value: bool) {
        match self.stack.last_mut() {
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field, .. }) => {
                widget.boolean = value;
                *field = None;
            }
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if matches!(*field, Some(0 | 1)) && target.is_none() => {
                *target = Some(if *field == Some(0) {
                    semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null)
                } else {
                    semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(value))
                });
                *field = None;
            }
            _ => {}
        }
    }

    pub(super) fn end_json(&mut self, kind: mounted::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.json_destination.is_none() {
            return Ok(false);
        }
        let matches = matches!((self.json_stack.last(), kind), (Some(Generation2dMountedJsonFrame::Array(_)), mounted::RetainedValueContainer::List) | (Some(Generation2dMountedJsonFrame::Object { .. }), mounted::RetainedValueContainer::Map));
        if !matches {
            return Err("generation2d-mounted.json-container-mismatch");
        }
        let value = match self.json_stack.pop().ok_or("generation2d-mounted.json-end")? {
            Generation2dMountedJsonFrame::Array(values) => semio_framework_value::DslValue::Array(values),
            Generation2dMountedJsonFrame::Object { values, key: None } => semio_framework_value::DslValue::Object(values),
            Generation2dMountedJsonFrame::Object { .. } => return Err("generation2d-mounted.json-key-without-value"),
        };
        self.assign_json(value)?;
        Ok(true)
    }

    pub(super) fn begin_dictionary(&mut self, destination: Generation2dMountedDictionaryDestination, count: u64) -> Result<(), &'static str> {
        let rows = usize::try_from(count).map_err(|_| "generation2d-mounted.dictionary-count")?;
        let mut values = Vec::new();
        values.try_reserve_exact(rows).map_err(|_| "generation2d-mounted.dictionary-preflight")?;
        values.resize_with(rows, Generation2dMountedDictionaryEntryOwner::default);
        let mut present = Vec::new();
        present.try_reserve_exact(rows).map_err(|_| "generation2d-mounted.dictionary-presence-preflight")?;
        present.resize(rows, false);
        self.push(Generation2dMountedContainerOwner::Dictionary { destination, rows: values, field: None, present, next: 0 })
    }

    pub(super) fn finish_dictionary(&mut self, destination: Generation2dMountedDictionaryDestination, rows: Vec<Generation2dMountedDictionaryEntryOwner>) -> Result<(), &'static str> {
        let mut dictionary = semio_framework_artifact_flow_flow::neural::Dictionary::new();
        for row in rows {
            dictionary = dictionary.insert(row.key, row.value.ok_or("generation2d-mounted.dictionary-value")?);
        }
        match destination {
            Generation2dMountedDictionaryDestination::Widget { parent, field } => match self.stack.get_mut(parent) {
                Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field: active, .. }) if *active == Some(field) => {
                    widget.dictionaries[if field == 1 { 1 } else { 0 }] = dictionary;
                    *active = None;
                }
                _ => return Err("generation2d-mounted.dictionary-widget-owner"),
            },
            Generation2dMountedDictionaryDestination::Value { parent } => match self.stack.get_mut(parent) {
                Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { value, .. }, field, .. }) if *field == Some(5) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Dictionary(dictionary));
                    *field = None;
                }
                _ => return Err("generation2d-mounted.dictionary-value-owner"),
            },
        }
        Ok(())
    }

    pub(super) fn begin_record(&mut self, count: u64) -> Result<(), &'static str> {
        if count > 64 {
            return Err("generation2d-mounted.record-field-count");
        }
        let root_field = self.current_root_field();
        if let Some(table) = self.stack.len().checked_sub(1) {
            if let Some(Generation2dMountedContainerOwner::Dictionary { field: Some(1), present, next, .. }) = self.stack.get_mut(table) {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mounted.dictionary-value-row")?;
                *next = row + 1;
                let owner = Generation2dMountedNeuralOwner::TableRow { table, row };
                return self.push(Generation2dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner: Generation2dMountedRecordOwner::NeuralValue { owner, value: None } });
            }
            if let Some(Generation2dMountedContainerOwner::DictionaryEntries { rows, .. }) = self.stack.get_mut(table) {
                let row = rows.len();
                rows.try_reserve(1).map_err(|_| "generation2d-mounted.dictionary-entry-preflight")?;
                rows.push(Generation2dMountedDictionaryEntryOwner::default());
                return self.push(Generation2dMountedContainerOwner::DictionaryEntry { entries: table, row, field: None });
            }
            if let Some(Generation2dMountedContainerOwner::DictionaryEntry { entries, row, field: Some(1) }) = self.stack.get_mut(table) {
                let owner = Generation2dMountedNeuralOwner::EntryRow { entries: *entries, row: *row };
                return self.push(Generation2dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner: Generation2dMountedRecordOwner::NeuralValue { owner, value: None } });
            }
        }
        let owner = if self.stack.is_empty() {
            Generation2dMountedRecordOwner::Root
        } else if root_field == Some(1) {
            Generation2dMountedRecordOwner::Camera(semio_framework_artifact_flow_flow::CameraJson::default())
        } else if root_field == Some(2) {
            let keyword = match self.stack.last_mut() {
                Some(Generation2dMountedContainerOwner::Statements { keyword, .. }) => keyword.take().ok_or("generation2d-mounted.widget-keyword")?,
                _ => return Err("generation2d-mounted.widget-statements-owner"),
            };
            Generation2dMountedRecordOwner::Widget(Generation2dMountedWidgetOwner { keyword, ..Generation2dMountedWidgetOwner::default() })
        } else if root_field == Some(4) {
            let key = match self.stack.last_mut() {
                Some(Generation2dMountedContainerOwner::LayoutMap { key }) => key.take().ok_or("generation2d-mounted.layout-key")?,
                _ => return Err("generation2d-mounted.layout-map-owner"),
            };
            Generation2dMountedRecordOwner::Layout { key, value: semio_framework_artifact_flow_flow::WidgetLayout { x: 0.0, y: 0.0 } }
        } else {
            Generation2dMountedRecordOwner::Structural
        };
        self.push(Generation2dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner })
    }

}
//#endregion 🔖️MountedCanonicalPackSession
