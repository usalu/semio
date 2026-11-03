/** ➕ `insert-layer-water` wire twin: the leaf payload `InsertLayerWater`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997LayerWater, parseEn1997LayerWater } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLayerWater {
  mutation: "insertLayerWater";
  index: number;
  layerWater: En1997LayerWater;
}

export const parseInsertLayerWater: NormWireReader<InsertLayerWater> = normWireObject<InsertLayerWater>({ mutation: normWireRequired(normWireLiteral("insertLayerWater")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), layerWater: normWireRequired(parseEn1997LayerWater) });
