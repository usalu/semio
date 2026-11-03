/** 🗼️ `insert-tower-action` wire twin: the leaf payload `InsertTowerAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ForceAction, parseForceAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTowerAction {
  id: string;
  index?: number | null;
  element: ForceAction;
}

export const parseInsertTowerAction: NormWireReader<InsertTowerAction> = normWireObject<InsertTowerAction>({ id: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), element: normWireRequired(parseForceAction) });
