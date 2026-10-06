//! 🥇️ The document-port control maps cross as the golden wire bytes the TypeScript closed grammar wrote into the binding
//! fixture (`codec.control`): the Rust codec encodes every command and receipt to exactly those bytes and reads them back.
use semio_framework_plugin::document_backbone_binding::{
    decode_document_backbone_binding_command_v1, require_document_backbone_binding_receipt_v1, DocumentBackboneBindingCommandV1, DocumentBackboneBindingOperationV1, DocumentBackboneBindingReceiptV1, DOCUMENT_BACKBONE_BINDING_SCHEMA_V1,
};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn the_rust_codec_writes_and_reads_the_typescript_golden_control_bytes() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧫️fixtures/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).expect("the binding fixture is read")).expect("the binding fixture parses");
    let rows = fixture["codec"]["control"].as_array().expect("control rows");
    let mut departures = Vec::new();
    for row in rows {
        let (id, value, golden) = (row["id"].as_str().expect("id"), &row["value"], row["hex"].as_str().expect("hex"));
        let bytes = (0..golden.len()).step_by(2).map(|index| u8::from_str_radix(&golden[index..index + 2], 16).expect("golden hex")).collect::<Vec<_>>();
        let operation = value["operation"].as_str().expect("operation");
        let owner = DocumentBackboneBindingCommandV1 {
            operation: if matches!(operation, "retire" | "retired") { DocumentBackboneBindingOperationV1::Retire } else { DocumentBackboneBindingOperationV1::Bind },
            instance_id: u32::try_from(value["instanceId"].as_u64().expect("instanceId")).expect("instance id fits"),
            binding_generation: value["bindingGeneration"].as_str().expect("bindingGeneration").parse().expect("generation"),
            uri: value["uri"].as_str().expect("uri").to_string(),
        };
        if value["schema"] == DOCUMENT_BACKBONE_BINDING_SCHEMA_V1 {
            let written = hex(&owner.encode().expect("the command encodes"));
            let read = decode_document_backbone_binding_command_v1(&bytes);
            eprintln!("{id}: rust writes {} bytes, golden {} bytes, equal={}; rust reads the golden bytes as {read:?}", written.len() / 2, golden.len() / 2, written == golden);
            if written != golden {
                departures.push(format!("{id}: rust writes {written}, golden {golden}"));
            }
            if read != Ok(Some(owner)) {
                departures.push(format!("{id}: rust reads the golden command as {read:?}"));
            }
            continue;
        }
        let code = value["code"].as_str().map(|code| &*Box::leak(code.to_string().into_boxed_str()));
        let receipt = match (operation, code) {
            ("bound", None) => DocumentBackboneBindingReceiptV1::bound(&owner),
            ("retired", None) => DocumentBackboneBindingReceiptV1::retired(&owner),
            ("refused", Some(code)) => DocumentBackboneBindingReceiptV1::refused(&owner, code),
            other => panic!("{id}: no receipt is {other:?}"),
        };
        let written = hex(&receipt.encode());
        let read = require_document_backbone_binding_receipt_v1(&bytes, &owner);
        eprintln!("{id}: rust writes {} bytes, golden {} bytes, equal={}; rust reads the golden bytes as {read:?}", written.len() / 2, golden.len() / 2, written == golden);
        if written != golden {
            departures.push(format!("{id}: rust writes {written}, golden {golden}"));
        }
        if read != code.map_or(Ok(()), |code| Err(code.to_string())) {
            departures.push(format!("{id}: rust reads the golden receipt as {read:?}"));
        }
    }
    assert!(rows.len() >= 5 && departures.is_empty(), "the control wire departs in {} place(s):\n{}", departures.len(), departures.join("\n"));
}
