/** 🌀️ `change-layer-oedometric-modulus` wire twin: the leaf payload `ChangeLayerOedometricModulus`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerOedometricModulus {
  mutation: "changeLayerOedometricModulus";
  id: string;
  newOedometricModulus: number;
}

export const parseChangeLayerOedometricModulus: NormWireReader<ChangeLayerOedometricModulus> = normWireObject<ChangeLayerOedometricModulus>({ mutation: normWireRequired(normWireLiteral("changeLayerOedometricModulus")), id: normWireRequired(normWireString), newOedometricModulus: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
