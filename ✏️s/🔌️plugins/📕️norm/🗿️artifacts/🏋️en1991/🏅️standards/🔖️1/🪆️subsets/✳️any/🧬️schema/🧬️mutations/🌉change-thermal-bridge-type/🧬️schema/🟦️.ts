/** 🌉 `change-thermal-bridge-type` wire twin: the leaf payload `ChangeThermalBridgeType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeThermalBridgeType {
  newThermalBridgeType: number;
}

export const parseChangeThermalBridgeType: NormWireReader<ChangeThermalBridgeType> = normWireObject<ChangeThermalBridgeType>({ newThermalBridgeType: normWireRequired(normWireRange(normWireInteger, {"minimum":1,"maximum":3})) });
