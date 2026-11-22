/** 🪪 Stages byte-addressed PDF probes and emits the focused parser law's concrete source. */
import assert from "node:assert/strict";
import { join, resolve } from "node:path";

const ticket = import.meta.dir;
const mode = process.argv[2];
assert(mode === "red" || mode === "green");
const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules";
const header = "🔤️lexer/🪪️pdf-header/🦀️.rs";
const indirect = "🔤️lexer/🪪️indirect-header/🦀️.rs";
const probe = "🔤️lexer/🧭️xref-section/🦀️.rs";
const footer = "🔗️xref/🔎️footer/🦀️.rs";
const read = "byte_at: impl Fn(usize) -> Option<u8>";
const signature = ["source: &[u8], grant: usize", read + ", grant: usize"];
const sourceRead = ["source.get(self.position).copied()", "byte_at(self.position)"];
const direct = (name: string, source: string, args: string) => [
  `${name}(${source}, ${args}`,
  `${name}(|position| ${source}.get(position).copied(), ${args}`,
];
const edits: Record<string, readonly (readonly string[])[]> = {
  [header]: [signature, sourceRead, direct("cursor.advance","source","MAXIMUM_OFFSET")],
  [indirect]: [signature, sourceRead, direct("cursor.advance","pdf","grant"), direct("cursor.advance","source","source.len()"),
    direct(".advance",'b"7 65536 obj "',"64"), direct(".advance",'b"7 0 object "',"64")],
  [probe]: [signature, sourceRead, direct("cursor.advance","source","1")],
  [footer]: [
    ["    marker: usize,", "    marker: usize,\n    length: usize,"],
    ["Self { position: length, marker: 0, phase", "Self { position: length, marker: 0, length, phase"],
    ["pub fn advance(&mut self, bytes: &[u8], budget: usize)", `pub fn advance(&mut self, ${read}, budget: usize)`],
    ["pub(crate) fn advance_counted(&mut self, bytes: &[u8], budget: usize)", `pub(crate) fn advance_counted(&mut self, ${read}, budget: usize)`],
    ["self.advance_counted(bytes, budget)", "self.advance_counted(byte_at, budget)"],
    ["let Some(&byte) = bytes.get(self.position)", "let Some(byte) = byte_at(self.position)", "2"],
    ["bytes.get(self.marker - 1).copied()", "byte_at(self.marker - 1)"],
    ["bytes.get(self.position).copied()", "byte_at(self.position)"],
    ["self.settle(bytes.len(),", "self.settle(self.length,", "2"],
  ],
  "🔤️lexer/🦀️.rs": [direct("cursor.advance","data","4096).map_err")],
  "🔤️lexer/🧭️object-scan/🦀️.rs": [direct("header.advance","source","remaining")],
  "🔤️lexer/📦️object-frame/🦀️.rs": [direct("header.advance","source","grant.work")],
  "🔤️lexer/📐️indirect-length/🦀️.rs": [direct("self.header.advance","data","grant.work")],
  "🔤️lexer/🪪️pdf-header/🧪️tests/🔬️unit/🦀️.rs": [
    ["cursor.advance(&source, 1","cursor.advance(|position| source.get(position).copied(), 1"],
    ["cursor.advance(&source, 0","cursor.advance(|position| source.get(position).copied(), 0"],
    direct("cursor.advance","source","source.len()"),
  ],
  "🔗️xref/🦀️.rs": [direct("cursor.advance","data","4096")],
  "🔗️xref/🌐️file-structure/🦀️.rs": [direct("self.probe.advance","source","grant.work")],
  "🔗️xref/🔎️offset-validation/🦀️.rs": [direct("cursor.advance","source","remaining")],
  "🔗️xref/🧵️assembly/🦀️.rs": [direct("footer.advance","source","1")],
  "🔗️xref/🧭️revisions/🦀️.rs": [direct("probe.advance","source","grant.work"), direct("footer.advance","pdf","1")],
  "🔗️xref/🛤️structured-file/🦀️.rs": [direct("cursor.advance_counted","source","grant.work")],
  "🔗️xref/🔎️footer/🧪️tests/🔬️unit/🦀️.rs": [direct("cursor.advance","file","0"),direct("cursor.advance","file","budget")],
};
const staged = new Map<string,string>();
const preimages = new Map<string,string>();
for (const [relative, changes] of Object.entries(edits)) {
  const path = resolve(root, relative);
  assert(Buffer.byteLength(path) <= 256, relative);
  const before = await Bun.file(path).text();
  preimages.set(relative, new Bun.CryptoHasher("sha256").update(before).digest("hex"));
  let candidate = before;
  if (mode === "green") {
    for (const [from, to, count = "1"] of changes) {
      assert.equal(candidate.split(from!).length - 1, Number(count), relative + " expected " + from);
      candidate = candidate.replaceAll(from!, to!);
    }
  }
  staged.set(relative,candidate);
}
const extract = (relative: string) => staged.get(relative)!.split("\n#[cfg(test)]")[0]!
  .replace(/^\/\/!.*\n/gmu,"")
  .replace(/^use semio_s_artifact_stdio_pdf_snapshot_2::[^\n]+/mu,"use super::ObjRef;");
const law = await Bun.file(join(ticket,"📜️pdf-byte-probe-law.rs")).text();
const fixture = await Bun.file(join(ticket,"🔣️segmented-pdf-source.json")).json();
const launch = law
  .replaceAll("$HEADER_OFFSET", String(fixture.headerOffset))
  .replaceAll("$PDF_VERSION", fixture.version)
  .replaceAll("$CATALOG_OFFSET", String(fixture.catalogOffset))
  .replaceAll("$XREF_OFFSET", String(fixture.xrefOffset));
const modules = [header,indirect,probe,footer].map((path,index) => `mod probe_${index} {\n${extract(path)}\n}\n`).join("\n");
await Bun.write(join(ticket,"🗑️generated","pdf-probe-" + mode + ".rs"),modules + "\n" + launch);
if (mode === "green") {
  const overlay = [...staged].map(([path, body]) => [join(root, path), preimages.get(path)!, body]);
  await Bun.write(join(ticket, "🗑️generated", "pdf-byte-probe-overlay.json"), JSON.stringify(overlay));
}
console.log("[DEBUG] Validated exact edits across",staged.size,"source paths for",mode);
