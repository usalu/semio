/** 🚗 `change-accidental-vehicle-speed` wire twin: the leaf payload `ChangeAccidentalVehicleSpeed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccidentalVehicleSpeed {
  id: string;
  newVehicleSpeed: number;
}

export const parseChangeAccidentalVehicleSpeed: NormWireReader<ChangeAccidentalVehicleSpeed> = normWireObject<ChangeAccidentalVehicleSpeed>({ id: normWireRequired(normWireString), newVehicleSpeed: normWireRequired(normWireNumber) });
