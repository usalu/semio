/** ⚙ `change-rotating-machine-eccentricity` wire twin: the leaf payload `ChangeRotatingMachineEccentricity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRotatingMachineEccentricity {
  id: string;
  newEccentricity: number;
}

export const parseChangeRotatingMachineEccentricity: NormWireReader<ChangeRotatingMachineEccentricity> = normWireObject<ChangeRotatingMachineEccentricity>({ id: normWireRequired(normWireString), newEccentricity: normWireRequired(normWireNumber) });
