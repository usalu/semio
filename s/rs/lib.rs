//! 🖥️ S studio CQRS — programs, app instances, media graph on `vcs`.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use vcs::{create_document_vcs_envelope, materialize_document_projection, DocumentBackboneRef, DocumentVcs, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff, VcsError};

pub const S_STUDIO_SCHEMA: &str = "s.studio";
pub const S_MEDIA_GRAPH_SCHEMA: &str = "s.media-graph";

//#region 🔖Schemas
/// @emoji 🔗 Handle to an app instance's own vcs document — app content is never embedded on the
/// studio document, only referenced (mirrors os-core's `OsDocumentRef`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SDocumentRef {
    pub document_id: String,
    pub schema: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SAppInstance {
    pub id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub yields: String,
    pub document: SDocumentRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraphPort {
    pub id: String,
    pub resource_kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraphNode {
    pub id: String,
    pub instance_id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub inputs: Vec<SMediaGraphPort>,
    pub outputs: Vec<SMediaGraphPort>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraphEdge {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SMediaGraph {
    pub schema: String,
    pub nodes: Vec<SMediaGraphNode>,
    pub edges: Vec<SMediaGraphEdge>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SStudioProjection {
    pub programs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    pub app_instances: Vec<SAppInstance>,
    pub media_graph: SMediaGraph,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaGraphPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum StudioOperation {
    SetActiveProgram {
        #[serde(skip_serializing_if = "Option::is_none")]
        program_id: Option<String>,
    },
    SetActiveAlternative {
        #[serde(skip_serializing_if = "Option::is_none")]
        alternative_id: Option<String>,
    },
    SpawnAppInstance {
        instance: SAppInstance,
        position: MediaGraphPosition,
    },
    RemoveAppInstance {
        instance_id: String,
    },
    ConnectMediaPorts {
        edge: SMediaGraphEdge,
    },
    DisconnectMediaEdge {
        edge_id: String,
    },
    MoveMediaNode {
        node_id: String,
        x: f64,
        y: f64,
    },
}

pub type SStudioVcs = DocumentVcs<SStudioProjection, StudioOperation>;
pub type SStudioEnvelope = DocumentVcsEnvelope<SStudioProjection, StudioOperation>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SStudioDocument {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub vcs: SStudioVcs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<DocumentBackboneRef>,
}
//#endregion 🔖Schemas

//#region 🔖Projection
static S_ID: AtomicU64 = AtomicU64::new(0);

pub fn create_s_id(prefix: &str) -> String {
    let n = S_ID.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

pub fn empty_media_graph() -> SMediaGraph {
    SMediaGraph { schema: S_MEDIA_GRAPH_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

pub fn default_studio_projection() -> SStudioProjection {
    SStudioProjection { programs: Vec::new(), active_program_id: None, active_alternative_id: None, app_instances: Vec::new(), media_graph: empty_media_graph() }
}

pub fn create_empty_studio_document(id: &str, name: &str) -> SStudioDocument {
    SStudioDocument { schema: S_STUDIO_SCHEMA.into(), id: id.into(), name: name.into(), vcs: create_document_vcs_envelope(S_STUDIO_SCHEMA, id, default_studio_projection(), None).vcs, backbone: None }
}

pub fn apply_studio_operation(projection: &SStudioProjection, operation: &StudioOperation) -> SStudioProjection {
    let mut next = projection.clone();
    match operation {
        StudioOperation::SetActiveProgram { program_id } => {
            next.active_program_id = program_id.clone();
        }
        StudioOperation::SetActiveAlternative { alternative_id } => {
            next.active_alternative_id = alternative_id.clone();
        }
        StudioOperation::SpawnAppInstance { instance, position } => {
            if !next.programs.contains(&instance.program_id) {
                next.programs.push(instance.program_id.clone());
            }
            let node = SMediaGraphNode { id: create_s_id("node"), instance_id: instance.id.clone(), label: instance.label.clone(), x: position.x, y: position.y, inputs: Vec::new(), outputs: Vec::new() };
            next.media_graph.nodes.push(node);
            next.app_instances.push(instance.clone());
        }
        StudioOperation::RemoveAppInstance { instance_id } => {
            let node_id = next.media_graph.nodes.iter().find(|node| node.instance_id == *instance_id).map(|node| node.id.clone());
            next.app_instances.retain(|instance| instance.id != *instance_id);
            next.media_graph.nodes.retain(|node| node.instance_id != *instance_id);
            if let Some(node_id) = node_id {
                next.media_graph.edges.retain(|edge| edge.source_node_id != node_id && edge.target_node_id != node_id);
            }
        }
        StudioOperation::ConnectMediaPorts { edge } => {
            next.media_graph.edges.push(edge.clone());
        }
        StudioOperation::DisconnectMediaEdge { edge_id } => {
            next.media_graph.edges.retain(|edge| edge.id != *edge_id);
        }
        StudioOperation::MoveMediaNode { node_id, x, y } => {
            for node in &mut next.media_graph.nodes {
                if node.id == *node_id {
                    node.x = *x;
                    node.y = *y;
                }
            }
        }
    }
    next
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum StudioDiff {
    #[default]
    Empty,
    SetActiveProgram {
        #[serde(skip_serializing_if = "Option::is_none")]
        program_id: Option<String>,
    },
    SetActiveAlternative {
        #[serde(skip_serializing_if = "Option::is_none")]
        alternative_id: Option<String>,
    },
    SpawnAppInstance {
        instance: SAppInstance,
        position: MediaGraphPosition,
    },
    RemoveAppInstance {
        instance_id: String,
    },
    ConnectMediaPorts {
        edge: SMediaGraphEdge,
    },
    DisconnectMediaEdge {
        edge_id: String,
    },
    MoveMediaNode {
        node_id: String,
        x: f64,
        y: f64,
    },
}

impl OperationDiff<SStudioProjection> for StudioDiff {
    fn apply(&self, projection: &SStudioProjection) -> SStudioProjection {
        let operation = match self {
            StudioDiff::Empty => return projection.clone(),
            StudioDiff::SetActiveProgram { program_id } => StudioOperation::SetActiveProgram { program_id: program_id.clone() },
            StudioDiff::SetActiveAlternative { alternative_id } => StudioOperation::SetActiveAlternative { alternative_id: alternative_id.clone() },
            StudioDiff::SpawnAppInstance { instance, position } => StudioOperation::SpawnAppInstance { instance: instance.clone(), position: position.clone() },
            StudioDiff::RemoveAppInstance { instance_id } => StudioOperation::RemoveAppInstance { instance_id: instance_id.clone() },
            StudioDiff::ConnectMediaPorts { edge } => StudioOperation::ConnectMediaPorts { edge: edge.clone() },
            StudioDiff::DisconnectMediaEdge { edge_id } => StudioOperation::DisconnectMediaEdge { edge_id: edge_id.clone() },
            StudioDiff::MoveMediaNode { node_id, x, y } => StudioOperation::MoveMediaNode { node_id: node_id.clone(), x: *x, y: *y },
        };
        apply_studio_operation(projection, &operation)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, StudioDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<SStudioProjection> for StudioOperation {
    type Diff = StudioDiff;

    fn diff(&self, _projection: &SStudioProjection) -> StudioDiff {
        match self {
            StudioOperation::SetActiveProgram { program_id } => StudioDiff::SetActiveProgram { program_id: program_id.clone() },
            StudioOperation::SetActiveAlternative { alternative_id } => StudioDiff::SetActiveAlternative { alternative_id: alternative_id.clone() },
            StudioOperation::SpawnAppInstance { instance, position } => StudioDiff::SpawnAppInstance { instance: instance.clone(), position: position.clone() },
            StudioOperation::RemoveAppInstance { instance_id } => StudioDiff::RemoveAppInstance { instance_id: instance_id.clone() },
            StudioOperation::ConnectMediaPorts { edge } => StudioDiff::ConnectMediaPorts { edge: edge.clone() },
            StudioOperation::DisconnectMediaEdge { edge_id } => StudioDiff::DisconnectMediaEdge { edge_id: edge_id.clone() },
            StudioOperation::MoveMediaNode { node_id, x, y } => StudioDiff::MoveMediaNode { node_id: node_id.clone(), x: *x, y: *y },
        }
    }

    fn backwards(&self, projection: &SStudioProjection) -> Vec<Self> {
        match self {
            StudioOperation::SetActiveProgram { .. } => vec![StudioOperation::SetActiveProgram { program_id: projection.active_program_id.clone() }],
            StudioOperation::SetActiveAlternative { .. } => vec![StudioOperation::SetActiveAlternative { alternative_id: projection.active_alternative_id.clone() }],
            StudioOperation::SpawnAppInstance { instance, .. } => vec![StudioOperation::RemoveAppInstance { instance_id: instance.id.clone() }],
            StudioOperation::RemoveAppInstance { instance_id } => projection
                .app_instances
                .iter()
                .find(|i| i.id == *instance_id)
                .map(|instance| {
                    let node = projection.media_graph.nodes.iter().find(|n| n.instance_id == *instance_id);
                    vec![StudioOperation::SpawnAppInstance { instance: instance.clone(), position: MediaGraphPosition { x: node.map(|n| n.x).unwrap_or(0.0), y: node.map(|n| n.y).unwrap_or(0.0) } }]
                })
                .unwrap_or_default(),
            StudioOperation::ConnectMediaPorts { edge } => vec![StudioOperation::DisconnectMediaEdge { edge_id: edge.id.clone() }],
            StudioOperation::DisconnectMediaEdge { edge_id } => projection.media_graph.edges.iter().find(|e| e.id == *edge_id).map(|edge| vec![StudioOperation::ConnectMediaPorts { edge: edge.clone() }]).unwrap_or_default(),
            StudioOperation::MoveMediaNode { node_id, .. } => projection.media_graph.nodes.iter().find(|n| n.id == *node_id).map(|node| vec![StudioOperation::MoveMediaNode { node_id: node_id.clone(), x: node.x, y: node.y }]).unwrap_or_default(),
        }
    }
}

pub fn materialize_studio_projection(document: &SStudioDocument, applied_edit_ids: &[String]) -> Result<SStudioProjection, VcsError> {
    let envelope = SStudioEnvelope { schema: document.schema.clone(), id: document.id.clone(), vcs: document.vcs.clone(), backbone: document.backbone.clone(), active_alternative_id: None };
    materialize_document_projection(&envelope, applied_edit_ids)
}
//#endregion 🔖Projection

//#region 🔖Dsl
// 📜 Handcrafted textual DSL for `SStudioProjection` (`vcs::DocumentDsl`, extension `.sstudio`) and
// one-line op-text for `StudioOperation` (`vcs::OpText`, see `🔖OpText`) — replaces any future JSON
// fixture format. Grammar mirrors `vcs`'s own `@marker key=value ... "trailing text"` structural-line
// convention (see `writer`'s `writer_dsl` module for the sibling pattern); hand-rolled locally since
// `vcs`'s escaping/kv-line helpers are private to that crate. Ports reference their owning node by
// `node=<id>` so `@port` lines never need to trail their `@node` line positionally.
mod studio_dsl {
    use super::{MediaGraphPosition, SAppInstance, SDocumentRef, SMediaGraph, SMediaGraphEdge, SMediaGraphNode, SMediaGraphPort, SStudioProjection, StudioOperation};
    use std::collections::HashMap;
    use vcs::{TextError, TextSpan};

    //#region Lexer
    /// 🔐 Escapes `\`, `"` and newlines so arbitrary label text fits inside one quoted field.
    fn escape_text(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                _ => out.push(ch),
            }
        }
        out
    }

    fn unescape_text(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        let mut chars = value.chars();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => out.push('\n'),
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(ch);
            }
        }
        out
    }

    /// 🔎 Finds the char index of the unescaped opening `"` of a trailing quoted field, mirroring
    /// `vcs`'s private `find_unescaped_trailing_quote` (kept in lock-step, see that doc comment).
    fn find_unescaped_trailing_quote(chars: &[char]) -> Option<usize> {
        if chars.is_empty() || *chars.last().unwrap() != '"' {
            return None;
        }
        let last = chars.len() - 1;
        let mut i = last;
        while i > 0 {
            i -= 1;
            if chars[i] == '"' {
                let mut backslashes = 0;
                let mut j = i;
                while j > 0 && chars[j - 1] == '\\' {
                    backslashes += 1;
                    j -= 1;
                }
                if backslashes % 2 == 0 {
                    return Some(i);
                }
            }
        }
        None
    }

    /// 🧾 One parsed `@marker key=value ...` line plus its optional trailing quoted text field.
    struct KvLine {
        marker: String,
        fields: HashMap<String, String>,
        text: Option<String>,
    }

    fn parse_kv_line(line: &str, line_no: u32) -> Result<KvLine, TextError> {
        let trimmed = line.trim_end();
        let chars: Vec<char> = trimmed.chars().collect();
        let (head, text) = match find_unescaped_trailing_quote(&chars) {
            Some(open) => {
                let content: String = chars[open + 1..chars.len() - 1].iter().collect();
                let head: String = chars[..open].iter().collect();
                (head.trim_end().to_string(), Some(unescape_text(&content)))
            }
            None => (trimmed.to_string(), None),
        };
        let mut tokens = head.split_whitespace();
        let marker = tokens
            .next()
            .ok_or_else(|| TextError::new("expected a marker or operation name", TextSpan::at(line_no, 1)))?
            .to_string();
        let mut fields = HashMap::new();
        for token in tokens {
            let (key, value) = token
                .split_once('=')
                .ok_or_else(|| TextError::new(format!("expected key=value token, got '{token}'"), TextSpan::at(line_no, 1)))?;
            fields.insert(key.to_string(), value.to_string());
        }
        Ok(KvLine { marker, fields, text })
    }

    fn field<'a>(fields: &'a HashMap<String, String>, key: &str, line_no: u32) -> Result<&'a str, TextError> {
        fields
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| TextError::new(format!("missing field '{key}'"), TextSpan::at(line_no, 1)))
    }

    fn optional_id(value: &str) -> Option<String> {
        if value == "-" {
            None
        } else {
            Some(value.to_string())
        }
    }

    fn print_optional_id(value: &Option<String>) -> String {
        value.clone().unwrap_or_else(|| "-".to_string())
    }

    fn parse_f64(value: &str, key: &str, line_no: u32) -> Result<f64, TextError> {
        value
            .parse::<f64>()
            .map_err(|_| TextError::new(format!("expected number for '{key}', got '{value}'"), TextSpan::at(line_no, 1)))
    }

    /// 🔢 Prints an `f64` via Rust's shortest round-trippable `Display` form (`"0"`, not `"0.0"`).
    fn fmt_num(value: f64) -> String {
        value.to_string()
    }

    fn split_programs(value: &str) -> Vec<String> {
        if value == "-" {
            Vec::new()
        } else {
            value.split(',').map(|entry| entry.to_string()).collect()
        }
    }

    fn join_programs(programs: &[String]) -> String {
        if programs.is_empty() {
            "-".to_string()
        } else {
            programs.join(",")
        }
    }
    //#endregion Lexer

    //#region Document
    /// 📥 Parses a full `.sstudio` document: an `@studio` header (programs/active program/active
    /// alternative), an `@graph` header (media graph schema), then any number of `@instance`/`@node`/
    /// `@port`/`@edge` records in any order — `@port` lines resolve their owning node by `node=<id>`
    /// rather than relying on file position.
    pub fn parse_document(source: &str) -> Result<SStudioProjection, TextError> {
        let mut programs: Vec<String> = Vec::new();
        let mut active_program_id: Option<String> = None;
        let mut active_alternative_id: Option<String> = None;
        let mut graph_schema = String::new();
        let mut app_instances: Vec<SAppInstance> = Vec::new();
        let mut nodes: Vec<SMediaGraphNode> = Vec::new();
        let mut edges: Vec<SMediaGraphEdge> = Vec::new();
        let mut seen_studio_header = false;

        for (index, raw_line) in source.lines().enumerate() {
            let line_no = index as u32 + 1;
            let trimmed = raw_line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let parsed = parse_kv_line(raw_line, line_no)?;
            match parsed.marker.as_str() {
                "@studio" => {
                    seen_studio_header = true;
                    programs = split_programs(field(&parsed.fields, "programs", line_no)?);
                    active_program_id = optional_id(field(&parsed.fields, "active", line_no)?);
                    active_alternative_id = optional_id(field(&parsed.fields, "activealt", line_no)?);
                }
                "@graph" => {
                    graph_schema = field(&parsed.fields, "schema", line_no)?.to_string();
                }
                "@instance" => {
                    let id = field(&parsed.fields, "id", line_no)?.to_string();
                    let program_id = field(&parsed.fields, "program", line_no)?.to_string();
                    let app_id = field(&parsed.fields, "app", line_no)?.to_string();
                    let yields = field(&parsed.fields, "yields", line_no)?.to_string();
                    let document_id = field(&parsed.fields, "docid", line_no)?.to_string();
                    let schema = field(&parsed.fields, "docschema", line_no)?.to_string();
                    let label = parsed.text.clone().unwrap_or_default();
                    app_instances.push(SAppInstance { id, program_id, app_id, label, yields, document: SDocumentRef { document_id, schema } });
                }
                "@node" => {
                    let id = field(&parsed.fields, "id", line_no)?.to_string();
                    let instance_id = field(&parsed.fields, "instance", line_no)?.to_string();
                    let x = parse_f64(field(&parsed.fields, "x", line_no)?, "x", line_no)?;
                    let y = parse_f64(field(&parsed.fields, "y", line_no)?, "y", line_no)?;
                    let label = parsed.text.clone().unwrap_or_default();
                    nodes.push(SMediaGraphNode { id, instance_id, label, x, y, inputs: Vec::new(), outputs: Vec::new() });
                }
                "@port" => {
                    let node_id = field(&parsed.fields, "node", line_no)?.to_string();
                    let dir = field(&parsed.fields, "dir", line_no)?.to_string();
                    let id = field(&parsed.fields, "id", line_no)?.to_string();
                    let resource_kind = field(&parsed.fields, "kind", line_no)?.to_string();
                    let node = nodes
                        .iter_mut()
                        .find(|node| node.id == node_id)
                        .ok_or_else(|| TextError::new(format!("@port references unknown node '{node_id}'"), TextSpan::at(line_no, 1)))?;
                    let port = SMediaGraphPort { id, resource_kind };
                    match dir.as_str() {
                        "in" => node.inputs.push(port),
                        "out" => node.outputs.push(port),
                        other => return Err(TextError::expected("expected 'in' or 'out'", TextSpan::at(line_no, 1), other.to_string())),
                    }
                }
                "@edge" => {
                    let id = field(&parsed.fields, "id", line_no)?.to_string();
                    let source_node_id = field(&parsed.fields, "srcnode", line_no)?.to_string();
                    let source_port_id = field(&parsed.fields, "srcport", line_no)?.to_string();
                    let target_node_id = field(&parsed.fields, "tgtnode", line_no)?.to_string();
                    let target_port_id = field(&parsed.fields, "tgtport", line_no)?.to_string();
                    edges.push(SMediaGraphEdge { id, source_node_id, source_port_id, target_node_id, target_port_id });
                }
                other => return Err(TextError::new(format!("unknown structural line kind '{other}'"), TextSpan::at(line_no, 1))),
            }
        }
        if !seen_studio_header {
            return Err(TextError::new("expected a leading '@studio' header line", TextSpan::at(1, 1)));
        }
        Ok(SStudioProjection {
            programs,
            active_program_id,
            active_alternative_id,
            app_instances,
            media_graph: SMediaGraph { schema: graph_schema, nodes, edges },
        })
    }

    /// 📤 Prints an `SStudioProjection` back to its `.sstudio` DSL form (see {@link parse_document}).
    pub fn print_document(projection: &SStudioProjection) -> String {
        let mut out = format!(
            "@studio programs={} active={} activealt={}",
            join_programs(&projection.programs),
            print_optional_id(&projection.active_program_id),
            print_optional_id(&projection.active_alternative_id),
        );
        out.push('\n');
        out.push_str(&format!("@graph schema={}", projection.media_graph.schema));
        for instance in &projection.app_instances {
            out.push('\n');
            out.push_str(&format!(
                "@instance id={} program={} app={} yields={} docid={} docschema={} \"{}\"",
                instance.id,
                instance.program_id,
                instance.app_id,
                instance.yields,
                instance.document.document_id,
                instance.document.schema,
                escape_text(&instance.label),
            ));
        }
        for node in &projection.media_graph.nodes {
            out.push('\n');
            out.push_str(&format!(
                "@node id={} instance={} x={} y={} \"{}\"",
                node.id,
                node.instance_id,
                fmt_num(node.x),
                fmt_num(node.y),
                escape_text(&node.label),
            ));
            for port in &node.inputs {
                out.push('\n');
                out.push_str(&format!("@port node={} dir=in id={} kind={}", node.id, port.id, port.resource_kind));
            }
            for port in &node.outputs {
                out.push('\n');
                out.push_str(&format!("@port node={} dir=out id={} kind={}", node.id, port.id, port.resource_kind));
            }
        }
        for edge in &projection.media_graph.edges {
            out.push('\n');
            out.push_str(&format!(
                "@edge id={} srcnode={} srcport={} tgtnode={} tgtport={}",
                edge.id, edge.source_node_id, edge.source_port_id, edge.target_node_id, edge.target_port_id,
            ));
        }
        out
    }
    //#endregion Document

    //#region Operation
    /// 📥 Parses a single one-line `StudioOperation` — one variant per marker (`setActiveProgram`,
    /// `setActiveAlternative`, `spawnAppInstance`, `removeAppInstance`, `connectMediaPorts`,
    /// `disconnectMediaEdge`, `moveMediaNode`).
    pub fn parse_operation(line: &str) -> Result<StudioOperation, TextError> {
        let parsed = parse_kv_line(line, 1)?;
        match parsed.marker.as_str() {
            "setActiveProgram" => Ok(StudioOperation::SetActiveProgram { program_id: optional_id(field(&parsed.fields, "id", 1)?) }),
            "setActiveAlternative" => Ok(StudioOperation::SetActiveAlternative { alternative_id: optional_id(field(&parsed.fields, "id", 1)?) }),
            "spawnAppInstance" => {
                let id = field(&parsed.fields, "id", 1)?.to_string();
                let program_id = field(&parsed.fields, "program", 1)?.to_string();
                let app_id = field(&parsed.fields, "app", 1)?.to_string();
                let yields = field(&parsed.fields, "yields", 1)?.to_string();
                let document_id = field(&parsed.fields, "docid", 1)?.to_string();
                let schema = field(&parsed.fields, "docschema", 1)?.to_string();
                let x = parse_f64(field(&parsed.fields, "x", 1)?, "x", 1)?;
                let y = parse_f64(field(&parsed.fields, "y", 1)?, "y", 1)?;
                let label = parsed.text.clone().unwrap_or_default();
                Ok(StudioOperation::SpawnAppInstance {
                    instance: SAppInstance { id, program_id, app_id, label, yields, document: SDocumentRef { document_id, schema } },
                    position: MediaGraphPosition { x, y },
                })
            }
            "removeAppInstance" => Ok(StudioOperation::RemoveAppInstance { instance_id: field(&parsed.fields, "id", 1)?.to_string() }),
            "connectMediaPorts" => Ok(StudioOperation::ConnectMediaPorts {
                edge: SMediaGraphEdge {
                    id: field(&parsed.fields, "id", 1)?.to_string(),
                    source_node_id: field(&parsed.fields, "srcnode", 1)?.to_string(),
                    source_port_id: field(&parsed.fields, "srcport", 1)?.to_string(),
                    target_node_id: field(&parsed.fields, "tgtnode", 1)?.to_string(),
                    target_port_id: field(&parsed.fields, "tgtport", 1)?.to_string(),
                },
            }),
            "disconnectMediaEdge" => Ok(StudioOperation::DisconnectMediaEdge { edge_id: field(&parsed.fields, "id", 1)?.to_string() }),
            "moveMediaNode" => Ok(StudioOperation::MoveMediaNode {
                node_id: field(&parsed.fields, "id", 1)?.to_string(),
                x: parse_f64(field(&parsed.fields, "x", 1)?, "x", 1)?,
                y: parse_f64(field(&parsed.fields, "y", 1)?, "y", 1)?,
            }),
            other => Err(TextError::expected(
                format!("unknown studio operation '{other}'"),
                TextSpan::at(1, 1),
                "setActiveProgram | setActiveAlternative | spawnAppInstance | removeAppInstance | connectMediaPorts | disconnectMediaEdge | moveMediaNode",
            )),
        }
    }

    /// 📤 Prints a `StudioOperation` back to its one-line op text (see {@link parse_operation}).
    pub fn print_operation(operation: &StudioOperation) -> String {
        match operation {
            StudioOperation::SetActiveProgram { program_id } => format!("setActiveProgram id={}", print_optional_id(program_id)),
            StudioOperation::SetActiveAlternative { alternative_id } => format!("setActiveAlternative id={}", print_optional_id(alternative_id)),
            StudioOperation::SpawnAppInstance { instance, position } => format!(
                "spawnAppInstance id={} program={} app={} yields={} docid={} docschema={} x={} y={} \"{}\"",
                instance.id,
                instance.program_id,
                instance.app_id,
                instance.yields,
                instance.document.document_id,
                instance.document.schema,
                fmt_num(position.x),
                fmt_num(position.y),
                escape_text(&instance.label),
            ),
            StudioOperation::RemoveAppInstance { instance_id } => format!("removeAppInstance id={instance_id}"),
            StudioOperation::ConnectMediaPorts { edge } => format!(
                "connectMediaPorts id={} srcnode={} srcport={} tgtnode={} tgtport={}",
                edge.id, edge.source_node_id, edge.source_port_id, edge.target_node_id, edge.target_port_id,
            ),
            StudioOperation::DisconnectMediaEdge { edge_id } => format!("disconnectMediaEdge id={edge_id}"),
            StudioOperation::MoveMediaNode { node_id, x, y } => format!("moveMediaNode id={node_id} x={} y={}", fmt_num(*x), fmt_num(*y)),
        }
    }
    //#endregion Operation
}

impl vcs::DocumentDsl for SStudioProjection {
    const EXTENSION: &'static str = "sstudio";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        studio_dsl::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        studio_dsl::print_document(self)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
impl vcs::OpText for StudioOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        studio_dsl::parse_operation(line)
    }

    fn print_op(&self) -> String {
        studio_dsl::print_operation(self)
    }
}
//#endregion 🔖OpText

//#region 🔖StudioStore
pub struct StudioStore {
    inner: DocumentVcsStore<SStudioProjection, StudioOperation>,
    name: String,
}

impl StudioStore {
    pub fn new(document: SStudioDocument) -> Self {
        let envelope = SStudioEnvelope { schema: document.schema, id: document.id, vcs: document.vcs, backbone: document.backbone, active_alternative_id: None };
        Self { inner: DocumentVcsStore::new(envelope), name: document.name }
    }

    pub fn generation(&self) -> u64 {
        self.inner.generation()
    }

    pub fn projection(&self) -> Result<SStudioProjection, VcsError> {
        self.inner.projection()
    }

    pub fn document(&self) -> SStudioDocument {
        let envelope = self.inner.envelope();
        SStudioDocument { schema: envelope.schema.clone(), id: envelope.id.clone(), name: self.name.clone(), vcs: envelope.vcs.clone(), backbone: envelope.backbone.clone() }
    }

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        self.inner.dispatch_json(command_json)
    }

    pub fn dispatch_apply(&mut self, operations: Vec<StudioOperation>) -> Result<(), VcsError> {
        self.inner.dispatch(DocumentVcsCommand::Apply { operations, description: None })
    }

    /// @emoji 📡 Pumps any queued inbound backbone messages into the edit timeline.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.inner.tick()
    }

    /// @emoji 🔗 Resolves and attaches a backbone by uri inside the wasm sandbox (every scheme
    /// forwards to the host over the injected `BackboneChannelPort`, a pure queue).
    #[cfg(target_arch = "wasm32")]
    pub fn attach_backbone(&mut self, uri: &str) -> Result<(), VcsError> {
        self.inner.attach_backbone_uri(uri)
    }

    /// @emoji 🚧 Native attach is a documented no-operation: `s` only runs as a WASM plugin in the browser
    /// today (no native caller exists), and wiring its native path onto `framework/sync`'s
    /// `DocumentHost` is `s`'s own `DocumentApp` migration (WS-F's last wave), not this compile fix.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn attach_backbone(&mut self, _uri: &str) -> Result<(), VcsError> {
        Ok(())
    }

    pub fn detach_backbone(&mut self) {
        self.inner.detach_backbone();
    }
}
//#endregion 🔖StudioStore

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
pub mod wasm_bridge {
    use super::*;
    use std::sync::Mutex;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct StudioStoreHandle {
        store: Mutex<StudioStore>,
    }

    #[wasm_bindgen]
    impl StudioStoreHandle {
        #[wasm_bindgen(constructor)]
        pub fn new(document_json: &str) -> Result<StudioStoreHandle, JsValue> {
            let document: SStudioDocument = serde_json::from_str(document_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self { store: Mutex::new(StudioStore::new(document)) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            let mut store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            store.dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            let store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            let projection = store.projection().map_err(|e| JsValue::from_str(&e.to_string()))?;
            serde_json::to_string(&projection).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> Result<u32, JsValue> {
            let store = self.store.lock().map_err(|_| JsValue::from_str("lock poisoned"))?;
            Ok(store.generation() as u32)
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawns_app_instance_through_cqrs_dispatch() {
        let mut store = StudioStore::new(create_empty_studio_document("studio", "Studio"));
        let instance = SAppInstance { id: "app-1".into(), program_id: "draw".into(), app_id: "draw".into(), label: "Draw".into(), yields: "graph.dag".into(), document: SDocumentRef { document_id: "doc-1".into(), schema: "draw.document".into() } };
        store.dispatch_apply(vec![StudioOperation::SpawnAppInstance { instance: instance.clone(), position: MediaGraphPosition { x: 0.0, y: 0.0 } }]).expect("spawn");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.app_instances.len(), 1);
        assert_eq!(projection.media_graph.nodes.len(), 1);
    }

    #[test]
    fn undo_after_spawn() {
        let mut store = StudioStore::new(create_empty_studio_document("studio", "Studio"));
        let instance = SAppInstance { id: "app-1".into(), program_id: "draw".into(), app_id: "draw".into(), label: "Draw".into(), yields: "graph.dag".into(), document: SDocumentRef { document_id: "doc-1".into(), schema: "draw.document".into() } };
        store.dispatch_apply(vec![StudioOperation::SpawnAppInstance { instance, position: MediaGraphPosition { x: 0.0, y: 0.0 } }]).expect("spawn");
        store.dispatch_json(r#"{"kind":"undo"}"#).expect("undo");
        assert_eq!(store.projection().expect("projection").app_instances.len(), 0);
    }

    //#region 🔖DslAndOpText
    fn sample_studio_projection() -> SStudioProjection {
        SStudioProjection {
            programs: vec!["draw".into(), "writer".into()],
            active_program_id: Some("draw".into()),
            active_alternative_id: None,
            app_instances: vec![SAppInstance {
                id: "app-1".into(),
                program_id: "draw".into(),
                app_id: "draw".into(),
                label: "Semio \"Emblem\"".into(),
                yields: "2d.drawing".into(),
                document: SDocumentRef { document_id: "doc-1".into(), schema: "draw.document".into() },
            }],
            media_graph: SMediaGraph {
                schema: S_MEDIA_GRAPH_SCHEMA.into(),
                nodes: vec![SMediaGraphNode {
                    id: "node-1".into(),
                    instance_id: "app-1".into(),
                    label: "Draw\nNode".into(),
                    x: 40.0,
                    y: 80.0,
                    inputs: vec![SMediaGraphPort { id: "app-1:in".into(), resource_kind: "2d.drawing".into() }],
                    outputs: vec![SMediaGraphPort { id: "app-1:out".into(), resource_kind: "2d.drawing".into() }],
                }],
                edges: vec![SMediaGraphEdge {
                    id: "edge-1".into(),
                    source_node_id: "node-1".into(),
                    source_port_id: "app-1:out".into(),
                    target_node_id: "node-1".into(),
                    target_port_id: "app-1:in".into(),
                }],
            },
        }
    }

    #[test]
    fn studio_dsl_round_trips_empty_and_sample_projections() {
        vcs::test_support::assert_dsl_round_trip(&default_studio_projection());
        vcs::test_support::assert_dsl_round_trip(&sample_studio_projection());
    }

    #[test]
    fn studio_op_text_round_trips_every_variant() {
        vcs::test_support::assert_op_line_round_trip(&StudioOperation::SetActiveProgram { program_id: Some("draw".into()) });
        vcs::test_support::assert_op_line_round_trip(&StudioOperation::SetActiveProgram { program_id: None });
        vcs::test_support::assert_op_line_round_trip(&StudioOperation::SetActiveAlternative { alternative_id: Some("alt-1".into()) });
        vcs::test_support::assert_op_line_round_trip(&StudioOperation::SetActiveAlternative { alternative_id: None });
        let instance = SAppInstance {
            id: "app-2".into(),
            program_id: "writer".into(),
            app_id: "writer".into(),
            label: "Jack \"Notes\"".into(),
            yields: "text.document".into(),
            document: SDocumentRef { document_id: "doc-2".into(), schema: "writer.document".into() },
        };
        vcs::test_support::assert_op_line_round_trip(&StudioOperation::SpawnAppInstance { instance, position: MediaGraphPosition { x: 12.0, y: 24.0 } });
        vcs::test_support::assert_op_line_round_trip(&StudioOperation::RemoveAppInstance { instance_id: "app-1".into() });
        let edge = SMediaGraphEdge { id: "edge-2".into(), source_node_id: "node-1".into(), source_port_id: "p-out".into(), target_node_id: "node-2".into(), target_port_id: "p-in".into() };
        vcs::test_support::assert_op_line_round_trip(&StudioOperation::ConnectMediaPorts { edge });
        vcs::test_support::assert_op_line_round_trip(&StudioOperation::DisconnectMediaEdge { edge_id: "edge-1".into() });
        vcs::test_support::assert_op_line_round_trip(&StudioOperation::MoveMediaNode { node_id: "node-1".into(), x: 5.0, y: 6.0 });
    }

    #[test]
    fn studio_document_text_round_trips_through_the_store() {
        let envelope = create_document_vcs_envelope::<SStudioProjection, StudioOperation>(S_STUDIO_SCHEMA, "studio", sample_studio_projection(), None);
        let store: DocumentVcsStore<SStudioProjection, StudioOperation> = DocumentVcsStore::new(envelope);
        vcs::test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DslAndOpText
}
//#endregion 🧪Tests
