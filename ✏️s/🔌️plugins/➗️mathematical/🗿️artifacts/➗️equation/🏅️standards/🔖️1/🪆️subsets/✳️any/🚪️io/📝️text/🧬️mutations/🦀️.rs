//! ⚡️ Equation artifact — hand-rolled `OpText`/`OpBinary` for `EquationMutation`.
//! `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../../🧬️mutations/🦀️.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted
//! here, one keyword per semantic verb, grammar `keyword key1=value1 key2=value2 ...`.

use crate::schema::mutations::EquationMutation;

// 🪆️ Direct absolute paths, not the `schema::mutations` shim: these leaf modules moved to
// their real owning subset (ticket
// 26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION),
// so they are no longer reachable through `✳️any::schema::mutations::<name>`.
use crate::standards::v1::subsets::any::schema::snapshot::EquationNodeLabel;
use crate::standards::v1::subsets::{
    equation::schema::mutations::change_coefficient::ChangeCoefficient,
    geometry::schema::mutations::{insert_point::InsertPoint, move_points::MovePoints, remove_point::RemovePoint, replace_points::ReplacePoints, set_point_positions::{EquationPointPosition, SetPointPositions}},
    graph::schema::mutations::{
        change_graph_directed::ChangeGraphDirected, change_node_label::ChangeNodeLabel, connect_nodes::ConnectNodes, create_node::CreateNode, delete_node::DeleteNode, delete_nodes::DeleteNodes, disconnect_nodes::DisconnectNodes, move_node::MoveNode,
        move_nodes::MoveNodes, replace_graph::ReplaceGraph, set_node_positions::{EquationNodePosition, SetNodePositions}, update_graph_algorithm::UpdateGraphAlgorithm,
    },
};
use crate::{EquationGraph, EquationPoint};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other scalar's text form stays space-free and tokenizable by [`tokenize_args`].
fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
fn dec_str(s: &str) -> Result<String, String> {
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
fn enc_opt_str(s: &Option<String>) -> String {
    match s {
        Some(v) => enc_str(v),
        None => "-".to_string(),
    }
}
fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(dec_str(s)?))
    }
}
fn enc_f64(v: f64) -> String {
    format!("{v}")
}
fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
fn enc_usize(v: usize) -> String {
    v.to_string()
}
fn dec_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
/// 📍️ The optional roster position of `create-node`/`connect-nodes`: absent appends, so the key is
/// printed only when present.
fn enc_index(index: Option<usize>) -> String {
    index.map(|value| format!(" index={}", enc_usize(value))).unwrap_or_default()
}
fn write_opt_usize_bin(out: &mut Vec<u8>, index: Option<usize>) {
    match index {
        Some(value) => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, value as u64);
        }
        None => out.push(0),
    }
}
fn read_opt_usize_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<usize>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)),
        other => Err(format!("bad option tag {other}")),
    }
}
fn enc_bool(v: bool) -> String {
    v.to_string()
}
fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}
fn enc_points(points: &[EquationPoint]) -> String {
    format!("[{}]", points.iter().map(|p| format!("{},{}", p.x, p.y)).collect::<Vec<_>>().join(";"))
}
fn dec_points(s: &str) -> Result<Vec<EquationPoint>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected bracketed point list, got {s:?}"))?;
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    inner
        .split(';')
        .map(|pair| {
            let (x, y) = pair.split_once(',').ok_or_else(|| format!("bad point pair {pair:?}"))?;
            Ok(EquationPoint { x: dec_f64(x)?, y: dec_f64(y)? })
        })
        .collect()
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value —
/// needed because node labels/algorithm ids may contain spaces.
fn tokenize_args(rest: &str) -> Vec<String> {
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
fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️GraphCodec
/// 🕸️ Whole-`EquationGraph` text form (used by `replace-graph`) — a quoted first-party JSON
/// string (`pack::json::to_json_string`/`from_json_str`, over `EquationGraph`'s own
/// `ToValue`/`FromValue`) rather than a second handcrafted graph grammar; `enc_str`/`dec_str`'s
/// backslash/quote escaping round-trips it byte-for-byte.
fn enc_graph(graph: &EquationGraph) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(graph))
}
fn dec_graph(s: &str) -> Result<EquationGraph, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}
//#endregion 🔖️GraphCodec

//#region 🔖️OpText
fn print_equation_mutation(mutation: &EquationMutation) -> String {
    match mutation {
        EquationMutation::ChangeGraphDirected(p) => format!("change-graph-directed new-directed={}", enc_bool(p.new_directed)),
        EquationMutation::UpdateGraphAlgorithm(p) => format!("update-graph-algorithm new-algorithm={} new-algorithm-seed={}", enc_str(&p.new_algorithm), enc_opt_str(&p.new_algorithm_seed)),
        EquationMutation::ReplaceGraph(p) => format!("replace-graph graph={}", enc_graph(&p.graph)),
        EquationMutation::CreateNode(p) => format!("create-node id={} label={} x={} y={}{}", enc_str(&p.id), enc_str(&p.label), enc_f64(p.x), enc_f64(p.y), enc_index(p.index)),
        EquationMutation::DeleteNode(p) => format!("delete-node id={}", enc_str(&p.id)),
        EquationMutation::DeleteNodes(p) => format!("delete-nodes ids={}", enc_str(&p.ids.join(","))),
        EquationMutation::ChangeNodeLabel(p) => format!("change-node-label id={} new-label={}", enc_str(&p.id), enc_str(&p.new_label)),
        EquationMutation::MoveNode(p) => format!("move-node id={} x={} y={}", enc_str(&p.id), enc_f64(p.x), enc_f64(p.y)),
        EquationMutation::ConnectNodes(p) => format!("connect-nodes id={} source={} target={}{}", enc_str(&p.id), enc_str(&p.source), enc_str(&p.target), enc_index(p.index)),
        EquationMutation::DisconnectNodes(p) => format!("disconnect-nodes id={}", enc_str(&p.id)),
        EquationMutation::ReplacePoints(p) => format!("replace-points points={}", enc_points(&p.points)),
        EquationMutation::InsertPoint(p) => format!("insert-point index={} x={} y={}", enc_usize(p.index), enc_f64(p.x), enc_f64(p.y)),
        EquationMutation::RemovePoint(p) => format!("remove-point index={}", enc_usize(p.index)),
        EquationMutation::MovePoints(p) => format!("move-points indices={} dx={} dy={}", enc_str(&semio_framework_pack_json::to_json_string(&p.indices)), enc_f64(p.dx), enc_f64(p.dy)),
        EquationMutation::ChangeCoefficient(p) => format!("change-coefficient label={} numer={} denom={}", enc_usize(p.label.0 as usize), enc_str(&p.numer), enc_str(&p.denom)),
        EquationMutation::MoveNodes(p) => format!("move-nodes ids={} dx={} dy={}", enc_str(&semio_framework_pack_json::to_json_string(&p.ids)), enc_f64(p.dx), enc_f64(p.dy)),
        EquationMutation::SetNodePositions(p) => format!("set-node-positions positions={}", enc_str(&semio_framework_pack_json::to_json_string(&p.positions))),
        EquationMutation::SetPointPositions(p) => format!("set-point-positions positions={}", enc_str(&semio_framework_pack_json::to_json_string(&p.positions))),
    }
}

fn parse_equation_mutation(line: &str) -> Result<EquationMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("equation mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-graph-directed" => Ok(EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: dec_bool(&arg("new-directed")?)? })),
        "update-graph-algorithm" => Ok(EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: dec_str(&arg("new-algorithm")?)?, new_algorithm_seed: dec_opt_str(&arg("new-algorithm-seed")?)? })),
        "replace-graph" => Ok(EquationMutation::ReplaceGraph(ReplaceGraph { graph: dec_graph(&arg("graph")?)? })),
        "create-node" => Ok(EquationMutation::CreateNode(CreateNode { id: dec_str(&arg("id")?)?, label: dec_str(&arg("label")?)?, x: dec_f64(&arg("x")?)?, y: dec_f64(&arg("y")?)?, index: args.get("index").map(|value| dec_usize(value)).transpose()? })),
        "delete-node" => Ok(EquationMutation::DeleteNode(DeleteNode { id: dec_str(&arg("id")?)? })),
        "delete-nodes" => Ok(EquationMutation::DeleteNodes(DeleteNodes { ids: dec_str(&arg("ids")?)?.split(',').filter(|s| !s.is_empty()).map(str::to_string).collect() })),
        "change-node-label" => Ok(EquationMutation::ChangeNodeLabel(ChangeNodeLabel { id: dec_str(&arg("id")?)?, new_label: dec_str(&arg("new-label")?)? })),
        "move-node" => Ok(EquationMutation::MoveNode(MoveNode { id: dec_str(&arg("id")?)?, x: dec_f64(&arg("x")?)?, y: dec_f64(&arg("y")?)? })),
        "connect-nodes" => Ok(EquationMutation::ConnectNodes(ConnectNodes { id: dec_str(&arg("id")?)?, source: dec_str(&arg("source")?)?, target: dec_str(&arg("target")?)?, index: args.get("index").map(|value| dec_usize(value)).transpose()? })),
        "disconnect-nodes" => Ok(EquationMutation::DisconnectNodes(DisconnectNodes { id: dec_str(&arg("id")?)? })),
        "replace-points" => Ok(EquationMutation::ReplacePoints(ReplacePoints { points: dec_points(&arg("points")?)? })),
        "insert-point" => Ok(EquationMutation::InsertPoint(InsertPoint { index: dec_usize(&arg("index")?)?, x: dec_f64(&arg("x")?)?, y: dec_f64(&arg("y")?)? })),
        "remove-point" => Ok(EquationMutation::RemovePoint(RemovePoint { index: dec_usize(&arg("index")?)? })),
        "move-points" => Ok(EquationMutation::MovePoints(MovePoints { indices: semio_framework_pack_json::from_json_str(&dec_str(&arg("indices")?)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())?, dx: dec_f64(&arg("dx")?)?, dy: dec_f64(&arg("dy")?)? })),
        "change-coefficient" => Ok(EquationMutation::ChangeCoefficient(ChangeCoefficient { label: EquationNodeLabel(dec_usize(&arg("label")?)? as u64), numer: dec_str(&arg("numer")?)?, denom: dec_str(&arg("denom")?)? })),
        "move-nodes" => Ok(EquationMutation::MoveNodes(MoveNodes { ids: semio_framework_pack_json::from_json_str(&dec_str(&arg("ids")?)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())?, dx: dec_f64(&arg("dx")?)?, dy: dec_f64(&arg("dy")?)? })),
        "set-node-positions" => Ok(EquationMutation::SetNodePositions(SetNodePositions { positions: semio_framework_pack_json::from_json_str::<Vec<EquationNodePosition>>(&dec_str(&arg("positions")?)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())? })),
        "set-point-positions" => Ok(EquationMutation::SetPointPositions(SetPointPositions { positions: semio_framework_pack_json::from_json_str::<Vec<EquationPointPosition>>(&dec_str(&arg("positions")?)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())? })),
        other => Err(format!("equation mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for EquationMutation {
    fn print_op(&self) -> String {
        print_equation_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_equation_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}
fn write_opt_str_bin(out: &mut Vec<u8>, s: &Option<String>) {
    match s {
        Some(v) => {
            out.push(1);
            write_str_bin(out, v);
        }
        None => out.push(0),
    }
}
fn read_opt_str_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_bin(reader)?)),
        other => Err(format!("bad option tag {other}")),
    }
}
fn write_points_bin(out: &mut Vec<u8>, points: &[EquationPoint]) {
    store::pack_rt::write_varint_u64(out, points.len() as u64);
    for point in points {
        out.extend_from_slice(&point.x.to_le_bytes());
        out.extend_from_slice(&point.y.to_le_bytes());
    }
}
fn read_points_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<EquationPoint>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| Ok(EquationPoint { x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())? })).collect()
}

impl protocol::OpBinary for EquationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            EquationMutation::ChangeGraphDirected(_) => 0,
            EquationMutation::UpdateGraphAlgorithm(_) => 1,
            EquationMutation::ReplaceGraph(_) => 2,
            EquationMutation::CreateNode(_) => 3,
            EquationMutation::DeleteNode(_) => 4,
            EquationMutation::DeleteNodes(_) => 5,
            EquationMutation::ChangeNodeLabel(_) => 6,
            EquationMutation::MoveNode(_) => 7,
            EquationMutation::ConnectNodes(_) => 8,
            EquationMutation::DisconnectNodes(_) => 9,
            EquationMutation::ReplacePoints(_) => 10,
            EquationMutation::InsertPoint(_) => 11,
            EquationMutation::RemovePoint(_) => 12,
            EquationMutation::MovePoints(_) => 13,
            EquationMutation::ChangeCoefficient(_) => 14,
            EquationMutation::MoveNodes(_) => 15,
            EquationMutation::SetNodePositions(_) => 16,
            EquationMutation::SetPointPositions(_) => 17,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            EquationMutation::ChangeGraphDirected(p) => out.push(p.new_directed as u8),
            EquationMutation::UpdateGraphAlgorithm(p) => {
                write_str_bin(&mut out, &p.new_algorithm);
                write_opt_str_bin(&mut out, &p.new_algorithm_seed);
            }
            EquationMutation::ReplaceGraph(p) => write_str_bin(&mut out, &enc_graph(&p.graph)),
            EquationMutation::CreateNode(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.label);
                out.extend_from_slice(&p.x.to_le_bytes());
                out.extend_from_slice(&p.y.to_le_bytes());
                write_opt_usize_bin(&mut out, p.index);
            }
            EquationMutation::DeleteNode(p) => write_str_bin(&mut out, &p.id),
            EquationMutation::DeleteNodes(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.ids.len() as u64);
                for id in &p.ids {
                    write_str_bin(&mut out, id);
                }
            }
            EquationMutation::ChangeNodeLabel(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.new_label);
            }
            EquationMutation::MoveNode(p) => {
                write_str_bin(&mut out, &p.id);
                out.extend_from_slice(&p.x.to_le_bytes());
                out.extend_from_slice(&p.y.to_le_bytes());
            }
            EquationMutation::ConnectNodes(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.source);
                write_str_bin(&mut out, &p.target);
                write_opt_usize_bin(&mut out, p.index);
            }
            EquationMutation::DisconnectNodes(p) => write_str_bin(&mut out, &p.id),
            EquationMutation::ReplacePoints(p) => write_points_bin(&mut out, &p.points),
            EquationMutation::InsertPoint(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.index as u64);
                out.extend_from_slice(&p.x.to_le_bytes());
                out.extend_from_slice(&p.y.to_le_bytes());
            }
            EquationMutation::RemovePoint(p) => store::pack_rt::write_varint_u64(&mut out, p.index as u64),
            EquationMutation::MovePoints(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.indices.len() as u64);
                for index in &p.indices {
                    store::pack_rt::write_varint_u64(&mut out, *index as u64);
                }
                out.extend_from_slice(&p.dx.to_le_bytes());
                out.extend_from_slice(&p.dy.to_le_bytes());
            }
            EquationMutation::ChangeCoefficient(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.label.0);
                write_str_bin(&mut out, &p.numer);
                write_str_bin(&mut out, &p.denom);
            }
            EquationMutation::MoveNodes(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.ids.len() as u64);
                for id in &p.ids {
                    write_str_bin(&mut out, id);
                }
                out.extend_from_slice(&p.dx.to_le_bytes());
                out.extend_from_slice(&p.dy.to_le_bytes());
            }
            EquationMutation::SetNodePositions(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.positions.len() as u64);
                for position in &p.positions {
                    write_str_bin(&mut out, &position.id);
                    out.extend_from_slice(&position.x.to_le_bytes());
                    out.extend_from_slice(&position.y.to_le_bytes());
                }
            }
            EquationMutation::SetPointPositions(p) => {
                store::pack_rt::write_varint_u64(&mut out, p.positions.len() as u64);
                for position in &p.positions {
                    store::pack_rt::write_varint_u64(&mut out, position.index as u64);
                    out.extend_from_slice(&position.x.to_le_bytes());
                    out.extend_from_slice(&position.y.to_le_bytes());
                }
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: reader.read_u8().map_err(|e| malformed("new_directed", reader.position(), e.to_string()))? != 0 })),
            1 => {
                let new_algorithm = read_str_bin(&mut reader).map_err(|e| malformed("new_algorithm", reader.position(), e))?;
                let new_algorithm_seed = read_opt_str_bin(&mut reader).map_err(|e| malformed("new_algorithm_seed", reader.position(), e))?;
                Ok(EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm, new_algorithm_seed }))
            }
            2 => {
                let text = read_str_bin(&mut reader).map_err(|e| malformed("graph", reader.position(), e))?;
                Ok(EquationMutation::ReplaceGraph(ReplaceGraph { graph: dec_graph(&text).map_err(|e| malformed("graph", reader.position(), e))? }))
            }
            3 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let label = read_str_bin(&mut reader).map_err(|e| malformed("label", reader.position(), e))?;
                let x = reader.read_f64_le().map_err(|e| malformed("x", reader.position(), e.to_string()))?;
                let y = reader.read_f64_le().map_err(|e| malformed("y", reader.position(), e.to_string()))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(EquationMutation::CreateNode(CreateNode { id, label, x, y, index }))
            }
            4 => Ok(EquationMutation::DeleteNode(DeleteNode { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            5 => {
                let count = reader.read_varint_u64().map_err(|e| malformed("ids", reader.position(), e.to_string()))?;
                let ids = (0..count).map(|_| read_str_bin(&mut reader)).collect::<Result<Vec<_>, _>>().map_err(|e| malformed("ids", reader.position(), e))?;
                Ok(EquationMutation::DeleteNodes(DeleteNodes { ids }))
            }
            6 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_label = read_str_bin(&mut reader).map_err(|e| malformed("new_label", reader.position(), e))?;
                Ok(EquationMutation::ChangeNodeLabel(ChangeNodeLabel { id, new_label }))
            }
            7 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let x = reader.read_f64_le().map_err(|e| malformed("x", reader.position(), e.to_string()))?;
                let y = reader.read_f64_le().map_err(|e| malformed("y", reader.position(), e.to_string()))?;
                Ok(EquationMutation::MoveNode(MoveNode { id, x, y }))
            }
            8 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let source = read_str_bin(&mut reader).map_err(|e| malformed("source", reader.position(), e))?;
                let target = read_str_bin(&mut reader).map_err(|e| malformed("target", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(EquationMutation::ConnectNodes(ConnectNodes { id, source, target, index }))
            }
            9 => Ok(EquationMutation::DisconnectNodes(DisconnectNodes { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            10 => Ok(EquationMutation::ReplacePoints(ReplacePoints { points: read_points_bin(&mut reader).map_err(|e| malformed("points", reader.position(), e))? })),
            11 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("index", reader.position(), e.to_string()))? as usize;
                let x = reader.read_f64_le().map_err(|e| malformed("x", reader.position(), e.to_string()))?;
                let y = reader.read_f64_le().map_err(|e| malformed("y", reader.position(), e.to_string()))?;
                Ok(EquationMutation::InsertPoint(InsertPoint { index, x, y }))
            }
            12 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("index", reader.position(), e.to_string()))? as usize;
                Ok(EquationMutation::RemovePoint(RemovePoint { index }))
            }
            13 => {
                let count = reader.read_varint_u64().map_err(|e| malformed("indices", reader.position(), e.to_string()))?;
                let indices = (0..count).map(|_| reader.read_varint_u64().map(|index| index as usize)).collect::<Result<Vec<_>, _>>().map_err(|e| malformed("indices", reader.position(), e.to_string()))?;
                let dx = reader.read_f64_le().map_err(|e| malformed("dx", reader.position(), e.to_string()))?;
                let dy = reader.read_f64_le().map_err(|e| malformed("dy", reader.position(), e.to_string()))?;
                Ok(EquationMutation::MovePoints(MovePoints { indices, dx, dy }))
            }
            14 => {
                let label = reader.read_varint_u64().map_err(|e| malformed("label", reader.position(), e.to_string()))?;
                let numer = read_str_bin(&mut reader).map_err(|e| malformed("numer", reader.position(), e))?;
                let denom = read_str_bin(&mut reader).map_err(|e| malformed("denom", reader.position(), e))?;
                Ok(EquationMutation::ChangeCoefficient(ChangeCoefficient { label: EquationNodeLabel(label), numer, denom }))
            }
            15 => {
                let count = reader.read_varint_u64().map_err(|e| malformed("ids", reader.position(), e.to_string()))?;
                let ids = (0..count).map(|_| read_str_bin(&mut reader)).collect::<Result<Vec<_>, _>>().map_err(|e| malformed("ids", reader.position(), e))?;
                let dx = reader.read_f64_le().map_err(|e| malformed("dx", reader.position(), e.to_string()))?;
                let dy = reader.read_f64_le().map_err(|e| malformed("dy", reader.position(), e.to_string()))?;
                Ok(EquationMutation::MoveNodes(MoveNodes { ids, dx, dy }))
            }
            16 => {
                let count = reader.read_varint_u64().map_err(|e| malformed("positions", reader.position(), e.to_string()))?;
                let positions = (0..count)
                    .map(|_| -> Result<EquationNodePosition, String> { Ok(EquationNodePosition { id: read_str_bin(&mut reader)?, x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())? }) })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| malformed("positions", reader.position(), e))?;
                Ok(EquationMutation::SetNodePositions(SetNodePositions { positions }))
            }
            17 => {
                let count = reader.read_varint_u64().map_err(|e| malformed("positions", reader.position(), e.to_string()))?;
                let positions = (0..count)
                    .map(|_| -> Result<EquationPointPosition, String> { Ok(EquationPointPosition { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize, x: reader.read_f64_le().map_err(|e| e.to_string())?, y: reader.read_f64_le().map_err(|e| e.to_string())? }) })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| malformed("positions", reader.position(), e))?;
                Ok(EquationMutation::SetPointPositions(SetPointPositions { positions }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<EquationMutation> {
    vec![
        EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: false }),
        EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: "bfs".into(), new_algorithm_seed: Some("a b".into()) }),
        EquationMutation::ReplaceGraph(ReplaceGraph { graph: EquationGraph::default() }),
        EquationMutation::CreateNode(CreateNode { id: "z".into(), label: "Node Z".into(), x: 1.5, y: -2.5, index: None }),
        EquationMutation::CreateNode(CreateNode { id: "y".into(), label: "Node Y".into(), x: 0.5, y: 2.0, index: Some(1) }),
        EquationMutation::DeleteNode(DeleteNode { id: "a".into() }),
        EquationMutation::DeleteNodes(DeleteNodes { ids: vec!["a".into(), "b".into()] }),
        EquationMutation::ChangeNodeLabel(ChangeNodeLabel { id: "a".into(), new_label: "New Label".into() }),
        EquationMutation::MoveNode(MoveNode { id: "a".into(), x: 10.0, y: 20.0 }),
        EquationMutation::ConnectNodes(ConnectNodes { id: "e9".into(), source: "a".into(), target: "d".into(), index: None }),
        EquationMutation::ConnectNodes(ConnectNodes { id: "e8".into(), source: "b".into(), target: "c".into(), index: Some(0) }),
        EquationMutation::DisconnectNodes(DisconnectNodes { id: "e1".into() }),
        EquationMutation::ReplacePoints(ReplacePoints { points: vec![EquationPoint { x: 1.0, y: 2.0 }] }),
        EquationMutation::InsertPoint(InsertPoint { index: 0, x: 3.0, y: 4.0 }),
        EquationMutation::RemovePoint(RemovePoint { index: 0 }),
        EquationMutation::MovePoints(MovePoints { indices: vec![0, 2], dx: 7.0, dy: -8.5 }),
        EquationMutation::ChangeCoefficient(ChangeCoefficient { label: EquationNodeLabel(3), numer: "5".into(), denom: "2".into() }),
        EquationMutation::MoveNodes(MoveNodes { ids: vec!["a".into(), "b".into()], dx: 40.0, dy: -12.5 }),
        EquationMutation::SetNodePositions(SetNodePositions { positions: vec![EquationNodePosition { id: "a".into(), x: 120.0, y: 40.0 }, EquationNodePosition { id: "b".into(), x: -30.5, y: 260.0 }] }),
        EquationMutation::SetPointPositions(SetPointPositions { positions: vec![EquationPointPosition { index: 0, x: 40.0, y: 220.0 }, EquationPointPosition { index: 2, x: 360.5, y: -14.0 }] }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{EquationDiff, EquationSnapshot};
use semio_framework_value::ToValue;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use crate::standards::v1::subsets::{
    equation::schema::mutations::change_coefficient,
    geometry::schema::mutations::{insert_point, move_points, remove_point, replace_points, set_point_positions},
    graph::schema::mutations::{change_graph_directed, change_node_label, connect_nodes, create_node, delete_node, delete_nodes, disconnect_nodes, move_node, move_nodes, replace_graph, set_node_positions, update_graph_algorithm},
};

/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `EquationMutation` nor
/// `EquationSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn equation_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<EquationSnapshot, String> { semio_framework_pack_json::from_json_str::<EquationSnapshot>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: EquationMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let messages_json = semio_framework_pack_json::from_dsl_value(&forward.messages().to_vec().to_value());
    let inverse_messages_json = semio_framework_pack_json::from_dsl_value(&inverse_messages.to_value());
    let report = semio_framework_pack_json::object([
        ("base".to_string(), semio_framework_pack_json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), semio_framework_pack_json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), messages_json),
        ("inverseSteps".to_string(), semio_framework_pack_json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), inverse_messages_json),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
}
pub use mutations_codec::*;
