/** 📏️ `change-beam-span-m` wire twin: the leaf payload `ChangeBeamSpanM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSpanM {
  id: string;
  newSpanM: number;
}

export const parseChangeBeamSpanM: NormWireReader<ChangeBeamSpanM> = normWireObject<ChangeBeamSpanM>({ id: normWireRequired(normWireString), newSpanM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
