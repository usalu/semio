/** 🚪️ Registered Rust IO metadata for the Sequence subset, using the shared wire contract. */
import { parseDialectCoordinate, type IoEntryDescriptor } from "../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";

const SEQUENCE = parseDialectCoordinate("s.sequence.sequence@1/*");
const CSV = parseDialectCoordinate("s.stdio.csv@rfc4180/*");
const MD = parseDialectCoordinate("s.stdio.md@commonmark/*");
const JSON_DIALECT = parseDialectCoordinate("s.stdio.json@rfc8259/*");
const TXT = parseDialectCoordinate("s.stdio.txt@utf-8/*");

export const ioEntries: IoEntryDescriptor[] = [
  { from: SEQUENCE, into: CSV, fidelity: "Lossy", sniffs: false },
  { from: CSV, into: SEQUENCE, fidelity: "Lossy", sniffs: true },
  { from: SEQUENCE, into: MD, fidelity: "Canonical", sniffs: false },
  { from: MD, into: SEQUENCE, fidelity: "Canonical", sniffs: true },
  { from: SEQUENCE, into: JSON_DIALECT, fidelity: "Exact", sniffs: false },
  { from: JSON_DIALECT, into: SEQUENCE, fidelity: "Exact", sniffs: true },
  { from: SEQUENCE, into: TXT, fidelity: "Exact", sniffs: false },
  { from: TXT, into: SEQUENCE, fidelity: "Exact", sniffs: true },
];
