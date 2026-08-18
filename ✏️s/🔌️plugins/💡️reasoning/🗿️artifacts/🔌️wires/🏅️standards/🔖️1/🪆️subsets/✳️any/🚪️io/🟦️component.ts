// 🚪️ IoEntryDescriptor[] mirror for `s.reasoning.wires@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️component.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to (json exercises an exact struct mapping, md wraps the full `.wires` DSL text
// losslessly, csv/txt/svg/png are honest not-yet-implemented/no-op stubs — same fidelity/sniffs
// shape either way). Shaped inline (no generated `IoEntryDescriptor` type checked in anywhere yet
// in this repo) so this file has no fragile forward reference; swap for a real import once ts-rs
// generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const WIRES = "s.reasoning.wires@1/*";
const CSV = "s.stdio.csv@rfc4180/*";
const MD = "s.stdio.md@commonmark/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const SVG = "s.stdio.svg@1.1/*";
const PNG = "s.stdio.png@1.2/*";
const TXT = "s.stdio.txt@utf-8/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: WIRES, into: CSV, fidelity: "Lossy", sniffs: false },
  { from: CSV, into: WIRES, fidelity: "Lossy", sniffs: false },
  { from: WIRES, into: MD, fidelity: "Canonical", sniffs: false },
  { from: MD, into: WIRES, fidelity: "Canonical", sniffs: false },
  { from: WIRES, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: WIRES, fidelity: "Exact", sniffs: false },
  { from: WIRES, into: SVG, fidelity: "Lossy", sniffs: false },
  { from: SVG, into: WIRES, fidelity: "Lossy", sniffs: false },
  { from: WIRES, into: PNG, fidelity: "Lossy", sniffs: false },
  { from: PNG, into: WIRES, fidelity: "Lossy", sniffs: false },
  { from: WIRES, into: TXT, fidelity: "Lossy", sniffs: false },
  { from: TXT, into: WIRES, fidelity: "Lossy", sniffs: false },
];
