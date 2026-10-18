//! 🗃️ Containers and widgets of the typed snapshot owner: the container start, the widget assembly and the container end. See [`super`].

use super::mounted_owner_state::{
    Generation2dMountedContainerOwner, Generation2dMountedDictionaryDestination, Generation2dMountedGenerationOwner, Generation2dMountedJsonFrame, Generation2dMountedNeuralOwner, Generation2dMountedRecordOwner, Generation2dMountedSynapseOwner,
    Generation2dMountedTypedSnapshotOwner, Generation2dMountedWidgetOwner, GENERATION2D_REQUIRED_SNAPSHOT_FIELDS,
};
use store::mounted_pack_rt as mounted;

//#region 🔖️MountedCanonicalPackSession
impl Generation2dMountedTypedSnapshotOwner {
    pub(super) fn begin_container(&mut self, kind: mounted::RetainedValueContainer, count: u64) -> Result<(), &'static str> {
        let root_field = self.current_root_field().ok_or("generation2d-mounted.container-root")?;
        match kind {
            mounted::RetainedValueContainer::Statements => self.push(Generation2dMountedContainerOwner::Statements { root_field, keyword: None }),
            mounted::RetainedValueContainer::Table if root_field == 2 => {
                let parent = self.stack.len().checked_sub(1).ok_or("generation2d-mounted.dictionary-parent")?;
                let destination = match self.stack.get(parent) {
                    Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(_), field: Some(field @ (1 | 5)), .. }) => Generation2dMountedDictionaryDestination::Widget { parent, field: *field },
                    Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { .. }, field: Some(5), .. }) => Generation2dMountedDictionaryDestination::Value { parent },
                    _ => return self.push(Generation2dMountedContainerOwner::Structural { kind, root_field: Some(root_field) }),
                };
                self.begin_dictionary(destination, count)
            }
            mounted::RetainedValueContainer::List | mounted::RetainedValueContainer::Tuple if root_field == 2 => {
                let (parent, field) = match self.stack.last() {
                    Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { value: None, .. }, field: Some(5), .. }) => {
                        let parent = self.stack.len() - 1;
                        let mut rows = Vec::new();
                        rows.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mounted.dictionary-entry-count")?).map_err(|_| "generation2d-mounted.dictionary-entry-preflight")?;
                        return self.push(Generation2dMountedContainerOwner::DictionaryEntries { destination: Generation2dMountedDictionaryDestination::Value { parent }, rows });
                    }
                    Some(Generation2dMountedContainerOwner::Record { field: Some(field), .. }) => (self.stack.len() - 1, *field),
                    _ => return Err("generation2d-mounted.sequence-owner"),
                };
                let mut values = Vec::new();
                values.try_reserve_exact(count as usize).map_err(|_| "generation2d-mounted.sequence-preflight")?;
                self.push(Generation2dMountedContainerOwner::Strings { parent, field, values })
            }
            mounted::RetainedValueContainer::Table if root_field == 3 => {
                let rows = usize::try_from(count).map_err(|_| "generation2d-mounted.synapse-count")?;
                let mut values = Vec::new();
                values.try_reserve_exact(rows).map_err(|_| "generation2d-mounted.synapse-preflight")?;
                values.resize_with(rows, Generation2dMountedSynapseOwner::default);
                let mut present = Vec::new();
                present.try_reserve_exact(rows).map_err(|_| "generation2d-mounted.synapse-presence-preflight")?;
                present.resize(rows, false);
                self.push(Generation2dMountedContainerOwner::Synapses { rows: values, field: None, present, next: 0 })
            }
            mounted::RetainedValueContainer::Map if root_field == 4 => self.push(Generation2dMountedContainerOwner::LayoutMap { key: None }),
            mounted::RetainedValueContainer::Table if root_field == 7 => {
                let rows = usize::try_from(count).map_err(|_| "generation2d-mounted.generation-count")?;
                let mut values = Vec::new();
                values.try_reserve_exact(rows).map_err(|_| "generation2d-mounted.generation-preflight")?;
                values.resize_with(rows, Generation2dMountedGenerationOwner::default);
                let mut present = Vec::new();
                present.try_reserve_exact(rows).map_err(|_| "generation2d-mounted.generation-presence-preflight")?;
                present.resize(rows, false);
                self.push(Generation2dMountedContainerOwner::Generations { rows: values, field: None, present, next: 0 })
            }
            mounted::RetainedValueContainer::Map if root_field == 7 => {
                let table = self.stack.len().checked_sub(1).ok_or("generation2d-mounted.generation-table")?;
                let row = match self.stack.get_mut(table) {
                    Some(Generation2dMountedContainerOwner::Generations { field: Some(2), present, next, .. }) => {
                        let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mounted.generation-row")?;
                        *next = row + 1;
                        row
                    }
                    _ => return Err("generation2d-mounted.generation-values-owner"),
                };
                self.json_destination = Some((table, row));
                self.json_stack.push(Generation2dMountedJsonFrame::Object { values: Vec::new(), key: None });
                Ok(())
            }
            mounted::RetainedValueContainer::Wire if root_field == 3 => {
                let table = self.stack.len().checked_sub(1).ok_or("generation2d-mounted.wire-table")?;
                let row = match self.stack.get_mut(table) {
                    Some(Generation2dMountedContainerOwner::Synapses { present, next, .. }) => {
                        let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mounted.wire-row")?;
                        *next = row + 1;
                        row
                    }
                    _ => return Err("generation2d-mounted.wire-table"),
                };
                self.push(Generation2dMountedContainerOwner::Wire { table, row, roles: [0; 6], roles_len: 0, role: 0, nodes: 0 })
            }
            _ => self.push(Generation2dMountedContainerOwner::Structural { kind, root_field: Some(root_field) }),
        }
    }

    fn finish_widget(owner: Generation2dMountedWidgetOwner) -> Result<semio_framework_artifact_flow_flow::Widget, &'static str> {
        let [id, second, third, _fourth] = owner.strings;
        let [value, min, max, step] = owner.numbers;
        let [first_list, second_list] = owner.lists;
        let [first_dictionary, second_dictionary] = owner.dictionaries;
        let [first_dynamic, second_dynamic] = owner.dynamic;
        Ok(match owner.keyword.as_str() {
            "neuron" => semio_framework_artifact_flow_flow::Widget::Neuron { id, neuron_kind: second, params: first_dictionary, input_ports: first_list, output_ports: second_list, preview: owner.boolean },
            "input-slider" => semio_framework_artifact_flow_flow::Widget::InputSlider { id, label: second, value, min, max, step },
            "input-note" => semio_framework_artifact_flow_flow::Widget::InputNote { id, text: second },
            "input-image" => semio_framework_artifact_flow_flow::Widget::InputImage { id, src: second },
            "variable" => semio_framework_artifact_flow_flow::Widget::Variable { id, name: second, schema: third },
            "output-preview" => {
                let mut expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
                for entry in first_list {
                    expanded.insert(entry);
                }
                semio_framework_artifact_flow_flow::Widget::OutputPreview { id, preview: second_dictionary, expanded }
            }
            "output-action" => semio_framework_artifact_flow_flow::Widget::OutputAction { id, action: second },
            "output-export" => semio_framework_artifact_flow_flow::Widget::OutputExport { id, format: second },
            "cluster" => semio_framework_artifact_flow_flow::Widget::Cluster {
                id,
                name: second,
                tree: semio_framework_value::FromValue::from_value(first_dynamic.ok_or("generation2d-mounted.cluster-tree")?).map_err(|_| "generation2d-mounted.cluster-tree-shape")?,
                flow: semio_framework_value::FromValue::from_value(second_dynamic.ok_or("generation2d-mounted.cluster-flow")?).map_err(|_| "generation2d-mounted.cluster-flow-shape")?,
            },
            _ => return Err("generation2d-mounted.widget-variant"),
        })
    }

    pub(super) fn end(&mut self, kind: mounted::RetainedValueContainer) -> Result<(), &'static str> {
        let owner = self.stack.pop().ok_or("generation2d-mounted.end-without-owner")?;
        match owner {
            Generation2dMountedContainerOwner::Record { root_field: None, seen, owner: Generation2dMountedRecordOwner::Root, field: None } => {
                if seen & GENERATION2D_REQUIRED_SNAPSHOT_FIELDS != GENERATION2D_REQUIRED_SNAPSHOT_FIELDS {
                    return Err("generation2d-mounted.snapshot-fields-missing");
                }
            }
            Generation2dMountedContainerOwner::Record { root_field: Some(1), owner: Generation2dMountedRecordOwner::Camera(camera), field: None, .. } => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.host_snapshot.camera = camera;
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedContainerOwner::Record { root_field: Some(2), owner: Generation2dMountedRecordOwner::Widget(widget), field: None, .. } => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.host_snapshot.widgets.push(Self::finish_widget(widget)?);
            }
            Generation2dMountedContainerOwner::Record { root_field: Some(4), owner: Generation2dMountedRecordOwner::Layout { key, value }, field: None, .. } => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.host_snapshot.layout.insert(key, value);
            }
            Generation2dMountedContainerOwner::Strings { parent, field, values } => match self.stack.get_mut(parent) {
                Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field: active, .. }) => {
                    *widget.lists.get_mut(if field == 4 { 1 } else { 0 }).ok_or("generation2d-mounted.widget-list-field")? = values;
                    *active = None;
                }
                _ => return Err("generation2d-mounted.widget-list-owner"),
            },
            Generation2dMountedContainerOwner::Synapses { rows, .. } if kind == mounted::RetainedValueContainer::Table => {
                let target = &mut self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.host_snapshot.synapses;
                for row in rows {
                    target.push(semio_framework_artifact_flow_flow::SynapseSpec { id: row.id, from: row.from, to: row.to, from_port: row.from_port, to_port: row.to_port });
                }
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedContainerOwner::LayoutMap { key: None } if kind == mounted::RetainedValueContainer::Map => {
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedContainerOwner::Generations { rows, .. } if kind == mounted::RetainedValueContainer::Table => {
                let target = &mut self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.generation.cold_builder_mut()?.generations;
                for row in rows {
                    target.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: row.id, name: row.name, values: row.values.into_iter().collect() });
                }
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedContainerOwner::Record { root_field: Some(2), field: None, owner: Generation2dMountedRecordOwner::NeuralValue { owner: Generation2dMountedNeuralOwner::TableRow { table, row }, value: Some(value) }, .. } if kind == mounted::RetainedValueContainer::Record => {
                match self.stack.get_mut(table) {
                    Some(Generation2dMountedContainerOwner::Dictionary { rows, field: Some(1), .. }) => {
                        rows.get_mut(row).ok_or("generation2d-mounted.dictionary-value-row")?.value = Some(value);
                    }
                    _ => return Err("generation2d-mounted.dictionary-value-table"),
                }
            }
            Generation2dMountedContainerOwner::Record { root_field: Some(2), field: None, owner: Generation2dMountedRecordOwner::NeuralValue { owner: Generation2dMountedNeuralOwner::EntryRow { entries, row }, value: Some(value) }, .. } if kind == mounted::RetainedValueContainer::Record => {
                match self.stack.get_mut(entries) {
                    Some(Generation2dMountedContainerOwner::DictionaryEntries { rows, .. }) => {
                        rows.get_mut(row).ok_or("generation2d-mounted.dictionary-entry-row")?.value = Some(value);
                    }
                    _ => return Err("generation2d-mounted.dictionary-entry-list"),
                }
                if let Some(Generation2dMountedContainerOwner::DictionaryEntry { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedContainerOwner::DictionaryEntry { field: None, .. } if kind == mounted::RetainedValueContainer::Record => {}
            Generation2dMountedContainerOwner::DictionaryEntries { destination, rows } if matches!(kind, mounted::RetainedValueContainer::List | mounted::RetainedValueContainer::Tuple) => self.finish_dictionary(destination, rows)?,
            Generation2dMountedContainerOwner::Dictionary { destination, rows, field: Some(1), .. } if kind == mounted::RetainedValueContainer::Table => self.finish_dictionary(destination, rows)?,
            Generation2dMountedContainerOwner::Wire { .. } if kind == mounted::RetainedValueContainer::Wire => {}
            Generation2dMountedContainerOwner::Statements { .. } => {
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedContainerOwner::Structural { kind: expected, .. } if expected == kind => {
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Structural, .. } => {}
            _ => return Err("generation2d-mounted.container-owner-mismatch"),
        }
        Ok(())
    }




}
//#endregion 🔖️MountedCanonicalPackSession
