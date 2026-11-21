"""🧩 Repairs the WGPU extension query's public return boundary."""
import hashlib
import json
import os
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated/native-wgpu-save/extension-query"
ROOT = next(Path("🧰️framework").glob("🛍*/💻*/🔨*/📺*/🧑*/🎯*/🧊*"))
RUNTIME = ROOT / "🧊️renderer/📦️kernel-runtime"


def main():
    """📥 Guards and optionally publishes the exact native extension query."""
    OUT.mkdir(parents=True, exist_ok=True)
    core = RUNTIME / "🧮️kernel-core/🦀️.rs"
    registry = RUNTIME / "🏊️kernel-pool/🧩️app-lifecycle-and-realm-close/🦀️.rs"
    pool = ROOT / "🏊️kernel-pool/🦀️.rs"
    editing = {
        core: [('''
    pub fn extensions_of(&self, plugin_id: &str) -> &[ExtensionRecord] {
        self.by_parent.get(plugin_id).map(Vec::as_slice).unwrap_or(&[])
    }
''', ""),
               ("pub fn extension_index() -> &'static ExtensionIndex {", "pub fn extensions_of(plugin_id: &str) -> &'static [ExtensionRecord] {"),
               ("    INDEX.get_or_init(ExtensionIndex::load)", "    INDEX.get_or_init(ExtensionIndex::load).by_parent.get(plugin_id).map(Vec::as_slice).unwrap_or(&[])")],
        registry: [("extension_index().extensions_of(plugin_id)", "extensions_of(plugin_id)")],
        pool: [("decode_actor_turn_result, extension_index,", "decode_actor_turn_result, extensions_of,")],
    }
    rows = []
    for index, (path, edits) in enumerate(editing.items()):
        before = path.read_bytes()
        source = before.decode()
        for old, new in edits:
            assert source.count(old) == 1, (path, old)
            source = source.replace(old, new)
        after = source.encode()
        name = f"{index:02}.rs"
        (OUT / name).write_bytes(after)
        rows.append({"file": str(path), "source": name, "before": hashlib.sha256(before).hexdigest(),
                     "after": hashlib.sha256(after).hexdigest()})
    (OUT / "receipt.json").write_text(json.dumps(rows, ensure_ascii=False, indent=2) + "\n")
    if "--install" in sys.argv:
        for row in rows:
            path = Path(row["file"])
            assert hashlib.sha256(path.read_bytes()).hexdigest() == row["before"], path
            temporary = path.with_name(".extension-" + row["source"])
            temporary.write_bytes((OUT / row["source"]).read_bytes())
            os.replace(temporary, path)
        assert all(hashlib.sha256(Path(row["file"]).read_bytes()).hexdigest() == row["after"] for row in rows)
        (OUT.parent / "extension-query-installed.json").write_text(json.dumps(rows, ensure_ascii=False, indent=2) + "\n")


if __name__ == "__main__":
    main()
