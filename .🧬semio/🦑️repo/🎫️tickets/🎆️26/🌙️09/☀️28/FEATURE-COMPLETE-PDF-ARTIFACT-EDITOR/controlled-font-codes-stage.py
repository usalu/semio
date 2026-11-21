"""🔢️ Stage fixed-width PDF text-code admission without changing the live source epoch."""

from pathlib import Path
import difflib
import json
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "font-codes"
EDITS = {
    "🔨️modules/🖋️content/🦀️.rs": [
        ("/// 🔤 The fonts a content stream may select, by resource name.", "/// 🔢️ Physical width of a qualified, fixed-width PDF font codespace.\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\npub enum PdfFixedCodeWidth { One = 1, Two = 2 }\n\n/// 🔤 The fonts a content stream may select, by resource name.", 1),
        ("    fn controlled_type1(&self, _name: &str) -> Option<PdfBaseEncoding> { None }", "    fn controlled_type1(&self, _name: &str) -> Option<PdfBaseEncoding> { None }\n    /// 🔒️ Supplies a separately admitted codespace whose physical codes must be retained.\n    fn controlled_code_width(&self, _name: &str) -> Option<PdfFixedCodeWidth> { None }", 1),
    ],
    "🔨️modules/🖋️content/🧵️program/🦀️.rs": [
        ("use super::PdfBaseEncoding;", "use super::{PdfBaseEncoding, PdfFixedCodeWidth};", 1),
        ("    encoding: Option<PdfBaseEncoding>,", "    encoding: Option<PdfBaseEncoding>,\n    code_width: Option<PdfFixedCodeWidth>,", 1),
        ("            encoding: None,", "            encoding: None,\n            code_width: None,", 1),
        ("            self.encoding = self.selected_font().and_then(|name| fonts.controlled_type1(name));\n            if self.encoding.is_none() && self.fonts.codec(fonts).is_some() { return self.defer(start, end); }", "            if !self.select_text_encoding(fonts) { return self.defer(start, end); }", 2),
        ("            if self.encoding.is_some() && first_array(&self.operands).len() > CONTROLLED_TEXT_ITEMS { return self.defer(start, end); }", "            if (self.encoding.is_some() || self.code_width.is_some())\n                && first_array(&self.operands).len() > CONTROLLED_TEXT_ITEMS { return self.defer(start, end); }", 1),
        ("    fn start_text(&mut self, target: TextTarget) -> ContentProgramReceipt {\n        if self.encoding.is_some() && target.bytes(&self.operands).len() > CONTROLLED_TEXT_BYTES {\n            let (start, end) = self.operator_span;\n            return self.defer(start, end);\n        }\n        self.phase = Phase::ScanText { target, index: 0, bytes: 0 };\n        worked(0, 0)\n    }", "    fn select_text_encoding(&mut self, fonts: &dyn FontTable) -> bool {\n        self.encoding = self.selected_font().and_then(|name| fonts.controlled_type1(name));\n        self.code_width = self.selected_font().and_then(|name| fonts.controlled_code_width(name));\n        self.encoding.is_some() || self.code_width.is_some() || self.fonts.codec(fonts).is_none()\n    }\n\n    fn native_width(&self) -> usize { self.code_width.map_or(1, |width| width as usize) }\n\n    fn start_text(&mut self, target: TextTarget) -> ContentProgramReceipt {\n        let length = target.bytes(&self.operands).len();\n        if (self.encoding.is_some() || self.code_width.is_some()) && length > CONTROLLED_TEXT_BYTES {\n            let (start, end) = self.operator_span;\n            return self.defer(start, end);\n        }\n        self.phase = if self.code_width.is_some() {\n            if !length.is_multiple_of(self.native_width()) {\n                self.phase = Phase::Failed(\"PDF text ends inside its admitted fixed-width codespace\");\n                return failure(\"PDF text ends inside its admitted fixed-width codespace\");\n            }\n            Phase::AllocateText { target, utf8_bytes: None }\n        } else {\n            Phase::ScanText { target, index: 0, bytes: 0 }\n        };\n        worked(0, 0)\n    }", 1),
        ("        let len = target.bytes(&self.operands).len();\n        let Some(bytes) = utf8_bytes.or_else(|| len.checked_mul(size_of::<u32>()))", "        let len = target.bytes(&self.operands).len() / self.native_width();\n        let Some(bytes) = utf8_bytes.or_else(|| len.checked_mul(size_of::<u32>()))", 1),
        ("        if let Some(byte) = target.bytes(&self.operands).get(index).copied() {\n            match self.text.as_mut().expect(\"allocated PDF semantic text\") {", "        let width = self.native_width();\n        let source = target.bytes(&self.operands);\n        if let Some(byte) = source.get(index).copied() {\n            match self.text.as_mut().expect(\"allocated PDF semantic text\") {", 1),
        ("                PdfTextString::Codes { codes } => codes.push(u32::from(byte)),", "                PdfTextString::Codes { codes } => {\n                    let code = if width == 2 { u32::from(u16::from_be_bytes([byte, source[index + 1]])) } else { u32::from(byte) };\n                    codes.push(code);\n                }", 1),
        ("            self.phase = Phase::CopyText { target, index: index + 1 };", "            self.phase = Phase::CopyText { target, index: index + width };", 1),
    ],
    "🔨️modules/⬇️lift/🧩️lifter-and-document/📜️page-content/🧺️program/🦀️.rs": [
        ("        let unmapped_text = text_operator(operator) && self.source.selected_font().and_then(|name| fonts.controlled_type1(name)).is_none();", "        let unmapped_text = text_operator(operator) && !self.source.selected_font()\n            .is_some_and(|name| fonts.controlled_type1(name).is_some() || fonts.controlled_code_width(name).is_some());", 1),
    ],
}


def stage(base):
    OUT.mkdir(parents=True, exist_ok=True)
    manifest = {}
    for index, (relative, edits) in enumerate(EDITS.items()):
        source = base / relative
        before = source.read_text()
        after = before
        for old, new, count in edits:
            assert after.count(old) == count, (relative, old)
            after = after.replace(old, new)
        replacement = OUT / f"{index}.rs"
        replacement.write_text(after)
        manifest[relative] = str(replacement)
        patch = difflib.unified_diff(before.splitlines(True), after.splitlines(True), "a/" + str(source), "b/" + str(source))
        (OUT / f"{index}.patch").write_text("".join(patch))
    (OUT / "stages.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n")


if __name__ == "__main__":
    stage(Path(sys.argv[1]))
