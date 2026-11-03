/** 🏠️ `change-floor-damping` wire twin: the leaf payload `ChangeFloorDamping`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorDamping {
  floorId: string;
  newValue: number;
}

export const parseChangeFloorDamping: NormWireReader<ChangeFloorDamping> = normWireObject<ChangeFloorDamping>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
