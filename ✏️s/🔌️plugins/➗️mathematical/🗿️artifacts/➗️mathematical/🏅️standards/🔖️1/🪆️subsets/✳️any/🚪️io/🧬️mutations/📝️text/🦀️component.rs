//! ⚡️ Mathematical artifact — hand-rolled `OpText`/`OpBinary` for `MathematicalMutation`.
//! `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../🦀️component.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted
//! here, one keyword per semantic verb, grammar `keyword key1=value1 key2=value2 ...`.

pub use crate::artifacts::mathematical::schema::mutations::MathematicalMutation;

use crate::artifacts::mathematical::schema::mutations::{
    change_coefficient::mutation::ChangeCoefficient, change_graph_directed::mutation::ChangeGraphDirected, change_node_label::mutation::ChangeNodeLabel, connect_nodes::mutation::ConnectNodes, create_node::mutation::CreateNode,
    delete_node::mutation::DeleteNode, delete_nodes::mutation::DeleteNodes, disconnect_nodes::mutation::DisconnectNodes, insert_point::mutation::InsertPoint, move_node::mutation::MoveNode, move_point::mutation::MovePoint,
    remove_point::mutation::RemovePoint, replace_graph::mutation::ReplaceGraph, replace_points::mutation::ReplacePoints, update_graph_algorithm::mutation::UpdateGraphAlgorithm,
};
use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationNodeLabel;
use crate::artifacts::mathematical::{MathematicalGraph, MathematicalPoint};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other scalar's text form stays space-free and tokenizable by [`tokenize_args`].
async fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
async fn dec_str(s: &str) -> Result<String, String> {
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {s:?}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(out)
}
async fn enc_opt_str(s: &Option<String>) -> String {
    match s {
        Some(v) => enc_str(v),
        None => "-".to_string(),
    }
}
async fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(dec_str(s)?))
    }
}
async fn enc_f64(v: f64) -> String {
    format!("{v}")
}
async fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
async fn enc_usize(v: usize) -> String {
    v.to_string()
}
async fn dec_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
async fn enc_bool(v: bool) -> String {
    v.to_string()
}
async fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}
async fn enc_points(points: &[MathematicalPoint]) -> String {
    format!("[{}]", points.iter().map(|p| format!("{},{}", p.x, p.y)).collect::<Vec<_>>().join(";"))
}
async fn dec_points(s: &str) -> Result<Vec<MathematicalPoint>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected bracketed point list, got {s:?}"))?;
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    inner
        .split(';')
        .map(|pair| {
            let (x, y) = pair.split_once(',').ok_or_else(|| format!("bad point pair {pair:?}"))?;
            Ok(MathematicalPoint { x: dec_f64(x)?, y: dec_f64(y)? })
        })
        .collect()
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value —
/// needed because node labels/algorithm ids may contain spaces.
async fn tokenize_args(rest: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                current.push(c);
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
async fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️GraphCodec
/// 🕸️ Whole-`MathematicalGraph` text form (used by `replace-graph`) — a quoted JSON string
/// (`MathematicalGraph` already derives `Serialize`/`Deserialize`) rather than a second handcrafted
/// graph grammar; `enc_str`/`dec_str`'s backslash/quote escaping round-trips it byte-for-byte.
async fn enc_graph(graph: &MathematicalGraph) -> String {
    enc_str(&serde_json::to_string(graph).expect("MathematicalGraph always serializes"))
}
async fn dec_graph(s: &str) -> Result<MathematicalGraph, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️GraphCodec

//#region 🔖️OpText
async fn print_mathematical_mutation(mutation: &MathematicalMutation) -> String {
    match mutation {
        MathematicalMutation::ChangeGraphDirected(p) => format!("change-graph-directed new-directed={}", enc_bool(p.new_directed)),
        MathematicalMutation::UpdateGraphAlgorithm(p) => format!("update-graph-algorithm new-algorithm={} new-algorithm-seed={}", enc_str(&p.new_algorithm), enc_opt_str(&p.new_algorithm_seed)),
        MathematicalMutation::ReplaceGraph(p) => format!("replace-graph graph={}", enc_graph(&p.graph)),
        MathematicalMutation::CreateNode(p) => format!("create-node id={} label={} x={} y={}", enc_str(&p.id), enc_str(&p.label), enc_f64(p.x), enc_f64(p.y)),
        MathematicalMutation::DeleteNode(p) => format!("delete-node id={}", enc_str(&p.id)),
        MathematicalMutation::DeleteNodes(p) => format!("delete-nodes ids={}", enc_str(&p.ids.join(","))),
        MathematicalMutation::ChangeNodeLabel(p) => format!("change-node-label id={} new-label={}", enc_str(&p.id), enc_str(&p.new_label)),
        MathematicalMutation::MoveNode(p) => format!("move-node id={} x={} y={}", enc_str(&p.id), enc_f64(p.x), enc_f64(p.y)),
        MathematicalMutation::ConnectNodes(p) => format!("connect-nodes id={} source={} target={}", enc_str(&p.id), enc_str(&p.source), enc_str(&p.target)),
        MathematicalMutation::DisconnectNodes(p) => format!("disconnect-nodes id={}", enc_str(&p.id)),
        MathematicalMutation::ReplacePoints(p) => format!("replace-points points={}", enc_points(&p.points)),
        MathematicalMutation::InsertPoint(p) => format!("insert-point index={} x={} y={}", enc_usize(p.index), enc_f64(p.x), enc_f64(p.y)),
        MathematicalMutation::RemovePoint(p) => format!("remove-point index={}", enc_usize(p.index)),
        MathematicalMutation::MovePoint(p) => format!("move-point index={} x={} y={}", enc_usize(p.index), enc_f64(p.x), enc_f64(p.y)),
        MathematicalMutation::ChangeCoefficient(p) => format!("change-coefficient label={} numer={} denom={}", enc_usize(p.label.0 as usize), enc_str(&p.numer), enc_str(&p.denom)),
    }
}

async fn parse_mathematical_mutation(line: &str) -> Result<MathematicalMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("mathematical mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-graph-directed" => Ok(MathematicalMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: dec_bool(&arg("new-directed")?)? })),
        "update-graph-algorithm" => Ok(MathematicalMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: dec_str(&arg("new-algorithm")?)?, new_algorithm_seed: dec_opt_str(&arg("new-algorithm-seed")?)? })),
        "replace-graph" => Ok(MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: dec_graph(&arg("graph")?)? })),
        "create-node" => Ok(MathematicalMutation::CreateNode(CreateNode { id: dec_str(&arg("id")?)?, label: dec_str(&arg("label")?)?, x: dec_f64(&arg("x")?)?, y: dec_f64(&arg("y")?)? })),
        "delete-node" => Ok(MathematicalMutation::DeleteNode(DeleteNode { id: dec_str(&arg("id")?)? })),
        "delete-nodes" => Ok(MathematicalMutation::DeleteNodes(DeleteNodes { ids: dec_str(&arg("ids")?)?.split(',').filter(|s| !s.is_empty()).map(str::to_string).collect() })),
        "change-node-label" => Ok(MathematicalMutation::ChangeNodeLabel(ChangeNodeLabel { id: dec_str(&arg("id")?)?, new_label: dec_str(&arg("new-label")?)? })),
        "move-node" => Ok(MathematicalMutation::MoveNode(MoveNode { id: dec_str(&arg("id")?)?, x: dec_f64(&arg("x")?)?, y: dec_f64(&arg("y")?)? })),
        "connect-nodes" => Ok(MathematicalMutation::ConnectNodes(ConnectNodes { id: dec_str(&arg("id")?)?, source: dec_str(&arg("source")?)?, target: dec_str(&arg("target")?)? })),
        "disconnect-nodes" => Ok(MathematicalMutation::DisconnectNodes(DisconnectNodes { id: dec_str(&arg("id")?)? })),
        "replace-points" => Ok(MathematicalMutation::ReplacePoints(ReplacePoints { points: dec_points(&arg("points")?)? })),
        "insert-point" => Ok(MathematicalMutation::InsertPoint(InsertPoint { index: dec_usize(&arg("index")?)?, x: dec_f64(&arg("x")?)?, y: dec_f64(&arg("y")?)? })),
        "remove-point" => Ok(MathematicalMutation::RemovePoint(RemovePoint { index: dec_usize(&arg("index")?)? })),
        "move-point" => Ok(MathematicalMutation::MovePoint(MovePoint { index: dec_usize(&arg("index")?)?, x: dec_f64(&arg("x")?)?, y: dec_f64(&arg("y")?)? })),
        "change-coefficient" => Ok(MathematicalMutation::ChangeCoefficient(ChangeCoefficient { label: EquationNodeLabel(dec_usize(&arg("label")?)? as u64), numer: dec_str(&arg("numer")?)?, denom: dec_str(&arg("denom")?)? })),
        other => Err(format!("mathematical mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for MathematicalMutation {
    async fn print_op(&self) -> String {
        print_mathematical_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_mathematical_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
async fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
async fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}
async fn write_opt_str_bin(out: &mut Vec<u8>, s: &Option<String>) {
    match s {
        Some(v) => {
            out.push(1);
            write_str_bin(out, v);
        }
        None => out.push(0),
    }
}
async fn read_opt_str_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_bin(reader)?)),
        other => Err(format!("bad option tag {other}")),
    }
}
async fn write_points_bin(out: &mut Vec<u8>, points: &[MathematicalPoint]) {
    store::pack_rt::write_varint_u64(out, points.len() as u64);
    for point in points {
        out.extend_from_slice(&point.x.to_le_bytes());
        out.extend_from_slice(&point.y.to_le_bytes());
    }
}
async fn read_points_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<MathematicalPoint>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| Ok(MathematicalPoint { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())? })).collect()
}

impl protocol::OpBinary for MathematicalMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            MathematicalMutation::ChangeGraphDirected(_) => 0,
            MathematicalMutation::UpdateGraphAlgorithm(_) => 1,
            MathematicalMutation::ReplaceGraph(_) => 2,
            MathematicalMutation::CreateNode(_) => 3,
            MathematicalMutation::DeleteNode(_) => 4,
            MathematicalMutation::DeleteNodes(_) => 5,
            MathematicalMutation::ChangeNodeLabel(_) => 6,
            MathematicalMutation::MoveNode(_) => 7,
            MathematicalMutation::ConnectNodes(_) => 8,
            MathematicalMutation::DisconnectNodes(_) => 9,
            MathematicalMutation::ReplacePoints(_) => 10,
            MathematicalMutation::InsertPoint(_) => 11,
            MathematicalMutation::RemovePoint(_) => 12,
            MathematicalMutation::MovePoint(_) => 13,
            MathematicalMutation::ChangeCoefficient(_) => 14,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            MathematicalMutation::ChangeGraphDirected(p) => out.push(p.new_directed as u8),
            MathematicalMutation::UpdateGraphAlgorithm(p) => {
                write_str_bin(&mut out, &p.new_algorithm);
                write_opt_str_bin(&mut out, &p.new_algorithm_seed);
            }
            MathematicalMutation::ReplaceGraph(p) => write_str_bin(&mut out, &enc_graph(&p.graph)),
            MathematicalMutation::CreateNode(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.label);
                out.extend_from_slice(&p.x.to_le_bytes());
                out.extend_from_slice(&p.y.to_le_bytes());
            }
            MathematicalMutation::DeleteNode(p) => write_str_bin(&mut out, &p.id),
            MathematicalMutation::DeleteNodes(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.ids.len() as u64);
                for id in &p.ids {
                    write_str_bin(&mut out, id);
                }
            }
            MathematicalMutation::ChangeNodeLabel(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.new_label);
            }
            MathematicalMutation::MoveNode(p) => {
                write_str_bin(&mut out, &p.id);
                out.extend_from_slice(&p.x.to_le_bytes());
                out.extend_from_slice(&p.y.to_le_bytes());
            }
            MathematicalMutation::ConnectNodes(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.source);
                write_str_bin(&mut out, &p.target);
            }
            MathematicalMutation::DisconnectNodes(p) => write_str_bin(&mut out, &p.id),
            MathematicalMutation::ReplacePoints(p) => write_points_bin(&mut out, &p.points),
            MathematicalMutation::InsertPoint(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.index as u64);
                out.extend_from_slice(&p.x.to_le_bytes());
                out.extend_from_slice(&p.y.to_le_bytes());
            }
            MathematicalMutation::RemovePoint(p) => store::pack_rt::write_varint_u64(&mut out, p.index as u64),
            MathematicalMutation::MovePoint(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.index as u64);
                out.extend_from_slice(&p.x.to_le_bytes());
                out.extend_from_slice(&p.y.to_le_bytes());
            }
            MathematicalMutation::ChangeCoefficient(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.label.0);
                write_str_bin(&mut out, &p.numer);
                write_str_bin(&mut out, &p.denom);
            }
        }
        Ok(out)
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(MathematicalMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: reader.read_u8().map_err(|e| malformed("new_directed", reader.position(), e.to_string()))? != 0 })),
            1 => {
                let new_algorithm = read_str_bin(&mut reader).map_err(|e| malformed("new_algorithm", reader.position(), e))?;
                let new_algorithm_seed = read_opt_str_bin(&mut reader).map_err(|e| malformed("new_algorithm_seed", reader.position(), e))?;
                Ok(MathematicalMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm, new_algorithm_seed }))
            }
            2 => {
                let text = read_str_bin(&mut reader).map_err(|e| malformed("graph", reader.position(), e))?;
                Ok(MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: dec_graph(&text).map_err(|e| malformed("graph", reader.position(), e))? }))
            }
            3 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let label = read_str_bin(&mut reader).map_err(|e| malformed("label", reader.position(), e))?;
                let x = reader.read_f64_le().map_err(|e| malformed("x", reader.position(), e.to_string()))?;
                let y = reader.read_f64_le().map_err(|e| malformed("y", reader.position(), e.to_string()))?;
                Ok(MathematicalMutation::CreateNode(CreateNode { id, label, x, y }))
            }
            4 => Ok(MathematicalMutation::DeleteNode(DeleteNode { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            5 => {
                let count = reader.read_varint_u64().map_err(|e| malformed("ids", reader.position(), e.to_string()))?;
                let ids = (0..count).map(|_| read_str_bin(&mut reader)).collect::<Result<Vec<_>, _>>().map_err(|e| malformed("ids", reader.position(), e))?;
                Ok(MathematicalMutation::DeleteNodes(DeleteNodes { ids }))
            }
            6 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_label = read_str_bin(&mut reader).map_err(|e| malformed("new_label", reader.position(), e))?;
                Ok(MathematicalMutation::ChangeNodeLabel(ChangeNodeLabel { id, new_label }))
            }
            7 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let x = reader.read_f64_le().map_err(|e| malformed("x", reader.position(), e.to_string()))?;
                let y = reader.read_f64_le().map_err(|e| malformed("y", reader.position(), e.to_string()))?;
                Ok(MathematicalMutation::MoveNode(MoveNode { id, x, y }))
            }
            8 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let source = read_str_bin(&mut reader).map_err(|e| malformed("source", reader.position(), e))?;
                let target = read_str_bin(&mut reader).map_err(|e| malformed("target", reader.position(), e))?;
                Ok(MathematicalMutation::ConnectNodes(ConnectNodes { id, source, target }))
            }
            9 => Ok(MathematicalMutation::DisconnectNodes(DisconnectNodes { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            10 => Ok(MathematicalMutation::ReplacePoints(ReplacePoints { points: read_points_bin(&mut reader).map_err(|e| malformed("points", reader.position(), e))? })),
            11 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("index", reader.position(), e.to_string()))? as usize;
                let x = reader.read_f64_le().map_err(|e| malformed("x", reader.position(), e.to_string()))?;
                let y = reader.read_f64_le().map_err(|e| malformed("y", reader.position(), e.to_string()))?;
                Ok(MathematicalMutation::InsertPoint(InsertPoint { index, x, y }))
            }
            12 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("index", reader.position(), e.to_string()))? as usize;
                Ok(MathematicalMutation::RemovePoint(RemovePoint { index }))
            }
            13 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("index", reader.position(), e.to_string()))? as usize;
                let x = reader.read_f64_le().map_err(|e| malformed("x", reader.position(), e.to_string()))?;
                let y = reader.read_f64_le().map_err(|e| malformed("y", reader.position(), e.to_string()))?;
                Ok(MathematicalMutation::MovePoint(MovePoint { index, x, y }))
            }
            14 => {
                let label = reader.read_varint_u64().map_err(|e| malformed("label", reader.position(), e.to_string()))?;
                let numer = read_str_bin(&mut reader).map_err(|e| malformed("numer", reader.position(), e))?;
                let denom = read_str_bin(&mut reader).map_err(|e| malformed("denom", reader.position(), e))?;
                Ok(MathematicalMutation::ChangeCoefficient(ChangeCoefficient { label: EquationNodeLabel(label), numer, denom }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<MathematicalMutation> {
    vec![
        MathematicalMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: false }),
        MathematicalMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: "bfs".into(), new_algorithm_seed: Some("a b".into()) }),
        MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: MathematicalGraph::default() }),
        MathematicalMutation::CreateNode(CreateNode { id: "z".into(), label: "Node Z".into(), x: 1.5, y: -2.5 }),
        MathematicalMutation::DeleteNode(DeleteNode { id: "a".into() }),
        MathematicalMutation::DeleteNodes(DeleteNodes { ids: vec!["a".into(), "b".into()] }),
        MathematicalMutation::ChangeNodeLabel(ChangeNodeLabel { id: "a".into(), new_label: "New Label".into() }),
        MathematicalMutation::MoveNode(MoveNode { id: "a".into(), x: 10.0, y: 20.0 }),
        MathematicalMutation::ConnectNodes(ConnectNodes { id: "e9".into(), source: "a".into(), target: "d".into() }),
        MathematicalMutation::DisconnectNodes(DisconnectNodes { id: "e1".into() }),
        MathematicalMutation::ReplacePoints(ReplacePoints { points: vec![MathematicalPoint { x: 1.0, y: 2.0 }] }),
        MathematicalMutation::InsertPoint(InsertPoint { index: 0, x: 3.0, y: 4.0 }),
        MathematicalMutation::RemovePoint(RemovePoint { index: 0 }),
        MathematicalMutation::MovePoint(MovePoint { index: 0, x: 7.0, y: 8.0 }),
        MathematicalMutation::ChangeCoefficient(ChangeCoefficient { label: EquationNodeLabel(3), numer: "5".into(), denom: "2".into() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{OpBinary, OpText};

    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <MathematicalMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <MathematicalMutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
