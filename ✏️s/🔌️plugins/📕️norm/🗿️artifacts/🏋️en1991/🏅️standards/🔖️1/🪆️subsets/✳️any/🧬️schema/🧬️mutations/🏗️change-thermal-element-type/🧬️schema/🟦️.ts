/** 🏗️ `change-thermal-element-type` wire twin: the leaf payload `ChangeThermalElementType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeThermalElementType {
  newThermalElementType: string;
}

export const parseChangeThermalElementType: NormWireReader<ChangeThermalElementType> = normWireObject<ChangeThermalElementType>({ newThermalElementType: normWireRequired(normWireString) });
