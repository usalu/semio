/** 🏗️ `insert-crane-action` wire twin: the leaf payload `InsertCraneAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ForceAction, parseForceAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertCraneAction {
  id: string;
  index?: number | null;
  element: ForceAction;
}

export const parseInsertCraneAction: NormWireReader<InsertCraneAction> = normWireObject<InsertCraneAction>({ id: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), element: normWireRequired(parseForceAction) });
