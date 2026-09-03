// 🚪️ IoEntryDescriptor[] mirror for `s.mathematical.equation@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to (json is a real exact round trip, md wraps the full native DSL text losslessly, csv
// flattens to one row per graph node — id/label/x/y, dropping edges/geometry/equation — txt is an
// honest not-yet-implemented stub). Shaped inline (no generated `IoEntryDescriptor` type checked in
// anywhere yet in this repo) so this file has no fragile forward reference; swap for a real import
// once owned schema generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const EQUATION = "s.mathematical.equation@1/*";
const CSV = "s.stdio.csv@rfc4180/*";
const MD = "s.stdio.md@commonmark/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const TXT = "s.stdio.txt@utf-8/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: EQUATION, into: CSV, fidelity: "Lossy", sniffs: false },
  { from: CSV, into: EQUATION, fidelity: "Lossy", sniffs: false },
  { from: EQUATION, into: MD, fidelity: "Canonical", sniffs: false },
  { from: MD, into: EQUATION, fidelity: "Canonical", sniffs: false },
  { from: EQUATION, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: EQUATION, fidelity: "Exact", sniffs: false },
  { from: EQUATION, into: TXT, fidelity: "Lossy", sniffs: false },
  { from: TXT, into: EQUATION, fidelity: "Lossy", sniffs: false },
];
