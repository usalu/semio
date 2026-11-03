/** 🏋️ `change-anchor-action-category` wire twin: the leaf payload `ChangeAnchorActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PsiCategory, parsePsiCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAnchorActionCategory {
  anchorId: string;
  actionId: string;
  newCategory: PsiCategory;
}

export const parseChangeAnchorActionCategory: NormWireReader<ChangeAnchorActionCategory> = normWireObject<ChangeAnchorActionCategory>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newCategory: normWireRequired(parsePsiCategory) });
