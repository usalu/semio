/** 🏋️ `change-action-category` wire twin: the leaf payload `ChangeActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionCategory, parseActionCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeActionCategory {
  mutation: "changeActionCategory";
  memberId: string;
  actionId: string;
  newCategory: ActionCategory;
}

export const parseChangeActionCategory: NormWireReader<ChangeActionCategory> = normWireObject<ChangeActionCategory>({ mutation: normWireRequired(normWireLiteral("changeActionCategory")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newCategory: normWireRequired(parseActionCategory) });
