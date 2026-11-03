/** ➕️ `insert-seismic-slope` wire twin: the leaf payload `InsertSeismicSlope`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Slope, parseEn1998Slope } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSeismicSlope {
  mutation: "insertSeismicSlope";
  index?: number | null;
  seismicSlope: En1998Slope;
}

export const parseInsertSeismicSlope: NormWireReader<InsertSeismicSlope> = normWireObject<InsertSeismicSlope>({ mutation: normWireRequired(normWireLiteral("insertSeismicSlope")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), seismicSlope: normWireRequired(parseEn1998Slope) });
