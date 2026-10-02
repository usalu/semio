/** 🌀️ `change-layer-oedometric-modulus` wire twin: the leaf payload `ChangeLayerOedometricModulus`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLayerOedometricModulus {
  mutation: "changeLayerOedometricModulus";
  id: string;
  newOedometricModulus: number;
}

export const parseChangeLayerOedometricModulus: NormWireReader<ChangeLayerOedometricModulus> = normWireObject<ChangeLayerOedometricModulus>({ mutation: normWireRequired(normWireLiteral("changeLayerOedometricModulus")), id: normWireRequired(normWireString), newOedometricModulus: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
