/** ➕️ `insert-case` wire twin: the leaf payload `InsertCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997CaseRow, parseEn1997CaseRow } from "../../../📸️snapshot/🟦️.ts";

export interface InsertCase {
  mutation: "insertCase";
  index: number;
  case: En1997CaseRow;
}

export const parseInsertCase: NormWireReader<InsertCase> = normWireObject<InsertCase>({ mutation: normWireRequired(normWireLiteral("insertCase")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), case: normWireRequired(parseEn1997CaseRow) });
