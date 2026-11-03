/** ➕️ `insert-accidental-case` wire twin: the leaf payload `InsertAccidentalCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AccidentalCase, parseAccidentalCase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAccidentalCase {
  index: number;
  item: AccidentalCase;
}

export const parseInsertAccidentalCase: NormWireReader<InsertAccidentalCase> = normWireObject<InsertAccidentalCase>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseAccidentalCase) });
