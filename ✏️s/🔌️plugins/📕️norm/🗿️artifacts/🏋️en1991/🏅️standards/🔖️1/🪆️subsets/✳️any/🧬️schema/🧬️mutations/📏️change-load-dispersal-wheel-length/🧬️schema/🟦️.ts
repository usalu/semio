/** 📏 `change-load-dispersal-wheel-length` wire twin: the leaf payload `ChangeLoadDispersalWheelLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadDispersalWheelLength {
  id: string;
  newWheelLength: number;
}

export const parseChangeLoadDispersalWheelLength: NormWireReader<ChangeLoadDispersalWheelLength> = normWireObject<ChangeLoadDispersalWheelLength>({ id: normWireRequired(normWireString), newWheelLength: normWireRequired(normWireNumber) });
