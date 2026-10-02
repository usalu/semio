/** 🌉️ `insert-thermal-bridge` wire twin: the leaf payload `InsertThermalBridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108ThermalBridge, parseDin4108ThermalBridge } from "../../../📸️snapshot/🟦️.ts";

export interface InsertThermalBridge {
  index: number;
  bridge: Din4108ThermalBridge;
}

export const parseInsertThermalBridge: NormWireReader<InsertThermalBridge> = normWireObject<InsertThermalBridge>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), bridge: normWireRequired(parseDin4108ThermalBridge) });
