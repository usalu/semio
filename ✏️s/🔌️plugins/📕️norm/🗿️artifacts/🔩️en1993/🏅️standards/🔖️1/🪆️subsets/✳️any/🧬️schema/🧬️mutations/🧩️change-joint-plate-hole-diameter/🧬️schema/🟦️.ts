/** 🧩️ `change-joint-plate-hole-diameter` wire twin: the leaf payload `ChangeJointPlateHoleDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateHoleDiameter {
  id: string;
  newHoleDiameter: number;
}

export const parseChangeJointPlateHoleDiameter: NormWireReader<ChangeJointPlateHoleDiameter> = normWireObject<ChangeJointPlateHoleDiameter>({ id: normWireRequired(normWireString), newHoleDiameter: normWireRequired(normWireNumber) });
