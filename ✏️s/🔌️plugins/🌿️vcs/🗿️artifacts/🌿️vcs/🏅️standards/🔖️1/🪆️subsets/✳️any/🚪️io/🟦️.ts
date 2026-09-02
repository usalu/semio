// 🚪️ IoEntryDescriptor[] mirror for `s.vcs.vcs@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to (json is a real, lossless conversion; csv/xlsx/zip are pre-migration lossy struct
// coercions, kept as-is — reshaping the domain mapping is out of this cutover's scope; txt is an
// honest not-yet-implemented stub). Shaped inline (no generated `IoEntryDescriptor` type checked in
// anywhere yet in this repo) so this file has no fragile forward reference; swap for a real import
// once owned schema generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const VCS = "s.vcs.vcs@1/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const CSV = "s.stdio.csv@rfc4180/*";
const XLSX = "s.stdio.xlsx@ecma-376/*";
const ZIP = "s.stdio.zip@2.0/*";
const TXT = "s.stdio.txt@utf-8/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: VCS, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: VCS, fidelity: "Exact", sniffs: false },
  { from: VCS, into: CSV, fidelity: "Lossy", sniffs: false },
  { from: CSV, into: VCS, fidelity: "Lossy", sniffs: false },
  { from: VCS, into: XLSX, fidelity: "Lossy", sniffs: false },
  { from: XLSX, into: VCS, fidelity: "Lossy", sniffs: false },
  { from: VCS, into: ZIP, fidelity: "Lossy", sniffs: false },
  { from: ZIP, into: VCS, fidelity: "Lossy", sniffs: false },
  { from: VCS, into: TXT, fidelity: "Lossy", sniffs: false },
  { from: TXT, into: VCS, fidelity: "Lossy", sniffs: false },
];
