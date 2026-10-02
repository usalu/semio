/** 📐️ `change-layer-phi-prime` wire twin: the leaf payload `ChangeLayerPhiPrime`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLayerPhiPrime {
  mutation: "changeLayerPhiPrime";
  id: string;
  newPhiPrimeDeg: number;
}

export const parseChangeLayerPhiPrime: NormWireReader<ChangeLayerPhiPrime> = normWireObject<ChangeLayerPhiPrime>({ mutation: normWireRequired(normWireLiteral("changeLayerPhiPrime")), id: normWireRequired(normWireString), newPhiPrimeDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":90})) });
