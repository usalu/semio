/** 🏠️ `change-floor-psi2` wire twin: the leaf payload `ChangeFloorPsi2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorPsi2 {
  floorId: string;
  newValue: number;
}

export const parseChangeFloorPsi2: NormWireReader<ChangeFloorPsi2> = normWireObject<ChangeFloorPsi2>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
