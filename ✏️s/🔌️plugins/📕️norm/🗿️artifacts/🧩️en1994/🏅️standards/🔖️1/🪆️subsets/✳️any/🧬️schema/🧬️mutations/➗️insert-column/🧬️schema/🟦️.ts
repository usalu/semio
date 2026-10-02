/** ➗️ `insert-column` wire twin: the leaf payload `InsertColumn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CompositeColumn, parseCompositeColumn } from "../../../📸️snapshot/🟦️.ts";

export interface InsertColumn {
  index: number;
  column: CompositeColumn;
}

export const parseInsertColumn: NormWireReader<InsertColumn> = normWireObject<InsertColumn>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), column: normWireRequired(parseCompositeColumn) });
