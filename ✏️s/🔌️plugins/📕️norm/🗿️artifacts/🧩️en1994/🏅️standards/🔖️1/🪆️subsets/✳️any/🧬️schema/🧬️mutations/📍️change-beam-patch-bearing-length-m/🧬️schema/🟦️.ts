/** 📍️ `change-beam-patch-bearing-length-m` wire twin: the leaf payload `ChangeBeamPatchBearingLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamPatchBearingLengthM {
  id: string;
  newPatchBearingLengthM: number;
}

export const parseChangeBeamPatchBearingLengthM: NormWireReader<ChangeBeamPatchBearingLengthM> = normWireObject<ChangeBeamPatchBearingLengthM>({ id: normWireRequired(normWireString), newPatchBearingLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
