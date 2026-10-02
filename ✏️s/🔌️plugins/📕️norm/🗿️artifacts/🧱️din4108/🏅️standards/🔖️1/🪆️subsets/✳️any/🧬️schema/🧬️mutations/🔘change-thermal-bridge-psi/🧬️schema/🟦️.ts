/** 🔘 `change-thermal-bridge-psi` wire twin: the leaf payload `ChangeThermalBridgePsi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeThermalBridgePsi {
  bridgeId: string;
  newPsi: number;
}

export const parseChangeThermalBridgePsi: NormWireReader<ChangeThermalBridgePsi> = normWireObject<ChangeThermalBridgePsi>({ bridgeId: normWireRequired(normWireString), newPsi: normWireRequired(normWireNumber) });
