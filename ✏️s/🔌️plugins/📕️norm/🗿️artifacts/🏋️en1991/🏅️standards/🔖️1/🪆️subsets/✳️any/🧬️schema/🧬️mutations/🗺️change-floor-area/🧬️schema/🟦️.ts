/** 🗺️ `change-floor-area` wire twin: the leaf payload `ChangeFloorArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorArea {
  id: string;
  newArea: number;
}

export const parseChangeFloorArea: NormWireReader<ChangeFloorArea> = normWireObject<ChangeFloorArea>({ id: normWireRequired(normWireString), newArea: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
