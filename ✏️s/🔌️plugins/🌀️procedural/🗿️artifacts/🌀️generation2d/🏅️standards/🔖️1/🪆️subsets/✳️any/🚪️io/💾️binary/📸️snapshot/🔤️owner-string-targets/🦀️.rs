//! 🔤️ String targets of the typed snapshot owner: the owner constructor, the container stack, the root field cursor and the string and symbol assignment. See [`super`].

use super::mounted_owner_state::{
    Generation2dMountedContainerOwner, Generation2dMountedDslFrame, Generation2dMountedJsonFrame, Generation2dMountedRecordOwner, Generation2dMountedStringOwner, Generation2dMountedStringTarget, Generation2dMountedTypedSnapshotOwner,
    GENERATION2D_MOUNTED_TYPED_DEPTH,
};
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use store::mounted_pack_rt as mounted;

//#region 🔖️MountedCanonicalPackSession
impl Generation2dMountedTypedSnapshotOwner {
    pub(super) fn new() -> Result<Self, &'static str> {
        let mut stack = Vec::new();
        stack.try_reserve_exact(GENERATION2D_MOUNTED_TYPED_DEPTH).map_err(|_| "generation2d-mounted.typed-stack-preflight")?;
        let mut json_stack = Vec::new();
        json_stack.try_reserve_exact(GENERATION2D_MOUNTED_TYPED_DEPTH).map_err(|_| "generation2d-mounted.json-stack-preflight")?;
        let mut dsl_stack = Vec::new();
        dsl_stack.try_reserve_exact(GENERATION2D_MOUNTED_TYPED_DEPTH).map_err(|_| "generation2d-mounted.dsl-stack-preflight")?;
        let candidate = Generation2dSnapshot {
            host_snapshot: semio_framework_artifact_flow_flow::FlowHostSnapshot {
                schema: String::new(),
                camera: semio_framework_artifact_flow_flow::CameraJson::default(),
                widgets: Vec::new(),
                synapses: Vec::new(),
                layout: semio_framework_artifact_flow_flow::OrderedMap::new(),
            },
            generation: semio_framework_artifact_playbook_playbook::GenerationPlayRoot::default(),
        };
        Ok(Self { candidate: std::mem::ManuallyDrop::new(Some(candidate)), stack, string: None, pending_table_rows: None, json_stack, json_destination: None, dsl_stack, dsl_destination: None, complete: false, handed_back: false })
    }

    pub(super) fn push(&mut self, owner: Generation2dMountedContainerOwner) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("generation2d-mounted.typed-depth");
        }
        self.stack.push(owner);
        Ok(())
    }

    pub(super) fn current_root_field(&self) -> Option<u16> {
        self.stack.iter().rev().find_map(|owner| match owner {
            Generation2dMountedContainerOwner::Record { root_field, field, .. } => root_field.or(*field),
            Generation2dMountedContainerOwner::Statements { root_field, .. } => Some(*root_field),
            Generation2dMountedContainerOwner::Strings { parent, .. } => self.stack.get(*parent).and_then(|owner| match owner {
                Generation2dMountedContainerOwner::Record { root_field, .. } => *root_field,
                _ => None,
            }),
            Generation2dMountedContainerOwner::Synapses { .. } | Generation2dMountedContainerOwner::Wire { .. } => Some(3),
            Generation2dMountedContainerOwner::LayoutMap { .. } => Some(4),
            Generation2dMountedContainerOwner::Generations { .. } => Some(7),
            Generation2dMountedContainerOwner::Dictionary { .. } | Generation2dMountedContainerOwner::DictionaryEntries { .. } | Generation2dMountedContainerOwner::DictionaryEntry { .. } => Some(2),
            Generation2dMountedContainerOwner::Structural { root_field, .. } => *root_field,
        })
    }

    fn string_target(&mut self) -> Result<Generation2dMountedStringTarget, &'static str> {
        let index = self.stack.len().checked_sub(1).ok_or("generation2d-mounted.string-without-owner")?;
        if self.json_destination.is_some() {
            return Ok(match self.json_stack.last() {
                Some(Generation2dMountedJsonFrame::Object { key: None, .. }) => Generation2dMountedStringTarget::JsonKey,
                _ => Generation2dMountedStringTarget::JsonValue,
            });
        }
        if self.dsl_destination.is_some() {
            return Ok(match self.dsl_stack.last() {
                Some(Generation2dMountedDslFrame::Object { key: None, .. }) => Generation2dMountedStringTarget::DslKey,
                _ => Generation2dMountedStringTarget::DslValue,
            });
        }
        match &mut self.stack[index] {
            Generation2dMountedContainerOwner::Record { root_field: None, field: Some(field), .. } => Ok(Generation2dMountedStringTarget::Root(*field)),
            Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { .. }, field: Some(4), .. } => Ok(Generation2dMountedStringTarget::NeuralText(index)),
            Generation2dMountedContainerOwner::DictionaryEntry { field: Some(0), .. } => Ok(Generation2dMountedStringTarget::DictionaryEntryKey(index)),
            Generation2dMountedContainerOwner::Record { field: Some(field), .. } => Ok(Generation2dMountedStringTarget::Record(index, *field)),
            Generation2dMountedContainerOwner::Statements { keyword: None, .. } => Ok(Generation2dMountedStringTarget::StatementKeyword(index)),
            Generation2dMountedContainerOwner::Strings { .. } => Ok(Generation2dMountedStringTarget::Sequence(index)),
            Generation2dMountedContainerOwner::Synapses { field: Some(field), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mounted.synapse-row")?;
                *next = row + 1;
                Ok(Generation2dMountedStringTarget::Synapse(index, row, *field as u8))
            }
            Generation2dMountedContainerOwner::LayoutMap { key: None } => Ok(Generation2dMountedStringTarget::LayoutKey(index)),
            Generation2dMountedContainerOwner::Generations { field: Some(field), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mounted.generation-row")?;
                *next = row + 1;
                Ok(Generation2dMountedStringTarget::Generation(index, row, *field as u8))
            }
            Generation2dMountedContainerOwner::Dictionary { field: Some(0), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mounted.dictionary-key-row")?;
                *next = row + 1;
                Ok(Generation2dMountedStringTarget::DictionaryKey(index, row))
            }
            Generation2dMountedContainerOwner::Wire { roles, roles_len, role, .. } if *role < *roles_len => {
                let target = roles[*role];
                *role += 1;
                Ok(Generation2dMountedStringTarget::Wire(index, target))
            }
            _ => Err("generation2d-mounted.string-owner-role"),
        }
    }

    pub(super) fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("generation2d-mounted.string-overlap");
        }
        self.string = Some(Generation2dMountedStringOwner { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    pub(super) fn begin_symbol(&mut self, symbol: u64, catalog: &mounted::RetainedPackCatalogCursor) -> Result<(), &'static str> {
        if self.string.is_none() {
            self.begin_string()?;
        }
        let chars = catalog.symbol_chars(symbol).map_err(|_| "generation2d-mounted.symref")?;
        let owner = self.string.as_mut().expect("P2 mounted string retained");
        owner.value.try_reserve_exact(chars).map_err(|_| "generation2d-mounted.symbol-preflight")?;
        owner.symbol = Some((symbol, 0, chars));
        if chars == 0 {
            self.finish_string()?;
        }
        Ok(())
    }


    pub(super) fn finish_string(&mut self) -> Result<(), &'static str> {
        let owner = self.string.take().ok_or("generation2d-mounted.string-handoff")?;
        match owner.target {
            Generation2dMountedStringTarget::Root(0) => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.host_snapshot.schema = owner.value;
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedStringTarget::Root(5) => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.generation.cold_builder_mut()?.selected_generation_id = Some(owner.value);
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedStringTarget::Root(6) => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.generation.cold_builder_mut()?.preview_text = Some(owner.value);
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedStringTarget::Record(index, field) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field: active, .. }) => {
                    *widget.strings.get_mut(field as usize).ok_or("generation2d-mounted.widget-string-field")? = owner.value;
                    *active = None;
                }
                _ => return Err("generation2d-mounted.record-string-owner"),
            },
            Generation2dMountedStringTarget::StatementKeyword(index) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::Statements { keyword, .. }) => *keyword = Some(owner.value),
                _ => return Err("generation2d-mounted.statement-keyword-owner"),
            },
            Generation2dMountedStringTarget::Sequence(index) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::Strings { values, .. }) => values.push(owner.value),
                _ => return Err("generation2d-mounted.sequence-string-owner"),
            },
            Generation2dMountedStringTarget::Synapse(index, row, 0) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::Synapses { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mounted.synapse-row")?.id = owner.value,
                _ => return Err("generation2d-mounted.synapse-string-owner"),
            },
            Generation2dMountedStringTarget::LayoutKey(index) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::LayoutMap { key }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation2d-mounted.layout-key-owner"),
            },
            Generation2dMountedStringTarget::Generation(index, row, 0) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::Generations { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mounted.generation-row")?.id = owner.value,
                _ => return Err("generation2d-mounted.generation-string-owner"),
            },
            Generation2dMountedStringTarget::Generation(index, row, 1) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::Generations { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mounted.generation-row")?.name = owner.value,
                _ => return Err("generation2d-mounted.generation-string-owner"),
            },
            Generation2dMountedStringTarget::DictionaryKey(index, row) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::Dictionary { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mounted.dictionary-key-row")?.key = owner.value,
                _ => return Err("generation2d-mounted.dictionary-key-owner"),
            },
            Generation2dMountedStringTarget::DictionaryEntryKey(index) => {
                let (entries, row) = match self.stack.get_mut(index) {
                    Some(Generation2dMountedContainerOwner::DictionaryEntry { entries, row, field }) => {
                        *field = None;
                        (*entries, *row)
                    }
                    _ => return Err("generation2d-mounted.dictionary-entry-owner"),
                };
                match self.stack.get_mut(entries) {
                    Some(Generation2dMountedContainerOwner::DictionaryEntries { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mounted.dictionary-entry-row")?.key = owner.value,
                    _ => return Err("generation2d-mounted.dictionary-entries-owner"),
                }
            }
            Generation2dMountedStringTarget::NeuralText(index) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { value, .. }, field, .. }) if *field == Some(4) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(owner.value)));
                    *field = None;
                }
                _ => return Err("generation2d-mounted.neural-text-owner"),
            },
            Generation2dMountedStringTarget::JsonKey => match self.json_stack.last_mut() {
                Some(Generation2dMountedJsonFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation2d-mounted.json-key-owner"),
            },
            Generation2dMountedStringTarget::JsonValue => self.assign_json(semio_framework_value::DslValue::String(owner.value))?,
            Generation2dMountedStringTarget::DslKey => match self.dsl_stack.last_mut() {
                Some(Generation2dMountedDslFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation2d-mounted.dsl-key-owner"),
            },
            Generation2dMountedStringTarget::DslValue => self.assign_dsl(semio_framework_value::DslValue::String(owner.value))?,
            Generation2dMountedStringTarget::Wire(index, role) => {
                let (table, row) = match self.stack.get(index) {
                    Some(Generation2dMountedContainerOwner::Wire { table, row, .. }) => (*table, *row),
                    _ => return Err("generation2d-mounted.wire-owner"),
                };
                let synapse = match self.stack.get_mut(table) {
                    Some(Generation2dMountedContainerOwner::Synapses { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mounted.wire-row")?,
                    _ => return Err("generation2d-mounted.wire-table"),
                };
                match role {
                    0 => synapse.from = owner.value,
                    2 => synapse.from_port = owner.value,
                    3 => synapse.to = owner.value,
                    5 => synapse.to_port = owner.value,
                    _ => drop(owner.value),
                }
            }
            _ => return Err("generation2d-mounted.string-field"),
        }
        Ok(())
    }

}
//#endregion 🔖️MountedCanonicalPackSession
