/** ❄️ `change-sheet-action-category` wire twin: the leaf payload `ChangeSheetActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionCategory, parseActionCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSheetActionCategory {
  mutation: "changeSheetActionCategory";
  coldFormedId: string;
  actionId: string;
  newCategory: ActionCategory;
}

export const parseChangeSheetActionCategory: NormWireReader<ChangeSheetActionCategory> = normWireObject<ChangeSheetActionCategory>({ mutation: normWireRequired(normWireLiteral("changeSheetActionCategory")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newCategory: normWireRequired(parseActionCategory) });
