/** ➕️ `insert-case` wire twin: the leaf payload `InsertCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CaseRow, parseCaseRow } from "../../../📸️snapshot/🟦️.ts";

export interface InsertCase {
  index: number;
  case: CaseRow;
}

export const parseInsertCase: NormWireReader<InsertCase> = normWireObject<InsertCase>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), case: normWireRequired(parseCaseRow) });
