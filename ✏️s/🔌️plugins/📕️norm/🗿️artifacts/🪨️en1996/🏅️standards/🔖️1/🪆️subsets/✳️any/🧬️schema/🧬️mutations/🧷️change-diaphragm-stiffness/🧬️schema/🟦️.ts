/** 🧷 `change-diaphragm-stiffness` wire twin: the leaf payload `ChangeDiaphragmStiffness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmStiffness {
  id: string;
  newDiaphragmStiffnessNPerM: number;
}

export const parseChangeDiaphragmStiffness: NormWireReader<ChangeDiaphragmStiffness> = normWireObject<ChangeDiaphragmStiffness>({ id: normWireRequired(normWireString), newDiaphragmStiffnessNPerM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000000000})) });
