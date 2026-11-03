/** 🎯️ `change-category-detail-type` wire twin: the leaf payload `ChangeCategoryDetailType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DetailType, parseDetailType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeCategoryDetailType {
  mutation: "changeCategoryDetailType";
  fatigueCategoryId: string;
  newDetailType: DetailType;
}

export const parseChangeCategoryDetailType: NormWireReader<ChangeCategoryDetailType> = normWireObject<ChangeCategoryDetailType>({ mutation: normWireRequired(normWireLiteral("changeCategoryDetailType")), fatigueCategoryId: normWireRequired(normWireString), newDetailType: normWireRequired(parseDetailType) });
