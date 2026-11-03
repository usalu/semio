/** 🥞️ `change-layer-gamma` wire twin: the leaf payload `ChangeLayerGamma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerGamma {
  mutation: "changeLayerGamma";
  id: string;
  newGamma: number;
}

export const parseChangeLayerGamma: NormWireReader<ChangeLayerGamma> = normWireObject<ChangeLayerGamma>({ mutation: normWireRequired(normWireLiteral("changeLayerGamma")), id: normWireRequired(normWireString), newGamma: normWireRequired(normWireNumber) });
