/** 🧩️ `change-joint-plate-eccentricity` wire twin: the leaf payload `ChangeJointPlateEccentricity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateEccentricity {
  id: string;
  newEccentricity: number;
}

export const parseChangeJointPlateEccentricity: NormWireReader<ChangeJointPlateEccentricity> = normWireObject<ChangeJointPlateEccentricity>({ id: normWireRequired(normWireString), newEccentricity: normWireRequired(normWireNumber) });
