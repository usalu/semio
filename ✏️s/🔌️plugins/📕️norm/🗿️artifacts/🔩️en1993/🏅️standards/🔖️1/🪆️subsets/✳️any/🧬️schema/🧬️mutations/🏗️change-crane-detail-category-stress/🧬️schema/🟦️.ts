/** 🏗️ `change-crane-detail-category-stress` wire twin: the leaf payload `ChangeCraneDetailCategoryStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneDetailCategoryStress {
  id: string;
  newDetailCategoryStress: number;
}

export const parseChangeCraneDetailCategoryStress: NormWireReader<ChangeCraneDetailCategoryStress> = normWireObject<ChangeCraneDetailCategoryStress>({ id: normWireRequired(normWireString), newDetailCategoryStress: normWireRequired(normWireNumber) });
