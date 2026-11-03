/** 🎯️ `change-category-detail-category` wire twin: the leaf payload `ChangeCategoryDetailCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCategoryDetailCategory {
  mutation: "changeCategoryDetailCategory";
  fatigueCategoryId: string;
  newDetailCategory: string;
}

export const parseChangeCategoryDetailCategory: NormWireReader<ChangeCategoryDetailCategory> = normWireObject<ChangeCategoryDetailCategory>({ mutation: normWireRequired(normWireLiteral("changeCategoryDetailCategory")), fatigueCategoryId: normWireRequired(normWireString), newDetailCategory: normWireRequired(normWireString) });
