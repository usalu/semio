/** 🏗️ `change-thermal-element-type` wire twin: the leaf payload `ChangeThermalElementType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeThermalElementType {
  newThermalElementType: string;
}

export const parseChangeThermalElementType: NormWireReader<ChangeThermalElementType> = normWireObject<ChangeThermalElementType>({ newThermalElementType: normWireRequired(normWireString) });
