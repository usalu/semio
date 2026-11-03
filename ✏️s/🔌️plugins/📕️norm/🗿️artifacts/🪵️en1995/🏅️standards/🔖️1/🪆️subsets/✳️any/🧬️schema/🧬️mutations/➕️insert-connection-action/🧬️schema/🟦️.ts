/** ➕️ `insert-connection-action` wire twin: the leaf payload `InsertConnectionAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ConnectionAction, parseConnectionAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConnectionAction {
  connectionId: string;
  index: number;
  action: ConnectionAction;
}

export const parseInsertConnectionAction: NormWireReader<InsertConnectionAction> = normWireObject<InsertConnectionAction>({ connectionId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), action: normWireRequired(parseConnectionAction) });
