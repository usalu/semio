//! 🎞️ Protocol hub wire frames: the lane-tagged `ClientFrame`/`ServerFrame` envelopes a
//! browser/native sync client exchanges with the collaboration hub, plus their binary codec. Frozen
//! contract: `.repo/🎫/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment`
//! §`protocol_wire`.
//!
//! **Deviation** (explicitly permitted by the contract's `🔖Codec` note): the byte encoding is
//! `lane: u8` followed by a varint-u64 length prefix and a `serde_json` body, not a fully
//! hand-rolled bincode-shaped binary layout. The frame *types* below are the frozen contract; only
//! this crate's byte encoding exercises the contract's declared "one degree of freedom". Rationale:
//! `ClientFrame`/`ServerFrame` carry deeply nested, schema-erased `serde_json::Value` payloads
//! (inside `protocol_causal::DocumentDiff`/`InverseOperation`, `ClientFrame::Presence`,
//! `ServerFrame::Error`, …) alongside `Vec<OperationEnvelope>` batches — a hand-rolled binary layout
//! for this shape would duplicate `serde`'s derive machinery field-by-field for no round-trip
//! benefit, since the payloads are opaque JSON already. `protocol_core`/`protocol_causal` are not
//! depended on for the varint primitive itself (this crate has no `pack_core` path dependency per
//! the frozen `Deps:` line), so the tiny LEB128 helpers below are a deliberate, self-contained
//! duplication of `pack_core::{write_varint_u64, read_varint_u64}`'s algorithm — not a reimplemented
//! *format*, just the same well-known encoding used everywhere else in this repo.

//#region 🔖Lane
/// @emoji 🛣️ Which logical channel a wire frame travels on: `Command` for causally-ordered,
/// durable operation batches; `Preview` for ephemeral, best-effort UI-state broadcast.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    Command = 0,
    Preview = 1,
}

impl Lane {
    fn to_byte(self) -> u8 {
        self as u8
    }

    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Lane::Command),
            1 => Some(Lane::Preview),
            _ => None,
        }
    }
}
//#endregion 🔖Lane

//#region 🔖ClientFrame
/// @emoji 📨 One frame a client sends to the hub.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClientFrame {
    Hello {
        wire_version: u32,
        protocol_version: u32,
        schema: String,
        pack_schema_hash: [u8; 32],
        actor: protocol_core::ActorId,
        token: Option<String>,
        resume_token: Option<String>,
        frontier: Option<protocol_causal::FrontierSummary>,
    },
    Commands {
        batch_id: u64,
        envelopes: Vec<protocol_causal::OperationEnvelope>,
    },
    FrontierAdvertise {
        frontier: protocol_causal::FrontierSummary,
    },
    PreviewPublish {
        key: String,
        seq: u64,
        payload: Vec<u8>,
    },
    Presence {
        peer: serde_json::Value,
    },
    CreditGrant {
        n: u32,
    },
    Bye,
}
//#endregion 🔖ClientFrame

//#region 🔖ServerFrame
/// @emoji 🚀 How a `ServerFrame::Welcome` seeds a freshly (re)connected client's local state.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Bootstrap {
    None,
    Snapshot { pack_hash: [u8; 32], inline: Option<Vec<u8>> },
    Tail,
}

/// @emoji ⚖️ How the hub resolved one submitted operation against concurrent history.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ApplyOutcome {
    Accepted,
    Transformed { envelope: protocol_causal::OperationEnvelope },
    Rejected { reason: String },
}

/// @emoji 🪜 One stage of a submitted batch's lifecycle, from `Received` to `Applied`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AckStage {
    Received,
    Persisted,
    Applied { outcome: ApplyOutcome },
}

/// @emoji 📬 One frame the hub sends to a client.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ServerFrame {
    Welcome {
        session_id: String,
        resume_token: String,
        server_frontier: protocol_causal::FrontierSummary,
        bootstrap: Bootstrap,
    },
    SnapshotChunk {
        seq: u32,
        bytes: Vec<u8>,
    },
    SnapshotDone {
        seq_count: u32,
    },
    Commands {
        envelopes: Vec<protocol_causal::OperationEnvelope>,
        origin: protocol_core::ActorId,
        frontier: protocol_causal::FrontierSummary,
    },
    Ack {
        batch_id: u64,
        stages: Vec<AckStage>,
        frontier: protocol_causal::FrontierSummary,
    },
    Preview {
        actor: protocol_core::ActorId,
        key: String,
        seq: u64,
        payload: Vec<u8>,
    },
    Presence {
        peers: Vec<serde_json::Value>,
    },
    CreditGrant {
        n: u32,
    },
    Error {
        code: String,
        message: String,
    },
}
//#endregion 🔖ServerFrame

//#region 🔖Codec
// Binary frame encode/decode: `lane: u8` + varint-u64 body length + serde_json bytes — see the
// module-level docstring for why this crate takes the contract's permitted deviation instead of a
// fully hand-rolled binary layout.

/// @emoji ✏️ Writes `value` as an unsigned LEB128 varint (minimal length) — a self-contained twin
/// of `pack_core::write_varint_u64`'s algorithm (see module docstring for why it's duplicated here
/// rather than path-depended on).
fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
    let mut remaining = value;
    loop {
        let byte = (remaining & 0x7F) as u8;
        remaining >>= 7;
        if remaining == 0 {
            out.push(byte);
            break;
        }
        out.push(byte | 0x80);
    }
}

/// @emoji 📖 Reads an unsigned LEB128 varint starting at `*pos`, advancing `*pos` past it.
fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, protocol_core::ProtocolError> {
    let start = *pos;
    let mut result: u64 = 0;
    for i in 0..10usize {
        let byte = *bytes
            .get(*pos)
            .ok_or_else(|| malformed("wire frame varint", *pos as u64, "truncated"))?;
        *pos += 1;
        let more = byte & 0x80 != 0;
        let payload = (byte & 0x7F) as u64;
        if i == 9 && (more || payload > 1) {
            return Err(malformed("wire frame varint", start as u64, "overlong varint (exceeds 10 bytes / 64 bits)"));
        }
        result |= payload << (i as u32 * 7);
        if !more {
            return Ok(result);
        }
    }
    Err(malformed("wire frame varint", start as u64, "overlong varint (exceeds 10 bytes)"))
}

fn malformed(what: &'static str, offset: u64, detail: &str) -> protocol_core::ProtocolError {
    protocol_core::ProtocolError::Malformed { what, offset, detail: detail.to_string() }
}

fn encode_frame<T: serde::Serialize>(frame: &T, lane: Lane) -> Vec<u8> {
    let json = serde_json::to_vec(frame).expect("wire frame types are always JSON-serializable");
    let mut out = Vec::with_capacity(1 + 5 + json.len());
    out.push(lane.to_byte());
    write_varint_u64(&mut out, json.len() as u64);
    out.extend_from_slice(&json);
    out
}

fn decode_frame<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<(Lane, T), protocol_core::ProtocolError> {
    let lane_byte = *bytes.first().ok_or_else(|| malformed("wire frame", 0, "empty frame"))?;
    let lane = Lane::from_byte(lane_byte).ok_or_else(|| malformed("wire frame lane byte", 0, &format!("unknown lane {lane_byte:#x}")))?;
    let mut pos = 1usize;
    let len = read_varint_u64(bytes, &mut pos)? as usize;
    let body = bytes
        .get(pos..pos + len)
        .ok_or_else(|| malformed("wire frame body", pos as u64, "declared length exceeds available bytes"))?;
    let frame: T = serde_json::from_slice(body).map_err(|error| malformed("wire frame json", pos as u64, &error.to_string()))?;
    Ok((lane, frame))
}

/// @emoji 📤 Encodes one `ClientFrame` on the given `Lane`.
pub fn encode_client_frame(frame: &ClientFrame, lane: Lane) -> Vec<u8> {
    encode_frame(frame, lane)
}

/// @emoji 📥 Decodes one `ClientFrame`, returning the `Lane` it was tagged with.
pub fn decode_client_frame(bytes: &[u8]) -> Result<(Lane, ClientFrame), protocol_core::ProtocolError> {
    decode_frame(bytes)
}

/// @emoji 📤 Encodes one `ServerFrame` on the given `Lane`.
pub fn encode_server_frame(frame: &ServerFrame, lane: Lane) -> Vec<u8> {
    encode_frame(frame, lane)
}

/// @emoji 📥 Decodes one `ServerFrame`, returning the `Lane` it was tagged with.
pub fn decode_server_frame(bytes: &[u8]) -> Result<(Lane, ServerFrame), protocol_core::ProtocolError> {
    decode_frame(bytes)
}
//#endregion 🔖Codec

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸Fixtures
    fn sample_envelope(id: &str) -> protocol_causal::OperationEnvelope {
        protocol_causal::OperationEnvelope {
            operation_id: protocol_core::OperationId(id.to_string()),
            document_id: protocol_core::DocumentId("document-1".to_string()),
            actor: protocol_core::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol_causal::DocumentDiff { schema: "diff.v1".to_string(), payload: serde_json::json!({"value": id}) },
            inverse: protocol_causal::InverseOperation { schema: "diff.v1".to_string(), inverse_diff: serde_json::json!({}) },
            timestamp: protocol_core::HybridLogicalTimestamp::new(1, 0),
        }
    }

    fn sample_frontier() -> protocol_causal::FrontierSummary {
        protocol_causal::FrontierSummary {
            document_id: protocol_core::DocumentId("document-1".to_string()),
            head_edit_ordinal: 5,
            head_edit_id: "edit-5".to_string(),
            last_commit_seq: 2,
            chain_hash: [7u8; 32],
        }
    }
    //#endregion 🧸Fixtures

    //#region 🔖Lane
    #[test]
    fn lane_byte_round_trips() {
        assert_eq!(Lane::from_byte(Lane::Command.to_byte()), Some(Lane::Command));
        assert_eq!(Lane::from_byte(Lane::Preview.to_byte()), Some(Lane::Preview));
        assert_eq!(Lane::from_byte(2), None);
    }
    //#endregion 🔖Lane

    //#region 🔖ClientFrame
    fn assert_client_round_trips(frame: &ClientFrame, lane: Lane) {
        let bytes = encode_client_frame(frame, lane);
        let (decoded_lane, decoded_frame) = decode_client_frame(&bytes).expect("decode must succeed");
        assert_eq!(decoded_lane, lane);
        assert_eq!(&decoded_frame, frame);
    }

    #[test]
    fn client_frame_hello_round_trips() {
        assert_client_round_trips(
            &ClientFrame::Hello {
                wire_version: 1,
                protocol_version: 1,
                schema: "schema.v1".to_string(),
                pack_schema_hash: [1u8; 32],
                actor: protocol_core::ActorId("actor-1".to_string()),
                token: Some("token".to_string()),
                resume_token: None,
                frontier: Some(sample_frontier()),
            },
            Lane::Command,
        );
    }

    #[test]
    fn client_frame_hello_with_no_optionals_round_trips() {
        assert_client_round_trips(
            &ClientFrame::Hello {
                wire_version: 1,
                protocol_version: 1,
                schema: "schema.v1".to_string(),
                pack_schema_hash: [0u8; 32],
                actor: protocol_core::ActorId("actor-2".to_string()),
                token: None,
                resume_token: None,
                frontier: None,
            },
            Lane::Command,
        );
    }

    #[test]
    fn client_frame_commands_round_trips() {
        assert_client_round_trips(&ClientFrame::Commands { batch_id: 42, envelopes: vec![sample_envelope("op-1"), sample_envelope("op-2")] }, Lane::Command);
    }

    #[test]
    fn client_frame_frontier_advertise_round_trips() {
        assert_client_round_trips(&ClientFrame::FrontierAdvertise { frontier: sample_frontier() }, Lane::Command);
    }

    #[test]
    fn client_frame_preview_publish_round_trips() {
        assert_client_round_trips(&ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview);
    }

    #[test]
    fn client_frame_presence_round_trips() {
        assert_client_round_trips(&ClientFrame::Presence { peer: serde_json::json!({"cursor": [1, 2]}) }, Lane::Preview);
    }

    #[test]
    fn client_frame_credit_grant_round_trips() {
        assert_client_round_trips(&ClientFrame::CreditGrant { n: 16 }, Lane::Command);
    }

    #[test]
    fn client_frame_bye_round_trips() {
        assert_client_round_trips(&ClientFrame::Bye, Lane::Command);
    }
    //#endregion 🔖ClientFrame

    //#region 🔖ServerFrame
    fn assert_server_round_trips(frame: &ServerFrame, lane: Lane) {
        let bytes = encode_server_frame(frame, lane);
        let (decoded_lane, decoded_frame) = decode_server_frame(&bytes).expect("decode must succeed");
        assert_eq!(decoded_lane, lane);
        assert_eq!(&decoded_frame, frame);
    }

    #[test]
    fn server_frame_welcome_round_trips_for_every_bootstrap_variant() {
        for bootstrap in [Bootstrap::None, Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: Some(vec![9, 9]) }, Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: None }, Bootstrap::Tail] {
            assert_server_round_trips(
                &ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: sample_frontier(), bootstrap },
                Lane::Command,
            );
        }
    }

    #[test]
    fn server_frame_snapshot_chunk_round_trips() {
        assert_server_round_trips(&ServerFrame::SnapshotChunk { seq: 0, bytes: vec![1, 2, 3, 4] }, Lane::Command);
    }

    #[test]
    fn server_frame_snapshot_done_round_trips() {
        assert_server_round_trips(&ServerFrame::SnapshotDone { seq_count: 4 }, Lane::Command);
    }

    #[test]
    fn server_frame_commands_round_trips() {
        assert_server_round_trips(
            &ServerFrame::Commands { envelopes: vec![sample_envelope("op-1")], origin: protocol_core::ActorId("actor-1".to_string()), frontier: sample_frontier() },
            Lane::Command,
        );
    }

    #[test]
    fn server_frame_ack_round_trips_for_every_stage_and_apply_outcome_variant() {
        for outcome in [ApplyOutcome::Accepted, ApplyOutcome::Transformed { envelope: sample_envelope("op-1") }, ApplyOutcome::Rejected { reason: "conflict".to_string() }] {
            assert_server_round_trips(
                &ServerFrame::Ack { batch_id: 7, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome }], frontier: sample_frontier() },
                Lane::Command,
            );
        }
    }

    #[test]
    fn server_frame_preview_round_trips() {
        assert_server_round_trips(&ServerFrame::Preview { actor: protocol_core::ActorId("actor-1".to_string()), key: "cursor".to_string(), seq: 3, payload: vec![5, 6] }, Lane::Preview);
    }

    #[test]
    fn server_frame_presence_round_trips() {
        assert_server_round_trips(&ServerFrame::Presence { peers: vec![serde_json::json!({"id": "a"}), serde_json::json!({"id": "b"})] }, Lane::Preview);
    }

    #[test]
    fn server_frame_credit_grant_round_trips() {
        assert_server_round_trips(&ServerFrame::CreditGrant { n: 32 }, Lane::Command);
    }

    #[test]
    fn server_frame_error_round_trips() {
        assert_server_round_trips(&ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command);
    }
    //#endregion 🔖ServerFrame

    //#region 🔖Codec
    #[test]
    fn decode_client_frame_rejects_empty_bytes() {
        let err = decode_client_frame(&[]).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "wire frame", .. }));
    }

    #[test]
    fn decode_client_frame_rejects_unknown_lane_byte() {
        let err = decode_client_frame(&[2u8, 0]).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "wire frame lane byte", .. }));
    }

    #[test]
    fn decode_client_frame_rejects_truncated_body() {
        let mut bytes = encode_client_frame(&ClientFrame::Bye, Lane::Command);
        bytes.push(0xFF); // grow the declared length beyond what actually follows
        let extended_len_bytes = {
            let mut out = Vec::new();
            out.push(Lane::Command.to_byte());
            write_varint_u64(&mut out, 999);
            out
        };
        assert!(decode_client_frame(&extended_len_bytes).is_err());
        let _ = bytes; // original well-formed encoding stays valid; only the crafted one is truncated
    }

    #[test]
    fn decode_server_frame_rejects_malformed_json_body() {
        let mut out = Vec::new();
        out.push(Lane::Command.to_byte());
        let json = b"{not valid json";
        write_varint_u64(&mut out, json.len() as u64);
        out.extend_from_slice(json);
        let err = decode_server_frame(&out).unwrap_err();
        assert!(matches!(err, protocol_core::ProtocolError::Malformed { what: "wire frame json", .. }));
    }

    #[test]
    fn different_lanes_produce_different_leading_bytes_but_same_body() {
        let command_bytes = encode_client_frame(&ClientFrame::Bye, Lane::Command);
        let preview_bytes = encode_client_frame(&ClientFrame::Bye, Lane::Preview);
        assert_eq!(command_bytes[0], 0);
        assert_eq!(preview_bytes[0], 1);
        assert_eq!(command_bytes[1..], preview_bytes[1..]);
    }
    //#endregion 🔖Codec
}
//#endregion 🧪Tests
