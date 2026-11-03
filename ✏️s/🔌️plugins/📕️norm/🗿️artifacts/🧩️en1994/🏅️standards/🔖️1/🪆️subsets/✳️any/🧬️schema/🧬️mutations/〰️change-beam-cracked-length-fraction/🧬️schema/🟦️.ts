/** 〰️ `change-beam-cracked-length-fraction` wire twin: the leaf payload `ChangeBeamCrackedLengthFraction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamCrackedLengthFraction {
  id: string;
  newCrackedLengthFraction: number;
}

export const parseChangeBeamCrackedLengthFraction: NormWireReader<ChangeBeamCrackedLengthFraction> = normWireObject<ChangeBeamCrackedLengthFraction>({ id: normWireRequired(normWireString), newCrackedLengthFraction: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
