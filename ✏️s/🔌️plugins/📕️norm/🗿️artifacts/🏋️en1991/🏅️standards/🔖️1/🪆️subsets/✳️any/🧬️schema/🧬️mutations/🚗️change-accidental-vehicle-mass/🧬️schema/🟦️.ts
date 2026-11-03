/** 🚗 `change-accidental-vehicle-mass` wire twin: the leaf payload `ChangeAccidentalVehicleMass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccidentalVehicleMass {
  id: string;
  newVehicleMass: number;
}

export const parseChangeAccidentalVehicleMass: NormWireReader<ChangeAccidentalVehicleMass> = normWireObject<ChangeAccidentalVehicleMass>({ id: normWireRequired(normWireString), newVehicleMass: normWireRequired(normWireNumber) });
