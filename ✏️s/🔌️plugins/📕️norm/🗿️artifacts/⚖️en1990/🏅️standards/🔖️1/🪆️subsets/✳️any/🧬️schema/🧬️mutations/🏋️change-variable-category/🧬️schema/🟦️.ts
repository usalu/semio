/** 🏋️ `change-variable-category` wire twin: the leaf payload `ChangeVariableCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVariableCategory {
  mutation: "changeVariableCategory";
  id: string;
  newCategory: string;
}

export const parseChangeVariableCategory: NormWireReader<ChangeVariableCategory> = normWireObject<ChangeVariableCategory>({ mutation: normWireRequired(normWireLiteral("changeVariableCategory")), id: normWireRequired(normWireString), newCategory: normWireRequired(normWireString) });
