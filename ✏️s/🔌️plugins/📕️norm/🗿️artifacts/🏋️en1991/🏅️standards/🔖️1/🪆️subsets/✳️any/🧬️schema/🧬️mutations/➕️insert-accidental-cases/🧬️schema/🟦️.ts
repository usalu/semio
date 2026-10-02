/** ➕️ `insert-accidental-cases` wire twin: the leaf payload `InsertAccidentalCases`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AccidentalCase, parseAccidentalCase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAccidentalCases {
  index: number;
  item: AccidentalCase;
}

export const parseInsertAccidentalCases: NormWireReader<InsertAccidentalCases> = normWireObject<InsertAccidentalCases>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseAccidentalCase) });
