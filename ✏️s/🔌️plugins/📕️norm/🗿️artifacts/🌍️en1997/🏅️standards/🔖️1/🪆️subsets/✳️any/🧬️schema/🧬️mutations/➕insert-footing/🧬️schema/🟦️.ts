/** ➕ `insert-footing` wire twin: the leaf payload `InsertFooting`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997SpreadFoundation, parseEn1997SpreadFoundation } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFooting {
  mutation: "insertFooting";
  index: number;
  footing: En1997SpreadFoundation;
}

export const parseInsertFooting: NormWireReader<InsertFooting> = normWireObject<InsertFooting>({ mutation: normWireRequired(normWireLiteral("insertFooting")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), footing: normWireRequired(parseEn1997SpreadFoundation) });
