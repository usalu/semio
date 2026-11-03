/** 📏 `change-load-dispersal-wheel-width` wire twin: the leaf payload `ChangeLoadDispersalWheelWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadDispersalWheelWidth {
  id: string;
  newWheelWidth: number;
}

export const parseChangeLoadDispersalWheelWidth: NormWireReader<ChangeLoadDispersalWheelWidth> = normWireObject<ChangeLoadDispersalWheelWidth>({ id: normWireRequired(normWireString), newWheelWidth: normWireRequired(normWireNumber) });
