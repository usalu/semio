"""🧩️ G11 landing set (session 13, coordinator item b): the agent transaction carries owned-child op groups.

P8's `p8-agent-lane` made the agent preview run the SAME retained job the shell runs and refuse, by name, every lane an agent
transaction cannot carry (`interactive-job.agent-lane-uncarried`). Owned children were one of those lanes: flow ×10 and
sequence ×12 child-group verbs (and every other composing plugin's child edits) were refused on the agent lane while the
shell applies them as one composite gesture through `store::CompositionCoordinator::dispatch_group`.

This set closes that lane at the root, end to end:
- `🏪️store`: `GroupMeta.group_id` — a caller-owned group identity (an agent transaction id) instead of the content-minted one,
  so the gateway's `TransactionUndo{group_id: txn_id}` reaches every member of the group.
- `📡️spr/🧵️channel` (CHANNEL_VERSION 18): `AppFrame::Emit.child_ops` and `AppCommand::TransactionPrepare.prepared_child_ops`
  (trailing, opaque to the channel: the guest's `ChildEmit` list as one wire pack), both codecs incl. the paged route decoder
  and its bounded close stages; Rust + TS twins, the shared hex fixtures and the version pin.
- `🔌️plugin` SDK: `ChildEmit` crosses the wire (`ChildEmit::encode_groups`/`decode_groups`); the preview carries the child
  groups (bounded by the retained job's own output cap exactly as on the shell lane; viewer apps refuse them) instead of
  refusing them; `transaction_prepare`
  admits them (a child the instance does not hold is refused by name); `transaction_commit` of a group commits parent +
  children as ONE composite gesture through `dispatch_emit_group` with `group_id = txn_id`; `transaction_undo`/`redo` of a
  group move every member through `CompositionCoordinator::undo_group`/`redo_group`. `EmitWire` replaces the anonymous
  op-pack triple.
- `🌉️mcp` gateway: `PreparedOps.children` carries the bytes from `Emit` to `TransactionPrepare` unmodified; an invocation that
  touches only children is a real change (not `no-change`).

usage: python3 g11-child-carrier.py [--apply]   (default: dry run; --apply writes every file or none)"""
import os
import sys
import time

ROOT = "/Users/ueli/Documents/semio"
OS = f"{ROOT}/🧰️framework/🛍️products/💻️os"
MOD = f"{OS}/🔨️modules"
STORE = f"{MOD}/🏪️store/🦀️.rs"
CHANNEL = f"{MOD}/📡️spr/🧵️channel/🦀️.rs"
CHANNEL_TESTS = f"{MOD}/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs"
VERSION_PIN = f"{OS}/🧫️fixtures/📡️channel/🔖️channel-version.json"
TXN_FIXTURE = f"{OS}/🧫️fixtures/📡️channel/🧾️app-command-transaction.json"
TS = f"{OS}/🟦️.ts"
TS_TESTS = f"{OS}/🧪️tests/🧪️backbone-envelope-io/🟦️.ts"
SDK = f"{MOD}/🔌️plugin/🦀️.rs"
SDK_TXN_TESTS = f"{MOD}/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs"
SDK_LIVE_TESTS = f"{MOD}/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs"
MCP_DISPATCH = f"{MOD}/🌉️mcp/🔀️dispatch/🦀️.rs"
MCP_WORKSPACE = f"{MOD}/🌉️mcp/🏠️workspace/🦀️.rs"
MCP_LONG = f"{MOD}/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs"
MCP_CHANNEL_TESTS = f"{MOD}/🌉️mcp/🐚️channel/🧪️tests/🔬️quick/🦀️.rs"
MCP_CHANNEL = f"{MOD}/🌉️mcp/🐚️channel/🦀️.rs"

H = []


def hunk(path, old, new, count=1):
    H.append((path, old, new, count))


#region store
hunk(STORE, """pub struct GroupMeta {
    pub actor: Option<String>,
    pub description: Option<String>,
    pub coalesce_key: Option<String>,
}""", """pub struct GroupMeta {
    pub actor: Option<String>,
    pub description: Option<String>,
    pub coalesce_key: Option<String>,
    /// 🪪️ The group identity every member's tail edit is stamped with when the caller already owns one
    /// (an agent transaction's `txn_id`, so the gateway's `TransactionUndo{group_id}` names this group);
    /// `None` mints the content-addressed invocation id from the parent edit and the child op fingerprints.
    pub group_id: Option<String>,
}""")
hunk(STORE, """        let invocation_id = mint_invocation_id(&parent_ref.artifact_id, &parent_edit_fingerprint, &child_fingerprints).await;
""", """        let invocation_id = match meta.group_id.clone() {
            Some(group_id) => group_id,
            None => mint_invocation_id(&parent_ref.artifact_id, &parent_edit_fingerprint, &child_fingerprints).await,
        };
""")
#endregion store

#region channel
hunk(CHANNEL, "pub const CHANNEL_VERSION: u32 = 17;", "pub const CHANNEL_VERSION: u32 = 18;")
hunk(CHANNEL, """        prepared_ops: Vec<Vec<u8>>,
        label: String,
        origin: Vec<u8>,
    },
    /// ✅️ Phase-2 commit for one transaction member. CHANNEL_VERSION 9 wire addition.""", """        prepared_ops: Vec<Vec<u8>>,
        label: String,
        origin: Vec<u8>,
        /// 🧩️ The pre-planned form's owned-child op groups (CHANNEL_VERSION 18 trailing addition): the composing
        /// guest's own `ChildEmit` list as one wire pack, exactly as its `AppFrame::Emit.child_ops` preview produced
        /// it; empty when the gesture touches no owned child. The channel carries it opaque.
        prepared_child_ops: Vec<u8>,
    },
    /// ✅️ Phase-2 commit for one transaction member. CHANNEL_VERSION 9 wire addition.""")
hunk(CHANNEL, """        output: Vec<u8>,
        diagnostics: Vec<u8>,
    },
    /// 📝️ Draft-lane pack snapshot (volatile; never enters a Change/Checkpoint).""", """        output: Vec<u8>,
        diagnostics: Vec<u8>,
        /// 🧩️ The owned-child op groups of a previewed agent gesture (CHANNEL_VERSION 18 trailing addition): the
        /// composing guest's `ChildEmit` list as one wire pack, handed back unmodified on
        /// `AppCommand::TransactionPrepare.prepared_child_ops`; empty when the gesture touches no owned child.
        child_ops: Vec<u8>,
    },
    /// 📝️ Draft-lane pack snapshot (volatile; never enters a Change/Checkpoint).""")
hunk(CHANNEL, """                Ok(AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops: ops, label, origin: fields.next().expect("retained transaction origin") })""", """                let origin = fields.next().expect("retained transaction origin");
                Ok(AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops: ops, label, origin, prepared_child_ops: fields.next().expect("retained transaction child op groups") })""")
hunk(CHANNEL, "        17 => Some((3, true, 2)),", "        17 => Some((3, true, 3)),")
hunk(CHANNEL, """            (4, AppCommand::TransactionPrepare { label, .. }) => Some(std::mem::take(label).into_bytes()),
""", """            (4, AppCommand::TransactionPrepare { label, .. }) => Some(std::mem::take(label).into_bytes()),
            (5, AppCommand::TransactionPrepare { prepared_child_ops, .. }) => Some(std::mem::take(prepared_child_ops)),
""")
hunk(CHANNEL, """                    (4, AppCommand::TransactionPrepare { label, .. }) => *label = String::from_utf8(field).expect("decoded transaction label remains valid UTF-8"),
""", """                    (4, AppCommand::TransactionPrepare { label, .. }) => *label = String::from_utf8(field).expect("decoded transaction label remains valid UTF-8"),
                    (5, AppCommand::TransactionPrepare { prepared_child_ops, .. }) => *prepared_child_ops = field,
""")
hunk(CHANNEL, """                    AppCommand::TransactionPrepare { txn_id, mutation_id, payload, prepared_ops, label, origin, .. } => {
                        let mut fields = vec![txn_id.into_bytes(), mutation_id.into_bytes(), payload, label.into_bytes(), origin];""", """                    AppCommand::TransactionPrepare { txn_id, mutation_id, payload, prepared_ops, label, origin, prepared_child_ops, .. } => {
                        let mut fields = vec![txn_id.into_bytes(), mutation_id.into_bytes(), payload, label.into_bytes(), origin, prepared_child_ops];""")
hunk(CHANNEL, """        AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, label, origin } => {""", """        AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, label, origin, prepared_child_ops } => {""")
hunk(CHANNEL, """            out.string(label)?;
            out.bytes(origin)?;
        }""", """            out.string(label)?;
            out.bytes(origin)?;
            out.bytes(prepared_child_ops)?;
        }""")
hunk(CHANNEL, """            label: crate::os_spr::read_str(bytes, &mut pos)?,
            origin: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        18 => AppCommand::TransactionCommit""", """            label: crate::os_spr::read_str(bytes, &mut pos)?,
            origin: crate::os_spr::read_bytes(bytes, &mut pos)?,
            prepared_child_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        18 => AppCommand::TransactionCommit""")
hunk(CHANNEL, """        AppFrame::Emit { in_reply_to, document_ops, config_ops, draft_ops, output, diagnostics } => {
            out.push(10);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, document_ops);
            crate::os_spr::write_bytes(&mut out, config_ops);
            crate::os_spr::write_bytes(&mut out, draft_ops);
            crate::os_spr::write_bytes(&mut out, output);
            crate::os_spr::write_bytes(&mut out, diagnostics);
        }""", """        AppFrame::Emit { in_reply_to, document_ops, config_ops, draft_ops, output, diagnostics, child_ops } => {
            out.push(10);
            crate::os_spr::write_varint_u64(&mut out, *in_reply_to);
            crate::os_spr::write_bytes(&mut out, document_ops);
            crate::os_spr::write_bytes(&mut out, config_ops);
            crate::os_spr::write_bytes(&mut out, draft_ops);
            crate::os_spr::write_bytes(&mut out, output);
            crate::os_spr::write_bytes(&mut out, diagnostics);
            crate::os_spr::write_bytes(&mut out, child_ops);
        }""")
hunk(CHANNEL, """            output: crate::os_spr::read_bytes(bytes, &mut pos)?,
            diagnostics: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        11 => AppFrame::Draft""", """            output: crate::os_spr::read_bytes(bytes, &mut pos)?,
            diagnostics: crate::os_spr::read_bytes(bytes, &mut pos)?,
            child_ops: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },
        11 => AppFrame::Draft""")
#endregion channel

#region channel tests + fixtures
hunk(CHANNEL_TESTS, "label: String::new(), origin: Vec::new() }", "label: String::new(), origin: Vec::new(), prepared_child_ops: Vec::new() }", 3)
hunk(CHANNEL_TESTS, "label: \"l\".to_string(), origin: vec![9] }", "label: \"l\".to_string(), origin: vec![9], prepared_child_ops: Vec::new() }", 3)
hunk(CHANNEL_TESTS, """    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], label: "l".to_string(), origin: vec![9], prepared_child_ops: Vec::new() }).await;
}""", """    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], label: "l".to_string(), origin: vec![9], prepared_child_ops: Vec::new() }).await;
    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 7, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1]], label: "l".to_string(), origin: vec![9], prepared_child_ops: vec![5, 6] }).await;
    assert_command_round_trips(&AppCommand::TransactionPrepare { seq: 8, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: Vec::new(), label: "l".to_string(), origin: vec![9], prepared_child_ops: vec![5, 6] }).await;
}""")
hunk(CHANNEL_TESTS, """    assert_paged_route_admits(&AppCommand::TransactionCommit { seq: 3, txn_id: "t".to_string() }).await;""", """    assert_paged_route_admits(&AppCommand::TransactionPrepare { seq: 7, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1]], label: "l".to_string(), origin: vec![9], prepared_child_ops: vec![5, 6] }).await;
    assert_paged_route_admits(&AppCommand::TransactionCommit { seq: 3, txn_id: "t".to_string() }).await;""")
hunk(CHANNEL_TESTS, """    assert_frame_round_trips(&AppFrame::Emit { in_reply_to: 14, document_ops: vec![1], config_ops: vec![2], draft_ops: vec![3], output: vec![4], diagnostics: vec![5] }).await;""", """    assert_frame_round_trips(&AppFrame::Emit { in_reply_to: 14, document_ops: vec![1], config_ops: vec![2], draft_ops: vec![3], output: vec![4], diagnostics: vec![5], child_ops: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Emit { in_reply_to: 15, document_ops: Vec::new(), config_ops: Vec::new(), draft_ops: Vec::new(), output: vec![4], diagnostics: Vec::new(), child_ops: vec![6, 7, 8] }).await;""")
hunk(CHANNEL_TESTS, """        ("Emit", AppFrame::Emit { in_reply_to: 1, document_ops: vec![1], config_ops: vec![], draft_ops: vec![], output: vec![2], diagnostics: vec![] }),""", """        ("Emit", AppFrame::Emit { in_reply_to: 1, document_ops: vec![1], config_ops: vec![], draft_ops: vec![], output: vec![2], diagnostics: vec![], child_ops: vec![] }),""")
hunk(CHANNEL_TESTS, """        ("TransactionPreparePrePlanned", AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], label: "l".to_string(), origin: vec![9], prepared_child_ops: Vec::new() }),""", """        ("TransactionPreparePrePlanned", AppCommand::TransactionPrepare { seq: 2, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1], vec![2, 2]], label: "l".to_string(), origin: vec![9], prepared_child_ops: Vec::new() }),
        ("TransactionPreparePrePlannedChildren", AppCommand::TransactionPrepare { seq: 7, txn_id: "t".to_string(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![vec![1]], label: "l".to_string(), origin: vec![9], prepared_child_ops: vec![5, 6] }),""")
hunk(CHANNEL_TESTS, """        "TransactionPrepareOwner" => "11010174016d0109000000",
        "TransactionPreparePrePlanned" => "110201740000020101020202016c0109",""", """        "TransactionPrepareOwner" => "11010174016d010900000000",
        "TransactionPreparePrePlanned" => "110201740000020101020202016c010900",
        "TransactionPreparePrePlannedChildren" => "110701740000010101016c0109020506",""")
hunk(CHANNEL_TESTS, """        "Emit" => "0a0101010000010200",""", """        "Emit" => "0a010101000001020000",""")
hunk(CHANNEL_TESTS, """    assert_eq!(command_vectors.len(), 6, "🧾️app-command-transaction.json vector count changed");""", """    assert_eq!(command_vectors.len(), 7, "🧾️app-command-transaction.json vector count changed");""")
hunk(VERSION_PIN, '"channelVersion": 17', '"channelVersion": 18')
hunk(TXN_FIXTURE, """  "TransactionPrepareOwner": "11010174016d0109000000",
  "TransactionPreparePrePlanned": "110201740000020101020202016c0109",""", """  "TransactionPrepareOwner": "11010174016d010900000000",
  "TransactionPreparePrePlanned": "110201740000020101020202016c010900",
  "TransactionPreparePrePlannedChildren": "110701740000010101016c0109020506",""")
#endregion channel tests + fixtures

#region ts twin
hunk(TS, """        readonly prepared_ops: readonly (readonly number[])[];
        readonly label: string;
        readonly origin: readonly number[];
      };
    }""", """        readonly prepared_ops: readonly (readonly number[])[];
        readonly label: string;
        readonly origin: readonly number[];
        readonly prepared_child_ops: readonly number[];
      };
    }""")
hunk(TS, """  | { readonly Emit: { readonly in_reply_to: number; readonly document_ops: readonly number[]; readonly config_ops: readonly number[]; readonly draft_ops: readonly number[]; readonly output: readonly number[]; readonly diagnostics: readonly number[] } }""", """  | { readonly Emit: { readonly in_reply_to: number; readonly document_ops: readonly number[]; readonly config_ops: readonly number[]; readonly draft_ops: readonly number[]; readonly output: readonly number[]; readonly diagnostics: readonly number[]; readonly child_ops: readonly number[] } }""")
hunk(TS, """    writeStr(out, cmd.transactionPrepare.label);
    writeBytes(out, cmd.transactionPrepare.origin);""", """    writeStr(out, cmd.transactionPrepare.label);
    writeBytes(out, cmd.transactionPrepare.origin);
    writeBytes(out, cmd.transactionPrepare.prepared_child_ops);""")
hunk(TS, """      const origin = readBytes(bytes, pos);
      return { transactionPrepare: { seq, txn_id, mutation_id, payload, prepared_ops, label, origin } };""", """      const origin = readBytes(bytes, pos);
      const prepared_child_ops = readBytes(bytes, pos);
      return { transactionPrepare: { seq, txn_id, mutation_id, payload, prepared_ops, label, origin, prepared_child_ops } };""")
hunk(TS, """    writeBytes(out, frame.Emit.output);
    writeBytes(out, frame.Emit.diagnostics);""", """    writeBytes(out, frame.Emit.output);
    writeBytes(out, frame.Emit.diagnostics);
    writeBytes(out, frame.Emit.child_ops);""")
hunk(TS, """      return { Emit: { in_reply_to: readVarintU64(bytes, pos), document_ops: readBytes(bytes, pos), config_ops: readBytes(bytes, pos), draft_ops: readBytes(bytes, pos), output: readBytes(bytes, pos), diagnostics: readBytes(bytes, pos) } };""", """      return { Emit: { in_reply_to: readVarintU64(bytes, pos), document_ops: readBytes(bytes, pos), config_ops: readBytes(bytes, pos), draft_ops: readBytes(bytes, pos), output: readBytes(bytes, pos), diagnostics: readBytes(bytes, pos), child_ops: readBytes(bytes, pos) } };""")
hunk(TS, """      transactionPrepare: { seq: this.nextSeq(), txn_id: txnId, mutation_id: mutationId, payload: Array.from(payload), prepared_ops: [], label: "", origin: [] },""", """      transactionPrepare: { seq: this.nextSeq(), txn_id: txnId, mutation_id: mutationId, payload: Array.from(payload), prepared_ops: [], label: "", origin: [], prepared_child_ops: [] },""")
hunk(TS, """        prepared_ops: preparedOps.map((op) => Array.from(op)),
        label,
        origin: Array.from(origin),
      },""", """        prepared_ops: preparedOps.map((op) => Array.from(op)),
        label,
        origin: Array.from(origin),
        prepared_child_ops: [],
      },""")
hunk(TS, "export const APP_CHANNEL_VERSION = 17;", "export const APP_CHANNEL_VERSION = 18;")
hunk(TS_TESTS, """      { transactionPrepare: { seq: 21, txn_id: "txn-1", mutation_id: "s.demo#kind", payload: [1, 2], prepared_ops: [], label: "", origin: [] } },
      { transactionPrepare: { seq: 22, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "step-1", origin: [9] } },""", """      { transactionPrepare: { seq: 21, txn_id: "txn-1", mutation_id: "s.demo#kind", payload: [1, 2], prepared_ops: [], label: "", origin: [], prepared_child_ops: [] } },
      { transactionPrepare: { seq: 22, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "step-1", origin: [9], prepared_child_ops: [] } },
      { transactionPrepare: { seq: 36, txn_id: "txn-2", mutation_id: "", payload: [], prepared_ops: [], label: "step-2", origin: [9], prepared_child_ops: [5, 6] } },""")
hunk(TS_TESTS, """      { Emit: { in_reply_to: 11, document_ops: [1], config_ops: [2], draft_ops: [3], output: [4], diagnostics: [5] } },""", """      { Emit: { in_reply_to: 11, document_ops: [1], config_ops: [2], draft_ops: [3], output: [4], diagnostics: [5], child_ops: [] } },
      { Emit: { in_reply_to: 11, document_ops: [], config_ops: [], draft_ops: [], output: [4], diagnostics: [], child_ops: [6, 7, 8] } },""")
hunk(TS_TESTS, """        ["Emit", { Emit: { in_reply_to: 1, document_ops: [1], config_ops: [], draft_ops: [], output: [2], diagnostics: [] } }],""", """        ["Emit", { Emit: { in_reply_to: 1, document_ops: [1], config_ops: [], draft_ops: [], output: [2], diagnostics: [], child_ops: [] } }],""")
hunk(TS_TESTS, """        Emit: "0a0101010000010200",""", """        Emit: "0a010101000001020000",""")
hunk(TS_TESTS, """        TransactionPrepareOwner: { transactionPrepare: { seq: 1, txn_id: "t", mutation_id: "m", payload: [9], prepared_ops: [], label: "", origin: [] } },
        TransactionPreparePrePlanned: { transactionPrepare: { seq: 2, txn_id: "t", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "l", origin: [9] } },""", """        TransactionPrepareOwner: { transactionPrepare: { seq: 1, txn_id: "t", mutation_id: "m", payload: [9], prepared_ops: [], label: "", origin: [], prepared_child_ops: [] } },
        TransactionPreparePrePlanned: { transactionPrepare: { seq: 2, txn_id: "t", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "l", origin: [9], prepared_child_ops: [] } },
        TransactionPreparePrePlannedChildren: { transactionPrepare: { seq: 7, txn_id: "t", mutation_id: "", payload: [], prepared_ops: [[1]], label: "l", origin: [9], prepared_child_ops: [5, 6] } },""")
#endregion ts twin

#region sdk
hunk(SDK, """    #[derive(Clone, Debug, PartialEq, Serialize, ToValue)]
    pub struct ChildEmit {""", """    #[derive(Clone, Debug, PartialEq, Serialize, ToValue, FromValue)]
    pub struct ChildEmit {""")
hunk(SDK, """    impl ChildEmit {
        pub(crate) fn close_one(""", """    impl ChildEmit {
        /// 🧩️ The wire form of one agent gesture's owned-child share (`AppFrame::Emit.child_ops` →
        /// gateway → `AppCommand::TransactionPrepare.prepared_child_ops`): the exact `ChildEmit` list as one
        /// wire pack, empty bytes when the gesture touches no owned child, so a childless transaction's wire
        /// is unchanged.
        pub fn encode_groups(children: &[ChildEmit]) -> Vec<u8> {
            if children.is_empty() {
                return Vec::new();
            }
            store::pack_rt::encode_wire_value(&store::ToValue::to_value(&children.to_vec()))
        }

        /// 🧩️ Decodes [`ChildEmit::encode_groups`]; a pack that is not a `ChildEmit` list is refused by name.
        pub fn decode_groups(bytes: &[u8]) -> Result<Vec<ChildEmit>, Fault> {
            if bytes.is_empty() {
                return Ok(Vec::new());
            }
            let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("transaction.child-groups-malformed"), format!("owned-child op groups did not decode: {error}")))?;
            <Vec<ChildEmit> as store::FromValue>::from_value(value).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("transaction.child-groups-malformed"), format!("owned-child op groups are not a ChildEmit list: {error}")))
        }

        pub(crate) fn close_one(""")
hunk(SDK, """    pub(crate) struct PendingTransaction<Op> {
        pub(crate) txn_id: String,
        pub(crate) ops: Vec<Op>,
        pub(crate) label: String,
        pub(crate) origin: protocol::MutationOrigin,
        pub(crate) base_generation: u64,
    }""", """    pub(crate) struct PendingTransaction<Op> {
        pub(crate) txn_id: String,
        pub(crate) ops: Vec<Op>,
        pub(crate) children: Vec<ChildEmit>,
        pub(crate) label: String,
        pub(crate) origin: protocol::MutationOrigin,
        pub(crate) base_generation: u64,
    }

    /// 🧾️ The op packs one dispatch produced, per lane, as the channel's `AppFrame::Emit` carries them:
    /// `document`/`config`/`draft` are `protocol::encode_ops_vec` packs, `children` the owned-child share
    /// ([`ChildEmit::encode_groups`]), filled only by the agent preview (an applying dispatch has already
    /// committed its children in-guest).
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct EmitWire {
        pub document: Vec<u8>,
        pub config: Vec<u8>,
        pub draft: Vec<u8>,
        pub children: Vec<u8>,
    }""")
hunk(SDK, """        /// 🧾 Drains the last Emit op packs captured during `handle_command_frame` (PureCommand path).
        async fn take_last_emit_wire(&mut self) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)>;""", """        /// 🧾 Drains the last Emit op packs captured during `handle_command_frame` (PureCommand path).
        async fn take_last_emit_wire(&mut self) -> Option<EmitWire>;""")
hunk(SDK, """        async fn transaction_prepare(&mut self, txn_id: &str, mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], label: &str, origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome;""", """        /// `prepared_child_ops` is the pre-planned form's owned-child share ([`ChildEmit::encode_groups`]).
        async fn transaction_prepare(&mut self, txn_id: &str, mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], prepared_child_ops: &[u8], label: &str, origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome;""")
hunk(SDK, """        /// 🧾 Last Emit op packs produced by `dispatch_emit` — consumed by `AppCommand::PureCommand`.
        last_emit_wire: Option<(Vec<u8>, Vec<u8>, Vec<u8>)>,""", """        /// 🧾 Last Emit op packs produced by `dispatch_emit` — consumed by `AppCommand::PureCommand`.
        last_emit_wire: Option<EmitWire>,""")
hunk(SDK, """            self.last_emit_wire = Some((protocol::encode_ops_vec(&artifact_op_bytes), protocol::encode_ops_vec(&config_op_bytes), protocol::encode_ops_vec(&draft_op_bytes)));

            // 📝️ Draft lane""", """            self.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: Vec::new() });

            // 📝️ Draft lane""")
hunk(SDK, """            let uncarried = [
                ("owned children", !emit.child_emits.is_empty()),
                ("a whole-document replacement\"""", """            let uncarried = [
                ("a whole-document replacement\"""")
hunk(SDK, """            if A::ROLE == AppRole::Viewer && !emit.artifact_mutations.is_empty() {
                return Err(viewer_read_only_fault(&address.action_id));
            }
            let mut artifact_op_bytes = Vec::with_capacity(emit.artifact_mutations.len());
            for op in emit.artifact_mutations.iter() {
                artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
            }""", """            if A::ROLE == AppRole::Viewer && (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty()) {
                return Err(viewer_read_only_fault(&address.action_id));
            }
            let mut artifact_op_bytes = Vec::with_capacity(emit.artifact_mutations.len());
            for op in emit.artifact_mutations.iter() {
                artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
            }
            for child in emit.child_emits.iter() {
                if self.children.get(&(child.slot.clone(), child.child_id.clone())).is_none() {
                    return Err(Fault::new(
                        FaultOrigin::Framework,
                        FaultCode::new("interactive-job.agent-lane-child-missing"),
                        format!("action '{}' edits owned child {:?} in slot {:?}, which this instance does not hold", address.action_id, child.child_id, child.slot),
                    ));
                }
            }
            let child_op_bytes = ChildEmit::encode_groups(&emit.child_emits);""")
hunk(SDK, """            let priced = artifact_op_bytes.iter().chain(config_op_bytes.iter()).chain(draft_op_bytes.iter()).map(Vec::len).sum::<usize>();
            let contract = proof.contract();
            if priced > contract.max_output_bytes {
                return Err(Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.preview-output"),
                    format!("previewed action '{}' produced {priced} op byte(s); its exact output cap is {}", address.action_id, contract.max_output_bytes),
                ));
            }
            self.last_emit_wire = Some((protocol::encode_ops_vec(&artifact_op_bytes), protocol::encode_ops_vec(&config_op_bytes), protocol::encode_ops_vec(&draft_op_bytes)));""", """            let priced = artifact_op_bytes.iter().chain(config_op_bytes.iter()).chain(draft_op_bytes.iter()).map(Vec::len).sum::<usize>();
            let contract = proof.contract();
            if priced > contract.max_output_bytes {
                return Err(Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.preview-output"),
                    format!("previewed action '{}' produced {priced} op byte(s); its exact output cap is {}", address.action_id, contract.max_output_bytes),
                ));
            }
            self.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: child_op_bytes });""")
hunk(SDK, """                ("draftOps".into(), DslValue::String(draft_op_bytes.len().to_string())),
                ("opBytes".into(), DslValue::String(priced.to_string())),""", """                ("draftOps".into(), DslValue::String(draft_op_bytes.len().to_string())),
                ("childGroups".into(), DslValue::String(emit.child_emits.len().to_string())),
                ("opBytes".into(), DslValue::String(priced.to_string())),""")
hunk(SDK, """            if let Some(transaction) = self.pending_transaction.as_mut() {
                if let Some(operation) = transaction.ops.pop() {
                    drop(operation);
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            }""", """            if let Some(transaction) = self.pending_transaction.as_mut() {
                if let Some(operation) = transaction.ops.pop() {
                    drop(operation);
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                if let Some(child) = transaction.children.last_mut() {
                    let step = child.close_one(1, maximum_bytes);
                    if step == PluginCloseStep::Complete {
                        transaction.children.pop();
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                    return step;
                }
            }""")
hunk(SDK, """            if let Some(wire) = self.last_emit_wire.as_mut() {
                for bytes in [&mut wire.0, &mut wire.1, &mut wire.2] {""", """            if let Some(wire) = self.last_emit_wire.as_mut() {
                for bytes in [&mut wire.document, &mut wire.config, &mut wire.draft, &mut wire.children] {""")
hunk(SDK, """        async fn take_last_emit_wire(&mut self) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)> {
            self.last_emit_wire.take()
        }""", """        async fn take_last_emit_wire(&mut self) -> Option<EmitWire> {
            self.last_emit_wire.take()
        }""")
hunk(SDK, """        async fn transaction_prepare(&mut self, txn_id: &str, mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], label: &str, origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome {
            if self.pending_transaction.is_some() {
                return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(Self::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", "a transaction is already pending on this instance")) };
            }""", """        async fn transaction_prepare(&mut self, txn_id: &str, mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], prepared_child_ops: &[u8], label: &str, origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome {
            if self.pending_transaction.is_some() {
                return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(Self::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", "a transaction is already pending on this instance")) };
            }
            let children = match ChildEmit::decode_groups(prepared_child_ops) {
                Ok(children) => children,
                Err(fault) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(fault) },
            };
            if let Some(missing) = children.iter().find(|child| self.children.get(&(child.slot.clone(), child.child_id.clone())).is_none()) {
                return TransactionPrepareOutcome {
                    foreign: Vec::new(),
                    rejection: Some(Self::transaction_fault(FaultOrigin::Plugin, "transaction.member-rejected", format!("owned child {:?} in slot {:?} is not held by this instance", missing.child_id, missing.slot))),
                };
            }""")
hunk(SDK, """            let (ops, resolved_label, resolved_origin): (Vec<A::Mutation>, String, protocol::MutationOrigin) = if !prepared_ops.is_empty() {""", """            let (ops, resolved_label, resolved_origin): (Vec<A::Mutation>, String, protocol::MutationOrigin) = if !prepared_ops.is_empty() || !children.is_empty() {""")
hunk(SDK, """            self.pending_transaction = Some(PendingTransaction { txn_id: txn_id.to_string(), ops, label: resolved_label, origin: resolved_origin, base_generation });""", """            self.pending_transaction = Some(PendingTransaction { txn_id: txn_id.to_string(), ops, children, label: resolved_label, origin: resolved_origin, base_generation });""")
hunk(SDK, """            let PendingTransaction { txn_id, ops, label, origin, .. } = pending;
            let description = if label.is_empty() { None } else { Some(label) };
            self.store.set_local_actor_id(Some(meta.actor.clone())).map_err(|error| error.into_fault())?;""", """            let PendingTransaction { txn_id, ops, children, label, origin, .. } = pending;
            let description = if label.is_empty() { None } else { Some(label) };
            if !children.is_empty() {
                return self.commit_transaction_group(&txn_id, ops, children, description, origin, meta).await;
            }
            self.store.set_local_actor_id(Some(meta.actor.clone())).map_err(|error| error.into_fault())?;""")
hunk(SDK, """        async fn transaction_undo(&mut self, group_id: &str) -> Result<(), Fault> {
            if self.store.tail_group_id().await.as_deref() != Some(group_id) {""", """        async fn transaction_undo(&mut self, group_id: &str) -> Result<(), Fault> {
            if !self.children.is_empty() {
                return self.transaction_group_history("undo", group_id).await;
            }
            if self.store.tail_group_id().await.as_deref() != Some(group_id) {""")
hunk(SDK, """        async fn transaction_redo(&mut self, group_id: &str) -> Result<(), Fault> {
            match self.store.redo_tail().await {""", """        async fn transaction_redo(&mut self, group_id: &str) -> Result<(), Fault> {
            if !self.children.is_empty() {
                return self.transaction_group_history("redo", group_id).await;
            }
            match self.store.redo_tail().await {""")
hunk(SDK, """        #[allow(clippy::too_many_arguments)]
        async fn dispatch_emit_group(
            &mut self,
            verb: &str,
            artifact_mutations: &[A::Mutation],
            child_emits: &[ChildEmit],
            description: &Option<String>,
            effects: Vec<Effect>,
            events: Vec<AppEvent>,
            ui_scope: UiDirtyScope,
            config_edit_id: Option<String>,
            meta: &ActionMeta,
        ) -> Result<InvocationResult, Fault> {""", """        /// @emoji 🧩️ Commits an agent transaction that carries owned-child op groups as ONE composite gesture
        /// (`dispatch_emit_group`, the same path the shell lane's child-group verbs take) whose group identity is
        /// `txn_id` on every member it touches, so the gateway's `TransactionUndo{group_id: txn_id}` moves them all.
        /// The parent's tail edit (when the group touched the parent) carries the prepared origin like a solitary
        /// commit does. Returns the parent's edit id, else the first touched child's.
        async fn commit_transaction_group(&mut self, txn_id: &str, ops: Vec<A::Mutation>, children: Vec<ChildEmit>, description: Option<String>, origin: protocol::MutationOrigin, meta: &ActionMeta) -> Result<String, Fault> {
            let parent_edits_before = self.store.envelope().vcs.edits.len();
            let result = self
                .dispatch_emit_group(&format!("transaction:{txn_id}"), &ops, &children, &description, Vec::new(), Vec::new(), UiDirtyScope::Full, None, meta, Some(txn_id.to_string()))
                .await
                .map_err(|fault| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", fault.message))?;
            if self.store.envelope().vcs.edits.len() > parent_edits_before {
                self.store.stamp_tail_origin(origin).await.map_err(|error| Self::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
                return Ok(self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default());
            }
            Ok(result.inverse_group.member_edits.first().map(|edit| edit.edit_id.clone()).unwrap_or_default())
        }

        /// @emoji ↩️ `transaction_undo`/`transaction_redo` for a composing instance: moves every member (parent and
        /// owned children) whose tail carries `group_id` through `CompositionCoordinator::undo_group`/`redo_group`,
        /// republishing each moved child's content like the shell lane's group history does. A group no member
        /// carries is refused by name, exactly like the solitary path.
        async fn transaction_group_history(&mut self, action: &str, group_id: &str) -> Result<(), Fault> {
            self.admit_child_content_publication_span(self.children.len())?;
            let parent_id = self.store.envelope().id.clone();
            let parent_dialect: ArtifactDialect = A::DIALECT.into();
            if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
                return Err(plugin_sdk_fault("composition history requires the parent's exact declared dialect"));
            }
            let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
            let child_refs: Vec<ArtifactRef> = self.children.entries_physical().map(|entry| entry.reference.clone()).collect();
            let mut members: Vec<(&ArtifactRef, &mut M)> = Vec::with_capacity(child_refs.len());
            for (reference, entry) in child_refs.iter().zip(self.children.entries_mut_physical()) {
                members.push((reference, &mut entry.member));
            }
            let report = if action == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut members, group_id).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.store, &mut members, group_id).await };
            drop(members);
            self.cache = None;
            for (reference, _) in report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id) {
                let Some((slot, child_id)) = self.children.entries().find(|entry| entry.reference.artifact_id == reference.artifact_id).map(|entry| (entry.owner.slot.clone(), entry.reference.artifact_id.clone())) else {
                    return Err(plugin_sdk_fault("group history moved a child without exact immutable-root authority"));
                };
                let publication_generation = self.admit_child_content_publication()?;
                self.publish_child_content_member(publication_generation, &slot, &child_id).await?;
            }
            if report.undone.is_empty() {
                let skipped = report.skipped.iter().map(|(reference, error)| format!("{} ({error})", reference.artifact_id)).collect::<Vec<_>>().join(", ");
                return Err(plugin_sdk_fault(format!("transaction_{action}: no member of this instance carries group {group_id:?} at its {} tail{}", if action == "undo" { "applied" } else { "redo" }, if skipped.is_empty() { String::new() } else { format!(" (skipped: {skipped})") })));
            }
            self.record_command(action, ActionKind::History, None, None, None, None);
            Ok(())
        }

        #[allow(clippy::too_many_arguments)]
        async fn dispatch_emit_group(
            &mut self,
            verb: &str,
            artifact_mutations: &[A::Mutation],
            child_emits: &[ChildEmit],
            description: &Option<String>,
            effects: Vec<Effect>,
            events: Vec<AppEvent>,
            ui_scope: UiDirtyScope,
            config_edit_id: Option<String>,
            meta: &ActionMeta,
            group_id: Option<String>,
        ) -> Result<InvocationResult, Fault> {""")
hunk(SDK, """            let group_meta = GroupMeta { actor: Some(meta.actor.clone()), description: (*description).clone(), coalesce_key: None };""", """            let group_meta = GroupMeta { actor: Some(meta.actor.clone()), description: (*description).clone(), coalesce_key: None, group_id };""")
hunk(SDK, """                let result = self.dispatch_emit_group(verb, &artifact_mutations, &child_emits, &description, effects, events, ui_scope, config_edit_id, meta).await?;""", """                let result = self.dispatch_emit_group(verb, &artifact_mutations, &child_emits, &description, effects, events, ui_scope, config_edit_id, meta, None).await?;""")
hunk(SDK, """            let result = self.dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, &pending.description, Vec::new(), Vec::new(), UiDirtyScope::None, None, &mounted.meta).await;""", """            let result = self.dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, &pending.description, Vec::new(), Vec::new(), UiDirtyScope::None, None, &mounted.meta, None).await;""")
hunk(SDK, """                let (document_ops, config_ops, draft_ops) = emit_wire;
                frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics) });""", """                let EmitWire { document: document_ops, config: config_ops, draft: draft_ops, children: child_ops } = emit_wire;
                frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics), child_ops });""", 2)
hunk(SDK, """                    let (document_ops, config_ops, draft_ops) = emit_wire;
                    frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics) });""", """                    let EmitWire { document: document_ops, config: config_ops, draft: draft_ops, children: child_ops } = emit_wire;
                    frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics), child_ops });""")
hunk(SDK, """                        Ok((result, (document_ops, config_ops, draft_ops))) => {
                            mutated = true;
                            let output = encode_wire_serialized(&result.output);
                            let diagnostics = encode_wire_serialized(&result.diagnostics);
                            frames.push(protocol::AppFrame::Emit { in_reply_to: seq, document_ops, config_ops, draft_ops, output, diagnostics });""", """                        Ok((result, EmitWire { document: document_ops, config: config_ops, draft: draft_ops, children: child_ops })) => {
                            mutated = true;
                            let output = encode_wire_serialized(&result.output);
                            let diagnostics = encode_wire_serialized(&result.diagnostics);
                            frames.push(protocol::AppFrame::Emit { in_reply_to: seq, document_ops, config_ops, draft_ops, output, diagnostics, child_ops });""")
hunk(SDK, """                protocol::AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, label, origin } => {""", """                protocol::AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, label, origin, prepared_child_ops } => {""")
hunk(SDK, """                                Ok(resolve_ready(instance.app.transaction_prepare(&txn_id, &mutation_id, &payload, &prepared_ops, &label, decoded_origin)))""", """                                Ok(resolve_ready(instance.app.transaction_prepare(&txn_id, &mutation_id, &payload, &prepared_ops, &prepared_child_ops, &label, decoded_origin)))""")
hunk(SDK, """            let outcome = app.transaction_prepare(txn_id, "", &[], &prepared_ops, label, Some(origin.clone())).await;""", """            let outcome = app.transaction_prepare(txn_id, "", &[], &prepared_ops, &[], label, Some(origin.clone())).await;""")
hunk(SDK, """            let outcome = app.transaction_prepare(txn_id, "", &[], &prepared_ops, label, Some(protocol::MutationOrigin::Owner)).await;""", """            let outcome = app.transaction_prepare(txn_id, "", &[], &prepared_ops, &[], label, Some(protocol::MutationOrigin::Owner)).await;""")
hunk(SDK_LIVE_TESTS, """prepared_ops: Vec::new(), label: String::new(), origin: Vec::new() }).await;""", """prepared_ops: Vec::new(), label: String::new(), origin: Vec::new(), prepared_child_ops: Vec::new() }).await;""")
hunk(SDK_LIVE_TESTS, """prepared_ops: vec![operation], label: "receipt fixture".into(), origin: Vec::new() }).await;""", """prepared_ops: vec![operation], label: "receipt fixture".into(), origin: Vec::new(), prepared_child_ops: Vec::new() }).await;""")
#endregion sdk

#region mcp
hunk(MCP_DISPATCH, """#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub struct PreparedOps {
    pub document: Vec<Vec<u8>>,
    pub config: Vec<Vec<u8>>,
    pub draft: Vec<Vec<u8>>,
}

impl PreparedOps {
    fn op_counts(&self) -> serde_json::Value {
        serde_json::json!({ "document": self.document.len(), "config": self.config.len(), "draft": self.draft.len() })
    }

    /// 📭️ Whether the preview produced no operation in any lane: nothing to prepare, commit or undo.
    pub fn is_empty(&self) -> bool {
        self.document.is_empty() && self.config.is_empty() && self.draft.is_empty()
    }
}""", """/// `children` is the composing guest's owned-child share of the same gesture (`AppFrame::Emit.child_ops`, the guest's
/// `ChildEmit` wire pack) — carried from `Emit` to `TransactionPrepare` byte for byte, never decoded here; empty when the
/// gesture touches no owned child. The shell route's deferred plan never produces it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub struct PreparedOps {
    pub document: Vec<Vec<u8>>,
    pub config: Vec<Vec<u8>>,
    pub draft: Vec<Vec<u8>>,
    #[serde(default)]
    #[value(default)]
    pub children: Vec<u8>,
}

impl PreparedOps {
    fn op_counts(&self) -> serde_json::Value {
        serde_json::json!({ "document": self.document.len(), "config": self.config.len(), "draft": self.draft.len(), "childOpBytes": self.children.len() })
    }

    /// 📭️ Whether the preview produced no operation in any lane and touched no owned child: nothing to prepare,
    /// commit or undo.
    pub fn is_empty(&self) -> bool {
        self.document.is_empty() && self.config.is_empty() && self.draft.is_empty() && self.children.is_empty()
    }
}""")
hunk(MCP_DISPATCH, """                AppFrame::Emit { ops: PreparedOps { document: vec![payload], config: Vec::new(), draft: Vec::new() }, warnings: Vec::new() }""", """                AppFrame::Emit { ops: PreparedOps { document: vec![payload], config: Vec::new(), draft: Vec::new(), children: Vec::new() }, warnings: Vec::new() }""")
hunk(MCP_WORKSPACE, """                        store::AppFrame::Emit { document_ops, config_ops, draft_ops, .. } => {
                            AppFrame::Emit { ops: PreparedOps { document: ops_pack_lane(document_ops)?, config: ops_pack_lane(config_ops)?, draft: ops_pack_lane(draft_ops)? }, warnings: Vec::new() }
                        }""", """                        store::AppFrame::Emit { document_ops, config_ops, draft_ops, child_ops, .. } => {
                            AppFrame::Emit { ops: PreparedOps { document: ops_pack_lane(document_ops)?, config: ops_pack_lane(config_ops)?, draft: ops_pack_lane(draft_ops)?, children: child_ops }, warnings: Vec::new() }
                        }""")
hunk(MCP_WORKSPACE, """                        store::AppCommand::TransactionPrepare { seq: 0, txn_id: txn_id.clone(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: ops.document, label, origin: Vec::new() },""", """                        store::AppCommand::TransactionPrepare { seq: 0, txn_id: txn_id.clone(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: ops.document, label, origin: Vec::new(), prepared_child_ops: ops.children },""")
hunk(MCP_LONG, """                ops: PreparedOps { document: ops.document.clone(), config: Vec::new(), draft: Vec::new() },""", """                ops: PreparedOps { document: ops.document.clone(), config: Vec::new(), draft: Vec::new(), children: ops.children.clone() },""")
hunk(MCP_CHANNEL, """            Ok(AppFrame::Emit { ops: PreparedOps { document: decode_lane(ops.get("document"))?, config: decode_lane(ops.get("config"))?, draft: decode_lane(ops.get("draft"))? }""", """            Ok(AppFrame::Emit { ops: PreparedOps { document: decode_lane(ops.get("document"))?, config: decode_lane(ops.get("config"))?, draft: decode_lane(ops.get("draft"))?, children: Vec::new() }""")
hunk(MCP_CHANNEL_TESTS, """Ok(AppFrame::Emit { ops: PreparedOps { document: vec![vec![1, 2, 3]], config: Vec::new(), draft: Vec::new() }""", """Ok(AppFrame::Emit { ops: PreparedOps { document: vec![vec![1, 2, 3]], config: Vec::new(), draft: Vec::new(), children: Vec::new() }""")
hunk(f"{MOD}/🌉️mcp/🐚️channel/🧪️tests/🔬️quick/🦀️.rs", """ops: PreparedOps { document: vec![vec![1, 2, 3]], config: Vec::new(), draft: Vec::new() }, label: "append paragraph\"""", """ops: PreparedOps { document: vec![vec![1, 2, 3]], config: Vec::new(), draft: Vec::new(), children: Vec::new() }, label: "append paragraph\"""")
#endregion mcp


#region laws
SDK_CONTRACT_TESTS = f"{MOD}/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
MCP_DISPATCH_TESTS = f"{MOD}/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs"
hunk(SDK_CONTRACT_TESTS, """    #[semio_framework_async_macros::async_test]
    async fn a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames() {""", """    #[semio_framework_async_macros::async_test]
    async fn an_agent_transaction_carries_owned_child_op_groups_and_commits_undoes_and_redoes_them_as_one_group() {
        let mut app = contract_composed_app().await;
        app.register_child("slot", "child-1", test_child_dialect().await, new_test_child("child-1").await.expect("construct child")).await.expect("register child");
        let children = vec![ChildEmit::of::<TestSnapshot, _>("slot", "child-1", &[TestMutation::SetCount(SetCount { value: 5 })])];
        let wire = ChildEmit::encode_groups(&children);
        assert!(!wire.is_empty(), "a gesture that touches a child carries its group");
        assert_eq!(ChildEmit::decode_groups(&wire).expect("the wire pack decodes"), children, "the group survives the wire byte for byte");
        assert!(ChildEmit::encode_groups(&[]).is_empty(), "a childless gesture leaves the wire unchanged");
        assert_eq!(ChildEmit::decode_groups(&[7, 7, 7]).expect_err("rubbish is refused").code.as_str(), "transaction.child-groups-malformed");

        let parent_op = <TestMutation as ::protocol::OpBinary>::encode_op(&TestMutation::SetLabel(SetLabel { value: "agent".into() })).expect("encode parent op");
        let outcome = app.transaction_prepare("txn-agent-1", "", &[], &[parent_op], &wire, "agent composite", Some(protocol::MutationOrigin::Owner)).await;
        assert!(outcome.rejection.is_none(), "prepare admits a held child: {:?}", outcome.rejection.as_ref().map(|fault| &fault.message));
        let edit_id = app.transaction_commit("txn-agent-1", &meta()).await.expect("commit the composite transaction");
        assert!(!edit_id.is_empty());
        macro_rules! child_count {
            ($app:expr) => {{
                let TestMembers::Child(child_store) = &mut $app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("child stays registered").member;
                child_store.snapshot().expect("child snapshot").count
            }};
        }
        assert_eq!(child_count!(app), 5, "the child applied the agent's op");
        assert_eq!(app.test_snapshot().await.label, "agent", "the parent applied the agent's op");
        assert_eq!(app.store.tail_group_id().await.as_deref(), Some("txn-agent-1"), "the group identity is the transaction id");

        app.transaction_undo("txn-agent-1").await.expect("undo the whole group");
        assert_eq!(child_count!(app), 0, "undo reverts the child");
        assert_ne!(app.test_snapshot().await.label, "agent", "undo reverts the parent");
        app.transaction_redo("txn-agent-1").await.expect("redo the whole group");
        assert_eq!(child_count!(app), 5, "redo reapplies the child");
        assert_eq!(app.test_snapshot().await.label, "agent", "redo reapplies the parent");
        assert!(app.transaction_undo("txn-someone-else").await.is_err(), "a group no member carries is refused by name");

        let only_child = ChildEmit::encode_groups(&[ChildEmit::of::<TestSnapshot, _>("slot", "child-1", &[TestMutation::SetCount(SetCount { value: 8 })])]);
        let outcome = app.transaction_prepare("txn-agent-2", "", &[], &[], &only_child, "", Some(protocol::MutationOrigin::Owner)).await;
        assert!(outcome.rejection.is_none(), "a children-only transaction is a pre-planned transaction");
        app.transaction_commit("txn-agent-2", &meta()).await.expect("commit the children-only transaction");
        assert_eq!(child_count!(app), 8);
        app.transaction_undo("txn-agent-2").await.expect("undo the children-only group");
        assert_eq!(child_count!(app), 5);

        let stray = ChildEmit::encode_groups(&[ChildEmit::of::<TestSnapshot, _>("slot", "ghost", &[TestMutation::SetCount(SetCount { value: 1 })])]);
        let refused = app.transaction_prepare("txn-agent-3", "", &[], &[], &stray, "", Some(protocol::MutationOrigin::Owner)).await;
        assert_eq!(refused.rejection.expect("a child this instance does not hold is refused").code.as_str(), "transaction.member-rejected");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames() {""")
hunk(MCP_DISPATCH, """    force_undo_fails: bool,
    empty_preview: bool,
}""", """    force_undo_fails: bool,
    empty_preview: bool,
    child_preview: Option<Vec<u8>>,
}""")
hunk(MCP_DISPATCH, """force_commit_fault: None, force_undo_fails: false, empty_preview: false }""", """force_commit_fault: None, force_undo_fails: false, empty_preview: false, child_preview: None }""")
hunk(MCP_DISPATCH, """            AppCommand::PureCommand { .. } if self.empty_preview => AppFrame::Emit { ops: PreparedOps::default(), warnings: Vec::new() },""", """            AppCommand::PureCommand { .. } if self.empty_preview => AppFrame::Emit { ops: PreparedOps::default(), warnings: Vec::new() },
            AppCommand::PureCommand { .. } if self.child_preview.is_some() => AppFrame::Emit { ops: PreparedOps { children: self.child_preview.clone().unwrap_or_default(), ..PreparedOps::default() }, warnings: Vec::new() },""")
hunk(MCP_DISPATCH, """    pub fn force_empty_preview(&self, instance: u32) {
        self.with_instance(instance, |state| state.empty_preview = true);
    }""", """    pub fn force_empty_preview(&self, instance: u32) {
        self.with_instance(instance, |state| state.empty_preview = true);
    }

    /// 🧩️ Every later preview on `instance` edits only owned children, carrying `children` as the guest's child groups.
    pub fn force_child_preview(&self, instance: u32, children: Vec<u8>) {
        self.with_instance(instance, |state| state.child_preview = Some(children));
    }""")
hunk(MCP_DISPATCH_TESTS, """    assert_eq!(prepared.preview["opsCount"], serde_json::json!({ "document": 0, "config": 0, "draft": 0 }));""", """    assert_eq!(prepared.preview["opsCount"], serde_json::json!({ "document": 0, "config": 0, "draft": 0, "childOpBytes": 0 }));""")
hunk(MCP_DISPATCH_TESTS, """//#endregion 🔖️NoChange""", """#[test]
fn an_action_that_edits_only_owned_children_commits_their_groups_byte_for_byte() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = single_capability_catalog(synthetic_capability("flow.editor.addWidget", &["artifacts.write"], ApprovalMode::Never, false));
    let session = SessionHandle::new("sess_children");
    let principal = principal(&["artifact.write"]);
    let groups = vec![9, 8, 7, 6];
    channel.force_child_preview(0, groups.clone());

    let prepared = adapter.prepare(&catalog, &principal, &session, "flow.editor.addWidget", serde_json::json!({}), 0, 0).unwrap();
    assert_eq!(prepared.preview["opsCount"], serde_json::json!({ "document": 0, "config": 0, "draft": 0, "childOpBytes": 4 }));
    let report = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap();

    assert_eq!(report.status, InvocationStatus::Succeeded);
    assert!(!report.warnings.contains(&NO_CHANGE_WARNING.to_string()), "a child-only edit is a change: {:?}", report.warnings);
    assert!(report.undo_token.is_some(), "a committed child group can be undone");
    let carried = channel
        .frame_log()
        .iter()
        .find_map(|(_, command)| match command {
            AppCommand::TransactionPrepare { ops, .. } => Some(ops.children.clone()),
            _ => None,
        })
        .expect("a TransactionPrepare was sent");
    assert_eq!(carried, groups, "the child groups reach the guest byte for byte");
}
//#endregion 🔖️NoChange""")
RENDERER_RUNTIME_TESTS = f"{MOD}/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx"
hunk(RENDERER_RUNTIME_TESTS, """{ transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.b#mutate", payload: [1], prepared_ops: [], label: "", origin: [] } },""", """{ transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.b#mutate", payload: [1], prepared_ops: [], label: "", origin: [], prepared_child_ops: [] } },""")
#endregion laws


def main():
    apply = "--apply" in sys.argv
    texts = {}
    failures = []
    for path, old, new, count in H:
        if path not in texts:
            texts[path] = open(path, encoding="utf-8").read()
        found = texts[path].count(old)
        if found != count:
            failures.append(f"{os.path.relpath(path, ROOT)}: expected {count}, found {found}: {old[:110]!r}")
            continue
        texts[path] = texts[path].replace(old, new)
    for failure in failures:
        print("MISS", failure)
    print(f"hunks {len(H)}, files {len(texts)}, misses {len(failures)}")
    if failures:
        sys.exit(1)
    if not apply:
        print("dry run clean")
        return
    backup = f"{ROOT}/.🧬semio/🌐hub/s13-g11-carrier-backup/{time.strftime('%H%M%S')}"
    os.makedirs(backup, exist_ok=True)
    for index, path in enumerate(texts):
        with open(path, encoding="utf-8") as original, open(f"{backup}/{index:02d}-{os.path.basename(os.path.dirname(path))}", "w", encoding="utf-8") as saved:
            saved.write(f"{path}\n")
            saved.write(original.read())
    for path, text in texts.items():
        with open(path, "w", encoding="utf-8") as target:
            target.write(text)
    print(f"applied; backups in {backup}")


main()
