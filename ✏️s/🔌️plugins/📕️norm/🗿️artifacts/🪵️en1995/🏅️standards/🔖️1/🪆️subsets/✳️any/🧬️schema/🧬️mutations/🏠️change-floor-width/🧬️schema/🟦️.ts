/** 🏠️ `change-floor-width` wire twin: the leaf payload `ChangeFloorWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorWidth {
  floorId: string;
  newValue: number;
}

export const parseChangeFloorWidth: NormWireReader<ChangeFloorWidth> = normWireObject<ChangeFloorWidth>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
