//! 📦️ Procedural3d artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::procedural3d::Procedural3dSnapshot;
#[cfg(test)]
use store::PackError;

/// 📦️ Encodes a `Procedural3dSnapshot` to its binary pack form.
pub fn encode(document: &Procedural3dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 🔬️ Batch decode exists only for constitutional equivalence tests; mounted UI code
/// has no production symbol that can reach the whole-document decoder.
#[cfg(test)]
pub fn decode(bytes: &[u8]) -> Result<Procedural3dSnapshot, PackError> {
    <Procedural3dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural3d::dsl as procedural3d_dsl;
    use flow::Widget;
    use semio_framework_os_kernel::os_store::test_support;

    #[test]
    fn dsl_pack_equivalence_empty_projection() {
        test_support::assert_dsl_pack_equivalence(&Procedural3dSnapshot::default());
    }

    #[test]
    fn dsl_pack_equivalence_example_fixture() {
        let projection = procedural3d_dsl::parse_dsl(procedural3d_dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT).expect("parse 🌀️default.procedural3d fixture");
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_with_generation_state() {
        let mut projection = Procedural3dSnapshot::default();
        let mut values = serde_json::Map::new();
        // 🌱️ Fractional (not whole-number) so `dsl::from_dsl_value`'s int-normalization of whole
        // `DslValue::Number`s (an engine-owned behavior, see the sibling dsl test) doesn't make this
        // round trip spuriously unequal.
        values.insert("count".into(), serde_json::json!(3.5));
        projection.generation.generations.push(flow::playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.selected_generation_id = Some("generation-1".into());
        projection.generation.preview_text = Some("42".into());
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_covers_every_widget_kind() {
        let mut projection = Procedural3dSnapshot::default();
        projection.fixture.widgets = vec![
            Widget::InputSlider { id: "slider".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
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
        let projection = procedural3d_dsl::parse_dsl(procedural3d_dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT).expect("parse fixture");
        let bytes = encode(&projection);
        assert!(bytes.starts_with(b"P3D3"));
        assert_eq!(decode(&bytes).expect("decode"), projection);
        let mut wrong = bytes;
        wrong[..4].copy_from_slice(b"P2D2");
        assert!(decode(&wrong).is_err());
    }
}
//#endregion 🧪️Tests

use store::mounted_pack_rt as mounted;

//#region 🔖️MountedCanonicalPackSession
const PROCEDURAL3D_MOUNTED_PREFIX: [u8; 4] = *b"P3D3";
const PROCEDURAL3D_MOUNTED_TYPED_DEPTH: usize = 12;
const PROCEDURAL3D_REQUIRED_SNAPSHOT_FIELDS: u16 = 0b1001_1111;

#[derive(Default)]
struct Procedural3dMountedWidgetOwner {
    keyword: String,
    strings: [String; 4],
    numbers: [f64; 4],
    boolean: bool,
    lists: [Vec<String>; 2],
    dictionaries: [flow::neural::Dictionary; 2],
    dynamic: [Option<dsl::DslValue>; 2],
}

#[derive(Default)]
struct Procedural3dMountedSynapseOwner {
    id: String,
    from: String,
    to: String,
    from_port: String,
    to_port: String,
}

#[derive(Default)]
struct Procedural3dMountedGenerationOwner {
    id: String,
    name: String,
    values: serde_json::Map<String, serde_json::Value>,
}

#[derive(Default)]
struct Procedural3dMountedDictionaryEntryOwner {
    key: String,
    value: Option<flow::neural::Value>,
}

#[derive(Clone, Copy)]
enum Procedural3dMountedDictionaryDestination {
    Widget { parent: usize, field: u16 },
    Value { parent: usize },
}

enum Procedural3dMountedRecordOwner {
    Root,
    Camera(flow::CameraJson),
    Layout { key: String, value: flow::WidgetLayout },
    Widget(Procedural3dMountedWidgetOwner),
    NeuralValue { table: usize, row: usize, value: Option<flow::neural::Value> },
    Structural,
}

enum Procedural3dMountedContainerOwner {
    Record { root_field: Option<u16>, field: Option<u16>, seen: u16, owner: Procedural3dMountedRecordOwner },
    Statements { root_field: u16, keyword: Option<String> },
    Strings { parent: usize, field: u16, values: Vec<String> },
    Synapses { rows: Vec<Procedural3dMountedSynapseOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    LayoutMap { key: Option<String> },
    Generations { rows: Vec<Procedural3dMountedGenerationOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    Dictionary { destination: Procedural3dMountedDictionaryDestination, rows: Vec<Procedural3dMountedDictionaryEntryOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    Wire { table: usize, row: usize, roles: [u8; 6], roles_len: usize, role: usize, nodes: usize },
    Structural { kind: mounted::RetainedValueContainer, root_field: Option<u16> },
}

#[derive(Clone, Copy)]
enum Procedural3dMountedStringTarget {
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

struct Procedural3dMountedStringOwner {
    target: Procedural3dMountedStringTarget,
    value: String,
    remaining: Option<u64>,
    symbol: Option<(u64, usize, usize)>,
}

enum Procedural3dMountedJsonFrame {
    Array(Vec<serde_json::Value>),
    Object { values: serde_json::Map<String, serde_json::Value>, key: Option<String> },
}

enum Procedural3dMountedDslFrame {
    Array(Vec<dsl::DslValue>),
    Object { values: Vec<(String, dsl::DslValue)>, key: Option<String> },
}

/// 🧬️ Fixed-depth schema owner consuming catalog/value events directly into P3 domain
/// fields, with one scalar byte opportunity per retained grant. It has no generic record tree
/// and cannot invoke a batch pack decoder.
struct Procedural3dMountedTypedSnapshotOwner {
    candidate: std::mem::ManuallyDrop<Option<Procedural3dSnapshot>>,
    stack: Vec<Procedural3dMountedContainerOwner>,
    string: Option<Procedural3dMountedStringOwner>,
    pending_table_rows: Option<u64>,
    json_stack: Vec<Procedural3dMountedJsonFrame>,
    json_destination: Option<(usize, usize)>,
    dsl_stack: Vec<Procedural3dMountedDslFrame>,
    dsl_destination: Option<(usize, usize)>,
    complete: bool,
    handed_back: bool,
}

impl Procedural3dMountedTypedSnapshotOwner {
    fn new() -> Result<Self, &'static str> {
        let mut stack = Vec::new();
        stack.try_reserve_exact(PROCEDURAL3D_MOUNTED_TYPED_DEPTH).map_err(|_| "procedural3d-mounted.typed-stack-preflight")?;
        let mut json_stack = Vec::new();
        json_stack.try_reserve_exact(PROCEDURAL3D_MOUNTED_TYPED_DEPTH).map_err(|_| "procedural3d-mounted.json-stack-preflight")?;
        let mut dsl_stack = Vec::new();
        dsl_stack.try_reserve_exact(PROCEDURAL3D_MOUNTED_TYPED_DEPTH).map_err(|_| "procedural3d-mounted.dsl-stack-preflight")?;
        let candidate = Procedural3dSnapshot {
            fixture: flow::FlowFixture { schema: String::new(), camera: flow::CameraJson::default(), widgets: Vec::new(), synapses: Vec::new(), layout: std::collections::BTreeMap::new() },
            generation: flow::playbook::GenerationPlayState::default(),
        };
        Ok(Self { candidate: std::mem::ManuallyDrop::new(Some(candidate)), stack, string: None, pending_table_rows: None, json_stack, json_destination: None, dsl_stack, dsl_destination: None, complete: false, handed_back: false })
    }

    fn push(&mut self, owner: Procedural3dMountedContainerOwner) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("procedural3d-mounted.typed-depth");
        }
        self.stack.push(owner);
        Ok(())
    }

    fn current_root_field(&self) -> Option<u16> {
        self.stack.iter().rev().find_map(|owner| match owner {
            Procedural3dMountedContainerOwner::Record { root_field, field, .. } => root_field.or(*field),
            Procedural3dMountedContainerOwner::Statements { root_field, .. } => Some(*root_field),
            Procedural3dMountedContainerOwner::Strings { parent, .. } => self.stack.get(*parent).and_then(|owner| match owner {
                Procedural3dMountedContainerOwner::Record { root_field, .. } => *root_field,
                _ => None,
            }),
            Procedural3dMountedContainerOwner::Synapses { .. } | Procedural3dMountedContainerOwner::Wire { .. } => Some(3),
            Procedural3dMountedContainerOwner::LayoutMap { .. } => Some(4),
            Procedural3dMountedContainerOwner::Generations { .. } => Some(7),
            Procedural3dMountedContainerOwner::Dictionary { .. } => Some(2),
            Procedural3dMountedContainerOwner::Structural { root_field, .. } => *root_field,
        })
    }

    fn string_target(&mut self) -> Result<Procedural3dMountedStringTarget, &'static str> {
        let index = self.stack.len().checked_sub(1).ok_or("procedural3d-mounted.string-without-owner")?;
        if self.json_destination.is_some() {
            return Ok(match self.json_stack.last() {
                Some(Procedural3dMountedJsonFrame::Object { key: None, .. }) => Procedural3dMountedStringTarget::JsonKey,
                _ => Procedural3dMountedStringTarget::JsonValue,
            });
        }
        if self.dsl_destination.is_some() {
            return Ok(match self.dsl_stack.last() {
                Some(Procedural3dMountedDslFrame::Object { key: None, .. }) => Procedural3dMountedStringTarget::DslKey,
                _ => Procedural3dMountedStringTarget::DslValue,
            });
        }
        match &mut self.stack[index] {
            Procedural3dMountedContainerOwner::Record { root_field: None, field: Some(field), .. } => Ok(Procedural3dMountedStringTarget::Root(*field)),
            Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::NeuralValue { .. }, field: Some(4), .. } => Ok(Procedural3dMountedStringTarget::NeuralText(index)),
            Procedural3dMountedContainerOwner::Record { field: Some(field), .. } => Ok(Procedural3dMountedStringTarget::Record(index, *field)),
            Procedural3dMountedContainerOwner::Statements { keyword: None, .. } => Ok(Procedural3dMountedStringTarget::StatementKeyword(index)),
            Procedural3dMountedContainerOwner::Strings { .. } => Ok(Procedural3dMountedStringTarget::Sequence(index)),
            Procedural3dMountedContainerOwner::Synapses { field: Some(field), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("procedural3d-mounted.synapse-row")?;
                *next = row + 1;
                Ok(Procedural3dMountedStringTarget::Synapse(index, row, *field as u8))
            }
            Procedural3dMountedContainerOwner::LayoutMap { key: None } => Ok(Procedural3dMountedStringTarget::LayoutKey(index)),
            Procedural3dMountedContainerOwner::Generations { field: Some(field), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("procedural3d-mounted.generation-row")?;
                *next = row + 1;
                Ok(Procedural3dMountedStringTarget::Generation(index, row, *field as u8))
            }
            Procedural3dMountedContainerOwner::Dictionary { field: Some(0), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("procedural3d-mounted.dictionary-key-row")?;
                *next = row + 1;
                Ok(Procedural3dMountedStringTarget::DictionaryKey(index, row))
            }
            Procedural3dMountedContainerOwner::Wire { roles, roles_len, role, .. } if *role < *roles_len => {
                let target = roles[*role];
                *role += 1;
                Ok(Procedural3dMountedStringTarget::Wire(index, target))
            }
            _ => Err("procedural3d-mounted.string-owner-role"),
        }
    }

    fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("procedural3d-mounted.string-overlap");
        }
        self.string = Some(Procedural3dMountedStringOwner { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    fn begin_symbol(&mut self, symbol: u64, catalog: &mounted::RetainedPackCatalogCursor) -> Result<(), &'static str> {
        if self.string.is_none() {
            self.begin_string()?;
        }
        let chars = catalog.symbol_chars(symbol).map_err(|_| "procedural3d-mounted.symref")?;
        let owner = self.string.as_mut().expect("P3 mounted string retained");
        owner.value.try_reserve_exact(chars).map_err(|_| "procedural3d-mounted.symbol-preflight")?;
        owner.symbol = Some((symbol, 0, chars));
        if chars == 0 {
            self.finish_string()?;
        }
        Ok(())
    }

    fn grant_symbol(&mut self, catalog: &mounted::RetainedPackCatalogCursor) -> Result<bool, &'static str> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, chars)) = owner.symbol else { return Ok(false) };
        owner.value.push(catalog.symbol_char(symbol, index).map_err(|_| "procedural3d-mounted.symref-char")?.ok_or("procedural3d-mounted.symref-short")?);
        if index + 1 == chars {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("P3 mounted symbol retained").symbol = Some((symbol, index + 1, chars));
        }
        Ok(true)
    }

    fn finish_string(&mut self) -> Result<(), &'static str> {
        let owner = self.string.take().ok_or("procedural3d-mounted.string-handoff")?;
        match owner.target {
            Procedural3dMountedStringTarget::Root(0) => {
                self.candidate.as_mut().ok_or("procedural3d-mounted.snapshot-owner")?.fixture.schema = owner.value;
                if let Some(Procedural3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Procedural3dMountedStringTarget::Root(5) => {
                self.candidate.as_mut().ok_or("procedural3d-mounted.snapshot-owner")?.generation.selected_generation_id = Some(owner.value);
                if let Some(Procedural3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Procedural3dMountedStringTarget::Root(6) => {
                self.candidate.as_mut().ok_or("procedural3d-mounted.snapshot-owner")?.generation.preview_text = Some(owner.value);
                if let Some(Procedural3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Procedural3dMountedStringTarget::Record(index, field) => match self.stack.get_mut(index) {
                Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Widget(widget), field: active, .. }) => {
                    *widget.strings.get_mut(field as usize).ok_or("procedural3d-mounted.widget-string-field")? = owner.value;
                    *active = None;
                }
                _ => return Err("procedural3d-mounted.record-string-owner"),
            },
            Procedural3dMountedStringTarget::StatementKeyword(index) => match self.stack.get_mut(index) {
                Some(Procedural3dMountedContainerOwner::Statements { keyword, .. }) => *keyword = Some(owner.value),
                _ => return Err("procedural3d-mounted.statement-keyword-owner"),
            },
            Procedural3dMountedStringTarget::Sequence(index) => match self.stack.get_mut(index) {
                Some(Procedural3dMountedContainerOwner::Strings { values, .. }) => values.push(owner.value),
                _ => return Err("procedural3d-mounted.sequence-string-owner"),
            },
            Procedural3dMountedStringTarget::Synapse(index, row, 0) => match self.stack.get_mut(index) {
                Some(Procedural3dMountedContainerOwner::Synapses { rows, .. }) => rows.get_mut(row).ok_or("procedural3d-mounted.synapse-row")?.id = owner.value,
                _ => return Err("procedural3d-mounted.synapse-string-owner"),
            },
            Procedural3dMountedStringTarget::LayoutKey(index) => match self.stack.get_mut(index) {
                Some(Procedural3dMountedContainerOwner::LayoutMap { key }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("procedural3d-mounted.layout-key-owner"),
            },
            Procedural3dMountedStringTarget::Generation(index, row, 0) => match self.stack.get_mut(index) {
                Some(Procedural3dMountedContainerOwner::Generations { rows, .. }) => rows.get_mut(row).ok_or("procedural3d-mounted.generation-row")?.id = owner.value,
                _ => return Err("procedural3d-mounted.generation-string-owner"),
            },
            Procedural3dMountedStringTarget::Generation(index, row, 1) => match self.stack.get_mut(index) {
                Some(Procedural3dMountedContainerOwner::Generations { rows, .. }) => rows.get_mut(row).ok_or("procedural3d-mounted.generation-row")?.name = owner.value,
                _ => return Err("procedural3d-mounted.generation-string-owner"),
            },
            Procedural3dMountedStringTarget::DictionaryKey(index, row) => match self.stack.get_mut(index) {
                Some(Procedural3dMountedContainerOwner::Dictionary { rows, .. }) => rows.get_mut(row).ok_or("procedural3d-mounted.dictionary-key-row")?.key = owner.value,
                _ => return Err("procedural3d-mounted.dictionary-key-owner"),
            },
            Procedural3dMountedStringTarget::NeuralText(index) => match self.stack.get_mut(index) {
                Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::NeuralValue { value, .. }, field, .. }) if *field == Some(4) && value.is_none() => {
                    *value = Some(flow::neural::Value::Atom(flow::neural::Atom::String(owner.value)));
                    *field = None;
                }
                _ => return Err("procedural3d-mounted.neural-text-owner"),
            },
            Procedural3dMountedStringTarget::JsonKey => match self.json_stack.last_mut() {
                Some(Procedural3dMountedJsonFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("procedural3d-mounted.json-key-owner"),
            },
            Procedural3dMountedStringTarget::JsonValue => self.assign_json(serde_json::Value::String(owner.value))?,
            Procedural3dMountedStringTarget::DslKey => match self.dsl_stack.last_mut() {
                Some(Procedural3dMountedDslFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("procedural3d-mounted.dsl-key-owner"),
            },
            Procedural3dMountedStringTarget::DslValue => self.assign_dsl(dsl::DslValue::String(owner.value))?,
            Procedural3dMountedStringTarget::Wire(index, role) => {
                let (table, row) = match self.stack.get(index) {
                    Some(Procedural3dMountedContainerOwner::Wire { table, row, .. }) => (*table, *row),
                    _ => return Err("procedural3d-mounted.wire-owner"),
                };
                let synapse = match self.stack.get_mut(table) {
                    Some(Procedural3dMountedContainerOwner::Synapses { rows, .. }) => rows.get_mut(row).ok_or("procedural3d-mounted.wire-row")?,
                    _ => return Err("procedural3d-mounted.wire-table"),
                };
                match role {
                    0 => synapse.from = owner.value,
                    1 => synapse.from_port = owner.value,
                    3 => synapse.to = owner.value,
                    4 => synapse.to_port = owner.value,
                    _ => drop(owner.value),
                }
            }
            _ => return Err("procedural3d-mounted.string-field"),
        }
        Ok(())
    }

    fn assign_json(&mut self, value: serde_json::Value) -> Result<(), &'static str> {
        match self.json_stack.last_mut() {
            Some(Procedural3dMountedJsonFrame::Array(values)) => values.push(value),
            Some(Procedural3dMountedJsonFrame::Object { values, key }) => {
                values.insert(key.take().ok_or("procedural3d-mounted.json-value-key")?, value);
            }
            None => {
                let (table, row) = self.json_destination.take().ok_or("procedural3d-mounted.json-destination")?;
                let values = match value {
                    serde_json::Value::Object(values) => values,
                    _ => return Err("procedural3d-mounted.generation-values-shape"),
                };
                match self.stack.get_mut(table) {
                    Some(Procedural3dMountedContainerOwner::Generations { rows, .. }) => rows.get_mut(row).ok_or("procedural3d-mounted.generation-row")?.values = values,
                    _ => return Err("procedural3d-mounted.generation-values-table"),
                }
            }
        }
        Ok(())
    }

    fn assign_dsl(&mut self, value: dsl::DslValue) -> Result<(), &'static str> {
        match self.dsl_stack.last_mut() {
            Some(Procedural3dMountedDslFrame::Array(values)) => values.push(value),
            Some(Procedural3dMountedDslFrame::Object { values, key }) => values.push((key.take().ok_or("procedural3d-mounted.dsl-value-key")?, value)),
            None => {
                let (parent, slot) = self.dsl_destination.take().ok_or("procedural3d-mounted.dsl-destination")?;
                match self.stack.get_mut(parent) {
                    Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Widget(widget), field, .. }) if *field == Some((slot + 2) as u16) => {
                        widget.dynamic[slot] = Some(value);
                        *field = None;
                    }
                    _ => return Err("procedural3d-mounted.dsl-widget-owner"),
                }
            }
        }
        Ok(())
    }

    fn end_dsl(&mut self, kind: mounted::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.dsl_destination.is_none() {
            return Ok(false);
        }
        let value = match self.dsl_stack.pop().ok_or("procedural3d-mounted.dsl-end")? {
            Procedural3dMountedDslFrame::Array(values) if kind == mounted::RetainedValueContainer::List => dsl::DslValue::Array(values),
            Procedural3dMountedDslFrame::Object { values, key: None } if kind == mounted::RetainedValueContainer::Map => dsl::DslValue::Object(values),
            _ => return Err("procedural3d-mounted.dsl-container-mismatch"),
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
            Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Widget(widget), field: Some(field @ (2 | 3)), .. }) if widget.keyword == "cluster" => *field,
            _ => return Ok(false),
        };
        self.dsl_destination = Some((parent, usize::from(field - 2)));
        Ok(true)
    }

    fn assign_f64(&mut self, value: f64) -> Result<(), &'static str> {
        match self.stack.last_mut() {
            Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if *field == Some(3) && target.is_none() => {
                *target = Some(flow::neural::Value::Atom(flow::neural::Atom::Decimal(value)));
                *field = None;
            }
            Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Camera(camera), field, .. }) => match field.take() {
                Some(0) => camera.x = value,
                Some(1) => camera.y = value,
                Some(2) => camera.zoom = value,
                _ => return Err("procedural3d-mounted.camera-field"),
            },
            Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Widget(widget), field, .. }) => {
                let field = field.take().ok_or("procedural3d-mounted.widget-number-owner")?;
                *widget.numbers.get_mut(field.saturating_sub(1) as usize).ok_or("procedural3d-mounted.widget-number-field")? = value;
            }
            Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Layout { value: layout, .. }, field, .. }) => match field.take() {
                Some(0) => layout.x = value,
                Some(1) => layout.y = value,
                _ => return Err("procedural3d-mounted.layout-field"),
            },
            _ => return Err("procedural3d-mounted.number-owner"),
        }
        Ok(())
    }

    fn assign_bool(&mut self, value: bool) {
        match self.stack.last_mut() {
            Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Widget(widget), field, .. }) => {
                widget.boolean = value;
                *field = None;
            }
            Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if matches!(*field, Some(0 | 1)) && target.is_none() => {
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
        let matches = matches!((self.json_stack.last(), kind), (Some(Procedural3dMountedJsonFrame::Array(_)), mounted::RetainedValueContainer::List) | (Some(Procedural3dMountedJsonFrame::Object { .. }), mounted::RetainedValueContainer::Map));
        if !matches {
            return Err("procedural3d-mounted.json-container-mismatch");
        }
        let value = match self.json_stack.pop().ok_or("procedural3d-mounted.json-end")? {
            Procedural3dMountedJsonFrame::Array(values) => serde_json::Value::Array(values),
            Procedural3dMountedJsonFrame::Object { values, key: None } => serde_json::Value::Object(values),
            Procedural3dMountedJsonFrame::Object { .. } => return Err("procedural3d-mounted.json-key-without-value"),
        };
        self.assign_json(value)?;
        Ok(true)
    }

    fn begin_dictionary(&mut self, destination: Procedural3dMountedDictionaryDestination, count: u64) -> Result<(), &'static str> {
        let rows = usize::try_from(count).map_err(|_| "procedural3d-mounted.dictionary-count")?;
        let mut values = Vec::new();
        values.try_reserve_exact(rows).map_err(|_| "procedural3d-mounted.dictionary-preflight")?;
        values.resize_with(rows, Procedural3dMountedDictionaryEntryOwner::default);
        let mut present = Vec::new();
        present.try_reserve_exact(rows).map_err(|_| "procedural3d-mounted.dictionary-presence-preflight")?;
        present.resize(rows, false);
        self.push(Procedural3dMountedContainerOwner::Dictionary { destination, rows: values, field: None, present, next: 0 })
    }

    fn finish_dictionary(&mut self, destination: Procedural3dMountedDictionaryDestination, rows: Vec<Procedural3dMountedDictionaryEntryOwner>) -> Result<(), &'static str> {
        let mut dictionary = flow::neural::Dictionary::new();
        for row in rows {
            dictionary = dictionary.insert(row.key, row.value.ok_or("procedural3d-mounted.dictionary-value")?);
        }
        match destination {
            Procedural3dMountedDictionaryDestination::Widget { parent, field } => match self.stack.get_mut(parent) {
                Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Widget(widget), field: active, .. }) if *active == Some(field) => {
                    widget.dictionaries[if field == 1 { 1 } else { 0 }] = dictionary;
                    *active = None;
                }
                _ => return Err("procedural3d-mounted.dictionary-widget-owner"),
            },
            Procedural3dMountedDictionaryDestination::Value { parent } => match self.stack.get_mut(parent) {
                Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::NeuralValue { value, .. }, field, .. }) if *field == Some(5) && value.is_none() => {
                    *value = Some(flow::neural::Value::Dictionary(dictionary));
                    *field = None;
                }
                _ => return Err("procedural3d-mounted.dictionary-value-owner"),
            },
        }
        Ok(())
    }

    fn begin_record(&mut self, count: u64) -> Result<(), &'static str> {
        if count > 64 {
            return Err("procedural3d-mounted.record-field-count");
        }
        let root_field = self.current_root_field();
        if let Some(table) = self.stack.len().checked_sub(1) {
            if let Some(Procedural3dMountedContainerOwner::Dictionary { field: Some(1), present, next, .. }) = self.stack.get_mut(table) {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("procedural3d-mounted.dictionary-value-row")?;
                *next = row + 1;
                return self.push(Procedural3dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner: Procedural3dMountedRecordOwner::NeuralValue { table, row, value: None } });
            }
        }
        let owner = if self.stack.is_empty() {
            Procedural3dMountedRecordOwner::Root
        } else if root_field == Some(1) {
            Procedural3dMountedRecordOwner::Camera(flow::CameraJson::default())
        } else if root_field == Some(2) {
            let keyword = match self.stack.last_mut() {
                Some(Procedural3dMountedContainerOwner::Statements { keyword, .. }) => keyword.take().ok_or("procedural3d-mounted.widget-keyword")?,
                _ => return Err("procedural3d-mounted.widget-statements-owner"),
            };
            Procedural3dMountedRecordOwner::Widget(Procedural3dMountedWidgetOwner { keyword, ..Procedural3dMountedWidgetOwner::default() })
        } else if root_field == Some(4) {
            let key = match self.stack.last_mut() {
                Some(Procedural3dMountedContainerOwner::LayoutMap { key }) => key.take().ok_or("procedural3d-mounted.layout-key")?,
                _ => return Err("procedural3d-mounted.layout-map-owner"),
            };
            Procedural3dMountedRecordOwner::Layout { key, value: flow::WidgetLayout { x: 0.0, y: 0.0 } }
        } else {
            Procedural3dMountedRecordOwner::Structural
        };
        self.push(Procedural3dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner })
    }

    fn begin_container(&mut self, kind: mounted::RetainedValueContainer, count: u64) -> Result<(), &'static str> {
        let root_field = self.current_root_field().ok_or("procedural3d-mounted.container-root")?;
        match kind {
            mounted::RetainedValueContainer::Statements => self.push(Procedural3dMountedContainerOwner::Statements { root_field, keyword: None }),
            mounted::RetainedValueContainer::Table if root_field == 2 => {
                let parent = self.stack.len().checked_sub(1).ok_or("procedural3d-mounted.dictionary-parent")?;
                let destination = match self.stack.get(parent) {
                    Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Widget(_), field: Some(field @ (1 | 5)), .. }) => Procedural3dMountedDictionaryDestination::Widget { parent, field: *field },
                    Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::NeuralValue { .. }, field: Some(5), .. }) => Procedural3dMountedDictionaryDestination::Value { parent },
                    _ => return self.push(Procedural3dMountedContainerOwner::Structural { kind, root_field: Some(root_field) }),
                };
                self.begin_dictionary(destination, count)
            }
            mounted::RetainedValueContainer::List | mounted::RetainedValueContainer::Tuple if root_field == 2 => {
                let (parent, field) = match self.stack.last() {
                    Some(Procedural3dMountedContainerOwner::Record { field: Some(field), .. }) => (self.stack.len() - 1, *field),
                    _ => return Err("procedural3d-mounted.sequence-owner"),
                };
                let mut values = Vec::new();
                values.try_reserve_exact(count as usize).map_err(|_| "procedural3d-mounted.sequence-preflight")?;
                self.push(Procedural3dMountedContainerOwner::Strings { parent, field, values })
            }
            mounted::RetainedValueContainer::Table if root_field == 3 => {
                let rows = usize::try_from(count).map_err(|_| "procedural3d-mounted.synapse-count")?;
                let mut values = Vec::new();
                values.try_reserve_exact(rows).map_err(|_| "procedural3d-mounted.synapse-preflight")?;
                values.resize_with(rows, Procedural3dMountedSynapseOwner::default);
                let mut present = Vec::new();
                present.try_reserve_exact(rows).map_err(|_| "procedural3d-mounted.synapse-presence-preflight")?;
                present.resize(rows, false);
                self.push(Procedural3dMountedContainerOwner::Synapses { rows: values, field: None, present, next: 0 })
            }
            mounted::RetainedValueContainer::Map if root_field == 4 => self.push(Procedural3dMountedContainerOwner::LayoutMap { key: None }),
            mounted::RetainedValueContainer::Table if root_field == 7 => {
                let rows = usize::try_from(count).map_err(|_| "procedural3d-mounted.generation-count")?;
                let mut values = Vec::new();
                values.try_reserve_exact(rows).map_err(|_| "procedural3d-mounted.generation-preflight")?;
                values.resize_with(rows, Procedural3dMountedGenerationOwner::default);
                let mut present = Vec::new();
                present.try_reserve_exact(rows).map_err(|_| "procedural3d-mounted.generation-presence-preflight")?;
                present.resize(rows, false);
                self.push(Procedural3dMountedContainerOwner::Generations { rows: values, field: None, present, next: 0 })
            }
            mounted::RetainedValueContainer::Map if root_field == 7 => {
                let table = self.stack.len().checked_sub(1).ok_or("procedural3d-mounted.generation-table")?;
                let row = match self.stack.get_mut(table) {
                    Some(Procedural3dMountedContainerOwner::Generations { field: Some(2), present, next, .. }) => {
                        let row = (*next..present.len()).find(|row| present[*row]).ok_or("procedural3d-mounted.generation-row")?;
                        *next = row + 1;
                        row
                    }
                    _ => return Err("procedural3d-mounted.generation-values-owner"),
                };
                self.json_destination = Some((table, row));
                self.json_stack.push(Procedural3dMountedJsonFrame::Object { values: serde_json::Map::new(), key: None });
                Ok(())
            }
            mounted::RetainedValueContainer::Wire if root_field == 3 => {
                let table = self.stack.len().checked_sub(1).ok_or("procedural3d-mounted.wire-table")?;
                let row = match self.stack.get_mut(table) {
                    Some(Procedural3dMountedContainerOwner::Synapses { present, next, .. }) => {
                        let row = (*next..present.len()).find(|row| present[*row]).ok_or("procedural3d-mounted.wire-row")?;
                        *next = row + 1;
                        row
                    }
                    _ => return Err("procedural3d-mounted.wire-table"),
                };
                self.push(Procedural3dMountedContainerOwner::Wire { table, row, roles: [0; 6], roles_len: 0, role: 0, nodes: 0 })
            }
            _ => self.push(Procedural3dMountedContainerOwner::Structural { kind, root_field: Some(root_field) }),
        }
    }

    fn finish_widget(owner: Procedural3dMountedWidgetOwner) -> Result<flow::Widget, &'static str> {
        let [id, second, third, _fourth] = owner.strings;
        let [value, min, max, step] = owner.numbers;
        let [first_list, second_list] = owner.lists;
        let [first_dictionary, second_dictionary] = owner.dictionaries;
        let [first_dynamic, second_dynamic] = owner.dynamic;
        Ok(match owner.keyword.as_str() {
            "neuron" => flow::Widget::Neuron { id, neuron_kind: second, params: first_dictionary, input_ports: first_list, output_ports: second_list, preview: owner.boolean },
            "input-slider" => flow::Widget::InputSlider { id, value, min, max, step },
            "input-note" => flow::Widget::InputNote { id, text: second },
            "input-image" => flow::Widget::InputImage { id, src: second },
            "variable" => flow::Widget::Variable { id, name: second, schema: third },
            "output-preview" => {
                let mut expanded = std::collections::BTreeSet::new();
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
                tree: dsl::from_dsl_value(first_dynamic.ok_or("procedural3d-mounted.cluster-tree")?).map_err(|_| "procedural3d-mounted.cluster-tree-shape")?,
                flow: dsl::from_dsl_value(second_dynamic.ok_or("procedural3d-mounted.cluster-flow")?).map_err(|_| "procedural3d-mounted.cluster-flow-shape")?,
            },
            _ => return Err("procedural3d-mounted.widget-variant"),
        })
    }

    fn end(&mut self, kind: mounted::RetainedValueContainer) -> Result<(), &'static str> {
        let owner = self.stack.pop().ok_or("procedural3d-mounted.end-without-owner")?;
        match owner {
            Procedural3dMountedContainerOwner::Record { root_field: None, seen, owner: Procedural3dMountedRecordOwner::Root, field: None } => {
                if seen & PROCEDURAL3D_REQUIRED_SNAPSHOT_FIELDS != PROCEDURAL3D_REQUIRED_SNAPSHOT_FIELDS {
                    return Err("procedural3d-mounted.snapshot-fields-missing");
                }
            }
            Procedural3dMountedContainerOwner::Record { root_field: Some(1), owner: Procedural3dMountedRecordOwner::Camera(camera), field: None, .. } => {
                self.candidate.as_mut().ok_or("procedural3d-mounted.snapshot-owner")?.fixture.camera = camera;
                if let Some(Procedural3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Procedural3dMountedContainerOwner::Record { root_field: Some(2), owner: Procedural3dMountedRecordOwner::Widget(widget), field: None, .. } => {
                self.candidate.as_mut().ok_or("procedural3d-mounted.snapshot-owner")?.fixture.widgets.push(Self::finish_widget(widget)?);
            }
            Procedural3dMountedContainerOwner::Record { root_field: Some(4), owner: Procedural3dMountedRecordOwner::Layout { key, value }, field: None, .. } => {
                self.candidate.as_mut().ok_or("procedural3d-mounted.snapshot-owner")?.fixture.layout.insert(key, value);
            }
            Procedural3dMountedContainerOwner::Strings { parent, field, values } => match self.stack.get_mut(parent) {
                Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Widget(widget), field: active, .. }) => {
                    *widget.lists.get_mut(if field == 4 { 1 } else { 0 }).ok_or("procedural3d-mounted.widget-list-field")? = values;
                    *active = None;
                }
                _ => return Err("procedural3d-mounted.widget-list-owner"),
            },
            Procedural3dMountedContainerOwner::Synapses { rows, .. } if kind == mounted::RetainedValueContainer::Table => {
                let target = &mut self.candidate.as_mut().ok_or("procedural3d-mounted.snapshot-owner")?.fixture.synapses;
                for row in rows {
                    target.push(flow::SynapseSpec { id: row.id, from: row.from, to: row.to, from_port: row.from_port, to_port: row.to_port });
                }
                if let Some(Procedural3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Procedural3dMountedContainerOwner::LayoutMap { key: None } if kind == mounted::RetainedValueContainer::Map => {
                if let Some(Procedural3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Procedural3dMountedContainerOwner::Generations { rows, .. } if kind == mounted::RetainedValueContainer::Table => {
                let target = &mut self.candidate.as_mut().ok_or("procedural3d-mounted.snapshot-owner")?.generation.generations;
                for row in rows {
                    target.push(flow::playbook::FormGeneration { id: row.id, name: row.name, values: row.values });
                }
                if let Some(Procedural3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Procedural3dMountedContainerOwner::Record { root_field: Some(2), field: None, owner: Procedural3dMountedRecordOwner::NeuralValue { table, row, value: Some(value) }, .. } if kind == mounted::RetainedValueContainer::Record => {
                match self.stack.get_mut(table) {
                    Some(Procedural3dMountedContainerOwner::Dictionary { rows, field: Some(1), .. }) => {
                        rows.get_mut(row).ok_or("procedural3d-mounted.dictionary-value-row")?.value = Some(value);
                    }
                    _ => return Err("procedural3d-mounted.dictionary-value-table"),
                }
            }
            Procedural3dMountedContainerOwner::Dictionary { destination, rows, field: Some(1), .. } if kind == mounted::RetainedValueContainer::Table => self.finish_dictionary(destination, rows)?,
            Procedural3dMountedContainerOwner::Wire { .. } if kind == mounted::RetainedValueContainer::Wire => {}
            Procedural3dMountedContainerOwner::Statements { .. } => {
                if let Some(Procedural3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Procedural3dMountedContainerOwner::Structural { kind: expected, .. } if expected == kind => {
                if let Some(Procedural3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::Structural, .. } => {}
            _ => return Err("procedural3d-mounted.container-owner-mismatch"),
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
                            values.try_reserve_exact(usize::try_from(count).map_err(|_| "procedural3d-mounted.dsl-count")?).map_err(|_| "procedural3d-mounted.dsl-preflight")?;
                            self.dsl_stack.push(Procedural3dMountedDslFrame::Array(values));
                        }
                        Container::Map => {
                            let mut values = Vec::new();
                            values.try_reserve_exact(usize::try_from(count).map_err(|_| "procedural3d-mounted.dsl-count")?).map_err(|_| "procedural3d-mounted.dsl-preflight")?;
                            self.dsl_stack.push(Procedural3dMountedDslFrame::Object { values, key: None });
                        }
                        _ => return Err("procedural3d-mounted.dsl-container"),
                    }
                    return Ok(());
                }
                if self.json_destination.is_some() {
                    match kind {
                        Container::List => {
                            let mut values = Vec::new();
                            values.try_reserve_exact(usize::try_from(count).map_err(|_| "procedural3d-mounted.json-count")?).map_err(|_| "procedural3d-mounted.json-preflight")?;
                            self.json_stack.push(Procedural3dMountedJsonFrame::Array(values));
                        }
                        Container::Map => self.json_stack.push(Procedural3dMountedJsonFrame::Object { values: serde_json::Map::new(), key: None }),
                        _ => return Err("procedural3d-mounted.json-container"),
                    }
                    return Ok(());
                }
                if kind == Container::Table && self.pending_table_rows.take() != Some(count) {
                    return Err("procedural3d-mounted.table-row-count");
                }
                self.begin_container(kind, count)?;
            }
            Token::Unsigned { role: Role::FieldId, value } if value < 16 => match self.stack.last_mut() {
                Some(Procedural3dMountedContainerOwner::Record { field, seen, .. }) if field.is_none() => {
                    *field = Some(value as u16);
                    *seen |= 1 << value;
                }
                _ => return Err("procedural3d-mounted.field-owner"),
            },
            Token::Unsigned { role: Role::TableRows, value } => self.pending_table_rows = Some(value),
            Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
                Some(
                    Procedural3dMountedContainerOwner::Synapses { field, present, next, .. } | Procedural3dMountedContainerOwner::Generations { field, present, next, .. } | Procedural3dMountedContainerOwner::Dictionary { field, present, next, .. },
                ) => {
                    *field = Some(u16::try_from(value).map_err(|_| "procedural3d-mounted.table-field")?);
                    present.fill(false);
                    *next = 0;
                }
                _ => {}
            },
            Token::Tag { value: 0x06 | 0x07, .. } => self.begin_string()?,
            Token::Unsigned { role: Role::StringLength, value } => {
                let owner = self.string.as_mut().ok_or("procedural3d-mounted.string-length-owner")?;
                owner.value.try_reserve_exact(value as usize).map_err(|_| "procedural3d-mounted.string-preflight")?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::StringChar(value) => {
                let owner = self.string.as_mut().ok_or("procedural3d-mounted.string-char-owner")?;
                owner.value.push(value);
                let remaining = owner.remaining.as_mut().ok_or("procedural3d-mounted.string-char-length")?;
                *remaining = remaining.checked_sub(value.len_utf8() as u64).ok_or("procedural3d-mounted.string-width")?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::Symbol, value } => self.begin_symbol(value, catalog)?,
            Token::Tag { value: 0x11, .. } if self.json_destination.is_none() => {
                self.begin_dsl()?;
            }
            Token::F64(value) if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Number(f64::from_bits(value)))?,
            Token::F64(value) if self.json_destination.is_some() => self.assign_json(serde_json::Number::from_f64(f64::from_bits(value)).map(serde_json::Value::Number).ok_or("procedural3d-mounted.json-number")?)?,
            Token::F64(value) => self.assign_f64(f64::from_bits(value))?,
            Token::Signed(value) if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Number(value as f64))?,
            Token::Signed(value) if self.json_destination.is_some() => self.assign_json(serde_json::Value::Number(value.into()))?,
            Token::Signed(value) => match self.stack.last_mut() {
                Some(Procedural3dMountedContainerOwner::Record { owner: Procedural3dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if *field == Some(2) && target.is_none() => {
                    *target = Some(flow::neural::Value::Atom(flow::neural::Atom::Integer(value)));
                    *field = None;
                }
                _ => return Err("procedural3d-mounted.integer-owner"),
            },
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.json_destination.is_some() => self.assign_json(serde_json::Value::Number(value.into()))?,
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Number(value as f64))?,
            Token::Tag { value: 0x01, .. } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Bool(false))?,
            Token::Tag { value: 0x02, .. } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Bool(true))?,
            Token::Tag { value: 0x12, .. } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Null)?,
            Token::Tag { value: 0x01, .. } if self.json_destination.is_some() => self.assign_json(serde_json::Value::Bool(false))?,
            Token::Tag { value: 0x02, .. } if self.json_destination.is_some() => self.assign_json(serde_json::Value::Bool(true))?,
            Token::Tag { value: 0x12, .. } if self.json_destination.is_some() => self.assign_json(serde_json::Value::Null)?,
            Token::Tag { value: 0x11, .. } if self.json_destination.is_some() => {}
            Token::Tag { value: 0x01, .. } => self.assign_bool(false),
            Token::Tag { value: 0x02, .. } => self.assign_bool(true),
            Token::Tag { value: 0x00, .. } => {
                if let Some(Procedural3dMountedContainerOwner::Record { root_field: None, field, .. }) = self.stack.last_mut() {
                    if matches!(*field, Some(5 | 6)) {
                        *field = None;
                    }
                }
            }
            Token::WirePresence(_) => {}
            Token::WireNodePresence(presence) => match self.stack.last_mut() {
                Some(Procedural3dMountedContainerOwner::Wire { roles, roles_len, nodes, .. }) => {
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
                _ => return Err("procedural3d-mounted.wire-node-owner"),
            },
            Token::TablePresence { rows, value } => match self.stack.last_mut() {
                Some(Procedural3dMountedContainerOwner::Synapses { present, .. } | Procedural3dMountedContainerOwner::Generations { present, .. } | Procedural3dMountedContainerOwner::Dictionary { present, .. }) if rows as usize == present.len() => {
                    if value == 0 {
                        present.fill(true);
                    }
                }
                _ => {}
            },
            Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
                Some(Procedural3dMountedContainerOwner::Synapses { present, .. } | Procedural3dMountedContainerOwner::Generations { present, .. } | Procedural3dMountedContainerOwner::Dictionary { present, .. }) => {
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
                    return Err("procedural3d-mounted.typed-terminal-populated");
                }
                self.complete = true;
            }
            Token::Tag { .. } | Token::Unsigned { .. } | Token::Signed(_) | Token::Byte(_) | Token::WireLabelPresence(_) => {}
        }
        Ok(())
    }

    fn take(&mut self) -> Option<Procedural3dSnapshot> {
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

impl Drop for Procedural3dMountedTypedSnapshotOwner {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Procedural3d mounted typed snapshot owner reached Drop before handoff or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Procedural3dMountedPackPhase {
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
pub struct Procedural3dMountedPackSession {
    phase: Procedural3dMountedPackPhase,
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
    typed: std::mem::ManuallyDrop<Option<Procedural3dMountedTypedSnapshotOwner>>,
    catalog_value: std::mem::ManuallyDrop<Option<mounted::RetainedPackCatalog>>,
    source_complete: bool,
    segment_complete: bool,
    anchor_ready: bool,
    catalog_complete: bool,
    value_sealed: bool,
    value_complete: bool,
}

impl Procedural3dMountedPackSession {
    pub fn new(expected_bytes: usize, maximum_items: usize) -> Result<Self, &'static str> {
        if expected_bytes <= PROCEDURAL3D_MOUNTED_PREFIX.len() || expected_bytes > store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES || maximum_items == 0 {
            return Err("procedural3d-mounted.exact-credits");
        }
        Ok(Self {
            phase: Procedural3dMountedPackPhase::Prefix,
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
        if self.prefix != PROCEDURAL3D_MOUNTED_PREFIX {
            return Err("procedural3d-mounted.schema-discriminator");
        }
        let canonical = self.expected_bytes - PROCEDURAL3D_MOUNTED_PREFIX.len();
        let pages = canonical.div_ceil(mounted::RETAINED_PACK_PAGE_BYTES);
        let maximum_symbols = self.maximum_items.min(u32::MAX as usize) as u32;
        let limits = || mounted::PackLimits {
            max_file_len: canonical as u64,
            max_segment_len: canonical as u64,
            max_symbols: maximum_symbols,
            max_depth: PROCEDURAL3D_MOUNTED_TYPED_DEPTH as u16,
            max_items: self.maximum_items as u64,
            max_total_alloc: canonical as u64,
        };
        *self.source = Some(mounted::RetainedPackSourceCursor::try_new(pages, canonical)?);
        *self.anchor = Some(mounted::RetainedPackAnchorCursor::new());
        *self.segment = Some(mounted::RetainedPackSegmentCursor::try_new(limits()).map_err(|_| "procedural3d-mounted.segment-preflight")?);
        *self.catalog = Some(mounted::RetainedPackCatalogCursor::try_new(limits(), maximum_symbols as usize, self.maximum_items, canonical).map_err(|_| "procedural3d-mounted.catalog-preflight")?);
        *self.value = Some(mounted::RetainedValueCursor::try_new(limits()).map_err(|_| "procedural3d-mounted.value-preflight")?);
        *self.typed = Some(Procedural3dMountedTypedSnapshotOwner::new()?);
        self.phase = Procedural3dMountedPackPhase::Ingress;
        Ok(())
    }

    pub fn admit_byte(&mut self, value: u8) -> Result<(), u8> {
        if !matches!(self.phase, Procedural3dMountedPackPhase::Prefix | Procedural3dMountedPackPhase::Ingress) || self.admitted == self.expected_bytes {
            return Err(value);
        }
        if self.prefix_len < PROCEDURAL3D_MOUNTED_PREFIX.len() {
            if value != PROCEDURAL3D_MOUNTED_PREFIX[self.prefix_len] {
                return Err(value);
            }
            self.prefix[self.prefix_len] = value;
            self.prefix_len += 1;
            self.admitted += 1;
            if self.prefix_len == PROCEDURAL3D_MOUNTED_PREFIX.len() && self.allocate_after_discriminator().is_err() {
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
        let page = mounted::RetainedPackPage::try_from_array(std::mem::replace(&mut self.page, [0; mounted::RETAINED_PACK_PAGE_BYTES]), len).map_err(|_| "procedural3d-mounted.page-owner")?;
        self.page_len = 0;
        let source = self.source.as_mut().ok_or("procedural3d-mounted.source-owner")?;
        source.preflight_page(len)?;
        source.admit_page(page).map_err(|_| "procedural3d-mounted.producer-handback")
    }

    pub fn seal(&mut self) -> Result<(), &'static str> {
        if self.admitted != self.expected_bytes || self.prefix_len != PROCEDURAL3D_MOUNTED_PREFIX.len() {
            return Err("procedural3d-mounted.exact-byte-seal");
        }
        self.flush_page()?;
        self.source.as_mut().ok_or("procedural3d-mounted.source-owner")?.seal()?;
        self.phase = Procedural3dMountedPackPhase::Drive;
        Ok(())
    }

    pub fn grant(&mut self) -> Result<bool, &'static str> {
        if matches!(self.phase, Procedural3dMountedPackPhase::Ready | Procedural3dMountedPackPhase::Published) {
            return Ok(true);
        }
        if self.phase != Procedural3dMountedPackPhase::Drive {
            return Err("procedural3d-mounted.missing-seal");
        }
        if self.typed.as_mut().ok_or("procedural3d-mounted.typed-owner")?.grant_symbol(self.catalog.as_ref().ok_or("procedural3d-mounted.catalog-owner")?)? {
            return Ok(false);
        }
        if !self.value_complete {
            if let Some(token) = self.value.as_mut().ok_or("procedural3d-mounted.value-owner")?.grant().map_err(|_| "procedural3d-mounted.value-malformed")? {
                self.value_complete = matches!(token, mounted::RetainedValueToken::Complete { .. });
                self.typed.as_mut().expect("P3 typed owner retained").accept(token, self.catalog.as_ref().expect("P3 catalog retained"))?;
                return Ok(false);
            }
        }
        if self.catalog_complete && !self.value_sealed {
            let bytes = self.catalog.as_ref().expect("P3 catalog retained").document_bytes();
            self.value.as_mut().expect("P3 value owner retained").seal(bytes).map_err(|_| "procedural3d-mounted.value-seal")?;
            self.value_sealed = true;
            return Ok(false);
        }
        if !self.segment_complete && (self.segment.as_ref().ok_or("procedural3d-mounted.segment-owner")?.preflight().is_err() || self.source_complete) {
            if let Some(event) = self.segment.as_mut().expect("P3 segment retained").grant().map_err(|_| "procedural3d-mounted.segment-malformed")? {
                self.segment_complete = matches!(event, mounted::RetainedPackSegmentEvent::PackComplete { .. });
                let catalog = self.catalog.as_mut().expect("P3 catalog retained");
                catalog.admit(event).map_err(|_| "procedural3d-mounted.catalog-backpressure")?;
                if let Some(event) = catalog.grant().map_err(|_| "procedural3d-mounted.catalog-malformed")? {
                    match event {
                        mounted::RetainedPackCatalogEvent::DocumentByte { index, value, .. } => self.value.as_mut().expect("P3 value retained").admit_byte(index, value).map_err(|_| "procedural3d-mounted.value-backpressure")?,
                        mounted::RetainedPackCatalogEvent::Complete => self.catalog_complete = true,
                        _ => {}
                    }
                }
                return Ok(false);
            }
        }
        if !self.source_complete && self.segment.as_ref().expect("P3 segment retained").preflight().is_ok() {
            if let Some(event) = self.source.as_mut().ok_or("procedural3d-mounted.source-owner")?.grant()? {
                self.source_complete = matches!(event, mounted::RetainedPackSourceEvent::Complete { .. });
                self.anchor.as_mut().expect("P3 anchor retained").grant(Some(event)).map_err(|_| "procedural3d-mounted.anchor-malformed")?;
                self.segment.as_mut().expect("P3 segment retained").admit(event).map_err(|_| "procedural3d-mounted.segment-handback")?;
                return Ok(false);
            }
        }
        if self.source_complete && !self.anchor_ready {
            self.anchor_ready = self.anchor.as_mut().expect("P3 anchor retained").grant(None).map_err(|_| "procedural3d-mounted.anchor-malformed")?;
            return Ok(false);
        }
        if self.anchor_ready && self.catalog_complete && self.value_complete && self.catalog_value.is_none() {
            let superblock = self.anchor.as_mut().expect("P3 anchor retained").take().ok_or("procedural3d-mounted.anchor-handoff")?;
            *self.catalog_value = self.catalog.as_mut().expect("P3 catalog retained").take(superblock).map_err(|_| "procedural3d-mounted.catalog-validation")?;
            self.phase = Procedural3dMountedPackPhase::Ready;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn take(&mut self) -> Option<Procedural3dSnapshot> {
        if self.phase != Procedural3dMountedPackPhase::Ready {
            return None;
        }
        let value = self.typed.as_mut()?.take()?;
        self.phase = Procedural3dMountedPackPhase::Published;
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
        self.phase = Procedural3dMountedPackPhase::Closing;
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<bool, &'static str> {
        self.phase = Procedural3dMountedPackPhase::Closing;
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
        self.phase = Procedural3dMountedPackPhase::Closed;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == Procedural3dMountedPackPhase::Closed && self.source.is_none() && self.anchor.is_none() && self.segment.is_none() && self.catalog.is_none() && self.value.is_none() && self.typed.is_none() && self.catalog_value.is_none()
    }
}

impl Drop for Procedural3dMountedPackSession {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Procedural3d mounted canonical pack session reached Drop before exact terminal-empty close");
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

    fn close(session: &mut Procedural3dMountedPackSession) {
        session.request_cancel();
        for _ in 0..100_000 {
            if session.close_step(1, mounted::RETAINED_PACK_PAGE_BYTES).expect("P3 retained session close") {
                assert!(session.terminal_is_empty());
                return;
            }
        }
        panic!("P3 retained session did not reach terminal-empty close");
    }

    #[test]
    fn non_empty_canonical_snapshot_round_trips_one_grant_at_a_time() {
        let mut expected = Procedural3dSnapshot::default();
        let nested = flow::neural::Dictionary::new().insert("enabled", flow::neural::Value::Atom(flow::neural::Atom::Boolean(true)));
        let params = flow::neural::Dictionary::new().insert("gain", flow::neural::Value::Atom(flow::neural::Atom::Decimal(2.5))).insert("nested", flow::neural::Value::Dictionary(nested));
        expected.fixture.widgets.push(flow::Widget::Neuron { id: "retained-neuron".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true });
        let mut expanded = std::collections::BTreeSet::new();
        expanded.insert("answer".into());
        let preview = flow::neural::Dictionary::new().insert("answer", flow::neural::Value::Atom(flow::neural::Atom::String("visible".into())));
        expected.fixture.widgets.push(flow::Widget::OutputPreview { id: "retained-preview".into(), preview, expanded });
        expected.fixture.widgets.push(flow::Widget::Cluster { id: "retained-cluster".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() });
        expected.fixture.synapses.push(flow::SynapseSpec { id: "retained-synapse".into(), from: "retained-neuron".into(), to: "retained-preview".into(), from_port: "out".into(), to_port: String::new() });
        expected.fixture.layout.insert("retained-neuron".into(), flow::WidgetLayout { x: 12.5, y: -8.25 });
        expected.fixture.layout.insert("retained-preview".into(), flow::WidgetLayout { x: 36.0, y: -8.25 });
        let mut values = serde_json::Map::new();
        values.insert("nested".into(), serde_json::json!({"array": [true, null, 3.5], "text": "retained"}));
        expected.generation.generations.push(flow::playbook::FormGeneration { id: "retained-generation".into(), name: "Generation".into(), values });
        expected.generation.selected_generation_id = Some("retained-generation".into());
        expected.generation.preview_text = Some("preview".into());
        assert!(!expected.fixture.widgets.is_empty());
        assert!(!expected.fixture.synapses.is_empty());
        assert!(!expected.fixture.layout.is_empty());
        let bytes = encode(&expected);
        assert_eq!(&bytes[..4], &PROCEDURAL3D_MOUNTED_PREFIX);
        let expected_ledger = bytes[4..].iter().fold(0xcbf2_9ce4_8422_2325u64, |ledger, byte| (ledger ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3));
        let mut session = Procedural3dMountedPackSession::new(bytes.len(), 8_192).expect("P3 retained snapshot preflight");
        for byte in bytes {
            session.admit_byte(byte).expect("one admitted snapshot byte");
        }
        assert_eq!(session.canonical_ingress_ledger(), expected_ledger, "bytes after P3D3 must be the unchanged canonical SPK stream");
        session.seal().expect("exact snapshot seal");
        let mut ready = false;
        for _ in 0..1_000_000 {
            if session.grant().expect("one retained snapshot grant") {
                ready = true;
                break;
            }
        }
        assert!(ready, "P3 retained canonical route must converge");
        let actual = session.take().expect("typed snapshot handoff");
        assert_eq!(actual.fixture.synapses.len(), expected.fixture.synapses.len(), "typed synapse owner must retain the exact row census");
        assert_eq!(actual.fixture.synapses.last(), expected.fixture.synapses.last(), "typed synapse owner must retain the exact non-empty appended row");
        assert_eq!(synapse_digest(actual.fixture.synapses.last().expect("typed retained synapse")), synapse_digest(expected.fixture.synapses.last().expect("expected retained synapse")));
        assert_eq!(actual.fixture.layout, expected.fixture.layout, "typed layout owner must retain the semantically attached widget positions");
        assert_eq!(actual, expected, "all typed snapshot owners must round-trip exactly");
        close(&mut session);
    }

    #[test]
    fn p2d2_is_rejected_before_semantic_allocation() {
        let mut session = Procedural3dMountedPackSession::new(8, 8).expect("P3 hostile discriminator preflight");
        session.admit_byte(b'P').expect("shared first discriminator byte");
        assert!(!session.semantic_allocated());
        assert_eq!(session.admit_byte(b'2'), Err(b'2'));
        assert!(!session.semantic_allocated());
        close(&mut session);
    }
}
