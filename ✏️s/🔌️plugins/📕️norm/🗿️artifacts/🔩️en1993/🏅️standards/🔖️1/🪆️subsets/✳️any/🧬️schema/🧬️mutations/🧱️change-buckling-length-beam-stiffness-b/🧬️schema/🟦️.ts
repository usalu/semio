/** 🧱️ `change-buckling-length-beam-stiffness-b` wire twin: the leaf payload `ChangeBucklingLengthBeamStiffnessB`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBucklingLengthBeamStiffnessB {
  id: string;
  newBeamStiffnessB: number;
}

export const parseChangeBucklingLengthBeamStiffnessB: NormWireReader<ChangeBucklingLengthBeamStiffnessB> = normWireObject<ChangeBucklingLengthBeamStiffnessB>({ id: normWireRequired(normWireString), newBeamStiffnessB: normWireRequired(normWireNumber) });
