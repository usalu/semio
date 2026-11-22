"""🧾 Stage bounded native readback and an exact preflight of the persisted archive."""
from difflib import unified_diff
from pathlib import Path
import re

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
os = next(root.glob("🧰*/🛍*/💻*/🔨*"))
folder_store = next(os.glob("🏪*/🔄*/🖥*/📥*/🔄*/🦀*.rs"))
codec = next(os.glob("📡*/🧵*/🗜*/📚*/🦀*.rs"))
shell = next(os.glob("📺*/🧑*/🧱*/🐚*/🎯*/🧊*/📥*/📂*/🎨*/📂*.rs"))
output = ticket / "🗑️generated/native-archive-readback"
output.mkdir(parents=True, exist_ok=True)
diff = []
changes = {}


def stage(path, *replacements):
    before = path.read_text()
    after = before
    for old, new in replacements:
        assert after.count(old) == 1, (path, old)
        after = after.replace(old, new)
    changes[path] = before, after
    relative = path.relative_to(root)
    diff.extend(unified_diff(before.splitlines(True), after.splitlines(True), f"a/{relative}", f"b/{relative}"))
    return before, after


candidate = folder_store.with_name("📖️archive-readback.rs").relative_to(root)
assert len(str(root / candidate).encode("utf-16-le")) // 2 <= 256
scan = (ticket / "native-archive-readback.rs").read_text()
diff.extend(unified_diff([], scan.splitlines(True), "/dev/null", f"b/{candidate}"))
changes[root / candidate] = "", scan
checksum = """fn extend_folder_checksum(checksum: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(checksum, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3))
}
"""
before_storage, _ = stage(
    folder_store,
    ('use semio_os_store_sync::now_ms;', '''use semio_os_store_sync::now_ms;
#[cfg(not(target_arch = "wasm32"))]
#[path = "📖️archive-readback.rs"]
mod archive_readback;

#[cfg(not(target_arch = "wasm32"))]
''' + checksum),
    ("bytes.iter().fold(0xcbf29ce484222325, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3))",
     "extend_folder_checksum(0xcbf29ce484222325, bytes)"),
    (
        "    /// 🪪️ Updates an exact pair/archive baseline while retaining every recursive member.",
        '''    /// 🔍 Verifies the current durable archive in bounded reads under the folder's shared lock.
    pub fn archive_matches(
        &self, document_id: &str, expected: &[u8], cancelled: impl Fn() -> bool,
    ) -> Result<bool, semio_os_vcs::VcsError> {
        let _guard = self.writer.lock().map_err(|error| semio_os_vcs::VcsError::Backbone(error.to_string()))?;
        let mut file = match std::fs::File::open(self.event_path()) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(semio_os_vcs::VcsError::Backbone(error.to_string())),
        };
        file.try_lock_shared().map_err(|error| semio_os_vcs::VcsError::Backbone(format!("folder event reader busy: {error}")))?;
        let length = file.metadata().map_err(|error| semio_os_vcs::VcsError::Backbone(error.to_string()))?.len();
        archive_readback::archive_matches(&mut file, length, document_id, expected, cancelled)
            .map_err(|error| semio_os_vcs::VcsError::Backbone(error.to_string()))
    }

    /// 🪪️ Updates an exact pair/archive baseline while retaining every recursive member.''',
    ),
)

stage(
    shell,
    ('''    let readback = storage.read_archive(document).await.map_err(|error| error.to_string())?;
    if readback.as_deref() != Some(encoded.as_slice()) { return Err("Open File could not verify the local document archive".into()); }
    if cancel.is_cancelled_now() { return Ok(None); }''', '''    let readback = storage.archive_matches(document, &encoded, || cancel.is_cancelled_now());
    if cancel.is_cancelled_now() { return Ok(None); }
    if !readback.map_err(|error| error.to_string())? { return Err("Open File could not verify the local document archive".into()); }'''),
)

old_writer = '''        protocol::write_str(out, &entry.reference.artifact_id);
        protocol::write_str(out, &entry.reference.artifact_kind);
        protocol::write_str(out, &entry.reference.standard);
        protocol::write_str(out, &entry.reference.subset);
        protocol::write_str(out, &entry.owner.parent.artifact_id);
        protocol::write_str(out, &entry.owner.parent.artifact_kind);
        protocol::write_str(out, &entry.owner.parent.standard);
        protocol::write_str(out, &entry.owner.parent.subset);
        protocol::write_str(out, &entry.owner.slot);
        protocol::write_str(out, &entry.owner.child_id);
        protocol::write_bytes(out, &entry.envelope_pack);'''
before_codec, after_codec = stage(
    codec,
    (old_writer, '''        for label in archive_member_labels(entry) {
            protocol::write_str(out, label);
        }
        protocol::write_bytes(out, &entry.envelope_pack);'''),
    ('''/// 🗃️ Encodes one exact persisted recursive-document archive with an explicit version owner.''', '''fn archive_member_labels(entry: &OwnedDocumentMemberPackEntry) -> [&str; 10] {
    [
        &entry.reference.artifact_id, &entry.reference.artifact_kind,
        &entry.reference.standard, &entry.reference.subset,
        &entry.owner.parent.artifact_id, &entry.owner.parent.artifact_kind,
        &entry.owner.parent.standard, &entry.owner.parent.subset,
        &entry.owner.slot, &entry.owner.child_id,
    ]
}

fn archive_wire_size(archive: &DocumentArchivePack) -> usize {
    let varint = |value: u64| ((u64::BITS - value.leading_zeros()).div_ceil(7) as usize).max(1);
    let field = |bytes: &[u8]| varint(bytes.len() as u64).saturating_add(bytes.len());
    let header = 1 + varint(archive.members.len() as u64);
    let mut size = header.saturating_add(field(&archive.parent_pack)).saturating_add(field(&archive.parent_spr));
    for entry in &archive.members {
        size = size.saturating_add(varint(u64::from(entry.ordinal)));
        for label in archive_member_labels(entry) {
            size = size.saturating_add(field(label.as_bytes()));
        }
        size = size.saturating_add(field(&entry.envelope_pack));
    }
    size
}

/// 🗃️ Encodes one exact persisted recursive-document archive with an explicit version owner.'''),
    ('''    let mut bytes = vec![DOCUMENT_ARCHIVE_VERSION];
    write_document_archive(&mut bytes, archive);
    if bytes.len() > DOCUMENT_ARCHIVE_MAXIMUM_BYTES {
        return Err(malformed("document archive bytes", 0, "archive exceeds its fixed byte authority"));
    }
    Ok(bytes)''', '''    let size = archive_wire_size(archive);
    if size > DOCUMENT_ARCHIVE_MAXIMUM_BYTES {
        return Err(malformed("document archive bytes", 0, "archive exceeds its fixed byte authority"));
    }
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size).map_err(|_| malformed("document archive bytes", 0, "archive backing allocation refused"))?;
    bytes.push(DOCUMENT_ARCHIVE_VERSION);
    write_document_archive(&mut bytes, archive);
    debug_assert_eq!(bytes.len(), size);
    Ok(bytes)'''),
)
(output / "native-archive-readback.patch").write_text("".join(diff))

schema = next(os.glob("📡*/🧵*/📦*/🧬*/🦀*.rs")).read_text()
schema = "\n".join(line for line in schema.splitlines()
                   if not line.startswith(("//!","#[serde","use ")))
schema = re.sub(r",?\s*(?:serde|semio_framework_value(?:_derive)?)::\w+", "", schema)
codecs = [text[text.index("pub(super) fn write_document_archive"):text.index("pub(in super::super) fn document_archive_into_fields")]
          for text in (before_codec, after_codec)]
constants = re.findall(r'^(?:pub )?const (?:FOLDER_EVENT_MAGIC|MAX_FOLDER_EVENT_BYTES|DOCUMENT_PUT_EVENT|BLOB_PUT_EVENT|BLOB_DELETE_EVENT|DOCUMENT_ARCHIVE_PUT_EVENT):[^\n]+',
                       before_storage, re.MULTILINE)
assert len(constants) == 6
(output / "records.rs").write_text("\n".join(constants) + "\n" + checksum + "\n" + schema + "\n")
(output / "legacy.rs").write_text(codecs[0])
(output / "preflight.rs").write_text(codecs[1])
(output / "readback.rs").write_text(scan.split("\n", 1)[1])
