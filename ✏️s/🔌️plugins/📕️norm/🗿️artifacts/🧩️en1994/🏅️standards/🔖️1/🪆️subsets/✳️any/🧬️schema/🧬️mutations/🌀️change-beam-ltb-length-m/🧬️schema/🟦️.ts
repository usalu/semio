/** 🌀️ `change-beam-ltb-length-m` wire twin: the leaf payload `ChangeBeamLtbLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamLtbLengthM {
  id: string;
  newLtbLengthM: number;
}

export const parseChangeBeamLtbLengthM: NormWireReader<ChangeBeamLtbLengthM> = normWireObject<ChangeBeamLtbLengthM>({ id: normWireRequired(normWireString), newLtbLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
