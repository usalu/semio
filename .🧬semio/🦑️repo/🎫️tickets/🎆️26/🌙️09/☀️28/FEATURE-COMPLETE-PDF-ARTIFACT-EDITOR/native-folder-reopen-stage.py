"""📥 Stage the native verified journal reader, scoped wire, and typed Store receiving port."""
from difflib import unified_diff
from pathlib import Path
import runpy

ticket = Path(__file__).resolve().parent
base = runpy.run_path(str(ticket / "large-reopen-base.py"))
root, channel, out = (base[name] for name in ("ROOT", "CHANNEL", "OUT"))
os = channel.parent.parent
changes = {}


def insert(path, anchor, content):
    old, current = changes.get(path, (path.read_text(), path.read_text()))
    assert current.count(anchor) == 1, (path, anchor)
    changes[path] = old, current.replace(anchor, anchor + content)


def add(path, filename):
    assert not path.exists(), path
    changes[path] = "", (ticket / filename).read_text()


native = os / "🏪️store/🔄️sync/🖥️native/📥️lifted/🔄️sync/🦀️.rs"
insert(native, "use semio_os_store_sync::now_ms;", """
#[cfg(not(target_arch = "wasm32"))]
#[path = "📖️archive-source.rs"]
mod archive_source;
#[cfg(not(target_arch = "wasm32"))]
pub use archive_source::{NativeFolderArchiveSource, NativeFolderArchiveSeal, FOLDER_ARCHIVE_PAGE_BYTES};""")
add(native.with_name("📖️archive-source.rs"), "native-folder-reopen.rs")
ingress = channel / "📦️archive/🛬️ingress"
insert(ingress / "🦀️.rs", "pub type DocumentArchivePages = PagedBytes<{isize::MAX as usize}>;", '''

#[path = "📥️folder-load.rs"]
pub mod folder_archive_load;''')
wire = (ticket / "native-folder-reopen-wire.rs").read_text().replace("use super::archive_ingress::{", "use super::{")
changes[ingress / "📥️folder-load.rs"] = "", wire
store = os / "🏪️store/🚪️io/🦀️.rs"
insert(store, "pub trait ArtifactStoreSnapshot {", '''
    /// 📥 Selects the typed receiver for its authored canonical Pack borrowed from fixed pages.
    fn admit_paged_snapshot(
        _source: &dyn protocol::RetainedWireByteSource,
        _limits: protocol::PackLimits,
        _native: &semio_framework_value::NativeDecodeControl<'_>,
        _grant: semio_framework_value::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn PagedSnapshotReceiver<Self>>, semio_framework_value::RetainedCloneProgress)>, ValueError>
    where Self: Sized {
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "snapshot has no declared paged original receiver"))
    }
''')
insert(store, "pub use semio_os_spr::{OpBinary, OpText, ProtocolError};", '''

/// 🧬 The original typed snapshot and every partial receive owner stay within one native decoder.
pub trait PagedSnapshotReceiver<P>: Send {
    fn demands(&self, source: &dyn protocol::RetainedWireByteSource, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn step(
        &mut self, source: &dyn protocol::RetainedWireByteSource,
        native: &mut semio_framework_value::NativeDecodeControl<'_>, body: usize,
        effect: &mut semio_framework_value::RetainedAllocationEffect<'_>,
    ) -> semio_framework_value::RetainedWorkOutcome<semio_framework_value::RetainedCloneProgress, semio_framework_value::OriginalReceivingPrimary>;
    fn is_ready(&self) -> bool;
    fn take_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn take_into(&mut self, output: &mut Option<P>, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneProgress, ValueError>;
    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll;
    fn terminal_is_empty(&self) -> bool;
}''')

patch = []
for path, (old, new) in changes.items():
    assert len(str(path).encode("utf-16-le")) // 2 <= 256, path
    assert (path.read_text() if path.is_file() else "") == old, path
    rel = path.relative_to(root)
    patch.extend(unified_diff(old.splitlines(True), new.splitlines(True), f"a/{rel}" if old else "/dev/null", f"b/{rel}"))
(out / "native-folder-source.patch").write_text("".join(patch))
(out / "folder_source.rs").write_text((ticket / "native-folder-reopen.rs").read_text())
(out / "folder_wire.rs").write_text(wire)

journal = (ticket / "native-archive-readback.rs").read_text()
storage = native.read_text()
constants = storage[storage.index("pub const FOLDER_EVENT_MAGIC"):storage.index("struct FolderEvent {")]
constants = "\n".join(line for line in constants.splitlines() if line.startswith(("const ", "pub const ")))
begin = storage.index("fn folder_writer(")
end = storage.index("/// 💾️ Replaces", begin)
new = storage[storage.index("    pub fn new(folder:"):storage.index("    fn checksum(")]
frozen = f'''{constants}
pub struct FolderEventLogStorage {{
    folder: std::path::PathBuf,
    writer: std::sync::Arc<std::sync::Mutex<()>>,
}}
{storage[begin:end]}
impl FolderEventLogStorage {{ {new} }}
fn extend_folder_checksum(checksum: u64, bytes: &[u8]) -> u64 {{
    bytes.iter().fold(checksum, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3))
}}
#[allow(dead_code)]
#[path = "archive_readback.rs"]
mod archive_readback;
#[path = "folder_source.rs"]
pub mod folder_source;
'''
(out / "archive_readback.rs").write_text(journal)
(out / "folder.rs").write_text(frozen)
