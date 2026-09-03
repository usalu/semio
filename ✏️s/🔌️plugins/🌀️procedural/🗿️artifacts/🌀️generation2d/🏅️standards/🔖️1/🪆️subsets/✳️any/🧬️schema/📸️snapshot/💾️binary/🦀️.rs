//! 📦️ Generation2d artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::generation2d::Generation2dSnapshot;
#[cfg(test)]
use store::PackError;

/// 📦️ Encodes a `Generation2dSnapshot` to its binary pack form.
pub fn encode(document: &Generation2dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 🔬️ Batch decode exists only for constitutional equivalence tests; mounted UI code
/// has no production symbol that can reach the whole-document decoder.
#[cfg(test)]
pub fn decode(bytes: &[u8]) -> Result<Generation2dSnapshot, PackError> {
    <Generation2dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::generation2d::dsl as generation2d_dsl;
    use flow::Widget;
    use semio_framework_os_kernel::os_store::test_support;

    #[test]
    fn dsl_pack_equivalence_empty_projection() {
        test_support::assert_dsl_pack_equivalence(&Generation2dSnapshot::default());
    }

    #[test]
    fn dsl_pack_equivalence_example_fixture() {
        let projection = generation2d_dsl::parse_dsl(generation2d_dsl::GENERATION2D_EXAMPLE_TEXT).expect("parse 🌀️default.generation2d fixture");
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_with_generation_state() {
        let mut projection = Generation2dSnapshot::default();
        let mut values: flow::playbook::PlaybookValues = std::collections::HashMap::new();
        // 🌱️ Fractional (not whole-number) so `dsl::from_dsl_value`'s int-normalization of whole
        // `DslValue::Number`s (an engine-owned behavior, see the sibling dsl test) doesn't make this
        // round trip spuriously unequal.
        values.insert("count".into(), dsl::DslValue::float(3.5));
        projection.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(flow::playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some("generation-1".into());
        projection.generation.cold_builder_mut().expect("unique cold generation owner").preview_text = Some("42".into());
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_covers_every_widget_kind() {
        let mut projection = Generation2dSnapshot::default();
        projection.fixture.widgets = vec![
            Widget::InputSlider { id: "slider".into(), label: "Number".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
            Widget::InputImage { id: "image".into(), src: "data:image/png;base64,abc".into() },
            Widget::Variable { id: "variable".into(), name: "value".into(), schema: "dictionary".into() },
            Widget::OutputAction { id: "action".into(), action: "export".into() },
            Widget::OutputExport { id: "export".into(), format: "svg".into() },
            Widget::Cluster { id: "cluster".into(), name: "Group".into(), tree: Default::default(), flow: Default::default() },
        ];
        projection.fixture.synapses = vec![];
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn pack_round_trips() {
        let projection = generation2d_dsl::parse_dsl(generation2d_dsl::GENERATION2D_EXAMPLE_TEXT).expect("parse fixture");
        let bytes = encode(&projection);
        assert!(bytes.starts_with(b"P2D2"));
        assert_eq!(decode(&bytes).expect("decode"), projection);
        let mut wrong = bytes;
        wrong[..4].copy_from_slice(b"P3D3");
        assert!(decode(&wrong).is_err());
    }
}
//#endregion 🧪️Tests

use store::mounted_pack_rt as mounted;

//#region 🔖️MountedCanonicalPackSession
const GENERATION2D_MOUNTED_PREFIX: [u8; 4] = *b"P2D2";
const GENERATION2D_MOUNTED_TYPED_DEPTH: usize = 12;
const GENERATION2D_REQUIRED_SNAPSHOT_FIELDS: u16 = 0b1001_1111;

#[derive(Default)]
struct Generation2dMountedWidgetOwner {
    keyword: String,
    strings: [String; 4],
    numbers: [f64; 4],
    boolean: bool,
    lists: [Vec<String>; 2],
    dictionaries: [flow::neural::Dictionary; 2],
    dynamic: [Option<dsl::DslValue>; 2],
}

#[derive(Default)]
struct Generation2dMountedSynapseOwner {
    id: String,
    from: String,
    to: String,
    from_port: String,
    to_port: String,
}

#[derive(Default)]
struct Generation2dMountedGenerationOwner {
    id: String,
    name: String,
    values: Vec<(String, dsl::DslValue)>,
}

#[derive(Default)]
struct Generation2dMountedDictionaryEntryOwner {
    key: String,
    value: Option<flow::neural::Value>,
}

#[derive(Clone, Copy)]
enum Generation2dMountedDictionaryDestination {
    Widget { parent: usize, field: u16 },
    Value { parent: usize },
}

enum Generation2dMountedRecordOwner {
    Root,
    Camera(flow::CameraJson),
    Layout { key: String, value: flow::WidgetLayout },
    Widget(Generation2dMountedWidgetOwner),
    NeuralValue { table: usize, row: usize, value: Option<flow::neural::Value> },
    Structural,
}

enum Generation2dMountedContainerOwner {
    Record { root_field: Option<u16>, field: Option<u16>, seen: u16, owner: Generation2dMountedRecordOwner },
    Statements { root_field: u16, keyword: Option<String> },
    Strings { parent: usize, field: u16, values: Vec<String> },
    Synapses { rows: Vec<Generation2dMountedSynapseOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    LayoutMap { key: Option<String> },
    Generations { rows: Vec<Generation2dMountedGenerationOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    Dictionary { destination: Generation2dMountedDictionaryDestination, rows: Vec<Generation2dMountedDictionaryEntryOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    Wire { table: usize, row: usize, roles: [u8; 6], roles_len: usize, role: usize, nodes: usize },
    Structural { kind: mounted::RetainedValueContainer, root_field: Option<u16> },
}

#[derive(Clone, Copy)]
enum Generation2dMountedStringTarget {
    Root(u16),
    Record(usize, u16),
    StatementKeyword(usize),
    Sequence(usize),
    Synapse(usize, usize, u8),
    LayoutKey(usize),
    Generation(usize, usize, u8),
    DictionaryKey(usize, usize),
    NeuralText(usize),
    JsonKey,
    JsonValue,
    DslKey,
    DslValue,
    Wire(usize, u8),
}

struct Generation2dMountedStringOwner {
    target: Generation2dMountedStringTarget,
    value: String,
    remaining: Option<u64>,
    symbol: Option<(u64, usize, usize)>,
}

enum Generation2dMountedJsonFrame {
    Array(Vec<dsl::DslValue>),
    Object { values: Vec<(String, dsl::DslValue)>, key: Option<String> },
}

enum Generation2dMountedDslFrame {
    Array(Vec<dsl::DslValue>),
    Object { values: Vec<(String, dsl::DslValue)>, key: Option<String> },
}

/// 🧬️ Fixed-depth schema owner consuming catalog/value events directly into P2 domain
/// fields, with one scalar byte opportunity per retained grant. It has no generic record tree
/// and cannot invoke a batch pack decoder.
struct Generation2dMountedTypedSnapshotOwner {
    candidate: std::mem::ManuallyDrop<Option<Generation2dSnapshot>>,
    stack: Vec<Generation2dMountedContainerOwner>,
    string: Option<Generation2dMountedStringOwner>,
    pending_table_rows: Option<u64>,
    json_stack: Vec<Generation2dMountedJsonFrame>,
    json_destination: Option<(usize, usize)>,
    dsl_stack: Vec<Generation2dMountedDslFrame>,
    dsl_destination: Option<(usize, usize)>,
    complete: bool,
    handed_back: bool,
}

impl Generation2dMountedTypedSnapshotOwner {
    fn new() -> Result<Self, &'static str> {
        let mut stack = Vec::new();
        stack.try_reserve_exact(GENERATION2D_MOUNTED_TYPED_DEPTH).map_err(|_| "generation2d-mounted.typed-stack-preflight")?;
        let mut json_stack = Vec::new();
        json_stack.try_reserve_exact(GENERATION2D_MOUNTED_TYPED_DEPTH).map_err(|_| "generation2d-mounted.json-stack-preflight")?;
        let mut dsl_stack = Vec::new();
        dsl_stack.try_reserve_exact(GENERATION2D_MOUNTED_TYPED_DEPTH).map_err(|_| "generation2d-mounted.dsl-stack-preflight")?;
        let candidate = Generation2dSnapshot {
            fixture: flow::FlowFixture { schema: String::new(), camera: flow::CameraJson::default(), widgets: Vec::new(), synapses: Vec::new(), layout: flow::OrderedMap::new() },
            generation: flow::playbook::GenerationPlayRoot::default(),
        };
        Ok(Self { candidate: std::mem::ManuallyDrop::new(Some(candidate)), stack, string: None, pending_table_rows: None, json_stack, json_destination: None, dsl_stack, dsl_destination: None, complete: false, handed_back: false })
    }

    fn push(&mut self, owner: Generation2dMountedContainerOwner) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("generation2d-mounted.typed-depth");
        }
        self.stack.push(owner);
        Ok(())
    }

    fn current_root_field(&self) -> Option<u16> {
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
            Generation2dMountedContainerOwner::Dictionary { .. } => Some(2),
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

    fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("generation2d-mounted.string-overlap");
        }
        self.string = Some(Generation2dMountedStringOwner { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    fn begin_symbol(&mut self, symbol: u64, catalog: &mounted::RetainedPackCatalogCursor) -> Result<(), &'static str> {
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

    fn grant_symbol(&mut self, catalog: &mounted::RetainedPackCatalogCursor) -> Result<bool, &'static str> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, chars)) = owner.symbol else { return Ok(false) };
        owner.value.push(catalog.symbol_char(symbol, index).map_err(|_| "generation2d-mounted.symref-char")?.ok_or("generation2d-mounted.symref-short")?);
        if index + 1 == chars {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("P2 mounted symbol retained").symbol = Some((symbol, index + 1, chars));
        }
        Ok(true)
    }

    fn finish_string(&mut self) -> Result<(), &'static str> {
        let owner = self.string.take().ok_or("generation2d-mounted.string-handoff")?;
        match owner.target {
            Generation2dMountedStringTarget::Root(0) => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.fixture.schema = owner.value;
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
            Generation2dMountedStringTarget::NeuralText(index) => match self.stack.get_mut(index) {
                Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { value, .. }, field, .. }) if *field == Some(4) && value.is_none() => {
                    *value = Some(flow::neural::Value::Atom(flow::neural::Atom::String(owner.value)));
                    *field = None;
                }
                _ => return Err("generation2d-mounted.neural-text-owner"),
            },
            Generation2dMountedStringTarget::JsonKey => match self.json_stack.last_mut() {
                Some(Generation2dMountedJsonFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation2d-mounted.json-key-owner"),
            },
            Generation2dMountedStringTarget::JsonValue => self.assign_json(dsl::DslValue::String(owner.value))?,
            Generation2dMountedStringTarget::DslKey => match self.dsl_stack.last_mut() {
                Some(Generation2dMountedDslFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation2d-mounted.dsl-key-owner"),
            },
            Generation2dMountedStringTarget::DslValue => self.assign_dsl(dsl::DslValue::String(owner.value))?,
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
                    1 => synapse.from_port = owner.value,
                    3 => synapse.to = owner.value,
                    4 => synapse.to_port = owner.value,
                    _ => drop(owner.value),
                }
            }
            _ => return Err("generation2d-mounted.string-field"),
        }
        Ok(())
    }

    fn assign_json(&mut self, value: dsl::DslValue) -> Result<(), &'static str> {
        match self.json_stack.last_mut() {
            Some(Generation2dMountedJsonFrame::Array(values)) => values.push(value),
            Some(Generation2dMountedJsonFrame::Object { values, key }) => {
                values.push((key.take().ok_or("generation2d-mounted.json-value-key")?, value));
            }
            None => {
                let (table, row) = self.json_destination.take().ok_or("generation2d-mounted.json-destination")?;
                let values = match value {
                    dsl::DslValue::Object(values) => values,
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

    fn assign_dsl(&mut self, value: dsl::DslValue) -> Result<(), &'static str> {
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

    fn end_dsl(&mut self, kind: mounted::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.dsl_destination.is_none() {
            return Ok(false);
        }
        let value = match self.dsl_stack.pop().ok_or("generation2d-mounted.dsl-end")? {
            Generation2dMountedDslFrame::Array(values) if kind == mounted::RetainedValueContainer::List => dsl::DslValue::Array(values),
            Generation2dMountedDslFrame::Object { values, key: None } if kind == mounted::RetainedValueContainer::Map => dsl::DslValue::Object(values),
            _ => return Err("generation2d-mounted.dsl-container-mismatch"),
        };
        self.assign_dsl(value)?;
        Ok(true)
    }

    fn begin_dsl(&mut self) -> Result<bool, &'static str> {
        if self.dsl_destination.is_some() {
            return Ok(true);
        }
        let Some(parent) = self.stack.len().checked_sub(1) else { return Ok(false) };
        let field = match self.stack.get(parent) {
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field: Some(field @ (2 | 3)), .. }) if widget.keyword == "cluster" => *field,
            _ => return Ok(false),
        };
        self.dsl_destination = Some((parent, usize::from(field - 2)));
        Ok(true)
    }

    fn assign_f64(&mut self, value: f64) -> Result<(), &'static str> {
        match self.stack.last_mut() {
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if *field == Some(3) && target.is_none() => {
                *target = Some(flow::neural::Value::Atom(flow::neural::Atom::Decimal(value)));
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

    fn assign_bool(&mut self, value: bool) {
        match self.stack.last_mut() {
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field, .. }) => {
                widget.boolean = value;
                *field = None;
            }
            Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if matches!(*field, Some(0 | 1)) && target.is_none() => {
                *target = Some(if *field == Some(0) { flow::neural::Value::Atom(flow::neural::Atom::Null) } else { flow::neural::Value::Atom(flow::neural::Atom::Boolean(value)) });
                *field = None;
            }
            _ => {}
        }
    }

    fn end_json(&mut self, kind: mounted::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.json_destination.is_none() {
            return Ok(false);
        }
        let matches = matches!((self.json_stack.last(), kind), (Some(Generation2dMountedJsonFrame::Array(_)), mounted::RetainedValueContainer::List) | (Some(Generation2dMountedJsonFrame::Object { .. }), mounted::RetainedValueContainer::Map));
        if !matches {
            return Err("generation2d-mounted.json-container-mismatch");
        }
        let value = match self.json_stack.pop().ok_or("generation2d-mounted.json-end")? {
            Generation2dMountedJsonFrame::Array(values) => dsl::DslValue::Array(values),
            Generation2dMountedJsonFrame::Object { values, key: None } => dsl::DslValue::Object(values),
            Generation2dMountedJsonFrame::Object { .. } => return Err("generation2d-mounted.json-key-without-value"),
        };
        self.assign_json(value)?;
        Ok(true)
    }

    fn begin_dictionary(&mut self, destination: Generation2dMountedDictionaryDestination, count: u64) -> Result<(), &'static str> {
        let rows = usize::try_from(count).map_err(|_| "generation2d-mounted.dictionary-count")?;
        let mut values = Vec::new();
        values.try_reserve_exact(rows).map_err(|_| "generation2d-mounted.dictionary-preflight")?;
        values.resize_with(rows, Generation2dMountedDictionaryEntryOwner::default);
        let mut present = Vec::new();
        present.try_reserve_exact(rows).map_err(|_| "generation2d-mounted.dictionary-presence-preflight")?;
        present.resize(rows, false);
        self.push(Generation2dMountedContainerOwner::Dictionary { destination, rows: values, field: None, present, next: 0 })
    }

    fn finish_dictionary(&mut self, destination: Generation2dMountedDictionaryDestination, rows: Vec<Generation2dMountedDictionaryEntryOwner>) -> Result<(), &'static str> {
        let mut dictionary = flow::neural::Dictionary::new();
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
                    *value = Some(flow::neural::Value::Dictionary(dictionary));
                    *field = None;
                }
                _ => return Err("generation2d-mounted.dictionary-value-owner"),
            },
        }
        Ok(())
    }

    fn begin_record(&mut self, count: u64) -> Result<(), &'static str> {
        if count > 64 {
            return Err("generation2d-mounted.record-field-count");
        }
        let root_field = self.current_root_field();
        if let Some(table) = self.stack.len().checked_sub(1) {
            if let Some(Generation2dMountedContainerOwner::Dictionary { field: Some(1), present, next, .. }) = self.stack.get_mut(table) {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mounted.dictionary-value-row")?;
                *next = row + 1;
                return self.push(Generation2dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner: Generation2dMountedRecordOwner::NeuralValue { table, row, value: None } });
            }
        }
        let owner = if self.stack.is_empty() {
            Generation2dMountedRecordOwner::Root
        } else if root_field == Some(1) {
            Generation2dMountedRecordOwner::Camera(flow::CameraJson::default())
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
            Generation2dMountedRecordOwner::Layout { key, value: flow::WidgetLayout { x: 0.0, y: 0.0 } }
        } else {
            Generation2dMountedRecordOwner::Structural
        };
        self.push(Generation2dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner })
    }

    fn begin_container(&mut self, kind: mounted::RetainedValueContainer, count: u64) -> Result<(), &'static str> {
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

    fn finish_widget(owner: Generation2dMountedWidgetOwner) -> Result<flow::Widget, &'static str> {
        let [id, second, third, _fourth] = owner.strings;
        let [value, min, max, step] = owner.numbers;
        let [first_list, second_list] = owner.lists;
        let [first_dictionary, second_dictionary] = owner.dictionaries;
        let [first_dynamic, second_dynamic] = owner.dynamic;
        Ok(match owner.keyword.as_str() {
            "neuron" => flow::Widget::Neuron { id, neuron_kind: second, params: first_dictionary, input_ports: first_list, output_ports: second_list, preview: owner.boolean },
            "input-slider" => flow::Widget::InputSlider { id, label: second, value, min, max, step },
            "input-note" => flow::Widget::InputNote { id, text: second },
            "input-image" => flow::Widget::InputImage { id, src: second },
            "variable" => flow::Widget::Variable { id, name: second, schema: third },
            "output-preview" => {
                let mut expanded = flow::OrderedSet::new();
                for entry in first_list {
                    expanded.insert(entry);
                }
                flow::Widget::OutputPreview { id, preview: second_dictionary, expanded }
            }
            "output-action" => flow::Widget::OutputAction { id, action: second },
            "output-export" => flow::Widget::OutputExport { id, format: second },
            "cluster" => flow::Widget::Cluster {
                id,
                name: second,
                tree: dsl::from_dsl_value(first_dynamic.ok_or("generation2d-mounted.cluster-tree")?).map_err(|_| "generation2d-mounted.cluster-tree-shape")?,
                flow: dsl::from_dsl_value(second_dynamic.ok_or("generation2d-mounted.cluster-flow")?).map_err(|_| "generation2d-mounted.cluster-flow-shape")?,
            },
            _ => return Err("generation2d-mounted.widget-variant"),
        })
    }

    fn end(&mut self, kind: mounted::RetainedValueContainer) -> Result<(), &'static str> {
        let owner = self.stack.pop().ok_or("generation2d-mounted.end-without-owner")?;
        match owner {
            Generation2dMountedContainerOwner::Record { root_field: None, seen, owner: Generation2dMountedRecordOwner::Root, field: None } => {
                if seen & GENERATION2D_REQUIRED_SNAPSHOT_FIELDS != GENERATION2D_REQUIRED_SNAPSHOT_FIELDS {
                    return Err("generation2d-mounted.snapshot-fields-missing");
                }
            }
            Generation2dMountedContainerOwner::Record { root_field: Some(1), owner: Generation2dMountedRecordOwner::Camera(camera), field: None, .. } => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.fixture.camera = camera;
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedContainerOwner::Record { root_field: Some(2), owner: Generation2dMountedRecordOwner::Widget(widget), field: None, .. } => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.fixture.widgets.push(Self::finish_widget(widget)?);
            }
            Generation2dMountedContainerOwner::Record { root_field: Some(4), owner: Generation2dMountedRecordOwner::Layout { key, value }, field: None, .. } => {
                self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.fixture.layout.insert(key, value);
            }
            Generation2dMountedContainerOwner::Strings { parent, field, values } => match self.stack.get_mut(parent) {
                Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::Widget(widget), field: active, .. }) => {
                    *widget.lists.get_mut(if field == 4 { 1 } else { 0 }).ok_or("generation2d-mounted.widget-list-field")? = values;
                    *active = None;
                }
                _ => return Err("generation2d-mounted.widget-list-owner"),
            },
            Generation2dMountedContainerOwner::Synapses { rows, .. } if kind == mounted::RetainedValueContainer::Table => {
                let target = &mut self.candidate.as_mut().ok_or("generation2d-mounted.snapshot-owner")?.fixture.synapses;
                for row in rows {
                    target.push(flow::SynapseSpec { id: row.id, from: row.from, to: row.to, from_port: row.from_port, to_port: row.to_port });
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
                    target.push(flow::playbook::FormGeneration { id: row.id, name: row.name, values: row.values.into_iter().collect() });
                }
                if let Some(Generation2dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMountedContainerOwner::Record { root_field: Some(2), field: None, owner: Generation2dMountedRecordOwner::NeuralValue { table, row, value: Some(value) }, .. } if kind == mounted::RetainedValueContainer::Record => {
                match self.stack.get_mut(table) {
                    Some(Generation2dMountedContainerOwner::Dictionary { rows, field: Some(1), .. }) => {
                        rows.get_mut(row).ok_or("generation2d-mounted.dictionary-value-row")?.value = Some(value);
                    }
                    _ => return Err("generation2d-mounted.dictionary-value-table"),
                }
            }
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

    fn accept(&mut self, token: mounted::RetainedValueToken, catalog: &mounted::RetainedPackCatalogCursor) -> Result<(), &'static str> {
        use mounted::{RetainedValueContainer as Container, RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Begin { kind: Container::Record, count } => self.begin_record(count)?,
            Token::Begin { kind, count } => {
                if self.dsl_destination.is_some() {
                    match kind {
                        Container::List => {
                            let mut values = Vec::new();
                            values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mounted.dsl-count")?).map_err(|_| "generation2d-mounted.dsl-preflight")?;
                            self.dsl_stack.push(Generation2dMountedDslFrame::Array(values));
                        }
                        Container::Map => {
                            let mut values = Vec::new();
                            values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mounted.dsl-count")?).map_err(|_| "generation2d-mounted.dsl-preflight")?;
                            self.dsl_stack.push(Generation2dMountedDslFrame::Object { values, key: None });
                        }
                        _ => return Err("generation2d-mounted.dsl-container"),
                    }
                    return Ok(());
                }
                if self.json_destination.is_some() {
                    match kind {
                        Container::List => {
                            let mut values = Vec::new();
                            values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mounted.json-count")?).map_err(|_| "generation2d-mounted.json-preflight")?;
                            self.json_stack.push(Generation2dMountedJsonFrame::Array(values));
                        }
                        Container::Map => self.json_stack.push(Generation2dMountedJsonFrame::Object { values: Vec::new(), key: None }),
                        _ => return Err("generation2d-mounted.json-container"),
                    }
                    return Ok(());
                }
                if kind == Container::Table && self.pending_table_rows.take() != Some(count) {
                    return Err("generation2d-mounted.table-row-count");
                }
                self.begin_container(kind, count)?;
            }
            Token::Unsigned { role: Role::FieldId, value } if value < 16 => match self.stack.last_mut() {
                Some(Generation2dMountedContainerOwner::Record { field, seen, .. }) if field.is_none() => {
                    *field = Some(value as u16);
                    *seen |= 1 << value;
                }
                _ => return Err("generation2d-mounted.field-owner"),
            },
            Token::Unsigned { role: Role::TableRows, value } => self.pending_table_rows = Some(value),
            Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
                Some(
                    Generation2dMountedContainerOwner::Synapses { field, present, next, .. } | Generation2dMountedContainerOwner::Generations { field, present, next, .. } | Generation2dMountedContainerOwner::Dictionary { field, present, next, .. },
                ) => {
                    *field = Some(u16::try_from(value).map_err(|_| "generation2d-mounted.table-field")?);
                    present.fill(false);
                    *next = 0;
                }
                _ => {}
            },
            Token::Tag { value: 0x06 | 0x07, .. } => self.begin_string()?,
            Token::Unsigned { role: Role::StringLength, value } => {
                let owner = self.string.as_mut().ok_or("generation2d-mounted.string-length-owner")?;
                owner.value.try_reserve_exact(value as usize).map_err(|_| "generation2d-mounted.string-preflight")?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::StringChar(value) => {
                let owner = self.string.as_mut().ok_or("generation2d-mounted.string-char-owner")?;
                owner.value.push(value);
                let remaining = owner.remaining.as_mut().ok_or("generation2d-mounted.string-char-length")?;
                *remaining = remaining.checked_sub(value.len_utf8() as u64).ok_or("generation2d-mounted.string-width")?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::Symbol, value } => self.begin_symbol(value, catalog)?,
            Token::Tag { value: 0x11, .. } if self.json_destination.is_none() => {
                self.begin_dsl()?;
            }
            Token::F64(value) if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::float(f64::from_bits(value)))?,
            Token::F64(value) if self.json_destination.is_some() => self.assign_json(dsl::DslValue::float(f64::from_bits(value)))?,
            Token::F64(value) => self.assign_f64(f64::from_bits(value))?,
            Token::Signed(value) if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::int(value))?,
            Token::Signed(value) if self.json_destination.is_some() => self.assign_json(dsl::DslValue::int(value))?,
            Token::Signed(value) => match self.stack.last_mut() {
                Some(Generation2dMountedContainerOwner::Record { owner: Generation2dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if *field == Some(2) && target.is_none() => {
                    *target = Some(flow::neural::Value::Atom(flow::neural::Atom::Integer(value)));
                    *field = None;
                }
                _ => return Err("generation2d-mounted.integer-owner"),
            },
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.json_destination.is_some() => self.assign_json(dsl::DslValue::uint(value))?,
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::uint(value))?,
            Token::Tag { value: 0x01, .. } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Bool(false))?,
            Token::Tag { value: 0x02, .. } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Bool(true))?,
            Token::Tag { value: 0x12, .. } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Null)?,
            Token::Tag { value: 0x01, .. } if self.json_destination.is_some() => self.assign_json(dsl::DslValue::Bool(false))?,
            Token::Tag { value: 0x02, .. } if self.json_destination.is_some() => self.assign_json(dsl::DslValue::Bool(true))?,
            Token::Tag { value: 0x12, .. } if self.json_destination.is_some() => self.assign_json(dsl::DslValue::Null)?,
            Token::Tag { value: 0x11, .. } if self.json_destination.is_some() => {}
            Token::Tag { value: 0x01, .. } => self.assign_bool(false),
            Token::Tag { value: 0x02, .. } => self.assign_bool(true),
            Token::Tag { value: 0x00, .. } => {
                if let Some(Generation2dMountedContainerOwner::Record { root_field: None, field, .. }) = self.stack.last_mut() {
                    if matches!(*field, Some(5 | 6)) {
                        *field = None;
                    }
                }
            }
            Token::WirePresence(_) => {}
            Token::WireNodePresence(presence) => match self.stack.last_mut() {
                Some(Generation2dMountedContainerOwner::Wire { roles, roles_len, nodes, .. }) => {
                    let base = if *nodes == 0 { 0 } else { 3 };
                    roles[*roles_len] = base;
                    *roles_len += 1;
                    if presence & 1 != 0 {
                        roles[*roles_len] = base + 1;
                        *roles_len += 1;
                    }
                    if presence & 2 != 0 {
                        roles[*roles_len] = base + 2;
                        *roles_len += 1;
                    }
                    *nodes += 1;
                }
                _ => return Err("generation2d-mounted.wire-node-owner"),
            },
            Token::TablePresence { rows, value } => match self.stack.last_mut() {
                Some(Generation2dMountedContainerOwner::Synapses { present, .. } | Generation2dMountedContainerOwner::Generations { present, .. } | Generation2dMountedContainerOwner::Dictionary { present, .. }) if rows as usize == present.len() => {
                    if value == 0 {
                        present.fill(true);
                    }
                }
                _ => {}
            },
            Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
                Some(Generation2dMountedContainerOwner::Synapses { present, .. } | Generation2dMountedContainerOwner::Generations { present, .. } | Generation2dMountedContainerOwner::Dictionary { present, .. }) => {
                    for bit in 0..8 {
                        let row = first_row as usize + bit;
                        if row < present.len() {
                            present[row] = value & (1 << bit) != 0;
                        }
                    }
                }
                _ => {}
            },
            Token::End(kind) => {
                if !self.end_dsl(kind)? && !self.end_json(kind)? {
                    self.end(kind)?;
                }
            }
            Token::Complete { .. } => {
                if !self.stack.is_empty() || self.string.is_some() || !self.json_stack.is_empty() || self.json_destination.is_some() || !self.dsl_stack.is_empty() || self.dsl_destination.is_some() {
                    return Err("generation2d-mounted.typed-terminal-populated");
                }
                self.complete = true;
            }
            Token::Tag { .. } | Token::Unsigned { .. } | Token::Signed(_) | Token::Byte(_) | Token::WireLabelPresence(_) => {}
        }
        Ok(())
    }

    fn take(&mut self) -> Option<Generation2dSnapshot> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.candidate.take()
    }

    fn close_step(&mut self) -> bool {
        self.string = None;
        self.json_destination = None;
        self.dsl_destination = None;
        if self.dsl_stack.pop().is_some() {
            return false;
        }
        if self.json_stack.pop().is_some() {
            return false;
        }
        if self.stack.pop().is_some() {
            return false;
        }
        drop(self.candidate.take());
        self.handed_back = true;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.candidate.is_none() && self.stack.is_empty() && self.string.is_none() && self.json_stack.is_empty() && self.json_destination.is_none() && self.dsl_stack.is_empty() && self.dsl_destination.is_none()
    }
}

impl Drop for Generation2dMountedTypedSnapshotOwner {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Generation2d mounted typed snapshot owner reached Drop before handoff or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation2dMountedPackPhase {
    Prefix,
    Ingress,
    Drive,
    Ready,
    Published,
    Closing,
    Closed,
}

/// 🧵️ Worker-owned mounted session. P2D2 is rejected before semantic allocation; every byte
/// after it is handed unchanged to the canonical retained page source.
pub struct Generation2dMountedPackSession {
    phase: Generation2dMountedPackPhase,
    expected_bytes: usize,
    maximum_items: usize,
    prefix: [u8; 4],
    prefix_len: usize,
    page: [u8; mounted::RETAINED_PACK_PAGE_BYTES],
    page_len: usize,
    admitted: usize,
    canonical_ledger: u64,
    source: std::mem::ManuallyDrop<Option<mounted::RetainedPackSourceCursor>>,
    anchor: std::mem::ManuallyDrop<Option<mounted::RetainedPackAnchorCursor>>,
    segment: std::mem::ManuallyDrop<Option<mounted::RetainedPackSegmentCursor>>,
    catalog: std::mem::ManuallyDrop<Option<mounted::RetainedPackCatalogCursor>>,
    value: std::mem::ManuallyDrop<Option<mounted::RetainedValueCursor>>,
    typed: std::mem::ManuallyDrop<Option<Generation2dMountedTypedSnapshotOwner>>,
    catalog_value: std::mem::ManuallyDrop<Option<mounted::RetainedPackCatalog>>,
    source_complete: bool,
    segment_complete: bool,
    anchor_ready: bool,
    catalog_complete: bool,
    value_sealed: bool,
    value_complete: bool,
}

impl Generation2dMountedPackSession {
    pub fn new(expected_bytes: usize, maximum_items: usize) -> Result<Self, &'static str> {
        if expected_bytes <= GENERATION2D_MOUNTED_PREFIX.len() || expected_bytes > store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES || maximum_items == 0 {
            return Err("generation2d-mounted.exact-credits");
        }
        Ok(Self {
            phase: Generation2dMountedPackPhase::Prefix,
            expected_bytes,
            maximum_items,
            prefix: [0; 4],
            prefix_len: 0,
            page: [0; mounted::RETAINED_PACK_PAGE_BYTES],
            page_len: 0,
            admitted: 0,
            canonical_ledger: 0xcbf2_9ce4_8422_2325,
            source: std::mem::ManuallyDrop::new(None),
            anchor: std::mem::ManuallyDrop::new(None),
            segment: std::mem::ManuallyDrop::new(None),
            catalog: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            typed: std::mem::ManuallyDrop::new(None),
            catalog_value: std::mem::ManuallyDrop::new(None),
            source_complete: false,
            segment_complete: false,
            anchor_ready: false,
            catalog_complete: false,
            value_sealed: false,
            value_complete: false,
        })
    }

    fn allocate_after_discriminator(&mut self) -> Result<(), &'static str> {
        if self.prefix != GENERATION2D_MOUNTED_PREFIX {
            return Err("generation2d-mounted.schema-discriminator");
        }
        let canonical = self.expected_bytes - GENERATION2D_MOUNTED_PREFIX.len();
        let pages = canonical.div_ceil(mounted::RETAINED_PACK_PAGE_BYTES);
        let maximum_symbols = self.maximum_items.min(u32::MAX as usize) as u32;
        let limits = || mounted::PackLimits {
            max_file_len: canonical as u64,
            max_segment_len: canonical as u64,
            max_symbols: maximum_symbols,
            max_depth: GENERATION2D_MOUNTED_TYPED_DEPTH as u16,
            max_items: self.maximum_items as u64,
            max_total_alloc: canonical as u64,
        };
        *self.source = Some(mounted::RetainedPackSourceCursor::try_new(pages, canonical)?);
        *self.anchor = Some(mounted::RetainedPackAnchorCursor::new());
        *self.segment = Some(mounted::RetainedPackSegmentCursor::try_new(limits()).map_err(|_| "generation2d-mounted.segment-preflight")?);
        *self.catalog = Some(mounted::RetainedPackCatalogCursor::try_new(limits(), maximum_symbols as usize, self.maximum_items, canonical).map_err(|_| "generation2d-mounted.catalog-preflight")?);
        *self.value = Some(mounted::RetainedValueCursor::try_new(limits()).map_err(|_| "generation2d-mounted.value-preflight")?);
        *self.typed = Some(Generation2dMountedTypedSnapshotOwner::new()?);
        self.phase = Generation2dMountedPackPhase::Ingress;
        Ok(())
    }

    pub fn admit_byte(&mut self, value: u8) -> Result<(), u8> {
        if !matches!(self.phase, Generation2dMountedPackPhase::Prefix | Generation2dMountedPackPhase::Ingress) || self.admitted == self.expected_bytes {
            return Err(value);
        }
        if self.prefix_len < GENERATION2D_MOUNTED_PREFIX.len() {
            if value != GENERATION2D_MOUNTED_PREFIX[self.prefix_len] {
                return Err(value);
            }
            self.prefix[self.prefix_len] = value;
            self.prefix_len += 1;
            self.admitted += 1;
            if self.prefix_len == GENERATION2D_MOUNTED_PREFIX.len() && self.allocate_after_discriminator().is_err() {
                self.admitted -= 1;
                return Err(value);
            }
            return Ok(());
        }
        self.page[self.page_len] = value;
        self.canonical_ledger ^= u64::from(value);
        self.canonical_ledger = self.canonical_ledger.wrapping_mul(0x0000_0100_0000_01b3);
        self.page_len += 1;
        self.admitted += 1;
        if self.page_len == mounted::RETAINED_PACK_PAGE_BYTES && self.flush_page().is_err() {
            self.admitted -= 1;
            return Err(value);
        }
        Ok(())
    }

    fn flush_page(&mut self) -> Result<(), &'static str> {
        if self.page_len == 0 {
            return Ok(());
        }
        let len = self.page_len;
        let page = mounted::RetainedPackPage::try_from_array(std::mem::replace(&mut self.page, [0; mounted::RETAINED_PACK_PAGE_BYTES]), len).map_err(|_| "generation2d-mounted.page-owner")?;
        self.page_len = 0;
        let source = self.source.as_mut().ok_or("generation2d-mounted.source-owner")?;
        source.preflight_page(len)?;
        source.admit_page(page).map_err(|_| "generation2d-mounted.producer-handback")
    }

    pub fn seal(&mut self) -> Result<(), &'static str> {
        if self.admitted != self.expected_bytes || self.prefix_len != GENERATION2D_MOUNTED_PREFIX.len() {
            return Err("generation2d-mounted.exact-byte-seal");
        }
        self.flush_page()?;
        self.source.as_mut().ok_or("generation2d-mounted.source-owner")?.seal()?;
        self.phase = Generation2dMountedPackPhase::Drive;
        Ok(())
    }

    pub fn grant(&mut self) -> Result<bool, &'static str> {
        if matches!(self.phase, Generation2dMountedPackPhase::Ready | Generation2dMountedPackPhase::Published) {
            return Ok(true);
        }
        if self.phase != Generation2dMountedPackPhase::Drive {
            return Err("generation2d-mounted.missing-seal");
        }
        if self.typed.as_mut().ok_or("generation2d-mounted.typed-owner")?.grant_symbol(self.catalog.as_ref().ok_or("generation2d-mounted.catalog-owner")?)? {
            return Ok(false);
        }
        if !self.value_complete {
            if let Some(token) = self.value.as_mut().ok_or("generation2d-mounted.value-owner")?.grant().map_err(|_| "generation2d-mounted.value-malformed")? {
                self.value_complete = matches!(token, mounted::RetainedValueToken::Complete { .. });
                self.typed.as_mut().expect("P2 typed owner retained").accept(token, self.catalog.as_ref().expect("P2 catalog retained"))?;
                return Ok(false);
            }
        }
        if self.catalog_complete && !self.value_sealed {
            let bytes = self.catalog.as_ref().expect("P2 catalog retained").document_bytes();
            self.value.as_mut().expect("P2 value owner retained").seal(bytes).map_err(|_| "generation2d-mounted.value-seal")?;
            self.value_sealed = true;
            return Ok(false);
        }
        if !self.segment_complete && (self.segment.as_ref().ok_or("generation2d-mounted.segment-owner")?.preflight().is_err() || self.source_complete) {
            if let Some(event) = self.segment.as_mut().expect("P2 segment retained").grant().map_err(|_| "generation2d-mounted.segment-malformed")? {
                self.segment_complete = matches!(event, mounted::RetainedPackSegmentEvent::PackComplete { .. });
                let catalog = self.catalog.as_mut().expect("P2 catalog retained");
                catalog.admit(event).map_err(|_| "generation2d-mounted.catalog-backpressure")?;
                if let Some(event) = catalog.grant().map_err(|_| "generation2d-mounted.catalog-malformed")? {
                    match event {
                        mounted::RetainedPackCatalogEvent::DocumentByte { index, value, .. } => self.value.as_mut().expect("P2 value retained").admit_byte(index, value).map_err(|_| "generation2d-mounted.value-backpressure")?,
                        mounted::RetainedPackCatalogEvent::Complete => self.catalog_complete = true,
                        _ => {}
                    }
                }
                return Ok(false);
            }
        }
        if !self.source_complete && self.segment.as_ref().expect("P2 segment retained").preflight().is_ok() {
            if let Some(event) = self.source.as_mut().ok_or("generation2d-mounted.source-owner")?.grant()? {
                self.source_complete = matches!(event, mounted::RetainedPackSourceEvent::Complete { .. });
                self.anchor.as_mut().expect("P2 anchor retained").grant(Some(event)).map_err(|_| "generation2d-mounted.anchor-malformed")?;
                self.segment.as_mut().expect("P2 segment retained").admit(event).map_err(|_| "generation2d-mounted.segment-handback")?;
                return Ok(false);
            }
        }
        if self.source_complete && !self.anchor_ready {
            self.anchor_ready = self.anchor.as_mut().expect("P2 anchor retained").grant(None).map_err(|_| "generation2d-mounted.anchor-malformed")?;
            return Ok(false);
        }
        if self.anchor_ready && self.catalog_complete && self.value_complete && self.catalog_value.is_none() {
            let superblock = self.anchor.as_mut().expect("P2 anchor retained").take().ok_or("generation2d-mounted.anchor-handoff")?;
            *self.catalog_value = self.catalog.as_mut().expect("P2 catalog retained").take(superblock).map_err(|_| "generation2d-mounted.catalog-validation")?;
            self.phase = Generation2dMountedPackPhase::Ready;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn take(&mut self) -> Option<Generation2dSnapshot> {
        if self.phase != Generation2dMountedPackPhase::Ready {
            return None;
        }
        let value = self.typed.as_mut()?.take()?;
        self.phase = Generation2dMountedPackPhase::Published;
        Some(value)
    }

    pub fn progress(&self) -> Option<mounted::RetainedPackSourceProgress> {
        self.source.as_ref().map(mounted::RetainedPackSourceCursor::progress)
    }

    #[cfg(test)]
    fn canonical_ingress_ledger(&self) -> u64 {
        self.canonical_ledger
    }

    #[cfg(test)]
    fn semantic_allocated(&self) -> bool {
        self.typed.is_some()
    }

    pub fn request_cancel(&mut self) {
        if let Some(source) = self.source.as_mut() {
            source.request_cancel();
        }
        self.phase = Generation2dMountedPackPhase::Closing;
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<bool, &'static str> {
        self.phase = Generation2dMountedPackPhase::Closing;
        if maximum_items == 0 || maximum_bytes < mounted::RETAINED_PACK_PAGE_BYTES {
            return Ok(false);
        }
        if let Some(catalog) = self.catalog_value.as_mut() {
            if catalog.symbols.pop().is_some() || catalog.chunks.pop().is_some() {
                return Ok(false);
            }
            drop(self.catalog_value.take());
            return Ok(false);
        }
        if let Some(typed) = self.typed.as_mut() {
            if !typed.close_step() {
                return Ok(false);
            }
            drop(self.typed.take());
            return Ok(false);
        }
        if let Some(value) = self.value.as_mut() {
            if value.close_step(1) != mounted::RetainedPackCloseStep::Complete {
                return Ok(false);
            }
            drop(self.value.take());
            return Ok(false);
        }
        if let Some(catalog) = self.catalog.as_mut() {
            if catalog.close_step(1) != mounted::RetainedPackCloseStep::Complete {
                return Ok(false);
            }
            drop(self.catalog.take());
            return Ok(false);
        }
        if let Some(segment) = self.segment.as_mut() {
            segment.close_step();
            drop(self.segment.take());
            return Ok(false);
        }
        if let Some(anchor) = self.anchor.as_mut() {
            anchor.close_step();
            drop(self.anchor.take());
            return Ok(false);
        }
        if let Some(source) = self.source.as_mut() {
            if source.close_step(1, mounted::RETAINED_PACK_PAGE_BYTES)? != mounted::RetainedPackCloseStep::Complete {
                return Ok(false);
            }
            drop(self.source.take());
            return Ok(false);
        }
        self.phase = Generation2dMountedPackPhase::Closed;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == Generation2dMountedPackPhase::Closed && self.source.is_none() && self.anchor.is_none() && self.segment.is_none() && self.catalog.is_none() && self.value.is_none() && self.typed.is_none() && self.catalog_value.is_none()
    }
}

impl Drop for Generation2dMountedPackSession {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Generation2d mounted canonical pack session reached Drop before exact terminal-empty close");
    }
}
//#endregion 🔖️MountedCanonicalPackSession

#[cfg(test)]
mod retained_mounted_laws {
    use super::*;

    fn synapse_digest(synapse: &flow::SynapseSpec) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        for field in [&synapse.id, &synapse.from, &synapse.from_port, &synapse.to, &synapse.to_port] {
            digest ^= field.len() as u64;
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
            for byte in field.as_bytes() {
                digest ^= u64::from(*byte);
                digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        digest
    }

    fn close(session: &mut Generation2dMountedPackSession) {
        session.request_cancel();
        for _ in 0..100_000 {
            if session.close_step(1, mounted::RETAINED_PACK_PAGE_BYTES).expect("P2 retained session close") {
                assert!(session.terminal_is_empty());
                return;
            }
        }
        panic!("P2 retained session did not reach terminal-empty close");
    }

    #[test]
    fn non_empty_canonical_snapshot_round_trips_one_grant_at_a_time() {
        let mut expected = Generation2dSnapshot::default();
        let nested = flow::neural::Dictionary::new().insert("enabled", flow::neural::Value::Atom(flow::neural::Atom::Boolean(true)));
        let params = flow::neural::Dictionary::new().insert("gain", flow::neural::Value::Atom(flow::neural::Atom::Decimal(2.5))).insert("nested", flow::neural::Value::Dictionary(nested));
        expected.fixture.widgets.push(flow::Widget::Neuron { id: "retained-neuron".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true });
        let mut expanded = flow::OrderedSet::new();
        expanded.insert("answer".into());
        let preview = flow::neural::Dictionary::new().insert("answer", flow::neural::Value::Atom(flow::neural::Atom::String("visible".into())));
        expected.fixture.widgets.push(flow::Widget::OutputPreview { id: "retained-preview".into(), preview, expanded });
        expected.fixture.widgets.push(flow::Widget::Cluster { id: "retained-cluster".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() });
        expected.fixture.synapses.push(flow::SynapseSpec { id: "retained-synapse".into(), from: "retained-neuron".into(), to: "retained-preview".into(), from_port: "out".into(), to_port: String::new() });
        let mut values: flow::playbook::PlaybookValues = std::collections::HashMap::new();
        values.insert(
            "nested".into(),
            dsl::DslValue::object([("array".to_string(), dsl::DslValue::Array(vec![dsl::DslValue::Bool(true), dsl::DslValue::Null, dsl::DslValue::float(3.5)])), ("text".to_string(), dsl::DslValue::String("retained".to_string()))]),
        );
        expected.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(flow::playbook::FormGeneration { id: "retained-generation".into(), name: "Generation".into(), values });
        expected.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some("retained-generation".into());
        expected.generation.cold_builder_mut().expect("unique cold generation owner").preview_text = Some("preview".into());
        assert!(!expected.fixture.widgets.is_empty());
        assert!(!expected.fixture.synapses.is_empty());
        let bytes = encode(&expected);
        assert_eq!(&bytes[..4], &GENERATION2D_MOUNTED_PREFIX);
        let expected_ledger = bytes[4..].iter().fold(0xcbf2_9ce4_8422_2325u64, |ledger, byte| (ledger ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3));
        let mut session = Generation2dMountedPackSession::new(bytes.len(), 8_192).expect("P2 retained snapshot preflight");
        for byte in bytes {
            session.admit_byte(byte).expect("one admitted snapshot byte");
        }
        assert_eq!(session.canonical_ingress_ledger(), expected_ledger, "bytes after P2D2 must be the unchanged canonical SPK stream");
        session.seal().expect("exact snapshot seal");
        let mut ready = false;
        for _ in 0..1_000_000 {
            if session.grant().expect("one retained snapshot grant") {
                ready = true;
                break;
            }
        }
        assert!(ready, "P2 retained canonical route must converge");
        let actual = session.take().expect("typed snapshot handoff");
        assert_eq!(actual.fixture.synapses.len(), expected.fixture.synapses.len(), "typed synapse owner must retain the exact row census");
        assert_eq!(actual.fixture.synapses.last(), expected.fixture.synapses.last(), "typed synapse owner must retain the exact non-empty appended row");
        assert_eq!(synapse_digest(actual.fixture.synapses.last().expect("typed retained synapse")), synapse_digest(expected.fixture.synapses.last().expect("expected retained synapse")));
        assert_eq!(actual, expected, "all typed snapshot owners must round-trip exactly");
        close(&mut session);
    }

    #[test]
    fn p3d3_is_rejected_before_semantic_allocation() {
        let mut session = Generation2dMountedPackSession::new(8, 8).expect("P2 hostile discriminator preflight");
        session.admit_byte(b'P').expect("shared first discriminator byte");
        assert!(!session.semantic_allocated());
        assert_eq!(session.admit_byte(b'3'), Err(b'3'));
        assert!(!session.semantic_allocated());
        close(&mut session);
    }
}
