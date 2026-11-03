/** 🥞️ `change-layer-depth-top` wire twin: the leaf payload `ChangeLayerDepthTop`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerDepthTop {
  mutation: "changeLayerDepthTop";
  id: string;
  newDepthTop: number;
}

export const parseChangeLayerDepthTop: NormWireReader<ChangeLayerDepthTop> = normWireObject<ChangeLayerDepthTop>({ mutation: normWireRequired(normWireLiteral("changeLayerDepthTop")), id: normWireRequired(normWireString), newDepthTop: normWireRequired(normWireNumber) });
