/** ✏️ `change-tank-content-category` wire twin: the leaf payload `ChangeTankContentCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ImposedCategory, parseImposedCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTankContentCategory {
  mutation: "changeTankContentCategory";
  id: string;
  newContentCategory: ImposedCategory;
}

export const parseChangeTankContentCategory: NormWireReader<ChangeTankContentCategory> = normWireObject<ChangeTankContentCategory>({ mutation: normWireRequired(normWireLiteral("changeTankContentCategory")), id: normWireRequired(normWireString), newContentCategory: normWireRequired(parseImposedCategory) });
