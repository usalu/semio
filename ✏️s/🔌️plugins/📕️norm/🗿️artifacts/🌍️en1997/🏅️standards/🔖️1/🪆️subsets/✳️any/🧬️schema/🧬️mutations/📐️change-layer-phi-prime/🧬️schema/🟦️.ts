/** 📐️ `change-layer-phi-prime` wire twin: the leaf payload `ChangeLayerPhiPrime`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerPhiPrime {
  mutation: "changeLayerPhiPrime";
  id: string;
  newPhiPrimeDeg: number;
}

export const parseChangeLayerPhiPrime: NormWireReader<ChangeLayerPhiPrime> = normWireObject<ChangeLayerPhiPrime>({ mutation: normWireRequired(normWireLiteral("changeLayerPhiPrime")), id: normWireRequired(normWireString), newPhiPrimeDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":90})) });
