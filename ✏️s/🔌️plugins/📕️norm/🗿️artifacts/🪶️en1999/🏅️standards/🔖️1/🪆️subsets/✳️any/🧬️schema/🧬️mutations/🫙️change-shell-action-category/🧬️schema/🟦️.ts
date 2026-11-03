/** 🫙 `change-shell-action-category` wire twin: the leaf payload `ChangeShellActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionCategory, parseActionCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeShellActionCategory {
  mutation: "changeShellActionCategory";
  shellId: string;
  actionId: string;
  newCategory: ActionCategory;
}

export const parseChangeShellActionCategory: NormWireReader<ChangeShellActionCategory> = normWireObject<ChangeShellActionCategory>({ mutation: normWireRequired(normWireLiteral("changeShellActionCategory")), shellId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newCategory: normWireRequired(parseActionCategory) });
