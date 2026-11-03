/** 🏋️ `insert-anchor-action` wire twin: the leaf payload `InsertAnchorAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LoadCaseActions, parseLoadCaseActions } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAnchorAction {
  anchorId: string;
  index: number;
  anchorAction: LoadCaseActions;
}

export const parseInsertAnchorAction: NormWireReader<InsertAnchorAction> = normWireObject<InsertAnchorAction>({ anchorId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), anchorAction: normWireRequired(parseLoadCaseActions) });
