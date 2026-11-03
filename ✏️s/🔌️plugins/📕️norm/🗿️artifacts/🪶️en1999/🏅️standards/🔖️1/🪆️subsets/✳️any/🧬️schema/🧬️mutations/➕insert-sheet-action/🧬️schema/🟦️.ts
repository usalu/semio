/** 🗂️ `insert-sheet-action` wire twin: the leaf payload `InsertSheetAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type MemberAction, parseMemberAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSheetAction {
  mutation: "insertSheetAction";
  coldFormedId: string;
  index: number;
  action: MemberAction;
}

export const parseInsertSheetAction: NormWireReader<InsertSheetAction> = normWireObject<InsertSheetAction>({ mutation: normWireRequired(normWireLiteral("insertSheetAction")), coldFormedId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), action: normWireRequired(parseMemberAction) });
