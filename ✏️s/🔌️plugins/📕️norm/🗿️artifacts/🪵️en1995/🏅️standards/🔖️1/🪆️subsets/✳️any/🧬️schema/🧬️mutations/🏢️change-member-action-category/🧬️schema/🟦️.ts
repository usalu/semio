/** 🏢️ `change-member-action-category` wire twin: the leaf payload `ChangeMemberActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ImposedCategory, parseImposedCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberActionCategory {
  memberId: string;
  actionId: string;
  newValue: ImposedCategory;
}

export const parseChangeMemberActionCategory: NormWireReader<ChangeMemberActionCategory> = normWireObject<ChangeMemberActionCategory>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(parseImposedCategory) });
