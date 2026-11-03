/** 🏠️ `change-floor-permanent-load` wire twin: the leaf payload `ChangeFloorPermanentLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorPermanentLoad {
  floorId: string;
  newValue: number;
}

export const parseChangeFloorPermanentLoad: NormWireReader<ChangeFloorPermanentLoad> = normWireObject<ChangeFloorPermanentLoad>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
