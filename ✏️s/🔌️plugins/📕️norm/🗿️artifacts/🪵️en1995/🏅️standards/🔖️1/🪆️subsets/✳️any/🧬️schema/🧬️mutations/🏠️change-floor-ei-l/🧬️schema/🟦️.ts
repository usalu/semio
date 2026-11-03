/** 🏠️ `change-floor-ei-l` wire twin: the leaf payload `ChangeFloorEiL`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorEiL {
  floorId: string;
  newValue: number;
}

export const parseChangeFloorEiL: NormWireReader<ChangeFloorEiL> = normWireObject<ChangeFloorEiL>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
