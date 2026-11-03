/** 🥞️ `change-layer-depth-bottom` wire twin: the leaf payload `ChangeLayerDepthBottom`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerDepthBottom {
  mutation: "changeLayerDepthBottom";
  id: string;
  newDepthBottom: number;
}

export const parseChangeLayerDepthBottom: NormWireReader<ChangeLayerDepthBottom> = normWireObject<ChangeLayerDepthBottom>({ mutation: normWireRequired(normWireLiteral("changeLayerDepthBottom")), id: normWireRequired(normWireString), newDepthBottom: normWireRequired(normWireNumber) });
