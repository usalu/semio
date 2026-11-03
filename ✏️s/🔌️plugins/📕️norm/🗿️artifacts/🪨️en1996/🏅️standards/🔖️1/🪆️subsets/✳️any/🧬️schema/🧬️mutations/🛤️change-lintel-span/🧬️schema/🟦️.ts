/** 🛤️ `change-lintel-span` wire twin: the leaf payload `ChangeLintelSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLintelSpan {
  wallId: string;
  id: string;
  newSpanM: number;
}

export const parseChangeLintelSpan: NormWireReader<ChangeLintelSpan> = normWireObject<ChangeLintelSpan>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newSpanM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
