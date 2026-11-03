/** 🚛 `change-special-vehicle-y` wire twin: the leaf payload `ChangeSpecialVehicleY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSpecialVehicleY {
  id: string;
  newY: number;
}

export const parseChangeSpecialVehicleY: NormWireReader<ChangeSpecialVehicleY> = normWireObject<ChangeSpecialVehicleY>({ id: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });
