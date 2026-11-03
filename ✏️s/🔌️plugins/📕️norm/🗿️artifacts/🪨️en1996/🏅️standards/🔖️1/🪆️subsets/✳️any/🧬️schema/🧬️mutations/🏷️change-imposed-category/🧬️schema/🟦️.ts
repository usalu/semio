/** 🏷️ `change-imposed-category` wire twin: the leaf payload `ChangeImposedCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996ImposedCategory, parseEn1996ImposedCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeImposedCategory {
  wallId: string;
  id: string;
  newImposedCategory: En1996ImposedCategory;
}

export const parseChangeImposedCategory: NormWireReader<ChangeImposedCategory> = normWireObject<ChangeImposedCategory>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newImposedCategory: normWireRequired(parseEn1996ImposedCategory) });
