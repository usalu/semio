"""📚️ Stages the JSON grammar's mixed numeric-child seam and PDF-specific projection."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "set-image"
JSON = Path("🧰️framework/🔨️modules/🎒️pack/🔤️json")
PDF = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base")
ROOT = JSON / "🦀️.rs"
GRAMMAR = JSON / "🧮️parser-and-grammar-cursor/🦀️.rs"
NUMBERS = JSON / "📥️decode/🔢️array/🦀️.rs"
MIXED = PDF / "🚪️io/💾️binary/🧬️mutations/🖼️mixed/🦀️.rs"


def change(source, marker, replacement):
    assert source.count(marker) == 1, marker
    return source.replace(marker, replacement, 1)


def numbers():
    src = (TICKET / "paged-number-array.rs").read_text(encoding="utf8")
    src = change(src, "use semio_framework_pack_json::{JsonNumberCursor, Number as JsonNumber};",
        "use super::{JsonNumberCursor, Number as JsonNumber};")
    src = change(src, "    limit: usize,\n", "    limit: usize,\n    start: usize,\n    embedded: bool,\n")
    src = change(src, "        let bytes = source.get(range).ok_or_else(invalid)?;",
        "        let start = range.start;\n        let bytes = source.get(range).ok_or_else(invalid)?;")
    src = change(src, "limit: maximum_items, phase:", "limit: maximum_items, start, embedded: false, phase:")
    pos = src.index("    pub fn admission_demand(")
    src = src[:pos] + '''    /// 🧷️ Opens a numeric child in its original payload, stopping at its closing bracket.
    pub fn admit_embedded(
        source: &'a [u8], start: usize, maximum_bytes: usize, maximum_items: usize, grant: RetainedCloneGrant,
    ) -> Result<Option<Self>, ValueError> {
        Self::admit(source, start..source.len(), maximum_bytes, maximum_items, grant).map(|value|
            value.map(|mut cursor| { cursor.embedded = true; cursor }))
    }

    pub fn finished_span(&self) -> Option<JsonNumberArraySpan<'a>> {
        self.is_ready().then(|| JsonNumberArraySpan {
            source: self.source, range: self.start..self.start + self.position,
        })
    }

''' + src[pos:]
    src = change(src, "(Phase::Value { empty: true } | Phase::Delimiter, Some(b']')) => Phase::Tail,",
        "(Phase::Value { empty: true } | Phase::Delimiter, Some(b']')) => if self.embedded { Phase::Ready } else { Phase::Tail },")
    src += '''
/// 📍️ A completed field extent tied to the same immutable byte backing.
pub struct JsonNumberArraySpan<'a> {
    source: &'a [u8],
    range: std::ops::Range<usize>,
}

impl JsonNumberArraySpan<'_> {
    pub fn range(&self, source: &[u8]) -> Option<std::ops::Range<usize>> {
        std::ptr::eq(self.source, source).then(|| self.range.clone())
    }
}
'''
    return src


def stage(workspace, verify):
    original = {path: (workspace / path).read_bytes() for path in (ROOT, GRAMMAR)}
    root = change(original[ROOT].decode(), "pub use lexer::*;", '''pub use lexer::*;

#[path = "📥️decode/🔢️array/🦀️.rs"]
mod paged_number_array;
pub use paged_number_array::{JsonNumberArrayCursor, JsonNumberArraySpan, PagedJsonNumbers};''')
    grammar = original[GRAMMAR].decode() + "\n" + (TICKET / "set-image-grammar.rs").read_text(encoding="utf8")
    mixed = (TICKET / "set-image-mixed.rs").read_text(encoding="utf8")
    for path in (NUMBERS, MIXED):
        assert not (workspace / path).exists(), path
    if not verify:
        OUT.mkdir(parents=True, exist_ok=True)
    receipt = {}
    for index, (path, src) in enumerate(((ROOT, root), (GRAMMAR, grammar), (NUMBERS, numbers()), (MIXED, mixed))):
        assert len(str((workspace / path).resolve()).encode("utf-16-le")) // 2 <= 256
        draft = OUT / f"{index}.rs"
        data = src.encode()
        before = original.get(path)
        receipt[str(path)] = {"stage": draft.name, "before": sha256(before).hexdigest() if before else None, "after": sha256(data).hexdigest()}
        if verify:
            assert draft.read_bytes() == data
        else:
            draft.write_bytes(data)
    witness = OUT / "receipt.json"
    if verify:
        assert json.loads(witness.read_bytes()) == receipt
    else:
        witness.write_text(json.dumps(receipt, ensure_ascii=False) + "\n", encoding="utf8")
    print(f"[DEBUG] {'verified' if verify else 'staged'} four mixed JSON numeric projection paths")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    stage(args.workspace.resolve(), args.verify)
