//! ⚖️ Equation artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! `protocol::OpBinary for EquationMutation` is implemented directly in `crate::op`
//! (see that module's doc comment). This component only adds the thin artifact-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law and a whole-store round trip.
//!
//! The app's typed `EquationCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in `✏️editor/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::op::EquationMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `EquationMutation` to its binary command form.
pub fn encode_op(operation: &EquationMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `EquationMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<EquationMutation, protocol::ProtocolError> {
    EquationMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
use crate::schema::mutations::EquationMutation;
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
use crate::standards::v1::subsets::any::io::text::mutations::{enc_graph,dec_graph};
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
}
