/** 🧪️ Links the staged native graph and identity decoder to the segmented source law. */
import assert from "node:assert/strict";
import { join } from "node:path";

const ticket = import.meta.dir;
const generated = join(ticket, "🗑️generated");
const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules";
const basePath = join(root, "..");
const red = process.argv[2] === "red";
const compressed = process.argv[2] === "compressed";
type Candidate = [string, string, string];
const overlay = new Map(
  (await Bun.file(join(generated, red ? "pdf-cos-byte-overlay.json" : compressed ? "pdf-compressed-byte-overlay.json" : "pdf-graph-byte-overlay.json")).json() as Candidate[]).map(([path, , body]) => [path, body]),
);
const get = async (path: string) => overlay.get(path) ?? await Bun.file(path).text();
const read = (relative: string) => get(join(root, relative));
const extract = (value: string) => value.split("\n#[cfg(test)]")[0]!.replace(/^\/\/!.*\n/gmu, "");
const module = async (name: string, relative: string, omitPaths = false) => {
  let content = extract(await read(relative));
  if (omitPaths) content = content.replace(/#\[path = "[^"]+"\]\nmod \w+;/gu, "");
  return "mod " + name + " {\n" + content + "\n}";
};
const prior = await Bun.file(join(generated, "pdf-cos-green.rs")).text();
let base = prior.slice(0, prior.indexOf("use std::{cell::Cell"));
assert(base.length > 0);
const lexerEnd = "pub use frame::{IndirectFrameCursor, IndirectFrameStatus};";
assert(base.includes(lexerEnd));
base = base.replace(lexerEnd, [
  lexerEnd,
  await module("stream_span", "🔤️lexer/📏️stream-span/🦀️.rs"),
  "pub use stream_span::{StreamSpan, StreamSpanCursor};",
  await module("normal", "🔤️lexer/🧭️normal-object/🦀️.rs"),
  "pub use normal::{NormalObject, NormalObjectStatus, PdfNormalObjectCursor};",
  await module("indirect_length", "🔤️lexer/📐️indirect-length/🦀️.rs"),
  "pub use indirect_length::PdfIndirectLengthCursor;",
  await module("dictionary", "🔤️lexer/🧹️stream-dictionary/🦀️.rs"),
  "pub use dictionary::PdfStreamDictionaryCursor;",
  await module("header", "🔤️lexer/🪪️pdf-header/🦀️.rs"),
  "pub use header::{PdfHeader, PdfHeaderCursor, PdfHeaderStep};",
  await module("section_probe", "🔤️lexer/🧭️xref-section/🦀️.rs"),
  "pub use section_probe::{XrefSectionKind, XrefSectionProbe};",
].join("\n"));
base = base.replace("mod lexer {", "pub mod lexer {")
  .replace("let offset = index.checked_sub(PREFIX)?;", "let Some(offset) = index.checked_sub(PREFIX) else { return Some(b' '); };")
  .replace("pub mod schema {", "pub mod modules { pub use crate::{filters, lexer, xref}; }\npub mod schema {")
  .replace("pub mod io {", "pub mod io { pub mod error { pub use crate::{malformed, PResult, PdfEngineError}; }");
const metadata = await get(join(basePath, "🧬️schema/📸️snapshot/📐️model/📄️document-metadata/🦀️.rs"));
const date = metadata.match(/pub struct PdfDate \{[\s\S]*?\n\}/u)?.[0];
assert(date);
base = base.replace("pub struct PdfDate;", date.replace(/^\s*#\[value[^\n]+\n/gmu, "\n"))
  .replace("pub mod snapshot { pub use crate::model::*; }", "pub mod snapshot { pub use crate::{document_metadata::PdfDate, model::*}; }")
  .replace("pub mod text_string { pub fn encode_text_string(text: &str) -> Vec<u8> { text.as_bytes().to_vec() } }", "pub use crate::text_string;")
  .replace("pub mod text { pub mod snapshot { pub mod date { pub fn print_pdf_date(_: &crate::document_metadata::PdfDate) -> String { unreachable!() } } } }",
    "pub mod text { pub mod snapshot { pub use crate::{date, retained_text}; } }");
const classicAt = base.indexOf("mod classic {");
const sectionAt = base.indexOf("mod section {", classicAt);
assert(classicAt >= 0 && sectionAt > classicAt);
base = base.slice(0, classicAt) + await module("classic", "🔗️xref/📑️classic/🦀️.rs") + "\n" + base.slice(sectionAt);
const trait = "🧰️framework/🔨️modules/🌱️value/📦️paged/📖️read/🎟️.rs";
const rangeStart = base.indexOf("pub trait ByteReadSource");
const rangeEnd = base.indexOf("const PREFIX:");
assert(rangeStart >= 0 && rangeEnd > rangeStart);
base = base.slice(0, rangeStart) + extract(await get(trait)) + "\n" + base.slice(rangeEnd);
const xref = "🔗️xref/";
const stream = xref + "💧️stream/";
const codec = "🗜️filters/";
const streamData = codec + "🪣️stream-data/";
let declarations = await Bun.file(join(ticket, "📜️pdf-graph-byte-link-types.rs")).text();
if (compressed) {
  declarations = declarations.split("#[derive(Clone, Copy)]\npub struct FlateGrant")[0]!;
  const deflate = "🧰️framework/🔨️modules/🗜️deflate";
  const deflateParts = await Promise.all(Object.entries({
    errors_and_tables: "📋️errors-and-tables",
    bit_io_and_huffman: "🔁️bit-io-and-huffman",
    inflate_state: "🧱️inflate-state",
    inflater: "🚰️inflater",
  }).map(async ([name, path]) => "mod " + name + " {\n" + extract(await get(join(deflate, path, "🦀️.rs"))) + "\n}"));
  const retirement = (await get("🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/📏️physical/🦀️.rs"))
    .match(/pub struct RetirementDemand \{[\s\S]*?\n\}/u)?.[0];
  assert(retirement);
  declarations += [
    "#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]\n" + retirement,
    "extern crate self as semio_framework_deflate;",
    "mod deflate { use crate::{ValueError,ValueRefusalKind};",
    ...deflateParts,
    "pub use errors_and_tables::*; use bit_io_and_huffman::*; pub use inflate_state::*; pub use inflater::*;",
    "}",
    "pub use deflate::*;",
    (await module("flate", codec + "🌀️bounded-flate/🦀️.rs", true)).replace(
      "pub use predictor::PdfPredictorCursor;",
      (await module("predictor", codec + "🌀️bounded-flate/🧮️predictor/🦀️.rs")) + "\npub use predictor::PdfPredictorCursor;",
    ),
    "pub use flate::*;",
    await module("lzw", codec + "🧬️bounded-lzw/🦀️.rs"),
    "pub use lzw::PdfLzwCursor;",
    "pub use xref::{XrefStreamRowCursor,XrefStreamSectionCursor,XrefStreamReceipt,XrefStreamEvent};",
  ].join("\n");
}
const graphInput = extract(await read(xref + "📥️retained-graph/🦀️.rs"));
const graph = graphInput
  .replace('#[path = "📏️compressed-length/🦀️.rs"]\nmod compressed_length;', await module("compressed_length", xref + "📥️retained-graph/📏️compressed-length/🦀️.rs"));
const xrefStreamEnum = extract(await read(stream + "📦️packages/🦀️rust/🦀️.rs")).match(/#\[derive\([^\n]+\)\]\npub enum XrefEntry \{[\s\S]*?\n\}/u)?.[0];
assert(xrefStreamEnum);
const index = (await module("index", stream + "📇️index/🦀️.rs", true)).replace(
  "pub use revision_merge::RevisionIndex;",
  (await module("revision_merge", stream + "📇️index/🧩️revision-merge/🦀️.rs")) + "\npub use revision_merge::RevisionIndex;",
);
const types = await Bun.file(join(ticket, "🔣️segmented-pdf-source.json")).json();
const law = (await Bun.file(join(ticket, "📜️pdf-graph-byte-law.rs")).text())
  .replaceAll("$SOURCE_BYTES", String(types.sourceBytes))
  .replaceAll("$IMAGE_SIDE", String(types.imageSide))
  .replaceAll("$XREF_STREAM_FIXTURE", join(process.cwd(), root, "../🧪️tests/🧫️fixtures/🔗️incremental-free/🌀️flate.pdf"));
const preparedLaw = (await Bun.file(join(ticket, "📜️pdf-prepared-byte-law.rs")).text())
  .replaceAll("$CLASSIC_INCREMENTAL_FIXTURE", join(process.cwd(), root, "../🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf"))
  .replaceAll("$XREF_STREAM_FIXTURE", join(process.cwd(), root, "../🧪️tests/🧫️fixtures/🔗️incremental-free/🌀️flate.pdf"))
  .replaceAll("$COMPRESSED_LENGTH_FIXTURE", join(process.cwd(), root, "../🧪️tests/🧫️fixtures/📦️object-stream/📏️length.pdf"));
const textPath = join(basePath, "🚪️io");
const writerKeys = (await read(xref + "🦀️.rs")).match(/^pub const WRITER_TRAILER_KEYS:.*$/mu)?.[0];
assert(writerKeys);
await Bun.write(join(generated, red ? "pdf-graph-byte-red.rs" : compressed ? "pdf-compressed-byte-link.rs" : "pdf-graph-byte-link.rs"), [
  base, declarations,
  "const SEGMENTED_CODECS: bool = " + compressed + ";",
  "pub mod text_string {\n" + extract(await get(join(textPath, "🔤️text-string/🦀️.rs"))) + "\n}",
  "pub mod date {\n" + extract(await get(join(textPath, "📝️text/📸️snapshot/📅️date/🦀️.rs"))) + "\n}",
  "pub mod retained_text {\n" + extract(await get(join(textPath, "📝️text/📸️snapshot/🪪️retained-text/🦀️.rs"))) + "\n}",
  "pub mod filters { use crate::{PdfDictEntry, PdfObject, PdfStreamFilter, ValueError, ValueRefusalKind, PdfEngineError, PResult, malformed};",
  "use crate::{PdfPredictor, PdfFlateCursor, PdfPredictorCursor, PdfLzwCursor, FlateGrant, FlateStatus};",
  (await module("stream_data", streamData + "🦀️.rs", true))
    .replace("use packet_filters::SimpleFilterCursor;", (await module("packet_filters", streamData + "✂️packet-filters/🦀️.rs")) + "\nuse packet_filters::SimpleFilterCursor;")
    .replace("use ascii85::Ascii85Cursor;", (await module("ascii85", streamData + "🔡️ascii85/🦀️.rs")) + "\nuse ascii85::Ascii85Cursor;"),
  "pub use stream_data::*;",
  await module("decoded_stream", codec + "🎚️decoded-stream/🦀️.rs"),
  "pub use decoded_stream::*;",
  "pub fn decode_stream(_: &[PdfDictEntry], _: &[u8]) -> PResult<(Vec<u8>, Vec<PdfStreamFilter>)> { unreachable!() } }",
  "pub mod xref {",
  xrefStreamEnum,
  writerKeys,
  "pub use crate::{ClassicSectionCursor, ClassicSectionEvent, ClassicSectionReceipt, FooterCursor, FooterStep};",
  ...(compressed ? [
    await module("stream_rows", stream + "🦀️.rs"),
    "pub use stream_rows::*;",
    await module("stream_section", stream + "🧭️section/🦀️.rs"),
    "pub use stream_section::*;",
  ] : [
    "#[derive(Debug)] pub enum XrefStreamError { Structure(&'static str), Decode(crate::ValueError) }",
    "pub use crate::{XrefStreamSectionCursor, XrefStreamReceipt, XrefStreamEvent};",
  ]),
  index, "pub use index::*;",
  await module("offset_validation", xref + "🔎️offset-validation/🦀️.rs"), "pub use offset_validation::*;",
  await module("revisions", xref + "🧭️revisions/🦀️.rs"), "pub use revisions::*;",
  await module("assembly", xref + "🧵️assembly/🦀️.rs"), "pub use assembly::*;",
  await module("trailer_fold", xref + "📨️trailer-fold/🦀️.rs"), "pub use trailer_fold::*;",
  await module("structured", xref + "🛤️structured-file/🦀️.rs"), "pub use structured::*;",
  await module("structure", xref + "🌐️file-structure/🦀️.rs"), "pub use structure::*;",
  await module("normal_projection", xref + "📦️normal-projection/🦀️.rs"), "pub use normal_projection::*;",
  await module("file_order", xref + "📚️file-order/🦀️.rs"), "pub use file_order::*;",
  await module("materialization", xref + "🗃️graph-materialization/🦀️.rs"), "pub use materialization::*;",
  await module("object_stream", stream + "📦️object-stream/🦀️.rs"), "pub use object_stream::*;",
  "mod graph {", graph, preparedLaw, "}", "pub use graph::*;",
  "}",
  "use xref::XrefEntry;",
  compressed ? await Bun.file(join(ticket, "📜️pdf-compressed-byte-law.rs")).text() : "fn exercise_codecs() {}",
  law,
].join("\n"));
console.log("[DEBUG] Emitted focused graph and stream link for the composed candidate");

