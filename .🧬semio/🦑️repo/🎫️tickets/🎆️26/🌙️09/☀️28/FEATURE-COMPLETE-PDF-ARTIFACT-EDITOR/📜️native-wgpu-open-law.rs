//! 🧪 Uses the real SPR command encoder and native folder storage.
extern crate semio_os_kernel as protocol;
use semio_framework::NATURAL_FILE_PORT_ID;
use semio_framework_async::CancelToken;
use semio_framework_value::{FromValue, ToValue};
use semio_framework::manifest::MediaWireFormat;
use semio_os_kernel::{AppCommand, pack_rt};
use semio_plugin_decl_app::MediaArtifactDescriptor;
use semio_framework::kernel::COMMAND_MAXIMUM_BYTES;
use std::{future::Future, io::Read, path::{Path, PathBuf}, task::{Context, Poll, Waker}};

struct NativeMediaDeclaration { reference: bool, format_kind: String }
const EXPECTED_NATIVE_FILE_BYTES: usize = usize::MAX /* wire fixture */;

/* import command */
/* bounded reader */

fn wait<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..10000 {
        if let Poll::Ready(result) = future.as_mut().poll(&mut cx) { return result; }
        std::thread::yield_now();
    }
    panic!("Open law did not settle");
}

#[test]
fn natural_open_addresses_exact_owned_bytes_and_uses_the_registered_binary_descriptor() {
    let seq = 0x2030405060708090;
    let declaration = NativeMediaDeclaration { reference: true, format_kind: "s.stdio.pdf@1.7".into() };
    assert_eq!(wait(declaration.natural_file_payload_limit()).unwrap(), EXPECTED_NATIVE_FILE_BYTES);
    let chosen = b"%PDF-1.7\n\xff\0\x83\n%%EOF".to_vec();
    let command = import_command(seq, &declaration, chosen.clone()).unwrap();
    assert!(wait(semio_os_spr::encode_app_command(&command)).is_ok());
    let AppCommand::MediaIn { seq: received, port, descriptor, data } = command else { panic!("wrong command") };
    assert_eq!((seq, NATURAL_FILE_PORT_ID, chosen), (received, port.as_str(), data));
    let binary = MediaArtifactDescriptor::from_value(pack_rt::decode_wire_value(&descriptor).unwrap()).unwrap();
    assert_eq!(binary.port_id.as_deref(), Some(NATURAL_FILE_PORT_ID));
    assert_eq!(binary.kind_id.as_deref(), Some(declaration.format_kind.as_str()));
    assert!(matches!(binary.wire, MediaWireFormat::Binary { format_kind } if format_kind == declaration.format_kind));
    assert!(import_command(seq, &declaration, vec![]).is_err());
    let unregistered = NativeMediaDeclaration { reference: false, format_kind: declaration.format_kind };
    assert!(import_command(seq, &unregistered, vec![1]).is_err());
}

#[test]
fn reads_exact_binary_file_and_retires_cancelled_picker() {
    let directory = PathBuf::from(file!()).parent().unwrap().join("open-input");
    std::fs::create_dir_all(&directory).unwrap();
    let file = directory.join("picked.pdf");
    let bytes: Vec<u8> = b"%PDF-1.7".iter().cloned().chain((0..9700).map(|n| (n % 257) as u8)).collect();
    std::fs::write(&file, &bytes).unwrap();
    let reopened = wait(read_selected_file(&file, EXPECTED_NATIVE_FILE_BYTES, || false)).unwrap().unwrap();
    assert_eq!(reopened, bytes);
    std::fs::write(directory.join("readback.pdf"), reopened).unwrap();
    let checks = std::cell::Cell::new(0);
    assert!(wait(read_selected_file(&file, EXPECTED_NATIVE_FILE_BYTES, || {
        let count = checks.get() + 1;
        checks.set(count);
        count > 1
    })).unwrap().is_none());
    assert_eq!(checks.get(), 2);
    std::fs::File::create(&file).unwrap().set_len(EXPECTED_NATIVE_FILE_BYTES as u64 + 1).unwrap();
    assert!(wait(read_selected_file(&file, EXPECTED_NATIVE_FILE_BYTES, || false)).is_err());
}

/* cancellable import */

#[cfg(native_open_import)]
#[test]
fn cancelled_native_import_drops_the_inflight_kernel_request() {
    use std::cell::Cell;
    struct PendingRequest<'a>(&'a Cell<bool>);
    impl Future for PendingRequest<'_> {
        type Output = Result<(), String>;
        fn poll(self: std::pin::Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> { Poll::Pending }
    }
    impl Drop for PendingRequest<'_> {
        fn drop(&mut self) { self.0.set(true); }
    }

    let cancel = CancelToken::root_now();
    let dropped = Cell::new(false);
    let mut transfer = std::pin::pin!(wait_native_file_import(&cancel, PendingRequest(&dropped)));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(transfer.as_mut().poll(&mut cx).is_pending());
    assert!(!dropped.get());
    cancel.cancel_now();
    assert!(matches!(transfer.as_mut().poll(&mut cx), Poll::Ready(Ok(false))));
    assert!(dropped.get());
    let live = CancelToken::root_now();
    assert!(wait(wait_native_file_import(&live, std::future::ready(Ok(())))).unwrap());
}

#[cfg(all(not(target_arch = "wasm32"), native_open_folder))]
#[test]
fn admits_the_first_archive_and_preserves_existing_durable_bytes() {
    use semio_os_kernel::sync::FolderEventLogStorage;
    use semio_os_spr::{DocumentArchivePack, OwnedDocumentMemberPackEntry};
    let directory = PathBuf::from(file!()).parent().unwrap().join("open-folder");
    let id = format!("mounted-fixture-{}", std::process::id());
    let archive = |root: &[u8]| semio_os_spr::encode_document_archive_bytes(&DocumentArchivePack {
        parent_pack: root.to_vec(), parent_spr: b"spr\0".to_vec(),
        members: vec![OwnedDocumentMemberPackEntry { envelope_pack: vec![0x7f, 0x80], ..Default::default() }],
    }).unwrap();
    let first = archive(b"checked first document");
    let rejected = archive(b"conflicting fresh document");
    let storage = FolderEventLogStorage::new(directory.clone());
    assert!(wait(storage.create_archive_if_absent(&id, "stdio.pdf.1.7", &first)).unwrap());
    assert!(!wait(storage.create_archive_if_absent(&id, "stdio.pdf.1.7", &rejected)).unwrap());
    let reopened = FolderEventLogStorage::new(directory);
    assert_eq!(wait(reopened.read_archive(&id)).unwrap().as_deref(), Some(first.as_slice()));
}
