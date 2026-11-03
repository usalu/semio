/** ➕️ `insert-bearing` wire twin: the leaf payload `InsertBearing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Bearing, parseEn1998Bearing } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBearing {
  mutation: "insertBearing";
  index?: number | null;
  bearing: En1998Bearing;
}

export const parseInsertBearing: NormWireReader<InsertBearing> = normWireObject<InsertBearing>({ mutation: normWireRequired(normWireLiteral("insertBearing")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), bearing: normWireRequired(parseEn1998Bearing) });
