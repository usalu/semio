/** ➕ `insert-slope` wire twin: the leaf payload `InsertSlope`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997Slope, parseEn1997Slope } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSlope {
  mutation: "insertSlope";
  index: number;
  slope: En1997Slope;
}

export const parseInsertSlope: NormWireReader<InsertSlope> = normWireObject<InsertSlope>({ mutation: normWireRequired(normWireLiteral("insertSlope")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), slope: normWireRequired(parseEn1997Slope) });
