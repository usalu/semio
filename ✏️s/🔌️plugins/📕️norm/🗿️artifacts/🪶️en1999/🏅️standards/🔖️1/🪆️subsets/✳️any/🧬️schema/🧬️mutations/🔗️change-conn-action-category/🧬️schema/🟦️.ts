/** 🔗️ `change-conn-action-category` wire twin: the leaf payload `ChangeConnActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionCategory, parseActionCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeConnActionCategory {
  mutation: "changeConnActionCategory";
  connectionId: string;
  actionId: string;
  newCategory: ActionCategory;
}

export const parseChangeConnActionCategory: NormWireReader<ChangeConnActionCategory> = normWireObject<ChangeConnActionCategory>({ mutation: normWireRequired(normWireLiteral("changeConnActionCategory")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newCategory: normWireRequired(parseActionCategory) });
