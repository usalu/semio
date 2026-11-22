mod folder;
use folder::{FolderEventLogStorage, folder_source::NativeFolderArchiveSource};
use folder_wire::*;
mod folder_wire;
use ingress as archive_ingress;

struct JournalTransport {
    source: NativeFolderArchiveSource,
    key: FolderArchiveLoadKey,
}

impl std::io::Read for JournalTransport {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        let offset = self.source.offered();
        let count = self.source.read_next(output, &|| false)?;
        if count == 0 { return Ok(0); }
        let request = FolderArchiveLoadRequest {
            key: self.key,
            action: FolderArchiveLoadAction::Page { offset: offset as u32, bytes: &output[..count] },
        };
        let bytes = request.encode().unwrap();
        let actual = FolderArchiveLoadRequest::decode(&bytes).unwrap();
        assert_eq!(actual, request);
        let wrong_scope = FolderArchiveLoadRequest { key: FolderArchiveLoadKey { lifetime: self.key.lifetime + 1, ..self.key }, ..request };
        let reply = FolderArchiveLoadReply::Received { through: (offset + count) as u32 };
        let mut output = Vec::with_capacity(FolderArchiveLoadReply::MAXIMUM_BYTES);
        assert!(reply.encode_into(actual, &mut output));
        assert_eq!(FolderArchiveLoadReply::decode(request, &output), Some(reply));
        assert!(FolderArchiveLoadReply::decode(wrong_scope, &output).is_none());
        Ok(count)
    }
}

fn journal(
    args: &[String], extent: usize, limits: DocumentArchiveIngressLimits, digest: [u8; 32],
) -> (bool, usize) {
    let storage = FolderEventLogStorage::new(args[1].clone().into());
    let name = "high-entropy-reopen-fixture";
    let schema = "stdio.pdf.1.7";
    assert!(storage.open_archive_source(name, "stdio.pdf.1.4", limits.archive_bytes, &|| false).is_err());
    assert!(storage.open_archive_source("another-document", schema, limits.archive_bytes, &|| false).unwrap().is_none());
    assert!(storage.open_archive_source(name, schema, extent - 1, &|| false).is_err());
    let mut source = storage.open_archive_source(name, schema, limits.archive_bytes, &|| false).unwrap().unwrap();
    let seal = source.seal();
    assert_eq!((seal.extent, seal.sha256), (extent, digest));
    assert_eq!(source.read_next(&mut [0; 16], &|| true).unwrap_err().kind(), std::io::ErrorKind::Interrupted);
    assert_eq!(source.offered(), 0);
    let key = FolderArchiveLoadKey { instance: 37, activation: 0, lifetime: 0, sequence: 65 };
    let request = FolderArchiveLoadRequest { key, action: FolderArchiveLoadAction::Begin {
        seal: DocumentArchiveIngressSeal { bytes: extent, sha256: digest }, document: name, schema,
    }};
    assert_eq!(FolderArchiveLoadRequest::decode(&request.encode().unwrap()), Some(request));
    let reply = FolderArchiveLoadReply::Admitted { activation: 17, lifetime: 29 };
    let mut response = Vec::with_capacity(FolderArchiveLoadReply::MAXIMUM_BYTES);
    assert!(reply.encode_into(request, &mut response));
    assert_eq!(FolderArchiveLoadReply::decode(request, &response), Some(reply));
    let key = FolderArchiveLoadKey { activation: 17, lifetime: 29, ..key };
    let result = run(JournalTransport { source, key }, extent, limits, digest, "journal", Path::new(&args[8]));
    assert!(storage.archive_source_is_current(name, schema, seal, limits.archive_bytes, &|| false).unwrap());
    let candidate = std::fs::read(&args[9]).unwrap();
    let mut journal = std::fs::OpenOptions::new().append(true).open(storage.event_path()).unwrap();
    journal.write_all(&candidate).unwrap();
    journal.sync_data().unwrap();
    assert!(!storage.archive_source_is_current(name, schema, seal, limits.archive_bytes, &|| false).unwrap());
    result
}
