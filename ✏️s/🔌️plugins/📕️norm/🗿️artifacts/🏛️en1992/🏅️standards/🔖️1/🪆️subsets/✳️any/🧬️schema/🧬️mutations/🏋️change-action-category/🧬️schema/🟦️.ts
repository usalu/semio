/** 🏋️ `change-action-category` wire twin: the leaf payload `ChangeActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PsiCategory, parsePsiCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeActionCategory {
  memberId: string;
  actionId: string;
  newCategory: PsiCategory;
}

export const parseChangeActionCategory: NormWireReader<ChangeActionCategory> = normWireObject<ChangeActionCategory>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newCategory: normWireRequired(parsePsiCategory) });
