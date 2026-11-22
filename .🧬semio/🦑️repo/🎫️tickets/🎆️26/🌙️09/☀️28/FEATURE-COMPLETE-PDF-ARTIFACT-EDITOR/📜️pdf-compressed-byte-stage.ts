/** 🧬 Stages segmented controlled filter and compressed xref reads. */
import assert from "node:assert/strict";
import { join, resolve } from "node:path";

const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules";
const generated = join(import.meta.dir, "🗑️generated");
type Candidate = [string, string, string];
const overlay = new Map<string, Candidate>(
  (await Bun.file(join(generated, "pdf-graph-byte-overlay.json")).json() as Candidate[]).map(row => [row[0], row]),
);
const digest = (bytes: string) => new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
const filters = "🗜️filters/";
const span = "🔤️lexer/📏️stream-span/🦀️.rs";
const xref = "🔗️xref/";
const stream = filters + "🪣️stream-data/";
const flate = filters + "🌀️bounded-flate/";
const generic = "source: &(impl ByteReadSource + ?Sized)";

async function stage(relative: string, edits: Array<[string, string, number?]>) {
  const path = join(root, relative);
  const live = await Bun.file(path).text();
  const previous = overlay.get(path);
  const before = digest(live);
  if (previous) assert.equal(before, previous[1], path);
  let body = previous?.[2] ?? live;
  for (const [from, to, count = 1] of edits) {
    assert.equal(body.split(from).length - 1, count, relative + " expected " + from);
    body = body.replaceAll(from, to);
  }
  assert(Buffer.byteLength(resolve(path)) <= 256, path);
  overlay.set(path, [path, before, body]);
}
await stage(span, [
  ["    pub(super) fn advance_bytes", "    /// 🔍️ Scans a framed extent through its stable bounded byte reader.\n    pub fn advance_bytes"],
]);
await stage(flate + "🦀️.rs", [
  ["use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};", "use semio_framework_value::{ByteReadSource, RetirementDemand, ValueError, ValueRefusalKind};"],
  ["source: &[u8], output:", generic + ", output:", 2],
  ["length != source.len()", "length != source.byte_len()"],
  ["Some(source.len())", "Some(source.byte_len())"],
  ["source.len() - if self.framing", "source.byte_len() - if self.framing"],
  ["            if self.pending_input.is_none() && self.source_offset < raw_end {\n"
    + "                self.pending_input = Some(source[self.source_offset]);\n                self.source_offset += 1;\n            }\n            step.work += 1;",
    "            step.work += 1;\n            if self.pending_input.is_none() && self.source_offset < raw_end {\n"
    + "                self.pending_input = Some(source.byte_at(self.source_offset).ok_or_else(missing_source)?);\n                self.source_offset += 1;\n            }"],
  ["                        let expected = u32::from_be_bytes(source[raw_end..].try_into().expect(\"four-byte RFC 1950 checksum\"));\n"
    + "                        self.source_offset = source.len();",
    "                        let mut checksum = [0; 4];\n"
    + "                        if source.read_bytes(raw_end, &mut checksum) != checksum.len() { return Err(missing_source()); }\n"
    + "                        let expected = u32::from_be_bytes(checksum);\n"
    + "                        self.source_offset = source.byte_len();"],
  ["fn zlib_window(source: &[u8])", "fn missing_source() -> ValueError { ValueError::invariant(\"PDF Flate source lost an admitted byte\") }\n\nfn zlib_window(" + generic + ")"],
  ["if source.len() < 6", "if source.byte_len() < 6"],
  ["    let cmf = source[0];\n    let flg = source[1];",
    "    let cmf = source.byte_at(0).ok_or_else(missing_source)?;\n    let flg = source.byte_at(1).ok_or_else(missing_source)?;"],
]);
await stage(flate + "🧮️predictor/🦀️.rs", [
  ["source: &[u8], output:", generic + ", output:", 3],
]);
await stage(filters + "🧬️bounded-lzw/🦀️.rs", [
  ["use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};", "use semio_framework_value::{ByteReadSource, RetirementDemand, ValueError, ValueRefusalKind};"],
  ["source: &[u8], output:", generic + ", output:", 2],
  ["source.len()", "source.byte_len()", 2],
  ["let byte = *source.get(self.input).ok_or_else(|| format_error(\"truncated PDF LZW stream before EOD\"))?;",
    "let byte = source.byte_at(self.input).ok_or_else(|| {\n"
    + "                    if self.input < source.byte_len() { ValueError::invariant(\"PDF LZW source lost an admitted byte\") }\n"
    + "                    else { format_error(\"truncated PDF LZW stream before EOD\") }\n"
    + "                })?;"],
]);
for (const path of ["✂️packet-filters/🦀️.rs", "🔡️ascii85/🦀️.rs"]) {
  await stage(stream + path, [
    ["#[derive(Clone, Copy)]", "use semio_framework_value::ByteReadSource;\n\n#[derive(Clone, Copy)]"],
    ["source: &[u8],", generic + ","],
    ["source.get(self.input).copied()", "source.byte_at(self.input)"],
  ]);
}
await stage(stream + "✂️packet-filters/🦀️.rs", [
  ["    pub(super) complete: bool,", "    pub(super) complete: bool,\n    pub(super) source_changed: bool,"],
  ["            let byte = source.byte_at(self.input);",
    "            let byte = source.byte_at(self.input);\n"
    + "            if byte.is_none() && self.input < source.byte_len() {\n"
    + "                return SimpleFilterReceipt { input: self.input, written, work, complete: false, source_changed: true };\n"
    + "            }"],
  ["complete: self.complete && self.pending.is_none() }", "complete: self.complete && self.pending.is_none(), source_changed: false }"],
]);
await stage(stream + "🔡️ascii85/🦀️.rs", [
  ["    pub(super) done: Result<bool, &'static str>,", "    pub(super) done: Result<bool, &'static str>,\n    pub(super) source_changed: bool,"],
  ["            let byte = source.byte_at(self.input);\n            work += 1;",
    "            let byte = source.byte_at(self.input);\n            work += 1;\n"
    + "            if byte.is_none() && self.input < source.byte_len() {\n"
    + "                return Ascii85Receipt { input: self.input, written, work, done: Ok(false), source_changed: true };\n"
    + "            }"],
  ["Ascii85Receipt { input: self.input, written, work, done }", "Ascii85Receipt { input: self.input, written, work, done, source_changed: false }"],
]);
await stage(stream + "🦀️.rs", [
  ["            let coded = || source.contiguous_bytes().ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, \"PDF encoded stream decoder requires contiguous input\"));\n", ""],
  ["decoder.advance(coded()?, &mut self.staging,", "decoder.advance(source, &mut self.staging,", 5],
  ["                    receipt.work += step.work;", "                    receipt.work += step.work;\n                    if step.source_changed { return Err(ValueError::invariant(\"PDF packet stream source lost an admitted byte\")); }", 2],
]);
await stage(xref + "💧️stream/🧭️section/🦀️.rs", [
  ["use semio_framework_value::{ValueError, ValueRefusalKind};", "use semio_framework_value::{ByteReadRange, ByteReadSource, ValueError, ValueRefusalKind};"],
  ["source: &[u8], grant: CosGrant", generic + ", grant: CosGrant", 2],
  ["self.frame.advance(source, grant, false)", "self.frame.advance_bytes(|position| source.byte_at(position), grant, false)"],
  ["self.plan_fields(source.len(),", "self.plan_fields(source.byte_len(),"],
  ["cursor.advance(source, grant.work)", "cursor.advance_bytes(source.byte_len(), |position| source.byte_at(position), grant.work)"],
  ["                if let Some(span) = result.span { self.phase = Phase::Decode(span); }",
    "                if result.source_changed { return Err(XrefStreamError::Structure(\"xref stream source lost an admitted byte\")); }\n"
    + "                if let Some(span) = result.span { self.phase = Phase::Decode(span); }"],
  ["let raw = source.get(span.start..span.end).ok_or(XrefStreamError::Structure(\"xref stream source changed\"))?;",
    "let raw = span.end.checked_sub(span.start).and_then(|length| ByteReadRange::new(source, span.start, length)).ok_or(XrefStreamError::Structure(\"xref stream source changed\"))?;"],
  ["self.decoder.advance(dict, raw, stream_grant(grant), false)", "self.decoder.advance(dict, &raw, stream_grant(grant), false)"],
]);
await stage(xref + "🧭️revisions/🦀️.rs", [
  ["use semio_framework_value::{ByteReadSource, ValueError, ValueRefusalKind};", "use semio_framework_value::ByteReadSource;"],
  ["            Self::Stream(cursor) => match source.contiguous_bytes() {\n"
    + "                Some(bytes) => stream(cursor.advance(bytes, grant, false), target),\n"
    + "                None => XrefRevisionReceipt { event: Err(XrefStreamError::Decode(ValueError::literal(ValueRefusalKind::OwnershipLimit, \"PDF xref stream requires contiguous input\"))), ..response(XrefRevisionEvent::More) },\n"
    + "            },",
    "            Self::Stream(cursor) => stream(cursor.advance(source, grant, false), target),"],
]);
await Bun.write(join(generated, "pdf-compressed-byte-overlay.json"), JSON.stringify([...overlay.values()]));
console.log("[DEBUG] Staged exact controlled compressed PDF reads across", overlay.size, "source paths");

