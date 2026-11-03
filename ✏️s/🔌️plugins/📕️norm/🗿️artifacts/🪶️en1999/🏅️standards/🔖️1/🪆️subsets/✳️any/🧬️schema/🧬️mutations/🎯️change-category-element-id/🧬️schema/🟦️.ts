/** 🎯️ `change-category-element-id` wire twin: the leaf payload `ChangeCategoryElementId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCategoryElementId {
  mutation: "changeCategoryElementId";
  fatigueCategoryId: string;
  newElementId: string;
}

export const parseChangeCategoryElementId: NormWireReader<ChangeCategoryElementId> = normWireObject<ChangeCategoryElementId>({ mutation: normWireRequired(normWireLiteral("changeCategoryElementId")), fatigueCategoryId: normWireRequired(normWireString), newElementId: normWireRequired(normWireString) });
