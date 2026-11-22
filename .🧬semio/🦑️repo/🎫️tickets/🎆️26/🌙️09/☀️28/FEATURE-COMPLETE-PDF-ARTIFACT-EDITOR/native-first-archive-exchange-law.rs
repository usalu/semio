//! 🧪 Checks the exact staged first-archive wire encoder and receiver against neutral frames.
mod protocol { pub mod wire { #[allow(dead_code)] pub mod command_ingress { include!(env!("SEMIO_ARCHIVE_WIRE_TYPES")); } } }
mod exchange { include!(env!("SEMIO_ARCHIVE_EXCHANGE")); }
use exchange::{FirstArchiveAction as Action, FirstArchiveKey, FirstArchiveReply as Reply, FirstArchiveRequest};
use protocol::wire::command_ingress::*;

const DOCUMENT_ARCHIVE_MAXIMUM_BYTES: usize = 4_194_304;
pub struct AppDocumentIdentity { pub app_instance_id: u32, pub parent_document_id: Option<String>, pub base_revision: u64 }

fn key() -> FirstArchiveKey { FirstArchiveKey { instance: 42, operation: 0x0020_0000_0000_0002, generation: 0x0020_0000_0000_0003, sequence: 0x0020_0000_0000_0005 } }

#[test]
fn exact_request_geometry() {
    let requests = [
        Action::Open, Action::Preview { offset: 4096 }, Action::Acknowledge { offset: 4096, length: 693 },
        Action::Seal, Action::Cancel, Action::Close,
    ];
    let independent = std::fs::read(env!("SEMIO_ARCHIVE_REQUESTS")).unwrap();
    for (action, frame) in requests.into_iter().zip(independent.chunks_exact(43)) {
        let request = FirstArchiveRequest { key: key(), action };
        assert_eq!(request.encode(), frame);
        assert_eq!(FirstArchiveRequest::decode(frame), Some(request));
    }
    let key = key();
    for action in [Action::Preview { offset: 1 }, Action::Preview { offset: 4_194_304 }, Action::Acknowledge { offset: 0, length: 0 },
        Action::Acknowledge { offset: 0, length: 4097 }] {
        assert!(FirstArchiveRequest::decode(&FirstArchiveRequest { key, action }.encode()).is_none());
    }
}

#[test]
fn opening_and_page_replies_are_scoped_and_bounded() {
    let key = key();
    let open = FirstArchiveRequest { key, action: Action::Open };
    let reply = Reply::Opened { extent: 2_130_273, document: "pdf-document", generation: 7, revision: [5; 32] };
    let mut output = Vec::with_capacity(reply.wire_length());
    assert!(reply.encode_into(open, &mut output));
    assert!(Reply::is_wire(&output) && FirstArchiveRequest::is_wire(&open.encode()));
    let independent = std::fs::read(env!("SEMIO_ARCHIVE_REPLY")).unwrap();
    assert_eq!(output, independent);
    let Reply::Opened { document, revision, .. } = Reply::decode(open, &output).unwrap() else { panic!("wrong reply"); };
    let identity = Reply::opened_identity(key.instance, document, &revision);
    assert_eq!(identity.app_instance_id, 42);
    assert_eq!(identity.parent_document_id.as_deref(), Some("pdf-document"));
    assert_eq!(identity.base_revision, 0x0505_0505_0505_0505);
    assert!(Reply::decode(FirstArchiveRequest { key: FirstArchiveKey { sequence: key.sequence + 1, ..key }, ..open }, &output).is_none());
    assert!(Reply::decode(FirstArchiveRequest { action: Action::Preview { offset: 0 }, ..open }, &output).is_none());

    let preview = FirstArchiveRequest { action: Action::Preview { offset: 4096 }, ..open };
    let page = [0x47; 4096];
    output = Vec::with_capacity(Reply::Page(&page).wire_length());
    assert!(Reply::Page(&page).encode_into(preview, &mut output));
    assert!(matches!(Reply::decode(preview, &output), Some(Reply::Page(bytes)) if bytes == page));
    assert!(Reply::decode(FirstArchiveRequest { action: Action::Preview { offset: 8192 }, ..preview }, &output).is_none());
    output.push(0);
    assert!(Reply::decode(preview, &output).is_none());
}

#[test]
fn only_the_accepted_exact_complete_supplies_an_archive_key() {
    let key = key();
    let declaration = NaturalSourceWireDeclaration { format_kind: "s.stdio.pdf@1.7".into(), extension: ".pdf".into(),
        media_type: "application/pdf".into(), binary: true };
    let loan = NaturalSourceWireLoan { operation_id: format!("{:016x}", key.operation), generation: format!("{:016x}", key.generation),
        instance_id: key.instance, declaration, extent: 9_437_877 };
    let mut ack = NaturalSourceWireAck { loan: Some(loan), owner: u64::from(key.instance), seq: key.sequence, kind: 241,
        extent: 9_437_877, phase: NaturalSourceWirePhase::Complete, page_index: None, result: NaturalSourceWireResult::Accepted };
    assert_eq!(FirstArchiveKey::completed(&ack), Some(key));
    ack.result = NaturalSourceWireResult::Pending;
    assert!(FirstArchiveKey::completed(&ack).is_none());
    ack.result = NaturalSourceWireResult::Accepted;
    ack.owner = u64::from(key.instance + 1);
    assert!(FirstArchiveKey::completed(&ack).is_none());
    ack.owner = u64::from(key.instance);
    ack.loan.as_mut().unwrap().generation = "002000000000000X".into();
    assert!(FirstArchiveKey::completed(&ack).is_none());
}
