"""📦 Stage the shared store's borrowed first-genesis archive source."""
from difflib import unified_diff
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
store = next(root.glob("🧰*/🛍*/💻*/🔨*/🏪*/🚪*/📝*"))
module = store / "🦀️.rs"
path = store / "📦️document-spr-pack-and-history-composition/🧳️first-genesis.rs"
out = ticket / "🗑️generated/native-first-genesis"
out.mkdir(parents=True, exist_ok=True)
assert len(str(path).encode("utf-16-le")) // 2 <= 256

before = module.read_text()
anchor = 'mod document_spr_pack_and_history_composition;'
assert before.count(anchor) == 1
after = before.replace(anchor, anchor + '''
#[path = "📦️document-spr-pack-and-history-composition/🧳️first-genesis.rs"]
mod first_document_archive;
pub use first_document_archive::FirstGenesisArchive;''')
relative = module.relative_to(root)
diff = list(unified_diff(before.splitlines(True), after.splitlines(True), f"a/{relative}", f"b/{relative}"))
relative = path.relative_to(root)
source = (ticket / "native-first-genesis-archive.rs").read_text()
changes = {module: (before, after), path: ("", source)}
diff.extend(unified_diff([], source.splitlines(True), "/dev/null", f"b/{relative}"))
(out / "native-first-genesis.patch").write_text("".join(diff))
