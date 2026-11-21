"""📘 Stages the reusable numeric source module beside the existing JSON lexer."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "paged-number" / "overlay"
JSON = Path("🧰️framework/🔨️modules/🎒️pack/🔤️json")
ROOT = JSON / "🦀️.rs"
MODULE = JSON / "📥️decode/🔢️array/🦀️.rs"


def stage(workspace, verify):
    original = (workspace / ROOT).read_bytes()
    assert not (workspace / MODULE).exists()
    root = original.decode()
    marker = "pub use lexer::*;"
    assert root.count(marker) == 1
    root = root.replace(marker, marker + '\n\n#[path = "📥️decode/🔢️array/🦀️.rs"]\n'
        'mod paged_number_array;\npub use paged_number_array::{JsonNumberArrayCursor, PagedJsonNumbers};', 1)
    module = (TICKET / "paged-number-array.rs").read_text(encoding="utf8")
    marker = "use semio_framework_pack_json::{JsonNumberCursor, Number as JsonNumber};"
    assert module.count(marker) == 1
    module = module.replace(marker, "use super::{JsonNumberCursor, Number as JsonNumber};", 1)
    if not verify:
        OUT.mkdir(parents=True, exist_ok=True)
    receipt = {}
    for index, (path, before, after) in enumerate(((ROOT, original, root.encode()), (MODULE, None, module.encode()))):
        artifact = OUT / f"{index}.rs"
        assert len(str((workspace / path).resolve()).encode("utf-16-le")) // 2 <= 256
        receipt[str(path)] = {"stage": artifact.name, "before": sha256(before).hexdigest() if before else None, "after": sha256(after).hexdigest()}
        if verify:
            assert artifact.read_bytes() == after
        else:
            artifact.write_bytes(after)
    witness = OUT / "receipt.json"
    if verify:
        assert json.loads(witness.read_bytes()) == receipt
    else:
        witness.write_text(json.dumps(receipt, ensure_ascii=False) + "\n", encoding="utf8")
    print(f"[DEBUG] {'verified' if verify else 'staged'} two framework JSON numeric projection paths")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    stage(args.workspace.resolve(), args.verify)
