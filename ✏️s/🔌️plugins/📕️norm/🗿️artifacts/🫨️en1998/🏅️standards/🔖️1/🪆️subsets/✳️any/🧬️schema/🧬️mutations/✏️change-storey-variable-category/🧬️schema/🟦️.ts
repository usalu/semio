/** ✏️ `change-storey-variable-category` wire twin: the leaf payload `ChangeStoreyVariableCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ImposedCategory, parseImposedCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeStoreyVariableCategory {
  mutation: "changeStoreyVariableCategory";
  buildingId: string;
  storeyId: string;
  id: string;
  newCategory: ImposedCategory;
}

export const parseChangeStoreyVariableCategory: NormWireReader<ChangeStoreyVariableCategory> = normWireObject<ChangeStoreyVariableCategory>({ mutation: normWireRequired(normWireLiteral("changeStoreyVariableCategory")), buildingId: normWireRequired(normWireString), storeyId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCategory: normWireRequired(parseImposedCategory) });
