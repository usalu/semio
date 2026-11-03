/** 📚️ `change-load-case-category` wire twin: the leaf payload `ChangeLoadCaseCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadCaseCategory {
  id: string;
  newCategory: string;
}

export const parseChangeLoadCaseCategory: NormWireReader<ChangeLoadCaseCategory> = normWireObject<ChangeLoadCaseCategory>({ id: normWireRequired(normWireString), newCategory: normWireRequired(normWireString) });
