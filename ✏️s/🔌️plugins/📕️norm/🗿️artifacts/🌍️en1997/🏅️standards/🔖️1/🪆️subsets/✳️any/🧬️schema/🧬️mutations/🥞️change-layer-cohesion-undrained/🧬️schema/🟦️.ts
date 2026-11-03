/** 🥞️ `change-layer-cohesion-undrained` wire twin: the leaf payload `ChangeLayerCohesionUndrained`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerCohesionUndrained {
  mutation: "changeLayerCohesionUndrained";
  id: string;
  newCohesionUndrained: number;
}

export const parseChangeLayerCohesionUndrained: NormWireReader<ChangeLayerCohesionUndrained> = normWireObject<ChangeLayerCohesionUndrained>({ mutation: normWireRequired(normWireLiteral("changeLayerCohesionUndrained")), id: normWireRequired(normWireString), newCohesionUndrained: normWireRequired(normWireNumber) });
