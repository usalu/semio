//! 🧬️ Forms artifact — document mutation dispatch enum.

use crate::artifacts::forms::schema::diff::text::diff_from_mutation;
use crate::artifacts::forms::schema::diff::FormsDiff;
use crate::artifacts::forms::{FormQuestion, FormsSnapshot, FormStep};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️FormMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum FormMutation {
    AddStep {
        #[dsl(block)]
        step: FormStep,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveStep {
        step_id: String,
    },
    MoveStep {
        step_id: String,
        index: usize,
    },
    AddBlock {
        step_id: String,
        #[dsl(block)]
        block: FormQuestion,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveBlock {
        step_id: String,
        block_id: String,
    },
    MoveBlock {
        block_id: String,
        from_step_id: String,
        to_step_id: String,
        index: usize,
    },
    UpdateBlock {
        step_id: String,
        #[dsl(block)]
        block: FormQuestion,
    },
    UpdateStep {
        #[dsl(block)]
        step: FormStep,
    },
    UpdatePlaybook {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
}

pub fn apply_form_edit_mutation(spec: &FormsSnapshot, mutation: &FormMutation) -> FormsSnapshot {
    let mut next = spec.clone();
    match mutation {
        FormMutation::AddStep { step, index } => {
            let at = index.unwrap_or(next.steps.len());
            next.steps.insert(at.min(next.steps.len()), step.clone());
        }
        FormMutation::RemoveStep { step_id } => {
            next.steps.retain(|step| step.id != *step_id);
        }
        FormMutation::MoveStep { step_id, index } => {
            if let Some(from) = next.steps.iter().position(|step| step.id == *step_id) {
                let step = next.steps.remove(from);
                let at = (*index).min(next.steps.len());
                next.steps.insert(at, step);
            }
        }
        FormMutation::AddBlock { step_id, block, index } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    let at = index.unwrap_or(step.blocks.len());
                    step.blocks.insert(at.min(step.blocks.len()), block.clone());
                }
            }
        }
        FormMutation::RemoveBlock { step_id, block_id } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    step.blocks.retain(|block| block.id != *block_id);
                }
            }
        }
        FormMutation::MoveBlock { block_id, from_step_id, to_step_id, index } => {
            let mut moving = None;
            for step in &mut next.steps {
                if step.id == *from_step_id {
                    if let Some(pos) = step.blocks.iter().position(|b| b.id == *block_id) {
                        moving = Some(step.blocks.remove(pos));
                    }
                }
            }
            if let Some(block) = moving {
                for step in &mut next.steps {
                    if step.id == *to_step_id {
                        let at = (*index).min(step.blocks.len());
                        step.blocks.insert(at, block.clone());
                    }
                }
            }
        }
        FormMutation::UpdateBlock { step_id, block } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    for entry in &mut step.blocks {
                        if entry.id == block.id {
                            *entry = block.clone();
                        }
                    }
                }
            }
        }
        FormMutation::UpdateStep { step } => {
            for entry in &mut next.steps {
                if entry.id == step.id {
                    *entry = step.clone();
                }
            }
        }
        FormMutation::UpdatePlaybook { title } => {
            next.title = title.clone();
        }
    }
    next
}

pub fn inverse_form_mutation(spec: &FormsSnapshot, mutation: &FormMutation) -> Vec<FormMutation> {
    <FormMutation as Mutation<FormsSnapshot>>::inverse(mutation, spec)
}

impl Mutation<FormsSnapshot> for FormMutation {
    type Diff = FormsDiff;

    fn diff(&self, base: &FormsSnapshot) -> FormsDiff {
        diff_from_mutation(base, self)
    }

    fn inverse(&self, snapshot: &FormsSnapshot) -> Vec<Self> {
        match self {
            FormMutation::AddStep { step, .. } => vec![FormMutation::RemoveStep { step_id: step.id.clone() }],
            FormMutation::RemoveStep { step_id } => snapshot
                .steps
                .iter()
                .find(|s| s.id == *step_id)
                .map(|step| vec![FormMutation::AddStep { step: step.clone(), index: None }])
                .unwrap_or_default(),
            FormMutation::MoveStep { step_id, .. } => snapshot
                .steps
                .iter()
                .position(|s| s.id == *step_id)
                .map(|index| vec![FormMutation::MoveStep { step_id: step_id.clone(), index }])
                .unwrap_or_default(),
            FormMutation::AddBlock { step_id, block, index: _ } => {
                vec![FormMutation::RemoveBlock { step_id: step_id.clone(), block_id: block.id.clone() }]
            }
            FormMutation::RemoveBlock { step_id, block_id } => {
                for step in &snapshot.steps {
                    if step.id == *step_id {
                        if let Some(block) = step.blocks.iter().find(|b| b.id == *block_id) {
                            return vec![FormMutation::AddBlock { step_id: step_id.clone(), block: block.clone(), index: None }];
                        }
                    }
                }
                Vec::new()
            }
            FormMutation::MoveBlock { block_id, from_step_id, to_step_id, index } => {
                for step in &snapshot.steps {
                    if step.id == *from_step_id {
                        if let Some(pos) = step.blocks.iter().position(|b| b.id == *block_id) {
                            return vec![FormMutation::MoveBlock {
                                block_id: block_id.clone(),
                                from_step_id: to_step_id.clone(),
                                to_step_id: from_step_id.clone(),
                                index: pos,
                            }];
                        }
                    }
                }
                let _ = index;
                Vec::new()
            }
            FormMutation::UpdateBlock { step_id, block } => {
                for step in &snapshot.steps {
                    if step.id == *step_id {
                        if let Some(prev) = step.blocks.iter().find(|b| b.id == block.id) {
                            return vec![FormMutation::UpdateBlock { step_id: step_id.clone(), block: prev.clone() }];
                        }
                    }
                }
                Vec::new()
            }
            FormMutation::UpdateStep { step } => snapshot
                .steps
                .iter()
                .find(|s| s.id == step.id)
                .map(|prev| vec![FormMutation::UpdateStep { step: prev.clone() }])
                .unwrap_or_default(),
            FormMutation::UpdatePlaybook { .. } => vec![FormMutation::UpdatePlaybook { title: snapshot.title.clone() }],
        }
    }
}

impl protocol::OpText for FormMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for FormMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️FormMutation

//#region 🔖️PlaybookBridge
impl From<FormsSnapshot> for flow::playbook::PlaybookSpec {
    fn from(snapshot: FormsSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            version: snapshot.version,
            title: snapshot.title,
            steps: snapshot.steps,
        }
    }
}

impl From<flow::playbook::PlaybookSpec> for FormsSnapshot {
    fn from(spec: flow::playbook::PlaybookSpec) -> Self {
        Self {
            schema: spec.schema,
            id: spec.id,
            version: spec.version,
            title: spec.title,
            steps: spec.steps,
        }
    }
}

/// 🌉️ Playbook kernel helpers still typed on `PlaybookSpec`.
pub fn as_playbook_spec(snapshot: &FormsSnapshot) -> flow::playbook::PlaybookSpec {
    snapshot.clone().into()
}
//#endregion 🔖️PlaybookBridge
