/** ↔️ `change-beam-adjacent-span-m` wire twin: the leaf payload `ChangeBeamAdjacentSpanM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamAdjacentSpanM {
  id: string;
  newAdjacentSpanM: number;
}

export const parseChangeBeamAdjacentSpanM: NormWireReader<ChangeBeamAdjacentSpanM> = normWireObject<ChangeBeamAdjacentSpanM>({ id: normWireRequired(normWireString), newAdjacentSpanM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
