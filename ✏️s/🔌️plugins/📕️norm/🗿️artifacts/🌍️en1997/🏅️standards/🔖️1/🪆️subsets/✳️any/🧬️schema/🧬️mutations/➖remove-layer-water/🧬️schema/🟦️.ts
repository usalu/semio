/** ➖ `remove-layer-water` wire twin: the leaf payload `RemoveLayerWater`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveLayerWater {
  mutation: "removeLayerWater";
  index: number;
}

export const parseRemoveLayerWater: NormWireReader<RemoveLayerWater> = normWireObject<RemoveLayerWater>({ mutation: normWireRequired(normWireLiteral("removeLayerWater")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
