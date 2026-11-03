/** 🧱️ `change-buckling-length-column-stiffness-a` wire twin: the leaf payload `ChangeBucklingLengthColumnStiffnessA`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBucklingLengthColumnStiffnessA {
  id: string;
  newColumnStiffnessA: number;
}

export const parseChangeBucklingLengthColumnStiffnessA: NormWireReader<ChangeBucklingLengthColumnStiffnessA> = normWireObject<ChangeBucklingLengthColumnStiffnessA>({ id: normWireRequired(normWireString), newColumnStiffnessA: normWireRequired(normWireNumber) });
