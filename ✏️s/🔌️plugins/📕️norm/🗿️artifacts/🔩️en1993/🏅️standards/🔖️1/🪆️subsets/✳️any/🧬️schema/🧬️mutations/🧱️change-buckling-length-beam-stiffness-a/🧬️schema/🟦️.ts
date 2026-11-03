/** 🧱️ `change-buckling-length-beam-stiffness-a` wire twin: the leaf payload `ChangeBucklingLengthBeamStiffnessA`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBucklingLengthBeamStiffnessA {
  id: string;
  newBeamStiffnessA: number;
}

export const parseChangeBucklingLengthBeamStiffnessA: NormWireReader<ChangeBucklingLengthBeamStiffnessA> = normWireObject<ChangeBucklingLengthBeamStiffnessA>({ id: normWireRequired(normWireString), newBeamStiffnessA: normWireRequired(normWireNumber) });
