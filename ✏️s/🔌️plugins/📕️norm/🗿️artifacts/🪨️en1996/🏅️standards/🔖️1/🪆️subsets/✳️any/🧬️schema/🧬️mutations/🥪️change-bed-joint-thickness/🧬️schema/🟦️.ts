/** 🥪️ `change-bed-joint-thickness` wire twin: the leaf payload `ChangeBedJointThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBedJointThickness {
  id: string;
  newBedJointThicknessM: number;
}

export const parseChangeBedJointThickness: NormWireReader<ChangeBedJointThickness> = normWireObject<ChangeBedJointThickness>({ id: normWireRequired(normWireString), newBedJointThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
