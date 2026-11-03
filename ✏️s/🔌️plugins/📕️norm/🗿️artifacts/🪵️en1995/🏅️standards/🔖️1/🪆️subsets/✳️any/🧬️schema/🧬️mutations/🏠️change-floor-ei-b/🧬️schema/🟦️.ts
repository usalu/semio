/** 🏠️ `change-floor-ei-b` wire twin: the leaf payload `ChangeFloorEiB`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorEiB {
  floorId: string;
  newValue: number;
}

export const parseChangeFloorEiB: NormWireReader<ChangeFloorEiB> = normWireObject<ChangeFloorEiB>({ floorId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
