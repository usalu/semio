/** 🌉️ `change-bridge-heavy-vehicle-weight` wire twin: the leaf payload `ChangeBridgeHeavyVehicleWeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeHeavyVehicleWeight {
  id: string;
  newHeavyVehicleWeight: number;
}

export const parseChangeBridgeHeavyVehicleWeight: NormWireReader<ChangeBridgeHeavyVehicleWeight> = normWireObject<ChangeBridgeHeavyVehicleWeight>({ id: normWireRequired(normWireString), newHeavyVehicleWeight: normWireRequired(normWireNumber) });
