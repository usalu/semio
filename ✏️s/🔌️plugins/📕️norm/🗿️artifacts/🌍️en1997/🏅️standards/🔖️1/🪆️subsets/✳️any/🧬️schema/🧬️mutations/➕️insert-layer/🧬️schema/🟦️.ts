/** ➕️ `insert-layer` wire twin: the leaf payload `InsertLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997SoilLayer, parseEn1997SoilLayer } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLayer {
  mutation: "insertLayer";
  index: number;
  layer: En1997SoilLayer;
}

export const parseInsertLayer: NormWireReader<InsertLayer> = normWireObject<InsertLayer>({ mutation: normWireRequired(normWireLiteral("insertLayer")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), layer: normWireRequired(parseEn1997SoilLayer) });
