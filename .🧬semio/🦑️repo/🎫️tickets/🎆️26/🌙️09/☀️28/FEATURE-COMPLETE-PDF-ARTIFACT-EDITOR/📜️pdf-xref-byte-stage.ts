/** 🗂️ Stage bounded classic xref row and indirect-offset reads over segmented storage. */
import assert from "node:assert/strict";
import { join, resolve } from "node:path";

const ticket = import.meta.dir;
const mode = process.argv[2];
assert(mode === "red" || mode === "green");
const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules";
const generated = join(ticket, "🗑️generated");
const digest = (body: string) => new Bun.CryptoHasher("sha256").update(body).digest("hex");
type Candidate = [path: string, preimage: string, body: string];
const earlier = await Bun.file(join(generated, "pdf-byte-probe-overlay.json")).json() as Candidate[];
const overlay = new Map(earlier.map(row => [row[0], row]));
const classic = "🔗️xref/📑️classic/🦀️.rs";
const offset = "🔗️xref/🔎️offset-validation/🦀️.rs";
const index = "🔗️xref/💧️stream/📇️index";
const read = "byte_at: impl Fn(usize) -> Option<u8>";
const adapter = (source: string) => "|position| " + source + ".get(position).copied()";
const edits: Record<string, [string, string][]> = {
  [classic]: [
    ["    token_start: Option<usize>,", "    token: [u8; 20],\n    token_length: usize,"],
    ["token_start: None", "token: Default::default(), token_length: 0"],
    ["    pub fn position(&self) -> usize {\n        self.position\n    }", "    pub fn position(&self) -> usize { self.position }"],
    ["source: &[u8], budget: usize", read + ", budget: usize"],
    ["            let Some(&byte) = source.get(self.position)", "            work += 1;\n            let Some(byte) = byte_at(self.position)"],
    ["            work += 1;\n            let whitespace", "            let whitespace"],
    ["            if let Some(start) = self.token_start {", "            if self.token_length != 0 {"],
    ["                    self.token_start = None;\n                    match self.finish(&source[start..self.position]) {", "                    match self.finish() {"],
    ['                    if self.position - start >= 20 { break Err("xref token exceeds the numeric field width"); }', '                    if self.token_length == self.token.len() { break Err("xref token exceeds the numeric field width"); }\n                    self.token[self.token_length] = byte;\n                    self.token_length += 1;'],
    ["            if self.comment {\n                self.comment = !matches!(byte, b'\\r' | b'\\n');\n            } else if byte == b'%' {\n                self.comment = true;", "            if self.comment || byte == b'%' {\n                self.comment = !matches!(byte, b'\\r' | b'\\n');"],
    ["                self.token_start = Some(self.position);", "                self.token[0] = byte;\n                self.token_length = 1;"],
    ["    fn finish(&mut self, bytes: &[u8])", "    fn finish(&mut self)"],
    ["        let token = std::str::from_utf8(bytes)", "        let length = std::mem::take(&mut self.token_length);\n        let token = std::str::from_utf8(&self.token[..length])"],
  ],
  "🔗️xref/📋️section/🦀️.rs": [["self.rows.advance(source, grant.work)", "self.rows.advance(" + adapter("source") + ", grant.work)"]],
  "🔗️xref/📑️classic/🧪️tests/🔬️unit/🦀️.rs": [["cursor.advance(pdf, budget)", "cursor.advance(" + adapter("pdf") + ", budget)"]],
  [offset]: [
    ["source: &[u8], entries:", read + ", entries:"],
    ["cursor.advance(" + adapter("source") + ", remaining)", "cursor.advance(&byte_at, remaining)"],
  ],
  "🔗️xref/🛤️structured-file/🦀️.rs": [["cursor.advance(source, index, grant.work, false)", "cursor.advance(" + adapter("source") + ", index, grant.work, false)"]],
  "🔗️xref/🦀️.rs": [
    ["let (work, status) = validation.advance(source, &rows, 1, false);", "let (work, status) = validation.advance(" + adapter("source") + ", &rows, 1, false);"],
    ["XrefOffsetCursor::default().advance(source, &rows, 1, true)", "XrefOffsetCursor::default().advance(" + adapter("source") + ", &rows, 1, true)"],
  ],
};
const staged = new Map<string, { before: string; after: string }>();
for (const [relative, changes] of Object.entries(edits)) {
  const path = join(root, relative);
  assert(Buffer.byteLength(resolve(path)) <= 256, relative);
  const live = await Bun.file(path).text();
  const previous = overlay.get(path);
  const preimage = digest(live);
  if (previous) assert.equal(preimage, previous[1], relative + " changed since the probe stage");
  const before = previous?.[2] ?? live;
  let after = before;
  for (const [from, to] of changes) {
    const count = from.startsWith("let (work, status) = validation.advance") ? 2 : 1;
    assert.equal(after.split(from).length - 1, count, relative + " expected " + from);
    after = after.replaceAll(from, to);
  }
  staged.set(relative, { before, after });
  overlay.set(path, [path, preimage, after]);
}

const source = (relative: string) => Bun.file(resolve(root, relative)).text();
const extract = (value: string) => value.split("\n#[cfg(test)]")[0]!.replace(/^\/\/!.*\n/gmu, "");
const probes = await Bun.file(join(generated, "pdf-probe-green.rs")).text();
const probeCode = probes.split("/// 🧬 Supplies the PDF model's two identifier fields")[0]!;
const byteRead = extract(await Bun.file("🧰️framework/🔨️modules/🌱️value/📦️paged/📖️read/🎟️.rs").text());
const mergeCode = extract(await source(index + "/🧩️revision-merge/🦀️.rs"))
  .replace("use semio_s_artifact_stdio_pdf_lexer::CosGrant;", "use crate::CosGrant;");
const indexCode = extract(await source(index + "/🦀️.rs"))
  .replace("use semio_s_artifact_stdio_pdf_lexer::CosGrant;", "use crate::CosGrant;")
  .replace(/#\[path = "[^"]+"\]\nmod revision_merge;/u, () => "mod revision_merge {\n" + mergeCode + "\n}");
const model = await source("🔗️xref/💧️stream/📦️packages/🦀️rust/🦀️.rs");
const entry = model.match(/#\[derive\([^\n]+\)\]\npub enum XrefEntry \{[\s\S]*?\n\}/u)?.[0];
assert(entry);
const offsetCode = extract(staged.get(offset)![mode === "green" ? "after" : "before"])
  .replace("use semio_s_artifact_stdio_pdf_lexer::{HeaderStep, IndirectHeaderCursor};", "use crate::probe_1::{HeaderStep, IndirectHeaderCursor};")
  .replace("use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::ObjRef;", "use crate::ObjRef;");
const classicCode = extract(staged.get(classic)![mode === "green" ? "after" : "before"]);
const law = await Bun.file(join(ticket, "📜️pdf-xref-byte-law.rs")).text();
const fixture = await Bun.file(join(ticket, "🔣️segmented-pdf-source.json")).json();
const boundLaw = law.replaceAll("$SOURCE_BYTES", String(fixture.sourceBytes))
  .replaceAll("$CATALOG_OFFSET", String(fixture.catalogOffset))
  .replaceAll("$XREF_OFFSET", String(fixture.xrefOffset));
await Bun.write(join(generated, "pdf-xref-" + mode + ".rs"), [
  probeCode, byteRead, entry, "mod index {", indexCode, "}", "mod offset {", offsetCode, "}",
  "mod classic {", classicCode, "}", boundLaw,
].join("\n"));
if (mode === "green") {
  await Bun.write(join(generated, "pdf-xref-byte-overlay.json"), JSON.stringify([...overlay.values()]));
}
console.log("[DEBUG] Validated exact edits across", staged.size, "source paths for", mode);

