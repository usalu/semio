/** 🧱️ Stages segmented typed COS and classic xref trailer parsing. */
import assert from "node:assert/strict";
import { join, resolve } from "node:path";

const ticket = import.meta.dir;
const generated = join(ticket, "🗑️generated");
const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules";
const mode = process.argv[2];
assert(mode === "red" || mode === "green");
type Candidate = [path: string, preimage: string, body: string];
const earlier = await Bun.file(join(generated, "pdf-xref-byte-overlay.json")).json() as Candidate[];
const overlay = new Map(earlier.map(row => [row[0], row]));
const digest = (body: string) => new Bun.CryptoHasher("sha256").update(body).digest("hex");
const cos = "🔤️lexer/🧱️containers";
const strings = "🔤️lexer/🔤️strings/🦀️.rs";
const section = "🔗️xref/📋️section/🦀️.rs";
const token = cos + "/🔡️tokens/🦀️.rs";
const frame = "🔤️lexer/📦️object-frame/🦀️.rs";
const prefix = "🔤️lexer/📦️stream-prefix/🦀️.rs";
const indirect = "🔤️lexer/🪪️indirect-header/🦀️.rs";
const sourceOf = (relative: string) => Bun.file(resolve(root, relative)).text();
const callback = "byte_at: impl Fn(usize) -> Option<u8>";
const adapter = "|position| source.get(position).copied()";
const edits: Record<string, [string, string][]> = {
  [cos + "/🦀️.rs"]: [
    [
      "    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> CosReceipt {\n        let mut turn",
      "    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> CosReceipt {\n"
      + "        self.advance_bytes(" + adapter + ", grant, cancelled)\n"
      + "    }\n\n"
      + "    /// 🔹️ Advances the same parser over stable caller-owned segmented bytes.\n"
      + "    pub fn advance_bytes(&mut self, " + callback + ", grant: CosGrant, cancelled: bool) -> CosReceipt {\n"
      + "        let mut turn",
    ],
    ["self.step(source, &mut turn)", "self.step(&byte_at, &mut turn)"],
    ["fn step(&mut self, source: &[u8], turn:", "fn step(&mut self, byte_at: &impl Fn(usize) -> Option<u8>, turn:"],
    ["self.reader.ends_reference_lookahead(source) && self.promote_integer()", "self.integers[0].is_some() && self.reader.ends_reference_lookahead(byte_at) && self.promote_integer()"],
    ["self.reader.step(source, limit,", "self.reader.step(byte_at, limit,"],
  ],
  [token]: [
    ["    scan: Scan,", "    scan: Scan,\n    atom: [u8; MAX_SCALAR_BYTES],"],
    ["Self { position, scan: Scan::Idle }", "Self { position, scan: Scan::Idle, atom: [0; MAX_SCALAR_BYTES] }"],
    ["source: &[u8], scalar_limit:", "byte_at: &impl Fn(usize) -> Option<u8>, scalar_limit:"],
    ["let byte = source.get(self.position).copied();", "let byte = match self.scan {\n            Scan::Angle(_) | Scan::Bytes { .. } => None,\n            _ => byte_at(self.position),\n        };"],
    ["source.get(self.position + 1) == Some(&b'<')", "self.position.checked_add(1).and_then(byte_at) == Some(b'<')"],
    ["source.get(self.position + 1) == Some(&b'>')", "self.position.checked_add(1).and_then(byte_at) == Some(b'>')"],
    [
      "                if byte.is_some_and(|b| !is_ws(b) && !is_delim(b)) {\n"
        + "                    if self.position - *start >= scalar_limit { return Err(\"PDF COS scalar token limit exceeded\"); }\n"
        + "                    self.position += 1;\n"
        + "                    return Ok(None);\n"
        + "                }\n"
        + "                let raw = source.get(*start..self.position).ok_or(\"PDF COS source changed\")?;",
      "                let length = self.position - *start;\n"
        + "                if let Some(byte) = byte.filter(|&byte| !is_ws(byte) && !is_delim(byte)) {\n"
        + "                    if length >= scalar_limit { return Err(\"PDF COS scalar token limit exceeded\"); }\n"
        + "                    self.atom[length] = byte;\n"
        + "                    self.position += 1;\n"
        + "                    return Ok(None);\n"
        + "                }\n"
        + "                let raw = &self.atom[..length];",
    ],
    ["parser.advance(source, &mut scratch, 1)", "parser.advance_bytes(byte_at, &mut scratch, 1)"],
    ["self, source: &[u8]) -> bool", "self, byte_at: &impl Fn(usize) -> Option<u8>) -> bool"],
    ["source.get(self.position).is_none_or(|&byte|", "byte_at(self.position).is_none_or(|byte|"],
  ],
  [strings]: [
    [
      "    pub fn advance(&mut self, source: &[u8], output: &mut [u8], budget: usize) -> StringReceipt {\n        let mut written",
      "    pub fn advance(&mut self, source: &[u8], output: &mut [u8], budget: usize) -> StringReceipt {\n"
      + "        self.advance_bytes(" + adapter + ", output, budget)\n"
      + "    }\n\n"
      + "    pub(super) fn advance_bytes(&mut self, " + callback + ", output: &mut [u8], budget: usize) -> StringReceipt {\n"
      + "        let mut written",
    ],
    ["let Some(&byte) = source.get(self.position)", "let Some(byte) = byte_at(self.position)"],
  ],
  [frame]: [
    [
      "    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> IndirectFrameReceipt {\n        let mut receipt",
      "    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> IndirectFrameReceipt {\n"
      + "        self.advance_bytes(" + adapter + ", grant, cancelled)\n"
      + "    }\n\n"
      + "    /// 🧷️ Frames the same indirect object using a bounded reader for its original bytes.\n"
      + "    pub fn advance_bytes(&mut self, " + callback + ", grant: CosGrant, cancelled: bool) -> IndirectFrameReceipt {\n"
      + "        let mut receipt",
    ],
    ["header.advance(" + adapter + ", grant.work)", "header.advance(&byte_at, grant.work)"],
    ["released_bytes } = cursor.advance(source, grant, false);", "released_bytes } = cursor.advance_bytes(&byte_at, grant, false);"],
    ["prefix.advance(source, grant.work)", "prefix.advance_bytes(&byte_at, grant.work)"],
  ],
  [prefix]: [
    [
      "    pub fn advance(&mut self, source: &[u8], grant: usize) -> (usize, StreamPrefixStep) {\n        let mut work",
      "    pub fn advance(&mut self, source: &[u8], grant: usize) -> (usize, StreamPrefixStep) {\n"
      + "        self.advance_bytes(" + adapter + ", grant)\n"
      + "    }\n\n"
      + "    pub(super) fn advance_bytes(&mut self, " + callback + ", grant: usize) -> (usize, StreamPrefixStep) {\n"
      + "        let mut work",
    ],
    ["source.get(self.position).copied()", "byte_at(self.position)"],
  ],
  [section]: [
    ["source: &[u8], grant: CosGrant", callback + ", grant: CosGrant"],
    ["trailer.advance(source, grant, false)", "trailer.advance_bytes(&byte_at, grant, false)"],
    ["self.rows.advance(" + adapter + ", grant.work)", "self.rows.advance(byte_at, grant.work)"],
  ],
  "🔗️xref/🧭️revisions/🦀️.rs": [
    ["Self::Table(cursor) => table(cursor.advance(source, grant, false), target)", "Self::Table(cursor) => table(cursor.advance(" + adapter + ", grant, false), target)"],
  ],
  "🔗️xref/🧪️tests/🔬️unit/🦀️.rs": [
    ["let turn = cursor.advance(source, grant, false);", "let turn = cursor.advance(" + adapter + ", grant, false);"],
  ],
};

for (const [relative, transforms] of Object.entries(edits)) {
  const path = join(root, relative);
  const live = await sourceOf(relative);
  const previous = overlay.get(path);
  const before = digest(live);
  if (previous) assert.equal(previous[1], before, relative + " changed");
  let candidate = previous?.[2] ?? live;
  if (mode === "green") {
    for (const [original, updated] of transforms) {
      assert.equal(candidate.split(original).length, 2, relative + " expected " + original);
      candidate = candidate.replace(original, updated);
    }
  }
  assert(Buffer.byteLength(resolve(path)) <= 256, relative);
  overlay.set(path, [path, before, candidate]);
}
if (mode === "green") {
  await Bun.write(join(generated, "pdf-cos-byte-overlay.json"), JSON.stringify([...overlay.values()]));
}
const chosen = async (path: string) => overlay.get(join(root, path))?.[2] ?? await sourceOf(path);
const extract = (value: string) => value.split("\n#[cfg(test)]")[0]!.replace(/^\/\/!.*\n/gmu, "");
const models = await Bun.file(resolve(root, "../🧬️schema/📸️snapshot/📐️model/🔗️object-model/🦀️.rs")).text();
const plainModel = models
  .replace(/^\s*#\[(canonical_json|value)[^\n]+\n/gmu, "\n")
  .replace(/#\[value\([^\]]*\)\]\s*/gu, "")
  .replaceAll(/(?:semio_framework_value::(?:RetireOwned|CanonicalJsonTree|RetainedClone)|value_derive::(?:ToValue|FromValue)), /gu, "")
  .replaceAll(/, (?:semio_framework_value::(?:RetireOwned|CanonicalJsonTree|RetainedClone)|value_derive::(?:ToValue|FromValue))/gu, "");
const earlierLaw = await Bun.file(join(generated, "pdf-xref-green.rs")).text();
const region = (start: string, end: string) => {
  assert(earlierLaw.includes(start) && earlierLaw.includes(end), start);
  return earlierLaw.slice(earlierLaw.indexOf(start), earlierLaw.indexOf(end));
};
const fixture = await Bun.file(join(ticket, "🔣️segmented-pdf-source.json")).json();
const core = extract(await chosen(cos + "/🦀️.rs"))
  .replace(/#\[path = "[^"]+"\]\nmod (buffers|tokens);/gu, "");
const trailerLaw = (await Bun.file(join(ticket, "📜️pdf-cos-byte-law.rs")).text())
  .replaceAll("$SOURCE_BYTES", String(fixture.sourceBytes))
  .replaceAll("$XREF_OFFSET", String(fixture.xrefOffset));
await Bun.write(join(generated, "pdf-cos-" + mode + ".rs"), [
  'extern crate self as semio_s_artifact_stdio_pdf_lexer;',
  'extern crate self as semio_s_artifact_stdio_pdf_model;',
  'extern crate self as semio_s_artifact_stdio_pdf_snapshot_2;',
  'pub mod document_metadata { #[derive(Clone, Debug, PartialEq)] pub struct PdfDate; }',
  'pub mod standards { pub mod v1_7 { pub mod subsets { pub mod base {',
  'pub mod schema { pub mod snapshot { pub use crate::model::*; } }',
  'pub mod io {',
  'pub mod text_string { pub fn encode_text_string(text: &str) -> Vec<u8> { text.as_bytes().to_vec() } }',
  'pub mod text { pub mod snapshot { pub mod date { pub fn print_pdf_date(_: &crate::document_metadata::PdfDate) -> String { unreachable!() } } } }',
  '}',
  '} } } }',
  "mod model {\n" + extract(plainModel) + "\n}",
  "pub use model::{ObjRef, PdfDecimal, PdfDictEntry, PdfObject};",
  "mod lexer { use crate::{is_delim, is_ws};",
  core,
  "mod buffers {\n" + extract(await chosen(cos + "/📦️buffers/🦀️.rs")) + "\n}",
  "mod tokens {\n" + extract(await chosen(token)) + "\n}",
  "mod strings {\n" + extract(await chosen(strings)) + "\n}",
  "pub use strings::{PdfByteStringCursor, StringStep};",
  "mod prefix {\n" + extract(await chosen(prefix)) + "\n}",
  "pub use prefix::{StreamPrefixCursor, StreamPrefixStep};",
  "mod indirect {\n" + extract(await chosen(indirect)) + "\n}",
  "pub use indirect::{HeaderStep, IndirectHeaderCursor};",
  "mod frame {\n" + extract(await chosen(frame)) + "\n}",
  "pub use frame::{IndirectFrameCursor, IndirectFrameStatus};",
  "}",
  "pub use lexer::{CosGrant, CosLimits, CosStatus, CosReceipt, PdfCosCursor, PdfByteStringCursor, StringStep, IndirectFrameCursor, IndirectFrameStatus};",
  region("fn is_ws(", "const PREFIX:").split("\n")[0]!,
  region("fn is_delim(", "const PREFIX:").split("\n")[0]!,
  "fn hex_val(value: u8) -> u8 { (value as char).to_digit(16).unwrap() as u8 }",
  "mod classic {\n" + extract(await chosen("🔗️xref/📑️classic/🦀️.rs")) + "\n}",
  "mod section {\n" + extract(await chosen(section)) + "\n}",
  "mod footer {\n" + extract(await chosen("🔗️xref/🔎️footer/🦀️.rs")) + "\n}",
  "mod links {\n" + extract(await chosen("🔤️lexer/🪢️xref-links/🦀️.rs")) + "\n}",
  region("pub trait ByteReadSource", "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum XrefEntry"),
  region("const PREFIX:", "fn charge("),
  trailerLaw,
].join("\n"));
console.log("[DEBUG] Validated segmented classic trailer and COS source edits across", overlay.size, "paths for", mode);

