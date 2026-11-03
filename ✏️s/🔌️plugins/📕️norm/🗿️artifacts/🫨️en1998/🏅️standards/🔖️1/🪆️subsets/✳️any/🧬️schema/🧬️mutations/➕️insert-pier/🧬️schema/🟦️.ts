/** ➕️ `insert-pier` wire twin: the leaf payload `InsertPier`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Pier, parseEn1998Pier } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPier {
  mutation: "insertPier";
  index?: number | null;
  pier: En1998Pier;
}

export const parseInsertPier: NormWireReader<InsertPier> = normWireObject<InsertPier>({ mutation: normWireRequired(normWireLiteral("insertPier")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), pier: normWireRequired(parseEn1998Pier) });
