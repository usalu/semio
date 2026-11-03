/** 🥞️ `change-layer-poisson-ratio` wire twin: the leaf payload `ChangeLayerPoissonRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerPoissonRatio {
  mutation: "changeLayerPoissonRatio";
  id: string;
  newPoissonRatio: number;
}

export const parseChangeLayerPoissonRatio: NormWireReader<ChangeLayerPoissonRatio> = normWireObject<ChangeLayerPoissonRatio>({ mutation: normWireRequired(normWireLiteral("changeLayerPoissonRatio")), id: normWireRequired(normWireString), newPoissonRatio: normWireRequired(normWireNumber) });
