/** ↕️ `change-wall-thickness` wire twin: the leaf payload `ChangeWallThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallThickness {
  id: string;
  newThicknessM: number;
}

export const parseChangeWallThickness: NormWireReader<ChangeWallThickness> = normWireObject<ChangeWallThickness>({ id: normWireRequired(normWireString), newThicknessM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
