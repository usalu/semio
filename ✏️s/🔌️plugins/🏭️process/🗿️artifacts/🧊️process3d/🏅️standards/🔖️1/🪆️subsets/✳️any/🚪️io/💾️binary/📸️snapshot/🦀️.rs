//! binary rep for stdio.json 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessStep, Stock, WorkingSolid, Workshop, WorkshopMachine};
use framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use store::mounted_pack_rt as mounted;

impl store::ArtifactPack for Process3dSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
}
pub use snapshot_codec::*;

mod mounted_snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessStep, Stock, WorkingSolid, Workshop, WorkshopMachine};
use framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use store::mounted_pack_rt as mounted;
//#region 🔖️MountedTypedSnapshotOwner
/// 📐️ Structural nesting bound of the mounted route: root → workshop → machine → capability →
/// rule/recipe statement → record is the deepest path, each level costing a few pack frames.
const PROCESS3D_MOUNTED_DEPTH: u16 = 32;

/// 🧵️ The mounted canonical session of this artifact: its own semio pack header, then the unchanged
/// canonical `.spk` stream the derived [`store::ArtifactPack`] encode writes.
pub type Process3dMountedPackSession = mounted::RetainedTypedPackSession<Process3dMountedSnapshotOwner>;



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dMountedList {
    Machines,
    Capabilities,
    Parameters,
    Steps,
    Tools,
}

/// 🗂️ One open container. Typed frames own the unbounded collections (machines, capabilities and the
/// root lists) and fill domain values directly; leaf frames build one bounded record, sequence or
/// statement value that its typed owner converts through the derived `DslField` on completion.
enum Process3dMountedFrame {
    Root { spec: semio_framework_dsl_record::RecordSpec, field: Option<u16> },
    Workshop { spec: semio_framework_dsl_record::RecordSpec, field: Option<u16> },
    Machine { spec: semio_framework_dsl_record::RecordSpec, value: WorkshopMachine, field: Option<u16> },
    Capability { spec: semio_framework_dsl_record::RecordSpec, value: Capability, field: Option<u16> },
    List { list: Process3dMountedList, item: semio_framework_dsl_record::Shape },
    Rules { variants: Vec<(String, semio_framework_dsl_record::RecordSpecProducer)>, keyword: Option<String> },
    Record { spec: semio_framework_dsl_record::RecordSpec, record: semio_framework_dsl_record::RecordValue, field: Option<u16> },
    Sequence { item: semio_framework_dsl_record::Shape, tuple: bool, items: Vec<semio_framework_dsl_record::FieldValue> },
    Statements { variants: Vec<(String, semio_framework_dsl_record::RecordSpecProducer)>, items: Vec<(String, semio_framework_dsl_record::RecordValue)>, keyword: Option<String> },
}

struct Process3dMountedString {
    value: String,
    remaining: Option<u64>,
    symbol: Option<(u64, usize, usize)>,
}

/// 🧬️ Fixed-depth typed owner consuming catalog/value events directly into `Process3dSnapshot`
/// fields, one scalar or structural event per retained grant. It never builds the whole document as
/// a schema-erased record and cannot reach a batch pack decoder.
pub struct Process3dMountedSnapshotOwner {
    candidate: std::mem::ManuallyDrop<Option<Process3dSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    stack: Vec<Process3dMountedFrame>,
    string: Option<Process3dMountedString>,
    complete: bool,
    handed_back: bool,
}

fn process3d_field_shape(spec: &semio_framework_dsl_record::RecordSpec, id: u16) -> Result<(&str, semio_framework_dsl_record::Shape), &'static str> {
    let field = spec.fields.iter().find(|field| field.id == id).ok_or("process3d-mounted.unknown-field")?;
    let shape = match &field.shape {
        semio_framework_dsl_record::Shape::Block(inner) => (**inner).clone(),
        shape => shape.clone(),
    };
    Ok((field.key.as_str(), shape))
}

fn process3d_variant_shape(variants: &[(String, semio_framework_dsl_record::RecordSpecProducer)], keyword: &Option<String>) -> Result<semio_framework_dsl_record::Shape, &'static str> {
    let keyword = keyword.as_deref().ok_or("process3d-mounted.statement-keyword")?;
    variants.iter().find(|(name, _)| name == keyword).map(|(_, spec)| semio_framework_dsl_record::Shape::Record(*spec)).ok_or("process3d-mounted.statement-variant")
}

fn process3d_list_item(shape: &semio_framework_dsl_record::Shape) -> Result<semio_framework_dsl_record::Shape, &'static str> {
    match shape {
        semio_framework_dsl_record::Shape::List(item) => Ok((**item).clone()),
        _ => Err("process3d-mounted.list-shape"),
    }
}

fn process3d_empty_child<S>() -> store::ArtifactChild<S> {
    store::ArtifactChild::new(String::new(), store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } })
}

fn process3d_text(value: semio_framework_dsl_record::FieldValue) -> Result<String, &'static str> {
    match value {
        semio_framework_dsl_record::FieldValue::Text(text) => Ok(text),
        _ => Err("process3d-mounted.text-shape"),
    }
}

fn process3d_field<T: semio_framework_dsl_record::DslField>(value: &semio_framework_dsl_record::FieldValue) -> Result<T, &'static str> {
    T::from_value(value).map_err(|_| "process3d-mounted.field-shape")
}

impl Process3dMountedSnapshotOwner {
    fn new() -> Result<Self, &'static str> {
        let mut stack = Vec::new();
        stack.try_reserve_exact(PROCESS3D_MOUNTED_DEPTH as usize).map_err(|_| "process3d-mounted.stack-preflight")?;
        let candidate = Process3dSnapshot {
            workshop: Workshop { machines: Vec::new() },
            stock_id: String::new(),
            stock_label: String::new(),
            stock_pose: Pose::default(),
            stock_payload: Stock { id: String::new(), label: String::new(), solid: WorkingSolid::Box { width: 0.0, depth: 0.0, height: 0.0 }, pose: Pose::default() },
            stock_solid: process3d_empty_child(),
            steps: process3d_empty_child(),
            step_payloads: Vec::new(),
            tool_solids: Vec::new(),
        };
        Ok(Self { candidate: std::mem::ManuallyDrop::new(Some(candidate)), retirement: std::mem::ManuallyDrop::new(None), stack, string: None, complete: false, handed_back: false })
    }

    fn candidate(&mut self) -> Result<&mut Process3dSnapshot, &'static str> {
        self.candidate.as_mut().ok_or("process3d-mounted.snapshot-owner")
    }

    fn push(&mut self, frame: Process3dMountedFrame) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("process3d-mounted.depth");
        }
        self.stack.push(frame);
        Ok(())
    }

    fn capability_below(&mut self) -> Result<&mut Capability, &'static str> {
        let index = self.stack.len().checked_sub(2).ok_or("process3d-mounted.capability-owner")?;
        match self.stack.get_mut(index) {
            Some(Process3dMountedFrame::Capability { value, .. }) => Ok(value),
            _ => Err("process3d-mounted.capability-owner"),
        }
    }

    fn expected_shape(&self) -> Result<semio_framework_dsl_record::Shape, &'static str> {
        match self.stack.last().ok_or("process3d-mounted.shape-owner")? {
            Process3dMountedFrame::Root { spec, field: Some(id) }
            | Process3dMountedFrame::Workshop { spec, field: Some(id) }
            | Process3dMountedFrame::Machine { spec, field: Some(id), .. }
            | Process3dMountedFrame::Capability { spec, field: Some(id), .. }
            | Process3dMountedFrame::Record { spec, field: Some(id), .. } => Ok(process3d_field_shape(spec, *id)?.1),
            Process3dMountedFrame::List { item, .. } | Process3dMountedFrame::Sequence { item, .. } => Ok(item.clone()),
            Process3dMountedFrame::Rules { variants, keyword } | Process3dMountedFrame::Statements { variants, keyword, .. } => process3d_variant_shape(variants, keyword),
            _ => Err("process3d-mounted.shape-field"),
        }
    }

    fn typed_key(&self) -> Option<&str> {
        match self.stack.last()? {
            Process3dMountedFrame::Root { spec, field: Some(id) } | Process3dMountedFrame::Workshop { spec, field: Some(id) } | Process3dMountedFrame::Machine { spec, field: Some(id), .. } | Process3dMountedFrame::Capability { spec, field: Some(id), .. } => {
                process3d_field_shape(spec, *id).ok().map(|(key, _)| key)
            }
            _ => None,
        }
    }

    fn typed_list(&mut self, list: Process3dMountedList, item: semio_framework_dsl_record::Shape, count: usize) -> Result<Process3dMountedFrame, &'static str> {
        let reserved = match list {
            Process3dMountedList::Machines => self.candidate()?.workshop.machines.try_reserve_exact(count),
            Process3dMountedList::Steps => self.candidate()?.step_payloads.try_reserve_exact(count),
            Process3dMountedList::Tools => self.candidate()?.tool_solids.try_reserve_exact(count),
            Process3dMountedList::Capabilities => match self.stack.last_mut() {
                Some(Process3dMountedFrame::Machine { value, .. }) => value.capabilities.try_reserve_exact(count),
                _ => return Err("process3d-mounted.capability-list-owner"),
            },
            Process3dMountedList::Parameters => match self.stack.last_mut() {
                Some(Process3dMountedFrame::Capability { value, .. }) => value.parameters.try_reserve_exact(count),
                _ => return Err("process3d-mounted.parameter-list-owner"),
            },
        };
        reserved.map_err(|_| "process3d-mounted.list-preflight")?;
        Ok(Process3dMountedFrame::List { list, item })
    }

    fn begin(&mut self, kind: mounted::RetainedValueContainer, count: u64) -> Result<(), &'static str> {
        use mounted::RetainedValueContainer as Container;
        let count = usize::try_from(count).map_err(|_| "process3d-mounted.count")?;
        if self.stack.is_empty() {
            if kind != Container::Record || self.complete {
                return Err("process3d-mounted.root-shape");
            }
            return self.push(Process3dMountedFrame::Root { spec: Process3dSnapshot::__dsl_spec(), field: None });
        }
        let shape = self.expected_shape()?;
        let key = self.typed_key().map(str::to_owned);
        let under = match self.stack.last() {
            Some(Process3dMountedFrame::List { list, .. }) => Some(*list),
            _ => None,
        };
        let frame = match (key.as_deref(), under, kind) {
            (Some("workshop"), _, Container::Record) => Process3dMountedFrame::Workshop { spec: Workshop::__dsl_spec(), field: None },
            (Some("machines"), _, Container::List) => self.typed_list(Process3dMountedList::Machines, process3d_list_item(&shape)?, count)?,
            (Some("step-payloads"), _, Container::List) => self.typed_list(Process3dMountedList::Steps, process3d_list_item(&shape)?, count)?,
            (Some("tool-solids"), _, Container::List) => self.typed_list(Process3dMountedList::Tools, process3d_list_item(&shape)?, count)?,
            (Some("capabilities"), _, Container::List) => self.typed_list(Process3dMountedList::Capabilities, process3d_list_item(&shape)?, count)?,
            (Some("parameters"), _, Container::List) => self.typed_list(Process3dMountedList::Parameters, process3d_list_item(&shape)?, count)?,
            (Some("rules"), _, Container::Statements) => match shape {
                semio_framework_dsl_record::Shape::Statements(variants) => {
                    self.capability_rules_reserve(count)?;
                    Process3dMountedFrame::Rules { variants, keyword: None }
                }
                _ => return Err("process3d-mounted.rules-shape"),
            },
            (_, Some(Process3dMountedList::Machines), Container::Record) => {
                let value = WorkshopMachine { id: String::new(), label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() };
                Process3dMountedFrame::Machine { spec: WorkshopMachine::__dsl_spec(), value, field: None }
            }
            (_, Some(Process3dMountedList::Capabilities), Container::Record) => {
                let value = Capability { id: String::new(), label: String::new(), icon_id: String::new(), recipe: MeasureRecipe::DiscCut { diameter: String::new(), kerf: String::new() }, parameters: Vec::new(), rules: Vec::new() };
                Process3dMountedFrame::Capability { spec: Capability::__dsl_spec(), value, field: None }
            }
            (_, _, Container::Record) => match shape {
                semio_framework_dsl_record::Shape::Record(spec) => Process3dMountedFrame::Record { spec: (spec.ordinary)(), record: semio_framework_dsl_record::RecordValue::default(), field: None },
                _ => return Err("process3d-mounted.record-shape"),
            },
            (_, _, Container::Tuple | Container::List | Container::PackedF64 | Container::PackedVarint) => {
                let (item, tuple) = match shape {
                    semio_framework_dsl_record::Shape::Tuple(item, _) => (*item, true),
                    semio_framework_dsl_record::Shape::List(item) => (*item, false),
                    semio_framework_dsl_record::Shape::Coord(_) | semio_framework_dsl_record::Shape::Dir | semio_framework_dsl_record::Shape::Dim(_) | semio_framework_dsl_record::Shape::Range => (semio_framework_dsl_record::Shape::Float, true),
                    _ => return Err("process3d-mounted.sequence-shape"),
                };
                let mut items = Vec::new();
                items.try_reserve_exact(count).map_err(|_| "process3d-mounted.sequence-preflight")?;
                Process3dMountedFrame::Sequence { item, tuple, items }
            }
            (_, _, Container::Statements) => match shape {
                semio_framework_dsl_record::Shape::Statements(variants) => {
                    let mut items = Vec::new();
                    items.try_reserve_exact(count).map_err(|_| "process3d-mounted.statements-preflight")?;
                    Process3dMountedFrame::Statements { variants, items, keyword: None }
                }
                _ => return Err("process3d-mounted.statements-shape"),
            },
            _ => return Err("process3d-mounted.container"),
        };
        self.push(frame)
    }

    fn capability_rules_reserve(&mut self, count: usize) -> Result<(), &'static str> {
        match self.stack.last_mut() {
            Some(Process3dMountedFrame::Capability { value, .. }) => value.rules.try_reserve_exact(count).map_err(|_| "process3d-mounted.rules-preflight"),
            _ => Err("process3d-mounted.rules-owner"),
        }
    }

    fn clear_field(&mut self) {
        if let Some(Process3dMountedFrame::Root { field, .. } | Process3dMountedFrame::Workshop { field, .. } | Process3dMountedFrame::Machine { field, .. } | Process3dMountedFrame::Capability { field, .. }) = self.stack.last_mut() {
            *field = None;
        }
    }

    fn deliver(&mut self, value: semio_framework_dsl_record::FieldValue) -> Result<(), &'static str> {
        let absent = matches!(value, semio_framework_dsl_record::FieldValue::Absent);
        let key = self.typed_key().map(str::to_owned);
        match self.stack.last_mut().ok_or("process3d-mounted.value-owner")? {
            Process3dMountedFrame::Root { .. } => {
                self.clear_field();
                if absent {
                    return Ok(());
                }
                let candidate = self.candidate()?;
                match key.as_deref() {
                    Some("stock-id") => candidate.stock_id = process3d_text(value)?,
                    Some("stock-label") => candidate.stock_label = process3d_text(value)?,
                    Some("stock-pose") => candidate.stock_pose = process3d_field(&value)?,
                    Some("stock-payload") => candidate.stock_payload = process3d_field(&value)?,
                    Some("stock-solid") => candidate.stock_solid = process3d_field(&value)?,
                    Some("steps") => candidate.steps = process3d_field(&value)?,
                    _ => return Err("process3d-mounted.root-field"),
                }
            }
            Process3dMountedFrame::Machine { value: machine, field, .. } => {
                *field = None;
                if absent {
                    return Ok(());
                }
                match key.as_deref() {
                    Some("id") => machine.id = process3d_text(value)?,
                    Some("label") => machine.label = process3d_text(value)?,
                    Some("icon-id") => machine.icon_id = process3d_text(value)?,
                    Some("catalog-id") => machine.catalog_id = Some(process3d_text(value)?),
                    _ => return Err("process3d-mounted.machine-field"),
                }
            }
            Process3dMountedFrame::Capability { value: capability, field, .. } => {
                *field = None;
                if absent {
                    return Ok(());
                }
                match key.as_deref() {
                    Some("id") => capability.id = process3d_text(value)?,
                    Some("label") => capability.label = process3d_text(value)?,
                    Some("icon-id") => capability.icon_id = process3d_text(value)?,
                    Some("recipe") => capability.recipe = process3d_field(&value)?,
                    _ => return Err("process3d-mounted.capability-field"),
                }
            }
            Process3dMountedFrame::List { list: Process3dMountedList::Parameters, .. } => {
                let parameter: CapabilityParameter = process3d_field(&value)?;
                self.capability_below()?.parameters.push(parameter);
            }
            Process3dMountedFrame::List { list: Process3dMountedList::Steps, .. } => {
                let step: ProcessStep = process3d_field(&value)?;
                self.candidate()?.step_payloads.push(step);
            }
            Process3dMountedFrame::List { list: Process3dMountedList::Tools, .. } => {
                let tool = process3d_field(&value)?;
                self.candidate()?.tool_solids.push(tool);
            }
            Process3dMountedFrame::Rules { keyword, .. } => match (keyword.take(), value) {
                (None, semio_framework_dsl_record::FieldValue::Text(name)) => *keyword = Some(name),
                (Some(name), semio_framework_dsl_record::FieldValue::Record(record)) => {
                    let rule = <CapabilityRule as semio_framework_dsl_record::DslVariants>::from_named_record(&name, &record).map_err(|_| "process3d-mounted.rule-shape")?;
                    self.capability_below()?.rules.push(rule);
                }
                _ => return Err("process3d-mounted.rule-owner"),
            },
            Process3dMountedFrame::Record { record, field, .. } => {
                let id = field.take().ok_or("process3d-mounted.record-field")?;
                if !absent {
                    record.fields.insert(id, value);
                }
            }
            Process3dMountedFrame::Sequence { item, items, .. } => items.push(match (item, value) {
                (semio_framework_dsl_record::Shape::UInt | semio_framework_dsl_record::Shape::Count, semio_framework_dsl_record::FieldValue::Int(signed)) => semio_framework_dsl_record::FieldValue::UInt(u64::try_from(signed).map_err(|_| "process3d-mounted.unsigned-item")?),
                (semio_framework_dsl_record::Shape::Enum(_), semio_framework_dsl_record::FieldValue::Int(signed)) => semio_framework_dsl_record::FieldValue::Enum(u32::try_from(signed).map_err(|_| "process3d-mounted.enum-item")?),
                (_, value) => value,
            }),
            Process3dMountedFrame::Statements { keyword, items, .. } => match (keyword.take(), value) {
                (None, semio_framework_dsl_record::FieldValue::Text(name)) => *keyword = Some(name),
                (Some(name), semio_framework_dsl_record::FieldValue::Record(record)) => items.push((name, record)),
                _ => return Err("process3d-mounted.statement-owner"),
            },
            _ => return Err("process3d-mounted.value-owner"),
        }
        Ok(())
    }

    fn end(&mut self, kind: mounted::RetainedValueContainer) -> Result<(), &'static str> {
        use mounted::RetainedValueContainer as Container;
        match (self.stack.pop().ok_or("process3d-mounted.end-owner")?, kind) {
            (Process3dMountedFrame::Record { spec, mut record, field: None }, Container::Record) => {
                for field in &spec.fields {
                    if !record.fields.contains_key(&field.id){record.fields.insert(field.id,semio_framework_dsl_record::FieldValue::Absent);}
                }
                self.deliver(semio_framework_dsl_record::FieldValue::Record(record))
            }
            (Process3dMountedFrame::Sequence { tuple, items, .. }, Container::Tuple | Container::List | Container::PackedF64 | Container::PackedVarint) => {
                self.deliver(if tuple { semio_framework_dsl_record::FieldValue::Tuple(items) } else { semio_framework_dsl_record::FieldValue::List(items) })
            }
            (Process3dMountedFrame::Statements { items, keyword: None, .. }, Container::Statements) => self.deliver(semio_framework_dsl_record::FieldValue::Statements(items)),
            (Process3dMountedFrame::Machine { value, field: None, .. }, Container::Record) => {
                self.candidate()?.workshop.machines.push(value);
                Ok(())
            }
            (Process3dMountedFrame::Capability { value, field: None, .. }, Container::Record) => match self.stack.len().checked_sub(2).and_then(|index| self.stack.get_mut(index)) {
                Some(Process3dMountedFrame::Machine { value: machine, .. }) => {
                    machine.capabilities.push(value);
                    Ok(())
                }
                _ => Err("process3d-mounted.capability-parent"),
            },
            (Process3dMountedFrame::List { .. }, Container::List) | (Process3dMountedFrame::Rules { keyword: None, .. }, Container::Statements) | (Process3dMountedFrame::Workshop { field: None, .. }, Container::Record) => {
                self.clear_field();
                Ok(())
            }
            (Process3dMountedFrame::Root { field: None, .. }, Container::Record) => Ok(()),
            _ => Err("process3d-mounted.container-mismatch"),
        }
    }

    fn field_id(&mut self, id: u64) -> Result<(), &'static str> {
        let id = u16::try_from(id).map_err(|_| "process3d-mounted.field-id")?;
        match self.stack.last_mut() {
            Some(
                Process3dMountedFrame::Root { spec, field }
                | Process3dMountedFrame::Workshop { spec, field }
                | Process3dMountedFrame::Machine { spec, field, .. }
                | Process3dMountedFrame::Capability { spec, field, .. }
                | Process3dMountedFrame::Record { spec, field, .. },
            ) if field.is_none() && spec.fields.iter().any(|candidate| candidate.id == id) => {
                *field = Some(id);
                Ok(())
            }
            _ => Err("process3d-mounted.field-owner"),
        }
    }

    fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("process3d-mounted.string-overlap");
        }
        self.string = Some(Process3dMountedString { value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    fn finish_string(&mut self) -> Result<(), &'static str> {
        let owner = self.string.take().ok_or("process3d-mounted.string-handoff")?;
        self.deliver(semio_framework_dsl_record::FieldValue::Text(owner.value))
    }
}

impl mounted::RetainedTypedPackOwner for Process3dMountedSnapshotOwner {
    type Value = Process3dSnapshot;

    fn accept(&mut self, token: mounted::RetainedValueToken, catalog: &mounted::RetainedPackCatalogCursor) -> Result<(), &'static str> {
        use mounted::{RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Begin { kind, count } => self.begin(kind, count),
            Token::End(kind) => self.end(kind),
            Token::Unsigned { role: Role::FieldId, value } => self.field_id(value),
            Token::Tag { value: 0x06 | 0x07, .. } => self.begin_string(),
            Token::Unsigned { role: Role::Symbol, value } => {
                if self.string.is_none() {
                    self.begin_string()?;
                }
                let chars = catalog.symbol_chars(value).map_err(|_| "process3d-mounted.symref")?;
                let owner = self.string.as_mut().expect("process3d mounted string retained");
                owner.value.try_reserve_exact(chars).map_err(|_| "process3d-mounted.symbol-preflight")?;
                owner.symbol = Some((value, 0, chars));
                if chars == 0 {
                    self.finish_string()?;
                }
                Ok(())
            }
            Token::Unsigned { role: Role::StringLength, value } => {
                let owner = self.string.as_mut().ok_or("process3d-mounted.string-length-owner")?;
                owner.value.try_reserve_exact(usize::try_from(value).map_err(|_| "process3d-mounted.string-length")?).map_err(|_| "process3d-mounted.string-preflight")?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
                Ok(())
            }
            Token::StringChar(value) => {
                let owner = self.string.as_mut().ok_or("process3d-mounted.string-char-owner")?;
                owner.value.push(value);
                let remaining = owner.remaining.as_mut().ok_or("process3d-mounted.string-char-length")?;
                *remaining = remaining.checked_sub(value.len_utf8() as u64).ok_or("process3d-mounted.string-width")?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
                Ok(())
            }
            Token::Tag { value: 0x00, .. } => self.deliver(semio_framework_dsl_record::FieldValue::Absent),
            Token::Tag { value: 0x01, .. } => self.deliver(semio_framework_dsl_record::FieldValue::Bool(false)),
            Token::Tag { value: 0x02, .. } => self.deliver(semio_framework_dsl_record::FieldValue::Bool(true)),
            Token::Signed(value) => self.deliver(semio_framework_dsl_record::FieldValue::Int(value)),
            Token::Unsigned { role: Role::Unsigned, value } => self.deliver(semio_framework_dsl_record::FieldValue::UInt(value)),
            Token::Unsigned { role: Role::Enum, value } => self.deliver(semio_framework_dsl_record::FieldValue::Enum(u32::try_from(value).map_err(|_| "process3d-mounted.enum")?)),
            Token::F64(value) => self.deliver(semio_framework_dsl_record::FieldValue::Float(f64::from_bits(value))),
            Token::Complete { .. } => {
                if !self.stack.is_empty() || self.string.is_some() {
                    return Err("process3d-mounted.typed-terminal-populated");
                }
                self.complete = true;
                Ok(())
            }
            Token::Tag { .. } => Ok(()),
            _ => Err("process3d-mounted.token-shape"),
        }
    }

    fn grant_symbol(&mut self, catalog: &mounted::RetainedPackCatalogCursor) -> Result<bool, &'static str> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, chars)) = owner.symbol else { return Ok(false) };
        owner.value.push(catalog.symbol_char(symbol, index).map_err(|_| "process3d-mounted.symref-char")?.ok_or("process3d-mounted.symref-short")?);
        if index + 1 == chars {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("process3d mounted symbol retained").symbol = Some((symbol, index + 1, chars));
        }
        Ok(true)
    }

    fn take(&mut self) -> Option<Process3dSnapshot> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.candidate.take()
    }

    fn close_step(&mut self) -> bool {
        if self.string.take().is_some() {
            return false;
        }
        if let Some(frame) = self.stack.pop() {
            match frame {
                Process3dMountedFrame::Machine { value, .. } => {
                    if let Some(candidate) = self.candidate.as_mut() {
                        candidate.workshop.machines.push(value);
                    }
                }
                Process3dMountedFrame::Capability { value, .. } => {
                    if let Some(Process3dMountedFrame::Machine { value: machine, .. }) = self.stack.len().checked_sub(2).and_then(|index| self.stack.get_mut(index)) {
                        machine.capabilities.push(value);
                    }
                }
                frame => drop(frame),
            }
            return false;
        }
        if let Some(retirement) = self.retirement.as_mut() {
            if matches!(retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), Ok(store::SnapshotRetirementStep::Complete)) && retirement.terminal_is_empty() {
                drop(self.retirement.take());
            }
            return false;
        }
        if let Some(candidate) = self.candidate.take() {
            *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&crate::standards::v1::subsets::any::io::binary::mutations::Process3dSnapshotRetirementFactory, candidate));
            return false;
        }
        self.handed_back = true;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.candidate.is_none() && self.retirement.is_none() && self.stack.is_empty() && self.string.is_none()
    }
}

impl Drop for Process3dMountedSnapshotOwner {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || mounted::RetainedTypedPackOwner::terminal_is_empty(self), "Process3d mounted typed snapshot owner reached Drop before handoff or terminal-empty close");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-mounted-laws/🦀️.rs"]
mod retained_mounted_laws;
//#endregion 🔖️MountedTypedSnapshotOwner

pub fn process3d_mounted_pack_session(expected_bytes: usize, maximum_items: usize) -> Result<Process3dMountedPackSession, &'static str> {
    let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Process3dSnapshot as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|_| "process3d-mounted.envelope")?;
    Process3dMountedPackSession::new(store::semio_format::wrap_binary(&envelope, &[]), expected_bytes, maximum_items, PROCESS3D_MOUNTED_DEPTH, Process3dMountedSnapshotOwner::new)
}
}
pub use mounted_snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::standards::v1::subsets::any::schema::*;
use crate::{Capability, CapabilityParameter, CapabilityRule, MachineCatalog, MeasureRecipe, Pose, ProcessStep, Stock, StockQuantity, Workshop, WorkshopMachine};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use store::ArtifactDsl;
use crate::standards::v1::subsets::any::io::text::snapshot::{PROCESS_3D_CONCRETE_FOREST_EXAMPLE_TEXT as CONCRETE_FOREST_EXAMPLE_DSL, PROCESS_3D_PLATE_EXAMPLE_TEXT as PLATE_EXAMPLE_DSL, PROCESS_3D_TIMBER_EXAMPLE_TEXT as TIMBER_EXAMPLE_DSL};

/// 🧩️ Shared capability-parameter/rule builders for every built-in domain catalog below — pulled out
/// of the four (formerly per-material-file) private copies so the identical helper exists exactly once.
pub(crate) fn parameter(id: &str, label: &str, value: f64) -> CapabilityParameter {
    CapabilityParameter { id: id.into(), label: label.into(), value }
}
}
pub use snapshot_wire_codec::*;
