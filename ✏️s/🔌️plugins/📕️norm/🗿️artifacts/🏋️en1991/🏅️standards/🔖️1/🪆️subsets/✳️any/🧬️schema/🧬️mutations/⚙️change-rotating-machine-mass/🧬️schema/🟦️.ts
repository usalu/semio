/** ⚙ `change-rotating-machine-mass` wire twin: the leaf payload `ChangeRotatingMachineMass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRotatingMachineMass {
  id: string;
  newMass: number;
}

export const parseChangeRotatingMachineMass: NormWireReader<ChangeRotatingMachineMass> = normWireObject<ChangeRotatingMachineMass>({ id: normWireRequired(normWireString), newMass: normWireRequired(normWireNumber) });
