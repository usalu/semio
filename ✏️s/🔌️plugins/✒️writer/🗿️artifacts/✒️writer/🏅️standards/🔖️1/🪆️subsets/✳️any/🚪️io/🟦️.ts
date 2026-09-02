// 🚪️ IoEntryDescriptor[] mirror for `s.writer.writer@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to (json is Exact via full serde round trip; txt/md/pdf/docx are Lossy — plain-text
// content only, no `schema`/`id`/`uri`/`language_id`). Shaped inline (no generated
// `IoEntryDescriptor` type checked in anywhere yet in this repo) so this file has no fragile
// forward reference; swap for a real import once owned schema generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const WRITER = "s.writer.writer@1/*";
const TXT = "s.stdio.txt@utf-8/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const MD = "s.stdio.md@commonmark/*";
const PDF = "s.stdio.pdf@1.4/*";
const DOCX = "s.stdio.docx@ecma-376/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: TXT, into: WRITER, fidelity: "Lossy", sniffs: false },
  { from: WRITER, into: TXT, fidelity: "Lossy", sniffs: false },
  { from: JSON_DIALECT, into: WRITER, fidelity: "Exact", sniffs: false },
  { from: WRITER, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: MD, into: WRITER, fidelity: "Lossy", sniffs: false },
  { from: WRITER, into: MD, fidelity: "Lossy", sniffs: false },
  { from: PDF, into: WRITER, fidelity: "Lossy", sniffs: false },
  { from: WRITER, into: PDF, fidelity: "Lossy", sniffs: false },
  { from: DOCX, into: WRITER, fidelity: "Lossy", sniffs: false },
  { from: WRITER, into: DOCX, fidelity: "Lossy", sniffs: false },
];
