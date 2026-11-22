"""📦 Stage the shared archive receipt, bound egress, and native append seam."""
from difflib import unified_diff
from pathlib import Path
import runpy

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
os = next(root.glob("🧰*/🛍*/💻*/🔨*"))
channel = next(os.glob("📡*/🧵*/🦀*.rs"))
native = next(os.glob("🏪*/🔄*/🖥*/📥*/🔄*/🦀*.rs"))
text = next(os.glob("🏪*/🚪*/📝*/🦀*.rs"))
out = ticket / "🗑️generated/native-first-archive-disk"
out.mkdir(parents=True, exist_ok=True)
runpy.run_path(str(ticket / "native-archive-readback-stage.py"))
runpy.run_path(str(ticket / "native-first-genesis-stage.py"))
diff = []
changes = {}


def insert(path, anchor, addition):
    before = path.read_text()
    assert before.count(anchor) == 1, path
    after = before.replace(anchor, anchor + "\n" + addition)
    changes[path] = before, after
    relative = path.relative_to(root)
    diff.extend(unified_diff(before.splitlines(True), after.splitlines(True), f"a/{relative}", f"b/{relative}"))


def add(ticket_file, path):
    assert len(str(path).encode("utf-16-le")) // 2 <= 256
    relative = path.relative_to(root)
    source = (ticket / ticket_file).read_text()
    changes[path] = "", source
    diff.extend(unified_diff([], source.splitlines(True), "/dev/null", f"b/{relative}"))


insert(channel, "mod archive_schema;", '''#[path = "📦️archive/🧾️first-genesis-receipt.rs"]
mod first_genesis_receipt;
#[path = "📦️archive/📬️first-genesis-exchange.rs"]
mod first_genesis_exchange;
pub use first_genesis_exchange::{FirstArchiveAction, FirstArchiveKey, FirstArchiveReply, FirstArchiveRequest};
pub use first_genesis_receipt::{
    FIRST_GENESIS_ARCHIVE_PAGE_BYTES, FirstGenesisArchiveOffer, FirstGenesisArchivePageReceipt,
    FirstGenesisArchiveScope, FirstGenesisArchiveSeal,
};''')
add("native-first-archive-receipt.rs", channel.parent / "📦️archive/🧾️first-genesis-receipt.rs")
add("native-first-archive-exchange.rs", channel.parent / "📦️archive/📬️first-genesis-exchange.rs")
insert(text, "mod document_spr_pack_and_history_composition;", '''#[path = "📦️document-spr-pack-and-history-composition/🎟️first-genesis.rs"]
mod scoped_first_genesis;
pub use scoped_first_genesis::ScopedFirstGenesisArchive;''')
add("native-first-archive-scoped.rs", text.parent / "📦️document-spr-pack-and-history-composition/🎟️first-genesis.rs")
insert(native, "use semio_os_store_sync::now_ms;", '''#[cfg(not(target_arch = "wasm32"))]
#[path = "💾️first-genesis-archive.rs"]
mod first_genesis_archive;
#[cfg(not(target_arch = "wasm32"))]
pub use first_genesis_archive::{FirstArchiveCommit, NativeFirstArchiveCapture};''')
add("native-first-archive-disk.rs", native.with_name("💾️first-genesis-archive.rs"))
(out / "native-first-archive-disk.patch").write_text("".join(diff))
