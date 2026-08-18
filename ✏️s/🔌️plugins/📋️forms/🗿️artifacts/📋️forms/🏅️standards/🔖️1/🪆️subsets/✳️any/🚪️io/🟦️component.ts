// 🚪️ IoEntryDescriptor[] mirror for `s.forms.forms@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️component.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to (json exercises a real Exact round trip; csv export is a real Lossy flattened-row
// projection, csv import + xlsx + zip both directions are honest not-yet-implemented stubs, same
// fidelity/sniffs shape either way). Shaped inline (no generated `IoEntryDescriptor` type checked
// in anywhere yet in this repo) so this file has no fragile forward reference; swap for a real
// import once ts-rs generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const FORMS = "s.forms.forms@1/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const CSV = "s.stdio.csv@rfc4180/*";
const XLSX = "s.stdio.xlsx@ecma-376/*";
const ZIP = "s.stdio.zip@2.0/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: FORMS, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: FORMS, fidelity: "Exact", sniffs: false },
  { from: FORMS, into: CSV, fidelity: "Lossy", sniffs: false },
  { from: CSV, into: FORMS, fidelity: "Lossy", sniffs: false },
  { from: FORMS, into: XLSX, fidelity: "Lossy", sniffs: false },
  { from: XLSX, into: FORMS, fidelity: "Lossy", sniffs: false },
  { from: FORMS, into: ZIP, fidelity: "Lossy", sniffs: false },
  { from: ZIP, into: FORMS, fidelity: "Lossy", sniffs: false },
];
