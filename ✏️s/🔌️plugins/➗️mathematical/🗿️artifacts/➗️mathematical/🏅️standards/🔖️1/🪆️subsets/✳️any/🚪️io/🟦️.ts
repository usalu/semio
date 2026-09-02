// 🚪️ IoEntryDescriptor[] mirror for `s.mathematical.mathematical@1/*` (ticket
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

const MATHEMATICAL = "s.mathematical.mathematical@1/*";
const CSV = "s.stdio.csv@rfc4180/*";
const MD = "s.stdio.md@commonmark/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const TXT = "s.stdio.txt@utf-8/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: MATHEMATICAL, into: CSV, fidelity: "Lossy", sniffs: false },
  { from: CSV, into: MATHEMATICAL, fidelity: "Lossy", sniffs: false },
  { from: MATHEMATICAL, into: MD, fidelity: "Canonical", sniffs: false },
  { from: MD, into: MATHEMATICAL, fidelity: "Canonical", sniffs: false },
  { from: MATHEMATICAL, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: MATHEMATICAL, fidelity: "Exact", sniffs: false },
  { from: MATHEMATICAL, into: TXT, fidelity: "Lossy", sniffs: false },
  { from: TXT, into: MATHEMATICAL, fidelity: "Lossy", sniffs: false },
];
