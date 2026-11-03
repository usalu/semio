/** ➕️ `insert-soil-layer` wire twin: the leaf payload `InsertSoilLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998SoilLayer, parseEn1998SoilLayer } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSoilLayer {
  mutation: "insertSoilLayer";
  index?: number | null;
  layer: En1998SoilLayer;
}

export const parseInsertSoilLayer: NormWireReader<InsertSoilLayer> = normWireObject<InsertSoilLayer>({ mutation: normWireRequired(normWireLiteral("insertSoilLayer")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), layer: normWireRequired(parseEn1998SoilLayer) });
