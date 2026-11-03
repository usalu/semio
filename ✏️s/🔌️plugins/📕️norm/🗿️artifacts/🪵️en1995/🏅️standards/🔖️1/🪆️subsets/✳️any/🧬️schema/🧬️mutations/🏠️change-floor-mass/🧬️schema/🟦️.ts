/** 🏠️ `change-floor-mass` wire twin: the leaf payload `ChangeFloorMass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorMass {
  floorId: string;
  newValue: number;
}

export const parseChangeFloorMass: NormWireReader<ChangeFloorMass> = normWireObject<ChangeFloorMass>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
