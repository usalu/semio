/** 🧱️ `change-buckling-length-column-stiffness-b` wire twin: the leaf payload `ChangeBucklingLengthColumnStiffnessB`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBucklingLengthColumnStiffnessB {
  id: string;
  newColumnStiffnessB: number;
}

export const parseChangeBucklingLengthColumnStiffnessB: NormWireReader<ChangeBucklingLengthColumnStiffnessB> = normWireObject<ChangeBucklingLengthColumnStiffnessB>({ id: normWireRequired(normWireString), newColumnStiffnessB: normWireRequired(normWireNumber) });
