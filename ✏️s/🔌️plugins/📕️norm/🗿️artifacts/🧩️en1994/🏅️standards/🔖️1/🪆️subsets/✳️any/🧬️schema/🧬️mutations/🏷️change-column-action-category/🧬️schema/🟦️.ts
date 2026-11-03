/** 🏷️ `change-column-action-category` wire twin: the leaf payload `ChangeColumnActionCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnActionCategory {
  columnId: string;
  id: string;
  newCategory: string;
}

export const parseChangeColumnActionCategory: NormWireReader<ChangeColumnActionCategory> = normWireObject<ChangeColumnActionCategory>({ columnId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCategory: normWireRequired(normWireString) });
