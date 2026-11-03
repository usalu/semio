/** 🛢️ `insert-tank` wire twin: the leaf payload `InsertTank`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Tank, parseEn1998Tank } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTank {
  mutation: "insertTank";
  index?: number | null;
  tank: En1998Tank;
}

export const parseInsertTank: NormWireReader<InsertTank> = normWireObject<InsertTank>({ mutation: normWireRequired(normWireLiteral("insertTank")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), tank: normWireRequired(parseEn1998Tank) });
