//! 🧾️ Extracts archive mutations and compares the cold and retained first-party decoders.

use semio_os_spr::{decode_document_archive_bytes, decode_history};
use semio_os_spr::{OpBinary, ProtocolError};
use semio_os_dsl::{
    original_receiving::{ReceivingStock, RetainedCloneGrant, receive_to_completion},
    tagged_value_binary::{self, VariantTag},
};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_os_pack::pack_rt::{decode_wire_value, encode_wire_value};
use std::{future::Future, path::Path};

fn complete<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    for _ in 0..8192 {
        if let std::task::Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
    panic!("archive decoder exceeded the poll bound");
}

const PROTOCOL: &str = include_str!("../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio");

#[derive(Debug, PartialEq)]
struct OldMutation { value: DslValue }

semio_framework_value::artifact_retire_struct!(OldMutation{value});

impl FromValue for OldMutation {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Self { value })
    }
}

impl OpBinary for OldMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(encode_wire_value(&self.value))
    }

    fn decode_op(binary: &[u8]) -> Result<Self, ProtocolError> {
        decode_wire_value(binary).map(|value| Self { value }).map_err(|error| ProtocolError::Malformed {
            what: "gis terrain operation", offset: 0, detail: error.to_string(),
        })
    }
}

semio_os_dsl::original_pack_value_operation_receiving!(OldMutation);

#[derive(Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct ElevationSource {
    bounds: [f64; 4],
    rows_millimetres: Vec<Vec<i32>>,
}

#[derive(Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct ReplaceElevationSource {
    new_elevation_source: Option<ElevationSource>,
}

#[derive(Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive)]
enum AuthoredMutation {
    ReplaceElevationSource(ReplaceElevationSource),
}

impl OpBinary for AuthoredMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        tagged_value_binary::encode_op(PROTOCOL, VariantTag::Key, self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> {
        tagged_value_binary::decode_op(PROTOCOL, VariantTag::Key, bytes)
    }
}

semio_os_dsl::original_pack_value_operation_receiving!(AuthoredMutation, tagged_key = PROTOCOL);

fn receiving_stock(maximum_copy_bytes: usize) -> ReceivingStock {
    let stock = RetainedCloneGrant {
        maximum_items: 1 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30,
        maximum_release_bytes: 1 << 30, maximum_depth: 4096,
    };
    let quantum = RetainedCloneGrant {
        maximum_items: 64, maximum_copy_bytes, maximum_capacity_bytes: 16_777_216,
        maximum_release_bytes: 1_048_576, maximum_depth: 4096,
    };
    ReceivingStock { stock, quantum, fuel: 1 << 20 }
}

fn verify_historical(binary: &[u8]) {
    let old = OldMutation::decode_op(binary).expect("the historical cold operation codec");
    assert_eq!(binary, old.encode_op().unwrap());
    let source = old.value.get("ReplaceElevationSource").unwrap().get("newElevationSource").unwrap();
    let rows = source.get("rows").and_then(DslValue::as_array).unwrap();
    assert_eq!(rows.len(), 33);
    assert!(source.get("rowsMillimetres").is_none());
    assert!(AuthoredMutation::decode_op(binary).is_err());
    for copy in [32_768, 49_152] {
        let old_fault = receive_to_completion::<OldMutation>(binary, (0, binary.len()), receiving_stock(copy))
            .expect_err("whole-Pack receiving must reject a historical container-less payload");
        let cause = old_fault.message();
        assert_eq!(cause, "checksum mismatch in header at offset 20");
        println!("[DEBUG] copy={copy} historical original receiver: {cause}");
    }
}

fn verify_current(canonical_path: &Path, side: i32) {
    let authored = AuthoredMutation::ReplaceElevationSource(ReplaceElevationSource {
        new_elevation_source: Some(ElevationSource {
            bounds: [6.625, 44.625, 7.125, 45.125],
            rows_millimetres: (0..side).map(|row| (0..side).map(|column| row * 1000 + column * 11 - 100).collect()).collect(),
        }),
    });
    let canonical = authored.encode_op().expect("the authored Terrain OpBinary encoder");
    assert_eq!(&canonical[..2], &[1, 2]);
    assert_eq!(AuthoredMutation::decode_op(&canonical).unwrap(), authored);
    std::fs::write(canonical_path, &canonical).unwrap();
    for copy in [32_768, 49_152, 65_536] {
        let received = match receive_to_completion::<AuthoredMutation>(&canonical, (0, canonical.len()), receiving_stock(copy)) {
            Ok(received) => received,
            Err(error) => {
                println!("[DEBUG] side={side} copy={copy} canonical original receiver: {}", error.message());
                continue;
            }
        };
        assert_eq!(received, authored);
        let mut embedded = b"prefix-sentinel".to_vec();
        let start = embedded.len();
        embedded.extend_from_slice(&canonical);
        let end = embedded.len();
        embedded.extend_from_slice(b"suffix-sentinel");
        let received = receive_to_completion::<AuthoredMutation>(&embedded, (start, end), receiving_stock(copy))
            .expect("the retained Terrain receiver honours the original bounded span");
        assert_eq!(received, authored);
        println!("[DEBUG] side={side} copy={copy} canonical original receiver and bounded span matched");
    }
}

fn extract(archive_path: &Path, output: &Path) {
    let bytes = std::fs::read(archive_path).unwrap();
    let archive = complete(decode_document_archive_bytes(&bytes)).unwrap();
    std::fs::write(output.join("parent.spk"), &archive.parent_pack).unwrap();
    std::fs::write(output.join("parent.spr"), &archive.parent_spr).unwrap();
    let history = complete(decode_history(&archive.parent_spr, &semio_os_spr::DecodeOptions::default())).unwrap();
    println!("[DEBUG] document={} schema={} edits={} transitions={}", history.doc_id, history.schema, history.edits.len(), history.transitions.len());
    for (index, edit) in history.edits.iter().enumerate() {
        for (lane, operations) in [("forward", &edit.ops), ("inverse", &edit.inverse)] {
            for (position, payload) in operations.iter().enumerate() {
                let binary = payload.binary.as_deref().expect("an authored OpBinary payload");
                let name = format!("{index}-{lane}-{position}.op");
                std::fs::write(output.join(&name), binary).unwrap();
                println!("[DEBUG] {name} edit={:?} binary={} text={}", edit.verb, binary.len(), payload.text.as_ref().map_or(0, String::len));
                verify_historical(binary);
            }
        }
    }
    verify_current(&output.join("authored-small.op.canonical"), 2);
    verify_current(&output.join("authored.op.canonical"), 33);
}

fn main() {
    let archive = std::env::args_os().nth(1).unwrap();
    let output = std::env::args_os().nth(2).unwrap();
    std::thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        extract(Path::new(&archive), Path::new(&output));
    }).unwrap().join().unwrap();
}
