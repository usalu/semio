/** 🌉 `change-thermal-bridge-type` wire twin: the leaf payload `ChangeThermalBridgeType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeThermalBridgeType {
  newThermalBridgeType: number;
}

export const parseChangeThermalBridgeType: NormWireReader<ChangeThermalBridgeType> = normWireObject<ChangeThermalBridgeType>({ newThermalBridgeType: normWireRequired(normWireRange(normWireInteger, {"minimum":1,"maximum":3})) });
