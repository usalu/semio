/** 🚛 `change-special-vehicle-designation` wire twin: the leaf payload `ChangeSpecialVehicleDesignation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSpecialVehicleDesignation {
  id: string;
  newDesignation: string;
}

export const parseChangeSpecialVehicleDesignation: NormWireReader<ChangeSpecialVehicleDesignation> = normWireObject<ChangeSpecialVehicleDesignation>({ id: normWireRequired(normWireString), newDesignation: normWireRequired(normWireString) });
