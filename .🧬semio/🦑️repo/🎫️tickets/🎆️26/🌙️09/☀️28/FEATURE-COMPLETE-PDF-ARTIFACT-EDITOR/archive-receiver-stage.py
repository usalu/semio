"""🧾 Stages the format-1 PDF mutation archive receiver for review."""

from argparse import ArgumentParser
import hashlib
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
BASE = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base")
BINARY = BASE / "🚪️io/💾️binary/🧬️mutations"
TEST = BASE / "🧬️schema/🧬️mutations/🧪️tests/🔬️unit"
OUT = TICKET / "🗑️generated" / "archive-receiver"


def replace(source, marker, value):
    assert source.count(marker) == 1, marker
    return source.replace(marker, value, 1)


def candidate():
    source = (TICKET / "📜️pdf-framed-receiving.rs").read_text()
    source = replace(
        source, "//#region 🫴️Original aggregate receiving",
        "//! 📩️ Receives format-1 archived PDF mutations using their original JSON extent.\n\n"
        "use super::{BINARY_TAG_REGISTRY, MAX_PAYLOAD_BYTES, PdfMutation};\n"
        "use pack::format::{RetainedVarintCursor, RetainedVarintStep};",
    )
    source = replace(source, "\n//#endregion 🫴️Original aggregate receiving", "")
    source = replace(source, "enum FramePhase { Version, Tag, Length, Payload }", "enum FramePhase { Version, Tag, Length(RetainedVarintCursor), Payload }")
    source = replace(source, "self.phase = FramePhase::Length;", "self.phase = FramePhase::Length(RetainedVarintCursor::default());")
    start = source.index("            FramePhase::Length => {")
    end = source.index("            FramePhase::Payload =>", start)
    source = source[:start] + '''            FramePhase::Length(mut cursor) => {
                let step = cursor.admit(byte, self.position as u64)
                    .map_err(|_| receiving_invalid("PDF mutation frame length varint is invalid"))?;
                self.position += 1;
                match step {
                    RetainedVarintStep::Pending => self.phase = FramePhase::Length(cursor),
                    RetainedVarintStep::Complete(length) => {
                        if length > MAX_PAYLOAD_BYTES as u64 {
                            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF mutation frame length exceeds its declared bound"));
                        }
                        if length != (self.end - self.position) as u64 {
                            return Err(receiving_invalid("PDF mutation payload extent differs from its frame"));
                        }
                        self.phase = FramePhase::Payload;
                    }
                }
            }
''' + source[end:]
    source = replace(source, "let demand = receiving_metadata(size_of::<PdfMutationReceiving>());", "let demand = receiving_metadata(0);")
    source = replace(
        source, "if demand.copy_bytes > body || !retained_clone_grant_admits(effect.grant(), demand)",
        "if effect.maximum_work_units() == 0 || demand.copy_bytes > body || !retained_clone_grant_admits(effect.grant(), demand)",
    )
    source = replace(source, "if !retained_clone_grant_admits(grant, demand) {\n            return done", "if effect.maximum_work_units() == 0 || !retained_clone_grant_admits(grant, demand) {\n            return done")
    source = replace(source, "if self.phase != ReceivingPhase::Closing { return Ok(1); }", "if self.phase == ReceivingPhase::Closed { return Ok(0); }\n        if self.phase != ReceivingPhase::Closing { return Ok(1); }")
    source = replace(source, "    fn next_selected_work_byte_demand", "    fn next_work_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demand(usize::MAX)?.copy_bytes) }\n    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }\n    fn next_selected_work_byte_demand")
    source = replace(source, "fn terminal_is_empty(&self) -> bool { self.phase == ReceivingPhase::Closed }", """fn terminal_is_empty(&self) -> bool {
        self.phase == ReceivingPhase::Closed && self.output.terminal_is_empty()
            && self.value.terminal_is_empty() && self.parser.terminal_is_empty() && self.json_error.terminal_is_empty()
    }""")
    return source


def stage(root, verify):
    binary = BINARY / "🦀️.rs"
    schema = BASE / "🧬️schema/🧬️mutations/🦀️.rs"
    grammar = BINARY / "📡️.protocol.semio"
    tests = TEST / "🦀️.rs"
    existing = {path: (root / path).read_text(encoding="utf8") for path in (binary, schema, grammar, tests)}
    mutations = replace(existing[schema], "dsl::original_pack_value_operation_receiving!(PdfMutation);\n", "")
    original = replace(existing[binary], "//#region 🧱️Framing", "#[path = \"📩️receiving/🦀️.rs\"]\nmod receiving;\npub use receiving::{PdfMutationReceiving, PdfMutationReceivingRetirement};\n\n//#region 🧱️Framing")
    protocol_marker = "# Real op frame: "
    assert protocol_marker in existing[grammar]
    protocol = existing[grammar].replace(protocol_marker,
        "# Aggregate op frame is format u8, tag u8, canonical unsigned varint JSON byte length, then exactly that many JSON bytes.\n"
        "# Direct leaf op frame: ", 1)
    cases = (TICKET / "archive-cases.json").read_text(encoding="utf8")
    rust_tests = (TICKET / "archive-receiver-test.rs").read_text(encoding="utf8")
    changes = {
        binary: original,
        schema: mutations,
        grammar: protocol,
        tests: existing[tests] + "\n" + rust_tests,
        TEST / "archive-cases.json": cases,
        BINARY / "📩️receiving/🦀️.rs": candidate(),
    }
    if not verify:
        OUT.mkdir(parents=True, exist_ok=True)
    receipt = {}
    for index, (relative, source) in enumerate(changes.items()):
        before = (root / relative).read_bytes() if (root / relative).is_file() else None
        if relative not in existing:
            assert before is None, relative
        else:
            assert before == existing[relative].encode(), relative
        after = source.encode()
        overlay = OUT / f"{index}{relative.suffix}"
        if verify:
            assert overlay.read_bytes() == after, relative
        else:
            overlay.write_bytes(after)
        assert len(str((root / relative).resolve()).encode("utf-16-le")) // 2 <= 256, relative
        receipt[str(relative)] = {"overlay": overlay.name,
            "before": hashlib.sha256(before).hexdigest() if before is not None else None,
            "after": hashlib.sha256(after).hexdigest()}
    witness = OUT / "receipt.json"
    if verify:
        assert json.loads(witness.read_bytes()) == receipt
    else:
        witness.write_text(json.dumps(receipt, ensure_ascii=False) + "\n", encoding="utf8")
    print(f"[DEBUG] {'verified' if verify else 'staged'} {len(changes)} archive receiver paths")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("root", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    stage(args.root.resolve(), args.verify)
