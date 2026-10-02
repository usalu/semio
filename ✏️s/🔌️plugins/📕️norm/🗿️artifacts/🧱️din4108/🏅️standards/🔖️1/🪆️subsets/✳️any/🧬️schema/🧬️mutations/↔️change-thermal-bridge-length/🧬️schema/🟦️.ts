/** ↔️ `change-thermal-bridge-length` wire twin: the leaf payload `ChangeThermalBridgeLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeThermalBridgeLength {
  bridgeId: string;
  newLengthM: number;
}

export const parseChangeThermalBridgeLength: NormWireReader<ChangeThermalBridgeLength> = normWireObject<ChangeThermalBridgeLength>({ bridgeId: normWireRequired(normWireString), newLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
