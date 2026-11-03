/** ✏️ `change-bridge-variable-category` wire twin: the leaf payload `ChangeBridgeVariableCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ImposedCategory, parseImposedCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBridgeVariableCategory {
  mutation: "changeBridgeVariableCategory";
  bridgeId: string;
  id: string;
  newCategory: ImposedCategory;
}

export const parseChangeBridgeVariableCategory: NormWireReader<ChangeBridgeVariableCategory> = normWireObject<ChangeBridgeVariableCategory>({ mutation: normWireRequired(normWireLiteral("changeBridgeVariableCategory")), bridgeId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCategory: normWireRequired(parseImposedCategory) });
