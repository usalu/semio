/** 🏛️ `insert-column-action` wire twin: the leaf payload `InsertColumnAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ColumnAction, parseColumnAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertColumnAction {
  columnId: string;
  index: number;
  action: ColumnAction;
}

export const parseInsertColumnAction: NormWireReader<InsertColumnAction> = normWireObject<InsertColumnAction>({ columnId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), action: normWireRequired(parseColumnAction) });
