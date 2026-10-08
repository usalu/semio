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
    geometry::schema::mutations::{insert_point::InsertPoint, move_points::MovePoints, remove_point::RemovePoint, set_point_positions::{EquationPointPosition, SetPointPositions}},
    graph::schema::mutations::{
        change_graph_directed::ChangeGraphDirected, change_node_label::ChangeNodeLabel, connect_nodes::ConnectNodes, create_node::CreateNode, delete_node::DeleteNode, delete_nodes::DeleteNodes, disconnect_nodes::DisconnectNodes, move_node::MoveNode,
        move_nodes::MoveNodes, set_node_positions::{EquationNodePosition, SetNodePositions}, update_graph_algorithm::UpdateGraphAlgorithm,
    },
};

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



//#region 🔖️OpText
fn print_equation_mutation(mutation: &EquationMutation) -> String {
    match mutation {
        EquationMutation::ChangeGraphDirected(p) => format!("change-graph-directed new-directed={}", enc_bool(p.new_directed)),
        EquationMutation::UpdateGraphAlgorithm(p) => format!("update-graph-algorithm new-algorithm={} new-algorithm-seed={}", enc_str(&p.new_algorithm), enc_opt_str(&p.new_algorithm_seed)),
        EquationMutation::CreateNode(p) => format!("create-node id={} label={} x={} y={}{}", enc_str(&p.id), enc_str(&p.label), enc_f64(p.x), enc_f64(p.y), enc_index(p.index)),
        EquationMutation::DeleteNode(p) => format!("delete-node id={}", enc_str(&p.id)),
        EquationMutation::DeleteNodes(p) => format!("delete-nodes ids={}", enc_str(&p.ids.join(","))),
        EquationMutation::ChangeNodeLabel(p) => format!("change-node-label id={} new-label={}", enc_str(&p.id), enc_str(&p.new_label)),
        EquationMutation::MoveNode(p) => format!("move-node id={} x={} y={}", enc_str(&p.id), enc_f64(p.x), enc_f64(p.y)),
        EquationMutation::ConnectNodes(p) => format!("connect-nodes id={} source={} target={}{}", enc_str(&p.id), enc_str(&p.source), enc_str(&p.target), enc_index(p.index)),
        EquationMutation::DisconnectNodes(p) => format!("disconnect-nodes id={}", enc_str(&p.id)),
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
        "create-node" => Ok(EquationMutation::CreateNode(CreateNode { id: dec_str(&arg("id")?)?, label: dec_str(&arg("label")?)?, x: dec_f64(&arg("x")?)?, y: dec_f64(&arg("y")?)?, index: args.get("index").map(|value| dec_usize(value)).transpose()? })),
        "delete-node" => Ok(EquationMutation::DeleteNode(DeleteNode { id: dec_str(&arg("id")?)? })),
        "delete-nodes" => Ok(EquationMutation::DeleteNodes(DeleteNodes { ids: dec_str(&arg("ids")?)?.split(',').filter(|s| !s.is_empty()).map(str::to_string).collect() })),
        "change-node-label" => Ok(EquationMutation::ChangeNodeLabel(ChangeNodeLabel { id: dec_str(&arg("id")?)?, new_label: dec_str(&arg("new-label")?)? })),
        "move-node" => Ok(EquationMutation::MoveNode(MoveNode { id: dec_str(&arg("id")?)?, x: dec_f64(&arg("x")?)?, y: dec_f64(&arg("y")?)? })),
        "connect-nodes" => Ok(EquationMutation::ConnectNodes(ConnectNodes { id: dec_str(&arg("id")?)?, source: dec_str(&arg("source")?)?, target: dec_str(&arg("target")?)?, index: args.get("index").map(|value| dec_usize(value)).transpose()? })),
        "disconnect-nodes" => Ok(EquationMutation::DisconnectNodes(DisconnectNodes { id: dec_str(&arg("id")?)? })),
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








//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<EquationMutation> {
    vec![
        EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: false }),
        EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: "bfs".into(), new_algorithm_seed: Some("a b".into()) }),
        EquationMutation::CreateNode(CreateNode { id: "z".into(), label: "Node Z".into(), x: 1.5, y: -2.5, index: None }),
        EquationMutation::CreateNode(CreateNode { id: "y".into(), label: "Node Y".into(), x: 0.5, y: 2.0, index: Some(1) }),
        EquationMutation::DeleteNode(DeleteNode { id: "a".into() }),
        EquationMutation::DeleteNodes(DeleteNodes { ids: vec!["a".into(), "b".into()] }),
        EquationMutation::ChangeNodeLabel(ChangeNodeLabel { id: "a".into(), new_label: "New Label".into() }),
        EquationMutation::MoveNode(MoveNode { id: "a".into(), x: 10.0, y: 20.0 }),
        EquationMutation::ConnectNodes(ConnectNodes { id: "e9".into(), source: "a".into(), target: "d".into(), index: None }),
        EquationMutation::ConnectNodes(ConnectNodes { id: "e8".into(), source: "b".into(), target: "c".into(), index: Some(0) }),
        EquationMutation::DisconnectNodes(DisconnectNodes { id: "e1".into() }),
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
    geometry::schema::mutations::{insert_point, move_points, remove_point, set_point_positions},
    graph::schema::mutations::{change_graph_directed, change_node_label, connect_nodes, create_node, delete_node, delete_nodes, disconnect_nodes, move_node, move_nodes, set_node_positions, update_graph_algorithm},
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
    let forward = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation, &base);
    if let Ok(next) = protocol::apply_diff(forward.diff(), &applied) {
        applied = next;
    }
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &undone);
        if let Ok(next) = protocol::apply_diff(outcome.diff(), &undone) {
            undone = next;
        }
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
