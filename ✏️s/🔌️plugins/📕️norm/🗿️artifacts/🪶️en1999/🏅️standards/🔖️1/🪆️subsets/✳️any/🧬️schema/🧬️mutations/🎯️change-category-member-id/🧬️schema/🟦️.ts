/** 🎯️ `change-category-member-id` wire twin: the leaf payload `ChangeCategoryMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCategoryMemberId {
  mutation: "changeCategoryMemberId";
  fatigueCategoryId: string;
  newMemberId: string;
}

export const parseChangeCategoryMemberId: NormWireReader<ChangeCategoryMemberId> = normWireObject<ChangeCategoryMemberId>({ mutation: normWireRequired(normWireLiteral("changeCategoryMemberId")), fatigueCategoryId: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
