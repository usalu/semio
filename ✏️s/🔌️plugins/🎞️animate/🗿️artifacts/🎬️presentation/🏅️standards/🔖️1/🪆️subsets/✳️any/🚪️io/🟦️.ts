// 🚪️ IoEntryDescriptor[] mirror for `s.animate.presentation@1/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Mirrors the real Rust `io()` entries
// (`🦀️.rs`) — one `Serializer`/`Deserializer` pair per foreign stdio dialect this subset
// bridges to. json is a real structural round trip (`Exact`); md wraps the whole native DSL text
// losslessly in one paragraph block (`Canonical`); pptx/pdf/svg/png are pre-existing degenerate
// placeholders (structural `serde_json` coercion or raw pack-byte passthrough, not real format
// conversion — `Lossy`); txt is an honest not-yet-implemented stub (`Lossy`). Shaped inline (no
// generated `IoEntryDescriptor` type checked in anywhere yet in this repo) so this file has no
// fragile forward reference; swap for a real import once owned schema generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

const PRESENTATION = "s.animate.presentation@1/*";
const JSON_DIALECT = "s.stdio.json@rfc8259/*";
const MD = "s.stdio.md@commonmark/*";
const PDF = "s.stdio.pdf@1.4/*";
const PPTX = "s.stdio.pptx@ecma-376/*";
const SVG = "s.stdio.svg@1.1/*";
const PNG = "s.stdio.png@1.2/*";
const TXT = "s.stdio.txt@utf-8/*";

export const ioEntries: IoEntryDescriptorMirror[] = [
  { from: PRESENTATION, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: PRESENTATION, fidelity: "Exact", sniffs: false },
  { from: PRESENTATION, into: MD, fidelity: "Canonical", sniffs: false },
  { from: MD, into: PRESENTATION, fidelity: "Canonical", sniffs: false },
  { from: PRESENTATION, into: PDF, fidelity: "Lossy", sniffs: false },
  { from: PDF, into: PRESENTATION, fidelity: "Lossy", sniffs: false },
  { from: PRESENTATION, into: PPTX, fidelity: "Lossy", sniffs: false },
  { from: PPTX, into: PRESENTATION, fidelity: "Lossy", sniffs: false },
  { from: PRESENTATION, into: SVG, fidelity: "Lossy", sniffs: false },
  { from: SVG, into: PRESENTATION, fidelity: "Lossy", sniffs: false },
  { from: PRESENTATION, into: PNG, fidelity: "Lossy", sniffs: false },
  { from: PNG, into: PRESENTATION, fidelity: "Lossy", sniffs: false },
  { from: PRESENTATION, into: TXT, fidelity: "Lossy", sniffs: false },
  { from: TXT, into: PRESENTATION, fidelity: "Lossy", sniffs: false },
];
