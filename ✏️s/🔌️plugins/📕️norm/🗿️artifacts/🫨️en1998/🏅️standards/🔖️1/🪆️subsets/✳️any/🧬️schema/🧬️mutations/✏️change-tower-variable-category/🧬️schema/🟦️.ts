/** ✏️ `change-tower-variable-category` wire twin: the leaf payload `ChangeTowerVariableCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ImposedCategory, parseImposedCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTowerVariableCategory {
  mutation: "changeTowerVariableCategory";
  towerId: string;
  id: string;
  newCategory: ImposedCategory;
}

export const parseChangeTowerVariableCategory: NormWireReader<ChangeTowerVariableCategory> = normWireObject<ChangeTowerVariableCategory>({ mutation: normWireRequired(normWireLiteral("changeTowerVariableCategory")), towerId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCategory: normWireRequired(parseImposedCategory) });
