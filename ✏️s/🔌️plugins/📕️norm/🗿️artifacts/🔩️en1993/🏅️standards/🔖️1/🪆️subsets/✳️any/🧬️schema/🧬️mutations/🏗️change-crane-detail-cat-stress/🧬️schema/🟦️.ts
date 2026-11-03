/** 🏗️ `change-crane-detail-cat-stress` wire twin: the leaf payload `ChangeCraneDetailCatStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneDetailCatStress {
  id: string;
  newDetailCategoryStress: number;
}

export const parseChangeCraneDetailCatStress: NormWireReader<ChangeCraneDetailCatStress> = normWireObject<ChangeCraneDetailCatStress>({ id: normWireRequired(normWireString), newDetailCategoryStress: normWireRequired(normWireNumber) });
