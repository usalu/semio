"""🖼️ Stages a format-1 SetImage leaf with direct exact u32 storage."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "set-u32"
JSON = Path("🧰️framework/🔨️modules/🎒️pack/🔤️json")
PDF = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base")
ROOT = JSON / "🦀️.rs"
GRAMMAR = JSON / "🧮️parser-and-grammar-cursor/🦀️.rs"
ARRAY = JSON / "📥️decode/🔢️u32/🦀️.rs"
PROJECTION = PDF / "🚪️io/💾️binary/🧬️mutations/🖼️samples/🦀️.rs"


def change(source, marker, replacement):
    assert source.count(marker) == 1, marker
    return source.replace(marker, replacement, 1)


def projection():
    text = (TICKET / "set-image-mixed.rs").read_text(encoding="utf8")
    text = change(text, "Projects the exact SetImage Samples record with native paged children.",
                  "Projects the exact SetImage Samples record with concrete typed samples.")
    text = text.replace("JsonNumberArrayCursor", "JsonU32ArrayCursor").replace("PagedJsonNumbers", "Vec<u32>")
    text = change(text, "JsonU32ArrayCursor, Vec<u32>", "JsonU32ArrayCursor")
    begin = text.index("use semio_framework_value_native_owned::")
    end = text.index("const PATH", begin)
    text = text[:begin] + "use crate::standards::v1_7::subsets::base::schema::snapshot::PdfImageBody;\n\n" + text[end:]
    begin = text.index("#[derive(semio_framework_value::RetireOwned)]")
    end = text.index("/// 🪢", begin)
    text = text[:begin] + '''#[derive(semio_framework_value::RetireOwned)]
pub struct SetImageSource {
    pub metadata: DslValue,
    pub body: PdfImageBody,
}

''' + text[end:]
    begin = text.index("fn split(")
    end = text.index("impl PathSelection", begin)
    text = text[:begin] + text[end:]
    text = change(text, "    payload: &'a [u8],\n    phase:", "    payload: &'a [u8],\n    backing: (usize, usize),\n    phase:")
    text = change(text, "original, payload, parser, phase:",
                  "original, payload, backing: (grant.maximum_capacity_bytes, grant.maximum_release_bytes), parser, phase:")
    text = change(text, "self.array = JsonU32ArrayCursor::admit_embedded(self.payload, start, MAX_PAYLOAD_BYTES, MAX_PAYLOAD_BYTES, grant)?;",
                  "self.array = JsonU32ArrayCursor::admit(self.payload, start, self.backing.0, self.backing.1, grant)?;")
    text = change(text, "3 * size_of::<DslValue>() + 2 * size_of::<Option<SetImageSource>>()",
                  "size_of::<Option<DslValue>>() + size_of::<PdfImageBody>() + size_of::<Option<SetImageSource>>()")
    begin = text.index("        let [root_index, image_index, values_index]")
    end = text.index("        self.phase = Phase::Ready;", begin)
    text = text[:begin] + '''        let values = self.pages.value.take().ok_or_else(|| invalid("SetImage Samples values must be a JSON numeric array"))?;
        let metadata = self.value.value.take().expect("validated SetImage root");
        *self.output.value = Some(SetImageSource { metadata, body: PdfImageBody::Samples { values } });
''' + text[end:]
    return text


def stage(workspace, verify):
    original = {path: (workspace / path).read_bytes() for path in (ROOT, GRAMMAR)}
    root = change(original[ROOT].decode(), "pub use lexer::*;", '''pub use lexer::*;

#[path = "📥️decode/🔢️u32/🦀️.rs"]
mod u32_array;
pub use u32_array::{JsonNumberArraySpan, JsonU32ArrayCursor};''')
    grammar = original[GRAMMAR].decode() + "\n" + (TICKET / "set-image-grammar.rs").read_text(encoding="utf8")
    array = (TICKET / "set-image-u32-array.rs").read_text(encoding="utf8")
    array = change(array, "use crate::{JsonNumberArraySpan, JsonNumberCursor, Number as JsonNumber};",
                   "use crate::{JsonNumberCursor, Number as JsonNumber};")
    array += '''
/// 📌 A completed numeric field extent tied to the original immutable source.
pub struct JsonNumberArraySpan<'a> {
    source: &'a [u8],
    range: std::ops::Range<usize>,
}

impl<'a> JsonNumberArraySpan<'a> {
    fn finished(source: &'a [u8], range: std::ops::Range<usize>) -> Self { Self { source, range } }

    pub fn range(&self, source: &[u8]) -> Option<std::ops::Range<usize>> {
        std::ptr::eq(self.source, source).then(|| self.range.clone())
    }
}
'''
    if not verify:
        OUT.mkdir(parents=True, exist_ok=True)
    receipt = {}
    for index, (path, after) in enumerate(((ROOT, root), (GRAMMAR, grammar), (ARRAY, array), (PROJECTION, projection()))):
        assert len(str((workspace / path).resolve()).encode("utf-16-le")) // 2 <= 256
        before = original.get(path)
        if before is None:
            assert not (workspace / path).exists(), path
        draft = OUT / f"{index}.rs"
        data = after.encode()
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
    print(f"[DEBUG] {'verified' if verify else 'staged'} four typed sample receiver paths")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    stage(args.workspace.resolve(), args.verify)
