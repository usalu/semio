/** 🚛 `remove-special-vehicle` wire twin: the leaf payload `RemoveSpecialVehicle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSpecialVehicle {
  id: string;
}

export const parseRemoveSpecialVehicle: NormWireReader<RemoveSpecialVehicle> = normWireObject<RemoveSpecialVehicle>({ id: normWireRequired(normWireString) });
