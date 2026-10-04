/** 🏷️ `change-thermal-bridge-bb2-type` wire twin: the leaf payload `ChangeThermalBridgeBb2Type`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeThermalBridgeBb2Type {
  bridgeId: string;
  newBb2Type: string;
}

export const parseChangeThermalBridgeBb2Type: NormWireReader<ChangeThermalBridgeBb2Type> = normWireObject<ChangeThermalBridgeBb2Type>({ bridgeId: normWireRequired(normWireString), newBb2Type: normWireRequired(normWireString) });
