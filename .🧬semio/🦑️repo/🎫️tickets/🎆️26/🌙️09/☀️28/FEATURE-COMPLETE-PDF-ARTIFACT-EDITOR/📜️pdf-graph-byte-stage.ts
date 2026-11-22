/** 🗃️ Stages range-backed normal projection and paid graph import streams. */
import assert from "node:assert/strict";
import { join, resolve } from "node:path";

const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules";
const generated = join(import.meta.dir, "🗑️generated");
type Candidate = [path: string, preimage: string, body: string];
const overlay = new Map<string, Candidate>(
  (await Bun.file(join(generated, "pdf-cos-byte-overlay.json")).json() as Candidate[]).map(row => [row[0], row]),
);
const trait = "🧰️framework/🔨️modules/🌱️value/📦️paged/📖️read/🎟️.rs";
const normal = "🔤️lexer/🧭️normal-object/🦀️.rs";
const span = "🔤️lexer/📏️stream-span/🦀️.rs";
const indirect = "🔤️lexer/📐️indirect-length/🦀️.rs";
const stream = "🗜️filters/🪣️stream-data/🦀️.rs";
const decoded = "🗜️filters/🎚️decoded-stream/🦀️.rs";
const projection = "🔗️xref/📦️normal-projection/🦀️.rs";
const graph = "🔗️xref/📥️retained-graph/🦀️.rs";
const compressed = "🔗️xref/📥️retained-graph/📏️compressed-length/🦀️.rs";
const revisions = "🔗️xref/🧭️revisions/🦀️.rs";
const assembly = "🔗️xref/🧵️assembly/🦀️.rs";
const structured = "🔗️xref/🛤️structured-file/🦀️.rs";
const fileStructure = "🔗️xref/🌐️file-structure/🦀️.rs";
const generic = "source: &(impl ByteReadSource + ?Sized)";
const byteAt = "byte_at: impl Fn(usize) -> Option<u8>";
const sliceReader = "|position| source.get(position).copied()";
const dataReader = "|position| data.get(position).copied()";
const sourceReader = "|position| source.byte_at(position)";
const digest = (body: string) => new Bun.CryptoHasher("sha256").update(body).digest("hex");
const edits: [string, (edit: (a: string, b: string, count?: number) => void) => void][] = [
  [trait, change => {
    change(" fn byte_at(&self,index:usize)->Option<u8>;", " fn byte_at(&self,index:usize)->Option<u8>;\n /// 🗂️ Borrows existing whole-span storage when it is already contiguous.\n fn contiguous_bytes(&self)->Option<&[u8]>{None}");
    change("fn byte_len(&self)->usize{self.len()}fn byte_at", "fn byte_len(&self)->usize{self.len()}fn contiguous_bytes(&self)->Option<&[u8]>{Some(self)}fn byte_at", 2);
    change("fn byte_len(&self)->usize{N}fn byte_at", "fn byte_len(&self)->usize{N}fn contiguous_bytes(&self)->Option<&[u8]>{Some(self)}fn byte_at");
    change("fn byte_len(&self)->usize{(**self).byte_len()}fn byte_at", "fn byte_len(&self)->usize{(**self).byte_len()}fn contiguous_bytes(&self)->Option<&[u8]>{(**self).contiguous_bytes()}fn byte_at");
    change("fn byte_len(&self)->usize{self.length}fn byte_at", "fn byte_len(&self)->usize{self.length}fn contiguous_bytes(&self)->Option<&[u8]>{self.source.contiguous_bytes()?.get(self.start..self.start+self.length)}fn byte_at");
  }],
  [span, change => {
    change("    pub inspected: usize,", "    pub inspected: usize,\n    pub source_changed: bool,");
    change("    pub fn advance(&mut self, source: &[u8], grant: usize) -> StreamSpanStep {\n        let mut inspected",
      "    pub fn advance(&mut self, source: &[u8], grant: usize) -> StreamSpanStep {\n"
      + "        self.advance_bytes(source.len(), " + sliceReader + ", grant)\n"
      + "    }\n\n"
      + "    pub(super) fn advance_bytes(&mut self, source_len: usize, " + byteAt + ", grant: usize) -> StreamSpanStep {\n"
      + "        let mut inspected");
    change("StreamSpanStep { inspected, span:", "StreamSpanStep { inspected, source_changed: false, span:", 2);
    change("if self.position == source.len()", "if self.position == source_len");
    change("end: source.len(), resume: source.len()", "end: source_len, resume: source_len");
    change("            let byte = source[self.position];\n            inspected += 1;",
      "            inspected += 1;\n            let Some(byte) = byte_at(self.position) else {\n                return StreamSpanStep { inspected, source_changed: true, span: None };\n            };");
  }],
  [normal, change => {
    change("    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> NormalObjectReceipt {\n        if cancelled",
      "    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> NormalObjectReceipt {\n"
      + "        self.advance_bytes(source.len(), " + sliceReader + ", grant, cancelled)\n"
      + "    }\n\n"
      + "    /// 🧲️ Resolves a typed object and exact stream bounds without coalescing the source.\n"
      + "    pub fn advance_bytes(&mut self, source_len: usize, " + byteAt + ", grant: CosGrant, cancelled: bool) -> NormalObjectReceipt {\n"
      + "        if cancelled");
    change("self.frame.advance(source, grant, false)", "self.frame.advance_bytes(&byte_at, grant, false)");
    change("self.plan_length(start, position, source.len(),", "self.plan_length(start, position, source_len,");
    change("cursor.advance(source, grant.work)", "cursor.advance_bytes(source_len, &byte_at, grant.work)");
    change("                if let Some(span) = result.span {", "                if result.source_changed {\n                    Err(\"PDF stream source lost a byte inside its declared extent\")\n                } else if let Some(span) = result.span {");
  }],
  [indirect, change => {
    change("    pub fn advance(&mut self, data: &[u8], mut grant: CosGrant, cancelled: bool) -> CosReceipt {\n        if cancelled",
      "    pub fn advance(&mut self, data: &[u8], grant: CosGrant, cancelled: bool) -> CosReceipt {\n"
      + "        self.advance_bytes(" + dataReader + ", grant, cancelled)\n"
      + "    }\n\n"
      + "    /// 🎚️ Reads the exact indirect-length owner over segmented original bytes.\n"
      + "    pub fn advance_bytes(&mut self, " + byteAt + ", mut grant: CosGrant, cancelled: bool) -> CosReceipt {\n"
      + "        if cancelled");
    change("self.header.advance(" + dataReader + ", grant.work)", "self.header.advance(&byte_at, grant.work)");
    change("match data.get(self.position).copied()", "match byte_at(self.position)");
    change("value.advance(data, grant, false)", "value.advance_bytes(&byte_at, grant, false)");
  }],
  [stream, change => {
    change("use semio_framework_value::{ValueError, ValueRefusalKind};", "use semio_framework_value::{ByteReadSource, ValueError, ValueRefusalKind};");
    change("source: &[u8]", generic, 2);
    change("source.len()", "source.byte_len()", 5);
    change("                    let desired = self.output.capacity().max(64).saturating_mul(2).min(self.maximum_output);",
      "                    let desired = if matches!(self.decoder, Decoder::Identity) {\n"
      + "                        source.byte_len()\n"
      + "                    } else {\n"
      + "                        self.output.capacity().max(64).saturating_mul(2).min(self.maximum_output)\n"
      + "                    };");
    change(
      "            let outcome = match &mut self.decoder {",
      "            let coded = || source.contiguous_bytes().ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, \"PDF encoded stream decoder requires contiguous input\"));\n"
      + "            let outcome = match &mut self.decoder {",
    );
    change("self.staging[..count].copy_from_slice(&source[self.input..self.input + count]);",
      "if source.read_bytes(self.input, &mut self.staging[..count]) != count {\n"
      + "                        receipt.work += count;\n"
      + "                        return Err(ValueError::invariant(\"PDF stream source lost an admitted fragment\"));\n"
      + "                    }");
    change("decoder.advance(source, &mut self.staging,", "decoder.advance(coded()?, &mut self.staging,", 5);
  }],
  [decoded, change => {
    change("use semio_framework_value::{ValueError, ValueRefusalKind};", "use semio_framework_value::{ByteReadSource, ValueError, ValueRefusalKind};");
    change("raw: &[u8], mut grant:", "raw: &(impl ByteReadSource + ?Sized), mut grant:");
    change("        let source = self.input.as_deref().unwrap_or(raw);\n        let mut decoded = decoder.advance(source, grant, false);",
      "        let mut decoded = match self.input.as_ref() {\n"
      + "            Some(source) => decoder.advance(source, grant, false),\n"
      + "            None => decoder.advance(raw, grant, false),\n"
      + "        };");
  }],
  [projection, change => {
    change("use semio_framework_value::ValueRefusalKind;", "use semio_framework_value::{ByteReadRange, ByteReadSource, ValueRefusalKind};");
    change("source: &[u8]", generic, 3);
    change("self.frame.advance(source, grant, false)", "self.frame.advance_bytes(source.byte_len(), |position| source.byte_at(position), grant, false)");
    change("source.get(span.start..span.end)", "span.end.checked_sub(span.start).and_then(|length| ByteReadRange::new(source, span.start, length))");
    change("decoder.advance(dict, raw, stream_grant(grant), false)", "decoder.advance(dict, &raw, stream_grant(grant), false)");
  }],
  [graph, change => {
    change("use crate::standards::v1_7::subsets::base::io::text::snapshot::retained_text", 
      "use semio_framework_value::{ByteReadRange, ByteReadSource};\nuse crate::standards::v1_7::subsets::base::io::text::snapshot::retained_text");
    change("source: &[u8], index: &XrefIndex", generic + ", index: &XrefIndex");
    change("source: &[u8], grant: CosGrant, cancelled: bool) -> PdfByteGraphReceipt", generic + ", grant: CosGrant, cancelled: bool) -> PdfByteGraphReceipt", 2);
    change("fn advance_normal(&mut self, source: &[u8],", "fn advance_normal(&mut self, " + generic + ",");
    change("fn advance_length(&mut self, source: &[u8],", "fn advance_length(&mut self, " + generic + ",");
    change("let mut step = cursor.advance(source, grant, false);", "let mut step = cursor.advance_bytes(|position| source.byte_at(position), grant, false);");
    change("fn raw_stream(&mut self, source: &[u8],", "fn raw_stream(&mut self, " + generic + ",");
    change("source.get(raw.span.start..raw.span.end)", "raw.span.end.checked_sub(raw.span.start).and_then(|length| ByteReadRange::new(source, raw.span.start, length))");
    change("if bytes.len() > self.maximum_stream_bytes", "let length = bytes.byte_len();\n        if length > self.maximum_stream_bytes");
    change("        if raw.bytes.capacity() < bytes.len() {\n"
      + "            if bytes.len() > grant.allocation_bytes { return report(Ok(GraphImportEvent::NeedsAllocation { bytes: bytes.len() }), 0); }\n"
      + "            if raw.bytes.try_reserve_exact(bytes.len()).is_err()", 
      "        if raw.bytes.capacity() < length {\n"
      + "            if length > grant.allocation_bytes { return report(Ok(GraphImportEvent::NeedsAllocation { bytes: length }), 0); }\n"
      + "            if raw.bytes.try_reserve_exact(length).is_err()");
    change("        if raw.bytes.len() < bytes.len() {\n"
      + "            let start = raw.bytes.len();\n"
      + "            let count = (bytes.len() - start).min(grant.work);\n"
      + "            raw.bytes.extend_from_slice(&bytes[start..start + count]);",
      "        if raw.bytes.len() < length {\n"
      + "            let start = raw.bytes.len();\n"
      + "            let count = (length - start).min(grant.work);\n"
      + "            raw.bytes.resize(start + count, 0);\n"
      + "            if bytes.read_bytes(start, &mut raw.bytes[start..]) != count {\n"
      + "                raw.bytes.truncate(start);\n"
      + "                return report(Err(XrefStreamError::Structure(\"raw PDF stream source lost an admitted fragment\")), count);\n"
      + "            }");
    change("let Some(bytes) = source.get(file.origin..) else { return byte_error(\"PDF source origin has changed\"); };",
      "let Some(bytes) = source.byte_len().checked_sub(file.origin).and_then(|length| ByteReadRange::new(source, file.origin, length)) else { return byte_error(\"PDF source origin has changed\"); };");
    change(".advance(bytes, &file.index, grant, false)", ".advance(&bytes, &file.index, grant, false)");
  }],
  [compressed, change => {
    change("use super::{", "use semio_framework_value::ByteReadSource;\n\nuse super::{");
    change("source: &[u8], index: &XrefIndex", generic + ", index: &XrefIndex", 2);
  }],
  [revisions, change => {
    change("use super::{", "use semio_framework_value::{ByteReadSource, ValueError, ValueRefusalKind};\nuse super::{");
    change("source: &[u8], grant: CosGrant", generic + ", grant: CosGrant", 2);
    change("probe.advance(" + sliceReader + ", grant.work)", "probe.advance(" + sourceReader + ", grant.work)");
    change("table(cursor.advance(" + sliceReader + ", grant, false), target)", "table(cursor.advance(" + sourceReader + ", grant, false), target)");
    change("            Self::Stream(cursor) => stream(cursor.advance(source, grant, false), target),",
      "            Self::Stream(cursor) => match source.contiguous_bytes() {\n"
      + "                Some(bytes) => stream(cursor.advance(bytes, grant, false), target),\n"
      + "                None => XrefRevisionReceipt { event: Err(XrefStreamError::Decode(ValueError::literal(ValueRefusalKind::OwnershipLimit, \"PDF xref stream requires contiguous input\"))), ..response(XrefRevisionEvent::More) },\n"
      + "            },");
  }],
  [assembly, change => {
    change("use super::{", "use semio_framework_value::ByteReadSource;\nuse super::{");
    change("source: &[u8], grant: CosGrant", generic + ", grant: CosGrant");
    change("offset < source.len()", "offset < source.byte_len()", 2);
    change("                if !self.revisions.admit_visit(target, admitted) {\n                    return fault(\"xref revision lost its requested target\", 1);\n                }",
      "                if !self.revisions.admit_visit(target, admitted) { return fault(\"xref revision lost its requested target\", 1); }");
  }],
  [structured, change => {
    change("use super::{", "use semio_framework_value::ByteReadSource;\nuse super::{");
    change("source: &[u8], grant: CosGrant", generic + ", grant: CosGrant");
    change("cursor.advance_counted(" + sliceReader + ", grant.work)", "cursor.advance_counted(" + sourceReader + ", grant.work)");
    change("cursor.advance(" + sliceReader + ", index, grant.work, false)", "cursor.advance(" + sourceReader + ", index, grant.work, false)");
    change("        if !matches!(self.phase, Phase::Complete { .. }) { return None; }\n        let index = self.index.take()?;",
      "        let index = matches!(self.phase, Phase::Complete { .. }).then(|| self.index.take()).flatten()?;");
  }],
  [fileStructure, change => {
    change("use super::{", "use semio_framework_value::{ByteReadRange, ByteReadSource};\nuse super::{");
    change("source: &[u8], grant: CosGrant", generic + ", grant: CosGrant");
    change("self.probe.advance(" + sliceReader + ", grant.work)", "self.probe.advance(" + sourceReader + ", grant.work)");
    change("    origin: usize,\n    maximum_stream_bytes:", "    origin: usize,\n    absolute_recovery: bool,\n    maximum_stream_bytes:");
    change("phase: Phase::Header, origin: 0, maximum_stream_bytes", "phase: Phase::Header, origin: 0, absolute_recovery: false, maximum_stream_bytes");
    change("StructuredXrefCursor::new(source.len(),", "StructuredXrefCursor::new(source.byte_len(),");
    change("                let step = cursor.advance(&source[self.origin..], grant, false);",
      "                let Some(bytes) = source.byte_len().checked_sub(self.origin).and_then(|length| ByteReadRange::new(source, self.origin, length)) else {\n"
      + "                    return PdfFileStructureReceipt { event: Err(XrefStreamError::Structure(\"PDF source origin has changed\")), ..result(Event::More, 1) };\n"
      + "                };\n"
      + "                let step = cursor.advance(&bytes, grant, false);");
    change("                        } else {\n                            self.phase = Phase::Complete { intact, root_resolves };",
      "                        } else if !intact && self.absolute_recovery {\n"
      + "                            Ok(Event::RecoveryRequired)\n"
      + "                        } else {\n"
      + "                            self.phase = Phase::Complete { intact, root_resolves };");
    change("                    status => status.map(map_event),",
      "                    Ok(StructuredXrefEvent::RecoveryRequired) if source.contiguous_bytes().is_none() && self.origin == 0 && self.header.is_some_and(|header| header.offset() != 0) => {\n"
      + "                        self.absolute_recovery = true;\n"
      + "                        cursor.begin_close();\n"
      + "                        self.phase = Phase::RetryRelative;\n"
      + "                        Ok(Event::More)\n"
      + "                    }\n"
      + "                    status => status.map(map_event),");
    change("                    self.xref = Some(StructuredXrefCursor::new(source.len() - self.origin,", 
      "                    let Some(length) = source.byte_len().checked_sub(self.origin) else { self.phase = Phase::NotPdf; return result(Event::NotPdf, 1); };\n"
      + "                    self.xref = Some(StructuredXrefCursor::new(length,");
  }],
  ["🔗️xref/📑️classic/🦀️.rs", change => {
    change("                if self.remaining == 0 {\n                    self.field = Field::FirstObject;\n                } else {\n                    self.number = self.number.checked_add(1).ok_or(\"xref object number overflow\")?;\n                    self.field = Field::Offset;\n                }",
      "                self.field = if self.remaining == 0 { Field::FirstObject } else {\n                    self.number = self.number.checked_add(1).ok_or(\"xref object number overflow\")?;\n                    Field::Offset\n                };");
  }],
];
for (const [relative, stage] of edits) {
  const path = relative === trait ? trait : join(root, relative);
  assert(Buffer.byteLength(resolve(path)) <= 256, path);
  const live = await Bun.file(path).text();
  const previous = overlay.get(path);
  const before = digest(live);
  if (previous) assert.equal(previous[1], before, path + " changed");
  let body = previous?.[2] ?? live;
  stage((from, to, count = 1) => {
    assert.equal(body.split(from).length - 1, count, path + " expected " + from);
    body = body.replaceAll(from, to);
  });
  overlay.set(path, [path, before, body]);
}
await Bun.write(join(generated, "pdf-graph-byte-overlay.json"), JSON.stringify([...overlay.values()]));
console.log("[DEBUG] Staged exact segmented PDF graph changes across", overlay.size, "source paths");

