/** 🌉️ `insert-thermal-bridge` wire twin: the leaf payload `InsertThermalBridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";
import { type Din4108ThermalBridge, parseDin4108ThermalBridge } from "../../../📸️snapshot/🟦️.ts";

export interface InsertThermalBridge {
  index: number | null;
  bridge: Din4108ThermalBridge;
}

export const parseInsertThermalBridge: NormWireReader<InsertThermalBridge> = normWireObject<InsertThermalBridge>({ index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null), bridge: normWireRequired(parseDin4108ThermalBridge) });
