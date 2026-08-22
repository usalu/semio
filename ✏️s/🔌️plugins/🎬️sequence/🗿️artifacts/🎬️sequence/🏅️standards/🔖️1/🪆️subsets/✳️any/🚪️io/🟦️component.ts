// 🚪️ IoEntryDescriptor[] mirror for `s.sequence.sequence@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️component.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to (csv/md/json exercise real conversions; txt is an honest not-yet-implemented stub,
// same fidelity/sniffs shape either way). Shaped inline (no generated `IoEntryDescriptor` type
// checked in anywhere yet in this repo) so this file has no fragile forward reference; swap for a
// real import once owned schema generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const SEQUENCE = "s.sequence.sequence@1/*";
const CSV = "s.stdio.csv@rfc4180/*";
const MD = "s.stdio.md@commonmark/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const TXT = "s.stdio.txt@utf-8/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: SEQUENCE, into: CSV, fidelity: "Lossy", sniffs: false },
  { from: CSV, into: SEQUENCE, fidelity: "Lossy", sniffs: false },
  { from: SEQUENCE, into: MD, fidelity: "Canonical", sniffs: false },
  { from: MD, into: SEQUENCE, fidelity: "Canonical", sniffs: false },
  { from: SEQUENCE, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: SEQUENCE, fidelity: "Exact", sniffs: false },
  { from: SEQUENCE, into: TXT, fidelity: "Lossy", sniffs: false },
  { from: TXT, into: SEQUENCE, fidelity: "Lossy", sniffs: false },
];
